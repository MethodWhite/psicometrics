from pydantic import BaseModel, Field


class HumanDesignSubmission(BaseModel):
    birth_date: str = Field(pattern=r"^\d{4}-\d{2}-\d{2}$")
    birth_time: str = Field(pattern=r"^\d{2}:\d{2}$")
    birth_location: str = Field(min_length=1, max_length=200)
    language: str = Field(default="es", pattern=r"^(es|en)$")


class HumanDesignCenter(BaseModel):
    defined: bool
    name: str


class HumanDesignResult(BaseModel):
    type: str
    type_info: dict
    strategy: str
    authority: str
    authority_info: dict
    profile: str
    profile_info: dict
    centers: dict[str, bool]
    personality_gates: list[int]
    design_gates: list[int]
    summary: str
