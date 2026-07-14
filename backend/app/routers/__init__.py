from app.routers.auth import router as auth_router
from app.routers.health import router as health_router
from app.routers.tests import router as tests_router

__all__ = ["auth_router", "health_router", "tests_router"]
