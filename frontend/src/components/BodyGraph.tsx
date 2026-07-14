import { useTranslation } from 'react-i18next'

interface BodyGraphProps {
  centers: Record<string, boolean>
}

const CENTER_POSITIONS: Record<string, { x: number; y: number; label: { es: string; en: string } }> = {
  head: { x: 150, y: 30, label: { es: 'Corona', en: 'Head' } },
  ajna: { x: 150, y: 90, label: { es: 'Ajna', en: 'Ajna' } },
  throat: { x: 150, y: 150, label: { es: 'Garganta', en: 'Throat' } },
  g: { x: 150, y: 220, label: { es: 'G/Identidad', en: 'G/Identity' } },
  heart: { x: 80, y: 200, label: { es: 'Corazón', en: 'Heart' } },
  sacral: { x: 150, y: 310, label: { es: 'Sacral', en: 'Sacral' } },
  solar_plexus: { x: 220, y: 280, label: { es: 'Plexo Solar', en: 'Solar Plexus' } },
  splenic: { x: 80, y: 300, label: { es: 'Splénico', en: 'Splenic' } },
  root: { x: 150, y: 380, label: { es: 'Raíz', en: 'Root' } },
}

export function BodyGraph({ centers }: BodyGraphProps) {
  const { i18n } = useTranslation()
  const lang = i18n.language as 'es' | 'en'

  return (
    <svg viewBox="0 0 300 420" className="w-full max-w-sm mx-auto">
      {/* Channels (simplified connections) */}
      <g stroke="rgba(255,255,255,0.1)" strokeWidth="2" fill="none">
        <line x1="150" y1="50" x2="150" y2="70" /> {/* Head to Ajna */}
        <line x1="150" y1="110" x2="150" y2="130" /> {/* Ajna to Throat */}
        <line x1="150" y1="170" x2="150" y2="200" /> {/* Throat to G */}
        <line x1="150" y1="240" x2="150" y2="290" /> {/* G to Sacral */}
        <line x1="150" y1="330" x2="150" y2="360" /> {/* Sacral to Root */}
        <line x1="100" y1="210" x2="130" y2="210" /> {/* Heart to G */}
        <line x1="170" y1="240" x2="200" y2="260" /> {/* G to Solar Plexus */}
        <line x1="100" y1="280" x2="130" y2="290" /> {/* Splenic to Sacral */}
      </g>

      {/* Centers */}
      {Object.entries(CENTER_POSITIONS).map(([key, pos]) => {
        const defined = centers[key] ?? false
        return (
          <g key={key}>
            <polygon
              points={
                key === 'head' || key === 'root'
                  ? `${pos.x},${pos.y - 15} ${pos.x + 15},${pos.y + 5} ${pos.x - 15},${pos.y + 5}`
                  : key === 'heart'
                  ? `${pos.x},${pos.y - 12} ${pos.x + 12},${pos.y} ${pos.x},${pos.y + 12} ${pos.x - 12},${pos.y}`
                  : `${pos.x - 15},${pos.y - 15} ${pos.x + 15},${pos.y - 15} ${pos.x + 15},${pos.y + 15} ${pos.x - 15},${pos.y + 15}`
              }
              fill={defined ? '#6366f1' : 'transparent'}
              stroke={defined ? '#6366f1' : 'rgba(255,255,255,0.3)'}
              strokeWidth="2"
              className="transition-all duration-500"
            />
            <text
              x={pos.x}
              y={pos.y + 30}
              textAnchor="middle"
              className="fill-slate-400 text-[8px]"
            >
              {pos.label[lang]}
            </text>
          </g>
        )
      })}
    </svg>
  )
}
