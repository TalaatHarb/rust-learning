import { readdirSync, readFileSync, statSync, existsSync } from 'node:fs'
import { join, resolve } from 'node:path'
import { spawnSync } from 'node:child_process'

const root = resolve(new URL('..', import.meta.url).pathname)
const contentRoot = join(root, 'content')

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

const roadmapPath = join(contentRoot, 'roadmaps', 'foundations.json')
const unitPath = join(contentRoot, 'units', 'ownership.json')
const exercisePath = join(contentRoot, 'exercises', 'ownership.json')

const roadmap = loadJson(roadmapPath)
const unit = loadJson(unitPath)
const exercise = loadJson(exercisePath)

assert(typeof roadmap.id === 'string', 'Roadmap must have string id')
assert(Array.isArray(roadmap.modules), 'Roadmap modules must be an array')
assert(typeof unit.id === 'string', 'Unit must have string id')
assert(Array.isArray(unit.prerequisite_unit_ids), 'Unit prerequisites must be an array')
assert(typeof exercise.id === 'string', 'Exercise must have string id')
assert(typeof exercise.template_path === 'string', 'Exercise template_path is required')

const ids = []
for (const file of listJsonFiles(contentRoot)) {
  const data = loadJson(file)
  if (typeof data.id === 'string') ids.push({ id: data.id, file })
}

const duplicateIds = ids
  .map(({ id }) => id)
  .filter((id, index, arr) => arr.indexOf(id) !== index)

assert(duplicateIds.length === 0, `Duplicate content IDs detected: ${duplicateIds.join(', ')}`)

const roadmapUnitIds = new Set(roadmap.modules.flatMap((module) => module.unit_ids ?? []))
assert(roadmapUnitIds.has(unit.id), `Roadmap must reference unit id ${unit.id}`)
assert(unit.exercise_id === exercise.id, `Unit exercise_id must reference existing exercise id ${exercise.id}`)

for (const prerequisiteId of unit.prerequisite_unit_ids) {
  assert(typeof prerequisiteId === 'string', 'Prerequisite IDs must be strings')
}

const adjacency = new Map([[unit.id, unit.prerequisite_unit_ids]])
const visiting = new Set()
const visited = new Set()

function detectCycle(node) {
  if (visiting.has(node)) return true
  if (visited.has(node)) return false

  visiting.add(node)
  for (const neighbor of adjacency.get(node) ?? []) {
    if (neighbor === unit.id) return true
    if (adjacency.has(neighbor) && detectCycle(neighbor)) return true
  }

  visiting.delete(node)
  visited.add(node)
  return false
}

assert(!detectCycle(unit.id), 'Dependency cycle detected in unit prerequisites')

const templatePath = join(root, exercise.template_path)
const starterPath = join(templatePath, exercise.starter_file)
const expectedSolutionPath = join(templatePath, exercise.expected_solution_file)
assert(existsSync(templatePath), `Template path does not exist: ${templatePath}`)
assert(existsSync(starterPath), `Starter file does not exist: ${starterPath}`)
assert(existsSync(expectedSolutionPath), `Expected solution file does not exist: ${expectedSolutionPath}`)

const cargoCheck = spawnSync('cargo', ['check'], { cwd: templatePath, stdio: 'pipe', encoding: 'utf-8' })
assert(cargoCheck.status === 0, `Exercise template must compile:\n${cargoCheck.stderr}`)

console.log('Content validation passed')
