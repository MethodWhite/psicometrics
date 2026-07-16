import json
from pathlib import Path


_DATA_DIR = Path(__file__).resolve().parent.parent / "data"

with open(_DATA_DIR / "eq_questions.json", encoding="utf-8") as _f:
    _DATA = json.load(_f)

_DIMENSIONS = _DATA["dimensions"]
_QUESTIONS = _DATA["questions"]
_SCALE_MIN = _DATA["scale"]["min"]
_SCALE_MAX = _DATA["scale"]["max"]

_DIM_ITEMS: dict[str, list[int]] = {}
_REVERSED: dict[int, bool] = {}
for q in _QUESTIONS:
    _DIM_ITEMS.setdefault(q["dimension"], []).append(q["id"])
    if "invertida" in q.get("text", {}).get("es", "") or "reversed" in q.get("text", {}).get("en", ""):
        _REVERSED[q["id"]] = True

_DIM_ORDER = ["self_awareness", "self_regulation", "empathy", "social_skills"]

_DIM_LABELS = {
    "self_awareness": {"es": "Autoconciencia", "en": "Self-Awareness"},
    "self_regulation": {"es": "Autorregulación", "en": "Self-Regulation"},
    "empathy": {"es": "Empatía", "en": "Empathy"},
    "social_skills": {"es": "Habilidades Sociales", "en": "Social Skills"},
}


def _normalize(raw_avg: float) -> float:
    return round(((raw_avg - _SCALE_MIN) / (_SCALE_MAX - _SCALE_MIN)) * 100, 1)


def score_emotional_intelligence(responses: dict[int | str, int], lang: str = "es") -> dict:
    raw_scores: dict[str, list[float]] = {d: [] for d in _DIM_ITEMS}
    valid_ids = {q["id"] for q in _QUESTIONS}

    for qid_str, raw_val in responses.items():
        qid = int(qid_str)
        if qid not in valid_ids:
            continue
        clamped = max(_SCALE_MIN, min(_SCALE_MAX, int(raw_val)))
        if qid in _REVERSED:
            val = _SCALE_MAX + _SCALE_MIN - clamped
        else:
            val = float(clamped)
        for dim, items in _DIM_ITEMS.items():
            if qid in items:
                raw_scores[dim].append(val)
                break

    scores: dict[str, float] = {}
    for dim in _DIM_ORDER:
        vals = raw_scores.get(dim, [])
        if vals:
            avg = sum(vals) / len(vals)
        else:
            avg = _SCALE_MIN
        scores[dim] = _normalize(avg)

    eq_total = round(sum(scores.values()) / len(scores), 1)

    labeled_scores = {}
    for dim in _DIM_ORDER:
        label = _DIM_LABELS[dim][lang if lang in _DIM_LABELS[dim] else "en"]
        labeled_scores[label] = scores[dim]

    level = (
        "bajo" if eq_total < 40 else ("moderado" if eq_total < 60 else "alto")
    )
    level_en = (
        "low" if eq_total < 40 else ("moderate" if eq_total < 60 else "high")
    )

    eq_labels = {
        "es": {
            "bajo": "Tu inteligencia emocional general está por debajo del promedio. Puede ser un área de crecimiento importante.",
            "moderado": "Tu inteligencia emocional está en un rango promedio. Hay espacio para desarrollar habilidades específicas.",
            "alto": "Tu inteligencia emocional es sólida. Gestionas bien tus emociones y relaciones.",
        },
        "en": {
            "low": "Your overall EQ is below average. This may be an important growth area.",
            "moderate": "Your emotional intelligence is in the average range. There is room to develop specific skills.",
            "high": "Your emotional intelligence is strong. You manage your emotions and relationships well.",
        },
    }

    return {
        "scores": labeled_scores,
        "eq_total": eq_total,
        "level": level_en if lang == "en" else level,
        "eq_description": eq_labels[lang if lang in eq_labels else "en"][
            level_en if lang == "en" else level
        ],
    }
