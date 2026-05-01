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

catalog=(
  "10 2 3 3 lean-proofs/PrimeArithmetic/Generated/BoundedK/Base10M2Pair33.lean"
  "14 2 13 11 lean-proofs/PrimeArithmetic/Generated/BoundedK/Base14M2PairDB.lean"
  "22 2 17 19 lean-proofs/PrimeArithmetic/Generated/BoundedK/Base22M2PairHJ.lean"
  "34 2 25 9 lean-proofs/PrimeArithmetic/Generated/BoundedK/Base34M2PairP9.lean"
)

tmp_dir=""
cleanup() {
  if [[ -n "$tmp_dir" && -d "$tmp_dir" ]]; then
    rm -rf "$tmp_dir"
  fi
}
trap cleanup EXIT

if [[ "$mode" == "verify" ]]; then
  runtime_dir="$repo_root/lean-proofs/PrimeArithmetic/Generated/Runtime"
  mkdir -p "$runtime_dir"
  tmp_dir="$(mktemp -d "$runtime_dir/.verify-bounded-k.XXXXXX")"
fi

run_export() {
  local base="$1"
  local middle_length="$2"
  local outer="$3"
  local inner="$4"
  local out="$5"

  cargo run --bin export_bounded_k_transfer_witness -- \
    --base "$base" \
    --middle-length "$middle_length" \
    --outer "$outer" \
    --inner "$inner" \
    --out "$out" >/dev/null
}

echo "Lean bounded-k generated catalog: $mode"

for entry in "${catalog[@]}"; do
  read -r base middle_length outer inner tracked <<<"$entry"

  if [[ "$mode" == "regenerate" ]]; then
    echo "  regenerating $tracked"
    run_export "$base" "$middle_length" "$outer" "$inner" "$tracked"
  else
    echo "  verifying $tracked"
    backup="$tmp_dir/$(basename "$tracked").bak"
    cp "$tracked" "$backup"
    run_export "$base" "$middle_length" "$outer" "$inner" "$tracked"
    if ! diff -u "$backup" "$tracked" >/dev/null; then
      echo "Generated catalog drift detected for $tracked" >&2
      diff -u "$backup" "$tracked" || true
      cp "$backup" "$tracked"
      exit 1
    fi
    cp "$backup" "$tracked"
  fi
done

echo "Lean bounded-k generated catalog $mode passed."
