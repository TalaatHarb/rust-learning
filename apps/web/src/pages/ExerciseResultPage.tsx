import { useMemo } from 'react'
import { useParams } from 'react-router-dom'
import { useAuth } from 'react-oidc-context'
import { useQuery } from '@tanstack/react-query'
import { fetchAttempt } from '../lib/api'

export function ExerciseResultPage() {
  const { attemptId = 'latest' } = useParams()
  const auth = useAuth()

  const shouldFetch = auth.isAuthenticated && attemptId !== 'latest'

  const query = useQuery({
    queryKey: ['attempt', attemptId],
    queryFn: () => fetchAttempt(auth.user!.access_token, attemptId),
    enabled: shouldFetch,
    refetchInterval: (data) => {
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

  return (
    <section className="panel">
      <h2>Exercise Result</h2>
      {body ? (
        <p>{body}</p>
      ) : (
        <>
          <p>Attempt: {query.data?.attempt_id}</p>
          <p>Status: {query.data?.status}</p>
          <p>Duration: {query.data?.duration_ms ?? 0} ms</p>
          <h3>Stdout</h3>
          <pre className="code-block">
            <code>{query.data?.stdout || '(empty)'}</code>
          </pre>
          <h3>Stderr</h3>
          <pre className="code-block">
            <code>{query.data?.stderr || '(empty)'}</code>
          </pre>
        </>
      )}
    </section>
  )
}
