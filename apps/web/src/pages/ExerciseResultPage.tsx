import { useMemo } from 'react'
import { Link, useParams } from 'react-router-dom'
import { useAuth } from 'react-oidc-context'
import { useQuery } from '@tanstack/react-query'
import { fetchAttempt } from '../lib/api'

type ResultTone = 'success' | 'active' | 'warning' | 'error' | 'neutral'

function formatAttemptStatus(status: string) {
  switch (status) {
    case 'PASSED':
      return 'Passed'
    case 'RUNNING':
      return 'Running'
    case 'QUEUED':
      return 'Queued'
    case 'FAILED':
      return 'Failed'
    case 'TIMEOUT':
      return 'Timed out'
    case 'ERROR':
      return 'Error'
    default:
      return status
        .toLowerCase()
        .split('_')
        .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
        .join(' ')
  }
}

function statusTone(status: string): ResultTone {
  switch (status) {
    case 'PASSED':
      return 'success'
    case 'QUEUED':
    case 'RUNNING':
      return 'active'
    case 'FAILED':
    case 'TIMEOUT':
      return 'warning'
    case 'ERROR':
      return 'error'
    default:
      return 'neutral'
  }
}

export function ExerciseResultPage() {
  const { attemptId = 'latest' } = useParams()
  const auth = useAuth()

  const shouldFetch = auth.isAuthenticated && attemptId !== 'latest'

  const query = useQuery({
    queryKey: ['attempt', attemptId],
    queryFn: () => fetchAttempt(auth.user!.access_token, attemptId),
    enabled: shouldFetch,
    refetchInterval: (queryState) => {
      const data = queryState.state.data
      if (!data) return 1500
      return data.status === 'QUEUED' || data.status === 'RUNNING' ? 1500 : false
    },
  })

  const body = useMemo(() => {
    if (!auth.isAuthenticated) return 'Login to see execution results.'
    if (attemptId === 'latest') return 'Submit an exercise to see the latest attempt.'
    if (query.isLoading) return 'Loading attempt result...'
    if (query.isError || !query.data)
      return query.error instanceof Error ? query.error.message : 'Failed to load attempt.'

    return null
  }, [auth.isAuthenticated, attemptId, query.data, query.error, query.isError, query.isLoading])

  if (body) {
    return (
      <section className="panel result-empty-state">
        <h2>Exercise Result</h2>
        <p>{body}</p>
      </section>
    )
  }

  const attempt = query.data
  if (!attempt) {
    return null
  }

  const tone = statusTone(attempt.status)
  const formattedStatus = formatAttemptStatus(attempt.status)
  const hasStdout = attempt.stdout.trim().length > 0
  const hasStderr = attempt.stderr.trim().length > 0

  return (
    <div className="result-page">
      <section className="panel roadmap-hero result-hero">
        <div className="roadmap-hero__copy">
          <span className="roadmap-hero__eyebrow">Execution result</span>
          <h2>{formattedStatus} attempt</h2>
          <p>
            Review execution status, runtime details, and separated output streams without
            digging through one combined block.
          </p>
        </div>
        <div className="roadmap-hero__actions">
          <Link className="inline-link roadmap-hero__cta" to="/progress">
            View progress
          </Link>
          <Link className="inline-link roadmap-hero__cta" to="/roadmap">
            Open roadmap
          </Link>
        </div>
      </section>

      <section className="roadmap-summary" aria-label="Attempt summary">
        <article className={`panel roadmap-stat result-stat result-stat--${tone}`}>
          <span className="roadmap-stat__label">Status</span>
          <strong>{formattedStatus}</strong>
          <span>
            {attempt.status === 'RUNNING' || attempt.status === 'QUEUED'
              ? 'This page refreshes automatically while execution is in progress'
              : 'Execution completed and the final result is available'}
          </span>
        </article>
        <article className="panel roadmap-stat result-stat">
          <span className="roadmap-stat__label">Duration</span>
          <strong>{attempt.duration_ms ?? 0} ms</strong>
          <span>Captured execution time from the runner</span>
        </article>
        <article className="panel roadmap-stat result-stat">
          <span className="roadmap-stat__label">Attempt ID</span>
          <strong className="result-stat__attempt-id">{attempt.attempt_id.slice(0, 8)}</strong>
          <span className="result-stat__mono">{attempt.attempt_id}</span>
        </article>
      </section>

      <section className="dashboard-grid">
        <article className="panel dashboard-card dashboard-card--highlight">
          <div className="dashboard-section__header">
            <div>
              <span className="roadmap-stat__label">Exercise</span>
              <h3>{attempt.exercise_id}</h3>
            </div>
            <span className={`result-badge result-badge--${tone}`}>{formattedStatus}</span>
          </div>
          <p>
            Use the output panels below to inspect the command output and compiler or test
            failures separately.
          </p>
          <div className="dashboard-card__actions">
            <Link className="inline-link" to="/roadmap">
              Back to roadmap
            </Link>
            <Link className="inline-link" to="/progress">
              Back to progress
            </Link>
          </div>
        </article>

        <article className="panel dashboard-card">
          <div className="dashboard-section__header">
            <div>
              <span className="roadmap-stat__label">Stream summary</span>
              <h3>Output breakdown</h3>
            </div>
          </div>
          <div className="dashboard-list">
            <div className="dashboard-list__item">
              <div>
                <strong>Standard output</strong>
                <p>{hasStdout ? 'Captured program and test output is available below.' : 'No stdout was produced.'}</p>
              </div>
            </div>
            <div className="dashboard-list__item">
              <div>
                <strong>Standard error</strong>
                <p>{hasStderr ? 'Compiler errors, warnings, or test failures are separated below.' : 'No stderr was produced.'}</p>
              </div>
            </div>
          </div>
        </article>
      </section>

      <section className="result-sections">
        <article className="panel result-section">
          <div className="dashboard-section__header">
            <div>
              <span className="roadmap-stat__label">Output</span>
              <h3>Standard output</h3>
            </div>
          </div>
          <pre className="code-block result-terminal">
            <code>{hasStdout ? attempt.stdout : '(empty)'}</code>
          </pre>
        </article>

        <article className={`panel result-section ${hasStderr ? 'result-section--error' : ''}`}>
          <div className="dashboard-section__header">
            <div>
              <span className="roadmap-stat__label">Errors</span>
              <h3>Standard error</h3>
            </div>
          </div>
          <pre className="code-block result-terminal result-terminal--stderr">
            <code>{hasStderr ? attempt.stderr : '(empty)'}</code>
          </pre>
        </article>
      </section>
    </div>
  )
}
