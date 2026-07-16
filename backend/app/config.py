from __future__ import annotations

from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        extra="ignore",
    )

    # ── App ──────────────────────────────────────────────────────────
    app_name: str = "Psicometrics API"
    debug: bool = False

    # ── Database ──────────────────────────────────────────────────────
    database_url: str = (
        "postgresql+asyncpg://psicometrics:psicometrics@localhost:5432/psicometrics"
    )
    database_echo: bool = False
    database_pool_size: int = 10
    database_max_overflow: int = 20

    # ── Security ──────────────────────────────────────────────────────
    jwt_secret: str = "change-me-in-production"
    jwt_algorithm: str = "HS256"
    jwt_expire_minutes: int = 60

    # ── CORS ──────────────────────────────────────────────────────────
    cors_origins: list[str] = ["*"]

    # ── Stripe ────────────────────────────────────────────────────────
    stripe_secret_key: str = ""
    stripe_webhook_secret: str = ""
    stripe_price_monthly: str = ""
    stripe_price_yearly: str = ""

    # ── Rate limiting ─────────────────────────────────────────────────
    rate_limit_tokens: int = 120
    rate_limit_refill_seconds: float = 60.0

    # ── Prometheus ────────────────────────────────────────────────────
    metrics_enabled: bool = True

    # ── Sentry / logging ──────────────────────────────────────────────
    sentry_dsn: str = ""
    log_level: str = "INFO"


settings = Settings()
