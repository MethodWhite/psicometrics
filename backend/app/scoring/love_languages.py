import json
from pathlib import Path


_DATA_DIR = Path(__file__).resolve().parent.parent / "data"

with open(_DATA_DIR / "love_languages_questions.json", encoding="utf-8") as _f:
    _DATA = json.load(_f)

_DIMENSIONS = _DATA["dimensions"]
_QUESTIONS = _DATA["questions"]
_SCALE_MIN = _DATA["scale"]["min"]
_SCALE_MAX = _DATA["scale"]["max"]

_DIM_ITEMS: dict[str, list[int]] = {}
for q in _QUESTIONS:
    _DIM_ITEMS.setdefault(q["dimension"], []).append(q["id"])


def _normalize(raw_avg: float) -> float:
    return round(((raw_avg - _SCALE_MIN) / (_SCALE_MAX - _SCALE_MIN)) * 100, 1)


_DIM_LABELS = {
    "words_of_affirmation": {"es": "Palabras de Afirmación", "en": "Words of Affirmation"},
    "quality_time": {"es": "Tiempo de Calidad", "en": "Quality Time"},
    "receiving_gifts": {"es": "Recepción de Regalos", "en": "Receiving Gifts"},
    "acts_of_service": {"es": "Actos de Servicio", "en": "Acts of Service"},
    "physical_touch": {"es": "Contacto Físico", "en": "Physical Touch"},
}


def score_love_languages(responses: dict[int | str, int], lang: str = "es") -> dict:
    raw_scores: dict[str, list[float]] = {d: [] for d in _DIM_ITEMS}

    valid_ids = {q["id"] for q in _QUESTIONS}

    for qid_str, raw_val in responses.items():
        qid = int(qid_str)
        if qid not in valid_ids:
            continue
        for dim, items in _DIM_ITEMS.items():
            if qid in items:
                clamped = max(_SCALE_MIN, min(_SCALE_MAX, int(raw_val)))
                raw_scores[dim].append(float(clamped))
                break

    scores: dict[str, float] = {}
    for dim in _DIM_ITEMS:
        vals = raw_scores[dim]
        if vals:
            avg = sum(vals) / len(vals)
        else:
            avg = _SCALE_MIN
        scores[dim] = _normalize(avg)

    labeled_scores = {}
    for dim, key in _DIM_LABELS.items():
        labeled_scores[key[lang if lang in key else "en"]] = scores[dim]

    ranked = sorted(scores.items(), key=lambda x: x[1], reverse=True)
    primary_key = ranked[0][0]
    secondary_key = ranked[1][0]

    primary_lang = _DIM_LABELS[primary_key][lang if lang in _DIM_LABELS[primary_key] else "en"]
    secondary_lang = _DIM_LABELS[secondary_key][lang if lang in _DIM_LABELS[secondary_key] else "en"]

    language_ranking = [
        {
            "language": _DIM_LABELS[dim][lang if lang in _DIM_LABELS[dim] else "en"],
            "score": score,
        }
        for dim, score in ranked
    ]

    return {
        "scores": labeled_scores,
        "primary_language": primary_lang,
        "secondary_language": secondary_lang,
        "language_ranking": language_ranking,
    }
