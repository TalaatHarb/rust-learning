import { useState } from 'react'
import { CodeEditor } from '../components/CodeEditor'

const starterCode = `fn main() {
    let message = String::from("hello");
    // TODO: borrow message and print it twice.
}`

export function ExercisePage() {
  const [code, setCode] = useState(starterCode)

  return (
    <>
      <section className="panel">
        <h2>Exercise</h2>
        <p>Update the code so ownership is preserved and tests can pass.</p>
      </section>
      <CodeEditor code={code} onChange={setCode} />
      <button type="button">Run tests</button>
    </>
  )
}
