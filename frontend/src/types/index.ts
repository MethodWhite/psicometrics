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

export interface BigFiveResult {
  scores: Record<string, number>
  facets: Record<string, number>
  profile_summary: string
  percentiles: Record<string, number>
}

export interface MBTIResult {
  type_code: string
  scores: Record<string, number>
  percentages: Record<string, number>
  profile_summary: string
}

export interface EnneagramResult {
  dominant_type: number
  wing: number
  scores: Record<string, number>
  profile_summary: string
}

export interface DISCResult {
  primary_style: string
  secondary_style: string
  scores: Record<string, number>
  profile_summary: string
}

export interface DarkTriadResult {
  scores: { machiavellianism: number; narcissism: number; psychopathy: number }
  dark_core: number
  risk_level: 'minimal' | 'low' | 'moderate' | 'high'
  profile_summary: string
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
}

export interface HumanDesignFormData {
  birth_date: string
  birth_time: string
  birth_location: string
  language: string
}
