import { useEffect, useState } from 'react'
import { Link, useNavigate, useParams } from 'react-router-dom'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useAuth } from 'react-oidc-context'
import { CodeEditor } from '../components/CodeEditor'
import { fetchUnit, submitExercise } from '../lib/api'

export function ExercisePage() {
  const auth = useAuth()
  const navigate = useNavigate()
  const { unitId = 'ownership' } = useParams()
  const [submissionError, setSubmissionError] = useState<string | null>(null)

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ['unit', unitId],
    queryFn: () => fetchUnit(unitId),
  })

  const [code, setCode] = useState('')

  const submission = useMutation({
    mutationFn: async () => {
      if (!auth.user?.access_token) {
        throw new Error('Please login before submitting code.')
      }
      if (!data) {
        throw new Error('Learning unit not loaded.')
      }

      return submitExercise(auth.user.access_token, {
        exercise_id: data.exercise_id,
        code,
      })
    },
    onSuccess: (response) => {
      navigate(`/result/${response.attempt_id}`)
    },
    onError: (mutationError) => {
      setSubmissionError(
        mutationError instanceof Error ? mutationError.message : 'Submission failed',
      )
    },
  })

  useEffect(() => {
    if (data?.starter_code && code.length === 0) {
      setCode(data.starter_code)
    }
  }, [code.length, data?.starter_code])

  if (isLoading) {
    return (
      <section className="panel">
        <h2>Exercise</h2>
        <p>Loading exercise...</p>
      </section>
    )
  }

  if (isError || !data) {
    return (
      <section className="panel">
        <h2>Exercise</h2>
        <p>{error instanceof Error ? error.message : 'Failed to load exercise.'}</p>
      </section>
    )
  }

  return (
    <div className="exercise-page">
      <section className="panel">
        <h2>Exercise</h2>
        <p>Implement the exercise and run tests.</p>
        <h3>Hints</h3>
        <ul>
          {data.hints.map((hint) => (
            <li key={hint}>{hint}</li>
          ))}
        </ul>
      </section>
      <CodeEditor code={code} onChange={setCode} />
      <div className="actions">
        <button type="button" onClick={() => submission.mutate()} disabled={submission.isPending}>
          {submission.isPending ? 'Submitting…' : 'Run tests'}
        </button>
        <Link className="inline-link" to="/result/latest">
          View latest result
        </Link>
      </div>
      {submissionError && (
        <section className="panel panel--error">
          <p>{submissionError}</p>
        </section>
      )}
    </div>
  )
}
