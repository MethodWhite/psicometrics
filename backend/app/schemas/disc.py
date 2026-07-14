from pydantic import BaseModel, Field


class DISCAnswer(BaseModel):
    question_id: int = Field(ge=1, le=28)
    value: str = Field(pattern=r"^[abcd]$")


class DISCSubmission(BaseModel):
    answers: list[DISCAnswer] = Field(min_length=28, max_length=28)
    language: str = Field(default="es", pattern=r"^(es|en)$")


class DISCResult(BaseModel):
    primary_style: str
    secondary_style: str
    scores: dict[str, float]
    profile_summary: str
