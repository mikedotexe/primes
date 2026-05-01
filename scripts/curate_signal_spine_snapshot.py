#!/usr/bin/env python3
"""Curate a compact signal-spine snapshot for git.

Raw signal-spine runs can contain logs, CSVs, PNGs, and report bundles. This
script promotes only the human summary plus a compact manifest subset.
"""

from __future__ import annotations

import argparse
import json
from datetime import datetime, timezone
from pathlib import Path


DEFAULT_OUT_DIR = Path("reports/signal-spine/curated")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--run-dir",
        required=True,
        type=Path,
        help="Signal-spine run directory containing signal_summary.md and run_manifest.json",
    )
    parser.add_argument(
        "--out-dir",
        default=DEFAULT_OUT_DIR,
        type=Path,
        help=f"Curated output directory (default: {DEFAULT_OUT_DIR})",
    )
    return parser.parse_args()


def compact_command(row: dict) -> dict:
    return {
        "group": row.get("group"),
        "name": row.get("name"),
        "exit_code": row.get("exit_code"),
        "passed": row.get("exit_code") == 0,
        "duration_seconds": row.get("duration_seconds"),
        "output_paths": row.get("output_paths", []),
        "command": row.get("command"),
    }


def compact_manifest(manifest: dict, run_dir: Path) -> dict:
    return {
        "curated_at_utc": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "source_run_dir": str(run_dir),
        "run_id": manifest.get("run_id"),
        "generated_at_utc": manifest.get("generated_at_utc"),
        "repo_root": manifest.get("repo_root"),
        "groups": manifest.get("groups", []),
        "settings": manifest.get("settings", {}),
        "commands": [compact_command(row) for row in manifest.get("commands", [])],
        "key_metrics": manifest.get("key_metrics", {}),
    }


def main() -> int:
    args = parse_args()
    run_dir = args.run_dir
    summary_path = run_dir / "signal_summary.md"
    manifest_path = run_dir / "run_manifest.json"

    if not run_dir.is_dir():
        raise SystemExit(f"run directory does not exist: {run_dir}")
    if not summary_path.is_file():
        raise SystemExit(f"missing signal summary: {summary_path}")
    if not manifest_path.is_file():
        raise SystemExit(f"missing run manifest: {manifest_path}")

    manifest = json.loads(manifest_path.read_text())
    args.out_dir.mkdir(parents=True, exist_ok=True)

    (args.out_dir / "latest_signal_summary.md").write_text(summary_path.read_text())
    (args.out_dir / "latest_run_manifest.json").write_text(
        json.dumps(compact_manifest(manifest, run_dir), indent=2) + "\n"
    )

    print(f"curated signal-spine snapshot from {run_dir} into {args.out_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
