import { BrowserRouter, Routes, Route } from 'react-router-dom'
import { LanguageSwitcher } from './components/LanguageSwitcher'
import { Home } from './pages/Home'
import { TestPage } from './pages/TestPage'
import { ResultsPage } from './pages/ResultsPage'

export default function App() {
  return (
    <BrowserRouter>
      <div className="min-h-screen">
        {/* Navigation */}
        <nav className="fixed top-0 left-0 right-0 z-50 bg-black/20 backdrop-blur-xl border-b border-white/10">
          <div className="max-w-5xl mx-auto px-4 h-16 flex items-center justify-between">
            <a href="/" className="text-xl font-bold gradient-text">
              PsicoMetrics
            </a>
            <div className="flex items-center gap-4">
              <LanguageSwitcher />
            </div>
          </div>
        </nav>

        {/* Main content */}
        <main className="pt-16">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/test/:testType" element={<TestPage />} />
            <Route path="/results/:testType" element={<ResultsPage />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  )
}
