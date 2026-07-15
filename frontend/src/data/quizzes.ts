export interface QuizQuestion {
  question: string
  options: { text: string; points: Record<string, number> }[]
}

export interface QuizResult {
  id: string
  title: string
  description: string
  minScore: number
  maxScore: number
}

export interface Quiz {
  slug: string
  title: string
  description: string
  category: string
  icon: string
  questions: QuizQuestion[]
  results: QuizResult[]
  image?: string
}

export const quizzes: Quiz[] = [
  // 1. LOVE LANGUAGE QUIZ (5 questions)
  {
    slug: 'lenguaje-amor',
    title: '¿Cuál es tu Lenguaje del Amor?',
    description: 'Descubre cómo prefieres dar y recibir amor según la teoría de los 5 lenguajes del amor de Gary Chapman.',
    category: 'love-languages',
    icon: '💝',
    questions: [
      {
        question: '¿Qué te hace sentir más amado/a?',
        options: [
          { text: 'Que me digan "te quiero" o me halaguen', points: { 'words': 1 } },
          { text: 'Que me ayuden con algo sin que lo pida', points: { 'acts': 1 } },
          { text: 'Recibir un regalo sorpresa', points: { 'gifts': 1 } },
          { text: 'Pasar tiempo a solas con mi pareja', points: { 'time': 1 } },
          { text: 'Que me abracen o tomen de la mano', points: { 'touch': 1 } },
        ],
      },
      {
        question: '¿Qué es lo que más valoras de tu pareja?',
        options: [
          { text: 'Sus palabras de ánimo y aprecio', points: { 'words': 1 } },
          { text: 'Cuando hace cosas por ti para ayudarte', points: { 'acts': 1 } },
          { text: 'Los detalles y sorpresas que te da', points: { 'gifts': 1 } },
          { text: 'El tiempo de calidad que pasan juntos', points: { 'time': 1 } },
          { text: 'El contacto físico y la cercanía', points: { 'touch': 1 } },
        ],
      },
      {
        question: '¿Cómo expresas tu amor más naturalmente?',
        options: [
          { text: 'Diciendo cumplidos y palabras bonitas', points: { 'words': 1 } },
          { text: 'Haciendo favores y ayudando', points: { 'acts': 1 } },
          { text: 'Dando regalos significativos', points: { 'gifts': 1 } },
          { text: 'Dedicando tiempo sin distracciones', points: { 'time': 1 } },
          { text: 'Con abrazos, besos y caricias', points: { 'touch': 1 } },
        ],
      },
      {
        question: '¿Qué te duele más en una relación?',
        options: [
          { text: 'La falta de palabras de afirmación', points: { 'words': 1 } },
          { text: 'Que no te ayuden cuando lo necesitas', points: { 'acts': 1 } },
          { text: 'Que se olviden de fechas especiales', points: { 'gifts': 1 } },
          { text: 'La falta de atención y tiempo juntos', points: { 'time': 1 } },
          { text: 'La distancia física y falta de contacto', points: { 'touch': 1 } },
        ],
      },
      {
        question: '¿Qué recuerdo de amor te hace más feliz?',
        options: [
          { text: 'Una nota o mensaje inesperado', points: { 'words': 1 } },
          { text: 'Cuando alguien te ayudó en un momento difícil', points: { 'acts': 1 } },
          { text: 'Un regalo que significaba mucho', points: { 'gifts': 1 } },
          { text: 'Un día entero dedicado a ti', points: { 'time': 1 } },
          { text: 'Un abrazo que lo dijo todo', points: { 'touch': 1 } },
        ],
      },
    ],
    results: [
      { id: 'words', title: 'Palabras de Afirmación', description: 'Tu lenguaje principal son las palabras. Te sientes amado cuando escuchas "te quiero", recibes halagos sinceros y palabras de ánimo. Las críticas duelen profundamente. Comunica a tu pareja que las palabras positivas son esenciales para ti.', minScore: 0, maxScore: 5 },
      { id: 'acts', title: 'Actos de Servicio', description: 'Para ti, las acciones hablan más que las palabras. Te sientes amado cuando tu pareja hace cosas por ti: preparar el café, ayudarte con tareas o hacerte la vida más fácil. Recuerda que pedir ayuda también es válido.', minScore: 0, maxScore: 5 },
      { id: 'gifts', title: 'Recepción de Regalos', description: 'Los regalos son tu lenguaje del amor. No se trata del valor material, sino del pensamiento y esfuerzo detrás. Un regalo bien elegido te hace sentir visto y valorado. Aprecia los pequeños detalles cotidianos.', minScore: 0, maxScore: 5 },
      { id: 'time', title: 'Tiempo de Calidad', description: 'Valoras la atención plena por encima de todo. Necesitas tiempo ininterrumpido con tu pareja, sin teléfonos ni distracciones. Las conversaciones profundas y las actividades compartidas te llenan el alma.', minScore: 0, maxScore: 5 },
      { id: 'touch', title: 'Contacto Físico', description: 'El contacto físico es esencial para sentirte amado. Abrazos, besos, tomarse de la mano, caricias. La distancia física prolongada es especialmente difícil para ti. El sexo también es una expresión importante de amor.', minScore: 0, maxScore: 5 },
    ],
  },

  // 2. INTROVERT / EXTRAVERT QUIZ (10 questions)
  {
    slug: 'introvertido-extrovertido',
    title: '¿Eres Introvertido o Extrovertido?',
    description: 'Descubre tu nivel de extraversión con este test rápido basado en el modelo Big Five de personalidad.',
    category: 'big-five',
    icon: '🔄',
    questions: [
      {
        question: 'Después de una semana socialmente intensa, ¿cómo recargas?',
        options: [
          { text: 'Buscando tiempo a solas para desconectar', points: { 'I': 1 } },
          { text: 'Saliendo con amigos para desconectar', points: { 'E': 1 } },
        ],
      },
      {
        question: 'En una fiesta, ¿dónde te encuentras más cómodo/a?',
        options: [
          { text: 'En una conversación profunda con 1-2 personas', points: { 'I': 1 } },
          { text: 'Conectando con diferentes grupos de personas', points: { 'E': 1 } },
        ],
      },
      {
        question: 'Cuando tienes que pensar en algo importante, prefieres:',
        options: [
          { text: 'Pensarlo en silencio y luego compartirlo', points: { 'I': 1 } },
          { text: 'Hablar con otros para ordenar tus ideas', points: { 'E': 1 } },
        ],
      },
      {
        question: '¿Cómo te sientes con el trabajo en equipo?',
        options: [
          { text: 'Prefiero trabajar solo o en grupos muy pequeños', points: { 'I': 1 } },
          { text: 'Me energiza colaborar con muchas personas', points: { 'E': 1 } },
        ],
      },
      {
        question: '¿Qué tipo de conversación prefieres?',
        options: [
          { text: 'Conversaciones profundas sobre temas significativos', points: { 'I': 1 } },
          { text: 'Charlas ligeras y variadas con diferentes personas', points: { 'E': 1 } },
        ],
      },
      {
        question: 'En tu tiempo libre, ¿qué disfrutas más?',
        options: [
          { text: 'Leer, ver una serie o un hobby tranquilo', points: { 'I': 1 } },
          { text: 'Salir, conocer gente o actividades sociales', points: { 'E': 1 } },
        ],
      },
      {
        question: '¿Cómo te describirían tus amigos?',
        options: [
          { text: 'Reservado/a, buen oyente, reflexivo/a', points: { 'I': 1 } },
          { text: 'Sociable, hablador/a, enérgico/a', points: { 'E': 1 } },
        ],
      },
      {
        question: 'Cuando necesitas concentrarte, prefieres:',
        options: [
          { text: 'Silencio absoluto y sin interrupciones', points: { 'I': 1 } },
          { text: 'Un ambiente animado con algo de ruido de fondo', points: { 'E': 1 } },
        ],
      },
      {
        question: '¿Cómo manejas las reuniones sociales largas?',
        options: [
          { text: 'Disfruto las primeras horas pero luego me agoto', points: { 'I': 1 } },
          { text: 'Podría seguir toda la noche sin problema', points: { 'E': 1 } },
        ],
      },
      {
        question: '¿Qué prefieres en tus vacaciones?',
        options: [
          { text: 'Un destino tranquilo para relajarte', points: { 'I': 1 } },
          { text: 'Un lugar animado con muchas actividades', points: { 'E': 1 } },
        ],
      },
    ],
    results: [
      { id: 'introvert', title: 'Introvertido/a', description: 'Eres predominantemente introvertido/a. Recargas energía en la soledad o con personas muy cercanas. Prefieres la profundidad sobre la cantidad en tus relaciones. Tu mundo interior es rico y valoras el tiempo para reflexionar. Esto no es timidez — es tu forma natural de procesar el mundo.', minScore: 7, maxScore: 10 },
      { id: 'ambivert', title: 'Ambivertido/a', description: 'Tienes un equilibrio entre introversión y extraversión. Eres flexible: puedes disfrutar tanto de fiestas animadas como de tardes tranquilas en casa. Esta adaptabilidad es una gran fortaleza. Según el contexto y tu estado de ánimo, puedes inclinarte hacia un lado u otro.', minScore: 4, maxScore: 6 },
      { id: 'extravert', title: 'Extrovertido/a', description: 'Eres predominantemente extrovertido/a. Recargas energía en la interacción social y buscas entornos estimulantes. Piensas en voz alta y procesas externamente. Tu energía y sociabilidad son contagiosas. Recuerda también crear espacio para la introspección.', minScore: 0, maxScore: 3 },
    ],
  },

  // 3. ENNEAGRAM ABRIDGED (9 questions - one per type)
  {
    slug: 'eneagrama-rapido',
    title: '¿Cuál es tu Eneatipo? (Versión Rápida)',
    description: 'Una versión abreviada del test de Eneagrama para identificar tu tipo predominante en solo 9 preguntas.',
    category: 'enneagram',
    icon: '🔵',
    questions: [
      {
        question: '¿Qué describe mejor tu motivación principal?',
        options: [
          { text: 'Hacer lo correcto y mejorar el mundo', points: { 'type1': 1 } },
          { text: 'Ser amado y necesitado por los demás', points: { 'type2': 1 } },
          { text: 'Tener éxito y ser admirado', points: { 'type3': 1 } },
          { text: 'Ser auténtico y encontrar mi identidad única', points: { 'type4': 1 } },
          { text: 'Adquirir conocimiento y ser competente', points: { 'type5': 1 } },
          { text: 'Sentirme seguro y preparado para lo peor', points: { 'type6': 1 } },
          { text: 'Disfrutar la vida y evitar el dolor', points: { 'type7': 1 } },
          { text: 'Ser fuerte y evitar ser controlado', points: { 'type8': 1 } },
          { text: 'Mantener la paz y evitar conflictos', points: { 'type9': 1 } },
        ],
      },
      {
        question: '¿Cuál es tu mayor miedo?',
        options: [
          { text: 'Ser corrupto, defectuoso o malo', points: { 'type1': 1 } },
          { text: 'Ser no deseado o no amado', points: { 'type2': 1 } },
          { text: 'Ser considerado un fracaso', points: { 'type3': 1 } },
          { text: 'No tener identidad o ser insignificante', points: { 'type4': 1 } },
          { text: 'Ser invadido, abrumado o incompetente', points: { 'type5': 1 } },
          { text: 'La incertidumbre y ser traicionado', points: { 'type6': 1 } },
          { text: 'El dolor, el aburrimiento y la limitación', points: { 'type7': 1 } },
          { text: 'Ser vulnerable o controlado por otros', points: { 'type8': 1 } },
          { text: 'El conflicto y la separación de los demás', points: { 'type9': 1 } },
        ],
      },
      {
        question: '¿Cuál es tu mayor fortaleza?',
        options: [
          { text: 'Integridad y altos estándares', points: { 'type1': 1 } },
          { text: 'Generosidad y empatía', points: { 'type2': 1 } },
          { text: 'Carisma y capacidad de logro', points: { 'type3': 1 } },
          { text: 'Creatividad y autenticidad', points: { 'type4': 1 } },
          { text: 'Sabiduría y pensamiento analítico', points: { 'type5': 1 } },
          { text: 'Lealtad y responsabilidad', points: { 'type6': 1 } },
          { text: 'Optimismo y entusiasmo', points: { 'type7': 1 } },
          { text: 'Valentía y liderazgo', points: { 'type8': 1 } },
          { text: 'Paciencia y capacidad de mediar', points: { 'type9': 1 } },
        ],
      },
      {
        question: '¿Cómo reaccionas ante el estrés?',
        options: [
          { text: 'Me vuelvo más crítico y controlador', points: { 'type1': 1 } },
          { text: 'Busco ayudar a otros para sentirme mejor', points: { 'type2': 1 } },
          { text: 'Trabajo más duro para tener el control', points: { 'type3': 1 } },
          { text: 'Me retiro a mi mundo interior', points: { 'type4': 1 } },
          { text: 'Necesito más tiempo a solas para procesar', points: { 'type5': 1 } },
          { text: 'Imagino todos los escenarios negativos posibles', points: { 'type6': 1 } },
          { text: 'Busco distracciones y planes alternativos', points: { 'type7': 1 } },
          { text: 'Tomo el control de la situación', points: { 'type8': 1 } },
          { text: 'Me desconecto y evito pensar en ello', points: { 'type9': 1 } },
        ],
      },
      {
        question: '¿Qué frase resuena más contigo?',
        options: [
          { text: '"Hazlo bien o no lo hagas"', points: { 'type1': 1 } },
          { text: '"Es mejor dar que recibir"', points: { 'type2': 1 } },
          { text: '"El éxito es la mejor venganza"', points: { 'type3': 1 } },
          { text: '"Sé tú mismo, los demás ya están ocupados"', points: { 'type4': 1 } },
          { text: '"El conocimiento es poder"', points: { 'type5': 1 } },
          { text: '"Más vale prevenir que curar"', points: { 'type6': 1 } },
          { text: '"La vida es demasiado corta para..."', points: { 'type7': 1 } },
          { text: '"La mejor defensa es un buen ataque"', points: { 'type8': 1 } },
          { text: '"No hay problema que no pueda esperar"', points: { 'type9': 1 } },
        ],
      },
      {
        question: '¿Qué emoción te cuesta más manejar?',
        options: [
          { text: 'La ira (la reprimo o la canalizo)', points: { 'type1': 1 } },
          { text: 'El orgullo por mi generosidad', points: { 'type2': 1 } },
          { text: 'La vergüenza por no ser suficiente', points: { 'type3': 1 } },
          { text: 'La envidia por lo que otros tienen', points: { 'type4': 1 } },
          { text: 'La avaricia por mi tiempo y energía', points: { 'type5': 1 } },
          { text: 'El miedo y la ansiedad constantes', points: { 'type6': 1 } },
          { text: 'La gula por experiencias y placeres', points: { 'type7': 1 } },
          { text: 'El exceso y la dificultad para controlarme', points: { 'type8': 1 } },
          { text: 'La pereza y la tendencia a desconectarme', points: { 'type9': 1 } },
        ],
      },
      {
        question: '¿Cómo te describirían tus amigos más cercanos?',
        options: [
          { text: 'Íntegro/a, responsable y a veces perfeccionista', points: { 'type1': 1 } },
          { text: 'Generoso/a, atento/a y siempre dispuesto/a a ayudar', points: { 'type2': 1 } },
          { text: 'Carismático/a, exitoso/a y orientado/a a metas', points: { 'type3': 1 } },
          { text: 'Auténtico/a, sensible y profundamente creativo/a', points: { 'type4': 1 } },
          { text: 'Inteligente, independiente y un poco reservado/a', points: { 'type5': 1 } },
          { text: 'Leal, responsable y siempre preparado/a', points: { 'type6': 1 } },
          { text: 'Divertido/a, aventurero/a y lleno/a de energía', points: { 'type7': 1 } },
          { text: 'Directo/a, fuerte y protector/a', points: { 'type8': 1 } },
          { text: 'Tranquilo/a, paciente y fácil de llevar', points: { 'type9': 1 } },
        ],
      },
      {
        question: '¿Qué necesitas mejorar para ser más feliz?',
        options: [
          { text: 'Aceptar la imperfección y relajarme', points: { 'type1': 1 } },
          { text: 'Aprender a recibir y poner límites', points: { 'type2': 1 } },
          { text: 'Conectar con mis emociones auténticas', points: { 'type3': 1 } },
          { text: 'Valorar lo cotidiano, no solo lo excepcional', points: { 'type4': 1 } },
          { text: 'Salir de mi cabeza y conectar con otros', points: { 'type5': 1 } },
          { text: 'Confiar más en mí mismo y en los demás', points: { 'type6': 1 } },
          { text: 'Comprometerme y estar presente en el ahora', points: { 'type7': 1 } },
          { text: 'Ser vulnerable y permitirme sentir', points: { 'type8': 1 } },
          { text: 'Ocupar mi espacio y expresar mis opiniones', points: { 'type9': 1 } },
        ],
      },
      {
        question: '¿Qué tipo de película te atrae más?',
        options: [
          { text: 'Dramas sobre dilemas morales y justicia', points: { 'type1': 1 } },
          { text: 'Historias de amor y sacrificio personal', points: { 'type2': 1 } },
          { text: 'Películas sobre triunfo y superación', points: { 'type3': 1 } },
          { text: 'Arte, drama psicológico o cine independiente', points: { 'type4': 1 } },
          { text: 'Ciencia ficción, documentales o misterio', points: { 'type5': 1 } },
          { text: 'Suspenso, thrillers y tramas impredecibles', points: { 'type6': 1 } },
          { text: 'Comedia, aventura o fantasía', points: { 'type7': 1 } },
          { text: 'Acción, westerns o dramas de poder', points: { 'type8': 1 } },
          { text: 'Cualquier cosa que me haga sentir bien', points: { 'type9': 1 } },
        ],
      },
    ],
    results: [
      { id: 'type1', title: 'Tipo 1 — El Perfeccionista', description: 'Eres un idealista con un fuerte sentido del bien y del mal. Tu integridad y altos estándares te hacen confiable. Tu camino de crecimiento es aprender a aceptar la imperfección y ser más flexible contigo mismo y con los demás.', minScore: 3, maxScore: 9 },
      { id: 'type2', title: 'Tipo 2 — El Ayudador', description: 'Eres generoso/a y tienes un corazón enorme. Tu capacidad para cuidar de los demás es tu superpoder. Tu camino de crecimiento es aprender a recibir tanto como das y a cuidar de ti mismo/a.', minScore: 3, maxScore: 9 },
      { id: 'type3', title: 'Tipo 3 — El Triunfador', description: 'Eres ambicioso/a, carismático/a y orientado/a al logro. Tu capacidad para alcanzar metas es admirable. Tu camino de crecimiento es conectar con tu ser auténtico más allá de tus logros.', minScore: 3, maxScore: 9 },
      { id: 'type4', title: 'Tipo 4 — El Individualista', description: 'Eres sensible, creativo/a y profundamente auténtico/a. Tu riqueza emocional es un don. Tu camino de crecimiento es encontrar valor en la vida ordinaria, no solo en lo excepcional.', minScore: 3, maxScore: 9 },
      { id: 'type5', title: 'Tipo 5 — El Investigador', description: 'Tienes una mente analítica y una sed insaciable de conocimiento. Tu capacidad de comprensión profunda es única. Tu camino de crecimiento es compartir tu sabiduría y conectar con tu cuerpo.', minScore: 3, maxScore: 9 },
      { id: 'type6', title: 'Tipo 6 — El Leal', description: 'Eres leal, responsable y siempre anticipas problemas. Tu capacidad de prever riesgos es invaluable. Tu camino de crecimiento es desarrollar confianza en ti mismo/a y en los demás.', minScore: 3, maxScore: 9 },
      { id: 'type7', title: 'Tipo 7 — El Entusiasta', description: 'Eres optimista, aventurero/a y lleno/a de energía. Tu entusiasmo es contagioso. Tu camino de crecimiento es aprender a estar presente con todas las experiencias, incluyendo las difíciles.', minScore: 3, maxScore: 9 },
      { id: 'type8', title: 'Tipo 8 — El Desafiador', description: 'Eres fuerte, directo/a y protector/a con quienes quieres. Tu liderazgo natural es imponente. Tu camino de crecimiento es permitirte ser vulnerable y conectar con tu ternura.', minScore: 3, maxScore: 9 },
      { id: 'type9', title: 'Tipo 9 — El Pacificador', description: 'Eres tranquilo/a, paciente y buscas armonía en todo. Tu capacidad de ver múltiples perspectivas es un don. Tu camino de crecimiento es ocupar tu espacio y valorar tus propias opiniones.', minScore: 3, maxScore: 9 },
    ],
  },

  // 4. COMMUNICATION STYLE (8 questions)
  {
    slug: 'estilo-comunicacion',
    title: '¿Cuál es tu Estilo de Comunicación?',
    description: 'Descubre cómo te comunicas con los demás y cómo mejorar tu efectividad comunicativa según tu perfil.',
    category: 'self-improvement',
    icon: '💬',
    questions: [
      {
        question: 'Cuando expresas una opinión, ¿cómo lo haces?',
        options: [
          { text: 'Directamente, voy al grano sin rodeos', points: { 'direct': 1 } },
          { text: 'Busco mantener la armonía, incluso si no digo todo', points: { 'harmony': 1 } },
          { text: 'Exploro diferentes perspectivas sin fijar posición', points: { 'exploratory': 1 } },
          { text: 'Con datos y argumentos lógicos que respaldan mi posición', points: { 'analytical': 1 } },
        ],
      },
      {
        question: '¿Cómo manejas un desacuerdo?',
        options: [
          { text: 'Lo enfrento directamente, prefiero resolverlo rápido', points: { 'direct': 1 } },
          { text: 'Evito el conflicto, prefiero ceder para mantener la paz', points: { 'harmony': 1 } },
          { text: 'Exploro diferentes ángulos antes de decidir', points: { 'exploratory': 1 } },
          { text: 'Presento evidencia y razono mi posición', points: { 'analytical': 1 } },
        ],
      },
      {
        question: '¿Qué valoras más en una conversación?',
        options: [
          { text: 'La eficiencia y llegar a conclusiones claras', points: { 'direct': 1 } },
          { text: 'La conexión emocional y la empatía', points: { 'harmony': 1 } },
          { text: 'La exploración de ideas y posibilidades', points: { 'exploratory': 1 } },
          { text: 'La precisión y el rigor de los argumentos', points: { 'analytical': 1 } },
        ],
      },
      {
        question: 'En una reunión de trabajo, ¿cómo participas?',
        options: [
          { text: 'Tomando la iniciativa y liderando la conversación', points: { 'direct': 1 } },
          { text: 'Apoyando las ideas de otros y buscando consenso', points: { 'harmony': 1 } },
          { text: 'Proponiendo ideas creativas y alternativas', points: { 'exploratory': 1 } },
          { text: 'Analizando datos y preguntando por evidencia', points: { 'analytical': 1 } },
        ],
      },
      {
        question: '¿Qué te frustra más al comunicarte?',
        options: [
          { text: 'La falta de claridad o que den rodeos', points: { 'direct': 1 } },
          { text: 'La rudeza o falta de consideración', points: { 'harmony': 1 } },
          { text: 'La rigidez o cerrarse a nuevas ideas', points: { 'exploratory': 1 } },
          { text: 'La falta de datos o argumentos débiles', points: { 'analytical': 1 } },
        ],
      },
      {
        question: '¿Cómo prefieres recibir feedback?',
        options: [
          { text: 'Directo y sin rodeos, aunque sea duro', points: { 'direct': 1 } },
          { text: 'Con cuidado y consideración por mis sentimientos', points: { 'harmony': 1 } },
          { text: 'Como parte de una conversación abierta', points: { 'exploratory': 1 } },
          { text: 'Con datos concretos y ejemplos específicos', points: { 'analytical': 1 } },
        ],
      },
      {
        question: '¿Cómo te preparas para una conversación importante?',
        options: [
          { text: 'Defino mis puntos clave y voy directo', points: { 'direct': 1 } },
          { text: 'Piensen en cómo se sentirá la otra persona', points: { 'harmony': 1 } },
          { text: 'No me preparo mucho, confío en mi improvisación', points: { 'exploratory': 1 } },
          { text: 'Investigo, preparo datos y estructuro mis argumentos', points: { 'analytical': 1 } },
        ],
      },
      {
        question: '¿Qué tipo de comunicador te gustaría ser?',
        options: [
          { text: 'Claro, directo y convincente', points: { 'direct': 1 } },
          { text: 'Empático, cálido y conector', points: { 'harmony': 1 } },
          { text: 'Inspirador, creativo y visionario', points: { 'exploratory': 1 } },
          { text: 'Preciso, riguroso y creíble', points: { 'analytical': 1 } },
        ],
      },
    ],
    results: [
      { id: 'direct', title: 'Comunicador Directo', description: 'Eres claro, directo y valoras la eficiencia. Tu estilo es asertivo y orientado a la acción. Los demás saben exactamente qué piensas. Para mejorar, practica la escucha activa y considera el impacto emocional de tus palabras en los demás.', minScore: 5, maxScore: 8 },
      { id: 'harmony', title: 'Comunicador Armonioso', description: 'Eres diplomático, empático y valoras las relaciones. Te preocupas por cómo se sienten los demás. Tu capacidad de crear armonía es valiosa. Para mejorar, practica expresar tus propias opiniones aunque puedan generar conflicto.', minScore: 5, maxScore: 8 },
      { id: 'exploratory', title: 'Comunicador Explorador', description: 'Eres creativo, abierto y generas ideas. Disfrutas explorar posibilidades sin cerrarte. Tu flexibilidad es inspiradora. Para mejorar, trabaja en concretar ideas y tomar decisiones cuando sea necesario.', minScore: 5, maxScore: 8 },
      { id: 'analytical', title: 'Comunicador Analítico', description: 'Eres preciso, riguroso y basado en datos. Tus argumentos son sólidos y bien estructurados. Tu credibilidad es tu fortaleza. Para mejorar, conecta emocionalmente y no subestimes el poder de la intuición.', minScore: 5, maxScore: 8 },
    ],
  },

  // 5. EMOTIONAL INTELLIGENCE (10 questions)
  {
    slug: 'inteligencia-emocional-rapido',
    title: '¿Qué tan Inteligente Emocionalmente Eres?',
    description: 'Evalúa rápidamente tu inteligencia emocional en áreas como autoconciencia, empatía, regulación y habilidades sociales.',
    category: 'emotional-intelligence',
    icon: '💎',
    questions: [
      {
        question: 'Cuando sientes una emoción intensa, ¿qué haces?',
        options: [
          { text: 'La identifico y nombro la emoción específica', points: { 'high': 1 } },
          { text: 'La siento pero me cuesta ponerle nombre', points: { 'mid': 1 } },
          { text: 'Actúo sin darme cuenta de lo que siento', points: { 'low': 1 } },
        ],
      },
      {
        question: '¿Cómo respondes cuando alguien te crítica?',
        options: [
          { text: 'Escucho, evalúo si hay verdad y aprendo', points: { 'high': 1 } },
          { text: 'Me lo tomo personal pero luego lo proceso', points: { 'mid': 1 } },
          { text: 'Me pongo a la defensiva inmediatamente', points: { 'low': 1 } },
        ],
      },
      {
        question: '¿Puedes mantener la calma bajo presión?',
        options: [
          { text: 'Sí, tengo técnicas para regularme', points: { 'high': 1 } },
          { text: 'A veces, depende del día', points: { 'mid': 1 } },
          { text: 'No, reacciono impulsivamente', points: { 'low': 1 } },
        ],
      },
      {
        question: 'Cuando alguien está triste, ¿cómo reaccionas?',
        options: [
          { text: 'Ofrezco compañía y valido sus sentimientos', points: { 'high': 1 } },
          { text: 'Intento animarlo/a o dar soluciones', points: { 'mid': 1 } },
          { text: 'Me siento incómodo/a y no sé qué hacer', points: { 'low': 1 } },
        ],
      },
      {
        question: '¿Conoces tus fortalezas y debilidades?',
        options: [
          { text: 'Sí, las tengo claras y trabajo en ellas', points: { 'high': 1 } },
          { text: 'Tengo una idea general pero no muy precisa', points: { 'mid': 1 } },
          { text: 'No suelo reflexionar sobre eso', points: { 'low': 1 } },
        ],
      },
      {
        question: '¿Cómo manejas el estrés prolongado?',
        options: [
          { text: 'Identifico la causa y tomo acción', points: { 'high': 1 } },
          { text: 'Aguanto hasta que pase', points: { 'mid': 1 } },
          { text: 'Exploto o me derrumbo', points: { 'low': 1 } },
        ],
      },
      {
        question: '¿Qué tan bien lees las emociones de los demás?',
        options: [
          { text: 'Muy bien, capto señales sutiles', points: { 'high': 1 } },
          { text: 'A veces, si son muy evidentes', points: { 'mid': 1 } },
          { text: 'Se me da muy mal', points: { 'low': 1 } },
        ],
      },
      {
        question: 'Cuando cometes un error, ¿cómo lo manejas?',
        options: [
          { text: 'Lo reconozco, aprendo y sigo adelante', points: { 'high': 1 } },
          { text: 'Me culpo pero luego lo supero', points: { 'mid': 1 } },
          { text: 'Lo justifico o echo la culpa a otros', points: { 'low': 1 } },
        ],
      },
      {
        question: '¿Cómo manejas las relaciones difíciles?',
        options: [
          { text: 'Comunico mis límites con respeto', points: { 'high': 1 } },
          { text: 'Las evito o las soporto en silencio', points: { 'mid': 1 } },
          { text: 'Reacciono mal o corto la relación', points: { 'low': 1 } },
        ],
      },
      {
        question: '¿Practicas la escucha activa?',
        options: [
          { text: 'Sí, escucho sin interrumpir ni juzgar', points: { 'high': 1 } },
          { text: 'Lo intento pero a veces me distraigo', points: { 'mid': 1 } },
          { text: 'Suelo pensar en mi respuesta mientras otros hablan', points: { 'low': 1 } },
        ],
      },
    ],
    results: [
      { id: 'high', title: 'Alta Inteligencia Emocional', description: 'Tienes una inteligencia emocional desarrollada. Reconoces tus emociones, las manejas efectivamente y te conectas con los demás con empatía. Continúa practicando y enseñando a otros lo que sabes.', minScore: 7, maxScore: 10 },
      { id: 'mid', title: 'Inteligencia Emocional Moderada', description: 'Tienes bases sólidas de inteligencia emocional pero áreas de mejora. Trabaja en la autoconciencia y la regulación emocional. La práctica de mindfulness y la comunicación asertiva te ayudarán.', minScore: 4, maxScore: 6 },
      { id: 'low', title: 'Áreas de Oportunidad', description: 'La inteligencia emocional se puede desarrollar como cualquier habilidad. Empieza por practicar la autoconciencia: identifica y nombra tus emociones diariamente. La lectura sobre IE y la terapia pueden ser grandes aliados.', minScore: 0, maxScore: 3 },
    ],
  },

  // 6. DECISION-MAKING STYLE (6 questions)
  {
    slug: 'estilo-decisiones',
    title: '¿Cuál es tu Estilo de Toma de Decisiones?',
    description: 'Descubre cómo tomas decisiones: ¿racional, intuitivo, dependiente o espontáneo? Conoce tu estilo para mejorar tu proceso decisorio.',
    category: 'self-improvement',
    icon: '🤔',
    questions: [
      {
        question: 'Ante una decisión importante, ¿qué haces primero?',
        options: [
          { text: 'Analizo todos los datos y opciones disponibles', points: { 'rational': 1 } },
          { text: 'Confío en mi corazonada o instinto', points: { 'intuitive': 1 } },
          { text: 'Busco consejo de personas de confianza', points: { 'dependent': 1 } },
          { text: 'Decido rápido, confío en mi improvisación', points: { 'spontaneous': 1 } },
        ],
      },
      {
        question: '¿Cuánto tiempo te tomas para decidir?',
        options: [
          { text: 'El tiempo necesario para analizar a fondo', points: { 'rational': 1 } },
          { text: 'Lo que me dicte la intuición', points: { 'intuitive': 1 } },
          { text: 'Hasta que consulte con las personas adecuadas', points: { 'dependent': 1 } },
          { text: 'Muy poco, decido sobre la marcha', points: { 'spontaneous': 1 } },
        ],
      },
      {
        question: '¿Qué te da más confianza al decidir?',
        options: [
          { text: 'Tener datos, evidencias y análisis', points: { 'rational': 1 } },
          { text: 'Sentir que es la decisión correcta', points: { 'intuitive': 1 } },
          { text: 'Saber que otros apoyan mi decisión', points: { 'dependent': 1 } },
          { text: 'La emoción y la oportunidad del momento', points: { 'spontaneous': 1 } },
        ],
      },
      {
        question: '¿Cómo manejas el riesgo al decidir?',
        options: [
          { text: 'Calculo probabilidades y mitigo riesgos', points: { 'rational': 1 } },
          { text: 'Sigo mi instinto sobre qué riesgos tomar', points: { 'intuitive': 1 } },
          { text: 'Prefiero decisiones seguras y consultadas', points: { 'dependent': 1 } },
          { text: 'Acepto el riesgo, me emociona la incertidumbre', points: { 'spontaneous': 1 } },
        ],
      },
      {
        question: '¿Cómo te sientes después de decidir?',
        options: [
          { text: 'Seguro porque analicé todas las variables', points: { 'rational': 1 } },
          { text: 'Tranquilo porque confío en mi intuición', points: { 'intuitive': 1 } },
          { text: 'Necesito reafirmación de que fue correcta', points: { 'dependent': 1 } },
          { text: ' emocionado por ver qué pasa, sin mirar atrás', points: { 'spontaneous': 1 } },
        ],
      },
      {
        question: '¿Cuál es tu mayor debilidad al decidir?',
        options: [
          { text: 'Parálisis por análisis: demasiada información', points: { 'rational': 1 } },
          { text: 'A veces ignoro datos importantes', points: { 'intuitive': 1 } },
          { text: 'Dependencia excesiva de la opinión de otros', points: { 'dependent': 1 } },
          { text: 'Impulsividad: decido sin pensar suficiente', points: { 'spontaneous': 1 } },
        ],
      },
    ],
    results: [
      { id: 'rational', title: 'Estilo Racional', description: 'Eres analítico y metódico al decidir. Evaluas opciones, sopesas pros y contras, y buscas la decisión óptima. Tu fortaleza es la calidad de tus decisiones. Cuida la "parálisis por análisis" — a veces una buena decisión a tiempo es mejor que la decisión perfecta tarde.', minScore: 3, maxScore: 6 },
      { id: 'intuitive', title: 'Estilo Intuitivo', description: 'Confías en tu intuición y corazonadas. Procesas información de manera holística y llegas a conclusiones sin siempre saber cómo. Tu fortaleza es la velocidad y la creatividad. Complementa tu intuición con datos cuando las decisiones sean muy importantes.', minScore: 3, maxScore: 6 },
      { id: 'dependent', title: 'Estilo Dependiente', description: 'Buscas consejo y validación antes de decidir. Valoras la opinión de expertos y personas de confianza. Tu fortaleza es que consideras múltiples perspectivas. Trabaja en confiar más en tu propio juicio para decisiones cotidianas.', minScore: 3, maxScore: 6 },
      { id: 'spontaneous', title: 'Estilo Espontáneo', description: 'Decides rápido y confías en tu capacidad de adaptación. Te gusta la acción y no temes la incertidumbre. Tu fortaleza es la agilidad y la capacidad de aprovechar oportunidades. Para decisiones importantes, tómate al menos un momento para reflexionar.', minScore: 3, maxScore: 6 },
    ],
  },

  // 7. STRESS HANDLING (8 questions)
  {
    slug: 'manejo-estres',
    title: '¿Cómo Manejas el Estrés?',
    description: 'Descubre tu estilo de afrontamiento al estrés y qué estrategias funcionan mejor para ti según tu perfil.',
    category: 'health',
    icon: '🧘',
    questions: [
      {
        question: 'Cuando estás bajo presión, ¿qué haces?',
        options: [
          { text: 'Hago un plan y lo ejecuto paso a paso', points: { 'task': 1 } },
          { text: 'Busco apoyo emocional en otros', points: { 'social': 1 } },
          { text: 'Trato de ver el lado positivo', points: { 'optimistic': 1 } },
          { text: 'Me desconecto para no pensar en ello', points: { 'avoidant': 1 } },
        ],
      },
      {
        question: '¿Qué te ayuda más a relajarte?',
        options: [
          { text: 'Resolver el problema que me estresa', points: { 'task': 1 } },
          { text: 'Hablar con alguien de confianza', points: { 'social': 1 } },
          { text: 'Hacer ejercicio o meditación', points: { 'optimistic': 1 } },
          { text: 'Ver series, videojuegos o distraerme', points: { 'avoidant': 1 } },
        ],
      },
      {
        question: 'Ante un problema grande, tu primer pensamiento es:',
        options: [
          { text: '"Necesito un plan para resolver esto"', points: { 'task': 1 } },
          { text: '"Necesito hablar con alguien"', points: { 'social': 1 } },
          { text: '"Esto también pasará"', points: { 'optimistic': 1 } },
          { text: '"No quiero pensar en esto ahora"', points: { 'avoidant': 1 } },
        ],
      },
      {
        question: '¿Cómo describirías tu nivel de estrés diario?',
        options: [
          { text: 'Gestionable, tengo sistemas para manejarlo', points: { 'task': 1 } },
          { text: 'Varía según mis relaciones', points: { 'social': 1 } },
          { text: 'Bajo, tiendo a no estresarme mucho', points: { 'optimistic': 1 } },
          { text: 'Alto, evito pensar en ello', points: { 'avoidant': 1 } },
        ],
      },
      {
        question: 'Cuando algo sale mal en el trabajo, ¿cómo reaccionas?',
        options: [
          { text: 'Analizo qué salió mal y cómo arreglarlo', points: { 'task': 1 } },
          { text: 'Comparto mi frustración con compañeros', points: { 'social': 1 } },
          { text: 'Pienso que mañana será mejor', points: { 'optimistic': 1 } },
          { text: 'Me tomo un descanso para desconectar', points: { 'avoidant': 1 } },
        ],
      },
      {
        question: '¿Qué estrategia te funciona mejor a largo plazo?',
        options: [
          { text: 'Organización y planificación anticipada', points: { 'task': 1 } },
          { text: 'Mantener una red de apoyo sólida', points: { 'social': 1 } },
          { text: 'Practicar gratitud y mindfulness', points: { 'optimistic': 1 } },
          { text: 'Tener hobbies que me desconecten', points: { 'avoidant': 1 } },
        ],
      },
      {
        question: '¿Cómo afecta el estrés a tu cuerpo?',
        options: [
          { text: 'Tensión muscular y dolores de cabeza', points: { 'task': 1 } },
          { text: 'Problemas digestivos o cambios de apetito', points: { 'social': 1 } },
          { text: 'Noto poca reacción física', points: { 'optimistic': 1 } },
          { text: 'Fatiga y problemas de sueño', points: { 'avoidant': 1 } },
        ],
      },
      {
        question: '¿Qué te gustaría mejorar en tu manejo del estrés?',
        options: [
          { text: 'Ser más flexible cuando los planes fallan', points: { 'task': 1 } },
          { text: 'Aprender a poner límites emocionales', points: { 'social': 1 } },
          { text: ' Ser más proactivo en lugar de solo positivo', points: { 'optimistic': 1 } },
          { text: 'Enfrentar los problemas en lugar de evitarlos', points: { 'avoidant': 1 } },
        ],
      },
    ],
    results: [
      { id: 'task', title: 'Estilo Orientado a la Tarea', description: 'Tu estrategia principal es resolver el problema que causa el estrés. Eres práctico/a, organizado/a y proactivo/a. Tu fortaleza es que enfrentas los problemas de frente. Recuerda también cuidar tu bienestar emocional, no solo resolver problemas externos.', minScore: 4, maxScore: 8 },
      { id: 'social', title: 'Estilo Orientado al Apoyo Social', description: 'Tu estrategia principal es buscar apoyo en otros. Hablar con personas de confianza te ayuda a procesar y sentirte acompañado/a. Tu fortaleza es tu red de apoyo. Recuerda también desarrollar estrategias para cuando no puedas hablar con alguien.', minScore: 4, maxScore: 8 },
      { id: 'optimistic', title: 'Estilo Optimista y Regulador', description: 'Mantienes una actitud positiva y usas técnicas de regulación emocional. Tu fortaleza es tu resiliencia y capacidad de ver el lado bueno. Asegúrate de no caer en el "pensamiento positivo" tóxico que ignora problemas reales.', minScore: 4, maxScore: 8 },
      { id: 'avoidant', title: 'Estilo Evitativo', description: 'Tiendes a distraerte o desconectarte del estrés. Esto puede ser útil a corto plazo pero no resuelve la causa. Trabaja en incorporar estrategias de afrontamiento activo: enfrentar los problemas gradualmente y desarrollar herramientas de regulación.', minScore: 4, maxScore: 8 },
    ],
  },

  // 8. LEADERSHIP STYLE (8 questions)
  {
    slug: 'estilo-liderazgo',
    title: '¿Cuál es tu Estilo de Liderazgo?',
    description: 'Descubre tu estilo natural de liderazgo y cómo potenciarlo según tu tipo de personalidad.',
    category: 'self-improvement',
    icon: '👔',
    questions: [
      {
        question: '¿Cómo motivas a tu equipo?',
        options: [
          { text: 'Con una visión clara y metas ambiciosas', points: { 'visionary': 1 } },
          { text: 'Apoyando y desarrollando a cada persona', points: { 'coach': 1 } },
          { text: 'Con estructura, orden y expectativas claras', points: { 'organizer': 1 } },
          { text: 'Dando libertad y confianza para innovar', points: { 'delegator': 1 } },
        ],
      },
      {
        question: '¿Cómo tomas decisiones como líder?',
        options: [
          { text: 'Decido yo con la información disponible', points: { 'visionary': 1 } },
          { text: 'Busco consenso y input del equipo', points: { 'coach': 1 } },
          { text: 'Basado en datos y procedimientos establecidos', points: { 'organizer': 1 } },
          { text: 'Delego la decisión a quien corresponda', points: { 'delegator': 1 } },
        ],
      },
      {
        question: '¿Qué valoras más en tu equipo?',
        options: [
          { text: 'Iniciativa y capacidad de ejecución', points: { 'visionary': 1 } },
          { text: 'Colaboración y buen ambiente', points: { 'coach': 1 } },
          { text: 'Responsabilidad y cumplimiento', points: { 'organizer': 1 } },
          { text: 'Creatividad y autonomía', points: { 'delegator': 1 } },
        ],
      },
      {
        question: '¿Cómo manejas los conflictos en el equipo?',
        options: [
          { text: 'Intervengo directamente para resolverlos', points: { 'visionary': 1 } },
          { text: 'Medio y busco que todos sean escuchados', points: { 'coach': 1 } },
          { text: 'Aplico las políticas y procedimientos', points: { 'organizer': 1 } },
          { text: 'Dejo que el equipo lo resuelva solo', points: { 'delegator': 1 } },
        ],
      },
      {
        question: '¿Qué tipo de líder admiras?',
        options: [
          { text: 'Steve Jobs — visión y determinación', points: { 'visionary': 1 } },
          { text: 'Oprah Winfrey — empatía y desarrollo', points: { 'coach': 1 } },
          { text: 'Angela Merkel — orden y pragmatismo', points: { 'organizer': 1 } },
          { text: 'Richard Branson — libertad y confianza', points: { 'delegator': 1 } },
        ],
      },
      {
        question: '¿Cómo comunicas tu visión?',
        options: [
          { text: 'Con pasión y claridad hacia el objetivo', points: { 'visionary': 1 } },
          { text: 'Conectando con los valores del equipo', points: { 'coach': 1 } },
          { text: 'Con planes detallados y cronogramas', points: { 'organizer': 1 } },
          { text: 'Compartiendo el "por qué" y dejando el "cómo"', points: { 'delegator': 1 } },
        ],
      },
      {
        question: '¿Cómo reaccionas ante el fracaso?',
        options: [
          { text: 'Analizo qué aprender y sigo adelante', points: { 'visionary': 1 } },
          { text: 'Apoyo al equipo para que no se desanime', points: { 'coach': 1 } },
          { text: 'Reviso los procesos para evitar que se repita', points: { 'organizer': 1 } },
          { text: 'Lo veo como parte del proceso de innovación', points: { 'delegator': 1 } },
        ],
      },
      {
        question: '¿Qué debilidad de liderazgo reconoces?',
        options: [
          { text: 'Puedo ser impaciente o autoritario', points: { 'visionary': 1 } },
          { text: 'Me cuesta tomar decisiones impopulares', points: { 'coach': 1 } },
          { text: 'Puedo ser demasiado rígido/a', points: { 'organizer': 1 } },
          { text: 'A veces parezco distante o desconectado/a', points: { 'delegator': 1 } },
        ],
      },
    ],
    results: [
      { id: 'visionary', title: 'Líder Visionario', description: 'Inspiras con una visión clara y motivas a tu equipo hacia metas ambiciosas. Eres estratégico/a, decidido/a y carismático/a. Tu desafío es equilibrar tu visión con las aportaciones del equipo y no atropellar a quienes no siguen tu ritmo.', minScore: 4, maxScore: 8 },
      { id: 'coach', title: 'Líder Coach', description: 'Tu fortaleza es desarrollar el potencial de tu equipo. Eres empático/a, inspirador/a y creas un ambiente de confianza. Tu desafío es tomar decisiones difíciles cuando sea necesario sin descuidar el bienestar del equipo.', minScore: 4, maxScore: 8 },
      { id: 'organizer', title: 'Líder Organizador', description: 'Eres estructurado/a, confiable y creas sistemas que funcionan. Tu equipo sabe exactamente qué esperar. Tu fortaleza es la consistencia. Tu desafío es ser más flexible y abrirte a la innovación cuando sea necesario.', minScore: 4, maxScore: 8 },
      { id: 'delegator', title: 'Líder Delegador', description: 'Confías en tu equipo y les das autonomía para innovar. Creas un ambiente de libertad y responsabilidad. Tu fortaleza es empoderar a otros. Tu desafío es mantener el rumbo claro y estar presente cuando el equipo te necesite.', minScore: 4, maxScore: 8 },
    ],
  },

  // 9. MORNING PERSON VS NIGHT OWL (6 questions)
  {
    slug: 'matutino-nocturno',
    title: '¿Eres Mañanero o Nocturno?',
    description: 'Descubre tu cronotipo y cómo afecta a tu productividad, energía y hábitos de sueño.',
    category: 'health',
    icon: '🌙',
    questions: [
      {
        question: 'Si no tuvieras obligaciones, ¿a qué hora te acostarías?',
        options: [
          { text: 'Antes de las 10 PM', points: { 'morning': 1 } },
          { text: 'Entre 10 PM y 12 AM', points: { 'neutral': 1 } },
          { text: 'Entre 12 AM y 2 AM', points: { 'night': 1 } },
          { text: 'Después de las 2 AM', points: { 'night': 2 } },
        ],
      },
      {
        question: '¿A qué hora te sientes más productivo/a?',
        options: [
          { text: 'Temprano en la mañana (6-9 AM)', points: { 'morning': 1 } },
          { text: 'Media mañana (9 AM - 12 PM)', points: { 'morning': 1 } },
          { text: 'Tarde (2-6 PM)', points: { 'neutral': 1 } },
          { text: 'Noche (8 PM - 12 AM)', points: { 'night': 1 } },
          { text: 'Madrugada (12 AM - 4 AM)', points: { 'night': 2 } },
        ],
      },
      {
        question: '¿Cómo es tu despertar natural (sin despertador)?',
        options: [
          { text: 'Me despierto temprano y descansado/a', points: { 'morning': 1 } },
          { text: 'Me despierto a una hora moderada', points: { 'neutral': 1 } },
          { text: 'Me cuesta mucho despertarme temprano', points: { 'night': 1 } },
          { text: 'Mi ritmo natural es despertarme muy tarde', points: { 'night': 2 } },
        ],
      },
      {
        question: '¿Cuándo tienes más energía física?',
        options: [
          { text: 'A primera hora de la mañana', points: { 'morning': 1 } },
          { text: 'A media mañana o mediodía', points: { 'neutral': 1 } },
          { text: 'Por la tarde', points: { 'neutral': 1 } },
          { text: 'Por la noche', points: { 'night': 1 } },
          { text: 'Muy entrada la noche', points: { 'night': 2 } },
        ],
      },
      {
        question: '¿Cómo te sientes acerca de las mañanas?',
        options: [
          { text: 'Las disfruto, es mi mejor momento', points: { 'morning': 1 } },
          { text: 'Son aceptables, funciono bien', points: { 'neutral': 1 } },
          { text: 'Las tolero pero no las disfruto', points: { 'night': 1 } },
          { text: 'Las detesto, no funciono hasta tarde', points: { 'night': 2 } },
        ],
      },
      {
        question: 'En vacaciones, ¿cómo es tu ciclo de sueño?',
        options: [
          { text: 'Similar a mi rutina, me acuesto y levanto temprano', points: { 'morning': 1 } },
          { text: 'Me acuesto un poco más tarde pero no mucho', points: { 'neutral': 1 } },
          { text: 'Mi horario se retrasa significativamente', points: { 'night': 1 } },
          { text: 'Vivo de noche y duermo de día', points: { 'night': 2 } },
        ],
      },
    ],
    results: [
      { id: 'morning', title: 'Alondra (Mañanero/a)', description: 'Eres una persona matutina. Te despiertas temprano con energía y tu mejor rendimiento es en las primeras horas del día. Por la noche, tu energía disminuye. Aprovecha las mañanas para tareas que requieren concentración. Eres más alineado/a con los horarios sociales convencionales.', minScore: 4, maxScore: 6 },
      { id: 'neutral', title: 'Colibrí (Intermedio/a)', description: 'Tienes un cronotipo intermedio. No eres ni marcadamente matutino/a ni nocturno/a. Puedes adaptarte a diferentes horarios con relativa facilidad. Tu flexibilidad es una ventaja. Escucha a tu cuerpo: hay momentos del día donde tu energía es más alta de forma natural.', minScore: 2, maxScore: 5 },
      { id: 'night', title: 'Búho (Nocturno/a)', description: 'Eres una persona nocturna. Tu energía y creatividad aumentan por la noche. Prefieres acostarte tarde y despertar tarde. La sociedad está diseñada para alondras, lo que puede ser frustrante. Si puedes, adapta tu horario laboral a tu ritmo natural. Protege tu tiempo de sueño aunque te acuestes tarde.', minScore: 4, maxScore: 8 },
    ],
  },

  // 10. LEARNING STYLE (8 questions)
  {
    slug: 'estilo-aprendizaje',
    title: '¿Cuál es tu Estilo de Aprendizaje?',
    description: 'Descubre cómo aprendes mejor: visual, auditivo, lector/escritor o kinestésico, y optimiza tu forma de estudiar.',
    category: 'education',
    icon: '📚',
    questions: [
      {
        question: 'Cuando aprendes algo nuevo, ¿qué te funciona mejor?',
        options: [
          { text: 'Ver diagramas, videos o demostraciones', points: { 'visual': 1 } },
          { text: 'Escuchar explicaciones o discusiones', points: { 'auditory': 1 } },
          { text: 'Leer instrucciones o tomar notas', points: { 'reading': 1 } },
          { text: 'Hacerlo yo mismo de manera práctica', points: { 'kinesthetic': 1 } },
        ],
      },
      {
        question: 'Cuando sigues instrucciones, prefieres:',
        options: [
          { text: 'Un mapa visual o diagrama de pasos', points: { 'visual': 1 } },
          { text: 'Que alguien te las explique verbalmente', points: { 'auditory': 1 } },
          { text: 'Instrucciones escritas detalladas', points: { 'reading': 1 } },
          { text: 'Ir probando sobre la marcha', points: { 'kinesthetic': 1 } },
        ],
      },
      {
        question: '¿Qué tipo de clase te resulta más fácil seguir?',
        options: [
          { text: 'Con presentaciones visuales y gráficos', points: { 'visual': 1 } },
          { text: 'Con debates y explicaciones orales', points: { 'auditory': 1 } },
          { text: 'Con lecturas y apuntes bien organizados', points: { 'reading': 1 } },
          { text: 'Talleres prácticos y ejercicios', points: { 'kinesthetic': 1 } },
        ],
      },
      {
        question: 'Para recordar información, ¿qué te ayuda más?',
        options: [
          { text: 'Asociarla con imágenes o colores', points: { 'visual': 1 } },
          { text: 'Grabarla o explicarla en voz alta', points: { 'auditory': 1 } },
          { text: 'Escribirla varias veces o resumirla', points: { 'reading': 1 } },
          { text: 'Practicarla con ejercicios reales', points: { 'kinesthetic': 1 } },
        ],
      },
      {
        question: 'Cuando usas una app nueva, ¿cómo aprendes?',
        options: [
          { text: 'Viendo un tutorial en video', points: { 'visual': 1 } },
          { text: 'Escuchando un podcast o explicación', points: { 'auditory': 1 } },
          { text: 'Leyendo la documentación o manual', points: { 'reading': 1 } },
          { text: 'Explorando la app directamente', points: { 'kinesthetic': 1 } },
        ],
      },
      {
        question: '¿Qué haces en una conferencia o charla?',
        options: [
          { text: 'Tomo notas visuales o dibujo conceptos', points: { 'visual': 1 } },
          { text: 'Escucho atentamente y grabo si puedo', points: { 'auditory': 1 } },
          { text: 'Tomo notas detalladas y escribo', points: { 'reading': 1 } },
          { text: 'Necesito moverme o hacer algo mientras', points: { 'kinesthetic': 1 } },
        ],
      },
      {
        question: '¿Cómo prefieres estudiar para un examen?',
        options: [
          { text: 'Usando mapas mentales y diagramas', points: { 'visual': 1 } },
          { text: 'Explicando el tema a alguien más', points: { 'auditory': 1 } },
          { text: 'Haciendo resúmenes y leyendo apuntes', points: { 'reading': 1 } },
          { text: 'Haciendo ejercicios prácticos y tests', points: { 'kinesthetic': 1 } },
        ],
      },
      {
        question: 'Si tuvieras que aprender a cocinar un plato nuevo:',
        options: [
          { text: 'Buscaría un video en YouTube', points: { 'visual': 1 } },
          { text: 'Le pediría a alguien que me explique', points: { 'auditory': 1 } },
          { text: 'Seguiría una receta escrita', points: { 'reading': 1 } },
          { text: 'Iría probando con los ingredientes', points: { 'kinesthetic': 1 } },
        ],
      },
    ],
    results: [
      { id: 'visual', title: 'Aprendiz Visual', description: 'Aprendes mejor viendo. Los diagramas, mapas mentales, videos y gráficos son tus mejores aliados. Usa colores, dibujos y organizadores visuales. Visualiza la información para recordarla mejor. YouTube y los infográficos son tus mejores herramientas.', minScore: 4, maxScore: 8 },
      { id: 'auditory', title: 'Aprendiz Auditivo', description: 'Aprendes mejor escuchando. Las explicaciones orales, debates, podcasts y grabaciones te funcionan muy bien. Lee en voz alta, explica conceptos a otros y participa en discusiones. Los audiolibros y podcasts educativos son ideales para ti.', minScore: 4, maxScore: 8 },
      { id: 'reading', title: 'Aprendiz Lector/Escritor', description: 'Aprendes mejor leyendo y escribiendo. Los textos, apuntes, resúmenes y listas son tu fuerte. Toma notas detalladas, escribe resúmenes con tus palabras. Leer libros, artículos y documentación es cómo mejor asimilas información.', minScore: 4, maxScore: 8 },
      { id: 'kinesthetic', title: 'Aprendiz Kinestésico', description: 'Aprendes mejor haciendo. Necesitas experiencia práctica y movimiento. Los ejercicios, laboratorios, juegos de rol y proyectos reales son tu mejor método. Estudia en sesiones cortas, toma descansos para moverte y busca formas de hacer tangible lo que aprendes.', minScore: 4, maxScore: 8 },
    ],
  },
]

export function getQuizBySlug(slug: string): Quiz | undefined {
  return quizzes.find(q => q.slug === slug)
}

export function getQuizzesByCategory(category: string): Quiz[] {
  return quizzes.filter(q => q.category === category)
}
