import { useTranslation } from 'react-i18next'
import { TestCard } from '../components/TestCard'

const TESTS = [
  { type: 'big_five', name: 'Big Five / OCEAN' },
  { type: 'mbti', name: 'MBTI — Myers-Briggs' },
  { type: 'enneagram', name: 'Enneagrama' },
  { type: 'disc', name: 'DISC' },
  { type: 'dark_triad', name: 'Triada Oscura — SD3' },
  { type: 'human_design', name: 'Diseño Humano' },
]

export function Home() {
  const { t } = useTranslation()

  return (
    <div className="min-h-screen">
      {/* Hero */}
      <section className="pt-20 pb-16 px-4">
        <div className="max-w-4xl mx-auto text-center">
          <h1 className="text-5xl md:text-7xl font-bold mb-4">
            <span className="gradient-text">{t('app.title')}</span>
          </h1>
          <p className="text-xl md:text-2xl text-slate-300 mb-2">{t('app.subtitle')}</p>
          <p className="text-slate-500 max-w-2xl mx-auto mt-4">
            {t('app.description')}
          </p>
        </div>
      </section>

      {/* Test Grid */}
      <section className="px-4 pb-20">
        <div className="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {TESTS.map((test) => (
            <TestCard
              key={test.type}
              testType={test.type}
              name={test.name}
              description={t(`tests.${test.type}.description`)}
            />
          ))}
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t border-white/10 py-8 px-4 text-center text-slate-600 text-sm">
        <p>PsicoMetrics v1.0 — Tier S++ SecDevOps</p>
        <p className="mt-1">Tests basados en estándares científicos validados</p>
      </footer>
    </div>
  )
}
