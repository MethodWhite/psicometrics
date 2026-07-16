import json
from pathlib import Path


_DATA_DIR = Path(__file__).resolve().parent.parent / "data"

with open(_DATA_DIR / "enneagram_questions.json", encoding="utf-8") as _f:
    _DATA = json.load(_f)

_TYPES_INFO = _DATA["types"]
_QUESTIONS = [q for q in _DATA["questions"] if not q.get("attention_check")]
_SCALE_MIN = _DATA["scale"]["min"]
_SCALE_MAX = _DATA["scale"]["max"]

_TYPE_ITEMS: dict[str, list[int]] = {}
for q in _QUESTIONS:
    _TYPE_ITEMS.setdefault(str(q["type"]), []).append(q["id"])

_TYPE_ORDER = [str(i) for i in range(1, 10)]

_TYPE_NAMES = {
    "1": {"es": "El Perfeccionista", "en": "The Perfectionist"},
    "2": {"es": "El Ayudador", "en": "The Helper"},
    "3": {"es": "El Triunfador", "en": "The Achiever"},
    "4": {"es": "El Individualista", "en": "The Individualist"},
    "5": {"es": "El Investigador", "en": "The Investigator"},
    "6": {"es": "El Leal", "en": "The Loyalist"},
    "7": {"es": "El Entusiasta", "en": "The Enthusiast"},
    "8": {"es": "El Desafío", "en": "The Challenger"},
    "9": {"es": "El Pacificador", "en": "The Peacemaker"},
}

_WING_MAP = {
    "1": [9, 2],
    "2": [1, 3],
    "3": [2, 4],
    "4": [3, 5],
    "5": [4, 6],
    "6": [5, 7],
    "7": [6, 8],
    "8": [7, 9],
    "9": [8, 1],
}

_TYPE_SUMMARIES = {
    "1": {
        "es": "Principios sólidos, búsqueda de mejora continua. Pueden ser críticos y perfeccionistas.",
        "en": "Strong principles, continuous improvement. Can be critical and perfectionistic.",
    },
    "2": {
        "es": "Generosos, atentos a las necesidades ajenas. Pueden descuidar sus propias necesidades.",
        "en": "Generous, attentive to others' needs. May neglect their own needs.",
    },
    "3": {
        "es": "Orientados al logro, adaptables y eficientes. Pueden desconectarse de sus emociones.",
        "en": "Achievement-oriented, adaptable, efficient. May disconnect from their emotions.",
    },
    "4": {
        "es": "Creativos, auténticos, emocionalmente profundos. Pueden ser melancólicos y sentir que algo falta.",
        "en": "Creative, authentic, emotionally deep. May be melancholic and feel something is missing.",
    },
    "5": {
        "es": "Analíticos, observadores, independientes. Pueden aislarse y acumular conocimiento sin aplicar.",
        "en": "Analytical, observant, independent. May isolate and hoard knowledge without applying.",
    },
    "6": {
        "es": "Leales, comprometidos, alerta. Pueden ser ansiosos y dudar de sí mismos.",
        "en": "Loyal, committed, alert. May be anxious and self-doubting.",
    },
    "7": {
        "es": "Entusiastas, versátiles, optimistas. Pueden evitar el dolor y dispersar su energía.",
        "en": "Enthusiastic, versatile, optimistic. May avoid pain and scatter their energy.",
    },
    "8": {
        "es": "Decididos, protectores, directos. Pueden ser dominantes y evitar la vulnerabilidad.",
        "en": "Decisive, protective, direct. May be domineering and avoid vulnerability.",
    },
    "9": {
        "es": "Pacíficos, receptivos, armoniosos. Pueden evitar conflictos y perder su propia dirección.",
        "en": "Peaceful, receptive, harmonious. May avoid conflict and lose their own direction.",
    },
}

_WING_NAMES = {
    "1w9": {"es": "1 ala 9 (El Idealista)", "en": "1 wing 9 (The Idealist)"},
    "1w2": {"es": "1 ala 2 (El Abogado)", "en": "1 wing 2 (The Advocate)"},
    "2w1": {"es": "2 ala 1 (El Servidor)", "en": "2 wing 1 (The Servant)"},
    "2w3": {"es": "2 ala 3 (El Anfitrión)", "en": "2 wing 3 (The Host)"},
    "3w2": {"es": "3 ala 2 (El Encantador)", "en": "3 wing 2 (The Charmer)"},
    "3w4": {"es": "3 ala 4 (El Profesional)", "en": "3 wing 4 (The Professional)"},
    "4w3": {"es": "4 ala 3 (El Aristócrata)", "en": "4 wing 3 (The Aristocrat)"},
    "4w5": {"es": "4 ala 5 (El Bohemio)", "en": "4 wing 5 (The Bohemian)"},
    "5w4": {"es": "5 ala 4 (El Iconoclasta)", "en": "5 wing 4 (The Iconoclast)"},
    "5w6": {"es": "5 ala 6 (El Solucionador)", "en": "5 wing 6 (The Problem Solver)"},
    "6w5": {"es": "6 ala 5 (El Defensor)", "en": "6 wing 5 (The Defender)"},
    "6w7": {"es": "6 ala 7 (El Compañero)", "en": "6 wing 7 (The Buddy)"},
    "7w6": {"es": "7 ala 6 (El Animador)", "en": "7 wing 6 (The Entertainer)"},
    "7w8": {"es": "7 ala 8 (El Realista)", "en": "7 wing 8 (The Realist)"},
    "8w7": {"es": "8 ala 7 (El Independiente)", "en": "8 wing 7 (The Maverick)"},
    "8w9": {"es": "8 ala 9 (El Oso)", "en": "8 wing 9 (The Bear)"},
    "9w8": {"es": "9 ala 8 (El Mediador)", "en": "9 wing 8 (The Mediator)"},
    "9w1": {"es": "9 ala 1 (El Pacificador)", "en": "9 wing 1 (The Peacemaker)"},
}


def _normalize(raw_avg: float) -> float:
    return round(((raw_avg - _SCALE_MIN) / (_SCALE_MAX - _SCALE_MIN)) * 100, 1)


def score_enneagram(responses: dict[int | str, int], lang: str = "es") -> dict:
    raw_scores: dict[str, list[float]] = {t: [] for t in _TYPE_ORDER}
    valid_ids = {q["id"] for q in _QUESTIONS}

    for qid_str, raw_val in responses.items():
        qid = int(qid_str)
        if qid not in valid_ids:
            continue
        clamped = max(_SCALE_MIN, min(_SCALE_MAX, int(raw_val)))
        for t, items in _TYPE_ITEMS.items():
            if qid in items:
                raw_scores[t].append(float(clamped))
                break

    scores: dict[str, float] = {}
    for t in _TYPE_ORDER:
        vals = raw_scores.get(t, [])
        if vals:
            avg = sum(vals) / len(vals)
        else:
            avg = _SCALE_MIN
        scores[t] = _normalize(avg)

    ranked = sorted(_TYPE_ORDER, key=lambda t: scores[t], reverse=True)
    dominant = ranked[0]
    second = ranked[1]

    wings = _WING_MAP[dominant]
    w1, w2 = str(wings[0]), str(wings[1])
    if scores.get(w1, 0) >= scores.get(w2, 0):
        wing_type = w1
    else:
        wing_type = w2

    wing_code = f"{dominant}w{wing_type}"
    wing_name = _WING_NAMES.get(
        wing_code,
        {"es": f"Tipo {dominant} con ala {wing_type}", "en": f"Type {dominant} wing {wing_type}"},
    )

    labeled_scores = {}
    for t in _TYPE_ORDER:
        name = _TYPES_INFO[t].get(lang, _TYPES_INFO[t].get("en", f"Type {t}"))
        labeled_scores[f"Tipo {t} ({name})" if lang == "es" else f"Type {t} ({name})"] = scores[t]

    core_fear = _TYPES_INFO[dominant]["core_fear"].get(lang, _TYPES_INFO[dominant]["core_fear"]["en"])
    core_desire = _TYPES_INFO[dominant]["core_desire"].get(lang, _TYPES_INFO[dominant]["core_desire"]["en"])
    summary = _TYPE_SUMMARIES[dominant].get(lang, _TYPE_SUMMARIES[dominant]["en"])

    dominant_name = _TYPE_NAMES[dominant].get(lang, _TYPE_NAMES[dominant]["en"])
    wing_name_str = wing_name.get(lang, wing_name["en"])

    profile_title = f"{dominant_name} ({wing_name_str})"
    profile_summary = (
        f"{profile_title}: {summary}\n"
        f"{'Miedo básico' if lang == 'es' else 'Core fear'}: {core_fear}\n"
        f"{'Deseo básico' if lang == 'es' else 'Core desire'}: {core_desire}"
    )

    recommendations = {
        "es": [
            f"Como {dominant_name}, tu camino de crecimiento incluye reconocer tus patrones automáticos.",
            f"Trabaja en integrar las cualidades positivas de tu ala {wing_type} y de los tipos en tus flechas de integración.",
            "Practica la auto-observación sin juicio para identificar cuándo actúas desde tu mecanismo de defensa.",
            "Considera llevar un diario de autoconocimiento para profundizar en tu tipo de eneagrama.",
        ],
        "en": [
            f"As a {dominant_name}, your growth path includes recognizing your automatic patterns.",
            f"Work on integrating the positive qualities of your wing {wing_type} and your integration arrow types.",
            "Practice non-judgmental self-observation to identify when you act from your defense mechanism.",
            "Consider keeping a self-awareness journal to deepen your enneagram type understanding.",
        ],
    }

    interpretation = (
        f"{profile_title}: {summary} "
        f"{'Miedo básico' if lang == 'es' else 'Core fear'}: {core_fear}. "
        f"{'Deseo básico' if lang == 'es' else 'Core desire'}: {core_desire}."
    )

    return {
        "dominant_type": int(dominant),
        "dominant_name": dominant_name,
        "wing": int(wing_type),
        "wing_code": wing_code,
        "wing_name": wing_name_str,
        "scores": labeled_scores,
        "profile_summary": profile_summary,
        "interpretation": interpretation,
        "recommendations": recommendations[lang if lang in recommendations else "en"],
    }
