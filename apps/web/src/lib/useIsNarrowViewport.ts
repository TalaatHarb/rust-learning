import { useEffect, useState } from 'react'

const narrowViewportQuery = '(max-width: 640px)'

export function useIsNarrowViewport() {
  const getMatches = () =>
    typeof window !== 'undefined' && window.matchMedia(narrowViewportQuery).matches

  const [isNarrowViewport, setIsNarrowViewport] = useState(getMatches)

  useEffect(() => {
    if (typeof window === 'undefined') {
      return
    }

    const mediaQuery = window.matchMedia(narrowViewportQuery)
    const updateMatch = () => setIsNarrowViewport(mediaQuery.matches)

    updateMatch()
    mediaQuery.addEventListener('change', updateMatch)

    return () => {
      mediaQuery.removeEventListener('change', updateMatch)
    }
  }, [])

  return isNarrowViewport
}
