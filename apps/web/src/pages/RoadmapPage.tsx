import { Link } from 'react-router-dom'

const milestones = [
  { id: 'variables', label: 'Variables' },
  { id: 'functions', label: 'Functions' },
  { id: 'ownership', label: 'Ownership' },
  { id: 'control-flow', label: 'Control flow (planned)' },
  { id: 'borrowing', label: 'Borrowing (planned)' },
  { id: 'references', label: 'References (planned)' },
  { id: 'slices', label: 'Slices (planned)' },
]

export function RoadmapPage() {
  return (
    <section className="panel">
      <h2>Roadmap</h2>
      <p>Current implemented unit flow covers Variables, Functions, and Ownership.</p>
      <ul>
        {milestones.map((milestone) => (
          <li key={milestone.id}>
            {['variables', 'functions', 'ownership'].includes(milestone.id) ? (
              <Link className="inline-link" to={`/unit/${milestone.id}`}>
                {milestone.label}
              </Link>
            ) : (
              milestone.label
            )}
          </li>
        ))}
      </ul>
    </section>
  )
}
