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

export type AttemptResponse = {
  attempt_id: string
  exercise_id: string
  status: string
  stdout: string
  stderr: string
  duration_ms: number | null
}

export type ProgressOverviewResponse = {
  unit_id: string
  status: string
  completed_attempts: number
  latest_attempt_id: string | null
}
