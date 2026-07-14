from pydantic import BaseModel, Field


class HealthResponse(BaseModel):
    status: str = "ok"
    version: str
    database: str = "connected"


class ErrorResponse(BaseModel):
    detail: str
    code: str
