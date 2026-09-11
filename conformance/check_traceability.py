#!/usr/bin/env python3
"""Structural check that traceability.yaml covers the SPEC.md acceptance list.

Lenient mode (default): the entries must structurally mirror the acceptance
bullets; non-verified statuses warn. Strict mode (--strict, enforced from
issue #16): every entry must be verified with at least one test.
"""

import pathlib
import re
import sys

import yaml

ROOT = pathlib.Path(__file__).resolve().parent.parent
SPEC = ROOT / "SPEC.md"
TRACE = ROOT / "conformance" / "traceability.yaml"
STATUSES = {"planned", "partial", "verified"}


def acceptance_bullets() -> list[str]:
    text = SPEC.read_text(encoding="utf-8")
    match = re.search(r"^## Acceptance\n(.*?)\Z", text, re.S | re.M)
    if not match:
        raise SystemExit("SPEC.md has no '## Acceptance' section")
    section = match.group(1)
    section = re.sub(r"^\s*\n", "", section, count=1)
    return re.findall(r"^- (.+)$", section, re.M)


def main() -> int:
    bullets = acceptance_bullets()
    data = yaml.safe_load(TRACE.read_text(encoding="utf-8"))
    entries = data["entries"]
    problems: list[str] = []

    if len(bullets) != len(entries):
        problems.append(
            f"count mismatch: SPEC.md has {len(bullets)} acceptance bullets, "
            f"traceability.yaml has {len(entries)} entries"
        )
    for i, entry in enumerate(entries):
        for field in ("id", "title", "issues", "tests", "status"):
            if field not in entry:
                problems.append(f"entry {i}: missing field {field!r}")
        if entry.get("status") not in STATUSES:
            problems.append(f"entry {entry.get('id')}: bad status {entry.get('status')!r}")
        if not isinstance(entry.get("issues"), list) or not entry["issues"]:
            problems.append(f"entry {entry.get('id')}: issues must be a non-empty list")
        if not isinstance(entry.get("tests"), list):
            problems.append(f"entry {entry.get('id')}: tests must be a list")
        if entry.get("id") != f"a-{i + 1:02d}":
            problems.append(f"entry {i}: id must be a-{i + 1:02d}, got {entry.get('id')!r}")

    if problems:
        for p in problems:
            print(f"traceability: {p}", file=sys.stderr)
        return 1

    strict = "--strict" in sys.argv
    for entry in entries:
        if strict and (entry["status"] != "verified" or not entry["tests"]):
            print(f"traceability: strict mode: {entry['id']} not verified", file=sys.stderr)
            return 1
        if entry["status"] != "verified":
            print(f"traceability: {entry['id']} is {entry['status']} (not verified yet)")
    print(f"traceability: OK ({len(entries)} entries structurally match SPEC.md)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
