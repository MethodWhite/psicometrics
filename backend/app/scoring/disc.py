"""DISC Scoring Algorithm — 4 Dimensions.

28 items (7 per dimension), multiple choice A/B/C/D format.
Each answer maps to one of the 4 DISC dimensions.
"""

from app.data_loader import load_questions


ANSWER_TO_DIMENSION = {"a": "D", "b": "I", "c": "S", "d": "C"}

DIMENSION_NAMES = {
    "D": {"es": "Dominancia", "en": "Dominance"},
    "I": {"es": "Influencia", "en": "Influence"},
    "S": {"es": "Estabilidad", "en": "Steadiness"},
    "C": {"es": "Conciencia", "en": "Conscientiousness"},
}

STYLE_DESCRIPTIONS = {
    "es": {
        "D": "Dominante: directo, decidido, orientado a resultados.",
        "I": "Influyente: entusiasta, optimista, sociable.",
        "S": "Estable: paciente, confiable,合作.",
        "C": "Consciente: analítico, preciso, sistemático.",
    },
    "en": {
        "D": "Dominant: direct, decisive, results-oriented.",
        "I": "Influential: enthusiastic, optimistic, sociable.",
        "S": "Steady: patient, reliable, cooperative.",
        "C": "Conscientious: analytical, precise, systematic.",
    },
}


def score_disc(answers: dict[int, str], language: str = "es") -> dict:
    """Score DISC test from question answers.

    Args:
        answers: dict mapping question_id (1-28) to value ('a','b','c','d')
        language: 'es' or 'en'

    Returns:
        dict with primary_style, secondary_style, scores (0-100 per dimension), profile_summary
    """
    questions = load_questions("disc")

    # Count responses per dimension
    dim_counts: dict[str, int] = {"D": 0, "I": 0, "S": 0, "C": 0}

    for q in questions["questions"]:
        qid = q["id"]
        if qid not in answers:
            continue

        response = answers[qid]
        dimension = ANSWER_TO_DIMENSION.get(response)
        if dimension:
            dim_counts[dimension] += 1

    # Calculate normalized scores (0-100)
    total = sum(dim_counts.values())
    scores = {}
    for dim in ["D", "I", "S", "C"]:
        if total > 0:
            scores[dim] = round((dim_counts[dim] / total) * 100, 1)
        else:
            scores[dim] = 25.0

    # Determine primary and secondary styles
    sorted_dims = sorted(scores.items(), key=lambda x: x[1], reverse=True)
    primary = sorted_dims[0][0]
    secondary = sorted_dims[1][0]

    summary = _generate_disc_summary(primary, secondary, scores, language)

    return {
        "primary_style": primary,
        "secondary_style": secondary,
        "scores": scores,
        "profile_summary": summary,
    }


def _generate_disc_summary(primary: str, secondary: str, scores: dict, language: str) -> str:
    descs = STYLE_DESCRIPTIONS.get(language, STYLE_DESCRIPTIONS["en"])
    p_desc = descs.get(primary, "")
    s_desc = descs.get(secondary, "")

    if language == "es":
        return (
            f"Estilo primario: {DIMENSION_NAMES[primary]['es']} ({scores[primary]:.0f}%). "
            f"Estilo secundario: {DIMENSION_NAMES[secondary]['es']} ({scores[secondary]:.0f}%). "
            f"{p_desc} "
            f"Con influencia secundaria: {s_desc}"
        )
    return (
        f"Primary style: {DIMENSION_NAMES[primary]['en']} ({scores[primary]:.0f}%). "
        f"Secondary style: {DIMENSION_NAMES[secondary]['en']} ({scores[secondary]:.0f}%). "
        f"{p_desc} "
        f"With secondary influence: {s_desc}"
    )
