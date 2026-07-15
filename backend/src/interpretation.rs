use std::collections::HashMap;

// ─── Public API ────────────────────────────────────────────────────

/// Returns per-factor interpretation objects keyed by factor/trait name.
/// Each value contains: level, description, daily_life, work, relationships.
pub fn get_interpretation(
    test_type: &str,
    scores: &HashMap<String, f64>,
    lang: &str,
) -> serde_json::Value {
    match test_type {
        "big_five" => big_five_interp(scores, lang),
        "dark_triad" => dark_triad_interp(scores, lang),
        "disc" => disc_interp(scores, lang),
        "enneagram" => enneagram_interp(scores, lang),
        "mbti" => mbti_interp(scores, lang),
        "human_design" => human_design_interp(scores, lang),
        _ => serde_json::json!({}),
    }
}

/// Returns per-factor recommendation arrays + top-level fields.
/// Shape: { "recommendations": { factor: [tip, ...] },
///         "career_recommendations": [str, ...],
///         "growth_areas": [str, ...] }
pub fn get_recommendations(
    test_type: &str,
    scores: &HashMap<String, f64>,
    lang: &str,
) -> serde_json::Value {
    match test_type {
        "big_five" => big_five_recommend(scores, lang),
        "dark_triad" => dark_triad_recommend(scores, lang),
        "disc" => disc_recommend(scores, lang),
        "enneagram" => enneagram_recommend(scores, lang),
        "mbti" => mbti_recommend(scores, lang),
        "human_design" => human_design_recommend(scores, lang),
        _ => serde_json::json!({}),
    }
}

// ─── Helper ───────────────────────────────────────────────────────

fn level_label(score: f64) -> &'static str {
    if score >= 60.0 {
        "high"
    } else if score >= 30.0 {
        "moderate"
    } else {
        "low"
    }
}

fn factor_obj(
    score: f64,
    lo_desc: &str, lo_daily: &str, lo_work: &str, lo_rel: &str,
    mid_desc: &str, mid_daily: &str, mid_work: &str, mid_rel: &str,
    hi_desc: &str, hi_daily: &str, hi_work: &str, hi_rel: &str,
) -> serde_json::Value {
    let (level, desc, daily, work, rel) = if score >= 60.0 {
        ("high", hi_desc, hi_daily, hi_work, hi_rel)
    } else if score >= 30.0 {
        ("moderate", mid_desc, mid_daily, mid_work, mid_rel)
    } else {
        ("low", lo_desc, lo_daily, lo_work, lo_rel)
    };
    serde_json::json!({
        "level": level,
        "description": desc,
        "daily_life": daily,
        "work": work,
        "relationships": rel,
    })
}

// ─── BIG FIVE ─────────────────────────────────────────────────────

fn big_five_interp(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let mut out = serde_json::Map::new();
    let en = lang == "en";

    // Openness
    if let Some(&s) = scores.get("O") {
        out.insert("O".into(), factor_obj(s,
            /* low */
            if en { "Prefers practical, concrete tasks over abstract ideas." } else { "Prefiere tareas prácticas y concretas sobre ideas abstractas." },
            if en { "Sticks to familiar routines and traditions." } else { "Se apega a rutinas y tradiciones conocidas." },
            if en { "Excels in structured environments with clear procedures." } else { "Destaca en entornos estructurados con procedimientos claros." },
            if en { "Enjoys predictable, stable relationships." } else { "Disfruta relaciones predecibles y estables." },
            /* moderate */
            if en { "Shows a balanced blend of pragmatism and curiosity." } else { "Muestra una combinación equilibrada de pragmatismo y curiosidad." },
            if en { "Enjoys both routine and occasional novelty." } else { "Disfruta tanto la rutina como la novedad ocasional." },
            if en { "Adaptable to both structured and creative work settings." } else { "Se adapta a entornos laborales tanto estructurados como creativos." },
            if en { "Open to new experiences in relationships while valuing consistency." } else { "Abierto a nuevas experiencias en relaciones valorando la consistencia." },
            /* high */
            if en { "Highly curious, imaginative, and intellectually explorative." } else { "Altamente curioso/a, imaginativo/a y explorador/a intelectual." },
            if en { "Seeks novelty, enjoys travel, art, and abstract thinking." } else { "Busca novedad, disfruta viajes, arte y pensamiento abstracto." },
            if en { "Thrives in creative, innovative, and non-routine roles." } else { "Prospera en roles creativos, innovadores y no rutinarios." },
            if en { "Values intellectual connection and shared exploration in relationships." } else { "Valora la conexión intelectual y la exploración compartida en relaciones." },
        ));
    }

    // Conscientiousness
    if let Some(&s) = scores.get("C") {
        out.insert("C".into(), factor_obj(s,
            if en { "Spontaneous, flexible, and prefers keeping options open." } else { "Espontáneo/a, flexible, prefiere mantener opciones abiertas." },
            if en { "May struggle with strict schedules and detailed planning." } else { "Puede tener dificultades con horarios estrictos y planificación detallada." },
            if en { "Adaptable to changing priorities but may miss deadlines." } else { "Se adapta a prioridades cambiantes pero puede no cumplir plazos." },
            if en { "Easygoing and forgiving in personal connections." } else { "Relajado/a y tolerante en conexiones personales." },
            if en { "Shows moderate organization and dependability." } else { "Muestra organización y confiabilidad moderadas." },
            if en { "Balances structure with flexibility in daily routines." } else { "Equilibra estructura y flexibilidad en rutinas diarias." },
            if en { "Reliable on most tasks; can manage both planning and spontaneity." } else { "Con fiable en la mayoría de tareas; maneja planificación y espontaneidad." },
            if en { "Generally dependable while allowing space for improvisation." } else { "Generalmente confiable permitiendo espacio para la improvisación." },
            if en { "Highly organized, disciplined, and goal-oriented." } else { "Altamente organizado/a, disciplinado/a y orientado/a a metas." },
            if en { "Plans ahead, keeps commitments, and maintains order." } else { "Planifica con anticipación, cumple compromisos y mantiene orden." },
            if en { "Excels in roles requiring precision, planning, and reliability." } else { "Destaca en roles que requieren precisión, planificación y confiabilidad." },
            if en { "Reliable partner who follows through on promises." } else { "Pareja confiable que cumple lo prometido." },
        ));
    }

    // Extraversion
    if let Some(&s) = scores.get("E") {
        out.insert("E".into(), factor_obj(s,
            if en { "Introverted — gains energy from solitude and deep reflection." } else { "Introvertido/a — obtiene energía de la soledad y reflexión profunda." },
            if en { "Prefers quiet environments and meaningful one-on-one interactions." } else { "Prefiere entornos tranquilos e interacciones significativas uno a uno." },
            if en { "Works well independently or in small, focused groups." } else { "Trabaja bien de forma independiente o en grupos pequeños y enfocados." },
            if en { "Values deep, intimate connections over a wide social circle." } else { "Valora conexiones profundas e íntimas sobre un círculo social amplio." },
            if en { "Shows a balanced mix of sociability and need for solitude." } else { "Muestra una mezcla equilibrada de sociabilidad y necesidad de soledad." },
            if en { "Comfortable in both social and solitary settings." } else { "Cómodo/a tanto en entornos sociales como solitarios." },
            if en { "Adaptable to teamwork and independent work alike." } else { "Se adapta tanto al trabajo en equipo como al independiente." },
            if en { "Enjoys social activities but also values personal space." } else { "Disfruta actividades sociales pero también valora el espacio personal." },
            if en { "Outgoing, energetic, and thrives in social situations." } else { "Extrovertido/a, energético/a, prospera en situaciones sociales." },
            if en { "Seeks social stimulation and enjoys group activities." } else { "Busca estimulación social y disfruta actividades grupales." },
            if en { "Excels in roles involving teamwork, networking, and public contact." } else { "Destaca en roles que implican trabajo en equipo, networking y contacto público." },
            if en { "Warm and expressive; builds broad social networks easily." } else { "Cálido/a y expresivo/a; construye redes sociales amplias fácilmente." },
        ));
    }

    // Agreeableness
    if let Some(&s) = scores.get("A") {
        out.insert("A".into(), factor_obj(s,
            if en { "Competitive, direct, and prioritizes logic over harmony." } else { "Competitivo/a, directo/a, prioriza la lógica sobre la armonía." },
            if en { "Comfortable with debate and standing firm on opinions." } else { "Cómodo/a con el debate y manteniéndose firme en sus opiniones." },
            if en { "Effective in negotiations, critical analysis, and leadership roles." } else { "Efectivo/a en negociaciones, análisis crítico y roles de liderazgo." },
            if en { "Values honesty and authenticity; may come across as blunt." } else { "Valora la honestidad y autenticidad; puede parecer directo/a." },
            if en { "Generally cooperative while maintaining healthy assertiveness." } else { "Generalmente cooperativo/a manteniendo una asertividad saludable." },
            if en { "Gets along with most people while setting clear boundaries." } else { "Se lleva bien con la mayoría mientras establece límites claros." },
            if en { "Collaborates well but can push back when necessary." } else { "Colabora bien pero puede oponerse cuando es necesario." },
            if en { "Balances empathy with self-advocacy in personal bonds." } else { "Equilibra empatía con defensa personal en vínculos." },
            if en { "Cooperative, empathetic, and values social harmony." } else { "Cooperativo/a, empático/a, valora la armonía social." },
            if en { "Avoids conflict and goes out of the way to help others." } else { "Evita conflictos y se esfuerza por ayudar a otros." },
            if en { "Excellent in team-oriented, supportive, and people-focused roles." } else { "Excelente en roles orientados al equipo, apoyo y enfoque en personas." },
            if en { "Nurturing, considerate, and deeply attuned to others' feelings." } else { "Cariñoso/a, considerado/a y muy atento/a a los sentimientos ajenos." },
        ));
    }

    // Neuroticism
    if let Some(&s) = scores.get("N") {
        out.insert("N".into(), factor_obj(s,
            if en { "Emotionally stable, calm, and resilient under pressure." } else { "Emocionalmente estable, calmado/a y resiliente bajo presión." },
            if en { "Rarely experiences negative emotions; handles stress effectively." } else { "Rara vez experimenta emociones negativas; maneja el estrés efectivamente." },
            if en { "Thrives in high-pressure, fast-paced environments." } else { "Prospera en entornos de alta presión y ritmo rápido." },
            if en { "Provides a calming, stabilizing presence in relationships." } else { "Provee una presencia calmada y estabilizadora en relaciones." },
            if en { "Moderate emotional reactivity; handles daily ups and downs well." } else { "Reactividad emocional moderada; maneja bien altibajos diarios." },
            if en { "Occasionally worries but generally recovers quickly." } else { "Ocasionalmente se preocupa pero generalmente se recupera rápido." },
            if en { "Functions well in most environments; stress may occasionally affect performance." } else { "Funciona bien en la mayoría de entornos; el estrés puede afectar ocasionalmente." },
            if en { "Responds to relationship conflicts thoughtfully." } else { "Responde a conflictos de relación de manera reflexiva." },
            if en { "Experiences intense emotions, prone to worry and stress sensitivity." } else { "Experimenta emociones intensas, propenso/a a preocupación y sensibilidad al estrés." },
            if en { "May overthink situations and struggle with anxiety." } else { "Puede sobreanalizar situaciones y tener dificultades con la ansiedad." },
            if en { "Benefits from supportive, low-stress work environments." } else { "Se beneficia de entornos laborales de bajo estrés y apoyo." },
            if en { "Deeply attuned to emotional dynamics; needs reassurance and security." } else { "Muy atento/a a dinámicas emocionales; necesita seguridad y apoyo." },
        ));
    }

    serde_json::Value::Object(out)
}

fn big_five_recommend(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let en = lang == "en";
    let mut recs = serde_json::Map::new();

    let tips = |s: f64, lo: &[&str], mid: &[&str], hi: &[&str]| -> Vec<serde_json::Value> {
        let src = if s >= 60.0 { hi } else if s >= 30.0 { mid } else { lo };
        src.iter().map(|t| serde_json::json!(t)).collect()
    };

    if let Some(&s) = scores.get("O") {
        recs.insert("O".into(), serde_json::Value::Array(tips(s,
            &[if en { "Try exploring one new hobby or idea each month to broaden perspectives." } else { "Intenta explorar un nuevo pasatiempo o idea cada mes para ampliar perspectivas." },
              if en { "Practice asking 'what if' questions to stretch your imagination." } else { "Practica hacer preguntas de 'qué pasaría si' para estirar tu imaginación." },
              if en { "Read outside your usual topics to invite creative thinking." } else { "Lee fuera de tus temas habituales para invitar al pensamiento creativo." }],
            &[if en { "Continue balancing practical tasks with creative exploration." } else { "Continúa equilibrando tareas prácticas con exploración creativa." },
              if en { "Share your curiosity with others to enrich your social connections." } else { "Comparte tu curiosidad con otros para enriquecer conexiones sociales." }],
            &[if en { "Channel your creativity into concrete projects that produce tangible results." } else { "Canaliza tu creatividad en proyectos concretos que produzcan resultados tangibles." },
              if en { "Be mindful that others may prefer routine; respect different paces." } else { "Ten en cuenta que otros pueden preferir rutina; respeta diferentes ritmos." },
              if en { "Ground big ideas with practical steps to avoid overwhelm." } else { "Fundamenta grandes ideas con pasos prácticos para evitar abrumarte." }],
        )));
    }

    if let Some(&s) = scores.get("C") {
        recs.insert("C".into(), serde_json::Value::Array(tips(s,
            &[if en { "Use a simple planner or digital tool to track important deadlines." } else { "Usa una agenda simple o herramienta digital para seguir fechas importantes." },
              if en { "Break larger tasks into smaller, manageable steps." } else { "Divide tareas grandes en pasos más pequeños y manejables." },
              if en { "Set one daily priority to build consistency gradually." } else { "Establece una prioridad diaria para construir consistencia gradualmente." }],
            &[if en { "Keep using organizational systems that work for you." } else { "Sigue usando sistemas organizativos que funcionen para ti." },
              if en { "Allow yourself flexibility when plans change unexpectedly." } else { "Permítete flexibilidad cuando los planes cambien inesperadamente." }],
            &[if en { "Avoid over-scheduling; leave buffer time between commitments." } else { "Evita sobre-programar; deja tiempo de margen entre compromisos." },
              if en { "Practice flexibility when others don't meet your standards." } else { "Practica flexibilidad cuando otros no cumplan tus estándares." },
              if en { "Remember that rest and spontaneity are also productive." } else { "Recuerda que el descanso y la espontaneidad también son productivos." }],
        )));
    }

    if let Some(&s) = scores.get("E") {
        recs.insert("E".into(), serde_json::Value::Array(tips(s,
            &[if en { "Schedule regular alone time to recharge without guilt." } else { "Programa tiempo a solas regularmente para recargar sin culpa." },
              if en { "Push yourself to speak up in meetings at least once." } else { "Empújate a hablar en reuniones al menos una vez." },
              if en { "Build deeper connections with a few close friends rather than many acquaintances." } else { "Construye conexiones profundas con pocos amigos cercanos." }],
            &[if en { "Honor your need for both social time and solitude." } else { "Honra tu necesidad de tiempo social y soledad." },
              if en { "Use your adaptability to connect with different personality types." } else { "Usa tu adaptabilidad para conectar con diferentes tipos de personalidad." }],
            &[if en { "Practice active listening to balance your natural expressiveness." } else { "Practica escucha activa para equilibrar tu expresividad natural." },
              if en { "Respect that others may need quiet time to think." } else { "Respeta que otros pueden necesitar tiempo tranquilo para pensar." },
              if en { "Channel your social energy toward meaningful causes." } else { "Canaliza tu energía social hacia causas significativas." }],
        )));
    }

    if let Some(&s) = scores.get("A") {
        recs.insert("A".into(), serde_json::Value::Array(tips(s,
            &[if en { "Practice active empathy by asking how others feel before offering solutions." } else { "Practica empatía activa preguntando cómo se sienten otros antes de ofrecer soluciones." },
              if en { "Notice when competitiveness harms relationships versus when it helps." } else { "Nota cuándo la competitividad daña relaciones versus cuándo ayuda." },
              if en { "Collaborate on small projects to build cooperative habits." } else { "Colabora en proyectos pequeños para construir hábitos cooperativos." }],
            &[if en { "Continue balancing assertiveness with cooperation." } else { "Continúa equilibrando asertividad con cooperación." },
              if en { "Stand your ground on important issues while staying open to compromise." } else { "Mantente firme en temas importantes mientras permaneces abierto/a al compromiso." }],
            &[if en { "Set healthy boundaries to avoid being taken advantage of." } else { "Establece límites saludables para evitar que se aprovechen de ti." },
              if en { "Practice giving constructive feedback even when it might cause tension." } else { "Practica dar retroalimentación constructiva incluso si puede causar tensión." },
              if en { "Remember that saying 'no' is sometimes the kindest choice." } else { "Recuerda que decir 'no' a veces es la opción más amable." }],
        )));
    }

    if let Some(&s) = scores.get("N") {
        recs.insert("N".into(), serde_json::Value::Array(tips(s,
            &[if en { "Use your emotional stability to support others during crises." } else { "Usa tu estabilidad emocional para apoyar a otros durante crisis." },
              if en { "Check in with your own feelings periodically — stability can sometimes mask suppressed emotions." } else { "Revisa tus propios sentimientos periódicamente — la estabilidad a veces puede ocultar emociones suprimidas." }],
            &[if en { "Maintain healthy coping strategies like exercise or journaling." } else { "Mantén estrategias de afrontamiento saludables como ejercicio o escribir." },
              if en { "Build a support network for times when stress increases." } else { "Construye una red de apoyo para momentos de mayor estrés." }],
            &[if en { "Practice mindfulness and grounding techniques to manage anxiety." } else { "Practica atención plena y técnicas de conexión a tierra para manejar la ansiedad." },
              if en { "Seek environments and relationships that provide emotional safety." } else { "Busca entornos y relaciones que brinden seguridad emocional." },
              if en { "Challenge negative thought patterns with evidence-based reframing." } else { "Desafía patrones de pensamiento negativos con reencuadre basado en evidencia." },
              if en { "Consider therapy or coaching to build emotional resilience tools." } else { "Considera terapia o coaching para construir herramientas de resiliencia emocional." }],
        )));
    }

    let (career, growth): (&[&str], &[&str]) = if en {
        (&[
            "Research & academia — leverage your intellectual curiosity",
            "Creative arts & design — channel your imagination",
            "Project management — use your organizational strengths",
            "Healthcare & social services — apply your empathy",
            "Entrepreneurship — combine multiple strengths",
            "Technology & engineering — analytical problem-solving",
        ][..], &[
            "Self-awareness: understand how your traits shape your interactions",
            "Adaptability: flex your style to different situations",
            "Emotional regulation: build resilience for challenges",
            "Communication: express needs clearly and listen actively",
            "Growth mindset: treat feedback as a learning opportunity",
        ][..])
    } else {
        (&[
            "Investigación y academia — aprovecha tu curiosidad intelectual",
            "Artes creativas y diseño — canaliza tu imaginación",
            "Gestión de proyectos — usa tus fortalezas organizativas",
            "Salud y servicios sociales — aplica tu empatía",
            "Emprendimiento — combina múltiples fortalezas",
            "Tecnología e ingeniería — resolución analítica de problemas",
        ][..], &[
            "Autoconocimiento: entiende cómo tus rasgos moldean tus interacciones",
            "Adaptabilidad: flexibiliza tu estilo ante diferentes situaciones",
            "Regulación emocional: construye resiliencia para desafíos",
            "Comunicación: expresa necesidades claramente y escucha activamente",
            "Mentalidad de crecimiento: trata la retroalimentación como oportunidad de aprendizaje",
        ][..])
    };

    serde_json::json!({
        "recommendations": recs,
        "career_recommendations": career.iter().map(|c| serde_json::json!(c)).collect::<Vec<_>>(),
        "growth_areas": growth.iter().map(|g| serde_json::json!(g)).collect::<Vec<_>>(),
    })
}

// ─── DARK TRIAD ───────────────────────────────────────────────────

fn dark_triad_interp(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let mut out = serde_json::Map::new();
    let en = lang == "en";

    if let Some(&s) = scores.get("machiavellianism") {
        out.insert("machiavellianism".into(), factor_obj(s,
            if en { "Low strategic thinking — prefers straightforward, honest interactions." } else { "Bajo pensamiento estratégico — prefiere interacciones directas y honestas." },
            if en { "Approaches social situations with transparency and trust." } else { "Enfrenta situaciones sociales con transparencia y confianza." },
            if en { "May be at a disadvantage in highly competitive political environments." } else { "Puede estar en desventaja en entornos políticos altamente competitivos." },
            if en { "Builds trust through authenticity and consistency." } else { "Construye confianza mediante autenticidad y consistencia." },
            if en { "Moderate strategic thinking; can navigate social dynamics when needed." } else { "Pensamiento estratégico moderado; puede navegar dinámicas sociales cuando es necesario." },
            if en { "Balances pragmatism with genuine connection." } else { "Equilibra pragmatismo con conexión genuina." },
            if en { "Functions well in most organizational settings." } else { "Funciona bien en la mayoría de entornos organizacionales." },
            if en { "Generally authentic while aware of social dynamics." } else { "Generalmente auténtico/a mientras es consciente de dinámicas sociales." },
            if en { "Highly strategic and skilled at reading social situations." } else { "Altamente estratégico/a y hábil para leer situaciones sociales." },
            if en { "May calculate social interactions to achieve desired outcomes." } else { "Puede calcular interacciones sociales para lograr resultados deseados." },
            if en { "Excels in negotiation, politics, and competitive environments." } else { "Destaca en negociación, política y entornos competitivos." },
            if en { "Risk of manipulative patterns that may erode trust over time." } else { "Riesgo de patrones manipuladores que pueden erosionar la confianza." },
        ));
    }

    if let Some(&s) = scores.get("narcissism") {
        out.insert("narcissism".into(), factor_obj(s,
            if en { "Humble and grounded; does not seek excessive attention." } else { "Humilde y con los pies en la tierra; no busca atención excesiva." },
            if en { "Comfortable sharing credit and celebrating others' achievements." } else { "Cómodo/a compartiendo crédito y celebrando logros ajenos." },
            if en { "Works well in teams without needing the spotlight." } else { "Trabaja bien en equipos sin necesidad de ser el centro de atención." },
            if en { "Modest and considerate; values equality in relationships." } else { "Modesto/a y considerado/a; valora la igualdad en relaciones." },
            if en { "Displays healthy self-confidence without grandiosity." } else { "Muestra confianza saludable sin grandiosidad." },
            if en { "Seeks recognition appropriately without dominating social spaces." } else { "Busca reconocimiento apropiadamente sin dominar espacios sociales." },
            if en { "Can lead effectively while remaining approachable." } else { "Puede liderar efectivamente mientras permanece accesible." },
            if en { "Balances self-esteem with genuine interest in others." } else { "Equilibra autoestima con interés genuino en otros." },
            if en { "Strong sense of entitlement and need for admiration." } else { "Fuerte sentido de derecho y necesidad de admiración." },
            if en { "May dominate conversations and exaggerate achievements." } else { "Puede dominar conversaciones y exagerar logros." },
            if en { "Attracted to high-visibility roles; may struggle with teamwork." } else { "Atraído/a por roles de alta visibilidad; puede tener dificultades con trabajo en equipo." },
            if en { "May struggle with deep reciprocity; risks alienating others." } else { "Puede tener dificultades con reciprocidad profunda; riesgo de alienar a otros." },
        ));
    }

    if let Some(&s) = scores.get("psychopathy") {
        out.insert("psychopathy".into(), factor_obj(s,
            if en { "Empathic, conscientious, and emotionally attuned to others." } else { "Empático/a, concienzudo/a y sintonizado/a emocionalmente con otros." },
            if en { "Feels deeply affected by others' suffering; may need emotional boundaries." } else { "Se siente profundamente afectado/a por el sufrimiento ajeno; puede necesitar límites." },
            if en { "May find high-stakes or emotionally tough environments draining." } else { "Puede encontrar agotadores entornos de alta presión o emocionalmente duros." },
            if en { "Highly compassionate and nurturing in personal bonds." } else { "Altamente compasivo/a y cariñoso/a en vínculos personales." },
            if en { "Typical emotional responsiveness and impulse control." } else { "Capacidad de respuesta emocional y control de impulsos típicos." },
            if en { "Experiences normal range of empathy and social concern." } else { "Experimenta rango normal de empatía y preocupación social." },
            if en { "Functions well in standard work environments." } else { "Funciona bien en entornos laborales estándar." },
            if en { "Maintains balanced emotional connections." } else { "Mantiene conexiones emocionales equilibradas." },
            if en { "Low empathy, impulsivity, and reduced emotional responsiveness." } else { "Baja empatía, impulsividad y respuesta emocional reducida." },
            if en { "May engage in risky or thrill-seeking behavior." } else { "Puede involucrarse en comportamientos de riesgo o búsqueda de emociones." },
            if en { "May excel in high-risk, detached roles but struggle with team cohesion." } else { "Puede destacar en roles de alto riesgo y desapegados pero tener dificultades con cohesión de equipo." },
            if en { "Challenges forming deep emotional bonds; may appear indifferent." } else { "Desafíos para formar vínculos emocionales profundos; puede parecer indiferente." },
        ));
    }

    serde_json::Value::Object(out)
}

fn dark_triad_recommend(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let en = lang == "en";
    let mut recs = serde_json::Map::new();

    let tips = |s: f64, lo: &[&str], mid: &[&str], hi: &[&str]| -> Vec<serde_json::Value> {
        let src = if s >= 60.0 { hi } else if s >= 30.0 { mid } else { lo };
        src.iter().map(|t| serde_json::json!(t)).collect()
    };

    if let Some(&s) = scores.get("machiavellianism") {
        recs.insert("machiavellianism".into(), serde_json::Value::Array(tips(s,
            &[if en { "Maintain your authentic, straightforward approach to relationships." } else { "Mantén tu enfoque auténtico y directo en las relaciones." },
              if en { "Develop basic negotiation skills to protect your interests." } else { "Desarrolla habilidades básicas de negociación para proteger tus intereses." }],
            &[if en { "Use your strategic insight ethically to navigate complex situations." } else { "Usa tu perspicacia estratégica éticamente para navegar situaciones complejas." },
              if en { "Balance pragmatism with genuine care for others' wellbeing." } else { "Equilibra el pragmatismo con el cuidado genuino del bienestar ajeno." }],
            &[if en { "Practice transparent communication to build lasting trust." } else { "Practica comunicación transparente para construir confianza duradera." },
              if en { "Check your motives: ensure you're not exploiting others for gain." } else { "Revisa tus motivos: asegúrate de no estar explotando a otros para beneficio propio." },
              if en { "Invest in genuine relationships — they provide long-term value." } else { "Invierte en relaciones genuinas — proveen valor a largo plazo." }],
        )));
    }

    if let Some(&s) = scores.get("narcissism") {
        recs.insert("narcissism".into(), serde_json::Value::Array(tips(s,
            &[if en { "Continue practicing humility and sharing credit with others." } else { "Continúa practicando humildad y compartiendo crédito con otros." },
              if en { "Advocate for yourself when appropriate — your voice matters too." } else { "Defiéndete cuando sea apropiado — tu voz también importa." }],
            &[if en { "Cultivate genuine interest in others' perspectives and achievements." } else { "Cultiva interés genuino en las perspectivas y logros de otros." },
              if en { "Seek feedback to ensure self-perception aligns with reality." } else { "Busca retroalimentación para asegurar que tu autopercepción se alinee con la realidad." }],
            &[if en { "Practice active listening without steering conversations back to yourself." } else { "Practica escucha activa sin dirigir conversaciones hacia ti." },
              if en { "Work on empathy: imagine others' experiences and feelings." } else { "Trabaja en empatía: imagina las experiencias y sentimientos de otros." },
              if en { "Consider how your need for admiration affects your relationships." } else { "Considera cómo tu necesidad de admiración afecta tus relaciones." }],
        )));
    }

    if let Some(&s) = scores.get("psychopathy") {
        recs.insert("psychopathy".into(), serde_json::Value::Array(tips(s,
            &[if en { "Protect your emotional energy with healthy boundaries." } else { "Protege tu energía emocional con límites saludables." },
              if en { "Your empathy is a strength — use it in helping professions." } else { "Tu empatía es una fortaleza — úsala en profesiones de ayuda." }],
            &[if en { "Maintain your balanced emotional awareness in daily life." } else { "Mantén tu conciencia emocional equilibrada en la vida diaria." },
              if en { "Cultivate compassion while protecting yourself from emotional burnout." } else { "Cultiva compasión mientras te proteges del agotamiento emocional." }],
            &[if en { "Practice pausing before acting on impulses or risky decisions." } else { "Practica pausar antes de actuar por impulsos o decisiones riesgosas." },
              if en { "Develop empathy through journaling, therapy, or mindfulness practices." } else { "Desarrolla empatía mediante escritura, terapia o prácticas de atención plena." },
              if en { "Build accountability structures to stay connected with others." } else { "Construye estructuras de responsabilidad para mantenerte conectado/a con otros." }],
        )));
    }

    let (career, growth): (&[&str], &[&str]) = if en {
        (&[
            "Diplomacy & mediation — use social awareness constructively",
            "Leadership & management — channel strategic thinking ethically",
            "Helping professions — leverage empathy and compassion",
        ][..], &[
            "Authenticity: build trust through genuine connection",
            "Empathy: understand and value others' emotional experiences",
            "Self-reflection: examine your impact on those around you",
            "Impulse regulation: pause before acting on strong urges",
        ][..])
    } else {
        (&[
            "Diplomacia y mediación — usa la conciencia social constructivamente",
            "Liderazgo y gestión — canaliza pensamiento estratégico éticamente",
            "Profesiones de ayuda — aprovecha empatía y compasión",
        ][..], &[
            "Autenticidad: construye confianza mediante conexión genuina",
            "Empatía: comprende y valora las experiencias emocionales ajenas",
            "Autorreflexión: examina tu impacto en quienes te rodean",
            "Regulación de impulsos: pausa antes de actuar por impulsos fuertes",
        ][..])
    };

    serde_json::json!({
        "recommendations": recs,
        "career_recommendations": career.iter().map(|c| serde_json::json!(c)).collect::<Vec<_>>(),
        "growth_areas": growth.iter().map(|g| serde_json::json!(g)).collect::<Vec<_>>(),
    })
}

// ─── DISC ─────────────────────────────────────────────────────────

fn disc_interp(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let mut out = serde_json::Map::new();
    let en = lang == "en";

    if let Some(&s) = scores.get("D") {
        out.insert("D".into(), factor_obj(s,
            if en { "Reserved and cautious; prefers to avoid the spotlight." } else { "Reservado/a y cauteloso/a; prefiere evitar el centro de atención." },
            if en { "Takes measured, low-risk approaches to challenges." } else { "Toma enfoques mesurados y de bajo riesgo ante desafíos." },
            if en { "Thrives in supportive, non-competitive roles." } else { "Prospera en roles de apoyo, no competitivos." },
            if en { "Gentle and accommodating in personal interactions." } else { "Gentil y complaciente en interacciones personales." },
            if en { "Moderately assertive; can take charge when necessary." } else { "Moderadamente asertivo/a; puede tomar el mando cuando es necesario." },
            if en { "Balances leadership with collaboration." } else { "Equilibra liderazgo con colaboración." },
            if en { "Functions well in both team and leadership roles." } else { "Funciona bien tanto en roles de equipo como de liderazgo." },
            if en { "Assertive when needed while remaining approachable." } else { "Asertivo/a cuando es necesario mientras permanece accesible." },
            if en { "Direct, decisive, and results-oriented." } else { "Directo/a, decidido/a y orientado/a a resultados." },
            if en { "Takes charge in situations and enjoys challenges." } else { "Toma el mando en situaciones y disfruta los desafíos." },
            if en { "Excels in leadership, entrepreneurship, and high-pressure roles." } else { "Excelente en liderazgo, emprendimiento y roles de alta presión." },
            if en { "May come across as domineering; needs to practice active listening." } else { "Puede parecer dominante; necesita practicar escucha activa." },
        ));
    }

    if let Some(&s) = scores.get("I") {
        out.insert("I".into(), factor_obj(s,
            if en { "Introspective and reserved; prefers solitude over socializing." } else { "Introspectivo/a y reservado/a; prefiere soledad sobre socializar." },
            if en { "Enjoys quiet, focused activities without external stimulation." } else { "Disfruta actividades tranquilas y enfocadas sin estimulación externa." },
            if en { "Excels in independent, analytical, or technical roles." } else { "Destaca en roles independientes, analíticos o técnicos." },
            if en { "Values deep one-on-one connections over large social circles." } else { "Valora conexiones profundas uno a uno sobre círculos sociales amplios." },
            if en { "Moderately sociable; comfortable in both group and solo settings." } else { "Moderadamente sociable; cómodo/a en grupo y en solitario." },
            if en { "Enjoys social interaction but also needs time to recharge." } else { "Disfruta la interacción social pero también necesita tiempo para recargar." },
            if en { "Works well in team settings with adequate alone time." } else { "Trabaja bien en equipo con tiempo a solas adecuado." },
            if en { "Builds connections at a comfortable, sustainable pace." } else { "Construye conexiones a un ritmo cómodo y sostenible." },
            if en { "Outgoing, enthusiastic, and thrives on social interaction." } else { "Extrovertido/a, entusiasta y prospera con la interacción social." },
            if en { "Seeks social activities and energizes group settings." } else { "Busca actividades sociales y energiza entornos grupales." },
            if en { "Excels in sales, public relations, and people-oriented roles." } else { "Excelente en ventas, relaciones públicas y roles orientados a personas." },
            if en { "Warm and expressive; naturally draws people in." } else { "Cálido/a y expresivo/a; naturalmente atrae a las personas." },
        ));
    }

    if let Some(&s) = scores.get("S") {
        out.insert("S".into(), factor_obj(s,
            if en { "Independent and change-oriented; may resist routine." } else { "Independiente y orientado/a al cambio; puede resistir la rutina." },
            if en { "Seeks variety and dislikes monotonous schedules." } else { "Busca variedad y no le gustan los horarios monótonos." },
            if en { "Prefers dynamic, fast-changing environments." } else { "Prefiere entornos dinámicos y de cambio rápido." },
            if en { "Values excitement and spontaneity in relationships." } else { "Valora la emoción y espontaneidad en relaciones." },
            if en { "Generally steady while accepting reasonable change." } else { "Generalmente estable mientras acepta cambios razonables." },
            if en { "Balances need for stability with openness to change." } else { "Equilibra necesidad de estabilidad con apertura al cambio." },
            if en { "Reliable in stable roles; adapts when transitions are well-managed." } else { "Con fiable en roles estables; se adapta cuando las transiciones son bien gestionadas." },
            if en { "Loyal and consistent while flexible enough to grow." } else { "Leal y consistente mientras suficientemente flexible para crecer." },
            if en { "Patient, reliable, and values stability and consistency." } else { "Paciente, confiable y valora la estabilidad y consistencia." },
            if en { "Maintains steady routines and dislikes abrupt changes." } else { "Mantiene rutinas estables y no le gustan los cambios bruscos." },
            if en { "Excels in support roles, customer service, and administration." } else { "Excelente en roles de apoyo, servicio al cliente y administración." },
            if en { "Deeply loyal and supportive; provides a stable presence." } else { "Profundamente leal y solidario/a; provee una presencia estable." },
        ));
    }

    if let Some(&s) = scores.get("C") {
        out.insert("C".into(), factor_obj(s,
            if en { "Flexible with rules; prefers big-picture over details." } else { "Flexible con reglas; prefiere el panorama general sobre detalles." },
            if en { "Less concerned with precision and strict protocols." } else { "Menos preocupado/a por la precisión y protocolos estrictos." },
            if en { "Thrives in creative roles without rigid quality standards." } else { "Prospera en roles creativos sin estándares de calidad rígidos." },
            if en { "Relaxed and easygoing about plans and commitments." } else { "Relajado/a y tranquilo/a sobre planes y compromisos." },
            if en { "Moderately detail-oriented; balances quality with efficiency." } else { "Moderadamente orientado/a a detalles; equilibra calidad con eficiencia." },
            if en { "Follows procedures generally while knowing when to bend them." } else { "Sigue procedimientos generalmente sabiendo cuándo flexibilizarlos." },
            if en { "Produces quality work without being slowed by perfectionism." } else { "Produce trabajo de calidad sin ser ralentizado/a por perfeccionismo." },
            if en { "Dependable without being rigid." } else { "Con fiable sin ser rígido/a." },
            if en { "Precise, analytical, and detail-oriented." } else { "Preciso/a, analítico/a y orientado/a a detalles." },
            if en { "Follows rules, checks work thoroughly, values accuracy." } else { "Sigue reglas, verifica su trabajo minuciosamente, valora la precisión." },
            if en { "Excels in data analysis, accounting, quality control, and research." } else { "Excelente en análisis de datos, contabilidad, control de calidad e investigación." },
            if en { "Reliable and thorough; may need to relax expectations of others." } else { "Con fiable y minucioso/a; puede necesitar relajar expectativas sobre otros." },
        ));
    }

    serde_json::Value::Object(out)
}

fn disc_recommend(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let en = lang == "en";
    let mut recs = serde_json::Map::new();

    let tips = |s: f64, lo: &[&str], mid: &[&str], hi: &[&str]| -> Vec<serde_json::Value> {
        let src = if s >= 60.0 { hi } else if s >= 30.0 { mid } else { lo };
        src.iter().map(|t| serde_json::json!(t)).collect()
    };

    if let Some(&s) = scores.get("D") {
        recs.insert("D".into(), serde_json::Value::Array(tips(s,
            &[if en { "Build confidence by taking small leadership opportunities." } else { "Construye confianza tomando pequeñas oportunidades de liderazgo." },
              if en { "Practice expressing your opinions assertively." } else { "Practica expresar tus opiniones de forma asertiva." }],
            &[if en { "Continue developing your balanced leadership style." } else { "Continúa desarrollando tu estilo de liderazgo equilibrado." },
              if en { "Step up when your skills are needed." } else { "Da un paso al frente cuando tus habilidades sean necesarias." }],
            &[if en { "Practice patience with team members who need more time." } else { "Practica paciencia con miembros del equipo que necesitan más tiempo." },
              if en { "Solicit input before making decisions that affect others." } else { "Solicita opiniones antes de tomar decisiones que afecten a otros." },
              if en { "Tone down directness when dealing with sensitive situations." } else { "Modera la franqueza al tratar situaciones sensibles." }],
        )));
    }

    if let Some(&s) = scores.get("I") {
        recs.insert("I".into(), serde_json::Value::Array(tips(s,
            &[if en { "Step out of your comfort zone by joining one social activity." } else { "Sal de tu zona de confort uniéndote a una actividad social." },
              if en { "Share your ideas more openly in group settings." } else { "Comparte tus ideas más abiertamente en entornos grupales." }],
            &[if en { "Leverage your social comfort to build professional networks." } else { "Aprovecha tu comodidad social para construir redes profesionales." },
              if en { "Honor your need for both connection and solitude." } else { "Honra tu necesidad de conexión y soledad." }],
            &[if en { "Practice active listening — not every interaction needs your energy." } else { "Practica escucha activa — no toda interacción necesita tu energía." },
              if en { "Give others space to contribute; avoid dominating conversations." } else { "Da espacio a otros para contribuir; evita dominar conversaciones." },
              if en { "Focus your enthusiasm on projects that truly matter." } else { "Enfoca tu entusiasmo en proyectos que realmente importan." }],
        )));
    }

    if let Some(&s) = scores.get("S") {
        recs.insert("S".into(), serde_json::Value::Array(tips(s,
            &[if en { "Embrace change as an opportunity for growth." } else { "Abraza el cambio como oportunidad de crecimiento." },
              if en { "Build routines that provide anchor points amid change." } else { "Construye rutinas que provean puntos de anclaje en medio del cambio." }],
            &[if en { "Maintain your steady presence while staying open to innovation." } else { "Mantén tu presencia estable mientras te mantienes abierto/a a la innovación." },
              if en { "Help others navigate change with your calm approach." } else { "Ayuda a otros a navegar el cambio con tu enfoque calmado." }],
            &[if en { "Push yourself to adapt when innovation requires it." } else { "Empújate a adaptarte cuando la innovación lo requiera." },
              if en { "Support others through transitions with your patience." } else { "Apoya a otros durante transiciones con tu paciencia." },
              if en { "Avoid becoming complacent; seek continuous improvement." } else { "Evita volverte complaciente; busca mejora continua." }],
        )));
    }

    if let Some(&s) = scores.get("C") {
        recs.insert("C".into(), serde_json::Value::Array(tips(s,
            &[if en { "Develop basic organization habits to improve follow-through." } else { "Desarrolla hábitos básicos de organización para mejorar el seguimiento." },
              if en { "Use checklists for tasks requiring precision." } else { "Usa listas de verificación para tareas que requieren precisión." }],
            &[if en { "Continue applying your analytical skills to solve problems." } else { "Continúa aplicando tus habilidades analíticas para resolver problemas." },
              if en { "Balance depth of analysis with timely decision-making." } else { "Equilibra profundidad de análisis con toma de decisiones oportuna." }],
            &[if en { "Avoid perfectionism — know when 'good enough' is sufficient." } else { "Evita el perfeccionismo — sabe cuándo 'suficientemente bueno' es suficiente." },
              if en { "Delegate tasks that don't require your precision level." } else { "Delega tareas que no requieran tu nivel de precisión." },
              if en { "Practice making decisions with incomplete information." } else { "Practica tomar decisiones con información incompleta." }],
        )));
    }

    let (career, growth): (&[&str], &[&str]) = if en {
        (&[
            "D: Leadership, entrepreneurship, management",
            "I: Sales, public relations, entertainment, coaching",
            "S: Counseling, healthcare, customer service, education",
            "C: Accounting, engineering, IT, data analysis, research",
        ][..], &[
            "Self-awareness: identify your DISC style and its impact on others",
            "Flexibility: adapt your style to different situations and people",
            "Communication: bridge style gaps with diverse personalities",
            "Conflict resolution: use DISC insights to de-escalate tensions",
        ][..])
    } else {
        (&[
            "D: Liderazgo, emprendimiento, gestión",
            "I: Ventas, relaciones públicas, entretenimiento, coaching",
            "S: Consejería, salud, servicio al cliente, educación",
            "C: Contabilidad, ingeniería, TI, análisis de datos, investigación",
        ][..], &[
            "Autoconocimiento: identifica tu estilo DISC y su impacto en otros",
            "Flexibilidad: adapta tu estilo a diferentes situaciones y personas",
            "Comunicación: puentea brechas de estilo con personalidades diversas",
            "Resolución de conflictos: usa insights DISC para reducir tensiones",
        ][..])
    };

    serde_json::json!({
        "recommendations": recs,
        "career_recommendations": career.iter().map(|c| serde_json::json!(c)).collect::<Vec<_>>(),
        "growth_areas": growth.iter().map(|g| serde_json::json!(g)).collect::<Vec<_>>(),
    })
}

// ─── ENNEAGRAM ────────────────────────────────────────────────────

fn enneagram_interp(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let mut out = serde_json::Map::new();
    let en = lang == "en";

    for t in 1..=9 {
        let key = t.to_string();
        if let Some(&s) = scores.get(&key) {
            let (desc, daily, work, rel) = match t {
                1 => (
                    if en { "Principled, purposeful, and self-controlled." } else { "Principios, propósito y autocontrol." },
                    if en { "Strives for order and improvement in daily surroundings." } else { "Se esfuerza por el orden y la mejora en su entorno diario." },
                    if en { "Diligent, detail-oriented, and values quality and ethics." } else { "Diligente, orientado/a a detalles, valora calidad y ética." },
                    if en { "Holds self and others to high standards; may be critical." } else { "Se exige mucho a sí mismo/a y a otros; puede ser crítico/a." },
                ),
                2 => (
                    if en { "Caring, generous, and people-pleasing." } else { "Cariñoso/a, generoso/a y complaciente." },
                    if en { "Seeks to help others and be appreciated for it." } else { "Busca ayudar a otros y ser apreciado/a por ello." },
                    if en { "Excellent in supportive, service-oriented roles." } else { "Excelente en roles de apoyo y orientados al servicio." },
                    if en { "Warm and nurturing; may neglect own needs helping others." } else { "Cálido/a y cariñoso/a; puede descuidar sus propias necesidades ayudando a otros." },
                ),
                3 => (
                    if en { "Ambitious, image-conscious, and driven to succeed." } else { "Ambicioso/a, consciente de su imagen y motivado/a por el éxito." },
                    if en { "Focuses on goals, productivity, and being admired." } else { "Se enfoca en metas, productividad y ser admirado/a." },
                    if en { "Thrives in competitive, achievement-oriented environments." } else { "Prospera en entornos competitivos y orientados al logro." },
                    if en { "Charismatic but may prioritize image over authentic connection." } else { "Carismático/a pero puede priorizar la imagen sobre la conexión auténtica." },
                ),
                4 => (
                    if en { "Creative, sensitive, and introspective." } else { "Creativo/a, sensible e introspectivo/a." },
                    if en { "Seeks beauty, meaning, and authentic self-expression." } else { "Busca belleza, significado y autoexpresión auténtica." },
                    if en { "Excels in artistic, design, and self-expressive fields." } else { "Destaca en campos artísticos, de diseño y autoexpresión." },
                    if en { "Deeply emotional; may struggle with feeling misunderstood." } else { "Profundamente emocional; puede tener dificultades sintiéndose incomprendido/a." },
                ),
                5 => (
                    if en { "Analytical, perceptive, and knowledge-seeking." } else { "Analítico/a, perceptivo/a y buscador/a de conocimiento." },
                    if en { "Enjoys solitary research, learning, and observing." } else { "Disfruta investigación solitaria, aprendizaje y observación." },
                    if en { "Thrives in specialized, technical, or academic roles." } else { "Prospera en roles especializados, técnicos o académicos." },
                    if en { "Values intellectual connection; may keep emotional distance." } else { "Valora la conexión intelectual; puede mantener distancia emocional." },
                ),
                6 => (
                    if en { "Loyal, responsible, and security-oriented." } else { "Leal, responsable y orientado/a a la seguridad." },
                    if en { "Anticipates risks and prepares for worst-case scenarios." } else { "Anticipa riesgos y se prepara para escenarios adversos." },
                    if en { "Diligent and thorough; excels in risk assessment roles." } else { "Diligente y minucioso/a; destaca en roles de evaluación de riesgos." },
                    if en { "Committed and supportive but may be anxious or suspicious." } else { "Comprometido/a y solidario/a pero puede ser ansioso/a o sospechoso/a." },
                ),
                7 => (
                    if en { "Enthusiastic, spontaneous, and pleasure-seeking." } else { "Entusiasta, espontáneo/a y buscador/a de placer." },
                    if en { "Seeks new experiences, variety, and positive stimulation." } else { "Busca nuevas experiencias, variedad y estimulación positiva." },
                    if en { "Thrives in creative, dynamic, and multi-faceted roles." } else { "Prospera en roles creativos, dinámicos y multifacéticos." },
                    if en { "Fun and engaging; may avoid difficult emotions or commitment." } else { "Divertido/a y atractivo/a; puede evitar emociones difíciles o compromiso." },
                ),
                8 => (
                    if en { "Assertive, protective, and control-oriented." } else { "Asertivo/a, protector/a y orientado/a al control." },
                    if en { "Takes charge and confronts challenges directly." } else { "Toma el mando y enfrenta desafíos directamente." },
                    if en { "Natural leader; excels in management and entrepreneurship." } else { "Líder nato; destaca en gestión y emprendimiento." },
                    if en { "Intense and loyal; may struggle with vulnerability." } else { "Intenso/a y leal; puede tener dificultades con la vulnerabilidad." },
                ),
                _ => ( // 9
                    if en { "Easygoing, agreeable, and harmony-seeking." } else { "Tranquilo/a, agradable y buscador/a de armonía." },
                    if en { "Creates peaceful environments and avoids conflict." } else { "Crea entornos pacíficos y evita conflictos." },
                    if en { "Works well in collaborative, low-conflict settings." } else { "Trabaja bien en entornos colaborativos y de bajo conflicto." },
                    if en { "Warm and accepting; may neglect own priorities." } else { "Cálido/a y tolerante; puede descuidar sus propias prioridades." },
                ),
            };

            let level_s = s;
            let level = level_label(level_s);
            let (desc_mod, daily_mod, work_mod, rel_mod) = if level_s >= 60.0 {
                (format!("{} {}", if en { "Strongly expressed:" } else { "Fuertemente expresado:" }, desc),
                 format!("{}", daily),
                 format!("{}", work),
                 format!("{}", rel))
            } else if level_s >= 30.0 {
                (format!("{} {}", if en { "Moderately expressed:" } else { "Moderadamente expresado:" }, desc),
                 format!("{}", daily),
                 format!("{}", work),
                 format!("{}", rel))
            } else {
                (format!("{} {}", if en { "Minimally expressed:" } else { "Mínimamente expresado:" }, desc),
                 format!("{}", if en { "This type is less characteristic of you." } else { "Este tipo te es menos característico." }),
                 format!("{}", if en { "This type has less influence on your work style." } else { "Este tipo influye menos en tu estilo laboral." }),
                 format!("{}", if en { "This type's patterns are less dominant in your relationships." } else { "Los patrones de este tipo son menos dominantes en tus relaciones." }))
            };
            let _ = (desc_mod, daily_mod, work_mod, rel_mod);

            let alt_desc = if en { "This enneagram type is not strongly expressed." } else { "Este tipo de eneagrama no está fuertemente expresado." };
            out.insert(key, serde_json::json!({
                "level": level,
                "description": if level_s >= 30.0 { desc } else { alt_desc },
                "daily_life": daily,
                "work": work,
                "relationships": rel,
            }));
        }
    }

    serde_json::Value::Object(out)
}

fn enneagram_recommend(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let en = lang == "en";
    let mut recs = serde_json::Map::new();

    // Identify dominant type for focused recommendations
    let dominant = (1..=9).max_by(|&a, &b| {
        let sa = scores.get(&a.to_string()).copied().unwrap_or(0.0);
        let sb = scores.get(&b.to_string()).copied().unwrap_or(0.0);
        sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
    }).unwrap_or(1);

    let _dom_score = scores.get(&dominant.to_string()).copied().unwrap_or(0.0);

    for t in 1..=9 {
        let key = t.to_string();
        if let Some(&s) = scores.get(&key) {
            let tips_list: &[&str] = if s < 30.0 {
                match t {
                    1 => &[if en { "This type is not dominant — focus on your primary type's growth." } else { "Este tipo no es dominante — enfócate en el crecimiento de tu tipo principal." }],
                    2 => &[if en { "This type is not dominant — focus on your primary type's growth." } else { "Este tipo no es dominante — enfócate en el crecimiento de tu tipo principal." }],
                    3 => &[if en { "This type is not dominant — focus on your primary type's growth." } else { "Este tipo no es dominante — enfócate en el crecimiento de tu tipo principal." }],
                    4 => &[if en { "This type is not dominant — focus on your primary type's growth." } else { "Este tipo no es dominante — enfócate en el crecimiento de tu tipo principal." }],
                    5 => &[if en { "This type is not dominant — focus on your primary type's growth." } else { "Este tipo no es dominante — enfócate en el crecimiento de tu tipo principal." }],
                    6 => &[if en { "This type is not dominant — focus on your primary type's growth." } else { "Este tipo no es dominante — enfócate en el crecimiento de tu tipo principal." }],
                    7 => &[if en { "This type is not dominant — focus on your primary type's growth." } else { "Este tipo no es dominante — enfócate en el crecimiento de tu tipo principal." }],
                    8 => &[if en { "This type is not dominant — focus on your primary type's growth." } else { "Este tipo no es dominante — enfócate en el crecimiento de tu tipo principal." }],
                    _ => &[if en { "This type is not dominant — focus on your primary type's growth." } else { "Este tipo no es dominante — enfócate en el crecimiento de tu tipo principal." }],
                }
            } else {
                match t {
                    1 => &[if en { "Practice self-compassion when things aren't perfect." } else { "Practica autocompasión cuando las cosas no sean perfectas." },
                            if en { "Allow others to do tasks their own way." } else { "Permite que otros hagan tareas a su manera." }],
                    2 => &[if en { "Practice receiving help without guilt." } else { "Practica recibir ayuda sin culpa." },
                            if en { "Set boundaries to protect your own energy and needs." } else { "Establece límites para proteger tu propia energía y necesidades." }],
                    3 => &[if en { "Connect with your authentic self beyond achievements." } else { "Conecta con tu ser auténtico más allá de los logros." },
                            if en { "Value rest and being over doing and performing." } else { "Valora el descanso y el ser sobre el hacer y el rendir." }],
                    4 => &[if en { "Practice gratitude for the ordinary moments." } else { "Practica gratitud por los momentos ordinarios." },
                            if en { "Connect with others through shared experiences, not just shared melancholy." } else { "Conecta con otros mediante experiencias compartidas, no solo melancolía compartida." }],
                    5 => &[if en { "Share your knowledge generously with others." } else { "Comparte tu conocimiento generosamente con otros." },
                            if en { "Practice engaging emotionally, not just intellectually." } else { "Practica involucrarte emocionalmente, no solo intelectualmente." }],
                    6 => &[if en { "Cultivate trust in yourself and your decisions." } else { "Cultiva confianza en ti mismo/a y tus decisiones." },
                            if en { "Challenge worst-case-scenario thinking with evidence." } else { "Desafía el pensamiento de escenario catastrófico con evidencia." }],
                    7 => &[if en { "Practice staying with difficult emotions rather than escaping." } else { "Practica permanecer con emociones difíciles en lugar de escapar." },
                            if en { "Commit to one project at a time for deeper mastery." } else { "Comprométete con un proyecto a la vez para mayor dominio." }],
                    8 => &[if en { "Practice vulnerability as a strength, not a weakness." } else { "Practica la vulnerabilidad como fortaleza, no debilidad." },
                            if en { "Consider others' perspectives before asserting your own." } else { "Considera las perspectivas ajenas antes de imponer la tuya." }],
                    _ => &[if en { "Assert your own needs and opinions more directly." } else { "Expresa tus propias necesidades y opiniones más directamente." },
                            if en { "Practice initiating rather than always following." } else { "Practica iniciar en lugar de siempre seguir." }],
                }
            };
            recs.insert(key, tips_list.iter().map(|t| serde_json::json!(t)).collect::<Vec<_>>().into());
        }
    }

    let (career, growth): (&[&str], &[&str]) = if en {
        let pair = match dominant {
            1 => (&["Quality assurance, auditing, law, ethics compliance"][..], &["Self-compassion, flexibility, letting go of perfection"][..]),
            2 => (&["Counseling, healthcare, teaching, human resources"][..], &["Boundary-setting, self-care, receiving help"][..]),
            3 => (&["Sales, marketing, management, entrepreneurship"][..], &["Authenticity, vulnerability, slowing down"][..]),
            4 => (&["Creative arts, design, writing, therapy"][..], &["Gratitude, emotional regulation, embracing the ordinary"][..]),
            5 => (&["Research, engineering, data science, academia"][..], &["Emotional engagement, sharing knowledge, collaboration"][..]),
            6 => (&["Risk management, security, planning, analysis"][..], &["Self-trust, courage, challenging anxious thoughts"][..]),
            7 => (&["Innovation, entertainment, travel, entrepreneurship"][..], &["Commitment, depth, emotional presence"][..]),
            8 => (&["Leadership, entrepreneurship, law, politics"][..], &["Vulnerability, patience, shared decision-making"][..]),
            _ => (&["Mediation, counseling, human resources, diplomacy"][..], &["Assertiveness, prioritization, self-advocacy"][..]),
        };
        pair
    } else {
        let pair = match dominant {
            1 => (&["Control de calidad, auditoría, derecho, cumplimiento ético"][..], &["Autocompasión, flexibilidad, dejar el perfeccionismo"][..]),
            2 => (&["Consejería, salud, enseñanza, recursos humanos"][..], &["Establecer límites, autocuidado, recibir ayuda"][..]),
            3 => (&["Ventas, marketing, gestión, emprendimiento"][..], &["Autenticidad, vulnerabilidad, disminuir el ritmo"][..]),
            4 => (&["Artes creativas, diseño, escritura, terapia"][..], &["Gratitud, regulación emocional, abrazar lo ordinario"][..]),
            5 => (&["Investigación, ingeniería, ciencia de datos, academia"][..], &["Compromiso emocional, compartir conocimiento, colaboración"][..]),
            6 => (&["Gestión de riesgos, seguridad, planificación, análisis"][..], &["Autoconfianza, coraje, desafiar pensamientos ansiosos"][..]),
            7 => (&["Innovación, entretenimiento, viajes, emprendimiento"][..], &["Compromiso, profundidad, presencia emocional"][..]),
            8 => (&["Liderazgo, emprendimiento, derecho, política"][..], &["Vulnerabilidad, paciencia, toma de decisiones compartida"][..]),
            _ => (&["Mediación, consejería, recursos humanos, diplomacia"][..], &["Asertividad, priorización, autodefensa"][..]),
        };
        pair
    };

    serde_json::json!({
        "recommendations": recs,
        "career_recommendations": career.iter().map(|c| serde_json::json!(c)).collect::<Vec<_>>(),
        "growth_areas": growth.iter().map(|g| serde_json::json!(g)).collect::<Vec<_>>(),
    })
}

// ─── MBTI ─────────────────────────────────────────────────────────

fn mbti_interp(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let mut out = serde_json::Map::new();
    let en = lang == "en";

    // For MBTI the score is percentage toward the second pole (higher = more N, F, P, I etc)
    // We normalize: 0-30 = clear pole A, 30-60 = moderate, 60-100 = clear pole B

    if let Some(&s) = scores.get("EI") {
        // 0 = E, 100 = I
        let _e_score = 100.0 - s;
        out.insert("EI".into(), factor_obj(s,
            /* extraverted */
            if en { "Extraverted — energized by social interaction and group activities." } else { "Extrovertido/a — energizado/a por interacción social y actividades grupales." },
            if en { "Seeks company, enjoys conversations and collaborative environments." } else { "Busca compañía, disfruta conversaciones y entornos colaborativos." },
            if en { "Thrives in team settings, networking, and active roles." } else { "Prospera en entornos de equipo, networking y roles activos." },
            if en { "Expressive and sociable; builds relationships through shared activities." } else { "Expresivo/a y sociable; construye relaciones mediante actividades compartidas." },
            /* balanced */
            if en { "Balanced between extraversion and introversion." } else { "Equilibrado/a entre extroversión e introversión." },
            if en { "Comfortable in both social and solitary settings." } else { "Cómodo/a tanto en entornos sociales como solitarios." },
            if en { "Adaptable to teamwork and independent work equally." } else { "Se adapta tanto al trabajo en equipo como al independiente." },
            if en { "Flexible social style; adapts well to different personalities." } else { "Estilo social flexible; se adapta bien a diferentes personalidades." },
            /* introverted */
            if en { "Introverted — energized by solitude and internal reflection." } else { "Introvertido/a — energizado/a por soledad y reflexión interna." },
            if en { "Prefers quiet environments and meaningful one-on-one interactions." } else { "Prefiere entornos tranquilos e interacciones significativas uno a uno." },
            if en { "Excels in focused, independent roles with minimal interruptions." } else { "Destaca en roles enfocados e independientes con mínimas interrupciones." },
            if en { "Values deep connections and thoughtful communication." } else { "Valora conexiones profundas y comunicación reflexiva." },
        ));
    }

    if let Some(&s) = scores.get("SN") {
        let s_score = s; // percentage toward N
        out.insert("SN".into(), factor_obj(s_score,
            /* sensing */
            if en { "Sensing — focuses on concrete facts, details, and present reality." } else { "Sensorial — se enfoca en hechos concretos, detalles y realidad presente." },
            if en { "Pays attention to practical details and real-world information." } else { "Presta atención a detalles prácticos e información del mundo real." },
            if en { "Thrives with clear procedures, hands-on tasks, and data-driven work." } else { "Prospera con procedimientos claros, tareas prácticas y trabajo basado en datos." },
            if en { "Expresses care through practical actions and reliability." } else { "Expresa cuidado mediante acciones prácticas y confiabilidad." },
            /* balanced */
            if en { "Balances practical observation with imaginative thinking." } else { "Equilibra observación práctica con pensamiento imaginativo." },
            if en { "Notices details while also considering future possibilities." } else { "Nota detalles mientras también considera posibilidades futuras." },
            if en { "Combines data analysis with creative problem-solving." } else { "Combina análisis de datos con resolución creativa de problemas." },
            if en { "Appreciates both practical gestures and visionary ideas." } else { "Aprecia tanto gestos prácticos como ideas visionarias." },
            /* intuitive */
            if en { "Intuitive — focuses on patterns, possibilities, and future potential." } else { "Intuitivo/a — se enfoca en patrones, posibilidades y potencial futuro." },
            if en { "Thinks about the big picture and imagines future scenarios." } else { "Piensa en el panorama general e imagina escenarios futuros." },
            if en { "Excels in strategic planning, innovation, and creative fields." } else { "Destaca en planificación estratégica, innovación y campos creativos." },
            if en { "Values deep conversations about ideas and future possibilities." } else { "Valora conversaciones profundas sobre ideas y posibilidades futuras." },
        ));
    }

    if let Some(&s) = scores.get("TF") {
        let t_score = s; // percentage toward F
        out.insert("TF".into(), factor_obj(t_score,
            /* thinking */
            if en { "Thinking — makes decisions based on logic, principles, and objective analysis." } else { "Pensamiento — toma decisiones basadas en lógica, principios y análisis objetivo." },
            if en { "Approaches problems analytically and values fairness and truth." } else { "Enfrenta problemas analíticamente y valora justicia y verdad." },
            if en { "Excels in roles requiring critical analysis, strategy, and objectivity." } else { "Destaca en roles que requieren análisis crítico, estrategia y objetividad." },
            if en { "Communicates directly; values honesty over emotional comfort." } else { "Se comunica directamente; valora honestidad sobre comodidad emocional." },
            /* balanced */
            if en { "Balances logical analysis with consideration for people's feelings." } else { "Equilibra análisis lógico con consideración por los sentimientos de las personas." },
            if en { "Makes fair decisions while maintaining empathy." } else { "Toma decisiones justas mientras mantiene empatía." },
            if en { "Adapts decision-making style to the situation at hand." } else { "Adapta estilo de toma de decisiones a la situación." },
            if en { "Communicates with both clarity and sensitivity." } else { "Se comunica con claridad y sensibilidad." },
            /* feeling */
            if en { "Feeling — makes decisions based on values, empathy, and harmony." } else { "Sentimiento — toma decisiones basadas en valores, empatía y armonía." },
            if en { "Prioritizes others' wellbeing and emotional atmosphere." } else { "Prioriza el bienestar ajeno y la atmósfera emocional." },
            if en { "Thrives in people-oriented roles like counseling, teaching, and HR." } else { "Prospera en roles orientados a personas como consejería, enseñanza y RRHH." },
            if en { "Empathetic and nurturing; values harmony and emotional connection." } else { "Empático/a y cariñoso/a; valora armonía y conexión emocional." },
        ));
    }

    if let Some(&s) = scores.get("JP") {
        let j_score = s; // percentage toward P
        out.insert("JP".into(), factor_obj(j_score,
            /* judging */
            if en { "Judging — prefers structure, planning, and decisive closure." } else { "Juzgador/a — prefiere estructura, planificación y cierre decisivo." },
            if en { "Likes organized schedules and completing tasks before relaxing." } else { "Le gustan los horarios organizados y completar tareas antes de relajarse." },
            if en { "Excels in roles requiring organization, deadlines, and follow-through." } else { "Destaca en roles que requieren organización, plazos y seguimiento." },
            if en { "Reliable and committed; values punctuality and clear plans." } else { "Con fiable y comprometido/a; valora puntualidad y planes claros." },
            /* balanced */
            if en { "Balances structure with flexibility and spontaneity." } else { "Equilibra estructura con flexibilidad y espontaneidad." },
            if en { "Plans when needed but adapts easily to changes." } else { "Planifica cuando es necesario pero se adapta fácilmente a cambios." },
            if en { "Manages deadlines while staying open to new information." } else { "Maneja plazos mientras permanece abierto/a a nueva información." },
            if en { "Dependable without being rigid about plans." } else { "Con fiable sin ser rígido/a sobre los planes." },
            /* perceiving */
            if en { "Perceiving — prefers flexibility, spontaneity, and keeping options open." } else { "Perceptor/a — prefiere flexibilidad, espontaneidad y mantener opciones abiertas." },
            if en { "Adapts to changing circumstances and enjoys last-minute opportunities." } else { "Se adapta a circunstancias cambiantes y disfruta oportunidades de último momento." },
            if en { "Thrives in dynamic, ever-changing environments and creative roles." } else { "Prospera en entornos dinámicos y cambiantes y roles creativos." },
            if en { "Easygoing and spontaneous; enjoys going with the flow." } else { "Relajado/a y espontáneo/a; disfruta fluir con el momento." },
        ));
    }

    serde_json::Value::Object(out)
}

fn mbti_recommend(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let en = lang == "en";
    let mut recs = serde_json::Map::new();

    let tips = |s: f64, lo: &[&str], mid: &[&str], hi: &[&str]| -> Vec<serde_json::Value> {
        let src = if s >= 60.0 { hi } else if s >= 30.0 { mid } else { lo };
        src.iter().map(|t| serde_json::json!(t)).collect()
    };

    if let Some(&s) = scores.get("EI") {
        recs.insert("EI".into(), serde_json::Value::Array(tips(s,
            &[if en { "Push yourself to engage in one social activity per week." } else { "Empújate a participar en una actividad social por semana." },
              if en { "Share your ideas in group settings — your perspective matters." } else { "Comparte tus ideas en grupo — tu perspectiva importa." }],
            &[if en { "Continue developing both your social and solitary skills." } else { "Continúa desarrollando habilidades sociales y solitarias." },
              if en { "Use your flexibility to bridge between extraverts and introverts." } else { "Usa tu flexibilidad para conectar extrovertidos e introvertidos." }],
            &[if en { "Schedule alone time to recharge without guilt." } else { "Programa tiempo a solas para recargar sin culpa." },
              if en { "Practice expressing your thoughts verbally in meetings." } else { "Practica expresar tus pensamientos verbalmente en reuniones." }],
        )));
    }

    if let Some(&s) = scores.get("SN") {
        recs.insert("SN".into(), serde_json::Value::Array(tips(s,
            &[if en { "Practice brainstorming and imaginative thinking exercises." } else { "Practica lluvia de ideas y ejercicios de pensamiento imaginativo." },
              if en { "Read fiction or explore art to strengthen your intuitive side." } else { "Lee ficción o explora arte para fortalecer tu lado intuitivo." }],
            &[if en { "Use both data and intuition when making important decisions." } else { "Usa datos e intuición al tomar decisiones importantes." },
              if en { "Appreciate both concrete results and creative possibilities." } else { "Aprecia resultados concretos y posibilidades creativas." }],
            &[if en { "Ground your ideas in practical, actionable steps." } else { "Fundamenta tus ideas en pasos prácticos y accionables." },
              if en { "Pay attention to details — they matter for execution." } else { "Presta atención a los detalles — importan para la ejecución." }],
        )));
    }

    if let Some(&s) = scores.get("TF") {
        recs.insert("TF".into(), serde_json::Value::Array(tips(s,
            &[if en { "Consider how decisions affect people, not just outcomes." } else { "Considera cómo las decisiones afectan a las personas, no solo los resultados." },
              if en { "Practice acknowledging others' emotions before offering solutions." } else { "Practica reconocer las emociones ajenas antes de ofrecer soluciones." }],
            &[if en { "Continue balancing logic and empathy in your decisions." } else { "Continúa equilibrando lógica y empatía en tus decisiones." },
              if en { "Adapt your communication style to your audience." } else { "Adapta tu estilo de comunicación a tu audiencia." }],
            &[if en { "Practice making decisions based on principles, not just emotions." } else { "Practica tomar decisiones basadas en principios, no solo emociones." },
              if en { "Develop thicker skin for situations that require objectivity." } else { "Desarrolla una piel más gruesa para situaciones que requieren objetividad." }],
        )));
    }

    if let Some(&s) = scores.get("JP") {
        recs.insert("JP".into(), serde_json::Value::Array(tips(s,
            &[if en { "Practice spontaneity — try an unplanned activity." } else { "Practica espontaneidad — prueba una actividad no planificada." },
              if en { "Loosen your schedule occasionally to embrace the unexpected." } else { "Afloja tu horario ocasionalmente para abrazar lo inesperado." }],
            &[if en { "Maintain your planning skills while staying adaptable." } else { "Mantén tus habilidades de planificación mientras te mantienes adaptable." },
              if en { "Use your balanced style to mediate between structured and flexible team members." } else { "Usa tu estilo equilibrado para mediar entre miembros estructurados y flexibles." }],
            &[if en { "Build basic organizational systems to improve follow-through." } else { "Construye sistemas organizativos básicos para mejorar el seguimiento." },
              if en { "Set personal deadlines to ensure timely completion." } else { "Establece plazos personales para asegurar finalización oportuna." }],
        )));
    }

    let (career, growth): (&[&str], &[&str]) = if en {
        (&[
            "ENTJ/INTJ: Executive leadership, strategy, engineering",
            "ENFP/INFP: Creative arts, counseling, writing, education",
            "ISTJ/ESTJ: Management, law enforcement, accounting, administration",
            "ENFJ/INFJ: Teaching, healthcare, nonprofit leadership, coaching",
            "ISTP/ESTP: Entrepreneurship, trades, emergency services, sales",
            "ESFP/ISFP: Performing arts, design, hospitality, healthcare",
        ][..], &[
            "Self-awareness: recognize your preferences and blind spots",
            "Cognitive flexibility: develop your less-preferred functions",
            "Communication: adapt your style across different types",
            "Stress management: identify what drains vs energizes you",
        ][..])
    } else {
        (&[
            "ENTJ/INTJ: Liderazgo ejecutivo, estrategia, ingeniería",
            "ENFP/INFP: Artes creativas, consejería, escritura, educación",
            "ISTJ/ESTJ: Gestión, seguridad, contabilidad, administración",
            "ENFJ/INFJ: Enseñanza, salud, liderazgo sin fines de lucro, coaching",
            "ISTP/ESTP: Emprendimiento, oficios, servicios de emergencia, ventas",
            "ESFP/ISFP: Artes escénicas, diseño, hospitalidad, salud",
        ][..], &[
            "Autoconocimiento: reconoce tus preferencias y puntos ciegos",
            "Flexibilidad cognitiva: desarrolla tus funciones menos preferidas",
            "Comunicación: adapta tu estilo a través de diferentes tipos",
            "Manejo del estrés: identifica qué te drena vs te energiza",
        ][..])
    };

    serde_json::json!({
        "recommendations": recs,
        "career_recommendations": career.iter().map(|c| serde_json::json!(c)).collect::<Vec<_>>(),
        "growth_areas": growth.iter().map(|g| serde_json::json!(g)).collect::<Vec<_>>(),
    })
}

// ─── HUMAN DESIGN ─────────────────────────────────────────────────

fn human_design_interp(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let mut out = serde_json::Map::new();
    let en = lang == "en";

    // Human Design uses type as a string stored in scores as "type_value"
    // The actual type comes from the calculate function; scores might have a "type_key"
    // We provide a generic interpretation based on type if present
    if let Some(&_s) = scores.get("type_key") {
        // Will be filled by the scoring function with the actual type string
        // Placeholder — the scoring function for HD will add interpretation directly
    }

    // Provide interpretation even without type_key
    let type_desc = match scores.get("type_value").map(|f| *f as u32) {
        Some(1) => {
            if en {
                ("Manifestor", "Initiating energy — designed to lead and start new things.", "Independent and impact-driven.")
            } else {
                ("Manifestador", "Energía iniciadora — diseñado/a para liderar y comenzar cosas nuevas.", "Independiente e impulsado/a por el impacto.")
            }
        }
        Some(2) => {
            if en {
                ("Generator", "Sustainable energy — designed to respond and build.", "Patient and mastery-oriented.")
            } else {
                ("Generador", "Energía sostenible — diseñado/a para responder y construir.", "Paciente y orientado/a al dominio.")
            }
        }
        Some(3) => {
            if en {
                ("Manifesting Generator", "Multi-energetic — designed to respond and initiate fast.", "Fast, versatile, and multi-passionate.")
            } else {
                ("Generador Manifestador", "Multi-energético/a — diseñado/a para responder e iniciar rápido.", "Rápido/a, versátil y multi-apasionado/a.")
            }
        }
        Some(4) => {
            if en {
                ("Projector", "Guiding energy — designed to manage and direct others.", "Perceptive and people-oriented.")
            } else {
                ("Proyector", "Energía guía — diseñado/a para gestionar y dirigir a otros.", "Perceptivo/a y orientado/a a personas.")
            }
        }
        Some(5) => {
            if en {
                ("Reflector", "Reflective energy — designed to sample and reflect the whole.", "Adaptive and community-oriented.")
            } else {
                ("Reflector", "Energía reflectora — diseñado/a para muestrear y reflejar el todo.", "Adaptable y orientado/a a la comunidad.")
            }
        }
        _ => return serde_json::Value::Object(out), // No type data available
    };

    out.insert("type".into(), serde_json::json!({
        "level": "high",
        "description": type_desc.1,
        "daily_life": type_desc.2,
        "work": type_desc.2,
        "relationships": type_desc.2,
    }));

    serde_json::Value::Object(out)
}

fn human_design_recommend(scores: &HashMap<String, f64>, lang: &str) -> serde_json::Value {
    let en = lang == "en";
    let mut recs = serde_json::Map::new();

    let (type_recs, career, growth) = match scores.get("type_value").map(|f| *f as u32) {
        Some(1) => (
            vec![
                if en { "Practice informing others before you act — it builds trust." } else { "Practica informar a otros antes de actuar — construye confianza." },
                if en { "Your impact is powerful; use it to initiate positive change." } else { "Tu impacto es poderoso; úsalo para iniciar cambios positivos." },
                if en { "Give yourself space between initiations to avoid burnout." } else { "Date espacio entre iniciativas para evitar agotamiento." },
            ],
            if en { "Entrepreneurship, innovation, leadership, pioneering ventures" } else { "Emprendimiento, innovación, liderazgo, proyectos pioneros" },
            if en { "Patience, collaboration, informing others, channeling initiative" } else { "Paciencia, colaboración, informar a otros, canalizar iniciativa" },
        ),
        Some(2) => (
            vec![
                if en { "Wait to respond — let life bring opportunities to you." } else { "Espera para responder — deja que la vida te traiga oportunidades." },
                if en { "Your energy is sustainable when you're doing what you love." } else { "Tu energía es sostenible cuando haces lo que amas." },
                if en { "Say no to things that don't energize you." } else { "Di no a cosas que no te energizan." },
            ],
            if en { "Skilled trades, arts, healthcare, any mastery-based profession" } else { "Oficios calificados, artes, salud, cualquier profesión basada en maestría" },
            if en { "Patience, deep mastery, following what excites you, responding not chasing" } else { "Paciencia, dominio profundo, seguir lo que te apasiona, responder no perseguir" },
        ),
        Some(3) => (
            vec![
                if en { "Respond first, then inform — combine Generator and Manifestor strategy." } else { "Responde primero, luego informa — combina estrategia Generadora y Manifestadora." },
                if en { "You have many passions; honor them without overwhelming yourself." } else { "Tienes muchas pasiones; honrarlas sin abrumarte." },
                if en { "Channel your speed into focused sprints rather than constant motion." } else { "Canaliza tu velocidad en sprints enfocados en lugar de movimiento constante." },
            ],
            if en { "Multimedia, project management, entrepreneurship, creative direction" } else { "Multimedia, gestión de proyectos, emprendimiento, dirección creativa" },
            if en { "Focus, completion, prioritization, balancing speed with depth" } else { "Enfoque, finalización, priorización, equilibrar velocidad con profundidad" },
        ),
        Some(4) => (
            vec![
                if en { "Wait for the invitation — recognition opens the right doors." } else { "Espera la invitación — el reconocimiento abre las puertas correctas." },
                if en { "Your gift is seeing others clearly; share your insights when asked." } else { "Tu don es ver claramente a otros; comparte tus percepciones cuando te pregunten." },
                if en { "Build a network of people who recognize your genius." } else { "Construye una red de personas que reconozcan tu genio." },
            ],
            if en { "Management, coaching, consulting, HR, leadership development" } else { "Gestión, coaching, consultoría, RRHH, desarrollo de liderazgo" },
            if en { "Self-promotion, patience, waiting to be recognized, delegation" } else { "Autopromoción, paciencia, esperar a ser reconocido/a, delegación" },
        ),
        _ => (
            vec![
                if en { "Your gift is objectivity — use it wisely in group settings." } else { "Tu don es la objetividad — úsalo sabiamente en entornos grupales." },
                if en { "Wait a full lunar cycle before making major life decisions." } else { "Espera un ciclo lunar completo antes de tomar decisiones importantes." },
                if en { "Sample different environments to find where you feel most aligned." } else { "Muestrea diferentes entornos para encontrar dónde te sientes más alineado/a." },
            ],
            if en { "Community organizing, mediation, environmental work, social research" } else { "Organización comunitaria, mediación, trabajo ambiental, investigación social" },
            if en { "Decision-making consistency, grounding, finding your community, reflection" } else { "Consistencia en decisiones, conexión a tierra, encontrar tu comunidad, reflexión" },
        ),
    };

    recs.insert("type".into(), type_recs.into_iter().map(|r| serde_json::json!(r)).collect::<Vec<_>>().into());

    serde_json::json!({
        "recommendations": recs,
        "career_recommendations": vec![serde_json::json!(career)],
        "growth_areas": vec![serde_json::json!(growth)],
    })
}
