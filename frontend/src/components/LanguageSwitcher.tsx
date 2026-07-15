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
      className="px-3 py-1.5 rounded-lg bg-surface border border-border text-sm font-medium text-content
                 hover:bg-surface-secondary transition-all duration-200"
    >
      {i18n.language === 'es' ? 'EN' : 'ES'}
    </button>
  )
}
