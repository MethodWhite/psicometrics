"""Interpretation utilities for all psicometrics test types.

Provides per-factor descriptions and recommendations in Spanish and English.
"""

_TEST_TYPES = [
    "big_five",
    "mbti",
    "enneagram",
    "disc",
    "dark_triad",
    "human_design",
    "love_languages",
    "attachment_style",
    "emotional_intelligence",
    "career_aptitude",
    "via_strengths",
]

# ──────────────────────────────────────────────
# Big Five / OCEAN
# ──────────────────────────────────────────────
_BIG_FIVE_INTERPRETATIONS = {
    "O": {
        "es": {
            "low": "Prefieres lo familiar y lo tradicional. Eres práctico/a, realista y te enfocas en lo concreto.",
            "moderate": "Tienes un equilibrio entre apertura a nuevas experiencias y preferencia por lo familiar.",
            "high": "Eres curioso/a, creativo/a y abierto/a a nuevas ideas y experiencias.",
        },
        "en": {
            "low": "You prefer the familiar and traditional. You are practical, realistic, and focus on the concrete.",
            "moderate": "You have a balance between openness to new experiences and preference for the familiar.",
            "high": "You are curious, creative, and open to new ideas and experiences.",
        },
    },
    "C": {
        "es": {
            "low": "Eres flexible y espontáneo/a. Prefieres no planificar demasiado y adaptarte sobre la marcha.",
            "moderate": "Muestras un equilibrio entre organización y flexibilidad en tu enfoque.",
            "high": "Eres organizado/a, responsable y confiable. Planificas y cumples con tus compromisos.",
        },
        "en": {
            "low": "You are flexible and spontaneous. You prefer not to over-plan and adapt on the go.",
            "moderate": "You show balance between organization and flexibility in your approach.",
            "high": "You are organized, responsible, and reliable. You plan and fulfill commitments.",
        },
    },
    "E": {
        "es": {
            "low": "Eres reservado/a e independiente. Valoras tu tiempo a solas y las interacciones profundas.",
            "moderate": "Tienes un equilibrio entre la vida social y el tiempo a solas.",
            "high": "Eres sociable, enérgico/a y disfrutas la interacción social y las actividades grupales.",
        },
        "en": {
            "low": "You are reserved and independent. You value alone time and deep interactions.",
            "moderate": "You have a balance between social life and alone time.",
            "high": "You are sociable, energetic, and enjoy social interaction and group activities.",
        },
    },
    "A": {
        "es": {
            "low": "Eres competitivo/a y directo/a. Priorizas tus intereses y eres honesto/a en tus opiniones.",
            "moderate": "Muestras un equilibrio entre asertividad y cooperación con los demás.",
            "high": "Eres cooperativo/a, compasivo/a y valoras la armonía en las relaciones interpersonales.",
        },
        "en": {
            "low": "You are competitive and direct. You prioritize your interests and are honest in your opinions.",
            "moderate": "You show balance between assertiveness and cooperation with others.",
            "high": "You are cooperative, compassionate, and value harmony in interpersonal relationships.",
        },
    },
    "N": {
        "es": {
            "low": "Eres emocionalmente estable, calmado/a y resiliente ante el estrés.",
            "moderate": "Tienes un equilibrio en tu respuesta emocional ante situaciones estresantes.",
            "high": "Eres sensible al estrés y experimentas emociones negativas con mayor frecuencia.",
        },
        "en": {
            "low": "You are emotionally stable, calm, and resilient in the face of stress.",
            "moderate": "You have balance in your emotional response to stressful situations.",
            "high": "You are sensitive to stress and experience negative emotions more frequently.",
        },
    },
}

_BIG_FIVE_RECOMMENDATIONS = {
    "es": {
        "career": [
            "Carreras creativas y de innovación si tienes alta Apertura (arte, investigación, tecnología).",
            "Roles estructurados si tienes alta Responsabilidad (administración, finanzas, derecho).",
            "Roles de interacción social si tienes alta Extraversión (ventas, marketing, educación).",
            "Profesiones de ayuda si tienes alta Amabilidad (salud, trabajo social, RRHH).",
            "Roles de alta presión si tienes bajo Neuroticismo (emergencias, liderazgo).",
        ],
        "growth": [
            "Practica la apertura a nuevas ideas si tu Apertura es baja.",
            "Desarrolla hábitos de organización si tu Responsabilidad es baja.",
            "Amplía tu red social si tu Extraversión es baja.",
            "Practica la empatía si tu Amabilidad es baja.",
            "Desarrolla técnicas de manejo del estrés si tu Neuroticismo es alto.",
        ],
    },
    "en": {
        "career": [
            "Creative and innovation careers if high Openness (art, research, technology).",
            "Structured roles if high Conscientiousness (administration, finance, law).",
            "Social interaction roles if high Extraversion (sales, marketing, education).",
            "Helping professions if high Agreeableness (healthcare, social work, HR).",
            "High-pressure roles if low Neuroticism (emergencies, leadership).",
        ],
        "growth": [
            "Practice openness to new ideas if your Openness is low.",
            "Develop organization habits if your Conscientiousness is low.",
            "Expand your social network if your Extraversion is low.",
            "Practice empathy if your Agreeableness is low.",
            "Develop stress management techniques if your Neuroticism is high.",
        ],
    },
}

# ──────────────────────────────────────────────
# Generic fallback
# ──────────────────────────────────────────────
_GENERIC_RECOMMENDATIONS = {
    "es": [
        "Reflexiona sobre tus resultados y cómo se alinean con tu autopercepción.",
        "Identifica áreas de fortaleza y oportunidades de desarrollo personal.",
        "Comparte tus resultados con personas de confianza para obtener retroalimentación.",
        "Considera consultar con un profesional para una interpretación más profunda.",
    ],
    "en": [
        "Reflect on your results and how they align with your self-perception.",
        "Identify areas of strength and opportunities for personal development.",
        "Share your results with trusted people for feedback.",
        "Consider consulting a professional for a deeper interpretation.",
    ],
}

# ──────────────────────────────────────────────
# Public API
# ──────────────────────────────────────────────


def get_interpretation(
    test_type: str, scores: dict, lang: str = "es"
) -> dict:
    """Return per-factor descriptions for a given test type and scores.

    Args:
        test_type: One of the supported test type keys.
        scores: Dict of factor/dimension names to numeric scores (0-100).
        lang: 'es' or 'en'.

    Returns:
        Dict mapping factor keys to interpretation strings.
    """
    if test_type == "big_five":
        result = {}
        for factor_key, score in scores.items():
            if factor_key in _BIG_FIVE_INTERPRETATIONS:
                level = (
                    "low" if score < 35 else ("moderate" if score < 55 else "high")
                )
                interp = _BIG_FIVE_INTERPRETATIONS[factor_key]
                lang_map = interp.get(lang, interp["en"])
                result[factor_key] = lang_map[level]
        return result

    return {}


def get_recommendations(
    test_type: str, scores: dict, lang: str = "es"
) -> dict:
    """Return career and growth recommendations for a given test type.

    Args:
        test_type: One of the supported test type keys.
        scores: Dict of scores.
        lang: 'es' or 'en'.

    Returns:
        Dict with 'career' and 'growth' lists.
    """
    if test_type == "big_five":
        return _BIG_FIVE_RECOMMENDATIONS.get(
            lang, _BIG_FIVE_RECOMMENDATIONS["en"]
        )

    return {
        "career": _GENERIC_RECOMMENDATIONS.get(lang, _GENERIC_RECOMMENDATIONS["en"]),
        "growth": _GENERIC_RECOMMENDATIONS.get(lang, _GENERIC_RECOMMENDATIONS["en"]),
    }
