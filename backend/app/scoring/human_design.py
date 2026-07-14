"""Human Design Calculator.

Based on the Human Design System (Ra Uru Hu).
Calculates Type, Strategy, Authority, Profile, and Centers from birth data.
Uses astronomical calculations for planetary positions.
"""

import math
from datetime import datetime, timezone
from typing import Any

from app.data_loader import load_questions


# I Ching 64 Gates mapped to zodiac positions (simplified)
# Each gate corresponds to ~5.625 degrees of the zodiac
GATE_MAP = list(range(1, 65))  # Gates 1-64

#太阳星座到闸门的映射 (simplified mapping)
SUN_GATE_OFFSET = 41  # The Sun's position offset for the Design calculation

# Type determination based on motor connection to throat
MOTOR_CENTERS = {"sacral", "solar_plexus", "heart", "root"}
THROAT_CENTER = "throat"
G_CENTER = "g"


def calculate_human_design(birth_date: str, birth_time: str, birth_location: str) -> dict:
    """Calculate Human Design chart from birth data.

    Args:
        birth_date: ISO format date (YYYY-MM-DD)
        birth_time: 24h format (HH:MM)
        birth_location: City or coordinates

    Returns:
        dict with type, strategy, authority, profile, centers, description
    """
    # Parse birth data
    dt = datetime.strptime(f"{birth_date} {birth_time}", "%Y-%m-%d %H:%M")

    # Calculate planetary positions (simplified)
    planets = _calculate_planets(dt)

    # Calculate gates from planetary positions
    personality_gates = _calculate_gates(planets["personality"])
    design_gates = _calculate_gates(planets["design"])

    # Determine defined/undefined centers
    centers = _determine_centers(personality_gates, design_gates)

    # Determine type based on motor connections
    hd_type = _determine_type(centers)

    # Determine authority based on defined centers
    authority = _determine_authority(centers)

    # Calculate profile from personality/design gates
    profile = _calculate_profile(personality_gates, design_gates)

    # Load descriptions from data file
    data = load_questions("human_design")

    # Get type info
    type_info = data["types"].get(hd_type, {})
    authority_info = data["authorities"].get(authority, {})
    profile_info = data["profiles"].get(profile, {})

    return {
        "type": hd_type,
        "type_info": type_info,
        "strategy": type_info.get("en", {}).get("strategy", ""),
        "authority": authority,
        "authority_info": authority_info,
        "profile": profile,
        "profile_info": profile_info,
        "centers": centers,
        "personality_gates": personality_gates,
        "design_gates": design_gates,
        "planets": planets,
        "summary": _generate_summary(hd_type, authority, profile, centers, "es"),
    }


def _calculate_planets(dt: datetime) -> dict:
    """Calculate simplified planetary positions for a given datetime."""
    # Simplified astronomical calculation
    # In production, use Swiss Ephemeris or similar library
    day_of_year = dt.timetuple().tm_yday
    hour = dt.hour + dt.minute / 60.0

    # Personality (conscious) planets - based on birth time
    personality_sun = (day_of_year * 0.9856 + hour * 0.0417) % 360
    personality_moon = (day_of_year * 13.176 + hour * 0.549) % 360
    personality_mercury = (personality_sun + 30 + (day_of_year % 30) * 0.5) % 360
    personality_venus = (personality_sun + 50 + (day_of_year % 60) * 0.3) % 360
    personality_mars = (personality_sun + 120 + (day_of_year % 90) * 0.2) % 360
    personality_jupiter = (personality_sun + 60 + (day_of_year % 400) * 0.08) % 360
    personality_saturn = (personality_sun + 90 + (day_of_year % 360) * 0.03) % 360
    personality_uranus = (personality_sun + 180 + (day_of_year % 365) * 0.01) % 360
    personality_neptune = (personality_sun + 210 + (day_of_year % 600) * 0.006) % 360
    personality_pluto = (personality_sun + 240 + (day_of_year % 900) * 0.004) % 360

    # Design (unconscious) planets - 88 days before birth (simplified)
    design_offset = 88
    design_sun = (personality_sun - design_offset * 0.9856) % 360
    design_moon = (personality_moon - design_offset * 13.176) % 360
    design_mercury = (design_sun + 30) % 360
    design_venus = (design_sun + 50) % 360
    design_mars = (design_sun + 120) % 360
    design_jupiter = (design_sun + 60) % 360
    design_saturn = (design_sun + 90) % 360
    design_uranus = (design_sun + 180) % 360
    design_neptune = (design_sun + 210) % 360
    design_pluto = (design_sun + 240) % 360

    return {
        "personality": {
            "sun": personality_sun,
            "moon": personality_moon,
            "mercury": personality_mercury,
            "venus": personality_venus,
            "mars": personality_mars,
            "jupiter": personality_jupiter,
            "saturn": personality_saturn,
            "uranus": personality_uranus,
            "neptune": personality_neptune,
            "pluto": personality_pluto,
        },
        "design": {
            "sun": design_sun,
            "moon": design_moon,
            "mercury": design_mercury,
            "venus": design_venus,
            "mars": design_mars,
            "jupiter": design_jupiter,
            "saturn": design_saturn,
            "uranus": design_uranus,
            "neptune": design_neptune,
            "pluto": design_pluto,
        },
    }


def _calculate_gates(planets: dict) -> list[int]:
    """Calculate I Ching gates from planetary positions."""
    gates = []
    for planet_pos in planets.values():
        # Each gate spans 360/64 = 5.625 degrees
        gate = int((planet_pos % 360) / 5.625) + 1
        if gate > 64:
            gate = 64
        gates.append(gate)
    return gates


def _determine_centers(personality_gates: list[int], design_gates: list[int]) -> dict:
    """Determine which centers are defined based on gates."""
    all_gates = set(personality_gates + design_gates)

    # Simplified center determination based on gate ranges
    # In production, use the full channel-gate-center mapping
    centers = {}

    # Head Center: Gates 64, 61, 63
    centers["head"] = bool(all_gates & {64, 61, 63})

    # Ajna Center: Gates 47, 24, 4, 17, 43, 11
    centers["ajna"] = bool(all_gates & {47, 24, 4, 17, 43, 11})

    # Throat Center: Gates 62, 56, 35, 12, 45, 33, 8, 31, 27, 20, 16, 23, 2, 15, 10
    centers["throat"] = bool(all_gates & {62, 56, 35, 12, 45, 33, 8, 31, 27, 20, 16, 23, 2, 15, 10})

    # G Center: Gates 1, 13, 25, 46, 2, 15, 10, 7
    centers["g"] = bool(all_gates & {1, 13, 25, 46, 2, 15, 10, 7})

    # Heart/Ego Center: Gates 21, 51, 26, 40
    centers["heart"] = bool(all_gates & {21, 51, 26, 40})

    # Sacral Center: Gates 5, 14, 29, 59, 9, 3, 42, 27, 34, 21, 51, 26, 40
    centers["sacral"] = bool(all_gates & {5, 14, 29, 59, 9, 3, 42, 27, 34})

    # Solar Plexus: Gates 36, 22, 37, 6, 49, 55, 30
    centers["solar_plexus"] = bool(all_gates & {36, 22, 37, 6, 49, 55, 30})

    # Splenic Center: Gates 48, 57, 44, 50, 32, 28, 18
    centers["splenic"] = bool(all_gates & {48, 57, 44, 50, 32, 28, 18})

    # Root Center: Gates 53, 60, 52, 19, 39, 41, 58, 38, 54, 61, 60, 19, 39, 41
    centers["root"] = bool(all_gates & {53, 60, 52, 19, 39, 41, 58, 38, 54})

    return centers


def _determine_type(centers: dict) -> str:
    """Determine Human Design type based on motor connections."""
    has_sacral = centers.get("sacral", False)
    has_throat = centers.get("throat", False)
    has_motor = centers.get("solar_plexus", False) or centers.get("heart", False) or centers.get("root", False)

    # Generator types have defined Sacral
    if has_sacral:
        if has_motor and has_throat:
            return "Manifesting Generator"
        return "Generator"

    # Manifestors have motor connected to throat (simplified)
    if has_motor and has_throat:
        return "Manifestor"

    # Reflector has no defined centers
    if not any(centers.values()):
        return "Reflector"

    # Projector has no motor and no sacral
    return "Projector"


def _determine_authority(centers: dict) -> str:
    """Determine authority based on defined centers."""
    # Emotional authority takes precedence
    if centers.get("solar_plexus", False):
        return "Emotional"

    if centers.get("sacral", False):
        return "Sacral"

    if centers.get("splenic", False):
        return "Splenic"

    if centers.get("heart", False):
        return "Ego"

    if centers.get("g", False) and centers.get("throat", False):
        return "Self-Projected"

    if centers.get("ajna", False) and centers.get("head", False):
        return "Mental"

    return "Lunar"


def _calculate_profile(personality_gates: list[int], design_gates: list[int]) -> str:
    """Calculate profile from personality and design gates."""
    # Simplified profile calculation based on gate patterns
    # In production, use the full profile calculation algorithm
    p_sum = sum(personality_gates) % 6 + 1
    d_sum = sum(design_gates) % 6 + 1

    profiles = [
        "1/3", "1/4", "2/4", "2/5", "3/5", "3/6",
        "4/6", "4/1", "5/1", "5/2", "6/2", "6/3"
    ]

    idx = (p_sum * 3 + d_sum * 7) % len(profiles)
    return profiles[idx]


def _generate_summary(hd_type: str, authority: str, profile: str, centers: dict, language: str) -> str:
    """Generate a natural language summary."""
    data = load_questions("human_design")
    type_info = data["types"].get(hd_type, {})
    authority_info = data["authorities"].get(authority, {})

    # Handle nested language keys
    type_localized = type_info.get(language, type_info.get("en", {}))
    if isinstance(type_localized, str):
        type_localized = {"name": type_localized}

    auth_localized = authority_info.get(language, authority_info.get("en", {}))
    if isinstance(auth_localized, str):
        auth_localized = {"description": auth_localized}

    defined_count = sum(1 for v in centers.values() if v)
    total_count = len(centers)

    if language == "es":
        return (
            f"Eres un/una {type_localized.get('name', hd_type)}. "
            f"Tu estrategia es: {type_localized.get('strategy', '')}. "
            f"Tu autoridad es: {authority} — {auth_localized.get('description', '')}. "
            f"Tu perfil es {profile}. "
            f"Tienes {defined_count} de {total_count} centros definidos."
        )
    return (
        f"You are a {type_localized.get('name', hd_type)}. "
        f"Your strategy is: {type_localized.get('strategy', '')}. "
        f"Your authority is: {authority} — {auth_localized.get('description', '')}. "
        f"Your profile is {profile}. "
        f"You have {defined_count} of {total_count} centers defined."
    )
