# Residue Fold Notes

This note covers the most stable Agda slice in this tree: residue-fold
convolution, CRT pushforward checks, and the small executable proof harness
around them.

## Current Scope

The following modules are the relevant audited surfaces:

- `Core/ResidueFold.agda`: residue-step convolution definitions and algebra
- `Core/CRTVector.agda`: LCM/CRT projection machinery
- `Theorems/MirrorObstruction.agda`: mirror-obstruction executable theorem area
- `Tests/DevProofs.agda`: small concrete checks that normalize on fixed cases

According to [`STATUS.md`](STATUS.md), these files currently type-check cleanly.

## What Is Actually Verified Here

- convolution and fold structure for residue counting are formalized in Agda
- selected CRT pushforward identities are checked on concrete patterns
- the mirror-obstruction example is represented in a machine-checked module
- the `DevProofs` module gives a compact executable sanity harness

## Limits

- some results in this area are executable checks on representative cases rather
  than full proof-term generalizations
- this note should not be read as proof coverage for the whole `agda-proofs/`
  tree
- broader repo claims still come from [`../CLAIMS.md`](../CLAIMS.md),
  [`../EVIDENCE.md`](../EVIDENCE.md), and [`STATUS.md`](STATUS.md)

## Local Verification

```bash
cd agda-proofs
agda --safe Core/ResidueFold.agda
agda --safe Core/CRTVector.agda
agda --safe Theorems/MirrorObstruction.agda
agda --safe Tests/DevProofs.agda
./verify-residue-fold.sh
```

For density-explorer context, see
[`CRT_IMPLEMENTATION_SUMMARY.md`](CRT_IMPLEMENTATION_SUMMARY.md) and
[`../tools/density-explorer/OPTIMIZATIONS.md`](../tools/density-explorer/OPTIMIZATIONS.md).
