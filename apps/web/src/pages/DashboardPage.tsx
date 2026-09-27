import { useAuth } from 'react-oidc-context'
import { useQuery } from '@tanstack/react-query'
import { fetchProgress } from '../lib/api'

export function DashboardPage() {
  const auth = useAuth()

  const progress = useQuery({
    queryKey: ['progress', 'dashboard'],
    queryFn: () => fetchProgress(auth.user!.access_token),
    enabled: auth.isAuthenticated,
  })

  return (
    <section className="panel">
      <h2>Dashboard</h2>
      {!auth.isAuthenticated && (
        <p>Sign in to track your module progress and resume learning.</p>
      )}
      {auth.isAuthenticated && progress.isLoading && <p>Loading progress...</p>}
      {auth.isAuthenticated && progress.data && (
        <>
          <p>Resume unit: {progress.data.unit_id}</p>
          <p>Status: {progress.data.status}</p>
          <p>Latest attempt: {progress.data.latest_attempt_id ?? 'No attempts yet'}</p>
        </>
      )}
      {auth.isAuthenticated && progress.isError && (
        <p>{progress.error instanceof Error ? progress.error.message : 'Failed to load progress.'}</p>
      )}
    </section>
  )
}
