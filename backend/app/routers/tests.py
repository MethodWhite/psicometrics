"""API Routers for test endpoints."""

from fastapi import APIRouter, HTTPException, Request
from pydantic import BaseModel

from app.data_loader import load_questions
from app.schemas.big_five import BigFiveResult, BigFiveSubmission
from app.schemas.dark_triad import DarkTriadResult, DarkTriadSubmission
from app.schemas.disc import DISCResult, DISCSubmission
from app.schemas.enneagram import EnneagramResult, EnneagramSubmission
from app.schemas.human_design import HumanDesignResult, HumanDesignSubmission
from app.schemas.mbti import MBTIResult, MBTISubmission
from app.scoring import (
    calculate_human_design,
    score_big_five,
    score_dark_triad,
    score_disc,
    score_enneagram,
    score_mbti,
)
from app.security.audit import audit_logger

router = APIRouter(prefix="/api/v1/tests", tags=["tests"])

VALID_TYPES = ["big_five", "mbti", "enneagram", "disc", "dark_triad", "human_design"]


class TestInfo(BaseModel):
    test_type: str
    name: dict[str, str]
    description: dict[str, str]
    item_count: int
    test_mode: str  # "questions" or "birth_data"


@router.get("", response_model=list[TestInfo])
async def list_tests():
    """List available personality tests."""
    results = []
    for tt in VALID_TYPES:
        data = load_questions(tt)
        item_count = len(data.get("questions", []))
        test_mode = "birth_data" if tt == "human_design" else "questions"
        results.append(
            TestInfo(
                test_type=tt,
                name=data["name"],
                description=data["description"],
                item_count=item_count,
                test_mode=test_mode,
            )
        )
    return results


@router.get("/{test_type}")
async def get_test_questions(test_type: str, lang: str = "es"):
    """Get questions for a specific test in the requested language."""
    if test_type not in VALID_TYPES:
        raise HTTPException(status_code=404, detail=f"Test type '{test_type}' not found")

    data = load_questions(test_type)

    # Human Design doesn't have questions - return form fields
    if test_type == "human_design":
        return {
            "test_type": test_type,
            "name": data["name"].get(lang, data["name"]["es"]),
            "description": data["description"].get(lang, data["description"]["es"]),
            "questions": [],
            "test_mode": "birth_data",
            "types": {
                k: v.get(lang, v.get("en", {}))
                for k, v in data["types"].items()
            },
            "authorities": {
                k: v.get(lang, v.get("en", {}))
                for k, v in data["authorities"].items()
            },
            "profiles": {
                k: v.get(lang, v.get("en", {}))
                for k, v in data["profiles"].items()
            },
        }

    # Localize questions
    questions = []
    for q in data["questions"]:
        localized = {"id": q["id"], "text": q["text"].get(lang, q["text"]["es"])}
        if "facet" in q:
            localized["facet"] = q["facet"]
        if "reverse" in q:
            localized["reverse"] = q["reverse"]
        if "dichotomy" in q:
            localized["dichotomy"] = q["dichotomy"]
        if "pole" in q:
            localized["pole"] = q["pole"]
        if "type" in q:
            localized["type"] = q["type"]
        if "dimension" in q:
            localized["dimension"] = q["dimension"]
        if "trait" in q:
            localized["trait"] = q["trait"]
        questions.append(localized)

    # Localize metadata
    result = {
        "test_type": test_type,
        "name": data["name"].get(lang, data["name"]["es"]),
        "description": data["description"].get(lang, data["description"]["es"]),
        "questions": questions,
        "test_mode": "questions",
    }

    # Add test-specific metadata
    if test_type == "big_five":
        result["scale"] = {
            "min": data["scale"]["min"],
            "max": data["scale"]["max"],
            "labels": data["scale"]["labels"].get(lang, data["scale"]["labels"]["es"]),
        }
        result["factors"] = {
            k: v.get(lang, v["es"]) for k, v in data["factors"].items()
        }
    elif test_type == "mbti":
        result["dichotomies"] = {
            k: {kk: vv.get(lang, vv.get("es", vv)) if isinstance(vv, dict) else vv
                for kk, vv in v.items()}
            for k, v in data["dichotomies"].items()
        }
    elif test_type == "enneagram":
        result["scale"] = {
            "min": data["scale"]["min"],
            "max": data["scale"]["max"],
            "labels": data["scale"]["labels"].get(lang, data["scale"]["labels"]["es"]),
        }
        result["types"] = {
            k: {kk: vv.get(lang, vv.get("es", vv)) if isinstance(vv, dict) else vv
                for kk, vv in v.items()}
            for k, v in data["types"].items()
        }
    elif test_type == "disc":
        result["dimensions"] = {
            k: {kk: vv.get(lang, vv.get("es", vv)) if isinstance(vv, dict) else vv
                for kk, vv in v.items()}
            for k, v in data["dimensions"].items()
        }
    elif test_type == "dark_triad":
        result["scale"] = {
            "min": data["scale"]["min"],
            "max": data["scale"]["max"],
            "labels": data["scale"]["labels"].get(lang, data["scale"]["labels"]["es"]),
        }
        result["traits"] = {
            k: {kk: vv.get(lang, vv.get("es", vv)) if isinstance(vv, dict) else vv
                for kk, vv in v.items()}
            for k, v in data["traits"].items()
        }

    return result


@router.post("/{test_type}/submit")
async def submit_test(test_type: str, request: Request):
    """Submit test answers/data and get results."""
    session_id = request.state.session_id if hasattr(request.state, "session_id") else None

    if test_type not in VALID_TYPES:
        raise HTTPException(status_code=404, detail=f"Test type '{test_type}' not found")

    # Log submission attempt
    audit_logger.log(
        "test_submit",
        session_id=session_id,
        metadata={"test_type": test_type},
    )

    try:
        body = await request.json()

        if test_type == "big_five":
            sub = BigFiveSubmission.model_validate(body)
            answers = {a.question_id: a.value for a in sub.answers}
            result = score_big_five(answers, sub.language)
            return BigFiveResult(**result)

        elif test_type == "mbti":
            sub = MBTISubmission.model_validate(body)
            answers = {a.question_id: a.value for a in sub.answers}
            result = score_mbti(answers, sub.language)
            return MBTIResult(**result)

        elif test_type == "enneagram":
            sub = EnneagramSubmission.model_validate(body)
            answers = {a.question_id: a.value for a in sub.answers}
            result = score_enneagram(answers, sub.language)
            return EnneagramResult(**result)

        elif test_type == "disc":
            sub = DISCSubmission.model_validate(body)
            answers = {a.question_id: a.value for a in sub.answers}
            result = score_disc(answers, sub.language)
            return DISCResult(**result)

        elif test_type == "dark_triad":
            sub = DarkTriadSubmission.model_validate(body)
            answers = {a.question_id: a.value for a in sub.answers}
            result = score_dark_triad(answers, sub.language)
            return DarkTriadResult(**result)

        elif test_type == "human_design":
            sub = HumanDesignSubmission.model_validate(body)
            result = calculate_human_design(sub.birth_date, sub.birth_time, sub.birth_location)
            return HumanDesignResult(**result)

    except Exception as e:
        audit_logger.log(
            "test_submit_error",
            session_id=session_id,
            metadata={"test_type": test_type, "error": str(e)},
        )
        raise HTTPException(status_code=400, detail=str(e))
