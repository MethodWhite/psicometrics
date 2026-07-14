"""Enneagram Scoring Algorithm — 9 Types + Wings.

81 items (9 per type), 4-point Likert scale.
Dominant type = highest score. Wing = adjacent type with next highest score.
"""

from app.data_loader import load_questions


WING_ADJACENCY = {
    1: (9, 2), 2: (1, 3), 3: (2, 4), 4: (3, 5), 5: (4, 6),
    6: (5, 7), 7: (6, 8), 8: (7, 9), 9: (8, 1),
}


def score_enneagram(answers: dict[int, int], language: str = "es") -> dict:
    """Score Enneagram test from question answers.

    Args:
        answers: dict mapping question_id (1-81) to value (1-4)
        language: 'es' or 'en'

    Returns:
        dict with dominant_type, wing, scores (0-100 per type), profile_summary
    """
    questions = load_questions("enneagram")

    # Accumulate raw scores per type
    type_raw: dict[int, list[float]] = {i: [] for i in range(1, 10)}

    for q in questions["questions"]:
        qid = q["id"]
        if qid not in answers:
            continue

        value = answers[qid]
        enne_type = q["type"]
        type_raw[enne_type].append(float(value))

    # Calculate normalized scores (0-100) per type
    type_scores = {}
    for t in range(1, 10):
        values = type_raw[t]
        if values:
            mean = sum(values) / len(values)
            # Normalize: 1-4 scale -> 0-100
            type_scores[t] = round((mean - 1) / 3 * 100, 1)
        else:
            type_scores[t] = 0.0

    # Find dominant type (highest score)
    dominant = max(type_scores, key=type_scores.get)  # type: ignore

    # Find wing (adjacent type with highest score)
    left, right = WING_ADJACENCY[dominant]
    wing_candidates = {left: type_scores[left], right: type_scores[right]}
    wing = max(wing_candidates, key=wing_candidates.get)  # type: ignore

    summary = _generate_enneagram_summary(dominant, wing, type_scores, language)

    return {
        "dominant_type": dominant,
        "wing": wing,
        "scores": type_scores,
        "profile_summary": summary,
    }


TYPE_DESCRIPTIONS = {
    "es": {
        1: "El Perfeccionista: ético, íntegro, autocontrolado.",
        2: "El Ayudador: generoso, demostrativo, posesivo.",
        3: "El Triunfador: adaptable, ambicioso, orientado a la imagen.",
        4: "El Individualista: expresivo, dramático, autoabsorbido.",
        5: "El Investigador: innovador, periférico, isolacionista.",
        6: "El Leal: comprometido, responsable, ansioso.",
        7: "El Entusiasta: espontáneo, versátil, disperso.",
        8: "El Desafío: decisivo, confrontativo, intimidante.",
        9: "El Pacificador: receptivo, complaciente, resignado.",
    },
    "en": {
        1: "The Perfectionist: ethical, integrity, self-controlled.",
        2: "The Helper: generous, demonstrative, possessive.",
        3: "The Achiever: adaptable, ambitious, image-oriented.",
        4: "The Individualist: expressive, dramatic, self-absorbed.",
        5: "The Investigator: innovative, peripheral, isolationist.",
        6: "The Loyalist: committed, responsible, anxious.",
        7: "The Enthusiast: spontaneous, versatile, scattered.",
        8: "The Challenger: decisive, confrontational, intimidating.",
        9: "The Peacemaker: receptive, accommodating, resigned.",
    },
}


def _generate_enneagram_summary(dominant: int, wing: int, scores: dict, language: str) -> str:
    descs = TYPE_DESCRIPTIONS.get(language, TYPE_DESCRIPTIONS["en"])
    desc = descs.get(dominant, f"Tipo {dominant}")

    wing_desc = descs.get(wing, f"Tipo {wing}")

    # Calculate wing influence
    dom_score = scores.get(dominant, 0)
    wing_score = scores.get(wing, 0)
    total = dom_score + wing_score
    wing_pct = (wing_score / total * 100) if total > 0 else 0

    if language == "es":
        return (
            f"Tipo {dominant}w{wing}: {desc} "
            f"con influencia del ala {wing} ({wing_pct:.0f}%). "
            f"Tus puntajes más altos son: Tipo {dominant} ({dom_score:.0f}%), "
            f"Tipo {wing} ({wing_score:.0f}%)."
        )
    return (
        f"Type {dominant}w{wing}: {desc} "
        f"with wing {wing} influence ({wing_pct:.0f}%). "
        f"Your highest scores: Type {dominant} ({dom_score:.0f}%), "
        f"Type {wing} ({wing_score:.0f}%)."
    )
