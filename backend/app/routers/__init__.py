"""Re-export all API routers for inclusion in the FastAPI app."""

from .health import router as health_router
from .auth import router as auth_router
from .tests import router as tests_router
from .accounts import router as accounts_router
from .community import router as community_router
from .community import blog_router
from .payments import router as payments_router
from .b2b import router as b2b_router
from .analytics import router as analytics_router
from .onboarding import router as onboarding_router

__all__ = [
    "health_router",
    "auth_router",
    "tests_router",
    "accounts_router",
    "community_router",
    "blog_router",
    "payments_router",
    "b2b_router",
    "analytics_router",
    "onboarding_router",
]
