#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

usage() {
  cat <<EOF
Usage: $0 [verify|regenerate|timing] [--json-out <path>] [--repeat <n>]

Verifies, regenerates, or times the generated proof-carrying witness Lean
catalog gate. The timing mode follows the verify path and reports per-exporter
and per-Lake-target durations. Use --repeat with timing to report aggregate
min/median/max durations across repeated local runs.
EOF
}

mode="${1:-verify}"
if [[ $# -gt 0 ]]; then
  shift
fi
json_out=""
repeat_count=1

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

certificate_dir="docs/witness"
generated_dir="lean-proofs/PrimeArithmetic/Generated/Witness"
manifest="docs/witness/witness_lean_catalog_manifest.json"
matrix_manifest="docs/witness/witness_policy_matrix_lean_catalog_manifest.json"
catalog_checks="$generated_dir/CatalogChecks.lean"
catalog_check_shards=(
  "$generated_dir/CatalogChecksShard01.lean"
  "$generated_dir/CatalogChecksShard02.lean"
  "$generated_dir/CatalogChecksShard03.lean"
)
matrix_catalog_checks="$generated_dir/MatrixCatalogChecks.lean"
matrix_catalog_check_shards=(
  "$generated_dir/MatrixCatalogChecksShard01.lean"
  "$generated_dir/MatrixCatalogChecksShard02.lean"
  "$generated_dir/MatrixCatalogChecksShard03.lean"
  "$generated_dir/MatrixCatalogChecksShard04.lean"
  "$generated_dir/MatrixCatalogChecksShard05.lean"
  "$generated_dir/MatrixCatalogChecksShard06.lean"
)
tracked=(
  "$manifest"
  "$matrix_manifest"
  "$catalog_checks"
  "${catalog_check_shards[@]}"
  "$matrix_catalog_checks"
  "${matrix_catalog_check_shards[@]}"
  "$generated_dir/Seed60.lean"
  "$generated_dir/Teaching38.lean"
  "$generated_dir/TimestampPolicy29Trial0.lean"
  "$generated_dir/MatrixDecimalReadable22.lean"
  "$generated_dir/MatrixDecimalClassic22.lean"
  "$generated_dir/MatrixDecimalBreathing22.lean"
  "$generated_dir/MatrixDecimalReadable64.lean"
  "$generated_dir/MatrixDecimalReadable96.lean"
  "$generated_dir/MatrixDecimalClassic64.lean"
  "$generated_dir/MatrixDecimalBreathing64.lean"
  "$generated_dir/MatrixDecimalBreathing96.lean"
  "$generated_dir/MatrixDecimalClassic96.lean"
  "$generated_dir/MatrixBase30Wheel64.lean"
  "$generated_dir/MatrixBase30Wheel96.lean"
  "$generated_dir/MatrixBase6Compact18.lean"
  "$generated_dir/MatrixBase12Compact18.lean"
  "$generated_dir/MatrixBase6Compact64.lean"
  "$generated_dir/MatrixBase6Compact96.lean"
  "$generated_dir/MatrixBase12Compact64.lean"
  "$generated_dir/MatrixBase12Compact96.lean"
  "$generated_dir/MatrixBase30Wheel18.lean"
)
modules=(
  "PrimeArithmetic.Generated.Witness.CatalogChecks"
  "PrimeArithmetic.Generated.Witness.MatrixCatalogChecks"
)
wrapper_modules=(
  "PrimeArithmetic.Witness.TeachingSeedCertificate"
)
backup_dir=""
timing_rows=()
current_timing_run=1

cleanup() {
  if [[ -n "$backup_dir" && -d "$backup_dir" ]]; then
    rm -rf "$backup_dir"
  fi
}
trap cleanup EXIT

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
  done < <(find "$generated_dir" -maxdepth 1 -type f -name "$pattern" -print)
}

run_export() {
  cargo run --quiet --bin export_proof_carrying_witness_lean_certificate -- \
    --catalog \
    --certificate-dir "$certificate_dir" \
    --out-dir "$generated_dir" \
    --manifest-out "$manifest" >/dev/null
  cargo run --quiet --bin export_proof_carrying_witness_lean_catalog_checks -- \
    --manifest "$manifest" \
    --out "$catalog_checks" \
    --shard-size 1 \
    --module-prefix PrimeArithmetic.Generated.Witness >/dev/null
  prune_unexpected_shards 'CatalogChecksShard*.lean' "${catalog_check_shards[@]}"
  cargo run --quiet --bin export_proof_carrying_witness_lean_certificate -- \
    --policy-matrix-catalog \
    --certificate-dir "$certificate_dir/policy_matrix" \
    --out-dir "$generated_dir" \
    --manifest-out "$matrix_manifest" >/dev/null
  cargo run --quiet --bin export_proof_carrying_witness_lean_catalog_checks -- \
    --manifest "$matrix_manifest" \
    --out "$matrix_catalog_checks" \
    --shard-size 3 \
    --module-prefix PrimeArithmetic.Generated.Witness >/dev/null
  prune_unexpected_shards 'MatrixCatalogChecksShard*.lean' "${matrix_catalog_check_shards[@]}"
}

run_export_timed() {
  timed_cmd export-canonical-lean-catalog \
    cargo run --quiet --bin export_proof_carrying_witness_lean_certificate -- \
      --catalog \
      --certificate-dir "$certificate_dir" \
      --out-dir "$generated_dir" \
      --manifest-out "$manifest" >/dev/null
  timed_cmd export-canonical-catalog-checks \
    cargo run --quiet --bin export_proof_carrying_witness_lean_catalog_checks -- \
      --manifest "$manifest" \
      --out "$catalog_checks" \
      --shard-size 1 \
      --module-prefix PrimeArithmetic.Generated.Witness >/dev/null
  timed_cmd prune-canonical-catalog-check-shards \
    prune_unexpected_shards 'CatalogChecksShard*.lean' "${catalog_check_shards[@]}"
  timed_cmd export-policy-matrix-lean-catalog \
    cargo run --quiet --bin export_proof_carrying_witness_lean_certificate -- \
      --policy-matrix-catalog \
      --certificate-dir "$certificate_dir/policy_matrix" \
      --out-dir "$generated_dir" \
      --manifest-out "$matrix_manifest" >/dev/null
  timed_cmd export-policy-matrix-catalog-checks \
    cargo run --quiet --bin export_proof_carrying_witness_lean_catalog_checks -- \
      --manifest "$matrix_manifest" \
      --out "$matrix_catalog_checks" \
      --shard-size 3 \
      --module-prefix PrimeArithmetic.Generated.Witness >/dev/null
  timed_cmd prune-policy-matrix-check-shards \
    prune_unexpected_shards 'MatrixCatalogChecksShard*.lean' "${matrix_catalog_check_shards[@]}"
}

verify_no_drift() {
  for path in "${tracked[@]}"; do
    backup="$backup_dir/$(basename "$path")"
    if ! cmp -s "$backup" "$path"; then
      echo "Generated witness Lean certificate drift detected for $path" >&2
      diff -u "$backup" "$path" || true
      for restore_path in "${tracked[@]}"; do
        cp "$backup_dir/$(basename "$restore_path")" "$restore_path"
      done
      exit 1
    fi
  done
}

build_catalog_targets() {
  (
    cd lean-proofs
    # Catalog check modules import every generated witness module and elaborate a
    # silent declaration reference for every manifest theorem link.
    lake build "${modules[@]}" "${wrapper_modules[@]}" >/dev/null
  )
}

build_catalog_targets_timed() {
  for module in "${modules[@]}" "${wrapper_modules[@]}"; do
    timed_cmd "lake-build:$module" \
      bash -c 'cd lean-proofs && lake build "$1" >/dev/null' bash "$module"
  done
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

print(f"Witness Lean proof timing ({repeat_count} run{'s' if repeat_count != 1 else ''}):")
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
    "schema_version": "proof-carrying-witness-lean-timing-v2",
    "command": "scripts/lean_proof_carrying_witness_certificate.sh timing",
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

echo "Lean proof-carrying witness certificate: $mode"

if [[ "$mode" == "regenerate" ]]; then
  echo "  regenerating generated witness Lean catalog"
  run_export
elif [[ "$mode" == "timing" ]]; then
  backup_dir="$(mktemp -d)"
  for path in "${tracked[@]}"; do
    cp "$path" "$backup_dir/$(basename "$path")"
  done
  echo "  timing generated witness Lean catalog"
  for ((run = 1; run <= repeat_count; run++)); do
    current_timing_run="$run"
    if [[ "$repeat_count" != "1" ]]; then
      echo "  timing run $run/$repeat_count"
    fi
    run_export_timed
    verify_no_drift
    build_catalog_targets_timed
  done
  print_timing_report
  write_timing_json
else
  backup_dir="$(mktemp -d)"
  for path in "${tracked[@]}"; do
    cp "$path" "$backup_dir/$(basename "$path")"
  done
  echo "  verifying generated witness Lean catalog"
  run_export
  verify_no_drift
fi

if [[ "$mode" != "timing" ]]; then
  build_catalog_targets
fi

echo "Lean proof-carrying witness certificate $mode passed."
