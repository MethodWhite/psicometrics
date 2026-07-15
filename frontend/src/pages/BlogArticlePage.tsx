import { useParams, Link } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'
import { blogArticles, getRelatedArticles } from '../data/blog-articles'
import { getCategoryName } from './BlogPage'

export default function BlogArticlePage() {
  const { slug } = useParams<{ slug: string }>()
  const article = blogArticles.find(a => a.slug === slug)

  if (!article) {
    return (
      <div className="min-h-screen px-4 py-24 text-center">
        <span className="text-6xl block mb-4">🔍</span>
        <h1 className="text-2xl font-bold text-content mb-2">Artículo no encontrado</h1>
        <p className="text-content-muted mb-6">El artículo que buscas no existe o ha sido eliminado.</p>
        <Link to="/blog" className="btn-primary inline-block">Volver al blog</Link>
      </div>
    )
  }

  const related = getRelatedArticles(article.slug, article.category, 3)

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('es-ES', { year: 'numeric', month: 'long', day: 'numeric' })
  }

  return (
    <>
      <Helmet>
        <title>{article.title} — PsicoMetrics Blog</title>
        <meta name="description" content={article.excerpt} />
        <meta property="og:title" content={article.title} />
        <meta property="og:description" content={article.excerpt} />
        <meta property="og:type" content="article" />
        <meta property="article:published_time" content={article.publishedDate} />
        <meta property="article:author" content={article.author} />
        <meta property="article:section" content={getCategoryName(article.category)} />
        {article.tags.map(tag => (
          <meta key={tag} property="article:tag" content={tag} />
        ))}
      </Helmet>

      <div className="min-h-screen px-4 py-8">
        <article className="max-w-3xl mx-auto">
          {/* Breadcrumb */}
          <div className="mb-6 flex items-center gap-2 text-sm text-content-muted">
            <Link to="/" className="hover:text-content transition-colors">Inicio</Link>
            <span>/</span>
            <Link to="/blog" className="hover:text-content transition-colors">Blog</Link>
            <span>/</span>
            <Link to={`/blog?category=${article.category}`} className="hover:text-content transition-colors">
              {getCategoryName(article.category)}
            </Link>
          </div>

          {/* Header */}
          <header className="mb-8">
            <div className="flex items-center gap-3 mb-4">
              <span className="text-sm font-medium text-indigo-400 uppercase tracking-wider">
                {getCategoryName(article.category)}
              </span>
              <span className="text-content-muted text-sm">{formatDate(article.publishedDate)}</span>
            </div>
            <h1 className="text-3xl md:text-4xl font-bold text-content leading-tight mb-4">{article.title}</h1>
            <p className="text-lg text-content-secondary leading-relaxed">{article.excerpt}</p>
          </header>

          {/* Category Banner */}
          <div className="h-48 md:h-64 rounded-2xl bg-gradient-to-br from-indigo-600/20 to-purple-600/20 flex items-center justify-center mb-8">
            <span className="text-6xl">{getCategoryIcon(article.category)}</span>
          </div>

          {/* Content */}
          <div className="prose-content">
            {article.content.split('\n').map((line, i) => {
              if (line.startsWith('## ')) {
                return (
                  <h2 key={i} className="text-2xl font-bold text-content mt-10 mb-4">
                    {line.replace('## ', '')}
                  </h2>
                )
              }
              if (line.startsWith('### ')) {
                return (
                  <h3 key={i} className="text-xl font-bold text-content mt-8 mb-3">
                    {line.replace('### ', '')}
                  </h3>
                )
              }
              if (line.trim() === '') {
                return <div key={i} className="h-4" />
              }
              return (
                <p key={i} className="text-content-secondary leading-relaxed mb-4 text-base">
                  {line}
                </p>
              )
            })}
          </div>

          {/* Tags */}
          <div className="mt-8 pt-6 border-t border-border">
            <div className="flex flex-wrap gap-2">
              {article.tags.map(tag => (
                <span key={tag} className="px-3 py-1 bg-surface-secondary rounded-full text-xs text-content-muted">
                  #{tag}
                </span>
              ))}
            </div>
          </div>

          {/* Author */}
          <div className="mt-8 p-6 card">
            <div className="flex items-start gap-4">
              <div className="w-14 h-14 rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-white font-bold text-xl flex-shrink-0">
                CM
              </div>
              <div>
                <h3 className="font-bold text-content">{article.author}</h3>
                <p className="text-sm text-content-muted mt-1">
                  Psicóloga clínica especializada en evaluación de la personalidad. Doctora en Psicología por la Universidad Autónoma de Madrid.
                </p>
              </div>
            </div>
          </div>

          {/* CTA */}
          <div className="mt-8 p-8 card text-center bg-gradient-to-br from-indigo-600/10 to-purple-600/10 border-indigo-500/20">
            <span className="text-4xl mb-4 block">🧠</span>
            <h3 className="text-xl font-bold text-content mb-2">Descubre tu tipo de personalidad</h3>
            <p className="text-content-secondary mb-6">
              Realiza nuestros tests científicos gratuitos y conoce más sobre ti mismo.
            </p>
            <Link to="/" className="btn-primary inline-block">
              Comenzar test
            </Link>
          </div>

          {/* Related Articles */}
          {related.length > 0 && (
            <section className="mt-12">
              <h2 className="text-xl font-bold text-content mb-6">Artículos relacionados</h2>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                {related.map(rel => (
                  <Link key={rel.slug} to={`/blog/${rel.slug}`} className="card group">
                    <span className="text-xs font-medium text-indigo-400 uppercase tracking-wider">
                      {getCategoryName(rel.category)}
                    </span>
                    <h3 className="text-base font-bold text-content mt-1 mb-2 group-hover:text-indigo-400 transition-colors">
                      {rel.title}
                    </h3>
                    <p className="text-sm text-content-secondary leading-relaxed line-clamp-2">{rel.excerpt}</p>
                  </Link>
                ))}
              </div>
            </section>
          )}
        </article>
      </div>
    </>
  )
}

function getCategoryIcon(category: string): string {
  const icons: Record<string, string> = {
    'big-five': '🧠',
    'mbti': '🎭',
    'enneagram': '🔵',
    'relationships': '💞',
    'career': '💼',
    'parenting': '👨‍👩‍👧‍👦',
    'education': '📚',
    'health': '🏥',
    'spirituality': '🕯️',
  }
  return icons[category] || '📝'
}
