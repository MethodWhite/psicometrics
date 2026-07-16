"""In-memory community store for forums, testimonials, and stories.

Thread-safe CRUD for forum posts with comments, testimonials,
success stories, and article comments.  Includes ``seed_data()``
to populate sample content.
"""

from __future__ import annotations

import threading
from datetime import datetime, timezone
from typing import Any
from uuid import uuid4


def _utcnow() -> str:
    return datetime.now(timezone.utc).isoformat()


def _new_id() -> str:
    return uuid4().hex[:12]


class CommunityStore:
    """Thread-safe in-memory community content store."""

    def __init__(self) -> None:
        self._lock = threading.Lock()

        # Forum posts: post_id -> dict
        self._posts: dict[str, dict[str, Any]] = {}
        # Comments: comment_id -> dict
        self._comments: dict[str, dict[str, Any]] = {}
        # Testimonials
        self._testimonials: list[dict[str, Any]] = []
        # Success stories
        self._stories: list[dict[str, Any]] = []
        # Article comments
        self._article_comments: dict[str, list[dict[str, Any]]] = {}

    # ── forum posts ───────────────────────────────────────────────────

    def create_post(self, author: str, title: str,
                    body: str, mbti_type: str = "",
                    tags: list[str] | None = None) -> dict[str, Any]:
        post: dict[str, Any] = {
            "id": _new_id(),
            "author": author,
            "title": title,
            "body": body,
            "mbti_type": mbti_type,
            "tags": tags or [],
            "created_at": _utcnow(),
            "updated_at": _utcnow(),
            "comment_ids": [],
            "upvotes": 0,
        }
        with self._lock:
            self._posts[post["id"]] = post
        return post

    def get_post(self, post_id: str) -> dict[str, Any] | None:
        return self._posts.get(post_id)

    def list_posts(self, mbti_type: str = "",
                   tag: str = "",
                   limit: int = 50) -> list[dict[str, Any]]:
        results = list(self._posts.values())
        if mbti_type:
            results = [p for p in results
                       if p.get("mbti_type", "").upper() == mbti_type.upper()]
        if tag:
            results = [p for p in results if tag in p.get("tags", [])]
        results.sort(key=lambda p: p.get("created_at", ""), reverse=True)
        return results[:limit]

    def delete_post(self, post_id: str) -> bool:
        with self._lock:
            post = self._posts.pop(post_id, None)
            if post is None:
                return False
            for cid in post.get("comment_ids", []):
                self._comments.pop(cid, None)
        return True

    def upvote_post(self, post_id: str) -> dict[str, Any] | None:
        with self._lock:
            post = self._posts.get(post_id)
            if post:
                post["upvotes"] = post.get("upvotes", 0) + 1
            return post

    # ── comments (on forum posts) ─────────────────────────────────────

    def add_comment(self, post_id: str, author: str,
                    body: str) -> dict[str, Any] | None:
        comment: dict[str, Any] = {
            "id": _new_id(),
            "post_id": post_id,
            "author": author,
            "body": body,
            "created_at": _utcnow(),
        }
        with self._lock:
            if post_id not in self._posts:
                return None
            self._comments[comment["id"]] = comment
            self._posts[post_id].setdefault("comment_ids", []).append(
                comment["id"]
            )
        return comment

    def get_comments(self, post_id: str) -> list[dict[str, Any]]:
        post = self._posts.get(post_id)
        if not post:
            return []
        return [
            self._comments[cid] for cid in post.get("comment_ids", [])
            if cid in self._comments
        ]

    def delete_comment(self, comment_id: str,
                       post_id: str | None = None) -> bool:
        with self._lock:
            comment = self._comments.pop(comment_id, None)
            if comment is None:
                return False
            pid = post_id or comment.get("post_id", "")
            post = self._posts.get(pid)
            if post and comment_id in post.get("comment_ids", []):
                post["comment_ids"].remove(comment_id)
        return True

    # ── testimonials ──────────────────────────────────────────────────

    def add_testimonial(self, author: str, text: str,
                        rating: int = 5,
                        mbti_type: str = "") -> dict[str, Any]:
        entry: dict[str, Any] = {
            "id": _new_id(),
            "author": author,
            "text": text,
            "rating": max(1, min(5, rating)),
            "mbti_type": mbti_type,
            "created_at": _utcnow(),
        }
        with self._lock:
            self._testimonials.append(entry)
        return entry

    def list_testimonials(self, limit: int = 20) -> list[dict[str, Any]]:
        with self._lock:
            return sorted(
                self._testimonials,
                key=lambda t: t.get("created_at", ""),
                reverse=True,
            )[:limit]

    # ── success stories ───────────────────────────────────────────────

    def add_story(self, author: str, title: str,
                  body: str, mbti_type: str = "") -> dict[str, Any]:
        story: dict[str, Any] = {
            "id": _new_id(),
            "author": author,
            "title": title,
            "body": body,
            "mbti_type": mbti_type,
            "created_at": _utcnow(),
        }
        with self._lock:
            self._stories.append(story)
        return story

    def list_stories(self, limit: int = 20) -> list[dict[str, Any]]:
        with self._lock:
            return sorted(
                self._stories,
                key=lambda s: s.get("created_at", ""),
                reverse=True,
            )[:limit]

    # ── article comments ──────────────────────────────────────────────

    def add_article_comment(self, article_id: str, author: str,
                            body: str) -> dict[str, Any]:
        comment: dict[str, Any] = {
            "id": _new_id(),
            "article_id": article_id,
            "author": author,
            "body": body,
            "created_at": _utcnow(),
        }
        with self._lock:
            self._article_comments.setdefault(article_id, []).append(comment)
        return comment

    def get_article_comments(
        self, article_id: str,
    ) -> list[dict[str, Any]]:
        return list(self._article_comments.get(article_id, []))

    # ── type stats ────────────────────────────────────────────────────

    def get_type_stats(self,
                       mbti_type: str) -> dict[str, Any]:
        """Return aggregate stats for a given MBTI type."""
        mt = mbti_type.upper()
        with self._lock:
            posts = [
                p for p in self._posts.values()
                if p.get("mbti_type", "").upper() == mt
            ]
            testimonials = [
                t for t in self._testimonials
                if t.get("mbti_type", "").upper() == mt
            ]
            stories = [
                s for s in self._stories
                if s.get("mbti_type", "").upper() == mt
            ]

        return {
            "mbti_type": mt,
            "forum_posts": len(posts),
            "testimonials": len(testimonials),
            "stories": len(stories),
        }

    # ── seed data ─────────────────────────────────────────────────────

    def seed_data(self) -> None:
        """Populate the store with sample content."""
        if self._posts:
            return  # already seeded

        # Forum posts
        p1 = self.create_post(
            "Alex", "INTJ here — how do you handle small talk?",
            "I find small talk draining. Any tips from other types?",
            mbti_type="INTJ", tags=["social", "small-talk"],
        )
        p2 = self.create_post(
            "Sam", "ENFP appreciation thread",
            "Let's share what we love about being ENFP!",
            mbti_type="ENFP", tags=["positive", "enfp"],
        )
        p3 = self.create_post(
            "Jordan", "How did you discover your type?",
            "Share your story of finding MBTI.",
            tags=["discussion", "discovery"],
        )

        # Comments
        self.add_comment(p1["id"], "Taylor",
                         "ISTP here — I just nod and smile.")
        self.add_comment(p2["id"], "Riley",
                         "ENFP energy is contagious!")
        self.add_comment(p3["id"], "Casey",
                         "I took the test on Psicometrics!")

        # Testimonials
        self.add_testimonial(
            "Maria", "Psicometrics helped me understand my team better!",
            rating=5, mbti_type="ENFJ",
        )
        self.add_testimonial(
            "Carlos", "The Big Five report was incredibly accurate.",
            rating=4, mbti_type="ISTJ",
        )

        # Success stories
        self.add_story(
            "Emma", "From conflict to collaboration",
            "Understanding my partner's ENFP type vs my INTJ...",
            mbti_type="INTJ",
        )

        # Article comments
        self.add_article_comment(
            "article-big-five-101", "Luis",
            "Great intro to the Big Five model!"
        )
