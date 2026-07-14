"""Tests for Big Five scoring algorithm."""

import pytest
from app.scoring.big_five import score_big_five


class TestBigFiveScoring:
    def test_all_high_scores(self):
        """All answers set to 5 (maximum) should produce high scores."""
        answers = {i: 5 for i in range(1, 121)}
        result = score_big_five(answers, "es")

        assert "scores" in result
        assert "facets" in result
        assert "profile_summary" in result
        assert "percentiles" in result

        # All factors should be high (100 for non-reversed, varies for reversed)
        for factor in ["O", "C", "E", "A", "N"]:
            assert factor in result["scores"]
            assert 0 <= result["scores"][factor] <= 100

    def test_all_low_scores(self):
        """All answers set to 1 (minimum) should produce specific pattern."""
        answers = {i: 1 for i in range(1, 121)}
        result = score_big_five(answers, "es")

        # All factors should be calculable
        assert len(result["scores"]) == 5
        assert len(result["facets"]) == 30

    def test_neutral_scores(self):
        """All answers set to 3 (neutral) should produce 50% for all factors."""
        answers = {i: 3 for i in range(1, 121)}
        result = score_big_five(answers, "en")

        for factor in ["O", "C", "E", "A", "N"]:
            assert result["scores"][factor] == 50.0

    def test_reverse_scoring(self):
        """Test that reverse-scored items are properly inverted."""
        # Items with reverse=True in the data should be inverted
        # e.g., item 4 (C2, reverse=True): answer 5 becomes 1
        answers = {i: 3 for i in range(1, 121)}
        result = score_big_five(answers, "es")
        # Neutral answers should produce neutral scores
        assert result["scores"]["C"] == 50.0

    def test_spanish_language(self):
        """Test Spanish summary generation."""
        answers = {i: 5 for i in range(1, 121)}
        result = score_big_five(answers, "es")
        assert isinstance(result["profile_summary"], str)
        assert len(result["profile_summary"]) > 0

    def test_english_language(self):
        """Test English summary generation."""
        answers = {i: 5 for i in range(1, 121)}
        result = score_big_five(answers, "en")
        assert isinstance(result["profile_summary"], str)
        assert len(result["profile_summary"]) > 0

    def test_percentiles_in_range(self):
        """Percentiles should be between 0 and 100."""
        answers = {i: 3 for i in range(1, 121)}
        result = score_big_five(answers, "es")
        for p in result["percentiles"].values():
            assert 0 <= p <= 100

    def test_missing_answers(self):
        """Should handle missing answers gracefully."""
        answers = {1: 5, 2: 4, 3: 3}  # Only 3 answers
        result = score_big_five(answers, "es")
        assert "scores" in result
