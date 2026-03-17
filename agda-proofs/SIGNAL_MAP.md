# Agda Signal Map

This note highlights where the strongest formal signal currently lives in the
`agda-proofs/` tree, what remains structurally promising but assumption-heavy,
and which repair targets are most likely to improve the proof/test surface next.

## Tier 1: Strongest Verified Spine

These modules currently type-check cleanly and carry the highest signal-to-noise
ratio.

In this note, "clean" means clean-local unless stated otherwise. There are
currently no known clean-local boundary cases in the maintained clean list.
The active certification lane is now also locally clean:
`Theorems/Abstract/SymmetryFiniteReflect.agda`,
`Examples/CertifiedResonanceParam.agda`, and
`Examples/CertifiedResonanceParamDyn.agda` all compile without local postulates.
The standard even-base path now derives its observed fixed-point classifier
constructively, while noncanonical paths consume that classifier explicitly.
Defer to [`STATUS.md`](STATUS.md) for the current boundary notes.

### Residue and Spec Engine

- `Core/ResidueFold.agda`
- `Core/CRTVector.agda`
- `Core/ResidueClassesComplete.agda`
- `Specs/SpacingResidueModel.agda`
- `Specs/PalindromeEvenDivides.agda`
- `Specs/Tests.agda`
- `Tests/DevProofs.agda`
- `Tests/Spec/ResidueCollapseSpec.agda`
- `Examples/Base10ResidueFilter.agda`
- `Tests/Spec/Base10ResidueFilterSpec.agda`
- `Tests/Spec/ResidueClassesRingSpec.agda`
- `Tests/Spec/ResidueClassesUnitsSpec.agda`
- `Theorems/ElbowEvents.agda`

Why this matters:

- this is the cleanest formal bridge to the repo's residue-centered
  computational story
- the modules are executable enough to serve as regression checks, not just
  aspirational theorem shells
- the CRT/LCM path directly supports tooling already used elsewhere in the repo
- `Core/ResidueClassesComplete.agda` now gives the residue story a safe
  constructive commutative-ring foundation rather than a postulated shell
- the base-10 filter pair now gives a clean theorem-on-concrete-examples bridge
  from the residue story to a classical prime-filter fact

### Abstract Certification Spine

- `Theorems/Abstract/SymmetryImpliesRepulsion.agda`
- `Theorems/Abstract/SymmetryFromList.agda`
- `Theorems/Abstract/ConstrainedOrbitals.agda`
- `Theorems/Abstract/SymmetryFiniteReflect.agda`
- `Theorems/Abstract/BucketsAutoMatch.agda`
- `Theorems/Abstract/WindowCertificate.agda`
- `Examples/CertifiedResonanceComplete.agda`
- `Examples/CertifiedResonanceParam.agda`
- `Examples/CertifiedResonanceParamDyn.agda`

Why this matters:

- this is the best current case that the repo has a real certification pipeline,
  not just disconnected notes
- `SymmetryFiniteReflect.agda` is now locally clean, so the modular reflection
  layer is no longer the weak link in the active certification path
- `CertifiedResonanceComplete.agda` is especially important because it anchors a
  concrete base-6 example with no postulates
- `WindowCertificate.agda` now gives that spine a clean-local dual-certificate
  builder instead of a builder-plus-example shell blend
- `CertifiedResonanceParamDyn.agda` now keeps its runtime example shell outside
  the active wrapper module, so the dual wrapper boundary is cleaner too

### Empirical-Data Ingestion and Derived Facts

- `Theorems/ElbowsFromCSV.agda`
- `Theorems/GlobalElbowFacts.agda`
- `Theorems/GapDivisibility.agda`
- `Theorems/CoordinateEigenspace.agda`
- `Theorems/ConstrainedOrbitals.agda`
- `Theorems/AffineTransformComputation.agda`
- `Theorems/MirrorObstruction.agda`
- `Theorems/RationalStatistics.agda`
- `Theorems/SpectralRigidity.agda`
- `Complete/OrthogonalityFloat.agda`
- `Advanced/Statistics.agda`

Why this matters:

- these files show a second kind of signal: machine-checked handling of derived
  facts and statistics around empirical structures
- `ElbowsFromCSV.agda` is notable because it turns pipeline output into typed
  theorem data rather than leaving it as a loose CSV artifact
- the elbow event source, CSV ingestion, and global elbow facts now form a
  clean end-to-end slice instead of a clean-local layer over a postulated event
  source
- `GapDivisibility.agda` and `CoordinateEigenspace.agda` now keep the gap and
  eigenspace sides of the "hexagonal" story in the clean-local tier instead of
  the failing bucket
- `Complete/OrthogonalityFloat.agda` now gives the orthogonality lane a live
  executable backend again instead of a broken parser-era float module

## Tier 2: Promising but Assumption-Heavy Extensions

These modules compile, but they depend on postulates and should be described as
structured scaffolds rather than finished proofs.

- `Core/HonoraryZero.agda`
- `Core/ArithmeticHelpers.agda`
- `Core/ConstellationPowerLaw.agda`
- `Core/Discriminant.agda`
- `Core/GoldbachPhaseLocks.agda`
- `Core/GoldenRatio.agda`
- `Core/LagrangePoints.agda`
- `Core/OrthogonalityFramework.agda`
- `Core/PhaseLocks.agda`
- `Core/Radical.agda`
- `Core/ResidueCollapse.agda`
- `Core/ResidueClasses.agda`
- `Core/Spectral.agda`
- `Core/TwoPBase.agda`
- `Advanced/Orthogonality.agda`
- `Integration/ComputationalBridge.agda`
- `Integration/PrimeDensityFramework.agda`
- `LagrangePoints/Examples.agda`
- `LagrangePoints/ResidueField.agda`
- `LagrangePoints/TemplateExtension.agda`
- `Verification/ExclusiveConfigurations.agda`
- `Verification/GCDParadoxComputation.agda`
- `Verification/ResonanceComputation.agda`
- `Examples/UniMathIntegration.agda`
- `Theorems/Abstract/SymmetryFiniteReflect.agda`
- `Theorems/BifurcationRoster.agda`
- `Theorems/AffineTransform.agda`
- `Theorems/ConstellationCriticalLine.agda`
- `Theorems/CoprimalityRequirement.agda`
- `Theorems/CoordinateConstellationScaling.agda`
- `Theorems/HardyLittlewoodSingularSeries.agda`
- `Theorems/HexagonalUnification.agda`
- `Theorems/PhaseLockSymmetry.agda`
- `Theorems/RadicalDivisibilityFilter.agda`
- `Theorems/ResidueSymmetry.agda`
- `Theorems/SymmetryImpliesRepulsion.agda`
- `Theorems/UniversalSymmetryRepulsion.agda`
- `LagrangePoints/ZeroPaddedPrimes/Alphabet036.agda`
- `LagrangePoints/ZeroPaddedPrimes/Examples036.agda`
- `LagrangePoints/ZeroPaddedPrimes/Asymmetry.agda`

Steelman reading:

- these files still matter because they preserve the shape of the next proof
  steps
- the residue/radical/collapse trio is no longer uniformly weak: the ring side
  is now constructive and safe, so the remaining residue bottleneck is the
  postulated radical/collapse side
- within the collapse layer specifically, the active signal is now cleaner:
  canonical examples and the weak comparison bridge are constructive, and the
  remaining open theorem is the general coverage/regularity bridge
- the certification stack above the complete base-6 example is real, even if
  some helper bridges remain postulated
- the zero-padded connector subtree is best treated as a narrow formal shell for
  one empirical pair, not as a general connector theory
- `Theorems/CoordinateConstellationScaling.agda` now compiles as a smaller
  empirical scaffold, which is a healthier place for its signal than the old
  parse-era pseudo-theorem shell
- `Theorems/ConstellationCriticalLine.agda` and
  `Theorems/HardyLittlewoodSingularSeries.agda` now also compile as explicit
  heuristic shells, which makes their remaining debt easier to classify
- `Theorems/HexagonalUnification.agda` now compiles as a narrower synthesis
  shell built on live base witnesses rather than on unfinished perfect-number
  machinery
- `Theorems/PhaseLockSymmetry.agda` and `Theorems/ResidueSymmetry.agda` now
  compile as explicit instantiation shells instead of failing at the import
  boundary
- `Theorems/CoprimalityRequirement.agda` and
  `Theorems/RadicalDivisibilityFilter.agda` now compile as explicit filter shells
  instead of failing at stale type-signature syntax
- `Theorems/AffineTransform.agda` now compiles as a narrower theorem shell, and
  `Theorems/AffineTransformComputation.agda` now keeps the affine computation
  side live as a clean local regression/data surface
- `Core/Discriminant.agda` now compiles as a narrower analytical shell with
  live recorded observations instead of failing at the parser boundary
- `Core/GoldbachPhaseLocks.agda` now compiles as a narrower bridge shell with
  live base-22/base-26 witnesses instead of failing outright
- `Core/GoldenRatio.agda` now compiles as a narrower analytical shell with
  live crossover and Fibonacci observations instead of broken real-analysis
  scaffolding
- `Core/LagrangePoints.agda` now compiles as a narrower canonical-pair shell
  with live counting helpers instead of hole-driven insertion code
- `Core/OrthogonalityFramework.agda` now compiles as a narrower shell with live
  raw/HL-normalized correlation observations and an explicit open decorrelation
  bridge
- `Core/Spectral.agda`, `Core/TwoPBase.agda`, and `Core/PhaseLocks.agda` now
  compile again as a live shell stack for QR/NQR vocabulary, `2p` bases, and
  symmetric prime-pair structure
- `Advanced/Orthogonality.agda` now compiles as a narrower experiment shell
  layered on the recovered core orthogonality file instead of stale float code

## Tier 3: Exploratory or Overextended Zones

These are where the repo carries ideas, but not yet a trustworthy verification
surface.

- selected `Verification/` and `Integration/` modules
- most non-abstract theorem files in `Theorems/`

Constructive reading:

- there is often a real mathematical intuition underneath these files
- the problem is usually proof completeness, API drift, or too many postulated
  concepts arriving at once
- these areas should be narrowed and repaired one dependency layer at a time

## Best Near-Term Repair Targets

### 1. Radical theorem-surface continuation

Scope:

- `Core/Radical.agda`
- `Core/ResidueClasses.agda`
- the theorem-level radical vs totient distinctions used by the residue
  framework and the repo’s public terminology

Why it remains high-signal:

- the radical layer now has constructive `rad(12)`, `rad(30)`, `rad ≠ φ`, and
  several multiplicativity-derived composite examples, so the remaining open
  surface is narrower and easier to reason about
- `Core/Radical.agda` still carries theorem-level postulates that look
  recoverable from the repo’s existing examples and standard number-theory
  facts
- continuing there would strengthen the repo’s standard-term spine without
  forcing a full constructive factorization development yet

### 2. Certification postulate reduction

Why it matters:

- the Agda tree now compiles end-to-end, so the next bottleneck is not parser
  recovery but proof-surface quality
- the certification stack already has strong clean and narrow-postulated layers,
  and its remaining postulates are now concentrated in a small number of
  productive bridge points

## Current Verifier

Use:

```bash
cd agda-proofs
./scripts/verify-clean-spine.sh
```

This checks the current clean spine without pretending the rest of the tree is
stable.

Update after Tracks 13-63:

- `Theorems/ElbowEvents.agda` is now correctly classified as clean-local and is
  part of the maintained clean spine
- the elbow event source, CSV ingestion, and global elbow facts now form a
  clean end-to-end slice rather than a clean-local layer over a postulated
  event source
- `Theorems/TotientDensity.agda` now reuses the shared rational/prime/GCD
  infrastructure constructively and has a much narrower postulated surface
- `Theorems/CoordinateConstellationScaling.agda` now compiles again as a
  current-syntax empirical scaffold instead of failing at the parser level
- `Theorems/ConstellationCriticalLine.agda` now compiles as a current-syntax
  critical-line heuristic shell
- `Theorems/HardyLittlewoodSingularSeries.agda` now compiles as a current-syntax
  singular-series shell tied to the recovered critical-line layer
- `Theorems/GapDivisibility.agda` and `Theorems/CoordinateEigenspace.agda` now
  compile cleanly, giving the hexagonal story live gap and eigenspace witnesses
- `Theorems/HexagonalUnification.agda` now compiles as a narrower synthesis shell
- `Theorems/PhaseLockSymmetry.agda` and `Theorems/ResidueSymmetry.agda` now
  compile as narrower symmetry-instantiation shells
- `Theorems/CoprimalityRequirement.agda` and
  `Theorems/RadicalDivisibilityFilter.agda` now compile as narrower filter shells
- `Theorems/ConstrainedOrbitals.agda` now compiles again as a genuinely
  constructive non-abstract wrapper
- `Theorems/SymmetryImpliesRepulsion.agda` now compiles as a narrower midpoint
  wrapper shell instead of failing outright
- `Theorems/AffineTransform.agda` now compiles as a narrower affine theorem
  shell instead of failing at the module boundary
- `Theorems/AffineTransformComputation.agda` now compiles as a clean
  computation shell with maintained base-6 affine checks
- `Theorems/UniversalSymmetryRepulsion.agda` now compiles as a universal
  symmetry shell with a live perfect-bucket -> honorary-zero theorem core
- `Core/ConstellationPowerLaw.agda` now compiles as a narrower analytical shell
  instead of failing on rational arithmetic namespace drift
- `Core/ArithmeticHelpers.agda` now compiles as a narrower helper shell with
  constructive regrouping lemmas and an explicit example-template surface
- `Core/Discriminant.agda` now compiles as a narrower analytical shell with
  constructive helpers and explicit open perfect-square / Legendre bridges
- `Core/GoldbachPhaseLocks.agda` now compiles as a narrower bridge shell with
  live base-22/base-26 witnesses and an explicit open equivalence layer
- `Core/GoldenRatio.agda` now compiles as a narrower analytical shell with
  live crossover and Fibonacci observations instead of failing at parser-era
  real-analysis scaffolding
- `Core/LagrangePoints.agda` now compiles as a narrower canonical-pair shell
  with live point-count and digit-count helpers
- `Core/OrthogonalityFramework.agda` now compiles as a narrower orthogonality
  shell with live correlation-status observations
- `Core/Spectral.agda` now compiles as a narrower spectral vocabulary shell
- `Core/TwoPBase.agda` now compiles as a narrower `2p` base shell with live
  residue sets for bases 6, 10, and 14
- `Core/PhaseLocks.agda` now compiles as a narrower midpoint/distance shell for
  symmetric prime pairs in bases `2p`
- `Advanced/Orthogonality.agda` now compiles as a narrower orthogonality
  experiment shell layered on the recovered core orthogonality file
- `Complete/OrthogonalityFloat.agda` now compiles again as a clean executable
  float backend instead of failing on parser drift and stale builtin imports
- `Integration/ComputationalBridge.agda` now compiles as a narrower
  current-syntax integration shell with live CRT/residue exports, phase-lock
  exports, the canonical Lagrange export, and discriminant summaries
- `Integration/PrimeDensityFramework.agda` now compiles as a narrower unified
  framework shell with live residue admissibility, phase-lock context,
  orthogonality status, and optional Lagrange/discriminant slices
- `LagrangePoints/Examples.agda` now compiles as a narrower canonical case-study
  shell with the two reported hits, reflected open positions, and center-void
  question preserved explicitly
- `LagrangePoints/ResidueField.agda` now compiles as a narrower residue-screen
  shell with the canonical small-prime screen and the two reported compatible
  positions preserved explicitly
- `LagrangePoints/TemplateExtension.agda` now compiles as a narrower
  asymmetric-template wrapper that keeps the abstract honorary-zero bridge live
- `Test/TestRecord.agda` and `Test/TestRecordSimple.agda` now compile as clean
  parser regressions
- `Tests/InvariantTests.agda` now compiles as a smaller executable dynamic
  regression shell, with one direct helper-path `PointwiseSafe` witness and
  one matching helper-agnostic negative `InZone` counterexample
- `Verification/ExclusiveConfigurations.agda`,
  `Verification/GCDParadoxComputation.agda`, and
  `Verification/ResonanceComputation.agda` now compile as narrower
  reported-data verification shells
- `Examples/UniMathIntegration.agda` now compiles again as a namespaced
  migration note rather than failing at the module boundary
- `Examples/CertifiedResonance.agda` now compiles as a narrower generated
  wrapper over the live base-6 certificate rather than failing on parser drift
- `Examples/CertifiedResonanceParam.agda` now compiles clean-locally as a
  one-shot certification wrapper over clean `BucketsAutoMatch.agda`, with the
  canonical even-base observed fixed-point classifier now derived
  constructively and the noncanonical imported boundary narrowed to an
  explicit `ObservedFixedPointClassifier` contract in
  `SymmetryFiniteReflect.agda`
- `Theorems/Abstract/WindowCertificate.agda` is now clean-local: the builder is
  constructive, the static boundary is explicit as support-count plus
  non-fixed-residue evidence, the dynamic boundary is the smaller
  `PointwiseSafe` contract with `StableOrbital` derived internally, and the old
  hypothetical Base-14 shell has been extracted into
  `Examples/WINDOW_CERTIFICATE_BASE14_SKETCH.md`
- `Examples/CertifiedResonanceParamDyn.agda` is now clean-locally as a dual
  wrapper over clean `BucketsAutoMatch.agda`, with an explicit
  `ObservedFixedPointClassifier` as its noncanonical imported boundary, a
  constructive canonical-even helper path, the narrower `PointwiseSafe`
  dynamic contract, and the old Base-6 runtime witness shell moved into
  `Examples/CERTIFIED_RESONANCE_PARAM_DYN_BASE6_SKETCH.md`; the extracted
  notes now show the maintained `pointwiseSafeCons` / `pointwiseSafeNil`
  consumption path instead of a raw `StableOrbital` witness
- `Theorems/Abstract/SymmetryFiniteReflect.agda` now splits the concrete
  reflection burden into explicit contracts: `HalfTurnMidpoint mid` remains
  available when needed, the canonical even-base observed fixed-point
  classifier is constructive, `reflect-involutive` and `reflect mid mid` are
  constructive, and generic callers now consume an explicit
  `ObservedFixedPointClassifier` contract instead of a hidden internal theorem
- `Theorems/Abstract/BucketsAutoMatch.agda` now computes
  `indices-with-residue`, support soundness/completeness, support disjointness,
  support-list uniqueness, `auto-mate-support-lengths`,
  `auto-mate-equivariant`, `auto-mate-no-fixed`,
  `auto-mate-residue-distinct`, and `auto-mate-involutive` constructively, and
  keeps `SupportCountsAgree` as an explicit API contract rather than a local
  theorem postulate
- `Examples/CertifiedResonanceParam.agda` and
  `Examples/CertifiedResonanceParamDyn.agda` now discharge
  `SupportCountsAgree` constructively for their concrete `countResid` path;
  the dynamic lane now has maintained `PointwiseSafe` smart constructors, so
  external generators no longer need to assemble the internal `All` witness by
  hand
  that lane no longer depends on any imported auto-pairing theorem

- `Tests/Spec/ResidueClassesRingSpec.agda` is no longer blocked; it now works as
  an interface test over the repaired residue-ring API
- `Tests/Spec/ResidueClassesUnitsSpec.agda` now gives the repaired residue
  unit/coprime bridge a direct concrete regression surface
- `Examples/Base10ResidueFilter.agda` and
  `Tests/Spec/Base10ResidueFilterSpec.agda` now form a live clean regression
  surface for the base-10 coprimality filter
- `Core.ResidueClassesComplete.agda` now proves `units ↔ coprime`
  constructively for `m > 1`
- `Core.ResidueClassesComplete.agda` now also provides a constructive safe
  commutative-ring witness
- `Tests/Spec/ResidueClassesRingSpec.agda` and
  `Tests/Spec/ResidueClassesUnitsSpec.agda` now pass `--safe` and are part of
  the maintained clean spine
- `Core/Radical.agda`, `Core/ResidueCollapse.agda`, and
  `Core/ResidueClasses.agda` now compile as postulated framework layers rather
  than failing outright
- `Core/Radical.agda` now carries a constructive `rad(12) ≢ φ(12)`
  counterexample rather than postulating the distinction entirely
- `Core/Radical.agda` now also derives `rad(12)`, `rad(30)`, `rad(6)`,
  `rad(10)`, `rad(18)`, `rad(20)`, and `rad(60)` constructively from
  multiplicativity plus the remaining base witnesses
- `Core/ResidueClasses.agda` no longer postulates wheel-class coprimality; that
  bridge is now proved constructively from filtered-list membership
- `Core/ResidueCollapse.agda` now factors its remaining coverage gap into
  threshold coverage plus above-threshold stability; the canonical examples and
  weak filtering bridge are constructive
- the active Agda docs now state that there are currently no maintained
  clean-local boundary cases
- the collapse story in `Core/ResidueClasses.agda` now matches the tested
  frequency-regularity interpretation instead of the older “missing classes”
  phrasing
- the open work in this area is narrower and more meaningful:
  the still-postulated radical proof core plus certification-stack postulate
  reduction, especially `autoPerfectBuckets` and the runtime-generation shells
