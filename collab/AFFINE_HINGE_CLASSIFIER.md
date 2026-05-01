# Affine Hinge Classifier Boundary

**Status**: exploratory classifier tranche, not a public-claim widening pass
**Updated**: April 2026

## What This Tranche Is For

This lane asks a narrower question than the current hinge reports:

- can the local affine comparison language reveal a cleaner discriminator for
  the maintained hinge species surface?

The first surface is intentionally hinge-centered:

- main bases: `10, 14, 22, 26`
- appendix/outgroups: `34, 6`
- middle lengths: `M = 1` and `M = 2`
- primary surface: `M = 2` active pairs with the maintained hinge categories

This is classifier exploration rather than theorem declaration.

## Lean vs Agda

This tranche is Lean-first.

- Lean 4 is the primary theorem engine for the local affine comparison
  language.
- Agda mirrors the chosen concepts in a narrow shell so vocabulary stays
  aligned, but it does not force proof parity in v1.

## Four Local Relation Labels

For one coprime modulus, comparing `k = (0,0)` to another lane gives:

- `identity`: same local shift and same local gradient
- `shift_only`: same local shift, different local gradient
- `gradient_only`: different local shift, same local gradient
- `shift_and_gradient`: different local shift and different local gradient

These labels are about the local affine map itself, not about primes directly.

## Important Distinctions

The affine lane uses three nearby but different objects:

1. **Affine local map equality**
   Two lanes have the same local affine map at a modulus when both the local
   shift and local gradient agree.

2. **Zero-seed-class equality**
   Two lanes cut out the same forbidden seed class for divisibility at a
   modulus.

3. **Hinge species labels**
   `persistent_core`, `persistence_only`, `core_only`, and `active_neither`
   remain report-layer labels downstream of the exact arithmetic.

The affine atlas is meant to help explain those labels, not to redefine them.

## Current Expected Reading

- the affine lane is being used to reveal a classifier, not yet to assert one
- exact local shift / gradient / zero-seed comparisons are promising because
  they live closer to the theorem substrate than the threshold-shape language
- the first exact local bridge is now sharper: affine gradient equality is
  period-locked, meaning the middle positions agree modulo the multiplicative
  order of the base unit at the chosen coprime modulus
- once that lock holds, the remaining local split is shift-controlled
  (`identity` vs `gradient_only`); when it fails, the complementary split is
  also shift-controlled (`shift_only` vs `shift_and_gradient`)
- if the affine-only search finds no exact separator, that still counts as a
  successful sharpening pass so long as it tells us what the next Lean question
  should be
