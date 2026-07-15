import { useState } from 'react'
import { Link } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'
import { quizzes } from '../data/quizzes'
import { TAG_CATEGORIES } from '../data/taxonomy'

const QUIZ_CATEGORY_ICONS: Record<string, string> = {
  'love-languages': '💝',
  'big-five': '🧠',
  'enneagram': '🔵',
  'self-improvement': '🌱',
  'emotional-intelligence': '💎',
  'health': '🏥',
  'education': '📚',
}

export default function QuizzesPage() {
  const [activeCategory, setActiveCategory] = useState<string | null>(null)

  const filtered = activeCategory
    ? quizzes.filter(q => q.category === activeCategory)
    : quizzes

  return (
    <>
      <Helmet>
        <title>Tests Rápidos de Personalidad — PsicoMetrics</title>
        <meta name="description" content="Tests rápidos y divertidos de personalidad: lenguaje del amor, introversión, eneagrama, inteligencia emocional y más." />
        <meta property="og:title" content="Tests Rápidos de Personalidad — PsicoMetrics" />
        <meta property="og:description" content="Descubre aspectos divertidos de tu personalidad con nuestros tests rápidos." />
      </Helmet>

      <div className="min-h-screen px-4 py-8">
        <div className="max-w-5xl mx-auto">
          {/* Header */}
          <div className="mb-8">
            <Link to="/" className="text-content-muted hover:text-content transition-colors text-sm mb-4 inline-block">
              ← Volver al inicio
            </Link>
            <h1 className="text-3xl font-bold text-content mb-3">Tests Rápidos</h1>
            <p className="text-content-secondary text-lg leading-relaxed">
              Descubre aspectos divertidos de tu personalidad con estos tests rápidos.
              Solo toma 2-5 minutos cada uno y no necesitas crear una cuenta.
            </p>
          </div>

          {/* Category Filters */}
          <section className="mb-8">
            <div className="flex flex-wrap gap-3">
              <button
                onClick={() => setActiveCategory(null)}
                className={`px-4 py-2 rounded-full text-sm font-medium transition-all ${
                  !activeCategory
                    ? 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/30'
                    : 'bg-surface-secondary text-content-muted border border-border hover:border-border-hover'
                }`}
              >
                Todos
              </button>
              {Array.from(new Set(quizzes.map(q => q.category))).map(cat => {
                const catInfo = TAG_CATEGORIES.find(tc => tc.id === cat)
                return (
                  <button
                    key={cat}
                    onClick={() => setActiveCategory(cat)}
                    className={`px-4 py-2 rounded-full text-sm font-medium transition-all flex items-center gap-2 ${
                      activeCategory === cat
                        ? 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/30'
                        : 'bg-surface-secondary text-content-muted border border-border hover:border-border-hover'
                    }`}
                  >
                    <span>{catInfo?.icon || QUIZ_CATEGORY_ICONS[cat] || '📝'}</span>
                    {catInfo?.name || cat}
                  </button>
                )
              })}
            </div>
          </section>

          {/* Quiz Grid */}
          <section>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {filtered.map(quiz => (
                <Link
                  key={quiz.slug}
                  to={`/quiz/${quiz.slug}`}
                  className="card group !p-0 overflow-hidden hover:border-indigo-500/30 transition-all duration-300"
                >
                  <div className="h-32 bg-gradient-to-br from-indigo-600/20 to-purple-600/20 flex items-center justify-center">
                    <span className="text-5xl">{quiz.icon}</span>
                  </div>
                  <div className="p-5">
                    <span className="text-xs font-medium text-indigo-400 uppercase tracking-wider">
                      {quiz.category === 'love-languages' ? 'Lenguajes del Amor' :
                       quiz.category === 'big-five' ? 'Big Five' :
                       quiz.category === 'enneagram' ? 'Eneagrama' :
                       quiz.category === 'self-improvement' ? 'Auto-mejora' :
                       quiz.category === 'emotional-intelligence' ? 'Inteligencia Emocional' :
                       quiz.category === 'health' ? 'Salud' :
                       quiz.category === 'education' ? 'Educación' : quiz.category}
                    </span>
                    <h3 className="text-lg font-bold text-content mt-1 mb-2 group-hover:text-indigo-400 transition-colors">
                      {quiz.title}
                    </h3>
                    <p className="text-sm text-content-secondary leading-relaxed line-clamp-2">{quiz.description}</p>
                    <div className="mt-3 flex items-center gap-2 text-xs text-content-muted">
                      <span>{quiz.questions.length} preguntas</span>
                      <span>·</span>
                      <span>{quiz.results.length} resultados</span>
                    </div>
                  </div>
                </Link>
              ))}
            </div>
          </section>

          {/* Empty state */}
          {filtered.length === 0 && (
            <div className="text-center py-12">
              <span className="text-4xl mb-4 block">🔍</span>
              <p className="text-content-muted">No hay tests en esta categoría todavía.</p>
            </div>
          )}

          {/* CTA */}
          <section className="mt-12 p-8 card text-center bg-gradient-to-br from-indigo-600/10 to-purple-600/10 border-indigo-500/20">
            <span className="text-4xl mb-4 block">🧠</span>
            <h2 className="text-xl font-bold text-content mb-2">¿Quieres un análisis más profundo?</h2>
            <p className="text-content-secondary mb-6">
              Realiza nuestros tests científicos completos con más de 50 preguntas cada uno.
            </p>
            <Link to="/" className="btn-primary inline-block">
              Tests completos →
            </Link>
          </section>
        </div>
      </div>
    </>
  )
}
