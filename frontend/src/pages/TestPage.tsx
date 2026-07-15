import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import { LikertScale } from '../components/LikertScale'
import { DichotomyChoice } from '../components/DichotomyChoice'
import { HumanDesignForm } from '../components/HumanDesignForm'
import { LoadingSkeleton } from '../components/LoadingSkeleton'
import type { TestData } from '../types'

export function TestPage() {
  const { testType } = useParams<{ testType: string }>()
  const navigate = useNavigate()
  const { t, i18n } = useTranslation()

  const [testData, setTestData] = useState<TestData | null>(null)
  const [currentQuestion, setCurrentQuestion] = useState(0)
  const [answers, setAnswers] = useState<Record<number, number | string>>({})
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)

  useEffect(() => {
    const fetchTest = async () => {
      try {
        const response = await fetch(`/api/v1/tests/${testType}?lang=${i18n.language}`)
        const data = await response.json()
        setTestData(data)
      } catch (error) {
        console.error('Failed to load test:', error)
      } finally {
        setLoading(false)
      }
    }
    fetchTest()
  }, [testType, i18n.language])

  if (loading || !testData) {
    return (
      <div className="min-h-screen px-4 py-24 max-w-2xl mx-auto">
        <div className="mb-8">
          <div className={`skeleton-pulse rounded h-4 w-16 bg-border mb-4`} />
          <div className={`skeleton-pulse rounded h-8 w-64 bg-border mb-2`} />
        </div>
        <div className="card space-y-4">
          <LoadingSkeleton variant="text" lines={4} />
          <div className="flex justify-between pt-4">
            <div className={`skeleton-pulse rounded h-10 w-28 bg-border`} />
            <div className={`skeleton-pulse rounded h-10 w-28 bg-border`} />
          </div>
        </div>
      </div>
    )
  }

  // Human Design uses a form instead of questions
  if (testType === 'human_design') {
    const handleHDSubmit = async (data: { birth_date: string; birth_time: string; birth_location: string }) => {
      setSubmitting(true)
      try {
        const response = await fetch(`/api/v1/tests/human_design/submit`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            ...data,
            language: i18n.language,
          }),
        })
        const result = await response.json()
        navigate(`/results/human_design`, { state: { result, testType: 'human_design' } })
      } catch (error) {
        console.error('Failed to submit Human Design:', error)
      } finally {
        setSubmitting(false)
      }
    }

    return (
      <div className="min-h-screen px-4 py-8">
        <div className="max-w-2xl mx-auto">
          <div className="mb-8">
            <button
              onClick={() => navigate('/')}
              className="text-content-secondary hover:text-content transition-colors mb-4"
            >
              ← {t('app.back')}
            </button>
            <h2 className="text-2xl font-bold text-content">{testData.name}</h2>
            <p className="text-content-secondary mt-2">{testData.description}</p>
          </div>

          <div className="card">
            <HumanDesignForm onSubmit={handleHDSubmit} loading={submitting} />
          </div>
        </div>
      </div>
    )
  }

  const questions = testData.questions
  const question = questions[currentQuestion]
  const progress = ((currentQuestion + 1) / questions.length) * 100
  const isLast = currentQuestion === questions.length - 1

  const handleAnswer = (value: number | string) => {
    setAnswers((prev) => ({ ...prev, [question.id]: value }))
  }

  const handleNext = () => {
    if (currentQuestion < questions.length - 1) {
      setCurrentQuestion((prev) => prev + 1)
    }
  }

  const handlePrevious = () => {
    if (currentQuestion > 0) {
      setCurrentQuestion((prev) => prev - 1)
    }
  }

  const handleSubmit = async () => {
    setSubmitting(true)
    try {
      const formattedAnswers = Object.entries(answers).map(([id, value]) => ({
        question_id: parseInt(id),
        value,
      }))

      const response = await fetch(`/api/v1/tests/${testType}/submit`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          answers: formattedAnswers,
          language: i18n.language,
        }),
      })

      const result = await response.json()
      navigate(`/results/${testType}`, { state: { result, testType } })
    } catch (error) {
      console.error('Failed to submit test:', error)
    } finally {
      setSubmitting(false)
    }
  }

  const canProceed = answers[question.id] !== undefined

  return (
    <div className="min-h-screen px-4 py-8">
      <div className="max-w-2xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <button
            onClick={() => navigate('/')}
            className="text-content-secondary hover:text-content transition-colors mb-4"
          >
            ← {t('app.back')}
          </button>
          <h2 className="text-2xl font-bold text-content">{testData.name}</h2>
        </div>

        {/* Progress */}
        <div className="mb-8">
          <div className="flex justify-between text-sm text-content-secondary mb-2">
            <span>{t('app.question_of', { current: currentQuestion + 1, total: questions.length })}</span>
            <span>{progress.toFixed(0)}%</span>
          </div>
          <div className="progress-bar">
            <div className="progress-fill" style={{ width: `${progress}%` }} />
          </div>
        </div>

        {/* Question */}
        <div className="card mb-8">
          <p className="text-lg text-content mb-6 leading-relaxed">
            {question.text}
          </p>

          {/* Likert scale for Big Five, Enneagram, Dark Triad */}
          {(testType === 'big_five' || testType === 'enneagram' || testType === 'dark_triad') && testData.scale && (
            <LikertScale
              value={answers[question.id] as number || null}
              onChange={(v) => handleAnswer(v)}
              labels={testData.scale.labels}
            />
          )}

          {/* MBTI dichotomy choice */}
          {testType === 'mbti' && question.dichotomy && (
            <DichotomyChoice
              value={answers[question.id] as string || null}
              onChange={(v) => handleAnswer(v)}
              optionA={question.text}
              optionB={questions.find(q => q.dichotomy === question.dichotomy && q.id !== question.id && q.pole !== question.pole)?.text || ''}
            />
          )}

          {/* DISC multiple choice */}
          {testType === 'disc' && (
            <div className="grid grid-cols-2 gap-3">
              {['a', 'b', 'c', 'd'].map((opt) => (
                <label
                  key={opt}
                  className={`text-center p-4 rounded-xl border cursor-pointer transition-all duration-200
                    ${
                      answers[question.id] === opt
                        ? 'bg-indigo-500/20 border-indigo-500 text-content'
                        : 'bg-surface-secondary border-border text-content-secondary hover:bg-surface-elevated'
                    }`}
                >
                  <input
                    type="radio"
                    name={`q-${question.id}`}
                    value={opt}
                    checked={answers[question.id] === opt}
                    onChange={() => handleAnswer(opt)}
                    className="sr-only"
                  />
                  <span className="block text-2xl mb-1">
                    {opt === 'a' ? '🎯' : opt === 'b' ? '💬' : opt === 'c' ? '🤝' : '📊'}
                  </span>
                  <span className="text-sm font-medium">
                    {opt === 'a' ? 'D' : opt === 'b' ? 'I' : opt === 'c' ? 'S' : 'C'}
                  </span>
                </label>
              ))}
            </div>
          )}
        </div>

        {/* Navigation */}
        <div className="flex justify-between">
          <button
            onClick={handlePrevious}
            disabled={currentQuestion === 0}
            className="btn-secondary disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {t('app.previous')}
          </button>

          {isLast ? (
            <button
              onClick={handleSubmit}
              disabled={!canProceed || submitting}
              className="btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {submitting ? t('app.loading') : t('app.submit')}
            </button>
          ) : (
            <button
              onClick={handleNext}
              disabled={!canProceed}
              className="btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {t('app.next')}
            </button>
          )}
        </div>
      </div>
    </div>
  )
}
