#!/usr/bin/env python3
"""Split a long HTML document by user-provided line numbers and convert each chunk to Markdown.

Usage:
    python tools/split_html_by_lines.py docs/documentation.html docs/kb 1 245 512 890 1200

If html2text is installed, it will be used for high-quality conversion; otherwise a simple
regex-based fallback is used. To get better output, install html2text first:

    pip install html2text
"""

import argparse
import re
import sys
from pathlib import Path

try:
    import html2text

    HTML2TEXT_AVAILABLE = True
except ImportError:
    HTML2TEXT_AVAILABLE = False


def convert_html_to_markdown(html: str) -> str:
    """Convert a raw HTML snippet to Markdown."""
    if HTML2TEXT_AVAILABLE:
        h = html2text.HTML2Text()
        h.ignore_links = False
        h.ignore_images = True
        h.escape_all = False
        return h.handle(html)

    # Fallback: strip tags and decode common entities.
    text = re.sub(r"<script[^>]*>.*?</script>", "", html, flags=re.DOTALL)
    text = re.sub(r"<style[^>]*>.*?</style>", "", text, flags=re.DOTALL)
    text = re.sub(r"<[^>]+>", "", text)
    text = text.replace("&lt;", "<")
    text = text.replace("&gt;", ">")
    text = text.replace("&amp;", "&")
    text = re.sub(r"\n{3,}", "\n\n", text)
    return text.strip()


def split_file(input_path: Path, split_lines: list[int], output_dir: Path) -> None:
    output_dir.mkdir(parents=True, exist_ok=True)
    lines = input_path.read_text(encoding="utf-8").splitlines()
    total_lines = len(lines)

    # Normalize split points: 1-based line numbers from the user.
    split_points = sorted({0, total_lines} | {l for l in split_lines if 0 < l <= total_lines})

    readme_lines = [
        "# Knowledge Base Index",
        "",
        f"Source: `{input_path}`",
        f"Total lines: {total_lines}",
        f"Total parts: {len(split_points) - 1}",
        "",
        "| Part | Line Range | File |",
        "|------|------------|------|",
    ]

    for i in range(len(split_points) - 1):
        start = split_points[i]  # 0-based index
        end = split_points[i + 1]
        chunk_html = "\n".join(lines[start:end])
        md = convert_html_to_markdown(chunk_html)

        part_num = i + 1
        out_file = output_dir / f"documentation_part_{part_num:03d}.md"
        header = (
            f"<!-- source: {input_path} -->\n"
            f"<!-- part: {part_num} -->\n"
            f"<!-- line_range: {start + 1}-{end} -->\n\n"
        )
        out_file.write_text(header + md, encoding="utf-8")

        readme_lines.append(
            f"| {part_num:03d} | {start + 1}-{end} | `{out_file.name}` |"
        )
        print(f"Wrote {out_file}")

    readme_path = output_dir / "README.md"
    readme_path.write_text("\n".join(readme_lines), encoding="utf-8")
    print(f"Wrote index {readme_path}")
    print(f"Done: {len(split_points) - 1} parts from {input_path}")


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Split an HTML document by line numbers and convert each chunk to Markdown."
    )
    parser.add_argument("--input", default="docs/documentation.html", help="Input HTML file")
    parser.add_argument("--output", default="docs/kb", help="Output directory")
    parser.add_argument(
        "lines", nargs="+", type=int, help="1-based line numbers where each new part starts"
    )
    args = parser.parse_args()

    if not HTML2TEXT_AVAILABLE:
        print(
            "Warning: html2text not installed. Using simple fallback conversion.\n"
            "Install html2text for better Markdown output: pip install html2text",
            file=sys.stderr,
        )

    split_file(Path(args.input), args.lines, Path(args.output))


if __name__ == "__main__":
    main()
