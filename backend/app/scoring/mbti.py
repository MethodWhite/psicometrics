import json
from pathlib import Path


_DATA_DIR = Path(__file__).resolve().parent.parent / "data"

with open(_DATA_DIR / "mbti_questions.json", encoding="utf-8") as _f:
    _DATA = json.load(_f)

_DICHOTOMIES = _DATA["dichotomies"]
_QUESTIONS = [q for q in _DATA["questions"] if not q.get("attention_check")]

_DICH_KEYS = ["EI", "SN", "TF", "JP"]

_DICH_POLES: dict[str, dict[str, list[int]]] = {}
for q in _QUESTIONS:
    dich = q["dichotomy"]
    pole = q["pole"]
    _DICH_POLES.setdefault(dich, {}).setdefault(pole, []).append(q["id"])

_TYPE_DESCRIPTIONS = {
    "INTJ": {
        "es": "Arquitecto: Estratégico, lógico, planificador. Visionario independiente con altos estándares.",
        "en": "Architect: Strategic, logical, planner. Independent visionary with high standards.",
    },
    "INTP": {
        "es": "Innovador: Analítico, curioso, teórico. Busca comprender los principios subyacentes.",
        "en": "Innovator: Analytical, curious, theoretical. Seeks to understand underlying principles.",
    },
    "ENTJ": {
        "es": "Comandante: Decidido, líder, eficiente. Organiza personas y recursos para lograr metas.",
        "en": "Commander: Decisive, leader, efficient. Organizes people and resources to achieve goals.",
    },
    "ENTP": {
        "es": "Visionario: Innovador, debate, versátil. Genera ideas y desafía el status quo.",
        "en": "Visionary: Innovative, debater, versatile. Generates ideas and challenges the status quo.",
    },
    "INFJ": {
        "es": "Consejero: Idealista, profundo, empático. Busca significado y conexiones auténticas.",
        "en": "Counselor: Idealistic, deep, empathetic. Seeks meaning and authentic connections.",
    },
    "INFP": {
        "es": "Mediador: Creativo, sensible, idealista. Guiado por valores y autenticidad.",
        "en": "Mediator: Creative, sensitive, idealistic. Guided by values and authenticity.",
    },
    "ENFJ": {
        "es": "Protagonista: Carismático, inspirador, cooperativo. Motiva a otros a crecer.",
        "en": "Protagonist: Charismatic, inspiring, cooperative. Motivates others to grow.",
    },
    "ENFP": {
        "es": "Abogado: Entusiasta, creativo, sociable. Explora posibilidades y conecta con otros.",
        "en": "Advocate: Enthusiastic, creative, sociable. Explores possibilities and connects with others.",
    },
    "ISTJ": {
        "es": "Inspector: Responsable, organizado, leal. Cumple con sus deberes y tradiciones.",
        "en": "Inspector: Responsible, organized, loyal. Fulfills duties and traditions.",
    },
    "ISFJ": {
        "es": "Protector: Atento, dedicado, práctico. Cuida de los demás con devoción silenciosa.",
        "en": "Protector: Attentive, dedicated, practical. Cares for others with quiet devotion.",
    },
    "ESTJ": {
        "es": "Supervisor: Eficiente, directo, estructurado. Hace cumplir reglas y mantiene el orden.",
        "en": "Supervisor: Efficient, direct, structured. Enforces rules and maintains order.",
    },
    "ESFJ": {
        "es": "Proveedor: Sociable, generoso, cooperativo. Crea armonía y apoya a los demás.",
        "en": "Provider: Sociable, generous, cooperative. Creates harmony and supports others.",
    },
    "ISTP": {
        "es": "Artesano: Práctico, observador, flexible. Resuelve problemas del mundo real.",
        "en": "Artisan: Practical, observant, flexible. Solves real-world problems.",
    },
    "ISFP": {
        "es": "Compositor: Artístico, sensible, tranquilo. Expresa belleza a través de la acción.",
        "en": "Composer: Artistic, sensitive, quiet. Expresses beauty through action.",
    },
    "ESTP": {
        "es": "Emprendedor: Enérgico, persuasivo, audaz. Actúa rápido y aprovecha oportunidades.",
        "en": "Entrepreneur: Energetic, persuasive, bold. Acts fast and seizes opportunities.",
    },
    "ESFP": {
        "es": "Animador: Espontáneo, entusiasta, sociable. Disfruta la vida y entretiene a otros.",
        "en": "Entertainer: Spontaneous, enthusiastic, sociable. Enjoys life and entertains others.",
    },
}

_DICH_DESCRIPTIONS = {
    "EI": {
        "E": {
            "es": "Extraversión: Te energiza la interacción social. Piensas en voz alta y procesas externamente.",
            "en": "Extraversion: Social interaction energizes you. You think out loud and process externally.",
        },
        "I": {
            "es": "Introversión: Te energiza la soledad. Piensas internamente y procesas antes de hablar.",
            "en": "Introversion: Solitude energizes you. You think internally and process before speaking.",
        },
    },
    "SN": {
        "S": {
            "es": "Sensación: Te enfocas en hechos concretos, detalles y experiencia práctica.",
            "en": "Sensing: You focus on concrete facts, details, and practical experience.",
        },
        "N": {
            "es": "Intuición: Te enfocas en patrones, posibilidades y el panorama general.",
            "en": "Intuition: You focus on patterns, possibilities, and the big picture.",
        },
    },
    "TF": {
        "T": {
            "es": "Pensamiento: Decides con lógica, objetividad y principios universales.",
            "en": "Thinking: You decide with logic, objectivity, and universal principles.",
        },
        "F": {
            "es": "Sentimiento: Decides con valores personales y consideración del impacto emocional.",
            "en": "Feeling: You decide with personal values and consideration of emotional impact.",
        },
    },
    "JP": {
        "J": {
            "es": "Juicio: Prefieres estructura, planificación y resolución. Buscas cierre.",
            "en": "Judging: You prefer structure, planning, and resolution. You seek closure.",
        },
        "P": {
            "es": "Percepción: Prefieres flexibilidad, espontaneidad y mantener opciones abiertas.",
            "en": "Perceiving: You prefer flexibility, spontaneity, and keeping options open.",
        },
    },
}


def score_mbti(responses: dict, lang: str = "es") -> dict:
    questions = _QUESTIONS
    if not questions:
        return _empty_mbti(lang)

    # Count 'a' vs 'b' responses per dichotomy
    dichotomy_responses = {"EI": {"a": 0, "b": 0}, "SN": {"a": 0, "b": 0}, "TF": {"a": 0, "b": 0}, "JP": {"a": 0, "b": 0}}

    for q in questions:
        qid = str(q["id"])
        raw = responses.get(qid) or responses.get(q["id"])
        if raw is None:
            continue
        response = str(raw).lower().strip()
        dichotomy = q.get("dichotomy", "")
        pole = q.get("pole", "")
        if dichotomy in dichotomy_responses:
            if response == "a":
                dichotomy_responses[dichotomy]["a"] += 1
            elif response == "b":
                dichotomy_responses[dichotomy]["b"] += 1
            elif response == pole:  # Direct pole value (E/I/S/N/T/F/J/P)
                dichotomy_responses[dichotomy]["a"] += 1
            elif response in ("e", "i", "s", "n", "t", "f", "j", "p"):
                if response.upper() == pole:
                    dichotomy_responses[dichotomy]["a"] += 1
                else:
                    dichotomy_responses[dichotomy]["b"] += 1

    type_code_parts = []
    scores = {}
    percentages = {}

    for dich in _DICH_KEYS:
        poles = list(_DICH_POLES[dich].keys())
        a_count = dichotomy_responses[dich].get("a", 0)
        b_count = dichotomy_responses[dich].get("b", 0)
        total = a_count + b_count

        if total > 0:
            pct_a = round((a_count / total) * 100, 1)
            pct_b = round((b_count / total) * 100, 1)
        else:
            pct_a = 50.0
            pct_b = 50.0

        if a_count >= b_count:
            winning_pole = poles[0]
            scores[dich] = round(pct_a, 1)
        else:
            winning_pole = poles[1]
            scores[dich] = round(pct_b, 1)

        percentages[dich] = {
            "pole_a": {"code": poles[0], "percentage": pct_a},
            "pole_b": {"code": poles[1], "percentage": pct_b},
        }
        type_code_parts.append(winning_pole)

    type_code = "".join(type_code_parts)
    type_description = _TYPE_DESCRIPTIONS.get(
        type_code,
        {"es": "Tipo no disponible.", "en": "Type not available."},
    )

    profile_parts = []
    for dich in _DICH_KEYS:
        poles = list(_DICH_POLES[dich].keys())
        a = poles[0]
        b = poles[1]
        a_pct = percentages[dich]["pole_a"]["percentage"]
        b_pct = percentages[dich]["pole_b"]["percentage"]
        winner = a if a_pct >= b_pct else b
        desc = _DICH_DESCRIPTIONS[dich][winner][lang if lang in _DICH_DESCRIPTIONS[dich][winner] else "en"]
        profile_parts.append(f"{winner} ({max(a_pct, b_pct)}%): {desc}")

    profile_summary = "\n".join(profile_parts)

    type_names = {
        "es": {
            "INTJ": "Arquitecto", "INTP": "Innovador", "ENTJ": "Comandante", "ENTP": "Visionario",
            "INFJ": "Consejero", "INFP": "Mediador", "ENFJ": "Protagonista", "ENFP": "Abogado",
            "ISTJ": "Inspector", "ISFJ": "Protector", "ESTJ": "Supervisor", "ESFJ": "Proveedor",
            "ISTP": "Artesano", "ISFP": "Compositor", "ESTP": "Emprendedor", "ESFP": "Animador",
        },
        "en": {
            "INTJ": "Architect", "INTP": "Innovator", "ENTJ": "Commander", "ENTP": "Visionary",
            "INFJ": "Counselor", "INFP": "Mediator", "ENFJ": "Protagonist", "ENFP": "Advocate",
            "ISTJ": "Inspector", "ISFJ": "Protector", "ESTJ": "Supervisor", "ESFJ": "Provider",
            "ISTP": "Artisan", "ISFP": "Composer", "ESTP": "Entrepreneur", "ESFP": "Entertainer",
        },
    }

    type_name = type_names.get(lang, type_names["en"]).get(type_code, type_code)

    recommendations = {
        "es": [
            f"Como {type_name} ({type_code}), aprovecha tu tendencia natural a procesar información de esta forma.",
            "Busca entornos que respeten tu estilo de comunicación y toma de decisiones.",
            "Desarrolla estrategias para adaptarte a estilos opuestos al tuyo cuando sea necesario.",
            "Reflexiona sobre cómo tus preferencias afectan tus relaciones y carrera profesional.",
        ],
        "en": [
            f"As a {type_name} ({type_code}), leverage your natural tendency to process information this way.",
            "Seek environments that respect your communication and decision-making style.",
            "Develop strategies to adapt to opposite styles when necessary.",
            "Reflect on how your preferences affect your relationships and career.",
        ],
    }

    return {
        "type_code": type_code,
        "type_name": type_name,
        "type_description": type_description[lang if lang in type_description else "en"],
        "scores": scores,
        "percentages": percentages,
        "profile_summary": profile_summary,
        "interpretation": type_description[lang if lang in type_description else "en"],
        "recommendations": recommendations[lang if lang in recommendations else "en"],
    }
