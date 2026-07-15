import type { Testimonial } from '../types'

interface Props {
  testimonial: Testimonial
}

export function TestimonialCard({ testimonial }: Props) {
  return (
    <div className="card animate-fadeInUp">
      <div className="flex items-center gap-1 mb-3">
        {[1, 2, 3, 4, 5].map((star) => (
          <svg
            key={star}
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            className={`w-4 h-4 ${star <= testimonial.rating ? 'text-amber-400' : 'text-border'}`}
            fill="currentColor"
          >
            <path fillRule="evenodd" d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.006 5.404.434c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.434 2.082-5.005z" clipRule="evenodd" />
          </svg>
        ))}
      </div>
      <p className="text-content-secondary italic mb-4 leading-relaxed">"{testimonial.text}"</p>
      <div className="flex items-center justify-between">
        <span className="font-semibold text-content">{testimonial.name}</span>
        <span className="text-xs font-bold text-content-muted bg-surface-secondary px-2 py-1 rounded-full border border-border">
          {testimonial.personality_type}
        </span>
      </div>
    </div>
  )
}
