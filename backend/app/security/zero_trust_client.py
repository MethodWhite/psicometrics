"""Zero Trust client for the Flutter API.

Implements challenge-response authentication for every API request.
Adapted from Synapsis ztf::challenge pattern.
"""

import hashlib
import hmac
import secrets
from dataclasses import dataclass
from threading import Lock


class ZeroTrustClient:
    """Zero Trust client that performs challenge-response for API calls.

    Before submitting test data, the client requests a challenge,
    computes HMAC-SHA256 response, and attaches it to the request headers.
    """

    def __init__(self, device_id: str | None = None):
        self._device_id = device_id or f"device-{secrets.token_hex(8)}"
        self._challenge_cache: dict[str, dict] = {}
        self._lock = Lock()

    def get_device_id(self) -> str:
        return self._device_id

    def compute_challenge_response(self, nonce: str) -> str:
        """Compute HMAC-SHA256 challenge response.

        Uses device_id as the shared secret (simplified).
        In production, uses hardware-backed key from TPM/Keystore.
        """
        secret = f"psicometrics-device-{self._device_id}".encode()
        return hmac.digest(secret, nonce.encode(), hashlib.sha256).hex()

    def get_auth_headers(self, challenge_id: str, response: str) -> dict[str, str]:
        """Get headers for Zero Trust verification."""
        return {
            "X-Challenge-Id": challenge_id,
            "X-Challenge-Response": response,
            "X-Device-Id": self._device_id,
        }
