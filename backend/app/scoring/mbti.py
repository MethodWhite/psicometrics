"""MBTI Scoring Algorithm — 4 Dichotomies.

72 items (18 per dichotomy), forced-choice A/B format.
Each dichotomy has equal items for each pole.
"""

from app.data_loader import load_questions


DICHOTOMY_MAP = {
    "EI": ("E", "I"),
    "SN": ("S", "N"),
    "TF": ("T", "F"),
    "JP": ("J", "P"),
}


def score_mbti(answers: dict[int, str], language: str = "es") -> dict:
    """Score MBTI test from question answers.

    Args:
        answers: dict mapping question_id (1-72) to value ('a' or 'b')
        language: 'es' or 'en'

    Returns:
        dict with type_code, scores (0-100 per dichotomy), percentages, profile_summary
    """
    questions = load_questions("mbti")

    # Count responses per dichotomy per pole
    counts: dict[str, dict[str, int]] = {d: {"a": 0, "b": 0} for d in DICHOTOMY_MAP}

    for q in questions["questions"]:
        qid = q["id"]
        if qid not in answers:
            continue

        dichotomy = q["dichotomy"]
        pole = q["pole"]
        response = answers[qid]

        # Map response to pole
        if response == "a":
            counts[dichotomy]["a"] += 1
        else:
            counts[dichotomy]["b"] += 1

    # Calculate percentages per dichotomy
    scores = {}
    percentages = {}
    type_code = ""

    for dichotomy, (pole_a, pole_b) in DICHOTOMY_MAP.items():
        total = counts[dichotomy]["a"] + counts[dichotomy]["b"]
        if total == 0:
            scores[dichotomy] = 50
            percentages[f"{pole_a}/{pole_b}"] = 50.0
            type_code += pole_a
            continue

        # Score: 0 = pole_a, 100 = pole_b
        score_b = (counts[dichotomy]["b"] / total) * 100
        scores[dichotomy] = round(score_b, 1)
        percentages[f"{pole_a}/{pole_b}"] = round(100 - score_b, 1)

        # Determine type letter
        if counts[dichotomy]["a"] >= counts[dichotomy]["b"]:
            type_code += pole_a
        else:
            type_code += pole_b

    summary = _generate_mbti_summary(type_code, percentages, language)

    return {
        "type_code": type_code,
        "scores": scores,
        "percentages": percentages,
        "profile_summary": summary,
    }


TYPE_DESCRIPTIONS = {
    "es": {
        "INTJ": "El Arquitecto: visionario, estratégico, determinado.",
        "INTP": "El Pensador: analítico, inventivo, curioso.",
        "ENTJ": "El Comandante: líder natural, decidido, asertivo.",
        "ENTP": "El Debatiere: innovador, ingenioso, versátil.",
        "INFJ": "El Defensor: idealista, compasivo, introspectivo.",
        "INFP": "El Mediador: idealista, empático, creativo.",
        "ENFJ": "El Protagonista: carismático, empático, inspirador.",
        "ENFP": "El Activista: entusiasta, creativo, sociable.",
        "ISTJ": "El Logístico: práctico, confiable, metódico.",
        "ISFJ": "El Protector: servicial, dedicado, cuidadoso.",
        "ESTJ": "El Ejecutivo: organizado, decidido, liderazgo.",
        "ESFJ": "El Cónsul: cooperativo, amable, social.",
        "ISTP": "El Virtuoso: práctico, observador, independiente.",
        "ISFP": "El Aventurero: flexible, creativo, sensible.",
        "ESTP": "El Emprendedor: enérgico, pragmático, atrevido.",
        "ESFP": "El Animador: espontáneo, entusiasta, generoso.",
    },
    "en": {
        "INTJ": "The Architect: visionary, strategic, determined.",
        "INTP": "The Logician: analytical, inventive, curious.",
        "ENTJ": "The Commander: natural leader, decisive, assertive.",
        "ENTP": "The Debater: innovative, witty, versatile.",
        "INFJ": "The Advocate: idealistic, compassionate, introspective.",
        "INFP": "The Mediator: idealistic, empathetic, creative.",
        "ENFJ": "The Protagonist: charismatic, empathetic, inspiring.",
        "ENFP": "The Campaigner: enthusiastic, creative, sociable.",
        "ISTJ": "The Logistician: practical, reliable, methodical.",
        "ISFJ": "The Defender: supportive, dedicated, careful.",
        "ESTJ": "The Executive: organized, decisive, leadership.",
        "ESFJ": "The Consul: cooperative, kind, social.",
        "ISTP": "The Virtuoso: practical, observant, independent.",
        "ISFP": "The Adventurer: flexible, creative, sensitive.",
        "ESTP": "The Entrepreneur: energetic, pragmatic, bold.",
        "ESFP": "The Entertainer: spontaneous, enthusiastic, generous.",
    },
}


def _generate_mbti_summary(type_code: str, percentages: dict, language: str) -> str:
    descs = TYPE_DESCRIPTIONS.get(language, TYPE_DESCRIPTIONS["en"])
    base_desc = descs.get(type_code, f"Tipo {type_code}")

    parts = [base_desc]

    for key, val in percentages.items():
        pole_a, pole_b = key.split("/")
        if val >= 60:
            parts.append(f"Fuerte preferencia por {pole_b} ({val:.0f}%).")
        elif val <= 40:
            parts.append(f"Fuerte preferencia por {pole_a} ({100 - val:.0f}%).")
        else:
            parts.append(f"Preferencia equilibrada entre {pole_a} y {pole_b}.")

    return " ".join(parts)
