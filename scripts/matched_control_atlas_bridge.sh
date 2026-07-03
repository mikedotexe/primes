#!/bin/bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

usage() {
  cat <<EOF
Usage: $0 [verify|regenerate|timing] [--json-out <path>] [--repeat <n>]

Verifies, regenerates, or times the matched-control empirical-to-Lean atlas
bridge. Timing mode follows the verify path and reports repeated per-step
durations for local performance diagnosis.
EOF
}

mode="${1:-verify}"
if [[ $# -gt 0 ]]; then
  shift
fi
json_out=""
repeat_count=1
tmp_dir=""
timing_rows=()
current_timing_run=1
generated_lean_dir="lean-proofs/PrimeArithmetic/Generated"
smoke_profile_check_umbrella="$generated_lean_dir/MatchedControlSmokeProfileCertificateChecks.lean"
smoke_profile_check_shards=(
  "$generated_lean_dir/MatchedControlSmokeProfileCertificateChecksShard01.lean"
  "$generated_lean_dir/MatchedControlSmokeProfileCertificateChecksShard02.lean"
  "$generated_lean_dir/MatchedControlSmokeProfileCertificateChecksShard03.lean"
  "$generated_lean_dir/MatchedControlSmokeProfileCertificateChecksShard04.lean"
  "$generated_lean_dir/MatchedControlSmokeProfileCertificateChecksShard05.lean"
)

while [[ $# -gt 0 ]]; do
  case "$1" in
    --json-out)
      if [[ $# -lt 2 ]]; then
        echo "--json-out requires a path" >&2
        exit 1
      fi
      json_out="$2"
      shift 2
      ;;
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

cleanup() {
  if [[ -n "$tmp_dir" && -d "$tmp_dir" ]]; then
    rm -rf "$tmp_dir"
  fi
}
trap cleanup EXIT

case "$mode" in
  verify|regenerate|timing)
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

if [[ "$mode" != "timing" && "$repeat_count" != "1" ]]; then
  echo "--repeat is only valid with timing mode" >&2
  exit 1
fi
if [[ "$mode" != "timing" && -n "$json_out" ]]; then
  echo "--json-out is only valid with timing mode" >&2
  exit 1
fi

now_ms() {
  python3 - <<'PY'
import time
print(time.monotonic_ns() // 1_000_000)
PY
}

timed_cmd() {
  local label="$1"
  shift
  local start_ms end_ms duration_ms status
  start_ms="$(now_ms)"
  set +e
  "$@"
  status=$?
  set -e
  end_ms="$(now_ms)"
  duration_ms=$((end_ms - start_ms))
  timing_rows+=("$current_timing_run|$label|$duration_ms|$status")
  return "$status"
}

prune_unexpected_shards() {
  local pattern="$1"
  shift
  local shard expected keep
  while IFS= read -r shard; do
    keep=false
    for expected in "$@"; do
      if [[ "$shard" == "$expected" ]]; then
        keep=true
        break
      fi
    done
    if [[ "$keep" == false ]]; then
      rm -f "$shard"
    fi
  done < <(find "$generated_lean_dir" -maxdepth 1 -type f -name "$pattern" -print)
}

assert_no_unexpected_shards() {
  local pattern="$1"
  shift
  local shard expected keep
  while IFS= read -r shard; do
    keep=false
    for expected in "$@"; do
      if [[ "$shard" == "$expected" ]]; then
        keep=true
        break
      fi
    done
    if [[ "$keep" == false ]]; then
      echo "Unexpected matched-control generated check shard: $shard" >&2
      return 1
    fi
  done < <(find "$generated_lean_dir" -maxdepth 1 -type f -name "$pattern" -print)
}

bridge_mode() {
  if [[ "$mode" == "timing" ]]; then
    printf "verify"
  else
    printf "%s" "$mode"
  fi
}

run_family_lane_catalog() {
  scripts/lean_matched_control_family_lanes.sh "$(bridge_mode)"
}

run_smoke_profile_certificates() {
  scripts/lean_matched_control_smoke_profile_certificates.sh "$(bridge_mode)" --skip-build
}

run_atlas_manifest() {
  scripts/matched_control_atlas_manifest.sh "$(bridge_mode)"
}

run_smoke_profile_lean_checks() {
  local generated_dir="$tmp_dir/matched_control_smoke_profile_certificate_checks_${current_timing_run}"
  local generated="$generated_dir/MatchedControlSmokeProfileCertificateChecks.lean"
  echo "Matched-control smoke-profile certificate Lean checks: $(bridge_mode)"
  mkdir -p "$generated_dir"
  cargo run --bin export_matched_control_smoke_profile_certificates -- \
    --format lean-silent-checks \
    --out "$generated" \
    --shard-size 4 \
    --module-prefix PrimeArithmetic.Generated \
    --shard-out-dir "$generated_dir" >/dev/null
  if [[ "$(bridge_mode)" == "regenerate" ]]; then
    cp "$generated" "$smoke_profile_check_umbrella"
    for shard in "${smoke_profile_check_shards[@]}"; do
      cp "$generated_dir/$(basename "$shard")" "$shard"
    done
    prune_unexpected_shards 'MatchedControlSmokeProfileCertificateChecksShard*.lean' \
      "${smoke_profile_check_shards[@]}"
  else
    if ! diff -u "$smoke_profile_check_umbrella" "$generated" >/dev/null; then
      echo "Matched-control smoke-profile certificate Lean check drift detected for $smoke_profile_check_umbrella" >&2
      diff -u "$smoke_profile_check_umbrella" "$generated" || true
      exit 1
    fi
    for shard in "${smoke_profile_check_shards[@]}"; do
      local generated_shard="$generated_dir/$(basename "$shard")"
      if ! diff -u "$shard" "$generated_shard" >/dev/null; then
        echo "Matched-control smoke-profile certificate Lean check shard drift detected for $shard" >&2
        diff -u "$shard" "$generated_shard" || true
        exit 1
      fi
    done
    assert_no_unexpected_shards 'MatchedControlSmokeProfileCertificateChecksShard*.lean' \
      "${smoke_profile_check_shards[@]}"
  fi
  (
    cd lean-proofs
    lake build PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificateChecks >/dev/null
  )
  echo "Matched-control smoke-profile certificate Lean checks $(bridge_mode) passed."
}

run_residue_mask_pair_proof_coverage() {
  local residue_dir="$tmp_dir/matched_control_residue_masks_${current_timing_run}"
  echo "Matched-control residue-mask pair proof coverage: verify"
  cargo run --bin export_matched_control_residue_masks -- \
    --panel smoke \
    --prime-bound 31 \
    --out-dir "$residue_dir" >/dev/null
  python3 - "$residue_dir/residue_masks.json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as handle:
    report = json.load(handle)

summary = report["summary"]
expected_summary = {
    "panel": "smoke",
    "panel_id": "canonical-smoke-v1",
    "prime_bound": 31,
    "lane_count": 16,
    "lane_modulus_row_count": 142,
    "pair_candidate_count": 90,
    "pair_fingerprint_row_count": 12,
    "pair_certified_count": 12,
    "pair_uncertified_count": 0,
    "same_boundary_candidate_count": 29,
    "same_boundary_k_distinction_candidate_count": 29,
}

summary_subset = {key: summary.get(key) for key in expected_summary}
if summary_subset != expected_summary:
    print("canonical smoke residue-mask summary drifted", file=sys.stderr)
    print(f"expected: {expected_summary}", file=sys.stderr)
    print(f"actual:   {summary_subset}", file=sys.stderr)
    sys.exit(1)

total = summary["pair_fingerprint_row_count"]
certified = summary["pair_certified_count"]
uncertified = summary["pair_uncertified_count"]

if uncertified != 0 or certified != total:
    print(
        "expected all pair fingerprints to be certified, "
        f"got certified={certified}, uncertified={uncertified}, total={total}",
        file=sys.stderr,
    )
    sys.exit(1)

candidate = summary.get("top_theorem_candidate")
expected_candidate = {
    "rank": 1,
    "selection_kind": "certified-follow-on-fingerprint",
    "left_family_code": "B10 ( 3, 3) k=(0,1) M=1",
    "right_family_code": "B10 ( 3, 3) k=(1,1) M=1",
    "pair_certified": True,
}
candidate_subset = None
if candidate is not None:
    candidate_subset = {key: candidate.get(key) for key in expected_candidate}
if candidate_subset != expected_candidate:
    print("canonical smoke residue-mask top theorem candidate drifted", file=sys.stderr)
    print(f"expected: {expected_candidate}", file=sys.stderr)
    print(f"actual:   {candidate_subset}", file=sys.stderr)
    sys.exit(1)

missing_residue_set_links = [
    (
        row["left_family_code"],
        row["right_family_code"],
    )
    for row in report["pair_fingerprint_rows"]
    if row.get("pair_certificate") is not None
    and not row.get("forbidden_residue_set_theorem")
]
if missing_residue_set_links:
    print(
        "expected every certified pair fingerprint to expose a top-level "
        "forbidden_residue_set_theorem",
        file=sys.stderr,
    )
    for left, right in missing_residue_set_links:
        print(f"missing: {left} vs {right}", file=sys.stderr)
    sys.exit(1)

print(f"pair-certified fingerprints: {certified}/{total}")
print("canonical smoke residue-mask summary matches expected counts")
print("canonical smoke residue-mask top theorem candidate matches expected row")
print("canonical smoke residue-mask top-level residue-set links are present")
PY
  echo "Matched-control residue-mask pair proof coverage verify passed."
}

run_theorem_queue() {
  local theorem_queue_tracked="docs/atlas/matched_control_theorem_queue.md"
  local theorem_queue_generated="$tmp_dir/matched_control_theorem_queue_${current_timing_run}.md"
  echo "Matched-control theorem queue: $(bridge_mode)"
  cargo run --bin export_matched_control_residue_masks -- \
    --panel smoke \
    --prime-bound 31 \
    --format theorem-queue \
    --out "$theorem_queue_generated" >/dev/null
  if [[ "$(bridge_mode)" == "regenerate" ]]; then
    cp "$theorem_queue_generated" "$theorem_queue_tracked"
  else
    if ! diff -u "$theorem_queue_tracked" "$theorem_queue_generated" >/dev/null; then
      echo "Matched-control theorem queue drift detected for $theorem_queue_tracked" >&2
      diff -u "$theorem_queue_tracked" "$theorem_queue_generated" || true
      exit 1
    fi
  fi
  echo "Matched-control theorem queue $(bridge_mode) passed."
}

run_top_candidate_lean_checks() {
  local tracked="lean-proofs/PrimeArithmetic/Generated/MatchedControlResidueTopCandidateChecks.lean"
  local generated="$tmp_dir/matched_control_residue_top_candidate_checks_${current_timing_run}.lean"
  echo "Matched-control residue-mask top theorem candidate Lean checks: $(bridge_mode)"
  cargo run --bin export_matched_control_residue_masks -- \
    --panel smoke \
    --prime-bound 31 \
    --format lean-candidate-silent-checks \
    --out "$generated" >/dev/null
  if [[ "$(bridge_mode)" == "regenerate" ]]; then
    cp "$generated" "$tracked"
  else
    if ! diff -u "$tracked" "$generated" >/dev/null; then
      echo "Matched-control residue-mask top theorem candidate Lean check drift detected for $tracked" >&2
      diff -u "$tracked" "$generated" || true
      exit 1
    fi
  fi
  (
    cd lean-proofs
    lake build PrimeArithmetic.Generated.MatchedControlResidueTopCandidateChecks >/dev/null
  )
  echo "Matched-control residue-mask top theorem candidate Lean checks $(bridge_mode) passed."
}

run_bridge_once() {
  echo "Matched-control atlas bridge: $(bridge_mode)"
  tmp_dir="$(mktemp -d)"
  run_family_lane_catalog
  run_smoke_profile_certificates
  run_atlas_manifest
  run_smoke_profile_lean_checks
  run_residue_mask_pair_proof_coverage
  run_theorem_queue
  run_top_candidate_lean_checks
  echo "Matched-control atlas bridge $(bridge_mode) passed."
}

run_bridge_timed_once() {
  timed_cmd matched-control-family-lane-catalog run_family_lane_catalog
  timed_cmd matched-control-smoke-profile-certificates run_smoke_profile_certificates
  timed_cmd matched-control-atlas-manifest run_atlas_manifest
  timed_cmd matched-control-smoke-profile-lean-checks run_smoke_profile_lean_checks
  timed_cmd residue-mask-pair-proof-coverage run_residue_mask_pair_proof_coverage
  timed_cmd matched-control-theorem-queue run_theorem_queue
  timed_cmd residue-mask-top-candidate-lean-checks run_top_candidate_lean_checks
}

print_timing_report() {
  python3 - "$repeat_count" "${timing_rows[@]}" <<'PY'
import statistics
import sys
from collections import defaultdict

repeat_count = int(sys.argv[1])
rows = []
for raw in sys.argv[2:]:
    run_index, step, duration_ms, status = raw.split("|")
    rows.append({
        "run_index": int(run_index),
        "step": step,
        "duration_ms": int(duration_ms),
        "status": int(status),
    })

by_run = defaultdict(list)
by_step = defaultdict(list)
for row in rows:
    by_run[row["run_index"]].append(row)
    by_step[row["step"]].append(row)

def fmt_ms(value):
    if isinstance(value, float) and not value.is_integer():
        return f"{value:.1f}"
    return str(int(value))

print(f"Matched-control atlas bridge timing ({repeat_count} run{'s' if repeat_count != 1 else ''}):")
print("  raw runs:")
print(f"    {'run':>3}  {'step':<58} {'ms':>10} {'status':>8}")
for run_index in sorted(by_run):
    total = 0
    max_status = 0
    for row in by_run[run_index]:
        total += row["duration_ms"]
        max_status = max(max_status, row["status"])
        print(f"    {run_index:>3}  {row['step']:<58} {row['duration_ms']:>10} {row['status']:>8}")
    print(f"    {run_index:>3}  {'total':<58} {total:>10} {max_status:>8}")

print("  aggregate:")
print(f"    {'step':<58} {'min_ms':>10} {'median_ms':>10} {'max_ms':>10} {'status':>8}")
for step in sorted(by_step):
    values = [row["duration_ms"] for row in by_step[step]]
    max_status = max(row["status"] for row in by_step[step])
    print(
        f"    {step:<58} {min(values):>10} "
        f"{fmt_ms(statistics.median(values)):>10} {max(values):>10} {max_status:>8}"
    )

totals = [sum(row["duration_ms"] for row in by_run[run_index]) for run_index in sorted(by_run)]
statuses = [max(row["status"] for row in by_run[run_index]) for run_index in sorted(by_run)]
print(
    f"    {'total':<58} {min(totals):>10} "
    f"{fmt_ms(statistics.median(totals)):>10} {max(totals):>10} {max(statuses):>8}"
)
PY
}

write_timing_json() {
  if [[ -z "$json_out" ]]; then
    return
  fi
  mkdir -p "$(dirname "$json_out")"
  python3 - "$json_out" "$repeat_count" "${timing_rows[@]}" <<'PY'
import json
import statistics
import sys
from collections import defaultdict

out = sys.argv[1]
repeat_count = int(sys.argv[2])
rows = []
for raw in sys.argv[3:]:
    run_index, step, duration_ms, status = raw.split("|")
    rows.append({
        "run_index": int(run_index),
        "step": step,
        "duration_ms": int(duration_ms),
        "status": int(status),
    })

by_run = defaultdict(list)
by_step = defaultdict(list)
for row in rows:
    by_run[row["run_index"]].append(row)
    by_step[row["step"]].append(row)

def median_value(values):
    value = statistics.median(values)
    return int(value) if isinstance(value, float) and value.is_integer() else value

run_totals = []
for run_index in sorted(by_run):
    run_rows = by_run[run_index]
    run_totals.append({
        "run_index": run_index,
        "total_duration_ms": sum(row["duration_ms"] for row in run_rows),
        "status": max(row["status"] for row in run_rows),
    })

summary_rows = []
for step in sorted(by_step):
    step_rows = by_step[step]
    values = [row["duration_ms"] for row in step_rows]
    summary_rows.append({
        "step": step,
        "count": len(values),
        "min_duration_ms": min(values),
        "median_duration_ms": median_value(values),
        "max_duration_ms": max(values),
        "status": max(row["status"] for row in step_rows),
    })

total_values = [row["total_duration_ms"] for row in run_totals]
if total_values:
    summary_rows.append({
        "step": "total",
        "count": len(total_values),
        "min_duration_ms": min(total_values),
        "median_duration_ms": median_value(total_values),
        "max_duration_ms": max(total_values),
        "status": max(row["status"] for row in run_totals),
    })

payload = {
    "schema_version": "proof-build-target-timing-v1",
    "command": "scripts/matched_control_atlas_bridge.sh timing",
    "repeat_count": repeat_count,
    "rows": rows,
    "run_totals": run_totals,
    "summary_rows": summary_rows,
    "total_duration_ms": sum(row["duration_ms"] for row in rows),
}
with open(out, "w", encoding="utf-8") as handle:
    json.dump(payload, handle, indent=2)
    handle.write("\n")
PY
}

if [[ "$mode" == "timing" ]]; then
  echo "Matched-control atlas bridge: timing"
  tmp_dir="$(mktemp -d)"
  for ((run = 1; run <= repeat_count; run++)); do
    current_timing_run="$run"
    if [[ "$repeat_count" != "1" ]]; then
      echo "  timing run $run/$repeat_count"
    fi
    run_bridge_timed_once
  done
  print_timing_report
  write_timing_json
  echo "Matched-control atlas bridge timing passed."
else
  run_bridge_once
fi
