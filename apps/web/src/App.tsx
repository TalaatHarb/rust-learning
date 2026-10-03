import { useEffect, useState } from 'react'
import { NavLink, Route, Routes, useLocation } from 'react-router-dom'
import { useAuth } from 'react-oidc-context'
import { DashboardPage } from './pages/DashboardPage'
import { ExercisePage } from './pages/ExercisePage'
import { ExerciseResultPage } from './pages/ExerciseResultPage'
import { LearningUnitPage } from './pages/LearningUnitPage'
import { LoginPage } from './pages/LoginPage'
import { ProgressPage } from './pages/ProgressPage'
import { RoadmapPage } from './pages/RoadmapPage'
import { ThemeSelector } from './components/ThemeSelector'

const navItems = [
  { to: '/', label: 'Dashboard' },
  { to: '/roadmap', label: 'Roadmap' },
  { to: '/result/latest', label: 'Result' },
  { to: '/progress', label: 'Progress' },
]

function App() {
  const auth = useAuth()
  const location = useLocation()
  const [isNavOpen, setIsNavOpen] = useState(false)
  const navigationItems = auth.isAuthenticated
    ? navItems
    : [...navItems, { to: '/login', label: 'Login' }]

  useEffect(() => {
    setIsNavOpen(false)
  }, [location.pathname])

  return (
    <div className="app-shell">
      <header className="topbar">
        <h1>Rust Learning Platform</h1>
        <div className="topbar__controls">
          <button
            type="button"
            className="menu-toggle"
            aria-expanded={isNavOpen}
            aria-controls="primary-navigation"
            aria-label={isNavOpen ? 'Close navigation menu' : 'Open navigation menu'}
            onClick={() => setIsNavOpen((value) => !value)}
          >
            <span className={isNavOpen ? 'menu-toggle__icon menu-toggle__icon--open' : 'menu-toggle__icon'}>
              <span className="menu-toggle__bar" />
              <span className="menu-toggle__bar" />
              <span className="menu-toggle__bar" />
            </span>
            <span className="menu-toggle__text">{isNavOpen ? 'Close menu' : 'Open menu'}</span>
          </button>
        </div>
      </header>
      <div className="layout">
        <nav
          id="primary-navigation"
          className={isNavOpen ? 'navigation navigation--open' : 'navigation'}
          aria-label="Primary"
        >
          <ThemeSelector />
          {navigationItems.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              className={({ isActive }) =>
                isActive ? 'nav-link nav-link--active' : 'nav-link'
              }
              onClick={() => setIsNavOpen(false)}
            >
              {item.label}
            </NavLink>
          ))}
          {auth.isAuthenticated ? (
            <button
              type="button"
              className="nav-link nav-button"
              onClick={() => {
                setIsNavOpen(false)
                void auth.signoutRedirect()
              }}
            >
              Logout
            </button>
          ) : null}
        </nav>
        <main className="content" aria-live="polite">
          <Routes>
            <Route path="/" element={<DashboardPage />} />
            <Route path="/login" element={<LoginPage />} />
            <Route path="/roadmap" element={<RoadmapPage />} />
            <Route path="/unit/:unitId" element={<LearningUnitPage />} />
            <Route path="/exercise/:unitId" element={<ExercisePage />} />
            <Route path="/result/:attemptId" element={<ExerciseResultPage />} />
            <Route path="/progress" element={<ProgressPage />} />
          </Routes>
        </main>
      </div>
    </div>
  )
}

export default App
