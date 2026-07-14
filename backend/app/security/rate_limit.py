"""Token bucket rate limiter — adapted from Synapsis arca RateLimiter pattern."""

import time
from collections import defaultdict
from dataclasses import dataclass, field
from threading import Lock


@dataclass
class TokenBucket:
    capacity: int
    refill_rate: float  # tokens per second
    tokens: float = field(init=False)
    last_refill: float = field(init=False, default_factory=time.monotonic)

    def __post_init__(self):
        self.tokens = float(self.capacity)

    def consume(self) -> bool:
        now = time.monotonic()
        elapsed = now - self.last_refill
        self.tokens = min(self.capacity, self.tokens + elapsed * self.refill_rate)
        self.last_refill = now

        if self.tokens >= 1:
            self.tokens -= 1
            return True
        return False


class RateLimiter:
    def __init__(self, requests_per_minute: int = 30, lockout_threshold: int = 10, lockout_seconds: int = 3600):
        self.requests_per_minute = requests_per_minute
        self.lockout_threshold = lockout_threshold
        self.lockout_seconds = lockout_seconds
        self._buckets: dict[str, TokenBucket] = defaultdict(
            lambda: TokenBucket(capacity=requests_per_minute, refill_rate=requests_per_minute / 60.0)
        )
        self._failures: dict[str, list[float]] = defaultdict(list)
        self._lockouts: dict[str, float] = {}
        self._lock = Lock()

    def is_locked_out(self, key: str) -> bool:
        if key in self._lockouts:
            if time.monotonic() - self._lockouts[key] < self.lockout_seconds:
                return True
            del self._lockouts[key]
            self._failures.pop(key, None)
        return False

    def record_failure(self, key: str) -> None:
        with self._lock:
            now = time.monotonic()
            self._failures[key] = [t for t in self._failures[key] if now - t < self.lockout_seconds]
            self._failures[key].append(now)
            if len(self._failures[key]) >= self.lockout_threshold:
                self._lockouts[key] = now

    def allow(self, key: str) -> bool:
        with self._lock:
            if self.is_locked_out(key):
                return False
            return self._buckets[key].consume()
