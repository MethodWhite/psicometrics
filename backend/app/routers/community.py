"""Community endpoints — forum, testimonials, stories, MBTI stats, blog comments."""

from __future__ import annotations

import uuid
from datetime import UTC, datetime
from typing import Any

from fastapi import APIRouter, HTTPException

# ── Routers ────────────────────────────────────────────────────────────────
# Split into two logical groups: community features and blog comments.

community_router = APIRouter(prefix="/api/v1/community", tags=["Community"])
blog_router = APIRouter(prefix="/api/v1/blog", tags=["Blog"])

# The package's ``__init__.py`` re-exports both as ``community_router`` and
# ``blog_router``.  We re-alias here for the ``__init__.py`` import.
router = community_router

# ── In-memory stores ──────────────────────────────────────────────────────
_posts: dict[str, dict[str, Any]] = {}  # post_id -> post
_post_comments: dict[str, list[dict[str, Any]]] = {}  # post_id -> comments
_testimonials: list[dict[str, Any]] = []
_stories: dict[str, dict[str, Any]] = {}  # story_id -> story
_blog_comments: dict[str, list[dict[str, Any]]] = {}  # blog_slug -> comments


# =========================================================================
# Community: Forum posts
# =========================================================================


@community_router.get("/posts")
async def list_posts():
    """List all forum posts (newest first)."""
    sorted_posts = sorted(_posts.values(), key=lambda p: p["created_at"], reverse=True)
    return {
        "posts": [
            {
                "id": pid,
                "title": p.get("title", ""),
                "author": p.get("author", ""),
                "excerpt": (p.get("content", "")[:200] + "…") if len(p.get("content", "")) > 200 else p.get("content", ""),
                "likes": len(p.get("liked_by", [])),
                "comment_count": len(_post_comments.get(pid, [])),
                "created_at": p["created_at"],
            }
            for pid, p in sorted_posts
        ],
        "total": len(_posts),
    }


@community_router.post("/posts")
async def create_post(payload: dict):
    """Create a new forum post."""
    title = payload.get("title", "").strip()
    content = payload.get("content", "").strip()
    author = payload.get("author", "anonymous")

    if not title or not content:
        raise HTTPException(status_code=400, detail="title and content are required")

    post_id = str(uuid.uuid4())
    _posts[post_id] = {
        "id": post_id,
        "title": title,
        "content": content,
        "author": author,
        "tags": payload.get("tags", []),
        "liked_by": [],
        "created_at": datetime.now(UTC).isoformat(),
        "updated_at": datetime.now(UTC).isoformat(),
    }
    _post_comments[post_id] = []

    return {
        "id": post_id,
        "title": title,
        "author": author,
        "created_at": _posts[post_id]["created_at"],
    }


@community_router.get("/posts/{post_id}")
async def get_post(post_id: str):
    """Get a single forum post by ID with full content."""
    post = _posts.get(post_id)
    if not post:
        raise HTTPException(status_code=404, detail="Post not found")

    return {
        **post,
        "likes": len(post.get("liked_by", [])),
        "comments": _post_comments.get(post_id, []),
    }


@community_router.post("/posts/{post_id}/like")
async def like_post(post_id: str, payload: dict):
    """Toggle a like on a forum post."""
    post = _posts.get(post_id)
    if not post:
        raise HTTPException(status_code=404, detail="Post not found")

    user = payload.get("user", "anonymous")
    liked_by: list[str] = post.setdefault("liked_by", [])

    if user in liked_by:
        liked_by.remove(user)
        action = "unliked"
    else:
        liked_by.append(user)
        action = "liked"

    return {"action": action, "likes": len(liked_by)}


@community_router.get("/posts/{post_id}/comments")
async def list_post_comments(post_id: str):
    """List all comments on a forum post."""
    if post_id not in _posts:
        raise HTTPException(status_code=404, detail="Post not found")
    return {"comments": _post_comments.get(post_id, [])}


@community_router.post("/posts/{post_id}/comments")
async def create_post_comment(post_id: str, payload: dict):
    """Add a comment to a forum post."""
    if post_id not in _posts:
        raise HTTPException(status_code=404, detail="Post not found")

    content = payload.get("content", "").strip()
    author = payload.get("author", "anonymous")
    if not content:
        raise HTTPException(status_code=400, detail="content is required")

    comment: dict[str, Any] = {
        "id": str(uuid.uuid4()),
        "post_id": post_id,
        "author": author,
        "content": content,
        "created_at": datetime.now(UTC).isoformat(),
    }
    _post_comments.setdefault(post_id, []).append(comment)
    return comment


# =========================================================================
# Community: Testimonials
# =========================================================================


@community_router.get("/testimonials")
async def list_testimonials():
    """List all user testimonials."""
    return {"testimonials": _testimonials, "total": len(_testimonials)}


@community_router.post("/testimonials")
async def create_testimonial(payload: dict):
    """Submit a user testimonial."""
    name = payload.get("name", "").strip()
    text = payload.get("text", "").strip()
    if not name or not text:
        raise HTTPException(status_code=400, detail="name and text are required")

    testimonial: dict[str, Any] = {
        "id": str(uuid.uuid4()),
        "name": name,
        "text": text,
        "test_type": payload.get("test_type", ""),
        "rating": payload.get("rating"),
        "created_at": datetime.now(UTC).isoformat(),
    }
    _testimonials.append(testimonial)
    return testimonial


# =========================================================================
# Community: Success stories
# =========================================================================


@community_router.get("/stories")
async def list_stories():
    """List all success stories."""
    sorted_stories = sorted(_stories.values(), key=lambda s: s["created_at"], reverse=True)
    return {
        "stories": [
            {
                "id": sid,
                "title": s.get("title", ""),
                "author": s.get("author", ""),
                "excerpt": (s.get("content", "")[:200] + "…") if len(s.get("content", "")) > 200 else s.get("content", ""),
                "likes": len(s.get("liked_by", [])),
                "created_at": s["created_at"],
            }
            for sid, s in sorted_stories
        ],
        "total": len(_stories),
    }


@community_router.post("/stories")
async def create_story(payload: dict):
    """Share a success story."""
    title = payload.get("title", "").strip()
    content = payload.get("content", "").strip()
    author = payload.get("author", "anonymous")
    if not title or not content:
        raise HTTPException(status_code=400, detail="title and content are required")

    story_id = str(uuid.uuid4())
    _stories[story_id] = {
        "id": story_id,
        "title": title,
        "content": content,
        "author": author,
        "test_type": payload.get("test_type", ""),
        "liked_by": [],
        "created_at": datetime.now(UTC).isoformat(),
    }
    return _stories[story_id]


@community_router.post("/stories/{story_id}/like")
async def like_story(story_id: str, payload: dict):
    """Toggle a like on a success story."""
    story = _stories.get(story_id)
    if not story:
        raise HTTPException(status_code=404, detail="Story not found")

    user = payload.get("user", "anonymous")
    liked_by: list[str] = story.setdefault("liked_by", [])

    if user in liked_by:
        liked_by.remove(user)
        action = "unliked"
    else:
        liked_by.append(user)
        action = "liked"

    return {"action": action, "likes": len(liked_by)}


# =========================================================================
# Community: MBTI type stats
# =========================================================================


@community_router.get("/types/{mbti_type}/stats")
async def mbti_type_stats(mbti_type: str):
    """Return aggregate statistics for an MBTI personality type.

    The type should be a 4-letter code (e.g. ``INFJ``, ``ENTP``).
    """
    valid_types = {
        "INTJ", "INTP", "ENTJ", "ENTP",
        "INFJ", "INFP", "ENFJ", "ENFP",
        "ISTJ", "ISFJ", "ESTJ", "ESFJ",
        "ISTP", "ISFP", "ESTP", "ESFP",
    }
    upper = mbti_type.upper()
    if upper not in valid_types:
        raise HTTPException(
            status_code=400,
            detail=f"Invalid MBTI type: {mbti_type!r}. Must be one of {sorted(valid_types)}",
        )

    return {
        "type": upper,
        "population_percentage": _MBTI_POPULATION.get(upper, 0),
        "common_careers": _MBTI_CAREERS.get(upper, []),
        "compatible_types": _MBTI_COMPAT.get(upper, []),
        "stories_count": sum(
            1 for s in _stories.values() if s.get("test_type", "").upper() == upper
        ),
    }


_MBTI_POPULATION: dict[str, float] = {
    "INTJ": 2.1, "INTP": 3.3, "ENTJ": 1.8, "ENTP": 3.2,
    "INFJ": 1.5, "INFP": 4.4, "ENFJ": 2.5, "ENFP": 8.1,
    "ISTJ": 11.6, "ISFJ": 13.8, "ESTJ": 8.7, "ESFJ": 12.3,
    "ISTP": 5.4, "ISFP": 8.8, "ESTP": 4.3, "ESFP": 8.5,
}

_MBTI_CAREERS: dict[str, list[str]] = {
    "INTJ": ["Software Engineer", "Scientist", "Lawyer", "Architect"],
    "INTP": ["Data Scientist", "Professor", "Software Developer", "Engineer"],
    "ENTJ": ["CEO", "Management Consultant", "Lawyer", "Entrepreneur"],
    "ENTP": ["Entrepreneur", "Marketing", "Lawyer", "Software Developer"],
    "INFJ": ["Counselor", "Writer", "Healthcare", "Non-profit Director"],
    "INFP": ["Writer", "Graphic Designer", "Counselor", "Teacher"],
    "ENFJ": ["Teacher", "HR Manager", "Counselor", "Sales Manager"],
    "ENFP": ["Journalist", "Actor", "Marketing", "Entrepreneur"],
    "ISTJ": ["Accountant", "Auditor", "Military Officer", "Judge"],
    "ISFJ": ["Nurse", "Teacher", "Social Worker", "Administrator"],
    "ESTJ": ["Manager", "Police Officer", "Military Officer", "Judge"],
    "ESFJ": ["Teacher", "HR Manager", "Sales", "Event Planner"],
    "ISTP": ["Engineer", "Mechanic", "Surgeon", "Pilot"],
    "ISFP": ["Artist", "Designer", "Chef", "Veterinarian"],
    "ESTP": ["Sales", "Entrepreneur", "Athlete", "Stock Trader"],
    "ESFP": ["Actor", "Sales", "Tour Guide", "Event Planner"],
}

_MBTI_COMPAT: dict[str, list[str]] = {
    "INTJ": ["ENFP", "ENTP", "INTP"],
    "INTP": ["ENTJ", "ENFJ", "INTJ"],
    "ENTJ": ["INTP", "INFP", "INTJ"],
    "ENTP": ["INFJ", "INTJ", "ENFJ"],
    "INFJ": ["ENFP", "ENTP", "INFP"],
    "INFP": ["ENFJ", "ENTJ", "INFJ"],
    "ENFJ": ["INFP", "INTP", "ENFP"],
    "ENFP": ["INFJ", "INTJ", "ENFJ"],
    "ISTJ": ["ESFP", "ESTP", "ISFJ"],
    "ISFJ": ["ESFP", "ESTJ", "ISTJ"],
    "ESTJ": ["ISFP", "ISTP", "ESFJ"],
    "ESFJ": ["ISTP", "ISFP", "ESTJ"],
    "ISTP": ["ESFJ", "ESTJ", "ISTJ"],
    "ISFP": ["ESTJ", "ESFJ", "ISTP"],
    "ESTP": ["ISFJ", "ISTJ", "ESFP"],
    "ESFP": ["ISTJ", "ISFJ", "ESTP"],
}


# =========================================================================
# Blog article comments
# =========================================================================


@blog_router.get("/{slug}/comments")
async def list_blog_comments(slug: str):
    """List all comments on a blog article."""
    return {"slug": slug, "comments": _blog_comments.get(slug, [])}


@blog_router.post("/{slug}/comments")
async def create_blog_comment(slug: str, payload: dict):
    """Add a comment to a blog article."""
    content = payload.get("content", "").strip()
    author = payload.get("author", "anonymous")
    if not content:
        raise HTTPException(status_code=400, detail="content is required")

    comment: dict[str, Any] = {
        "id": str(uuid.uuid4()),
        "slug": slug,
        "author": author,
        "content": content,
        "liked_by": [],
        "created_at": datetime.now(UTC).isoformat(),
    }
    _blog_comments.setdefault(slug, []).append(comment)
    return comment


@blog_router.post("/{slug}/comments/{comment_id}/like")
async def like_blog_comment(slug: str, comment_id: str, payload: dict):
    """Toggle a like on a blog comment."""
    comments = _blog_comments.get(slug, [])
    comment = next((c for c in comments if c["id"] == comment_id), None)
    if not comment:
        raise HTTPException(status_code=404, detail="Comment not found")

    user = payload.get("user", "anonymous")
    liked_by: list[str] = comment.setdefault("liked_by", [])

    if user in liked_by:
        liked_by.remove(user)
        action = "unliked"
    else:
        liked_by.append(user)
        action = "liked"

    return {"action": action, "likes": len(liked_by)}
