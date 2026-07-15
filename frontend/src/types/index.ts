export interface TestInfo {
  test_type: string
  name: string
  description: string
  item_count: number
  test_mode: 'questions' | 'birth_data'
}

export interface Question {
  id: number
  text: string
  facet?: string
  reverse?: boolean
  dichotomy?: string
  pole?: string
  type?: number
  dimension?: string
  trait?: string
}

export interface TestData {
  test_type: string
  name: string
  description: string
  questions: Question[]
  test_mode?: 'questions' | 'birth_data'
  scale?: {
    min: number
    max: number
    labels: string[]
  }
  factors?: Record<string, string>
  dichotomies?: Record<string, { es: string; en: string }>
  types?: Record<string, any>
  dimensions?: Record<string, { es: string; en: string; description: { es: string; en: string } }>
  traits?: Record<string, { es: string; en: string; description: { es: string; en: string } }>
}

export interface ValidityInfo {
  valid: boolean
  warnings: string[]
}

export interface BigFiveResult {
  scores: Record<string, number>
  facets: Record<string, number>
  profile_summary: string
  percentiles: Record<string, number>
  validity?: ValidityInfo
}

export interface MBTIResult {
  type_code: string
  scores: Record<string, number>
  percentages: Record<string, number>
  profile_summary: string
  validity?: ValidityInfo
}

export interface EnneagramResult {
  dominant_type: number
  wing: number
  scores: Record<string, number>
  profile_summary: string
  validity?: ValidityInfo
}

export interface DISCResult {
  primary_style: string
  secondary_style: string
  scores: Record<string, number>
  profile_summary: string
  validity?: ValidityInfo
}

export interface DarkTriadResult {
  scores: { machiavellianism: number; narcissism: number; psychopathy: number }
  dark_core: number
  risk_level: 'minimal' | 'low' | 'moderate' | 'high'
  profile_summary: string
  validity?: ValidityInfo
}

export interface HumanDesignResult {
  type: string
  type_info: any
  strategy: string
  authority: string
  authority_info: any
  profile: string
  profile_info: any
  centers: Record<string, boolean>
  personality_gates: number[]
  design_gates: number[]
  summary: string
  validity?: ValidityInfo
}

export interface HumanDesignFormData {
  birth_date: string
  birth_time: string
  birth_location: string
  language: string
}

export interface TestMetadata {
  instructions: string
  consent: string
  scientific_basis: string
}

export interface FactorInterpretation {
  level: 'low' | 'moderate' | 'high'
  daily_life: string
  work: string
  relationships: string
}

export interface Recommendations {
  growth_areas: string[]
  career_recommendations: string[]
}

export interface AccountInfo {
  account_id: string
  email: string
}

export interface SavedResult {
  result_id: string
  test_type: string
  created_at: string
}

export interface CompareResult {
  compatibility: number
  details: Record<string, { score_a: number; score_b: number; difference: number; compatible: boolean }>
  summary: string
}

export interface EvolutionData {
  points: EvolutionPoint[]
  dimensions: string[]
}

export interface EvolutionPoint {
  date: string
  scores: Record<string, number>
}
