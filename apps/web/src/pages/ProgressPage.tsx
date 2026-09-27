import { Link } from 'react-router-dom'
import { useAuth } from 'react-oidc-context'
import { useQuery } from '@tanstack/react-query'
import { fetchProgress } from '../lib/api'

export function ProgressPage() {
  const auth = useAuth()

  const progress = useQuery({
    queryKey: ['progress', 'overview'],
    queryFn: () => fetchProgress(auth.user!.access_token),
    enabled: auth.isAuthenticated,
  })

  if (!auth.isAuthenticated) {
    return (
      <section className="panel">
        <h2>User Progress</h2>
        <p>Login to view your learning progress.</p>
      </section>
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

  return (
    <section className="panel">
      <h2>User Progress</h2>
      <p>
        Resume next with{' '}
        <Link className="inline-link" to={`/unit/${progress.data.resume_unit_slug}`}>
          {progress.data.resume_unit_title}
        </Link>
        .
      </p>
      <ul>
        {progress.data.units.map((unit) => (
          <li key={unit.unit_id}>
            <Link className="inline-link" to={`/unit/${unit.unit_slug}`}>
              {unit.unit_title}
            </Link>{' '}
            — {unit.status} — completed attempts: {unit.completed_attempts} — latest attempt:{' '}
            {unit.latest_attempt_id ?? 'N/A'}
          </li>
        ))}
      </ul>
    </section>
  )
}
