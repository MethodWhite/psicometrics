export interface NavItem {
  label: string
  path: string
  icon?: string
  children?: NavItem[]
}

export const NAVIGATION: NavItem[] = [
  { label: 'Tests', path: '/', icon: '🧠' },
  { label: 'Types', path: '/types', icon: '🎭', children: [
    { label: 'All Types', path: '/types' },
    { label: 'Compare Types', path: '/compare-types' },
  ]},
  { label: 'Articles', path: '/blog', icon: '📝' },
  { label: 'Resources', path: '/resources', icon: '📚' },
  { label: 'Community', path: '/forum', icon: '💬' },
  { label: 'Quizzes', path: '/quiz', icon: '🎯' },
  { label: 'Encyclopedia', path: '/encyclopedia', icon: '📖' },
]

export const RESOURCE_CATEGORIES = [
  { id: 'book', label: 'Books', icon: '📚' },
  { id: 'app', label: 'Apps', icon: '📱' },
  { id: 'course', label: 'Courses', icon: '🎓' },
  { id: 'podcast', label: 'Podcasts', icon: '🎙️' },
  { id: 'video', label: 'Videos', icon: '🎬' },
  { id: 'website', label: 'Websites', icon: '🌐' },
]
