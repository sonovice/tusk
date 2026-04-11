#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path


LOG_PATTERN = re.compile(r"\b(warning|error)\b", re.IGNORECASE)


@dataclass
class FileResult:
    source: str
    mei: str | None
    svg: str | None
    convert_ok: bool
    render_ok: bool
    convert_exit: int
    render_exit: int | None
    convert_log: list[str]
    render_log: list[str]
    warnings: list[str]
    errors: list[str]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Convert MusicXML fixtures to MEI, render via Verovio, and summarize warnings/errors."
    )
    parser.add_argument(
        "--input-dir",
        type=Path,
        default=Path("tests/fixtures/musicxml"),
        help="Directory containing MusicXML fixtures.",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path("target/verovio-musicxml-analysis"),
        help="Directory for generated MEI, SVG, and reports.",
    )
    parser.add_argument(
        "--tusk-bin",
        default="target/debug/tusk",
        help="Path to the Tusk CLI binary.",
    )
    parser.add_argument(
        "--verovio-bin",
        default="verovio",
        help="Path to the Verovio CLI binary.",
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=None,
        help="Optional max number of files to process.",
    )
    return parser.parse_args()


def run(cmd: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(cmd, capture_output=True, text=True)


def collect_musicxml_files(root: Path, limit: int | None) -> list[Path]:
    files = [
        path
        for pattern in ("*.musicxml", "*.xml", "*.mxl")
        for path in root.rglob(pattern)
        if path.is_file()
    ]
    files = sorted(set(files))
    if limit is not None:
        files = files[:limit]
    return files


def classify_logs(lines: list[str]) -> tuple[list[str], list[str]]:
    warnings: list[str] = []
    errors: list[str] = []
    for line in lines:
        if not LOG_PATTERN.search(line):
            continue
        lowered = line.lower()
        if "error" in lowered:
            errors.append(line)
        elif "warning" in lowered:
            warnings.append(line)
    return warnings, errors


def normalize_log(text: str) -> list[str]:
    return [line.strip() for line in text.splitlines() if line.strip()]


def analyze_file(source: Path, input_root: Path, out_root: Path, tusk_bin: str, verovio_bin: str) -> FileResult:
    rel = source.relative_to(input_root)
    rel_no_suffix = rel.with_suffix("")
    mei_path = out_root / "mei" / rel_no_suffix.with_suffix(".mei")
    svg_path = out_root / "svg" / rel_no_suffix.with_suffix(".svg")
    mei_path.parent.mkdir(parents=True, exist_ok=True)
    svg_path.parent.mkdir(parents=True, exist_ok=True)

    convert = run([tusk_bin, "convert", str(source), str(mei_path)])
    convert_log = normalize_log(convert.stdout + convert.stderr)
    convert_ok = convert.returncode == 0 and mei_path.exists()

    render_log: list[str] = []
    render_exit: int | None = None
    render_ok = False
    if convert_ok:
        render = run(
            [
                verovio_bin,
                "-a",
                "-t",
                "svg",
                "-o",
                str(svg_path),
                str(mei_path),
            ]
        )
        render_exit = render.returncode
        render_log = normalize_log(render.stdout + render.stderr)
        render_ok = render.returncode == 0 and svg_path.exists()

    warnings, errors = classify_logs(convert_log + render_log)
    return FileResult(
        source=str(source),
        mei=str(mei_path) if mei_path.exists() else None,
        svg=str(svg_path) if svg_path.exists() else None,
        convert_ok=convert_ok,
        render_ok=render_ok,
        convert_exit=convert.returncode,
        render_exit=render_exit,
        convert_log=convert_log,
        render_log=render_log,
        warnings=warnings,
        errors=errors,
    )


def top_messages(results: list[FileResult], field: str, limit: int = 20) -> list[dict[str, object]]:
    counter: Counter[str] = Counter()
    examples: dict[str, str] = {}
    for result in results:
        for message in getattr(result, field):
            counter[message] += 1
            examples.setdefault(message, result.source)
    top: list[dict[str, object]] = []
    for message, count in counter.most_common(limit):
        top.append({"message": message, "count": count, "example": examples[message]})
    return top


def build_summary(results: list[FileResult]) -> dict[str, object]:
    convert_failures = [r for r in results if not r.convert_ok]
    render_failures = [r for r in results if r.convert_ok and not r.render_ok]
    flagged = [r for r in results if r.warnings or r.errors]

    warning_examples: dict[str, list[str]] = defaultdict(list)
    error_examples: dict[str, list[str]] = defaultdict(list)
    for result in results:
        for warning in result.warnings:
            if len(warning_examples[warning]) < 5:
                warning_examples[warning].append(result.source)
        for error in result.errors:
            if len(error_examples[error]) < 5:
                error_examples[error].append(result.source)

    return {
        "files_total": len(results),
        "convert_ok": sum(1 for r in results if r.convert_ok),
        "convert_failed": len(convert_failures),
        "render_ok": sum(1 for r in results if r.render_ok),
        "render_failed": len(render_failures),
        "flagged_files": len(flagged),
        "top_warnings": top_messages(results, "warnings"),
        "top_errors": top_messages(results, "errors"),
        "convert_failures": [
            {
                "source": r.source,
                "convert_exit": r.convert_exit,
                "log_tail": r.convert_log[-20:],
            }
            for r in convert_failures
        ],
        "render_failures": [
            {
                "source": r.source,
                "render_exit": r.render_exit,
                "log_tail": r.render_log[-20:],
            }
            for r in render_failures
        ],
        "warning_examples": warning_examples,
        "error_examples": error_examples,
    }


def write_markdown(summary: dict[str, object], out_path: Path) -> None:
    lines = [
        "# MusicXML -> MEI -> Verovio analysis",
        "",
        f"- files_total: {summary['files_total']}",
        f"- convert_ok: {summary['convert_ok']}",
        f"- convert_failed: {summary['convert_failed']}",
        f"- render_ok: {summary['render_ok']}",
        f"- render_failed: {summary['render_failed']}",
        f"- flagged_files: {summary['flagged_files']}",
        "",
        "## Top warnings",
    ]

    top_warnings = summary["top_warnings"]
    if top_warnings:
        for item in top_warnings:
            lines.append(f"- {item['count']}x {item['message']} ({item['example']})")
    else:
        lines.append("- none")

    lines.extend(["", "## Top errors"])
    top_errors = summary["top_errors"]
    if top_errors:
        for item in top_errors:
            lines.append(f"- {item['count']}x {item['message']} ({item['example']})")
    else:
        lines.append("- none")

    lines.extend(["", "## Convert failures"])
    convert_failures = summary["convert_failures"]
    if convert_failures:
        for item in convert_failures:
            lines.append(f"- {item['source']} (exit {item['convert_exit']})")
    else:
        lines.append("- none")

    lines.extend(["", "## Render failures"])
    render_failures = summary["render_failures"]
    if render_failures:
        for item in render_failures:
            lines.append(f"- {item['source']} (exit {item['render_exit']})")
    else:
        lines.append("- none")

    out_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()

    input_dir = args.input_dir.resolve()
    output_dir = args.output_dir.resolve()
    tusk_bin = shutil.which(args.tusk_bin) or args.tusk_bin
    verovio_bin = shutil.which(args.verovio_bin) or args.verovio_bin

    files = collect_musicxml_files(input_dir, args.limit)
    if not files:
        raise SystemExit(f"No MusicXML files found under {input_dir}")

    output_dir.mkdir(parents=True, exist_ok=True)

    results = [
        analyze_file(path, input_dir, output_dir, tusk_bin, verovio_bin)
        for path in files
    ]
    summary = build_summary(results)

    (output_dir / "results.json").write_text(
        json.dumps([result.__dict__ for result in results], indent=2),
        encoding="utf-8",
    )
    (output_dir / "summary.json").write_text(
        json.dumps(summary, indent=2),
        encoding="utf-8",
    )
    write_markdown(summary, output_dir / "summary.md")

    print(json.dumps(summary, indent=2))
    print(f"Reports written to {output_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
