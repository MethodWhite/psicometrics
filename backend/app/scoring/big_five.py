"""Big Five / IPIP-NEO-120 Scoring Algorithm.

Based on the International Personality Item Pool (ipip.ori.org).
120 items, 12 per factor (6 facets each), Likert 1-5 scale.
Reverse-scored items are marked in the question data.
"""

from app.data_loader import load_questions


FACET_TO_FACTOR = {
    "O1": "O", "O2": "O", "O3": "O", "O4": "O", "O5": "O", "O6": "O",
    "C1": "C", "C2": "C", "C3": "C", "C4": "C", "C5": "C", "C6": "C",
    "E1": "E", "E2": "E", "E3": "E", "E4": "E", "E5": "E", "E6": "E",
    "A1": "A", "A2": "A", "A3": "A", "A4": "A", "A5": "A", "A6": "A",
    "N1": "N", "N2": "N", "N3": "N", "N4": "N", "N5": "N", "N6": "N",
}

FACTOR_NAMES = {
    "O": {"es": "Apertura a la Experiencia", "en": "Openness to Experience"},
    "C": {"es": "Responsabilidad", "en": "Conscientiousness"},
    "E": {"es": "Extraversión", "en": "Extraversion"},
    "A": {"es": "Amabilidad", "en": "Agreeableness"},
    "N": {"es": "Neuroticismo", "en": "Neuroticism"},
}


def score_big_five(answers: dict[int, int], language: str = "es") -> dict:
    """Score Big Five test from question answers.

    Args:
        answers: dict mapping question_id (1-120) to value (1-5)
        language: 'es' or 'en' for profile text

    Returns:
        dict with factor_scores (0-100), facet_scores, profile_summary, percentiles
    """
    questions = load_questions("big_five")

    # Accumulate facet scores
    facet_raw: dict[str, list[float]] = {}
    for q in questions["questions"]:
        qid = q["id"]
        if qid not in answers:
            continue

        value = answers[qid]
        facet = q["facet"]
        reverse = q.get("reverse", False)

        if reverse:
            value = 6 - value  # Reverse: 1->5, 2->4, 3->3, 4->2, 5->1

        facet_raw.setdefault(facet, []).append(float(value))

    # Calculate facet scores (mean, normalized to 0-100)
    facet_scores = {}
    for facet, values in facet_raw.items():
        if values:
            mean = sum(values) / len(values)
            # Normalize: 1-5 scale -> 0-100
            facet_scores[facet] = round((mean - 1) / 4 * 100, 1)

    # Calculate factor scores (mean of constituent facets)
    factor_scores = {}
    for factor in ["O", "C", "E", "A", "N"]:
        facets = [f for f, v in FACET_TO_FACTOR.items() if v == factor]
        values = [facet_scores[f] for f in facets if f in facet_scores]
        if values:
            factor_scores[factor] = round(sum(values) / len(values), 1)
        else:
            factor_scores[factor] = 50.0

    # Generate profile summary
    summary = _generate_big_five_summary(factor_scores, language)

    # Generate percentiles (simplified normal distribution approximation)
    percentiles = {}
    for factor, score in factor_scores.items():
        # Approximate percentile from 0-100 score
        percentiles[factor] = round(_approx_percentile(score), 1)

    return {
        "scores": factor_scores,
        "facets": facet_scores,
        "profile_summary": summary,
        "percentiles": percentiles,
    }


def _approx_percentile(score: float) -> float:
    """Approximate percentile from a 0-100 score using normal distribution."""
    import math
    # Assume mean=50, std=15 (typical for standardized personality tests)
    z = (score - 50) / 15
    # Cumulative distribution function approximation
    t = 1 / (1 + 0.2316419 * abs(z))
    d = 0.3989422804014327 * math.exp(-z * z / 2)
    p = d * t * (0.3193815 + t * (-0.3565638 + t * (1.781478 + t * (-1.821256 + t * 1.330274))))
    if z > 0:
        return (1 - p) * 100
    return p * 100


def _generate_big_five_summary(scores: dict, language: str) -> str:
    """Generate a natural language summary of Big Five scores."""
    if language == "es":
        return _summary_es(scores)
    return _summary_en(scores)


def _summary_es(scores: dict) -> str:
    parts = []
    o, c, e, a, n = scores.get("O", 50), scores.get("C", 50), scores.get("E", 50), scores.get("A", 50), scores.get("N", 50)

    if o >= 70:
        parts.append("Tienes una alta apertura a la experiencia: eres creativo/a, imaginativo/a y disfrutas de las ideas nuevas.")
    elif o <= 30:
        parts.append("Tienes una apertura moderada: prefieres lo práctico y lo concreto sobre lo teórico.")
    else:
        parts.append("Tu apertura a la experiencia está en un nivel medio.")

    if c >= 70:
        parts.append("Eres muy responsable y organizado/a: te gusta cumplir con tus obligaciones y mantener el orden.")
    elif c <= 30:
        parts.append("Eres flexible y espontáneo/a: prefieres mantener las opciones abiertas.")
    else:
        parts.append("Tu nivel de responsabilidad está equilibrado.")

    if e >= 70:
        parts.append("Eres extraverso/a: disfrutas de la compañía de otros y te sientes energizado/a al interactuar.")
    elif e <= 30:
        parts.append("Eres introvertido/a: valoras tu tiempo a solas y te recargas en la tranquilidad.")
    else:
        parts.append("Tu nivel de extraversión está equilibrado.")

    if a >= 70:
        parts.append("Eres muy amable y cooperativo/a: te importan los sentimientos de los demás.")
    elif a <= 30:
        parts.append("Eres directo/a y analítico/a: priorizas la lógica sobre las emociones.")
    else:
        parts.append("Tu nivel de amabilidad está equilibrado.")

    if n >= 70:
        parts.append("Tiendes a experimentar emociones intensas y a preocuparte frecuentemente.")
    elif n <= 30:
        parts.append("Eres emocionalmente estable y relajado/a ante el estrés.")
    else:
        parts.append("Tu nivel de neuroticismo está equilibrado.")

    return " ".join(parts)


def _summary_en(scores: dict) -> str:
    parts = []
    o, c, e, a, n = scores.get("O", 50), scores.get("C", 50), scores.get("E", 50), scores.get("A", 50), scores.get("N", 50)

    if o >= 70:
        parts.append("You have high openness to experience: you are creative, imaginative, and enjoy new ideas.")
    elif o <= 30:
        parts.append("You have moderate openness: you prefer practical and concrete over theoretical.")
    else:
        parts.append("Your openness to experience is at a balanced level.")

    if c >= 70:
        parts.append("You are highly conscientious: you like fulfilling obligations and maintaining order.")
    elif c <= 30:
        parts.append("You are flexible and spontaneous: you prefer keeping options open.")
    else:
        parts.append("Your conscientiousness level is balanced.")

    if e >= 70:
        parts.append("You are extraverted: you enjoy others' company and feel energized by interaction.")
    elif e <= 30:
        parts.append("You are introverted: you value alone time and recharge in tranquility.")
    else:
        parts.append("Your extraversion level is balanced.")

    if a >= 70:
        parts.append("You are very agreeable and cooperative: you care about others' feelings.")
    elif a <= 30:
        parts.append("You are direct and analytical: you prioritize logic over emotions.")
    else:
        parts.append("Your agreeableness level is balanced.")

    if n >= 70:
        parts.append("You tend to experience intense emotions and worry frequently.")
    elif n <= 30:
        parts.append("You are emotionally stable and relaxed under stress.")
    else:
        parts.append("Your neuroticism level is balanced.")

    return " ".join(parts)
