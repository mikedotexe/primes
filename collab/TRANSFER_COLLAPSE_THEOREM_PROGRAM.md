# Transfer-Collapse Theorem Program

**Updated**: April 2026

This note freezes the boundary of the current direct-lane theorem program for
bounded-`k` transfer collapse.

The program is now deliberately narrower and stronger than the earlier
`best_k`-based story:

- theorem work compares `k = (0,0)` directly against each noncompact lane in
  `DEFAULT_BOUNDED_K_GRID`
- the governing exact ladder is:
  1. `profile_agreement`
  2. `admissible_set_equality`
  3. `no_positive_admissible_delta`
- no public `M >= 3` theorem should be claimed unless one rung survives both
  the `2p`-like and wheel-like class tracks on that direct comparison surface

## Why Direct Lane Comparisons Matter

The older `best_k` vs `k = (0,0)` comparison remains useful for report-layer
interpretation, but it is not a safe theorem surface.

Once `best_k = (0,0)`, the comparison becomes tautological. A theorem program
has to ask the harder question:

> what happens when `k = (0,0)` is compared directly to each maintained
> noncompact lane?

That is the only way to tell whether the apparent `M = 3` identity story is a
real exact collapse or just a consequence of picking the winner after the fact.

## Current Formal Shape

The theorem lane now has three distinct layers:

1. **Universal exact criterion**
   - [`PrimeArithmetic/Structure/BoundedKResidueProfile.lean`](../lean-proofs/PrimeArithmetic/Structure/BoundedKResidueProfile.lean)
   - [`PrimeArithmetic/Structure/BoundedKTransferCollapse.lean`](../lean-proofs/PrimeArithmetic/Structure/BoundedKTransferCollapse.lean)
   - this proves the exact conditional statement:
     profile agreement on the coprime local residue profiles forces
     candidatewise mask equality, zero gain/loss/churn, and zero admissible
     delta

2. **Parallel class-track wrappers**
   - [`PrimeArithmetic/Density/BoundedKTwoPrimeCollapse.lean`](../lean-proofs/PrimeArithmetic/Density/BoundedKTwoPrimeCollapse.lean)
   - [`PrimeArithmetic/Density/BoundedKWheelCollapse.lean`](../lean-proofs/PrimeArithmetic/Density/BoundedKWheelCollapse.lean)
   - these do not yet prove a positive `M >= 3` theorem; they keep the
     `2p`-like and wheel-like tracks aligned in theorem shape

3. **Generated direct-lane witnesses**
   - shell:
     [`PrimeArithmetic/Generated/BoundedKResidueProfileShell.lean`](../lean-proofs/PrimeArithmetic/Generated/BoundedKResidueProfileShell.lean)
   - exporter:
     [`src/bin/export_bounded_k_profile_witness.rs`](../src/bin/export_bounded_k_profile_witness.rs)
   - tracked counterexamples:
     [`PrimeArithmetic/Generated/BoundedKProfiles/Base6M3Pair15K00ToK01.lean`](../lean-proofs/PrimeArithmetic/Generated/BoundedKProfiles/Base6M3Pair15K00ToK01.lean)
     and
     [`PrimeArithmetic/Generated/BoundedKProfiles/Base30M3Pair11K00ToK01.lean`](../lean-proofs/PrimeArithmetic/Generated/BoundedKProfiles/Base30M3Pair11K00ToK01.lean)

## Stage 1 Audit Boundary

The maintained runtime audit is:

- [`examples/bounded_k_transfer_criterion_report.rs`](../examples/bounded_k_transfer_criterion_report.rs)

Its current result is decisive:

- on the full maintained direct lane-comparison surface, `M = 3` does **not**
  satisfy rung 1, 2, or 3 universally
- the strongest surviving global label at `M = 3` is still
  `fails_all_three`
- full profile agreement count at `M = 3`: `0`
- admissible-equality-only count at `M = 3`: `0`
- no-positive-admissible-delta-only count at `M = 3`: `1373`
- fails-all-three count at `M = 3`: `1171`

So the theorem program has already done something useful:
it ruled out the strongest naive direct-collapse statement on the maintained
surface.

## Tracked Negative Boundary

Two explicit direct-lane counterexamples now pin the current boundary:

- `2p`-like track:
  base `6`, `M = 3`, pair `(1,5)`, `k = (0,0) -> (0,1)`, admissible delta `+7`
- wheel-like track:
  base `30`, `M = 3`, pair `(1,1)`, `k = (0,0) -> (0,1)`, admissible delta `+3`

These are tracked precisely because they prevent accidental overclaiming of a
positive class theorem.

## What The Program Can Safely Claim Now

- There is a universal exact **conditional** theorem:
  if coprime local residue profiles agree, then the direct transfer comparison
  collapses to identity.
- The even-squarefree-`2p` and wheel-base tracks now have matched theorem
  wrappers and a shared proof vocabulary.
- The maintained direct-lane audit currently falsifies the strongest hoped-for
  positive `M = 3` theorem on the full tested surface.

## What The Program Should Not Claim Yet

- no universal public `M >= 3` transfer-collapse theorem
- no class theorem stronger than the strongest rung that survives direct audit
- no prime-density conclusion from this exact admissibility/transfer lane alone

## Live Next Step

The next serious theorem move is not to revive the failed global statement.
It is to search for a narrower invariant:

- a stronger hypothesis that implies one of the ladder rungs on direct lane
  comparisons, or
- a smaller controlled base/pair/lane class on which rung 2 or rung 3 really
  does survive.

That search should continue to use the direct comparison surface, not drift
back to `best_k` summaries.
