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
              ? 'bg-indigo-500/20 border-indigo-500 text-white shadow-xl shadow-indigo-500/20 scale-105'
              : 'bg-white/5 border-white/10 text-slate-400 hover:bg-white/10 hover:border-white/20'
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

      <div className="flex items-center text-slate-600 text-lg font-bold">VS</div>

      <label
        className={`flex-1 text-center p-6 rounded-2xl border-2 cursor-pointer transition-all duration-300
          ${
            value === 'b'
              ? 'bg-purple-500/20 border-purple-500 text-white shadow-xl shadow-purple-500/20 scale-105'
              : 'bg-white/5 border-white/10 text-slate-400 hover:bg-white/10 hover:border-white/20'
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
