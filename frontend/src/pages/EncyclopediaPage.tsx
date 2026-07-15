import { useState, useMemo } from 'react'
import { Link } from 'react-router-dom'
import { encyclopediaEntries } from '../data/encyclopedia'

const LETTERS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('')

export default function EncyclopediaPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [activeLetter, setActiveLetter] = useState<string | null>(null)
  const [expandedId, setExpandedId] = useState<string | null>(null)

  const filtered = useMemo(() => {
    let entries = encyclopediaEntries

    if (activeLetter) {
      entries = entries.filter(e => e.term[0].toUpperCase() === activeLetter)
    }

    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase()
      entries = entries.filter(e =>
        e.term.toLowerCase().includes(q) ||
        e.definition.toLowerCase().includes(q) ||
        e.category.toLowerCase().includes(q)
      )
    }

    return entries.sort((a, b) => a.term.localeCompare(b.term))
  }, [searchQuery, activeLetter])

  const categories = useMemo(() => {
    const cats = new Set(encyclopediaEntries.map(e => e.category))
    return Array.from(cats).sort()
  }, [])

  const [activeCategory, setActiveCategory] = useState<string | null>(null)

  const displayEntries = useMemo(() => {
    let entries = activeCategory
      ? filtered.filter(e => e.category === activeCategory)
      : filtered
    return entries
  }, [filtered, activeCategory])

  return (
    <div className="min-h-screen">
      <div className="max-w-4xl mx-auto px-4 py-12">
        {/* Header */}
        <div className="text-center mb-10">
          <h1 className="text-4xl md:text-5xl font-bold mb-4">
            <span className="gradient-text">Personality Encyclopedia</span>
          </h1>
          <p className="text-content-secondary text-lg max-w-2xl mx-auto">
            An A-Z reference of personality psychology terms, concepts, and frameworks.
            Search, browse by letter, or filter by category.
          </p>
        </div>

        {/* Search */}
        <div className="relative max-w-xl mx-auto mb-8">
          <svg
            className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-content-muted"
            fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}
          >
            <path strokeLinecap="round" strokeLinejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
          </svg>
          <input
            type="text"
            value={searchQuery}
            onChange={e => { setSearchQuery(e.target.value); setActiveLetter(null) }}
            placeholder="Search terms..."
            className="w-full pl-12 pr-4 py-3 rounded-xl bg-surface border border-border focus:border-primary/50 focus:outline-none transition-colors text-content"
          />
        </div>

        {/* A-Z Filter */}
        <div className="flex flex-wrap justify-center gap-1 mb-6">
          {LETTERS.map(letter => {
            const hasEntries = encyclopediaEntries.some(e => e.term[0].toUpperCase() === letter)
            return (
              <button
                key={letter}
                onClick={() => {
                  setActiveLetter(activeLetter === letter ? null : letter)
                  setSearchQuery('')
                }}
                disabled={!hasEntries}
                className={`w-8 h-8 text-xs font-medium rounded-lg transition-colors ${
                  activeLetter === letter
                    ? 'bg-primary text-white'
                    : hasEntries
                      ? 'text-content-secondary hover:bg-surface-secondary'
                      : 'text-content-muted/30 cursor-not-allowed'
                }`}
              >
                {letter}
              </button>
            )
          })}
        </div>

        {/* Category Filter */}
        <div className="flex flex-wrap justify-center gap-2 mb-10">
          <button
            onClick={() => setActiveCategory(null)}
            className={`px-3 py-1.5 text-xs font-medium rounded-full transition-colors ${
              activeCategory === null
                ? 'bg-primary/10 text-primary'
                : 'bg-surface border border-border text-content-muted hover:text-content'
            }`}
          >
            All
          </button>
          {categories.map(cat => (
            <button
              key={cat}
              onClick={() => setActiveCategory(activeCategory === cat ? null : cat)}
              className={`px-3 py-1.5 text-xs font-medium rounded-full transition-colors ${
                activeCategory === cat
                  ? 'bg-primary/10 text-primary'
                  : 'bg-surface border border-border text-content-muted hover:text-content'
              }`}
            >
              {cat}
            </button>
          ))}
        </div>

        {/* Results count */}
        <p className="text-sm text-content-muted mb-6">
          {displayEntries.length} term{displayEntries.length !== 1 ? 's' : ''}
          {activeCategory ? ` in ${activeCategory}` : ''}
          {activeLetter ? ` starting with ${activeLetter}` : ''}
        </p>

        {/* Entries */}
        <div className="space-y-3">
          {displayEntries.map((entry, i) => (
            <div
              key={entry.id}
              className="card animate-fadeInUp"
              style={{ animationDelay: `${i * 30}ms` }}
            >
              <button
                onClick={() => setExpandedId(expandedId === entry.id ? null : entry.id)}
                className="w-full flex items-center justify-between gap-4 text-left"
              >
                <div className="flex-1">
                  <h3 className="font-semibold text-content">{entry.term}</h3>
                  {expandedId !== entry.id && (
                    <p className="text-sm text-content-muted mt-1 line-clamp-2">
                      {entry.definition}
                    </p>
                  )}
                </div>
                <div className="flex items-center gap-3 shrink-0">
                  <span className="text-xs px-2 py-1 rounded-full bg-surface-secondary text-content-muted">
                    {entry.category}
                  </span>
                  <svg
                    className={`w-5 h-5 text-content-muted transition-transform duration-300 ${
                      expandedId === entry.id ? 'rotate-180' : ''
                    }`}
                    fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}
                  >
                    <path strokeLinecap="round" strokeLinejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
                  </svg>
                </div>
              </button>

              {expandedId === entry.id && (
                <div className="mt-4 pt-4 border-t border-border animate-fadeIn">
                  <p className="text-content-secondary leading-relaxed mb-4">
                    {entry.definition}
                  </p>

                  {entry.relatedTerms.length > 0 && (
                    <div className="flex flex-wrap items-center gap-2 mb-3">
                      <span className="text-xs text-content-muted">Related:</span>
                      {entry.relatedTerms.map(rt => {
                        const related = encyclopediaEntries.find(e => e.id === rt)
                        if (!related) return null
                        return (
                          <button
                            key={rt}
                            onClick={() => {
                              setExpandedId(rt)
                              setSearchQuery('')
                              setActiveLetter(null)
                              setActiveCategory(null)
                            }}
                            className="text-xs px-2 py-1 rounded-full bg-primary/5 text-primary hover:bg-primary/10 transition-colors"
                          >
                            {related.term}
                          </button>
                        )
                      })}
                    </div>
                  )}

                  {entry.relatedTest && (
                    <Link
                      to={`/test/${entry.relatedTest}`}
                      className="inline-flex items-center gap-2 text-sm text-primary hover:text-primary/80 transition-colors"
                    >
                      <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                        <path strokeLinecap="round" strokeLinejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                      </svg>
                      Take the {entry.relatedTest.replace('_', ' ')} test
                    </Link>
                  )}
                </div>
              )}
            </div>
          ))}
        </div>

        {displayEntries.length === 0 && (
          <div className="text-center py-16">
            <p className="text-content-muted text-lg">No terms found matching your search.</p>
            <button
              onClick={() => { setSearchQuery(''); setActiveLetter(null); setActiveCategory(null) }}
              className="btn-secondary mt-4 inline-block"
            >
              Clear filters
            </button>
          </div>
        )}
      </div>
    </div>
  )
}
