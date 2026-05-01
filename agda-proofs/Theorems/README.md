# Theorems Directory

This subtree contains a mix of abstract theorem frameworks, concrete theorem
modules, generated evidence files, and unfinished theorem drafts. Filenames
alone are not a reliable guide to what currently type-checks.

## Current Orientation

Use [`../STATUS.md`](../STATUS.md) to answer three questions:

- which theorem modules type-check cleanly
- which theorem modules only pass with postulates
- which theorem modules currently fail

As of March 2026, the strongest audited theorem-area modules include:

- `Abstract/SymmetryImpliesRepulsion.agda`
- `Abstract/SymmetryFromList.agda`
- `Abstract/ConstrainedOrbitals.agda`
- `AffineLaneComparisonShell.agda`
- `AffinePeriodLockShell.agda`
- `ElbowsFromCSV.agda`
- `GlobalElbowFacts.agda`
- `MirrorObstruction.agda`
- `RationalStatistics.agda`
- `SpectralRigidity.agda`

## Reading Guide

- `Abstract/`: general theorem scaffolding and reusable proof structure
- top-level theorem files: concrete theorem areas and derived facts
- `ElbowsFromCSV.agda`: generated theorem data from an empirical CSV pipeline

## Limits

- several prominent theorem filenames still correspond to failing modules
- some theorem files type-check only with postulates
- theorem names should not be treated as proof-complete claims without checking
  [`../STATUS.md`](../STATUS.md)

## Local Verification

```bash
cd agda-proofs
agda --safe Theorems/Abstract/SymmetryImpliesRepulsion.agda
agda --safe Theorems/MirrorObstruction.agda
agda --safe Theorems/ElbowsFromCSV.agda
```

For overall workspace orientation, start at [`../README.md`](../README.md).
