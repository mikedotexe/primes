# Legacy Agda Sketches

This directory contains older standalone Agda sketches that predate the active
`agda-proofs/` workspace.

## What Is Here

- `PrimeConcepts.agda`: early vocabulary and theorem sketches
- `EmpiricalEvidence.agda`: encoded observations and data-oriented scaffolding
- `SpacingResidueModel.agda`: spacing/residue proof sketch
- `PalindromeEvenDivides.agda`: palindrome divisibility sketch
- `DigitSumMod3.agda`: base-class modulo-3 sketch

## Current Status

- These files are exploratory and pedagogical.
- They are not the source of truth for current theorem coverage or public repo
  claims.
- Older command blocks that referenced missing Rust examples were removed during
  the March 2026 audit.

## Use This Directory For

- reading older proof sketches
- comparing terminology with the newer `agda-proofs/` stack
- experimenting locally with single-file Agda ideas

## Use These Docs For Current Status

- [`../agda-proofs/STATUS.md`](../agda-proofs/STATUS.md): module-by-module status
- [`../agda-proofs/README.md`](../agda-proofs/README.md): active formalization workspace
- [`../CLAIMS.md`](../CLAIMS.md): public claim registry
- [`../EVIDENCE.md`](../EVIDENCE.md): current evidence summary

## Local Checking

If Agda and the standard library are installed locally, type-check individual
files directly, for example:

```bash
cd agda
agda --library standard-library SpacingResidueModel.agda
```

Treat success here as local exploration, not as a substitute for the audited
status recorded in [`../agda-proofs/STATUS.md`](../agda-proofs/STATUS.md).
