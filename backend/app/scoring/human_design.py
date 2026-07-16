import json
import math
from datetime import date, datetime
from pathlib import Path


_DATA_DIR = Path(__file__).resolve().parent.parent / "data"

with open(_DATA_DIR / "human_design_data.json", encoding="utf-8") as _f:
    _DATA = json.load(_f)

_TYPES = _DATA["types"]
_AUTHORITIES = _DATA["authorities"]
_PROFILES = _DATA["profiles"]
_CENTERS = _DATA["centers"]

# 64 gates of the I Ching, each spanning 5.625 degrees of the ecliptic (360/64)
# Gate 1 starts at approximately 0° Aries (spring equinox, ~March 21)
# Each gate spans ~5.625 degrees (~5.6 days)
_GATES = list(range(1, 65))
_GATE_DEGREES = 360.0 / 64.0  # 5.625


def _sun_longitude(birth_date: date) -> float:
    """Calculate approximate ecliptic longitude of the Sun for a given date.
    Uses a simplified astronomical approximation.

    Returns degrees (0-360).
    """
    # Day of year
    doy = birth_date.timetuple().tm_yday
    # Approximate solar longitude (vernal equinox ~March 20 = day 79)
    # Solar longitude = 0 at vernal equinox
    # Earth's orbit is ~365.25 days
    # Mean anomaly (degrees)
    mean_anomaly = (doy - 79) * (360.0 / 365.25)
    # Simplified equation of center (eccentricity ~0.0167)
    # True anomaly = mean anomaly + equation of center
    eq_center = 1.915 * math.sin(math.radians(mean_anomaly)) + 0.02 * math.sin(
        math.radians(2 * mean_anomaly)
    )
    true_longitude = mean_anomaly + eq_center
    return true_longitude % 360.0


def _get_gate(sun_lon: float) -> int:
    """Determine which of the 64 gates the Sun is in based on ecliptic longitude."""
    gate_num = int(sun_lon / _GATE_DEGREES) + 1
    return min(gate_num, 64)


def _get_earth_gate(sun_lon: float) -> int:
    """Earth is opposite the Sun (180 degrees away)."""
    earth_lon = (sun_lon + 180.0) % 360.0
    gate = int(earth_lon / _GATE_DEGREES) + 1
    return min(gate, 64)


def _determine_type(birth_hour: float, sun_gate: int, earth_gate: int) -> str:
    """Simplified type determination based on birth characteristics.

    In Human Design, type is determined by which centers are defined (colored in).
    This simplified version uses birth time and gate position as heuristic.
    """
    # This is a simplified approximation. Full HD calculation requires
    # ephemeris data for all planets at birth and 88-89 days before birth.
    # For educational purposes, we use a deterministic function.

    # Sacral defined: Generator types (Generators + Manifesting Generators)
    # Throat connected to motor: Manifestor
    # No motors defined: Projector
    # All centers undefined (open): Reflector

    # Heuristic: use birth hour modulo
    hour_factor = (birth_hour * 7 + sun_gate + earth_gate) % 100

    if hour_factor < 35:
        return "Generator"
    if hour_factor < 50:
        return "Manifesting Generator"
    if hour_factor < 70:
        return "Manifestor"
    if hour_factor < 90:
        return "Projector"
    return "Reflector"


def _determine_authority(human_type: str, birth_hour: float) -> str:
    """Simplified authority determination based on type and birth data."""
    if human_type == "Generator":
        return "Sacral"
    if human_type == "Manifesting Generator":
        return "Emotional" if (birth_hour % 2 < 1) else "Sacral"
    if human_type == "Manifestor":
        if birth_hour % 3 < 1:
            return "Emotional"
        if birth_hour % 3 < 2:
            return "Splenic"
        return "Ego"
    if human_type == "Projector":
        return "Splenic"
    return "Lunar"


def _determine_profile(
    birth_hour: float, sun_gate: int, human_type: str
) -> str:
    """Simplified profile determination.

    Profile = conscious + unconscious hexagram lines.
    The 6 lines represent different archetypes.
    """
    conscious_line = max(1, min(6, (sun_gate % 6) + 1))

    type_hash = sum(ord(c) for c in human_type)
    unconscious_line = max(1, min(6, (type_hash + int(birth_hour)) % 6 + 1))

    profile_key = f"{conscious_line}/{unconscious_line}"

    # Common profiles: 1/3, 1/4, 2/4, 2/5, 3/5, 3/6, 4/6, 4/1, 5/1, 5/2, 6/2, 6/3
    valid_profiles = [
        "1/3", "1/4", "2/4", "2/5", "3/5", "3/6",
        "4/6", "4/1", "5/1", "5/2", "6/2", "6/3",
    ]
    if profile_key in valid_profiles:
        return profile_key
    # Fall back to closest valid profile
    return valid_profiles[(sun_gate - 1) % len(valid_profiles)]


def _determine_centers(
    human_type: str, birth_hour: float, sun_gate: int
) -> dict[str, bool]:
    """Simplified center definition (defined/undefined).

    Defined centers are 'colored in' in the Human Design chart.
    """
    # Different types have different centers defined
    if human_type == "Generator":
        defined = {"sacral": True, "throat": sun_gate % 2 == 0, "g": True}
    elif human_type == "Manifesting Generator":
        defined = {"sacral": True, "throat": True, "g": True, "solar_plexus": birth_hour % 2 < 1}
    elif human_type == "Manifestor":
        defined = {"throat": True, "heart": birth_hour % 3 < 1, "splenic": birth_hour % 3 < 2}
    elif human_type == "Projector":
        defined = {"splenic": True, "head": birth_hour % 2 < 1, "ajna": True}
    else:  # Reflector
        defined = {}

    # Add some additional centers based on gates
    if sun_gate % 3 == 0:
        defined["root"] = True
    if sun_gate % 5 == 0:
        defined["solar_plexus"] = True

    return defined


def score_human_design(
    birth_data: dict, lang: str = "es"
) -> dict:
    """Calculate Human Design profile from birth data.

    birth_data should contain:
        - date (str): YYYY-MM-DD or ISO date
        - time (str): HH:MM in 24h format
        - location (str): birth location name

    Returns a simplified Human Design chart.
    """
    date_str = birth_data.get("date", "")
    time_str = birth_data.get("time", "12:00")

    try:
        bd = date.fromisoformat(date_str) if date_str else date(2000, 1, 1)
    except (ValueError, TypeError):
        bd = date(2000, 1, 1)

    try:
        parts = time_str.split(":")
        birth_hour = int(parts[0]) + int(parts[1]) / 60.0
    except (ValueError, IndexError, TypeError):
        birth_hour = 12.0

    sun_lon = _sun_longitude(bd)
    sun_gate = _get_gate(sun_lon)
    earth_gate = _get_earth_gate(sun_lon)

    human_type = _determine_type(birth_hour, sun_gate, earth_gate)
    authority = _determine_authority(human_type, birth_hour)
    profile = _determine_profile(birth_hour, sun_gate, human_type)
    centers = _determine_centers(human_type, birth_hour, sun_gate)

    type_info = _TYPES.get(human_type, _TYPES["Generator"])
    type_data = type_info.get(lang, type_info["en"]) if isinstance(type_info.get(lang), dict) else type_info["en"]
    if isinstance(type_data, str):
        type_name = type_data
        strategy = type_info["en"]["strategy"]
        not_self = type_info["en"].get("not_self", "")
        signature = type_info["en"].get("signature", "")
        type_desc = type_info["en"].get("description", "")
    else:
        type_name = type_data.get("name", human_type)
        strategy = type_data.get("strategy", "")
        not_self = type_data.get("not_self", "")
        signature = type_data.get("signature", "")
        type_desc = type_data.get("description", "")

    authority_info = _AUTHORITIES.get(authority, _AUTHORITIES["Sacral"])
    authority_name = authority_info.get(lang, authority_info["en"]) if isinstance(authority_info.get(lang), str) else authority_info["en"]
    if isinstance(authority_name, str):
        authority_desc = authority_info.get("description", {}).get(lang, authority_info.get("description", {}).get("en", ""))
    else:
        authority_name = authority_info.get(lang, authority_info["en"])
        authority_desc = authority_info.get("description", {}).get(lang, authority_info["description"]["en"])

    profile_info = _PROFILES.get(profile, _PROFILES["1/3"])
    profile_name = profile_info.get(lang, profile_info["en"]) if isinstance(profile_info.get(lang), str) else profile_info["en"]
    if isinstance(profile_name, str):
        profile_desc = profile_info.get("description", {}).get(lang, profile_info.get("description", {}).get("en", ""))
    else:
        profile_name = profile_info.get(lang, profile_info["en"])
        profile_desc = profile_info.get("description", {}).get(lang, profile_info["description"]["en"])

    centers_result = {}
    for center_key, center_info in _CENTERS.items():
        is_defined = centers.get(center_key, False)
        center_name = center_info.get(lang, center_info["en"])
        question = center_info.get("question", {}).get(lang, center_info.get("question", {}).get("en", ""))
        centers_result[center_key] = {
            "name": center_name,
            "defined": is_defined,
            "question": question,
        }

    summary_parts = [
        f"{'Tipo' if lang == 'es' else 'Type'}: {type_name}",
        f"{'Estrategia' if lang == 'es' else 'Strategy'}: {strategy}",
        f"{'Autoridad' if lang == 'es' else 'Authority'}: {authority_name}",
        f"{'Perfil' if lang == 'es' else 'Profile'}: {profile_name}",
        f"{'Firma' if lang == 'es' else 'Signature'}: {signature}",
        f"{'No-Self' if lang == 'es' else 'Not-Self theme'}: {not_self}",
        "",
        type_desc,
        "",
        f"{'Centros definidos' if lang == 'es' else 'Defined centers'}:",
    ]
    for ck, ci in centers_result.items():
        if ci["defined"]:
            summary_parts.append(f"  - {ci['name']}")

    summary = "\n".join(summary_parts)

    return {
        "type": type_name,
        "type_key": human_type,
        "strategy": strategy,
        "authority": authority_name,
        "authority_key": authority,
        "authority_description": authority_desc,
        "profile": profile_name,
        "profile_key": profile,
        "profile_description": profile_desc,
        "not_self": not_self,
        "signature": signature,
        "centers": centers_result,
        "personality_gates": {
            "sun": {"gate": sun_gate, "description": f"Gate {sun_gate}"},
            "earth": {"gate": earth_gate, "description": f"Gate {earth_gate}"},
        },
        "design_gates": {
            "sun": {"gate": earth_gate, "description": f"Gate {earth_gate}"},
        },
        "summary": summary,
        "type_description": type_desc,
    }
