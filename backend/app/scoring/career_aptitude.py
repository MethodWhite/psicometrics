import json
from pathlib import Path


_DATA_DIR = Path(__file__).resolve().parent.parent / "data"

with open(_DATA_DIR / "career_aptitude_questions.json", encoding="utf-8") as _f:
    _DATA = json.load(_f)

_DIMENSIONS = _DATA["dimensions"]
_QUESTIONS = _DATA["questions"]
_SCALE_MIN = _DATA["scale"]["min"]
_SCALE_MAX = _DATA["scale"]["max"]

_DIM_ITEMS: dict[str, list[int]] = {}
for q in _QUESTIONS:
    _DIM_ITEMS.setdefault(q["dimension"], []).append(q["id"])

_DIM_KEYS = ["realistic", "investigative", "artistic", "social", "enterprising", "conventional"]

_RIASEC_CODES = {
    "realistic": "R",
    "investigative": "I",
    "artistic": "A",
    "social": "S",
    "enterprising": "E",
    "conventional": "C",
}

_DIM_LABELS = {
    "realistic": {"es": "Realista (R)", "en": "Realistic (R)"},
    "investigative": {"es": "Investigador (I)", "en": "Investigative (I)"},
    "artistic": {"es": "Artístico (A)", "en": "Artistic (A)"},
    "social": {"es": "Social (S)", "en": "Social (S)"},
    "enterprising": {"es": "Emprendedor (E)", "en": "Enterprising (E)"},
    "conventional": {"es": "Convencional (C)", "en": "Conventional (C)"},
}


def _normalize(raw_avg: float) -> float:
    return round(((raw_avg - _SCALE_MIN) / (_SCALE_MAX - _SCALE_MIN)) * 100, 1)


def score_career_aptitude(responses: dict[int | str, int], lang: str = "es") -> dict:
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
    for dim in _DIM_KEYS:
        vals = raw_scores.get(dim, [])
        if vals:
            avg = sum(vals) / len(vals)
        else:
            avg = _SCALE_MIN
        scores[dim] = _normalize(avg)

    ranked = sorted(_DIM_KEYS, key=lambda d: scores[d], reverse=True)
    top_three = ranked[:3]
    holland_code = "".join(_RIASEC_CODES[d] for d in top_three)

    labeled_scores = {}
    for dim in _DIM_KEYS:
        label = _DIM_LABELS[dim][lang if lang in _DIM_LABELS[dim] else "en"]
        labeled_scores[label] = scores[dim]

    career_descriptions = {
        ("R", "I", "A"): {
            "es": "Carreras en arquitectura, diseño industrial, ingeniería creativa.",
            "en": "Careers in architecture, industrial design, creative engineering.",
        },
        ("R", "I", "S"): {
            "es": "Carreras en educación técnica, terapia ocupacional, enfermería.",
            "en": "Careers in technical education, occupational therapy, nursing.",
        },
        ("R", "I", "E"): {
            "es": "Carreras en gestión de proyectos técnicos, ingeniería comercial.",
            "en": "Careers in technical project management, commercial engineering.",
        },
        ("R", "I", "C"): {
            "es": "Carreras en ingeniería de calidad, topografía, agricultura técnica.",
            "en": "Careers in quality engineering, surveying, technical agriculture.",
        },
        ("R", "A", "S"): {
            "es": "Carreras en arteterapia, educación artística, diseño inclusivo.",
            "en": "Careers in art therapy, art education, inclusive design.",
        },
        ("R", "A", "E"): {
            "es": "Carreras en arquitectura paisajista, desarrollo inmobiliario creativo.",
            "en": "Careers in landscape architecture, creative real estate development.",
        },
        ("R", "A", "C"): {
            "es": "Carreras en diseño técnico, dibujo arquitectónico, cartografía.",
            "en": "Careers in technical design, architectural drafting, cartography.",
        },
        ("R", "S", "E"): {
            "es": "Carreras en gestión deportiva, entrenamiento personal, liderazgo al aire libre.",
            "en": "Careers in sports management, personal training, outdoor leadership.",
        },
        ("R", "S", "C"): {
            "es": "Carreras en seguridad laboral, inspección técnica, logística.",
            "en": "Careers in occupational safety, technical inspection, logistics.",
        },
        ("R", "E", "C"): {
            "es": "Carreras en gestión de operaciones, cadena de suministro, construcción.",
            "en": "Careers in operations management, supply chain, construction.",
        },
        ("I", "A", "S"): {
            "es": "Carreras en psicología, terapia, investigación social, trabajo social clínico.",
            "en": "Careers in psychology, therapy, social research, clinical social work.",
        },
        ("I", "A", "E"): {
            "es": "Carreras en consultoría estratégica, innovación, desarrollo de productos.",
            "en": "Careers in strategic consulting, innovation, product development.",
        },
        ("I", "A", "C"): {
            "es": "Carreras en análisis de datos, investigación científica, laboratorio.",
            "en": "Careers in data analysis, scientific research, laboratory work.",
        },
        ("I", "S", "A"): {
            "es": "Carreras en medicina, psicología clínica, investigación biomédica.",
            "en": "Careers in medicine, clinical psychology, biomedical research.",
        },
        ("I", "S", "E"): {
            "es": "Carreras en dirección de proyectos de investigación, gestión de innovación.",
            "en": "Careers in research project management, innovation management.",
        },
        ("I", "E", "C"): {
            "es": "Carreras en finanzas cuantitativas, análisis de negocios, consultoría.",
            "en": "Careers in quantitative finance, business analysis, consulting.",
        },
        ("A", "S", "E"): {
            "es": "Carreras en relaciones públicas, marketing creativo, dirección de arte.",
            "en": "Careers in public relations, creative marketing, art direction.",
        },
        ("A", "S", "I"): {
            "es": "Carreras en comunicación, periodismo, docencia en artes.",
            "en": "Careers in communication, journalism, arts education.",
        },
        ("S", "E", "A"): {
            "es": "Carreras en recursos humanos, formación corporativa, coaching.",
            "en": "Careers in human resources, corporate training, coaching.",
        },
        ("S", "E", "C"): {
            "es": "Carreras en administración educativa, gestión de ONG, servicio público.",
            "en": "Careers in educational administration, NGO management, public service.",
        },
        ("E", "C", "S"): {
            "es": "Carreras en gestión empresarial, administración, banca, seguros.",
            "en": "Careers in business management, administration, banking, insurance.",
        },
        ("E", "C", "I"): {
            "es": "Carreras en finanzas corporativas, consultoría de gestión, inversiones.",
            "en": "Careers in corporate finance, management consulting, investments.",
        },
    }

    code_tuple = tuple(holland_code)
    career_path = career_descriptions.get(
        code_tuple,
        {"es": "Explora carreras que combinen tus intereses principales.", "en": "Explore careers combining your top interests."},
    )

    return {
        "scores": labeled_scores,
        "holland_code": holland_code,
        "career_suggestion": career_path[lang if lang in career_path else "en"],
    }
