export type UnitExample = {
  id: string
  title: string
  code: string
}

export type UnitResponse = {
  id: string
  title: string
  learning_objectives: string[]
  explanation: string
  examples: UnitExample[]
  compiler_error_examples: string[]
  exercise_id: string
  completion_criteria: string[]
  hints: string[]
  starter_code: string
}

export type SubmissionResponse = {
  attempt_id: string
  status: string
}

export type RoadmapUnitResponse = {
  id: string
  slug: string
  title: string
  prerequisite_unit_ids: string[]
  exercise_id: string
}

export type RoadmapModuleResponse = {
  id: string
  title: string
  units: RoadmapUnitResponse[]
}

export type RoadmapResponse = {
  id: string
  title: string
  modules: RoadmapModuleResponse[]
}

export type AttemptResponse = {
  attempt_id: string
  exercise_id: string
  status: string
  stdout: string
  stderr: string
  duration_ms: number | null
}

export type ProgressUnitResponse = {
  unit_id: string
  unit_slug: string
  unit_title: string
  status: string
  completed_attempts: number
  latest_attempt_id: string | null
}

export type ProgressOverviewResponse = {
  resume_unit_id: string
  resume_unit_slug: string
  resume_unit_title: string
  units: ProgressUnitResponse[]
}
