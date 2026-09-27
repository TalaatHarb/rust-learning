import { Link } from 'react-router-dom'
import { useQuery } from '@tanstack/react-query'
import { CodeBlock } from '../components/CodeBlock'
import { CodeEditor } from '../components/CodeEditor'
import { fetchOwnershipUnit } from '../lib/api'

export function LearningUnitPage() {
  const { data, isLoading, isError, error } = useQuery({
    queryKey: ['unit', 'ownership'],
    queryFn: fetchOwnershipUnit,
  })

  if (isLoading) {
    return (
      <section className="panel">
        <h2>Rust Ownership</h2>
        <p>Loading learning unit...</p>
      </section>
    )
  }

  if (isError || !data) {
    return (
      <section className="panel">
        <h2>Rust Ownership</h2>
        <p>{error instanceof Error ? error.message : 'Failed to load learning unit.'}</p>
      </section>
    )
  }

  return (
    <>
      <section className="panel">
        <h2>{data.title}</h2>
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
        <Link className="inline-link" to="/exercise/ownership">
          Start Exercise
        </Link>
      </div>
    </>
  )
}
