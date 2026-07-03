# Proof-Carrying Witness Search-Policy Atlas

Schema: `proof-carrying-witness-search-policy-atlas-v1`. Claim status: `search-replay-residue-only`.

This atlas summarizes deterministic residue-replay coverage for the maintained proof-carrying witness bundle. It is a search-policy and residue-funnel artifact, not a primality proof.

## Summary

- Artifacts: 3
- Lanes: 1
- Seed-origin policies: 3
- Total scanned/rejected/survivor counts: `12/6/6`
- Max first-accepted distance: 6
- First-accepted theorem coverage: true
- Primality proof status: `probable-prime-not-proof-certified`

## Coverage Rows

| Artifact | Policy | Digits | Input seed | Witness seed | Distance | Replay rejected/survivors | Geometry | Lean first-accepted theorem |
|---|---|---:|---:|---:|---:|---:|---|---|
| `seed60-canonical-128d` | `canonical-fixed-seed` | 128 | 60 | 60 | 0 | 0/1 | `none` | `PrimeArithmetic.Generated.Witness.Seed60.searchReplayFirstAcceptedSurvivor` |
| `teaching-seed0-38d` | `teaching-fixed-seed` | 38 | 0 | 3 | 3 | 2/2 | `mod3x1_mod11x1` | `PrimeArithmetic.Generated.Witness.Teaching38.searchReplayFirstAcceptedSurvivor` |
| `timestamp-policy-trial0-29d` | `timestamp-policy` | 29 | 1777651200000000000 | 1777651200000000006 | 6 | 4/3 | `mod3x2_mod7x1_mod17x1` | `PrimeArithmetic.Generated.Witness.TimestampPolicy29Trial0.searchReplayFirstAcceptedSurvivor` |

## Lane Rows

| Lane | Artifacts | Digits | Policies | Max distance | Scanned/rejected/survivors |
|---|---:|---|---|---:|---:|
| `base10_outer3_inner7_k2_1` | 3 | `29,38,128` | `canonical-fixed-seed,teaching-fixed-seed,timestamp-policy` | 6 | 12/6/6 |

## Rejection Moduli

| Modulus | Replay rejections | Rejection examples | Artifacts |
|---:|---:|---:|---:|
| 3 | 3 | 4 | 3 |
| 7 | 1 | 2 | 2 |
| 11 | 1 | 1 | 1 |
| 13 | 0 | 1 | 1 |
| 17 | 1 | 1 | 1 |
