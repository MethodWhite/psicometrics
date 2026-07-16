import json
import math
from pathlib import Path


_DATA_DIR = Path(__file__).resolve().parent.parent / "data"

with open(_DATA_DIR / "big_five_questions.json", encoding="utf-8") as _f:
    _DATA = json.load(_f)

_FACTOR_NAMES = _DATA["factors"]
_FACET_NAMES = _DATA["facets"]
_QUESTIONS = [q for q in _DATA["questions"] if not q.get("attention_check")]
_SCALE_MIN = _DATA["scale"]["min"]
_SCALE_MAX = _DATA["scale"]["max"]

# Build facet -> factor mapping
_FACET_FACTOR: dict[str, str] = {}
for facet_key, info in _FACET_NAMES.items():
    _FACET_FACTOR[facet_key] = info["factor"]

# Group questions by facet and collect reverse flags
_FACET_ITEMS: dict[str, list[dict]] = {}
for q in _QUESTIONS:
    _FACET_ITEMS.setdefault(q["facet"], []).append(q)

# All facets grouped by factor
_FACTOR_FACETS: dict[str, list[str]] = {}
for facet_key, factor in _FACET_FACTOR.items():
    _FACTOR_FACETS.setdefault(factor, []).append(facet_key)

_FACTOR_KEYS = ["O", "C", "E", "A", "N"]

# IPIP-NEO facet ordering per factor
_FACET_ORDER = {
    "O": ["O1", "O2", "O3", "O4", "O5", "O6"],
    "C": ["C1", "C2", "C3", "C4", "C5", "C6"],
    "E": ["E1", "E2", "E3", "E4", "E5", "E6"],
    "A": ["A1", "A2", "A3", "A4", "A5", "A6"],
    "N": ["N1", "N2", "N3", "N4", "N5", "N6"],
}


def _normalize(raw_avg: float) -> float:
    """Convert from [SCALE_MIN, SCALE_MAX] Likert scale to 0-100."""
    return round(((raw_avg - _SCALE_MIN) / (_SCALE_MAX - _SCALE_MIN)) * 100, 1)


def _percentile(score: float) -> float:
    """Approximate percentile using normal CDF (erf).
    Assumes scores are roughly normally distributed with
    mean=50, sd=20 on the 0-100 scale.
    """
    z = (score - 50.0) / 20.0
    cdf = 0.5 * (1.0 + math.erf(z / math.sqrt(2.0)))
    return round(cdf * 100, 1)


def _reverse(raw: float) -> float:
    """Reverse a Likert-scale value: for scale 1-5, 6 - raw."""
    return float(_SCALE_MAX + _SCALE_MIN - raw)


def score_big_five(responses: dict[int | str, int], lang: str = "es") -> dict:
    valid_ids = {q["id"] for q in _QUESTIONS}

    # 1. Compute facet-level raw averages
    facet_raw_avgs: dict[str, float] = {}
    for facet_key in _FACET_ITEMS:
        items = _FACET_ITEMS[facet_key]
        vals: list[float] = []
        for q in items:
            qid = q["id"]
            str_qid = str(qid)
            if str_qid in responses:
                raw_val = int(responses[str_qid])
            elif qid in responses:
                raw_val = int(responses[qid])
            else:
                continue
            clamped = max(_SCALE_MIN, min(_SCALE_MAX, raw_val))
            if q.get("reverse", False):
                vals.append(_reverse(float(clamped)))
            else:
                vals.append(float(clamped))
        if vals:
            facet_raw_avgs[facet_key] = sum(vals) / len(vals)
        else:
            facet_raw_avgs[facet_key] = float(_SCALE_MIN)

    # 2. Compute factor scores (average of facet averages, then normalize)
    factor_scores: dict[str, float] = {}
    factor_raw_avgs: dict[str, list[float]] = {f: [] for f in _FACTOR_KEYS}
    for facet_key, raw_avg in facet_raw_avgs.items():
        factor = _FACET_FACTOR.get(facet_key)
        if factor:
            factor_raw_avgs[factor].append(raw_avg)

    for f in _FACTOR_KEYS:
        vals = factor_raw_avgs[f]
        if vals:
            factor_avg = sum(vals) / len(vals)
        else:
            factor_avg = float(_SCALE_MIN)
        factor_scores[f] = _normalize(factor_avg)

    # 3. Compute facet scores (normalized)
    facet_scores: dict[str, float] = {}
    for facet_key, raw_avg in facet_raw_avgs.items():
        facet_scores[facet_key] = _normalize(raw_avg)

    # 4. Percentiles
    percentiles: dict[str, float] = {}
    for f in _FACTOR_KEYS:
        percentiles[f] = _percentile(factor_scores[f])

    # 5. Labeled scores (bilingual)
    labeled_scores: dict[str, float] = {}
    for f in _FACTOR_KEYS:
        name = _FACTOR_NAMES[f].get(lang, _FACTOR_NAMES[f]["en"])
        labeled_scores[name] = factor_scores[f]

    labeled_facets: dict[str, float] = {}
    for facet_key in sorted(_FACET_ITEMS.keys()):
        name = _FACET_NAMES[facet_key].get(lang, _FACET_NAMES[facet_key]["en"])
        if facet_key in facet_scores:
            labeled_facets[name] = facet_scores[facet_key]

    # 6. Profile summary (bilingual)
    def _level(s: float) -> str:
        if s < 30:
            return "bajo" if lang == "es" else "low"
        if s < 45:
            return "moderado-bajo" if lang == "es" else "moderately low"
        if s < 55:
            return "moderado" if lang == "es" else "moderate"
        if s < 70:
            return "moderado-alto" if lang == "es" else "moderately high"
        return "alto" if lang == "es" else "high"

    factor_descriptions = {
        "O": {
            "es": "Apertura: imaginación, creatividad, curiosidad intelectual, aprecio por el arte y la variedad.",
            "en": "Openness: imagination, creativity, intellectual curiosity, appreciation for art and variety.",
        },
        "C": {
            "es": "Responsabilidad: organización, productividad, responsabilidad, autodisciplina y orientación al logro.",
            "en": "Conscientiousness: organization, productivity, responsibility, self-discipline, achievement striving.",
        },
        "E": {
            "es": "Extraversión: sociabilidad, energía, asertividad, búsqueda de emociones y emocionalidad positiva.",
            "en": "Extraversion: sociability, energy, assertiveness, excitement-seeking, positive emotionality.",
        },
        "A": {
            "es": "Amabilidad: cooperación, confianza, altruismo, modestia y compasión hacia los demás.",
            "en": "Agreeableness: cooperation, trust, altruism, modesty, and compassion toward others.",
        },
        "N": {
            "es": "Neuroticismo: tendencia a experimentar emociones negativas como ansiedad, tristeza e irritabilidad.",
            "en": "Neuroticism: tendency to experience negative emotions such as anxiety, sadness, and irritability.",
        },
    }

    summary_parts = []
    for f in _FACTOR_KEYS:
        s = factor_scores[f]
        name = _FACTOR_NAMES[f].get(lang, _FACTOR_NAMES[f]["en"])
        level_str = _level(s)
        desc = factor_descriptions[f].get(lang, factor_descriptions[f]["en"])
        summary_parts.append(f"{name}: {s}/100 ({level_str}). {desc}")

    profile_summary = "\n".join(summary_parts)

    # 7. Interpretation
    interp_parts = []
    for f in _FACTOR_KEYS:
        s = factor_scores[f]
        name = _FACTOR_NAMES[f].get(lang, _FACTOR_NAMES[f]["en"])
        if s >= 65:
            interp = (
                f"Puntaje alto en {name}. " + {
                    "O": "Eres curioso, creativo y abierto a nuevas experiencias.",
                    "C": "Eres organizado, responsable y confiable.",
                    "E": "Eres sociable, enérgico y disfrutas la interacción social.",
                    "A": "Eres cooperativo, compasivo y valoras la armonía.",
                    "N": "Eres sensible al estrés y experimentas emociones negativas con frecuencia.",
                }[f]
                if lang == "es"
                else (
                    f"High score in {name}. " + {
                        "O": "You are curious, creative, and open to new experiences.",
                        "C": "You are organized, responsible, and reliable.",
                        "E": "You are sociable, energetic, and enjoy social interaction.",
                        "A": "You are cooperative, compassionate, and value harmony.",
                        "N": "You are sensitive to stress and experience negative emotions frequently.",
                    }[f]
                )
            )
        elif s <= 35:
            interp = (
                f"Puntaje bajo en {name}. " + {
                    "O": "Prefieres lo familiar y lo tradicional. Eres práctico y realista.",
                    "C": "Eres flexible, espontáneo y prefieres no planificar demasiado.",
                    "E": "Eres reservado, independiente y valoras tu tiempo a solas.",
                    "A": "Eres competitivo, directo y priorizas tus intereses.",
                    "N": "Eres emocionalmente estable, calmado y resiliente.",
                }[f]
                if lang == "es"
                else (
                    f"Low score in {name}. " + {
                        "O": "You prefer the familiar and traditional. You are practical and realistic.",
                        "C": "You are flexible, spontaneous, and prefer not to over-plan.",
                        "E": "You are reserved, independent, and value your alone time.",
                        "A": "You are competitive, direct, and prioritize your own interests.",
                        "N": "You are emotionally stable, calm, and resilient.",
                    }[f]
                )
            )
        else:
            interp = (
                f"Puntaje moderado en {name}. Muestras un equilibrio en esta dimensión."
                if lang == "es"
                else f"Moderate score in {name}. You show balance in this dimension."
            )
        interp_parts.append(interp)

    interpretation = "\n".join(interp_parts)

    # 8. Recommendations
    recommendations = {
        "es": [
            "Utiliza tu perfil para identificar entornos laborales y relaciones que complementen tus rasgos naturales.",
            "Las facetas con puntuaciones muy altas o muy bajas pueden ser áreas clave para el desarrollo personal.",
            "Recuerda que no hay perfiles 'buenos' o 'malos' — cada combinación tiene fortalezas únicas.",
            "Considera cómo tus rasgos influyen en tu comunicación, toma de decisiones y manejo del estrés.",
        ],
        "en": [
            "Use your profile to identify work environments and relationships that complement your natural traits.",
            "Facets with very high or very low scores may be key areas for personal development.",
            "Remember there are no 'good' or 'bad' profiles — each combination has unique strengths.",
            "Consider how your traits influence your communication, decision-making, and stress management.",
        ],
    }

    # 9. Career recommendations
    career_recs: dict[str, list[str]] = {
        "es": [],
        "en": [],
    }
    o, c, e, a, n = factor_scores["O"], factor_scores["C"], factor_scores["E"], factor_scores["A"], factor_scores["N"]

    if lang == "es":
        if o >= 55:
            career_recs["es"].append("Carreras creativas y que requieren innovación (arte, investigación, tecnología)")
        if c >= 55:
            career_recs["es"].append("Roles estructurados con responsabilidad (administración, finanzas, derecho)")
        if e >= 55:
            career_recs["es"].append("Roles que implican interacción social (ventas, marketing, educación)")
        if a >= 55:
            career_recs["es"].append("Profesiones de ayuda y cooperación (salud, trabajo social, RRHH)")
        if n <= 45:
            career_recs["es"].append("Roles de alta presión que requieren estabilidad emocional (emergencias, liderazgo)")
        if not career_recs["es"]:
            career_recs["es"].append("Explora carreras que se alineen con tus facetas más destacadas")
    else:
        if o >= 55:
            career_recs["en"].append("Creative and innovation-driven careers (art, research, technology)")
        if c >= 55:
            career_recs["en"].append("Structured roles with responsibility (administration, finance, law)")
        if e >= 55:
            career_recs["en"].append("Roles involving social interaction (sales, marketing, education)")
        if a >= 55:
            career_recs["en"].append("Helping and cooperative professions (healthcare, social work, HR)")
        if n <= 45:
            career_recs["en"].append("High-pressure roles requiring emotional stability (emergencies, leadership)")
        if not career_recs["en"]:
            career_recs["en"].append("Explore careers that align with your most prominent facets")

    # 10. Growth areas
    growth_areas: list[str] = []
    for f in _FACTOR_KEYS:
        s = factor_scores[f]
        if lang == "es":
            if f == "N" and s >= 60:
                growth_areas.append("Desarrollar técnicas de regulación emocional y manejo del estrés")
            if f == "O" and s <= 35:
                growth_areas.append("Practicar la apertura a nuevas ideas y experiencias")
            if f == "C" and s <= 35:
                growth_areas.append("Desarrollar hábitos de organización y planificación")
            if f == "E" and s <= 35:
                growth_areas.append("Ampliar tu red social y practicar habilidades de comunicación")
            if f == "A" and s <= 35:
                growth_areas.append("Practicar la empatía y la cooperación en situaciones cotidianas")
        else:
            if f == "N" and s >= 60:
                growth_areas.append("Develop emotional regulation and stress management techniques")
            if f == "O" and s <= 35:
                growth_areas.append("Practice openness to new ideas and experiences")
            if f == "C" and s <= 35:
                growth_areas.append("Develop organization and planning habits")
            if f == "E" and s <= 35:
                growth_areas.append("Expand your social network and practice communication skills")
            if f == "A" and s <= 35:
                growth_areas.append("Practice empathy and cooperation in daily situations")

    return {
        "scores": labeled_scores,
        "facets": labeled_facets,
        "profile_summary": profile_summary,
        "percentiles": percentiles,
        "interpretation": interpretation,
        "recommendations": recommendations[lang if lang in recommendations else "en"],
        "career_recommendations": career_recs.get(lang, career_recs["en"]),
        "growth_areas": growth_areas,
    }
