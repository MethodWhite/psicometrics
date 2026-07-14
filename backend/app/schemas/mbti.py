from pydantic import BaseModel, Field


class MBTIAnswer(BaseModel):
    question_id: int = Field(ge=1, le=72)
    value: str = Field(pattern=r"^[ab]$")


class MBTISubmission(BaseModel):
    answers: list[MBTIAnswer] = Field(min_length=72, max_length=72)
    language: str = Field(default="es", pattern=r"^(es|en)$")


class MBTIResult(BaseModel):
    type_code: str
    scores: dict[str, float]
    profile_summary: str
    percentages: dict[str, float]
