import { useMemo } from 'react'
import Editor from '@monaco-editor/react'
import { useIsNarrowViewport } from '../lib/useIsNarrowViewport'
import { useTheme } from '../lib/useTheme'

type CodeBlockProps = {
  title: string
  children: string
}

export function CodeBlock({ title, children }: CodeBlockProps) {
  const isNarrowViewport = useIsNarrowViewport()
  const { resolvedTheme } = useTheme()
  const lineNumbers: 'off' | 'on' = isNarrowViewport ? 'off' : 'on'
  const options = useMemo(
    () => ({
      minimap: { enabled: false },
      fontSize: isNarrowViewport ? 13 : 14,
      readOnly: true,
      automaticLayout: true,
      scrollBeyondLastLine: false,
      lineNumbersMinChars: isNarrowViewport ? 2 : 3,
      lineNumbers,
      glyphMargin: false,
      folding: !isNarrowViewport,
      padding: { top: 12, bottom: 12 },
      wordWrap: 'on' as const,
      wrappingStrategy: 'advanced' as const,
      overviewRulerBorder: false,
      scrollbar: {
        verticalScrollbarSize: isNarrowViewport ? 8 : 10,
        horizontalScrollbarSize: isNarrowViewport ? 8 : 10,
      },
    }),
    [isNarrowViewport, lineNumbers],
  )

  return (
    <section className="panel">
      <h3>{title}</h3>
      <div className="code-editor-shell">
        <Editor
          defaultLanguage="rust"
          height={isNarrowViewport ? '180px' : '220px'}
          theme={resolvedTheme === 'dark' ? 'vs-dark' : 'vs'}
          value={children}
          options={options}
        />
      </div>
    </section>
  )
}
