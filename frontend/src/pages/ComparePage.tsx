import { useState } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import type { CompareResult } from '../types'

export function ComparePage() {
  const location = useLocation()
  const navigate = useNavigate()
  const { t } = useTranslation()

  const state = location.state as { result?: Record<string, unknown>; testType?: string } | null
  const prefillA = state?.result ? JSON.stringify(state.result, null, 2) : ''

  const [resultA, setResultA] = useState(prefillA)
  const [resultB, setResultB] = useState('')
  const [testType, setTestType] = useState(state?.testType || 'big_five')
  const [compareResult, setCompareResult] = useState<CompareResult | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  const handleCompare = async () => {
    setError('')
    setCompareResult(null)

    let parsedA: Record<string, unknown>
    let parsedB: Record<string, unknown>
    try {
      parsedA = JSON.parse(resultA)
      parsedB = JSON.parse(resultB)
    } catch {
      setError('Invalid JSON in one or both inputs')
      return
    }

    setLoading(true)
    try {
      const response = await fetch(`/api/v1/tests/${testType}/compare`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ result_a: parsedA, result_b: parsedB }),
      })
      if (!response.ok) {
        const errBody = await response.text()
        throw new Error(errBody || 'Compare request failed')
      }
      const data: CompareResult = await response.json()
      setCompareResult(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Compare failed')
    } finally {
      setLoading(false)
    }
  }

  const compatibilityColor = (score: number) => {
    if (score >= 80) return 'text-green-400'
    if (score >= 60) return 'text-yellow-400'
    return 'text-red-400'
  }

  const compatibilityBg = (score: number) => {
    if (score >= 80) return 'bg-green-500/10 border-green-500/30'
    if (score >= 60) return 'bg-yellow-500/10 border-yellow-500/30'
    return 'bg-red-500/10 border-red-500/30'
  }

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
          <h2 className="text-2xl font-bold text-white">{t('compare.title')}</h2>
        </div>

        {/* Test type selector */}
        <div className="card mb-6">
          <label className="block text-sm text-slate-400 mb-2">{t('tests.name')}</label>
          <select
            value={testType}
            onChange={(e) => setTestType(e.target.value)}
            className="w-full bg-white/10 border border-white/20 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500"
          >
            <option value="big_five">Big Five / OCEAN</option>
            <option value="mbti">MBTI</option>
            <option value="enneagram">Enneagrama</option>
            <option value="disc">DISC</option>
            <option value="dark_triad">Triada Oscura</option>
          </select>
        </div>

        {/* Input areas */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
          <div className="card">
            <h3 className="text-lg font-bold text-white mb-2">{t('compare.result_a')}</h3>
            <p className="text-xs text-slate-500 mb-3">{t('compare.paste_hint')}</p>
            <textarea
              value={resultA}
              onChange={(e) => setResultA(e.target.value)}
              placeholder={t('compare.paste_a')}
              rows={12}
              className="w-full bg-black/30 border border-white/10 rounded-xl px-4 py-3 text-white font-mono text-sm focus:outline-none focus:border-indigo-500 resize-y"
            />
          </div>
          <div className="card">
            <h3 className="text-lg font-bold text-white mb-2">{t('compare.result_b')}</h3>
            <p className="text-xs text-slate-500 mb-3">{t('compare.paste_hint')}</p>
            <textarea
              value={resultB}
              onChange={(e) => setResultB(e.target.value)}
              placeholder={t('compare.paste_b')}
              rows={12}
              className="w-full bg-black/30 border border-white/10 rounded-xl px-4 py-3 text-white font-mono text-sm focus:outline-none focus:border-indigo-500 resize-y"
            />
          </div>
        </div>

        {/* Submit */}
        <div className="text-center mb-8">
          <button
            onClick={handleCompare}
            disabled={loading || !resultA.trim() || !resultB.trim()}
            className="btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? t('compare.comparing') : t('compare.submit')}
          </button>
        </div>

        {/* Error */}
        {error && (
          <div className="card border-red-500/30 bg-red-500/10 mb-6">
            <p className="text-red-400">{error}</p>
          </div>
        )}

        {/* Results */}
        {compareResult && (
          <div className="space-y-6">
            {/* Compatibility score */}
            <div className={`card text-center border ${compatibilityBg(compareResult.compatibility)}`}>
              <h3 className="text-lg text-slate-300 mb-2">{t('compare.compatibility')}</h3>
              <div className={`text-5xl font-black ${compatibilityColor(compareResult.compatibility)}`}>
                {compareResult.compatibility.toFixed(1)}%
              </div>
            </div>

            {/* Summary */}
            {compareResult.summary && (
              <div className="card">
                <h4 className="text-lg font-bold text-white mb-2">{t('compare.summary')}</h4>
                <p className="text-slate-300 leading-relaxed">{compareResult.summary}</p>
              </div>
            )}

            {/* Per-factor comparison */}
            {compareResult.details && (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {Object.entries(compareResult.details).map(([factor, detail]) => (
                  <div
                    key={factor}
                    className={`card ${detail.compatible ? 'border-green-500/20' : 'border-red-500/20'}`}
                  >
                    <h4 className="text-lg font-bold text-white mb-3">{factor}</h4>
                    <div className="space-y-2 text-sm">
                      <div className="flex justify-between">
                        <span className="text-slate-400">{t('compare.result_a')}</span>
                        <span className="text-white font-mono">{detail.score_a.toFixed(1)}</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-slate-400">{t('compare.result_b')}</span>
                        <span className="text-white font-mono">{detail.score_b.toFixed(1)}</span>
                      </div>
                      <div className="border-t border-white/10 pt-2 flex justify-between">
                        <span className="text-slate-400">{t('compare.difference')}</span>
                        <span className="font-mono">{detail.difference.toFixed(1)}</span>
                      </div>
                      <div className="pt-1">
                        <span
                          className={`text-xs font-bold ${
                            detail.compatible ? 'text-green-400' : 'text-red-400'
                          }`}
                        >
                          {detail.compatible ? t('compare.compatible') : t('compare.incompatible')}
                        </span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  )
}
