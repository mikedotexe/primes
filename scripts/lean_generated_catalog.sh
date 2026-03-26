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
  "3 6 5 1 lean-proofs/PrimeArithmetic/Generated/Examples/WindowP3Base6Span5.lean"
  "5 10 5 1 lean-proofs/PrimeArithmetic/Generated/Examples/WindowP5Base10Span5.lean"
  "5 12 17 1 lean-proofs/PrimeArithmetic/Generated/Examples/WindowP5Base12Span17.lean"
  "11 30 5 1 lean-proofs/PrimeArithmetic/Generated/Examples/WindowP11Base30Span5.lean"
  "101 30 29 1 lean-proofs/PrimeArithmetic/Generated/Examples/WindowP101Base30Span29.lean"
  "163 30 35 1 lean-proofs/PrimeArithmetic/Generated/Examples/WindowP163Base30Span35.lean"
  "41 210 5 1 lean-proofs/PrimeArithmetic/Generated/Examples/WindowP41Base210Span5.lean"
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
  tmp_dir="$(mktemp -d "$runtime_dir/.verify.XXXXXX")"
fi

run_export() {
  local p="$1"
  local base="$2"
  local span="$3"
  local radius="$4"
  local out="$5"

  cargo run --bin export_window_certificate -- \
    --p "$p" \
    --base "$base" \
    --window-span "$span" \
    --exclude-radius "$radius" \
    --out "$out" >/dev/null
}

echo "Lean generated-artifact catalog: $mode"

for entry in "${catalog[@]}"; do
  read -r p base span radius tracked <<<"$entry"

  if [[ "$mode" == "regenerate" ]]; then
    echo "  regenerating $tracked"
    run_export "$p" "$base" "$span" "$radius" "$tracked"
  else
    echo "  verifying $tracked"
    backup="$tmp_dir/$(basename "$tracked").bak"
    cp "$tracked" "$backup"
    run_export "$p" "$base" "$span" "$radius" "$tracked"
    if ! diff -u "$backup" "$tracked" >/dev/null; then
      echo "Generated catalog drift detected for $tracked" >&2
      diff -u "$backup" "$tracked" || true
      cp "$backup" "$tracked"
      exit 1
    fi
    cp "$backup" "$tracked"
  fi
done

echo "Lean generated-artifact catalog $mode passed."
