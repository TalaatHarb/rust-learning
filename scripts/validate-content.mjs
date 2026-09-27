import { readdirSync, readFileSync, statSync, existsSync, writeFileSync } from 'node:fs'
import { join, resolve } from 'node:path'
import { spawnSync } from 'node:child_process'

const root = resolve(new URL('..', import.meta.url).pathname)
const contentRoot = join(root, 'content')
const roadmapsRoot = join(contentRoot, 'roadmaps')

function listJsonFiles(dir) {
  const entries = readdirSync(dir)
  const files = []

  for (const entry of entries) {
    const full = join(dir, entry)
    const stat = statSync(full)
    if (stat.isDirectory()) {
      files.push(...listJsonFiles(full))
      continue
    }

    if (entry.endsWith('.json')) files.push(full)
  }

  return files
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message)
  }
}

function loadJson(path) {
  return JSON.parse(readFileSync(path, 'utf-8'))
}

function deriveSlug(id, expectedPrefix) {
  const parts = id.split('.')
  assert(parts[0] === expectedPrefix, `Expected ${expectedPrefix} id format, received ${id}`)
  const slug = parts.at(2)
  assert(typeof slug === 'string' && slug.length > 0, `Unable to derive slug from id ${id}`)
  return slug
}

const allJsonFiles = listJsonFiles(contentRoot)
const ids = []
for (const file of allJsonFiles) {
  const data = loadJson(file)
  if (typeof data.id === 'string') ids.push({ id: data.id, file })
}

const duplicateIds = ids
  .map(({ id }) => id)
  .filter((id, index, arr) => arr.indexOf(id) !== index)
assert(duplicateIds.length === 0, `Duplicate content IDs detected: ${duplicateIds.join(', ')}`)

const roadmapFiles = listJsonFiles(roadmapsRoot)
assert(roadmapFiles.length > 0, 'At least one roadmap file is required')

const referencedUnitIds = new Set()
for (const roadmapFile of roadmapFiles) {
  const roadmap = loadJson(roadmapFile)
  assert(typeof roadmap.id === 'string', `Roadmap must have string id: ${roadmapFile}`)
  assert(typeof roadmap.title === 'string', `Roadmap must have title: ${roadmapFile}`)
  assert(Array.isArray(roadmap.modules), `Roadmap modules must be an array: ${roadmapFile}`)

  for (const module of roadmap.modules) {
    assert(typeof module.id === 'string', `Roadmap module id must be string: ${roadmapFile}`)
    assert(typeof module.title === 'string', `Roadmap module title must be string: ${roadmapFile}`)
    assert(Array.isArray(module.unit_ids), `Roadmap module unit_ids must be an array: ${roadmapFile}`)

    for (const unitId of module.unit_ids) {
      assert(typeof unitId === 'string', `Roadmap unit id must be string: ${roadmapFile}`)
      referencedUnitIds.add(unitId)
    }
  }
}

assert(referencedUnitIds.size > 0, 'Roadmaps must include at least one unit id')

const unitFiles = new Map()
for (const unitId of referencedUnitIds) {
  const slug = deriveSlug(unitId, 'unit')
  const unitPath = join(contentRoot, 'units', `${slug}.json`)
  assert(existsSync(unitPath), `Unit file does not exist: ${unitPath}`)

  const unit = loadJson(unitPath)
  assert(unit.id === unitId, `Unit id mismatch for ${unitPath}; expected ${unitId}`)
  assert(typeof unit.title === 'string', `Unit title must be string: ${unitPath}`)
  assert(Array.isArray(unit.learning_objectives), `Unit learning_objectives must be an array: ${unitPath}`)
  assert(Array.isArray(unit.prerequisite_unit_ids), `Unit prerequisites must be an array: ${unitPath}`)
  assert(typeof unit.exercise_id === 'string', `Unit exercise_id must be a string: ${unitPath}`)

  unitFiles.set(unit.id, { unit, slug, unitPath })
}

for (const [unitId, { unit, unitPath }] of unitFiles.entries()) {
  for (const prerequisiteId of unit.prerequisite_unit_ids) {
    assert(typeof prerequisiteId === 'string', `Prerequisite ID must be string in ${unitPath}`)
    assert(
      unitFiles.has(prerequisiteId),
      `Prerequisite ${prerequisiteId} for unit ${unitId} must exist in roadmap-linked units`,
    )
  }
}

const visiting = new Set()
const visited = new Set()
function detectCycle(unitId, stack = []) {
  if (visiting.has(unitId)) {
    return [...stack, unitId]
  }
  if (visited.has(unitId)) return null

  visiting.add(unitId)
  const unit = unitFiles.get(unitId)?.unit
  for (const dependency of unit?.prerequisite_unit_ids ?? []) {
    const cycle = detectCycle(dependency, [...stack, unitId])
    if (cycle) return cycle
  }
  visiting.delete(unitId)
  visited.add(unitId)
  return null
}

for (const unitId of unitFiles.keys()) {
  const cycle = detectCycle(unitId)
  if (cycle) {
    throw new Error(`Dependency cycle detected in unit prerequisites: ${cycle.join(' -> ')}`)
  }
}

for (const [unitId, { unit }] of unitFiles.entries()) {
  const exerciseSlug = deriveSlug(unit.exercise_id, 'exercise')
  const exercisePath = join(contentRoot, 'exercises', `${exerciseSlug}.json`)
  assert(existsSync(exercisePath), `Exercise file does not exist: ${exercisePath}`)

  const exercise = loadJson(exercisePath)
  assert(exercise.id === unit.exercise_id, `Exercise id mismatch for unit ${unitId}`)
  assert(typeof exercise.title === 'string', `Exercise title is required for ${exercisePath}`)
  assert(typeof exercise.template_path === 'string', `Exercise template_path is required for ${unitId}`)
  assert(typeof exercise.starter_file === 'string', `Exercise starter_file is required for ${unitId}`)
  assert(
    typeof exercise.expected_solution_file === 'string',
    `Exercise expected_solution_file is required for ${unitId}`,
  )

  const templatePath = join(root, exercise.template_path)
  const starterPath = join(templatePath, exercise.starter_file)
  const expectedSolutionPath = join(templatePath, exercise.expected_solution_file)

  assert(existsSync(templatePath), `Template path does not exist: ${templatePath}`)
  assert(existsSync(starterPath), `Starter file does not exist: ${starterPath}`)
  assert(existsSync(expectedSolutionPath), `Expected solution file does not exist: ${expectedSolutionPath}`)

  const cargoCheck = spawnSync('cargo', ['check'], {
    cwd: templatePath,
    stdio: 'pipe',
    encoding: 'utf-8',
  })
  assert(cargoCheck.status === 0, `Exercise template must compile at ${templatePath}:\n${cargoCheck.stderr}`)

  const starterSource = readFileSync(starterPath, 'utf-8')
  const solutionSource = readFileSync(expectedSolutionPath, 'utf-8')

  try {
    writeFileSync(starterPath, solutionSource)
    const cargoTest = spawnSync('cargo', ['test', '--quiet'], {
      cwd: templatePath,
      stdio: 'pipe',
      encoding: 'utf-8',
    })
    assert(
      cargoTest.status === 0,
      `Exercise solution tests must pass at ${templatePath}:\n${cargoTest.stderr}`,
    )
  } finally {
    writeFileSync(starterPath, starterSource)
  }
}

console.log('Content validation passed')
