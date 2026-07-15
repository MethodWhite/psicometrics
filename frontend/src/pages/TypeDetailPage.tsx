import { useState, useMemo } from 'react'
import { useParams, Link } from 'react-router-dom'
import { typeHubs, getTypeHub } from '../data/type-hubs'
import { getCompatibility } from '../data/type-comparisons'

type TabId = 'overview' | 'strengths' | 'relationships' | 'career' | 'growth'

const TABS: { id: TabId; label: string }[] = [
  { id: 'overview', label: 'Overview' },
  { id: 'strengths', label: 'Strengths & Weaknesses' },
  { id: 'relationships', label: 'Relationships' },
  { id: 'career', label: 'Career & Learning' },
  { id: 'growth', label: 'Growth & Famous People' },
]

export default function TypeDetailPage() {
  const { typeCode } = useParams<{ typeCode: string }>()
  const [activeTab, setActiveTab] = useState<TabId>('overview')

  const type = useMemo(() => {
    if (!typeCode) return undefined
    return getTypeHub(typeCode.toUpperCase())
  }, [typeCode])

  if (!type) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <h2 className="text-2xl font-bold mb-4">Type not found</h2>
          <p className="text-content-muted mb-6">The MBTI type you are looking for does not exist.</p>
          <Link to="/types" className="btn-primary inline-block">Back to Type Hub</Link>
        </div>
      </div>
    )
  }

  const compatibilityRows = type.compatibleTypes.map(t => {
    const compat = getCompatibility(type.typeCode, t)
    return { typeCode: t, score: compat?.score ?? 50 }
  }).sort((a, b) => b.score - a.score)

  // const challengingRows = type.challengingTypes.map(t => {
  //   const compat = getCompatibility(type.typeCode, t)
  //   return { typeCode: t, score: compat?.score ?? 50 }
  // }).sort((a, b) => a.score - b.score)

  return (
    <div className="min-h-screen">
      {/* ── Hero ── */}
      <section
        className="relative pt-20 pb-16 px-4 overflow-hidden"
        style={{
          background: `linear-gradient(135deg, ${type.color}15, ${type.color}05)`,
        }}
      >
        <div className="max-6xl mx-auto">
          <Link
            to="/types"
            className="inline-flex items-center gap-2 text-sm text-content-muted hover:text-content mb-8 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18" />
            </svg>
            Back to Type Hub
          </Link>

          <div className="flex items-center gap-6 mb-6">
            <span className="text-6xl">{type.icon}</span>
            <div>
              <h1 className="text-5xl md:text-6xl font-bold" style={{ color: type.color }}>
                {type.typeCode}
              </h1>
              <p className="text-2xl text-content-secondary mt-1">{type.name}</p>
            </div>
          </div>
        </div>
      </section>

      {/* ── Tabs ── */}
      <nav className="sticky top-16 z-40 bg-surface/80 backdrop-blur-xl border-b border-border">
        <div className="max-w-6xl mx-auto px-4 flex overflow-x-auto gap-1">
          {TABS.map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`px-4 py-3 text-sm font-medium whitespace-nowrap border-b-2 transition-colors ${
                activeTab === tab.id
                  ? 'border-primary text-content'
                  : 'border-transparent text-content-muted hover:text-content'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </div>
      </nav>

      {/* ── Tab Content ── */}
      <div className="max-w-6xl mx-auto px-4 py-10">
        {/* Overview */}
        {activeTab === 'overview' && (
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
            <div className="lg:col-span-2 space-y-6">
              <div className="card">
                <h2 className="text-xl font-bold mb-4">About {type.name}</h2>
                {type.description.split('\n\n').map((para, i) => (
                  <p key={i} className="text-content-secondary leading-relaxed mb-4 last:mb-0">
                    {para}
                  </p>
                ))}
              </div>

              <div className="card">
                <h2 className="text-xl font-bold mb-4">Communication Style</h2>
                <p className="text-content-secondary leading-relaxed">{type.communicationStyle}</p>
              </div>

              <div className="card">
                <h2 className="text-xl font-bold mb-4">Stress Response</h2>
                <p className="text-content-secondary leading-relaxed">{type.stressResponse}</p>
              </div>
            </div>

            {/* Sidebar */}
            <div className="space-y-6">
              <div className="card">
                <h3 className="font-semibold mb-3">Compatible Types</h3>
                <div className="space-y-2">
                  {compatibilityRows.map(ct => (
                    <Link
                      key={ct.typeCode}
                      to={`/types/${ct.typeCode.toLowerCase()}`}
                      className="flex items-center justify-between p-2 rounded-lg hover:bg-surface-secondary transition-colors"
                    >
                      <span className="font-medium">{ct.typeCode}</span>
                      <span className={`text-sm font-semibold ${
                        ct.score >= 80 ? 'text-green-500' :
                        ct.score >= 60 ? 'text-yellow-500' : 'text-content-muted'
                      }`}>
                        {ct.score}%
                      </span>
                    </Link>
                  ))}
                </div>
              </div>

              <div className="card">
                <h3 className="font-semibold mb-3">Quick Facts</h3>
                <ul className="space-y-2 text-sm text-content-secondary">
                  <li><span className="font-medium text-content">Icon:</span> {type.icon}</li>
                  <li><span className="font-medium text-content">Strengths:</span> {type.strengths.length}</li>
                  <li><span className="font-medium text-content">Career Paths:</span> {type.careerPaths.length}</li>
                  <li><span className="font-medium text-content">Famous People:</span> {type.famousPeople.length}</li>
                </ul>
              </div>

              <Link
                to="/test/mbti"
                className="btn-primary text-center block text-lg py-3"
              >
                Take the MBTI Test
              </Link>
            </div>
          </div>
        )}

        {/* Strengths & Weaknesses */}
        {activeTab === 'strengths' && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div className="card">
              <h2 className="text-xl font-bold mb-4 text-green-600 dark:text-green-400">Strengths</h2>
              <ul className="space-y-2">
                {type.strengths.map((s, i) => (
                  <li key={i} className="flex items-start gap-2 text-content-secondary">
                    <svg className="w-5 h-5 text-green-500 shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                      <path strokeLinecap="round" strokeLinejoin="round" d="m4.5 12.75 6 6 9-13.5" />
                    </svg>
                    {s}
                  </li>
                ))}
              </ul>
            </div>

            <div className="card">
              <h2 className="text-xl font-bold mb-4 text-red-600 dark:text-red-400">Weaknesses</h2>
              <ul className="space-y-2">
                {type.weaknesses.map((w, i) => (
                  <li key={i} className="flex items-start gap-2 text-content-secondary">
                    <svg className="w-5 h-5 text-red-500 shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                      <path strokeLinecap="round" strokeLinejoin="round" d="M6 18 18 6M6 6l12 12" />
                    </svg>
                    {w}
                  </li>
                ))}
              </ul>
            </div>
          </div>
        )}

        {/* Relationships */}
        {activeTab === 'relationships' && (
          <div className="max-w-3xl">
            <div className="card mb-8">
              <h2 className="text-xl font-bold mb-4">Relationships & Compatibility</h2>
              {type.relationships.split('\n\n').map((para, i) => (
                <p key={i} className="text-content-secondary leading-relaxed mb-4 last:mb-0">
                  {para}
                </p>
              ))}
            </div>

            <h3 className="text-lg font-bold mb-4">Compatibility Matrix</h3>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              {compatibilityRows.map(ct => {
                const otherType = typeHubs.find(t => t.typeCode === ct.typeCode)
                return (
                  <Link
                    key={ct.typeCode}
                    to={`/types/${ct.typeCode.toLowerCase()}`}
                    className="card flex items-center gap-4 hover:shadow-lg transition-all"
                  >
                    <span className="text-2xl">{otherType?.icon}</span>
                    <div className="flex-1">
                      <p className="font-semibold">{ct.typeCode} &mdash; {otherType?.name}</p>
                      <div className="flex items-center gap-2 mt-1">
                        <div className="flex-1 h-2 rounded-full bg-border overflow-hidden">
                          <div
                            className={`h-full rounded-full transition-all ${
                              ct.score >= 80 ? 'bg-green-500' :
                              ct.score >= 60 ? 'bg-yellow-500' :
                              ct.score >= 40 ? 'bg-orange-500' : 'bg-red-500'
                            }`}
                            style={{ width: `${ct.score}%` }}
                          />
                        </div>
                        <span className={`text-sm font-semibold ${
                          ct.score >= 80 ? 'text-green-500' :
                          ct.score >= 60 ? 'text-yellow-500' :
                          ct.score >= 40 ? 'text-orange-500' : 'text-red-500'
                        }`}>
                          {ct.score}%
                        </span>
                      </div>
                    </div>
                  </Link>
                )
              })}
            </div>

            <div className="mt-8">
              <Link
                to={`/compare-types?types=${type.typeCode},`}
                className="btn-secondary inline-block"
              >
                Compare with another type
              </Link>
            </div>
          </div>
        )}

        {/* Career & Learning */}
        {activeTab === 'career' && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
            <div className="card">
              <h2 className="text-xl font-bold mb-4">Career Paths</h2>
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-2">
                {type.careerPaths.map((c, i) => (
                  <div key={i} className="flex items-center gap-2 p-2 rounded-lg bg-surface-secondary/50">
                    <span className="w-6 h-6 rounded-full bg-primary/10 text-primary text-xs flex items-center justify-center font-medium">
                      {i + 1}
                    </span>
                    <span className="text-sm text-content-secondary">{c}</span>
                  </div>
                ))}
              </div>
            </div>

            <div className="space-y-6">
              <div className="card">
                <h2 className="text-xl font-bold mb-4">Study Habits</h2>
                <p className="text-content-secondary leading-relaxed">{type.studyHabits}</p>
              </div>

              <div className="card">
                <h2 className="text-xl font-bold mb-4">Communication Style</h2>
                <p className="text-content-secondary leading-relaxed">{type.communicationStyle}</p>
              </div>
            </div>
          </div>
        )}

        {/* Growth & Famous People */}
        {activeTab === 'growth' && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
            <div className="card">
              <h2 className="text-xl font-bold mb-4">Personal Growth Tips</h2>
              <ul className="space-y-3">
                {type.growthTips.map((tip, i) => (
                  <li key={i} className="flex items-start gap-3 p-3 rounded-lg bg-surface-secondary/50">
                    <span className="w-7 h-7 rounded-full bg-primary/10 text-primary text-sm flex items-center justify-center font-bold shrink-0">
                      {i + 1}
                    </span>
                    <span className="text-content-secondary leading-relaxed">{tip}</span>
                  </li>
                ))}
              </ul>
            </div>

            <div className="space-y-6">
              <div className="card">
                <h2 className="text-xl font-bold mb-4">Famous {type.typeCode}s</h2>
                <div className="space-y-2">
                  {type.famousPeople.map((person, i) => (
                    <div key={i} className="flex items-center gap-2 p-2 rounded-lg hover:bg-surface-secondary transition-colors">
                      <span className="w-2 h-2 rounded-full" style={{ backgroundColor: type.color }} />
                      <span className="text-content-secondary text-sm">{person}</span>
                    </div>
                  ))}
                </div>
              </div>

              <div className="card">
                <h2 className="text-xl font-bold mb-4">Recommended Books</h2>
                <ul className="space-y-2">
                  {type.recommendedBooks.map((book, i) => (
                    <li key={i} className="flex items-start gap-2 text-content-secondary text-sm">
                      <span className="text-primary shrink-0">&#8226;</span>
                      {book}
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* ── CTA ── */}
      <section className="py-16 px-4 bg-gradient-to-b from-primary/5 to-transparent">
        <div className="max-w-2xl mx-auto text-center">
          <h2 className="text-3xl font-bold mb-4">Discover Your Type</h2>
          <p className="text-content-secondary mb-8">
            Take the official MBTI test and discover your personality type with detailed analysis.
          </p>
          <Link to="/test/mbti" className="btn-primary text-lg px-10 py-3 inline-block">
            Take the MBTI Test
          </Link>
        </div>
      </section>
    </div>
  )
}
