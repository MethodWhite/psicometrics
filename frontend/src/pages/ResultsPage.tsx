import { useLocation, useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import { RadarChart } from '../components/RadarChart'
import { BarChart } from '../components/BarChart'
import { BodyGraph } from '../components/BodyGraph'
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

export function ResultsPage() {
  const location = useLocation()
  const navigate = useNavigate()
  const { t, i18n } = useTranslation()
  const { testType, result } = (location.state as { testType?: string; result?: any }) || {}

  if (!result || !testType) {
    navigate('/')
    return null
  }

  const renderBigFiveResults = (r: BigFiveResult) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-white mb-4">{t('results.your_profile')}</h3>
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
        <h4 className="text-lg font-bold text-white mb-4">{t('results.score')}</h4>
        <BarChart
          data={Object.entries(r.scores).map(([key, value]) => ({
            label: FACTOR_LABELS[key]?.[i18n.language as 'es' | 'en'] || key,
            value,
            color: 'bg-gradient-to-r from-indigo-500 to-purple-500',
          }))}
        />
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-4">{t('results.percentile')}</h4>
        <BarChart
          data={Object.entries(r.percentiles).map(([key, value]) => ({
            label: FACTOR_LABELS[key]?.[i18n.language as 'es' | 'en'] || key,
            value,
            color: 'bg-gradient-to-r from-cyan-500 to-blue-500',
          }))}
        />
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-2">{t('app.about')}</h4>
        <p className="text-slate-300 leading-relaxed">{r.profile_summary}</p>
      </div>
    </div>
  )

  const renderMBTIResults = (r: MBTIResult) => (
    <div className="space-y-8">
      <div className="card text-center">
        <h3 className="text-2xl font-bold text-white mb-4">{t('results.your_profile')}</h3>
        <div className="text-6xl font-black gradient-text mb-4">{r.type_code}</div>
        <p className="text-slate-300 leading-relaxed">{r.profile_summary}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-4">{t('results.score')}</h4>
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
        <h3 className="text-2xl font-bold text-white mb-4">{t('results.your_profile')}</h3>
        <div className="text-5xl font-black gradient-text mb-2">
          Tipo {r.dominant_type}w{r.wing}
        </div>
        <p className="text-slate-400 mb-4">
          {t('results.dominant')}: Tipo {r.dominant_type} | {t('results.wing')}: {r.wing}
        </p>
        <p className="text-slate-300 leading-relaxed">{r.profile_summary}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-4">{t('results.score')}</h4>
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
        <h3 className="text-2xl font-bold text-white mb-4">{t('results.your_profile')}</h3>
        <div className="text-5xl font-black gradient-text mb-2">
          {r.primary_style}{r.secondary_style}
        </div>
        <p className="text-slate-400 mb-4">
          {t('results.primary')}: {DISC_LABELS[r.primary_style]?.[i18n.language as 'es' | 'en']} |{' '}
          {t('results.secondary')}: {DISC_LABELS[r.secondary_style]?.[i18n.language as 'es' | 'en']}
        </p>
        <p className="text-slate-300 leading-relaxed">{r.profile_summary}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-4">{t('results.score')}</h4>
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
        <h3 className="text-2xl font-bold text-white mb-4">{t('results.your_profile')}</h3>
        <div className={`inline-block px-4 py-2 rounded-full text-sm font-bold mb-4 ${RISK_COLORS[r.risk_level]}`}>
          {r.risk_level === 'minimal' ? 'Mínimo' : r.risk_level === 'low' ? 'Bajo' : r.risk_level === 'moderate' ? 'Moderado' : 'Alto'}
        </div>
        <p className="text-slate-300 leading-relaxed">{r.profile_summary}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-4">Dark Core Score</h4>
        <div className="text-center mb-4">
          <span className="text-4xl font-black gradient-text">{r.dark_core.toFixed(1)}%</span>
        </div>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-4">{t('results.score')}</h4>
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
        <h3 className="text-2xl font-bold text-white mb-4">{t('results.your_profile')}</h3>
        <div className="text-5xl font-black gradient-text mb-2">
          {r.type_info?.en?.name || r.type}
        </div>
        <p className="text-slate-400 mb-2">
          Perfil: {r.profile}
        </p>
        <p className="text-slate-300 leading-relaxed">{r.summary}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-4">Estrategia</h4>
        <p className="text-slate-300">{r.strategy}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-4">Autoridad</h4>
        <p className="text-slate-300">{r.authority_info?.en?.description || r.authority}</p>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-4">Body Graph</h4>
        <BodyGraph centers={r.centers} />
        <div className="mt-4 grid grid-cols-3 gap-2 text-xs">
          {Object.entries(r.centers).map(([key, defined]) => (
            <div key={key} className={`p-2 rounded ${defined ? 'bg-indigo-500/20 text-indigo-300' : 'bg-white/5 text-slate-500'}`}>
              {key}
            </div>
          ))}
        </div>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-4">Gates Personaliades</h4>
        <div className="flex flex-wrap gap-2">
          {r.personality_gates.map((gate) => (
            <span key={gate} className="px-2 py-1 bg-indigo-500/20 text-indigo-300 rounded text-sm">
              {gate}
            </span>
          ))}
        </div>
      </div>

      <div className="card">
        <h4 className="text-lg font-bold text-white mb-4">Gates Diseño</h4>
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
            className="text-slate-400 hover:text-white transition-colors mb-4"
          >
            ← {t('app.back')}
          </button>
          <h2 className="text-2xl font-bold text-white">{t('app.results')}</h2>
        </div>

        {testType === 'big_five' && renderBigFiveResults(result)}
        {testType === 'mbti' && renderMBTIResults(result)}
        {testType === 'enneagram' && renderEnneagramResults(result)}
        {testType === 'disc' && renderDISCResults(result)}
        {testType === 'dark_triad' && renderDarkTriadResults(result)}
        {testType === 'human_design' && renderHumanDesignResults(result)}

        <div className="mt-8 flex gap-4">
          <button onClick={() => navigate('/')} className="btn-primary flex-1">
            {t('results.retake')}
          </button>
        </div>
      </div>
    </div>
  )
}
