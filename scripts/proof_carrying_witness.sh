#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

artifact_dir="docs/witness"
artifacts=(
  "seed60_proof_carrying_witness.json"
  "teaching38_proof_carrying_witness.json"
  "timestamp_policy_29d_trial0_proof_carrying_witness.json"
  "witness_certificate_manifest.json"
  "witness_search_policy_atlas.json"
  "witness_search_policy_atlas.md"
  "policy_matrix/matrix_decimal_readable_22d_seed0.json"
  "policy_matrix/matrix_decimal_classic_22d_seed0.json"
  "policy_matrix/matrix_decimal_breathing_22d_seed0.json"
  "policy_matrix/matrix_decimal_readable_64d_seed0.json"
  "policy_matrix/matrix_decimal_readable_96d_seed0.json"
  "policy_matrix/matrix_decimal_classic_64d_seed0.json"
  "policy_matrix/matrix_decimal_breathing_64d_seed0.json"
  "policy_matrix/matrix_decimal_classic_96d_seed0.json"
  "policy_matrix/matrix_decimal_breathing_96d_seed0.json"
  "policy_matrix/matrix_base30_wheel_64d_seed0.json"
  "policy_matrix/matrix_base30_wheel_96d_seed0.json"
  "policy_matrix/matrix_base6_compact_18d_seed0.json"
  "policy_matrix/matrix_base12_compact_18d_seed0.json"
  "policy_matrix/matrix_base6_compact_64d_seed0.json"
  "policy_matrix/matrix_base6_compact_96d_seed0.json"
  "policy_matrix/matrix_base12_compact_64d_seed0.json"
  "policy_matrix/matrix_base12_compact_96d_seed0.json"
  "policy_matrix/matrix_base30_wheel_18d_seed0.json"
)
certificates=(
  "seed60_proof_carrying_witness.json"
  "teaching38_proof_carrying_witness.json"
  "timestamp_policy_29d_trial0_proof_carrying_witness.json"
)

assert_policy_matrix_full_coverage() {
  local atlas="$1"
  if ! grep -Eq '"unpromoted_replay_candidate_count"[[:space:]]*:[[:space:]]*0' "$atlas"; then
    echo "proof-carrying witness policy matrix has unpromoted replay candidates: $atlas" >&2
    grep '"unpromoted_replay_candidate_count"' "$atlas" >&2 || true
    exit 1
  fi
  if ! grep -Eq '"atlas_only_large_candidate_count"[[:space:]]*:[[:space:]]*0' "$atlas"; then
    echo "proof-carrying witness policy matrix has atlas-only large candidates: $atlas" >&2
    grep '"atlas_only_large_candidate_count"' "$atlas" >&2 || true
    exit 1
  fi
}

usage() {
  cat <<EOF
Usage: $0 verify|regenerate

Verifies or regenerates the canonical proof-carrying witness artifact bundle.
The certificate is construction/residue evidence, not a primality proof.
The gate fails if the canonical smoke policy matrix has unpromoted replay rows.
EOF
}

generate() {
  local out="$1"
  cargo run --quiet --bin export_proof_carrying_witness_bundle -- --out-dir "$out" >/dev/null
  cargo run --quiet --bin export_proof_carrying_witness_search_policy_atlas -- \
    --certificate-dir "$out" \
    --out-dir "$out" >/dev/null
  local matrix_tmp="$out/.policy_matrix_tmp"
  cargo run --quiet --bin export_proof_carrying_witness_policy_matrix -- \
    --out-dir "$matrix_tmp" >/dev/null
  assert_policy_matrix_full_coverage "$matrix_tmp/witness_policy_matrix_atlas.json"
  mkdir -p "$out/policy_matrix"
  cp "$matrix_tmp/certificates/matrix_decimal_readable_22d_seed0.json" \
    "$out/policy_matrix/matrix_decimal_readable_22d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_decimal_classic_22d_seed0.json" \
    "$out/policy_matrix/matrix_decimal_classic_22d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_decimal_breathing_22d_seed0.json" \
    "$out/policy_matrix/matrix_decimal_breathing_22d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_decimal_readable_64d_seed0.json" \
    "$out/policy_matrix/matrix_decimal_readable_64d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_decimal_readable_96d_seed0.json" \
    "$out/policy_matrix/matrix_decimal_readable_96d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_decimal_classic_64d_seed0.json" \
    "$out/policy_matrix/matrix_decimal_classic_64d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_decimal_breathing_64d_seed0.json" \
    "$out/policy_matrix/matrix_decimal_breathing_64d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_decimal_classic_96d_seed0.json" \
    "$out/policy_matrix/matrix_decimal_classic_96d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_decimal_breathing_96d_seed0.json" \
    "$out/policy_matrix/matrix_decimal_breathing_96d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_base30_wheel_64d_seed0.json" \
    "$out/policy_matrix/matrix_base30_wheel_64d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_base30_wheel_96d_seed0.json" \
    "$out/policy_matrix/matrix_base30_wheel_96d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_base6_compact_18d_seed0.json" \
    "$out/policy_matrix/matrix_base6_compact_18d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_base12_compact_18d_seed0.json" \
    "$out/policy_matrix/matrix_base12_compact_18d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_base6_compact_64d_seed0.json" \
    "$out/policy_matrix/matrix_base6_compact_64d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_base6_compact_96d_seed0.json" \
    "$out/policy_matrix/matrix_base6_compact_96d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_base12_compact_64d_seed0.json" \
    "$out/policy_matrix/matrix_base12_compact_64d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_base12_compact_96d_seed0.json" \
    "$out/policy_matrix/matrix_base12_compact_96d_seed0.json"
  cp "$matrix_tmp/certificates/matrix_base30_wheel_18d_seed0.json" \
    "$out/policy_matrix/matrix_base30_wheel_18d_seed0.json"
  rm -rf "$matrix_tmp"
}

case "${1:-}" in
  -h|--help)
    usage
    exit 0
    ;;
  regenerate)
    generate "$artifact_dir"
    scripts/lean_proof_carrying_witness_certificate.sh regenerate
    echo "regenerated $artifact_dir"
    ;;
  verify)
    tmp="$(mktemp -d)"
    trap 'rm -rf "$tmp"' EXIT
    generate "$tmp"
    for artifact in "${artifacts[@]}"; do
      if ! cmp -s "$artifact_dir/$artifact" "$tmp/$artifact"; then
        echo "proof-carrying witness artifact is stale: $artifact_dir/$artifact" >&2
        diff -u "$artifact_dir/$artifact" "$tmp/$artifact" >&2 || true
        exit 1
      fi
    done
    for certificate in "${certificates[@]}"; do
      cargo run --quiet --bin verify-proof-carrying-witness -- "$artifact_dir/$certificate"
    done
    scripts/lean_proof_carrying_witness_certificate.sh verify
    echo "verified $artifact_dir"
    ;;
  *)
    usage >&2
    exit 2
    ;;
esac
