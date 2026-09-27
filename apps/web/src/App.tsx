import { NavLink, Route, Routes } from 'react-router-dom'
import { DashboardPage } from './pages/DashboardPage'
import { ExercisePage } from './pages/ExercisePage'
import { ExerciseResultPage } from './pages/ExerciseResultPage'
import { LearningUnitPage } from './pages/LearningUnitPage'
import { LoginPage } from './pages/LoginPage'
import { ProgressPage } from './pages/ProgressPage'
import { RoadmapPage } from './pages/RoadmapPage'

const navItems = [
  { to: '/', label: 'Dashboard' },
  { to: '/roadmap', label: 'Roadmap' },
  { to: '/unit/ownership', label: 'Learning Unit' },
  { to: '/exercise/ownership', label: 'Exercise' },
  { to: '/result/latest', label: 'Result' },
  { to: '/progress', label: 'Progress' },
  { to: '/login', label: 'Login' },
]

function App() {
  return (
    <div className="app-shell">
      <header className="topbar">
        <h1>Rust Learning Platform</h1>
      </header>
      <div className="layout">
        <nav className="navigation" aria-label="Primary">
          {navItems.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              className={({ isActive }) =>
                isActive ? 'nav-link nav-link--active' : 'nav-link'
              }
            >
              {item.label}
            </NavLink>
          ))}
        </nav>
        <main className="content" aria-live="polite">
          <Routes>
            <Route path="/" element={<DashboardPage />} />
            <Route path="/login" element={<LoginPage />} />
            <Route path="/roadmap" element={<RoadmapPage />} />
            <Route path="/unit/:unitId" element={<LearningUnitPage />} />
            <Route path="/exercise/:exerciseId" element={<ExercisePage />} />
            <Route path="/result/:attemptId" element={<ExerciseResultPage />} />
            <Route path="/progress" element={<ProgressPage />} />
          </Routes>
        </main>
      </div>
    </div>
  )
}

export default App
