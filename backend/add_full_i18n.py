#!/usr/bin/env python3
"""
PsicoMetrics — Full i18n Expansion Script

Reads all 11 JSON data files in ../data/ and adds 16 additional languages
(de, it, nl, pl, ru, ja, ko, zh, ar, hi, tr, sv, da, fi, no, cs) to every
text dict that already has es/en/pt/fr translations.

Uses deep-translator (GoogleTranslator) with rate limiting.
Idempotent: skips language keys that already exist.
"""

import json
import os
import time
import glob as glob_mod

DATA_DIR = os.path.join(os.path.dirname(__file__), "data")

NEW_LANGUAGES = [
    "de", "it", "nl", "pl", "ru",
    "ja", "ko", "zh", "ar", "hi",
    "tr", "sv", "da", "fi", "no", "cs",
]

# Source languages to translate FROM (deep-translator Google supports these)
SOURCE_LANG = "en"
# Fields that are NOT text dicts and should be left as-is
NON_TEXT_KEYS = {"min", "max", "test_type", "id", "factor", "direction", "items"}


def is_text_dict(value):
    """Check if a value is a dict containing at least one of the source languages."""
    if not isinstance(value, dict):
        return False
    # Must have at least one of our known language keys
    return any(k in value for k in ("es", "en", "pt", "fr"))


def extract_text_values(obj, path=""):
    """Recursively find all (path, text) pairs to translate."""
    pairs = []
    if isinstance(obj, dict):
        if is_text_dict(obj):
            # This is a text-localization dict
            source_text = obj.get(SOURCE_LANG) or obj.get("es") or obj.get("en")
            if source_text and isinstance(source_text, str) and source_text.strip():
                existing = {k for k in obj if k in NEW_LANGUAGES}
                pairs.append((path, source_text, existing, obj))
            # Don't recurse into text dicts' language values
        else:
            # Recurse into regular dicts
            for k, v in obj.items():
                if k in NON_TEXT_KEYS:
                    continue
                pairs.extend(extract_text_values(v, f"{path}.{k}" if path else k))
    elif isinstance(obj, list):
        for i, v in enumerate(obj):
            pairs.extend(extract_text_values(v, f"{path}[{i}]"))
    return pairs


def build_rate_limiter(delay=0.5):
    """Simple rate limiter to avoid hitting Google Translate limits."""
    last_call = [0.0]

    def wait():
        elapsed = time.time() - last_call[0]
        if elapsed < delay:
            time.sleep(delay - elapsed)
        last_call[0] = time.time()

    return wait


def translate_text(text, target_lang, source_lang="en"):
    """Translate a single text using deep-translator's GoogleTranslator."""
    from deep_translator import GoogleTranslator

    translator = GoogleTranslator(source=source_lang, target=target_lang)
    result = translator.translate(text)
    return result


def main():
    try:
        from deep_translator import GoogleTranslator
    except ImportError:
        print("deep-translator not installed. Installing...")
        import subprocess, sys

        subprocess.check_call(
            [sys.executable, "-m", "pip", "install", "deep-translator"]
        )
        from deep_translator import GoogleTranslator

    rate_limit = build_rate_limiter(delay=0.6)

    json_files = sorted(glob_mod.glob(os.path.join(DATA_DIR, "*.json")))
    print(f"Found {len(json_files)} JSON data files:\n")

    total_translations = 0
    skipped_files = 0

    for filepath in json_files:
        filename = os.path.basename(filepath)
        print(f"\n{'='*60}")
        print(f"Processing: {filename}")
        print(f"{'='*60}")

        with open(filepath, "r", encoding="utf-8") as f:
            data = json.load(f)

        text_pairs = extract_text_values(data)
        print(f"  Found {len(text_pairs)} text-localization dicts to check")

        file_changed = False

        for path, source_text, existing, text_dict in text_pairs:
            for lang in NEW_LANGUAGES:
                if lang in existing:
                    continue  # Already translated — idempotent

                try:
                    rate_limit()
                    translated = translate_text(source_text, lang, SOURCE_LANG)
                    text_dict[lang] = translated
                    total_translations += 1
                    file_changed = True
                    print(f"    ✅ {path} → {lang}: {translated[:60]}...")
                except Exception as e:
                    print(f"    ❌ {path} → {lang}: ERROR — {e}")
                    # If we got rate-limited, back off more
                    if "429" in str(e) or "Too Many Requests" in str(e):
                        print("      Rate limited! Backing off 10s...")
                        time.sleep(10)

        if file_changed:
            with open(filepath, "w", encoding="utf-8") as f:
                json.dump(data, f, ensure_ascii=False, indent=2)
            print(f"  ✅ Saved {filename}")
        else:
            skipped_files += 1
            print(f"  ⏭️  No changes — all languages already present")

    print(f"\n{'='*60}")
    print(f"DONE! {total_translations} translations added across all files.")
    print(f"Files skipped (no changes): {skipped_files}")
    print(f"{'='*60}")


if __name__ == "__main__":
    main()
