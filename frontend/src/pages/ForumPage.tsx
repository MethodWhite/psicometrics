import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import type { ForumPost } from '../types'
import { ForumPostCard } from '../components/ForumPostCard'
import { LoadingSkeleton } from '../components/LoadingSkeleton'

const categories = [
  { value: 'all', label: 'All Posts' },
  { value: 'general', label: 'General' },
  { value: 'big_five', label: 'Big Five' },
  { value: 'mbti', label: 'MBTI' },
  { value: 'enneagram', label: 'Enneagram' },
]

export function ForumPage() {
  const navigate = useNavigate()
  const [posts, setPosts] = useState<ForumPost[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [category, setCategory] = useState('all')
  const [page, setPage] = useState(1)

  useEffect(() => {
    setLoading(true)
    setError('')
    const params = new URLSearchParams({ page: String(page), per_page: '20' })
    if (category !== 'all') params.set('category', category)

    fetch(`/api/v1/community/posts?${params}`)
      .then((r) => {
        if (!r.ok) throw new Error('Failed to load posts')
        return r.json()
      })
      .then((data: ForumPost[]) => {
        setPosts(data)
        setLoading(false)
      })
      .catch((err) => {
        setError(err.message)
        setLoading(false)
      })
  }, [category, page])

  return (
    <div className="min-h-screen px-4 py-8">
      <div className="max-w-3xl mx-auto">
        <div className="mb-6">
          <button
            onClick={() => navigate('/')}
            className="text-content-muted hover:text-content transition-colors mb-4 text-sm"
          >
            ← Back
          </button>
          <h1 className="text-2xl font-bold text-content">Community Forum</h1>
          <p className="text-content-secondary mt-1">
            Discuss personality types, share insights, and connect with others
          </p>
        </div>

        {/* Category filter */}
        <div className="flex flex-wrap gap-2 mb-6">
          {categories.map((cat) => (
            <button
              key={cat.value}
              onClick={() => { setCategory(cat.value); setPage(1) }}
              className={`px-3 py-1.5 rounded-full text-sm font-medium transition-all ${
                category === cat.value
                  ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30'
                  : 'bg-surface text-content-muted border border-border hover:text-content'
              }`}
            >
              {cat.label}
            </button>
          ))}
        </div>

        {/* New post button */}
        <div className="mb-6">
          <button
            onClick={() => navigate('/forum/new')}
            className="btn-primary text-sm"
          >
            + New Post
          </button>
        </div>

        {/* Error */}
        {error && (
          <div className="card border-red-500/30 bg-red-500/10 mb-6">
            <p className="text-red-400 text-sm">{error}</p>
          </div>
        )}

        {/* Loading */}
        {loading && (
          <div className="space-y-4">
            {[1, 2, 3].map((i) => (
              <LoadingSkeleton key={i} variant="card" />
            ))}
          </div>
        )}

        {/* Posts */}
        {!loading && !error && (
          <div className="space-y-4">
            {posts.length === 0 ? (
              <div className="card text-center py-12">
                <p className="text-content-muted">No posts yet. Be the first to start a discussion!</p>
              </div>
            ) : (
              posts.map((post) => (
                <ForumPostCard
                  key={post.id}
                  post={post}
                  onClick={() => navigate(`/forum/${post.id}`)}
                />
              ))
            )}
          </div>
        )}

        {/* Pagination */}
        {!loading && posts.length > 0 && (
          <div className="flex justify-center gap-4 mt-8">
            <button
              disabled={page <= 1}
              onClick={() => setPage((p) => Math.max(1, p - 1))}
              className="btn-secondary text-sm disabled:opacity-50"
            >
              Previous
            </button>
            <span className="flex items-center text-sm text-content-muted">Page {page}</span>
            <button
              disabled={posts.length < 20}
              onClick={() => setPage((p) => p + 1)}
              className="btn-secondary text-sm disabled:opacity-50"
            >
              Next
            </button>
          </div>
        )}
      </div>
    </div>
  )
}
