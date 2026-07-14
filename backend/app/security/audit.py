"""Audit logging with hash chain integrity — adapted from Synapsis audit_chain."""

import hashlib
import json
import time
import uuid
from dataclasses import dataclass, field


@dataclass
class AuditEntry:
    session_id: str | None
    action: str
    timestamp: float = field(default_factory=time.time)
    metadata: dict | None = None
    previous_hash: str | None = None
    current_hash: str = ""


class AuditLogger:
    def __init__(self):
        self._entries: list[AuditEntry] = []
        self._last_hash: str = "genesis"

    def _compute_hash(self, entry: AuditEntry) -> str:
        payload = json.dumps(
            {
                "session_id": entry.session_id,
                "action": entry.action,
                "timestamp": entry.timestamp,
                "metadata": entry.metadata,
                "previous_hash": entry.previous_hash,
            },
            sort_keys=True,
            default=str,
        )
        return hashlib.sha256(payload.encode()).hexdigest()

    def log(self, action: str, session_id: str | None = None, metadata: dict | None = None) -> AuditEntry:
        entry = AuditEntry(
            session_id=session_id,
            action=action,
            metadata=metadata,
            previous_hash=self._last_hash,
        )
        entry.current_hash = self._compute_hash(entry)
        self._last_hash = entry.current_hash
        self._entries.append(entry)
        return entry

    def verify_chain(self) -> bool:
        prev = "genesis"
        for entry in self._entries:
            if entry.previous_hash != prev:
                return False
            if entry.current_hash != self._compute_hash(entry):
                return False
            prev = entry.current_hash
        return True

    def get_entries(self, session_id: str | None = None) -> list[AuditEntry]:
        if session_id:
            return [e for e in self._entries if e.session_id == session_id]
        return list(self._entries)


audit_logger = AuditLogger()
