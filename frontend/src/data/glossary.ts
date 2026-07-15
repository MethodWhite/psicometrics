export interface GlossaryEntry {
  term: string
  definition: string
  encyclopediaId?: string
}

export const glossaryEntries: GlossaryEntry[] = [
  {
    term: 'Big Five',
    definition: 'A scientific model identifying five broad personality dimensions: Openness, Conscientiousness, Extraversion, Agreeableness, and Neuroticism (OCEAN).',
    encyclopediaId: 'big-five',
  },
  {
    term: 'Openness',
    definition: 'Tendency toward imagination, curiosity, aesthetic sensitivity, and intellectual exploration. High scorers are creative and open to new experiences.',
    encyclopediaId: 'openness',
  },
  {
    term: 'Conscientiousness',
    definition: 'Tendency to be organized, responsible, disciplined, and goal-oriented. Strong predictor of academic and professional success.',
    encyclopediaId: 'conscientiousness',
  },
  {
    term: 'Extraversion',
    definition: 'Tendency toward sociability, assertiveness, and positive emotionality. Extraverts gain energy from social interaction.',
    encyclopediaId: 'extraversion',
  },
  {
    term: 'Agreeableness',
    definition: 'Tendency toward compassion, cooperation, and concern for social harmony. High scorers are empathetic and trusting.',
    encyclopediaId: 'agreeableness',
  },
  {
    term: 'Neuroticism',
    definition: 'Tendency to experience negative emotions like anxiety, anger, and sadness. Low scorers are emotionally stable and resilient.',
    encyclopediaId: 'neuroticism',
  },
  {
    term: 'MBTI',
    definition: 'Myers-Briggs Type Indicator: a personality framework sorting people into 16 types based on four dichotomies (E/I, S/N, T/F, J/P).',
    encyclopediaId: 'mbti',
  },
  {
    term: 'Cognitive Functions',
    definition: 'The eight mental processes (Si, Se, Ni, Ne, Ti, Te, Fi, Fe) that form the basis of Jungian personality theory and MBTI type dynamics.',
    encyclopediaId: 'cognitive-functions',
  },
  {
    term: 'Introversion (MBTI)',
    definition: 'A preference for focusing on the inner world of ideas and reflections. Introverts gain energy from solitude.',
    encyclopediaId: 'mbti-introversion',
  },
  {
    term: 'Extraversion (MBTI)',
    definition: 'A preference for focusing on the outer world of people and activities. Extraverts gain energy from social interaction.',
    encyclopediaId: 'mbti-extraversion',
  },
  {
    term: 'Sensing',
    definition: 'A preference for focusing on concrete, factual, observable information through the five senses. Practical and detail-oriented.',
    encyclopediaId: 'sensing',
  },
  {
    term: 'Intuition',
    definition: 'A preference for focusing on patterns, possibilities, and abstract meanings. Imaginative and future-oriented.',
    encyclopediaId: 'intuition',
  },
  {
    term: 'Thinking',
    definition: 'A preference for making decisions based on logical analysis and objective principles. Values fairness and competence.',
    encyclopediaId: 'thinking',
  },
  {
    term: 'Feeling',
    definition: 'A preference for making decisions based on personal values, empathy, and concern for harmony.',
    encyclopediaId: 'feeling',
  },
  {
    term: 'Judging',
    definition: 'A preference for structure, planning, and decisiveness. Likes to have things settled and decided.',
    encyclopediaId: 'judging',
  },
  {
    term: 'Perceiving',
    definition: 'A preference for flexibility, spontaneity, and keeping options open. Adapts to circumstances as they arise.',
    encyclopediaId: 'perceiving',
  },
  {
    term: 'Enneagram',
    definition: 'A personality system describing nine interconnected types, each driven by a core motivation, fear, and desire. Includes wings and instinctual variants.',
    encyclopediaId: 'enneagram',
  },
  {
    term: 'Enneagram Wings',
    definition: 'The two types adjacent to a core Enneagram type that add a secondary flavor to personality patterns (e.g., 9w1 vs 9w8).',
    encyclopediaId: 'enneagram-wings',
  },
  {
    term: 'Instinctual Variants',
    definition: 'Three biological instincts (Self-Preservation, Social, Sexual) that interact with Enneagram type to create 27 subtype patterns.',
    encyclopediaId: 'instinctual-variants',
  },
  {
    term: 'Integration Arrow',
    definition: 'The direction on the Enneagram symbol showing qualities a type can access when growing and healthy.',
    encyclopediaId: 'integration-arrow',
  },
  {
    term: 'Personality',
    definition: 'Enduring patterns of thoughts, feelings, and behaviors that distinguish individuals, shaped by both genetics and environment.',
    encyclopediaId: 'personality',
  },
  {
    term: 'Temperament',
    definition: 'Biologically based, innate aspects of personality evident from infancy. Largely inherited and stable throughout life.',
    encyclopediaId: 'temperament',
  },
  {
    term: 'Character',
    definition: 'Aspects of personality shaped by experience, values, and deliberate moral development. Develops over time through learning.',
    encyclopediaId: 'character',
  },
  {
    term: 'Self-Concept',
    definition: 'The organized set of perceptions and beliefs one holds about oneself, including self-image, self-esteem, and ideal self.',
    encyclopediaId: 'self-concept',
  },
  {
    term: 'Emotional Intelligence',
    definition: 'The ability to perceive, understand, manage, and regulate emotions in oneself and others. Distinct from cognitive intelligence (IQ).',
    encyclopediaId: 'emotional-intelligence',
  },
  {
    term: 'Empathy',
    definition: 'The ability to understand and share the feelings of another. Includes cognitive empathy (perspective-taking) and emotional empathy.',
    encyclopediaId: 'empathy',
  },
  {
    term: 'Resilience',
    definition: 'The capacity to recover quickly from difficulties and adapt to change while maintaining mental health.',
    encyclopediaId: 'resilience',
  },
  {
    term: 'Growth Mindset',
    definition: 'The belief that abilities and personality can be developed through effort and learning, as opposed to being fixed.',
    encyclopediaId: 'growth-mindset',
  },
  {
    term: 'Identity',
    definition: 'The distinctive characteristics, beliefs, and affiliations defining a person. Central developmental task of adolescence per Erikson.',
    encyclopediaId: 'identity',
  },
  {
    term: 'Self-Esteem',
    definition: 'A subjective evaluation of one own worth and value. Influenced by early experiences, social comparisons, and achievements.',
    encyclopediaId: 'self-esteem',
  },
]

export function getGlossaryTerm(term: string): GlossaryEntry | undefined {
  return glossaryEntries.find(e => e.term.toLowerCase() === term.toLowerCase())
}

export function searchGlossary(query: string): GlossaryEntry[] {
  const q = query.toLowerCase()
  return glossaryEntries.filter(e =>
    e.term.toLowerCase().includes(q) || e.definition.toLowerCase().includes(q)
  )
}
