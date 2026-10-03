import { useMemo } from 'react'
import Editor from '@monaco-editor/react'
import { useIsNarrowViewport } from '../lib/useIsNarrowViewport'
import { useTheme } from '../lib/useTheme'

type CodeEditorProps = {
  code: string
  readOnly?: boolean
  onChange?: (value: string) => void
}

export function CodeEditor({ code, readOnly = false, onChange }: CodeEditorProps) {
  const isNarrowViewport = useIsNarrowViewport()
  const { resolvedTheme } = useTheme()
  const lineNumbers: 'off' | 'on' = isNarrowViewport ? 'off' : 'on'
  const wordWrap: 'off' | 'on' = isNarrowViewport ? 'on' : 'off'
  const height = readOnly
    ? isNarrowViewport
      ? '240px'
      : '360px'
    : isNarrowViewport
      ? '320px'
      : '420px'
  const options = useMemo(
    () => ({
      minimap: { enabled: false },
      fontSize: isNarrowViewport ? 13 : 14,
      readOnly,
      automaticLayout: true,
      scrollBeyondLastLine: false,
      lineNumbersMinChars: isNarrowViewport ? 2 : 3,
      lineNumbers,
      glyphMargin: false,
      folding: !isNarrowViewport,
      padding: { top: 12, bottom: 12 },
      wordWrap,
      wrappingStrategy: 'advanced' as const,
      overviewRulerBorder: false,
      scrollbar: {
        verticalScrollbarSize: isNarrowViewport ? 8 : 10,
        horizontalScrollbarSize: isNarrowViewport ? 8 : 10,
      },
    }),
    [isNarrowViewport, lineNumbers, readOnly, wordWrap],
  )

  return (
    <section className="panel">
      <h3>{readOnly ? 'Example' : 'Exercise Editor'}</h3>
      <div className="code-editor-shell">
        <Editor
          defaultLanguage="rust"
          height={height}
          theme={resolvedTheme === 'dark' ? 'vs-dark' : 'vs'}
          value={code}
          onChange={(value) => onChange?.(value ?? '')}
          options={options}
        />
      </div>
    </section>
  )
}
