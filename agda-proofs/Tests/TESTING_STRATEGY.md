# Agda Testing Strategy

This directory uses small executable proof checks as a complement to theorem
development. The goal is narrow: make selected proofs and specifications reduce
on concrete inputs, not to claim complete proof coverage for the entire Agda
workspace.

## Current Audited Test Surfaces

According to [`../STATUS.md`](../STATUS.md), these modules currently type-check
cleanly:

- `Specs/Tests.agda`
- `Tests/DevProofs.agda`
- `Tests/InvariantTests.agda`
- `Tests/Spec/ResidueCollapseSpec.agda`
- `Tests/Spec/Base10ResidueFilterSpec.agda`
- `Tests/Spec/ResidueClassesRingSpec.agda`
- `Tests/Spec/ResidueClassesUnitsSpec.agda`

The rest of the `Tests/Spec/` subtree remains exploratory. Those files are
useful design work, but they are not yet part of the clean-passing spine.

This is the audited clean-local test surface, not only the `--safe` subgroup.
`Tests/InvariantTests.agda` remains in the list because it is an executable
dynamic regression shell, but it is verified with plain `agda` rather than
`agda --safe`.

Within that clean test surface:

- `Tests/InvariantTests.agda` is the dynamic regression shell; it now carries
  one direct helper-path `PointwiseSafe` witness and one matching
  helper-agnostic negative `InZone` counterexample
- `Tests/Spec/Base10ResidueFilterSpec.agda` is the concrete-prime regression
  companion to `Examples/Base10ResidueFilter.agda`
- `Tests/Spec/ResidueClassesRingSpec.agda` and
  `Tests/Spec/ResidueClassesUnitsSpec.agda` are safe interface regressions over
  the constructive `Core/ResidueClassesComplete.agda` foundation

## What These Checks Are For

- catch mismatches between theorem intent and concrete computation
- keep a few representative proof applications easy to rerun locally
- provide executable examples for contributors working on the residue/spec path

## What These Checks Are Not

- a substitute for proof completion
- a guarantee that adjacent modules type-check
- evidence for repo-wide empirical claims on their own

## Local Verification

```bash
cd agda-proofs

# Safe subgroup
agda --safe Specs/Tests.agda
agda --safe Tests/DevProofs.agda
agda --safe Tests/Spec/ResidueCollapseSpec.agda
agda --safe Tests/Spec/Base10ResidueFilterSpec.agda
agda --safe Tests/Spec/ResidueClassesRingSpec.agda
agda --safe Tests/Spec/ResidueClassesUnitsSpec.agda

# Additional audited clean-local regression shell
agda Tests/InvariantTests.agda

./scripts/verify-clean-spine.sh
```

If you want broader module status, use [`../STATUS.md`](../STATUS.md). If you
want repository-level claim classification, use
[`../../CLAIMS.md`](../../CLAIMS.md) and
[`../../VERIFIED_FACTS_VS_SPECULATION.md`](../../VERIFIED_FACTS_VS_SPECULATION.md).
