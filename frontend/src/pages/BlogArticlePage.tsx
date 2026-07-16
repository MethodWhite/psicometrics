import { useParams, Link } from 'react-router-dom'
import { useState, useEffect, useMemo } from 'react'
import { Helmet } from 'react-helmet-async'
import { blogArticles, getRelatedArticles } from '../data/blog-articles'
import { getCategoryName } from './BlogPage'

export default function BlogArticlePage() {
  const { slug } = useParams<{ slug: string }>()
  const article = blogArticles.find(a => a.slug === slug)
  const [readingProgress, setReadingProgress] = useState(0)
  const [showToc, setShowToc] = useState(false)

  useEffect(() => {
    const onScroll = () => {
      const scrollTop = window.scrollY
      const docHeight = document.documentElement.scrollHeight - window.innerHeight
      setReadingProgress(docHeight > 0 ? Math.min((scrollTop / docHeight) * 100, 100) : 0)
    }
    window.addEventListener('scroll', onScroll)
    return () => window.removeEventListener('scroll', onScroll)
  }, [])

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

  // Parse content into sections for TOC
  const sections = useMemo(() => {
    return article.content
      .split('\n')
      .filter(l => l.startsWith('## '))
      .map(l => ({
        title: l.replace('## ', ''),
        id: l.replace('## ', '').toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/(^-|-$)/g, '')
      }))
  }, [article])

  // Reading time: ~200 words/min
  const wordCount = article.content.split(/\s+/).length
  const readingTime = Math.max(1, Math.round(wordCount / 200))

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('es-ES', { year: 'numeric', month: 'long', day: 'numeric' })
  }

  const shareUrl = typeof window !== 'undefined' ? window.location.href : ''

  const renderContent = (line: string, i: number) => {
    if (line.startsWith('## ')) {
      const id = line.replace('## ', '').toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/(^-|-$)/g, '')
      return (
        <h2 key={i} id={id} className="text-2xl font-bold text-content mt-12 mb-4 scroll-mt-20">
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
    if (line.startsWith('- **') && line.endsWith('**')) {
      return (
        <li key={i} className="text-content-secondary leading-relaxed mb-2 ml-6 list-disc">
          {line.replace(/^- \*\*|\*\*$/g, '')}
        </li>
      )
    }
    if (line.startsWith('- ')) {
      return (
        <li key={i} className="text-content-secondary leading-relaxed mb-2 ml-6 list-disc">
          {line.replace('- ', '')}
        </li>
      )
    }
    if (line.startsWith('> ')) {
      return (
        <blockquote key={i} className="border-l-4 border-indigo-500/50 pl-4 py-2 my-4 bg-indigo-500/5 rounded-r-lg italic text-content-secondary">
          {line.replace('> ', '')}
        </blockquote>
      )
    }
    if (line.trim() === '') {
      return <div key={i} className="h-4" />
    }
    return (
      <p key={i} className="text-content-secondary leading-relaxed mb-4 text-[17px]">
        {line}
      </p>
    )
  }

  return (
    <>
      <Helmet>
        <title>{article.title} — PsicoMetrics Blog</title>
        <meta name="description" content={article.excerpt} />
        <meta property="og:title" content={article.title} />
        <meta property="og:description" content={article.excerpt} />
        <meta property="og:type" content="article" />
        <meta property="og:url" content={shareUrl} />
        <meta property="article:published_time" content={article.publishedDate} />
        <meta property="article:author" content={article.author} />
        <meta property="article:section" content={getCategoryName(article.category)} />
        {article.tags.map(tag => (
          <meta key={tag} property="article:tag" content={tag} />
        ))}
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:title" content={article.title} />
        <meta name="twitter:description" content={article.excerpt} />
      </Helmet>

      {/* Reading progress bar */}
      <div className="fixed top-0 left-0 right-0 z-50 h-1 bg-transparent">
        <div
          className="h-full bg-gradient-to-r from-indigo-500 to-cyan-500 transition-all duration-150"
          style={{ width: `${readingProgress}%` }}
        />
      </div>

      <div className="min-h-screen px-4 py-8">
        <article className="max-w-3xl mx-auto">
          {/* Breadcrumb */}
          <div className="mb-6 flex items-center gap-2 text-sm text-content-muted flex-wrap">
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
            <div className="flex flex-wrap items-center gap-3 mb-4">
              <span className="text-sm font-medium text-indigo-400 uppercase tracking-wider">
                {getCategoryName(article.category)}
              </span>
              <span className="w-1 h-1 rounded-full bg-content-muted" />
              <span className="text-content-muted text-sm">{formatDate(article.publishedDate)}</span>
              <span className="w-1 h-1 rounded-full bg-content-muted" />
              <span className="text-content-muted text-sm">{readingTime} min de lectura</span>
            </div>
            <h1 className="text-3xl md:text-4xl lg:text-5xl font-bold text-content leading-tight mb-4">
              {article.title}
            </h1>
            <p className="text-lg md:text-xl text-content-secondary leading-relaxed">
              {article.excerpt}
            </p>
          </header>

          {/* Banner */}
          <div className="h-48 md:h-72 rounded-2xl bg-gradient-to-br from-indigo-600/20 via-purple-600/20 to-cyan-600/20 flex items-center justify-center mb-8 border border-white/5">
            <span className="text-7xl">{getCategoryIcon(article.category)}</span>
          </div>

          {/* Table of Contents */}
          {sections.length > 1 && (
            <div className="mb-10 p-6 card border-indigo-500/10">
              <button
                onClick={() => setShowToc(!showToc)}
                className="flex items-center justify-between w-full text-left"
              >
                <span className="font-bold text-content text-sm uppercase tracking-wider">Contenido</span>
                <span className={`transition-transform ${showToc ? 'rotate-180' : ''}`}>▼</span>
              </button>
              {showToc && (
                <nav className="mt-4 space-y-2">
                  {sections.map(s => (
                    <a
                      key={s.id}
                      href={`#${s.id}`}
                      onClick={e => { e.preventDefault(); document.getElementById(s.id)?.scrollIntoView({ behavior: 'smooth' }) }}
                      className="block text-sm text-content-secondary hover:text-indigo-400 transition-colors py-1"
                    >
                      {s.title}
                    </a>
                  ))}
                </nav>
              )}
            </div>
          )}

          {/* Content */}
          <div className="prose-content max-w-none">
            {article.content.split('\n').map((line, i) => renderContent(line, i))}
          </div>

          {/* Author bio */}
          <div className="mt-10 p-6 card bg-gradient-to-br from-indigo-500/5 to-purple-500/5">
            <div className="flex items-start gap-4">
              <div className="w-16 h-16 rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-white font-bold text-2xl flex-shrink-0">
                {article.author.split(' ').map(w => w[0]).join('').slice(0, 2).toUpperCase()}
              </div>
              <div>
                <h3 className="font-bold text-content text-lg">{article.author}</h3>
                <p className="text-sm text-content-muted mt-1 leading-relaxed">
                  Psicóloga clínica especializada en evaluación de la personalidad.
                  Doctora en Psicología por la Universidad Autónoma de Madrid.
                  Miembro de la International Society for the Study of Individual Differences (ISSID).
                </p>
                <div className="flex gap-3 mt-3">
                  <span className="text-xs text-indigo-400 cursor-pointer hover:underline">Twitter</span>
                  <span className="text-xs text-indigo-400 cursor-pointer hover:underline">LinkedIn</span>
                  <span className="text-xs text-indigo-400 cursor-pointer hover:underline">Web</span>
                </div>
              </div>
            </div>
          </div>

          {/* Tags */}
          <div className="mt-8 pt-6 border-t border-border">
            <div className="flex items-center gap-2 mb-4">
              <span className="text-sm font-bold text-content">Tags</span>
            </div>
            <div className="flex flex-wrap gap-2">
              {article.tags.map(tag => (
                <Link
                  key={tag}
                  to={`/tags/${tag.toLowerCase().replace(/[\s/]+/g, '-')}`}
                  className="px-3 py-1.5 bg-surface-secondary rounded-full text-xs text-content-muted hover:text-indigo-400 hover:bg-indigo-500/10 border border-border hover:border-indigo-500/30 transition-all"
                >
                  #{tag}
                </Link>
              ))}
            </div>
          </div>

          {/* Share */}
          <div className="mt-8 pt-6 border-t border-border">
            <div className="flex items-center gap-2 mb-4">
              <span className="text-sm font-bold text-content">Compartir</span>
            </div>
            <div className="flex flex-wrap gap-3">
              <a
                href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(article.title)}&url=${encodeURIComponent(shareUrl)}`}
                target="_blank" rel="noopener noreferrer"
                className="px-4 py-2 bg-surface-secondary rounded-lg text-sm text-content-muted hover:text-blue-400 hover:bg-blue-500/10 border border-border hover:border-blue-500/30 transition-all"
              >
                𝕏 Twitter
              </a>
              <a
                href={`https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(shareUrl)}`}
                target="_blank" rel="noopener noreferrer"
                className="px-4 py-2 bg-surface-secondary rounded-lg text-sm text-content-muted hover:text-blue-600 hover:bg-blue-500/10 border border-border hover:border-blue-500/30 transition-all"
              >
                Facebook
              </a>
              <a
                href={`https://wa.me/?text=${encodeURIComponent(article.title + ' ' + shareUrl)}`}
                target="_blank" rel="noopener noreferrer"
                className="px-4 py-2 bg-surface-secondary rounded-lg text-sm text-content-muted hover:text-green-500 hover:bg-green-500/10 border border-border hover:border-green-500/30 transition-all"
              >
                WhatsApp
              </a>
              <a
                href={`https://www.linkedin.com/shareArticle?mini=true&url=${encodeURIComponent(shareUrl)}&title=${encodeURIComponent(article.title)}`}
                target="_blank" rel="noopener noreferrer"
                className="px-4 py-2 bg-surface-secondary rounded-lg text-sm text-content-muted hover:text-blue-700 hover:bg-blue-500/10 border border-border hover:border-blue-500/30 transition-all"
              >
                LinkedIn
              </a>
              <button
                onClick={() => { navigator.clipboard.writeText(shareUrl) }}
                className="px-4 py-2 bg-surface-secondary rounded-lg text-sm text-content-muted hover:text-content hover:bg-white/5 border border-border hover:border-white/20 transition-all"
              >
                🔗 Copiar
              </button>
            </div>
          </div>

          {/* Newsletter CTA */}
          <div className="mt-10 p-8 card text-center bg-gradient-to-br from-indigo-600/10 via-purple-600/10 to-cyan-600/10 border-indigo-500/20">
            <span className="text-4xl mb-4 block">📬</span>
            <h3 className="text-xl font-bold text-content mb-2">Recibe más artículos como este</h3>
            <p className="text-content-secondary mb-6 max-w-md mx-auto">
              Suscríbete a nuestro newsletter semanal y recibe contenido exclusivo sobre personalidad, psicología y desarrollo personal.
            </p>
            <div className="flex gap-3 max-w-md mx-auto">
              <input
                type="email"
                placeholder="tu@email.com"
                className="flex-1 px-4 py-2.5 bg-surface-secondary border border-border rounded-xl text-content placeholder:text-content-muted/50 focus:outline-none focus:border-indigo-500/50 transition-colors text-sm"
              />
              <button className="btn-primary whitespace-nowrap text-sm">
                Suscribirme
              </button>
            </div>
          </div>

          {/* Related Articles */}
          {related.length > 0 && (
            <section className="mt-12">
              <h2 className="text-xl font-bold text-content mb-6">Artículos relacionados</h2>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                {related.map(rel => (
                  <Link key={rel.slug} to={`/blog/${rel.slug}`} className="card group !p-0 overflow-hidden">
                    <div className="h-24 bg-gradient-to-br from-indigo-500/15 to-purple-500/15 flex items-center justify-center">
                      <span className="text-2xl">{getCategoryIcon(rel.category)}</span>
                    </div>
                    <div className="p-4">
                      <span className="text-[10px] font-medium text-indigo-400 uppercase tracking-wider">
                        {getCategoryName(rel.category)}
                      </span>
                      <h3 className="text-sm font-bold text-content mt-1 mb-1 group-hover:text-indigo-400 transition-colors line-clamp-2">
                        {rel.title}
                      </h3>
                      <p className="text-xs text-content-secondary line-clamp-2 leading-relaxed">{rel.excerpt}</p>
                    </div>
                  </Link>
                ))}
              </div>
            </section>
          )}

          {/* CTA final */}
          <div className="mt-12 p-8 card text-center bg-gradient-to-br from-indigo-500/10 to-purple-600/10 border-indigo-500/20">
            <span className="text-5xl mb-4 block">🧠</span>
            <h3 className="text-2xl font-bold text-content mb-2">Descubre tu tipo de personalidad</h3>
            <p className="text-content-secondary mb-6 max-w-lg mx-auto">
              Realiza nuestros tests científicos gratuitos basados en estándares validados
              y obtén un perfil completo de tu personalidad.
            </p>
            <div className="flex flex-wrap gap-4 justify-center">
              <Link to="/" className="btn-primary">
                Comenzar test gratis
              </Link>
              <Link to="/types" className="btn-secondary">
                Explorar tipos
              </Link>
            </div>
          </div>

          {/* Back to top */}
          {readingProgress > 20 && (
            <button
              onClick={() => window.scrollTo({ top: 0, behavior: 'smooth' })}
              className="fixed bottom-8 right-8 w-12 h-12 rounded-full bg-indigo-500/20 border border-indigo-500/30 text-indigo-400 flex items-center justify-center hover:bg-indigo-500/30 transition-all backdrop-blur-sm z-40"
              aria-label="Volver arriba"
            >
              ↑
            </button>
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
