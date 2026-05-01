#!/usr/bin/env bash

set -u -o pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

RUN_ID="${SIGNAL_SPINE_RUN_ID:-$(date -u +%Y%m%dT%H%M%SZ)}"
OUT_ROOT="${SIGNAL_SPINE_OUT_DIR:-reports/signal-spine/$RUN_ID}"
MATCHED_SAMPLES="${SIGNAL_SPINE_MATCHED_SAMPLES:-100}"
MATCHED_MIN_SEED_LEN="${SIGNAL_SPINE_MATCHED_MIN_SEED_LEN:-1}"
MATCHED_MAX_SEED_LEN="${SIGNAL_SPINE_MATCHED_MAX_SEED_LEN:-2}"

DEFAULT_GROUPS=(core membrane connector affine transfer matched-control)
ALL_GROUPS=(core membrane connector affine transfer matched-control fast-generation phase-residual shift-phase unit-cycle base-neighbor base57-codec proof-catalog)

usage() {
  cat <<EOF
Usage: $0 [--all] [group ...]

Groups:
  core             Prime-count smoke, prime-anchor verification, library tests
  membrane         Deterministic generator plus palindrome/scaffold probes
  connector        Connector and comparative signal reports
  affine           Affine period-lock, residue-torus, gradient-transition, and classifier reports
  transfer         M2/M3 transfer collapse and direct transfer-criterion audit
  matched-control  Membrane-vs-random-coprime matched-control run
  fast-generation  Non-default deterministic u64 affine generation throughput smoke
  phase-residual   Non-default cross-base compact affine phase residual atlas
  shift-phase      Non-default curated shift-phase signal mining follow-up
  unit-cycle       Non-default unit-cycle normalized phase signal report
  base-neighbor    Non-default unit-cycle base-neighbor geometry scout
  base57-codec     Non-default base57 affine codec experiment
  proof-catalog    Generated Lean/Agda catalog drift checks

Environment:
  SIGNAL_SPINE_RUN_ID             Override run id
  SIGNAL_SPINE_OUT_DIR            Override output directory
  SIGNAL_SPINE_MATCHED_SAMPLES    Samples per family for matched-control (default: 100)
EOF
}

requested=()
if [[ "$#" -eq 0 ]]; then
  requested=("${DEFAULT_GROUPS[@]}")
else
  while [[ "$#" -gt 0 ]]; do
    case "$1" in
      -h|--help)
        usage
        exit 0
        ;;
      --all)
        requested=("${ALL_GROUPS[@]}")
        shift
        ;;
      *)
        requested+=("$1")
        shift
        ;;
    esac
  done
fi

for group in "${requested[@]}"; do
  known=false
  for candidate in "${ALL_GROUPS[@]}"; do
    if [[ "$group" == "$candidate" ]]; then
      known=true
      break
    fi
  done
  if [[ "$known" != true ]]; then
    echo "Unknown signal spine group: $group" >&2
    usage >&2
    exit 2
  fi
done

group_enabled() {
  local needle="$1"
  local group
  for group in "${requested[@]}"; do
    if [[ "$group" == "$needle" ]]; then
      return 0
    fi
  done
  return 1
}

mkdir -p "$OUT_ROOT/stdout" "$OUT_ROOT/core" "$OUT_ROOT/membrane" "$OUT_ROOT/connector" \
  "$OUT_ROOT/affine" "$OUT_ROOT/transfer" "$OUT_ROOT/matched-control" \
  "$OUT_ROOT/fast-generation" "$OUT_ROOT/phase-residual" "$OUT_ROOT/shift-phase" \
  "$OUT_ROOT/unit-cycle" "$OUT_ROOT/base-neighbor" "$OUT_ROOT/base57-codec" \
  "$OUT_ROOT/proof-catalog"

COMMANDS_TSV="$OUT_ROOT/commands.tsv"
printf "group\tname\texit_code\tduration_seconds\tlog_path\toutput_paths\tcommand\n" > "$COMMANDS_TSV"

json_array() {
  local IFS=,
  printf "%s" "$*"
}

run_cmd() {
  local group="$1"
  local name="$2"
  local outputs="$3"
  shift 3

  local log="$OUT_ROOT/stdout/${group}__${name}.log"
  local started ended status duration

  echo "==> [$group] $name"
  echo "    command: $*"
  started="$(date +%s)"
  "$@" > "$log" 2>&1
  status=$?
  ended="$(date +%s)"
  duration=$((ended - started))

  printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\n" \
    "$group" "$name" "$status" "$duration" "$log" "$outputs" "$*" >> "$COMMANDS_TSV"

  if [[ "$status" -ne 0 ]]; then
    echo "    exit: $status (see $log)"
  else
    echo "    ok (${duration}s)"
  fi
}

run_core() {
  run_cmd core prime_count_smoke "" \
    cargo run --release --example prime_count_smoke_test
  run_cmd core prime_verification "" \
    cargo run --release --example prime_verification_report
  run_cmd core lib_tests "" \
    cargo test --lib
}

run_membrane() {
  run_cmd membrane proper_generator "" \
    cargo run --release --example proper_membrane_generator
  run_cmd membrane palindrome_probe "" \
    cargo run --release --example membrane_palindrome_probe
  run_cmd membrane scaffold_probe "" \
    cargo run --release --example membrane_scaffold_probe
}

run_connector() {
  run_cmd connector connector_signal \
    "$OUT_ROOT/connector/connector_signal.json,$OUT_ROOT/connector/connector_signal_positions.csv,$OUT_ROOT/connector/connector_signal_sweep.csv" \
    cargo run --release --example connector_signal_report -- \
      --json-out "$OUT_ROOT/connector/connector_signal.json" \
      --csv-out "$OUT_ROOT/connector/connector_signal_positions.csv" \
      --sweep-csv-out "$OUT_ROOT/connector/connector_signal_sweep.csv"
  run_cmd connector comparative_signal \
    "$OUT_ROOT/connector/comparative_signal.json,$OUT_ROOT/connector/comparative_signal.csv" \
    cargo run --release --example comparative_signal_report -- \
      --json-out "$OUT_ROOT/connector/comparative_signal.json" \
      --csv-out "$OUT_ROOT/connector/comparative_signal.csv"
}

run_affine() {
  run_cmd affine period_lock "$OUT_ROOT/affine/period_lock" \
    cargo run --release --example affine_period_lock_report -- \
      --out-dir "$OUT_ROOT/affine/period_lock"
  run_cmd affine residue_torus "$OUT_ROOT/affine/residue_torus" \
    cargo run --release --example residue_torus_period_lock_report -- \
      --out-dir "$OUT_ROOT/affine/residue_torus"
  run_cmd affine gradient_transition "$OUT_ROOT/affine/gradient_transition" \
    cargo run --release --example affine_gradient_transition_report -- \
      --out-dir "$OUT_ROOT/affine/gradient_transition"
  run_cmd affine hinge_classifier "$OUT_ROOT/affine/hinge_classifier" \
    cargo run --release --example affine_hinge_classifier_report -- \
      --out-dir "$OUT_ROOT/affine/hinge_classifier"
}

run_transfer() {
  run_cmd transfer m2_m3_transfer_collapse "$OUT_ROOT/transfer/m2_m3_transfer_collapse" \
    cargo run --release --example m2_m3_transfer_collapse_report -- \
      --out-dir "$OUT_ROOT/transfer/m2_m3_transfer_collapse"
  run_cmd transfer bounded_k_transfer_criterion "$OUT_ROOT/transfer/bounded_k_transfer_criterion" \
    cargo run --release --example bounded_k_transfer_criterion_report -- \
      --out-dir "$OUT_ROOT/transfer/bounded_k_transfer_criterion"
}

run_matched_control() {
  run_cmd matched-control membrane_vs_random \
    "$OUT_ROOT/matched-control/membrane_vs_random.json,$OUT_ROOT/matched-control/membrane_vs_random.csv" \
    cargo run --release --example membrane_vs_random -- \
      --samples "$MATCHED_SAMPLES" \
      --min-seed-len "$MATCHED_MIN_SEED_LEN" \
      --max-seed-len "$MATCHED_MAX_SEED_LEN" \
      --json-out "$OUT_ROOT/matched-control/membrane_vs_random.json" \
      --csv-out "$OUT_ROOT/matched-control/membrane_vs_random.csv"
}

run_fast_generation() {
  run_cmd fast-generation membrane_prime_fast_smoke \
    "$OUT_ROOT/fast-generation/membrane_prime_fast.json,$OUT_ROOT/fast-generation/membrane_prime_fast_witnesses.csv" \
    cargo run --release --bin membrane-prime-fast -- \
      --base 10 \
      --outer 3 \
      --inner 7 \
      --k 2,1 \
      --middle-length 2 \
      --seed-count 10000 \
      --max-primes 5 \
      --json-out "$OUT_ROOT/fast-generation/membrane_prime_fast.json" \
      --csv-out "$OUT_ROOT/fast-generation/membrane_prime_fast_witnesses.csv"
  run_cmd fast-generation membrane_prime_throughput_report "$OUT_ROOT/fast-generation/throughput_report" \
    cargo run --release --example membrane_prime_throughput_report -- \
      --out-dir "$OUT_ROOT/fast-generation/throughput_report"
}

run_phase_residual() {
  run_cmd phase-residual affine_phase_residual_atlas "$OUT_ROOT/phase-residual/affine_phase_residual_atlas" \
    cargo run --release --example affine_phase_residual_atlas_report -- \
      --out-dir "$OUT_ROOT/phase-residual/affine_phase_residual_atlas"
}

run_shift_phase() {
  run_cmd shift-phase shift_phase_signal_mining "$OUT_ROOT/shift-phase/shift_phase_signal_mining" \
    cargo run --release --example shift_phase_signal_mining_report -- \
      --out-dir "$OUT_ROOT/shift-phase/shift_phase_signal_mining"
}

run_unit_cycle() {
  run_cmd unit-cycle unit_cycle_phase_signal "$OUT_ROOT/unit-cycle/unit_cycle_phase_signal" \
    cargo run --release --example unit_cycle_phase_signal_report -- \
      --out-dir "$OUT_ROOT/unit-cycle/unit_cycle_phase_signal"
}

run_base_neighbor() {
  run_cmd base-neighbor unit_cycle_base_neighbor "$OUT_ROOT/base-neighbor/unit_cycle_base_neighbor" \
    cargo run --release --example unit_cycle_base_neighbor_report -- \
      --out-dir "$OUT_ROOT/base-neighbor/unit_cycle_base_neighbor"
}

run_base57_codec() {
  run_cmd base57-codec base57_affine_codec "$OUT_ROOT/base57-codec/base57_affine_codec" \
    cargo run --release --example base57_affine_codec_report -- \
      --out-dir "$OUT_ROOT/base57-codec/base57_affine_codec"
}

run_proof_catalog() {
  run_cmd proof-catalog lean_generated_catalog "" \
    scripts/lean_generated_catalog.sh verify
  run_cmd proof-catalog lean_bounded_k_catalog "" \
    scripts/lean_bounded_k_catalog.sh verify
  run_cmd proof-catalog agda_generated_catalog "" \
    scripts/agda_generated_catalog.sh verify
}

for group in "${DEFAULT_GROUPS[@]}" fast-generation phase-residual shift-phase unit-cycle base-neighbor base57-codec proof-catalog; do
  if group_enabled "$group"; then
    case "$group" in
      core) run_core ;;
      membrane) run_membrane ;;
      connector) run_connector ;;
      affine) run_affine ;;
      transfer) run_transfer ;;
      matched-control) run_matched_control ;;
      fast-generation) run_fast_generation ;;
      phase-residual) run_phase_residual ;;
      shift-phase) run_shift_phase ;;
      unit-cycle) run_unit_cycle ;;
      base-neighbor) run_base_neighbor ;;
      base57-codec) run_base57_codec ;;
      proof-catalog) run_proof_catalog ;;
    esac
  fi
done

python3 - "$OUT_ROOT" "$RUN_ID" "$(json_array "${requested[@]}")" \
  "$MATCHED_SAMPLES" "$MATCHED_MIN_SEED_LEN" "$MATCHED_MAX_SEED_LEN" <<'PY'
import csv
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

out_root = Path(sys.argv[1])
run_id = sys.argv[2]
groups = [g for g in sys.argv[3].split(",") if g]
matched_samples = int(sys.argv[4])
matched_min_seed_len = int(sys.argv[5])
matched_max_seed_len = int(sys.argv[6])

commands = []
with (out_root / "commands.tsv").open(newline="") as handle:
    reader = csv.DictReader(handle, delimiter="\t")
    for row in reader:
        row["exit_code"] = int(row["exit_code"])
        row["duration_seconds"] = int(row["duration_seconds"])
        row["output_paths"] = [p for p in row["output_paths"].split(",") if p]
        commands.append(row)

def read(path):
    p = Path(path)
    if not p.exists():
        return ""
    return p.read_text(errors="replace")

def first_match(text, pattern):
    match = re.search(pattern, text, re.MULTILINE)
    return match.group(0).strip() if match else None

def markdown_bullets(text):
    bullets = []
    current = None
    for raw_line in text.splitlines():
        line = raw_line.strip()
        if line.startswith("- "):
            if current:
                bullets.append(current)
            current = line
        elif current and line and not line.startswith("#") and not line.startswith("|"):
            current += f" {line}"
        elif current:
            bullets.append(current)
            current = None
    if current:
        bullets.append(current)
    return bullets

def report_bullets(text):
    skip_prefixes = (
        "- output dir:",
        "- main bases:",
        "- appendix bases:",
        "- middle lengths:",
        "- from lane:",
        "- noncompact lanes:",
        "- Main bases:",
        "- Appendix bases:",
        "- Middle lengths:",
        "- Surfaces:",
    )
    return [
        bullet
        for bullet in markdown_bullets(text)
        if not any(bullet.startswith(prefix) for prefix in skip_prefixes)
    ]

metrics = {}
for command in commands:
    key = f"{command['group']}.{command['name']}"
    text = read(command["log_path"])
    snippets = []
    for pattern in [
        r"test result: ok\..*",
        r".*All smoke tests PASSED.*",
        r".*Maintained prime anchors.*verified successfully.*",
        r".*period lock exactly matches.*",
        r".*gradient_only.*direct-lane share.*",
        r".*M2 all.*",
        r".*M3 all.*",
        r".*Residual criterion:.*",
        r".*Raw-count broader-law candidate.*",
        r".*Density-corrected residual asymmetry.*",
    ]:
        found = first_match(text, pattern)
        if found:
            snippets.append(found)
    if snippets:
        metrics[key] = snippets

for report_path in [
    out_root / "affine" / "period_lock" / "report.md",
    out_root / "affine" / "residue_torus" / "report.md",
    out_root / "affine" / "gradient_transition" / "report.md",
    out_root / "fast-generation" / "throughput_report" / "report.md",
    out_root / "transfer" / "m2_m3_transfer_collapse" / "report.md",
    out_root / "transfer" / "bounded_k_transfer_criterion" / "report.md",
    out_root / "base-neighbor" / "unit_cycle_base_neighbor" / "report.md",
    out_root / "base57-codec" / "base57_affine_codec" / "report.md",
]:
    text = read(report_path)
    if text:
        bullets = report_bullets(text)
        if bullets:
            metrics[str(report_path.relative_to(out_root))] = bullets[:6]

manifest = {
    "run_id": run_id,
    "generated_at_utc": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
    "repo_root": str(Path.cwd()),
    "groups": groups,
    "settings": {
        "matched_samples": matched_samples,
        "matched_min_seed_len": matched_min_seed_len,
        "matched_max_seed_len": matched_max_seed_len,
    },
    "commands": commands,
    "key_metrics": metrics,
}
(out_root / "run_manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")

failed = [c for c in commands if c["exit_code"] != 0]
total_duration = sum(c["duration_seconds"] for c in commands)

lines = [
    "# Signal Spine Summary",
    "",
    f"- Run id: `{run_id}`",
    f"- Groups: `{', '.join(groups)}`",
    f"- Commands: `{len(commands)}` total, `{len(failed)}` failed",
    f"- Total command duration: `{total_duration}s`",
    "",
    "## Steelman Reading",
    "",
    "- Symmetric digit templates are strongest as affine seed-search surfaces: fixed layout gives `candidate = shift + gradient * seed`.",
    "- Exact residue filters explain much of the generator's usefulness; observed lift over naive random baselines must still be checked against coprime, same-size controls.",
    "- Affine lane signals such as period lock and gradient-only pockets are useful research lenses, not public density theorems.",
    "",
    "## Command Status",
    "",
]
for command in commands:
    mark = "PASS" if command["exit_code"] == 0 else "FAIL"
    lines.append(
        f"- `{mark}` `{command['group']}/{command['name']}` "
        f"({command['duration_seconds']}s) -> `{command['log_path']}`"
    )

if metrics:
    lines.extend(["", "## Key Extracts", ""])
    for key, snippets in metrics.items():
        lines.append(f"### `{key}`")
        for snippet in snippets:
            lines.append(f"- {snippet}")
        lines.append("")

if failed:
    lines.extend(["## Failures", ""])
    for command in failed:
        lines.append(f"- `{command['group']}/{command['name']}` exited `{command['exit_code']}`; see `{command['log_path']}`.")
else:
    lines.extend(["## Failures", "", "- None."])

(out_root / "signal_summary.md").write_text("\n".join(lines).rstrip() + "\n")
PY

echo
echo "Signal spine complete: $OUT_ROOT"
echo "  manifest: $OUT_ROOT/run_manifest.json"
echo "  summary:  $OUT_ROOT/signal_summary.md"

if awk -F '\t' 'NR > 1 && $3 != 0 { found = 1 } END { exit found ? 0 : 1 }' "$COMMANDS_TSV"; then
  echo "Signal spine completed with one or more failed commands." >&2
  exit 1
fi
