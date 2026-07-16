from __future__ import annotations

import json
from functools import lru_cache
from pathlib import Path
from typing import Any

DATA_DIR = Path(__file__).resolve().parent / "data"

# Map from API-facing test_type to actual data filename.
_FILENAME_OVERRIDES: dict[str, str] = {
    "human_design": "human_design_data.json",
    "emotional_intelligence": "eq_questions.json",
}


@lru_cache(maxsize=32)
def load_test_data(test_type: str) -> dict[str, Any]:
    """Load a JSON test-data file by test type (cached).

    The file name follows the convention ``<test_type>_questions.json``,
    with a few overrides (see ``_FILENAME_OVERRIDES``).

    Raises ``FileNotFoundError`` if the file does not exist.
    """
    if test_type in _FILENAME_OVERRIDES:
        filename = _FILENAME_OVERRIDES[test_type]
    else:
        filename = f"{test_type}_questions.json"
    path = DATA_DIR / filename
    if not path.exists():
        msg = f"No data file found for test type {test_type!r} (tried {path})"
        raise FileNotFoundError(msg)

    with path.open("r", encoding="utf-8") as fh:
        return json.load(fh)  # type: ignore[no-any-return]


@lru_cache(maxsize=1)
def load_all_metadata() -> list[dict[str, Any]]:
    """Return a list of ``{test_type, name, description}`` for every test."""
    results: list[dict[str, Any]] = []
    for path in sorted(DATA_DIR.glob("*.json")):
        data = json.loads(path.read_text(encoding="utf-8"))
        results.append({
            "test_type": data.get("test_type", path.stem),
            "name": data.get("name", {}),
            "description": data.get("description", ""),
        })
    return results
