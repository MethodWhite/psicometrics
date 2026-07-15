import { useState, useMemo } from 'react'
import { useLocation, useNavigate, Link } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import { RadarChart } from '../components/RadarChart'
import { BarChart } from '../components/BarChart'
import { BodyGraph } from '../components/BodyGraph'
import { PDFDownloadButton } from '../components/PDFDownloadButton'
import { RESOURCES } from '../data/resources'
import type { BigFiveResult, MBTIResult, EnneagramResult, DISCResult, DarkTriadResult, HumanDesignResult } from '../types'

const FACTOR_LABELS: Record<string, { es: string; en: string }> = {
  O: { es: 'Apertura', en: 'Openness' },
  C: { es: 'Responsabilidad', en: 'Conscientiousness' },
  E: { es: 'Extraversión', en: 'Extraversion' },
  A: { es: 'Amabilidad', en: 'Agreeableness' },
  N: { es: 'Neuroticismo', en: 'Neuroticism' },
}

const DISC_LABELS: Record<string, { es: string; en: string }> = {
  D: { es: 'Dominancia', en: 'Dominance' },
  I: { es: 'Influencia', en: 'Influence' },
  S: { es: 'Estabilidad', en: 'Steadiness' },
  C: { es: 'Conciencia', en: 'Conscientiousness' },
}

const DARK_TRIAD_LABELS: Record<string, { es: string; en: string }> = {
  machiavellianism: { es: 'Maquiavelismo', en: 'Machiavellianism' },
  narcissism: { es: 'Narcisismo', en: 'Narcissism' },
  psychopathy: { es: 'Psicopatía', en: 'Psychopathy' },
}

const RISK_COLORS: Record<string, string> = {
  minimal: 'text-green-400 bg-green-500/20',
  low: 'text-blue-400 bg-blue-500/20',
  moderate: 'text-yellow-400 bg-yellow-500/20',
  high: 'text-red-400 bg-red-500/20',
}

const LEVEL_COLORS: Record<string, string> = {
  low: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
  moderate: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
  high: 'bg-green-500/20 text-green-400 border-green-500/30',
}

const LEVEL_LABELS: Record<string, string> = {
  low: 'results.level_low',
  moderate: 'results.level_moderate',
  high: 'results.level_high',
}

function getResultTags(testType: string, result: any): { slug: string; name: string }[] {
  const tags: { slug: string; name: string }[] = []
  switch (testType) {
    case 'big_five':
      if (result.scores) {
        const factorMap: Record<string, string> = { O: 'openness', C: 'conscientiousness', E: 'extraversion', A: 'agreeableness', N: 'neuroticism' }
        for (const key of Object.keys(result.scores)) {
          const slug = factorMap[key]
          if (slug) tags.push({ slug, name: FACTOR_LABELS[key]?.es || key })
        }
      }
      break
    case 'mbti':
      if (result.type_code) {
        const slug = result.type_code.toLowerCase()
        tags.push({ slug, name: result.type_code })
      }
      break
    case 'enneagram':
      if (result.dominant_type) {
        tags.push({ slug: `type-${result.dominant_type}`, name: `Tipo ${result.dominant_type}` })
      }
      break
    case 'disc':
      if (result.primary_style) {
        const discMap: Record<string, string> = { D: 'dominance', I: 'influence', S: 'steadiness', C: 'conscientiousness-disc' }
        const slug = discMap[result.primary_style]
        if (slug) tags.push({ slug, name: DISC_LABELS[result.primary_style]?.es || result.primary_style })
      }
      if (result.secondary_style) {
        const discMap: Record<string, string> = { D: 'dominance', I: 'influence', S: 'steadiness', C: 'conscientiousness-disc' }
        const slug = discMap[result.secondary_style]
        if (slug) tags.push({ slug, name: DISC_LABELS[result.secondary_style]?.es || result.secondary_style })
      }
      break
    case 'dark_triad':
      tags.push({ slug: 'machiavellianism', name: 'Maquiavelismo' })
      tags.push({ slug: 'narcissism', name: 'Narcisismo' })
      tags.push({ slug: 'psychopathy', name: 'Psicopatía' })
      break
    case 'human_design':
      tags.push({ slug: 'human-design', name: 'Diseño Humano' })
      if (result.type) {
        const hdSlug = `human-design-${result.type.toLowerCase()}`
        tags.push({ slug: hdSlug, name: result.type })
      }
      break
  }
  return tags
}

function ResourcesForType({ testType, result }: { testType: string; result: any }) {
  const typeCode = result?.type_code || (result?.dominant_type ? `Type ${result.dominant_type}` : '')

  const resources = useMemo(() => {
    if (testType === 'mbti' && result?.type_code) {
      return RESOURCES
        .filter(r => r.recommendedBy.includes(result.type_code))
        .sort((a, b) => b.rating - a.rating)
        .slice(0, 5)
    }
    if (testType === 'enneagram' && result?.dominant_type) {
      return RESOURCES
        .filter(r => r.categories.includes('enneagram'))
        .sort((a, b) => b.rating - a.rating)
        .slice(0, 5)
    }
    if (testType === 'big_five') {
      return RESOURCES
        .filter(r => r.categories.includes('big-five'))
        .sort((a, b) => b.rating - a.rating)
        .slice(0, 5)
    }
    return []
  }, [testType, result])

  if (resources.length === 0) return null

  const linkPath = typeCode
    ? `/resources/${typeCode.replace('Type ', 'type-')}`
    : '/resources'

  return (
    <section className="mt-12">
      <div className="card">
        <h3 className="text-lg font-bold text-content mb-4 flex items-center gap-2">
          <span className="w-8 h-8 rounded-lg bg-gradient-to-br from-amber-500 to-orange-600 flex items-center justify-center text-sm">📚</span>
          Resources for your type
        </h3>
        <p className="text-content-secondary text-sm mb-4">
          Curated books, apps, and courses recommended for your personality profile.
        </p>
        <div className="space-y-3">
          {resources.map(r => (
            <div key={r.id} className="flex items-center justify-between gap-3 bg-surface-secondary rounded-xl p-3">
              <div className="min-w-0 flex-1">
                <p className="text-sm font-medium text-content truncate">{r.title}</p>
                <p className="text-xs text-content-muted">{r.author} · {r.type}</p>
              </div>
              <a
                href={r.url}
                target="_blank"
                rel="noopener noreferrer"
                className="btn-primary text-xs px-3 py-1.5 whitespace-nowrap"
              >
                Visit
              </a>
            </div>
          ))}
        </div>
        <Link
          to={linkPath}
          className="inline-block mt-4 text-sm text-primary hover:text-primary/80 transition-colors"
        >
          Browse all resources for your type →
        </Link>
      </div>
    </section>
  )
}

export function ResultsPage() {
  const location = useLocation()
  const navigate = useNavigate()
  const { t, i18n } = useTranslation()
  const { testType, result } = (location.state as { testType?: string; result?: any }) || {}

  const [saving, setSaving] = useState(false)
  const [sharing, setSharing] = useState(false)
  const [shareModal, setShareModal] = useState<{ url: string; code: string } | null>(null)
  const [exporting, setExporting] = useState(false)
  const [showPremiumModal, setShowPremiumModal] = useState(false)

  const handleSaveToAccount = async () => {
    const accountId = localStorage.getItem('psicometrics_account_id')
    if (!accountId) {
      navigate('/account')
      return
    }
    setSaving(true)
    try {
      const response = await fetch(`/api/v1/accounts/${accountId}/results`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ test_type: testType, result }),
      })
      if (!response.ok) throw new Error('Failed to save result')
    } catch (error) {
      console.error('Failed to save result:', error)
    } finally {
      setSaving(false)
    }
  }

  const getAccountId = () => localStorage.getItem('psicometrics_account_id')

  const handleShare = async () => {
    const accountId = getAccountId()
    if (!accountId) {
      navigate('/account')
      return
    }
    // First save the result if not already saved
    setSharing(true)
    try {
      // Save result first
      const saveRes = await fetch(`/api/v1/accounts/${accountId}/results`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ test_type: testType, result }),
      })
      if (!saveRes.ok) throw new Error('Failed to save')
      const { result_id } = await saveRes.json()

      // Create share link
      const shareRes = await fetch(`/api/v1/accounts/${accountId}/results/${result_id}/share`, {
        method: 'POST',
      })
      if (!shareRes.ok) throw new Error('Failed to create share link')
      const data = await shareRes.json()
      setShareModal({
        url: `${window.location.origin}${data.share_url}`,
        code: data.share_code,
      })
    } catch (error) {
      console.error('Share failed:', error)
      alert('Error al compartir. Asegúrate de tener una cuenta.')
    } finally {
      setSharing(false)
    }
  }

  const handleExportCSV = async () => {
    const accountId = getAccountId()
    if (!accountId) {
      navigate('/account')
      return
    }
    setExporting(true)
    try {
      const response = await fetch(`/api/v1/accounts/${accountId}/export/csv`)
      if (!response.ok) throw new Error('Export failed')
      const blob = await response.blob()
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = 'psicometrics_results.csv'
      a.click()
      URL.revokeObjectURL(url)
    } catch (error) {
      console.error('CSV export failed:', error)
      alert('Error al exportar CSV')
    } finally {
      setExporting(false)
    }
  }

  const handlePremiumCheckout = async () => {
    try {
      const response = await fetch('/api/v1/payments/create-checkout-session', {
        method: 'POST',
      })
      if (!response.ok) throw new Error('Checkout failed')
      const { url } = await response.json()
      window.open(url, '_blank')
    } catch (error) {
      console.error('Premium checkout failed:', error)
      alert('Error al iniciar el proceso premium')
    }
  }

  if (!result || !testType) {
    navigate('/')
    return null
  }

  const renderBigFiveResults = (r: BigFiveResult) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className="flex justify-center">
          <RadarChart
            data={Object.entries(r.scores).map(([key, value]) => ({
              label: FACTOR_LABELS[key]?.[i18n.language as 'es' | 'en'] || key,
              value,
            }))}
            size={350}
          />
        </div>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">{t('results.score')}</h4>
        <BarChart
          data={Object.entries(r.scores).map(([key, value]) => ({
            label: FACTOR_LABELS[key]?.[i18n.language as 'es' | 'en'] || key,
            value,
            color: 'bg-gradient-to-r from-indigo-500 to-purple-500',
          }))}
        />
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">{t('results.percentile')}</h4>
        <BarChart
          data={Object.entries(r.percentiles).map(([key, value]) => ({
            label: FACTOR_LABELS[key]?.[i18n.language as 'es' | 'en'] || key,
            value,
            color: 'bg-gradient-to-r from-cyan-500 to-blue-500',
          }))}
        />
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-2">{t('app.about')}</h4>
        <p className="text-content-secondary leading-relaxed">{r.profile_summary}</p>
      </div>
    </div>
  )

  const renderMBTIResults = (r: MBTIResult) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className="text-6xl font-black gradient-text mb-4">{r.type_code}</div>
        <p className="text-content-secondary leading-relaxed">{r.profile_summary}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">{t('results.score')}</h4>
        <BarChart
          data={Object.entries(r.percentages).map(([key, value]) => ({
            label: key,
            value,
            color: 'bg-gradient-to-r from-pink-500 to-rose-500',
          }))}
        />
      </div>
    </div>
  )

  const renderEnneagramResults = (r: EnneagramResult) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className="text-5xl font-black gradient-text mb-2">
          Tipo {r.dominant_type}w{r.wing}
        </div>
        <p className="text-content-muted mb-4">
          {t('results.dominant')}: Tipo {r.dominant_type} | {t('results.wing')}: {r.wing}
        </p>
        <p className="text-content-secondary leading-relaxed">{r.profile_summary}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">{t('results.score')}</h4>
        <BarChart
          data={Object.entries(r.scores).map(([key, value]) => ({
            label: `Tipo ${key}`,
            value,
            color: 'bg-gradient-to-r from-cyan-500 to-blue-500',
          }))}
        />
      </div>
    </div>
  )

  const renderDISCResults = (r: DISCResult) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className="text-5xl font-black gradient-text mb-2">
          {r.primary_style}{r.secondary_style}
        </div>
        <p className="text-content-muted mb-4">
          {t('results.primary')}: {DISC_LABELS[r.primary_style]?.[i18n.language as 'es' | 'en']} |{' '}
          {t('results.secondary')}: {DISC_LABELS[r.secondary_style]?.[i18n.language as 'es' | 'en']}
        </p>
        <p className="text-content-secondary leading-relaxed">{r.profile_summary}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">{t('results.score')}</h4>
        <BarChart
          data={Object.entries(r.scores).map(([key, value]) => ({
            label: DISC_LABELS[key]?.[i18n.language as 'es' | 'en'] || key,
            value,
            color: 'bg-gradient-to-r from-amber-500 to-orange-500',
          }))}
        />
      </div>
    </div>
  )

  const renderDarkTriadResults = (r: DarkTriadResult) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className={`inline-block px-4 py-2 rounded-full text-sm font-bold mb-4 ${RISK_COLORS[r.risk_level]}`}>
          {r.risk_level === 'minimal' ? 'Mínimo' : r.risk_level === 'low' ? 'Bajo' : r.risk_level === 'moderate' ? 'Moderado' : 'Alto'}
        </div>
        <p className="text-content-secondary leading-relaxed">{r.profile_summary}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">Dark Core Score</h4>
        <div className="text-center mb-4">
          <span className="text-4xl font-black gradient-text">{r.dark_core.toFixed(1)}%</span>
        </div>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">{t('results.score')}</h4>
        <BarChart
          data={Object.entries(r.scores).map(([key, value]) => ({
            label: DARK_TRIAD_LABELS[key]?.[i18n.language as 'es' | 'en'] || key,
            value,
            color: 'bg-gradient-to-r from-red-500 to-orange-500',
          }))}
        />
      </div>
    </div>
  )

  const renderHumanDesignResults = (r: HumanDesignResult) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className="text-5xl font-black gradient-text mb-2">
          {r.type_info?.en?.name || r.type}
        </div>
        <p className="text-content-muted mb-2">
          Perfil: {r.profile}
        </p>
        <p className="text-content-secondary leading-relaxed">{r.summary}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">Estrategia</h4>
        <p className="text-content-secondary">{r.strategy}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">Autoridad</h4>
        <p className="text-content-secondary">{r.authority_info?.en?.description || r.authority}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">Body Graph</h4>
        <BodyGraph centers={r.centers} />
        <div className="mt-4 grid grid-cols-3 gap-2 text-xs">
          {Object.entries(r.centers).map(([key, defined]) => (
            <div key={key} className={`p-2 rounded ${defined ? 'bg-indigo-500/20 text-indigo-300' : 'bg-surface-secondary text-content-muted'}`}>
              {key}
            </div>
          ))}
        </div>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">Gates Personaliades</h4>
        <div className="flex flex-wrap gap-2">
          {r.personality_gates.map((gate) => (
            <span key={gate} className="px-2 py-1 bg-indigo-500/20 text-indigo-300 rounded text-sm">
              {gate}
            </span>
          ))}
        </div>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">Gates Diseño</h4>
        <div className="flex flex-wrap gap-2">
          {r.design_gates.map((gate) => (
            <span key={gate} className="px-2 py-1 bg-purple-500/20 text-purple-300 rounded text-sm">
              {gate}
            </span>
          ))}
        </div>
      </div>
    </div>
  )

  return (
    <div className="min-h-screen px-4 py-8">
      <div className="max-w-2xl mx-auto">
        <div className="mb-8">
          <button
            onClick={() => navigate('/')}
            className="text-content-muted hover:text-content transition-colors mb-4"
          >
            ← {t('app.back')}
          </button>
          <h2 className="text-2xl font-bold text-content">{t('app.results')}</h2>
        </div>

        {testType === 'big_five' && renderBigFiveResults(result)}
        {testType === 'mbti' && renderMBTIResults(result)}
        {testType === 'enneagram' && renderEnneagramResults(result)}
        {testType === 'disc' && renderDISCResults(result)}
        {testType === 'dark_triad' && renderDarkTriadResults(result)}
        {testType === 'human_design' && renderHumanDesignResults(result)}

        {/* Interpretation */}
        {result.interpretation && (
          <section className="mt-12">
            <h2 className="text-2xl font-bold text-content mb-2">{t('results.interpretation')}</h2>
            <p className="text-content-muted text-sm mb-6">{t('results.interpretation_desc')}</p>
            <div className="space-y-6">
              {Object.entries(result.interpretation as Record<string, { level: string; daily_life: string; work: string; relationships: string }>).map(([factor, data]) => (
                <div key={factor} className="card">
                  <div className="flex items-center justify-between mb-4">
                    <h3 className="text-lg font-bold text-content capitalize">{factor.replace(/_/g, ' ')}</h3>
                    <span className={`px-3 py-1 rounded-full text-xs font-bold border ${LEVEL_COLORS[data.level] || 'bg-surface-secondary text-content-muted'}`}>
                      {t(LEVEL_LABELS[data.level] || data.level)}
                    </span>
                  </div>
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div className="bg-surface-secondary rounded-lg p-3">
                      <h4 className="text-xs font-bold text-content-muted uppercase tracking-wider mb-2">
                        {t('results.daily_life')}
                      </h4>
                      <p className="text-sm text-content-secondary leading-relaxed">{data.daily_life}</p>
                    </div>
                    <div className="bg-surface-secondary rounded-lg p-3">
                      <h4 className="text-xs font-bold text-content-muted uppercase tracking-wider mb-2">
                        {t('results.work')}
                      </h4>
                      <p className="text-sm text-content-secondary leading-relaxed">{data.work}</p>
                    </div>
                    <div className="bg-surface-secondary rounded-lg p-3">
                      <h4 className="text-xs font-bold text-content-muted uppercase tracking-wider mb-2">
                        {t('results.relationships')}
                      </h4>
                      <p className="text-sm text-content-secondary leading-relaxed">{data.relationships}</p>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Recommendations */}
        {(result.career_recommendations?.length > 0 || result.recommendations?.growth_areas?.length > 0) && (
          <section className="mt-12">
            <h2 className="text-2xl font-bold text-content mb-6">{t('results.recommendations')}</h2>
            <div className="space-y-6">
              {result.career_recommendations?.length > 0 && (
                <div className="card">
                  <h3 className="text-lg font-bold text-content mb-4 flex items-center gap-2">
                    <span className="w-8 h-8 rounded-lg bg-gradient-to-br from-amber-500 to-orange-600 flex items-center justify-center text-sm">💼</span>
                    {t('results.career_recommendations')}
                  </h3>
                  <ul className="space-y-2">
                    {result.career_recommendations.map((rec: string, i: number) => (
                      <li key={i} className="flex items-start gap-3 text-content-secondary">
                        <span className="text-indigo-400 mt-0.5 flex-shrink-0">✦</span>
                        <span className="leading-relaxed">{rec}</span>
                      </li>
                    ))}
                  </ul>
                </div>
              )}

              {result.recommendations?.growth_areas?.length > 0 && (
                <div className="card">
                  <h3 className="text-lg font-bold text-content mb-4 flex items-center gap-2">
                    <span className="w-8 h-8 rounded-lg bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center text-sm">🌱</span>
                    {t('results.growth_areas')}
                  </h3>
                  <ul className="space-y-2">
                    {result.recommendations.growth_areas.map((area: string, i: number) => (
                      <li key={i} className="flex items-start gap-3 text-content-secondary">
                        <span className="text-cyan-400 mt-0.5 flex-shrink-0">✦</span>
                        <span className="leading-relaxed">{area}</span>
                      </li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          </section>
        )}

        {/* Learn More */}
        {testType === 'mbti' && result?.type_code && (
          <section className="mt-12">
            <div className="card">
              <h3 className="text-lg font-bold text-content mb-4 flex items-center gap-2">
                <span className="w-8 h-8 rounded-lg bg-gradient-to-br from-pink-500 to-rose-600 flex items-center justify-center text-sm">🎭</span>
                Aprende más sobre tu tipo
              </h3>
              <p className="text-content-secondary text-sm mb-4">
                Descubre todo sobre la personalidad {result.type_code}: fortalezas, debilidades, relaciones, carreras y personajes famosos.
              </p>
              <a
                href={`/profile/${result.type_code}`}
                className="btn-primary inline-block text-center text-sm"
              >
                Perfil completo {result.type_code} →
              </a>
            </div>
          </section>
        )}

        {testType === 'enneagram' && result?.dominant_type && (
          <section className="mt-12">
            <div className="card">
              <h3 className="text-lg font-bold text-content mb-4 flex items-center gap-2">
                <span className="w-8 h-8 rounded-lg bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center text-sm">🔵</span>
                Aprende más sobre tu eneatipo
              </h3>
              <p className="text-content-secondary text-sm mb-4">
                Descubre todo sobre el Eneatipo {result.dominant_type}: motivaciones, fortalezas, camino de crecimiento y personajes famosos.
              </p>
              <a
                href={`/blog?category=enneagram`}
                className="btn-primary inline-block text-center text-sm"
              >
                Leer sobre Eneatipo {result.dominant_type} →
              </a>
            </div>
          </section>
        )}

        {testType === 'big_five' && (
          <section className="mt-12">
            <div className="card">
              <h3 className="text-lg font-bold text-content mb-4 flex items-center gap-2">
                <span className="w-8 h-8 rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-sm">🧠</span>
                Profundiza en los 5 grandes factores
              </h3>
              <p className="text-content-secondary text-sm mb-4">
                Explora artículos detallados sobre cada factor del modelo Big Five: Apertura, Responsabilidad, Extraversión, Amabilidad y Neuroticismo.
              </p>
              <a
                href="/blog?category=big-five"
                className="btn-primary inline-block text-center text-sm"
              >
                Artículos Big Five →
              </a>
            </div>
          </section>
        )}

        {/* Resources for your type */}
        {(testType === 'mbti' || testType === 'enneagram' || testType === 'big_five') && (
          <ResourcesForType testType={testType} result={result} />
        )}

        <section className="mt-8">
          <div className="card">
            <h3 className="text-lg font-bold text-content mb-4 flex items-center gap-2">
              <span className="w-8 h-8 rounded-lg bg-gradient-to-br from-green-500 to-teal-600 flex items-center justify-center text-sm">📤</span>
              Compartir en redes sociales
            </h3>
            <p className="text-content-secondary text-sm mb-4">
              Comparte tus resultados con tus amigos y descubre sus tipos de personalidad.
            </p>
            <div className="flex flex-wrap gap-3">
              <button
                onClick={() => {
                  const text = `¡Acabo de descubrir mi tipo de personalidad en PsicoMetrics! ${window.location.origin}`
                  window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(text)}`, '_blank', 'noopener,noreferrer')
                }}
                className="btn-secondary text-sm flex items-center gap-2"
              >
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
                Twitter / X
              </button>
              <button
                onClick={() => {
                  const url = window.location.origin
                  window.open(`https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(url)}`, '_blank', 'noopener,noreferrer')
                }}
                className="btn-secondary text-sm flex items-center gap-2"
              >
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"/></svg>
                Facebook
              </button>
              <button
                onClick={() => {
                  const text = `¡Acabo de descubrir mi tipo de personalidad en PsicoMetrics!`
                  window.open(`https://wa.me/?text=${encodeURIComponent(text + ' ' + window.location.origin)}`, '_blank', 'noopener,noreferrer')
                }}
                className="btn-secondary text-sm flex items-center gap-2"
              >
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413z"/></svg>
                WhatsApp
              </button>
              <button
                onClick={() => {
                  const text = `¡Acabo de descubrir mi tipo de personalidad en PsicoMetrics! ${window.location.origin}`
                  navigator.clipboard.writeText(text)
                }}
                className="btn-secondary text-sm flex items-center gap-2"
              >
                📋 Copiar enlace
              </button>
            </div>
          </div>
        </section>

        {/* Validation Warnings */}
        {result.validity && !result.validity.valid && result.validity.warnings?.length > 0 && (
          <div className="mt-8 card border-yellow-500/30 bg-yellow-500/10">
            <h4 className="text-lg font-bold text-yellow-400 mb-2">
              {t('results.validation_warnings')}
            </h4>
            <ul className="list-disc list-inside text-yellow-300/80 space-y-1">
              {result.validity.warnings.map((w: string, i: number) => (
                <li key={i}>{w}</li>
              ))}
            </ul>
          </div>
        )}

        {/* Action Buttons */}
        <div className="mt-8 flex flex-wrap gap-3">
          <PDFDownloadButton testType={testType} />
          <button
            onClick={() => navigate('/compare', { state: { result, testType } })}
            className="btn-secondary flex-1"
          >
            {t('results.compare_with')}
          </button>
          <button
            onClick={handleSaveToAccount}
            disabled={saving}
            className="btn-secondary flex-1"
          >
            {saving ? t('app.loading') : t('results.save_to_account')}
          </button>
          <button
            onClick={handleShare}
            disabled={sharing}
            className="btn-secondary flex-1"
          >
            {sharing ? t('app.loading') : '🔗 Compartir'}
          </button>
          <button
            onClick={handleExportCSV}
            disabled={exporting}
            className="btn-secondary flex-1"
          >
            {exporting ? t('app.loading') : '📊 Exportar CSV'}
          </button>
          <button
            onClick={() => setShowPremiumModal(true)}
            className="btn-primary flex-1"
          >
            ⭐ Informe Premium
          </button>
        </div>

        {/* Tag links */}
        <section className="mt-8">
          <div className="card">
            <h3 className="text-lg font-bold text-content mb-4 flex items-center gap-2">
              <span className="w-8 h-8 rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-sm">🏷️</span>
              Explorar más sobre tu perfil
            </h3>
            <p className="text-content-secondary text-sm mb-4">
              Descubre artículos y recursos relacionados con tu tipo de personalidad.
            </p>
            <div className="flex flex-wrap gap-2">
              {(() => {
                const tags = getResultTags(testType, result)
                return tags.map(tag => (
                  <a
                    key={tag.slug}
                    href={`/tags/${tag.slug}`}
                    className="px-3 py-1.5 bg-surface-secondary rounded-full text-xs text-content-muted hover:text-content hover:bg-indigo-500/10 border border-border hover:border-indigo-500/30 transition-all"
                  >
                    {tag.name}
                  </a>
                ))
              })()}
            </div>
            <div className="mt-4">
              <a href="/tags" className="text-sm text-indigo-400 hover:text-indigo-300 transition-colors">
                Explorar todos los tags →
              </a>
            </div>
          </div>
        </section>

        <div className="mt-8 flex gap-4">
          <button onClick={() => navigate('/')} className="btn-primary flex-1">
            {t('results.retake')}
          </button>
        </div>
      </div>

      {/* Share Modal */}
      {shareModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
          <div className="bg-surface border border-border rounded-2xl p-6 max-w-md w-full mx-4 shadow-2xl">
            <h3 className="text-lg font-bold text-content mb-2">Enlace público creado</h3>
            <p className="text-content-muted text-sm mb-4">
              Comparte este enlace para que otros vean tus resultados:
            </p>
            <div className="bg-surface-secondary rounded-xl p-3 mb-4">
              <code className="text-sm text-indigo-400 break-all">{shareModal.url}</code>
            </div>
            <div className="flex gap-3">
              <button
                onClick={() => {
                  navigator.clipboard.writeText(shareModal.url)
                }}
                className="btn-secondary flex-1 text-sm"
              >
                📋 Copiar enlace
              </button>
              <button
                onClick={() => setShareModal(null)}
                className="btn-primary flex-1 text-sm"
              >
                Cerrar
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Premium Modal */}
      {showPremiumModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
          <div className="bg-surface border border-border rounded-2xl p-6 max-w-md w-full mx-4 shadow-2xl">
            <div className="text-center mb-6">
              <span className="text-4xl">⭐</span>
              <h3 className="text-xl font-bold text-content mt-2">Informe Premium</h3>
              <p className="text-content-muted text-sm mt-1">
                Análisis exhaustivo de 5 páginas con desglose por facetas, tablas comparativas y plan de acción
              </p>
            </div>
            <ul className="space-y-2 mb-6 text-sm text-content-secondary">
              <li className="flex items-center gap-2">📋 Resumen ejecutivo</li>
              <li className="flex items-center gap-2">🔬 Análisis detallado por factor</li>
              <li className="flex items-center gap-2">📊 Tablas comparativas</li>
              <li className="flex items-center gap-2">🎯 Grid de recomendaciones</li>
              <li className="flex items-center gap-2">📅 Plan de acción 90 días</li>
            </ul>
            <div className="flex gap-3">
              <button
                onClick={() => setShowPremiumModal(false)}
                className="btn-secondary flex-1 text-sm"
              >
                Cancelar
              </button>
              <button
                onClick={handlePremiumCheckout}
                className="btn-primary flex-1 text-sm"
              >
                Obtener por $9.99
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
