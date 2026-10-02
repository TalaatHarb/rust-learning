import { useQuery } from '@tanstack/react-query'
import { Link } from 'react-router-dom'
import { useAuth } from 'react-oidc-context'
import { fetchProgress, fetchRoadmap } from '../lib/api'
import {
  isCompletedProgressStatus,
  isStartedProgressStatus,
  summarizeProgress,
} from '../lib/progress'
import type {
  ProgressOverviewResponse,
  ProgressUnitResponse,
  RoadmapModuleResponse,
  RoadmapUnitResponse,
} from '../types/api'

type UnitVisualState = 'completed' | 'active' | 'started' | 'available' | 'locked' | 'preview'

type DecoratedUnit = {
  unit: RoadmapUnitResponse
  progress: ProgressUnitResponse | undefined
  visualState: UnitVisualState
  statusLabel: string
  description: string
}

function statusSummary(
  unit: RoadmapUnitResponse,
  progress: ProgressUnitResponse | undefined,
  progressOverview: ProgressOverviewResponse | undefined,
  progressByUnitId: Map<string, ProgressUnitResponse>,
  isAuthenticated: boolean,
): Pick<DecoratedUnit, 'visualState' | 'statusLabel' | 'description'> {
  if (!isAuthenticated) {
    return {
      visualState: 'preview',
      statusLabel: 'Preview',
      description: 'Log in to track progress and unlock a personalized path.',
    }
  }

  if (isCompletedProgressStatus(progress?.status)) {
    return {
      visualState: 'completed',
      statusLabel: 'Completed',
      description:
        progress && progress.completed_attempts > 1
          ? `${progress.completed_attempts} successful submissions.`
          : 'Lesson complete.',
    }
  }

  if (progressOverview?.resume_unit_id === unit.id) {
    return {
      visualState: 'active',
      statusLabel: 'Up next',
      description: 'Recommended next lesson based on your current progress.',
    }
  }

  if (isStartedProgressStatus(progress?.status)) {
    return {
      visualState: 'started',
      statusLabel: 'In progress',
      description: 'You started this lesson. Jump back in and finish the exercise.',
    }
  }

  const unlocked = unit.prerequisite_unit_ids.every((prerequisiteId) =>
    isCompletedProgressStatus(progressByUnitId.get(prerequisiteId)?.status),
  )

  if (!unlocked) {
    return {
      visualState: 'locked',
      statusLabel: 'Locked',
      description:
        unit.prerequisite_unit_ids.length === 1
          ? 'Finish the prerequisite lesson to unlock this node.'
          : `Finish ${unit.prerequisite_unit_ids.length} prerequisite lessons to unlock this node.`,
    }
  }

  return {
    visualState: 'available',
    statusLabel: 'Ready',
    description: 'Available to start now.',
  }
}

function decorateModule(
  module: RoadmapModuleResponse,
  progressOverview: ProgressOverviewResponse | undefined,
  isAuthenticated: boolean,
) {
  const progressByUnitId = new Map(progressOverview?.units.map((unit) => [unit.unit_id, unit]) ?? [])

  return module.units.map((unit) => {
    const progress = progressByUnitId.get(unit.id)
    const summary = statusSummary(
      unit,
      progress,
      progressOverview,
      progressByUnitId,
      isAuthenticated,
    )

    return {
      unit,
      progress,
      ...summary,
    } satisfies DecoratedUnit
  })
}

function moduleCompletion(units: DecoratedUnit[]) {
  if (units.length === 0) {
    return 0
  }

  const completed = units.filter((unit) => unit.visualState === 'completed').length
  return Math.round((completed / units.length) * 100)
}

function roadmapSummary(
  modules: RoadmapModuleResponse[],
  progressOverview: ProgressOverviewResponse | undefined,
  isAuthenticated: boolean,
) {
  const totalUnits = modules.reduce((count, module) => count + module.units.length, 0)
  const progressSummary = summarizeProgress(progressOverview?.units ?? [])

  return {
    totalUnits,
    completedUnits: progressSummary.completed,
    startedUnits: progressSummary.started,
    completionPercent:
      isAuthenticated && totalUnits > 0
        ? Math.round((progressSummary.completed / totalUnits) * 100)
        : 0,
  }
}

export function RoadmapPage() {
  const auth = useAuth()

  const roadmap = useQuery({
    queryKey: ['roadmap', 'foundations'],
    queryFn: fetchRoadmap,
  })

  const progress = useQuery({
    queryKey: ['progress', 'overview'],
    queryFn: () => fetchProgress(auth.user!.access_token),
    enabled: auth.isAuthenticated && Boolean(auth.user),
  })

  if (roadmap.isLoading) {
    return (
      <section className="panel">
        <h2>Roadmap</h2>
        <p>Loading roadmap...</p>
      </section>
    )
  }

  if (roadmap.isError || !roadmap.data) {
    return (
      <section className="panel">
        <h2>Roadmap</h2>
        <p>{roadmap.error instanceof Error ? roadmap.error.message : 'Failed to load roadmap.'}</p>
      </section>
    )
  }

  const summary = roadmapSummary(roadmap.data.modules, progress.data, auth.isAuthenticated)

  return (
    <div className="roadmap-page">
      <section className="panel roadmap-hero">
        <div className="roadmap-hero__copy">
          <span className="roadmap-hero__eyebrow">Foundations learning path</span>
          <h2>{roadmap.data.title}</h2>
          <p>
            Follow the graph node-by-node, revisit started lessons, and keep momentum with a
            clear next target.
          </p>
        </div>
        <div className="roadmap-hero__actions">
          {auth.isAuthenticated && progress.data ? (
            <Link className="inline-link roadmap-hero__cta" to={`/unit/${progress.data.resume_unit_slug}`}>
              Resume {progress.data.resume_unit_title}
            </Link>
          ) : (
            <Link className="inline-link roadmap-hero__cta" to="/login">
              Log in for tracked progress
            </Link>
          )}
        </div>
      </section>

      <section className="roadmap-summary" aria-label="Roadmap summary">
        <article className="panel roadmap-stat">
          <span className="roadmap-stat__label">Modules</span>
          <strong>{roadmap.data.modules.length}</strong>
          <span>{summary.totalUnits} total lessons</span>
        </article>
        <article className="panel roadmap-stat">
          <span className="roadmap-stat__label">Completed</span>
          <strong>{auth.isAuthenticated ? summary.completedUnits : '--'}</strong>
          <span>
            {auth.isAuthenticated ? `${summary.completionPercent}% of the path complete` : 'Login to track wins'}
          </span>
        </article>
        <article className="panel roadmap-stat">
          <span className="roadmap-stat__label">In progress</span>
          <strong>{auth.isAuthenticated ? summary.startedUnits : '--'}</strong>
          <span>
            {auth.isAuthenticated
              ? 'Lessons you have already started'
              : 'Started lessons appear here after you sign in'}
          </span>
        </article>
      </section>

      {auth.isAuthenticated && progress.isLoading ? (
        <section className="panel">
          <p>Loading your progress map...</p>
        </section>
      ) : null}

      {auth.isAuthenticated && progress.isError ? (
        <section className="panel panel--error">
          <h3>Progress unavailable</h3>
          <p>
            {progress.error instanceof Error
              ? progress.error.message
              : 'Failed to load your roadmap progress.'}
          </p>
          <p>The roadmap structure is still available below.</p>
        </section>
      ) : null}

      {roadmap.data.modules.map((module) => {
        const units = decorateModule(module, progress.data, auth.isAuthenticated)
        const completionPercent = moduleCompletion(units)

        return (
          <section key={module.id} className="panel roadmap-module">
            <div className="roadmap-module__header">
              <div>
                <span className="roadmap-module__eyebrow">Module</span>
                <h3>{module.title}</h3>
              </div>
              <div className="roadmap-module__progress">
                <strong>{auth.isAuthenticated ? `${completionPercent}% complete` : 'Preview mode'}</strong>
                <div className="roadmap-module__bar" aria-hidden="true">
                  <span style={{ width: `${completionPercent}%` }} />
                </div>
              </div>
            </div>

            <ol className="roadmap-graph">
              {units.map((entry, index) => (
                <li
                  key={entry.unit.id}
                  className={`roadmap-node roadmap-node--${entry.visualState}`}
                  aria-label={`${entry.unit.title}: ${entry.statusLabel}`}
                >
                  <Link className="roadmap-node__link" to={`/unit/${entry.unit.slug}`}>
                    <span className="roadmap-node__circle">{index + 1}</span>
                    <span className="roadmap-node__card">
                      <span className="roadmap-node__status">{entry.statusLabel}</span>
                      <span className="roadmap-node__title">{entry.unit.title}</span>
                      <span className="roadmap-node__description">{entry.description}</span>
                      <span className="roadmap-node__meta">
                        {entry.unit.prerequisite_unit_ids.length > 0
                          ? `${entry.unit.prerequisite_unit_ids.length} prerequisite${entry.unit.prerequisite_unit_ids.length === 1 ? '' : 's'}`
                          : 'No prerequisites'}
                        {entry.progress?.latest_attempt_id ? ' · Result available' : ''}
                      </span>
                    </span>
                  </Link>
                  {entry.progress?.latest_attempt_id ? (
                    <Link className="roadmap-node__result" to={`/result/${entry.progress.latest_attempt_id}`}>
                      View latest result
                    </Link>
                  ) : null}
                </li>
              ))}
            </ol>
          </section>
        )
      })}
    </div>
  )
}
