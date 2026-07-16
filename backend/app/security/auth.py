from __future__ import annotations

import uuid
from datetime import UTC, datetime, timedelta
from typing import Any

import jwt
from argon2 import PasswordHasher
from argon2.exceptions import VerifyMismatchError
from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import JSONResponse
from starlette.types import ASGIApp

from app.config import settings

_ph = PasswordHasher()

# ── Password hashing ──────────────────────────────────────────────────


def hash_password(password: str) -> str:
    return _ph.hash(password)


def verify_password(password: str, password_hash: str) -> bool:
    try:
        return _ph.verify(password_hash, password)
    except VerifyMismatchError:
        return False


# ── JWT ───────────────────────────────────────────────────────────────


def create_jwt(
    account_id: uuid.UUID,
    *,
    expire_minutes: int | None = None,
    extra_claims: dict[str, Any] | None = None,
) -> str:
    now = datetime.now(UTC)
    payload: dict[str, Any] = {
        "sub": str(account_id),
        "iat": now,
        "exp": now + timedelta(minutes=expire_minutes or settings.jwt_expire_minutes),
        "jti": uuid.uuid4().hex,
    }
    if extra_claims:
        payload.update(extra_claims)
    return jwt.encode(payload, settings.jwt_secret, algorithm=settings.jwt_algorithm)


def verify_jwt(token: str) -> dict[str, Any] | None:
    try:
        payload = jwt.decode(
            token,
            settings.jwt_secret,
            algorithms=[settings.jwt_algorithm],
        )
        return payload
    except jwt.PyJWTError:
        return None


# ── Auth middleware ───────────────────────────────────────────────────


EXEMPT_PREFIXES = frozenset({
    "/docs",
    "/redoc",
    "/openapi.json",
    "/health",
    "/metrics",
    "/api/v1/health",
    "/api/v1/auth/challenge",
    "/api/v1/auth/verify",
    "/api/v1/auth/device/",
    "/api/v1/tests",
    "/api/v1/public",
})


class AuthMiddleware(BaseHTTPMiddleware):
    """Extract and validate JWT from Authorization header.

    On success, sets ``request.state.account_id`` (UUID) and
    ``request.state.token_payload`` (dict).  Exempt paths are skipped.
    """

    def __init__(self, app: ASGIApp) -> None:
        super().__init__(app)

    async def dispatch(
        self, request: Request, call_next: Any
    ) -> JSONResponse | Any:
        request.state.account_id = None  # type: ignore[attr-defined]
        request.state.token_payload = None  # type: ignore[attr-defined]

        path = request.url.path
        for prefix in EXEMPT_PREFIXES:
            if path == prefix or path.startswith(prefix + "/"):
                return await call_next(request)

        auth_header = request.headers.get("Authorization", "")
        if not auth_header.startswith("Bearer "):
            return JSONResponse(
                status_code=401,
                content={"detail": "Missing or malformed Authorization header"},
            )

        token = auth_header.removeprefix("Bearer ")
        payload = verify_jwt(token)
        if payload is None:
            return JSONResponse(
                status_code=401,
                content={"detail": "Invalid or expired token"},
            )

        request.state.account_id = uuid.UUID(payload["sub"])  # type: ignore[attr-defined]
        request.state.token_payload = payload  # type: ignore[attr-defined]
        return await call_next(request)
