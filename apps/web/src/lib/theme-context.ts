import { createContext } from 'react'
import type { ResolvedTheme, ThemePreference } from './theme'

export type ThemeContextValue = {
  themePreference: ThemePreference
  resolvedTheme: ResolvedTheme
  setThemePreference: (theme: ThemePreference) => void
}

export const ThemeContext = createContext<ThemeContextValue | undefined>(undefined)
