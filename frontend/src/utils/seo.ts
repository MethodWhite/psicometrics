export interface SEOMeta {
  title: string
  description: string
  image?: string
  type?: string
  keywords?: string
}

export const SEO_CONFIG: Record<string, SEOMeta> = {
  home: {
    title: 'PsicoMetrics — Tests de Personalidad Oficiales y Gratuitos',
    description:
      'Descubre tu personalidad con tests psicológicos oficiales validados científicamente. Big Five, MBTI, Enneagrama, DISC, Tríada Oscura y Diseño Humano. Resultados instantáneos y gratuitos.',
    keywords:
      'test de personalidad, Big Five, MBTI, enneagrama, DISC, psicología, personalidad, test gratuito, psicoMetrics',
    image: '/og-home.jpg',
  },
  big_five: {
    title: 'Test Big Five — Los 5 Grandes Factores de Personalidad',
    description:
      'Evalúa tu personalidad con el modelo Big Five (OCEAN): Apertura, Responsabilidad, Extraversión, Amabilidad y Neuroticismo. Test científico de 50 preguntas.',
    keywords: 'Big Five, OCEAN, test personalidad, apertura, responsabilidad, extraversión, amabilidad, neuroticismo',
    image: '/og-big-five.jpg',
  },
  mbti: {
    title: 'Test MBTI — Myers-Briggs Type Indicator Oficial',
    description:
      'Descubre tu tipo MBTI entre 16 personalidades. Test basado en las dicotomías de Jung: Introversión/Extraversión, Intuición/Sensorial, Pensamiento/Sentimiento, Juzgar/ Percibir.',
    keywords: 'MBTI, Myers-Briggs, 16 personalidades, INTJ, ENFP, test personalidad, tipo psicológico',
    image: '/og-mbti.jpg',
  },
  enneagram: {
    title: 'Test de Eneagrama — Descubre tu Tipo de Personalidad',
    description:
      'Conoce tu eneatipo dominante y tu ala con nuestro test de Eneagrama. 9 tipos de personalidad con análisis detallado de patrones emocionales y motivaciones.',
    keywords: 'eneagrama, eneatipo, test eneagrama, tipo 1, tipo 2, tipo 3, personalidad, ala',
    image: '/og-enneagram.jpg',
  },
  disc: {
    title: 'Test DISC — Evaluación de Estilos de Comportamiento',
    description:
      'Identifica tu estilo DISC: Dominancia, Influencia, Estabilidad o Conciencia. Test conductual para desarrollo profesional y personal.',
    keywords: 'DISC, test DISC, dominancia, influencia, estabilidad, conciencia, comportamiento, perfil DISC',
    image: '/og-disc.jpg',
  },
  dark_triad: {
    title: 'Test Tríada Oscura — Maquiavelismo, Narcisismo y Psicopatía',
    description:
      'Evalúa los rasgos de personalidad oscura: maquiavelismo, narcisismo y psicopatía. Test psicológico confidencial con resultados anónimos.',
    keywords: 'tríada oscura, maquiavelismo, narcisismo, psicopatía, personalidad oscura, test psicología',
    image: '/og-dark-triad.jpg',
  },
  human_design: {
    title: 'Diseño Humano — Carta Gráfica y Test de Tipo',
    description:
      'Descubre tu tipo de Diseño Humano: Manifestador, Generador, Generador Manifestante, Proyector o Reflector. Basado en tu fecha, hora y lugar de nacimiento.',
    keywords: 'diseño humano, human design, carta gráfica, body graph, generator, manifestador, proyector',
    image: '/og-human-design.jpg',
  },
  blog: {
    title: 'Blog de Psicología y Personalidad — PsicoMetrics',
    description:
      'Artículos sobre psicología de la personalidad: Big Five, MBTI, Eneagrama, relaciones, carrera profesional y crecimiento personal. Contenido basado en evidencia científica.',
    keywords: 'blog personalidad, psicología, artículos personalidad, Big Five, MBTI, eneagrama, relaciones, carrera',
    image: '/og-blog.jpg',
  },
}

const BASE_URL = 'https://psicometrics.app'

export function generateMetaTags(
  title: string,
  description: string,
  image?: string,
  type = 'website',
): string {
  const imageUrl = image ? `${BASE_URL}${image}` : `${BASE_URL}/og-default.jpg`
  return `
    <title>${escapeHtml(title)}</title>
    <meta name="description" content="${escapeHtml(description)}" />
    <meta property="og:title" content="${escapeHtml(title)}" />
    <meta property="og:description" content="${escapeHtml(description)}" />
    <meta property="og:type" content="${escapeHtml(type)}" />
    <meta property="og:url" content="${BASE_URL}" />
    <meta property="og:image" content="${escapeHtml(imageUrl)}" />
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="${escapeHtml(title)}" />
    <meta name="twitter:description" content="${escapeHtml(description)}" />
    <meta name="twitter:image" content="${escapeHtml(imageUrl)}" />
  `.trim()
}

export function generateStructuredData(type: 'organization' | 'webapp' | 'personality'): string {
  switch (type) {
    case 'organization':
      return JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'Organization',
        name: 'PsicoMetrics',
        url: BASE_URL,
        description: 'Tests de personalidad oficiales con estándares científicos validados.',
        foundingDate: '2024',
        sameAs: [],
      })
    case 'webapp':
      return JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'WebApplication',
        name: 'PsicoMetrics',
        url: BASE_URL,
        description: 'Tests de personalidad oficiales con estándares científicos validados.',
        applicationCategory: 'Personality Assessment',
        operatingSystem: 'All',
        offers: {
          '@type': 'Offer',
          price: '0',
          priceCurrency: 'USD',
        },
      })
    case 'personality':
      return JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'Article',
        publisher: {
          '@type': 'Organization',
          name: 'PsicoMetrics',
        },
      })
  }
}

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;')
}
