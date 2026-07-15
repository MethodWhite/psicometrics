import { useState, useMemo } from 'react'
import { Link } from 'react-router-dom'
import { RESOURCES, type Resource } from '../data/resources'
import { RESOURCE_CATEGORIES } from '../data/navigation'

const TYPE_CATEGORIES = [
  { id: 'all', label: 'All Types' },
  { id: 'big-five', label: 'Big Five' },
  { id: 'mbti', label: 'MBTI' },
  { id: 'enneagram', label: 'Enneagram' },
  { id: 'general-psychology', label: 'General Psychology' },
  { id: 'career', label: 'Career' },
  { id: 'relationships', label: 'Relationships' },
  { id: 'self-improvement', label: 'Self-Improvement' },
  { id: 'meditation', label: 'Meditation' },
]

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

function ResourceCard({ resource }: { resource: Resource }) {
  const [expanded, setExpanded] = useState(false)
  const typeInfo = RESOURCE_CATEGORIES.find(c => c.id === resource.type)
  const preview = resource.description.length > 150
    ? resource.description.slice(0, 150) + '...'
    : resource.description

  return (
    <div className="card hover:shadow-lg transition-all duration-300 animate-fadeIn">
      <div className="flex items-start gap-4">
        <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-primary/20 to-accent/20 flex items-center justify-center text-2xl flex-shrink-0">
          {typeInfo?.icon || '📄'}
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex items-start justify-between gap-2">
            <div>
              <h3 className="font-bold text-content text-sm leading-snug">{resource.title}</h3>
              {resource.author && (
                <p className="text-xs text-content-muted mt-0.5">{resource.author}</p>
              )}
            </div>
            <span className="text-xs font-medium text-content-muted bg-surface-secondary px-2 py-0.5 rounded-full whitespace-nowrap">
              {resource.price}
            </span>
          </div>

          <div className="flex items-center gap-3 mt-2">
            <StarRating rating={resource.rating} />
            <span className="text-xs text-content-muted">
              {resource.rating.toFixed(1)}
            </span>
          </div>

          <p className="text-xs text-content-secondary mt-2 leading-relaxed">
            {expanded ? resource.description : preview}
          </p>
          {resource.description.length > 150 && (
            <button
              onClick={() => setExpanded(!expanded)}
              className="text-xs text-primary hover:text-primary/80 mt-1 transition-colors"
            >
              {expanded ? 'Show less' : 'Read more'}
            </button>
          )}

          <div className="flex flex-wrap items-center gap-2 mt-3">
            {resource.categories.map(cat => (
              <Link
                key={cat}
                to={`/resources?category=${cat}`}
                className="text-xs px-2 py-0.5 rounded-full bg-primary/10 text-primary/80 hover:bg-primary/20 transition-colors"
              >
                {TYPE_CATEGORIES.find(t => t.id === cat)?.label || cat}
              </Link>
            ))}
            <a
              href={resource.url}
              target="_blank"
              rel="noopener noreferrer"
              className="btn-primary text-xs px-3 py-1 ml-auto"
            >
              Visit
            </a>
          </div>

          {resource.tags.length > 0 && (
            <div className="flex flex-wrap gap-1.5 mt-2">
              {resource.tags.slice(0, 4).map(tag => (
                <span
                  key={tag}
                  className="text-[10px] px-1.5 py-0.5 rounded bg-surface-secondary text-content-muted"
                >
                  #{tag}
                </span>
              ))}
              {resource.tags.length > 4 && (
                <span className="text-[10px] text-content-muted">
                  +{resource.tags.length - 4}
                </span>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default function ResourcesPage() {
  const [activeType, setActiveType] = useState<string>('book')
  const [activeCategory, setActiveCategory] = useState<string>('all')
  const [searchQuery, setSearchQuery] = useState('')
  const [sortBy, setSortBy] = useState<'rating' | 'title' | 'price'>('rating')

  const filteredResources = useMemo(() => {
    let result = RESOURCES.filter(r => r.type === activeType)

    if (activeCategory !== 'all') {
      result = result.filter(r => r.categories.includes(activeCategory))
    }

    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase()
      result = result.filter(r =>
        r.title.toLowerCase().includes(q) ||
        r.author?.toLowerCase().includes(q) ||
        r.description.toLowerCase().includes(q) ||
        r.tags.some(tag => tag.toLowerCase().includes(q))
      )
    }

    result.sort((a, b) => {
      if (sortBy === 'rating') return b.rating - a.rating
      if (sortBy === 'title') return a.title.localeCompare(b.title)
      return a.price.localeCompare(b.price)
    })

    return result
  }, [activeType, activeCategory, searchQuery, sortBy])

  return (
    <div className="min-h-screen px-4 py-8">
      <div className="max-w-4xl mx-auto">
        <div className="mb-8 animate-fadeIn">
          <h1 className="text-3xl font-bold gradient-text mb-2">Resources</h1>
          <p className="text-content-secondary text-sm">
            Curated books, apps, courses, and more to deepen your understanding of personality psychology
          </p>
        </div>

        {/* Type tabs */}
        <div className="flex flex-wrap gap-2 mb-6 animate-fadeIn delay-100">
          {RESOURCE_CATEGORIES.map(cat => (
            <button
              key={cat.id}
              onClick={() => { setActiveType(cat.id); setActiveCategory('all') }}
              className={`px-4 py-2 rounded-xl text-sm font-medium transition-all ${
                activeType === cat.id
                  ? 'bg-primary/20 text-primary shadow-sm'
                  : 'bg-surface text-content-muted hover:text-content hover:bg-surface-secondary border border-border'
              }`}
            >
              <span className="mr-1.5">{cat.icon}</span>
              {cat.label}
            </button>
          ))}
        </div>

        {/* Search & Filters */}
        <div className="flex flex-col sm:flex-row gap-3 mb-6 animate-fadeIn delay-200">
          <div className="flex-1 relative">
            <svg
              className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-content-muted"
              fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}
            >
              <path strokeLinecap="round" strokeLinejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
            </svg>
            <input
              type="text"
              placeholder={`Search ${activeType}s...`}
              value={searchQuery}
              onChange={e => setSearchQuery(e.target.value)}
              className="w-full pl-10 pr-4 py-2.5 bg-surface border border-border rounded-xl text-sm text-content placeholder:text-content-muted focus:outline-none focus:border-primary/50 transition-colors"
            />
          </div>
          <select
            value={sortBy}
            onChange={e => setSortBy(e.target.value as 'rating' | 'title' | 'price')}
            className="px-4 py-2.5 bg-surface border border-border rounded-xl text-sm text-content focus:outline-none focus:border-primary/50 transition-colors"
          >
            <option value="rating">Sort by Rating</option>
            <option value="title">Sort by Title</option>
            <option value="price">Sort by Price</option>
          </select>
        </div>

        {/* Category pills */}
        <div className="flex flex-wrap gap-1.5 mb-6 animate-fadeIn delay-300">
          <button
            onClick={() => setActiveCategory('all')}
            className={`text-xs px-3 py-1.5 rounded-full transition-colors ${
              activeCategory === 'all'
                ? 'bg-content text-bg-page'
                : 'bg-surface-secondary text-content-muted hover:text-content'
            }`}
          >
            All
          </button>
          {TYPE_CATEGORIES.slice(1).map(cat => (
            <button
              key={cat.id}
              onClick={() => setActiveCategory(cat.id)}
              className={`text-xs px-3 py-1.5 rounded-full transition-colors ${
                activeCategory === cat.id
                  ? 'bg-content text-bg-page'
                  : 'bg-surface-secondary text-content-muted hover:text-content'
              }`}
            >
              {cat.label}
            </button>
          ))}
        </div>

        {/* Results count */}
        <p className="text-xs text-content-muted mb-4 animate-fadeIn">
          {filteredResources.length} {activeType}{filteredResources.length !== 1 ? 's' : ''} found
        </p>

        {/* Resource list */}
        {filteredResources.length > 0 ? (
          <div className="space-y-4">
            {filteredResources.map(resource => (
              <ResourceCard key={resource.id} resource={resource} />
            ))}
          </div>
        ) : (
          <div className="card text-center py-12 animate-fadeIn">
            <p className="text-content-muted text-lg mb-2">No resources found</p>
            <p className="text-content-muted text-sm">
              Try adjusting your search or filters
            </p>
          </div>
        )}
      </div>
    </div>
  )
}
