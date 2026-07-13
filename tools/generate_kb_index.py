#!/usr/bin/env python3
"""Generate a detailed hierarchical index from docs/kb/*.md headings.

Usage:
    python tools/generate_kb_index.py

Output:
    docs/kb/INDEX.md
"""

from pathlib import Path

KB_DIR = Path("docs/kb")
OUTPUT = KB_DIR / "INDEX.md"


def extract_headings(text: str, max_depth: int = 4):
    headings = []
    for line in text.splitlines():
        stripped = line.lstrip()
        if stripped.startswith("#"):
            level = 0
            for ch in stripped:
                if ch == "#":
                    level += 1
                else:
                    break
            if level > max_depth:
                continue
            title = stripped[level:].strip()
            if title:
                headings.append((level, title))
    return headings


# Part titles can be manually mapped here. If a part is missing, the first heading is used.
PART_TITLES: dict[str, str] = {
    "documentation_part_001": "Table of Contents / Preamble",
    "documentation_part_002": "API > General",
    "documentation_part_003": "Functions",
    "documentation_part_004": "Objects",
    "documentation_part_005": "Inter-Mod Tutorial",
    "documentation_part_006": "Appendix",
}


def main():
    parts = sorted(KB_DIR.glob("documentation_part_*.md"))
    if not parts:
        print(f"No documentation_part_*.md files found in {KB_DIR}", file=__import__("sys").stderr)
        return

    lines = ["# Knowledge Base Detailed Index\n"]

    for part in parts:
        content = part.read_text(encoding="utf-8")
        headings = extract_headings(content)

        part_title = PART_TITLES.get(part.stem, part.stem)

        lines.append(f"## [{part.stem}] {part_title}\n")

        if headings:
            min_level = min(level for level, _ in headings)
            for level, title in headings:
                normalized = level - min_level + 1
                indent = "  " * (normalized - 1)
                lines.append(f"{indent}- {title}")

        lines.append("")

    OUTPUT.write_text("\n".join(lines), encoding="utf-8")
    print(f"Wrote {OUTPUT}")
    print(f"Indexed {len(parts)} part(s).")


if __name__ == "__main__":
    main()
