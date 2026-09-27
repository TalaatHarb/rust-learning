const milestones = [
  'Variables',
  'Functions',
  'Control flow',
  'Ownership',
  'Borrowing',
  'References',
  'Slices',
]

export function RoadmapPage() {
  return (
    <section className="panel">
      <h2>Roadmap</h2>
      <p>First complete vertical slice focuses on the Ownership module.</p>
      <ul>
        {milestones.map((milestone) => (
          <li key={milestone}>{milestone}</li>
        ))}
      </ul>
    </section>
  )
}
