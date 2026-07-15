import { useState } from 'react'
import { useTranslation } from 'react-i18next'

interface PDFDownloadButtonProps {
  testType: string
}

export function PDFDownloadButton({ testType }: PDFDownloadButtonProps) {
  const { t } = useTranslation()
  const [loading, setLoading] = useState(false)

  const handleDownload = async () => {
    setLoading(true)
    try {
      const response = await fetch(`/api/v1/tests/${testType}/report`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
      })
      if (!response.ok) throw new Error('Failed to generate PDF')
      const blob = await response.blob()
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `${testType}-report.pdf`
      document.body.appendChild(a)
      a.click()
      document.body.removeChild(a)
      URL.revokeObjectURL(url)
    } catch (error) {
      console.error('PDF download failed:', error)
    } finally {
      setLoading(false)
    }
  }

  return (
    <button
      onClick={handleDownload}
      disabled={loading}
      className="btn-secondary flex-1"
    >
      {loading ? t('app.loading') : t('results.download_pdf')}
    </button>
  )
}
