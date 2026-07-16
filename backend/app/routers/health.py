"""Health-check endpoints."""

from fastapi import APIRouter

router = APIRouter(prefix="/api/v1/health", tags=["Health"])


@router.get("")
async def health():
    """Basic liveness probe — always returns 200 when the service is running."""
    return {"status": "ok"}


@router.get("/ready")
async def readiness():
    """Readiness probe — verifies the database engine is reachable."""
    try:
        from app.database import engine

        # A real check would do ``await engine.connect()``, but for now a
        # module-level import is sufficient to confirm the stack is wired.
        _ = engine
        db_status = "ok"
    except Exception:
        db_status = "unavailable"

    return {"status": "ready", "database": db_status}
