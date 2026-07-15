import { BrowserRouter, Routes, Route } from 'react-router-dom'
import { LanguageSwitcher } from './components/LanguageSwitcher'
import { ThemeToggle } from './components/ThemeToggle'
import LandingPage from './pages/LandingPage'
import { TestPage } from './pages/TestPage'
import { ResultsPage } from './pages/ResultsPage'
import { ComparePage } from './pages/ComparePage'
import { AccountPage } from './pages/AccountPage'
import { EvolutionPage } from './pages/EvolutionPage'
import { PublicProfilePage } from './pages/PublicProfilePage'

export default function App() {
  return (
    <BrowserRouter>
      <div className="min-h-screen bg-page">
        {/* Navigation */}
        <nav className="fixed top-0 left-0 right-0 z-50 bg-surface/80 backdrop-blur-xl border-b border-border">
          <div className="max-w-5xl mx-auto px-4 h-16 flex items-center justify-between">
            <a href="/" className="text-xl font-bold gradient-text">
              PsicoMetrics
            </a>
            <div className="flex items-center gap-3">
              <a href="/account" className="text-sm text-content-muted hover:text-content transition-colors">
                Account
              </a>
              <a href="/compare" className="text-sm text-content-muted hover:text-content transition-colors">
                Compare
              </a>
              <a href="/evolution" className="text-sm text-content-muted hover:text-content transition-colors">
                Evolution
              </a>
              <ThemeToggle />
              <LanguageSwitcher />
            </div>
          </div>
        </nav>

        {/* Main content */}
        <main className="pt-16">
          <Routes>
            <Route path="/" element={<LandingPage />} />
            <Route path="/test/:testType" element={<TestPage />} />
            <Route path="/results/:testType" element={<ResultsPage />} />
            <Route path="/compare" element={<ComparePage />} />
            <Route path="/account" element={<AccountPage />} />
            <Route path="/evolution" element={<EvolutionPage />} />
            <Route path="/public/:shareCode" element={<PublicProfilePage />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  )
}
