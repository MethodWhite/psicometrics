import { useState, useRef, useEffect, type ReactNode } from 'react'
import { Link } from 'react-router-dom'
import { getGlossaryTerm } from '../data/glossary'

interface GlossaryPopoverProps {
  term: string
  children?: ReactNode
}

export function GlossaryPopover({ term, children }: GlossaryPopoverProps) {
  const [isVisible, setIsVisible] = useState(false)
  const [position, setPosition] = useState<'top' | 'bottom'>('bottom')
  const triggerRef = useRef<HTMLSpanElement>(null)
  const popoverRef = useRef<HTMLDivElement>(null)

  const entry = getGlossaryTerm(term)

  useEffect(() => {
    if (isVisible && triggerRef.current) {
      const rect = triggerRef.current.getBoundingClientRect()
      const spaceBelow = window.innerHeight - rect.bottom
      setPosition(spaceBelow < 250 ? 'top' : 'bottom')
    }
  }, [isVisible])

  if (!entry) return <>{children || term}</>

  return (
    <span
      ref={triggerRef}
      className="relative inline cursor-help border-b border-dotted border-primary/40 text-primary/80 hover:text-primary transition-colors"
      onMouseEnter={() => setIsVisible(true)}
      onMouseLeave={() => setIsVisible(false)}
      onClick={() => setIsVisible(!isVisible)}
    >
      {children || term}

      {isVisible && (
        <div
          ref={popoverRef}
          className={`absolute z-50 left-1/2 -translate-x-1/2 w-72 p-4 rounded-xl bg-surface-elevated border border-border shadow-xl ${
            position === 'top' ? 'bottom-full mb-2' : 'top-full mt-2'
          }`}
          onMouseEnter={() => setIsVisible(true)}
          onMouseLeave={() => setIsVisible(false)}
        >
          <p className="text-sm font-semibold text-content mb-1">{entry.term}</p>
          <p className="text-xs text-content-secondary leading-relaxed mb-2">{entry.definition}</p>
          {entry.encyclopediaId && (
            <Link
              to={`/encyclopedia#${entry.encyclopediaId}`}
              className="text-xs text-primary hover:text-primary/80 transition-colors inline-flex items-center gap-1"
            >
              Read more in Encyclopedia
              <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="m4.5 19.5 15-15m0 0H8.25m11.25 0v11.25" />
              </svg>
            </Link>
          )}
        </div>
      )}
    </span>
  )
}
