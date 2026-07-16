"""Authentication, account registration, and GDPR endpoints."""

from __future__ import annotations

import hashlib
import secrets
import uuid
from datetime import UTC, datetime, timedelta
from typing import Any

from fastapi import APIRouter, Header, HTTPException

from app.security import create_jwt, hash_password, verify_jwt, verify_password

from app.routers._store import (
    _accounts_by_email,
    _accounts_by_id,
    _challenges,
    _consent_log,
    _devices,
)

router = APIRouter(prefix="/api/v1", tags=["Auth"])

# ── Helpers ───────────────────────────────────────────────────────────────


def _extract_token(authorization: str | None) -> str:
    if not authorization or not authorization.startswith("Bearer "):
        raise HTTPException(status_code=401, detail="Missing or malformed Authorization header")
    return authorization.removeprefix("Bearer ")


def _resolve_from_header(authorization: str | None) -> dict[str, Any]:
    """Validate bearer token and return the associated account dict."""
    token = _extract_token(authorization)
    payload = verify_jwt(token)
    if not payload:
        raise HTTPException(status_code=401, detail="Invalid or expired token")
    account = _accounts_by_id.get(payload["sub"])
    if not account:
        raise HTTPException(status_code=404, detail="Account not found")
    return account


# ── Challenge / verify (device auth flow) ────────────────────────────────


@router.get("/auth/challenge")
async def get_challenge(agent_type: str = "web"):
    """Issue a cryptographic challenge (nonce) for device authentication.

    The client must sign the returned *nonce* and submit it via
    ``POST /auth/verify``.
    """
    challenge_id = uuid.uuid4().hex
    nonce = secrets.token_hex(32)
    expires_at = (datetime.now(UTC) + timedelta(minutes=5)).isoformat()
    _challenges[challenge_id] = {
        "nonce": nonce,
        "expires_at": expires_at,
        "agent_type": agent_type,
    }
    return {
        "challenge_id": challenge_id,
        "nonce": nonce,
        "expires_at": expires_at,
    }


@router.post("/auth/verify")
async def verify_challenge(payload: dict):
    """Verify a signed challenge and issue a short-lived session token."""
    challenge_id = payload.get("challenge_id", "")
    response = payload.get("response", "")

    if challenge_id not in _challenges:
        raise HTTPException(status_code=404, detail="Challenge not found")

    challenge = _challenges.pop(challenge_id)  # one-time use

    if datetime.now(UTC) > datetime.fromisoformat(challenge["expires_at"]):
        raise HTTPException(status_code=401, detail="Challenge expired")

    # Simple verification: the ``response`` is SHA-256 of the nonce.
    expected = hashlib.sha256(challenge["nonce"].encode()).hexdigest()
    if response != expected:
        raise HTTPException(status_code=401, detail="Invalid response")

    session_token = secrets.token_hex(32)
    return {"verified": True, "session_token": session_token}


@router.post("/auth/device/register")
async def register_device(payload: dict):
    """Register a device for notifications or device-scoped auth."""
    device_id = payload.get("device_id", "")
    platform = payload.get("platform", "")

    if not device_id or not platform:
        raise HTTPException(status_code=400, detail="device_id and platform are required")

    _devices[device_id] = {
        "device_id": device_id,
        "platform": platform,
        "signed_nonce": payload.get("signed_nonce"),
        "registered_at": datetime.now(UTC).isoformat(),
    }
    return {"status": "registered", "device_id": device_id}


# ── Email/password login ─────────────────────────────────────────────────


@router.post("/auth/login")
async def login(payload: dict):
    """Authenticate with email + password and receive a JWT.

    The returned *token* should be sent as ``Authorization: Bearer <token>``
    on subsequent requests.
    """
    email = payload.get("email", "")
    password = payload.get("password", "")

    account = _accounts_by_email.get(email)
    if not account or not verify_password(password, account["password_hash"]):
        raise HTTPException(status_code=401, detail="Invalid email or password")

    token = create_jwt(uuid.UUID(account["id"]))
    return {
        "token": token,
        "account": {"id": account["id"], "email": account["email"]},
    }


# ── Account registration ──────────────────────────────────────────────────


@router.post("/accounts/register")
async def register(payload: dict):
    """Create a new user account.

    Passwords are hashed with Argon2id before storage.
    """
    email = payload.get("email", "").strip().lower()
    password = payload.get("password", "")

    if not email or not password:
        raise HTTPException(status_code=400, detail="email and password are required")
    if len(password) < 6:
        raise HTTPException(status_code=400, detail="Password must be at least 6 characters")

    if email in _accounts_by_email:
        raise HTTPException(status_code=409, detail="Email already registered")

    account_id = str(uuid.uuid4())
    account: dict[str, Any] = {
        "id": account_id,
        "email": email,
        "password_hash": hash_password(password),
        "created_at": datetime.now(UTC).isoformat(),
        "updated_at": datetime.now(UTC).isoformat(),
    }
    _accounts_by_email[email] = account
    _accounts_by_id[account_id] = account

    return {"id": account_id, "email": email}


# ── Account read / GDPR ───────────────────────────────────────────────────


@router.get("/accounts/{account_id}")
async def get_account(account_id: str):
    """Return public account info by ID."""
    account = _accounts_by_id.get(account_id)
    if not account:
        raise HTTPException(status_code=404, detail="Account not found")
    return {
        "id": account["id"],
        "email": account["email"],
        "created_at": account["created_at"],
    }


@router.delete("/accounts/me")
async def delete_account(authorization: str | None = Header(None)):
    """GDPR-compliant account deletion — removes the authenticated account."""
    account = _resolve_from_header(authorization)
    _accounts_by_id.pop(account["id"], None)
    _accounts_by_email.pop(account["email"], None)
    return {"status": "deleted"}


@router.get("/accounts/me/export")
async def export_account_data(authorization: str | None = Header(None)):
    """GDPR-compliant data export — returns all stored data for the user."""
    account = _resolve_from_header(authorization)
    return {
        "account": {k: v for k, v in account.items() if k != "password_hash"},
        "consent_log": [c for c in _consent_log if c.get("account_id") == account["id"]],
    }


@router.post("/accounts/me/consent")
async def log_consent(payload: dict, authorization: str | None = Header(None)):
    """Log a GDPR consent action (e.g. ``terms_accepted``, ``data_processing``)."""
    account = _resolve_from_header(authorization)
    _consent_log.append({
        "account_id": account["id"],
        "action": payload.get("action", ""),
        "timestamp": datetime.now(UTC).isoformat(),
    })
    return {"status": "logged"}
