import { type ComponentPropsWithoutRef } from 'react'

type SkeletonVariant = 'card' | 'text' | 'chart' | 'list'

interface LoadingSkeletonProps extends ComponentPropsWithoutRef<'div'> {
  variant?: SkeletonVariant
  lines?: number
}

const baseClass = 'skeleton-pulse rounded bg-border'

/* ── Variant renderers ── */

function CardSkeleton() {
  return (
    <div className="card space-y-4">
      <div className={`${baseClass} h-14 w-14 rounded-xl`} />
      <div className={`${baseClass} h-5 w-3/4`} />
      <div className={`${baseClass} h-4 w-full`} />
      <div className={`${baseClass} h-4 w-5/6`} />
      <div className="flex gap-4 pt-2">
        <div className={`${baseClass} h-3 w-16`} />
        <div className={`${baseClass} h-3 w-3`} />
        <div className={`${baseClass} h-3 w-20`} />
      </div>
      <div className={`${baseClass} h-4 w-24`} />
    </div>
  )
}

function TextSkeleton({ lines = 3 }: { lines?: number }) {
  return (
    <div className="space-y-3" role="status" aria-label="Loading content">
      {Array.from({ length: lines }, (_, i) => (
        <div
          key={i}
          className={`${baseClass} h-4`}
          style={{ width: `${80 - i * 12}%` }}
        />
      ))}
    </div>
  )
}

function ChartSkeleton() {
  return (
    <div className="card flex flex-col items-center gap-4">
      <div className={`${baseClass} h-48 w-full rounded-xl`} />
      <div className="flex gap-6">
        <div className={`${baseClass} h-3 w-16`} />
        <div className={`${baseClass} h-3 w-16`} />
        <div className={`${baseClass} h-3 w-16`} />
      </div>
    </div>
  )
}

function ListSkeleton({ lines = 4 }: { lines?: number }) {
  return (
    <div className="space-y-3" role="status" aria-label="Loading list">
      {Array.from({ length: lines }, (_, i) => (
        <div key={i} className="flex items-center gap-4 p-4 rounded-xl bg-surface">
          <div className={`${baseClass} h-10 w-10 rounded-lg shrink-0`} />
          <div className="flex-1 space-y-2">
            <div className={`${baseClass} h-4 w-2/5`} />
            <div className={`${baseClass} h-3 w-4/5`} />
          </div>
          <div className={`${baseClass} h-4 w-16 shrink-0`} />
        </div>
      ))}
    </div>
  )
}

/* ── Main component ── */
export function LoadingSkeleton({ variant = 'text', lines, className, ...rest }: LoadingSkeletonProps) {
  const variants: Record<SkeletonVariant, () => JSX.Element> = {
    card: CardSkeleton,
    text: () => <TextSkeleton lines={lines} />,
    chart: ChartSkeleton,
    list: () => <ListSkeleton lines={lines} />,
  }

  const VariantComponent = variants[variant]

  return (
    <div className={className} aria-busy="true" aria-label="Loading" role="status" {...rest}>
      <VariantComponent />
    </div>
  )
}
