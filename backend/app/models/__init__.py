"""Pydantic models for the Psicometrics API."""

from __future__ import annotations

import uuid
from datetime import datetime
from typing import Any, Literal

from pydantic import BaseModel, EmailStr, Field


# ═══════════════════════════════════════════════════════════════════════
# Test metadata
# ═══════════════════════════════════════════════════════════════════════

TestType = Literal[
    "big_five",
    "mbti",
    "enneagram",
    "disc",
    "dark_triad",
    "human_design",
    "love_languages",
    "attachment_style",
    "eq",
    "emotional_intelligence",
    "career_aptitude",
    "via_strengths",
]

TEST_TYPES: tuple[TestType, ...] = (
    "big_five",
    "mbti",
    "enneagram",
    "disc",
    "dark_triad",
    "human_design",
    "love_languages",
    "attachment_style",
    "emotional_intelligence",
    "eq",
    "career_aptitude",
    "via_strengths",
)


class TestMetadata(BaseModel):
    test_type: TestType
    name: dict[str, str]
    description: str
    question_count: int
    duration_minutes: int = 0


class TestInfo(BaseModel):
    test_type: TestType
    name: dict[str, str]
    description: str
    questions: list[dict[str, Any]]


class TestListResponse(BaseModel):
    tests: list[TestMetadata]


# ═══════════════════════════════════════════════════════════════════════
# Shared submission / result
# ═══════════════════════════════════════════════════════════════════════


class TestSubmission(BaseModel):
    test_type: TestType
    answers: list[dict[str, Any]]


class TestResult(BaseModel):
    id: uuid.UUID
    test_type: TestType
    result: dict[str, Any]
    created_at: datetime


class TestResultList(BaseModel):
    results: list[TestResult]


# ═══════════════════════════════════════════════════════════════════════
# Big Five / OCEAN
# ═══════════════════════════════════════════════════════════════════════


class BigFiveScores(BaseModel):
    openness: float
    conscientiousness: float
    extraversion: float
    agreeableness: float
    neuroticism: float


class BigFiveFacets(BaseModel):
    openness_facets: dict[str, float] = {}
    conscientiousness_facets: dict[str, float] = {}
    extraversion_facets: dict[str, float] = {}
    agreeableness_facets: dict[str, float] = {}
    neuroticism_facets: dict[str, float] = {}


class BigFiveResult(BaseModel):
    scores: BigFiveScores
    facets: BigFiveFacets | None = None
    interpretation: dict[str, Any] = {}


# ═══════════════════════════════════════════════════════════════════════
# MBTI
# ═══════════════════════════════════════════════════════════════════════


class MBTIDimensions(BaseModel):
    extraversion: float  # E vs I
    sensing: float  # S vs N
    thinking: float  # T vs F
    judging: float  # J vs P


class MBTIResult(BaseModel):
    type: str  # e.g. "INTJ"
    dimensions: MBTIDimensions
    description: dict[str, str] = {}


# ═══════════════════════════════════════════════════════════════════════
# Enneagram
# ═══════════════════════════════════════════════════════════════════════


class EnneagramResult(BaseModel):
    primary_type: int  # 1-9
    wing: int | None = None
    scores: dict[str, float]  # type_number -> score
    tritype: list[int] | None = None
    description: dict[str, str] = {}


# ═══════════════════════════════════════════════════════════════════════
# DISC
# ═══════════════════════════════════════════════════════════════════════


class DISCScores(BaseModel):
    dominance: float
    influence: float
    steadiness: float
    conscientiousness: float


class DISCResult(BaseModel):
    scores: DISCScores
    primary_style: str  # D / I / S / C
    description: dict[str, str] = {}


# ═══════════════════════════════════════════════════════════════════════
# Dark Triad
# ═══════════════════════════════════════════════════════════════════════


class DarkTriadScores(BaseModel):
    narcissism: float
    machiavellianism: float
    psychopathy: float


class DarkTriadResult(BaseModel):
    scores: DarkTriadScores
    interpretation: dict[str, Any] = {}


# ═══════════════════════════════════════════════════════════════════════
# Human Design
# ═══════════════════════════════════════════════════════════════════════


class HumanDesignResult(BaseModel):
    type: str  # Generator / Manifestor / Projector / Reflector
    strategy: str
    authority: str
    profile: str
    definition: str
    centers: dict[str, Any] = {}
    channels: list[dict[str, Any]] = []
    gates: list[dict[str, Any]] = []
    description: dict[str, str] = {}


# ═══════════════════════════════════════════════════════════════════════
# Love Languages
# ═══════════════════════════════════════════════════════════════════════


class LoveLanguagesScores(BaseModel):
    words_of_affirmation: float = 0
    quality_time: float = 0
    receiving_gifts: float = 0
    acts_of_service: float = 0
    physical_touch: float = 0


class LoveLanguagesResult(BaseModel):
    scores: LoveLanguagesScores
    primary_language: str
    description: dict[str, str] = {}


# ═══════════════════════════════════════════════════════════════════════
# Attachment Style
# ═══════════════════════════════════════════════════════════════════════


class AttachmentStyleScores(BaseModel):
    secure: float
    anxious: float
    avoidant: float
    fearful_avoidant: float = 0


class AttachmentStyleResult(BaseModel):
    scores: AttachmentStyleScores
    primary_style: str  # secure / anxious / avoidant / fearful_avoidant
    description: dict[str, str] = {}


# ═══════════════════════════════════════════════════════════════════════
# Emotional Intelligence (EQ)
# ═══════════════════════════════════════════════════════════════════════


class EQDimensions(BaseModel):
    self_awareness: float
    self_regulation: float
    motivation: float
    empathy: float
    social_skills: float


class EQResult(BaseModel):
    scores: EQDimensions
    overall: float
    interpretation: dict[str, Any] = {}


# ═══════════════════════════════════════════════════════════════════════
# Career Aptitude
# ═══════════════════════════════════════════════════════════════════════


class CareerAptitudeScores(BaseModel):
    realistic: float
    investigative: float
    artistic: float
    social: float
    enterprising: float
    conventional: float


class CareerAptitudeResult(BaseModel):
    scores: CareerAptitudeScores
    top_careers: list[str] = []
    description: dict[str, str] = {}


# ═══════════════════════════════════════════════════════════════════════
# VIA Character Strengths
# ═══════════════════════════════════════════════════════════════════════


class VIAStrengthsResult(BaseModel):
    strengths: list[dict[str, Any]]  # sorted by score descending
    top_strengths: list[str] = []
    categories: dict[str, float] = {}
    interpretation: dict[str, Any] = {}


# ═══════════════════════════════════════════════════════════════════════
# Account
# ═══════════════════════════════════════════════════════════════════════


class AccountCreate(BaseModel):
    email: EmailStr
    password: str = Field(min_length=8, max_length=128)


class AccountLogin(BaseModel):
    email: EmailStr
    password: str


class AccountResponse(BaseModel):
    id: uuid.UUID
    email: str
    created_at: datetime


class TokenResponse(BaseModel):
    access_token: str
    token_type: str = "bearer"
    account: AccountResponse


# ═══════════════════════════════════════════════════════════════════════
# Community
# ═══════════════════════════════════════════════════════════════════════


class ForumPost(BaseModel):
    id: uuid.UUID
    account_id: uuid.UUID
    title: str
    body: str
    tags: list[str] = []
    created_at: datetime
    updated_at: datetime | None = None


class ForumPostCreate(BaseModel):
    title: str = Field(min_length=1, max_length=200)
    body: str = Field(min_length=1)
    tags: list[str] = []


class Comment(BaseModel):
    id: uuid.UUID
    account_id: uuid.UUID
    post_id: uuid.UUID
    body: str
    created_at: datetime


class CommentCreate(BaseModel):
    body: str = Field(min_length=1)


class ArticleComment(BaseModel):
    id: uuid.UUID
    account_id: uuid.UUID
    article_slug: str
    body: str
    created_at: datetime


class ArticleCommentCreate(BaseModel):
    article_slug: str
    body: str = Field(min_length=1)


class Testimonial(BaseModel):
    id: uuid.UUID
    account_id: uuid.UUID
    text: str
    test_type: TestType | None = None
    is_approved: bool = False
    created_at: datetime


class TestimonialCreate(BaseModel):
    text: str = Field(min_length=1)
    test_type: TestType | None = None


class UserStory(BaseModel):
    id: uuid.UUID
    account_id: uuid.UUID
    title: str
    story: str
    is_published: bool = False
    created_at: datetime


class UserStoryCreate(BaseModel):
    title: str = Field(min_length=1, max_length=200)
    story: str = Field(min_length=1)


# ═══════════════════════════════════════════════════════════════════════
# Payments
# ═══════════════════════════════════════════════════════════════════════


class SubscriptionTier(BaseModel):
    id: str
    name: str
    description: str
    price_monthly_cents: int
    price_yearly_cents: int
    features: list[str]


class CheckoutSession(BaseModel):
    url: str
    session_id: str


# ═══════════════════════════════════════════════════════════════════════
# Analytics
# ═══════════════════════════════════════════════════════════════════════


class AnalyticsEvent(BaseModel):
    event: str
    category: str = "general"
    label: str = ""
    value: float | None = None
    metadata: dict[str, Any] = {}


class AnalyticsSummary(BaseModel):
    total_users: int
    total_results: int
    results_by_test: dict[str, int]
    registrations_last_7d: int
    active_users_last_7d: int
