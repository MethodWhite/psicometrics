export interface Tag {
  slug: string
  name: string
  category: TagCategory
  description: string
}

export type TagCategory =
  | 'big-five'
  | 'mbti'
  | 'enneagram'
  | 'disc'
  | 'dark-triad'
  | 'human-design'
  | 'love-languages'
  | 'attachment'
  | 'emotional-intelligence'
  | 'career'
  | 'relationships'
  | 'parenting'
  | 'education'
  | 'health'
  | 'spirituality'
  | 'self-improvement'
  | 'science'
  | 'psychology'
  | 'culture'
  | 'humor'

export const TAGS: Tag[] = [
  // ── Big Five ──
  { slug: 'openness', name: 'Apertura', category: 'big-five', description: 'Apertura a la experiencia: creatividad, curiosidad intelectual y apreciación estética.' },
  { slug: 'conscientiousness', name: 'Responsabilidad', category: 'big-five', description: 'Responsabilidad: organización, disciplina y orientación al logro.' },
  { slug: 'extraversion', name: 'Extraversión', category: 'big-five', description: 'Extraversión: sociabilidad, energía y emocionalidad positiva.' },
  { slug: 'agreeableness', name: 'Amabilidad', category: 'big-five', description: 'Amabilidad: cooperación, empatía y armonía social.' },
  { slug: 'neuroticism', name: 'Neuroticismo', category: 'big-five', description: 'Neuroticismo: sensibilidad emocional y vulnerabilidad al estrés.' },

  // ── MBTI ──
  { slug: 'intj', name: 'INTJ - Arquitecto', category: 'mbti', description: 'INTJ: Estrategas visionarios, independientes y con gran capacidad de planificación a largo plazo.' },
  { slug: 'intp', name: 'INTP - Lógico', category: 'mbti', description: 'INTP: Pensadores analíticos que buscan comprender las leyes fundamentales del universo.' },
  { slug: 'entj', name: 'ENTJ - Comandante', category: 'mbti', description: 'ENTJ: Líderes natos con visión clara y determinación para hacerla realidad.' },
  { slug: 'entp', name: 'ENTP - Innovador', category: 'mbti', description: 'ENTP: Mentes inquietas que desafían el status quo con ingenio y creatividad.' },
  { slug: 'infj', name: 'INFJ - Consejero', category: 'mbti', description: 'INFJ: Idealistas empáticos con profunda comprensión de la naturaleza humana.' },
  { slug: 'infp', name: 'INFP - Idealista', category: 'mbti', description: 'INFP: Soñadores apasionados que buscan autenticidad y significado.' },
  { slug: 'enfj', name: 'ENFJ - Maestro', category: 'mbti', description: 'ENFJ: Líderes carismáticos que inspiran y desarrollan el potencial en los demás.' },
  { slug: 'enfp', name: 'ENFP - Explorador', category: 'mbti', description: 'ENFP: Espíritus libres llenos de entusiasmo y creatividad.' },
  { slug: 'istj', name: 'ISTJ - Supervisor', category: 'mbti', description: 'ISTJ: Pilares de responsabilidad, tradición y confiabilidad.' },
  { slug: 'isfj', name: 'ISFJ - Protector', category: 'mbti', description: 'ISFJ: Guardianes silenciosos que cuidan de los demás con dedicación.' },
  { slug: 'estj', name: 'ESTJ - Ejecutivo', category: 'mbti', description: 'ESTJ: Líderes organizados que hacen que las cosas funcionen.' },
  { slug: 'esfj', name: 'ESFJ - Embajador', category: 'mbti', description: 'ESFJ: Personas cálidas y sociables dedicadas al servicio de los demás.' },
  { slug: 'istp', name: 'ISTP - Artesano', category: 'mbti', description: 'ISTP: Maestros de la acción y la resolución práctica de problemas.' },
  { slug: 'isfp', name: 'ISFP - Artista', category: 'mbti', description: 'ISFP: Almas sensibles que expresan su vida interior a través del arte.' },
  { slug: 'estp', name: 'ESTP - Emprendedor', category: 'mbti', description: 'ESTP: Personas de acción audaces que viven el momento al máximo.' },
  { slug: 'esfp', name: 'ESFP - Animador', category: 'mbti', description: 'ESFP: Espíritus vibrantes que iluminan cualquier habitación con su energía.' },

  // ── Enneagram ──
  { slug: 'type-1', name: 'Tipo 1 - Perfeccionista', category: 'enneagram', description: 'Eneatipo 1: Idealistas con un fuerte sentido del bien y del mal.' },
  { slug: 'type-2', name: 'Tipo 2 - Ayudador', category: 'enneagram', description: 'Eneatipo 2: Generosos con una capacidad extraordinaria para cuidar de los demás.' },
  { slug: 'type-3', name: 'Tipo 3 - Triunfador', category: 'enneagram', description: 'Eneatipo 3: Ambiciosos, carismáticos y orientados al logro.' },
  { slug: 'type-4', name: 'Tipo 4 - Individualista', category: 'enneagram', description: 'Eneatipo 4: Almas sensibles que buscan identidad y significado.' },
  { slug: 'type-5', name: 'Tipo 5 - Investigador', category: 'enneagram', description: 'Eneatipo 5: Mentes analíticas con sed insaciable de conocimiento.' },
  { slug: 'type-6', name: 'Tipo 6 - Leal', category: 'enneagram', description: 'Eneatipo 6: Personas leales, responsables y atentas a los riesgos.' },
  { slug: 'type-7', name: 'Tipo 7 - Entusiasta', category: 'enneagram', description: 'Eneatipo 7: Espíritus libres que buscan felicidad y nuevas experiencias.' },
  { slug: 'type-8', name: 'Tipo 8 - Desafiador', category: 'enneagram', description: 'Eneatipo 8: Líderes poderosos que protegen a los vulnerables.' },
  { slug: 'type-9', name: 'Tipo 9 - Pacificador', category: 'enneagram', description: 'Eneatipo 9: Personas que buscan paz y armonía en todas las áreas.' },

  // ── DISC ──
  { slug: 'dominance', name: 'Dominancia (D)', category: 'disc', description: 'Estilo DISC Dominante: directo, firme y orientado a resultados.' },
  { slug: 'influence', name: 'Influencia (I)', category: 'disc', description: 'Estilo DISC Influyente: entusiasta, persuasivo y optimista.' },
  { slug: 'steadiness', name: 'Estabilidad (S)', category: 'disc', description: 'Estilo DISC Estable: paciente, leal y buen oyente.' },
  { slug: 'conscientiousness-disc', name: 'Conciencia (C)', category: 'disc', description: 'Estilo DISC Concienzudo: preciso, analítico y meticuloso.' },

  // ── Dark Triad ──
  { slug: 'machiavellianism', name: 'Maquiavelismo', category: 'dark-triad', description: 'Maquiavelismo: tendencia a manipular y explotar a otros para beneficio propio.' },
  { slug: 'narcissism', name: 'Narcisismo', category: 'dark-triad', description: 'Narcisismo: sentido de grandiosidad, necesidad de admiración y falta de empatía.' },
  { slug: 'psychopathy', name: 'Psicopatía', category: 'dark-triad', description: 'Psicopatía: comportamiento antisocial, impulsividad y falta de remordimiento.' },

  // ── Human Design ──
  { slug: 'human-design', name: 'Diseño Humano', category: 'human-design', description: 'Sistema de autoconocimiento que combina astrología, I Ching, Kabbalah y los chakras.' },
  { slug: 'human-design-generator', name: 'Generador', category: 'human-design', description: 'Tipo de Diseño Humano con energía sostenida para responder y construir.' },
  { slug: 'human-design-projector', name: 'Proyector', category: 'human-design', description: 'Tipo de Diseño Humano que guía y gestiona la energía de los demás.' },
  { slug: 'human-design-manifestor', name: 'Manifestador', category: 'human-design', description: 'Tipo de Diseño Humano que inicia y pone cosas en movimiento.' },
  { slug: 'human-design-reflector', name: 'Reflector', category: 'human-design', description: 'Tipo de Diseño Humano que refleja el estado de su entorno.' },

  // ── Love Languages ──
  { slug: 'words-of-affirmation', name: 'Palabras de Afirmación', category: 'love-languages', description: 'Lenguaje del amor: expresar afecto a través de palabras de aprecio y ánimo.' },
  { slug: 'acts-of-service', name: 'Actos de Servicio', category: 'love-languages', description: 'Lenguaje del amor: demostrar amor a través de acciones y ayuda práctica.' },
  { slug: 'receiving-gifts', name: 'Recepción de Regalos', category: 'love-languages', description: 'Lenguaje del amor: sentir amor a través de regalos significativos.' },
  { slug: 'quality-time', name: 'Tiempo de Calidad', category: 'love-languages', description: 'Lenguaje del amor: valorar la atención plena y el tiempo compartido.' },
  { slug: 'physical-touch', name: 'Contacto Físico', category: 'love-languages', description: 'Lenguaje del amor: expresar amor a través del contacto físico y la cercanía.' },

  // ── Attachment ──
  { slug: 'secure-attachment', name: 'Apego Seguro', category: 'attachment', description: 'Estilo de apego caracterizado por confianza, intimidad y autonomía equilibrada.' },
  { slug: 'anxious-attachment', name: 'Apego Ansioso', category: 'attachment', description: 'Estilo de apego con miedo al abandono y necesidad constante de reaseguramiento.' },
  { slug: 'avoidant-attachment', name: 'Apego Evitativo', category: 'attachment', description: 'Estilo de apego que valora la independencia y evita la intimidad emocional.' },
  { slug: 'fearful-avoidant', name: 'Apego Temeroso', category: 'attachment', description: 'Estilo de apego con deseo de cercanía pero miedo a ser lastimado.' },

  // ── Emotional Intelligence ──
  { slug: 'emotional-intelligence', name: 'Inteligencia Emocional', category: 'emotional-intelligence', description: 'Capacidad para reconocer, comprender y gestionar las emociones propias y ajenas.' },
  { slug: 'self-awareness', name: 'Autoconciencia', category: 'emotional-intelligence', description: 'Capacidad de reconocer y entender las propias emociones y su impacto.' },
  { slug: 'empathy', name: 'Empatía', category: 'emotional-intelligence', description: 'Capacidad para comprender y compartir los sentimientos de los demás.' },
  { slug: 'emotional-regulation', name: 'Regulación Emocional', category: 'emotional-intelligence', description: 'Habilidad para manejar las emociones de manera saludable y adaptativa.' },
  { slug: 'social-skills', name: 'Habilidades Sociales', category: 'emotional-intelligence', description: 'Competencias para interactuar efectivamente con los demás.' },

  // ── Career ──
  { slug: 'career', name: 'Carrera Profesional', category: 'career', description: 'Orientación vocacional y desarrollo profesional basado en la personalidad.' },
  { slug: 'leadership', name: 'Liderazgo', category: 'self-improvement', description: 'Desarrollo de habilidades de liderazgo auténtico y efectivo.' },
  { slug: 'teamwork', name: 'Trabajo en Equipo', category: 'self-improvement', description: 'Colaboración efectiva y dinámicas de equipo según la personalidad.' },
  { slug: 'productivity', name: 'Productividad', category: 'self-improvement', description: 'Métodos y estrategias para ser más productivo según tu tipo.' },

  // ── Relationships ──
  { slug: 'relationships', name: 'Relaciones', category: 'relationships', description: 'Dinámicas relacionales y compatibilidad entre tipos de personalidad.' },
  { slug: 'communication', name: 'Comunicación', category: 'relationships', description: 'Estrategias de comunicación efectiva según la personalidad.' },
  { slug: 'conflict-resolution', name: 'Resolución de Conflictos', category: 'relationships', description: 'Manejo constructivo de conflictos interpersonales.' },
  { slug: 'dating', name: 'Citas y Romance', category: 'relationships', description: 'Compatibilidad romántica y dinámicas de pareja.' },

  // ── Parenting ──
  { slug: 'parenting', name: 'Crianza', category: 'parenting', description: 'Estrategias de crianza adaptadas a la personalidad de padres e hijos.' },
  { slug: 'child-development', name: 'Desarrollo Infantil', category: 'parenting', description: 'Etapas del desarrollo infantil y cómo la personalidad emerge desde temprana edad.' },
  { slug: 'adolescence', name: 'Adolescencia', category: 'parenting', description: 'Navegando la adolescencia con comprensión de la personalidad.' },

  // ── Education ──
  { slug: 'education', name: 'Educación', category: 'education', description: 'Métodos educativos adaptados a diferentes tipos de personalidad.' },
  { slug: 'learning-styles', name: 'Estilos de Aprendizaje', category: 'education', description: 'Cómo cada tipo de personalidad aprende y procesa información.' },
  { slug: 'study-techniques', name: 'Técnicas de Estudio', category: 'education', description: 'Métodos de estudio efectivos según tu tipo de personalidad.' },

  // ── Health ──
  { slug: 'mental-health', name: 'Salud Mental', category: 'health', description: 'Bienestar psicológico y salud mental en relación con la personalidad.' },
  { slug: 'anxiety', name: 'Ansiedad', category: 'health', description: 'Trastornos de ansiedad: tipos, síntomas y estrategias de manejo.' },
  { slug: 'depression', name: 'Depresión', category: 'health', description: 'La depresión y su relación con los rasgos de personalidad.' },
  { slug: 'stress-management', name: 'Manejo del Estrés', category: 'health', description: 'Técnicas para manejar el estrés según tu perfil de personalidad.' },
  { slug: 'sleep-health', name: 'Salud del Sueño', category: 'health', description: 'Relación entre la personalidad y los hábitos de sueño.' },

  // ── Spirituality ──
  { slug: 'spirituality', name: 'Espiritualidad', category: 'spirituality', description: 'Crecimiento espiritual y conexión con algo más grande que uno mismo.' },
  { slug: 'mindfulness', name: 'Mindfulness', category: 'spirituality', description: 'Práctica de atención plena y conciencia del momento presente.' },
  { slug: 'meditation', name: 'Meditación', category: 'spirituality', description: 'Técnicas de meditación adaptadas a diferentes tipos de personalidad.' },
  { slug: 'yoga', name: 'Yoga', category: 'spirituality', description: 'Yoga como práctica de unión entre cuerpo, mente y espíritu.' },

  // ── Self-Improvement ──
  { slug: 'self-improvement', name: 'Auto-mejora', category: 'self-improvement', description: 'Desarrollo personal y crecimiento continuo basado en autoconocimiento.' },
  { slug: 'motivation', name: 'Motivación', category: 'self-improvement', description: 'Cómo mantener la motivación según tu tipo de personalidad.' },
  { slug: 'resilience', name: 'Resiliencia', category: 'self-improvement', description: 'Capacidad de recuperarse y crecer ante la adversidad.' },
  { slug: 'self-esteem', name: 'Autoestima', category: 'self-improvement', description: 'Construcción de una autoestima saludable y auténtica.' },
  { slug: 'growth-mindset', name: 'Mentalidad de Crecimiento', category: 'self-improvement', description: 'Creencia en la capacidad de desarrollar habilidades y talentos.' },
  { slug: 'habits', name: 'Hábitos', category: 'self-improvement', description: 'Formación de hábitos saludables según tu perfil de personalidad.' },
  { slug: 'creativity', name: 'Creatividad', category: 'self-improvement', description: 'Desarrollo del potencial creativo en diferentes tipos de personalidad.' },

  // ── Science & Psychology ──
  { slug: 'psychology', name: 'Psicología', category: 'psychology', description: 'Estudio científico de la mente, el comportamiento y los procesos mentales.' },
  { slug: 'neuroscience', name: 'Neurociencia', category: 'science', description: 'Estudio del sistema nervioso y su relación con la personalidad.' },
  { slug: 'personality-psychology', name: 'Psicología de la Personalidad', category: 'psychology', description: 'Rama de la psicología que estudia las diferencias individuales.' },
  { slug: 'cognitive-psychology', name: 'Psicología Cognitiva', category: 'psychology', description: 'Estudio de los procesos mentales como atención, memoria y pensamiento.' },

  // ── Culture & Humor ──
  { slug: 'culture', name: 'Cultura', category: 'culture', description: 'Influencia de la cultura en la expresión y desarrollo de la personalidad.' },
  { slug: 'humor', name: 'Humor', category: 'humor', description: 'El sentido del humor y su relación con los tipos de personalidad.' },
  { slug: 'pop-psychology', name: 'Psicología Pop', category: 'culture', description: 'La personalidad en la cultura popular y medios de comunicación.' },
]

export function getTagsByCategory(category: TagCategory): Tag[] {
  return TAGS.filter(t => t.category === category)
}

export function getTagBySlug(slug: string): Tag | undefined {
  return TAGS.find(t => t.slug === slug)
}

export function getRelatedTags(slug: string): Tag[] {
  const tag = getTagBySlug(slug)
  if (!tag) return []
  return TAGS.filter(t => t.category === tag.category && t.slug !== slug).slice(0, 6)
}

export function getTagsBySlugs(slugs: string[]): Tag[] {
  return slugs.map(s => getTagBySlug(s)).filter((t): t is Tag => t !== undefined)
}

export const TAG_CATEGORIES: { id: TagCategory; name: string; icon: string }[] = [
  { id: 'big-five', name: 'Big Five', icon: '🧠' },
  { id: 'mbti', name: 'MBTI', icon: '🎭' },
  { id: 'enneagram', name: 'Eneagrama', icon: '🔵' },
  { id: 'disc', name: 'DISC', icon: '📊' },
  { id: 'dark-triad', name: 'Tríada Oscura', icon: '🌑' },
  { id: 'human-design', name: 'Diseño Humano', icon: '✨' },
  { id: 'love-languages', name: 'Lenguajes del Amor', icon: '💝' },
  { id: 'attachment', name: 'Apego', icon: '🔗' },
  { id: 'emotional-intelligence', name: 'Inteligencia Emocional', icon: '💎' },
  { id: 'career', name: 'Carrera', icon: '💼' },
  { id: 'relationships', name: 'Relaciones', icon: '💞' },
  { id: 'parenting', name: 'Crianza', icon: '👨‍👩‍👧‍👦' },
  { id: 'education', name: 'Educación', icon: '📚' },
  { id: 'health', name: 'Salud', icon: '🏥' },
  { id: 'spirituality', name: 'Espiritualidad', icon: '🕯️' },
  { id: 'self-improvement', name: 'Auto-mejora', icon: '🌱' },
  { id: 'science', name: 'Ciencia', icon: '🔬' },
  { id: 'psychology', name: 'Psicología', icon: '📖' },
  { id: 'culture', name: 'Cultura', icon: '🌍' },
  { id: 'humor', name: 'Humor', icon: '😄' },
]
