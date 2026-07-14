import { useState } from 'react'
import { useTranslation } from 'react-i18next'

interface HumanDesignFormProps {
  onSubmit: (data: { birth_date: string; birth_time: string; birth_location: string }) => void
  loading?: boolean
}

export function HumanDesignForm({ onSubmit, loading }: HumanDesignFormProps) {
  const { t } = useTranslation()
  const [birthDate, setBirthDate] = useState('')
  const [birthTime, setBirthTime] = useState('')
  const [birthLocation, setBirthLocation] = useState('')

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    onSubmit({
      birth_date: birthDate,
      birth_time: birthTime,
      birth_location: birthLocation,
    })
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div>
        <label className="block text-sm font-medium text-slate-300 mb-2">
          {t('human_design.birth_date', 'Fecha de nacimiento')}
        </label>
        <input
          type="date"
          value={birthDate}
          onChange={(e) => setBirthDate(e.target.value)}
          required
          className="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white
                     focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
        />
      </div>

      <div>
        <label className="block text-sm font-medium text-slate-300 mb-2">
          {t('human_design.birth_time', 'Hora de nacimiento')}
        </label>
        <input
          type="time"
          value={birthTime}
          onChange={(e) => setBirthTime(e.target.value)}
          required
          className="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white
                     focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
        />
        <p className="text-xs text-slate-500 mt-1">
          {t('human_design.time_note', 'La hora exacta es importante para un cálculo preciso')}
        </p>
      </div>

      <div>
        <label className="block text-sm font-medium text-slate-300 mb-2">
          {t('human_design.birth_location', 'Lugar de nacimiento')}
        </label>
        <input
          type="text"
          value={birthLocation}
          onChange={(e) => setBirthLocation(e.target.value)}
          placeholder={t('human_design.location_placeholder', 'Ciudad, País')}
          required
          className="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500
                     focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
        />
      </div>

      <button
        type="submit"
        disabled={loading || !birthDate || !birthTime || !birthLocation}
        className="w-full btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {loading ? t('app.loading') : t('app.submit')}
      </button>
    </form>
  )
}
