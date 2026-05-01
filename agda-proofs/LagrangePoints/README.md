# Lagrange Points Subtree

This subtree contains Agda formalization attempts around prime connectors,
buffer-position scans, and the repository's Lagrange/metaphor vocabulary.

Arithmetic-first reading:

- `ConnectorHit = (pair, width, position, digit, direction)`
- `ResidueAdmissible` belongs to the exact residue-screen layer
- `DirectionalAsymmetry` remains an empirical/open layer question
- "Lagrange point" is still allowed as repo shorthand, but should be read as an
  optional alias for a productive connector position rather than as a proved
  equilibrium law

## Current Status

This area is exploratory, not part of the strongest verified Agda spine.

From [`../STATUS.md`](../STATUS.md):

- `Examples.agda` now passes with postulates as a narrowed canonical case-study
- `ResidueField.agda` now passes with postulates as a narrowed residue-screen shell
- `TemplateExtension.agda` now passes with postulates as a narrowed asymmetric-template wrapper
- `ZeroPaddedPrimes/Alphabet036.agda` passes with postulates
- `ZeroPaddedPrimes/Asymmetry.agda` passes with postulates
- `ZeroPaddedPrimes/Examples036.agda` passes with postulates

## What To Infer Carefully

- these modules preserve useful formal structure around one canonical connector
  pair
- `Examples.agda` is now the most direct formal entry point for that pair
- `ResidueField.agda` now preserves the residue-side mechanism shell for that pair
- `TemplateExtension.agda` now preserves the symmetry-wrapper side of that pair
- they do not currently establish a general theory of connector asymmetry or
  Lagrange-point behavior
- repo-level claims about connector asymmetry remain empirical and narrow; see
  [`../../CLAIMS.md`](../../CLAIMS.md) and
  [`../../EVIDENCE.md`](../../EVIDENCE.md)

## Local Verification

```bash
cd agda-proofs
agda LagrangePoints/ZeroPaddedPrimes/Asymmetry.agda
```

The legacy metaphor-heavy overview is preserved at
[`../../archive/agda-proofs/LagrangePoints/README_legacy.md`](../../archive/agda-proofs/LagrangePoints/README_legacy.md).
