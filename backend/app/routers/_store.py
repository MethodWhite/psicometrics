"""Shared in-memory data stores for all routers.

Using a separate module avoids circular imports when ``accounts.py``
needs access to the account dicts defined by ``auth.py``.
"""

from __future__ import annotations

from typing import Any

# ── Auth / Accounts ───────────────────────────────────────────────────────
_accounts_by_email: dict[str, dict[str, Any]] = {}  # email -> account
_accounts_by_id: dict[str, dict[str, Any]] = {}  # id -> account
_challenges: dict[str, dict[str, Any]] = {}  # challenge_id -> {nonce, …}
_devices: dict[str, dict[str, Any]] = {}  # device_id -> {platform, …}
_consent_log: list[dict[str, Any]] = []  # append-only GDPR consent log
