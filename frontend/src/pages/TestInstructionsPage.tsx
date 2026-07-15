import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import type { TestMetadata } from '../types'

export function TestInstructionsPage() {
  const { testType } = useParams<{ testType: string }>()
  const navigate = useNavigate()
  const { t, i18n } = useTranslation()

  const [metadata, setMetadata] = useState<TestMetadata | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [consentAccepted, setConsentAccepted] = useState(false)

  useEffect(() => {
    const fetchMetadata = async () => {
      try {
        const response = await fetch(`/api/v1/tests/${testType}/metadata?lang=${i18n.language}`)
        if (!response.ok) throw new Error('Failed to load metadata')
        const data = await response.json()
        setMetadata(data)
      } catch (err) {
        setError(err instanceof Error ? err.message : t('app.error'))
      } finally {
        setLoading(false)
      }
    }
    fetchMetadata()
  }, [testType, i18n.language, t])

  const handleStart = () => {
    navigate(`/test/${testType}`)
  }

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="w-12 h-12 border-4 border-indigo-500 border-t-transparent rounded-full animate-spin mx-auto mb-4" />
          <p className="text-slate-400">{t('app.loading')}</p>
        </div>
      </div>
    )
  }

  if (error || !metadata) {
    return (
      <div className="min-h-screen flex items-center justify-center px-4">
        <div className="card text-center max-w-md w-full">
          <div className="text-4xl mb-4">⚠️</div>
          <h2 className="text-xl font-bold text-white mb-2">{t('app.error')}</h2>
          <p className="text-slate-400 mb-6">{error || t('app.error')}</p>
          <button onClick={() => navigate('/')} className="btn-primary">
            ← {t('app.back')}
          </button>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen px-4 py-8">
      <div className="max-w-2xl mx-auto">
        {/* Back button */}
        <button
          onClick={() => navigate('/')}
          className="text-slate-400 hover:text-white transition-colors mb-8"
        >
          ← {t('app.back')}
        </button>

        {/* Title */}
        <h2 className="text-3xl font-bold gradient-text mb-8">
          {t('instructions.title')}
        </h2>

        {/* Scientific Basis */}
        <div className="card mb-6">
          <h3 className="text-lg font-bold text-white mb-3 flex items-center gap-2">
            <span className="w-8 h-8 rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-sm">🔬</span>
            {t('instructions.scientific_basis')}
          </h3>
          <p className="text-slate-300 leading-relaxed whitespace-pre-line">
            {metadata.scientific_basis}
          </p>
        </div>

        {/* Instructions */}
        <div className="card mb-6">
          <h3 className="text-lg font-bold text-white mb-3 flex items-center gap-2">
            <span className="w-8 h-8 rounded-lg bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center text-sm">📋</span>
            {t('instructions.instructions')}
          </h3>
          <p className="text-slate-300 leading-relaxed whitespace-pre-line">
            {metadata.instructions}
          </p>
        </div>

        {/* Consent */}
        <div className="card mb-8">
          <label className="flex items-start gap-3 cursor-pointer">
            <input
              type="checkbox"
              checked={consentAccepted}
              onChange={(e) => setConsentAccepted(e.target.checked)}
              className="mt-1 w-5 h-5 rounded border-white/20 bg-white/5 text-indigo-500 
                         focus:ring-indigo-500 focus:ring-offset-0 cursor-pointer
                         accent-indigo-500"
            />
            <span className="text-slate-300 leading-relaxed text-sm">
              {metadata.consent || t('instructions.consent')}
            </span>
          </label>
        </div>

        {/* Actions */}
        <div className="flex flex-col sm:flex-row gap-4">
          <button
            onClick={handleStart}
            disabled={!consentAccepted}
            className="btn-primary flex-1 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {t('instructions.start')}
          </button>
          <button
            onClick={() => navigate('/')}
            className="btn-secondary flex-1"
          >
            {t('app.back')}
          </button>
        </div>

        {!consentAccepted && (
          <p className="text-amber-400 text-sm mt-4 text-center">
            {t('instructions.accept_consent')}
          </p>
        )}
      </div>
    </div>
  )
}
