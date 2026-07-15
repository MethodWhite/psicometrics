import type { ForumPost } from '../types'

const categoryLabels: Record<string, string> = {
  general: 'General',
  big_five: 'Big Five',
  mbti: 'MBTI',
  enneagram: 'Enneagram',
}

const categoryColors: Record<string, string> = {
  general: 'bg-slate-500/20 text-slate-300 border-slate-500/30',
  big_five: 'bg-indigo-500/20 text-indigo-300 border-indigo-500/30',
  mbti: 'bg-emerald-500/20 text-emerald-300 border-emerald-500/30',
  enneagram: 'bg-amber-500/20 text-amber-300 border-amber-500/30',
}

interface Props {
  post: ForumPost
  onClick: () => void
}

export function ForumPostCard({ post, onClick }: Props) {
  const timeAgo = (ts: number) => {
    const diff = Date.now() / 1000 - ts
    if (diff < 3600) return `${Math.floor(diff / 60)}m ago`
    if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`
    return `${Math.floor(diff / 86400)}d ago`
  }

  return (
    <button
      onClick={onClick}
      className="card w-full text-left cursor-pointer hover:shadow-lg transition-all duration-200 animate-fadeInUp"
    >
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-2">
            <span className={`text-xs font-medium px-2 py-0.5 rounded-full border ${categoryColors[post.category] || categoryColors.general}`}>
              {categoryLabels[post.category] || post.category}
            </span>
            {post.personality_type && (
              <span className="text-xs font-bold text-content-muted bg-surface-secondary px-2 py-0.5 rounded-full border border-border">
                {post.personality_type}
              </span>
            )}
          </div>
          <h3 className="text-lg font-bold text-content mb-1 truncate">{post.title}</h3>
          <p className="text-sm text-content-secondary line-clamp-2 mb-3">{post.content}</p>
          <div className="flex items-center gap-4 text-xs text-content-muted">
            <span>{post.author_name}</span>
            <span>{timeAgo(post.created_at)}</span>
          </div>
        </div>
      </div>
      <div className="flex items-center gap-4 mt-3 pt-3 border-t border-border">
        <div className="flex items-center gap-1 text-sm text-content-muted">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" className="w-4 h-4">
            <path d="M7.493 18.5c-.425 0-.82-.236-.975-.632A7.5 7.5 0 016 15.125a7.5 7.5 0 01.518-2.743c.155-.396.55-.632.975-.632a1 1 0 01.904.56 7.5 7.5 0 002.392 2.393c.345.207.422.672.157.971l-5.133 5.796c-.226.256-.56.37-.888.27z" />
          </svg>
          {post.likes}
        </div>
        <div className="flex items-center gap-1 text-sm text-content-muted">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" className="w-4 h-4">
            <path fillRule="evenodd" d="M4.848 2.771A49.144 49.144 0 0112 2.25c2.43 0 4.817.178 7.152.52 1.978.292 3.348 2.024 3.348 3.97v6.02c0 1.946-1.37 3.678-3.348 3.97-1.94.284-3.916.455-5.922.505a.39.39 0 00-.266.112L8.78 21.53A.75.75 0 017.5 21v-3.955a48.842 48.842 0 01-2.652-.316c-1.978-.29-3.348-2.024-3.348-3.97V6.741c0-1.946 1.37-3.68 3.348-3.97z" clipRule="evenodd" />
          </svg>
          {post.comment_count}
        </div>
      </div>
    </button>
  )
}
