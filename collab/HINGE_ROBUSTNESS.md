# Hinge Robustness

**Updated**: April 2026
**Purpose**: freeze the maintained robustness reading for the hinge
atom-family explanation

This note sits one layer below
[`HINGE_ATOM_FAMILIES.md`](./HINGE_ATOM_FAMILIES.md). The atom-family note
classifies the families on the maintained surface; this note asks how much of
that ladder survives when we perturb the surface or the threshold vocabulary.

## Robustness Goal

The goal of this pass is narrow:

- strengthen the hinge explanation layer
- do **not** claim a universal base law
- prefer family-level stability over exact rule-string stability

So the main success criterion is:

> the family ladder stays stable, even if the exact primary rule string drifts.

That is more useful here than demanding that one exact rule survive every
stress test unchanged.

## Scenario Groups

The maintained report uses three groups of scenarios:

### Data-surface stability

These drop one fixed representative at a time while preserving at least one
positive `persistent_core` row.

Maintained scenarios:

- `baseline_main`
- `drop_rep_db`
- `drop_rep_31`
- `drop_rep_33`
- `drop_rep_nn`
- `drop_rep_hj`

These are the main pass/fail scenarios.

### Threshold-vocabulary stability

These pressure the current weak point directly: `threshold_shape` can win exact
finite cutpoint rules, so we restrict or quantize the threshold vocabulary
without changing the search semantics otherwise.

Maintained scenarios:

- `threshold_observed`
- `threshold_min_side_2`
- `threshold_quantized_3dp`
- `threshold_quantized_2dp`

These are also counted in the main stability tally.

### Adversarial catalog

These remove whole families on purpose so we can see which parts of the
explanation are carrying what.

Maintained scenarios:

- `no_overlap_boundary`
- `no_carry_through`
- `no_threshold_shape`

These are diagnostic only. They are **not** counted in the main stability
tally.

## Status Meanings

The report uses four scenario outcomes:

- `strong_pass`
  The family ladder is stable and an exact primary rule remains, with the
  primary exact rule string unchanged from baseline.
- `ladder_pass`
  The family ladder is stable, but the primary exact rule string drifts or
  disappears.
- `weakened`
  `overlap_boundary` stays deepest and theorem-adjacent, but one of the bridge
  or diagnostic placements changes.
- `fail`
  `overlap_boundary` loses `deepest` or `closest_to_theorem`.

The `fail` label here is local to this explanation surface. It does **not**
mean the whole repo story fails.

## Current Maintained Reading

On the current maintained hinge surface:

- `overlap_boundary` should remain deepest on the main baseline and under the
  threshold-vocabulary stress cases
- `carry_through` should remain a bridge family
- `threshold_shape` may keep real exact finite cutpoint power, but it should
  remain derived bridge language rather than substrate-exact theorem language
- `geometry` and `template_choice` should remain diagnostic

The actual current matrix is slightly sharper than the optimistic reading:

- baseline is a `strong_pass`
- all threshold-vocabulary scenarios are `strong_pass`
- `drop_rep_db` and `drop_rep_31` are `ladder_pass`
- `drop_rep_33` is a real `fail`
  because removing the base-10 `(3,3)` contrast demotes `overlap_boundary`
  from `deepest` to `bridge`
- `no_overlap_boundary` is an adversarial `fail`, exactly as intended
- `no_carry_through` and `no_threshold_shape` are adversarial `weakened`

So the current robust reading is:

> the hinge ladder is stable under threshold-vocabulary pressure, but not under
> every data-surface perturbation; the most revealing weakness is that removing
> the base-10 persistence-only contrast weakens the clean depth gap between
> overlap/boundary atoms and the bridge families.

That is a valuable result. It says the explanation is real, but not yet fully
locked.

## Appendix Boundary

Bases `34` and `6` stay in the appendix audit only.

They are used to ask:

- does the best main-surface primary rule create held-out false positives?
- does the base-34 boundary-release outgroup stay non-hinge-like?

Current maintained reading:

- base `34` stays non-hinge-like under the stability audits
- base `6` remains a tiny but informative witness, not a driver of the main
  family ranking

## Practical Boundary

This note does **not** claim:

- a universal law for all bases
- a theorem that the current baseline exact rule must survive every perturbation
- that threshold-derived rules are unimportant

It records a narrower boundary:

> the hinge explanation is strongest when judged by family-ladder stability,
> and the present weakness is not threshold vocabulary but dependence on a
> small persistence-only contrast set.
