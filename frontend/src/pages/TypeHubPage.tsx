import { useState, useEffect } from 'react'
import { Link, useSearchParams } from 'react-router-dom'
import { typeHubs, TYPE_CATEGORIES, type TypeCategory } from '../data/type-hubs'

const CATEGORIES: { key: TypeCategory | 'all'; label: string; description: string }[] = [
  { key: 'all', label: 'All Types', description: 'All 16 MBTI personality types' },
  { key: 'Analysts', label: 'Analysts', description: 'Rational, strategic, and intellectually curious' },
  { key: 'Diplomats', label: 'Diplomats', description: 'Empathetic, idealistic, and people-oriented' },
  { key: 'Sentinels', label: 'Sentinels', description: 'Practical, dependable, and detail-oriented' },
  { key: 'Explorers', label: 'Explorers', description: 'Spontaneous, energetic, and hands-on' },
]

export default function TypeHubPage() {
  const [searchParams, setSearchParams] = useSearchParams()
  const [activeCategory, setActiveCategory] = useState<TypeCategory | 'all'>('all')

  useEffect(() => {
    const cat = searchParams.get('category')
    if (cat && (cat === 'Analysts' || cat === 'Diplomats' || cat === 'Sentinels' || cat === 'Explorers')) {
      setActiveCategory(cat)
    }
  }, [searchParams])

  function handleCategoryChange(cat: TypeCategory | 'all') {
    setActiveCategory(cat)
    if (cat === 'all') {
      setSearchParams({})
    } else {
      setSearchParams({ category: cat })
    }
  }

  const filtered = activeCategory === 'all'
    ? typeHubs
    : typeHubs.filter(t => TYPE_CATEGORIES[t.typeCode]?.category === activeCategory)

  return (
    <div className="min-h-screen">
      <div className="max-w-6xl mx-auto px-4 py-12">
        {/* Header */}
        <div className="text-center mb-12">
          <h1 className="text-4xl md:text-5xl font-bold mb-4">
            <span className="gradient-text">MBTI Type Hub</span>
          </h1>
          <p className="text-content-secondary text-lg max-w-2xl mx-auto">
            Explore all 16 MBTI personality types in depth. Learn about their strengths, weaknesses,
            relationships, career paths, and more.
          </p>
        </div>

        {/* Category Filter */}
        <div className="flex flex-wrap justify-center gap-3 mb-12">
          {CATEGORIES.map(cat => (
            <button
              key={cat.key}
              onClick={() => handleCategoryChange(cat.key)}
              className={`px-5 py-2.5 rounded-xl text-sm font-medium transition-all duration-300 ${
                activeCategory === cat.key
                  ? 'bg-primary text-white shadow-lg shadow-primary/30'
                  : 'bg-surface border border-border text-content-secondary hover:border-primary/30 hover:text-content'
              }`}
            >
              {cat.label}
            </button>
          ))}
        </div>

        {/* Category description */}
        {activeCategory !== 'all' && (
          <p className="text-center text-content-muted mb-8">
            {CATEGORIES.find(c => c.key === activeCategory)?.description}
          </p>
        )}

        {/* Type Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {filtered.map((type, i) => (
            <Link
              key={type.typeCode}
              to={`/types/${type.typeCode.toLowerCase()}`}
              className="card group hover:scale-[1.02] transition-all duration-300 animate-fadeInUp"
              style={{ animationDelay: `${i * 60}ms` }}
            >
              <div className="flex items-center gap-3 mb-4">
                <span className="text-3xl">{type.icon}</span>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: type.color }}>
                    {type.typeCode}
                  </h3>
                  <p className="text-sm text-content-muted">{type.name}</p>
                </div>
              </div>
              <p className="text-sm text-content-secondary leading-relaxed line-clamp-3">
                {TYPE_CATEGORIES[type.typeCode]?.description}
              </p>
              <div className="mt-4 flex items-center gap-2 text-sm text-primary opacity-0 group-hover:opacity-100 transition-opacity">
                <span>Explore type</span>
                <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="m4.5 19.5 15-15m0 0H8.25m11.25 0v11.25" />
                </svg>
              </div>
            </Link>
          ))}
        </div>
      </div>
    </div>
  )
}
