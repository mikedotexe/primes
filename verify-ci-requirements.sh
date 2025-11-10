#!/bin/bash
echo "=== CI Requirements Verification ==="
echo ""

echo "📚 Checking certification documentation (7 files)..."
docs=(
  "CERTIFICATION_COMPLETE.md"
  "COMPLETE_CERTIFICATION_ARCHITECTURE.md"
  "COMPLETE_VERIFICATION_FRAMEWORK.md"
  "ABSTRACT_FRAMEWORK_INTEGRATION.md"
  "STATIC_TO_DYNAMIC_INVARIANTS.md"
  "QUICK_START_VERIFICATION.md"
  "DOCUMENTATION_MAP.md"
)

missing_docs=0
for doc in "${docs[@]}"; do
  if [ ! -f "$doc" ]; then
    echo "  ❌ Missing: $doc"
    missing_docs=$((missing_docs + 1))
  else
    echo "  ✓ Found: $doc"
  fi
done

echo ""
echo "🔬 Checking Agda modules (10 files)..."
modules=(
  "agda-proofs/Theorems/Abstract/SymmetryImpliesRepulsion.agda"
  "agda-proofs/Theorems/Abstract/SymmetryFromList.agda"
  "agda-proofs/Theorems/Abstract/ConstrainedOrbitals.agda"
  "agda-proofs/Theorems/Abstract/SymmetryFiniteReflect.agda"
  "agda-proofs/Theorems/Abstract/BucketsAutoMatch.agda"
  "agda-proofs/Theorems/Abstract/WindowCertificate.agda"
  "agda-proofs/Examples/CertifiedResonance.agda"
  "agda-proofs/Examples/CertifiedResonanceComplete.agda"
  "agda-proofs/Examples/CertifiedResonanceParam.agda"
  "agda-proofs/Examples/CertifiedResonanceParamDyn.agda"
)

missing_modules=0
for module in "${modules[@]}"; do
  if [ ! -f "$module" ]; then
    echo "  ❌ Missing: $module"
    missing_modules=$((missing_modules + 1))
  else
    echo "  ✓ Found: $module"
  fi
done

echo ""
echo "=== Summary ==="
if [ $missing_docs -eq 0 ] && [ $missing_modules -eq 0 ]; then
  echo "✅ All CI requirements satisfied!"
  echo "  - 7/7 documentation files present"
  echo "  - 10/10 Agda modules present"
  exit 0
else
  echo "❌ CI requirements NOT met:"
  echo "  - Missing docs: $missing_docs"
  echo "  - Missing modules: $missing_modules"
  exit 1
fi
