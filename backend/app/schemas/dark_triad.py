from pydantic import BaseModel, Field


class DarkTriadAnswer(BaseModel):
    question_id: int = Field(ge=1, le=27)
    value: int = Field(ge=1, le=5)


class DarkTriadSubmission(BaseModel):
    answers: list[DarkTriadAnswer] = Field(min_length=27, max_length=27)
    language: str = Field(default="es", pattern=r"^(es|en)$")


class DarkTriadResult(BaseModel):
    scores: dict[str, float]
    dark_core: float = Field(ge=0, le=100)
    risk_level: str
    profile_summary: str
