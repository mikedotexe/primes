# Lean 4 Proof Workspace

**Updated**: March 2026

This directory is the in-repo Lean 4 package for the active mathematical
formalization program.

Current scope:

- the package is mathlib-backed and self-contained
- the initial theorem family is the symmetry certification lane
- the symmetry lane now includes a concrete modular reflection layer on `Fin base`,
  a constructive balanced-bucket support layer for residue-index lists,
  a balanced-bucket reflection wrapper, a narrow window-certificate shell,
  a generated-data entrypoint for residue and position lists, a bundled
  proof-object layer for generated artifacts, explicit finite certificate
  examples, a reusable reflection-certificate wrapper, and a mirror-family
  theorem stated in standard finite-symmetry language
- the package now also has a template-structure lane: exact affine dependence
  on the middle block for fixed symmetric layouts, plus its modular search
  consequences, and now a local affine lane-comparison surface relating shift /
  gradient / zero-seed equality to the four report-facing local relation labels
- the package now also has a bounded-`k` transfer-collapse lane: exact
  compactness data, exact finite transfer-bucket combinatorics, local
  coprime-modulus residue profiles for direct lane comparisons, a universal
  conditional criterion saying profile agreement forces transfer identity, and
  parallel `2p`-like / wheel-like theorem wrappers together with tracked
  generated direct-lane witnesses
- the package now also has a connector-arithmetic lane: exact fixed-width
  forward/reverse concatenation formulas and canonical decimal `mod 3` / `mod 9`
  exclusion filters for connector values, together with a reusable residue-class
  profile API for connector families and maintained profile examples beyond the
  single canonical pair
- the package now also has a prime-engine correctness lane: exact arithmetic for
  odd-only segmented sieve candidates, first odd marked multiples, the wheel30
  admissible residues, and agreement between the wheel30 candidate
  representation and the filtered odd candidate domain
- the symmetry lane now also has a runtime-export path: a Rust exporter emits
  Lean-shaped window-certificate artifacts from actual prime windows, and the
  package now keeps a small tracked generated-example catalog live across more
  than one base
- the density-facing lane is restricted to exact arithmetic facts:
  coprimality, `rad(base)`, unit residues, a `ZMod`-units bridge,
  the standard negation involution on `ZMod` units,
  explicit CRT equivalences on unit groups,
  a canonical finite-family wheel-base CRT theorem on unit groups,
  squarefree-base simplifications, coprime-base product structure, finite
  wheel-base families, a finite-product wheel-unit CRT decomposition,
  finite-CRT residue classification, complement symmetry on admissible
  residues, explicit `φ(base) / 2` pair representatives and unit-negation
  orbit quotients, the same quotient restated as the orbit space of the
  order-two subgroup `{1, -1}`, a direct `ZMod`-unit witness back into the
  abstract midpoint-obstruction theorem, and concrete base modules

## Toolchain

- Lean: `v4.28.0`
- mathlib: `v4.28.0`

The package is pinned in:

- `lean-toolchain`
- `lakefile.lean`

## Canonical Commands

```bash
cd lean-proofs

# Download mathlib build artifacts when available
lake exe cache get

# Build the whole package
lake build

# Check the umbrella module directly
lake env lean PrimeArithmetic.lean

# Check the concrete certified example
lake env lean PrimeArithmetic/Symmetry/Base6Example.lean

# Check the generic unit-residue complement witness back into the symmetry lane
lake env lean PrimeArithmetic/Symmetry/UnitResidueComplementWitness.lean

# Check the concrete modular reflection layer on `Fin base`
lake env lean PrimeArithmetic/Symmetry/ModularReflection.lean

# Check the reusable reflection-certificate wrapper
lake env lean PrimeArithmetic/Symmetry/CertificateReflection.lean

# Check the constructive support-bucket substrate for later auto-pairing
lake env lean PrimeArithmetic/Symmetry/BalancedBucketSupport.lean

# Check the balanced-bucket reflection wrapper
lake env lean PrimeArithmetic/Symmetry/BalancedBucketReflection.lean

# Check the mirror-family obstruction theorem
lake env lean PrimeArithmetic/Symmetry/MirrorObstruction.lean

# Check the narrow window-certificate shell
lake env lean PrimeArithmetic/Symmetry/WindowCertificate.lean

# Check the generated-data entrypoint for residue and position lists
lake env lean PrimeArithmetic/Symmetry/WindowCertificateGenerated.lean

# Check the bundled proof-object layer for generated window artifacts
lake env lean PrimeArithmetic/Symmetry/WindowCertificateErgonomics.lean

# Check explicit finite certificate examples
lake env lean PrimeArithmetic/Symmetry/WindowCertificateExamples.lean

# Check the tracked runtime-exported certificate artifact
lake env lean PrimeArithmetic/Generated/Examples/WindowP3Base6Span5.lean

# Check the tracked runtime-exported decimal certificate artifact
lake env lean PrimeArithmetic/Generated/Examples/WindowP5Base10Span5.lean

# Check the tracked runtime-exported base-12 certificate artifact
lake env lean PrimeArithmetic/Generated/Examples/WindowP5Base12Span17.lean

# Check the first density-facing prerequisite
lake env lean PrimeArithmetic/Density/CoprimeFilter.lean

# Check the exact radical reformulation
lake env lean PrimeArithmetic/Density/RadicalFilter.lean

# Check the base-invariant unit-residue layer
lake env lean PrimeArithmetic/Density/UnitResidues.lean

# Check the complement-pair symmetry layer on unit residues
lake env lean PrimeArithmetic/Density/UnitResidueSymmetry.lean

# Check the standard-library `ZMod` units bridge
lake env lean PrimeArithmetic/Density/ZModUnits.lean

# Check the standard negation-symmetry statement on `ZMod` units
lake env lean PrimeArithmetic/Density/ZModUnitNegation.lean

# Check the explicit CRT theorem on unit groups
lake env lean PrimeArithmetic/Density/ZModUnitCRT.lean

# Check the affine template theorem for fixed symmetric layouts
lake env lean PrimeArithmetic/Structure/AffineTemplate.lean

# Check the modular search consequences of the affine template theorem
lake env lean PrimeArithmetic/Structure/AffineResidueSearch.lean

# Check the explicit seed classes induced by the affine residue map
lake env lean PrimeArithmetic/Structure/AffineSeedClasses.lean

# Check the local affine lane-comparison vocabulary
lake env lean PrimeArithmetic/Structure/AffineLaneComparison.lean

# Check the local affine period-lock criterion
lake env lean PrimeArithmetic/Structure/AffinePeriodLock.lean

# Check the bounded-k specialization of the period-lock criterion
lake env lean PrimeArithmetic/Structure/BoundedKPeriodLock.lean

# Check the concrete base-22 / mod-5 affine side-pocket theorem
lake env lean PrimeArithmetic/Density/Base22GradientPocket.lean

# Check the exact bounded-k compactness layer
lake env lean PrimeArithmetic/Structure/BoundedKTemplate.lean

# Check the exact finite mask-transfer combinatorics
lake env lean PrimeArithmetic/Structure/FiniteMaskTransfer.lean

# Check the direct lane-comparison residue-profile layer
lake env lean PrimeArithmetic/Structure/BoundedKResidueProfile.lean

# Check the universal transfer-collapse criterion
lake env lean PrimeArithmetic/Structure/BoundedKTransferCollapse.lean

# Check the generated direct-lane residue-profile shell
lake env lean PrimeArithmetic/Generated/BoundedKResidueProfileShell.lean

# Check the even-squarefree 2p-style wrapper family
lake env lean PrimeArithmetic/Density/BoundedKTwoPrimeCollapse.lean

# Check the wheel-base wrapper family
lake env lean PrimeArithmetic/Density/BoundedKWheelCollapse.lean

# Check tracked direct-lane counterexample witnesses
lake env lean PrimeArithmetic/Generated/BoundedKProfiles/Base6M3Pair15K00ToK01.lean
lake env lean PrimeArithmetic/Generated/BoundedKProfiles/Base30M3Pair11K00ToK01.lean

# Check the odd-only segmented sieve arithmetic layer
lake env lean PrimeArithmetic/Sieve/SegmentedSieve.lean

# Check the runtime odd-segment capacity and bound formulas
lake env lean PrimeArithmetic/Sieve/SegmentLayout.lean

# Check the runtime cross-off start and `2p` progression
lake env lean PrimeArithmetic/Sieve/RuntimeCrossOff.lean

# Check the runtime odd-upper-bound adjustment and collection indexing
lake env lean PrimeArithmetic/Sieve/RuntimeCollection.lean

# Check byte/bit coordinates for the odd-only segment writer and reader
lake env lean PrimeArithmetic/Sieve/SegmentBitCoordinates.lean

# Check bit-mask update/readback semantics for the odd-only segment
lake env lean PrimeArithmetic/Sieve/SegmentBitMasks.lean

# Check generic bounded multi-mark families on disjoint byte slots
lake env lean PrimeArithmetic/Sieve/BoundedByteFamilies.lean

# Check aggregated same-byte mask families
lake env lean PrimeArithmetic/Sieve/BoundedByteMasks.lean

# Check grouped multi-byte plan families
lake env lean PrimeArithmetic/Sieve/BoundedBytePlans.lean

# Check the tiny shared coordinate-to-byte-plan bridge
lake env lean PrimeArithmetic/Sieve/BoundedByteCoordinates.lean

# Check single-byte array update/readback semantics for the odd-only segment
lake env lean PrimeArithmetic/Sieve/SegmentByteArray.lean

# Check short odd-only runtime mark families on grouped plans
lake env lean PrimeArithmetic/Sieve/SegmentRuntimePlans.lean

# Check bounded runtime cross-off step families on grouped plans
lake env lean PrimeArithmetic/Sieve/SegmentRuntimeSteps.lean

# Check the exact wheel30 admissible-residue surface
lake env lean PrimeArithmetic/Sieve/Wheel30Residues.lean

# Check agreement between the wheel30 representation and the filtered odd domain
lake env lean PrimeArithmetic/Sieve/Wheel30Agreement.lean

# Check the runtime wheel30 linear index / byte / bit layout
lake env lean PrimeArithmetic/Sieve/Wheel30Indexing.lean

# Check byte/bit coordinates for the wheel30 segment writer and reader
lake env lean PrimeArithmetic/Sieve/Wheel30BitCoordinates.lean

# Check bit-mask update/readback semantics for the wheel30 segment
lake env lean PrimeArithmetic/Sieve/Wheel30BitMasks.lean

# Check single-byte array update/readback semantics for the wheel30 segment
lake env lean PrimeArithmetic/Sieve/Wheel30ByteArray.lean

# Check short wheel30 runtime mark families on grouped plans
lake env lean PrimeArithmetic/Sieve/Wheel30RuntimePlans.lean

# Check base-invariant rebasing for grouped wheel30 runtime families
lake env lean PrimeArithmetic/Sieve/Wheel30RuntimeRebase.lean

# Check the fixed-width connector arithmetic and canonical decimal filters
lake env lean PrimeArithmetic/Connector/ConcatenationFilters.lean

# Check the reusable connector-family residue profile layer
lake env lean PrimeArithmetic/Connector/ConcatenationFamilies.lean

# Check maintained profile examples beyond the canonical pair
lake env lean PrimeArithmetic/Connector/ConcatenationProfileExamples.lean

# Check the conservative Hardy-Littlewood notation and local-factor shell
lake env lean PrimeArithmetic/Analysis/HardyLittlewoodShell.lean

# Check the finite-product wheel-base CRT theorem on unit groups
lake env lean PrimeArithmetic/Density/WheelUnitCRT.lean

# Check the canonical finite-family wheel-base CRT theorem on unit groups
lake env lean PrimeArithmetic/Density/WheelUnitProductEquiv.lean

# Check the squarefree-base simplification layer
lake env lean PrimeArithmetic/Density/SquarefreeBases.lean

# Check the coprime-base product / CRT layer
lake env lean PrimeArithmetic/Density/CoprimeBaseProducts.lean

# Check the generic finite-prime-product / wheel-base layer
lake env lean PrimeArithmetic/Density/WheelBases.lean

# Check the finite-CRT classifier on wheel bases
lake env lean PrimeArithmetic/Density/WheelResidueClassifier.lean

# Check the explicit complement-pair representative layer
lake env lean PrimeArithmetic/Density/UnitResiduePairs.lean

# Check the orbit quotient for negation on `(ZMod base)ˣ`
lake env lean PrimeArithmetic/Density/ZModUnitOrbits.lean

# Check the same quotient in group-action form
lake env lean PrimeArithmetic/Density/ZModUnitAction.lean

# Check the direct `ZMod`-unit witness back into midpoint obstruction
lake env lean PrimeArithmetic/Symmetry/ZModUnitNegationWitness.lean

# Check one concrete cross-base residue module
lake env lean PrimeArithmetic/Density/Base12Residues.lean

# Check concrete odd/even composite residue modules
lake env lean PrimeArithmetic/Density/Base14Residues.lean
lake env lean PrimeArithmetic/Density/Base15Residues.lean
lake env lean PrimeArithmetic/Density/Base18Residues.lean
lake env lean PrimeArithmetic/Density/Base22Residues.lean
```

From the repo root, emit a Lean-shaped certificate artifact directly from a
runtime prime window:

```bash
cargo run --bin export_window_certificate -- \
  --p 5 \
  --base 10 \
  --window-span 5 \
  --exclude-radius 1 \
  --out lean-proofs/PrimeArithmetic/Generated/Examples/WindowP5Base10Span5.lean
```

From the repo root, verify or regenerate the whole tracked exported-artifact
catalog:

```bash
./scripts/lean_generated_catalog.sh verify
./scripts/lean_generated_catalog.sh regenerate
```

## Package Layout

- `PrimeArithmetic.lean`: umbrella import for the current Lean surface
- `PrimeArithmetic/Foundation/FinitePairing.lean`: generic involution and pairing
  scaffolding
- `PrimeArithmetic/Structure/AffineTemplate.lean`: exact affine form of the
  symmetric template in the middle block
- `PrimeArithmetic/Structure/AffineResidueSearch.lean`: modular consequences of
  the affine form, including base-divisor rigidity and affine residue
  permutations for coprime moduli
- `PrimeArithmetic/Structure/AffineSeedClasses.lean`: explicit seed classes for
  target residues, including the unique zero-residue class modulo a coprime
  modulus
- `PrimeArithmetic/Structure/AffinePeriodLock.lean`: exact local period-lock
  criterion showing that affine gradient equality is controlled by the
  multiplicative order of the base unit, with locked and unlocked corollaries
  for the four local relation labels
- `PrimeArithmetic/Structure/BoundedKPeriodLock.lean`: bounded-`k`
  specialization of the period-lock criterion, rewriting the local theorem in
  the lane-grid vocabulary and proving the `k = (0, 0) → (kOuter, kInner)`
  lock condition `kOuter + kInner ≡ 0` modulo the base-unit order
- `PrimeArithmetic/Density/Base22GradientPocket.lean`: concrete base-22,
  modulus-5 affine lane comparison for `M = 2`, proving the exact
  `identity`/`gradientOnly` split for `k = (0, 0)` vs `k = (2, 2)`
- `PrimeArithmetic/Sieve/SegmentedSieve.lean`: odd-only segmented sieve
  candidate encoding, inverse indexing, first odd multiple arithmetic, and
  divisibility/oddness invariants for the marking progression
- `PrimeArithmetic/Sieve/SegmentLayout.lean`: runtime segment constants,
  arithmetic span, raw upper bound, and the fact that in-range odd candidates
  map to indices below the segment capacity
- `PrimeArithmetic/Sieve/RuntimeCrossOff.lean`: runtime cross-off start logic,
  including the `p^2` branch, the odd-start correction, and the exact `2 * p`
  marking progression
- `PrimeArithmetic/Sieve/RuntimeCollection.lean`: runtime odd-upper-bound
  adjustment and the existence of the exact collection index for every odd
  candidate in the adjusted segment interval
- `PrimeArithmetic/Sieve/SegmentBitCoordinates.lean`: shared byte/bit
  coordinates for the odd-only `mark_composite` and `is_prime` formulas,
  including reconstruction of the segment index from byte and bit
- `PrimeArithmetic/Sieve/SegmentBitMasks.lean`: exact odd-only mask/update
  semantics for `1 << bit` and `((byte >> bit) & 1)`, including the fact that
  the writer sets the same bit the reader tests and preserves all other bits
- `PrimeArithmetic/Sieve/BoundedByteFamilies.lean`: generic bounded byte-array
  families for sieve-style bitsets, covering one-mark correctness, preservation
  under writes to other byte slots, and finite disjoint-slot multi-mark writes
- `PrimeArithmetic/Sieve/BoundedByteMasks.lean`: aggregated same-byte mask
  families for sieve-style bitsets, collapsing repeated writes in one byte into
  a single OR-mask update and proving readback for every listed target bit
- `PrimeArithmetic/Sieve/BoundedBytePlans.lean`: grouped multi-byte plan
  families for sieve-style bitsets, combining per-byte aggregated masks into a
  bounded plan list with exact flattening to single-bit traces and exact
  readback for every planned bit under pairwise distinct byte slots
- `PrimeArithmetic/Sieve/BoundedByteCoordinates.lean`: tiny shared bridge from
  local coordinate shells into grouped byte plans, covering fixed-mark
  read-written proofs and grouped coordinate-plan readback under aligned,
  distinct byte slots
- `PrimeArithmetic/Sieve/SegmentByteArray.lean`: bounded single-byte array
  semantics for the odd-only runtime segment, showing that the selected byte
  slot is updated exactly once, readback at the selected slot returns `1`, and
  the runtime shell is bridged into the generic bounded byte-family API through
  the shared coordinate shell
- `PrimeArithmetic/Sieve/SegmentRuntimePlans.lean`: short odd-only runtime mark
  families stated directly in segment coordinates and packaged on the grouped
  byte-plan layer
- `PrimeArithmetic/Sieve/SegmentRuntimeSteps.lean`: bounded odd-only
  `runtimeMarkedBy` step families, including canonical per-byte bucketing,
  packaged directly on the grouped byte-plan layer without hand-rewrapping
  each step as a segment coordinate
- `PrimeArithmetic/Sieve/Wheel30Residues.lean`: exact wheel30 residue set and
  its characterization as the admissible residues modulo `30`
- `PrimeArithmetic/Sieve/Wheel30Agreement.lean`: equivalence between wheel30
  representability and the odd candidate domain with the `mod 3` / `mod 5`
  filters, plus the exact base-30 CRT roundtrip back to the canonical
  `mod 30` representative
- `PrimeArithmetic/Sieve/Wheel30Indexing.lean`: runtime wheel30 slot order,
  linear index formula `cycle * 8 + slot`, and the corresponding byte / bit
  coordinates used by the wheel30 bit array
- `PrimeArithmetic/Sieve/Wheel30BitCoordinates.lean`: shared byte/bit
  coordinates for the wheel30 writer and reader, together with the candidate
  formula that recovers `some (cycle, slot)` directly from the runtime index
- `PrimeArithmetic/Sieve/Wheel30BitMasks.lean`: exact wheel30 mask/update
  semantics for the runtime `1 << bit` writer and `((byte >> bit) & 1)` reader,
  both generically from `some bit` coordinates and concretely on wheel30
  candidates
- `PrimeArithmetic/Sieve/Wheel30ByteArray.lean`: bounded single-byte array
  semantics for the wheel30 runtime segment, phrased directly in the runtime
  `(cycle, slot)` indexing surface and bridged into the generic bounded
  byte-family API through the shared coordinate shell
- `PrimeArithmetic/Sieve/Wheel30RuntimePlans.lean`: short wheel30 runtime mark
  families stated directly in executable `(cycle, slot)` coordinates and
  packaged on the grouped byte-plan layer
- `PrimeArithmetic/Sieve/Wheel30RuntimeRebase.lean`: base-invariant transport
  lemmas for grouped wheel30 runtime coordinate and plan families
- `PrimeArithmetic/Connector/ConcatenationFilters.lean`: exact fixed-width
  forward/reverse concatenation formulas, reduction modulo `m` when
  `base ≡ 1 (mod m)`, and canonical decimal `mod 3` / `mod 9` connector filters
- `PrimeArithmetic/Connector/ConcatenationFamilies.lean`: reusable pair-residue
  profiles and generic admissibility lemmas for whole connector families
- `PrimeArithmetic/Connector/ConcatenationProfileExamples.lean`: maintained
  instantiations of the connector-family API on additional preset pairs, beyond
  the single canonical pair
- `PrimeArithmetic/Analysis/HardyLittlewoodShell.lean`: conservative
  Hardy-Littlewood notation shell fixing pair-count conventions, exact
  odd-prime local-factor bookkeeping, radical invariance of the support, and the
  standard logarithmic / coverage transforms without asserting new density laws
- `PrimeArithmetic/Symmetry/MidpointObstruction.lean`: abstract midpoint
  obstruction theorem
- `PrimeArithmetic/Symmetry/Base6Example.lean`: concrete base-6 certified example
- `PrimeArithmetic/Symmetry/ModularReflection.lean`: concrete modular reflection
  on `Fin base`, including the fixed-point classifier `fixed -> 0 or base / 2`
- `PrimeArithmetic/Symmetry/BalancedBucketSupport.lean`: list-based support
  buckets, disjointness, balanced-length bookkeeping, and `zipPair`
  roundtrip lemmas for the later balanced-bucket certificate layer
- `PrimeArithmetic/Symmetry/BalancedBucketReflection.lean`: automatic
  reflection certificates from balanced residue counts, support-count
  agreement, and fixed-point exclusion
- `PrimeArithmetic/Symmetry/CertificateReflection.lean`: reusable certificate
  wrapper that turns reflection-equivariant observed data plus fixed-point
  exclusion into a `PerfectPairing`
- `PrimeArithmetic/Symmetry/WindowCertificate.lean`: narrow static/dynamic
  per-window certificate shell built from the balanced-bucket reflection layer
  plus pointwise midpoint-radius safety on positions
- `PrimeArithmetic/Symmetry/WindowCertificateGenerated.lean`: generated-data
  entrypoint from residue lists and position lists into the maintained window
  certificate shell, with automatically derived support counts
- `PrimeArithmetic/Symmetry/WindowCertificateErgonomics.lean`: bundled static
  and dual evidence objects for generated window artifacts, rebuilding the same
  certificates from a compact exported proof object
- `PrimeArithmetic/Symmetry/WindowCertificateExamples.lean`: explicit base-6 and
  base-10 finite certificates exercising the generated-data entrypoint, the
  bundled proof-object layer, and the static/window shells end to end
- `PrimeArithmetic/Generated/Examples/WindowP5Base10Span5.lean`: tracked
  runtime-exported certificate artifact generated from the prime window around
  `2 * 5^2`
- `PrimeArithmetic/Generated/Examples/WindowP3Base6Span5.lean`: tracked
  runtime-exported certificate artifact generated from the prime window around
  `2 * 3^2`
- `PrimeArithmetic/Generated/Examples/WindowP5Base12Span17.lean`: tracked
  runtime-exported certificate artifact generated from the prime window around
  `2 * 5^2`, using residue classes modulo `12`
- `PrimeArithmetic/Generated/Examples/WindowP11Base30Span5.lean`: tracked
  runtime-exported wheel-like base-`30` certificate artifact from the prime
  window around `2 * 11^2`
- `PrimeArithmetic/Generated/Examples/WindowP101Base30Span29.lean`: tracked
  runtime-exported base-`30` certificate artifact with six observed admissible
  residues in one finite window
- `PrimeArithmetic/Generated/Examples/WindowP163Base30Span35.lean`: tracked
  runtime-exported base-`30` certificate artifact with a larger balanced
  residue sample
- `PrimeArithmetic/Generated/Examples/WindowP41Base210Span5.lean`: tracked
  runtime-exported wheel-like base-`210` certificate artifact from the prime
  window around `2 * 41^2`
- `PrimeArithmetic/Generated/README.md`: generated artifact conventions and the
  canonical exporter invocation
- `PrimeArithmetic/Symmetry/MirrorObstruction.lean`: mirror-family midpoint
  obstruction via list reversal on `Fin (n + n)`
- `PrimeArithmetic/Symmetry/UnitResidueComplementWitness.lean`: generic even-base
  symmetry witness built from complement-paired unit residues
- `PrimeArithmetic/Density/CoprimeFilter.lean`: classical coprimality filter and
  base-10 specialization
- `PrimeArithmetic/Density/RadicalFilter.lean`: exact `rad(base)` reformulation
  and `rad ≠ φ` witness at base 12
- `PrimeArithmetic/Density/UnitResidues.lean`: admissible residue classes and
  their `φ(base)` count
- `PrimeArithmetic/Density/UnitResidueSymmetry.lean`: complement pairing on unit
  residues for bases above `2`
- `PrimeArithmetic/Density/SquarefreeBases.lean`: the simplification
  `Squarefree base -> rad(base) = base`
- `PrimeArithmetic/Density/ZModUnits.lean`: equivalence between unit residues and
  units of `ZMod base`
- `PrimeArithmetic/Density/ZModUnitNegation.lean`: negation pairing on
  `(ZMod base)ˣ`, even-cardinality consequence, and midpoint exclusion for even
  bases
- `PrimeArithmetic/Density/ZModUnitCRT.lean`: explicit CRT equivalences on
  `(ZMod (m * n))ˣ` and concrete split examples on unit groups
- `PrimeArithmetic/Density/WheelUnitCRT.lean`: finite-product wheel-base CRT
  decomposition on unit groups and family-level `φ(base) / 2` counts
- `PrimeArithmetic/Density/WheelUnitProductEquiv.lean`: the same wheel-base CRT
  theorem restated in canonical finite-family notation
- `PrimeArithmetic/Density/CoprimeBaseProducts.lean`: coprime-base product
  structure for radicals, unit residues, and units
- `PrimeArithmetic/Density/WheelBases.lean`: generic finite prime-product bases,
  with `rad` and `φ` formulas
- `PrimeArithmetic/Density/WheelResidueClassifier.lean`: finite-CRT residue
  classification and reconstruction for wheel bases
- `PrimeArithmetic/Density/UnitResiduePairs.lean`: explicit complement-pair
  representatives, pair-partition theorems, and the `φ(base) / 2` count
- `PrimeArithmetic/Density/ZModUnitOrbits.lean`: quotient of `(ZMod base)ˣ` by
  negation-orbit representatives and the corresponding `φ(base) / 2` count
- `PrimeArithmetic/Density/ZModUnitAction.lean`: the same quotient restated as
  the orbit space of the order-two subgroup `{1, -1}` acting on `(ZMod base)ˣ`
- `PrimeArithmetic/Density/Base6Residues.lean`: concrete unit residues mod 6
- `PrimeArithmetic/Density/Base10Residues.lean`: concrete unit residues mod 10
- `PrimeArithmetic/Density/Base12Residues.lean`: concrete unit residues mod 12
- `PrimeArithmetic/Density/Base14Residues.lean`: concrete unit residues mod 14
- `PrimeArithmetic/Density/Base15Residues.lean`: concrete unit residues mod 15
- `PrimeArithmetic/Density/Base18Residues.lean`: concrete unit residues mod 18
- `PrimeArithmetic/Density/Base22Residues.lean`: concrete unit residues mod 22
- `PrimeArithmetic/Density/Base22GradientPocket.lean`: concrete mod-5 affine
  side-pocket theorem for the base-22 `k = (0, 0)` vs `k = (2, 2)` comparison
- `PrimeArithmetic/Density/Base30Residues.lean`: concrete unit residues mod 30
- `PrimeArithmetic/Symmetry/ZModUnitNegationWitness.lean`: direct transport of
  negation on `(ZMod base)ˣ` into the abstract midpoint-obstruction witness
- `ROADMAP.md`: long-running Lean formalization ledger
- `THEOREM_INDEX.md`: mathematician-facing map from repo prose to current Lean
  theorem families

## Current Scope

What is proved here now:

- the abstract midpoint-obstruction theorem for perfectly paired residues
- the concrete modular reflection theorem on `Fin base`, including the
  classification of fixed points as `0` or `base / 2`
- a constructive balanced-bucket support layer: support lists of indices with a
  fixed residue, exact disjointness for distinct reflected residues, and
  list-based `zipPair` roundtrip lemmas on balanced disjoint supports
- a balanced-bucket reflection wrapper that turns residue counts, support-count
  agreement, and fixed-point exclusion into an actual `ReflectionCertificate`
- a reusable certificate layer that derives the abstract pairing witness from
  reflection-equivariant finite data together with exclusion of the fixed
  residues
- a narrow window-certificate shell that combines the static balanced-bucket
  symmetry certificate with a dynamic pointwise midpoint-radius safety contract
- explicit base-6 and base-10 finite certificates that exercise the balanced
  bucket and window shell APIs end to end
- a mirror-family theorem for even-length reversed index families, giving a
  clean Lean counterpart to the Agda mirror-obstruction signal
- the exact affine form of the symmetric template once base, boundary digits,
  padding counts, and middle width are fixed
- the modular consequence that every divisor of the base sees only the outer
  digit, so coprimality with the base is determined by the outer digit alone
- the modular consequence that for every modulus coprime to the base, the
  template defines an affine permutation of seed residues
- the resulting explicit seed classes for target residues, including the unique
  congruence class of seeds that forces divisibility by a fixed coprime modulus
- exact fixed-width forward/reverse connector concatenation formulas in an
  arbitrary base
- the reduction of those formulas to boundary-digit sums whenever
  `base ≡ 1 (mod m)`
- a reusable connector-family profile layer that turns known left/right residue
  classes into generic forward/reverse admissibility lemmas
- canonical decimal `mod 3` and `mod 9` exclusion filters for the maintained
  pair `(10301, 3007003007003)`, including direction-independence of the residue
  test
- a concrete base-6 witness matching the maintained Agda certified example
- the conservative density prerequisite `prime > base -> gcd(prime, base) = 1`
- the exact reformulation `gcd(m, radical(base)) = 1 ↔ gcd(m, base) = 1`
- the base-invariant unit-residue surface and its `Nat.totient` count
- complement pairing on unit residues for every base `> 2`, with the midpoint
  excluded from admissible residues
- a generic bridge from complement-paired unit residues back into the abstract
  midpoint-obstruction theorem for even bases
- the squarefree-base collapse `Squarefree base -> radical base = base`
- the bridge from unit residues to standard-library units of `ZMod base`
- the standard modular statement that negation on `(ZMod n)ˣ` is fixed-point
  free for `n > 2`, so the unit group has even cardinality and the midpoint
  class is excluded for even moduli
- the explicit CRT theorem on unit groups
  `(ZMod (m * n))ˣ ≃ (ZMod m)ˣ × (ZMod n)ˣ` for coprime bases
- a finite-product wheel-base CRT theorem on unit groups
  `(ZMod (wheelBase S))ˣ ≃ ∏ p ∈ S, (ZMod p)ˣ` expressed through a recursive
  tuple type on the chosen prime list
- the same wheel-base CRT theorem restated in canonical finite-family form
  `(ZMod (wheelBase S))ˣ ≃ ∀ p ∈ S, (ZMod p)ˣ`
- the CRT/product decomposition of admissible residues for coprime bases
- a generic wheel-base layer for finite products of distinct primes
- a finite-CRT classifier that characterizes admissible wheel-base residues by
  their local nonzero prime-factor conditions
- a canonical representative set for complement pairs on admissible residues,
  together with the exact count `φ(base) / 2`
- the corresponding negation-orbit quotient on `(ZMod base)ˣ`, again with
  cardinality `φ(base) / 2`
- the same `φ(base) / 2` quotient restated as the orbit space of the order-two
  subgroup `{1, -1}` acting on `(ZMod base)ˣ`
- a direct symmetry witness transporting negation on `(ZMod base)ˣ` back into
  the abstract midpoint-obstruction theorem
- explicit larger wheel-family examples at `210` and `2310`
- concrete residue modules for bases `6`, `10`, `12`, and `30`
- a generic base-family example at `210`, where the admissible residue count is
  proved to be `48`, with the exact local classifier `mod 2`, `3`, `5`, and `7`
- a generic base-family example at `2310`, where the admissible residue count
  is proved to be `480`, with the exact local classifier `mod 2`, `3`, `5`,
  `7`, and `11`
- the base-10 specialization corresponding to the maintained Agda residue-filter
  theorem, plus parallel concrete examples in other bases
- explicit small-modulus seed-class examples showing that the base-6 template
  is divisible by `7` exactly on the class `seed ≡ 1 (mod 7)` and the base-10
  template is divisible by `11` exactly on the class `seed ≡ 2 (mod 11)`

What is not claimed here yet:

- a completed Lean proof of any template-specific prime-density interpretation
- broader density bookkeeping beyond the exact filter/unit-residue layer
- a Lean port of the whole Agda workspace
- any automated OpenProver control loop

## Relationship To Other Formalization Work

- `agda-proofs/` remains the broader active proof workspace
- `lean-proofs/` is the Lean 4 lane for small, composable theorem families and
  explicit witnesses
- `THEOREM_INDEX.md` is the quickest map from older project language to current
  Lean statements
- `collab/openprover/README.md` describes how OpenProver can assist this Lean
  package without replacing it as the source of truth
