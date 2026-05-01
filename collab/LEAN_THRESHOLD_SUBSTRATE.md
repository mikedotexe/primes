# Lean Threshold Substrate

This note fixes the boundary of the new Lean 4 tranche for the bounded-`k`
threshold story.

## Exact Lean Vocabulary

- `compactness`:
  low padding / low diameter in the bounded-`k` symmetric template lane
- `admissibility mask`:
  the finite set of maintained prefilter-prime indices dividing a candidate
- `shared admissible overlap`:
  the `stableZero` transfer bucket, where both lanes have empty masks
- `transfer bucket`:
  one of `stableZero`, `gainZero`, `lossZero`, `stableNonzero`,
  `nonzeroChurn`

## What Lean Proves Exactly

- bounded-`k` compactness is an exact algebraic property of the template layout
- `k = (0,0)` minimizes width / diameter in that exact layout
- aligned mask transfer splits into the five exact buckets above
- admissible counts, admissible deltas, same-mask counts, and zero-union counts
  are exact finite combinatorial quantities on those buckets
- signed prime-flag deltas decompose exactly by transfer bucket
- generated witness modules for specific pairs certify exact finite counts and
  integer sign comparisons

## What Remains Empirical

- the catalog-wide `M = 1 → 2 → 3` collapse
- report-layer labels such as `stable_zero_led`, `boundary_led`, `persistent`,
  or `emergent`
- any claim that `M >= 3` holds as a theorem beyond the maintained finite
  catalog
- any cross-base “species” or “hinge” statement beyond the exact witness files

## Density Boundary

Prime density stays a guardrail, not the theorem object, in this tranche.

Lean is proving:
- exact finite admissibility-mask combinatorics
- exact finite signed count identities

Lean is not proving:
- a density theorem
- an asymptotic statement
- a template-specific prime-distribution law

## Language Boundary

“Chaos threshold” is report language only.

In Lean, the maintained statement should be phrased in arithmetic terms:
- compactness
- width / diameter
- admissibility-mask transfer
- shared-admissible overlap
- exact signed bucket deltas

Any future Lean engagement with `src/chaos/*` or the visual lane should begin
by extracting new exact arithmetic lemmas, not by formalizing simulation or
metaphor APIs directly.
