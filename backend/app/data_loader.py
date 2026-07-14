"""Load question data from JSON files."""

import json
from pathlib import Path

DATA_DIR = Path(__file__).parent / "data"

_cache: dict[str, dict] = {}


def load_questions(test_type: str) -> dict:
    if test_type in _cache:
        return _cache[test_type]

    # Try _questions.json first, then _data.json
    filepath = DATA_DIR / f"{test_type}_questions.json"
    if not filepath.exists():
        filepath = DATA_DIR / f"{test_type}_data.json"
    if not filepath.exists():
        raise FileNotFoundError(f"Question file not found for test type: {test_type}")

    with open(filepath, "r", encoding="utf-8") as f:
        data = json.load(f)

    _cache[test_type] = data
    return data
