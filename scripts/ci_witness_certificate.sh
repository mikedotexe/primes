#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

usage() {
  cat <<EOF
Usage: $0

Runs the CI-facing proof-carrying witness certificate gate:
  scripts/proof_carrying_witness.sh verify
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

exec scripts/proof_carrying_witness.sh verify
