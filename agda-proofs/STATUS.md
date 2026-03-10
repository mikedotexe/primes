# Agda Verification Status

**Last Updated**: 2026-03-09 (updated after Param/ParamDyn example postulate reduction)
**Agda Version**: 2.8.0 (installed via Homebrew)
**Standard Library**: v2.3
**Total modules**: 80

## Summary

| Category | Count | Percentage |
|----------|-------|------------|
| Pass (clean, no postulates) | 20 | 25.0% |
| Pass (with postulates) | 12 | 15.0% |
| Fail | 48 | 60.0% |
| **Total** | **80** | |

**Methodology**: Each module tested individually with `agda <file>` after clearing
the `_build/` cache directory. Exit code 0 = pass, exit code 42 = fail. Postulate
count from `grep -cw 'postulate' <file>`.

## Modules That Pass (Clean -- No Postulates)

These 20 modules type-check and contain no postulates. Their proofs are
machine-verified.

| Module | Description |
|--------|-------------|
| Theorems/Abstract/SymmetryImpliesRepulsion | Core abstract symmetry -> repulsion theorem |
| Theorems/Abstract/SymmetryFromList | Data ingestion: residue buckets -> PerfectBuckets -> HonoraryZero |
| Theorems/Abstract/ConstrainedOrbitals | Dynamic invariant constraints |
| Core/Primality | Primality definitions and basic properties |
| Core/CRTVector | Chinese Remainder Theorem vector operations |
| Core/Equiv | Equivalence relations |
| Core/ResidueFold | Residue folding operations |
| Specs/SpacingResidueModel | Executable spec: DP counts mod m with LCM lift |
| Specs/PalindromeEvenDivides | Executable spec: even-palindrome divisibility |
| Specs/Tests | Executable specification regression tests |
| Advanced/Statistics | Statistical primitives |
| Dependencies | Import validation |
| Test/SimpleImportTest | Import validation |
| Tests/DevProofs | Development proofs |
| Theorems/ElbowsFromCSV | Elbow event data ingestion |
| Theorems/GlobalElbowFacts | Global elbow analysis |
| Theorems/MirrorObstruction | Mirror obstruction theorem |
| Theorems/RationalStatistics | Rational number statistics |
| Theorems/SpectralRigidity | Spectral rigidity theorem |
| Examples/CertifiedResonanceComplete | Base 6 certification (all proofs, no postulates) |

## Modules That Pass (With Postulates)

These 12 modules type-check but use postulates (assumed axioms). Their conclusions
depend on unproven assumptions and are NOT fully machine-checked proofs.

| Module | Postulate Count | Notes |
|--------|----------------|-------|
| Core/HonoraryZero | 1 | Minor assumption |
| Theorems/Abstract/SymmetryFiniteReflect | 1 | Modular reflection postulate |
| Theorems/Abstract/BucketsAutoMatch | 5 | Auto-mate helper properties postulated |
| Theorems/Abstract/WindowCertificate | 5 | Dual certification with postulated helpers |
| Examples/CertifiedResonanceParam | 1 | Framework autoPerfectBuckets only; example proofs now constructive |
| Examples/CertifiedResonanceParamDyn | 2 | Framework autoPerfectBuckets + runtime proof-stable |
| LagrangePoints/ZeroPaddedPrimes/Alphabet036 | 8 | Alphabet and prime definitions postulated |
| LagrangePoints/ZeroPaddedPrimes/Asymmetry | 10 | Asymmetry properties postulated |
| LagrangePoints/ZeroPaddedPrimes/Examples036 | 8 | Example primes postulated |
| Theorems/BifurcationRoster | 1 | Minor assumption |
| Theorems/ElbowEvents | 4 | Event properties postulated |
| Theorems/TotientDensity | 39 | Almost entirely postulated (rationals, GCD, totient, Basel theorem) |

## Certification Stack Status

The core certification pipeline (from SymmetryImpliesRepulsion down to concrete examples)
is now **fully operational**:

| Module | Status | Notes |
|--------|--------|-------|
| SymmetryImpliesRepulsion | clean | Core theorem, no postulates |
| SymmetryFromList | clean | Fixed: added residue-distinct to PerfectBuckets record |
| ConstrainedOrbitals | clean | Dynamic invariant |
| SymmetryFiniteReflect | postulates (1) | Modular instantiation |
| BucketsAutoMatch | postulates (5) | Fixed: wired residue-distinct into PerfectBuckets |
| WindowCertificate | postulates (5) | Dual certification |
| CertifiedResonanceComplete | **clean** | Base 6 example (6 postulates eliminated March 2026) |
| CertifiedResonanceParam | postulates (1) | Example proofs now constructive; only framework autoPerfectBuckets remains |
| CertifiedResonanceParamDyn | postulates (2) | Example static proofs constructive; proof-stable + autoPerfectBuckets remain |

**Repair applied** (2026-03-09): SymmetryFromList.agda had an unsolved meta at line 84
because the `PerfectBuckets` record was missing its `residue-distinct` field (originally
moved to a postulate as a "parser bug workaround"). The fix added `residue-distinct`
back as a proper record field, eliminating the postulate. BucketsAutoMatch.agda had the
same issue in `perfectFromBalanced` and was fixed similarly. These two fixes restored
7 modules from failing to passing.

## Modules That Fail (48 modules)

Common failure patterns:
1. **stdlib 2.3 incompatibilities**: Several modules use imports or constructors
   that changed between stdlib versions.
2. **Missing dependencies**: Some modules import other failing modules.

### Full Failure List

Core/ (10 failing): ArithmeticHelpers, ConstellationPowerLaw, Discriminant,
GoldbachPhaseLocks, GoldenRatio, LagrangePoints, OrthogonalityFramework,
PhaseLocks, Radical, ResidueClasses, ResidueClassesComplete, ResidueCollapse,
Spectral, TwoPBase

Examples/ (2 failing): Base10ResidueFilter, CertifiedResonance, UniMathIntegration

Theorems/ (12 failing): AffineTransform, AffineTransformComputation,
ConstellationCriticalLine, ConstrainedOrbitals (non-Abstract),
CoordinateConstellationScaling, CoordinateEigenspace, CoprimalityRequirement,
GapDivisibility, HardyLittlewoodSingularSeries, HexagonalUnification,
PhaseLockSymmetry, RadicalDivisibilityFilter, ResidueSymmetry,
SymmetryImpliesRepulsion (non-Abstract), UniversalSymmetryRepulsion

Other (7 failing): Advanced/Orthogonality, Complete/OrthogonalityFloat,
Integration/ComputationalBridge, Integration/PrimeDensityFramework,
LagrangePoints/Examples, LagrangePoints/ResidueField, LagrangePoints/TemplateExtension,
Test/TestRecord, Test/TestRecordSimple, Tests/InvariantTests,
Tests/Spec/Base10ResidueFilterSpec, Tests/Spec/ResidueClassesRingSpec,
Tests/Spec/ResidueCollapseSpec, Verification/ExclusiveConfigurations,
Verification/GCDParadoxComputation, Verification/ResonanceComputation

## Repair History

### March 2026: CertifiedResonanceParam/ParamDyn example postulate reduction

**Problem**: Both `CertifiedResonanceParam.agda` and `CertifiedResonanceParamDyn.agda`
had postulated witnesses (proof-midVoid, proof-balanced) in their Example modules for
the same Base 6 data that CertifiedResonanceComplete proves constructively.

**Fix**: Replaced the postulated example witnesses with direct PerfectBuckets
construction using explicit fzero/fsuc case analysis (same technique as Track 20).
The examples now bypass the framework-level `autoPerfectBuckets` postulate entirely.

**Result**:
- CertifiedResonanceParam: Example module has 0 postulates (was 2).
  File total: 1 postulate (framework autoPerfectBuckets only).
- CertifiedResonanceParamDyn: Example module has 1 postulate (proof-stable, runtime
  dependent). File total: 2 postulates. Was 4 (3 example + 1 framework).

### March 2026: CertifiedResonanceComplete postulate elimination

**Problem**: `CertifiedResonanceComplete.agda` used 6 postulates for trivially
provable properties (involutive, no-fixed, equivariant, residue-distinct). The
postulates existed because the `#_` operator for Fin literals did not compute
in Agda's pattern matching.

**Fix**: Replaced all `#_` abbreviations (`# 0`, `# 1`, etc.) with explicit
`fzero`/`fsuc` constructor patterns. All 6 postulates became constructive proofs
via `refl` (equality proofs) or absurd patterns (inequality proofs).

**Result**: Module moves from "passes with postulates (6)" to "passes clean (0)."
Counts: 19 -> 20 clean, 13 -> 12 postulated. The Base 6 flagship certification is
now fully machine-checked with no assumptions.

### March 2026: SymmetryFromList + BucketsAutoMatch fix

**Problem**: `SymmetryFromList.agda` had a `PerfectBuckets` record missing the
`residue-distinct` field. The field was postulated separately but never supplied
to the `Pairing` record constructor in `pairingFromPerfect`. Similarly,
`BucketsAutoMatch.agda` omitted the field in `perfectFromBalanced`.

**Fix**: Added `residue-distinct` as a proper field of `PerfectBuckets` (eliminating
the postulate in SymmetryFromList). Wired the postulated `auto-mate-residue-distinct`
into the record literal in BucketsAutoMatch.

**Result**: 7 modules restored (18 -> 19 clean, 7 -> 13 with postulates, 55 -> 48 failing).

### March 2026: Initial audit

Full audit found 18 clean + 7 postulated + 55 failing out of 80 modules. Previous
STATUS.md claimed 11 working; actual was 25 passing (4 of the 11 confirmed, 14
previously untested found to pass).

## CI Status

The GitHub Actions workflow (`.github/workflows/agda-verification.yml`) attempts
to install Agda via `apt-get` on ubuntu-latest. This is unlikely to work because:
1. Ubuntu's package repositories typically don't have Agda 2.8.0
2. The stdlib setup assumes a specific installation path
3. The workflow has likely never passed in CI

Recommended fix: Use a Docker image with Agda pre-installed, or install via
GHCup/stack.

## Compilation Instructions

```bash
cd agda-proofs

# Test all 20 clean-passing modules
for f in \
  Theorems/Abstract/SymmetryImpliesRepulsion.agda \
  Theorems/Abstract/SymmetryFromList.agda \
  Theorems/Abstract/ConstrainedOrbitals.agda \
  Core/Primality.agda \
  Specs/SpacingResidueModel.agda \
  Specs/PalindromeEvenDivides.agda \
  Specs/Tests.agda \
  Advanced/Statistics.agda \
  Core/CRTVector.agda \
  Core/Equiv.agda \
  Core/ResidueFold.agda \
  Dependencies.agda \
  Test/SimpleImportTest.agda \
  Tests/DevProofs.agda \
  Theorems/ElbowsFromCSV.agda \
  Theorems/GlobalElbowFacts.agda \
  Theorems/MirrorObstruction.agda \
  Theorems/RationalStatistics.agda \
  Theorems/SpectralRigidity.agda \
  Examples/CertifiedResonanceComplete.agda; do
  echo -n "$(basename $f .agda): "
  agda "$f" > /dev/null 2>&1 && echo "PASS" || echo "FAIL"
done
```
