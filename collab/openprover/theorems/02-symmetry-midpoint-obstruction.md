Work on the abstract certification theorem behind the base-6 certified
resonance example.

Target statement:

Let `S` be a finite set with a distinguished midpoint `m` and an involution
`inv : S -> S` such that `inv (inv x) = x` for all `x` and `inv m = m`.
Suppose a finite family of observed residues admits a perfect pairing `mate`
with these properties:

- `mate` is involutive
- `mate` has no fixed points
- pairing is equivariant with the involution on residues
- each residue is distinct from the residue of its mate

Show that the midpoint residue `m` cannot occur among the observed residues.

Repo alignment:

- abstract spine: `agda-proofs/Theorems/Abstract/SymmetryImpliesRepulsion.agda`
- list bridge: `agda-proofs/Theorems/Abstract/SymmetryFromList.agda`
- concrete witness: `agda-proofs/Examples/CertifiedResonanceComplete.agda`
- Lean targets: `lean-proofs/PrimeArithmetic/Symmetry/MidpointObstruction.lean`,
  `lean-proofs/PrimeArithmetic/Symmetry/Base6Example.lean`
- concrete modular reflection layer:
  `lean-proofs/PrimeArithmetic/Symmetry/ModularReflection.lean`
- finite reflection-certificate layer:
  `lean-proofs/PrimeArithmetic/Symmetry/CertificateReflection.lean`
- mirror-family corollary:
  `lean-proofs/PrimeArithmetic/Symmetry/MirrorObstruction.lean`
- density/symmetry follow-on: `lean-proofs/PrimeArithmetic/Density/UnitResidueSymmetry.lean`
- standard modular follow-on: `lean-proofs/PrimeArithmetic/Density/ZModUnitNegation.lean`
- explicit pair-partition follow-on:
  `lean-proofs/PrimeArithmetic/Density/UnitResiduePairs.lean`
- unit-group orbit follow-on:
  `lean-proofs/PrimeArithmetic/Density/ZModUnitOrbits.lean`
- group-action follow-on:
  `lean-proofs/PrimeArithmetic/Density/ZModUnitAction.lean`
- abstract-bridge witness:
  `lean-proofs/PrimeArithmetic/Symmetry/UnitResidueComplementWitness.lean`
- direct `ZMod`-unit bridge witness:
  `lean-proofs/PrimeArithmetic/Symmetry/ZModUnitNegationWitness.lean`

Success criteria:

- give a clean proof decomposition into helper lemmas
- identify which parts are purely finite/involution combinatorics
- make the contradiction at a midpoint occurrence explicit
- if useful, connect the abstract midpoint obstruction to the concrete
  complement-pair symmetry now formalized for unit residues above base `2`
- if useful, restate the same midpoint/complement obstruction in the standard
  language of units of `ZMod n` and negation `u ↦ -u`
- if useful, identify the canonical complement-pair representative set whose
  cardinality is `φ(n) / 2`
- if useful, identify the corresponding quotient of `(ZMod n)ˣ` by negation
  orbits and explain why it has the same cardinality `φ(n) / 2`
- if useful, restate the same quotient as the orbit space of the order-two
  subgroup `{1, -1}` acting on `(ZMod n)ˣ`
- prefer reusing the new even-base complement witness rather than reproving the
  same midpoint exclusion from scratch
- if useful, show how the direct `ZMod`-unit witness is obtained by
  transporting negation into the abstract `PerfectPairing` interface
- if useful, show how the concrete `Fin base` reflection theorem and the
  reflection-certificate wrapper reduce mirror-style finite examples to the
  same abstract midpoint obstruction
- if useful, use the support-list / `zipPair` substrate in
  `lean-proofs/PrimeArithmetic/Symmetry/BalancedBucketSupport.lean` as the
  finite combinatorics layer for bucket-derived pairings
- if useful, reuse
  `lean-proofs/PrimeArithmetic/Symmetry/BalancedBucketReflection.lean` and
  `lean-proofs/PrimeArithmetic/Symmetry/WindowCertificate.lean` as the current
  static and static/dynamic certificate surfaces for balanced residue data
- if useful, use
  `lean-proofs/PrimeArithmetic/Symmetry/WindowCertificateExamples.lean` as the
  maintained explicit finite example layer for those certificate APIs
- if helpful, suggest a Lean-friendly or Agda-friendly lemma ordering

Do not:

- introduce membrane-density claims
- assume more arithmetic structure than the theorem actually needs
