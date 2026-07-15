import i18n from 'i18next'
import { initReactI18next } from 'react-i18next'
import es from './es.json'
import en from './en.json'
import pt from './pt.json'
import fr from './fr.json'
import de from './de.json'
import it from './it.json'
import nl from './nl.json'
import pl from './pl.json'
import ru from './ru.json'
import ja from './ja.json'
import ko from './ko.json'
import zh from './zh.json'
import ar from './ar.json'
import hi from './hi.json'
import tr from './tr.json'
import sv from './sv.json'
import da from './da.json'
import fi from './fi.json'
import no from './no.json'
import cs from './cs.json'

i18n.use(initReactI18next).init({
  resources: {
    es: { translation: es },
    en: { translation: en },
    pt: { translation: pt },
    fr: { translation: fr },
    de: { translation: de },
    it: { translation: it },
    nl: { translation: nl },
    pl: { translation: pl },
    ru: { translation: ru },
    ja: { translation: ja },
    ko: { translation: ko },
    zh: { translation: zh },
    ar: { translation: ar },
    hi: { translation: hi },
    tr: { translation: tr },
    sv: { translation: sv },
    da: { translation: da },
    fi: { translation: fi },
    no: { translation: no },
    cs: { translation: cs },
  },
  lng: 'es',
  fallbackLng: 'es',
  interpolation: {
    escapeValue: false,
  },
})

export default i18n
