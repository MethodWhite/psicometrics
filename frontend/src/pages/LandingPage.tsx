import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'
import { TestCard } from '../components/TestCard'

const TESTS = [
  { type: 'big_five', name: 'Big Five / OCEAN' },
  { type: 'mbti', name: 'MBTI — Myers-Briggs' },
  { type: 'enneagram', name: 'Enneagrama' },
  { type: 'disc', name: 'DISC' },
  { type: 'dark_triad', name: 'Triada Oscura — SD3' },
  { type: 'human_design', name: 'Diseño Humano' },
]

const FEATURES = [
  {
    icon: (
      <svg className="w-8 h-8 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
      </svg>
    ),
    title: 'Reportes PDF',
    description: 'Descarga reports detallados con análisis completo de cada test, gráficos y recomendaciones personalizadas.',
  },
  {
    icon: (
      <svg className="w-8 h-8 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 3v11.25A2.25 2.25 0 0 0 6 16.5h2.25M3.75 3h-1.5m1.5 0h16.5m0 0h1.5m-1.5 0v11.25A2.25 2.25 0 0 1 18 16.5h-2.25m-7.5 0h7.5m-7.5 0-1 3m8.5-3 1 3m0 0 .5 1.5m-.5-1.5h-9.5m0 0-.5 1.5m.75-9 3-3 2.148 2.148A12.061 12.061 0 0 1 16.5 7.605" />
      </svg>
    ),
    title: 'Comparación',
    description: 'Compara tus resultados entre distintos tests y descubre patrones en tu personalidad.',
  },
  {
    icon: (
      <svg className="w-8 h-8 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 0 1 3 19.875v-6.75ZM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 0 1-1.125-1.125V8.625ZM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 0 1-1.125-1.125V4.125Z" />
      </svg>
    ),
    title: 'Evolución',
    description: 'Monitorea tus cambios a lo largo del tiempo con gráficos de evolución y tendencias.',
  },
]

const FAQ = [
  { q: '¿Qué tests están disponibles?', a: 'Actualmente ofrecemos 6 tests: Big Five (OCEAN), MBTI, Enneagrama, DISC, Triada Oscura (SD3) y Diseño Humano. Cada uno está basado en estándares científicos validados.' },
  { q: '¿Los tests son gratuitos?', a: 'Sí, todos los tests son completamente gratuitos. Puedes realizarlos cuantas veces quieras y descargar tus resultados en PDF.' },
  { q: '¿Cómo se protegen mis datos?', a: 'Tus resultados se almacenan de forma segura y solo tú puedes acceder a ellos. No compartimos datos con terceros bajo ningún concepto.' },
  { q: '¿Puedo comparar resultados entre tests?', a: 'Sí. La sección de comparación te permite ver tus resultados de distintos tests lado a lado para encontrar patrones y consistencias.' },
  { q: '¿Necesito crear una cuenta?', a: 'No es necesario para realizar los tests. Si creas una cuenta podrás guardar tu historial, comparar resultados y ver tu evolución a lo largo del tiempo.' },
]

const TESTIMONIALS = [
  { name: 'María G.', role: 'Psicóloga Clínica', text: 'Una herramienta excelente para mis pacientes. Los tests son precisos y los reportes muy completos.' },
  { name: 'Carlos M.', role: 'Profesional de RRHH', text: 'Me ha ayudado a entender mejor la dinámica de mi equipo. Muy recomendable.' },
  { name: 'Ana L.', role: 'Estudiante de Psicología', text: 'La precisión de los tests me sorprendió. Ideal para complementar mis estudios.' },
]

export default function LandingPage() {
  const { t } = useTranslation()

  return (
    <div className="min-h-screen">
      {/* ── Hero ── */}
      <section className="relative pt-28 pb-20 px-4 overflow-hidden">
        <div className="absolute inset-0 bg-gradient-to-b from-primary/5 via-transparent to-transparent pointer-events-none" />
        <div className="max-w-4xl mx-auto text-center relative z-10">
          <h1 className="text-5xl md:text-7xl font-bold mb-6 animate-fadeInUp">
            <span className="gradient-text">PsicoMetrics</span>
          </h1>
          <p className="text-xl md:text-2xl text-content-secondary mb-4 animate-fadeInUp delay-100">
            {t('app.subtitle')}
          </p>
          <p className="text-content-muted max-w-2xl mx-auto mb-10 animate-fadeInUp delay-200">
            {t('app.description')}
          </p>
          <div className="flex flex-col sm:flex-row items-center justify-center gap-4 animate-fadeInUp delay-300">
            <Link to="/test/big_five" className="btn-primary text-lg px-8 py-3 inline-block">
              Comenzar tu evaluación
            </Link>
            <a href="#tests" className="btn-secondary text-lg px-8 py-3 inline-block text-center">
              Ver tests
            </a>
          </div>
        </div>
      </section>

      {/* ── Tests Section ── */}
      <section id="tests" className="px-4 pb-20">
        <div className="max-w-6xl mx-auto">
          <h2 className="text-3xl md:text-4xl font-bold text-center mb-4 animate-fadeIn">
            6 tests científicamente validados
          </h2>
          <p className="text-content-muted text-center max-w-xl mx-auto mb-12 animate-fadeIn delay-100">
            Cada test sigue metodologías reconocidas internacionalmente para ofrecerte resultados precisos y fiables.
          </p>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {TESTS.map((test, i) => (
              <div key={test.type} className="animate-scaleIn" style={{ animationDelay: `${i * 80}ms` }}>
                <TestCard
                  testType={test.type}
                  name={test.name}
                  description={t(`tests.${test.type}.description`)}
                />
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* ── Features ── */}
      <section className="py-20 px-4 bg-surface/50">
        <div className="max-w-6xl mx-auto">
          <h2 className="text-3xl md:text-4xl font-bold text-center mb-4 animate-fadeIn">
            Todo lo que necesitas
          </h2>
          <p className="text-content-muted text-center max-w-xl mx-auto mb-16 animate-fadeIn delay-100">
            Herramientas profesionales para entender tu personalidad a fondo.
          </p>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {FEATURES.map((feature, i) => (
              <div
                key={feature.title}
                className="text-center animate-fadeInUp"
                style={{ animationDelay: `${i * 150}ms` }}
              >
                <div className="w-16 h-16 rounded-2xl bg-primary/10 flex items-center justify-center mx-auto mb-5">
                  {feature.icon}
                </div>
                <h3 className="text-xl font-semibold mb-3">{feature.title}</h3>
                <p className="text-content-muted leading-relaxed">{feature.description}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* ── Testimonials ── */}
      <section className="py-20 px-4">
        <div className="max-w-6xl mx-auto">
          <h2 className="text-3xl md:text-4xl font-bold text-center mb-16 animate-fadeIn">
            Lo que dicen nuestros usuarios
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {TESTIMONIALS.map((item, i) => (
              <div
                key={item.name}
                className="card animate-scaleIn"
                style={{ animationDelay: `${i * 120}ms` }}
              >
                <svg className="w-8 h-8 text-primary/30 mb-4" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M14.017 21v-7.391c0-5.704 3.731-9.57 8.983-10.609l.995 2.151c-2.432.917-3.995 3.638-3.995 5.849h4v10H14.017zM0 21v-7.391c0-5.704 3.748-9.57 9-10.609l.996 2.151C7.563 6.068 6 8.789 6 11h4v10H0z" />
                </svg>
                <p className="text-content-secondary mb-6 leading-relaxed">{item.text}</p>
                <div>
                  <p className="font-semibold">{item.name}</p>
                  <p className="text-sm text-content-muted">{item.role}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* ── FAQ ── */}
      <section className="py-20 px-4 bg-surface/50">
        <div className="max-w-3xl mx-auto">
          <h2 className="text-3xl md:text-4xl font-bold text-center mb-4 animate-fadeIn">
            Preguntas frecuentes
          </h2>
          <p className="text-content-muted text-center mb-12 animate-fadeIn delay-100">
            Respuestas rápidas a las dudas más comunes.
          </p>
          <div className="space-y-3">
            {FAQ.map((item, i) => (
              <details
                key={item.q}
                className="card group open:border-primary/30 transition-all duration-300 animate-fadeInUp"
                style={{ animationDelay: `${i * 80}ms` }}
              >
                <summary className="cursor-pointer list-none flex items-center justify-between gap-4 select-none">
                  <span className="font-medium text-content">{item.q}</span>
                  <svg
                    className="w-5 h-5 text-content-muted shrink-0 transition-transform duration-300 group-open:rotate-180"
                    fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}
                  >
                    <path strokeLinecap="round" strokeLinejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
                  </svg>
                </summary>
                <p className="mt-4 text-content-secondary leading-relaxed">{item.a}</p>
              </details>
            ))}
          </div>
        </div>
      </section>

      {/* ── CTA Section ── */}
      <section className="py-20 px-4">
        <div className="max-w-2xl mx-auto text-center animate-scaleIn">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            ¿Listo para conocerte mejor?
          </h2>
          <p className="text-content-muted mb-8">
            Comienza con cualquiera de nuestros tests gratuitos y descubre más sobre tu personalidad.
          </p>
          <Link to="/test/big_five" className="btn-primary text-lg px-10 py-3 inline-block">
            Comenzar ahora
          </Link>
        </div>
      </section>

      {/* ── Footer ── */}
      <footer className="border-t border-border py-12 px-4">
        <div className="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-8">
          <div>
            <span className="text-xl font-bold gradient-text">PsicoMetrics</span>
            <p className="text-content-muted text-sm mt-2 leading-relaxed">
              Tests de personalidad oficiales con estándares científicos validados.
            </p>
          </div>
          <div>
            <h4 className="font-semibold mb-3">Tests</h4>
            <ul className="space-y-2 text-sm text-content-muted">
              {TESTS.map((t) => (
                <li key={t.type}>
                  <Link to={`/test/${t.type}`} className="hover:text-content transition-colors">{t.name}</Link>
                </li>
              ))}
            </ul>
          </div>
          <div>
            <h4 className="font-semibold mb-3">Enlaces</h4>
            <ul className="space-y-2 text-sm text-content-muted">
              <li><Link to="/compare" className="hover:text-content transition-colors">Comparar tests</Link></li>
              <li><Link to="/evolution" className="hover:text-content transition-colors">Mi evolución</Link></li>
              <li><Link to="/account" className="hover:text-content transition-colors">Mi cuenta</Link></li>
            </ul>
          </div>
        </div>
        <div className="max-w-6xl mx-auto mt-10 pt-6 border-t border-border text-center text-sm text-content-muted">
          <p>PsicoMetrics v1.0 — Tests basados en estándares científicos validados</p>
        </div>
      </footer>
    </div>
  )
}
