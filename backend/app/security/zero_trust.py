"""Zero Trust Challenge-Response Authentication.

Adapted from Synapsis ztf::challenge.
Implements challenge-response verification for every API request.
"""

import hashlib
import hmac
import secrets
import time
from collections import defaultdict
from dataclasses import dataclass
from threading import Lock


@dataclass
class Challenge:
    id: str
    nonce: str
    created_at: float
    expires_at: float
    agent_type: str
    verified: bool


class ChallengeError(Exception):
    pass


class ChallengeExpiredError(ChallengeError):
    pass


class ChallengeNotFoundError(ChallengeError):
    pass


class AlreadyVerifiedError(ChallengeError):
    pass


class RateLimitedError(ChallengeError):
    pass


class ZeroTrustProvider:
    """Zero Trust challenge-response provider.

    Every API request must present a valid challenge response.
    Challenges have TTL, are single-use, and rate-limited.
    """

    def __init__(self, secret_key: bytes | None = None, challenge_ttl: int = 300, max_failed: int = 5):
        self._secret_key = secret_key or secrets.token_bytes(32)
        self._challenge_ttl = challenge_ttl
        self._max_failed = max_failed
        self._challenges: dict[str, Challenge] = {}
        self._failed_attempts: dict[str, list[float]] = defaultdict(list)
        self._lock = Lock()

    def generate_challenge(self, session_id: str | None = None, agent_type: str = "unknown") -> Challenge:
        """Generate a new challenge for a client.

        Returns Challenge with nonce that client must sign.
        """
        challenge_id = secrets.token_hex(16)
        nonce = secrets.token_urlsafe(32)
        now = time.time()

        challenge = Challenge(
            id=challenge_id,
            nonce=nonce,
            created_at=now,
            expires_at=now + self._challenge_ttl,
            agent_type=agent_type,
            verified=False,
        )

        with self._lock:
            self._challenges[challenge_id] = challenge

        return challenge

    def get_challenge(self, challenge_id: str) -> Challenge | None:
        with self._lock:
            return self._challenges.get(challenge_id)

    def verify_response(self, challenge_id: str, response: str) -> Challenge:
        """Verify a challenge response.

        The response must be HMAC-SHA256(nonce, secret_key).
        """
        with self._lock:
            challenge = self._challenges.get(challenge_id)
            if not challenge:
                raise ChallengeNotFoundError()

            if challenge.expires_at < time.time():
                raise ChallengeExpiredError()

            if challenge.verified:
                raise AlreadyVerifiedError()

            # Rate limit check
            agent = challenge.agent_type
            now = time.time()
            self._failed_attempts[agent] = [
                t for t in self._failed_attempts[agent]
                if now - t < 3600
            ]
            if len(self._failed_attempts[agent]) >= self._max_failed:
                raise RateLimitedError()

            # Verify HMAC response
            expected = hmac.digest(
                self._secret_key,
                challenge.nonce.encode(),
                hashlib.sha256,
            )
            expected_b64 = expected.hex()

            if hmac.compare_digest(expected_b64, response):
                challenge.verified = True
                self._failed_attempts[agent].clear()
            else:
                self._failed_attempts[agent].append(now)
                raise ChallengeError("Verification failed")

        return challenge

    def verify_and_consume(self, challenge_id: str, response: str) -> bool:
        """Verify and remove challenge (single-use)."""
        challenge = self.verify_response(challenge_id, response)
        with self._lock:
            if challenge.verified:
                self._challenges.pop(challenge_id, None)
        return challenge.verified

    def compute_response(self, challenge: Challenge) -> str:
        """Compute HMAC-SHA256 response for a challenge."""
        return hmac.digest(
            self._secret_key,
            challenge.nonce.encode(),
            hashlib.sha256,
        ).hex()

    def cleanup_expired(self) -> int:
        now = time.time()
        with self._lock:
            expired = [k for k, v in self._challenges.items() if v.expires_at < now]
            for k in expired:
                del self._challenges[k]
        return len(expired)
