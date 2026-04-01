# Lean 4 Formalization Roadmap

**Updated**: March 2026

This roadmap treats the Lean package as a formalization program for finite
symmetry theorems and exact residue-class arithmetic. Throughout the active
docs, "membrane" is only a repository label; the Lean targets themselves are
stated in standard mathematical language.

## Tracks

- **Track 1**: symmetry certification in Lean, beginning with the abstract
  midpoint-obstruction theorem and a concrete base-6 witness
- **Track 2**: conservative density foundations, restricted to classical
  coprimality, radical, unit-residue, finite-CRT, and unit-group orbit facts;
  the generic arithmetic layer is now live
- **Track 2a**: exact template-structure facts, currently including the affine
  form of the fixed symmetric layout and its modular seed-search consequences
- **Track 2b**: exact prime-engine correctness facts, currently including the
  odd-only segmented sieve arithmetic and the wheel30 candidate surface
- **Track 3**: optional OpenProver-assisted proof search against this Lean
  package once Track 1 is stable locally and in CI

## Queued Future Tranches

These are the next durable targets after the current symmetry, residue-class,
CRT, orbit, and affine-template surfaces.

The affine residue-search tranche is now complete: the template lane has
explicit divisor-of-base residue rigidity, affine residue permutations for
coprime moduli, and explicit seed classes. The next two previously queued
density tranches are also complete: the wheel-base CRT lane now has a
canonical finite-family theorem, and the negation quotient now has a matching
group-action formulation via the order-two subgroup `{1, -1}`. The next two
symmetry tranches are now also complete: the Lean package has a concrete
modular reflection layer on `Fin base`, a reusable reflection-certificate
wrapper, a constructive balanced-bucket support substrate for reflected residue
supports, a balanced-bucket reflection wrapper, a narrow window-certificate
shell, a generated-data entrypoint from residue and position lists, explicit
finite certificate examples, a compact proof-object layer for generated
artifacts, and a mirror-family midpoint-obstruction theorem.
The exact connector arithmetic tranche is now
also complete: the Lean package has fixed-width forward/reverse concatenation
formulas, canonical decimal `mod 3` and `mod 9` exclusion filters for the
maintained connector pair, a reusable connector-family residue-profile API, and
maintained profile examples beyond the single canonical pair.
The prime-engine correctness tranche is now also complete: the Lean package
has the odd-only segmented sieve arithmetic layer, the exact wheel30 admissible
residues, and the agreement theorem connecting wheel30 candidates back to the
filtered odd candidate domain. The runtime-facing sieve-layout tranche is now
also live: the Lean package now matches the Rust segment constants and their
arithmetic span in
[`PrimeArithmetic/Sieve/SegmentLayout.lean`](PrimeArithmetic/Sieve/SegmentLayout.lean),
the runtime cross-off branch and `2 * p` marking progression in
[`PrimeArithmetic/Sieve/RuntimeCrossOff.lean`](PrimeArithmetic/Sieve/RuntimeCrossOff.lean),
the runtime odd-endpoint adjustment and collection-index witness in
[`PrimeArithmetic/Sieve/RuntimeCollection.lean`](PrimeArithmetic/Sieve/RuntimeCollection.lean),
the shared odd-only byte/bit coordinates in
[`PrimeArithmetic/Sieve/SegmentBitCoordinates.lean`](PrimeArithmetic/Sieve/SegmentBitCoordinates.lean),
the exact odd-only `1 << bit` mask and readback semantics in
[`PrimeArithmetic/Sieve/SegmentBitMasks.lean`](PrimeArithmetic/Sieve/SegmentBitMasks.lean),
the generic bounded multi-mark family on disjoint byte slots in
[`PrimeArithmetic/Sieve/BoundedByteFamilies.lean`](PrimeArithmetic/Sieve/BoundedByteFamilies.lean),
the aggregated same-byte mask family in
[`PrimeArithmetic/Sieve/BoundedByteMasks.lean`](PrimeArithmetic/Sieve/BoundedByteMasks.lean),
and the grouped multi-byte plan family in
[`PrimeArithmetic/Sieve/BoundedBytePlans.lean`](PrimeArithmetic/Sieve/BoundedBytePlans.lean),
the tiny shared coordinate bridge in
[`PrimeArithmetic/Sieve/BoundedByteCoordinates.lean`](PrimeArithmetic/Sieve/BoundedByteCoordinates.lean),
and the bounded single-byte runtime bridge in
[`PrimeArithmetic/Sieve/SegmentByteArray.lean`](PrimeArithmetic/Sieve/SegmentByteArray.lean),
the short odd-only runtime mark-family shell in
[`PrimeArithmetic/Sieve/SegmentRuntimePlans.lean`](PrimeArithmetic/Sieve/SegmentRuntimePlans.lean),
and it matches the wheel30 slot order, linear index formula, and byte/bit split
used by the runtime bit array in
[`PrimeArithmetic/Sieve/Wheel30Indexing.lean`](PrimeArithmetic/Sieve/Wheel30Indexing.lean),
together with the shared wheel30 writer/reader byte/bit coordinates in
[`PrimeArithmetic/Sieve/Wheel30BitCoordinates.lean`](PrimeArithmetic/Sieve/Wheel30BitCoordinates.lean),
plus the exact wheel30 `1 << bit` mask and readback semantics in
[`PrimeArithmetic/Sieve/Wheel30BitMasks.lean`](PrimeArithmetic/Sieve/Wheel30BitMasks.lean),
and the bounded single-byte wheel30 array shell in
[`PrimeArithmetic/Sieve/Wheel30ByteArray.lean`](PrimeArithmetic/Sieve/Wheel30ByteArray.lean),
plus the short wheel30 runtime mark-family shell in
[`PrimeArithmetic/Sieve/Wheel30RuntimePlans.lean`](PrimeArithmetic/Sieve/Wheel30RuntimePlans.lean).
The external window-export path is now also
live: the Rust binary
[`src/bin/export_window_certificate.rs`](../src/bin/export_window_certificate.rs)
extracts runtime prime-window positions and residues, checks the balanced and
fixed-point-free certificate preconditions, and emits Lean modules targeting
the generated-data window shell and bundled proof-object layer. The tracked
catalog now contains cross-base samples at
[`PrimeArithmetic/Generated/Examples/WindowP3Base6Span5.lean`](PrimeArithmetic/Generated/Examples/WindowP3Base6Span5.lean),
[`PrimeArithmetic/Generated/Examples/WindowP5Base10Span5.lean`](PrimeArithmetic/Generated/Examples/WindowP5Base10Span5.lean),
and
[`PrimeArithmetic/Generated/Examples/WindowP5Base12Span17.lean`](PrimeArithmetic/Generated/Examples/WindowP5Base12Span17.lean),
plus wheel-like larger-base samples at
[`PrimeArithmetic/Generated/Examples/WindowP11Base30Span5.lean`](PrimeArithmetic/Generated/Examples/WindowP11Base30Span5.lean),
[`PrimeArithmetic/Generated/Examples/WindowP101Base30Span29.lean`](PrimeArithmetic/Generated/Examples/WindowP101Base30Span29.lean),
[`PrimeArithmetic/Generated/Examples/WindowP163Base30Span35.lean`](PrimeArithmetic/Generated/Examples/WindowP163Base30Span35.lean),
and
[`PrimeArithmetic/Generated/Examples/WindowP41Base210Span5.lean`](PrimeArithmetic/Generated/Examples/WindowP41Base210Span5.lean),
keeping that path live inside the package build. The catalog regeneration path
is now also live via
[`scripts/lean_generated_catalog.sh`](../scripts/lean_generated_catalog.sh),
which can either rewrite the tracked examples or verify them against fresh
exporter output. The conservative analytic shell is now also live:
[`PrimeArithmetic/Analysis/HardyLittlewoodShell.lean`](PrimeArithmetic/Analysis/HardyLittlewoodShell.lean)
fixes pair-count conventions, odd-prime Goldbach local factors, radical
invariance of the local-factor support, and the standard logarithmic / coverage
transforms without asserting a new density theorem.

1. **Concrete first-step or first-byte runtime families, only if needed**:
   add more explicit short runtime-family lemmas only when a later executable
   agreement argument truly needs them. The generic grouped-plan and
   coordinate-shell layers are now already in place.
2. **Optional further catalog growth**:
   add more exported windows only when they support a concrete later argument,
   not just to increase file count.
3. **Exact Lagrange extraction, later**:
   revisit the Lagrange-facing code only by extracting connector or residue
   lemmas that can be restated in standard arithmetic terms.
4. **Verification and staging hygiene**:
   rerun the full Rust, Lean, and Agda verification surfaces and stage the
   accumulated work in a clean split.

Deferred hardening note:
- [`src/lagrange.rs`](../src/lagrange.rs) and the related gravity/tidal surface
  may still contain useful exact arithmetic signal, but any future Lean
  engagement there should begin by restating exact lemmas extracted from the
  code. Do not plan a Lean formalization of force-field, clustering, or
  equilibrium heuristics in their current metaphor-oriented form.

## Ledger

| Lean module | Repo source | Status | Depends on | Next step | OpenProver prompt |
|-------------|-------------|--------|------------|-----------|-------------------|
| `PrimeArithmetic/Foundation/FinitePairing` | `agda-proofs/Theorems/Abstract/SymmetryImpliesRepulsion.agda`, `agda-proofs/Theorems/Abstract/SymmetryFromList.agda` | active | mathlib core finite-type support | Keep the helper API minimal until a second certified symmetry example demands more structure | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Structure/AffineTemplate` | `agda-proofs/Theorems/AffineTransform.agda`, `historical/examples/affine_transform_verifier.rs`, older membrane affine-transform notes | proved in track 2a extension | mathlib core arithmetic | Reuse this when discussing fixed-layout evaluation or modular residue updates in the middle block, rather than treating the affine form as only an empirical trick | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Structure/AffineResidueSearch` | `PrimeArithmetic/Structure/AffineTemplate`, `agda-proofs/Theorems/AffineTransform.agda`, older residue-theory notes | proved in track 2a extension | `PrimeArithmetic/Structure/AffineTemplate`, `PrimeArithmetic/Density/UnitResidues` | Reuse this when the template should be treated as a modular search object: divisors of the base depend only on the outer digit, while coprime moduli induce affine residue permutations in the seed variable | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Structure/AffineSeedClasses` | `PrimeArithmetic/Structure/AffineResidueSearch`, `agda-proofs/Theorems/AffineTransform.agda`, older residue-theory notes | proved in track 2a extension | `PrimeArithmetic/Structure/AffineResidueSearch` | Reuse this when divisibility by a fixed coprime modulus should be expressed as a unique seed class modulo that modulus | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Symmetry/MidpointObstruction` | `agda-proofs/Theorems/Abstract/SymmetryImpliesRepulsion.agda` | proved in phase 1 | `PrimeArithmetic/Foundation/FinitePairing` | Stress the theorem against one more nontrivial witness before generalizing its interface | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Symmetry/Base6Example` | `agda-proofs/Examples/CertifiedResonanceComplete.agda` | proved in phase 1 | `PrimeArithmetic/Foundation/FinitePairing`, `PrimeArithmetic/Symmetry/MidpointObstruction` | Add a second concrete certified example only after the abstract symmetry API feels stable | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Symmetry/ModularReflection` | `agda-proofs/Theorems/Abstract/SymmetryFiniteReflect.agda` | proved in track 2 extension | `PrimeArithmetic/Foundation/FinitePairing` | Reuse this as the concrete `Fin base` reflection surface whenever the symmetry lane should be stated in modular arithmetic rather than only through ad hoc witnesses | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Symmetry/BalancedBucketSupport` | `agda-proofs/Theorems/Abstract/BucketsAutoMatch.agda` | proved support foundation | `PrimeArithmetic/Symmetry/ModularReflection` | Reuse these support-list, disjointness, and `zipPair` roundtrip lemmas as the stable combinatorial substrate for bucket-derived symmetry certificates | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Symmetry/BalancedBucketReflection` | `agda-proofs/Theorems/Abstract/BucketsAutoMatch.agda` | proved in track 2 extension | `PrimeArithmetic/Symmetry/BalancedBucketSupport`, `PrimeArithmetic/Symmetry/CertificateReflection` | Reuse this when balanced residue counts and support-count agreement should generate a reflection certificate automatically, without hand-written mate functions | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Symmetry/CertificateReflection` | `agda-proofs/Theorems/Abstract/SymmetryFiniteReflect.agda`, `agda-proofs/Theorems/Abstract/WindowCertificate.agda` | proved in track 2 extension | `PrimeArithmetic/Symmetry/ModularReflection`, `PrimeArithmetic/Symmetry/MidpointObstruction` | Reuse this smaller wrapper whenever a caller already has an explicit mate involution and only needs the abstract pairing witness | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Symmetry/WindowCertificate` | `agda-proofs/Theorems/Abstract/WindowCertificate.agda` | proved in track 2 extension | `PrimeArithmetic/Symmetry/BalancedBucketReflection` | Reuse this as the narrow static/dynamic shell for finite windows once residue buckets and midpoint-radius safety data have been extracted | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Symmetry/WindowCertificateGenerated` | `agda-proofs/Theorems/Abstract/WindowCertificate.agda`, `agda-proofs/Examples/CERTIFIED_RESONANCE_PARAM_DYN_BASE6_SKETCH.md` | proved in track 2 extension | `PrimeArithmetic/Symmetry/WindowCertificate`, `PrimeArithmetic/Symmetry/BalancedBucketSupport` | Reuse this when runtime or offline extraction produces residue lists and position lists directly; support counts are then derived automatically inside Lean | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Symmetry/WindowCertificateErgonomics` | `PrimeArithmetic/Symmetry/WindowCertificateGenerated`, `agda-proofs/Examples/CERTIFIED_RESONANCE_PARAM_DYN_BASE6_SKETCH.md` | proved in track 2 extension | `PrimeArithmetic/Symmetry/WindowCertificateGenerated` | Reuse this when generated artifacts should be passed to Lean as one compact static or dual evidence object rather than as a long argument list | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Symmetry/WindowCertificateExamples` | `agda-proofs/Examples/CERTIFIED_RESONANCE_PARAM_DYN_BASE6_SKETCH.md`, `agda-proofs/Examples/CertifiedResonanceParamDyn.agda` | proved in track 2 extension | `PrimeArithmetic/Symmetry/WindowCertificateGenerated` | Keep one or two maintained explicit finite examples live so the generated-data certificate APIs are exercised end to end on concrete data | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Generated/Examples/WindowP3Base6Span5` | `src/bin/export_window_certificate.rs`, runtime prime window around `2 * 3^2` | proved exported sample | `PrimeArithmetic/Symmetry/WindowCertificateErgonomics` | Keep the exporter path grounded on the smallest nontrivial even-base sample and reuse it when discussing catalog regeneration | `n/a` |
| `PrimeArithmetic/Generated/Examples/WindowP5Base10Span5` | `src/bin/export_window_certificate.rs`, runtime prime window around `2 * 5^2` | proved exported sample | `PrimeArithmetic/Symmetry/WindowCertificateErgonomics` | Keep one familiar decimal sample live in the tracked exported-artifact catalog | `n/a` |
| `PrimeArithmetic/Generated/Examples/WindowP5Base12Span17` | `src/bin/export_window_certificate.rs`, runtime prime window around `2 * 5^2` with base `12` residues | proved exported sample | `PrimeArithmetic/Symmetry/WindowCertificateErgonomics` | Keep at least one non-squarefree-base sample live so the exported-artifact catalog is not decimal-only | `n/a` |
| `PrimeArithmetic/Generated/Examples/WindowP11Base30Span5` | `src/bin/export_window_certificate.rs`, runtime prime window around `2 * 11^2` with base `30` residues | proved exported sample | `PrimeArithmetic/Symmetry/WindowCertificateErgonomics` | Keep a minimal wheel-like base-`30` reflected pair live in the catalog | `n/a` |
| `PrimeArithmetic/Generated/Examples/WindowP101Base30Span29` | `src/bin/export_window_certificate.rs`, runtime prime window around `2 * 101^2` with base `30` residues | proved exported sample | `PrimeArithmetic/Symmetry/WindowCertificateErgonomics` | Keep a medium-sized base-`30` sample live so the catalog exhibits more than a single reflected pair at wheel base `30` | `n/a` |
| `PrimeArithmetic/Generated/Examples/WindowP163Base30Span35` | `src/bin/export_window_certificate.rs`, runtime prime window around `2 * 163^2` with base `30` residues | proved exported sample | `PrimeArithmetic/Symmetry/WindowCertificateErgonomics` | Keep one slightly richer base-`30` sample live as the catalog's largest current wheel-like residue witness | `n/a` |
| `PrimeArithmetic/Generated/Examples/WindowP41Base210Span5` | `src/bin/export_window_certificate.rs`, runtime prime window around `2 * 41^2` with base `210` residues | proved exported sample | `PrimeArithmetic/Symmetry/WindowCertificateErgonomics` | Keep at least one larger wheel-like base beyond `30` live in the tracked exported-artifact catalog | `n/a` |
| `PrimeArithmetic/Symmetry/MirrorObstruction` | `agda-proofs/Theorems/MirrorObstruction.agda` | proved in track 2 extension | `PrimeArithmetic/Symmetry/ModularReflection`, `PrimeArithmetic/Symmetry/CertificateReflection` | Reuse this for mirror-indexed finite families before discussing any stronger structural interpretation of symmetric digit patterns | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Symmetry/UnitResidueComplementWitness` | `README.md`, `PrimeArithmetic/Density/UnitResidueSymmetry`, `agda-proofs/Theorems/Abstract/SymmetryImpliesRepulsion.agda` | proved in track 2 extension | `PrimeArithmetic/Symmetry/MidpointObstruction`, `PrimeArithmetic/Density/UnitResidueSymmetry` | Reuse this as the generic second symmetry witness before adding more bespoke certified examples | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Symmetry/ZModUnitNegationWitness` | `README.md`, `PrimeArithmetic/Density/ZModUnitNegation`, `PrimeArithmetic/Symmetry/UnitResidueComplementWitness` | proved in track 2 extension | `PrimeArithmetic/Symmetry/MidpointObstruction`, `PrimeArithmetic/Density/ZModUnitNegation`, `PrimeArithmetic/Symmetry/UnitResidueComplementWitness` | Use this when the abstract midpoint obstruction should be exhibited directly from negation on `(ZMod n)ˣ` rather than only through natural residue representatives | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Density/CoprimeFilter` | `agda-proofs/Examples/Base10ResidueFilter.agda`, `agda-proofs/Theorems/CoprimalityRequirement.agda` | proved in track 2 foundation | Track 1 green locally and in CI | Keep this as the entry lemma for `prime > base` arguments in later density modules | `collab/openprover/theorems/01-base10-prime-filter.md` |
| `PrimeArithmetic/Density/RadicalFilter` | `agda-proofs/Core/Radical.agda`, `agda-proofs/Theorems/RadicalDivisibilityFilter.agda` | proved in track 2 extension | `PrimeArithmetic/Density/CoprimeFilter` | Reuse this exact `rad(base)` layer before any broader density bookkeeping | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/UnitResidues` | `agda-proofs/Examples/Base10ResidueFilter.agda`, `agda-proofs/Core/Radical.agda`, `README.md` | proved in track 2 extension | `PrimeArithmetic/Density/CoprimeFilter`, `PrimeArithmetic/Density/RadicalFilter` | Grow future base examples by instantiating the generic unit-residue layer rather than reproving arithmetic facts | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/UnitResidueSymmetry` | `README.md`, `agda-proofs/Theorems/Abstract/SymmetryImpliesRepulsion.agda` | proved in track 2 extension | `PrimeArithmetic/Density/UnitResidues` | Use this as the symmetry-density bridge before adding more midpoint or complement-pair arguments | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Density/SquarefreeBases` | `agda-proofs/Core/Radical.agda`, `README.md`, `AGENTS.md` | proved in track 2 extension | `PrimeArithmetic/Density/RadicalFilter`, `PrimeArithmetic/Density/UnitResidues` | Reuse this simplification layer whenever a base is squarefree instead of carrying `rad(base)` explicitly | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/ZModUnits` | `README.md`, `agda-proofs/Examples/Base10ResidueFilter.agda` | proved in track 2 extension | `PrimeArithmetic/Density/UnitResidues` | Use this as the standard-library algebra bridge before any future `ZMod` or CRT-facing density lemmas | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/ZModUnitNegation` | `README.md`, `PrimeArithmetic/Density/ZModUnits`, `PrimeArithmetic/Symmetry/UnitResidueComplementWitness` | proved in track 2 extension | `PrimeArithmetic/Density/ZModUnits`, `PrimeArithmetic/Density/UnitResidues` | Use this as the standard modular statement of complement-pair symmetry, even cardinality, and midpoint exclusion before adding broader unit-group lemmas | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Density/ZModUnitCRT` | `README.md`, `PrimeArithmetic/Density/CoprimeBaseProducts`, `PrimeArithmetic/Density/WheelBases` | proved in track 2 extension | `PrimeArithmetic/Density/ZModUnits`, `PrimeArithmetic/Density/CoprimeBaseProducts` | Use this as the explicit unit-group CRT surface before adding larger multi-factor unit decompositions | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/WheelUnitCRT` | `README.md`, `PrimeArithmetic/Density/WheelBases`, `PrimeArithmetic/Density/ZModUnitCRT` | proved in track 2 extension | `PrimeArithmetic/Density/WheelBases`, `PrimeArithmetic/Density/ZModUnitCRT` | Reuse this family-level wheel-base unit decomposition before hand-writing larger prime-product unit splits | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/CoprimeBaseProducts` | `README.md`, `agda-proofs/Core/Radical.agda` | proved in track 2 extension | `PrimeArithmetic/Density/RadicalFilter`, `PrimeArithmetic/Density/ZModUnits` | Reuse this product structure before any higher-level CRT or multi-factor base analysis | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/WheelBases` | `README.md`, `AGENTS.md`, `agda-proofs/Core/Radical.agda` | proved in track 2 extension | `PrimeArithmetic/Density/SquarefreeBases`, `PrimeArithmetic/Density/UnitResidues` | Use this as the generic base-family layer before adding bigger squarefree or primorial-like examples | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/WheelResidueClassifier` | `README.md`, `AGENTS.md`, `agda-proofs/Examples/Base10ResidueFilter.agda` | proved in track 2 extension | `PrimeArithmetic/Density/WheelBases`, `PrimeArithmetic/Density/UnitResidues` | Reuse this local-to-global CRT classifier before hand-writing larger concrete residue sets | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/UnitResiduePairs` | `README.md`, `PrimeArithmetic/Density/UnitResidueSymmetry`, `PrimeArithmetic/Density/ZModUnitNegation` | proved in track 2 extension | `PrimeArithmetic/Density/UnitResidueSymmetry`, `PrimeArithmetic/Density/ZModUnits` | Use this when the finite complement-pair partition itself, not only even cardinality, needs to be stated explicitly | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Density/ZModUnitOrbits` | `README.md`, `PrimeArithmetic/Density/ZModUnitNegation`, `PrimeArithmetic/Density/UnitResiduePairs` | proved in track 2 extension | `PrimeArithmetic/Density/ZModUnitNegation`, `PrimeArithmetic/Density/UnitResiduePairs` | Use this when the quotient by negation-orbits on `(ZMod n)ˣ` is the natural statement, rather than only a representative set on natural residues | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Density/Base6Residues` | `agda-proofs/Examples/CertifiedResonanceComplete.agda`, `README.md` | proved concrete base module | `PrimeArithmetic/Density/UnitResidues` | Keep base 6 aligned with the certified symmetry example while the residue API stays small | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/Base10Residues` | `agda-proofs/Examples/Base10ResidueFilter.agda` | proved concrete base module | `PrimeArithmetic/Density/UnitResidues` | Use this as the most familiar concrete entrypoint for the cross-base residue story | `collab/openprover/theorems/01-base10-prime-filter.md` |
| `PrimeArithmetic/Density/Base12Residues` | `README.md`, `AGENTS.md` | proved concrete base module | `PrimeArithmetic/Density/UnitResidues`, `PrimeArithmetic/Density/RadicalFilter` | Emphasize the `rad(base)` versus `φ(base)` distinction in a non-squarefree base | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/Base30Residues` | `README.md`, `AGENTS.md` | proved concrete base module | `PrimeArithmetic/Density/UnitResidues` | Use base 30 as the wheel-like concrete example before any Hardy-Littlewood formalization | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/WheelUnitProductEquiv` | `PrimeArithmetic/Density/WheelUnitCRT`, `PrimeArithmetic/Density/ZModUnitCRT`, `PrimeArithmetic/Density/WheelBases` | proved in track 2 extension | `PrimeArithmetic/Density/WheelUnitCRT`, `PrimeArithmetic/Density/ZModUnitCRT` | Use this when the wheel-base CRT theorem should be stated in canonical finite-family notation `∀ p ∈ S, (ZMod p)ˣ` rather than only through recursive tuples | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Density/ZModUnitAction` | `PrimeArithmetic/Density/ZModUnitNegation`, `PrimeArithmetic/Density/ZModUnitOrbits`, `PrimeArithmetic/Density/UnitResiduePairs` | proved in track 2 extension | `PrimeArithmetic/Density/ZModUnitNegation`, `PrimeArithmetic/Density/ZModUnitOrbits` | Use this when the `φ(n) / 2` quotient should be stated in group-action language via the order-two subgroup `{1, -1}` acting on `(ZMod n)ˣ` | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
| `PrimeArithmetic/Sieve/SegmentedSieve` | `src/prime_sieve.rs` | proved in track 2b extension | mathlib core divisibility/arithmetic support | Reuse this when the odd-only candidate domain, inverse segment indexing, first odd marked multiple, or `2p` marking step should be treated as exact arithmetic rather than implementation detail | `n/a` |
| `PrimeArithmetic/Sieve/SegmentLayout` | `src/prime_sieve.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/SegmentedSieve` | Reuse this when the runtime constants `SEG_BYTES`, `SEG_BITS`, `SEG_ODDS`, or the raw segment upper bound should be related back to the odd-candidate arithmetic surface | `n/a` |
| `PrimeArithmetic/Sieve/RuntimeCrossOff` | `src/prime_sieve.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/SegmentedSieve` | Reuse this when the actual runtime cross-off start branch (`p^2` versus ceiling multiple), odd-start correction, or `2 * p` progression should be stated exactly rather than only described informally | `n/a` |
| `PrimeArithmetic/Sieve/RuntimeCollection` | `src/prime_sieve.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/SegmentLayout`, `PrimeArithmetic/Sieve/SegmentedSieve` | Reuse this when the adjusted odd segment endpoint or the exact collection index for an odd candidate in the runtime interval should be stated explicitly | `n/a` |
| `PrimeArithmetic/Sieve/SegmentBitCoordinates` | `src/prime_sieve.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/SegmentLayout`, `PrimeArithmetic/Sieve/SegmentedSieve` | Reuse this when the shared byte/bit coordinates behind `mark_composite` and `is_prime` should be stated exactly, or when those coordinates need to reconstruct the odd index and candidate | `n/a` |
| `PrimeArithmetic/Sieve/SegmentBitMasks` | `src/prime_sieve.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/SegmentBitCoordinates` | Reuse this when the executable odd-only mask `1 << bit` and readback `((byte >> bit) & 1)` should be stated exactly, or when the proof needs the fact that the writer sets the same bit the reader tests | `n/a` |
| `PrimeArithmetic/Sieve/BoundedByteFamilies` | `PrimeArithmetic/Sieve/SegmentBitMasks`, `PrimeArithmetic/Sieve/Wheel30BitMasks` | proved in track 2b extension | `PrimeArithmetic/Sieve/SegmentBitMasks` | Reuse this when a finite bounded byte family should be updated by a disjoint-slot list of marks, and every marked slot should read back as `1` afterward | `n/a` |
| `PrimeArithmetic/Sieve/BoundedByteMasks` | `PrimeArithmetic/Sieve/BoundedByteFamilies`, `src/prime_sieve.rs`, `src/prime_sieve/wheel30.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/BoundedByteFamilies` | Reuse this when repeated writes in one byte should be collapsed into a single OR-mask update, or when same-byte collisions should be stated more cleanly than as a long single-bit trace | `n/a` |
| `PrimeArithmetic/Sieve/BoundedBytePlans` | `PrimeArithmetic/Sieve/BoundedByteFamilies`, `PrimeArithmetic/Sieve/BoundedByteMasks`, `src/prime_sieve.rs`, `src/prime_sieve/wheel30.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/BoundedByteMasks` | Reuse this when a runtime or offline marking family is most naturally described as a small set of touched bytes, each with its own bit list, rather than as a raw flattened mark trace | `n/a` |
| `PrimeArithmetic/Sieve/BoundedByteCoordinates` | `PrimeArithmetic/Sieve/BoundedBytePlans`, `src/prime_sieve.rs`, `src/prime_sieve/wheel30.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/BoundedBytePlans` | Reuse this when a local runtime coordinate shell should be bridged into one fixed `ByteMark`, or when grouped coordinate plans should reuse the generic grouped-plan theorem family without repeating boilerplate | `n/a` |
| `PrimeArithmetic/Sieve/SegmentByteArray` | `src/prime_sieve.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/BoundedByteCoordinates`, `PrimeArithmetic/Sieve/SegmentBitMasks`, `PrimeArithmetic/Sieve/SegmentLayout` | Reuse this when the odd-only sieve should be treated as a bounded one-byte array update or bridged into the generic bounded byte-family surface through the shared coordinate shell | `n/a` |
| `PrimeArithmetic/Sieve/SegmentRuntimePlans` | `src/prime_sieve.rs`, `PrimeArithmetic/Sieve/RuntimeCrossOff` | proved in track 2b extension | `PrimeArithmetic/Sieve/BoundedByteCoordinates`, `PrimeArithmetic/Sieve/SegmentByteArray`, `PrimeArithmetic/Sieve/RuntimeCrossOff` | Reuse this when short executable odd-only mark families, especially those built from `runtimeMarkedBy`, should be bucketed by byte slot and discharged directly on the grouped-plan layer | `n/a` |
| `PrimeArithmetic/Sieve/SegmentRuntimeSteps` | `src/prime_sieve.rs`, `PrimeArithmetic/Sieve/RuntimeCrossOff` | proved in track 2b extension | `PrimeArithmetic/Sieve/SegmentRuntimePlans` | Reuse this when executable odd-only mark families are still most naturally expressed as bounded `runtimeMarkedBy` step values rather than already rewrapped candidate coordinates | `n/a` |
| `PrimeArithmetic/Sieve/Wheel30Residues` | `src/prime_sieve/wheel30.rs` | proved in track 2b extension | `PrimeArithmetic/Density/Base30Residues`, `PrimeArithmetic/Density/UnitResidues` | Reuse this as the exact wheel30 admissible-residue surface instead of hand-writing the surviving classes modulo `30` | `n/a` |
| `PrimeArithmetic/Sieve/Wheel30Agreement` | `src/prime_sieve.rs`, `src/prime_sieve/wheel30.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/SegmentedSieve`, `PrimeArithmetic/Sieve/Wheel30Residues` | Reuse this when wheel30 candidate encodings should be related back to the filtered odd candidate domain without discussing performance or bit-array layout | `n/a` |
| `PrimeArithmetic/Sieve/Wheel30Indexing` | `src/prime_sieve/wheel30.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/Wheel30Agreement` | Reuse this when the runtime wheel30 slot order, linear index `cycle * 8 + slot`, or byte/bit decomposition should be stated exactly rather than described informally | `n/a` |
| `PrimeArithmetic/Sieve/Wheel30BitCoordinates` | `src/prime_sieve/wheel30.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/Wheel30Indexing` | Reuse this when the shared writer/reader byte/bit coordinates for a wheel30 candidate should be stated exactly, or when those coordinates need to be packaged as `some (cycle, slot)` | `n/a` |
| `PrimeArithmetic/Sieve/Wheel30BitMasks` | `src/prime_sieve/wheel30.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/Wheel30BitCoordinates` | Reuse this when the executable wheel30 mask `1 << bit` and readback `((byte >> bit) & 1)` should be stated exactly, or when candidate-level corollaries are needed for the runtime slot order | `n/a` |
| `PrimeArithmetic/Sieve/Wheel30ByteArray` | `src/prime_sieve/wheel30.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/BoundedByteCoordinates`, `PrimeArithmetic/Sieve/Wheel30BitMasks` | Reuse this when the wheel30 sieve should be treated as a bounded one-byte array update in the runtime `(cycle, slot)` coordinates, or bridged into the generic bounded byte-family surface through the shared coordinate shell | `n/a` |
| `PrimeArithmetic/Sieve/Wheel30RuntimePlans` | `src/prime_sieve/wheel30.rs` | proved in track 2b extension | `PrimeArithmetic/Sieve/BoundedByteCoordinates`, `PrimeArithmetic/Sieve/Wheel30ByteArray` | Reuse this when short executable wheel30 mark families should be bucketed by cycle byte slot and discharged directly on the grouped-plan layer | `n/a` |
| `PrimeArithmetic/Connector/ConcatenationFilters` | `src/connector/mod.rs`, `src/connector/arithmetic.rs`, `src/connector/utils.rs`, `agda-proofs/LagrangePoints/ResidueField.agda`, `agda-proofs/LagrangePoints/ZeroPaddedPrimes/Asymmetry.agda` | proved in track 2a extension | `PrimeArithmetic/Structure/AffineTemplate` | Reuse this when connector scans should be stated as exact fixed-width arithmetic: forward and reverse concatenations reduce to the same residue sum when `base ≡ 1 (mod m)`, and the canonical decimal pair has exact `mod 3` / `mod 9` exclusion classes | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Connector/ConcatenationFamilies` | `PrimeArithmetic/Connector/ConcatenationFilters`, `src/connector/arithmetic.rs`, `src/connector/utils.rs` | proved in track 2a extension | `PrimeArithmetic/Connector/ConcatenationFilters` | Reuse this when a whole fixed-width connector family has a known left/right residue profile modulo `m`, so admissibility reduces to one generic connector class theorem rather than a bespoke proof per pair | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Connector/ConcatenationProfileExamples` | `examples/lagrange_tui_demo.rs`, `PrimeArithmetic/Connector/ConcatenationFamilies` | proved in track 2a extension | `PrimeArithmetic/Connector/ConcatenationFamilies` | Reuse this as the maintained client layer showing that the generic connector-family API applies cleanly to non-canonical preset pairs already present in the repo | `collab/openprover/theorems/03-base-radical-prime-filter.md` |
| `PrimeArithmetic/Analysis/HardyLittlewoodShell` | `src/hzlib/hardy_littlewood.rs`, `agda-proofs/Theorems/HardyLittlewoodSingularSeries.agda` | proved conservative shell | `PrimeArithmetic/Density/WheelUnitProductEquiv`, `PrimeArithmetic/Density/ZModUnitAction` | Reuse this when pair-count conventions, odd-prime local-factor bookkeeping, or the standard logarithmic / coverage transforms should be stated in Lean without overclaiming a new density theorem | `collab/openprover/theorems/04-diameter-density-reduction.md` |
| `n/a (OpenProver symmetry companion)` | `collab/openprover/README.md`, `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` | queued for track 3 | `PrimeArithmetic/Symmetry/MidpointObstruction`, `PrimeArithmetic/Symmetry/Base6Example` | Run assisted proof-search sessions against the in-repo Lean package after `lake build` and CI are green | `collab/openprover/theorems/02-symmetry-midpoint-obstruction.md` |
