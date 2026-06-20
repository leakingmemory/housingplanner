#!/usr/bin/env python3
"""Verify every UI string is translated in all languages.

The app wraps user-facing English strings in `tr(lang, "...")` (see src/i18n.rs).
This script:

  1. collects the set of keys actually used via `tr(...)` in the app source, then
  2. checks that every per-language translation table in `src/i18n.rs` contains
     each of those keys.

It exits non-zero and lists the gaps if any language is missing a translation,
so a newly added string can't silently fall back to English in another language.
Translating a string to the same text as English is fine — it just has to have an
explicit entry in the table.
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"

# Source files that call tr(...).
CALL_FILES = ["app.rs", "timeline.rs", "model.rs"]

# A Rust string literal (handles escaped quotes).
STRING = r'"((?:[^"\\]|\\.)*)"'


def used_keys() -> set[str]:
    """Keys passed as the second argument of `tr(<lang>, "...")`."""
    pat = re.compile(r"\btr\(\s*[^,()]+,\s*" + STRING + r"\s*,?\s*\)", re.DOTALL)
    keys: set[str] = set()
    for name in CALL_FILES:
        text = (SRC / name).read_text(encoding="utf-8")
        keys.update(m.group(1) for m in pat.finditer(text))
    return keys


def table_keys() -> dict[str, set[str]]:
    """{language_fn_name: set(match-arm keys)} parsed from src/i18n.rs.

    Each language table is a `fn xx(en: &'static str) -> &'static str` whose arms
    look like `"English key" => "translation",`. We collect the left-hand-side
    key of every such arm, scoped to the enclosing function.
    """
    text = (SRC / "i18n.rs").read_text(encoding="utf-8")
    fn_pat = re.compile(r"^\s*fn\s+(\w+)\s*\(\s*en:\s*&.*str")
    arm_pat = re.compile(r"^\s*" + STRING + r"\s*=>")
    tables: dict[str, set[str]] = {}
    current: str | None = None
    for line in text.splitlines():
        m = fn_pat.match(line)
        if m:
            current = m.group(1)
            tables.setdefault(current, set())
            continue
        if current:
            a = arm_pat.match(line)
            if a:
                tables[current].add(a.group(1))
    return tables


def main() -> int:
    used = used_keys()
    tables = table_keys()

    if not used:
        print("check-translations: found no tr() keys — parser may be broken", file=sys.stderr)
        return 2
    if not tables:
        print("check-translations: found no language tables — parser may be broken", file=sys.stderr)
        return 2

    langs = sorted(tables)
    failed = False
    for lang in langs:
        missing = sorted(used - tables[lang])
        if missing:
            failed = True
            print(f"\n✖ {lang}(): {len(missing)} untranslated string(s):", file=sys.stderr)
            for k in missing:
                print(f"    {k!r}", file=sys.stderr)

    # Keys translated but no longer used anywhere — a warning, not a failure.
    stale = sorted(set().union(*tables.values()) - used)
    if stale:
        print(
            f"\nℹ note: {len(stale)} key(s) present in tables but not used via tr():",
            file=sys.stderr,
        )
        for k in stale:
            print(f"    {k!r}", file=sys.stderr)

    if failed:
        print(
            "\ncheck-translations: FAILED — add the missing entries in src/i18n.rs",
            file=sys.stderr,
        )
        return 1

    print(f"check-translations: OK — {len(used)} strings translated across {len(langs)} languages")
    return 0


if __name__ == "__main__":
    sys.exit(main())
