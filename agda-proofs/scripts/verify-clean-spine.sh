#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

modules=(
  "Theorems/Abstract/SymmetryImpliesRepulsion.agda"
  "Theorems/Abstract/SymmetryFromList.agda"
  "Theorems/Abstract/ConstrainedOrbitals.agda"
  "Theorems/Abstract/BucketsAutoMatch.agda"
  "Theorems/Abstract/WindowCertificate.agda"
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
  "Theorems/MirrorObstruction.agda"
  "Theorems/RationalStatistics.agda"
  "Theorems/SpectralRigidity.agda"
  "Examples/CertifiedResonanceComplete.agda"
)

echo "Verifying clean Agda spine (${#modules[@]} modules)..."
for module in "${modules[@]}"; do
  echo "  - $module"
  agda --safe "$module" >/dev/null
done

echo "Clean Agda spine verified."
