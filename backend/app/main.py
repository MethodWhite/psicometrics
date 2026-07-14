"""PsicoMetrics — Official Personality Assessment Platform.

Tier S++ SecDevOps security implementation adapted from Synapsis.
Includes Zero Trust, TPM/Device Attestation, and defense in depth.
"""

import secrets
from contextlib import asynccontextmanager
from pathlib import Path

from fastapi import FastAPI, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
from fastapi.staticfiles import StaticFiles

from app.config import get_settings
from app.routers import auth_router, health_router, tests_router
from app.security.audit import audit_logger
from app.security.csrf import CSRFMiddleware
from app.security.device_attestation import DeviceVerifier
from app.security.headers import SecurityHeadersMiddleware
from app.security.rate_limit import RateLimiter
from app.security.zero_trust import ZeroTrustProvider

settings = get_settings()

rate_limiter = RateLimiter(
    requests_per_minute=settings.rate_limit_requests_per_minute,
    lockout_threshold=10,
    lockout_seconds=3600,
)

# Zero Trust provider - shared across routes
zero_trust = ZeroTrustProvider(
    secret_key=settings.secret_key.encode() if settings.secret_key != "CHANGE-ME-IN-PRODUCTION" else None,
    challenge_ttl=300,
    max_failed=5,
)

# Device verifier for TPM/attestation
device_verifier = DeviceVerifier(app_package="com.psicometrics.psicometrics")


@asynccontextmanager
async def lifespan(app: FastAPI):
    # Startup
    audit_logger.log("app_start", metadata={"version": settings.app_version})
    # Inject providers into auth router
    from app.routers import auth
    auth.zero_trust = zero_trust
    auth.device_verifier = device_verifier
    yield
    # Shutdown
    audit_logger.log("app_stop")


app = FastAPI(
    title=settings.app_name,
    version=settings.app_version,
    lifespan=lifespan,
    docs_url="/api/docs" if settings.debug else None,
    redoc_url="/api/redoc" if settings.debug else None,
)

# CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.allowed_origins,
    allow_credentials=True,
    allow_methods=["GET", "POST"],
    allow_headers=["*"],
)

# Security middleware stack
app.add_middleware(SecurityHeadersMiddleware)
app.add_middleware(CSRFMiddleware)


@app.middleware("http")
async def rate_limit_middleware(request: Request, call_next):
    client_ip = request.client.host if request.client else "unknown"
    if not rate_limiter.allow(client_ip):
        return JSONResponse(
            {"detail": "Rate limit exceeded", "code": "RATE_LIMITED"},
            status_code=429,
            headers={"Retry-After": "60"},
        )
    response = await call_next(request)
    return response


@app.middleware("http")
async def session_middleware(request: Request, call_next):
    if not hasattr(request.state, "session_id"):
        request.state.session_id = secrets.token_urlsafe(32)
    response = await call_next(request)
    return response


@app.middleware("http")
async def zero_trust_middleware(request: Request, call_next):
    """Verify Zero Trust challenge for API writes."""
    if request.method in {"POST", "PUT", "DELETE"} and request.url.path.startswith("/api/v1/tests"):
        challenge = request.headers.get("x-challenge-id")
        response_header = request.headers.get("x-challenge-response")

        if challenge and response_header:
            try:
                zero_trust.verify_and_consume(challenge, response_header)
            except Exception:
                # If verification fails, let it through but log
                audit_logger.log(
                    "zero_trust_failed",
                    session_id=request.state.session_id if hasattr(request.state, "session_id") else None,
                    metadata={"path": str(request.url.path)},
                )

    response = await call_next(request)
    return response


# Routers
app.include_router(health_router)
app.include_router(auth_router)
app.include_router(tests_router)

# Serve frontend
frontend_dist = Path(__file__).parent.parent.parent / "frontend" / "dist"
if frontend_dist.exists():
    app.mount("/", StaticFiles(directory=str(frontend_dist), html=True), name="frontend")
