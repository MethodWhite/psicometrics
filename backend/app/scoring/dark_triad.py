import json
import math
from pathlib import Path


_DATA_DIR = Path(__file__).resolve().parent.parent / "data"

with open(_DATA_DIR / "dark_triad_questions.json", encoding="utf-8") as _f:
    _DATA = json.load(_f)

_TRAITS_INFO = _DATA["traits"]
_QUESTIONS = [q for q in _DATA["questions"] if not q.get("attention_check")]

_TRAIT_ITEMS: dict[str, list[int]] = {}
for q in _QUESTIONS:
    _TRAIT_ITEMS.setdefault(q["trait"], []).append(q["id"])

_TRAIT_KEYS = sorted(_TRAIT_ITEMS.keys())  # ["M", "N", "P"]
_SCALE_MIN = _DATA["scale"]["min"]
_SCALE_MAX = _DATA["scale"]["max"]


def _normalize(raw_avg: float) -> float:
    """Normalize from [SCALE_MIN, SCALE_MAX] to 0-100."""
    return round(((raw_avg - _SCALE_MIN) / (_SCALE_MAX - _SCALE_MIN)) * 100, 1)


def _risk_level(scores: dict[str, float]) -> str:
    """Determine overall risk level based on Dark Core composite."""
    dark_core = sum(scores.values()) / len(scores)
    if dark_core >= 70:
        return "high"
    if dark_core >= 50:
        return "moderate"
    return "low"


_DESCRIPTIONS = {
    "M": {
        "es": "Maquiavelismo: tendencia a la manipulación estratégica, cinismo moral y enfoque pragmático en las relaciones interpersonales.",
        "en": "Machiavellianism: tendency toward strategic manipulation, moral cynicism, and a pragmatic approach to interpersonal relationships.",
    },
    "N": {
        "es": "Narcisismo: grandiosidad, necesidad de admiración, sentido de superioridad y expectativa de trato especial.",
        "en": "Narcissism: grandiosity, need for admiration, sense of superiority, and expectation of special treatment.",
    },
    "P": {
        "es": "Psicopatía: impulsividad, falta de empatía, desapego social y baja tolerancia a la frustración.",
        "en": "Psychopathy: impulsivity, lack of empathy, social detachment, and low frustration tolerance.",
    },
}


def score_dark_triad(responses: dict[int | str, int], lang: str = "es") -> dict:
    raw_scores: dict[str, list[float]] = {t: [] for t in _TRAIT_KEYS}
    valid_ids = {q["id"] for q in _QUESTIONS}

    for qid_str, raw_val in responses.items():
        qid = int(qid_str)
        if qid not in valid_ids:
            continue
        for trait, items in _TRAIT_ITEMS.items():
            if qid in items:
                clamped = max(_SCALE_MIN, min(_SCALE_MAX, int(raw_val)))
                raw_scores[trait].append(float(clamped))
                break

    trait_scores: dict[str, float] = {}
    for trait in _TRAIT_KEYS:
        vals = raw_scores[trait]
        if vals:
            avg = sum(vals) / len(vals)
        else:
            avg = _SCALE_MIN
        trait_scores[trait] = _normalize(avg)

    dark_core = round(sum(trait_scores.values()) / len(trait_scores), 1)
    risk = _risk_level(trait_scores)

    labeled_scores = {}
    for t in _TRAIT_KEYS:
        name = _TRAITS_INFO[t][lang if lang in _TRAITS_INFO[t] else "en"]
        labeled_scores[name] = trait_scores[t]

    summary_parts = []
    for t in _TRAIT_KEYS:
        val = trait_scores[t]
        name_en = _TRAITS_INFO[t]["en"]
        desc = _DESCRIPTIONS[t][lang if lang in _DESCRIPTIONS[t] else "en"]
        level = "bajo" if val < 40 else ("moderado" if val < 60 else "elevado")
        if lang == "en":
            level_en = "low" if val < 40 else ("moderate" if val < 60 else "elevated")
            summary_parts.append(f"{name_en}: {val}/100 ({level_en}). {desc}")
        else:
            summary_parts.append(
                f"{_TRAITS_INFO[t][lang]}: {val}/100 ({level}). {desc}"
            )

    profile_summary = "\n".join(summary_parts)

    risk_labels = {
        "es": {
            "low": "Bajo — Los rasgos de la triada oscura son mínimos. Predomina la cooperación y la empatía.",
            "moderate": "Moderado — Algunos rasgos están presentes pero dentro de rangos normales de la población.",
            "high": "Elevado — Varios rasgos de la triada oscura están significativamente presentes. Se recomienda reflexión.",
        },
        "en": {
            "low": "Low — Dark triad traits are minimal. Cooperation and empathy predominate.",
            "moderate": "Moderate — Some traits are present but within normal population ranges.",
            "high": "High — Several dark triad traits are significantly present. Reflection is recommended.",
        },
    }

    interpretations = {
        "M": {
            "es": lambda s: (
                "Puntuación baja en maquiavelismo. Prefieres relaciones directas y transparentes."
                if s < 40
                else (
                    "Puntuación moderada. Puedes ser estratégico ocasionalmente pero mantienes principios."
                    if s < 60
                    else "Puntuación elevada. Tiendes a usar la manipulación estratégica para lograr objetivos."
                )
            ),
            "en": lambda s: (
                "Low Machiavellianism score. You prefer direct and transparent relationships."
                if s < 40
                else (
                    "Moderate score. You can be strategic occasionally but maintain principles."
                    if s < 60
                    else "High score. You tend to use strategic manipulation to achieve goals."
                )
            ),
        },
        "N": {
            "es": lambda s: (
                "Puntuación baja en narcisismo. Eres humilde y no buscas reconocimiento constante."
                if s < 40
                else (
                    "Puntuación moderada. Tienes autoestima saludable sin caer en la grandiosidad."
                    if s < 60
                    else "Puntuación elevada. Buscas admiración y tiendes a sentirte superior."
                )
            ),
            "en": lambda s: (
                "Low Narcissism score. You are humble and don't seek constant recognition."
                if s < 40
                else (
                    "Moderate score. You have healthy self-esteem without grandiosity."
                    if s < 60
                    else "High score. You seek admiration and tend to feel superior."
                )
            ),
        },
        "P": {
            "es": lambda s: (
                "Puntuación baja en psicopatía. Eres empático y consideras las consecuencias de tus actos."
                if s < 40
                else (
                    "Puntuación moderada. Puedes ser impulsivo ocasionalmente pero mantienes conexiones."
                    if s < 60
                    else "Puntuación elevada. Bajos niveles de empatía y alta impulsividad."
                )
            ),
            "en": lambda s: (
                "Low Psychopathy score. You are empathetic and consider consequences."
                if s < 40
                else (
                    "Moderate score. You can be impulsive occasionally but maintain connections."
                    if s < 60
                    else "High score. Low empathy and high impulsivity."
                )
            ),
        },
    }

    interp_list = []
    for t in _TRAIT_KEYS:
        fn = interpretations[t][lang if lang in interpretations[t] else "en"]
        interp_list.append(fn(trait_scores[t]))
    interpretation = "\n".join(interp_list)

    recommendations = {
        "es": [
            "Reflexiona sobre cómo tus acciones afectan a los demás.",
            "Practica la empatía activa en tus relaciones cotidianas.",
            "Considera buscar retroalimentación honesta de personas de confianza.",
            "Si tus puntuaciones son elevadas, considera hablar con un profesional de la salud mental.",
        ],
        "en": [
            "Reflect on how your actions affect others.",
            "Practice active empathy in your daily relationships.",
            "Consider seeking honest feedback from trusted people.",
            "If your scores are high, consider speaking with a mental health professional.",
        ],
    }

    return {
        "scores": trait_scores,
        "labeled_scores": labeled_scores,
        "dark_core": dark_core,
        "risk_level": risk,
        "risk_description": risk_labels[lang if lang in risk_labels else "en"][risk],
        "profile_summary": profile_summary,
        "interpretation": interpretation,
        "recommendations": recommendations[lang if lang in recommendations else "en"],
    }
