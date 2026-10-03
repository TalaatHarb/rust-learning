import { useEffect, useMemo, useState, type ReactNode } from 'react'
import { ThemeContext } from './theme-context'

const themeStorageKey = 'rust-learning-theme-preference'
const systemThemeMediaQuery = '(prefers-color-scheme: dark)'

export type ThemePreference = 'system' | 'light' | 'dark'
export type ResolvedTheme = 'light' | 'dark'

function isResolvedTheme(value: string): value is ResolvedTheme {
  return value === 'light' || value === 'dark'
}

function getSystemTheme(): ResolvedTheme {
  if (typeof window === 'undefined') {
    return 'light'
  }

  return window.matchMedia(systemThemeMediaQuery).matches ? 'dark' : 'light'
}

function getStoredThemePreference(): ThemePreference {
  if (typeof window === 'undefined') {
    return 'system'
  }

  const storedValue = window.localStorage.getItem(themeStorageKey)
  return storedValue === 'light' || storedValue === 'dark' || storedValue === 'system'
    ? storedValue
    : 'system'
}

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [themePreference, setThemePreference] = useState<ThemePreference>(getStoredThemePreference)
  const [systemTheme, setSystemTheme] = useState<ResolvedTheme>(getSystemTheme)

  useEffect(() => {
    if (typeof window === 'undefined') {
      return
    }

    const mediaQuery = window.matchMedia(systemThemeMediaQuery)
    const syncSystemTheme = () => setSystemTheme(mediaQuery.matches ? 'dark' : 'light')

    syncSystemTheme()
    mediaQuery.addEventListener('change', syncSystemTheme)

    return () => {
      mediaQuery.removeEventListener('change', syncSystemTheme)
    }
  }, [])

  const resolvedTheme = themePreference === 'system' ? systemTheme : themePreference

  useEffect(() => {
    if (typeof window === 'undefined') {
      return
    }

    window.localStorage.setItem(themeStorageKey, themePreference)
  }, [themePreference])

  useEffect(() => {
    const root = document.documentElement
    root.dataset.theme = resolvedTheme
    root.dataset.themePreference = themePreference
    root.style.colorScheme = isResolvedTheme(resolvedTheme) ? resolvedTheme : 'light'
  }, [resolvedTheme, themePreference])

  const value = useMemo(
    () => ({ themePreference, resolvedTheme, setThemePreference }),
    [resolvedTheme, themePreference],
  )

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>
}
