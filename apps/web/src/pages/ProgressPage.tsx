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
      <p>Current unit: {progress.data.unit_id}</p>
      <p>Status: {progress.data.status}</p>
      <p>Completed attempts: {progress.data.completed_attempts}</p>
      <p>Latest attempt: {progress.data.latest_attempt_id ?? 'N/A'}</p>
    </section>
  )
}
