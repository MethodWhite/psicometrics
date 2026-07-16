"""In-memory analytics store with thread-safe event tracking.

Provides ``AnalyticsStore`` for tracking application events and
generating a dashboard summary.
"""

from __future__ import annotations

import threading
from collections import Counter, defaultdict
from datetime import datetime, timezone
from typing import Any

from app.config import settings


class AnalyticsStore:
    """Thread-safe in-memory event store for analytics.

    Events are kept as dicts in a list.  For production use,
    replace with a persistent store (e.g. ClickHouse, PostgreSQL).

    Usage::

        store = AnalyticsStore()
        store.track_event("test_completed", {"test_type": "big_five"})
        dashboard = store.get_dashboard()
    """

    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._events: list[dict[str, Any]] = []

        # Derived counters (updated on each event)
        self._total_users: int = 0
        self._total_tests: int = 0
        self._test_counts: Counter[str] = Counter()
        self._test_attempts: Counter[str] = Counter()
        self._registration_count: int = 0
        self._premium_signups: int = 0
        self._pro_signups: int = 0

    # ── event tracking ────────────────────────────────────────────────

    def track_event(self, name: str,
                    properties: dict[str, Any] | None = None) -> None:
        """Record an analytics event.

        Known event names:
            ``account_registered``
            ``test_started``
            ``test_completed``
            ``test_abandoned``
            ``report_downloaded``
            ``subscription_created``
            ``subscription_cancelled``
            ``login``
            ``password_reset``
        """
        props = properties or {}
        now = datetime.now(timezone.utc).isoformat()

        event: dict[str, Any] = {
            "name": name,
            "timestamp": now,
            "properties": props,
        }

        with self._lock:
            self._events.append(event)
            self._update_derived(name, props)

    def _update_derived(self, name: str,
                        props: dict[str, Any]) -> None:
        """Update aggregate counters (called under lock)."""
        if name == "account_registered":
            self._total_users += 1
            self._registration_count += 1
        elif name == "test_completed":
            self._total_tests += 1
            test_type = props.get("test_type", "unknown")
            self._test_counts[test_type] += 1
        elif name == "test_started":
            test_type = props.get("test_type", "unknown")
            self._test_attempts[test_type] += 1
        elif name == "subscription_created":
            tier = props.get("tier", "")
            if tier == "premium":
                self._premium_signups += 1
            elif tier == "pro":
                self._pro_signups += 1

    def get_dashboard(self) -> dict[str, Any]:
        """Return a snapshot of key metrics.

        Returns
        -------
        dict with keys:
            total_users, total_tests, popular_tests (top 5),
            conversion_rate (paid / registered),
            registrations_today, tests_today
        """
        with self._lock:
            total = self._total_users
            paid = self._premium_signups + self._pro_signups
            conversion = round(paid / total * 100, 1) if total else 0.0
            today = datetime.now(timezone.utc).date().isoformat()
            reg_today = sum(
                1 for e in self._events
                if e["name"] == "account_registered"
                and e["timestamp"].startswith(today)
            )
            tests_today = sum(
                1 for e in self._events
                if e["name"] == "test_completed"
                and e["timestamp"].startswith(today)
            )
            popular = self._test_counts.most_common(5)

        return {
            "total_users": total,
            "total_tests": self._total_tests,
            "popular_tests": [
                {"test_type": t, "count": c} for t, c in popular
            ],
            "conversion_rate": conversion,
            "premium_users": self._premium_signups,
            "pro_users": self._pro_signups,
            "registrations_today": reg_today,
            "tests_today": tests_today,
        }
