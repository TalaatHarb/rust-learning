import { CodeBlock } from '../components/CodeBlock'
import { CodeEditor } from '../components/CodeEditor'

const ownershipExample = `fn main() {
    let text = String::from("hello");
    consume(text);
    // println!("{text}"); // error: value moved
}

fn consume(data: String) {
    println!("{data}");
}`

export function LearningUnitPage() {
  return (
    <>
      <section className="panel">
        <h2>Rust Ownership</h2>
        <p>Understand move semantics, borrowing, and how Rust ensures memory safety.</p>
      </section>
      <CodeBlock title="Ownership example">{ownershipExample}</CodeBlock>
      <CodeEditor code={ownershipExample} readOnly />
    </>
  )
}
