import { Link } from 'react-router-dom'
import { useAuth } from 'react-oidc-context'
import { useQuery } from '@tanstack/react-query'
import { fetchProgress } from '../lib/api'
import {
  formatProgressStatus,
  isCompletedProgressStatus,
  isStartedProgressStatus,
  summarizeProgress,
} from '../lib/progress'

export function ProgressPage() {
  const auth = useAuth()

  const progress = useQuery({
    queryKey: ['progress', 'overview'],
    queryFn: () => fetchProgress(auth.user!.access_token),
    enabled: auth.isAuthenticated && Boolean(auth.user),
  })

  if (!auth.isAuthenticated) {
    return (
      <div className="progress-page">
        <section className="panel roadmap-hero progress-hero">
          <div className="roadmap-hero__copy">
            <span className="roadmap-hero__eyebrow">Progress tracker</span>
            <h2>See your learning trail</h2>
            <p>
              Sign in to view completed lessons, active work, and the next unit waiting on your
              path.
            </p>
          </div>
          <div className="roadmap-hero__actions">
            <Link className="inline-link roadmap-hero__cta" to="/login">
              Log in to sync progress
            </Link>
          </div>
        </section>
      </div>
    )
  }

  if (progress.isLoading) {
    return (
      <section className="panel">
        <h2>User Progress</h2>
        <p>Loading progress...</p>
      </section>
    )
  }

  if (progress.isError || !progress.data) {
    return (
      <section className="panel">
        <h2>User Progress</h2>
        <p>{progress.error instanceof Error ? progress.error.message : 'Failed to load progress.'}</p>
      </section>
    )
  }

  const summary = summarizeProgress(progress.data.units)
  const nextUnits = progress.data.units
    .filter((unit) => !isCompletedProgressStatus(unit.status))
    .slice(0, 3)

  return (
    <div className="progress-page">
      <section className="panel roadmap-hero progress-hero">
        <div className="roadmap-hero__copy">
          <span className="roadmap-hero__eyebrow">Your progress</span>
          <h2>Track the path, not just the score</h2>
          <p>
            Follow your completed lessons, keep unfinished work visible, and jump straight to
            the best next step.
          </p>
        </div>
        <div className="roadmap-hero__actions">
          <Link className="inline-link roadmap-hero__cta" to={`/unit/${progress.data.resume_unit_slug}`}>
            Resume {progress.data.resume_unit_title}
          </Link>
        </div>
      </section>

      <section className="roadmap-summary" aria-label="Progress summary">
        <article className="panel roadmap-stat">
          <span className="roadmap-stat__label">Completed</span>
          <strong>{summary.completed}</strong>
          <span>{summary.completionPercent}% of all lessons complete</span>
        </article>
        <article className="panel roadmap-stat">
          <span className="roadmap-stat__label">Started</span>
          <strong>{summary.started}</strong>
          <span>Lessons with work already in motion</span>
        </article>
        <article className="panel roadmap-stat">
          <span className="roadmap-stat__label">Remaining</span>
          <strong>{summary.remaining}</strong>
          <span>Lessons still ahead in the current path</span>
        </article>
      </section>

      <section className="dashboard-grid">
        <article className="panel dashboard-card dashboard-card--highlight">
          <div className="dashboard-section__header">
            <div>
              <span className="roadmap-stat__label">Next checkpoint</span>
              <h3>{progress.data.resume_unit_title}</h3>
            </div>
            <Link className="inline-link" to={`/unit/${progress.data.resume_unit_slug}`}>
              Open lesson
            </Link>
          </div>
          <p>Keep your streak moving by tackling the next recommended lesson.</p>
          <div className="dashboard-progress">
            <div className="roadmap-module__bar" aria-hidden="true">
              <span style={{ width: `${summary.completionPercent}%` }} />
            </div>
            <span>{summary.completed} of {summary.total} lessons completed</span>
          </div>
        </article>

        <article className="panel dashboard-card">
          <div className="dashboard-section__header">
            <div>
              <span className="roadmap-stat__label">Coming up</span>
              <h3>Shortest path forward</h3>
            </div>
            <Link className="inline-link" to="/roadmap">
              Open roadmap
            </Link>
          </div>

          {nextUnits.length > 0 ? (
            <div className="dashboard-list">
              {nextUnits.map((unit) => (
                <div key={unit.unit_id} className="dashboard-list__item">
                  <div>
                    <strong>{unit.unit_title}</strong>
                    <p>{formatProgressStatus(unit.status)}</p>
                  </div>
                  <Link className="inline-link" to={`/unit/${unit.unit_slug}`}>
                    Continue
                  </Link>
                </div>
              ))}
            </div>
          ) : (
            <p className="dashboard-empty">Everything is complete. Use the roadmap to revisit any lesson.</p>
          )}
        </article>
      </section>

      <section className="panel progress-trail-panel">
        <div className="dashboard-section__header">
          <div>
            <span className="roadmap-stat__label">Lesson timeline</span>
            <h3>Every unit on your trail</h3>
          </div>
        </div>

        <ol className="progress-trail">
          {progress.data.units.map((unit, index) => {
            const stateClass = progress.data.resume_unit_id === unit.unit_id
              ? 'active'
              : isCompletedProgressStatus(unit.status)
              ? 'completed'
              : isStartedProgressStatus(unit.status)
                ? 'started'
                : 'pending'

            return (
              <li
                key={unit.unit_id}
                className={`progress-entry progress-entry--${stateClass}`}
              >
                <div className="progress-entry__marker">{index + 1}</div>
                <div className="progress-entry__card">
                  <div className="dashboard-section__header">
                    <div>
                      <span className="roadmap-node__status">{formatProgressStatus(unit.status)}</span>
                      <h4>{unit.unit_title}</h4>
                    </div>
                    <Link className="inline-link" to={`/unit/${unit.unit_slug}`}>
                      Open lesson
                    </Link>
                  </div>
                  <p>
                    Completed attempts: {unit.completed_attempts}
                    {progress.data.resume_unit_id === unit.unit_id ? ' · Recommended next lesson' : ''}
                  </p>
                  <div className="dashboard-card__actions">
                    <Link className="inline-link" to={`/exercise/${unit.unit_slug}`}>
                      Practice
                    </Link>
                    {unit.latest_attempt_id ? (
                      <Link className="inline-link" to={`/result/${unit.latest_attempt_id}`}>
                        View result
                      </Link>
                    ) : null}
                  </div>
                </div>
              </li>
            )
          })}
        </ol>
      </section>
    </div>
  )
}
