import Editor from '@monaco-editor/react'

type CodeEditorProps = {
  code: string
  readOnly?: boolean
  onChange?: (value: string) => void
}

export function CodeEditor({ code, readOnly = false, onChange }: CodeEditorProps) {
  return (
    <section className="panel">
      <h3>{readOnly ? 'Example' : 'Exercise Editor'}</h3>
      <Editor
        defaultLanguage="rust"
        height="360px"
        value={code}
        onChange={(value) => onChange?.(value ?? '')}
        options={{
          minimap: { enabled: false },
          fontSize: 14,
          readOnly,
        }}
      />
    </section>
  )
}
