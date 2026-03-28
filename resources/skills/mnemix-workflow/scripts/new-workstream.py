#!/usr/bin/env python3
"""Create a new mnemix-workflow workstream from bundled templates."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


def slugify(value: str) -> str:
    value = value.strip().lower()
    value = re.sub(r"[^a-z0-9]+", "-", value)
    return value.strip("-")


def titleize(value: str) -> str:
    words = re.split(r"[\s_-]+", value.strip())
    return " ".join(word.capitalize() for word in words if word)


def parse_prefix(path: Path) -> int | None:
    match = re.match(r"^(\d+)-", path.name)
    if not match:
        return None
    return int(match.group(1))


def next_id(workstreams_dir: Path) -> int:
    existing = []
    for child in workstreams_dir.iterdir():
        if not child.is_dir():
            continue
        prefix = parse_prefix(child)
        if prefix is not None:
            existing.append(prefix)
    return (max(existing) + 1) if existing else 1


def format_id(value: int) -> str:
    return f"{value:03d}" if value <= 999 else str(value)


def find_repo_root(start: Path) -> Path | None:
    for candidate in (start, *start.parents):
        if (candidate / ".git").exists():
            return candidate
    return None


def copy_tree(src: Path, dst: Path, substitutions: dict[str, str]) -> None:
    dst.mkdir(parents=True, exist_ok=False)
    for item in src.rglob("*"):
        relative = item.relative_to(src)
        target = dst / relative
        if item.is_dir():
            target.mkdir(parents=True, exist_ok=True)
            continue

        content = item.read_text()
        for key, replacement in substitutions.items():
            content = content.replace(key, replacement)
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Create a new mnemix-workflow workstream from templates."
    )
    parser.add_argument("name", help="Human-readable workstream name")
    args = parser.parse_args()

    skill_root = Path(__file__).resolve().parents[1]
    templates_dir = skill_root / "assets" / "workstream"
    repo_root = find_repo_root(Path.cwd())

    if repo_root is None:
        print(
            "Repository root not found from the current working directory."
            " Run this command from inside a git repository or worktree.",
            file=sys.stderr,
        )
        return 1

    workstreams_dir = repo_root / "workflow" / "workstreams"

    if not templates_dir.is_dir():
        print(f"Template directory not found: {templates_dir}", file=sys.stderr)
        return 1

    workstreams_dir.mkdir(parents=True, exist_ok=True)

    slug = slugify(args.name)
    if not slug:
        print("Name must contain at least one letter or digit.", file=sys.stderr)
        return 1

    numeric_id = next_id(workstreams_dir)
    formatted_id = format_id(numeric_id)
    title = titleize(args.name)
    folder_name = f"{formatted_id}-{slug}"
    destination = workstreams_dir / folder_name

    if destination.exists():
        print(f"Workstream already exists: {destination}", file=sys.stderr)
        return 1

    substitutions = {
        "{{WORKSTREAM_ID}}": formatted_id,
        "{{WORKSTREAM_SLUG}}": slug,
        "{{WORKSTREAM_TITLE}}": title,
    }

    copy_tree(templates_dir, destination, substitutions)

    print(f"Created workstream: {destination.relative_to(repo_root)}")
    print("Next step: fill in spec.md, ux.md, plan.md, and tasks.md")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
