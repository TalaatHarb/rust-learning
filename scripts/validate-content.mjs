import { readdirSync, readFileSync, statSync, existsSync, writeFileSync } from 'node:fs'
import { join, resolve } from 'node:path'
import { spawnSync } from 'node:child_process'

const root = resolve(new URL('..', import.meta.url).pathname)
const contentRoot = join(root, 'content')
const roadmapPath = join(contentRoot, 'roadmaps', 'foundations.json')

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

const roadmap = loadJson(roadmapPath)
assert(typeof roadmap.id === 'string', 'Roadmap must have string id')
assert(Array.isArray(roadmap.modules), 'Roadmap modules must be an array')

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

const roadmapUnitIds = roadmap.modules.flatMap((module) => module.unit_ids ?? [])
assert(roadmapUnitIds.length > 0, 'Roadmap must include at least one unit id')

const unitFiles = new Map()
for (const unitId of roadmapUnitIds) {
  const slug = unitId.split('.').at(2)
  assert(typeof slug === 'string' && slug.length > 0, `Unable to derive slug from unit id ${unitId}`)

  const unitPath = join(contentRoot, 'units', `${slug}.json`)
  assert(existsSync(unitPath), `Unit file does not exist: ${unitPath}`)

  const unit = loadJson(unitPath)
  assert(unit.id === unitId, `Unit id mismatch for ${unitPath}; expected ${unitId}`)
  assert(Array.isArray(unit.prerequisite_unit_ids), `Unit prerequisites must be an array: ${unitPath}`)
  assert(typeof unit.exercise_id === 'string', `Unit exercise_id must be a string: ${unitPath}`)

  unitFiles.set(unit.id, unit)
}

for (const [unitId, unit] of unitFiles.entries()) {
  for (const prerequisiteId of unit.prerequisite_unit_ids) {
    assert(typeof prerequisiteId === 'string', `Prerequisite ID must be string in ${unitId}`)
    assert(
      unitFiles.has(prerequisiteId),
      `Prerequisite ${prerequisiteId} for unit ${unitId} must exist in roadmap units`,
    )
  }
}

const visiting = new Set()
const visited = new Set()
function detectCycle(node) {
  if (visiting.has(node)) return true
  if (visited.has(node)) return false

  visiting.add(node)
  const unit = unitFiles.get(node)
  for (const dependency of unit?.prerequisite_unit_ids ?? []) {
    if (detectCycle(dependency)) return true
  }
  visiting.delete(node)
  visited.add(node)
  return false
}

for (const unitId of unitFiles.keys()) {
  assert(!detectCycle(unitId), `Dependency cycle detected in unit prerequisites at ${unitId}`)
}

for (const [unitId, unit] of unitFiles.entries()) {
  const exerciseDomain = unit.exercise_id.split('.')
  const exerciseSlug = exerciseDomain.at(2)
  assert(
    typeof exerciseSlug === 'string' && exerciseSlug.length > 0,
    `Unable to derive exercise slug for ${unitId}`,
  )

  const exercisePath = join(contentRoot, 'exercises', `${exerciseSlug}.json`)
  assert(existsSync(exercisePath), `Exercise file does not exist: ${exercisePath}`)

  const exercise = loadJson(exercisePath)
  assert(exercise.id === unit.exercise_id, `Exercise id mismatch for unit ${unitId}`)
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
