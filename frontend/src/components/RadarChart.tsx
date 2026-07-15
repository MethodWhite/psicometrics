interface RadarChartProps {
  data: { label: string; value: number }[]
  size?: number
}

export function RadarChart({ data, size = 300 }: RadarChartProps) {
  const center = size / 2
  const radius = size * 0.35
  const angleStep = (2 * Math.PI) / data.length

  const getPoint = (index: number, value: number) => {
    const angle = angleStep * index - Math.PI / 2
    const r = (value / 100) * radius
    return {
      x: center + r * Math.cos(angle),
      y: center + r * Math.sin(angle),
    }
  }

  const points = data.map((item, i) => getPoint(i, item.value))
  const polygonPoints = points.map((p) => `${p.x},${p.y}`).join(' ')

  const gridLevels = [20, 40, 60, 80, 100]

  return (
    <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} className="text-border">
      {/* Grid circles */}
      {gridLevels.map((level) => (
        <circle
          key={level}
          cx={center}
          cy={center}
          r={(level / 100) * radius}
          fill="none"
          stroke="currentColor"
          strokeWidth="1"
        />
      ))}

      {/* Axis lines */}
      {data.map((_, i) => {
        const angle = angleStep * i - Math.PI / 2
        return (
          <line
            key={i}
            x1={center}
            y1={center}
            x2={center + radius * Math.cos(angle)}
            y2={center + radius * Math.sin(angle)}
            stroke="currentColor"
            strokeWidth="1"
          />
        )
      })}

      {/* Data polygon */}
      <polygon
        points={polygonPoints}
        fill="rgba(99, 102, 241, 0.2)"
        stroke="#6366f1"
        strokeWidth="2"
      />

      {/* Data points */}
      {points.map((p, i) => (
        <circle
          key={i}
          cx={p.x}
          cy={p.y}
          r="5"
          fill="#6366f1"
          stroke="currentColor"
          strokeWidth="2"
        />
      ))}

      {/* Labels */}
      {data.map((item, i) => {
        const angle = angleStep * i - Math.PI / 2
        const labelRadius = radius + 30
        const x = center + labelRadius * Math.cos(angle)
        const y = center + labelRadius * Math.sin(angle)
        return (
          <text
            key={i}
            x={x}
            y={y}
            textAnchor="middle"
            dominantBaseline="middle"
            className="fill-content-muted text-xs font-medium"
          >
            {item.label}
          </text>
        )
      })}
    </svg>
  )
}
