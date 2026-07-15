interface BarChartProps {
  data: { label: string; value: number; color: string }[]
  maxValue?: number
}

export function BarChart({ data, maxValue = 100 }: BarChartProps) {
  return (
    <div className="space-y-4">
      {data.map((item) => (
        <div key={item.label} className="space-y-1">
          <div className="flex justify-between text-sm">
            <span className="text-content-secondary font-medium">{item.label}</span>
            <span className="text-content-muted">{item.value.toFixed(1)}%</span>
          </div>
          <div className="h-3 bg-border rounded-full overflow-hidden">
            <div
              className={`h-full rounded-full transition-all duration-700 ease-out ${item.color}`}
              style={{ width: `${(item.value / maxValue) * 100}%` }}
            />
          </div>
        </div>
      ))}
    </div>
  )
}
