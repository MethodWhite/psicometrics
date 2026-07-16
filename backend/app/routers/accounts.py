"""Account-specific data endpoints — saved results, evolution, CSV export."""

from __future__ import annotations

import csv
import io
from datetime import UTC, datetime
from typing import Any

from fastapi import APIRouter, HTTPException, Response

# Shared in-memory account store (populated by ``auth.py``).
from app.routers._store import _accounts_by_id

router = APIRouter(prefix="/api/v1/accounts", tags=["Accounts"])

# ── In-memory stores ──────────────────────────────────────────────────────
# Keyed by ``(account_id, result_id)`` for easy lookup.
_saved_results: dict[str, list[dict[str, Any]]] = {}  # account_id -> [result, …]


@router.get("/{account_id}/results")
async def list_results(account_id: str):
    """Return all saved test results for an account."""
    if account_id not in _accounts_by_id:
        raise HTTPException(status_code=404, detail="Account not found")

    results = _saved_results.get(account_id, [])
    return {
        "account_id": account_id,
        "results": [
            {
                "id": r["id"],
                "test_type": r["test_type"],
                "created_at": r["created_at"],
                "summary": r.get("summary", {}),
            }
            for r in results
        ],
        "total": len(results),
    }


@router.post("/{account_id}/results")
async def save_result(account_id: str, payload: dict):
    """Save a test result to the account's history."""
    if account_id not in _accounts_by_id:
        raise HTTPException(status_code=404, detail="Account not found")

    import uuid

    result_entry: dict[str, Any] = {
        "id": str(uuid.uuid4()),
        "account_id": account_id,
        "test_type": payload.get("test_type", ""),
        "result": payload.get("result", {}),
        "summary": payload.get("summary", {}),
        "created_at": datetime.now(UTC).isoformat(),
    }
    _saved_results.setdefault(account_id, []).append(result_entry)

    return {
        "status": "saved",
        "result_id": result_entry["id"],
        "created_at": result_entry["created_at"],
    }


@router.get("/{account_id}/evolution/{test_type}")
async def get_evolution(account_id: str, test_type: str):
    """Return scores over time for a given test type (evolution tracking)."""
    if account_id not in _accounts_by_id:
        raise HTTPException(status_code=404, detail="Account not found")

    relevant = [
        r
        for r in _saved_results.get(account_id, [])
        if r["test_type"] == test_type
    ]
    if not relevant:
        return {
            "account_id": account_id,
            "test_type": test_type,
            "data_points": [],
            "message": "No results yet for this test type.",
        }

    return {
        "account_id": account_id,
        "test_type": test_type,
        "data_points": [
            {
                "date": r["created_at"],
                "scores": r.get("result", {}).get("scores", r.get("result", {})),
                "result_id": r["id"],
            }
            for r in sorted(relevant, key=lambda x: x["created_at"])
        ],
    }


@router.get("/{account_id}/export/csv")
async def export_csv(account_id: str):
    """Export all test results as a CSV file."""
    if account_id not in _accounts_by_id:
        raise HTTPException(status_code=404, detail="Account not found")

    results = _saved_results.get(account_id, [])
    output = io.StringIO()
    writer = csv.writer(output)
    writer.writerow(["result_id", "test_type", "created_at", "scores"])

    for r in results:
        scores = r.get("result", {}).get("scores", {})
        writer.writerow([
            r["id"],
            r["test_type"],
            r["created_at"],
            str(scores),
        ])

    csv_bytes = output.getvalue().encode("utf-8")
    return Response(
        content=csv_bytes,
        media_type="text/csv; charset=utf-8",
        headers={"Content-Disposition": f'attachment; filename="account_{account_id}_results.csv"'},
    )
