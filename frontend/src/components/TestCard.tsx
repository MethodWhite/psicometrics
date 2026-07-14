import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

interface TestCardProps {
  testType: string
  name: string
  description: string
}

const TEST_ICONS: Record<string, string> = {
  big_five: '🧠',
  mbti: '🎭',
  enneagram: '🔵',
  disc: '📊',
  dark_triad: '🌑',
  human_design: '⭐',
}

const TEST_COLORS: Record<string, string> = {
  big_five: 'from-indigo-500 to-purple-600',
  mbti: 'from-pink-500 to-rose-600',
  enneagram: 'from-cyan-500 to-blue-600',
  disc: 'from-amber-500 to-orange-600',
  dark_triad: 'from-red-500 to-orange-600',
  human_design: 'from-violet-500 to-indigo-600',
}

export function TestCard({ testType, name, description }: TestCardProps) {
  const { t } = useTranslation()
  const testInfo = t(`tests.${testType}`, { returnObjects: true }) as {
    items: string
    time: string
  }

  return (
    <Link to={`/test/${testType}`}>
      <div className="card group cursor-pointer">
        <div className={`w-14 h-14 rounded-xl bg-gradient-to-br ${TEST_COLORS[testType] || 'from-gray-500 to-gray-600'} flex items-center justify-center text-2xl mb-4 group-hover:scale-110 transition-transform`}>
          {TEST_ICONS[testType] || '📋'}
        </div>
        <h3 className="text-xl font-bold text-white mb-2">{name}</h3>
        <p className="text-slate-400 text-sm mb-4 leading-relaxed">{description}</p>
        <div className="flex items-center gap-4 text-xs text-slate-500">
          <span>{testInfo.items}</span>
          <span>•</span>
          <span>{testInfo.time}</span>
        </div>
        <div className="mt-4 flex items-center text-indigo-400 text-sm font-medium group-hover:text-indigo-300 transition-colors">
          {t('app.start_test')} →
        </div>
      </div>
    </Link>
  )
}
