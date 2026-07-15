import { useState, useEffect, useCallback } from 'react'
import { useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import type { AccountInfo, SavedResult } from '../types'

const ACCOUNT_ID_KEY = 'psicometrics_account_id'

function getStoredAccountId(): string | null {
  return localStorage.getItem(ACCOUNT_ID_KEY)
}

function storeAccountId(id: string) {
  localStorage.setItem(ACCOUNT_ID_KEY, id)
}

export function AccountPage() {
  const navigate = useNavigate()
  const { t } = useTranslation()

  const [email, setEmail] = useState('')
  const [manualAccountId, setManualAccountId] = useState('')
  const [accountId, setAccountId] = useState<string | null>(getStoredAccountId)
  const [accountInfo, setAccountInfo] = useState<AccountInfo | null>(null)
  const [savedResults, setSavedResults] = useState<SavedResult[]>([])
  const [registering, setRegistering] = useState(false)
  const [loadingResults, setLoadingResults] = useState(false)
  const [error, setError] = useState('')
  const [success, setSuccess] = useState('')

  const fetchResults = useCallback(async (id: string) => {
    setLoadingResults(true)
    setError('')
    try {
      const response = await fetch(`/api/v1/accounts/${id}/results`)
      if (!response.ok) throw new Error('Failed to fetch results')
      const data: SavedResult[] = await response.json()
      setSavedResults(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load results')
    } finally {
      setLoadingResults(false)
    }
  }, [])

  useEffect(() => {
    if (accountId) {
      fetchResults(accountId)
    }
  }, [accountId, fetchResults])

  const handleRegister = async () => {
    if (!email.trim()) return
    setRegistering(true)
    setError('')
    setSuccess('')
    try {
      const response = await fetch('/api/v1/accounts/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: email.trim() }),
      })
      if (!response.ok) throw new Error('Registration failed')
      const data: AccountInfo = await response.json()
      setAccountInfo(data)
      setAccountId(data.account_id)
      storeAccountId(data.account_id)
      setSuccess(t('account.register_success'))
      fetchResults(data.account_id)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Registration failed')
    } finally {
      setRegistering(false)
    }
  }

  const handleLoadManual = async () => {
    if (!manualAccountId.trim()) return
    setAccountId(manualAccountId.trim())
    storeAccountId(manualAccountId.trim())
    setSuccess('')
    setError('')
    fetchResults(manualAccountId.trim())
  }

  const handleClearAccount = () => {
    localStorage.removeItem(ACCOUNT_ID_KEY)
    setAccountId(null)
    setAccountInfo(null)
    setSavedResults([])
    setEmail('')
    setError('')
    setSuccess('')
  }

  const handleResultClick = (result: SavedResult) => {
    // We store the result_id so ResultsPage could potentially fetch it
    navigate(`/results/${result.test_type}`, {
      state: { resultId: result.result_id, testType: result.test_type },
    })
  }

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
          <h2 className="text-2xl font-bold text-white">{t('account.title')}</h2>
        </div>

        {/* Error / Success messages */}
        {error && (
          <div className="card border-red-500/30 bg-red-500/10 mb-6">
            <p className="text-red-400">{error}</p>
          </div>
        )}
        {success && (
          <div className="card border-green-500/30 bg-green-500/10 mb-6">
            <p className="text-green-400">{success}</p>
          </div>
        )}

        {!accountId ? (
          <div className="space-y-6">
            {/* Register */}
            <div className="card">
              <h3 className="text-lg font-bold text-white mb-4">{t('account.register')}</h3>
              <div className="flex gap-3">
                <input
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  placeholder={t('account.email_placeholder')}
                  className="flex-1 bg-white/10 border border-white/20 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500"
                />
                <button
                  onClick={handleRegister}
                  disabled={registering || !email.trim()}
                  className="btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {registering ? t('account.registering') : t('account.register')}
                </button>
              </div>
            </div>

            {/* Load existing account */}
            <div className="card">
              <h3 className="text-lg font-bold text-white mb-4">{t('account.your_id')}</h3>
              <div className="flex gap-3">
                <input
                  type="text"
                  value={manualAccountId}
                  onChange={(e) => setManualAccountId(e.target.value)}
                  placeholder={t('account.account_id_placeholder')}
                  className="flex-1 bg-white/10 border border-white/20 rounded-xl px-4 py-3 text-white font-mono text-sm focus:outline-none focus:border-indigo-500"
                />
                <button
                  onClick={handleLoadManual}
                  disabled={!manualAccountId.trim()}
                  className="btn-secondary disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {t('account.load')}
                </button>
              </div>
            </div>
          </div>
        ) : (
          <div className="space-y-6">
            {/* Account info */}
            <div className="card">
              <div className="flex items-start justify-between mb-4">
                <div>
                  <h3 className="text-lg font-bold text-white">{t('account.your_id')}</h3>
                  <p className="text-sm font-mono text-indigo-400 mt-1 break-all">{accountId}</p>
                </div>
                <button
                  onClick={handleClearAccount}
                  className="text-sm text-slate-500 hover:text-red-400 transition-colors"
                >
                  {t('app.back')}
                </button>
              </div>
              {accountInfo && (
                <p className="text-slate-400 text-sm">{accountInfo.email}</p>
              )}
            </div>

            {/* Saved results */}
            <div className="card">
              <h3 className="text-lg font-bold text-white mb-4">{t('account.saved_results')}</h3>
              {loadingResults ? (
                <p className="text-slate-400">{t('app.loading')}</p>
              ) : savedResults.length === 0 ? (
                <p className="text-slate-500">{t('account.no_results')}</p>
              ) : (
                <div className="space-y-2">
                  {savedResults.map((r) => (
                    <button
                      key={r.result_id}
                      onClick={() => handleResultClick(r)}
                      className="w-full text-left p-3 rounded-xl bg-white/5 hover:bg-white/10 border border-white/10 transition-all duration-200"
                    >
                      <div className="flex justify-between items-center">
                        <span className="text-white font-medium capitalize">
                          {r.test_type.replace(/_/g, ' ')}
                        </span>
                        <span className="text-slate-500 text-sm">
                          {new Date(r.created_at).toLocaleDateString()}
                        </span>
                      </div>
                      <div className="text-xs text-slate-600 mt-1 font-mono">{r.result_id}</div>
                    </button>
                  ))}
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
