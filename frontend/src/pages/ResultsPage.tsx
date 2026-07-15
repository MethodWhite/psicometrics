import { useState } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import { RadarChart } from '../components/RadarChart'
import { BarChart } from '../components/BarChart'
import { BodyGraph } from '../components/BodyGraph'
import { PDFDownloadButton } from '../components/PDFDownloadButton'
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
