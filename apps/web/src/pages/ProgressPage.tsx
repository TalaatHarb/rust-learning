const progressStates = ['NOT_STARTED', 'STARTED', 'ATTEMPTED', 'PASSED', 'MASTERED']

export function ProgressPage() {
  return (
    <section className="panel">
      <h2>User Progress</h2>
      <ol>
        {progressStates.map((state) => (
          <li key={state}>{state}</li>
        ))}
      </ol>
    </section>
  )
}
