from __future__ import annotations

import hashlib
import json
import uuid
from datetime import UTC, datetime
from typing import Any


class AuditLogEntry:
    """A single audit entry with hash-chain integrity."""

    __slots__ = (
        "id",
        "timestamp",
        "actor_id",
        "action",
        "resource",
        "detail",
        "previous_hash",
        "_hash",
    )

    def __init__(
        self,
        actor_id: str,
        action: str,
        resource: str,
        detail: dict[str, Any] | None = None,
        previous_hash: str = "",
    ) -> None:
        self.id = uuid.uuid4().hex[:12]
        self.timestamp = datetime.now(UTC)
        self.actor_id = actor_id
        self.action = action
        self.resource = resource
        self.detail = detail or {}
        self.previous_hash = previous_hash
        self._hash = self._compute_hash()

    def _compute_hash(self) -> str:
        payload = {
            "id": self.id,
            "timestamp": self.timestamp.isoformat(),
            "actor_id": self.actor_id,
            "action": self.action,
            "resource": self.resource,
            "detail": self.detail,
            "previous_hash": self.previous_hash,
        }
        return hashlib.sha256(
            json.dumps(payload, sort_keys=True, default=str).encode()
        ).hexdigest()

    @property
    def hash(self) -> str:
        return self._hash

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "timestamp": self.timestamp.isoformat(),
            "actor_id": self.actor_id,
            "action": self.action,
            "resource": self.resource,
            "detail": self.detail,
            "previous_hash": self.previous_hash,
            "hash": self._hash,
        }


class AuditLogger:
    """Append-only in-memory audit log with hash-chain verification."""

    def __init__(self) -> None:
        self._entries: list[AuditLogEntry] = []
        self._last_hash = ""

    def log(
        self,
        actor_id: str,
        action: str,
        resource: str,
        detail: dict[str, Any] | None = None,
    ) -> AuditLogEntry:
        entry = AuditLogEntry(actor_id, action, resource, detail, self._last_hash)
        self._entries.append(entry)
        self._last_hash = entry.hash
        return entry

    def verify_chain(self) -> bool:
        computed = ""
        for entry in self._entries:
            expected = entry._compute_hash()
            if entry.previous_hash != computed or entry.hash != expected:
                return False
            computed = entry.hash
        return True

    def recent(self, limit: int = 50) -> list[dict[str, Any]]:
        return [e.to_dict() for e in self._entries[-limit:]]

    @property
    def count(self) -> int:
        return len(self._entries)
