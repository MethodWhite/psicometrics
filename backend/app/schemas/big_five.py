from pydantic import BaseModel, Field


class BigFiveAnswer(BaseModel):
    question_id: int = Field(ge=1, le=120)
    value: int = Field(ge=1, le=5)


class BigFiveSubmission(BaseModel):
    answers: list[BigFiveAnswer] = Field(min_length=120, max_length=120)
    language: str = Field(default="es", pattern=r"^(es|en)$")


class BigFiveResult(BaseModel):
    scores: dict[str, float]
    facets: dict[str, float]
    profile_summary: str
    percentiles: dict[str, float]
