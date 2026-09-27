import { Link } from 'react-router-dom'
import { useQuery } from '@tanstack/react-query'
import { fetchRoadmap } from '../lib/api'

export function RoadmapPage() {
  const roadmap = useQuery({
    queryKey: ['roadmap', 'foundations'],
    queryFn: fetchRoadmap,
  })

  if (roadmap.isLoading) {
    return (
      <section className="panel">
        <h2>Roadmap</h2>
        <p>Loading roadmap...</p>
      </section>
    )
  }

  if (roadmap.isError || !roadmap.data) {
    return (
      <section className="panel">
        <h2>Roadmap</h2>
        <p>{roadmap.error instanceof Error ? roadmap.error.message : 'Failed to load roadmap.'}</p>
      </section>
    )
  }

  return (
    <section className="panel">
      <h2>{roadmap.data.title}</h2>
      <p>Curriculum metadata is loaded from the API roadmap content endpoint.</p>
      {roadmap.data.modules.map((module) => (
        <section key={module.id}>
          <h3>{module.title}</h3>
          <ul>
            {module.units.map((unit) => (
              <li key={unit.id}>
                <Link className="inline-link" to={`/unit/${unit.slug}`}>
                  {unit.title}
                </Link>
              </li>
            ))}
          </ul>
        </section>
      ))}
    </section>
  )
}
