#!/bin/bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

mode="${1:-verify}"

case "$mode" in
  verify|regenerate)
    ;;
  *)
    echo "Usage: $0 [verify|regenerate]" >&2
    exit 1
    ;;
esac

tracked="lean-proofs/PrimeArithmetic/Generated/MatchedControlFamilyLanes.lean"
tmp_dir=""

cleanup() {
  if [[ -n "$tmp_dir" && -d "$tmp_dir" ]]; then
    rm -rf "$tmp_dir"
  fi
}
trap cleanup EXIT

run_export() {
  local out="$1"
  cargo run --bin export_matched_control_family_lanes -- \
    --panel smoke \
    --out "$out" >/dev/null
}

echo "Lean matched-control family-lane catalog: $mode"

if [[ "$mode" == "regenerate" ]]; then
  echo "  regenerating $tracked"
  run_export "$tracked"
else
  runtime_dir="$repo_root/lean-proofs/PrimeArithmetic/Generated/Runtime"
  mkdir -p "$runtime_dir"
  tmp_dir="$(mktemp -d "$runtime_dir/.family-lanes.XXXXXX")"
  backup="$tmp_dir/$(basename "$tracked").bak"
  cp "$tracked" "$backup"
  echo "  verifying $tracked"
  run_export "$tracked"
  if ! diff -u "$backup" "$tracked" >/dev/null; then
    echo "Generated catalog drift detected for $tracked" >&2
    diff -u "$backup" "$tracked" || true
    cp "$backup" "$tracked"
    exit 1
  fi
fi

echo "Lean matched-control family-lane catalog $mode passed."
