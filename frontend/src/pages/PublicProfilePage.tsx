import { useEffect, useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import { RadarChart } from '../components/RadarChart'
import { BarChart } from '../components/BarChart'
import { BodyGraph } from '../components/BodyGraph'

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

export function PublicProfilePage() {
  const { shareCode } = useParams<{ shareCode: string }>()
  const navigate = useNavigate()
  const { t, i18n } = useTranslation()
  const [data, setData] = useState<any>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (!shareCode) return
    fetch(`/api/v1/public/${shareCode}`)
      .then((res) => {
        if (!res.ok) throw new Error('Profile not found')
        return res.json()
      })
      .then(setData)
      .catch((err) => setError(err.message))
      .finally(() => setLoading(false))
  }, [shareCode])

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-indigo-500" />
      </div>
    )
  }

  if (error || !data) {
    return (
      <div className="min-h-screen flex flex-col items-center justify-center px-4">
        <div className="text-6xl mb-4">🔗</div>
        <h2 className="text-2xl font-bold text-content mb-2">Perfil no encontrado</h2>
        <p className="text-content-muted mb-6">Este enlace de perfil público no existe o ha expirado.</p>
        <button onClick={() => navigate('/')} className="btn-primary">
          {t('app.back')}
        </button>
      </div>
    )
  }

  const { result, test_type: testType } = data

  const renderBigFiveResults = (r: any) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className="flex justify-center">
          <RadarChart
            data={Object.entries(r.scores || {}).map(([key, value]) => ({
              label: FACTOR_LABELS[key]?.[i18n.language as 'es' | 'en'] || key,
              value: value as number,
            }))}
            size={350}
          />
        </div>
      </div>
      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">{t('results.score')}</h4>
        <BarChart
          data={Object.entries(r.scores || {}).map(([key, value]) => ({
            label: FACTOR_LABELS[key]?.[i18n.language as 'es' | 'en'] || key,
            value: value as number,
            color: 'bg-gradient-to-r from-indigo-500 to-purple-500',
          }))}
        />
      </div>
      {r.profile_summary && (
        <div className="card">
          <h4 className="text-lg font-bold text-content mb-2">{t('app.about')}</h4>
          <p className="text-content-secondary leading-relaxed">{r.profile_summary}</p>
        </div>
      )}
    </div>
  )

  const renderMBTIResults = (r: any) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className="text-6xl font-black gradient-text mb-4">{r.type_code}</div>
        {r.profile_summary && <p className="text-content-secondary leading-relaxed">{r.profile_summary}</p>}
      </div>
      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">{t('results.score')}</h4>
        <BarChart
          data={Object.entries(r.percentages || {}).map(([key, value]) => ({
            label: key,
            value: value as number,
            color: 'bg-gradient-to-r from-pink-500 to-rose-500',
          }))}
        />
      </div>
    </div>
  )

  const renderEnneagramResults = (r: any) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className="text-5xl font-black gradient-text mb-2">Tipo {r.dominant_type}w{r.wing}</div>
        {r.profile_summary && <p className="text-content-secondary leading-relaxed">{r.profile_summary}</p>}
      </div>
      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">{t('results.score')}</h4>
        <BarChart
          data={Object.entries(r.scores || {}).map(([key, value]) => ({
            label: `Tipo ${key}`,
            value: value as number,
            color: 'bg-gradient-to-r from-cyan-500 to-blue-500',
          }))}
        />
      </div>
    </div>
  )

  const renderDISCResults = (r: any) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className="text-5xl font-black gradient-text mb-2">{r.primary_style}{r.secondary_style}</div>
        {r.profile_summary && <p className="text-content-secondary leading-relaxed">{r.profile_summary}</p>}
      </div>
      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">{t('results.score')}</h4>
        <BarChart
          data={Object.entries(r.scores || {}).map(([key, value]) => ({
            label: DISC_LABELS[key]?.[i18n.language as 'es' | 'en'] || key,
            value: value as number,
            color: 'bg-gradient-to-r from-amber-500 to-orange-500',
          }))}
        />
      </div>
    </div>
  )

  const renderDarkTriadResults = (r: any) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className="inline-block px-4 py-2 rounded-full text-sm font-bold mb-4 bg-red-500/20 text-red-400">
          {r.risk_level}
        </div>
        {r.profile_summary && <p className="text-content-secondary leading-relaxed">{r.profile_summary}</p>}
      </div>
      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">Dark Core</h4>
        <div className="text-center mb-4">
          <span className="text-4xl font-black gradient-text">{(r.dark_core || 0).toFixed(1)}%</span>
        </div>
      </div>
      <div className="card">
        <h4 className="text-lg font-bold text-content mb-4">{t('results.score')}</h4>
        <BarChart
          data={Object.entries(r.scores || {}).map(([key, value]) => ({
            label: DARK_TRIAD_LABELS[key]?.[i18n.language as 'es' | 'en'] || key,
            value: value as number,
            color: 'bg-gradient-to-r from-red-500 to-orange-500',
          }))}
        />
      </div>
    </div>
  )

  const renderHumanDesignResults = (r: any) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-content mb-4">{t('results.your_profile')}</h3>
        <div className="text-3xl font-black gradient-text mb-2">{r.type_info?.en?.name || r.type}</div>
        <p className="text-content-muted mb-2">Perfil: {r.profile}</p>
        {r.summary && <p className="text-content-secondary leading-relaxed">{r.summary}</p>}
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
        <BodyGraph centers={r.centers || {}} />
      </div>
    </div>
  )

  return (
    <div className="min-h-screen px-4 py-8">
      <div className="max-w-2xl mx-auto">
        <div className="mb-8 text-center">
          <div className="text-4xl mb-2">🔗</div>
          <h2 className="text-2xl font-bold text-content">Perfil Público</h2>
          <p className="text-content-muted text-sm">Resultados compartidos de PsicoMetrics</p>
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
            <h2 className="text-2xl font-bold text-content mb-6">{t('results.interpretation')}</h2>
            <div className="space-y-6">
              {Object.entries(result.interpretation as Record<string, any>).map(([factor, data]: [string, any]) => (
                <div key={factor} className="card">
                  <h3 className="text-lg font-bold text-content capitalize mb-2">{factor.replace(/_/g, ' ')}</h3>
                  <p className="text-content-secondary leading-relaxed">{data.daily_life || data.description}</p>
                </div>
              ))}
            </div>
          </section>
        )}

        {/* CTA */}
        <div className="mt-12 text-center">
          <p className="text-content-muted mb-4">¿Quieres descubrir tu personalidad?</p>
          <button onClick={() => navigate('/')} className="btn-primary text-lg px-8 py-3">
            🔬 Tomar un test ahora
          </button>
        </div>
      </div>
    </div>
  )
}
