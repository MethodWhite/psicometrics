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
import { ForumPage } from './pages/ForumPage'
import { ForumPostPage } from './pages/ForumPostPage'
import { TestimonialsPage } from './pages/TestimonialsPage'
import { StoriesPage } from './pages/StoriesPage'
import TypeHubPage from './pages/TypeHubPage'
import TypeDetailPage from './pages/TypeDetailPage'
import EncyclopediaPage from './pages/EncyclopediaPage'
import TypeComparisonPage from './pages/TypeComparisonPage'
import ResourcesPage from './pages/ResourcesPage'
import TypeResourcesPage from './pages/TypeResourcesPage'
import QuizzesPage from './pages/QuizzesPage'
import QuizPage from './pages/QuizPage'
import TagsPage from './pages/TagsPage'
import { NAVIGATION } from './data/navigation'
import { useState } from 'react'

function NavItem({ item }: { item: typeof NAVIGATION[number] }) {
  const [showChildren, setShowChildren] = useState(false)

  if (item.children) {
    return (
      <div className="relative" onMouseEnter={() => setShowChildren(true)} onMouseLeave={() => setShowChildren(false)}>
        <a
          href={item.path}
          className="text-sm text-content-muted hover:text-content transition-colors flex items-center gap-1"
        >
          {item.icon && <span>{item.icon}</span>}
          {item.label}
          <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
          </svg>
        </a>
        {showChildren && (
          <div className="absolute top-full left-0 mt-1 bg-surface border border-border rounded-xl shadow-lg py-2 min-w-[160px] z-50 animate-fadeIn">
            {item.children.map(child => (
              <a
                key={child.path}
                href={child.path}
                className="block px-4 py-2 text-sm text-content-muted hover:text-content hover:bg-surface-secondary transition-colors"
              >
                {child.label}
              </a>
            ))}
          </div>
        )}
      </div>
    )
  }

  return (
    <a
      href={item.path}
      className="text-sm text-content-muted hover:text-content transition-colors flex items-center gap-1"
    >
      {item.icon && <span>{item.icon}</span>}
      {item.label}
    </a>
  )
}

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
            <div className="hidden md:flex items-center gap-4">
              {NAVIGATION.map(item => (
                <NavItem key={item.path} item={item} />
              ))}
            </div>
            <div className="flex items-center gap-3">
              <a href="/account" className="text-sm text-content-muted hover:text-content transition-colors hidden sm:block">
                Account
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
            <Route path="/forum" element={<ForumPage />} />
            <Route path="/forum/:id" element={<ForumPostPage />} />
            <Route path="/testimonials" element={<TestimonialsPage />} />
            <Route path="/stories" element={<StoriesPage />} />
            <Route path="/resources" element={<ResourcesPage />} />
            <Route path="/resources/:type" element={<TypeResourcesPage />} />
            <Route path="/quiz" element={<QuizzesPage />} />
            <Route path="/quiz/:slug" element={<QuizPage />} />
            <Route path="/tags" element={<TagsPage />} />
            <Route path="/tags/:slug" element={<TagsPage />} />
            <Route path="/types" element={<TypeHubPage />} />
            <Route path="/types/:typeCode" element={<TypeDetailPage />} />
            <Route path="/compare-types" element={<TypeComparisonPage />} />
            <Route path="/encyclopedia" element={<EncyclopediaPage />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  )
}
