import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import { MemoryRouter } from 'react-router-dom'
import { TestCard } from '../components/TestCard'
import { LikertScale } from '../components/LikertScale'
import { BarChart } from '../components/BarChart'

// ─── TestCard ──────────────────────────────────────────────────────────

describe('TestCard', () => {
  it('renders test name and description', () => {
    render(
      <MemoryRouter>
        <TestCard
          testType="big_five"
          name="Big Five"
          description="Measures OCEAN personality traits"
        />
      </MemoryRouter>
    )
    expect(screen.getByText('Big Five')).toBeInTheDocument()
    expect(screen.getByText('Measures OCEAN personality traits')).toBeInTheDocument()
  })

  it('links to the correct test path', () => {
    render(
      <MemoryRouter>
        <TestCard
          testType="mbti"
          name="MBTI"
          description="Myers-Briggs Type Indicator"
        />
      </MemoryRouter>
    )
    const link = screen.getByRole('link')
    expect(link).toHaveAttribute('href', '/test/mbti')
  })

  it('renders different test types', () => {
    const types = [
      { testType: 'enneagram', name: 'Enneagram', desc: '9 personality types' },
      { testType: 'disc', name: 'DISC', desc: 'Behavioral assessment' },
      { testType: 'dark_triad', name: 'Dark Triad', desc: 'Dark personality traits' },
    ]
    for (const t of types) {
      const { unmount } = render(
        <MemoryRouter>
          <TestCard testType={t.testType} name={t.name} description={t.desc} />
        </MemoryRouter>
      )
      expect(screen.getByText(t.name)).toBeInTheDocument()
      expect(screen.getByText(t.desc)).toBeInTheDocument()
      unmount()
    }
  })
})

// ─── LikertScale ──────────────────────────────────────────────────────

describe('LikertScale', () => {
  const labels = ['Strongly Disagree', 'Disagree', 'Neutral', 'Agree', 'Strongly Agree']

  it('renders all option labels', () => {
    render(<LikertScale value={null} onChange={() => {}} labels={labels} />)
    for (const label of labels) {
      expect(screen.getByText(label)).toBeInTheDocument()
    }
  })

  it('calls onChange when an option is clicked', () => {
    const onChange = vi.fn()
    render(<LikertScale value={null} onChange={onChange} labels={labels} />)
    fireEvent.click(screen.getByText('Agree'))
    expect(onChange).toHaveBeenCalledWith(4)
  })

  it('highlights selected value', () => {
    render(<LikertScale value={3} onChange={() => {}} labels={labels} />)
    const neutral = screen.getByText('Neutral')
    expect(neutral.className).toContain('bg-indigo')
  })

  it('calls onChange with correct index for each option', () => {
    const onChange = vi.fn()
    render(<LikertScale value={null} onChange={onChange} labels={labels} />)
    fireEvent.click(screen.getByText('Strongly Disagree'))
    expect(onChange).toHaveBeenCalledWith(1)
    fireEvent.click(screen.getByText('Strongly Agree'))
    expect(onChange).toHaveBeenCalledWith(5)
  })

  it('handles empty labels array', () => {
    const { container } = render(<LikertScale value={null} onChange={() => {}} labels={[]} />)
    expect(container.firstChild).toBeInTheDocument()
  })
})

// ─── BarChart ─────────────────────────────────────────────────────────

describe('BarChart', () => {
  const sampleData = [
    { label: 'Openness', value: 72, color: 'bg-blue-500' },
    { label: 'Conscientiousness', value: 45, color: 'bg-green-500' },
    { label: 'Extraversion', value: 58, color: 'bg-purple-500' },
  ]

  it('renders all data items', () => {
    render(<BarChart data={sampleData} />)
    expect(screen.getByText('Openness')).toBeInTheDocument()
    expect(screen.getByText('Conscientiousness')).toBeInTheDocument()
    expect(screen.getByText('Extraversion')).toBeInTheDocument()
  })

  it('displays percentage values', () => {
    render(<BarChart data={sampleData} />)
    expect(screen.getByText('72.0%')).toBeInTheDocument()
    expect(screen.getByText('45.0%')).toBeInTheDocument()
    expect(screen.getByText('58.0%')).toBeInTheDocument()
  })

  it('renders with custom maxValue', () => {
    const data = [{ label: 'Test', value: 50, color: 'bg-red-500' }]
    render(<BarChart data={data} maxValue={200} />)
    expect(screen.getByText('50.0%')).toBeInTheDocument()
  })

  it('handles empty data', () => {
    const { container } = render(<BarChart data={[]} />)
    expect(container.firstChild).toBeInTheDocument()
  })

  it('clamps values at maxValue', () => {
    const data = [
      { label: 'Overflow', value: 150, color: 'bg-yellow-500' },
    ]
    render(<BarChart data={data} />)
    expect(screen.getByText('150.0%')).toBeInTheDocument()
  })
})
