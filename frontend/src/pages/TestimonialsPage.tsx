import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import type { Testimonial } from '../types'
import { TestimonialCard } from '../components/TestimonialCard'
import { LoadingSkeleton } from '../components/LoadingSkeleton'

export function TestimonialsPage() {
  const navigate = useNavigate()
  const [testimonials, setTestimonials] = useState<Testimonial[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [page, setPage] = useState(1)

  // New testimonial form
  const [showForm, setShowForm] = useState(false)
  const [form, setForm] = useState({ name: '', text: '', personality_type: '', rating: 5 })
  const [submitting, setSubmitting] = useState(false)
  const [formError, setFormError] = useState('')

  useEffect(() => {
    setLoading(true)
    fetch(`/api/v1/community/testimonials?page=${page}&per_page=20`)
      .then((r) => {
        if (!r.ok) throw new Error('Failed to load testimonials')
        return r.json()
      })
      .then((data: Testimonial[]) => {
        setTestimonials(data)
        setLoading(false)
      })
      .catch((err) => {
        setError(err.message)
        setLoading(false)
      })
  }, [page])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!form.name.trim() || !form.text.trim() || !form.personality_type.trim()) return
    setSubmitting(true)
    setFormError('')
    try {
      await fetch('/api/v1/community/testimonials', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(form),
      })
      setForm({ name: '', text: '', personality_type: '', rating: 5 })
      setShowForm(false)
      // Refresh
      const data = await fetch(`/api/v1/community/testimonials?page=1&per_page=20`).then((r) => r.json())
      setTestimonials(data as Testimonial[])
      setPage(1)
    } catch (err) {
      setFormError(err instanceof Error ? err.message : 'Failed to submit')
    } finally {
      setSubmitting(false)
    }
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
          <h1 className="text-2xl font-bold text-content">Testimonials</h1>
          <p className="text-content-secondary mt-1">
            See what others are saying about their Psicometrics experience
          </p>
        </div>

        <button
          onClick={() => setShowForm(!showForm)}
          className="btn-primary text-sm mb-6"
        >
          {showForm ? 'Cancel' : 'Share Your Experience'}
        </button>

        {/* Submit form */}
        {showForm && (
          <div className="card mb-6 animate-fadeIn">
            <h3 className="text-lg font-bold text-content mb-4">Share Your Testimonial</h3>
            <form onSubmit={handleSubmit} className="space-y-3">
              <input
                type="text"
                value={form.name}
                onChange={(e) => setForm({ ...form, name: e.target.value })}
                placeholder="Your name"
                className="w-full bg-black/30 border border-border rounded-xl px-4 py-2.5 text-content text-sm focus:outline-none focus:border-indigo-500"
              />
              <textarea
                value={form.text}
                onChange={(e) => setForm({ ...form, text: e.target.value })}
                placeholder="Share your experience..."
                rows={4}
                className="w-full bg-black/30 border border-border rounded-xl px-4 py-2.5 text-content text-sm focus:outline-none focus:border-indigo-500 resize-y"
              />
              <input
                type="text"
                value={form.personality_type}
                onChange={(e) => setForm({ ...form, personality_type: e.target.value })}
                placeholder="Your personality type (e.g. INTJ, Type 5)"
                className="w-full bg-black/30 border border-border rounded-xl px-4 py-2.5 text-content text-sm focus:outline-none focus:border-indigo-500"
              />
              <div className="flex items-center gap-2">
                <span className="text-sm text-content-muted">Rating:</span>
                {[1, 2, 3, 4, 5].map((star) => (
                  <button
                    key={star}
                    type="button"
                    onClick={() => setForm({ ...form, rating: star })}
                    className="p-0.5"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"
                      className={`w-6 h-6 ${star <= form.rating ? 'text-amber-400' : 'text-border'}`}
                    >
                      <path fillRule="evenodd" d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.006 5.404.434c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.434 2.082-5.005z" clipRule="evenodd" />
                    </svg>
                  </button>
                ))}
              </div>
              {formError && <p className="text-red-400 text-sm">{formError}</p>}
              <button
                type="submit"
                disabled={submitting || !form.name.trim() || !form.text.trim()}
                className="btn-primary text-sm disabled:opacity-50"
              >
                {submitting ? 'Submitting...' : 'Submit Testimonial'}
              </button>
            </form>
          </div>
        )}

        {error && (
          <div className="card border-red-500/30 bg-red-500/10 mb-6">
            <p className="text-red-400 text-sm">{error}</p>
          </div>
        )}

        {loading ? (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {[1, 2, 3, 4].map((i) => (
              <LoadingSkeleton key={i} variant="card" />
            ))}
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {testimonials.length === 0 ? (
              <div className="col-span-full card text-center py-12">
                <p className="text-content-muted">No testimonials yet.</p>
              </div>
            ) : (
              testimonials.map((t) => (
                <TestimonialCard key={t.id} testimonial={t} />
              ))
            )}
          </div>
        )}

        {!loading && testimonials.length > 0 && (
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
              disabled={testimonials.length < 20}
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
