import { useParams, Link } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'
import { blogArticles, getRelatedArticles } from '../data/blog-articles'

const MBTI_ICONS: Record<string, string> = {
  INTJ: '♟️', INTP: '🔬', ENTJ: '👑', ENTP: '💡',
  INFJ: '🕊️', INFP: '🌈', ENFJ: '🌟', ENFP: '🎨',
  ISTJ: '🏛️', ISFJ: '🤝', ESTJ: '📋', ESFJ: '🎪',
  ISTP: '🔧', ISFP: '🎵', ESTP: '🚀', ESFP: '🎭',
}

const MBTI_NAMES: Record<string, string> = {
  INTJ: 'Arquitecto', INTP: 'Lógico', ENTJ: 'Comandante', ENTP: 'Innovador',
  INFJ: 'Consejero', INFP: 'Idealista', ENFJ: 'Maestro', ENFP: 'Explorador',
  ISTJ: 'Supervisor', ISFJ: 'Protector', ESTJ: 'Ejecutivo', ESFJ: 'Embajador',
  ISTP: 'Artesano', ISFP: 'Artista', ESTP: 'Emprendedor', ESFP: 'Animador',
}

export default function DeepProfilePage() {
  const { mbtiType } = useParams<{ mbtiType: string }>()
  const typeCode = mbtiType?.toUpperCase() || ''

  const article = blogArticles.find(a => a.slug === `${typeCode.toLowerCase()}-personality`)

  if (!article || !MBTI_NAMES[typeCode]) {
    return (
      <div className="min-h-screen px-4 py-24 text-center">
        <span className="text-6xl block mb-4">🔍</span>
        <h1 className="text-2xl font-bold text-content mb-2">Perfil no encontrado</h1>
        <p className="text-content-muted mb-6">El tipo MBTI "{typeCode}" no existe o no está disponible.</p>
        <Link to="/blog?category=mbti" className="btn-primary inline-block">Ver tipos MBTI</Link>
      </div>
    )
  }

  const related = getRelatedArticles(article.slug, 'mbti', 4)

  return (
    <>
      <Helmet>
        <title>{article.title} — PsicoMetrics</title>
        <meta name="description" content={article.excerpt} />
        <meta property="og:title" content={article.title} />
        <meta property="og:description" content={article.excerpt} />
      </Helmet>

      <div className="min-h-screen px-4 py-8">
        <div className="max-w-4xl mx-auto">
          {/* Breadcrumb */}
          <div className="mb-6 flex items-center gap-2 text-sm text-content-muted">
            <Link to="/" className="hover:text-content transition-colors">Inicio</Link>
            <span>/</span>
            <Link to="/blog?category=mbti" className="hover:text-content transition-colors">Tipos MBTI</Link>
            <span>/</span>
            <span className="text-content">{typeCode}</span>
          </div>

          {/* Hero */}
          <div className="card !p-8 mb-8 text-center bg-gradient-to-br from-indigo-600/10 via-purple-600/10 to-pink-600/10">
            <div className="text-7xl mb-4">{MBTI_ICONS[typeCode]}</div>
            <h1 className="text-5xl font-black gradient-text mb-2">{typeCode}</h1>
            <p className="text-xl text-content-secondary font-medium mb-4">{MBTI_NAMES[typeCode]}</p>
            <p className="text-content-secondary max-w-2xl mx-auto">{article.excerpt}</p>
          </div>

          {/* Full Content */}
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

          {/* CTA */}
          <div className="mt-8 p-8 card text-center bg-gradient-to-br from-indigo-600/10 to-purple-600/10 border-indigo-500/20">
            <span className="text-4xl mb-4 block">🎯</span>
            <h3 className="text-xl font-bold text-content mb-2">¿Eres {typeCode}?</h3>
            <p className="text-content-secondary mb-6">
              Confirma tu tipo con nuestro test MBTI oficial y obtén un análisis completo de tu personalidad.
            </p>
            <Link to="/test/mbti" className="btn-primary inline-block">
              Tomar test MBTI
            </Link>
          </div>

          {/* Other MBTI Types */}
          {related.length > 0 && (
            <section className="mt-12">
              <h2 className="text-xl font-bold text-content mb-6">Otros tipos MBTI</h2>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                {related.map(rel => {
                  const code = rel.slug.split('-')[0].toUpperCase()
                  return (
                    <Link key={rel.slug} to={`/profile/${code}`} className="card group text-center">
                      <div className="text-3xl mb-2">{MBTI_ICONS[code] || '🎭'}</div>
                      <div className="text-lg font-bold text-content group-hover:text-indigo-400 transition-colors">{code}</div>
                      <div className="text-xs text-content-muted">{MBTI_NAMES[code] || ''}</div>
                    </Link>
                  )
                })}
              </div>
            </section>
          )}
        </div>
      </div>
    </>
  )
}
