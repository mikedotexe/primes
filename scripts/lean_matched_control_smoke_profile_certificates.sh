#!/bin/bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

usage() {
  cat <<EOF
Usage: $0 [verify|regenerate] [--skip-build]

Verifies or regenerates the tracked matched-control smoke-profile certificate
module. By default, the script also builds the generated Lean module. Use
--skip-build only when a caller immediately builds an importing cached check
module.
EOF
}

mode="${1:-verify}"
if [[ $# -gt 0 ]]; then
  shift
fi
skip_build=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-build)
      skip_build=true
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      exit 1
      ;;
  esac
done

case "$mode" in
  verify|regenerate)
    ;;
  -h|--help)
    usage
    exit 0
    ;;
  *)
    usage >&2
    exit 1
    ;;
esac

tracked="lean-proofs/PrimeArithmetic/Generated/MatchedControlSmokeProfileCertificates.lean"
tmp_dir=""

cleanup() {
  if [[ -n "$tmp_dir" && -d "$tmp_dir" ]]; then
    rm -rf "$tmp_dir"
  fi
}
trap cleanup EXIT

run_export() {
  local out="$1"
  cargo run --bin export_matched_control_smoke_profile_certificates -- \
    --format lean-module \
    --out "$out" >/dev/null
}

echo "Lean matched-control smoke-profile certificates: $mode"

if [[ "$mode" == "regenerate" ]]; then
  echo "  regenerating $tracked"
  run_export "$tracked"
else
  tmp_dir="$(mktemp -d)"
  generated="$tmp_dir/$(basename "$tracked")"
  echo "  verifying $tracked"
  run_export "$generated"
  if ! diff -u "$tracked" "$generated" >/dev/null; then
    echo "Generated smoke-profile certificate drift detected for $tracked" >&2
    diff -u "$tracked" "$generated" || true
    exit 1
  fi
fi

if [[ "$skip_build" == false ]]; then
  (
    cd lean-proofs
    lake build PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates >/dev/null
  )
else
  echo "  skipping Lean build; caller is expected to build an importing check module"
fi

echo "Lean matched-control smoke-profile certificates $mode passed."
