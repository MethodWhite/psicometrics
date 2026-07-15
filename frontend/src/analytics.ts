/**
 * PostHog Analytics — PsicoMetrics
 *
 * Initializes PostHog with env toggle for dev/prod.
 * Provides typed helpers for page views and custom events.
 */

let posthog: any = null
let enabled = false

const POSTHOG_API_KEY = import.meta.env.VITE_POSTHOG_API_KEY ?? ''
const POSTHOG_HOST = import.meta.env.VITE_POSTHOG_HOST ?? 'https://app.posthog.com'
const IS_DEV = import.meta.env.DEV

export function initAnalytics(): void {
  if (enabled) return

  // Skip in dev unless explicitly enabled via env
  if (IS_DEV && !import.meta.env.VITE_POSTHOG_DEV) {
    console.log('[analytics] PostHog disabled in dev (set VITE_POSTHOG_DEV=1 to enable)')
    enabled = false
    return
  }

  if (!POSTHOG_API_KEY) {
    console.warn('[analytics] VITE_POSTHOG_API_KEY not set — PostHog disabled')
    return
  }

  // Dynamically import posthog-js (tree-shakeable)
  import('posthog-js').then((mod) => {
    posthog = mod.default
    posthog.init(POSTHOG_API_KEY, {
      api_host: POSTHOG_HOST,
      person_profiles: 'identified_only',
      capture_pageview: false, // we handle page views manually
      loaded: () => {
        enabled = true
        console.log('[analytics] PostHog initialized')
      },
    })
  }).catch((err) => {
    console.warn('[analytics] Failed to load posthog-js:', err)
  })
}

export function trackPageView(): void {
  if (!enabled || !posthog) return
  posthog.capture('$pageview')
}

export function trackEvent(name: string, properties?: Record<string, unknown>): void {
  if (!enabled || !posthog) return
  posthog.capture(name, properties)
}

// ── Convenience helpers ──────────────────────────────────────────────────

export function trackTestStarted(testType: string): void {
  trackEvent('test_started', { test_type: testType })
}

export function trackTestCompleted(testType: string, scores: Record<string, number>): void {
  trackEvent('test_completed', { test_type: testType, scores })
}

export function trackReportDownloaded(testType: string): void {
  trackEvent('report_downloaded', { test_type: testType })
}

export function trackAccountCreated(): void {
  trackEvent('account_created')
}

export function trackPremiumPurchased(tier: string): void {
  trackEvent('premium_purchased', { tier })
}

export function trackComparisonMade(): void {
  trackEvent('comparison_made')
}
