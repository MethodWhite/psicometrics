"""In-memory account store with thread-safe operations.

Provides ``AccountStore`` for registration, result storage,
evolution tracking, account deletion, and GDPR data export.
"""

from __future__ import annotations

import hashlib
import secrets
import threading
from datetime import datetime, timezone
from typing import Any
from uuid import uuid4

from argon2 import PasswordHasher
from argon2.exceptions import VerifyMismatchError

_hasher = PasswordHasher()


def _utcnow() -> str:
    return datetime.now(timezone.utc).isoformat()


def _new_id() -> str:
    return uuid4().hex[:12]


def _hash_email(email: str) -> str:
    return hashlib.sha256(email.lower().strip().encode()).hexdigest()[:16]


class AccountStore:
    """Thread-safe in-memory account store.

    For production, replace with a persistent database.

    Usage::

        store = AccountStore()
        acct = await store.register("user@example.com", "s3cret!")
        stored = store.get_account(acct["id"])
        store.save_result(acct["id"], "big_five", {"scores": {...}})
        export = store.export_data(acct["id"])
    """

    def __init__(self) -> None:
        self._lock = threading.Lock()
        # account_id -> dict
        self._accounts: dict[str, dict[str, Any]] = {}
        # email_hash -> account_id
        self._email_index: dict[str, str] = {}

    # ── registration / auth ───────────────────────────────────────────

    def register(self, email: str,
                 password: str) -> dict[str, Any]:
        """Register a new account.

        Returns the account dict (without password hash).

        Raises ``ValueError`` if the email is already registered.
        """
        email_key = _hash_email(email)
        with self._lock:
            if email_key in self._email_index:
                raise ValueError(f"Email '{email}' is already registered.")

            acct_id = _new_id()
            pw_hash = _hasher.hash(password)
            now = _utcnow()
            account: dict[str, Any] = {
                "id": acct_id,
                "email": email,
                "email_hash": email_key,
                "password_hash": pw_hash,
                "created_at": now,
                "updated_at": now,
                "tier": "free",
                "results": [],
                "is_active": True,
            }
            self._accounts[acct_id] = account
            self._email_index[email_key] = acct_id

        # Return a safe copy (no password hash)
        return self._safe_account(account)

    def authenticate(self, email: str,
                     password: str) -> dict[str, Any] | None:
        """Verify credentials and return the account or ``None``."""
        email_key = _hash_email(email)
        with self._lock:
            acct_id = self._email_index.get(email_key)
            if not acct_id:
                return None
            account = self._accounts.get(acct_id)
            if not account or not account.get("is_active", True):
                return None
            try:
                _hasher.verify(account["password_hash"], password)
            except VerifyMismatchError:
                return None
            return self._safe_account(account)

    def get_account(self, account_id: str) -> dict[str, Any] | None:
        """Return account by ID (without password hash).

        Soft-deleted (inactive) accounts are hidden from normal lookups.
        """
        with self._lock:
            acct = self._accounts.get(account_id)
            if not acct or not acct.get("is_active", True):
                return None
            return self._safe_account(acct)

    def update_tier(self, account_id: str,
                    tier: str) -> dict[str, Any] | None:
        """Update the subscription tier."""
        with self._lock:
            acct = self._accounts.get(account_id)
            if not acct:
                return None
            acct["tier"] = tier
            acct["updated_at"] = _utcnow()
            return self._safe_account(acct)

    def delete_account(self, account_id: str) -> bool:
        """Soft-delete an account (set inactive)."""
        with self._lock:
            acct = self._accounts.get(account_id)
            if not acct:
                return False
            acct["is_active"] = False
            acct["updated_at"] = _utcnow()
            self._email_index.pop(acct.get("email_hash", ""), None)
        return True

    # ── results ───────────────────────────────────────────────────────

    def save_result(self, account_id: str, test_type: str,
                    result: dict[str, Any]) -> dict[str, Any] | None:
        """Store a test result for the given account."""
        entry: dict[str, Any] = {
            "id": _new_id(),
            "test_type": test_type,
            "result": result,
            "created_at": _utcnow(),
        }
        with self._lock:
            acct = self._accounts.get(account_id)
            if not acct:
                return None
            acct.setdefault("results", []).append(entry)
            acct["updated_at"] = _utcnow()
        return entry

    def get_results(
        self, account_id: str,
        test_type: str | None = None,
    ) -> list[dict[str, Any]]:
        """Return saved results, optionally filtered by test type."""
        with self._lock:
            acct = self._accounts.get(account_id)
            if not acct:
                return []
            results = list(acct.get("results", []))
        if test_type:
            results = [r for r in results
                       if r.get("test_type") == test_type]
        return sorted(results, key=lambda r: r.get("created_at", ""),
                       reverse=True)

    def get_evolution(
        self, account_id: str,
        test_type: str,
    ) -> list[dict[str, Any]]:
        """Return chronological score evolution for a test type.

        Each entry contains ``date``, ``test_type``, and all
        top-level keys from the stored result.
        """
        results = self.get_results(account_id, test_type)
        evolution = []
        for r in reversed(results):
            scores = r.get("result", {})
            entry: dict[str, Any] = {
                "date": r.get("created_at", ""),
                "test_type": test_type,
            }
            # Flatten scores into the evolution entry
            if isinstance(scores, dict):
                s = scores.get("scores", scores)
                if isinstance(s, dict):
                    entry.update(s)
                else:
                    entry["score"] = s
            evolution.append(entry)
        return evolution

    # ── GDPR data export ──────────────────────────────────────────────

    def export_data(self, account_id: str) -> dict[str, Any]:
        """Return all data held for an account (GDPR compliance).

        Includes account info, all test results, and metadata.
        """
        with self._lock:
            acct = self._accounts.get(account_id)
            if not acct:
                return {"error": "Account not found"}
            safe = self._safe_account(acct)
        return {
            "exported_at": _utcnow(),
            "account": safe,
            "results": self.get_results(account_id),
            "data_categories": [
                "account_info",
                "test_results",
                "subscription_tier",
            ],
        }

    # ── internal helpers ──────────────────────────────────────────────

    @staticmethod
    def _safe_account(account: dict[str, Any]) -> dict[str, Any]:
        """Return account dict without sensitive fields."""
        return {
            k: v for k, v in account.items()
            if k not in ("password_hash", "email_hash")
        }
