interface LikertScaleProps {
  value: number | null
  onChange: (value: number) => void
  labels: string[]
}

export function LikertScale({ value, onChange, labels }: LikertScaleProps) {
  return (
    <div className="flex flex-col gap-3">
      <div className="flex justify-between gap-2">
        {labels.map((label, index) => {
          const optionValue = index + 1
          return (
            <label
              key={optionValue}
              className={`flex-1 text-center p-3 rounded-xl border cursor-pointer transition-all duration-200 text-sm font-medium
                ${
                  value === optionValue
                    ? 'bg-indigo-500/20 border-indigo-500 text-white shadow-lg shadow-indigo-500/20'
                    : 'bg-white/5 border-white/10 text-slate-400 hover:bg-white/10 hover:border-white/20'
                }`}
            >
              <input
                type="radio"
                name="likert"
                value={optionValue}
                checked={value === optionValue}
                onChange={() => onChange(optionValue)}
                className="sr-only"
              />
              <span className="block text-lg mb-1">
                {value === optionValue ? '●' : '○'}
              </span>
              {label}
            </label>
          )
        })}
      </div>
    </div>
  )
}
