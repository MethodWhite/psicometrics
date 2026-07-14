"""Dark Triad SD3 Scoring Algorithm.

Short Dark Triad (SD3) — Jones & Paulhus (2014).
27 items, 9 per trait, Likert 1-5 scale.
Traits: Machiavellianism (M), Narcissism (N), Psychopathy (P).
"""

from app.data_loader import load_questions


TRAIT_NAMES = {
    "M": {"es": "Maquiavelismo", "en": "Machiavellianism"},
    "N": {"es": "Narcisismo", "en": "Narcissism"},
    "P": {"es": "Psicopatía", "en": "Psychopathy"},
}


def score_dark_triad(answers: dict[int, int], language: str = "es") -> dict:
    """Score Dark Triad test from question answers.

    Args:
        answers: dict mapping question_id (1-27) to value (1-5)
        language: 'es' or 'en'

    Returns:
        dict with scores (0-100 per trait), percentile equivalents, profile_summary, risk_level
    """
    questions = load_questions("dark_triad")

    # Accumulate raw scores per trait
    trait_raw: dict[str, list[float]] = {"M": [], "N": [], "P": []}

    for q in questions["questions"]:
        qid = q["id"]
        if qid not in answers:
            continue

        value = float(answers[qid])
        trait = q["trait"]
        trait_raw[trait].append(value)

    # Calculate normalized scores (0-100) per trait
    trait_scores = {}
    for trait in ["M", "N", "P"]:
        values = trait_raw[trait]
        if values:
            mean = sum(values) / len(values)
            # Normalize: 1-5 scale -> 0-100
            trait_scores[trait] = round((mean - 1) / 4 * 100, 1)
        else:
            trait_scores[trait] = 0.0

    # Calculate composite dark triad score
    dark_core = round(sum(trait_scores.values()) / 3, 1)

    # Determine risk level
    risk_level = _determine_risk_level(trait_scores, dark_core)

    # Generate profile summary
    summary = _generate_dark_triad_summary(trait_scores, risk_level, language)

    return {
        "scores": trait_scores,
        "dark_core": dark_core,
        "risk_level": risk_level,
        "profile_summary": summary,
    }


def _determine_risk_level(scores: dict, dark_core: float) -> str:
    """Determine risk level based on scores."""
    if dark_core >= 75 or any(s >= 80 for s in scores.values()):
        return "high"
    elif dark_core >= 50 or any(s >= 60 for s in scores.values()):
        return "moderate"
    elif dark_core >= 25:
        return "low"
    else:
        return "minimal"


def _generate_dark_triad_summary(scores: dict, risk_level: str, language: str) -> str:
    if language == "es":
        return _summary_es(scores, risk_level)
    return _summary_en(scores, risk_level)


def _summary_es(scores: dict, risk_level: str) -> str:
    parts = []
    m, n, p = scores.get("M", 0), scores.get("N", 0), scores.get("P", 0)

    if m >= 60:
        parts.append("Mostrás tendencias maquiavélicas significativas: manipulación estratégica, cinismo moral y visión desconfiada de los demás.")
    elif m >= 40:
        parts.append("Tenés un nivel moderado de maquiavelismo: podés ser estratégico/a en situaciones sociales pero sin tendencias manipulativas extremas.")
    else:
        parts.append("Mostrás bajos niveles de maquiavelismo: sos directo/a y honesto/a en tus interacciones.")

    if n >= 60:
        parts.append("Mostrás narcisismo elevado: grandiosidad, necesidad de admiración y sentimiento de entitledness.")
    elif n >= 40:
        parts.append("Tenés un nivel moderado de narcisismo: confianza saludable en ti mismo/a sin tendencias grandiosas extremas.")
    else:
        parts.append("Mostrás bajos niveles de narcisismo: humilde y con autoestima realista.")

    if p >= 60:
        parts.append("Mostrás rasgos psicopáticos significativos: impulsividad, falta de empatía y desapego social.")
    elif p >= 40:
        parts.append("Tenés un nivel moderado de psicopatía: podés ser frío/a en situaciones que lo requieren pero con capacidad de empatía.")
    else:
        parts.append("Mostrás bajos niveles de psicopatía: empático/a y con conciencia social.")

    risk_map = {
        "high": "Tu perfil indica un nivel ALTO de rasgos oscuros. Se recomienda auto-reflexión y, si es posible, consulta profesional.",
        "moderate": "Tu perfil indica un nivel MODERADO de rasgos oscuros. Algunos rasgos están elevados pero no en rangos preocupantes.",
        "low": "Tu perfil indica un nivel BAJO de rasgos oscuros. Tus interacciones sociales son mayormente saludables.",
        "minimal": "Tu perfil indica un nivel MÍNIMO de rasgos oscuros. Mostrás alta empatía y prosocialidad.",
    }
    parts.append(risk_map.get(risk_level, ""))

    return " ".join(parts)


def _summary_en(scores: dict, risk_level: str) -> str:
    parts = []
    m, n, p = scores.get("M", 0), scores.get("N", 0), scores.get("P", 0)

    if m >= 60:
        parts.append("You show significant Machiavellian tendencies: strategic manipulation, moral cynicism, and distrustful view of others.")
    elif m >= 40:
        parts.append("You have moderate Machiavellianism: you can be strategic in social situations without extreme manipulative tendencies.")
    else:
        parts.append("You show low Machiavellianism: you are direct and honest in your interactions.")

    if n >= 60:
        parts.append("You show high narcissism: grandiosity, need for admiration, and sense of entitlement.")
    elif n >= 40:
        parts.append("You have moderate narcissism: healthy self-confidence without extreme grandiose tendencies.")
    else:
        parts.append("You show low narcissism: humble with realistic self-esteem.")

    if p >= 60:
        parts.append("You show significant psychopathic traits: impulsivity, lack of empathy, and social detachment.")
    elif p >= 40:
        parts.append("You have moderate psychopathy: you can be cold in situations that require it but with capacity for empathy.")
    else:
        parts.append("You show low psychopathy: empathetic and socially aware.")

    risk_map = {
        "high": "Your profile indicates a HIGH level of dark traits. Self-reflection and, if possible, professional consultation is recommended.",
        "moderate": "Your profile indicates a MODERATE level of dark traits. Some traits are elevated but not in concerning ranges.",
        "low": "Your profile indicates a LOW level of dark traits. Your social interactions are mostly healthy.",
        "minimal": "Your profile indicates a MINIMAL level of dark traits. You show high empathy and prosociality.",
    }
    parts.append(risk_map.get(risk_level, ""))

    return " ".join(parts)
