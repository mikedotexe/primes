#!/bin/bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

usage() {
  cat <<EOF
Usage: $0 [verify|regenerate] [--deep] [--timeout-seconds <n>]

Verifies or regenerates the lightweight signal catalog. The default verify mode
checks deterministic drift plus shallow catalog row artifact/gate validity.
Use --deep to run every known row drift gate with the provided per-row timeout.
EOF
}

mode="verify"
if [[ $# -gt 0 && "$1" != --* ]]; then
  mode="$1"
  shift
fi
verifier_args=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --deep)
      verifier_args+=("--deep")
      shift
      ;;
    --timeout-seconds)
      if [[ $# -lt 2 ]]; then
        echo "--timeout-seconds requires a positive integer" >&2
        exit 1
      fi
      if ! [[ "$2" =~ ^[1-9][0-9]*$ ]]; then
        echo "--timeout-seconds requires a positive integer" >&2
        exit 1
      fi
      verifier_args+=("--timeout-seconds" "$2")
      shift 2
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

tracked_dir="docs/signal_catalog"
expected=(
  "signal_catalog.json"
  "signal_catalog.md"
  "artifact_manifest.json"
)
tmp_dir=""

cleanup() {
  if [[ -n "$tmp_dir" && -d "$tmp_dir" ]]; then
    rm -rf "$tmp_dir"
  fi
}
trap cleanup EXIT

run_export() {
  local out_dir="$1"
  cargo run --quiet --bin export_signal_catalog -- --out-dir "$out_dir" >/dev/null
}

run_catalog_verify() {
  local catalog="$1"
  if [[ ${#verifier_args[@]} -gt 0 ]]; then
    cargo run --quiet --bin verify_signal_catalog -- \
      --catalog "$catalog" \
      "${verifier_args[@]}"
  else
    cargo run --quiet --bin verify_signal_catalog -- --catalog "$catalog"
  fi
}

echo "Signal catalog: $mode"

if [[ "$mode" == "regenerate" ]]; then
  echo "  regenerating $tracked_dir"
  run_export "$tracked_dir"
else
  tmp_dir="$(mktemp -d)"
  echo "  verifying $tracked_dir"
  run_export "$tmp_dir"
  for file in "${expected[@]}"; do
    if ! diff -u "$tracked_dir/$file" "$tmp_dir/$file" >/dev/null; then
      echo "Signal catalog drift detected for $tracked_dir/$file" >&2
      diff -u "$tracked_dir/$file" "$tmp_dir/$file" || true
      exit 1
    fi
  done
fi

echo "  checking catalog row artifacts and drift gates"
run_catalog_verify "$tracked_dir/signal_catalog.json"

echo "Signal catalog $mode passed."
