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

tracked_dir="docs/connector"
expected=(
  "connector_signal_atlas.json"
  "connector_signal_atlas.md"
  "artifact_manifest.json"
  "connector_width6_stress.json"
  "connector_width6_stress.md"
  "connector_width6_stress_manifest/artifact_manifest.json"
  "connector_replication_null_atlas.json"
  "connector_replication_null_atlas.md"
  "connector_replication_null_atlas_manifest/artifact_manifest.json"
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
  cargo run --quiet --bin export_connector_signal_atlas -- --out-dir "$out_dir" >/dev/null
  cargo run --quiet --bin export_connector_width6_stress -- --out-dir "$out_dir" >/dev/null
  cargo run --quiet --bin export_connector_replication_null_atlas -- --out-dir "$out_dir" >/dev/null
}

run_proof_link_checks() {
  local atlas="$1"
  local lean_out="$2"
  local modules=()
  cargo run --quiet --bin export_connector_signal_atlas_checks -- \
    --atlas "$atlas" \
    --out "$lean_out" >/dev/null
  while IFS= read -r module; do
    modules+=("$module")
  done < <(python3 - "$atlas" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as handle:
    atlas = json.load(handle)

modules = {
    link["lean_module"]
    for link in atlas.get("proof_links", [])
    if link.get("lean_module")
}
modules.update(
    row["lean_module"]
    for row in atlas.get("residue_survivor_rows", [])
    if row.get("lean_module") and row.get("lean_theorem")
)
follow_up = atlas.get("residual_target_follow_up")
if follow_up and follow_up.get("residue_null_lean_module") and follow_up.get("residue_null_lean_theorem"):
    modules.add(follow_up["residue_null_lean_module"])

for module in sorted(modules):
    print(module)
PY
)
  (
    cd lean-proofs
    if [[ "${#modules[@]}" -gt 0 ]]; then
      lake build "${modules[@]}"
    fi
    lake env lean "$lean_out"
  )
}

run_stress_proof_link_checks() {
  local stress="$1"
  local lean_out="$2"
  local modules=()
  cargo run --quiet --bin export_connector_width6_stress_checks -- \
    --stress "$stress" \
    --out "$lean_out" >/dev/null
  while IFS= read -r module; do
    modules+=("$module")
  done < <(python3 - "$stress" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as handle:
    report = json.load(handle)

modules = set()
screen = report.get("ladder_peak_matched_control_screen") or {}
probe = screen.get("digit8_edge_zoom_probe") or {}
profile = probe.get("residue_profile") or {}
for cell in profile.get("cell_profiles", []):
    for row in cell.get("separator_rows", []):
        if row.get("lean_module") and row.get("lean_theorem"):
            modules.add(row["lean_module"])

for module in sorted(modules):
    print(module)
PY
)
  (
    cd lean-proofs
    if [[ "${#modules[@]}" -gt 0 ]]; then
      lake build "${modules[@]}"
    fi
    lake env lean "$lean_out"
  )
}

assert_stress_digit8_best_separator_coverage() {
  local stress="$1"
  python3 - "$stress" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as handle:
    report = json.load(handle)

screen = report.get("ladder_peak_matched_control_screen") or {}
probe = screen.get("digit8_edge_zoom_probe") or {}
profile = probe.get("residue_profile") or {}
unbacked = profile.get("digit8_best_separator_unbacked_count")
backed = profile.get("digit8_best_separator_theorem_backed_count")
if unbacked is None or backed is None:
    print("Missing digit-8 best-separator theorem coverage fields", file=sys.stderr)
    sys.exit(1)
if unbacked != 0:
    print(
        f"Digit-8 best-separator theorem coverage incomplete: backed={backed} unbacked={unbacked}",
        file=sys.stderr,
    )
    sys.exit(1)
PY
}

echo "Connector signal atlas: $mode"
tmp_dir="$(mktemp -d)"

if [[ "$mode" == "regenerate" ]]; then
  echo "  regenerating $tracked_dir"
  run_export "$tracked_dir"
else
  echo "  verifying $tracked_dir"
  run_export "$tmp_dir"
  for file in "${expected[@]}"; do
    if ! diff -u "$tracked_dir/$file" "$tmp_dir/$file" >/dev/null; then
      echo "Connector signal atlas drift detected for $tracked_dir/$file" >&2
      diff -u "$tracked_dir/$file" "$tmp_dir/$file" || true
      exit 1
    fi
  done
fi

echo "  checking Lean proof links and row theorem declarations"
run_proof_link_checks \
  "$tracked_dir/connector_signal_atlas.json" \
  "$tmp_dir/ConnectorSignalAtlasChecks.lean"
run_stress_proof_link_checks \
  "$tracked_dir/connector_width6_stress.json" \
  "$tmp_dir/ConnectorWidth6StressChecks.lean"
echo "  checking digit-8 best-separator theorem coverage"
assert_stress_digit8_best_separator_coverage \
  "$tracked_dir/connector_width6_stress.json"

echo "Connector signal atlas $mode passed."
