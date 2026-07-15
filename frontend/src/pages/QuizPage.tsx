import { useState, useCallback } from 'react'
import { useParams, Link } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'
import { getQuizBySlug } from '../data/quizzes'
import type { QuizResult } from '../data/quizzes'

type QuizState = 'start' | 'playing' | 'result'

export default function QuizPage() {
  const { slug } = useParams<{ slug: string }>()
  const quiz = slug ? getQuizBySlug(slug) : undefined

  const [quizState, setQuizState] = useState<QuizState>('start')
  const [currentQuestion, setCurrentQuestion] = useState(0)
  const [points, setPoints] = useState<Record<string, number>>({})
  const [result, setResult] = useState<QuizResult | null>(null)
  const [transitioning, setTransitioning] = useState(false)

  const handleStart = () => {
    setQuizState('playing')
    setCurrentQuestion(0)
    setPoints({})
    setResult(null)
  }

  const handleAnswer = useCallback((optionPoints: Record<string, number>) => {
    const merged = { ...points }
    for (const [key, val] of Object.entries(optionPoints)) {
      merged[key] = (merged[key] || 0) + val
    }
    setPoints(merged)

    if (currentQuestion < quiz!.questions.length - 1) {
      setTransitioning(true)
      setTimeout(() => {
        setCurrentQuestion(prev => prev + 1)
        setTransitioning(false)
      }, 300)
    } else {
      // Calculate result
      const maxKey = Object.entries(merged).sort((a, b) => b[1] - a[1])[0][0]
      const foundResult = quiz!.results.find(r => r.id === maxKey) || quiz!.results[0]
      setResult(foundResult)
      setQuizState('result')
    }
  }, [currentQuestion, points, quiz])

  const handleShare = async () => {
    if (!quiz || !result) return
    const text = `¡Acabo de hacer el test "${quiz.title}" en PsicoMetrics! Mi resultado: ${result.title}`
    try {
      await navigator.share({ title: quiz.title, text, url: window.location.href })
    } catch {
      navigator.clipboard.writeText(`${text} ${window.location.origin}/quiz/${quiz.slug}`)
    }
  }

  if (!quiz) {
    return (
      <div className="min-h-screen px-4 py-24 text-center">
        <span className="text-6xl block mb-4">🔍</span>
        <h1 className="text-2xl font-bold text-content mb-2">Test no encontrado</h1>
        <p className="text-content-muted mb-6">El test que buscas no existe.</p>
        <Link to="/quiz" className="btn-primary inline-block">Ver todos los tests</Link>
      </div>
    )
  }

  const progress = ((currentQuestion) / quiz.questions.length) * 100

  return (
    <>
      <Helmet>
        <title>{quiz.title} — PsicoMetrics</title>
        <meta name="description" content={quiz.description} />
      </Helmet>

      <div className="min-h-screen px-4 py-8">
        <div className="max-w-2xl mx-auto">
          {/* Breadcrumb */}
          <div className="mb-6 flex items-center gap-2 text-sm text-content-muted">
            <Link to="/" className="hover:text-content transition-colors">Inicio</Link>
            <span>/</span>
            <Link to="/quiz" className="hover:text-content transition-colors">Tests</Link>
            <span>/</span>
            <span className="text-content">{quiz.title}</span>
          </div>

          {quizState === 'start' && (
            <div className="card text-center py-12">
              <span className="text-6xl mb-6 block">{quiz.icon}</span>
              <h1 className="text-3xl font-bold text-content mb-4">{quiz.title}</h1>
              <p className="text-content-secondary text-lg mb-6 max-w-md mx-auto leading-relaxed">
                {quiz.description}
              </p>
              <div className="flex items-center justify-center gap-4 mb-8 text-sm text-content-muted">
                <span>{quiz.questions.length} preguntas</span>
                <span>·</span>
                <span>2-5 minutos</span>
                <span>·</span>
                <span>Sin registro</span>
              </div>
              <button
                onClick={handleStart}
                className="btn-primary text-lg px-8 py-3"
              >
                Comenzar test →
              </button>
            </div>
          )}

          {quizState === 'playing' && (
            <div>
              {/* Progress bar */}
              <div className="mb-6">
                <div className="flex justify-between text-sm text-content-muted mb-2">
                  <span>Pregunta {currentQuestion + 1} de {quiz.questions.length}</span>
                  <span>{Math.round(progress)}%</span>
                </div>
                <div className="h-2 bg-surface-secondary rounded-full overflow-hidden">
                  <div
                    className="h-full bg-gradient-to-r from-indigo-500 to-purple-500 rounded-full transition-all duration-500 ease-out"
                    style={{ width: `${progress}%` }}
                  />
                </div>
              </div>

              {/* Question card */}
              <div className={`card transition-all duration-300 ${transitioning ? 'opacity-0 translate-x-8' : 'opacity-100 translate-x-0'}`}>
                <h2 className="text-xl font-bold text-content mb-6">
                  {quiz.questions[currentQuestion].question}
                </h2>
                <div className="space-y-3">
                  {quiz.questions[currentQuestion].options.map((option, i) => (
                    <button
                      key={i}
                      onClick={() => handleAnswer(option.points)}
                      className="w-full text-left p-4 rounded-xl bg-surface-secondary hover:bg-indigo-500/10 border border-border hover:border-indigo-500/30 transition-all duration-200 text-content-secondary hover:text-content"
                    >
                      {option.text}
                    </button>
                  ))}
                </div>
              </div>
            </div>
          )}

          {quizState === 'result' && result && (
            <div className="card text-center py-8 animate-in">
              <span className="text-5xl mb-4 block">{quiz.icon}</span>
              <h2 className="text-2xl font-bold text-content mb-2">Tu resultado</h2>
              <div className="text-xl font-bold gradient-text mb-6">{result.title}</div>
              <p className="text-content-secondary leading-relaxed mb-8 text-left">
                {result.description}
              </p>

              {/* Share buttons */}
              <div className="flex flex-wrap gap-3 justify-center mb-8">
                <button
                  onClick={handleShare}
                  className="btn-secondary text-sm flex items-center gap-2"
                >
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"/></svg>
                  Compartir resultado
                </button>
                <button
                  onClick={() => {
                    const text = `¡${result.title}! Descubre más en PsicoMetrics`
                    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(text + ' ' + window.location.origin + '/quiz/' + quiz.slug)}`, '_blank', 'noopener,noreferrer')
                  }}
                  className="btn-secondary text-sm flex items-center gap-2"
                >
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
                  Compartir en X
                </button>
              </div>

              {/* Retake */}
              <button
                onClick={handleStart}
                className="btn-secondary text-sm mr-3"
              >
                Repetir test
              </button>
              <Link to="/quiz" className="btn-secondary text-sm">
                Más tests →
              </Link>

              {/* CTA to full test */}
              <div className="mt-8 p-6 card bg-gradient-to-br from-indigo-600/10 to-purple-600/10 border-indigo-500/20">
                <span className="text-3xl mb-3 block">🧠</span>
                <h3 className="text-lg font-bold text-content mb-2">¿Quieres un análisis más profundo?</h3>
                <p className="text-content-secondary text-sm mb-4">
                  Realiza los tests científicos completos de PsicoMetrics con decenas de preguntas y análisis detallados.
                </p>
                <Link to="/" className="btn-primary inline-block text-sm">
                  Ir a los tests completos →
                </Link>
              </div>
            </div>
          )}
        </div>
      </div>
    </>
  )
}
