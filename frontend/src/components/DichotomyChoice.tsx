interface DichotomyChoiceProps {
  value: string | null
  onChange: (value: string) => void
  optionA: string
  optionB: string
}

export function DichotomyChoice({ value, onChange, optionA, optionB }: DichotomyChoiceProps) {
  return (
    <div className="flex gap-4">
      <label
        className={`flex-1 text-center p-6 rounded-2xl border-2 cursor-pointer transition-all duration-300
          ${
            value === 'a'
              ? 'bg-indigo-500/20 border-indigo-500 text-content shadow-xl shadow-indigo-500/20 scale-105'
              : 'bg-surface-secondary border-border text-content-secondary hover:bg-surface-elevated'
          }`}
      >
        <input
          type="radio"
          name="dichotomy"
          value="a"
          checked={value === 'a'}
          onChange={() => onChange('a')}
          className="sr-only"
        />
        <span className="block text-3xl mb-2">
          {value === 'a' ? '◆' : '◇'}
        </span>
        <span className="text-lg font-semibold">{optionA}</span>
      </label>

      <div className="flex items-center text-content-muted text-lg font-bold">VS</div>

      <label
        className={`flex-1 text-center p-6 rounded-2xl border-2 cursor-pointer transition-all duration-300
          ${
            value === 'b'
              ? 'bg-purple-500/20 border-purple-500 text-content shadow-xl shadow-purple-500/20 scale-105'
              : 'bg-surface-secondary border-border text-content-secondary hover:bg-surface-elevated'
          }`}
      >
        <input
          type="radio"
          name="dichotomy"
          value="b"
          checked={value === 'b'}
          onChange={() => onChange('b')}
          className="sr-only"
        />
        <span className="block text-3xl mb-2">
          {value === 'b' ? '◆' : '◇'}
        </span>
        <span className="text-lg font-semibold">{optionB}</span>
      </label>
    </div>
  )
}
