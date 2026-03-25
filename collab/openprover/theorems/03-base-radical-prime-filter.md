Develop the classical theorem surface behind the repo's radical divisibility
filter.

Primary target:

For integers `B >= 2` and `p > B`, if `p` is prime then `gcd(p, B) = 1`.

Strengthening target:

Explain why this is equivalent to `gcd(p, rad(B)) = 1`, where `rad(B)` is the
product of the distinct prime factors of `B`.

Further strengthening target:

For squarefree or wheel-like bases, explain how this collapses to the cleaner
base-family story `rad(B) = B`, and how `φ(B)` becomes a finite product
`∏ (p - 1)` when `B` is a product of distinct primes.

Related structural target:

For a fixed symmetric template layout, explain why the evaluated value is
affine in the middle block,

`template(seed) = shift + gradient * seed`,

and therefore has the same affine form modulo any modulus. If a modulus divides
the base, this residue is seed-independent and determined by the outer digit.
If a modulus is coprime to the base, this affine map has invertible slope and
therefore determines a unique seed class for each target residue.

Context:

- this is a safe classical fact that supports the repo's residue-filter story
- it is narrower and more honest than any claim about template-specific density
- it lines up with the repair target called out in `agda-proofs/SIGNAL_MAP.md`

Repo alignment:

- `agda-proofs/Core/Radical.agda`
- `agda-proofs/Theorems/RadicalDivisibilityFilter.agda`
- `agda-proofs/Examples/Base10ResidueFilter.agda`
- `agda-proofs/Theorems/AffineTransform.agda`
- `lean-proofs/PrimeArithmetic/Structure/AffineTemplate.lean`
- `lean-proofs/PrimeArithmetic/Structure/AffineResidueSearch.lean`
- `lean-proofs/PrimeArithmetic/Structure/AffineSeedClasses.lean`
- `lean-proofs/PrimeArithmetic/Density/RadicalFilter.lean`
- `lean-proofs/PrimeArithmetic/Density/UnitResidues.lean`
- `lean-proofs/PrimeArithmetic/Density/SquarefreeBases.lean`
- `lean-proofs/PrimeArithmetic/Density/ZModUnits.lean`
- `lean-proofs/PrimeArithmetic/Density/ZModUnitCRT.lean`
- `lean-proofs/PrimeArithmetic/Density/WheelUnitCRT.lean`
- `lean-proofs/PrimeArithmetic/Density/WheelUnitProductEquiv.lean`
- `lean-proofs/PrimeArithmetic/Density/CoprimeBaseProducts.lean`
- `lean-proofs/PrimeArithmetic/Density/WheelBases.lean`
- `lean-proofs/PrimeArithmetic/Density/WheelResidueClassifier.lean`
- `lean-proofs/PrimeArithmetic/Density/UnitResiduePairs.lean`
- `lean-proofs/PrimeArithmetic/Density/ZModUnitOrbits.lean`
- `lean-proofs/PrimeArithmetic/Density/ZModUnitAction.lean`
- `lean-proofs/PrimeArithmetic/Density/Base10Residues.lean`
- `lean-proofs/PrimeArithmetic/Density/Base12Residues.lean`

Success criteria:

- provide a proof that cleanly separates the `gcd(p, B) = 1` part from the
  `rad(B)` reformulation
- identify the extra hypotheses under which the radical story simplifies to a
  direct squarefree or wheel-base theorem
- make explicit how the unit-group CRT theorem packages the same coprime-base
  decomposition on `(ZMod (m * n))ˣ`
- if useful, promote the binary CRT theorem to the wheel-base family theorem
  on unit groups for a product of distinct primes
- if useful, restate that wheel-base theorem in canonical finite-family form
  `∀ p ∈ S, (ZMod p)ˣ`
- make explicit how the wheel-base admissible residues are classified by local
  nonzero conditions modulo each prime factor
- if useful, connect the admissible-residue classification to the canonical
  complement-pair representative set of size `φ(B) / 2`
- if useful, connect the same `φ(B) / 2` count to the quotient of `(ZMod B)ˣ`
  by negation orbits
- if useful, connect the same quotient to the orbit space of the order-two
  subgroup `{1, -1}` acting on `(ZMod B)ˣ`
- if useful, explain why the fixed-layout template evaluation is automatically
  affine in the middle block and how that interacts with modular filtering
- if useful, separate the two modular regimes for the affine template:
  divisors of the base versus moduli coprime to the base
- if useful, make explicit the unique seed class modulo a coprime modulus that
  forces divisibility by that modulus
- note what extra lemmas would be needed to port this into a constructive proof
  assistant
- distinguish clearly between the exact filter fact and any stronger heuristic
  story

Do not:

- omit the `p > B` condition without replacing it by an equivalent hypothesis
- conflate `rad(B)` with `phi(B)`
