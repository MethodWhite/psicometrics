import json
from pathlib import Path


_DATA_DIR = Path(__file__).resolve().parent.parent / "data"

with open(_DATA_DIR / "attachment_style_questions.json", encoding="utf-8") as _f:
    _DATA = json.load(_f)

_DIMENSIONS = _DATA["dimensions"]
_QUESTIONS = _DATA["questions"]
_SCALE_MIN = _DATA["scale"]["min"]
_SCALE_MAX = _DATA["scale"]["max"]

_DIM_ITEMS: dict[str, list[int]] = {}
for q in _QUESTIONS:
    _DIM_ITEMS.setdefault(q["dimension"], []).append(q["id"])

_DIM_ORDER = ["secure", "anxious", "avoidant"]

_DIM_LABELS = {
    "secure": {"es": "Apego Seguro", "en": "Secure Attachment"},
    "anxious": {"es": "Apego Ansioso", "en": "Anxious Attachment"},
    "avoidant": {"es": "Apego Evitativo", "en": "Avoidant Attachment"},
}


def _normalize(raw_avg: float) -> float:
    return round(((raw_avg - _SCALE_MIN) / (_SCALE_MAX - _SCALE_MIN)) * 100, 1)


def score_attachment_style(responses: dict[int | str, int], lang: str = "es") -> dict:
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
    for dim in _DIM_ORDER:
        vals = raw_scores.get(dim, [])
        if vals:
            avg = sum(vals) / len(vals)
        else:
            avg = _SCALE_MIN
        scores[dim] = _normalize(avg)

    primary_dim = max(_DIM_ORDER, key=lambda d: scores[d])

    labeled_scores = {}
    for dim in _DIM_ORDER:
        label = _DIM_LABELS[dim][lang if lang in _DIM_LABELS[dim] else "en"]
        labeled_scores[label] = scores[dim]

    primary_label = _DIM_LABELS[primary_dim][lang if lang in _DIM_LABELS[primary_dim] else "en"]

    style_descriptions = {
        "secure": {
            "es": "Te sientes cómodo/a con la intimidad y la independencia. Confías en los demás y tienes relaciones estables y saludables.",
            "en": "You are comfortable with intimacy and independence. You trust others and have stable, healthy relationships.",
        },
        "anxious": {
            "es": "Buscas mucha cercanía y seguridad en tus relaciones. Puedes preocuparte por el rechazo o el abandono.",
            "en": "You seek a lot of closeness and reassurance in relationships. You may worry about rejection or abandonment.",
        },
        "avoidant": {
            "es": "Valoras tu independencia y puedes sentirte incómodo/a con demasiada intimidad. Tiendes a mantener distancia emocional.",
            "en": "You value your independence and may feel uncomfortable with too much intimacy. You tend to maintain emotional distance.",
        },
    }

    profile_summary = (
        f"{primary_label}: {scores[primary_dim]}/100 — "
        f"{style_descriptions[primary_dim][lang if lang in style_descriptions[primary_dim] else 'en']}"
    )

    return {
        "scores": labeled_scores,
        "primary_style": primary_label,
        "profile_summary": profile_summary,
    }
