#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

usage() {
  cat <<EOF
Usage: $0 timing [--json-out <path>] [--repeat <n>] [--cold-lean]

Times selected Lean umbrella/catalog targets for local proof-build diagnosis.
Warm-cache timing is the default. With --cold-lean, lake clean runs before each
repeat and is included as a timed step.
EOF
}

mode="${1:-timing}"
if [[ $# -gt 0 ]]; then
  shift
fi
json_out=""
repeat_count=1
cold_lean=false
timing_rows=()
current_timing_run=1
targets=(
  "PrimeArithmetic.Generated.Witness.CatalogChecks"
  "PrimeArithmetic.Generated.Witness.MatrixCatalogChecks"
  "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificates"
  "PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificateChecks"
  "PrimeArithmetic.Generated.MatchedControlResidueTopCandidateChecks"
  "PrimeArithmetic.Witness.SearchReplayCertificate"
  "PrimeArithmetic"
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

build_target() {
  local target="$1"
  (
    cd lean-proofs
    lake build "$target" >/dev/null
  )
}

clean_lean() {
  (
    cd lean-proofs
    lake clean >/dev/null
  )
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

print(f"Lean umbrella build timing ({repeat_count} run{'s' if repeat_count != 1 else ''}):")
print("  raw runs:")
print(f"    {'run':>3}  {'step':<72} {'ms':>10} {'status':>8}")
for run_index in sorted(by_run):
    total = 0
    max_status = 0
    for row in by_run[run_index]:
        total += row["duration_ms"]
        max_status = max(max_status, row["status"])
        print(f"    {run_index:>3}  {row['step']:<72} {row['duration_ms']:>10} {row['status']:>8}")
    print(f"    {run_index:>3}  {'total':<72} {total:>10} {max_status:>8}")

print("  aggregate:")
print(f"    {'step':<72} {'min_ms':>10} {'median_ms':>10} {'max_ms':>10} {'status':>8}")
for step in sorted(by_step):
    values = [row["duration_ms"] for row in by_step[step]]
    max_status = max(row["status"] for row in by_step[step])
    print(
        f"    {step:<72} {min(values):>10} "
        f"{fmt_ms(statistics.median(values)):>10} {max(values):>10} {max_status:>8}"
    )

totals = [sum(row["duration_ms"] for row in by_run[run_index]) for run_index in sorted(by_run)]
statuses = [max(row["status"] for row in by_run[run_index]) for run_index in sorted(by_run)]
print(
    f"    {'total':<72} {min(totals):>10} "
    f"{fmt_ms(statistics.median(totals)):>10} {max(totals):>10} {max(statuses):>8}"
)
PY
}

write_timing_json() {
  if [[ -z "$json_out" ]]; then
    return
  fi
  mkdir -p "$(dirname "$json_out")"
  local cache_mode="warm"
  if [[ "$cold_lean" == true ]]; then
    cache_mode="cold-lean"
  fi
  python3 - "$json_out" "$repeat_count" "$cache_mode" "${timing_rows[@]}" <<'PY'
import json
import statistics
import sys
from collections import defaultdict

out = sys.argv[1]
repeat_count = int(sys.argv[2])
cache_mode = sys.argv[3]
rows = []
for raw in sys.argv[4:]:
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
    "command": "scripts/lean_umbrella_build_timing.sh timing",
    "cache_mode": cache_mode,
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

echo "Lean umbrella build timing"
for ((run = 1; run <= repeat_count; run++)); do
  current_timing_run="$run"
  if [[ "$repeat_count" != "1" ]]; then
    echo "  timing run $run/$repeat_count"
  fi
  if [[ "$cold_lean" == true ]]; then
    timed_cmd lake-clean clean_lean
  fi
  for target in "${targets[@]}"; do
    timed_cmd "lake-build:$target" build_target "$target"
  done
done
print_timing_report
write_timing_json
echo "Lean umbrella build timing passed."
