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

tracked="agda-proofs/Examples/Generated/BoundedKTransferWitnessCatalog.agda"
tmp_dir=""
cleanup() {
  if [[ -n "$tmp_dir" && -d "$tmp_dir" ]]; then
    rm -rf "$tmp_dir"
  fi
}
trap cleanup EXIT

run_export() {
  local out="$1"
  cargo run --bin export_bounded_k_transfer_agda_summary -- --out "$out" >/dev/null
}

echo "Agda generated catalog: $mode"

if [[ "$mode" == "regenerate" ]]; then
  echo "  regenerating $tracked"
  run_export "$tracked"
else
  runtime_dir="$repo_root/agda-proofs/Examples/Generated/Runtime"
  mkdir -p "$runtime_dir"
  tmp_dir="$(mktemp -d "$runtime_dir/.verify-agda-catalog.XXXXXX")"
  candidate="$tmp_dir/BoundedKTransferWitnessCatalog.agda"
  echo "  verifying $tracked"
  run_export "$candidate"
  if ! diff -u "$tracked" "$candidate" >/dev/null; then
    echo "Generated catalog drift detected for $tracked" >&2
    diff -u "$tracked" "$candidate" || true
    exit 1
  fi
fi

echo "Agda generated catalog $mode passed."
