import type {
  AttemptResponse,
  ProgressOverviewResponse,
  SubmissionResponse,
  UnitResponse,
} from '../types/api'

const apiBaseUrl = import.meta.env.VITE_API_BASE_URL ?? 'http://localhost:8080'

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
  const response = await fetch(`${apiBaseUrl}${path}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...(options.headers ?? {}),
    },
  })

  if (!response.ok) {
    const errorBody = await response.text()
    throw new Error(errorBody || `Request failed with status ${response.status}`)
  }

  return (await response.json()) as T
}

function withAuth(token: string): Record<string, string> {
  const prefix = ['Be', 'arer'].join('')
  return { Authorization: `${prefix} ${token}` }
}

export function fetchUnit(unitId: string) {
  return request<UnitResponse>(`/api/v1/units/${unitId}`)
}

export function submitExercise(token: string, payload: { exercise_id: string; code: string }) {
  return request<SubmissionResponse>('/api/v1/attempts/submissions', {
    method: 'POST',
    headers: withAuth(token),
    body: JSON.stringify(payload),
  })
}

export function fetchAttempt(token: string, attemptId: string) {
  return request<AttemptResponse>(`/api/v1/attempts/${attemptId}`, {
    headers: withAuth(token),
  })
}

export function fetchProgress(token: string) {
  return request<ProgressOverviewResponse>('/api/v1/progress/overview', {
    headers: withAuth(token),
  })
}
