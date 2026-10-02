import type { ProgressUnitResponse } from '../types/api'

const completedStatuses = new Set(['PASSED', 'MASTERED'])
const startedStatuses = new Set(['STARTED', 'ATTEMPTED'])

export function isCompletedProgressStatus(status: string | undefined) {
  return status !== undefined && completedStatuses.has(status)
}

export function isStartedProgressStatus(status: string | undefined) {
  return status !== undefined && startedStatuses.has(status)
}

export function formatProgressStatus(status: string | undefined) {
  switch (status) {
    case 'MASTERED':
      return 'Mastered'
    case 'PASSED':
      return 'Completed'
    case 'STARTED':
    case 'ATTEMPTED':
      return 'In progress'
    case 'NOT_STARTED':
    case undefined:
      return 'Not started'
    default:
      return status
        .toLowerCase()
        .split('_')
        .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
        .join(' ')
  }
}

export function summarizeProgress(units: ProgressUnitResponse[]) {
  const completed = units.filter((unit) => isCompletedProgressStatus(unit.status)).length
  const started = units.filter((unit) => isStartedProgressStatus(unit.status)).length
  const total = units.length
  const remaining = Math.max(total - completed, 0)

  return {
    total,
    completed,
    started,
    remaining,
    completionPercent: total > 0 ? Math.round((completed / total) * 100) : 0,
  }
}
