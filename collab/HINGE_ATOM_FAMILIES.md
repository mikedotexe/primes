# Hinge Atom Families

**Updated**: April 2026
**Purpose**: freeze the maintained reading of the deterministic atom families
behind the hinge discriminator surface

This note sits one layer below
[`HINGE_DISCRIMINATOR.md`](./HINGE_DISCRIMINATOR.md). The discriminator note
freezes the target and shortcut boundary; this note explains which atom
families currently feel deepest, which ones are bridges, and which ones remain
diagnostic. For the downstream robustness pass that asks which parts of this
ladder actually survive scenario pressure, see
[`HINGE_ROBUSTNESS.md`](./HINGE_ROBUSTNESS.md).

## The Five Families

The maintained hinge search vocabulary now groups atoms into five families:

- `overlap_boundary`
  Exact transfer-sign/count language drawn from `stable_zero`, `boundary`,
  stable-zero margin, and shared-prime-rate effects at `M=2`
- `carry_through`
  `M=1` anomaly, `M=1` best-`k`, and related cross-`M` carry-through language
- `threshold_shape`
  observed cut-point atoms over overlap Jaccard, mask stability, churn, support
  ratio, anomaly size, and stable-zero margins
- `geometry`
  pair-geometry tags such as `gap_bucket` and `same_digit`
- `template_choice`
  whether the best lane is still noncompact, e.g. `best_k != k=(0,0)`

## Deterministic Depth Criteria

The report assigns each family a deterministic depth label:

- `deepest`
  The family has theorem class `ExactTransferSubstrate` and either:
  - wins a family-only exact rule on the primary or persistent split, or
  - its ablation destroys the primary exact separator
- `bridge`
  The family is not `deepest`, is not purely `Diagnostic`, and either:
  - wins a family-only exact rule on a secondary split, or
  - participates in exact mixed rules on the primary split
- `diagnostic`
  Everything else

The theorem-language labels are also deterministic:

- `closest_to_theorem`
  Reserved for `ExactTransferSubstrate` families whose winning exact rules are
  threshold-free
- `supporting_bridge`
  For `CrossMExactButEmpirical` and `DerivedThreshold` families that materially
  help exact rules without being direct substrate
- `not_yet_theorem_language`
  For diagnostic families

## Theorem-Adjacent vs Report-Useful

Two distinctions matter:

- **Theorem-adjacent**
  The atom family lines up directly with the exact transfer/compactness
  substrate that the repo is formalizing in Lean and Agda
- **Exactly useful in a report**
  The family can still help separate species or improve a small exact rule, but
  it depends on cross-`M` empirical boundaries, observed thresholds, or
  descriptive geometry rather than direct substrate

So a family can be exactly useful without yet being the right theorem language.

## Why Threshold Families Are Different

Threshold-derived atoms are explicit and deterministic, but they are still
derived from the observed feature surface. They are not the same thing as the
underlying transfer grammar.

That is why `threshold_shape` can still be a meaningful bridge family without
being treated as the deepest one. It often helps us summarize where the hinge
surface lives, but it does not yet replace the exact overlap/boundary grammar.

## Current Maintained Reading

On the maintained hinge surface:

- `overlap_boundary` is the deepest family and the closest to future theorem
  language
  - reason: it aligns with the exact transfer substrate and survives ablation
  - it also owns the clean persistent-split exact rule
- `carry_through` is a bridge family
  - reason: it captures real `M=1 -> M=2` inheritance and wins the
    core/persistence split, but it still leans on the empirical cross-`M`
    boundary rather than a purely `M=2` exact substrate
- `threshold_shape` is a second bridge family
  - reason: on the current finite surface it can even win exact cutpoint rules,
    but its atoms are still observed thresholds rather than transfer primitives
- `geometry` and `template_choice` are currently diagnostic
  - reason: they help classify the ecology and make the pictures more legible,
    but they do not yet explain the hinge with the same depth as the
    overlap/boundary grammar

## Practical Boundary

This note does **not** claim:

- a universal base law
- a general theorem about all hinge-like survival
- that geometry is mathematically inert forever

It only records the current maintained reading:

> overlap/boundary transfer atoms feel deepest because they align with the
> exact transfer grammar and survive ablation; carry-through and threshold
> shape are meaningful bridge families; geometry and template-choice are
> currently diagnostic rather than foundational.
