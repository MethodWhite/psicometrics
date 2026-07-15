#!/usr/bin/env python3
"""
Add Portuguese (pt-BR) and French (fr) translations to all JSON data files.

Reads each JSON file in the data/ directory and for every text field that has
"es" and "en" keys, adds "pt" and "fr" using Google Translate via deep-translator.
"""

import json
import os
import sys
import time
from pathlib import Path

DATA_DIR = Path(__file__).resolve().parent / "data"

try:
    from deep_translator import GoogleTranslator
except ImportError:
    print("Installing deep-translator...", file=sys.stderr)
    os.system(f"{sys.executable} -m pip install deep-translator")
    from deep_translator import GoogleTranslator

PT_TRANSLATOR = GoogleTranslator(source="es", target="pt")
FR_TRANSLATOR = GoogleTranslator(source="es", target="fr")


def translate(text: str, target: str) -> str:
    """Translate a string from Spanish to the target language."""
    text = text.strip()
    if not text:
        return text
    try:
        if target == "pt":
            result = PT_TRANSLATOR.translate(text)
        else:
            result = FR_TRANSLATOR.translate(text)
        return result.strip() if result else text
    except Exception as e:
        print(f"  [!] Translation error for '{text[:40]}...': {e}", file=sys.stderr)
        return text


def needs_translation(obj, lang: str) -> bool:
    """Check if a dict has string 'es' and 'en' values but is missing the target language."""
    return (
        isinstance(obj, dict)
        and "es" in obj
        and "en" in obj
        and lang not in obj
        and isinstance(obj.get("es"), str)
    )


def process_value(value, lang: str, depth: int = 0):
    """Recursively walk a JSON structure and translate missing language fields."""
    if isinstance(value, dict):
        # If this is a text-like dict with string "es"/"en" keys, translate it
        if needs_translation(value, lang):
            text_es = value.get("es", "")
            if text_es:
                translation = translate(str(text_es), lang)
                value[lang] = translation
                print(f"{'  ' * depth}  ✓ [{lang}] {str(text_es)[:50]} -> {translation[:50]}", file=sys.stderr)
            else:
                value[lang] = text_es
            return value

        # Otherwise recurse into each field
        for k, v in value.items():
            value[k] = process_value(v, lang, depth + 1)
        return value

    elif isinstance(value, list):
        return [process_value(item, lang, depth + 1) for item in value]

    return value


def process_file(filepath: Path):
    """Add pt and fr translations to a single JSON file."""
    print(f"\n{'=' * 60}", file=sys.stderr)
    print(f"Processing: {filepath.name}", file=sys.stderr)
    print(f"{'=' * 60}", file=sys.stderr)

    with open(filepath, "r", encoding="utf-8") as f:
        data = json.load(f)

    original = json.dumps(data, ensure_ascii=False, indent=2)

    for lang in ("pt", "fr"):
        print(f"\n  --- Adding {lang.upper()} translations ---", file=sys.stderr)
        data = process_value(data, lang)
        # Small delay to be polite to the translation service
        time.sleep(0.5)

    updated = json.dumps(data, ensure_ascii=False, indent=2)

    if original != updated:
        with open(filepath, "w", encoding="utf-8") as f:
            f.write(updated)
            f.write("\n")
        print(f"\n  ✓ Saved: {filepath.name}", file=sys.stderr)
        return True
    else:
        print(f"\n  - No changes: {filepath.name}", file=sys.stderr)
        return False


def count_missing_fields(data, lang: str) -> int:
    """Count how many text fields are missing the target language."""
    count = 0
    if isinstance(data, dict):
        if needs_translation(data, lang):
            count += 1
        for v in data.values():
            count += count_missing_fields(v, lang)
    elif isinstance(data, list):
        for item in data:
            count += count_missing_fields(item, lang)
    return count


def main():
    json_files = sorted(DATA_DIR.glob("*.json"))
    if not json_files:
        print(f"No JSON files found in {DATA_DIR}", file=sys.stderr)
        sys.exit(1)

    print(f"Found {len(json_files)} JSON files in {DATA_DIR}", file=sys.stderr)

    # First, count missing fields for reporting
    total_pt = total_fr = 0
    for fp in json_files:
        with open(fp, "r", encoding="utf-8") as f:
            data = json.load(f)
        total_pt += count_missing_fields(data, "pt")
        total_fr += count_missing_fields(data, "fr")

    print(f"\nMissing translations to add: {total_pt} (pt), {total_fr} (fr)", file=sys.stderr)

    if total_pt == 0 and total_fr == 0:
        print("\n✓ All translations already present!", file=sys.stderr)
        return

    # Process each file
    updated_count = 0
    for fp in json_files:
        if process_file(fp):
            updated_count += 1

    print(f"\n{'=' * 60}", file=sys.stderr)
    print(f"Done. Updated {updated_count}/{len(json_files)} files.", file=sys.stderr)


if __name__ == "__main__":
    main()
