import type { ChangeEvent } from 'react'
import { useTheme } from '../lib/useTheme'

function parseThemePreference(value: string) {
  return value === 'light' || value === 'dark' || value === 'system' ? value : null
}

export function ThemeSelector() {
  const { themePreference, setThemePreference } = useTheme()

  const handleChange = (event: ChangeEvent<HTMLSelectElement>) => {
    const nextThemePreference = parseThemePreference(event.currentTarget.value)
    if (nextThemePreference) {
      setThemePreference(nextThemePreference)
    }
  }

  return (
    <label className="theme-selector">
      <span className="theme-selector__label">Appearance</span>
      <span className="theme-selector__field">
        <select
          className="theme-selector__input"
          value={themePreference}
          onChange={handleChange}
          aria-label="Appearance"
        >
          <option value="system">System (default)</option>
          <option value="light">Light</option>
          <option value="dark">Dark</option>
        </select>
        <span className="theme-selector__icon" aria-hidden="true">
          ▾
        </span>
      </span>
    </label>
  )
}
