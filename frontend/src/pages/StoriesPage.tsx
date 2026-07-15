import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import type { UserStory } from '../types'
import { StoryCard } from '../components/StoryCard'
import { LoadingSkeleton } from '../components/LoadingSkeleton'

export function StoriesPage() {
  const navigate = useNavigate()
  const [stories, setStories] = useState<UserStory[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [showFeatured, setShowFeatured] = useState(false)
  const [page, setPage] = useState(1)
  const [selectedStory, setSelectedStory] = useState<UserStory | null>(null)

  useEffect(() => {
    setLoading(true)
    const params = new URLSearchParams({
      page: String(page),
      per_page: '20',
      featured: String(showFeatured),
    })

    fetch(`/api/v1/community/stories?${params}`)
      .then((r) => {
        if (!r.ok) throw new Error('Failed to load stories')
        return r.json()
      })
      .then((data: UserStory[]) => {
        setStories(data)
        setLoading(false)
      })
      .catch((err) => {
        setError(err.message)
        setLoading(false)
      })
  }, [showFeatured, page])

  const renderMarkdown = (content: string) => {
    return content
      .split('\n')
      .map((line, i) => {
        if (line.startsWith('## ')) {
          return <h2 key={i} className="text-xl font-bold text-content mt-6 mb-3">{line.slice(3)}</h2>
        }
        if (line.startsWith('### ')) {
          return <h3 key={i} className="text-lg font-bold text-content mt-4 mb-2">{line.slice(4)}</h3>
        }
        if (line.startsWith('**') && line.endsWith('**')) {
          return <p key={i} className="font-bold text-content mt-4 mb-2">{line.slice(2, -2)}</p>
        }
        if (line.trim() === '') {
          return <div key={i} className="h-2" />
        }
        if (line.startsWith('- ')) {
          return <li key={i} className="text-content-secondary ml-4 list-disc">{line.slice(2)}</li>
        }
        return <p key={i} className="text-content-secondary leading-relaxed">{line}</p>
      })
  }

  if (selectedStory) {
    return (
      <div className="min-h-screen px-4 py-8">
        <div className="max-w-3xl mx-auto">
          <button
            onClick={() => setSelectedStory(null)}
            className="text-content-muted hover:text-content transition-colors mb-4 text-sm"
          >
            ← Back to Stories
          </button>

          <div className="card animate-fadeIn">
            {selectedStory.featured && (
              <span className="inline-block text-xs font-bold text-amber-400 bg-amber-500/10 border border-amber-500/30 px-2 py-0.5 rounded-full mb-3">
                Featured Story
              </span>
            )}
            <h1 className="text-2xl font-bold text-content mb-2">{selectedStory.title}</h1>
            <div className="flex items-center gap-3 text-sm text-content-muted mb-6">
              <span className="font-medium text-content-secondary">{selectedStory.author_name}</span>
              <span className="text-xs font-bold bg-surface-secondary px-2 py-0.5 rounded-full border border-border">
                {selectedStory.personality_type}
              </span>
            </div>
            <div className="prose prose-invert max-w-none">
              {renderMarkdown(selectedStory.content)}
            </div>
            <div className="flex items-center gap-2 mt-6 pt-4 border-t border-border">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" className="w-5 h-5 text-content-muted">
                <path d="M7.493 18.5c-.425 0-.82-.236-.975-.632A7.5 7.5 0 016 15.125a7.5 7.5 0 01.518-2.743c.155-.396.55-.632.975-.632a1 1 0 01.904.56 7.5 7.5 0 002.392 2.393c.345.207.422.672.157.971l-5.133 5.796c-.226.256-.56.37-.888.27z" />
              </svg>
              <span className="text-sm text-content-muted">{selectedStory.likes} likes</span>
            </div>
          </div>
        </div>
      </div>
    )
  }

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
          <h1 className="text-2xl font-bold text-content">User Stories</h1>
          <p className="text-content-secondary mt-1">
            Real stories from people whose lives were changed by self-discovery
          </p>
        </div>

        <div className="flex items-center gap-4 mb-6">
          <button
            onClick={() => { setShowFeatured(false); setPage(1) }}
            className={`px-3 py-1.5 rounded-full text-sm font-medium transition-all ${
              !showFeatured
                ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30'
                : 'bg-surface text-content-muted border border-border hover:text-content'
            }`}
          >
            All Stories
          </button>
          <button
            onClick={() => { setShowFeatured(true); setPage(1) }}
            className={`px-3 py-1.5 rounded-full text-sm font-medium transition-all ${
              showFeatured
                ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30'
                : 'bg-surface text-content-muted border border-border hover:text-content'
            }`}
          >
            Featured
          </button>
        </div>

        {error && (
          <div className="card border-red-500/30 bg-red-500/10 mb-6">
            <p className="text-red-400 text-sm">{error}</p>
          </div>
        )}

        {loading ? (
          <div className="space-y-4">
            {[1, 2, 3].map((i) => (
              <LoadingSkeleton key={i} variant="card" />
            ))}
          </div>
        ) : (
          <div className="space-y-4">
            {stories.length === 0 ? (
              <div className="card text-center py-12">
                <p className="text-content-muted">No stories yet.</p>
              </div>
            ) : (
              stories.map((story) => (
                <StoryCard
                  key={story.id}
                  story={story}
                  onClick={() => setSelectedStory(story)}
                />
              ))
            )}
          </div>
        )}

        {!loading && stories.length > 0 && (
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
              disabled={stories.length < 20}
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
