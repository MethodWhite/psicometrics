"""Psicometrics API — FastAPI application entry-point."""

from __future__ import annotations

import uuid
from collections.abc import AsyncGenerator
from contextlib import asynccontextmanager
from datetime import UTC, datetime

from fastapi import FastAPI, Request
from fastapi.middleware.cors import CORSMiddleware
from prometheus_client import Counter, Gauge, Histogram, generate_latest
from starlette.responses import PlainTextResponse, Response

from app import __version__
from app.config import settings
from app.database import init_db
from app.security import AuditLogger, RateLimitMiddleware, SecurityHeadersMiddleware

# ── Metrics ──────────────────────────────────────────────────────────────

REQUEST_COUNT = Counter("http_requests_total", "Total HTTP requests", ["method", "path", "status"])
REQUEST_DURATION = Histogram(
    "http_request_duration_seconds",
    "HTTP request duration in seconds",
    ["method", "path"],
)
ACTIVE_REQUESTS = Gauge("http_requests_active", "Currently active requests", ["method"])

# ── Audit ────────────────────────────────────────────────────────────────

audit_logger = AuditLogger()

# ── Lifespan ─────────────────────────────────────────────────────────────


@asynccontextmanager
async def lifespan(_app: FastAPI) -> AsyncGenerator[None, None]:
    # Startup
    await init_db()
    _app.state.audit_logger = audit_logger
    yield
    # Shutdown — nothing special for now


# ── App ──────────────────────────────────────────────────────────────────

app = FastAPI(
    title=settings.app_name,
    version=__version__,
    lifespan=lifespan,
    docs_url="/docs",
    redoc_url="/redoc",
)

# ── Request-ID middleware (inline for simplicity) ────────────────────────


@app.middleware("http")
async def add_request_id(request: Request, call_next) -> Response:
    request_id = request.headers.get("X-Request-ID", uuid.uuid4().hex[:12])
    response = await call_next(request)
    response.headers["X-Request-ID"] = request_id
    return response


# ── Metrics middleware ───────────────────────────────────────────────────


@app.middleware("http")
async def metrics_middleware(request: Request, call_next) -> Response:
    if request.url.path == "/metrics":
        return await call_next(request)

    ACTIVE_REQUESTS.labels(method=request.method).inc()
    try:
        with REQUEST_DURATION.labels(method=request.method, path=request.url.path).time():
            response = await call_next(request)
    finally:
        ACTIVE_REQUESTS.labels(method=request.method).dec()

    REQUEST_COUNT.labels(
        method=request.method,
        path=request.url.path,
        status=response.status_code,
    ).inc()
    return response


# ── Third-party middleware stack ─────────────────────────────────────────

app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.cors_origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
app.add_middleware(SecurityHeadersMiddleware)
app.add_middleware(RateLimitMiddleware)
# Auth middleware disabled for public API — enable for production
# app.add_middleware(AuthMiddleware)


# ═══════════════════════════════════════════════════════════════════════
# Routes
# ═══════════════════════════════════════════════════════════════════════


@app.get("/health", tags=["system"])
async def health() -> dict:
    return {
        "status": "ok",
        "version": __version__,
        "timestamp": datetime.now(UTC).isoformat(),
    }


@app.get("/metrics", tags=["system"], include_in_schema=False)
async def metrics() -> PlainTextResponse:
    return PlainTextResponse(
        content=generate_latest().decode(),
        media_type="text/plain; charset=utf-8",
    )


# ── API v1 routes ────────────────────────────────────────────────────────

from app.routers import auth, tests, community, payments, analytics, b2b, onboarding, health

app.include_router(auth.router, prefix="/api/v1/auth", tags=["auth"])
app.include_router(tests.router, prefix="/api/v1/tests", tags=["tests"])
app.include_router(community.router, prefix="/api/v1/community", tags=["community"])
app.include_router(payments.router, prefix="/api/v1/payments", tags=["payments"])
app.include_router(analytics.router, prefix="/api/v1/analytics", tags=["analytics"])
app.include_router(b2b.router, prefix="/api/v1/b2b", tags=["b2b"])
app.include_router(onboarding.router, prefix="/api/v1/onboarding", tags=["onboarding"])
app.include_router(health.router, tags=["health"])
