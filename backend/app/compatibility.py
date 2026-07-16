"""Personality type / profile compatibility comparisons.

Supports Big Five factor-level comparison and MBTI type matching
with bilingual (es/en) descriptions.
"""

from __future__ import annotations

from typing import Any

BIG_FIVE_FACTORS = ["O", "C", "E", "A", "N"]

FACTOR_LABELS: dict[str, dict[str, str]] = {
    "es": {
        "O": "Apertura a la Experiencia",
        "C": "Responsabilidad",
        "E": "Extraversión",
        "A": "Amabilidad",
        "N": "Neuroticismo",
    },
    "en": {
        "O": "Openness to Experience",
        "C": "Conscientiousness",
        "E": "Extraversion",
        "A": "Agreeableness",
        "N": "Neuroticism",
    },
}

# MBTI type compatibility mapping.
# Scores: identical=90, similar=75, complementary=60, challenging=40.
_MBTI_COMPAT: dict[str, dict[str, int]] = {
    # ENFJ
    "ENFJ": {"ENFJ": 90, "ENFP": 75, "INFJ": 75, "INFP": 60,
             "ENTJ": 60, "ENTP": 60, "INTJ": 60, "INTP": 40,
             "ESFJ": 75, "ESFP": 60, "ISFJ": 60, "ISFP": 40,
             "ESTJ": 40, "ESTP": 40, "ISTJ": 40, "ISTP": 40},
    # ENFP
    "ENFP": {"ENFP": 90, "ENFJ": 75, "INFP": 75, "INFJ": 60,
             "ENTP": 75, "ENTJ": 60, "INTJ": 60, "INTP": 60,
             "ESFP": 75, "ESFJ": 60, "ISFP": 60, "ISFJ": 40,
             "ESTP": 60, "ESTJ": 40, "ISTP": 40, "ISTJ": 40},
    # INFJ
    "INFJ": {"INFJ": 90, "INFP": 75, "ENFJ": 75, "ENFP": 60,
             "INTJ": 75, "INTP": 60, "ENTJ": 60, "ENTP": 40,
             "ISFJ": 60, "ISFP": 60, "ESFJ": 40, "ESFP": 40,
             "ISTJ": 40, "ISTP": 40, "ESTJ": 40, "ESTP": 40},
    # INFP
    "INFP": {"INFP": 90, "INFJ": 75, "ENFP": 75, "ENFJ": 60,
             "INTP": 75, "INTJ": 60, "ENTP": 60, "ENTJ": 40,
             "ISFP": 75, "ISFJ": 60, "ESFP": 60, "ESFJ": 40,
             "ISTP": 40, "ISTJ": 40, "ESTP": 40, "ESTJ": 40},
    # ENTJ
    "ENTJ": {"ENTJ": 90, "ENTP": 75, "INTJ": 75, "INTP": 60,
             "ENFJ": 60, "ENFP": 60, "INFJ": 60, "INFP": 40,
             "ESTJ": 75, "ESTP": 60, "ISTJ": 60, "ISTP": 40,
             "ESFJ": 40, "ESFP": 40, "ISFJ": 40, "ISFP": 40},
    # ENTP
    "ENTP": {"ENTP": 90, "ENTJ": 75, "INTP": 75, "INTJ": 60,
             "ENFP": 75, "ENFJ": 60, "INFJ": 60, "INFP": 60,
             "ESTP": 75, "ESTJ": 60, "ISTP": 60, "ISTJ": 40,
             "ESFP": 60, "ESFJ": 40, "ISFP": 40, "ISFJ": 40},
    # INTJ
    "INTJ": {"INTJ": 90, "INTP": 75, "ENTJ": 75, "ENTP": 60,
             "INFJ": 75, "INFP": 60, "ENFJ": 60, "ENFP": 40,
             "ISTJ": 60, "ISTP": 60, "ESTJ": 40, "ESTP": 40,
             "ISFJ": 40, "ISFP": 40, "ESFJ": 40, "ESFP": 40},
    # INTP
    "INTP": {"INTP": 90, "INTJ": 75, "ENTP": 75, "ENTJ": 60,
             "INFP": 75, "INFJ": 60, "ENFP": 60, "ENFJ": 40,
             "ISTP": 60, "ISTJ": 60, "ESTP": 40, "ESTJ": 40,
             "ISFP": 40, "ISFJ": 40, "ESFP": 40, "ESFJ": 40},
    # ESFJ
    "ESFJ": {"ESFJ": 90, "ESFP": 75, "ISFJ": 75, "ISFP": 60,
             "ENFJ": 75, "ENFP": 60, "INFJ": 60, "INFP": 40,
             "ESTJ": 60, "ESTP": 40, "ISTJ": 60, "ISTP": 40,
             "ENTJ": 40, "ENTP": 40, "INTJ": 40, "INTP": 40},
    # ESFP
    "ESFP": {"ESFP": 90, "ESFJ": 75, "ISFP": 75, "ISFJ": 60,
             "ENFP": 75, "ENFJ": 60, "INFP": 60, "INFJ": 40,
             "ESTP": 60, "ESTJ": 40, "ISTP": 60, "ISTJ": 40,
             "ENTP": 60, "ENTJ": 40, "INTP": 40, "INTJ": 40},
    # ISFJ
    "ISFJ": {"ISFJ": 90, "ISFP": 75, "ESFJ": 75, "ESFP": 60,
             "INFJ": 60, "INFP": 60, "ENFJ": 40, "ENFP": 40,
             "ISTJ": 60, "ISTP": 60, "ESTJ": 40, "ESTP": 40,
             "INTJ": 40, "INTP": 40, "ENTJ": 40, "ENTP": 40},
    # ISFP
    "ISFP": {"ISFP": 90, "ISFJ": 75, "ESFP": 75, "ESFJ": 60,
             "INFP": 75, "INFJ": 60, "ENFP": 60, "ENFJ": 40,
             "ISTP": 60, "ISTJ": 60, "ESTP": 40, "ESTJ": 40,
             "INTP": 40, "INTJ": 40, "ENTP": 40, "ENTJ": 40},
    # ESTJ
    "ESTJ": {"ESTJ": 90, "ESTP": 75, "ISTJ": 75, "ISTP": 60,
             "ENTJ": 75, "ENTP": 60, "INTJ": 60, "INTP": 40,
             "ESFJ": 60, "ESFP": 40, "ISFJ": 60, "ISFP": 40,
             "ENFJ": 40, "ENFP": 40, "INFJ": 40, "INFP": 40},
    # ESTP
    "ESTP": {"ESTP": 90, "ESTJ": 75, "ISTP": 75, "ISTJ": 60,
             "ENTP": 75, "ENTJ": 60, "INTP": 60, "INTJ": 40,
             "ESFP": 60, "ESFJ": 40, "ISFP": 60, "ISFJ": 40,
             "ENFP": 60, "ENFJ": 40, "INFP": 40, "INFJ": 40},
    # ISTJ
    "ISTJ": {"ISTJ": 90, "ISTP": 75, "ESTJ": 75, "ESTP": 60,
             "INTJ": 60, "INTP": 60, "ENTJ": 40, "ENTP": 40,
             "ISFJ": 60, "ISFP": 60, "ESFJ": 40, "ESFP": 40,
             "INFJ": 40, "INFP": 40, "ENFJ": 40, "ENFP": 40},
    # ISTP
    "ISTP": {"ISTP": 90, "ISTJ": 75, "ESTP": 75, "ESTJ": 60,
             "INTP": 60, "INTJ": 60, "ENTP": 40, "ENTJ": 40,
             "ISFP": 60, "ISFJ": 60, "ESFP": 40, "ESFJ": 40,
             "INFP": 40, "INFJ": 40, "ENFP": 40, "ENFJ": 40},
}

_MBTI_LABELS: dict[str, dict[str, str]] = {
    "es": {
        "identical": "Identical — máxima compatibilidad",
        "similar": "Similar — alta compatibilidad",
        "complementary": "Complementaria — buena compatibilidad",
        "challenging": "Desafiante — diferencias significativas",
    },
    "en": {
        "identical": "Identical — maximum compatibility",
        "similar": "Similar — high compatibility",
        "complementary": "Complementary — good compatibility",
        "challenging": "Challenging — significant differences",
    },
}

_DESCRIPTIONS: dict[str, dict[str, str]] = {
    "es": {
        "identical": "Ambos comparten el mismo tipo de personalidad. "
        "Se entienden con facilidad y tienen estilos de comunicación similares.",
        "similar": "Comparten tres de las cuatro letras del tipo MBTI. "
        "Existe una base sólida de entendimiento mutuo.",
        "complementary": "Comparten dos letras. "
        "Sus diferencias pueden fortalecer la relación al complementarse.",
        "challenging": "Comparten una o ninguna letra. "
        "Pueden enfrentar malentendidos, pero también oportunidades de crecimiento.",
    },
    "en": {
        "identical": "Both share the same personality type. "
        "They understand each other easily with similar communication styles.",
        "similar": "They share three of the four MBTI letters. "
        "A solid foundation for mutual understanding.",
        "complementary": "They share two letters. "
        "Differences can strengthen the relationship through complementarity.",
        "challenging": "They share one or no letters. "
        "May face misunderstandings, but also growth opportunities.",
    },
}


def _big_five_compatibility(
    result_a: dict[str, float],
    result_b: dict[str, float],
    lang: str,
) -> dict[str, Any]:
    """Calculate Big Five compatibility between two profiles."""
    factors: list[dict[str, Any]] = []
    total_diff = 0.0

    for factor in BIG_FIVE_FACTORS:
        a_val = result_a.get(factor, 50)
        b_val = result_b.get(factor, 50)
        diff = round(abs(a_val - b_val), 1)
        total_diff += diff
        label = FACTOR_LABELS.get(lang, FACTOR_LABELS["en"]).get(factor, factor)
        factors.append({"factor": factor, "label": label, "diff": diff,
                        "a": a_val, "b": b_val})

    # Compatibility score: 0 diff → 100, max possible diff per factor is 100,
    # over 5 factors = 500.  Invert so lower diff → higher score.
    max_diff = 500.0
    compatibility_score = round(max(0, 100 - (total_diff / max_diff) * 100), 1)

    if lang == "es":
        desc = (
            f"Compatibilidad Big Five: {compatibility_score}%. "
            f"Las diferencias en cada factor suman {total_diff} puntos."
        )
    else:
        desc = (
            f"Big Five compatibility: {compatibility_score}%. "
            f"Factor differences total {total_diff} points."
        )

    return {"compatibility_score": compatibility_score, "description": desc,
            "factors": factors}


def _mbti_compatibility(
    type_a: str,
    type_b: str,
    lang: str,
) -> dict[str, Any]:
    """Calculate MBTI type compatibility."""
    type_a = type_a.upper().strip()
    type_b = type_b.upper().strip()

    row = _MBTI_COMPAT.get(type_a, {})
    score = row.get(type_b, 50)

    if score >= 85:
        level = "identical"
    elif score >= 70:
        level = "similar"
    elif score >= 50:
        level = "complementary"
    else:
        level = "challenging"

    labels = _MBTI_LABELS.get(lang, _MBTI_LABELS["en"])
    descs = _DESCRIPTIONS.get(lang, _DESCRIPTIONS["en"])

    description = f"{labels[level]}. {descs[level]}"

    return {
        "compatibility_score": score,
        "description": description,
        "factors": [
            {"label": "MBTI Type Match", "level": level,
             "type_a": type_a, "type_b": type_b},
        ],
    }


def compare_results(
    result_a: dict[str, Any],
    result_b: dict[str, Any],
    test_type: str,
    lang: str = "es",
) -> dict[str, Any]:
    """Compare two personality profiles and return compatibility info.

    Parameters
    ----------
    result_a, result_b:
        Full result dicts as returned by the scoring modules.
        Each should contain at least a ``scores`` sub-dict (Big Five)
        or a ``type`` key (MBTI).
    test_type:
        One of ``"big_five"``, ``"mbti"``.
    lang:
        Language code for descriptions (``"es"`` or ``"en"``).

    Returns
    -------
    dict with keys ``compatibility_score``, ``description``, ``factors``.
    """
    if test_type == "big_five":
        scores_a = result_a.get("scores", result_a)
        scores_b = result_b.get("scores", result_b)
        return _big_five_compatibility(scores_a, scores_b, lang)

    if test_type == "mbti":
        type_a = result_a.get("type", "INFJ")
        type_b = result_b.get("type", "INFJ")
        return _mbti_compatibility(type_a, type_b, lang)

    # Fallback: unknown test type
    labels = FACTOR_LABELS.get(lang, FACTOR_LABELS["en"])
    if lang == "es":
        desc = f"La compatibilidad para {test_type} no está implementada."
    else:
        desc = f"Compatibility for {test_type} is not implemented yet."
    return {"compatibility_score": 50, "description": desc, "factors": [
        {"label": labels.get("O", "Unknown"), "diff": 0},
    ]}
