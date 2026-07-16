import { useState, useEffect } from 'react'

interface ArticleComment {
  id: string
  article_slug: string
  author_name: string
  content: string
  created_at: number
  likes: number
}

interface Props {
  slug: string
}

export default function CommentSection({ slug }: Props) {
  const [comments, setComments] = useState<ArticleComment[]>([])
  const [loading, setLoading] = useState(true)
  const [authorName, setAuthorName] = useState('')
  const [content, setContent] = useState('')
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState('')

  useEffect(() => {
    fetchComments()
  }, [slug])

  const fetchComments = async () => {
    try {
      const res = await fetch(`/api/v1/blog/${slug}/comments`)
      if (res.ok) setComments(await res.json())
    } catch (e) {
      console.error('Failed to load comments:', e)
    } finally {
      setLoading(false)
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!authorName.trim() || !content.trim()) return
    setSubmitting(true)
    setError('')
    try {
      const res = await fetch(`/api/v1/blog/${slug}/comments`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ author_name: authorName.trim(), content: content.trim() }),
      })
      if (res.ok) {
        const comment = await res.json()
        setComments(prev => [comment, ...prev])
        setContent('')
      } else {
        const err = await res.json()
        setError(err.detail || 'Error al enviar comentario')
      }
    } catch {
      setError('Error de conexión')
    } finally {
      setSubmitting(false)
    }
  }

  const handleLike = async (commentId: string) => {
    try {
      await fetch(`/api/v1/blog/${slug}/comments/${commentId}/like`, { method: 'POST' })
      setComments(prev => prev.map(c => c.id === commentId ? { ...c, likes: c.likes + 1 } : c))
    } catch {}
  }

  const formatDate = (ts: number) => {
    const date = new Date(ts * 1000)
    const now = new Date()
    const diff = now.getTime() - date.getTime()
    const mins = Math.floor(diff / 60000)
    if (mins < 1) return 'Justo ahora'
    if (mins < 60) return `Hace ${mins} min`
    const hours = Math.floor(mins / 60)
    if (hours < 24) return `Hace ${hours}h`
    const days = Math.floor(hours / 24)
    if (days < 7) return `Hace ${days}d`
    return date.toLocaleDateString('es-ES', { day: 'numeric', month: 'short', year: 'numeric' })
  }

  return (
    <section className="mt-12">
      <h2 className="text-xl font-bold text-content mb-6 flex items-center gap-2">
        <span>💬</span>
        Comentarios {comments.length > 0 && <span className="text-sm font-normal text-content-muted">({comments.length})</span>}
      </h2>

      {/* Form */}
      <form onSubmit={handleSubmit} className="card p-6 mb-8">
        <h3 className="text-sm font-bold text-content mb-4">Deja tu comentario</h3>
        <div className="space-y-4">
          <input
            type="text"
            value={authorName}
            onChange={e => setAuthorName(e.target.value)}
            placeholder="Tu nombre"
            maxLength={60}
            required
            className="w-full px-4 py-2.5 bg-surface-secondary border border-border rounded-xl text-content placeholder:text-content-muted/50 focus:outline-none focus:border-indigo-500/50 transition-colors text-sm"
          />
          <textarea
            value={content}
            onChange={e => setContent(e.target.value)}
            placeholder="Escribí tu comentario..."
            maxLength={2000}
            rows={4}
            required
            className="w-full px-4 py-2.5 bg-surface-secondary border border-border rounded-xl text-content placeholder:text-content-muted/50 focus:outline-none focus:border-indigo-500/50 transition-colors text-sm resize-none"
          />
          <div className="flex items-center justify-between">
            <span className="text-xs text-content-muted">{content.length}/2000</span>
            <button
              type="submit"
              disabled={submitting || !authorName.trim() || !content.trim()}
              className="px-5 py-2 bg-indigo-500 text-white rounded-xl text-sm font-medium hover:bg-indigo-600 disabled:opacity-40 disabled:cursor-not-allowed transition-all"
            >
              {submitting ? 'Enviando...' : 'Publicar comentario'}
            </button>
          </div>
          {error && <p className="text-sm text-red-400">{error}</p>}
        </div>
      </form>

      {/* Comments list */}
      {loading ? (
        <div className="space-y-4">
          {[1, 2, 3].map(i => (
            <div key={i} className="card p-5 animate-pulse">
              <div className="h-4 bg-surface-secondary rounded w-1/4 mb-3" />
              <div className="h-3 bg-surface-secondary rounded w-3/4 mb-2" />
              <div className="h-3 bg-surface-secondary rounded w-1/2" />
            </div>
          ))}
        </div>
      ) : comments.length === 0 ? (
        <div className="text-center py-12 text-content-muted">
          <span className="text-3xl mb-3 block">💭</span>
          <p>Sé el primero en comentar</p>
        </div>
      ) : (
        <div className="space-y-4">
          {comments.map(comment => (
            <div key={comment.id} className="card p-5">
              <div className="flex items-start justify-between mb-3">
                <div className="flex items-center gap-3">
                  <div className="w-9 h-9 rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-white font-bold text-sm flex-shrink-0">
                    {comment.author_name.charAt(0).toUpperCase()}
                  </div>
                  <div>
                    <p className="text-sm font-bold text-content">{comment.author_name}</p>
                    <p className="text-xs text-content-muted">{formatDate(comment.created_at)}</p>
                  </div>
                </div>
                <button
                  onClick={() => handleLike(comment.id)}
                  className="flex items-center gap-1 text-xs text-content-muted hover:text-red-400 transition-colors"
                >
                  <span>{comment.likes > 0 ? '❤️' : '🤍'}</span>
                  <span>{comment.likes}</span>
                </button>
              </div>
              <p className="text-sm text-content-secondary leading-relaxed whitespace-pre-wrap">{comment.content}</p>
            </div>
          ))}
        </div>
      )}
    </section>
  )
}
