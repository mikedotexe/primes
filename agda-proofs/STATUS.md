# Agda Verification Status

**Last Updated**: 2026-04-14 (updated after affine hinge classifier mirror shell)
**Agda Version**: 2.8.0 (installed via Homebrew)
**Standard Library**: v2.3
**Total modules**: 87

## Summary

| Category | Count | Percentage |
|----------|-------|------------|
| Pass (clean-local, no local postulates) | 46 | 52.9% |
| Pass (with local postulates) | 41 | 47.1% |
| Fail | 0 | 0.0% |
| **Total** | **86** | |

**Methodology**: Each module tested individually with `agda <file>` after clearing
the `_build/` cache directory. Exit code 0 = pass, exit code 42 = fail. Postulate
count is file-local (`grep -cw 'postulate' <file>`), so "clean-local" means only
that the file itself has no local postulates. Clean-local modules that sit atop
postulated foundations are called out explicitly in the notes.

## Modules That Pass (Clean-Local -- No Local Postulates)

These 46 modules type-check and contain no local postulates. Most are fully
machine-checked within the maintained spine; any clean-local module that still
imports a postulated foundation is marked explicitly in the description.

| Module | Description |
|--------|-------------|
| Theorems/Abstract/SymmetryImpliesRepulsion | Core abstract symmetry -> repulsion theorem |
| Theorems/Abstract/SymmetryFromList | Data ingestion: residue buckets -> PerfectBuckets -> HonoraryZero |
| Theorems/Abstract/ConstrainedOrbitals | Dynamic invariant constraints with maintained `PointwiseSafe` smart constructors |
| Theorems/Abstract/SymmetryFiniteReflect | Clean modular reflection layer: `reflect-involutive`, `reflect mid mid`, and the canonical even-base observed fixed-point classifier are constructive; generic callers now consume an explicit `ObservedFixedPointClassifier` contract instead of a hidden half-turn shell |
| Theorems/Abstract/BucketsAutoMatch | Clean abstract certification bridge: support collection/transport, residue-distinctness, and involutive matching are now constructive; `SupportCountsAgree` is carried as an explicit API contract rather than a local theorem postulate |
| Theorems/Abstract/WindowCertificate | Clean dual-certificate builder over explicit static contracts plus the narrower `PointwiseSafe` dynamic contract; `StableOrbital` is now derived internally, and the hypothetical Base-14 sketch remains extracted |
| Theorems/Abstract/FiniteMaskTransfer | Clean five-bucket transfer partition for bounded-`k` witnesses: shared admissible overlap, admissible totals, same-mask totals, zero-union totals, and bucketwise good-count decomposition are now exact list-level identities |
| Core/Primality | Primality definitions and basic properties |
| Core/CRTVector | Chinese Remainder Theorem vector operations |
| Core/Equiv | Equivalence relations |
| Core/ResidueClassesComplete | Safe residue-ring foundation with constructive commutative ring witness |
| Core/ResidueFold | Residue folding operations |
| Specs/SpacingResidueModel | Executable spec: DP counts mod m with LCM lift |
| Specs/PalindromeEvenDivides | Executable spec: even-palindrome divisibility |
| Specs/Tests | Executable specification regression tests |
| Advanced/Statistics | Statistical primitives |
| Dependencies | Import validation |
| Test/TestRecord | Minimal record parser regression for pairing-style fields |
| Test/TestRecordSimple | Minimal record/open-order regression for pairing-style fields |
| Test/SimpleImportTest | Import validation |
| Tests/DevProofs | Development proofs |
| Tests/InvariantTests | Executable dynamic regression shell for bases 7, 14, and 18, now including one direct helper-path `PointwiseSafe` witness and a matching helper-agnostic negative `InZone` counterexample |
| Tests/Spec/ResidueCollapseSpec | Computational residue-frequency checks |
| Examples/Base10ResidueFilter | Safe base-10 coprimality filter with concrete last-digit examples |
| Tests/Spec/Base10ResidueFilterSpec | Concrete-prime regression surface for the base-10 filter |
| Tests/Spec/ResidueClassesRingSpec | Safe regression surface for the constructive residue-ring laws |
| Tests/Spec/ResidueClassesUnitsSpec | Safe regression surface for the constructive unit/coprime equivalence |
| Theorems/ElbowEvents | Clean elbow event source layer with positive and contrarian examples |
| Theorems/ElbowsFromCSV | Elbow event data ingestion |
| Theorems/GapDivisibility | Clean gap-divisibility ratios and perfect-number connection witnesses for bases 7, 14, and 18 |
| Theorems/GlobalElbowFacts | Global elbow analysis |
| Theorems/CoordinateEigenspace | Clean eigenspace witnesses for bases 7, 14, and 18 with repaired variance/coprimality helpers |
| Theorems/ConstrainedOrbitals | Non-abstract orbital wrapper recovered as a genuinely constructive narrative layer |
| Theorems/AffineTransformComputation | Clean affine computation shell with maintained base-6 residue checks and reported base-10 observations |
| Theorems/AffineLaneComparisonShell | Clean Lean-led mirror shell for local affine lane-comparison vocabulary: shift, gradient, zero-seed class, and the intended relation labels are stable in Agda, while the theorem engine remains Lean-first |
| Theorems/AffinePeriodLockShell | Clean Lean-led mirror shell for local affine period-lock vocabulary: gradient positions, locked/unlocked comparisons, and the relation-label split stay stable in Agda while the exact theorem remains Lean-first |
| Theorems/BoundedKCompactness | Clean bounded-`k` compactness shell: exact padding/diameter arithmetic, `k = (0,0)` minimization, and monotone coordinate shifts are constructive |
| Theorems/MirrorObstruction | Mirror obstruction theorem |
| Theorems/RationalStatistics | Rational number statistics |
| Theorems/SpectralRigidity | Spectral rigidity theorem |
| Complete/OrthogonalityFloat | Clean executable float backend for the prime-pair orthogonality experiment; not part of the maintained clean spine, but no local postulates remain |
| Examples/BoundedKTransferWitnessShell | Clean Agda shell for exact bounded-`k` witness summaries: signed deltas, overlap/boundary predicates, and exact admissible-count projections are stable and generation-friendly |
| Examples/CertifiedResonanceComplete | Base 6 certification (all proofs, no postulates) |
| Examples/Generated/BoundedKTransferWitnessCatalog | Clean auto-generated witness catalog from the Rust bounded-`k` lane for the maintained base-10, base-14, base-22, and base-34 `M = 2` contrasts |
| Examples/BoundedKTransferWitnesses | Clean maintained wrapper that re-exports the exact witness shell and the generated bounded-`k` witness catalog |
| Examples/CertifiedResonanceParam | Clean-local one-shot certification wrapper; the canonical even-base path no longer needs a manual midpoint witness, and the concrete `countResid` path discharges its support-count contract constructively without any imported auto-pairing theorem |
| Examples/CertifiedResonanceParamDyn | Clean-local dual certification wrapper; the canonical even-base path no longer needs a manual midpoint witness, and the old Base-6 runtime witness shell now lives in a separate sketch note |

Current maintained clean-local boundary cases: none known. Outside the
maintained clean spine, no current clean-local boundary cases are known in the
active certification lane either: `SymmetryFiniteReflect.agda`,
`CertifiedResonanceParam.agda`, and `CertifiedResonanceParamDyn.agda` are all
locally clean after the nonzero fixed-point recovery.

## Modules That Pass (With Local Postulates)

These 42 modules type-check but use local postulates (assumed axioms). Their
conclusions depend on unproven assumptions and are NOT fully machine-checked proofs.

| Module | Postulate Count | Notes |
|--------|----------------|-------|
| Core/HonoraryZero | 1 | Minor assumption |
| Core/ArithmeticHelpers | 1 | Current-syntax helper shell: constructive regrouping and factorization lemmas are live, while the division-algorithm/example-template layer remains explicit |
| Core/ConstellationPowerLaw | 1 | Current-syntax analytical shell: empirical twin/cousin/sexy ordering and near-`-1/2` fit are live as recorded data, while the universal power-law bridge remains explicit |
| Core/Discriminant | 5 | Current-syntax discriminant shell: constructive polynomial/discriminant helpers and recorded base-6/base-12 observations are live, while the perfect-square, Legendre, and HL bridge layers remain explicit |
| Core/GoldbachPhaseLocks | 6 | Current-syntax Goldbach bridge shell: concrete base-22/base-26 phase-lock and Goldbach-pair witnesses are live, while the equivalence and spectral bridge remain explicit |
| Core/GoldenRatio | 1 | Current-syntax golden-ratio shell: the base-14 crossover story, Fibonacci ratio observations, and multi-shell scaling data are live, while the irrationality and universality bridge remains explicit |
| Core/LagrangePoints | 1 | Current-syntax Lagrange-point shell: the canonical pair, two reported insertion points, and counting helpers are live, while general existence, clustering, and balance claims remain explicit |
| Core/OrthogonalityFramework | 1 | Current-syntax orthogonality shell: raw vs HL-normalized correlation observations and the dual-score framing are live, while the membrane singular-series decorrelation bridge remains explicit |
| Core/PhaseLocks | 1 | Current-syntax phase-lock shell: midpoint/distance structure and concrete `2p` examples are live, while the restricted-Goldbach and density bridge remains explicit |
| Core/Radical | 10 | Stable radical interface restored; `rad(12)`, `rad(30)`, `rad(12) ≢ φ(12)`, and several multiplicativity-derived examples are constructive, but the main proof core still remains postulated |
| Core/ResidueCollapse | 1 | Executable distinct-residue core, canonical examples, and weak filtering bridge are constructive; the remaining coverage gap is now split into threshold coverage and above-threshold stability |
| Core/ResidueClasses | 5 | Residue framework restored atop repaired radical/collapse/ring interfaces; wheel-class coprimality and the weak collapse comparison are now constructive |
| Core/Spectral | 1 | Current-syntax spectral shell: the `p mod 4` split, `χ(-1)` / `χ(2)` shell values, and primitive-root examples are live, while the full Legendre/Euler bridge remains explicit |
| Core/TwoPBase | 1 | Current-syntax `2p` base shell: concrete residue sets for bases 6, 10, and 14 are live, while the general radical/totient/framework bridge remains explicit |
| Advanced/Orthogonality | 1 | Current-syntax advanced orthogonality shell: prime-pair vs membrane experiment framing is live, while the float backend and large-sample computation bridge remain explicit |
| Examples/CertifiedResonance | 1 | Current-syntax generated wrapper over the live base-6 certificate: the concrete honorary-zero proof is reused from `CertifiedResonanceComplete`, while the runtime export/codegen bridge remains explicit |
| LagrangePoints/ZeroPaddedPrimes/Alphabet036 | 8 | Alphabet and prime definitions postulated |
| LagrangePoints/ZeroPaddedPrimes/Asymmetry | 10 | Asymmetry properties postulated |
| LagrangePoints/ZeroPaddedPrimes/Examples036 | 8 | Example primes postulated |
| LagrangePoints/Examples | 1 | Current-syntax canonical case-study shell: the two reported hits, reflected open positions, center-void question, and membrane connection are live, while the full scan and theory bridge remain explicit |
| LagrangePoints/ResidueField | 1 | Current-syntax residue-screen shell: the canonical small-prime screen and the two reported compatible positions are live, while the CRT/search/primality bridge remains explicit |
| LagrangePoints/TemplateExtension | 1 | Current-syntax asymmetric-template wrapper: buffer reflection, center position, and the abstract honorary-zero bridge are live, while the concrete pairing instantiation remains explicit |
| Examples/UniMathIntegration | 6 | Namespaced migration note now compiles again; it remains a postulated bridge sketch for replacing local foundations with UniMath imports |
| Integration/ComputationalBridge | 1 | Current-syntax integration shell: live CRT/residue exports, phase-lock shells, the canonical Lagrange export, and discriminant summaries are preserved, while the Rust/WASM/tool bridge remains explicit |
| Integration/PrimeDensityFramework | 1 | Current-syntax unified framework shell: residue admissibility, phase-lock context, orthogonality status, and optional Lagrange/discriminant slices are live, while the full predictor bridge remains explicit |
| Theorems/BifurcationRoster | 1 | Minor assumption |
| Theorems/ConstellationCriticalLine | 1 | Current-syntax critical-line heuristic shell: empirical `-1/2` anchor is live, while the zeta / RMT / pair-correlation bridge remains explicit |
| Theorems/CoprimalityRequirement | 1 | Current-syntax coprimality shell: example coprime/non-coprime boundary witnesses are live, while the divisibility and density theorems remain explicit |
| Theorems/CoordinateConstellationScaling | 3 | Current-syntax empirical scaffold: observed scaling ratios and base-14 constraint shell are live, but the theorem layer remains postulated |
| Theorems/AffineTransform | 3 | Current-syntax affine shell: membrane formula, base-6/base-10 examples, and reported residue observations are live, while the general affine residue proof remains explicit |
| Theorems/HardyLittlewoodSingularSeries | 1 | Current-syntax singular-series shell: constellation vocabulary and totient-style Euler-product connection are preserved, with the correlation bridge kept open |
| Theorems/HexagonalUnification | 1 | Current-syntax triple-manifestation shell: base 7/14/18 witnesses are live, while the universal and mechanism-level synthesis remains open |
| Theorems/PhaseLockSymmetry | 1 | Current-syntax phase-lock symmetry shell: concrete left/right pairing is live, while the bridge to the abstract honorary-zero theorem remains open |
| Theorems/RadicalDivisibilityFilter | 1 | Current-syntax radical-filter shell: example radical witnesses are live, while the exact filter theorem layer remains explicit |
| Theorems/ResidueSymmetry | 1 | Current-syntax residue-window symmetry shell: the `2p²` window story is preserved, while the constructive instantiation remains open |
| Theorems/SymmetryImpliesRepulsion | 1 | Non-abstract symmetry wrapper recovered as an honest shell: midpoint residue witnesses are live, while the abstract-instantiation bridge remains explicit |
| Theorems/UniversalSymmetryRepulsion | 1 | Universal symmetry shell: the perfect-bucket -> honorary-zero core theorem is live, while the example symmetry and sequence-testing layer remains explicit |
| Theorems/TotientDensity | 22 | Interface narrowed: rationals, prime, GCD, coprimality, and `φ(1)` are now live imports/definitions; the remaining postulates are the totient theorem core, Basel/limit shell, and phase-lock/HL links |
| Verification/ExclusiveConfigurations | 1 | Current-syntax exclusivity shell: the base-6 `(1,5)` case study, unique seed `4`, and deterministic-prime narrative are preserved, while exhaustive membrane/primality search remains explicit |
| Verification/GCDParadoxComputation | 1 | Current-syntax grouped-comparison shell: reported positive `gcd(base,3)` correlation and grouped means are preserved, while the statistical backend remains explicit |
| Verification/ResonanceComputation | 1 | Current-syntax resonance shell: reported `(7,11)` yields and the local peak at space size `3` are preserved, while the search backend remains explicit |

## Certification Stack Status

The core certification pipeline (from SymmetryImpliesRepulsion down to concrete
examples) is operational as a mixed clean/postulated stack:

| Module | Status | Notes |
|--------|--------|-------|
| SymmetryImpliesRepulsion | clean-local | Core theorem, no postulates |
| SymmetryFromList | clean-local | Fixed: added residue-distinct to PerfectBuckets record |
| ConstrainedOrbitals | clean-local | Dynamic invariant with maintained `PointwiseSafe` smart constructors and internal `StableOrbital` derivation |
| SymmetryFiniteReflect | clean-local | Modular instantiation is now locally clean: `mkSymReflect`, the canonical even-midpoint helper, and the canonical even observed fixed-point classifier are constructive, while generic callers consume an explicit `ObservedFixedPointClassifier` contract |
| BucketsAutoMatch | clean-local | Matching support collection/transport, support-length balancing, equivariance, `no-fixed`, `residue-distinct`, and involutive are constructive; the generic count burden is now explicit as the `SupportCountsAgree` contract on the exported convenience APIs |
| WindowCertificate | clean-local | Dual certification builder is now fully clean-local over explicit static contracts (`SupportCountsAgree` + `ObservedResiduesMove`) and the narrower `PointwiseSafe` dynamic contract; `StableOrbital` is derived internally, and the helper surface now includes maintained `PointwiseSafe` constructors |
| CertifiedResonanceComplete | **clean-local** | Base 6 example (6 postulates eliminated March 2026) |
| CertifiedResonanceParam | clean-local | One-shot wrapper is locally clean; the canonical even-base entry point now derives its observed fixed-point classifier constructively, while noncanonical paths consume that classifier explicitly |
| CertifiedResonanceParamDyn | clean-local | Dual wrapper is locally clean; the canonical even-base entry point now derives its observed fixed-point classifier constructively, noncanonical paths consume that classifier explicitly, and the dynamic side now consumes the narrower `PointwiseSafe` contract instead of raw `StableOrbital`; extracted usage notes were updated to match the helper-driven path |

**Repair applied** (2026-03-09): SymmetryFromList.agda had an unsolved meta at line 84
because the `PerfectBuckets` record was missing its `residue-distinct` field (originally
moved to a postulate as a "parser bug workaround"). The fix added `residue-distinct`
back as a proper record field, eliminating the postulate. BucketsAutoMatch.agda had the
same issue in `perfectFromBalanced` and was fixed similarly. These two fixes restored
7 modules from failing to passing.

## Modules That Fail (0 modules)

Common failure patterns:
1. **stdlib 2.3 incompatibilities**: Several modules use imports or constructors
   that changed between stdlib versions.
2. **Missing dependencies**: Some modules import other failing modules.

### Full Failure List

Core/: none

Examples/: none

Other: none

### March 2026: base-10 residue filter activation

**Problem**: `Examples/Base10ResidueFilter.agda` was still a proof sketch with
holes, outdated imports, and a stale "complete proof" framing. Its paired spec
module had path drift, postulated prime witnesses, and brittle hand-written
order proofs.

**Fix**:
- replaced `Examples/Base10ResidueFilter.agda` with a smaller safe module built
  on the maintained `Core.Primality` and stdlib coprimality theorem
  `prime⇒coprime`
- made the executable filter check `gcd n 10 ≟ 1`, which matches the theorem
  path cleanly while keeping the classical `{1,3,7,9}` story visible through
  concrete examples
- rewired `Tests/Spec/Base10ResidueFilterSpec.agda` to import the repaired
  module, derive prime witnesses from `isPrime?`, and use stdlib boolean-order
  reflection instead of fragile successor chains
- marked `Core/Primality.agda` as `--safe`, which allows the example and spec
  modules to remain in the clean safe spine

**Result**:
- `Examples/Base10ResidueFilter.agda` moves from failing to passing cleanly.
- `Tests/Spec/Base10ResidueFilterSpec.agda` moves from failing to passing cleanly.
- counts move from 22 clean-local / 16 with local postulates / 42 failing to
  24 clean-local / 16 with local postulates / 40 failing.

### March 2026: residue unit/coprime forward bridge repair

**Problem**: `Core/ResidueClassesComplete.agda` still treated the entire
unit/coprime interface as postulated, and one helper theorem,
`coprime-1⇒m>1`, was stronger than the maintained `IsUnit` surface could
honestly support.

**Fix**:
- removed the false `coprime-1⇒m>1` helper from the live interface
- proved `unit-→-coprime` constructively from the current `IsUnit` witness,
  the division algorithm, and gcd/divisibility facts from the stdlib
- made `units-are-coprime` require an explicit `m > 1` witness and derive its
  forward direction from the new constructive theorem
- left only the converse `coprime-→-unit` direction postulated

**Result**:
- `Core/ResidueClassesComplete.agda` and downstream residue modules still pass.
- the file-local `postulate` block count stays at 2 because the metric is
  block-based, but the remaining open surface is narrower and more honest:
  algebraic ring laws plus the `coprime -> unit` direction.

### March 2026: residue unit converse recovery

**Problem**: After the forward bridge repair, the maintained residue API still
treated `coprime -> unit` as its last theorem-level postulate, leaving the
`units ↔ coprime` interface incomplete even though the surrounding residue
surface was now stable enough to test directly.

**Fix**:
- reconstructed `coprime-→-unit` from the stdlib Bézout identity with explicit
  `m > 1` handling in both Bézout branches
- normalized the negative Bézout branch through the canonical `(m - 1)` witness
  instead of reintroducing hidden assumptions
- added `Tests/Spec/ResidueClassesUnitsSpec.agda` as a concrete regression
  surface for the repaired equivalence on mod 10 and mod 7 residues

**Result**:
- `Core/ResidueClassesComplete.agda` still passes with local postulates, but
  its file-local `postulate` block count drops from 2 to 1.
- the `units-are-coprime` bridge is now fully constructive for `m > 1`.
- counts move from 24 clean-local / 16 with local postulates / 40 failing to
  25 clean-local / 16 with local postulates / 40 failing.

### March 2026: residue ring-law reduction

**Problem**: After the unit converse recovery, the remaining local postulate in
`Core/ResidueClassesComplete.agda` still bundled the core algebraic ring laws
with the exported `residue-ring` witness. That left the maintained residue API
looking more assumption-heavy than it really was.

**Fix**:
- recovered addition/multiplication associativity and commutativity
  constructively from stdlib modular-arithmetic lemmas
- recovered additive and multiplicative identities, plus additive inverses,
  constructively against the maintained canonical representative interface
- recovered left distributivity constructively
- introduced an explicit `modulo` helper so the repaired proofs carry their
  `NonZero` witness directly instead of relying on fragile instance search

**Result**:
- `Core/ResidueClassesComplete.agda` now exports a constructive theorem layer
  for the basic ring laws as well as `units ↔ coprime`.
- the remaining local postulate is narrower: the bundled `residue-ring`
  witness itself, not the individual algebraic laws.
- `Tests/Spec/ResidueClassesRingSpec.agda`,
  `Tests/Spec/ResidueClassesUnitsSpec.agda`, and
  `Core/ResidueClasses.agda` all still compile against the tightened surface.

### March 2026: residue ring witness packaging

**Problem**: Even after the ring-law reduction, the final local postulate in
`Core/ResidueClassesComplete.agda` was still the bundled `residue-ring`
record. That left the residue foundation looking less complete than the actual
proof layer underneath it, and it kept the residue specs out of the maintained
safe spine.

**Fix**:
- rebuilt the exported `IsCommutativeRing` witness constructively from the now
  live law layer
- added constructive equivalence and congruence helpers for residue equality,
  addition, multiplication, and negation
- derived right distributivity from left distributivity plus multiplication
  commutativity
- promoted `Core/ResidueClassesComplete.agda`,
  `Tests/Spec/ResidueClassesRingSpec.agda`, and
  `Tests/Spec/ResidueClassesUnitsSpec.agda` to `--safe`
- added those three modules to the maintained clean-safe verifier

**Result**:
- `Core/ResidueClassesComplete.agda` moves from passing with local postulates to
  passing cleanly and safely.
- `Tests/Spec/ResidueClassesRingSpec.agda` and
  `Tests/Spec/ResidueClassesUnitsSpec.agda` move into the maintained safe spine.
- counts move from 25 clean-local / 16 with local postulates / 40 failing to
  26 clean-local / 15 with local postulates / 40 failing.
- the maintained clean-safe verifier grows from 23 modules to 26 modules.

### March 2026: residue collapse bridge recovery

**Problem**: The collapse layer still mixed one genuinely open theorem with
several weaker postulated placeholders, and `Core/ResidueClasses.agda` still
described collapse as “fewer residue classes appear” even though the active
`ResidueCollapseSpec` points to frequency regularity as the real signal.

**Fix**:
- recovered the weak collapse/filtering bridge constructively in
  `Core/ResidueCollapse.agda`
- recovered the canonical collapse-count examples constructively by
  normalization
- rewrote the collapse framing in `Core/ResidueClasses.agda` so it matches the
  tested story: regularity of residue frequencies, not missing residue classes
- replaced the framework-level collapse comparison in `Core/ResidueClasses.agda`
  with a constructive inhabited witness instead of a postulate

**Result**:
- `Core/ResidueCollapse.agda` and `Core/ResidueClasses.agda` still compile.
- the file-local `postulate` count for `Core/ResidueCollapse.agda` stays at 1
  because the remaining open `all-residues-appear` theorem still uses the same
  postulate block metric, but the live postulated surface is smaller.
- the remaining collapse gap is now the general coverage theorem, not the
  canonical examples or the weak comparison bridge.

### March 2026: radical counterexample recovery

**Problem**: `Core/Radical.agda` still treated the basic standard-language
counterexample `rad ≠ φ` as a theorem-level postulate even though the file
already carried the concrete witnesses `rad(12) = 6` and `φ(12) = 4`.

**Fix**:
- recovered `radical-not-totient` constructively from the existing
  `rad-of-12` and `totient-of-12` witnesses
- recovered the exported `rad-vs-totient-example` from the same `n = 12`
  witness instead of leaving it postulated separately
- tightened the module status note so the radical layer is described as a
  mixed scaffold with one constructive counterexample, not an entirely
  postulated theorem surface

**Result**:
- `Core/Radical.agda` and `Core/ResidueClasses.agda` still compile.
- the file-local `postulate` count for `Core/Radical.agda` drops from 13 to 11.
- the radical layer now carries an honest constructive distinction between
  `rad` and `φ`, even though the broader factorization and primality proofs
  remain postulated.

### March 2026: clean vs postulated boundary guard tightening

**Problem**: The active Agda-facing docs still relied on generic wording about
"clean-local" modules atop postulated foundations. That was no longer precise
enough: the current boundary cases are narrow and identifiable, and the testing
strategy still carried stale wording from before the safe residue-ring repair.

**Fix**:
- marked the current clean-local boundary cases explicitly in the clean-passing
  table: `Theorems/ElbowsFromCSV.agda` and
  `Theorems/GlobalElbowFacts.agda` both import postulated
  `Theorems/ElbowEvents.agda`
- updated `agda-proofs/README.md` to name those boundary cases directly instead
  of referring vaguely to a "small number"
- repaired `Tests/TESTING_STRATEGY.md` so the residue-ring specs are described
  as part of the maintained clean spine again
- extended `tools/check_active_doc_drift.sh` so it fails if the elbow boundary
  notes disappear from the active status/readme surfaces

**Result**:
- the live Agda docs now distinguish clean-local from postulated-foundation
  cases concretely rather than generically
- the maintained residue/spec clean spine is no longer blurred with the
  assumption-heavy elbow-event ingestion pair
- counts stay at 26 clean-local / 15 with local postulates / 40 failing; this
  was a boundary-clarification tranche, not a category-change tranche

### March 2026: radical example recovery

**Problem**: After the `rad ≠ φ` counterexample recovery, `Core/Radical.agda`
still kept several composite-number examples postulated even though the file
already had a postulated multiplicativity theorem plus the base example
witnesses needed to derive them.

**Fix**:
- recovered `rad-6`, `rad-10`, `rad-18`, `rad-20`, and `rad-60`
  constructively from `radical-multiplicative` plus the existing base/example
  witnesses
- added explicit coprimality witnesses for the relevant factor pairs
- tightened the radical module note so it distinguishes the constructive
  counterexample/example layer from the still-postulated proof core

**Result**:
- `Core/Radical.agda` and `Core/ResidueClasses.agda` still compile.
- the radical example surface is narrower and more honest: several composite
  examples are now actual terms instead of declarations.
- the file-local `postulate` count stays at 11 because the remaining base
  witnesses still share the same postulate block metric.

### March 2026: collapse coverage refinement

**Problem**: `Core/ResidueCollapse.agda` still exposed the remaining collapse
gap as one broad postulated theorem, `all-residues-appear`. That made the open
surface harder to reason about than it needed to be.

**Fix**:
- split the remaining collapse gap into two narrower postulated bridges:
  `threshold-covers-all-residues` and
  `coverage-stabilizes-above-threshold`
- rebuilt `all-residues-appear` as a derived theorem from those two narrower
  assumptions
- tightened the status wording so the collapse layer is described as a refined
  open bridge rather than a monolithic coverage theorem

**Result**:
- `Core/ResidueCollapse.agda` and `Core/ResidueClasses.agda` still compile.
- the remaining collapse gap is now sharper: threshold coverage plus
  above-threshold stability, rather than one opaque umbrella claim.
- the file-local `postulate` count stays at 1 because the split still lives in
  a single postulate block.

### March 2026: radical proof-core continuation

**Problem**: Even after the earlier radical example recovery, `Core/Radical.agda`
still left `rad(12)` and `rad(30)` in its remaining base-example postulate
surface even though both examples can be recovered from multiplicativity plus
the small base witnesses already present in the file.

**Fix**:
- moved the small base witnesses (`rad-2`, `rad-3`, `rad-4`, `rad-5`, `rad-7`,
  `rad-8`, `rad-9`) into the core radical witness surface
- recovered `rad-of-12` constructively from `radical-multiplicative 4 3`
- recovered `rad-of-30` constructively from `radical-multiplicative 6 5`
- left `rad-of-100` as the remaining dedicated base-example postulate

**Result**:
- `Core/Radical.agda` and `Core/ResidueClasses.agda` still compile.
- the file-local `postulate` count for `Core/Radical.agda` drops from 11 to 10.
- the radical example surface is narrower again: `rad(12)` and `rad(30)` are
  now actual terms inside the module instead of base declarations.

### March 2026: elbow event source recovery

**Problem**: The active Agda status surfaces still treated
`Theorems/ElbowEvents.agda` as a postulated module and therefore marked
`Theorems/ElbowsFromCSV.agda` and `Theorems/GlobalElbowFacts.agda` as current
clean-local boundary cases, even though the live elbow source file no longer
contained any uncommented `postulate` declarations.

**Fix**:
- verified that `Theorems/ElbowEvents.agda`,
  `Theorems/ElbowsFromCSV.agda`, and
  `Theorems/GlobalElbowFacts.agda` all type-check from the `agda-proofs/`
  workspace
- reclassified `Theorems/ElbowEvents.agda` into the clean-local set
- updated the maintained clean-spine verifier to include
  `Theorems/ElbowEvents.agda`
- replaced the stale elbow boundary note with the current truth: no known
  maintained clean-local boundary cases

**Result**:
- counts move from 26 clean-local / 15 with local postulates / 40 failing to
  27 clean-local / 14 with local postulates / 40 failing
- the maintained clean-safe verifier grows from 26 modules to 27 modules
- the elbow ingestion spine is now clean end-to-end rather than clean-local
  over a postulated event source

### March 2026: totient-density interface narrowing

**Problem**: `Theorems/TotientDensity.agda` still compiled only by carrying a
large duplicated helper surface for rationals, primehood, GCD, and
coprimality. That made the file look broader and more assumption-heavy than the
remaining analytic-number-theory gap actually was.

**Fix**:
- aligned `Theorems/RationalStatistics.agda` with the `--without-K` side of the
  theorem tree so it can be reused directly by `Theorems/TotientDensity.agda`
- replaced the local `TotientDensity` postulates for `ℚ`, basic rational
  arithmetic/comparison wrappers, `Prime`, `gcd`, and decidable coprimality
  with live imports and small definitional wrappers
- recovered `φ(1) = 1` constructively as `refl`
- kept the remaining open analytic shell explicit: totient multiplicativity and
  prime-power laws, Basel/limit statements, and the phase-lock/HL connection

**Result**:
- `Theorems/TotientDensity.agda` still compiles under `--safe --without-K`
- the file-local `postulate` line count drops from 39 to 22
- the remaining open surface is narrower and more legible: the file is no
  longer blocked on duplicated arithmetic and number-theory scaffolding

### March 2026: coordinate-scaling parse recovery

**Problem**: `Theorems/CoordinateConstellationScaling.agda` still carried the
empirical HL-mismatch story, but it had fallen into parse-era syntax drift:
obsolete existential notation, numeric shorthand that no longer parsed, and a
mixture of helper postulates that prevented even basic classification of the
module's remaining assumptions.

**Fix**:
- replaced the stale theorem statements with a smaller current-syntax scaffold
  that keeps the main vocabulary and concrete observations live:
  symmetric constellations, recorded `k=3 -> 5` / `k=5 -> 7` observations,
  the base-14 outer-coordinate shell, and the modified-scaling interface
- reused the repaired totient-density layer for `φ`, `fromℕ`, and rational
  helper operations instead of inventing a third local arithmetic surface
- kept the theorem layer explicitly postulated rather than presenting the
  empirical claims as if they were already mechanized

**Result**:
- `Theorems/CoordinateConstellationScaling.agda` now compiles again
- counts move from 27 clean-local / 14 with local postulates / 40 failing to
  27 clean-local / 15 with local postulates / 39 failing
- the remaining analytic-shell blockers are narrower and easier to prioritize:
  `Theorems/ConstellationCriticalLine.agda` and
  `Theorems/HardyLittlewoodSingularSeries.agda`

### March 2026: critical-line shell recovery

**Problem**: `Theorems/ConstellationCriticalLine.agda` still carried the
interesting `-1/2` critical-line heuristic, but it was trapped behind stale
existential syntax, negative-rational shorthand that no longer matched the
repo's active rational layer, and a broken dependency on the older
`Core/ConstellationPowerLaw.agda` shell.

**Fix**:
- replaced the stale theorem script with a smaller current-syntax shell built
  on the repaired coordinate-scaling and totient-density layers
- made the signed-exponent story explicit via a `SignedMagnitude` shell instead
  of pretending the current positive-rational helper layer already supports full
  signed arithmetic
- kept the real signal visible: the empirical `-1/2` anchor, the supporting
  scaling observations, and the open critical-line / pair-correlation bridge

**Result**:
- `Theorems/ConstellationCriticalLine.agda` now compiles
- counts move from 27 clean-local / 15 with local postulates / 39 failing to
  27 clean-local / 16 with local postulates / 38 failing
- the next dependency-correct analytic-shell target became
  `Theorems/HardyLittlewoodSingularSeries.agda`

### March 2026: Hardy-Littlewood shell recovery

**Problem**: `Theorems/HardyLittlewoodSingularSeries.agda` still presented
itself as a rigorous foundation, but in practice it was blocked by parse-era
notation, stale safe-boundary assumptions, and a shell that mixed local-factor
ideas with unfinished pair-correlation and calibration claims.

**Fix**:
- replaced the old script with a smaller compilable shell that keeps the prime
  constellation vocabulary, local obstruction terminology, twin-prime / Euler
  product intuition, and membrane-prediction scaffolding
- wired it to the recovered critical-line shell instead of the old broken power
  law dependency
- kept the HL asymptotic, correlation correction, and three-constant unification
  claims explicitly postulated rather than dressing them up as live proofs

**Result**:
- `Theorems/HardyLittlewoodSingularSeries.agda` now compiles
- counts move from 27 clean-local / 16 with local postulates / 38 failing to
  27 clean-local / 17 with local postulates / 37 failing
- the analytic-shell blockage has shifted away from parser drift and toward the
  next real theorem/dependency targets, especially `GapDivisibility` and
  `HexagonalUnification`

### March 2026: gap / eigenspace / hexagonal recovery

**Problem**: the next empirical-statistics bottleneck after the analytic-shell
tranche was the `GapDivisibility -> CoordinateEigenspace -> HexagonalUnification`
chain. `GapDivisibility.agda` still had a stale theorem name and broken
percentage witnesses, `CoordinateEigenspace.agda` was blocked by local scope and
termination drift, and `HexagonalUnification.agda` still contained explicit
holes and outdated perfect-number machinery.

**Fix**:
- repaired `GapDivisibility.agda` by restoring the missing `base18-enhanced`
  witness and removing the stale broken "extreme" proof path in favor of the
  corrected percentages
- repaired `CoordinateEigenspace.agda` by adding the missing implicit base
  binder, importing `if_then_else_`, and replacing the local recursive `gcd`
  with the stdlib version
- replaced `HexagonalUnification.agda` with a smaller current-syntax shell that
  keeps the base 7/14/18 triple-manifestation witnesses live while leaving the
  universal and mechanism-level claims explicit

**Result**:
- `GapDivisibility.agda`, `CoordinateEigenspace.agda`, and
  `HexagonalUnification.agda` now compile
- counts move from 27 clean-local / 17 with local postulates / 37 failing to
  29 clean-local / 18 with local postulates / 34 failing
- the next symmetry-facing bottleneck is now the import-boundary drift in
  `PhaseLockSymmetry.agda` and `ResidueSymmetry.agda`

### March 2026: symmetry-shell recovery

**Problem**: `PhaseLockSymmetry.agda` and `ResidueSymmetry.agda` had both
drifted into a bad mixed state: strict `--safe --without-K` headers pointed at
clean abstract infrastructure, but the bodies still contained older hole-filled
instantiation scripts and could not even cross the import boundary.

**Fix**:
- replaced `PhaseLockSymmetry.agda` with a smaller current-syntax shell that
  keeps the concrete left/right phase-lock pairing and base 6 / base 10 witness
  vocabulary live
- replaced `ResidueSymmetry.agda` with a smaller current-syntax shell that
  keeps the symmetric-window and `2p²` residue stories live
- left the actual abstract-instantiation bridges explicit via one postulate
  block in each file instead of dressing the unfinished proofs up as complete

**Result**:
- `PhaseLockSymmetry.agda` and `ResidueSymmetry.agda` now compile
- counts move from 29 clean-local / 18 with local postulates / 34 failing to
  29 clean-local / 20 with local postulates / 32 failing
- the next obvious parse-era pair is now
  `CoprimalityRequirement.agda` and `RadicalDivisibilityFilter.agda`

### March 2026: divisibility-filter recovery

**Problem**: `CoprimalityRequirement.agda` and
`RadicalDivisibilityFilter.agda` both still carried long theorem scripts whose
first visible blocker was the same stale "type signature cannot have a where
clause" pattern. In practice they were serving as idea containers, not live
proof modules.

**Fix**:
- replaced `CoprimalityRequirement.agda` with a smaller current-syntax shell
  that keeps the base/config examples and the intended divisibility/density
  claims explicit
- replaced `RadicalDivisibilityFilter.agda` with a smaller current-syntax shell
  that keeps the radical-vs-totient vocabulary and concrete base 10 / 12 / 100
  witnesses live
- corrected both module names to match their `Theorems/...` paths so the files
  now compile as proper theorem modules

**Result**:
- `CoprimalityRequirement.agda` and `RadicalDivisibilityFilter.agda` now compile
- counts move from 29 clean-local / 20 with local postulates / 32 failing to
  29 clean-local / 22 with local postulates / 30 failing
- the next obvious wrapper pair is now
  `Theorems/SymmetryImpliesRepulsion.agda` and
  `Theorems/ConstrainedOrbitals.agda`

### March 2026: non-abstract symmetry wrapper recovery

**Problem**: `Theorems/SymmetryImpliesRepulsion.agda` and
`Theorems/ConstrainedOrbitals.agda` were still failing even though their
abstract counterparts were already part of the strongest clean spine. That left
the repo's narrative wrapper layer looking more broken than the underlying
symmetry core actually was.

**Fix**:
- repaired `ConstrainedOrbitals.agda` into a current-syntax theorem module with
  standard-library negation and explicit impossible-branch handling, preserving
  its constructive orbital constraints without local postulates
- replaced `SymmetryImpliesRepulsion.agda` with a smaller wrapper shell that
  keeps midpoint residue witnesses live for bases 7, 14, and 18 while leaving
  the abstract-instantiation bridge explicit through one postulate block
- aligned both module names and imports with their `Theorems/...` paths so they
  compile as proper non-abstract wrappers again

**Result**:
- `Theorems/ConstrainedOrbitals.agda` now compiles cleanly
- `Theorems/SymmetryImpliesRepulsion.agda` now compiles with local postulates
- counts move from 29 clean-local / 22 with local postulates / 30 failing to
  30 clean-local / 23 with local postulates / 28 failing
- the next obvious wrapper pair is now
  `Theorems/AffineTransform.agda` and
  `Theorems/AffineTransformComputation.agda`

### March 2026: affine wrapper namespace recovery

**Problem**: `Theorems/AffineTransform.agda` and
`Theorems/AffineTransformComputation.agda` were still failing at the
module-name boundary before any of their actual affine content could be
evaluated. That left the affine idea looking like stale syntax drift instead of
an honest shell around a still-open theorem.

**Fix**:
- replaced `AffineTransform.agda` with a smaller current-syntax theorem shell
  that keeps the membrane formula, affine shift/gradient vocabulary, and the
  base-6/base-10 example families live while leaving the general affine residue
  proof explicit through postulates
- replaced `AffineTransformComputation.agda` with a clean computation shell that
  carries maintained base-6 residue checks constructively and records the
  larger base-10 observations as reported data
- repaired both files to use the stdlib 2.3 remainder API through a `toℕ`-based
  helper with explicit `NonZero` handling instead of the old `_mod_ : ℕ`
  assumption

**Result**:
- `Theorems/AffineTransform.agda` now compiles with local postulates
- `Theorems/AffineTransformComputation.agda` now compiles cleanly
- counts move from 30 clean-local / 23 with local postulates / 28 failing to
  31 clean-local / 24 with local postulates / 26 failing
- the next visible theorem-shell blocker is now
  `Theorems/UniversalSymmetryRepulsion.agda`

### March 2026: universal symmetry shell recovery

**Problem**: `Theorems/UniversalSymmetryRepulsion.agda` was the last failing
theorem-layer shell in that symmetry family. The first visible blockers were a
missing `Data.Nat._>_` import, a hole-filled `example-symmetry` section, and
two real type issues in the multiset/pairing core that only surfaced after the
placeholder drift was cleared.

**Fix**:
- imported `Data.Nat._>_` and moved the unfinished `example-symmetry` layer
  into the module's explicit postulate surface instead of leaving it as
  half-written code
- repaired the universe levels of `MS`, `PerfectBuckets`, and `HonoraryZero`
  so the multiset/indexed-family layer matches the actual `Set`-valued
  occurrence domain
- fixed the core contradiction step in
  `PerfectBucketsImplyHonoraryZero` so the proof composes the midpoint witness
  back to the original residue before applying `residue-distinct`

**Result**:
- `Theorems/UniversalSymmetryRepulsion.agda` now compiles with local postulates
- counts move from 31 clean-local / 24 with local postulates / 26 failing to
  31 clean-local / 25 with local postulates / 25 failing
- the next visible blocker is now `Core/ConstellationPowerLaw.agda`

### March 2026: constellation power-law arithmetic recovery

**Problem**: `Core/ConstellationPowerLaw.agda` still mixed real analytical
signal with stale `Data.Nat` / `Data.Rational` namespace overlap, hole-driven
proof placeholders, and a broader theorem surface than the maintained repo
evidence could currently support.

**Fix**:
- replaced the file with a smaller current-syntax analytical shell built on the
  maintained rational type from `Theorems.RationalStatistics`
- kept the core constellation vocabulary, the twin/cousin/sexy examples, the
  reported `R² = 0.8549` fit, and the near-`-1/2` empirical story live as
  concrete shell values
- moved the universal law, inverse-square-root interpretation, and monotonic
  ordering claims into an explicit postulate surface instead of leaving them
  mixed with stale arithmetic code and holes

**Result**:
- `Core/ConstellationPowerLaw.agda` now compiles with local postulates
- counts move from 31 clean-local / 25 with local postulates / 25 failing to
  31 clean-local / 26 with local postulates / 24 failing
- the next visible blocker is now `Core/ArithmeticHelpers.agda`

### March 2026: arithmetic helper parse recovery

**Problem**: `Core/ArithmeticHelpers.agda` still carried parse-era symbol-heavy
lemma names and stale helper notation, which prevented Agda from even reaching
the useful arithmetic content of the file.

**Fix**:
- replaced the file with a smaller current-syntax helper shell that keeps the
  constructive regrouping lemmas and factorization records live
- converted the old example/proof-template layer into an explicit postulate
  surface instead of mixing it with broken notation
- repaired the remaining binder and syntax issues so the helper shell now
  compiles cleanly as a truthful mixed constructive/postulated surface

**Result**:
- `Core/ArithmeticHelpers.agda` now compiles with local postulates
- counts move from 31 clean-local / 26 with local postulates / 24 failing to
  31 clean-local / 27 with local postulates / 23 failing
- the next visible blocker is now `Core/Discriminant.agda`

### March 2026: discriminant shell recovery

**Problem**: `Core/Discriminant.agda` was still blocked by parse-era import
drift and hole-driven analytical shell code. That kept the discriminant /
Legendre / algebraic-lock layer in the failing bucket even though a narrower
empirical shell was enough to preserve the live signal honestly.

**Fix**:
- replaced the file with a smaller current-syntax analytical shell
- kept the constructive discriminant helpers, perfect-square record, and
  recorded base-6 / base-12 observations live
- moved the perfect-square decision procedure, Legendre-symbol analysis,
  quality analysis, and HL/algebraic-lock bridge into an explicit postulate
  surface instead of leaving them mixed with parser drift

**Result**:
- `Core/Discriminant.agda` now compiles with local postulates
- counts move from 31 clean-local / 27 with local postulates / 23 failing to
  31 clean-local / 28 with local postulates / 22 failing

### March 2026: Goldbach phase-lock bridge recovery

**Problem**: `Core/GoldbachPhaseLocks.agda` still failed as a parser-drifted
bridge file, which meant the repo had no live Agda shell connecting the
phase-lock vocabulary to concrete Goldbach-pair witnesses.

**Fix**:
- replaced the file with a self-contained current-syntax bridge shell
- kept concrete base-22 and base-26 phase-lock / Goldbach witnesses live
- exposed the remaining equivalence, spectral, and residue bridge as an honest
  postulate surface instead of broken imported drift

**Result**:
- `Core/GoldbachPhaseLocks.agda` now compiles with local postulates
- counts move from 31 clean-local / 28 with local postulates / 22 failing to
  31 clean-local / 29 with local postulates / 21 failing
- the next visible blocker is now `Core/GoldenRatio.agda`

### March 2026: golden-ratio shell recovery

**Problem**: `Core/GoldenRatio.agda` still mixed a real scaling idea with
broken real-analysis scaffolding, parser-drifted `where` blocks, and
non-executable notation.

**Fix**:
- replaced the file with a smaller current-syntax golden-ratio shell
- kept the base-14 crossover story, Fibonacci ratio observations, and
  multi-shell scaling vocabulary live
- moved the irrationality, convergence, periodicity, and universality claims
  into an explicit postulate surface

**Result**:
- `Core/GoldenRatio.agda` now compiles with local postulates
- counts move from 31 clean-local / 29 with local postulates / 21 failing to
  31 clean-local / 30 with local postulates / 20 failing

### March 2026: Lagrange-point shell recovery

**Problem**: `Core/LagrangePoints.agda` still carried the canonical pair signal,
but it was buried under hole-driven insertion code, stale prime-proof plumbing,
and parser-era `where` blocks.

**Fix**:
- replaced the file with a smaller current-syntax shell
- kept the canonical concatenation pair, the two reported Lagrange points, and
  small constructive counting helpers live
- left the general existence, clustering, divisibility-balance, and membrane
  enhancement claims as an explicit postulate surface

**Result**:
- `Core/LagrangePoints.agda` now compiles with local postulates
- counts move from 31 clean-local / 30 with local postulates / 20 failing to
  31 clean-local / 31 with local postulates / 19 failing

### March 2026: orthogonality framework shell recovery

**Problem**: `Core/OrthogonalityFramework.agda` still relied on stale rational
machinery and broken proof code even though its strongest signal was empirical:
the raw/HL-normalized correlation story and the still-open membrane
decorrelation hypothesis.

**Fix**:
- replaced the file with a smaller current-syntax shell
- kept the signed correlation observations, correlation-status classifier, and
  dual-score framing live
- left the membrane singular series, full normalization, and theorem-level
  orthogonality bridge explicit

**Result**:
- `Core/OrthogonalityFramework.agda` now compiles with local postulates
- counts move from 31 clean-local / 31 with local postulates / 19 failing to
  31 clean-local / 32 with local postulates / 18 failing

### March 2026: spectral shell recovery

**Problem**: `Core/Spectral.agda` still failed in low-level Legendre-symbol
proof machinery even though downstream files mostly needed its vocabulary,
supplement values, and spectral family split.

**Fix**:
- replaced the file with a smaller current-syntax spectral shell
- kept the `±1` group, `p mod 4` spectral families, shell values for
  `χ(-1)` / `χ(2)`, and primitive-root examples live
- left the full Legendre, Euler, and QR/NQR proof bridge explicit

**Result**:
- `Core/Spectral.agda` now compiles with local postulates
- counts move from 31 clean-local / 32 with local postulates / 18 failing to
  31 clean-local / 33 with local postulates / 17 failing

### March 2026: `2p` base shell recovery

**Problem**: `Core/TwoPBase.agda` still mixed real `2p`-base signal with
parse-era `where` clauses and dependencies on unfinished residue/radical proof
layers.

**Fix**:
- replaced the file with a smaller current-syntax shell
- kept concrete `2p` bases and residue sets for 6, 10, and 14 live
- left the general radical, totient, and framework bridge explicit

**Result**:
- `Core/TwoPBase.agda` now compiles with local postulates
- counts move from 31 clean-local / 33 with local postulates / 17 failing to
  31 clean-local / 34 with local postulates / 16 failing

### March 2026: phase-lock shell recovery

**Problem**: `Core/PhaseLocks.agda` was still blocked by its old safe imports
and a large amount of legacy proof bulk, even though the most stable part of
the idea was the midpoint/distance shell around symmetric prime pairs in bases
`2p`.

**Fix**:
- replaced the file with a smaller current-syntax shell
- kept the midpoint structure, Goldbach/phase-lock conversion vocabulary, and
  concrete base-10/base-14/base-22 examples live
- left the restricted-Goldbach, spectral, residue-framework, and density bridge
  explicit

**Result**:
- `Core/PhaseLocks.agda` now compiles with local postulates
- counts move from 31 clean-local / 34 with local postulates / 16 failing to
  31 clean-local / 35 with local postulates / 15 failing
- the next visible blocker is now `Advanced/Orthogonality.agda`

### March 2026: advanced orthogonality shell recovery

**Problem**: `Advanced/Orthogonality.agda` still carried a useful experiment
framing, but it was blocked by parse-era clause syntax and a half-finished
float-statistics implementation that no longer matched the recovered core
orthogonality shell.

**Fix**:
- replaced the file with a smaller current-syntax advanced shell
- kept the prime-pair experiment framing, the raw-vs-HL interpretation, and the
  membrane comparison link to `Core/OrthogonalityFramework.agda` live
- moved the Babylonian/prime-pair computational backend and float-alignment
  bridge into an explicit postulate surface

**Result**:
- `Advanced/Orthogonality.agda` now compiles with local postulates
- counts move from 31 clean-local / 35 with local postulates / 15 failing to
  31 clean-local / 36 with local postulates / 14 failing
- the next visible blocker is now `Complete/OrthogonalityFloat.agda`

### March 2026: complete orthogonality-float recovery

**Problem**: `Complete/OrthogonalityFloat.agda` still had an executable backend
worth preserving, but it was blocked by parser drift, stale builtin imports,
and a handful of current-Agda compatibility issues around fixities,
termination, and stdlib IO levels.

**Fix**:
- repaired the malformed operator declarations and old `let`/helper syntax
- rewired the file onto current builtin/stdlib Nat, Float, String, product, and
  IO surfaces
- restored the executable float arithmetic path instead of collapsing the file
  into a shell
- added explicit termination pragmas where Agda 2.8.0 no longer inferred the
  recursion automatically

**Result**:
- `Complete/OrthogonalityFloat.agda` now compiles cleanly with no local
  postulates
- counts move from 31 clean-local / 36 with local postulates / 14 failing to
  32 clean-local / 36 with local postulates / 13 failing
- the next visible blocker is now `Integration/ComputationalBridge.agda`

### March 2026: computational bridge shell recovery

**Problem**: `Integration/ComputationalBridge.agda` still sat at the interface
between the repaired Agda core and downstream tooling, but it had drifted into
parser-era `where` clauses, stale imports, and helper definitions that no
longer matched the recovered core shells.

**Fix**:
- replaced the deleted parser-era file with a smaller current-syntax
  integration shell
- kept live CRT/residue exports executable through `Core/CRTVector.agda` and
  `Core/ResidueFold.agda`
- kept concrete phase-lock, canonical Lagrange, and discriminant summary
  exports live against the recovered core shells
- moved the old Rust/WASM/unified-CLI bridge into one explicit postulate block
  instead of leaving broken export code in place

**Result**:
- `Integration/ComputationalBridge.agda` now compiles with local postulates
- counts move from 32 clean-local / 36 with local postulates / 13 failing to
  32 clean-local / 37 with local postulates / 12 failing
- the next visible blocker is now `Integration/PrimeDensityFramework.agda`

### March 2026: prime-density framework shell recovery

**Problem**: `Integration/PrimeDensityFramework.agda` still attempted to be the
repo's unified residue / phase-lock / discriminant / symmetry analysis layer,
but it had drifted into parser-era local `where` declarations, stale imports,
and hole-driven theorem claims that no longer matched the recovered shell stack.

**Fix**:
- replaced the file with a smaller current-syntax unified framework shell
- kept residue admissibility, phase-lock context, orthogonality status, and
  optional Lagrange/discriminant slices live through maintained records
- recorded concrete framework views for base 10, base 14, a base-6
  discriminant slice, and the canonical connector shell
- moved the general residue / honorary-zero / discriminant / predictor bridge
  into one explicit postulate block instead of leaving broken theorem stubs

**Result**:
- `Integration/PrimeDensityFramework.agda` now compiles with local postulates
- counts move from 32 clean-local / 37 with local postulates / 12 failing to
  32 clean-local / 38 with local postulates / 11 failing
- the next visible blocker is now `LagrangePoints/Examples.agda`

### March 2026: Lagrange examples shell recovery

**Problem**: `LagrangePoints/Examples.agda` still acted like a notebook-style
full scan with local `where` declarations, duplicated primality placeholders,
and unfinished search holes. That obscured the real formal signal around the
canonical connector pair.

**Fix**:
- replaced the file with a smaller current-syntax canonical case-study shell
- kept the two reported insertion hits live, along with buffer reflections and
  the center-void question as structured example state
- kept the membrane-like second prime connection explicit through the existing
  narrow connector shell
- moved the full scan, residue-equilibrium search, and broader theory bridge
  into one explicit postulate block

**Result**:
- `LagrangePoints/Examples.agda` now compiles with local postulates
- counts move from 32 clean-local / 38 with local postulates / 11 failing to
  32 clean-local / 39 with local postulates / 10 failing
- the next visible blocker is now `LagrangePoints/ResidueField.agda`

### March 2026: Lagrange residue-field shell recovery

**Problem**: `LagrangePoints/ResidueField.agda` still mixed half-implemented
digit arithmetic, search code, and theorem sketches with the real residue-side
signal around the canonical connector pair. The result was a parser-era
mechanism file that obscured the small-prime screening story.

**Fix**:
- replaced the file with a smaller current-syntax residue-screen shell
- kept the canonical small-prime set, the two reported compatible positions,
  and the open non-hit positions live as structured state
- preserved the residue-side interpretation as a screening lane rather than a
  complete constructive search engine
- moved the CRT/search/primality bridge into one explicit postulate block

**Result**:
- `LagrangePoints/ResidueField.agda` now compiles with local postulates
- counts move from 32 clean-local / 39 with local postulates / 10 failing to
  32 clean-local / 40 with local postulates / 9 failing
- the next visible blocker is now `LagrangePoints/TemplateExtension.agda`

### March 2026: late-tree shell recovery tranche

**Problem**: After the Lagrange example and residue shells recovered, the tail of
the Agda tree still had a mixed bag of failures: one remaining Lagrange wrapper,
two tiny parser-regression files, a stale invariant test module, and several
verification/example shells that were blocked more by namespace drift and
hole-heavy scaffolding than by deep theorem dependencies.

**Fix**:
- recovered `LagrangePoints/TemplateExtension.agda` as a current-syntax
  asymmetric-template wrapper over the canonical connector case study
- repaired `Test/TestRecord.agda` and `Test/TestRecordSimple.agda` as clean
  parser regressions for pairing-style record fields
- replaced `Tests/InvariantTests.agda` with a smaller executable stable-orbital
  regression shell
- recovered `Verification/ExclusiveConfigurations.agda`,
  `Verification/ResonanceComputation.agda`, and
  `Verification/GCDParadoxComputation.agda` as current-syntax reported-data
  shells
- restored `Examples/UniMathIntegration.agda` as a namespaced migration note
  that can legally carry its postulated bridge surface

**Result**:
- `LagrangePoints/TemplateExtension.agda` now compiles with local postulates.
- `Test/TestRecord.agda`, `Test/TestRecordSimple.agda`, and
  `Tests/InvariantTests.agda` now compile cleanly.
- `Verification/ExclusiveConfigurations.agda`,
  `Verification/ResonanceComputation.agda`,
  `Verification/GCDParadoxComputation.agda`, and
  `Examples/UniMathIntegration.agda` now compile with local postulates.
- counts move from 32 clean-local / 40 with local postulates / 9 failing to
  35 clean-local / 45 with local postulates / 1 failing.
- the next visible blocker is now `Examples/CertifiedResonance.agda`.

### March 2026: certified resonance shell recovery

**Problem**: `Examples/CertifiedResonance.agda` was the final failing module in
the tree, but its remaining value was no longer as an independent proof sketch.
The fully constructive base-6 certificate already lived in
`Examples/CertifiedResonanceComplete.agda`; the stale module mainly preserved an
older runtime-generation wrapper with parser-era drift.

**Fix**:
- replaced the old proof-sketch wrapper with a current-syntax generated shell
  over the live base-6 certificate
- kept the concrete symmetry data, perfect-bucket witness, and honorary-zero
  result live by reusing the complete certificate surface
- moved the runtime residue export / code-generation bridge into one explicit
  postulate block

**Result**:
- `Examples/CertifiedResonance.agda` now compiles with local postulates
- counts move from 35 clean-local / 45 with local postulates / 1 failing to
  35 clean-local / 46 with local postulates / 0 failing
- there are currently no remaining failing Agda modules

### March 2026: residue framework surface repair

**Problem**: `Core/ResidueClasses.agda` was blocked by an old parse failure and
then by dependency drift in `Core/Radical.agda` and `Core/ResidueCollapse.agda`.
All three files carried real conceptual signal, but their live surfaces mixed
stale stdlib syntax, unsafe proof sketches, and interfaces that no longer
matched the repaired residue-ring layer.

**Fix**:
- repaired `Core/ResidueClasses.agda` to use the current residue-ring interface,
  a total `modℕ` helper, and the residue-layer `Coprime` definition
- hardened `Core/Radical.agda` into a current-syntax postulated interface by
  removing the clashing `_ ^ _` constructor, replacing unfinished holes with
  explicit postulates, and fixing declaration-order drift
- replaced `Core/ResidueCollapse.agda` with a smaller stable interface that
  keeps the executable `distinct-residues` core live and leaves the stronger
  collapse/filtering bridge explicit
- archived the pre-repair collapse sketch at
  `archive/agda-proofs/Core/ResidueCollapse_pre_interface_repair.agda`

**Result**:
- `Core/Radical.agda` moves from failing to passing with local postulates.
- `Core/ResidueCollapse.agda` moves from failing to passing with local postulates.
- `Core/ResidueClasses.agda` moves from failing to passing with local postulates.
- Counts: 22 clean-local / 13 with local postulates / 45 failing ->
  22 clean-local / 16 with local postulates / 42 failing.

### March 2026: wheel-class coprimality bridge repair

**Problem**: `Core/ResidueClasses.agda` still relied on a local postulate to claim
that every member of `valid-prime-residues base` is coprime to `radical base`,
even though the definition already filters by `gcd k (radical base) ≟ 1`.

**Fix**: Replaced the local `wheel-coprime-lemma` postulate with a constructive
proof using `Data.List.Membership.Propositional.Properties.∈-filter⁻`. The theorem
now extracts the `gcd ... ≡ 1` witness directly from list membership in the
filtered residue list.

**Result**:
- `Core/ResidueClasses.agda` still passes with local postulates, but its local
  postulate count drops from 6 to 5.
- the residue framework now has one real constructive bridge from the wheel-class
  API back to the coprimality criterion it exposes.

### March 2026: `ResidueClassesComplete` + `ResidueClassesRingSpec` repair

**Problem**: `Core/ResidueClassesComplete.agda` had drifted across multiple axes at
once: obsolete constructor syntax, stale stdlib imports, outdated algebraic record
shape, and a partially constructive proof attempt that no longer resolved under
Agda 2.8.0 / stdlib 2.3. `Tests/Spec/ResidueClassesRingSpec.agda` was blocked behind
that drift and still used the legacy import path and instance-argument syntax.

**Fix**: Replaced the live `Core/ResidueClassesComplete.agda` surface with a smaller,
current-syntax residue-ring foundation: executable residue-class data and operations,
stable exported theorem names, and explicit postulates where the constructive bridge
is still open. The pre-repair proof attempt was archived at
`archive/agda-proofs/Core/ResidueClassesComplete_pre_foundation_repair.agda`.
Updated `Tests/Spec/ResidueClassesRingSpec.agda` to import
`Core.ResidueClassesComplete`, use the current `[ _ ]mod_` helper, and drop `--safe`
because it now targets a postulated foundation module.

**Result**:
- `Core/ResidueClassesComplete.agda` moves from failing to passing with local postulates.
- `Tests/Spec/ResidueClassesRingSpec.agda` moves from failing to passing cleanly,
  but depends on the postulated residue-ring foundation rather than a fully
  constructive proof layer.
- Counts: 21 -> 22 clean-local, 12 -> 13 with local postulates, 47 -> 45 failing.

### March 2026: `ResidueCollapseSpec` repair

**Problem**: `Tests/Spec/ResidueCollapseSpec.agda` had drifted against the
current stdlib remainder API. It used an invalid partial application of `_mod_`
and imported outdated names from `Data.Nat.Properties`.

**Fix**: Reworked the test to use an explicit `modℕ` helper built from
`Data.Nat.DivMod._mod_` plus `Data.Fin.Base.toℕ`, and removed the stale
property imports.

**Result**: `ResidueCollapseSpec.agda` moves from failing to passing cleanly.
Counts: 20 -> 21 clean-local, 48 -> 47 failing.

## Repair History

### March 2026: CertifiedResonanceParam wrapper postulate removal

**Problem**: `CertifiedResonanceParam.agda` still carried a duplicate local
`autoPerfectBuckets` assumption even though the same certification bridge
already lived in `BucketsAutoMatch.agda`. That left the one-shot wrapper in the
postulated category for the wrong reason: duplicated scaffolding rather than a
genuine remaining theorem gap.

**Fix**: Replaced the local `autoPerfectBuckets` postulate with a small
constructive wrapper around `BucketsAutoMatch.perfectFromBalanced`, using
`countResid` as the count function and a local `BalancedBuckets` record built
from the already-supplied balanced-count witness.

**Result**:
- `CertifiedResonanceParam.agda` moves from `postulates (1)` to a clean-local
  boundary wrapper over `BucketsAutoMatch.agda`.
- `CertifiedResonanceParamDyn.agda` drops from `postulates (2)` to
  `postulates (1)` because the duplicated static auto-pairing assumption is
  gone and only the runtime `proof-stable` witness remains local.
- Counts move from 35 clean-local / 46 with local postulates / 0 failing to
  36 clean-local / 45 with local postulates / 0 failing.

### March 2026: BucketsAutoMatch helper reduction

**Problem**: `BucketsAutoMatch.agda` still treated both index collection and
list pairing as assumed helpers, even though those two pieces are purely
computational and do not require the remaining theorem-level pairing laws.

**Fix**: Replaced the local `indices-with-residue` and `zip-pair` assumptions
with constructive definitions. `indices-with-residue` now recurses directly on
`Fin n` via `fzero` / `fsuc`, and `zip-pair` now pairs two lists element-wise
while leaving unmatched indices untouched.

**Result**:
- `BucketsAutoMatch.agda` keeps the same category (`with local postulates`) but
  its remaining assumption surface is now narrower and more honest: the only
  local assumption left is the `auto-mate-*` law block.
- Category counts stay at 36 clean-local / 45 with local postulates / 0
  failing.

### March 2026: BucketsAutoMatch no-fixed recovery

**Problem**: `BucketsAutoMatch.agda` still treated `auto-mate-no-fixed` as an
assumed law even though it follows immediately from `auto-mate-residue-distinct`
once a hypothetical fixed point is pushed through `f`.

**Fix**: Replaced the local `auto-mate-no-fixed` postulate with a constructive
proof that applies `cong f` to a candidate fixed point and discharges the
result with `auto-mate-residue-distinct`.

**Result**:
- `BucketsAutoMatch.agda` stays in the `with local postulates` category, but
  the remaining open surface is now explicitly the three-law block
  `auto-mate-involutive / auto-mate-equivariant / auto-mate-residue-distinct`.
- Category counts stay at 36 clean-local / 45 with local postulates / 0
  failing.

### March 2026: Fixed-residue assumption surfacing

**Problem**: `auto-mate-residue-distinct` still looked like a theorem the
auto-matching layer ought to produce on its own, but the real missing input was
structural: the observed residues must avoid involution-fixed points.

**Fix**: Introduced `ObservedResiduesMove` as an explicit witness
`∀ i → inv (f i) ≢ f i`, replaced the local `auto-mate-residue-distinct`
postulate with a constructive proof from that witness plus
`auto-mate-equivariant`, and threaded the witness through
`perfectFromBalanced`, `honoraryZeroFromBalanced`, and the parameterized
certification wrappers.

**Result**:
- `BucketsAutoMatch.agda` still stays in the `with local postulates` category,
  but the remaining open surface is now explicitly the two-law block
  `auto-mate-involutive / auto-mate-equivariant`.
- the certification wrappers now name the fixed-residue burden honestly instead
  of hiding it inside a generic auto-mate postulate.
- Category counts stay at 36 clean-local / 45 with local postulates / 0
  failing.

### March 2026: Reflection fixed-point exclusion bridge

**Problem**: The certification wrappers had become honest about needing
`ObservedResiduesMove`, but that still left callers proving a generic
per-observation non-fixed-residue witness even in the concrete half-turn
reflection cases the repo actually uses.

**Fix**: Added `ReflectFixedPointExclusion` to
`SymmetryFiniteReflect.agda`, packaging the narrower modular reflection burden:
`zero-fixed / zeroVoid / midVoid`. The same module now exposes
`observedResiduesMoveFromFixedPointExclusion`, using a half-turn fixed-point
classification postulate to derive `ObservedResiduesMove` for `mkSymReflect`.
`CertifiedResonanceParam.agda` and `CertifiedResonanceParamDyn.agda` now
consume that concrete reflection witness instead of asking for a generic
`ObservedResiduesMove` function.

**Result**:
- `SymmetryFiniteReflect.agda` remains in the `with local postulates` category,
  but the fixed-point burden is now concrete and local to the modular
  reflection layer.
- the parameterized certification wrappers now align with the repo's actual
  half-turn use cases: prove zero is fixed, exclude zero and midpoint from the
  observed support, then derive `ObservedResiduesMove` internally.
- Category counts stay at 36 clean-local / 45 with local postulates / 0
  failing.

### March 2026: BucketsAutoMatch equivariance narrowing

**Problem**: `auto-mate-equivariant` was still a large end-to-end assumption.
The constructive code already built residue support lists and zipped them, but
the remaining gap was more specific: the file still lacked an honest bridge
from bucket counts to support-list lengths and from aligned `zip-pair` targets
back to residue values.

**Fix**: Replaced the top-level `auto-mate-equivariant` postulate with two
smaller assumptions, `support-counts-agree` and
`zip-pair-preserves-target-residue`, then recovered
`auto-mate-support-lengths` and `auto-mate-equivariant` constructively from
those plus the existing `BalancedBuckets` witness.

**Result**:
- `BucketsAutoMatch.agda` stays in the `with local postulates` category, but
  the old end-to-end equivariance claim is gone from the assumption surface.
- the remaining bridge is now more honest and local to the real missing facts:
  `auto-mate-involutive / support-counts-agree /
  zip-pair-preserves-target-residue`.
- Category counts stay at 36 clean-local / 45 with local postulates / 0
  failing.

### March 2026: Support-count agreement recovery for the `countResid` path

**Problem**: After the equivariance split, the concrete parameterized
certification wrappers still inherited the generic `support-counts-agree`
assumption from `BucketsAutoMatch.agda`, even though they already use the
concrete `countResid` function whose support-count meaning is straightforward.

**Fix**: Added `SupportCountsAgree`, `perfectFromBalancedWithSupport`, and
`honoraryZeroFromBalancedWithSupport` in `BucketsAutoMatch.agda`, then proved
`supportCountsAgreeCountResid` constructively in both
`CertifiedResonanceParam.agda` and `CertifiedResonanceParamDyn.agda`. Their
`autoPerfectBuckets` helpers now use the explicit-support path rather than the
generic `support-counts-agree` postulate.

**Result**:
- the concrete `countResid` certification lane no longer depends on the
  generic `support-counts-agree` assumption.
- the remaining concrete bridge in that lane is now just
  `auto-mate-involutive / zip-pair-preserves-target-residue`, plus the runtime
  `proof-stable` witness in the dynamic wrapper.
- Category counts stay at 36 clean-local / 45 with local postulates / 0
  failing.

### March 2026: Zip-pair support reduction

**Problem**: After Track 61, the remaining imported theorem in the concrete
`countResid` certification lane was still phrased in residue terms:
`zip-pair-preserves-target-residue`. The real unproved core was narrower: a
pure list-support alignment fact about where `zip-pair` sends source members.

**Fix**: Added constructive support lemmas in `BucketsAutoMatch.agda`:
`indices-with-residue-complete`, `indices-with-residue-sound`, and
`support-lists-disjoint`. Replaced the remaining residue-level assumption with
the smaller pure list theorem `zip-pair-sends-source-to-target-support`, then
recovered `zip-pair-preserves-target-residue` constructively from support
membership, soundness, and disjointness.

**Result**:
- `BucketsAutoMatch.agda` still stays in the `with local postulates` category,
  but the remaining imported `zip-pair` bridge is now purely about list-support
  alignment.
- the concrete `countResid` certification lane now depends on
  `auto-mate-involutive / zip-pair-sends-source-to-target-support`, plus the
  runtime `proof-stable` witness in the dynamic wrapper.
- Category counts stay at 36 clean-local / 45 with local postulates / 0
  failing.

### March 2026: Auto-mate involutive narrowing

**Problem**: After the support reduction, the concrete `countResid`
certification lane still imported `auto-mate-involutive` directly. That mixed a
real remaining gap with a larger theorem name than the code actually needed.

**Fix**: Added `auto-mate-second-step-shape` and
`auto-mate-involutive-from` in `BucketsAutoMatch.agda`. Replaced the direct
involutive assumption with the smaller pure support-alignment theorem
`zip-pair-roundtrips-on-disjoint-support`, then rebuilt
`auto-mate-involutive` constructively from support disjointness, support
lengths, equivariance, and that roundtrip lemma.

**Result**:
- `BucketsAutoMatch.agda` still stays in the `with local postulates` category,
  but the direct involutive axiom is gone from the remaining generic surface.
- the remaining generic burden is now the pure support-alignment trio
  `support-counts-agree / zip-pair-sends-source-to-target-support /
  zip-pair-roundtrips-on-disjoint-support`.
- the concrete `countResid` certification lane no longer depends on a direct
  imported involutive theorem; it now inherits only the smaller pure `zip-pair`
  support-alignment bridge, plus the runtime `proof-stable` witness in the
  dynamic wrapper.
- Category counts stay at 36 clean-local / 45 with local postulates / 0
  failing.

### March 2026: Zip-pair roundtrip narrowing

**Problem**: The remaining roundtrip theorem was still phrased too broadly as
`zip-pair-roundtrips-on-disjoint-support`. That statement is false for arbitrary
equal-length disjoint lists with duplicates, so the real missing structure was
not being named honestly.

**Fix**: Introduced `ListUnique`, proved `lift-fin-list-unique` and
`indices-with-residue-unique` constructively, and replaced the old generic
roundtrip assumption with the constructive theorem
`zip-pair-roundtrips-on-unique-disjoint-support`. The proof now derives
roundtrip behavior from support-list uniqueness plus the still-open
source-to-target transport theorem `zip-pair-sends-source-to-target-support`.

**Result**:
- `BucketsAutoMatch.agda` still stays in the `with local postulates` category,
  but the generic roundtrip theorem is gone from the remaining open surface.
- the remaining generic burden is now the smaller pair
  `support-counts-agree / zip-pair-sends-source-to-target-support`.
- the concrete `countResid` certification lane no longer depends on a generic
  roundtrip theorem; its remaining imported bridge is now just the pure
  source-to-target `zip-pair` support transport, plus the runtime
  `proof-stable` witness in the dynamic wrapper.
- Category counts stay at 36 clean-local / 45 with local postulates / 0
  failing.

### March 2026: Zip-pair transport recovery

**Problem**: After the roundtrip narrowing, the last imported theorem still
used in the concrete `countResid` certification lane was
`zip-pair-sends-source-to-target-support`.

**Fix**: Replaced that theorem with a constructive proof by direct induction on
the paired support lists. The proof follows the actual `zip-pair` control flow:
head members map to the right head, tail members either collapse to the right
head when they match the left head or recurse into the tail pairing.

**Result**:
- `BucketsAutoMatch.agda` still stays in the `with local postulates` category,
  but the generic transport theorem is gone from the remaining open surface.
- the remaining generic burden is now just `support-counts-agree`.
- the concrete `countResid` certification lane no longer depends on any
  imported auto-pairing theorem; only the runtime `proof-stable` witness
  remains local in the dynamic wrapper.
- Category counts stay at 36 clean-local / 45 with local postulates / 0
  failing.

### March 2026: Support-count agreement narrowing

**Problem**: After the transport recovery, `support-counts-agree` was the last
generic local theorem postulate inside `BucketsAutoMatch.agda`. That blurred
two different responsibilities: the clean auto-pairing logic in the module, and
extra evidence about arbitrary counting functions that belongs at the API
boundary instead.

**Fix**: Removed the local `support-counts-agree` postulate from
`BucketsAutoMatch.agda`, kept `SupportCountsAgree` as an explicit contract type,
and rewired the exported convenience APIs to accept that contract directly.
The concrete `countResid` wrappers already discharge that witness
constructively via `perfectFromBalancedWithSupport` /
`honoraryZeroFromBalancedWithSupport`, so no downstream theorem surface was
weakened.

**Result**:
- `BucketsAutoMatch.agda` moves from `with local postulates` to `clean-local`.
- the maintained clean spine grows from 27 modules to 28 modules and now
  includes `BucketsAutoMatch.agda`.
- the remaining certification boundary shifts outward to
  `SymmetryFiniteReflect.agda` and the dynamic wrapper's runtime
  `proof-stable` witness.
- counts move from 36 clean-local / 45 with local postulates / 0 failing to
  37 clean-local / 44 with local postulates / 0 failing.

### March 2026: WindowCertificate contract alignment

**Problem**: `WindowCertificate.agda` still read like the older certification
lane: it postulated `deriveHonoraryZero` and `deriveInviolability` even though
the underlying stack now exposes the real inputs explicitly. That hid the true
boundary between clean auto-pairing, fixed-point exclusion, and dynamic
stability.

**Fix**:
- introduced `StaticContracts` to bundle the actual static requirements:
  `SupportCountsAgree _≟Fin_ residues count` plus
  `ObservedResiduesMove S residues`
- rewired `StaticCertificate` to derive `honorary-zero` directly via
  `honoraryZeroFromBalanced`
- rewired `DynamicCertificate` to derive `inviolability` directly from
  `Inviolability stable-witness`
- removed the top-level `deriveHonoraryZero` and `deriveInviolability`
  postulates, leaving only the hypothetical example-shell postulate block

**Result**:
- `WindowCertificate.agda` still stays in the `with local postulates`
  category, but its builder surface is now constructive and names the real
  support-count and fixed-point burdens explicitly.
- the remaining local postulate block in the file is the bundled Base-14
  example shell, not the certification builder itself.
- category counts stay at 37 clean-local / 44 with local postulates / 0
  failing.

### March 2026: SymmetryFiniteReflect fixed-point contract split

**Problem**: `SymmetryFiniteReflect.agda` still bundled two different burdens
into one witness, `ReflectFixedPointExclusion`: arithmetic facts about the
half-turn reflection itself and observed-data facts about which residues appear.
That made the certification boundary less legible than it needed to be after
`WindowCertificate` was hardened.

**Fix**:
- split the old bundled witness into `HalfTurnFixedPointClassification` for the
  arithmetic side and `ObservedFixedPointExclusion` for the observed-support
  side
- renamed the remaining arithmetic postulates to the narrower
  `half-turn-zero-fixed` and `half-turn-fixed-points-are-zero-or-mid`
- added `canonicalHalfTurnFixedPointClassification` and rewired
  `observedResiduesMoveFromFixedPointContracts` plus the parameterized
  certification examples to use the split contract surface explicitly

**Result**:
- `SymmetryFiniteReflect.agda` stays in the `with local postulates` category,
  but the remaining arithmetic burden is now clearly separated from the
  residue-scan exclusions used by the certification wrappers.
- `CertifiedResonanceParam.agda` and `CertifiedResonanceParamDyn.agda` now ask
  for explicit arithmetic classification plus explicit support exclusion rather
  than a single mixed witness.
- category counts stay at 37 clean-local / 44 with local postulates / 0
  failing.

### March 2026: Half-turn arithmetic shell clarification

**Problem**: After the fixed-point contract split, the external arithmetic
surface in `SymmetryFiniteReflect.agda` was still a little wider than the
certification lane actually needed. The examples and docs still talked as if
they consumed the full split record, even though the runtime-facing path only
uses the fixed-point classifier itself.

**Fix**:
- narrowed the exported arithmetic contract from the record
  `HalfTurnFixedPointClassification` down to the simpler predicate
  `HalfTurnFixedPointClassifier`
- kept `half-turn-zero-fixed` as an internal arithmetic prerequisite used only
  to build the canonical classifier helper
- added the constructive helper `observedFixedPointExclusion`, which turns the
  classifier plus observed support exclusion into per-residue exclusion directly
- rewired the parameterized certification wrappers to request the classifier
  explicitly and updated the active docs to match

**Result**:
- the remaining arithmetic shell is named more precisely: the external boundary
  is the fixed-point classifier, while `half-turn-zero-fixed` stays internal to
  the canonical arithmetic helper path.
- the certification wrappers now ask only for the arithmetic shell they
  actually consume.
- category counts stay at 37 clean-local / 44 with local postulates / 0
  failing.

### March 2026: Half-turn zero-fixed shell positioning

**Problem**: After the classifier narrowing, `half-turn-zero-fixed` was still a
public theorem name in `SymmetryFiniteReflect.agda` even though no downstream
module consumed it directly. That made the file look like it exposed two
arithmetic shell layers when the certification lane actually uses only the
classifier.

**Fix**:
- moved `half-turn-zero-fixed` into a private postulate block inside
  `SymmetryFiniteReflect.agda`
- tightened the local comments so the file now states explicitly that the
  exported arithmetic shell is the fixed-point classifier and `half-turn-zero-fixed`
  only feeds the canonical helper path internally

**Result**:
- the internal vs external role of the half-turn arithmetic shell is now real
  in code, not just prose.
- the remaining external arithmetic boundary in `SymmetryFiniteReflect.agda`
  is just `HalfTurnFixedPointClassifier`.
- category counts stay at 37 clean-local / 44 with local postulates / 0
  failing.

### March 2026: Half-turn classifier shell reduction

**Problem**: After Track 70, the public certification wrappers still consumed
the broad `HalfTurnFixedPointClassifier` contract from
`SymmetryFiniteReflect.agda` even though they only needed the observed-support
instance of that theorem on their concrete residue lists.

**Fix**:
- added `ObservedFixedPointClassifier` plus the helper path
  `observedFixedPointClassifierFromClassifier` /
  `canonicalObservedFixedPointClassifier` inside
  `SymmetryFiniteReflect.agda`
- changed `observedFixedPointExclusion` and
  `observedResiduesMoveFromFixedPointContracts` to consume the observed-only
  classifier rather than the full arithmetic classifier
- added `observedResiduesMoveFromObservedSupportExclusion`, so the active
  certification wrappers can reuse the canonical arithmetic shell internally
  while asking callers only for observed support exclusion
- rewired `CertifiedResonanceParam.agda` and
  `CertifiedResonanceParamDyn.agda` to drop the explicit classifier argument

**Result**:
- the remaining public arithmetic shell in `SymmetryFiniteReflect.agda` is more
  honest: the full half-turn fixed-point classifier still exists, but the
  active certification lane no longer consumes it directly
- `CertifiedResonanceParam.agda` and `CertifiedResonanceParamDyn.agda` now ask
  only for observed support exclusion on their concrete support, not a broader
  arithmetic classifier contract
- category counts stay at 37 clean-local / 44 with local postulates / 0
  failing.

### March 2026: CertifiedResonanceParamDyn runtime shell extraction

**Problem**: After `WindowCertificate.agda` was cleaned up, the sharpest
remaining local postulate in the active certification wrappers was the embedded
Base-6 runtime witness shell in `CertifiedResonanceParamDyn.agda`. That made
the whole dual wrapper look assumption-heavy even though the active static path
was already clean-local.

**Fix**:
- extracted the old `Example-Base6-Dual` shell out of
  `CertifiedResonanceParamDyn.agda`
- preserved that usage shape in
  `Examples/CERTIFIED_RESONANCE_PARAM_DYN_BASE6_SKETCH.md`
- left `CertifiedResonanceParamDyn.agda` focused on the dual certification API
  and its clean-local static/runtime composition surface

**Result**:
- `CertifiedResonanceParamDyn.agda` moves from `postulates (1)` to a
  clean-local boundary wrapper
- the active certification lane now has two clean-local boundary wrappers
  (`CertifiedResonanceParam.agda` and `CertifiedResonanceParamDyn.agda`) over
  the same narrower `SymmetryFiniteReflect.agda` arithmetic shell
- category counts improve to 39 clean-local / 42 with local postulates / 0
  failing.

### March 2026: SymmetryFiniteReflect half-turn contract correction

**Problem**: The remaining arithmetic shell in `SymmetryFiniteReflect.agda`
still hid a false universal assumption: `half-turn-zero-fixed` was postulated
for arbitrary `mid`, even though `reflect mid 0 = 0` only holds for genuine
half-turn midpoint choices. That made the wrapper boundary look narrower than
it really was.

**Fix**:
- introduced the explicit public contract `HalfTurnMidpoint mid`
- removed the hidden universal `half-turn-zero-fixed` helper from the exported
  path
- changed the active wrappers to require `HalfTurnMidpoint mid` plus observed
  support exclusion
- reduced the remaining internal arithmetic postulate to the pointwise theorem
  `half-turn-fixed-point-case`

**Result**:
- the certification API is now mathematically honest about the half-turn
  midpoint witness it needs
- the remaining arithmetic bottleneck in `SymmetryFiniteReflect.agda` is
  smaller and better localized: an explicit midpoint witness plus a pointwise
  fixed-residue case theorem
- category counts stay at 39 clean-local / 42 with local postulates / 0
  failing, but the theorem surface is materially more rigorous.

### March 2026: Canonical half-turn midpoint recovery

**Problem**: After the contract correction, the active wrappers were honest but
still clumsy for the standard even-base case. Callers had to supply
`HalfTurnMidpoint mid` manually even when `mid` was just the canonical
`base / 2` residue in modulus `2h`.

**Fix**:
- recovered a constructive `canonicalEvenHalfTurnMidpoint` helper in
  `SymmetryFiniteReflect.agda` for the standard midpoint choice
- exposed `canonicalEvenMidpoint` so the canonical even-base residue is named
  directly in the formal layer
- added canonical-even convenience entry points in
  `CertifiedResonanceParam.agda` and `CertifiedResonanceParamDyn.agda`

**Result**:
- the standard even-base certification path no longer needs a manual midpoint
  witness
- the remaining arithmetic bottleneck is now sharper: the internal pointwise
  fixed-residue case theorem in `SymmetryFiniteReflect.agda`
- category counts stay at 39 clean-local / 42 with local postulates / 0
  failing, but the canonical certification lane is simpler and more
  constructive than before.

### March 2026: Half-turn fixed-point shell split

**Problem**: After the canonical midpoint recovery, the remaining fixed-point
burden in `SymmetryFiniteReflect.agda` was still packaged as one broad theorem:
every fixed residue is either `0` or `mid`. That obscured which part was still
open and which part was already cheap or constructive.

**Fix**:
- proved `reflect mid mid` constructively inside `SymmetryFiniteReflect.agda`
- replaced the broad `half-turn-fixed-point-case` postulate with the narrower
  shell theorem `half-turn-fixed-point-nonzero-is-mid`
- rebuilt the exported fixed-point classifier by a decidable split on
  `r ≟ finZero`, so the zero branch is now constructive and only the nonzero
  classification remains imported

**Result**:
- the remaining internal fixed-point bottleneck is now explicit: nonzero fixed
  residues must equal `mid`
- the standard classifier used by the certification wrappers is narrower and
  more honest than before
- category counts stay at 39 clean-local / 42 with local postulates / 0
  failing, but the symmetry shell is materially sharper.

### March 2026: Reflection involution recovery

**Problem**: Even after the fixed-point shell split, `mkSymReflect` still
rested on a postulated `reflect-involutive` law. That left the core symmetry
data itself depending on an arithmetic shell even though the remaining open
burden had already moved elsewhere.

**Fix**:
- recovered `reflect-involutive` constructively in
  `SymmetryFiniteReflect.agda`
- added a maintained `reflect-value` helper so the involution proof is stated
  at the `toℕ` level and then discharged with modular arithmetic
- left the only remaining local postulate in that file as the nonzero fixed
  residue classification theorem

**Result**:
- `mkSymReflect` is now fully constructive
- the remaining certification bottleneck in `SymmetryFiniteReflect.agda` is no
  longer a symmetry-law shell; it is only the nonzero fixed-point
  classification theorem
- category counts stay at 39 clean-local / 42 with local postulates / 0
  failing, but the symmetry core is materially stronger.

### March 2026: CertifiedResonanceParam/ParamDyn example postulate reduction

**Problem**: Both `CertifiedResonanceParam.agda` and `CertifiedResonanceParamDyn.agda`
had postulated witnesses (proof-midVoid, proof-balanced) in their Example modules for
the same Base 6 data that CertifiedResonanceComplete proves constructively.

**Fix**: Replaced the postulated example witnesses with direct PerfectBuckets
construction using explicit fzero/fsuc case analysis (same technique as Track 20).
The examples now bypass the framework-level `autoPerfectBuckets` postulate entirely.

**Result**:
- CertifiedResonanceParam: Example module has 0 local postulates (was 2).
  File total: 1 local postulate (framework autoPerfectBuckets only).
- CertifiedResonanceParamDyn: Example module has 1 local postulate (proof-stable, runtime
  dependent). File total: 2 local postulates. Was 4 (3 example + 1 framework).

### March 2026: CertifiedResonanceComplete postulate elimination

**Problem**: `CertifiedResonanceComplete.agda` used 6 postulates for trivially
provable properties (involutive, no-fixed, equivariant, residue-distinct). The
postulates existed because the `#_` operator for Fin literals did not compute
in Agda's pattern matching.

**Fix**: Replaced all `#_` abbreviations (`# 0`, `# 1`, etc.) with explicit
`fzero`/`fsuc` constructor patterns. All 6 postulates became constructive proofs
via `refl` (equality proofs) or absurd patterns (inequality proofs).

**Result**: Module moves from "passes with local postulates (6)" to
"passes clean-local (0)". Counts: 19 -> 20 clean-local, 13 -> 12 with local
postulates. The Base 6 flagship certification is now fully machine-checked
with no assumptions.

### March 2026: SymmetryFromList + BucketsAutoMatch fix

**Problem**: `SymmetryFromList.agda` had a `PerfectBuckets` record missing the
`residue-distinct` field. The field was postulated separately but never supplied
to the `Pairing` record constructor in `pairingFromPerfect`. Similarly,
`BucketsAutoMatch.agda` omitted the field in `perfectFromBalanced`.

**Fix**: Added `residue-distinct` as a proper field of `PerfectBuckets` (eliminating
the postulate in SymmetryFromList). Wired the postulated `auto-mate-residue-distinct`
into the record literal in BucketsAutoMatch.

**Result**: 7 modules restored (18 -> 19 clean-local, 7 -> 13 with local postulates, 55 -> 48 failing).

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

# Test the maintained 28-module clean-safe spine.
for f in \
  Theorems/Abstract/SymmetryImpliesRepulsion.agda \
  Theorems/Abstract/SymmetryFromList.agda \
  Theorems/Abstract/ConstrainedOrbitals.agda \
  Theorems/Abstract/BucketsAutoMatch.agda \
  Core/Primality.agda \
  Core/ResidueClassesComplete.agda \
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
  Tests/Spec/ResidueCollapseSpec.agda \
  Examples/Base10ResidueFilter.agda \
  Tests/Spec/Base10ResidueFilterSpec.agda \
  Tests/Spec/ResidueClassesRingSpec.agda \
  Tests/Spec/ResidueClassesUnitsSpec.agda \
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

Or use the maintained helper:

```bash
cd agda-proofs
./scripts/verify-clean-spine.sh
```
