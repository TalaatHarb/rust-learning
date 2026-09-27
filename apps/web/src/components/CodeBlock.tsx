import type { ReactNode } from 'react'

type CodeBlockProps = {
  title: string
  children: ReactNode
}

export function CodeBlock({ title, children }: CodeBlockProps) {
  return (
    <section className="panel">
      <h3>{title}</h3>
      <pre className="code-block">
        <code>{children}</code>
      </pre>
    </section>
  )
}
