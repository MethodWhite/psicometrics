import { useParams, Link } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'
import { TAG_CATEGORIES, getTagsByCategory, getTagBySlug, getRelatedTags } from '../data/taxonomy'
import { blogArticles } from '../data/blog-articles'
import type { TagCategory } from '../data/taxonomy'

export default function TagsPage() {
  const { slug } = useParams<{ slug: string }>()

  // Single tag view
  if (slug) {
    const tag = getTagBySlug(slug)
    if (!tag) {
      return (
        <div className="min-h-screen px-4 py-24 text-center">
          <span className="text-6xl block mb-4">🔍</span>
          <h1 className="text-2xl font-bold text-content mb-2">Tag no encontrado</h1>
          <p className="text-content-muted mb-6">El tag que buscas no existe.</p>
          <Link to="/tags" className="btn-primary inline-block">Ver todos los tags</Link>
        </div>
      )
    }

    const relatedTags = getRelatedTags(slug)
    const articles = blogArticles.filter(a =>
      a.tags.some(t => t.toLowerCase() === tag.name.toLowerCase() || t.toLowerCase().includes(tag.slug.toLowerCase()))
    )
    const catInfo = TAG_CATEGORIES.find(c => c.id === tag.category)

    return (
      <>
        <Helmet>
          <title>{tag.name} — PsicoMetrics</title>
          <meta name="description" content={tag.description} />
        </Helmet>

        <div className="min-h-screen px-4 py-8">
          <div className="max-w-4xl mx-auto">
            <div className="mb-6 flex items-center gap-2 text-sm text-content-muted">
              <Link to="/" className="hover:text-content transition-colors">Inicio</Link>
              <span>/</span>
              <Link to="/tags" className="hover:text-content transition-colors">Tags</Link>
              <span>/</span>
              <span className="text-content">{tag.name}</span>
            </div>

            <div className="card mb-8">
              <div className="flex items-center gap-4 mb-4">
                <span className="text-4xl">{catInfo?.icon || '🏷️'}</span>
                <div>
                  <h1 className="text-2xl font-bold text-content">{tag.name}</h1>
                  <span className="text-sm text-content-muted">{catInfo?.name || tag.category}</span>
                </div>
              </div>
              <p className="text-content-secondary leading-relaxed">{tag.description}</p>
            </div>

            {/* Related tags */}
            {relatedTags.length > 0 && (
              <div className="mb-8">
                <h2 className="text-lg font-bold text-content mb-3">Tags relacionados</h2>
                <div className="flex flex-wrap gap-2">
                  {relatedTags.map(rt => (
                    <Link
                      key={rt.slug}
                      to={`/tags/${rt.slug}`}
                      className="px-3 py-1 bg-surface-secondary rounded-full text-sm text-content-muted hover:text-content hover:bg-indigo-500/10 transition-colors"
                    >
                      {rt.name}
                    </Link>
                  ))}
                </div>
              </div>
            )}

            {/* Articles with this tag */}
            <h2 className="text-lg font-bold text-content mb-4">
              Artículos relacionados ({articles.length})
            </h2>
            {articles.length > 0 ? (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {articles.map(article => (
                  <Link
                    key={article.slug}
                    to={`/blog/${article.slug}`}
                    className="card group"
                  >
                    <span className="text-xs font-medium text-indigo-400 uppercase tracking-wider">
                      {article.category}
                    </span>
                    <h3 className="text-base font-bold text-content mt-1 mb-2 group-hover:text-indigo-400 transition-colors">
                      {article.title}
                    </h3>
                    <p className="text-sm text-content-secondary line-clamp-2">{article.excerpt}</p>
                  </Link>
                ))}
              </div>
            ) : (
              <p className="text-content-muted">No hay artículos con este tag todavía.</p>
            )}

            {/* Browse all tags link */}
            <div className="mt-8 text-center">
              <Link to="/tags" className="btn-secondary inline-block">
                Explorar todos los tags →
              </Link>
            </div>
          </div>
        </div>
      </>
    )
  }

  // All tags view
  return (
    <>
      <Helmet>
        <title>Tags de Personalidad — PsicoMetrics</title>
        <meta name="description" content="Explora todos los tags y categorías de psicología de la personalidad en PsicoMetrics." />
      </Helmet>

      <div className="min-h-screen px-4 py-8">
        <div className="max-w-4xl mx-auto">
          <div className="mb-6">
            <Link to="/" className="text-content-muted hover:text-content transition-colors text-sm inline-block mb-4">
              ← Volver al inicio
            </Link>
            <h1 className="text-3xl font-bold text-content mb-3">Tags de Personalidad</h1>
            <p className="text-content-secondary text-lg">
              Explora todos los conceptos de psicología de la personalidad organizados por categorías.
            </p>
          </div>

          <div className="space-y-8">
            {TAG_CATEGORIES.map(cat => {
              const tags = getTagsByCategory(cat.id as TagCategory)
              if (tags.length === 0) return null
              return (
                <section key={cat.id}>
                  <h2 className="text-xl font-bold text-content mb-4 flex items-center gap-2">
                    <span>{cat.icon}</span>
                    {cat.name}
                  </h2>
                  <div className="flex flex-wrap gap-2">
                    {tags.map(tag => (
                      <Link
                        key={tag.slug}
                        to={`/tags/${tag.slug}`}
                        className="px-3 py-1.5 bg-surface-secondary rounded-full text-sm text-content-muted hover:text-content hover:bg-indigo-500/10 border border-border hover:border-indigo-500/30 transition-all"
                      >
                        {tag.name}
                      </Link>
                    ))}
                  </div>
                </section>
              )
            })}
          </div>
        </div>
      </div>
    </>
  )
}
