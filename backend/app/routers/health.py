"""Health check endpoints."""

from fastapi import APIRouter
from sqlalchemy import text

from app.config import get_settings
from app.database import async_session

router = APIRouter(tags=["health"])


@router.get("/api/v1/health")
async def health_check():
    return {"status": "ok", "version": get_settings().app_version}


@router.get("/api/v1/health/ready")
async def readiness_check():
    try:
        async with async_session() as session:
            await session.execute(text("SELECT 1"))
        return {"status": "ready", "database": "connected"}
    except Exception as e:
        return {"status": "not ready", "database": str(e)}
