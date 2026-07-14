"""CSPRNG utilities — adapted from Synapsis security.rs getrandom pattern."""

import secrets


def secure_random_bytes(n: int) -> bytes:
    return secrets.token_bytes(n)


def secure_session_id() -> str:
    return secrets.token_urlsafe(32)


def secure_token() -> str:
    return secrets.token_hex(32)
