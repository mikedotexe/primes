#!/bin/bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

mode="${1:-check}"

case "$mode" in
  check)
    cargo run --bin verify_verification_spine -- check
    (cd lean-proofs && lake build)
    (cd agda-proofs && ./scripts/verify-clean-spine.sh)
    ;;
  regenerate)
    cargo run --bin verify_verification_spine -- regenerate
    ;;
  *)
    echo "Usage: $0 [check|regenerate]" >&2
    exit 1
    ;;
esac
