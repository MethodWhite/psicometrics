import { useState } from 'react'
import { Link } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'
import { useTranslation } from 'react-i18next'
import { blogArticles, BLOG_CATEGORIES, getFeaturedArticles, getPaginatedArticles } from '../data/blog-articles'

export default function BlogPage() {
  const { t } = useTranslation()
  const [activeCategory, setActiveCategory] = useState<string | null>(null)
  const [page, setPage] = useState(1)

  const featured = getFeaturedArticles()

  const filtered = activeCategory
    ? blogArticles.filter(a => a.category === activeCategory)
    : blogArticles

  const { articles, totalPages } = activeCategory
    ? { articles: filtered, totalPages: Math.ceil(filtered.length / 9) }
    : getPaginatedArticles(page)

  const displayArticles = activeCategory ? filtered.slice(0, 9) : articles

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('es-ES', { year: 'numeric', month: 'long', day: 'numeric' })
  }

  return (
    <>
      <Helmet>
        <title>Blog de Psicología y Personalidad — PsicoMetrics</title>
        <meta name="description" content="Artículos sobre psicología de la personalidad: Big Five, MBTI, Eneagrama, relaciones, carrera profesional y crecimiento personal." />
        <meta property="og:title" content="Blog de Psicología y Personalidad — PsicoMetrics" />
        <meta property="og:description" content="Artículos sobre psicología de la personalidad basados en evidencia científica." />
      </Helmet>

      <div className="min-h-screen px-4 py-8">
        <div className="max-w-5xl mx-auto">
          {/* Header */}
          <div className="mb-8">
            <Link to="/" className="text-content-muted hover:text-content transition-colors text-sm mb-4 inline-block">
              ← {t('app.back')}
            </Link>
            <h1 className="text-3xl font-bold text-content mb-3">Blog de Personalidad</h1>
            <p className="text-content-secondary text-lg leading-relaxed">
              Artículos basados en evidencia científica sobre Big Five, MBTI, Eneagrama y más.
              Aprende cómo funciona tu personalidad y cómo aplicarla en tu vida.
            </p>
          </div>

          {/* Featured */}
          {!activeCategory && (
            <section className="mb-12">
              <h2 className="text-xl font-bold text-content mb-6 flex items-center gap-2">
                <span className="text-yellow-400">★</span> Artículos Destacados
              </h2>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {featured.map(article => (
                  <Link key={article.slug} to={`/blog/${article.slug}`} className="card group !p-0 overflow-hidden">
                    <div className="h-40 bg-gradient-to-br from-indigo-600/30 to-purple-600/30 flex items-center justify-center">
                      <span className="text-4xl">{getCategoryIcon(article.category)}</span>
                    </div>
                    <div className="p-5">
                      <span className="text-xs font-medium text-indigo-400 uppercase tracking-wider">
                        {getCategoryName(article.category)}
                      </span>
                      <h3 className="text-lg font-bold text-content mt-1 mb-2 group-hover:text-indigo-400 transition-colors">
                        {article.title}
                      </h3>
                      <p className="text-sm text-content-secondary leading-relaxed line-clamp-3">{article.excerpt}</p>
                      <div className="mt-4 text-xs text-content-muted">{formatDate(article.publishedDate)}</div>
                    </div>
                  </Link>
                ))}
              </div>
            </section>
          )}

          {/* Categories */}
          <section className="mb-8">
            <h2 className="text-xl font-bold text-content mb-4">Categorías</h2>
            <div className="flex flex-wrap gap-3">
              <button
                onClick={() => { setActiveCategory(null); setPage(1) }}
                className={`px-4 py-2 rounded-full text-sm font-medium transition-all ${
                  !activeCategory
                    ? 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/30'
                    : 'bg-surface-secondary text-content-muted border border-border hover:border-border-hover'
                }`}
              >
                Todos
              </button>
              {BLOG_CATEGORIES.map(cat => (
                <button
                  key={cat.id}
                  onClick={() => { setActiveCategory(cat.id); setPage(1) }}
                  className={`px-4 py-2 rounded-full text-sm font-medium transition-all flex items-center gap-2 ${
                    activeCategory === cat.id
                      ? 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/30'
                      : 'bg-surface-secondary text-content-muted border border-border hover:border-border-hover'
                  }`}
                >
                  <span>{cat.icon}</span>
                  {cat.name}
                </button>
              ))}
            </div>
          </section>

          {/* Articles Grid */}
          <section>
            {displayArticles.length > 0 ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {displayArticles.map(article => (
                  <Link key={article.slug} to={`/blog/${article.slug}`} className="card group !p-0 overflow-hidden">
                    <div className="h-32 bg-gradient-to-br from-indigo-500/20 to-purple-500/20 flex items-center justify-center">
                      <span className="text-3xl">{getCategoryIcon(article.category)}</span>
                    </div>
                    <div className="p-5">
                      <span className="text-xs font-medium text-indigo-400 uppercase tracking-wider">
                        {getCategoryName(article.category)}
                      </span>
                      <h3 className="text-base font-bold text-content mt-1 mb-2 group-hover:text-indigo-400 transition-colors">
                        {article.title}
                      </h3>
                      <p className="text-sm text-content-secondary leading-relaxed line-clamp-2">{article.excerpt}</p>
                      <div className="mt-3 flex items-center gap-2 text-xs text-content-muted">
                        <span>{formatDate(article.publishedDate)}</span>
                        <span>·</span>
                        <span>{article.tags.length} tags</span>
                      </div>
                      {article.tags && article.tags.length > 0 && (
                        <div className="mt-3 flex flex-wrap gap-1.5">
                          {article.tags.slice(0, 3).map(tag => (
                            <span key={tag} className="px-2 py-0.5 bg-indigo-500/10 text-indigo-400 rounded text-[10px] font-medium">
                              {tag}
                            </span>
                          ))}
                          {article.tags.length > 3 && (
                            <span className="px-2 py-0.5 text-content-muted rounded text-[10px]">
                              +{article.tags.length - 3}
                            </span>
                          )}
                        </div>
                      )}
                    </div>
                  </Link>
                ))}
              </div>
            ) : (
              <div className="text-center py-12">
                <span className="text-4xl mb-4 block">📚</span>
                <p className="text-content-muted">No hay artículos en esta categoría todavía.</p>
              </div>
            )}
          </section>

          {/* Pagination */}
          {!activeCategory && totalPages > 1 && (
            <div className="mt-8 flex justify-center gap-3">
              <button
                onClick={() => setPage(p => Math.max(1, p - 1))}
                disabled={page === 1}
                className="px-4 py-2 rounded-lg bg-surface-secondary text-content-muted disabled:opacity-40 disabled:cursor-not-allowed hover:text-content transition-colors"
              >
                ← Anterior
              </button>
              <span className="px-4 py-2 text-content-muted">
                Página {page} de {totalPages}
              </span>
              <button
                onClick={() => setPage(p => Math.min(totalPages, p + 1))}
                disabled={page === totalPages}
                className="px-4 py-2 rounded-lg bg-surface-secondary text-content-muted disabled:opacity-40 disabled:cursor-not-allowed hover:text-content transition-colors"
              >
                Siguiente →
              </button>
            </div>
          )}
        </div>
      </div>
    </>
  )
}

function getCategoryIcon(category: string): string {
  const cat = BLOG_CATEGORIES.find(c => c.id === category)
  return cat?.icon || '📝'
}

export function getCategoryName(category: string): string {
  const cat = BLOG_CATEGORIES.find(c => c.id === category)
  return cat?.name || category
}
