"""Response validation for psicometric tests.

Detects suspicious response patterns (straight-lining, alternating),
missing answers, and flags attention-check failures.
"""

from __future__ import annotations

from typing import Any

ATTENTION_CHECK_IDS = {991, 992, 993}

KNOWN_TEST_TYPES = {
    "big_five",
    "mbti",
    "disc",
    "enneagram",
    "eq",
    "dark_triad",
    "attachment_style",
    "love_languages",
    "career_aptitude",
    "via_strengths",
    "human_design",
}

# Rough expected answer counts per test type.
EXPECTED_COUNTS: dict[str, int] = {
    "big_five": 120,
    "mbti": 72,
    "disc": 48,
    "enneagram": 108,
    "eq": 60,
    "dark_triad": 27,
    "attachment_style": 36,
    "love_languages": 30,
    "career_aptitude": 50,
    "via_strengths": 96,
    "human_design": 0,  # computed, not directly answered
}


def _detect_straight_line(answers: dict[int, int]) -> bool:
    """Check if all answered items have the same value."""
    vals = [v for v in answers.values() if v is not None]
    return len(set(vals)) == 1 and len(vals) > 3


def _detect_alternating(answers: dict[int, int]) -> bool:
    """Detect an A-B-A-B pattern in sequential answered items."""
    vals = [v for k, v in sorted(answers.items()) if v is not None]
    if len(vals) < 4:
        return False
    # Check A-B-A-B pattern (two distinct values alternating)
    unique = list(dict.fromkeys(vals))  # preserve order
    if len(unique) != 2:
        return False
    expected = [unique[i % 2] for i in range(len(vals))]
    return vals == expected


def validate_responses(
    test_type: str,
    answers: dict[int, int],
) -> dict[str, Any]:
    """Validate answer dict for a given test type.

    Returns
    -------
    dict with keys:
        is_valid             — bool
        warnings             — list[str]
        response_pattern     — "normal" | "straight_line" | "alternating"
        attention_check_failed — bool
    """
    warnings: list[str] = []
    response_pattern = "normal"
    attention_check_failed = False

    # ── unknown test type ─────────────────────────────────────────────
    if test_type not in KNOWN_TEST_TYPES:
        warnings.append(f"Unknown test type '{test_type}'; validation skipped.")
        return {
            "is_valid": True,
            "warnings": warnings,
            "response_pattern": "normal",
            "attention_check_failed": False,
        }

    # ── missing / empty ───────────────────────────────────────────────
    if not answers:
        return {
            "is_valid": False,
            "warnings": ["No answers provided."],
            "response_pattern": "normal",
            "attention_check_failed": False,
        }

    expected = EXPECTED_COUNTS.get(test_type, 0)

    # ── missing answers warning ───────────────────────────────────────
    if expected and len(answers) < expected:
        missing = expected - len(answers)
        warnings.append(
            f"Missing {missing} answer(s). "
            f"Expected {expected}, got {len(answers)}."
        )

    # ── straight-line pattern ─────────────────────────────────────────
    if _detect_straight_line(answers):
        warnings.append("Straight-line response pattern detected.")
        response_pattern = "straight_line"

    # ── alternating pattern ───────────────────────────────────────────
    if _detect_alternating(answers):
        warnings.append("Alternating response pattern detected (A-B-A-B).")
        response_pattern = "alternating"

    # ── attention check questions ─────────────────────────────────────
    for a_id in ATTENTION_CHECK_IDS:
        if a_id in answers:
            # Attention-check: expected answer is 5 (strongly agree)
            if answers[a_id] != 5:
                attention_check_failed = True
                warnings.append(
                    f"Attention check question {a_id} failed."
                )

    return {
        "is_valid": not attention_check_failed,
        "warnings": warnings,
        "response_pattern": response_pattern,
        "attention_check_failed": attention_check_failed,
    }
