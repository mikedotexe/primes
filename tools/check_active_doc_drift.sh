#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

build_examples=0
if [[ "${1:-}" == "--build-examples" ]]; then
  build_examples=1
fi

docs=(
  "README.md"
  "CLAUDE.md"
  "EVIDENCE.md"
  "STATUS.md"
  "ROADMAP.md"
  "examples/README.md"
  "agda/README.md"
  "agda-proofs/README.md"
  "agda-proofs/STATUS.md"
  "agda-proofs/RESIDUE_FOLD_README.md"
  "agda-proofs/CRT_IMPLEMENTATION_SUMMARY.md"
  "agda-proofs/AGDA_RESOURCES.md"
  "agda-proofs/FIX_IMPORTS.md"
  "agda-proofs/Tests/TESTING_STRATEGY.md"
  "agda-proofs/Theorems/README.md"
  "agda-proofs/LagrangePoints/README.md"
  "agda-proofs/LagrangePoints/ZeroPaddedPrimes/README.md"
  "wasm-demo/README.md"
  "pkg/WEB_TUI_README.md"
  "tools/README.md"
  "tools/orthogonality/README.md"
)

echo "Checking active-doc relative links..."
for doc in "${docs[@]}"; do
  while IFS= read -r link; do
    case "$link" in
      http*|mailto:*)
        continue
        ;;
    esac

    target_dir="$(dirname "$doc")"
    if [[ ! -e "$target_dir/$link" ]]; then
      echo "BROKEN LINK in $doc -> $link" >&2
      exit 1
    fi
  done < <(
    perl -ne 'while(/\[[^\]]+\]\(([^)#]+)(?:#[^)]+)?\)/g){print "$1\n"}' "$doc"
  )
done

echo "Checking top-level example count against STATUS.md..."
actual_examples="$(find examples -maxdepth 1 -name '*.rs' | wc -l | tr -d ' ')"
expected_examples="$(
  sed -nE 's/^\| Top-level examples \| ([0-9]+) compile \|.*$/\1/p' STATUS.md
)"

if [[ -z "$expected_examples" ]]; then
  echo "Could not parse top-level example count from STATUS.md" >&2
  exit 1
fi

if [[ "$actual_examples" != "$expected_examples" ]]; then
  echo "Example count drift: STATUS.md says $expected_examples but repo has $actual_examples top-level examples" >&2
  exit 1
fi

echo "Checking Agda status count alignment..."
root_agda_counts="$(
  sed -nE 's/^\| Agda status \| ([0-9]+) clean-local, ([0-9]+) with local postulates, ([0-9]+) failing \|.*$/\1 \2 \3/p' STATUS.md
)"
agda_clean="$(
  sed -nE 's/^\| Pass \(clean-local, no local postulates\) \| ([0-9]+) \|.*$/\1/p' agda-proofs/STATUS.md
)"
agda_postulated="$(
  sed -nE 's/^\| Pass \(with local postulates\) \| ([0-9]+) \|.*$/\1/p' agda-proofs/STATUS.md
)"
agda_failing="$(
  sed -nE 's/^\| Fail \| ([0-9]+) \|.*$/\1/p' agda-proofs/STATUS.md
)"

if [[ -z "$root_agda_counts" || -z "$agda_clean" || -z "$agda_postulated" || -z "$agda_failing" ]]; then
  echo "Could not parse Agda status counts from STATUS.md or agda-proofs/STATUS.md" >&2
  exit 1
fi

if [[ "$root_agda_counts" != "$agda_clean $agda_postulated $agda_failing" ]]; then
  echo "Agda count drift: STATUS.md says '$root_agda_counts' but agda-proofs/STATUS.md says '$agda_clean $agda_postulated $agda_failing'" >&2
  exit 1
fi

echo "Checking that active high-level docs do not hand-sync Agda counts..."
if rg -n "Agda.*[0-9]+.*(clean|postulate|fail)|[0-9]+ clean modules|[0-9]+ modules type-check|[0-9]+ modules fail" \
  README.md CLAUDE.md agda-proofs/README.md >/dev/null; then
  echo "Agda count wording drift: move high-level Agda counts into STATUS.md surfaces only" >&2
  exit 1
fi

echo "Checking Agda clean-local boundary notes..."
if ! grep -Fq 'Current maintained clean-local boundary cases: none known.' agda-proofs/README.md; then
  echo "Agda boundary-note drift: agda-proofs/README.md must state the current clean-local boundary-case truth" >&2
  exit 1
fi

if ! grep -Fq 'Current maintained clean-local boundary cases: none known.' agda-proofs/STATUS.md; then
  echo "Agda boundary-note drift: agda-proofs/STATUS.md must state the current clean-local boundary-case truth" >&2
  exit 1
fi

if [[ "$build_examples" -eq 1 ]]; then
  echo "Building top-level examples..."
  for source in examples/*.rs; do
    name="$(basename "$source" .rs)"
    cargo build --example "$name" >/dev/null
  done
fi

echo "Active-doc drift check passed."
