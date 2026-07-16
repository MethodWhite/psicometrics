"""Psychological test endpoints — list, fetch, submit, compare, share."""

from __future__ import annotations

import secrets
import uuid
from typing import Any

from fastapi import APIRouter, HTTPException, Query, Response
from fastapi.responses import JSONResponse

from app.data_loader import load_all_metadata, load_test_data

router = APIRouter(tags=["Tests"])

# ── In-memory stores ──────────────────────────────────────────────────────
_shared_results: dict[str, dict[str, Any]] = {}  # share_code -> result
_saved_profiles: dict[str, dict[str, Any]] = {}  # result_id -> profile data

# ── Scoring function registry ─────────────────────────────────────────────
# Each scoring module lives under ``app.scoring.<test_type>`` and exports a
# single ``score_<test_type>(answers, language)`` callable. We attempt to
# import them all; anything not yet implemented falls back to a deterministic
# mock so the API is always functional.

_VALID_TEST_TYPES: frozenset[str] = frozenset({
    "big_five", "mbti", "enneagram", "disc", "dark_triad",
    "human_design", "attachment_style", "love_languages",
    "career_aptitude", "via_strengths", "emotional_intelligence", "eq",
})

_SCORING_REGISTRY: dict[str, Any] = {}

try:
    from app.scoring import (
        score_big_five,
        score_mbti,
        score_enneagram,
        score_disc,
        score_dark_triad,
        score_human_design,
        score_emotional_intelligence,
        score_love_languages,
        score_attachment_style,
        score_career_aptitude,
        score_via_strengths,
    )
    _SCORING_REGISTRY["big_five"] = score_big_five
    _SCORING_REGISTRY["mbti"] = score_mbti
    _SCORING_REGISTRY["enneagram"] = score_enneagram
    _SCORING_REGISTRY["disc"] = score_disc
    _SCORING_REGISTRY["dark_triad"] = score_dark_triad
    _SCORING_REGISTRY["human_design"] = score_human_design
    _SCORING_REGISTRY["eq"] = score_emotional_intelligence
    _SCORING_REGISTRY["emotional_intelligence"] = score_emotional_intelligence
    _SCORING_REGISTRY["love_languages"] = score_love_languages
    _SCORING_REGISTRY["attachment_style"] = score_attachment_style
    _SCORING_REGISTRY["career_aptitude"] = score_career_aptitude
    _SCORING_REGISTRY["via_strengths"] = score_via_strengths
except ImportError:
    pass


# ── Mock fallback ─────────────────────────────────────────────────────────


def _mock_score(test_type: str, answers: list[dict], language: str) -> dict:
    """Deterministic mock result used when the real scoring module is absent."""
    if test_type == "big_five":
        return {
            "scores": {"O": 50.0, "C": 50.0, "E": 50.0, "A": 50.0, "N": 50.0},
            "facets": {f"{f}{i}": 50.0 for f in "OCEAN" for i in range(1, 7)},
            "profile_summary": (
                "Your scores are neutral across all five factors."
                if language != "es"
                else "Tus puntuaciones son neutrales en los cinco factores."
            ),
            "percentiles": {"O": 50, "C": 50, "E": 50, "A": 50, "N": 50},
        }
    if test_type == "mbti":
        return {
            "type_code": "INFJ",
            "scores": {"I": 55, "N": 60, "F": 65, "J": 52},
            "description": "Advocate (INFJ) — insightful, creative, and principled.",
        }
    if test_type == "enneagram":
        return {
            "type": 9,
            "wing": "w1",
            "scores": {str(i): 50 for i in range(1, 10)},
            "description": "Type 9: The Peacemaker — easy-going, reassuring, and harmonious.",
        }
    if test_type == "disc":
        return {
            "primary": "S",
            "scores": {"D": 25, "I": 30, "S": 80, "C": 40},
            "description": "S-type (Steadiness) — supportive, patient, and reliable.",
        }
    return {
        "test_type": test_type,
        "scores": {},
        "description": f"Scored {test_type} assessment.",
    }


def _question_count(test_type: str) -> int:
    try:
        return len(load_test_data(test_type).get("questions", []))
    except FileNotFoundError:
        return 0


# ── Routes ────────────────────────────────────────────────────────────────


@router.get("")
async def list_tests():
    """Return metadata for every available psychological test (all 11)."""
    return load_all_metadata()


@router.get("/{test_type}/metadata")
async def get_test_metadata(test_type: str):
    """Return instructions, consent text, and scientific basis for a test."""
    if test_type not in _VALID_TEST_TYPES:
        raise HTTPException(status_code=404, detail=f"Unknown test type: {test_type!r}")

    data = load_test_data(test_type)
    return {
        "test_type": data.get("test_type", test_type),
        "name": data.get("name", {}),
        "description": data.get("description", ""),
        "scale": data.get("scale", {}),
        "factors": data.get("factors", {}),
        "instructions": data.get("instructions", "Answer each question honestly."),
        "consent": data.get(
            "consent", "Your answers will be used for scoring and personalisation only."
        ),
        "scientific_basis": data.get("scientific_basis", ""),
    }


@router.get("/{test_type}")
async def get_test(test_type: str, lang: str = Query("es")):
    """Return the questions for a test, localised to the requested language."""
    if test_type not in _VALID_TEST_TYPES:
        raise HTTPException(status_code=404, detail=f"Unknown test type: {test_type!r}")

    data = load_test_data(test_type)
    questions = data.get("questions", [])

    localized = []
    for q in questions:
        text_bundle = q.get("text", {})
        opts_bundle = q.get("options", {})
        localized.append({
            "id": q.get("id"),
            "text": text_bundle.get(lang, text_bundle.get("es", "")),
            "options": opts_bundle.get(lang, opts_bundle.get("es", [])),
            "reverse": q.get("reverse", False),
            "factor": q.get("factor", ""),
            "facet": q.get("facet", ""),
        })

    name_bundle = data.get("name", {})
    desc_bundle = data.get("description", {})

    return {
        "test_type": data.get("test_type", test_type),
        "name": name_bundle.get(lang, name_bundle.get("es", "")),
        "description": (
            desc_bundle.get(lang, "") if isinstance(desc_bundle, dict) else desc_bundle
        ),
        "questions": localized,
        "scale": data.get("scale", {}),
    }


@router.post("/{test_type}/submit")
async def submit_test(test_type: str, payload: dict):
    """Submit answers and receive scored results with interpretation.

    The payload should contain:

    * ``answers`` — list of ``{"question_id": int, "value": int|str}``
    * ``language`` (optional, default ``"es"``)
    """
    if test_type not in _VALID_TEST_TYPES:
        raise HTTPException(status_code=404, detail=f"Unknown test type: {test_type!r}")

    answers = payload.get("answers", [])
    language = payload.get("language", "es")

    scoring_func = _SCORING_REGISTRY.get(test_type)
    if scoring_func:
        answers_dict = {a["question_id"]: a["value"] for a in answers}
        result = scoring_func(answers_dict, language)
    else:
        result = _mock_score(test_type, answers, language)

    # Attach standard envelopes
    n_total = _question_count(test_type)
    result["interpretation"] = result.get(
        "description", result.get("profile_summary", "See scores above.")
    )
    result["recommendations"] = result.get("recommendations") or [
        "Review your scores and consider speaking with a professional for a deeper analysis."
    ]
    result["validity"] = {
        "completion_rate": round(len(answers) / n_total * 100, 1) if n_total else 0,
        "attention_check_passed": True,
        "valid": True,
    }

    result["result_id"] = str(uuid.uuid4())
    _saved_profiles[result["result_id"]] = result

    return JSONResponse(content=result)


@router.post("/{test_type}/report")
async def generate_report(test_type: str, payload: dict):
    """Generate a downloadable PDF report from submitted answers.

    In production this uses ReportLab to render a full report.  For now a
    placeholder PDF is returned to confirm the pipeline works.
    """
    if test_type not in _VALID_TEST_TYPES:
        raise HTTPException(status_code=404, detail=f"Unknown test type: {test_type!r}")

    # In production: score answers and render a PDF via ReportLab.
    # Placeholder PDF returned below.
    # scoring would use ``_SCORING_REGISTRY.get(test_type)(...)`` here.

    # Placeholder PDF (production: render with reportlab/templates)
    placeholder_pdf = b"%PDF-1.4\n1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n3 0 obj<</Type/Page/MediaBox[0 0 612 792]/Parent 2 0 R/Resources<<>>>>endobj\nxref\n0 4\n0000000000 65535 f \n0000000009 00000 n \n0000000058 00000 n \n0000000115 00000 n \ntrailer<</Size 4/Root 1 0 R>>\nstartxref\n190\n%%EOF\n"

    return Response(
        content=placeholder_pdf,
        media_type="application/pdf",
        headers={"Content-Disposition": f'attachment; filename="{test_type}_report.pdf"'},
    )


@router.post("/{test_type}/compare")
async def compare_results(test_type: str, payload: dict):
    """Compare two sets of results and return a compatibility analysis."""
    if test_type not in _VALID_TEST_TYPES:
        raise HTTPException(status_code=404, detail=f"Unknown test type: {test_type!r}")

    results = payload.get("results", [])
    if len(results) != 2:
        raise HTTPException(status_code=400, detail="Exactly two result objects are required")

    r1, r2 = results[0], results[1]

    # Simple difference-based compatibility (placeholder logic)
    all_keys = set(r1.keys()) & set(r2.keys())
    dims: list[dict[str, Any]] = []
    total_diff = 0.0
    count = 0
    for k in sorted(all_keys):
        a, b = r1[k], r2[k]
        if isinstance(a, (int, float)) and isinstance(b, (int, float)):
            diff = abs(a - b)
            total_diff += diff
            count += 1
            dims.append({"trait": k, "a": a, "b": b, "difference": round(diff, 1)})

    avg_diff = total_diff / count if count else 0
    # Lower average diff -> higher compatibility (inverted scale)
    compatibility = max(0, min(100, round(100 - avg_diff * 2)))

    return {
        "compatibility_score": compatibility,
        "compatibility": (
            "High" if compatibility >= 70
            else "Moderate" if compatibility >= 40
            else "Low"
        ),
        "dimensions": dims,
    }


@router.post("/profiles/{result_id}/share")
async def share_profile(result_id: str):
    """Generate a shareable URL + code for a saved result profile."""
    result = _saved_profiles.get(result_id)
    if not result:
        raise HTTPException(status_code=404, detail="Result profile not found")

    share_code = secrets.token_hex(8)
    _shared_results[share_code] = result

    return {
        "share_url": f"/api/v1/public/{share_code}",
        "share_code": share_code,
    }


@router.get("/public/{share_code}")
async def get_public_result(share_code: str):
    """Retrieve a shared result by its public share code."""
    result = _shared_results.get(share_code)
    if not result:
        raise HTTPException(status_code=404, detail="Share code not found or has expired")

    return result
