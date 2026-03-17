# CRT Pushforward Summary

This note summarizes the Agda side of the residue-fold / LCM projection work
used by the density-explorer tooling.

## Relevant Files

- `Core/CRTVector.agda`
- `Core/ResidueFold.agda`
- `Tests/DevProofs.agda`
- `verify-residue-fold.sh`

## Current Verified Scope

The current Agda support is narrower than a full general CRT theorem. It gives:

- an executable representation of residue-count projection from an LCM modulus
  down to tracked smaller moduli
- concrete sanity checks showing that selected projections match direct DP
  computations on the included test patterns
- a reusable technical note for the optimization path described in
  [`../tools/density-explorer/OPTIMIZATIONS.md`](../tools/density-explorer/OPTIMIZATIONS.md)

According to [`STATUS.md`](STATUS.md), `Core/CRTVector.agda` and
`Tests/DevProofs.agda` are in the clean-passing set.

## Local Verification

```bash
cd agda-proofs
agda --safe Core/CRTVector.agda
agda --safe Tests/DevProofs.agda
./verify-residue-fold.sh
```

For the broader residue-fold context, see
[`RESIDUE_FOLD_README.md`](RESIDUE_FOLD_README.md).
