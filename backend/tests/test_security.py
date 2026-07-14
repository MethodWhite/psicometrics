"""Tests for security middleware."""

import pytest
from app.security.audit import AuditLogger
from app.security.rate_limit import RateLimiter


class TestAuditLogger:
    def test_log_entry(self):
        logger = AuditLogger()
        entry = logger.log("test_action", session_id="test-123")
        assert entry.action == "test_action"
        assert entry.session_id == "test-123"
        assert entry.current_hash != ""

    def test_hash_chain_integrity(self):
        logger = AuditLogger()
        for i in range(10):
            logger.log(f"action_{i}", metadata={"index": i})
        assert logger.verify_chain()

    def test_chain_break_detection(self):
        logger = AuditLogger()
        logger.log("action_1")
        logger.log("action_2")
        # Tamper with the chain
        logger._entries[1].previous_hash = "tampered"
        assert not logger.verify_chain()

    def test_session_filter(self):
        logger = AuditLogger()
        logger.log("action_1", session_id="s1")
        logger.log("action_2", session_id="s2")
        logger.log("action_3", session_id="s1")

        s1_entries = logger.get_entries(session_id="s1")
        assert len(s1_entries) == 2


class TestRateLimiter:
    def test_allows_within_limit(self):
        limiter = RateLimiter(requests_per_minute=10)
        for _ in range(10):
            assert limiter.allow("test-ip")

    def test_blocks_over_limit(self):
        limiter = RateLimiter(requests_per_minute=5)
        for _ in range(5):
            limiter.allow("test-ip")
        assert not limiter.allow("test-ip")

    def test_lockout_after_failures(self):
        limiter = RateLimiter(requests_per_minute=100, lockout_threshold=3, lockout_seconds=60)
        for _ in range(3):
            limiter.record_failure("test-ip")
        assert limiter.is_locked_out("test-ip")
        assert not limiter.allow("test-ip")

    def test_independent_keys(self):
        limiter = RateLimiter(requests_per_minute=2)
        limiter.allow("ip-1")
        limiter.allow("ip-1")
        assert not limiter.allow("ip-1")
        assert limiter.allow("ip-2")
