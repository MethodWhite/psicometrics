import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts'
import type { EvolutionPoint } from '../types'

const TEST_OPTIONS = [
  { value: 'big_five', label: 'Big Five / OCEAN' },
  { value: 'mbti', label: 'MBTI' },
  { value: 'enneagram', label: 'Enneagrama' },
  { value: 'disc', label: 'DISC' },
  { value: 'dark_triad', label: 'Triada Oscura' },
]

const COLORS = [
  '#6366f1',
  '#06b6d4',
  '#f59e0b',
  '#ef4444',
  '#10b981',
  '#ec4899',
  '#8b5cf6',
  '#14b8a6',
]

function getStoredAccountId(): string | null {
  return localStorage.getItem('psicometrics_account_id')
}

export function EvolutionPage() {
  const navigate = useNavigate()
  const { t } = useTranslation()

  const accountId = getStoredAccountId()
  const [testType, setTestType] = useState('big_five')
  const [data, setData] = useState<EvolutionPoint[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  useEffect(() => {
    if (!accountId) return

    const fetchEvolution = async () => {
      setLoading(true)
      setError('')
      try {
        const response = await fetch(`/api/v1/accounts/${accountId}/evolution/${testType}`)
        if (!response.ok) throw new Error('Failed to fetch evolution data')
        const points: EvolutionPoint[] = await response.json()
        setData(points)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load evolution data')
        setData([])
      } finally {
        setLoading(false)
      }
    }

    fetchEvolution()
  }, [accountId, testType])

  if (!accountId) {
    return (
      <div className="min-h-screen px-4 py-8">
        <div className="max-w-2xl mx-auto text-center pt-20">
          <button
            onClick={() => navigate('/')}
            className="text-slate-400 hover:text-white transition-colors mb-4 block text-left"
          >
            ← {t('app.back')}
          </button>
          <div className="card">
            <p className="text-slate-400">{t('evolution.account_required')}</p>
            <button onClick={() => navigate('/account')} className="btn-primary mt-4">
              {t('app.account')}
            </button>
          </div>
        </div>
      </div>
    )
  }

  // Collect all dimension keys across all data points
  const dimensions = [...new Set(data.flatMap((p) => Object.keys(p.scores)))]

  return (
    <div className="min-h-screen px-4 py-8">
      <div className="max-w-4xl mx-auto">
        <div className="mb-8">
          <button
            onClick={() => navigate('/')}
            className="text-slate-400 hover:text-white transition-colors mb-4"
          >
            ← {t('app.back')}
          </button>
          <h2 className="text-2xl font-bold text-white">{t('evolution.title')}</h2>
        </div>

        {/* Test type selector */}
        <div className="card mb-6">
          <label className="block text-sm text-slate-400 mb-2">{t('evolution.select_test')}</label>
          <select
            value={testType}
            onChange={(e) => setTestType(e.target.value)}
            className="w-full bg-white/10 border border-white/20 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500"
          >
            {TEST_OPTIONS.map((opt) => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
        </div>

        {/* Loading */}
        {loading && (
          <div className="flex justify-center py-12">
            <div className="w-12 h-12 border-4 border-indigo-500 border-t-transparent rounded-full animate-spin" />
          </div>
        )}

        {/* Error */}
        {error && (
          <div className="card border-red-500/30 bg-red-500/10 mb-6">
            <p className="text-red-400">{error}</p>
          </div>
        )}

        {/* Chart */}
        {!loading && !error && data.length > 0 && (
          <div className="card">
            <ResponsiveContainer width="100%" height={400}>
              <LineChart data={data} margin={{ top: 20, right: 30, left: 20, bottom: 10 }}>
                <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.1)" />
                <XAxis
                  dataKey="date"
                  stroke="#94a3b8"
                  tick={{ fill: '#94a3b8', fontSize: 12 }}
                  tickFormatter={(val: string) => {
                    const d = new Date(val)
                    return d.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })
                  }}
                />
                <YAxis
                  domain={[0, 100]}
                  stroke="#94a3b8"
                  tick={{ fill: '#94a3b8', fontSize: 12 }}
                />
                <Tooltip
                  contentStyle={{
                    background: 'rgba(15, 23, 42, 0.95)',
                    border: '1px solid rgba(255,255,255,0.1)',
                    borderRadius: '12px',
                    color: '#e2e8f0',
                  }}
                  labelFormatter={(val: string) =>
                    new Date(val).toLocaleDateString(undefined, {
                      year: 'numeric',
                      month: 'long',
                      day: 'numeric',
                    })
                  }
                />
                <Legend
                  wrapperStyle={{ paddingTop: '16px' }}
                />
                {dimensions.map((dim, i) => (
                  <Line
                    key={dim}
                    type="monotone"
                    dataKey={`scores.${dim}`}
                    name={dim}
                    stroke={COLORS[i % COLORS.length]}
                    strokeWidth={2}
                    dot={{ r: 4, fill: COLORS[i % COLORS.length] }}
                    activeDot={{ r: 6 }}
                  />
                ))}
              </LineChart>
            </ResponsiveContainer>
          </div>
        )}

        {/* No data */}
        {!loading && !error && data.length === 0 && (
          <div className="card text-center py-12">
            <p className="text-slate-500">{t('evolution.no_data')}</p>
          </div>
        )}
      </div>
    </div>
  )
}
