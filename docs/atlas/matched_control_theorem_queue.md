# Matched-Control Theorem Queue

This queue is generated from the residue-mask scanner summary. It is a theorem-planning surface, not a density or residual-mechanism claim.

- schema version: `matched-control-residue-masks-v4`
- panel: `smoke`
- panel id: `canonical-smoke-v1`
- prime bound: `31`
- pair fingerprints: `12`
- pair-certified fingerprints: `12`
- pair-uncertified fingerprints: `0`

## Top Candidate

- rank: `1`
- selection kind: `certified-follow-on-fingerprint`
- selection reason: all fingerprints are pair-certified; highest-ranked fingerprint remains the maintained follow-on target
- pair: `B10 ( 3, 3) k=(0,1) M=1` vs `B10 ( 3, 3) k=(1,1) M=1`
- base: `10`
- middle width: `1`
- same boundary digits: `true`
- bounded-k distinction: `true`
- rank bucket: `0`
- rank bucket label: `same boundary digits; distinct bounded-k profile`
- common moduli: `3, 7, 11, 13, 17, 19, 23, 29, 31`
- distinct excluded-class count: `8`
- overlap ratio fraction: `4151035350/7664025600`
- proof status pair: `exact-seed-class-separation/exact-seed-class-separation`
- pair certified: `true`

## Proof Links

- seed-mask separation: `PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenSeedMask_breathingM1_ne_symmetricM1_mod11`
- finite residue-set separation: `PrimeArithmetic.Density.Base10SeedClassSeparation.forbiddenResidues_breathingM1_ne_symmetricM1_mod11`
- equal survivor count: `PrimeArithmetic.Density.Base10SeedClassSeparation.survivorResidueCount_breathingM1_eq_symmetricM1_mod11`

## Queue Semantics

- `uncertified-pair-fingerprint`: add or repair Lean proof metadata before promoting a new exact theorem target.
- `certified-follow-on-fingerprint`: proof links already elaborate; use the row as the maintained planning anchor for the next explanatory theorem layer.
- The queue ranks exact local mask geometry only. It does not assert a prime-density mechanism.
