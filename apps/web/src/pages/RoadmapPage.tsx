const milestones = [
  'Variables and functions',
  'Ownership and borrowing',
  'Async Rust and Tokio',
  'Axum + SQLx projects',
]

export function RoadmapPage() {
  return (
    <section className="panel">
      <h2>Roadmap</h2>
      <ul>
        {milestones.map((milestone) => (
          <li key={milestone}>{milestone}</li>
        ))}
      </ul>
    </section>
  )
}
