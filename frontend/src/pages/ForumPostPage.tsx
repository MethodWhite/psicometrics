import { useEffect, useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import type { ForumPost, Comment } from '../types'
import { LoadingSkeleton } from '../components/LoadingSkeleton'

export function ForumPostPage() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const [post, setPost] = useState<ForumPost | null>(null)
  const [comments, setComments] = useState<Comment[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  // Comment form
  const [authorName, setAuthorName] = useState('')
  const [commentText, setCommentText] = useState('')
  const [submitting, setSubmitting] = useState(false)
  const [submitError, setSubmitError] = useState('')

  useEffect(() => {
    if (!id) return
    setLoading(true)
    Promise.all([
      fetch(`/api/v1/community/posts/${id}`).then((r) => {
        if (!r.ok) throw new Error('Post not found')
        return r.json()
      }),
      fetch(`/api/v1/community/posts/${id}/comments`).then((r) => r.json()),
    ])
      .then(([postData, commentsData]) => {
        setPost(postData as ForumPost)
        setComments(commentsData as Comment[])
        setLoading(false)
      })
      .catch((err) => {
        setError(err.message)
        setLoading(false)
      })
  }, [id])

  const handleLike = async () => {
    if (!id || !post) return
    try {
      await fetch(`/api/v1/community/posts/${id}/like`, { method: 'POST' })
      setPost({ ...post, likes: post.likes + 1 })
    } catch { /* ignore */ }
  }

  const handleComment = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!id || !commentText.trim() || !authorName.trim()) return
    setSubmitting(true)
    setSubmitError('')
    try {
      await fetch(`/api/v1/community/posts/${id}/comments`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ author_name: authorName, content: commentText }),
      })
      // Refresh comments
      const data = await fetch(`/api/v1/community/posts/${id}/comments`).then((r) => r.json())
      setComments(data as Comment[])
      setCommentText('')
      setSubmitting(false)
    } catch (err) {
      setSubmitError(err instanceof Error ? err.message : 'Failed to post comment')
      setSubmitting(false)
    }
  }

  const timeAgo = (ts: number) => {
    const diff = Date.now() / 1000 - ts
    if (diff < 3600) return `${Math.floor(diff / 60)}m ago`
    if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`
    if (diff < 2592000) return `${Math.floor(diff / 86400)}d ago`
    return `${Math.floor(diff / 2592000)}mo ago`
  }

  if (loading) {
    return (
      <div className="min-h-screen px-4 py-8">
        <div className="max-w-3xl mx-auto">
          <LoadingSkeleton variant="card" />
          <div className="mt-6">
            <LoadingSkeleton variant="list" lines={3} />
          </div>
        </div>
      </div>
    )
  }

  if (error || !post) {
    return (
      <div className="min-h-screen px-4 py-8">
        <div className="max-w-3xl mx-auto">
          <button onClick={() => navigate('/forum')} className="text-content-muted hover:text-content transition-colors mb-4 text-sm">
            ← Back to Forum
          </button>
          <div className="card border-red-500/30 bg-red-500/10">
            <p className="text-red-400">{error || 'Post not found'}</p>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen px-4 py-8">
      <div className="max-w-3xl mx-auto">
        <button
          onClick={() => navigate('/forum')}
          className="text-content-muted hover:text-content transition-colors mb-4 text-sm"
        >
          ← Back to Forum
        </button>

        {/* Post */}
        <div className="card mb-6">
          <div className="flex items-center gap-2 mb-3">
            <span className="text-xs font-medium px-2 py-0.5 rounded-full bg-indigo-500/20 text-indigo-300 border border-indigo-500/30">
              {post.category}
            </span>
            {post.personality_type && (
              <span className="text-xs font-bold text-content-muted bg-surface-secondary px-2 py-0.5 rounded-full border border-border">
                {post.personality_type}
              </span>
            )}
          </div>
          <h1 className="text-2xl font-bold text-content mb-2">{post.title}</h1>
          <div className="flex items-center gap-3 text-sm text-content-muted mb-4">
            <span className="font-medium text-content-secondary">{post.author_name}</span>
            <span>{timeAgo(post.created_at)}</span>
          </div>
          <div className="text-content-secondary leading-relaxed whitespace-pre-wrap mb-4">
            {post.content}
          </div>
          <button
            onClick={handleLike}
            className="flex items-center gap-1 text-sm text-content-muted hover:text-indigo-400 transition-colors"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" className="w-5 h-5">
              <path d="M7.493 18.5c-.425 0-.82-.236-.975-.632A7.5 7.5 0 016 15.125a7.5 7.5 0 01.518-2.743c.155-.396.55-.632.975-.632a1 1 0 01.904.56 7.5 7.5 0 002.392 2.393c.345.207.422.672.157.971l-5.133 5.796c-.226.256-.56.37-.888.27z" />
            </svg>
            {post.likes} {post.likes === 1 ? 'like' : 'likes'}
          </button>
        </div>

        {/* Comments */}
        <div className="card">
          <h2 className="text-lg font-bold text-content mb-4">
            Comments ({comments.length})
          </h2>

          {comments.length === 0 ? (
            <p className="text-content-muted text-sm mb-4">No comments yet. Be the first to share your thoughts!</p>
          ) : (
            <div className="space-y-4 mb-6">
              {comments.map((c) => (
                <div key={c.id} className="border-b border-border last:border-0 pb-4 last:pb-0">
                  <div className="flex items-center gap-2 mb-1">
                    <span className="font-medium text-sm text-content">{c.author_name}</span>
                    <span className="text-xs text-content-muted">{timeAgo(c.created_at)}</span>
                  </div>
                  <p className="text-sm text-content-secondary">{c.content}</p>
                </div>
              ))}
            </div>
          )}

          {/* Comment form */}
          <form onSubmit={handleComment} className="space-y-3">
            <input
              type="text"
              value={authorName}
              onChange={(e) => setAuthorName(e.target.value)}
              placeholder="Your name"
              className="w-full bg-black/30 border border-border rounded-xl px-4 py-2.5 text-content text-sm focus:outline-none focus:border-indigo-500"
            />
            <textarea
              value={commentText}
              onChange={(e) => setCommentText(e.target.value)}
              placeholder="Share your thoughts..."
              rows={3}
              className="w-full bg-black/30 border border-border rounded-xl px-4 py-2.5 text-content text-sm focus:outline-none focus:border-indigo-500 resize-y"
            />
            {submitError && <p className="text-red-400 text-sm">{submitError}</p>}
            <button
              type="submit"
              disabled={submitting || !commentText.trim() || !authorName.trim()}
              className="btn-primary text-sm disabled:opacity-50"
            >
              {submitting ? 'Posting...' : 'Post Comment'}
            </button>
          </form>
        </div>
      </div>
    </div>
  )
}
