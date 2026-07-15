import { useState, useMemo } from 'react'
import { Link } from 'react-router-dom'
import { typeHubs, getTypeHub } from '../data/type-hubs'
import { getCompatibility } from '../data/type-comparisons'

const TYPE_CODES = typeHubs.map(t => t.typeCode)

export default function TypeComparisonPage() {
  const [typeA, setTypeA] = useState('INTJ')
  const [typeB, setTypeB] = useState('ENFP')

  const comparison = useMemo(() => {
    const hubA = getTypeHub(typeA)
    const hubB = getTypeHub(typeB)
    const compat = getCompatibility(typeA, typeB)
    return { hubA, hubB, compat }
  }, [typeA, typeB])

  const { hubA, hubB, compat } = comparison

  return (
    <div className="min-h-screen">
      <div className="max-w-6xl mx-auto px-4 py-12">
        {/* Header */}
        <div className="text-center mb-10">
          <h1 className="text-4xl md:text-5xl font-bold mb-4">
            <span className="gradient-text">Type Comparison</span>
          </h1>
          <p className="text-content-secondary text-lg max-w-2xl mx-auto">
            Compare any two MBTI personality types side by side. Discover compatibility,
            shared strengths, and potential challenges.
          </p>
        </div>

        {/* Type Selectors */}
        <div className="flex items-center gap-4 justify-center mb-10">
          <div className="flex-1 max-w-xs">
            <label className="block text-sm text-content-muted mb-2">Type A</label>
            <select
              value={typeA}
              onChange={e => setTypeA(e.target.value)}
              className="w-full p-3 rounded-xl bg-surface border border-border text-content focus:border-primary/50 focus:outline-none"
            >
              {TYPE_CODES.map(code => (
                <option key={code} value={code}>
                  {code} &mdash; {getTypeHub(code)?.name}
                </option>
              ))}
            </select>
          </div>

          <div className="pt-6">
            <svg className="w-8 h-8 text-content-muted" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
            </svg>
          </div>

          <div className="flex-1 max-w-xs">
            <label className="block text-sm text-content-muted mb-2">Type B</label>
            <select
              value={typeB}
              onChange={e => setTypeB(e.target.value)}
              className="w-full p-3 rounded-xl bg-surface border border-border text-content focus:border-primary/50 focus:outline-none"
            >
              {TYPE_CODES.map(code => (
                <option key={code} value={code}>
                  {code} &mdash; {getTypeHub(code)?.name}
                </option>
              ))}
            </select>
          </div>
        </div>

        {hubA && hubB && compat ? (
          <>
            {/* Compatibility Score */}
            <div className="card text-center mb-10 max-w-md mx-auto">
              <h2 className="text-lg font-semibold mb-4">Compatibility Score</h2>
              <div className="relative w-32 h-32 mx-auto mb-4">
                <svg className="w-32 h-32 -rotate-90" viewBox="0 0 120 120">
                  <circle cx="60" cy="60" r="54" fill="none" stroke="var(--color-border)" strokeWidth="8" />
                  <circle
                    cx="60" cy="60" r="54" fill="none"
                    stroke={
                      compat.score >= 80 ? '#22c55e' :
                      compat.score >= 60 ? '#eab308' :
                      compat.score >= 40 ? '#f97316' : '#ef4444'
                    }
                    strokeWidth="8"
                    strokeDasharray={`${2 * Math.PI * 54}`}
                    strokeDashoffset={`${2 * Math.PI * 54 * (1 - compat.score / 100)}`}
                    strokeLinecap="round"
                    className="transition-all duration-1000"
                  />
                </svg>
                <div className="absolute inset-0 flex items-center justify-center">
                  <span className={`text-3xl font-bold ${
                    compat.score >= 80 ? 'text-green-500' :
                    compat.score >= 60 ? 'text-yellow-500' :
                    compat.score >= 40 ? 'text-orange-500' : 'text-red-500'
                  }`}>
                    {compat.score}%
                  </span>
                </div>
              </div>
              <p className="text-content-secondary leading-relaxed">{compat.description}</p>
            </div>

            {/* Side by Side */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-10">
              {/* Type A */}
              <div className="card">
                <div className="flex items-center gap-3 mb-4">
                  <span className="text-3xl">{hubA.icon}</span>
                  <div>
                    <h3 className="text-xl font-bold">{hubA.typeCode}</h3>
                    <p className="text-sm text-content-muted">{hubA.name}</p>
                  </div>
                </div>

                <div className="mb-4">
                  <h4 className="text-sm font-semibold text-content-muted uppercase tracking-wider mb-2">Strengths</h4>
                  <div className="flex flex-wrap gap-1.5">
                    {hubA.strengths.slice(0, 6).map(s => (
                      <span key={s} className="text-xs px-2 py-1 rounded-full bg-green-500/10 text-green-600 dark:text-green-400">
                        {s}
                      </span>
                    ))}
                  </div>
                </div>

                <div className="mb-4">
                  <h4 className="text-sm font-semibold text-content-muted uppercase tracking-wider mb-2">Career</h4>
                  <div className="flex flex-wrap gap-1.5">
                    {hubA.careerPaths.slice(0, 5).map(c => (
                      <span key={c} className="text-xs px-2 py-1 rounded-full bg-surface-secondary text-content-secondary">
                        {c}
                      </span>
                    ))}
                  </div>
                </div>

                <div>
                  <h4 className="text-sm font-semibold text-content-muted uppercase tracking-wider mb-2">Communication</h4>
                  <p className="text-sm text-content-secondary leading-relaxed">{hubA.communicationStyle}</p>
                </div>
              </div>

              {/* Type B */}
              <div className="card">
                <div className="flex items-center gap-3 mb-4">
                  <span className="text-3xl">{hubB.icon}</span>
                  <div>
                    <h3 className="text-xl font-bold">{hubB.typeCode}</h3>
                    <p className="text-sm text-content-muted">{hubB.name}</p>
                  </div>
                </div>

                <div className="mb-4">
                  <h4 className="text-sm font-semibold text-content-muted uppercase tracking-wider mb-2">Strengths</h4>
                  <div className="flex flex-wrap gap-1.5">
                    {hubB.strengths.slice(0, 6).map(s => (
                      <span key={s} className="text-xs px-2 py-1 rounded-full bg-green-500/10 text-green-600 dark:text-green-400">
                        {s}
                      </span>
                    ))}
                  </div>
                </div>

                <div className="mb-4">
                  <h4 className="text-sm font-semibold text-content-muted uppercase tracking-wider mb-2">Career</h4>
                  <div className="flex flex-wrap gap-1.5">
                    {hubB.careerPaths.slice(0, 5).map(c => (
                      <span key={c} className="text-xs px-2 py-1 rounded-full bg-surface-secondary text-content-secondary">
                        {c}
                      </span>
                    ))}
                  </div>
                </div>

                <div>
                  <h4 className="text-sm font-semibold text-content-muted uppercase tracking-wider mb-2">Communication</h4>
                  <p className="text-sm text-content-secondary leading-relaxed">{hubB.communicationStyle}</p>
                </div>
              </div>
            </div>

            {/* Detailed comparison table */}
            <div className="card mb-10">
              <h3 className="text-lg font-bold mb-4">Detailed Comparison</h3>
              <div className="overflow-x-auto">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b border-border">
                      <th className="text-left py-3 pr-4 font-semibold">Aspect</th>
                      <th className="text-left py-3 px-4 font-semibold" style={{ color: hubA.color }}>{hubA.typeCode}</th>
                      <th className="text-left py-3 px-4 font-semibold" style={{ color: hubB.color }}>{hubB.typeCode}</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-border">
                    <tr>
                      <td className="py-3 pr-4 font-medium text-content-muted">Name</td>
                      <td className="py-3 px-4 text-content-secondary">{hubA.name}</td>
                      <td className="py-3 px-4 text-content-secondary">{hubB.name}</td>
                    </tr>
                    <tr>
                      <td className="py-3 pr-4 font-medium text-content-muted">Icon</td>
                      <td className="py-3 px-4">{hubA.icon}</td>
                      <td className="py-3 px-4">{hubB.icon}</td>
                    </tr>
                    <tr>
                      <td className="py-3 pr-4 font-medium text-content-muted">Strengths</td>
                      <td className="py-3 px-4">
                        <ul className="list-disc list-inside space-y-1">
                          {hubA.strengths.slice(0, 5).map(s => <li key={s}>{s}</li>)}
                        </ul>
                      </td>
                      <td className="py-3 px-4">
                        <ul className="list-disc list-inside space-y-1">
                          {hubB.strengths.slice(0, 5).map(s => <li key={s}>{s}</li>)}
                        </ul>
                      </td>
                    </tr>
                    <tr>
                      <td className="py-3 pr-4 font-medium text-content-muted">Weaknesses</td>
                      <td className="py-3 px-4">
                        <ul className="list-disc list-inside space-y-1">
                          {hubA.weaknesses.slice(0, 5).map(w => <li key={w}>{w}</li>)}
                        </ul>
                      </td>
                      <td className="py-3 px-4">
                        <ul className="list-disc list-inside space-y-1">
                          {hubB.weaknesses.slice(0, 5).map(w => <li key={w}>{w}</li>)}
                        </ul>
                      </td>
                    </tr>
                    <tr>
                      <td className="py-3 pr-4 font-medium text-content-muted">Top Careers</td>
                      <td className="py-3 px-4">
                        <ul className="list-disc list-inside space-y-1">
                          {hubA.careerPaths.slice(0, 4).map(c => <li key={c}>{c}</li>)}
                        </ul>
                      </td>
                      <td className="py-3 px-4">
                        <ul className="list-disc list-inside space-y-1">
                          {hubB.careerPaths.slice(0, 4).map(c => <li key={c}>{c}</li>)}
                        </ul>
                      </td>
                    </tr>
                    <tr>
                      <td className="py-3 pr-4 font-medium text-content-muted">Compatible With</td>
                      <td className="py-3 px-4">
                        <div className="flex flex-wrap gap-1">
                          {hubA.compatibleTypes.map(t => (
                            <Link key={t} to={`/types/${t.toLowerCase()}`} className="text-xs px-2 py-0.5 rounded-full bg-primary/5 text-primary hover:bg-primary/10">
                              {t}
                            </Link>
                          ))}
                        </div>
                      </td>
                      <td className="py-3 px-4">
                        <div className="flex flex-wrap gap-1">
                          {hubB.compatibleTypes.map(t => (
                            <Link key={t} to={`/types/${t.toLowerCase()}`} className="text-xs px-2 py-0.5 rounded-full bg-primary/5 text-primary hover:bg-primary/10">
                              {t}
                            </Link>
                          ))}
                        </div>
                      </td>
                    </tr>
                    <tr>
                      <td className="py-3 pr-4 font-medium text-content-muted">Stress Response</td>
                      <td className="py-3 px-4 text-content-secondary">{hubA.stressResponse.slice(0, 120)}...</td>
                      <td className="py-3 px-4 text-content-secondary">{hubB.stressResponse.slice(0, 120)}...</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>

            {/* Explore individual types */}
            <div className="flex justify-center gap-4">
              <Link to={`/types/${typeA.toLowerCase()}`} className="btn-secondary">
                Explore {hubA.typeCode}
              </Link>
              <Link to={`/types/${typeB.toLowerCase()}`} className="btn-secondary">
                Explore {hubB.typeCode}
              </Link>
            </div>
          </>
        ) : (
          <div className="text-center py-16">
            <p className="text-content-muted">Select two types to compare them.</p>
          </div>
        )}
      </div>
    </div>
  )
}
