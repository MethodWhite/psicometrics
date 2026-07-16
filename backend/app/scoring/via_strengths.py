import json
from pathlib import Path


_DATA_DIR = Path(__file__).resolve().parent.parent / "data"

with open(_DATA_DIR / "via_strengths_questions.json", encoding="utf-8") as _f:
    _DATA = json.load(_f)

_DIMENSIONS = _DATA["dimensions"]
_QUESTIONS = _DATA["questions"]
_SCALE_MIN = _DATA["scale"]["min"]
_SCALE_MAX = _DATA["scale"]["max"]

_DIM_ITEMS: dict[str, list[int]] = {}
for q in _QUESTIONS:
    _DIM_ITEMS.setdefault(q["dimension"], []).append(q["id"])

_DIM_ORDER = ["wisdom", "courage", "humanity", "justice", "temperance", "transcendence"]

_DIM_LABELS = {
    "wisdom": {"es": "Sabiduría y Conocimiento", "en": "Wisdom and Knowledge"},
    "courage": {"es": "Coraje", "en": "Courage"},
    "humanity": {"es": "Humanidad", "en": "Humanity"},
    "justice": {"es": "Justicia", "en": "Justice"},
    "temperance": {"es": "Templanza", "en": "Temperance"},
    "transcendence": {"es": "Trascendencia", "en": "Transcendence"},
}


def _normalize(raw_avg: float) -> float:
    return round(((raw_avg - _SCALE_MIN) / (_SCALE_MAX - _SCALE_MIN)) * 100, 1)


def score_via_strengths(responses: dict[int | str, int], lang: str = "es") -> dict:
    raw_scores: dict[str, list[float]] = {d: [] for d in _DIM_ITEMS}
    valid_ids = {q["id"] for q in _QUESTIONS}

    for qid_str, raw_val in responses.items():
        qid = int(qid_str)
        if qid not in valid_ids:
            continue
        clamped = max(_SCALE_MIN, min(_SCALE_MAX, int(raw_val)))
        for dim, items in _DIM_ITEMS.items():
            if qid in items:
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

    labeled_scores = {}
    for dim in _DIM_ORDER:
        label = _DIM_LABELS[dim][lang if lang in _DIM_LABELS[dim] else "en"]
        labeled_scores[label] = scores[dim]

    signature_strengths = [
        _DIM_LABELS[dim][lang if lang in _DIM_LABELS[dim] else "en"]
        for dim in _DIM_ORDER
        if scores[dim] >= 70
    ]

    return {
        "scores": labeled_scores,
        "signature_strengths": signature_strengths,
    }
