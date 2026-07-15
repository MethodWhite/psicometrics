#!/usr/bin/env python3
"""
Generate 16 language JSON files for the frontend i18n.

Reads the English template (en.json) and creates skeleton files for:
de, it, nl, pl, ru, ja, ko, zh, ar, hi, tr, sv, da, fi, no, cs

If deep-translator is available, it translates each string.
Otherwise, it copies the English text as a placeholder.
"""

import json
import os
import sys
import time
import shutil

I18N_DIR = os.path.dirname(os.path.abspath(__file__))

NEW_LANGUAGES = [
    ("de", "German"),
    ("it", "Italian"),
    ("nl", "Dutch"),
    ("pl", "Polish"),
    ("ru", "Russian"),
    ("ja", "Japanese"),
    ("ko", "Korean"),
    ("zh", "Chinese Simplified"),
    ("ar", "Arabic"),
    ("hi", "Hindi"),
    ("tr", "Turkish"),
    ("sv", "Swedish"),
    ("da", "Danish"),
    ("fi", "Finnish"),
    ("no", "Norwegian"),
    ("cs", "Czech"),
]

SOURCE_LANG = "en"


def flatten_keys(obj, prefix=""):
    """Flatten a nested dict into dot-separated keys with their string values."""
    items = []
    if isinstance(obj, dict):
        for k, v in obj.items():
            path = f"{prefix}.{k}" if prefix else k
            if isinstance(v, str):
                items.append((path, v))
            elif isinstance(v, dict):
                items.extend(flatten_keys(v, path))
    return items


def translate_all(texts, target_lang):
    """Translate a list of texts using GoogleTranslator (batch)."""
    from deep_translator import GoogleTranslator

    translator = GoogleTranslator(source=SOURCE_LANG, target=target_lang)
    results = []
    for text in texts:
        try:
            translated = translator.translate(text)
            results.append(translated)
            time.sleep(0.3)  # rate limit
        except Exception as e:
            print(f"    ⚠️  Translation failed for '{text[:30]}...': {e}")
            results.append(text)  # fallback to English
    return results


def reconstruct(flat_items, translations):
    """Rebuild the nested JSON structure from (path, value) pairs."""
    result = {}
    for (path, _), trans in zip(flat_items, translations):
        parts = path.split(".")
        current = result
        for part in parts[:-1]:
            if part not in current:
                current[part] = {}
            current = current[part]
        current[parts[-1]] = trans
    return result


def main():
    # Read English template
    en_path = os.path.join(I18N_DIR, "en.json")
    if not os.path.exists(en_path):
        print(f"ERROR: {en_path} not found")
        sys.exit(1)

    with open(en_path, "r", encoding="utf-8") as f:
        en_data = json.load(f)

    flat = flatten_keys(en_data)
    texts = [t for _, t in flat]
    print(f"Loaded {len(texts)} strings from en.json")

    # Check if deep-translator is available
    translator_avail = False
    try:
        from deep_translator import GoogleTranslator

        translator_avail = True
        print("deep-translator found — will use Google Translate")
    except ImportError:
        print("deep-translator NOT found — will copy English as placeholder")
        print("Install: pip install deep-translator")

    for lang_code, lang_name in NEW_LANGUAGES:
        out_path = os.path.join(I18N_DIR, f"{lang_code}.json")

        if os.path.exists(out_path):
            print(f"⏭️  {lang_code}.json already exists — skipping")
            continue

        print(f"\n🌐 Generating {lang_code}.json ({lang_name})...")

        if translator_avail:
            translations = translate_all(texts, lang_code)
        else:
            translations = texts  # English placeholder

        new_data = reconstruct(flat, translations)

        with open(out_path, "w", encoding="utf-8") as f:
            json.dump(new_data, f, ensure_ascii=False, indent=2)

        print(f"   ✅ Written {len(translations)} strings to {lang_code}.json")

    print(f"\n{'='*50}")
    print("Done! All language files generated.")
    if not translator_avail:
        print("NOTE: Files contain English placeholders. Run with deep-translator for real translations.")
        print(f"      pip install deep-translator && python3 {__file__}")


if __name__ == "__main__":
    main()
