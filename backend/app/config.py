from functools import lru_cache

from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    model_config = {"env_file": ".env", "env_file_encoding": "utf-8"}

    # Application
    app_name: str = "PsicoMetrics"
    app_version: str = "1.0.0"
    debug: bool = False

    # Database
    database_url: str = "postgresql+asyncpg://synapsis:secret@localhost:5432/psicometrics"
    database_pool_size: int = 10
    database_max_overflow: int = 20

    # Redis
    redis_url: str = "redis://localhost:6379/1"

    # Security
    secret_key: str = "CHANGE-ME-IN-PRODUCTION"
    csrf_token_expiry_minutes: int = 30
    session_expiry_hours: int = 24
    rate_limit_requests_per_minute: int = 30
    rate_limit_submissions_per_minute: int = 5

    # CORS
    allowed_origins: list[str] = ["http://localhost:3000", "http://localhost:5173"]

    # CSP
    csp_report_uri: str = ""


@lru_cache
def get_settings() -> Settings:
    return Settings()
