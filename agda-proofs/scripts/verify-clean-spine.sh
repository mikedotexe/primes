#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

agda_bin=""

if [[ -n "${AGDA_BIN:-}" ]]; then
  if [[ ! -x "$AGDA_BIN" ]]; then
    echo "AGDA_BIN is set but not executable: $AGDA_BIN" >&2
    exit 1
  fi
  agda_bin="$AGDA_BIN"
elif agda_path="$(command -v agda 2>/dev/null)"; then
  agda_bin="$agda_path"
else
  for candidate in /opt/homebrew/bin/agda /usr/local/bin/agda; do
    if [[ -x "$candidate" ]]; then
      agda_bin="$candidate"
      break
    fi
  done
fi

if [[ -z "$agda_bin" ]]; then
  cat >&2 <<'EOF'
Unable to find an Agda executable.

Set AGDA_BIN=/path/to/agda, or install Agda in one of the supported locations:
  - agda on PATH
  - /opt/homebrew/bin/agda
  - /usr/local/bin/agda
EOF
  exit 1
fi

modules=(
  "Theorems/Abstract/SymmetryImpliesRepulsion.agda"
  "Theorems/Abstract/SymmetryFromList.agda"
  "Theorems/Abstract/ConstrainedOrbitals.agda"
  "Theorems/Abstract/BucketsAutoMatch.agda"
  "Theorems/Abstract/WindowCertificate.agda"
  "Theorems/Abstract/FiniteMaskTransfer.agda"
  "Core/Primality.agda"
  "Core/CRTVector.agda"
  "Core/Equiv.agda"
  "Core/ResidueClassesComplete.agda"
  "Core/ResidueFold.agda"
  "Specs/SpacingResidueModel.agda"
  "Specs/PalindromeEvenDivides.agda"
  "Specs/Tests.agda"
  "Advanced/Statistics.agda"
  "Dependencies.agda"
  "Test/SimpleImportTest.agda"
  "Tests/DevProofs.agda"
  "Tests/Spec/ResidueCollapseSpec.agda"
  "Examples/Base10ResidueFilter.agda"
  "Tests/Spec/Base10ResidueFilterSpec.agda"
  "Tests/Spec/ResidueClassesRingSpec.agda"
  "Tests/Spec/ResidueClassesUnitsSpec.agda"
  "Theorems/ElbowEvents.agda"
  "Theorems/ElbowsFromCSV.agda"
  "Theorems/GlobalElbowFacts.agda"
  "Theorems/BoundedKCompactness.agda"
  "Theorems/MirrorObstruction.agda"
  "Theorems/RationalStatistics.agda"
  "Theorems/SpectralRigidity.agda"
  "Examples/CertifiedResonanceComplete.agda"
  "Examples/BoundedKTransferWitnesses.agda"
)

echo "Using Agda binary: $agda_bin"
echo "Verifying clean Agda spine (${#modules[@]} modules)..."
for module in "${modules[@]}"; do
  echo "  - $module"
  "$agda_bin" --safe "$module" >/dev/null
done

echo "Clean Agda spine verified."
