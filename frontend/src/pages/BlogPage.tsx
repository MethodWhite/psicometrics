import { useMemo } from 'react'
import { Link, useSearchParams } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'
import { useTranslation } from 'react-i18next'
import { blogArticles, BLOG_CATEGORIES, getFeaturedArticles } from '../data/blog-articles'
import { getTagBySlug } from '../data/taxonomy'

const PER_PAGE = 9
const POPULAR_TAGS = ['openness', 'conscientiousness', 'extraversion', 'intj', 'infp', 'type-4', 'enneagram', 'relationships', 'career', 'mindfulness', 'leadership', 'productivity']

export default function BlogPage() {
  const { t } = useTranslation()
  const [searchParams, setSearchParams] = useSearchParams()

  const activeCategory = searchParams.get('category') || null
  const activeTags = searchParams.getAll('tag')
  const page = parseInt(searchParams.get('page') || '1', 10)
  const showTagFilter = searchParams.get('tags') === '1'

  const setParam = (key: string, value: string | null) => {
    setSearchParams(prev => {
      const next = new URLSearchParams(prev)
      if (value) next.set(key, value); else next.delete(key)
      next.set('page', '1')
      return next
    })
  }

  const toggleTag = (slug: string) => {
    setSearchParams(prev => {
      const next = new URLSearchParams(prev)
      const tags = next.getAll('tag')
      if (tags.includes(slug)) {
        next.delete('tag')
        tags.filter(t => t !== slug).forEach(t => next.append('tag', t))
      } else {
        next.append('tag', slug)
      }
      next.set('page', '1')
      return next
    })
  }

  const clearTag = (slug: string) => {
    setSearchParams(prev => {
      const next = new URLSearchParams(prev)
      next.delete('tag')
      const remaining = activeTags.filter(t => t !== slug)
      remaining.forEach(t => next.append('tag', t))
      next.set('page', '1')
      return next
    })
  }

  const filtered = useMemo(() => {
    let result = activeCategory
      ? blogArticles.filter(a => a.category === activeCategory)
      : [...blogArticles]

    if (activeTags.length > 0) {
      result = result.filter(a =>
        activeTags.some(tagSlug => {
          const tag = getTagBySlug(tagSlug)
          return tag && a.tags.some(t => t.toLowerCase() === tag.name.toLowerCase())
        })
      )
    }

    return result
  }, [activeCategory, activeTags])

  const totalPages = Math.max(1, Math.ceil(filtered.length / PER_PAGE))
  const clampedPage = Math.min(page, totalPages)
  const displayArticles = filtered.slice((clampedPage - 1) * PER_PAGE, clampedPage * PER_PAGE)

  const featured = useMemo(() => {
    if (activeCategory || activeTags.length > 0) return []
    return getFeaturedArticles()
  }, [activeCategory, activeTags])

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('es-ES', { year: 'numeric', month: 'long', day: 'numeric' })
  }

  const goToPage = (p: number) => {
    setSearchParams(prev => {
      const next = new URLSearchParams(prev)
      next.set('page', String(p))
      return next
    })
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }

  const getPageNumbers = (): (number | '...')[] => {
    const pages: (number | '...')[] = []
    const maxVisible = 5
    if (totalPages <= maxVisible + 2) {
      for (let i = 1; i <= totalPages; i++) pages.push(i)
    } else {
      pages.push(1)
      let start = Math.max(2, clampedPage - 1)
      let end = Math.min(totalPages - 1, clampedPage + 1)
      if (clampedPage <= 3) { start = 2; end = Math.min(totalPages - 1, maxVisible) }
      if (clampedPage >= totalPages - 2) { start = Math.max(2, totalPages - maxVisible + 1); end = totalPages - 1 }
      if (start > 2) pages.push('...')
      for (let i = start; i <= end; i++) pages.push(i)
      if (end < totalPages - 1) pages.push('...')
      pages.push(totalPages)
    }
    return pages
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
              {blogArticles.length} artículos basados en evidencia científica sobre
              Big Five, MBTI, Eneagrama y más. Aprende cómo funciona tu personalidad.
            </p>
          </div>

          {/* Active filters bar */}
          {activeTags.length > 0 && (
            <div className="mb-6 flex flex-wrap items-center gap-2 p-3 bg-indigo-500/5 rounded-xl border border-indigo-500/10">
              <span className="text-xs font-medium text-content-muted mr-1">Filtrado por:</span>
              {activeTags.map(slug => {
                const tag = getTagBySlug(slug)
                return (
                  <span key={slug} className="inline-flex items-center gap-1 px-2.5 py-1 bg-indigo-500/15 text-indigo-400 rounded-full text-xs font-medium">
                    {tag?.name || slug}
                    <button onClick={() => clearTag(slug)} className="hover:text-white transition-colors ml-0.5" aria-label={`Quitar filtro ${tag?.name || slug}`}>
                      ✕
                    </button>
                  </span>
                )
              })}
              <button onClick={() => setSearchParams(prev => { prev.delete('tag'); prev.set('page', '1'); return prev })} className="text-xs text-content-muted hover:text-content ml-2 underline">
                Limpiar
              </button>
            </div>
          )}

          {/* Featured */}
          {featured.length > 0 && (
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

          {/* Filters bar */}
          <div className="space-y-4 mb-8">
            {/* Category pills */}
            <div className="flex flex-wrap gap-2">
              <button
                onClick={() => { setParam('category', null) }}
                className={`px-3 py-1.5 rounded-full text-xs font-medium transition-all ${
                  !activeCategory && activeTags.length === 0
                    ? 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/30'
                    : 'bg-surface-secondary text-content-muted border border-border hover:border-border-hover'
                }`}
              >
                Todos
              </button>
              {BLOG_CATEGORIES.map(cat => (
                <button
                  key={cat.id}
                  onClick={() => setParam('category', cat.id)}
                  className={`px-3 py-1.5 rounded-full text-xs font-medium transition-all flex items-center gap-1.5 ${
                    activeCategory === cat.id
                      ? 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/30'
                      : 'bg-surface-secondary text-content-muted border border-border hover:border-border-hover'
                  }`}
                >
                  <span>{cat.icon}</span>
                  {cat.name}
                </button>
              ))}

              {/* Tag filter toggle */}
              <button
                onClick={() => setSearchParams(prev => { prev.set('tags', showTagFilter ? '0' : '1'); return prev })}
                className={`px-3 py-1.5 rounded-full text-xs font-medium transition-all border ${
                  showTagFilter
                    ? 'bg-purple-500/20 text-purple-400 border-purple-500/30'
                    : 'bg-surface-secondary text-content-muted border-border hover:border-border-hover'
                }`}
              >
                🏷️ Tags
              </button>
            </div>

            {/* Tag chips (collapsible) */}
            {showTagFilter && (
              <div className="p-4 bg-surface-secondary rounded-xl border border-border">
                <div className="flex flex-wrap gap-1.5">
                  {POPULAR_TAGS.map(slug => {
                    const tag = getTagBySlug(slug)
                    if (!tag) return null
                    const isActive = activeTags.includes(slug)
                    return (
                      <button
                        key={slug}
                        onClick={() => toggleTag(slug)}
                        className={`px-2.5 py-1 rounded-full text-xs font-medium transition-all ${
                          isActive
                            ? 'bg-purple-500/20 text-purple-400 border border-purple-500/30'
                            : 'bg-surface text-content-muted border border-border hover:border-border-hover hover:text-content'
                        }`}
                      >
                        {tag.name}
                      </button>
                    )
                  })}
                </div>
              </div>
            )}
          </div>

          {/* Results info */}
          <div className="flex items-center justify-between mb-4">
            <span className="text-xs text-content-muted">
              {filtered.length} artículo{filtered.length !== 1 && 's'}
              {activeCategory && ` en ${getCategoryName(activeCategory)}`}
              {activeTags.length > 0 && ` (filtrado por ${activeTags.length} tag${activeTags.length !== 1 ? 's' : ''})`}
            </span>
          </div>

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
                      {article.tags.length > 0 && (
                        <div className="mt-2 flex flex-wrap gap-1">
                          {article.tags.slice(0, 3).map(tag => (
                            <span key={tag} className="px-1.5 py-0.5 bg-indigo-500/8 text-indigo-400/80 rounded text-[9px] font-medium">
                              {tag}
                            </span>
                          ))}
                          {article.tags.length > 3 && (
                            <span className="text-[9px] text-content-muted">+{article.tags.length - 3}</span>
                          )}
                        </div>
                      )}
                    </div>
                  </Link>
                ))}
              </div>
            ) : (
              <div className="text-center py-16">
                <span className="text-5xl mb-4 block">🔍</span>
                <h3 className="text-xl font-bold text-content mb-2">Sin resultados</h3>
                <p className="text-content-muted mb-6">
                  No hay artículos que coincidan con {activeCategory && `la categoría y `}los tags seleccionados.
                </p>
                <button onClick={() => setSearchParams(new URLSearchParams())} className="btn-primary inline-block">
                  Limpiar filtros
                </button>
              </div>
            )}
          </section>

          {/* Pagination */}
          {totalPages > 1 && (
            <nav className="mt-10 flex flex-wrap items-center justify-center gap-1.5" aria-label="Paginación">
              <button
                onClick={() => goToPage(clampedPage - 1)}
                disabled={clampedPage === 1}
                className="px-3 py-2 rounded-lg text-sm font-medium bg-surface-secondary text-content-muted disabled:opacity-30 disabled:cursor-not-allowed hover:text-content transition-colors"
                aria-label="Página anterior"
              >←</button>
              {getPageNumbers().map((p, i) =>
                p === '...' ? (
                  <span key={`e${i}`} className="px-2 py-2 text-content-muted text-sm">...</span>
                ) : (
                  <button
                    key={p}
                    onClick={() => goToPage(p)}
                    className={`min-w-[40px] px-3 py-2 rounded-lg text-sm font-medium transition-all ${
                      clampedPage === p
                        ? 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/30'
                        : 'bg-surface-secondary text-content-muted hover:text-content border border-transparent'
                    }`}
                    aria-current={clampedPage === p ? 'page' : undefined}
                  >{p}</button>
                )
              )}
              <button
                onClick={() => goToPage(clampedPage + 1)}
                disabled={clampedPage === totalPages}
                className="px-3 py-2 rounded-lg text-sm font-medium bg-surface-secondary text-content-muted disabled:opacity-30 disabled:cursor-not-allowed hover:text-content transition-colors"
                aria-label="Página siguiente"
              >→</button>
            </nav>
          )}

          {totalPages > 1 && (
            <p className="text-center text-xs text-content-muted mt-4">
              Página {clampedPage} de {totalPages} · {filtered.length} artículo{filtered.length !== 1 && 's'}
            </p>
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
