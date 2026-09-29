import { Link } from 'react-router-dom'
import { useAuth } from 'react-oidc-context'
import { useQuery } from '@tanstack/react-query'
import { fetchProgress } from '../lib/api'

export function DashboardPage() {
  const auth = useAuth()

  const progress = useQuery({
    queryKey: ['progress', 'overview'],
    queryFn: () => fetchProgress(auth.user!.access_token),
    enabled: auth.isAuthenticated,
  })

  const completedUnits =
    progress.data?.units.filter((unit) => unit.status === 'PASSED' || unit.status === 'MASTERED').length ??
    0
  const totalUnits = progress.data?.units.length ?? 0

  return (
    <section className="panel">
      <h2>Dashboard</h2>
      {!auth.isAuthenticated && (
        <p>Sign in to track your module progress and resume learning.</p>
      )}
      {auth.isAuthenticated && progress.isLoading && <p>Loading progress...</p>}
      {auth.isAuthenticated && progress.data && (
        <>
          <p>
            Resume unit:{' '}
            <Link className="inline-link" to={`/unit/${progress.data.resume_unit_slug}`}>
              {progress.data.resume_unit_title}
            </Link>
          </p>
          <p>
            Completed units: {completedUnits} / {totalUnits}
          </p>
          <ul>
            {progress.data.units.map((unit) => (
              <li key={unit.unit_id}>
                {unit.unit_title}: {unit.status}
                {unit.latest_attempt_id && (
                  <>
                    {' — '}
                    <Link className="inline-link" to={`/result/${unit.latest_attempt_id}`}>
                      View latest result
                    </Link>
                  </>
                )}
              </li>
            ))}
          </ul>
        </>
      )}
      {auth.isAuthenticated && progress.isError && (
        <p>{progress.error instanceof Error ? progress.error.message : 'Failed to load progress.'}</p>
      )}
    </section>
  )
}
