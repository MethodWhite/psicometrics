import type { UserStory } from '../types'

interface Props {
  story: UserStory
  onClick: () => void
}

export function StoryCard({ story, onClick }: Props) {
  const timeAgo = (ts: number) => {
    const diff = Date.now() / 1000 - ts
    if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`
    if (diff < 2592000) return `${Math.floor(diff / 86400)}d ago`
    return `${Math.floor(diff / 2592000)}mo ago`
  }

  // Extract first heading from markdown for preview
  const preview = story.content
    .replace(/^##\s+.*$/m, '')
    .replace(/[#*\[\]]/g, '')
    .trim()
    .slice(0, 200)

  return (
    <button
      onClick={onClick}
      className="card w-full text-left cursor-pointer hover:shadow-lg transition-all duration-200 animate-fadeInUp"
    >
      {story.featured && (
        <span className="inline-block text-xs font-bold text-amber-400 bg-amber-500/10 border border-amber-500/30 px-2 py-0.5 rounded-full mb-3">
          Featured
        </span>
      )}
      <h3 className="text-xl font-bold text-content mb-2">{story.title}</h3>
      <p className="text-sm text-content-secondary leading-relaxed mb-4 line-clamp-3">
        {preview}...
      </p>
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3 text-xs text-content-muted">
          <span className="font-medium text-content-secondary">{story.author_name}</span>
          <span className="text-xs font-bold bg-surface-secondary px-2 py-0.5 rounded-full border border-border">
            {story.personality_type}
          </span>
          <span>{timeAgo(story.created_at)}</span>
        </div>
        <div className="flex items-center gap-1 text-sm text-content-muted">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" className="w-4 h-4">
            <path d="M7.493 18.5c-.425 0-.82-.236-.975-.632A7.5 7.5 0 016 15.125a7.5 7.5 0 01.518-2.743c.155-.396.55-.632.975-.632a1 1 0 01.904.56 7.5 7.5 0 002.392 2.393c.345.207.422.672.157.971l-5.133 5.796c-.226.256-.56.37-.888.27z" />
          </svg>
          {story.likes}
        </div>
      </div>
    </button>
  )
}
