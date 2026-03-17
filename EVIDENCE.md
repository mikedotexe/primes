# Evidence Summary

**Updated**: March 2026  
**Purpose**: current audited evidence overview for the repository

The previous long-form evidence file is preserved at
[`archive/EVIDENCE_legacy.md`](archive/EVIDENCE_legacy.md). This version keeps
only evidence that is still suitable for current public-facing docs.

## Current Verification Status

The canonical repo-level verification snapshot now lives in
[`STATUS.md`](STATUS.md). It records the current library-test, clippy,
top-level-example, and Agda status counts.

## Representative Membrane Results

Measured configurations (`n=1000`, Miller-Rabin, 20 rounds):

| Base | Boundary Digits | k | Prime Density | Verification |
|------|-----------------|---|---------------|--------------|
| 6    | (1, 5)          | (0, 0) | 33.0% | `cargo run --example prime_verification_report` |
| 30   | (11, 7)         | (0, 0) | 30.0% | `cargo run --example comprehensive_base_analysis` |
| 10   | (3, 7)          | (0, 0) | 18.5% | `cargo run --example comprehensive_base_analysis` |

These are measured examples, not universal maxima.

## Current Interpretation of the Density Effect

The strongest current evidence is that the membrane lift over naive random
integers is largely explained by coprimality filtering.

Key control result:

- membrane vs random-coprime structure ratio: about `1.020 +/- 0.053`
- interpretation: not statistically distinguishable from `1.0`
- verification entrypoint: `cargo run --example membrane_vs_random`

This is the main reason the current repo framing avoids claiming a demonstrated
membrane-specific mechanism.

## Structural Exact Probes

The exact probes in
[`examples/membrane_palindrome_probe.rs`](examples/membrane_palindrome_probe.rs)
and
[`examples/membrane_scaffold_probe.rs`](examples/membrane_scaffold_probe.rs)
add three useful constraints to the interpretation:

1. The membrane family is not reducible to ordinary palindromes. In the tested
   base-10 and base-6 families, non-palindromic membrane subsets retain clear
   prime density, and at even total digit lengths the palindromic subset drops
   out entirely while the non-palindromic membrane subset still contains
   verified primes.
2. For padded membrane configurations tested exactly, the canonical centered
   template does not consistently dominate nearby same-budget controls built
   from the same anchor multiset.
3. In broader independent-digit spacing families with matched boundary
   coprimality and zero budget, centered gap symmetry also does not show a
   consistent advantage; the deltas are small and change sign across the tested
   base-10 and base-6 families.

## Stable Findings Supported by Current Docs

These findings are the ones safest to cite today:

1. Boundary digits coprime to the base are essential.
2. Minimal padding `k=(0,0)` dominates for seed length `M >= 2`, with a
   documented base-10 `M=2` exception.
3. Compactness / diameter correlates strongly with observed density.
4. Connector asymmetry is real for the canonical pair `10301` and
   `3007003007003`, but broader generality remains open.
5. Membrane families tested exactly remain structurally broader than the
   palindrome subset.
6. Tested centered-gap scaffold controls do not show a consistent same-budget
   advantage in the currently audited families.

For source-by-source claim tracking, use [`CLAIMS.md`](CLAIMS.md).

## Verified Example Numbers

These concrete examples remain useful as checked anchor points:

| Description | Number | Status |
|-------------|--------|--------|
| Symmetric `(3,7)`, `k=(1,1)`, seed `5` | `307050703` | verified prime |
| Breathing `(3,3)`, `k=(0,1)`, seed `4` | `3304033` | verified prime |
| Breathing `(3,3)`, `k=(0,1)`, seed `5` | `3305033` | verified prime |
| Breathing `(3,3)`, `k=(0,1)`, seed `7` | `3307033` | verified prime |

The curated example programs remain the best way to regenerate or inspect these
cases.

## Recommended Source Hierarchy

If you need the most trustworthy current picture, prefer this order:

1. [`CLAIMS.md`](CLAIMS.md)
2. [`VERIFIED_FACTS_VS_SPECULATION.md`](VERIFIED_FACTS_VS_SPECULATION.md)
3. [`README.md`](README.md) and [`CLAUDE.md`](CLAUDE.md)
4. [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md)
5. [`archive/EVIDENCE_legacy.md`](archive/EVIDENCE_legacy.md) for older raw notes

## Notes

- `prime_verification_report` is useful because it checks documented examples and
  also exposes documentation mistakes that still need cleanup.
- The archived evidence file remains valuable for historical context, but it
  should not be treated as a uniformly audited source.
