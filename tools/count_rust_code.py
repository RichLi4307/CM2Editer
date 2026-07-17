#!/usr/bin/env python3
# -*- coding: utf-8 -*-
r"""
Rust 代码统计工具。

支持通过命令行指定目标目录或文件：
    python count_rust_code.py <path>
    python count_rust_code.py .
    python .\tools\count_rust_code.py D:\Workshop\CODE\CM2Editer\src

默认排除 target、.git、.kilo 目录（可用 --exclude 自定义）。
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from typing import Dict, List, NamedTuple, Tuple

# 强制使用 UTF-8 输出，避免 Windows 终端中文乱码
for stream in (sys.stdout, sys.stderr):
    reconfigure = getattr(stream, "reconfigure", None)
    if callable(reconfigure):
        reconfigure(encoding="utf-8", errors="replace")

_CJK_RANGES: List[Tuple[int, int]] = [
    (0x4E00, 0x9FFF),   # CJK Unified Ideographs
    (0x3400, 0x4DBF),   # CJK Extension A
    (0x20000, 0x2A6DF), # CJK Extension B
    (0x2A700, 0x2B73F), # CJK Extension C
    (0x2B740, 0x2B81F), # CJK Extension D
    (0x2B820, 0x2CEAF), # CJK Extensions E-G
    (0xF900, 0xFAFF),   # CJK Compatibility Ideographs
    (0x2F800, 0x2FA1F), # CJK Compatibility Supplement
    (0x3040, 0x309F),   # Hiragana
    (0x30A0, 0x30FF),   # Katakana
    (0xAC00, 0xD7AF),   # Hangul Syllables
    (0xFF00, 0xFF60),   # Fullwidth ASCII
    (0xFFE0, 0xFFE6),   # Fullwidth Symbol Punctuation
    (0x3000, 0x303F),   # CJK Symbols and Punctuation
    (0x3200, 0x32FF),   # Enclosed CJK Letters and Months
    (0x3300, 0x33FF),   # CJK Compatibility
]


class FileStats(NamedTuple):
    total_lines: int
    blank_lines: int
    comment_lines: int
    code_lines: int
    chars: int
    bytes_size: int
    func_count: int
    struct_count: int
    enum_count: int
    use_count: int


def display_width(text: str) -> int:
    """计算字符串在等宽字体下的显示宽度，中文字符按 2 个英文宽度计算。"""
    width = 0
    for ch in text:
        cp = ord(ch)
        if cp <= 0x1F or cp == 0x7F:
            continue
        if any(start <= cp <= end for start, end in _CJK_RANGES):
            width += 2
        else:
            width += 1
    return width


def ljust_display(text: str, width: int, fillchar: str = " ") -> str:
    """按显示宽度左对齐，不足部分在右侧填充。"""
    pad = width - display_width(text)
    if pad > 0:
        return text + fillchar * pad
    return text


def rjust_display(text: str, width: int, fillchar: str = " ") -> str:
    """按显示宽度右对齐，不足部分在左侧填充。"""
    pad = width - display_width(text)
    if pad > 0:
        return fillchar * pad + text
    return text


def center_display(text: str, width: int, fillchar: str = " ") -> str:
    """按显示宽度居中，不足部分在两侧填充。"""
    pad = width - display_width(text)
    if pad > 0:
        left = pad // 2
        right = pad - left
        return fillchar * left + text + fillchar * right
    return text


def collect_rust_files(root: Path, exclude: List[str]) -> List[Path]:
    """递归收集目录下所有 .rs 文件，跳过排除目录，按路径排序。"""
    exclude_set = set(exclude)
    root_name = root.name
    files: List[Path] = []
    for p in root.rglob("*.rs"):
        if not p.is_file():
            continue
        parts = p.relative_to(root).parts
        if any(part in exclude_set and part != root_name for part in parts):
            continue
        files.append(p)
    return sorted(files)


def analyze_file(path: Path) -> FileStats:
    """读取单个 Rust 文件并返回结构化的统计信息。"""
    try:
        raw = path.read_bytes()
        text = raw.decode("utf-8", errors="replace")
    except OSError as exc:
        print(f"无法读取文件 {path}: {exc}", file=sys.stderr)
        return FileStats(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)

    lines = text.splitlines()
    total_lines = len(lines)

    blank = comment = code = 0
    in_block_comment = False
    func_count = struct_count = enum_count = use_count = 0

    fn_pattern = re.compile(r"^\s*(pub(\s*\([^)]*\))?\s+)?(async\s+)?(unsafe\s+)?fn\s+\w")
    struct_pattern = re.compile(r"^\s*(pub(\s*\([^)]*\))?\s+)?struct\s+\w")
    enum_pattern = re.compile(r"^\s*(pub(\s*\([^)]*\))?\s+)?enum\s+\w")
    use_pattern = re.compile(r"^\s*(pub\s+)?use\s+")

    for line in lines:
        stripped = line.strip()

        if not stripped:
            blank += 1
            continue

        if in_block_comment:
            comment += 1
            if "*/" in stripped:
                in_block_comment = False
            continue

        if stripped.startswith("//"):
            comment += 1
            continue

        if stripped.startswith("/*"):
            comment += 1
            if "*/" not in stripped:
                in_block_comment = True
            continue

        code += 1

        if fn_pattern.match(line):
            func_count += 1
        elif struct_pattern.match(line):
            struct_count += 1
        elif enum_pattern.match(line):
            enum_count += 1
        elif use_pattern.match(line):
            use_count += 1

    return FileStats(
        total_lines=total_lines,
        blank_lines=blank,
        comment_lines=comment,
        code_lines=code,
        chars=len(text),
        bytes_size=len(raw),
        func_count=func_count,
        struct_count=struct_count,
        enum_count=enum_count,
        use_count=use_count,
    )


def format_number(value: int) -> str:
    return f"{value:,}"


def safe_div(numerator: float, denominator: float) -> float:
    """安全除法，除数为 0 时返回 0.0。"""
    return numerator / denominator if denominator != 0 else 0.0


def relative_path(file: Path, root: Path) -> str:
    """返回 file 相对于 root 的路径，无法相对时返回绝对路径。"""
    try:
        return str(file.relative_to(root))
    except ValueError:
        return str(file)


def print_summary(target: Path, root: Path, files: List[Path], all_stats: List[FileStats]) -> None:
    """打印汇总分析数据。"""
    totals: Dict[str, int] = {
        "files": len(files),
        "total_lines": sum(s.total_lines for s in all_stats),
        "blank_lines": sum(s.blank_lines for s in all_stats),
        "comment_lines": sum(s.comment_lines for s in all_stats),
        "code_lines": sum(s.code_lines for s in all_stats),
        "chars": sum(s.chars for s in all_stats),
        "bytes": sum(s.bytes_size for s in all_stats),
        "funcs": sum(s.func_count for s in all_stats),
        "structs": sum(s.struct_count for s in all_stats),
        "enums": sum(s.enum_count for s in all_stats),
        "uses": sum(s.use_count for s in all_stats),
    }

    sections: List[Tuple[str, List[Tuple[str, str]]]] = [
        ("目录", [("分析目标", str(target.resolve()))]),
        ("文件规模", [
            ("文件数", format_number(totals["files"])),
            ("总行数", format_number(totals["total_lines"])),
            ("代码行", format_number(totals["code_lines"])),
            ("注释行", format_number(totals["comment_lines"])),
            ("空行数", format_number(totals["blank_lines"])),
            ("字符数", format_number(totals["chars"])),
            ("字节数", format_number(totals["bytes"])),
        ]),
        ("代码结构", [
            ("函数", format_number(totals["funcs"])),
            ("结构体", format_number(totals["structs"])),
            ("枚举", format_number(totals["enums"])),
            ("use 导入", format_number(totals["uses"])),
        ]),
        ("平均指标", [
            ("每文件平均行数", f"{safe_div(totals['total_lines'], totals['files']):.1f}"),
            ("每文件平均代码行", f"{safe_div(totals['code_lines'], totals['files']):.1f}"),
            ("每文件平均字符数", f"{safe_div(totals['chars'], totals['files']):.1f}"),
            ("代码密度", f"{safe_div(totals['code_lines'], totals['total_lines']) * 100:.1f}%"),
            ("注释比例", f"{safe_div(totals['comment_lines'], totals['total_lines']) * 100:.1f}%"),
        ]),
    ]

    max_label_width = max(
        display_width(label)
        for _, items in sections
        for label, _ in items
    )

    print(f"\nRust 代码统计 — {target.resolve()}\n")
    print("=" * 50)

    for title, items in sections:
        print(f"\n[{title}]")
        for label, value in items:
            print(f"  {ljust_display(label + ':', max_label_width + 2)} {value}")

    print("\n[代码行 Top 5]")
    ranked = sorted(
        zip(files, all_stats),
        key=lambda x: x[1].code_lines,
        reverse=True,
    )[:5]
    if ranked:
        max_path_width = max(
            display_width(relative_path(file, root))
            for file, _ in ranked
        )
        for idx, (file, stats) in enumerate(ranked, 1):
            rel = relative_path(file, root)
            print(
                f"  {idx}. {ljust_display(rel, max_path_width + 2)} "
                f"{rjust_display(format_number(stats.code_lines), 8)}"
            )
    else:
        print("  （无）")


def print_file_table(root: Path, files: List[Path], all_stats: List[FileStats]) -> None:
    """打印每个文件的详细统计表格。"""
    headers = ("文件路径", "总行", "代码", "注释", "空行", "字符", "字节", "函数", "结构体")
    rows: List[Tuple[str, ...]] = []

    for file, stats in zip(files, all_stats):
        rel = relative_path(file, root)
        rows.append((
            rel,
            format_number(stats.total_lines),
            format_number(stats.code_lines),
            format_number(stats.comment_lines),
            format_number(stats.blank_lines),
            format_number(stats.chars),
            format_number(stats.bytes_size),
            format_number(stats.func_count),
            format_number(stats.struct_count),
        ))

    rows.append((
        "【总计】",
        format_number(sum(s.total_lines for s in all_stats)),
        format_number(sum(s.code_lines for s in all_stats)),
        format_number(sum(s.comment_lines for s in all_stats)),
        format_number(sum(s.blank_lines for s in all_stats)),
        format_number(sum(s.chars for s in all_stats)),
        format_number(sum(s.bytes_size for s in all_stats)),
        format_number(sum(s.func_count for s in all_stats)),
        format_number(sum(s.struct_count for s in all_stats)),
    ))

    col_widths = [
        max(
            display_width(headers[i]),
            max(display_width(row[i]) for row in rows),
            25 if i == 0 else 0,
        )
        for i in range(len(headers))
    ]

    def print_row(row: Tuple[str, ...], is_header: bool = False) -> None:
        cells = []
        for i, cell in enumerate(row):
            if i == 0:
                cells.append(ljust_display(cell, col_widths[i]))
            else:
                if is_header:
                    cells.append(center_display(cell, col_widths[i]))
                else:
                    cells.append(rjust_display(cell, col_widths[i]))
        print(" | ".join(cells))

    print("\n[文件明细]")
    print_row(headers, is_header=True)
    print("-+-".join("-" * col_widths[i] for i in range(len(headers))))
    for row in rows:
        print_row(row)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="统计 Rust 源代码规模与结构",
        epilog="示例: python count_rust_code.py src",
    )
    parser.add_argument(
        "path",
        nargs="?",
        default=".",
        help="目标目录或 .rs 文件（默认为当前目录）",
    )
    parser.add_argument(
        "--exclude",
        default="target,.git,.kilo",
        help="逗号分隔的排除目录名（默认 target,.git,.kilo；传空字符串禁用排除）",
    )
    args = parser.parse_args()

    exclude_dirs = [d.strip() for d in args.exclude.split(",") if d.strip()]

    target = Path(args.path).resolve()
    if not target.exists():
        print(f"路径不存在: {target}", file=sys.stderr)
        return 1

    if target.is_file():
        if target.suffix != ".rs":
            print(f"不是 Rust 文件: {target}", file=sys.stderr)
            return 1
        files = [target]
        root = target.parent
    else:
        files = collect_rust_files(target, exclude_dirs)
        root = target

    if not files:
        print(f"未在 {target} 下找到 .rs 文件。")
        return 0

    all_stats = [analyze_file(f) for f in files]

    print_summary(target, root, files, all_stats)
    print_file_table(root, files, all_stats)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
