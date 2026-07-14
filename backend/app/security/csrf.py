"""CSRF double-submit cookie protection."""

import hashlib
import secrets
import time

from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import JSONResponse, Response

CSRF_TOKEN_EXPIRY = 1800  # 30 minutes
CSRF_COOKIE_NAME = "csrf_token"
CSRF_HEADER_NAME = "x-csrf-token"


def generate_csrf_token() -> str:
    return secrets.token_urlsafe(32)


def hash_token(token: str) -> str:
    return hashlib.sha256(token.encode()).hexdigest()


class CSRFMiddleware(BaseHTTPMiddleware):
    SAFE_METHODS = {"GET", "HEAD", "OPTIONS"}
    # API routes skip CSRF (frontend handles it via SameSite cookies)
    SKIP_PATHS = {"/api/"}

    async def dispatch(self, request: Request, call_next) -> Response:
        # Skip CSRF for API routes and safe methods
        if request.method in self.SAFE_METHODS or any(request.url.path.startswith(p) for p in self.SKIP_PATHS):
            response = await call_next(request)
            # Set CSRF cookie for non-API safe requests (browser forms)
            if request.method in self.SAFE_METHODS and not any(request.url.path.startswith(p) for p in self.SKIP_PATHS):
                if not request.cookies.get(CSRF_COOKIE_NAME):
                    token = generate_csrf_token()
                    response.set_cookie(
                        CSRF_COOKIE_NAME,
                        token,
                        httponly=False,
                        samesite="strict",
                        max_age=CSRF_TOKEN_EXPIRY,
                        secure=not request.url.scheme == "http",
                    )
            return response

        # CSRF check for non-API POST/PUT/DELETE
        cookie_token = request.cookies.get(CSRF_COOKIE_NAME)
        header_token = request.headers.get(CSRF_HEADER_NAME)

        if not cookie_token or not header_token:
            return JSONResponse({"detail": "CSRF token missing", "code": "CSRF_MISSING"}, status_code=403)

        if not secrets.compare_digest(hash_token(cookie_token), hash_token(header_token)):
            return JSONResponse({"detail": "CSRF token invalid", "code": "CSRF_INVALID"}, status_code=403)

        return await call_next(request)
