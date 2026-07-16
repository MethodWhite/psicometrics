"""Analytics / telemetry endpoints."""

from __future__ import annotations

from datetime import UTC, datetime
from typing import Any

from fastapi import APIRouter

router = APIRouter(prefix="/api/v1/analytics", tags=["Analytics"])

# ── In-memory stores ──────────────────────────────────────────────────────
_events: list[dict[str, Any]] = []


@router.post("/track")
async def track_event(payload: dict):
    """Track a client-side event (page view, feature usage, etc.).

    Payload example:
    ```json
    {
      "event": "test_started",
      "properties": {"test_type": "big_five"},
      "account_id": "uuid-here"  // optional
    }
    ```
    """
    _events.append({
        "event": payload.get("event", "unknown"),
        "properties": payload.get("properties", {}),
        "account_id": payload.get("account_id"),
        "timestamp": datetime.now(UTC).isoformat(),
    })
    return {"status": "tracked"}


@router.get("/dashboard")
async def analytics_dashboard():
    """Return a summary dashboard of tracked events."""
    total = len(_events)
    by_event: dict[str, int] = {}
    for e in _events:
        by_event[e["event"]] = by_event.get(e["event"], 0) + 1

    return {
        "total_events": total,
        "events_by_type": by_event,
        "unique_accounts": len({e["account_id"] for e in _events if e["account_id"]}),
        "last_event": _events[-1] if _events else None,
    }
