#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

usage() {
  cat <<EOF
Usage: $0

Runs the CI-facing proof-catalog gate:
  scripts/signal_spine.sh proof-catalog

Set SIGNAL_SPINE_RUN_ID or SIGNAL_SPINE_OUT_DIR to control report artifacts.
EOF
}

case "${1:-}" in
  -h|--help)
    usage
    exit 0
    ;;
  "")
    ;;
  *)
    usage >&2
    exit 2
    ;;
esac

exec scripts/signal_spine.sh proof-catalog
