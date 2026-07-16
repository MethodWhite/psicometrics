import json
import math
from pathlib import Path


_DATA_DIR = Path(__file__).resolve().parent.parent / "data"

with open(_DATA_DIR / "disc_questions.json", encoding="utf-8") as _f:
    _DATA = json.load(_f)

_DIMENSIONS_INFO = _DATA["dimensions"]
_QUESTIONS = [q for q in _DATA["questions"] if not q.get("attention_check")]
_SCALE_MIN = _DATA["scale"]["min"] if "scale" in _DATA else 1
_SCALE_MAX = _DATA["scale"]["max"] if "scale" in _DATA else 5

_DIM_ITEMS: dict[str, list[int]] = {}
for q in _QUESTIONS:
    _DIM_ITEMS.setdefault(q["dimension"], []).append(q["id"])

_DIM_ORDER = ["D", "I", "S", "C"]

_DIM_LABELS = {
    "D": {
        "es": "Dominancia",
        "desc_es": "Enfocado en resultados, directo, decidido",
        "en": "Dominance",
        "desc_en": "Results-focused, direct, decisive",
    },
    "I": {
        "es": "Influencia",
        "desc_es": "Entusiasta, optimista, sociable",
        "en": "Influence",
        "desc_en": "Enthusiastic, optimistic, sociable",
    },
    "S": {
        "es": "Estabilidad",
        "desc_es": "Paciente, confiable, cooperador",
        "en": "Steadiness",
        "desc_en": "Patient, reliable, cooperative",
    },
    "C": {
        "es": "Conciencia",
        "desc_es": "Analítico, preciso, sistemático",
        "en": "Conscientiousness",
        "desc_en": "Analytical, precise, systematic",
    },
}

_STYLE_DESCRIPTIONS = {
    "D": {
        "es": "Las personas con alta Dominancia son directas, decididas y orientadas a resultados. Les gusta tomar el control y enfrentar desafíos.",
        "en": "People with high Dominance are direct, decisive, and results-oriented. They like taking control and facing challenges.",
    },
    "I": {
        "es": "Las personas con alta Influencia son entusiastas, optimistas y sociables. Disfrutan conocer gente y persuadir a otros.",
        "en": "People with high Influence are enthusiastic, optimistic, and sociable. They enjoy meeting people and persuading others.",
    },
    "S": {
        "es": "Las personas con alta Estabilidad son pacientes, confiables y cooperadoras. Valoran la armonía y la consistencia.",
        "en": "People with high Steadiness are patient, reliable, and cooperative. They value harmony and consistency.",
    },
    "C": {
        "es": "Las personas con alta Conciencia son analíticas, precisas y sistemáticas. Valoran la calidad y siguen procedimientos.",
        "en": "People with high Conscientiousness are analytical, precise, and systematic. They value quality and follow procedures.",
    },
}

_DISC_COMBOS = {
    ("D", "I"): {
        "es": "El Retador: Combina la determinación de la Dominancia con el carisma de la Influencia. Lidera inspirando y exigiendo resultados.",
        "en": "The Challenger: Combines Dominance determination with Influence charisma. Leads by inspiring and demanding results.",
    },
    ("D", "S"): {
        "es": "El Ejecutor: Mezcla la firmeza de la Dominancia con la paciencia de la Estabilidad. Lidera con autoridad calmada.",
        "en": "The Executor: Blends Dominance firmness with Steadiness patience. Leads with calm authority.",
    },
    ("D", "C"): {
        "es": "El Estratega: Une la decisión de la Dominancia con la precisión de la Conciencia. Planifica meticulosamente y ejecuta con determinación.",
        "en": "The Strategist: Unites Dominance decisiveness with Conscientiousness precision. Plans meticulously and executes with determination.",
    },
    ("I", "D"): {
        "es": "El Motivador: Combina el entusiasmo de la Influencia con la energía de la Dominancia. Inspira equipos hacia metas ambiciosas.",
        "en": "The Motivator: Combines Influence enthusiasm with Dominance energy. Inspires teams toward ambitious goals.",
    },
    ("I", "S"): {
        "es": "El Armonizador: Mezcla la sociabilidad de la Influencia con la calidez de la Estabilidad. Construye relaciones y mantiene la armonía.",
        "en": "The Harmonizer: Blends Influence sociability with Steadiness warmth. Builds relationships and maintains harmony.",
    },
    ("I", "C"): {
        "es": "El Comunicador: Une la expresividad de la Influencia con el rigor de la Conciencia. Comunica ideas complejas con claridad.",
        "en": "The Communicator: Unites Influence expressiveness with Conscientiousness rigor. Communicates complex ideas clearly.",
    },
    ("S", "D"): {
        "es": "El Protector: Combina la lealtad de la Estabilidad con la fuerza de la Dominancia. Defiende a su equipo con determinación.",
        "en": "The Protector: Combines Steadiness loyalty with Dominance strength. Defends their team with determination.",
    },
    ("S", "I"): {
        "es": "El Colaborador: Mezcla la cooperación de la Estabilidad con el optimismo de la Influencia. Trabaja en equipo con entusiasmo.",
        "en": "The Collaborator: Blends Steadiness cooperation with Influence optimism. Works in teams with enthusiasm.",
    },
    ("S", "C"): {
        "es": "El Analista: Une la consistencia de la Estabilidad con el rigor de la Conciencia. Trabaja con precisión y fiabilidad.",
        "en": "The Analyst: Unites Steadiness consistency with Conscientiousness rigor. Works with precision and reliability.",
    },
    ("C", "D"): {
        "es": "El Director: Combina el análisis de la Conciencia con la decisión de la Dominancia. Toma decisiones informadas y firmes.",
        "en": "The Director: Combines Conscientiousness analysis with Dominance decisiveness. Makes informed and firm decisions.",
    },
    ("C", "I"): {
        "es": "El Innovador: Mezcla la precisión de la Conciencia con la creatividad de la Influencia. Desarrolla soluciones novedosas y precisas.",
        "en": "The Innovator: Blends Conscientiousness precision with Influence creativity. Develops novel and precise solutions.",
    },
    ("C", "S"): {
        "es": "El Especialista: Une el análisis de la Conciencia con la paciencia de la Estabilidad. Domina su área con profundidad.",
        "en": "The Specialist: Unites Conscientiousness analysis with Steadiness patience. Masters their area with depth.",
    },
}


def _normalize(raw_avg: float) -> float:
    return round(((raw_avg - _SCALE_MIN) / (_SCALE_MAX - _SCALE_MIN)) * 100, 1)


def score_disc(responses: dict, lang: str = "es") -> dict:
    questions = _QUESTIONS
    counts: dict[str, int] = {d: 0 for d in ["D", "I", "S", "C"]}

    for q in questions:
        qid = str(q["id"])
        raw = responses.get(qid) or responses.get(q["id"])
        if raw is None:
            continue
        # Map response letter to dimension
        response = str(raw).lower().strip()
        dim_map = {"a": "D", "b": "I", "c": "S", "d": "C"}
        dim = dim_map.get(response)
        if dim:
            counts[dim] += 1

    total = sum(counts.values())
    scores: dict[str, float] = {}
    for dim in ["D", "I", "S", "C"]:
        pct = (counts[dim] / total * 100) if total > 0 else 25.0
        scores[dim] = round(pct, 1)

    ranked = sorted(["D", "I", "S", "C"], key=lambda d: scores[d], reverse=True)
    primary = ranked[0]
    secondary = ranked[1]

    labeled_scores = {}
    for dim in _DIM_ORDER:
        info = _DIM_LABELS[dim]
        label = info[lang if lang in info else "en"]
        desc = info.get(f"desc_{lang}", info.get("desc_en", ""))
        labeled_scores[label] = {"score": scores[dim], "description": desc}

    primary_info = _DIM_LABELS[primary]
    secondary_info = _DIM_LABELS[secondary]
    primary_label = primary_info[lang if lang in primary_info else "en"]
    secondary_label = secondary_info[lang if lang in secondary_info else "en"]

    combo = (primary, secondary)
    combo_desc = _DISC_COMBOS.get(
        combo,
        _DISC_COMBOS.get((secondary, primary), {"es": "", "en": ""}),
    )
    style_description = combo_desc[lang if lang in combo_desc else "en"]

    profile_summary = (
        f"{primary_label} / {secondary_label}: "
        f"{_STYLE_DESCRIPTIONS[primary][lang if lang in _STYLE_DESCRIPTIONS[primary] else 'en']} "
        + style_description
    )

    recommendations = {
        "es": [
            f"Aprovecha tu estilo {primary_label} en situaciones que requieran {_DIM_LABELS[primary].get('desc_es', '')}.",
            f"Complementa con tu estilo {secondary_label} para equilibrar tu perfil.",
            "Identifica situaciones donde puedas flexibilizar tu estilo natural.",
            "Comunícate considerando los estilos DISC de las personas con quienes trabajas.",
        ],
        "en": [
            f"Leverage your {primary_label} style in situations requiring {_DIM_LABELS[primary].get('desc_en', '')}.",
            f"Complement with your {secondary_label} style to balance your profile.",
            "Identify situations where you can flex beyond your natural style.",
            "Communicate considering the DISC styles of people you work with.",
        ],
    }

    return {
        "primary_style": primary_label,
        "secondary_style": secondary_label,
        "scores": labeled_scores,
        "profile_summary": profile_summary,
        "style_description": style_description,
        "interpretation": style_description,
        "recommendations": recommendations[lang if lang in recommendations else "en"],
    }
