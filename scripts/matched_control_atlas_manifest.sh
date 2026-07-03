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

tracked="docs/atlas/matched_control_smoke_atlas_manifest.json"
tmp_dir=""

cleanup() {
  if [[ -n "$tmp_dir" && -d "$tmp_dir" ]]; then
    rm -rf "$tmp_dir"
  fi
}
trap cleanup EXIT

run_export() {
  local out="$1"
  cargo run --bin export_matched_control_atlas_manifest -- \
    --panel smoke \
    --out "$out" >/dev/null
}

echo "Matched-control atlas manifest: $mode"

if [[ "$mode" == "regenerate" ]]; then
  echo "  regenerating $tracked"
  run_export "$tracked"
else
  tmp_dir="$(mktemp -d)"
  generated="$tmp_dir/$(basename "$tracked")"
  echo "  verifying $tracked"
  run_export "$generated"
  if ! diff -u "$tracked" "$generated" >/dev/null; then
    echo "Atlas manifest drift detected for $tracked" >&2
    diff -u "$tracked" "$generated" || true
    exit 1
  fi
fi

echo "Matched-control atlas manifest $mode passed."
