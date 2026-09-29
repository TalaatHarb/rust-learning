import { Link, useParams } from 'react-router-dom'
import { useAuth } from 'react-oidc-context'
import { useQuery } from '@tanstack/react-query'
import { CodeBlock } from '../components/CodeBlock'
import { CodeEditor } from '../components/CodeEditor'
import { fetchProgress, fetchUnit } from '../lib/api'

export function LearningUnitPage() {
  const { unitId = 'ownership' } = useParams()
  const auth = useAuth()

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ['unit', unitId],
    queryFn: () => fetchUnit(unitId),
  })
  const progress = useQuery({
    queryKey: ['progress', 'overview'],
    queryFn: () => fetchProgress(auth.user!.access_token),
    enabled: auth.isAuthenticated,
  })
  const latestAttemptId = progress.data?.units.find((unit) => unit.unit_slug === unitId)?.latest_attempt_id

  if (isLoading) {
    return (
      <section className="panel">
        <h2>Learning Unit</h2>
        <p>Loading learning unit...</p>
      </section>
    )
  }

  if (isError || !data) {
    return (
      <section className="panel">
        <h2>Learning Unit</h2>
        <p>{error instanceof Error ? error.message : 'Failed to load learning unit.'}</p>
      </section>
    )
  }

  return (
    <>
      <section className="panel">
        <h2>{data.title}</h2>
        {latestAttemptId && (
          <p>
            <Link className="inline-link" to={`/result/${latestAttemptId}`}>
              View latest exercise result
            </Link>
          </p>
        )}
        <p>{data.explanation}</p>
        <h3>Learning objectives</h3>
        <ul>
          {data.learning_objectives.map((objective) => (
            <li key={objective}>{objective}</li>
          ))}
        </ul>
        <h3>Completion criteria</h3>
        <ul>
          {data.completion_criteria.map((criteria) => (
            <li key={criteria}>{criteria}</li>
          ))}
        </ul>
      </section>
      {data.examples.map((example) => (
        <CodeBlock key={example.id} title={example.title}>
          {example.code}
        </CodeBlock>
      ))}
      <CodeEditor code={data.starter_code} readOnly />
      <div className="actions">
        <Link className="inline-link" to={`/exercise/${unitId}`}>
          Start Exercise
        </Link>
      </div>
    </>
  )
}
