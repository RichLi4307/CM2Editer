#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
无损统计当前文件夹及其子文件夹下所有 Rust 源代码文件的规模。

统计指标：
- 行数（总代码行，含空行）
- 非空行数
- 字符数（含空白、换行符）
- 字节数
"""

from __future__ import annotations

import sys
from pathlib import Path
from typing import List, Tuple

# 强制使用 UTF-8 输出，避免在 Windows 终端出现中文乱码
stdout_reconfigure = getattr(sys.stdout, "reconfigure", None)
if callable(stdout_reconfigure):
    stdout_reconfigure(encoding="utf-8", errors="replace")

stderr_reconfigure = getattr(sys.stderr, "reconfigure", None)
if callable(stderr_reconfigure):
    stderr_reconfigure(encoding="utf-8", errors="replace")


def collect_rust_files(root: Path) -> List[Path]:
    """递归收集目录下所有 .rs 文件，按路径排序。"""
    return sorted(p for p in root.rglob("*.rs") if p.is_file())


def analyze_file(path: Path) -> Tuple[int, int, int, int]:
    """
    读取单个 Rust 文件并返回统计信息：
    (总行数, 非空行数, 字符数, 字节数)
    """
    try:
        raw = path.read_bytes()
        text = raw.decode("utf-8", errors="replace")
    except OSError as exc:
        print(f"无法读取文件 {path}: {exc}", file=sys.stderr)
        return 0, 0, 0, 0

    lines = text.splitlines()
    total_lines = len(lines)
    non_empty_lines = sum(1 for line in lines if line.strip() != "")
    chars = len(text)
    bytes_size = len(raw)
    return total_lines, non_empty_lines, chars, bytes_size


def format_number(value: int) -> str:
    return f"{value:,}"


def main() -> int:
    root = Path.cwd()
    files = collect_rust_files(root)

    if not files:
        print(f"未在 {root.resolve()} 下找到 .rs 文件。")
        return 0

    headers = ("文件路径", "总行数", "非空行", "字符数", "字节数")
    rows: List[Tuple[str, str, str, str, str]] = []
    total_lines = total_non_empty = total_chars = total_bytes = 0

    for file in files:
        lines, non_empty, chars, bytes_size = analyze_file(file)
        total_lines += lines
        total_non_empty += non_empty
        total_chars += chars
        total_bytes += bytes_size
        rows.append(
            (
                str(file.relative_to(root)),
                format_number(lines),
                format_number(non_empty),
                format_number(chars),
                format_number(bytes_size),
            )
        )

    rows.append(
        (
            "【总计】",
            format_number(total_lines),
            format_number(total_non_empty),
            format_number(total_chars),
            format_number(total_bytes),
        )
    )

    col_widths = [
        max(len(row[i]) for row in [headers] + rows) for i in range(len(headers))
    ]

    def print_row(row: Tuple[str, ...], sep: str = " | ") -> None:
        cells = [cell.ljust(col_widths[i]) for i, cell in enumerate(row)]
        print(sep.join(cells))

    print_row(headers)
    print_row(tuple("-" * w for w in col_widths), sep="-+-")
    for row in rows:
        print_row(row)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
