import { useMemo } from 'react'
import { useParams, Link } from 'react-router-dom'
import { RESOURCES } from '../data/resources'

const TYPE_LABELS: Record<string, string> = {
  INTJ: 'The Architect',
  INTP: 'The Logician',
  ENTJ: 'The Commander',
  ENTP: 'The Debater',
  INFJ: 'The Advocate',
  INFP: 'The Mediator',
  ENFJ: 'The Protagonist',
  ENFP: 'The Campaigner',
  ISTJ: 'The Logistician',
  ISFJ: 'The Defender',
  ESTJ: 'The Executive',
  ESFJ: 'The Consul',
  ISTP: 'The Virtuoso',
  ISFP: 'The Adventurer',
  ESTP: 'The Entrepreneur',
  ESFP: 'The Entertainer',
}

const TYPE_IMAGES: Record<string, string> = {
  INTJ: '🎯', INTP: '🔬', ENTJ: '⚡', ENTP: '💡',
  INFJ: '🔮', INFP: '🌈', ENFJ: '🌟', ENFP: '✨',
  ISTJ: '🏛️', ISFJ: '🛡️', ESTJ: '📋', ESFJ: '🤝',
  ISTP: '🔧', ISFP: '🎨', ESTP: '🚀', ESFP: '🎭',
}

function StarRating({ rating }: { rating: number }) {
  const stars = []
  for (let i = 1; i <= 5; i++) {
    stars.push(
      <span
        key={i}
        className={`text-sm ${i <= Math.round(rating) ? 'text-amber-400' : 'text-content-muted/30'}`}
      >
        ★
      </span>
    )
  }
  return <span className="inline-flex gap-0.5">{stars}</span>
}

export default function TypeResourcesPage() {
  const { type } = useParams<{ type: string }>()
  const typeCode = type?.toUpperCase() || ''

  const recommended = useMemo(() => {
    if (!typeCode) return []
    return RESOURCES
      .filter(r => r.recommendedBy.includes(typeCode))
      .sort((a, b) => b.rating - a.rating)
  }, [typeCode])

  const relatedByCategory = useMemo(() => {
    if (!typeCode || recommended.length === 0) return []
    const recommendedIds = new Set(recommended.map(r => r.id))
    const topCategories = recommended.slice(0, 5).flatMap(r => r.categories)
    return RESOURCES
      .filter(r =>
        !recommendedIds.has(r.id) &&
        r.categories.some(c => topCategories.includes(c))
      )
      .sort((a, b) => b.rating - a.rating)
      .slice(0, 8)
  }, [typeCode, recommended])

  const label = TYPE_LABELS[typeCode] || 'Personality Type'
  const image = TYPE_IMAGES[typeCode] || '📚'

  if (!typeCode || !TYPE_LABELS[typeCode]) {
    return (
      <div className="min-h-screen px-4 py-8">
        <div className="max-w-2xl mx-auto text-center">
          <h1 className="text-2xl font-bold text-content mb-4">Type not found</h1>
          <p className="text-content-muted mb-6">
            The personality type &ldquo;{typeCode}&rdquo; is not recognized.
          </p>
          <Link to="/resources" className="btn-primary inline-block text-sm">
            Browse all resources →
          </Link>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen px-4 py-8">
      <div className="max-w-4xl mx-auto">
        <Link
          to="/resources"
          className="text-content-muted hover:text-content transition-colors text-sm mb-4 inline-block"
        >
          ← All Resources
        </Link>

        {/* Type header */}
        <div className="card mb-8 text-center animate-fadeIn">
          <div className="text-6xl mb-4">{image}</div>
          <h1 className="text-3xl font-bold gradient-text mb-2">{typeCode}</h1>
          <p className="text-content-secondary text-lg mb-2">{label}</p>
          <p className="text-content-muted text-sm">
            Curated resources recommended for {typeCode}s
          </p>
        </div>

        {/* Recommended section */}
        <section className="mb-10 animate-fadeIn delay-100">
          <h2 className="text-xl font-bold text-content mb-4 flex items-center gap-2">
            <span>Recommended for {typeCode}s</span>
            <span className="text-xs text-content-muted font-normal">
              ({recommended.length} resources)
            </span>
          </h2>

          {recommended.length > 0 ? (
            <div className="space-y-3">
              {recommended.map(r => (
                <div key={r.id} className="card hover:shadow-md transition-all">
                  <div className="flex items-start gap-3">
                    <div className="flex-1 min-w-0">
                      <div className="flex items-start justify-between gap-2">
                        <div>
                          <h3 className="font-bold text-content text-sm">{r.title}</h3>
                          {r.author && (
                            <p className="text-xs text-content-muted">{r.author}</p>
                          )}
                        </div>
                        <a
                          href={r.url}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="btn-primary text-xs px-3 py-1 whitespace-nowrap"
                        >
                          Visit
                        </a>
                      </div>
                      <div className="flex items-center gap-2 mt-1.5">
                        <StarRating rating={r.rating} />
                        <span className="text-xs text-content-muted">{r.rating.toFixed(1)}</span>
                        <span className="text-xs text-content-muted">·</span>
                        <span className="text-xs font-medium text-content-muted bg-surface-secondary px-1.5 py-0.5 rounded">
                          {r.type}
                        </span>
                        <span className="text-xs text-content-muted">·</span>
                        <span className="text-xs text-content-muted">{r.price}</span>
                      </div>
                      <p className="text-xs text-content-secondary mt-1.5 leading-relaxed line-clamp-2">
                        {r.description.slice(0, 120)}...
                      </p>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="card text-center py-8">
              <p className="text-content-muted text-sm">
                No specific recommendations yet for {typeCode}. Check back soon!
              </p>
            </div>
          )}
        </section>

        {/* What {typeCode}s are reading */}
        {recommended.filter(r => r.type === 'book').length > 0 && (
          <section className="mb-10 animate-fadeIn delay-200">
            <h2 className="text-xl font-bold text-content mb-4">
              What {typeCode}s are reading
            </h2>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              {recommended
                .filter(r => r.type === 'book')
                .slice(0, 6)
                .map(r => (
                  <div key={r.id} className="card hover:shadow-md transition-all">
                    <div className="flex items-start gap-3">
                      <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-primary/20 to-accent/20 flex items-center justify-center text-sm flex-shrink-0">
                        📚
                      </div>
                      <div className="min-w-0">
                        <h3 className="font-bold text-content text-xs">{r.title}</h3>
                        <p className="text-[10px] text-content-muted">{r.author}</p>
                        <div className="flex items-center gap-1 mt-1">
                          <StarRating rating={r.rating} />
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
            </div>
          </section>
        )}

        {/* Related resources */}
        {relatedByCategory.length > 0 && (
          <section className="animate-fadeIn delay-300">
            <h2 className="text-xl font-bold text-content mb-4">
              More resources you might like
            </h2>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              {relatedByCategory.slice(0, 6).map(r => (
                <div key={r.id} className="card hover:shadow-md transition-all">
                  <div className="flex items-start gap-3">
                    <div className="w-8 h-8 rounded-lg bg-surface-secondary flex items-center justify-center text-sm flex-shrink-0">
                      {r.type === 'book' ? '📚' : r.type === 'app' ? '📱' : r.type === 'course' ? '🎓' : r.type === 'podcast' ? '🎙️' : r.type === 'video' ? '🎬' : '🌐'}
                    </div>
                    <div className="min-w-0">
                      <h3 className="font-bold text-content text-xs">{r.title}</h3>
                      <p className="text-[10px] text-content-muted">{r.author}</p>
                      <div className="flex items-center gap-2 mt-1">
                        <StarRating rating={r.rating} />
                        <span className="text-[10px] text-content-muted">{r.rating.toFixed(1)}</span>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Top rated by others */}
        {recommended.length >= 3 && (
          <section className="mt-10 animate-fadeIn delay-300">
            <h2 className="text-xl font-bold text-content mb-4">
              Highest rated by {typeCode}s
            </h2>
            <div className="overflow-hidden rounded-xl border border-border">
              <table className="w-full text-sm">
                <thead>
                  <tr className="bg-surface-secondary">
                    <th className="text-left px-4 py-2 text-content-muted font-medium text-xs">Resource</th>
                    <th className="text-left px-4 py-2 text-content-muted font-medium text-xs hidden sm:table-cell">Type</th>
                    <th className="text-right px-4 py-2 text-content-muted font-medium text-xs">Rating</th>
                  </tr>
                </thead>
                <tbody>
                  {recommended
                    .sort((a, b) => b.rating - a.rating)
                    .slice(0, 5)
                    .map(r => (
                      <tr key={r.id} className="border-t border-border hover:bg-surface-secondary/50 transition-colors">
                        <td className="px-4 py-2.5">
                          <a
                            href={r.url}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-content font-medium text-xs hover:text-primary transition-colors"
                          >
                            {r.title}
                          </a>
                          <p className="text-[10px] text-content-muted">{r.author}</p>
                        </td>
                        <td className="px-4 py-2.5 text-content-muted text-xs hidden sm:table-cell capitalize">
                          {r.type}
                        </td>
                        <td className="px-4 py-2.5 text-right">
                          <div className="flex items-center justify-end gap-1">
                            <StarRating rating={r.rating} />
                            <span className="text-xs text-content-muted">{r.rating.toFixed(1)}</span>
                          </div>
                        </td>
                      </tr>
                    ))}
                </tbody>
              </table>
            </div>
          </section>
        )}
      </div>
    </div>
  )
}
