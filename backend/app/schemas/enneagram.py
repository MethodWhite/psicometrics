from pydantic import BaseModel, Field


class EnneagramAnswer(BaseModel):
    question_id: int = Field(ge=1, le=81)
    value: int = Field(ge=1, le=4)


class EnneagramSubmission(BaseModel):
    answers: list[EnneagramAnswer] = Field(min_length=81, max_length=81)
    language: str = Field(default="es", pattern=r"^(es|en)$")


class EnneagramResult(BaseModel):
    dominant_type: int
    wing: int
    scores: dict
    profile_summary: str
