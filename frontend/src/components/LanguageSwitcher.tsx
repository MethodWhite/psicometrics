import { useTranslation } from 'react-i18next'

export function LanguageSwitcher() {
  const { i18n } = useTranslation()

  const toggleLanguage = () => {
    const newLang = i18n.language === 'es' ? 'en' : 'es'
    i18n.changeLanguage(newLang)
  }

  return (
    <button
      onClick={toggleLanguage}
      className="px-3 py-1.5 rounded-lg bg-white/10 border border-white/20 text-sm font-medium
                 hover:bg-white/20 transition-all duration-200"
    >
      {i18n.language === 'es' ? 'EN' : 'ES'}
    </button>
  )
}
