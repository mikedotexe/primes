#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

usage() {
  cat <<EOF
Usage: $0 timing [--repeat <n>] [--out-dir <path>] [--cold-lean]

Builds a local proof-build performance atlas from repeated timing JSONs for:
  - witness Lean proof-catalog gate
  - matched-control atlas bridge
  - selected umbrella Lean build targets

Default output is reports/proof-build-observatory/<utc-run-id>/.
EOF
}

mode="${1:-timing}"
if [[ $# -gt 0 ]]; then
  shift
fi
repeat_count=3
run_id="$(date -u +%Y%m%dT%H%M%SZ)"
out_dir="reports/proof-build-observatory/$run_id"
cold_lean=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repeat)
      if [[ $# -lt 2 ]]; then
        echo "--repeat requires a positive integer" >&2
        exit 1
      fi
      if ! [[ "$2" =~ ^[1-9][0-9]*$ ]]; then
        echo "--repeat requires a positive integer" >&2
        exit 1
      fi
      repeat_count="$2"
      shift 2
      ;;
    --out-dir)
      if [[ $# -lt 2 ]]; then
        echo "--out-dir requires a path" >&2
        exit 1
      fi
      out_dir="$2"
      shift 2
      ;;
    --cold-lean)
      cold_lean=true
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
  timing)
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

raw_dir="$out_dir/raw"
witness_timing="$raw_dir/witness_lean_timing.json"
matched_control_timing="$raw_dir/matched_control_atlas_timing.json"
lean_umbrella_timing="$raw_dir/lean_umbrella_timing.json"
atlas_json="$out_dir/proof_build_observatory.json"
atlas_md="$out_dir/proof_build_observatory.md"
artifact_manifest="$out_dir/artifact_manifest.json"

mkdir -p "$raw_dir"

echo "Proof-build observatory timing"
echo "  repeats: $repeat_count"
echo "  out dir: $out_dir"

scripts/lean_proof_carrying_witness_certificate.sh timing \
  --repeat "$repeat_count" \
  --json-out "$witness_timing"

scripts/matched_control_atlas_bridge.sh timing \
  --repeat "$repeat_count" \
  --json-out "$matched_control_timing"

lean_args=(
  timing
  --repeat "$repeat_count"
  --json-out "$lean_umbrella_timing"
)
if [[ "$cold_lean" == true ]]; then
  lean_args+=(--cold-lean)
fi
scripts/lean_umbrella_build_timing.sh "${lean_args[@]}"

cargo run --quiet --bin export_proof_build_observatory -- \
  --witness-timing "$witness_timing" \
  --matched-control-timing "$matched_control_timing" \
  --lean-umbrella-timing "$lean_umbrella_timing" \
  --out-json "$atlas_json" \
  --out-md "$atlas_md" \
  --manifest-out "$artifact_manifest"

echo
echo "Proof-build observatory complete: $out_dir"
echo "  raw witness timing:         $witness_timing"
echo "  raw matched-control timing: $matched_control_timing"
echo "  raw Lean umbrella timing:   $lean_umbrella_timing"
echo "  atlas JSON:                 $atlas_json"
echo "  atlas Markdown:             $atlas_md"
echo "  artifact manifest:          $artifact_manifest"
