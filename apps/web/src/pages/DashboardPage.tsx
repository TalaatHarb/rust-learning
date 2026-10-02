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

export function DashboardPage() {
  const auth = useAuth()

  const progress = useQuery({
    queryKey: ['progress', 'overview'],
    queryFn: () => fetchProgress(auth.user!.access_token),
    enabled: auth.isAuthenticated && Boolean(auth.user),
  })

  if (!auth.isAuthenticated) {
    return (
      <div className="dashboard-page">
        <section className="panel roadmap-hero dashboard-hero">
          <div className="roadmap-hero__copy">
            <span className="roadmap-hero__eyebrow">Rust learning cockpit</span>
            <h2>Build momentum with a guided learning path</h2>
            <p>
              Track completed lessons, jump back into unfinished work, and follow a
              progress-first dashboard designed around the next best step.
            </p>
          </div>
          <div className="roadmap-hero__actions dashboard-hero__actions">
            <Link className="inline-link roadmap-hero__cta" to="/roadmap">
              Explore roadmap
            </Link>
            <Link className="inline-link roadmap-hero__cta" to="/login">
              Log in to save progress
            </Link>
          </div>
        </section>

        <section className="roadmap-summary" aria-label="Dashboard preview">
          <article className="panel roadmap-stat">
            <span className="roadmap-stat__label">Guided path</span>
            <strong>1</strong>
            <span>Foundations roadmap ready to start</span>
          </article>
          <article className="panel roadmap-stat">
            <span className="roadmap-stat__label">Progress sync</span>
            <strong>Live</strong>
            <span>Your completed and active lessons appear here after login</span>
          </article>
          <article className="panel roadmap-stat">
            <span className="roadmap-stat__label">Next step</span>
            <strong>Resume</strong>
            <span>Come back to exactly where you left off</span>
          </article>
        </section>
      </div>
    )
  }

  if (progress.isLoading) {
    return (
      <section className="panel">
        <h2>Dashboard</h2>
        <p>Loading progress...</p>
      </section>
    )
  }

  if (progress.isError || !progress.data) {
    return (
      <section className="panel panel--error">
        <h2>Dashboard</h2>
        <p>{progress.error instanceof Error ? progress.error.message : 'Failed to load progress.'}</p>
      </section>
    )
  }

  const summary = summarizeProgress(progress.data.units)
  const activeUnits = progress.data.units.filter((unit) => isStartedProgressStatus(unit.status)).slice(0, 3)
  const completedUnits = progress.data.units.filter((unit) => isCompletedProgressStatus(unit.status)).slice(0, 3)
  const latestResultUnit = progress.data.units.find((unit) => unit.latest_attempt_id)

  return (
    <div className="dashboard-page">
      <section className="panel roadmap-hero dashboard-hero">
        <div className="roadmap-hero__copy">
          <span className="roadmap-hero__eyebrow">Your learning dashboard</span>
          <h2>Keep moving through the roadmap</h2>
          <p>
            Resume your next lesson, monitor overall completion, and revisit the work that is
            already underway.
          </p>
        </div>
        <div className="roadmap-hero__actions dashboard-hero__actions">
          <Link className="inline-link roadmap-hero__cta" to={`/unit/${progress.data.resume_unit_slug}`}>
            Resume {progress.data.resume_unit_title}
          </Link>
          <Link className="inline-link roadmap-hero__cta" to="/roadmap">
            View roadmap
          </Link>
        </div>
      </section>

      <section className="roadmap-summary" aria-label="Dashboard summary">
        <article className="panel roadmap-stat">
          <span className="roadmap-stat__label">Completed</span>
          <strong>{summary.completed}</strong>
          <span>{summary.completionPercent}% of the learning path done</span>
        </article>
        <article className="panel roadmap-stat">
          <span className="roadmap-stat__label">In progress</span>
          <strong>{summary.started}</strong>
          <span>Lessons with active work or recent attempts</span>
        </article>
        <article className="panel roadmap-stat">
          <span className="roadmap-stat__label">Remaining</span>
          <strong>{summary.remaining}</strong>
          <span>{summary.total} lessons in the current path</span>
        </article>
      </section>

      <section className="dashboard-grid">
        <article className="panel dashboard-card dashboard-card--highlight">
          <span className="roadmap-stat__label">Resume next</span>
          <h3>{progress.data.resume_unit_title}</h3>
          <p>Your recommended next lesson is ready to continue.</p>
          <div className="dashboard-card__actions">
            <Link className="inline-link" to={`/unit/${progress.data.resume_unit_slug}`}>
              Open lesson
            </Link>
            <Link className="inline-link" to={`/exercise/${progress.data.resume_unit_slug}`}>
              Open exercise
            </Link>
          </div>
          <div className="dashboard-progress">
            <div className="roadmap-module__bar" aria-hidden="true">
              <span style={{ width: `${summary.completionPercent}%` }} />
            </div>
            <span>{summary.completionPercent}% overall completion</span>
          </div>
        </article>

        <article className="panel dashboard-card">
          <span className="roadmap-stat__label">Result shortcut</span>
          <h3>{latestResultUnit?.unit_title ?? 'No attempts yet'}</h3>
          <p>
            {latestResultUnit
              ? `${formatProgressStatus(latestResultUnit.status)} and ready to review or retry.`
              : 'Your latest attempt will appear here once you submit a lesson exercise.'}
          </p>
          <div className="dashboard-card__actions">
            {latestResultUnit?.latest_attempt_id ? (
              <Link className="inline-link" to={`/result/${latestResultUnit.latest_attempt_id}`}>
                View latest result
              </Link>
            ) : (
              <Link className="inline-link" to="/roadmap">
                Start the roadmap
              </Link>
            )}
          </div>
        </article>
      </section>

      <section className="dashboard-grid">
        <article className="panel dashboard-card">
          <div className="dashboard-section__header">
            <div>
              <span className="roadmap-stat__label">Active lessons</span>
              <h3>Continue what you started</h3>
            </div>
            <Link className="inline-link" to="/progress">
              View progress
            </Link>
          </div>

          {activeUnits.length > 0 ? (
            <div className="dashboard-list">
              {activeUnits.map((unit) => (
                <div key={unit.unit_id} className="dashboard-list__item">
                  <div>
                    <strong>{unit.unit_title}</strong>
                    <p>{formatProgressStatus(unit.status)}</p>
                  </div>
                  <div className="dashboard-card__actions">
                    <Link className="inline-link" to={`/unit/${unit.unit_slug}`}>
                      Resume lesson
                    </Link>
                    {unit.latest_attempt_id ? (
                      <Link className="inline-link" to={`/result/${unit.latest_attempt_id}`}>
                        Result
                      </Link>
                    ) : null}
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <p className="dashboard-empty">No active lessons yet — start one from the roadmap.</p>
          )}
        </article>

        <article className="panel dashboard-card">
          <div className="dashboard-section__header">
            <div>
              <span className="roadmap-stat__label">Completed lessons</span>
              <h3>Recent wins</h3>
            </div>
            <Link className="inline-link" to="/roadmap">
              Explore more
            </Link>
          </div>

          {completedUnits.length > 0 ? (
            <div className="dashboard-list">
              {completedUnits.map((unit) => (
                <div key={unit.unit_id} className="dashboard-list__item">
                  <div>
                    <strong>{unit.unit_title}</strong>
                    <p>
                      {formatProgressStatus(unit.status)} · {unit.completed_attempts} successful
                      submission{unit.completed_attempts === 1 ? '' : 's'}
                    </p>
                  </div>
                  <Link className="inline-link" to={`/unit/${unit.unit_slug}`}>
                    Review lesson
                  </Link>
                </div>
              ))}
            </div>
          ) : (
            <p className="dashboard-empty">Completed lessons will appear here as you finish units.</p>
          )}
        </article>
      </section>

      <section className="panel dashboard-card">
        <div className="dashboard-section__header">
          <div>
            <span className="roadmap-stat__label">Coming up</span>
            <h3>Shortest path forward</h3>
          </div>
          <Link className="inline-link" to="/progress">
            Full timeline
          </Link>
        </div>

        {progress.data.units
          .filter((unit) => !isCompletedProgressStatus(unit.status))
          .slice(0, 3).length > 0 ? (
          <div className="dashboard-list">
            {progress.data.units
              .filter((unit) => !isCompletedProgressStatus(unit.status))
              .slice(0, 3)
              .map((unit) => (
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
          <p className="dashboard-empty">You have completed the full path. Revisit any lesson from the roadmap.</p>
        )}
      </section>
    </div>
  )
}
