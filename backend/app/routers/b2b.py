"""B2B (business / team) endpoints."""

from __future__ import annotations

import uuid
from datetime import UTC, datetime
from typing import Any

from fastapi import APIRouter, HTTPException

router = APIRouter(prefix="/api/v1/b2b", tags=["B2B"])

# ── In-memory stores ──────────────────────────────────────────────────────
_teams: dict[str, dict[str, Any]] = {}  # team_id -> team
_team_members: dict[str, list[dict[str, Any]]] = {}  # team_id -> members
_invitations: dict[str, dict[str, Any]] = {}  # invite_id -> invite

# ── Routes ────────────────────────────────────────────────────────────────


@router.post("/register")
async def register_b2b(payload: dict):
    """Register an organisation / team for B2B access."""
    org_name = payload.get("org_name", "")
    admin_email = payload.get("admin_email", "")
    if not org_name or not admin_email:
        raise HTTPException(status_code=400, detail="org_name and admin_email are required")

    team_id = str(uuid.uuid4())
    _teams[team_id] = {
        "id": team_id,
        "org_name": org_name,
        "admin_email": admin_email,
        "plan": payload.get("plan", "professional"),
        "created_at": datetime.now(UTC).isoformat(),
        "active": True,
    }
    _team_members[team_id] = [
        {"email": admin_email, "role": "admin", "joined_at": datetime.now(UTC).isoformat()},
    ]
    return {"team_id": team_id, "org_name": org_name, "status": "registered"}


@router.get("/teams")
async def list_teams():
    """List all registered teams."""
    return {
        "teams": [
            {"id": tid, **{k: v for k, v in t.items() if k != "plan"}}
            for tid, t in _teams.items()
        ],
        "total": len(_teams),
    }


@router.post("/invite")
async def invite_member(payload: dict):
    """Invite a new member to an existing team."""
    team_id = payload.get("team_id", "")
    email = payload.get("email", "")
    role = payload.get("role", "member")

    if team_id not in _teams:
        raise HTTPException(status_code=404, detail="Team not found")
    if not email:
        raise HTTPException(status_code=400, detail="email is required")

    invite_id = str(uuid.uuid4())
    _invitations[invite_id] = {
        "id": invite_id,
        "team_id": team_id,
        "email": email,
        "role": role,
        "status": "pending",
        "created_at": datetime.now(UTC).isoformat(),
    }
    return {
        "invite_id": invite_id,
        "email": email,
        "role": role,
        "status": "pending",
        "message": f"Invitation sent to {email}",
    }
