"""Onboarding-flow management endpoints."""

from __future__ import annotations

import uuid
from datetime import UTC, datetime
from typing import Any

from fastapi import APIRouter, HTTPException

router = APIRouter(prefix="/api/v1/onboarding", tags=["Onboarding"])

# ── In-memory stores ──────────────────────────────────────────────────────
_onboarding: dict[str, dict[str, Any]] = {}  # email -> onboarding record
_unsubscribed: set[str] = set()


@router.post("/start")
async def start_onboarding(payload: dict):
    """Start the onboarding sequence for a new user.

    Creates a personalised onboarding plan that guides the user through
    their first tests and feature discovery.
    """
    email = payload.get("email", "")
    if not email:
        raise HTTPException(status_code=400, detail="email is required")

    if email in _unsubscribed:
        raise HTTPException(status_code=403, detail="This email has unsubscribed from onboarding")

    onboarding_id = str(uuid.uuid4())
    _onboarding[email] = {
        "id": onboarding_id,
        "email": email,
        "name": payload.get("name", ""),
        "step": "welcome",
        "completed_steps": [],
        "created_at": datetime.now(UTC).isoformat(),
        "active": True,
    }
    return {
        "onboarding_id": onboarding_id,
        "step": "welcome",
        "next_steps": [
            "Take your first personality test",
            "Explore your results",
            "Connect with the community",
        ],
    }


@router.post("/unsubscribe")
async def unsubscribe_onboarding(payload: dict):
    """Unsubscribe from onboarding emails and sequences."""
    email = payload.get("email", "")
    if not email:
        raise HTTPException(status_code=400, detail="email is required")

    _unsubscribed.add(email)
    record = _onboarding.get(email)
    if record:
        record["active"] = False
    return {"status": "unsubscribed", "email": email}


@router.get("/status")
async def onboarding_status(email: str = ""):
    """Get the current onboarding status for a user."""
    if not email:
        raise HTTPException(status_code=400, detail="email query parameter is required")

    if email in _unsubscribed:
        return {"email": email, "status": "unsubscribed"}

    record = _onboarding.get(email)
    if not record:
        return {"email": email, "status": "not_started"}

    return {
        "email": email,
        "status": "active" if record["active"] else "inactive",
        "step": record["step"],
        "completed_steps": record["completed_steps"],
        "created_at": record["created_at"],
    }
