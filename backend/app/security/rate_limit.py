from __future__ import annotations

import time
from collections.abc import Awaitable, Callable
from typing import MutableMapping

from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import JSONResponse
from starlette.types import ASGIApp

from app.config import settings


class TokenBucket:
    """Per-IP token-bucket rate limiter."""

    __slots__ = ("capacity", "refill_rate", "_buckets")

    def __init__(self, capacity: int, refill_seconds: float) -> None:
        self.capacity = capacity
        self.refill_rate = capacity / refill_seconds  # tokens per second
        self._buckets: MutableMapping[str, tuple[float, float]] = {}  # ip -> (tokens, last_refill)

    def consume(self, key: str, tokens: float = 1.0) -> bool:
        now = time.monotonic()
        current, last = self._buckets.get(key, (self.capacity, now))
        elapsed = now - last
        current = min(self.capacity, current + elapsed * self.refill_rate)

        if current < tokens:
            self._buckets[key] = (current, now)
            return False

        self._buckets[key] = (current - tokens, now)
        return True

    def cleanup(self) -> None:
        """Remove stale entries (older than refill window)."""
        now = time.monotonic()
        cutoff = now - (self.capacity / self.refill_rate) * 2
        self._buckets = {k: v for k, v in self._buckets.items() if v[1] > cutoff}


_bucket: TokenBucket | None = None


def get_bucket() -> TokenBucket:
    global _bucket
    if _bucket is None:
        _bucket = TokenBucket(settings.rate_limit_tokens, settings.rate_limit_refill_seconds)
    return _bucket


class RateLimitMiddleware(BaseHTTPMiddleware):
    """Reject requests that exceed the token-bucket limit."""

    def __init__(self, app: ASGIApp) -> None:
        super().__init__(app)
        self._bucket = get_bucket()

    async def dispatch(
        self, request: Request, call_next: Callable[[Request], Awaitable[Response]]
    ) -> JSONResponse | Response:
        # Skip rate-limiting for health/metrics endpoints
        if request.url.path in ("/health", "/metrics"):
            return await call_next(request)

        client_ip = request.client.host if request.client else "unknown"
        if not self._bucket.consume(client_ip):
            return JSONResponse(
                status_code=429,
                content={"detail": "Too many requests. Please slow down."},
                headers={"Retry-After": str(int(settings.rate_limit_refill_seconds))},
            )
        return await call_next(request)
