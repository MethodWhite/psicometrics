"""Zero Trust + Device Attestation API endpoints."""

from fastapi import APIRouter, HTTPException, Request
from pydantic import BaseModel

from app.security.zero_trust import ZeroTrustProvider, ChallengeError
from app.security.device_attestation import DeviceVerifier
from app.security.audit import audit_logger

router = APIRouter(prefix="/api/v1/auth", tags=["auth"])

# Shared providers (initialized in main.py)
zero_trust: ZeroTrustProvider | None = None
device_verifier: DeviceVerifier | None = None


class ChallengeRequest(BaseModel):
    session_id: str | None = None
    agent_type: str = "unknown"


class ChallengeResponse(BaseModel):
    challenge_id: str
    nonce: str
    expires_at: float


class VerifyRequest(BaseModel):
    challenge_id: str
    response: str


class VerifyResponse(BaseModel):
    verified: bool
    session_token: str | None = None


class DeviceRegisterRequest(BaseModel):
    device_id: str
    platform: str  # android, ios, web
    signed_nonce: str
    app_version: str = "1.0.0"


class DeviceRegisterResponse(BaseModel):
    device_id: str
    trusted: bool
    challenge: str | None = None


@router.get("/challenge")
async def get_challenge(session_id: str | None = None, agent_type: str = "unknown"):
    """Request a challenge for Zero Trust authentication."""
    if zero_trust is None:
        raise HTTPException(status_code=500, detail="Zero Trust not initialized")

    challenge = zero_trust.generate_challenge(session_id, agent_type)
    return ChallengeResponse(
        challenge_id=challenge.id,
        nonce=challenge.nonce,
        expires_at=challenge.expires_at,
    )


@router.post("/verify")
async def verify_challenge(request: Request, body: VerifyRequest):
    """Verify a challenge response.

    The client must send back HMAC-SHA256(original_nonce, shared_secret)
    """
    if zero_trust is None:
        raise HTTPException(status_code=500, detail="Zero Trust not initialized")

    session_id = request.state.session_id if hasattr(request.state, "session_id") else None

    try:
        result = zero_trust.verify_and_consume(body.challenge_id, body.response)
        session_token = None
        if result:
            session_token = secrets.token_urlsafe(32)

        audit_logger.log(
            "auth_challenge_verify",
            session_id=session_id,
            metadata={"challenge_id": body.challenge_id, "result": "verified" if result else "failed"},
        )

        return VerifyResponse(verified=result, session_token=session_token)
    except ChallengeError as e:
        audit_logger.log(
            "auth_challenge_error",
            session_id=session_id,
            metadata={"challenge_id": body.challenge_id, "error": str(e)},
        )
        return VerifyResponse(verified=False, session_token=None)


@router.post("/device/register")
async def register_device(body: DeviceRegisterRequest):
    """Register a device with attestation."""
    if device_verifier is None:
        raise HTTPException(status_code=500, detail="Device verifier not initialized")

    try:
        challenge = device_verifier.generate_challenge(body.device_id)
        return DeviceRegisterResponse(
            device_id=body.device_id,
            trusted=False,
            challenge=challenge,
        )
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))


@router.post("/device/attest")
async def attest_device(body: DeviceRegisterRequest):
    """Attest a device with signed challenge."""
    if device_verifier is None:
        raise HTTPException(status_code=500, detail="Device verifier not initialized")

    try:
        attestation = device_verifier.verify_device(
            body.device_id,
            body.platform,
            body.signed_nonce,
            body.app_version,
        )
        return DeviceRegisterResponse(
            device_id=body.device_id,
            trusted=True,
            challenge=None,
        )
    except ValueError as e:
        return DeviceRegisterResponse(
            device_id=body.device_id,
            trusted=False,
            challenge=None,
        )


import secrets  # noqa: E402
