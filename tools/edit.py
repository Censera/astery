#!/usr/bin/env python3
"""Make precise, verified edits to text files without loading file contents into the command line.

The tool reads the complete file internally, but editing commands identify small anchors,
line ranges, or regular expressions. Every mutating operation verifies its match count and
writes atomically only after all requested edits succeed.
"""

from __future__ import annotations

import argparse
import difflib
import os
import re
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path


class EditError(Exception):
    pass


@dataclass(frozen=True)
class Edit:
    kind: str
    start: str
    end: str | None = None
    replacement: str = ""
    count: int = 1


def match_positions(text: str, pattern: str, regex: bool = False) -> list[tuple[int, int]]:
    if regex:
        return [(match.start(), match.end()) for match in re.finditer(pattern, text, re.MULTILINE)]
    positions: list[tuple[int, int]] = []
    offset = 0
    while True:
        index = text.find(pattern, offset)
        if index < 0:
            return positions
        positions.append((index, index + len(pattern)))
        offset = index + max(len(pattern), 1)


def require_count(found: int, expected: int, description: str) -> None:
    if found != expected:
        raise EditError(f"expected {expected} match(es), found {found}: {description}")


def replace_matches(text: str, positions: list[tuple[int, int]], replacement: str) -> str:
    result = text
    for start, end in reversed(positions):
        result = result[:start] + replacement + result[end:]
    return result


def apply_edit(text: str, edit: Edit) -> str:
    if edit.count < 1:
        raise EditError("count must be at least 1")

    if edit.kind == "replace":
        positions = match_positions(text, edit.start)
        require_count(len(positions), edit.count, repr(edit.start))
        return replace_matches(text, positions, edit.replacement)

    if edit.kind == "regex":
        matches = list(re.finditer(edit.start, text, re.MULTILINE))
        require_count(len(matches), edit.count, edit.start)
        return re.sub(edit.start, edit.replacement, text, count=0, flags=re.MULTILINE)

    if edit.kind in {"before", "after", "delete"}:
        positions = match_positions(text, edit.start)
        require_count(len(positions), edit.count, repr(edit.start))
        if edit.kind == "before":
            return replace_matches(text, positions, edit.replacement + edit.start)
        if edit.kind == "after":
            return replace_matches(text, positions, edit.start + edit.replacement)
        return replace_matches(text, positions, "")

    if edit.kind in {"between", "section"}:
        if edit.end is None:
            raise EditError(f"{edit.kind} requires an end anchor")
        spans: list[tuple[int, int]] = []
        cursor = 0
        while True:
            start = text.find(edit.start, cursor)
            if start < 0:
                break
            end_start = text.find(edit.end, start + len(edit.start))
            if end_start < 0:
                raise EditError(f"end anchor not found after {edit.start!r}")
            end = end_start + len(edit.end)
            spans.append((start, end))
            cursor = end
        require_count(len(spans), edit.count, f"between {edit.start!r} and {edit.end!r}")
        if edit.kind == "section":
            return replace_matches(text, spans, edit.replacement)
        replacements = []
        for first, last in spans:
            inner_start = first + len(edit.start)
            inner_end = last - len(edit.end)
            replacements.append((first, last, text[first:inner_start] + edit.replacement + text[inner_end:last]))
        result = text
        for first, last, replacement in reversed(replacements):
            result = result[:first] + replacement + result[last:]
        return result

    if edit.kind == "lines":
        if edit.end is None:
            raise EditError("lines requires an end line")
        try:
            first = int(edit.start)
            last = int(edit.end)
        except ValueError as exc:
            raise EditError("line range must be numeric") from exc
        if first < 1 or last < first:
            raise EditError("invalid line range")
        lines = text.splitlines(keepends=True)
        if last > len(lines):
            raise EditError(f"line range {first}-{last} exceeds file length {len(lines)}")
        newline = "\n"
        if lines and "\r\n" in lines[0]:
            newline = "\r\n"
        replacement = edit.replacement
        if replacement and not replacement.endswith(("\n", "\r\n")):
            replacement += newline
        return "".join(lines[: first - 1]) + replacement + "".join(lines[last:])

    raise EditError(f"unknown edit kind: {edit.kind}")


def line_number(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


def numbered_chunk(text: str, first: int, last: int) -> str:
    if first < 1 or last < first:
        raise EditError("invalid line range")
    lines = text.splitlines(keepends=False)
    if last > len(lines):
        raise EditError(f"line range {first}-{last} exceeds file length {len(lines)}")
    width = len(str(last))
    return "\n".join(f"{number:>{width}} | {lines[number - 1]}" for number in range(first, last + 1))


def context(text: str, start: int, end: int, radius: int) -> str:
    first = line_number(text, start)
    last = line_number(text, max(start, end - 1))
    lines = text.splitlines(keepends=False)
    return numbered_chunk(text, max(1, first - radius), min(len(lines), last + radius))


def search(text: str, pattern: str, regex: bool, radius: int) -> str:
    positions = match_positions(text, pattern, regex=regex)
    if not positions:
        raise EditError(f"no match: {pattern!r}")
    chunks = []
    for index, (start, end) in enumerate(positions, 1):
        chunks.append(f"match {index}: lines {line_number(text, start)}-{line_number(text, max(start, end - 1))}")
        chunks.append(context(text, start, end, radius))
    return "\n\n".join(chunks)


def diff_text(old: str, new: str, path: Path) -> str:
    return "".join(
        difflib.unified_diff(
            old.splitlines(keepends=True),
            new.splitlines(keepends=True),
            fromfile=str(path),
            tofile=str(path),
        )
    )


def atomic_write(path: Path, text: str) -> None:
    mode = path.stat().st_mode
    with tempfile.NamedTemporaryFile("w", encoding="utf-8", dir=path.parent, delete=False) as file:
        temporary = Path(file.name)
        file.write(text)
        file.flush()
        os.fsync(file.fileno())
    try:
        os.chmod(temporary, mode)
        os.replace(temporary, path)
    except Exception:
        temporary.unlink(missing_ok=True)
        raise


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Search and precisely edit text files using small chunks and anchors.")
    parser.add_argument("path", type=Path)
    parser.add_argument("--write", action="store_true", help="write changes; otherwise only inspect or print")
    parser.add_argument("--diff", action="store_true", help="print the unified diff")
    parser.add_argument("--show", nargs=2, metavar=("FIRST", "LAST"), help="print a numbered line chunk")
    parser.add_argument("--search", metavar="TEXT", help="find exact text and print matching line context")
    parser.add_argument("--regex-search", metavar="PATTERN", help="find regex matches and print matching line context")
    parser.add_argument("--context", type=int, default=2, help="context lines for search output (default: 2)")
    parser.add_argument(
        "--edit",
        action="append",
        nargs="+",
        metavar="ARG",
        help=(
            "edit command: replace OLD NEW [COUNT]; regex PATTERN REPLACEMENT [COUNT]; "
            "before|after|delete TEXT [COUNT]; between START END REPLACEMENT [COUNT]; "
            "section START END REPLACEMENT [COUNT]; lines FIRST LAST REPLACEMENT"
        ),
    )
    return parser


def parse_edit(raw: list[str]) -> Edit:
    if not raw:
        raise EditError("empty edit")
    kind = raw[0]
    if kind in {"replace", "regex", "before", "after", "delete"}:
        if len(raw) not in {3, 4}:
            raise EditError(f"{kind} expects TEXT REPLACEMENT [COUNT]")
        return Edit(kind, raw[1], None, raw[2], int(raw[3]) if len(raw) == 4 else 1)
    if kind in {"between", "section"}:
        if len(raw) not in {4, 5}:
            raise EditError(f"{kind} expects START END REPLACEMENT [COUNT]")
        return Edit(kind, raw[1], raw[2], raw[3], int(raw[4]) if len(raw) == 5 else 1)
    if kind == "lines":
        if len(raw) != 4:
            raise EditError("lines expects FIRST LAST REPLACEMENT")
        return Edit(kind, raw[1], raw[2], raw[3], 1)
    raise EditError(f"unknown edit kind: {kind}")


def main() -> int:
    args = build_parser().parse_args()
    try:
        text = args.path.read_text(encoding="utf-8")
    except OSError as exc:
        print(f"error: cannot read {args.path}: {exc}", file=sys.stderr)
        return 1

    try:
        if args.show is not None:
            first, last = map(int, args.show)
            print(numbered_chunk(text, first, last))
        elif args.search is not None:
            print(search(text, args.search, regex=False, radius=args.context))
        elif args.regex_search is not None:
            print(search(text, args.regex_search, regex=True, radius=args.context))

        edits = [parse_edit(raw) for raw in (args.edit or [])]
        result = text
        for edit in edits:
            result = apply_edit(result, edit)
    except (EditError, ValueError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    if args.diff:
        output = diff_text(text, result, args.path)
        if output:
            sys.stdout.write(output)
        elif not args.show and not args.search and not args.regex_search:
            print("no changes")

    if args.write and result != text:
        try:
            atomic_write(args.path, result)
        except OSError as exc:
            print(f"error: cannot write {args.path}: {exc}", file=sys.stderr)
            return 1
    elif not args.show and not args.search and not args.regex_search and not args.diff and not args.write:
        sys.stdout.write(result)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
