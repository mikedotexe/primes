# Connector Signal Atlas

This deterministic atlas indexes maintained connector-pair scans, exact residue guardrails, and conservative analytic transform facts. It is not a connector law and does not claim a prime-density mechanism.

- Schema: `connector-signal-atlas-v2`
- Artifact: `connector-signal-atlas`
- Matched budget: widths [5, 6, 7], digits [1, 2, 3, 4, 5, 6, 7, 8, 9], residue moduli [3, 9]
- Density-aware small-prime layer: [2, 3, 5, 7, 11, 13, 17, 19]

## Claim Status

| Layer | Status |
|---|---|
| Residue filters | `exact-direction-independent` |
| Residue survivor null | `theorem-backed-equal-directional-survivor-counts` |
| Analytic guardrail | `coverage-transform-only` |
| Residual comparison | `empirical-not-generalized` |
| Density mechanism | `not-claimed` |

## Maintained Pair Deltas

| Pair | Raw hit delta | Raw rate delta pp | Corrected expected-hit delta | Corrected ratio delta |
|---|---:|---:|---:|---:|
| Canonical pair (10301 ∘ 3007003007003) | -2 | -1.852 | -0.013313 | -0.242597 |
| Zero-padded membrane (10301 ∘ 30305070305070303) | -1 | -0.926 | 0.888493 | -0.239879 |
| Twin-prime profile (11 ∘ 13) | 8 | 7.407 | 0.494220 | 0.390872 |
| Sophie Germain profile (23 ∘ 47) | 1 | 0.926 | 0.590301 | 0.022042 |

## Directional Null Layer

The exact residue layer has equal directional survivor counts where the theorem applies; residual hit gaps remain empirical.

| Pair | Modulus | Blocked class | Forward survivors | Reverse survivors | Delta | Proof status | Lean theorem |
|---|---:|---:|---:|---:|---:|---|---|
| Canonical pair (10301 ∘ 3007003007003) | 3 | 2 | 108 | 108 | 0 | `theorem-backed-equal-directional-survivor-counts` | `canonicalProfileMod3_forward_reverse_survivor_count_eq` |
| Canonical pair (10301 ∘ 3007003007003) | 9 | 8 | 144 | 144 | 0 | `theorem-backed-equal-directional-survivor-counts` | `canonicalProfileMod9_forward_reverse_survivor_count_eq` |
| Zero-padded membrane (10301 ∘ 30305070305070303) | 3 | 1 | 108 | 108 | 0 | `theorem-backed-equal-directional-survivor-counts` | `zeroPaddedMembraneProfileMod3_forward_reverse_survivor_count_eq` |
| Zero-padded membrane (10301 ∘ 30305070305070303) | 9 | 1 | 144 | 144 | 0 | `theorem-backed-equal-directional-survivor-counts` | `zeroPaddedMembraneProfileMod9_forward_reverse_survivor_count_eq` |
| Twin-prime profile (11 ∘ 13) | 3 | 0 | 108 | 108 | 0 | `theorem-backed-equal-directional-survivor-counts` | `twinSmallProfileMod3_forward_reverse_survivor_count_eq` |
| Twin-prime profile (11 ∘ 13) | 9 | 3 | 144 | 144 | 0 | `theorem-backed-equal-directional-survivor-counts` | `twinSmallProfileMod9_forward_reverse_survivor_count_eq` |
| Sophie Germain profile (23 ∘ 47) | 3 | 2 | 108 | 108 | 0 | `theorem-backed-equal-directional-survivor-counts` | `sophieSmallProfileMod3_forward_reverse_survivor_count_eq` |
| Sophie Germain profile (23 ∘ 47) | 9 | 2 | 144 | 144 | 0 | `theorem-backed-equal-directional-survivor-counts` | `sophieSmallProfileMod9_forward_reverse_survivor_count_eq` |

## Residual Boundary Summary

The exact local filter is direction-neutral in every maintained row; any remaining asymmetry is outside this theorem layer.

| Pair | Residue survivor deltas | Residue equality | Raw hit delta | Corrected expected-hit delta | Corrected residual ratio delta | Boundary interpretation |
|---|---|---|---:|---:|---:|---|
| Canonical pair (10301 ∘ 3007003007003) | `mod3:0, mod9:0` | true | -2 | -0.013313 | -0.242597 | `exact-local-filter-direction-neutral-residual-gap-outside-theorem-layer` |
| Zero-padded membrane (10301 ∘ 30305070305070303) | `mod3:0, mod9:0` | true | -1 | 0.888493 | -0.239879 | `exact-local-filter-direction-neutral-residual-gap-outside-theorem-layer` |
| Twin-prime profile (11 ∘ 13) | `mod3:0, mod9:0` | true | 8 | 0.494220 | 0.390872 | `exact-local-filter-direction-neutral-residual-gap-outside-theorem-layer` |
| Sophie Germain profile (23 ∘ 47) | `mod3:0, mod9:0` | true | 1 | 0.590301 | 0.022042 | `exact-local-filter-direction-neutral-residual-gap-outside-theorem-layer` |

## Residual Target Picker

This picker chooses the maintained pair with the largest absolute corrected residual ratio delta after the exact residue-survivor null filter has been applied. It is a next experiment/theorem target, not a connector law.

| Pair | Selection rule | Residue survivor deltas | Absolute corrected residual ratio delta | Signed corrected residual ratio delta | Raw hit delta | Target status |
|---|---|---|---:|---:|---:|---|
| Twin-prime profile (11 ∘ 13) | `largest-absolute-corrected-residual-ratio-delta-after-residue-survivor-null` | `mod3:0, mod9:0` | 0.390872 | 0.390872 | 8 | `empirical-residual-target-not-mechanism-claim` |

## Residual Target Follow-Up

This bounded scan follows the selected target only. It keeps the residue-null guardrail in view while exposing width-level empirical hit/expectation rows.

| Direction | Width | Residue-admissible candidates | Prime hits | Corrected expected hits | Observed/corrected ratio |
|---|---:|---:|---:|---:|---:|
<!-- target: Twin-prime profile (11 ∘ 13); scope: selected-target-only;widths=[5, 6, 7];digits=[1, 2, 3, 4, 5, 6, 7, 8, 9];residue-moduli=[3, 9]; status: bounded-scan-follow-up-not-mechanism-claim; residue survivor deltas: mod3:0, mod9:0; theorem: PrimeArithmetic.Connector.ConcatenationProfileExamples.twinSmallProfileMod3_mod9_forward_reverse_survivor_count_eq -->
| forward | 5 | 30 | 7 | 5.997861 | 1.167083 |
| forward | 6 | 36 | 8 | 6.176595 | 1.295212 |
| forward | 7 | 42 | 11 | 7.079418 | 1.553800 |
| reverse | 5 | 30 | 4 | 5.630131 | 0.710463 |
| reverse | 6 | 36 | 2 | 5.849771 | 0.341894 |
| reverse | 7 | 42 | 12 | 7.279752 | 1.648408 |

### Width Contrast Pick

| Width | Selection rule | Forward ratio | Reverse ratio | Signed gap | Absolute gap | Forward hits | Reverse hits | Status |
|---:|---|---:|---:|---:|---:|---:|---:|---|
| 6 | `largest-absolute-forward-reverse-observed-to-corrected-ratio-gap` | 1.295212 | 0.341894 | 0.953318 | 0.953318 | 8 | 2 | `empirical-width-contrast-not-mechanism-claim` |

### Width Contrast Micro-Atlas

This micro-atlas compares the selected width-level empirical contrast with the theorem-backed residue-survivor null layer. It is a bounded follow-up target, not a mechanism claim.

| Pair | Width | Residue survivor deltas | Exact-layer decision | Forward hits | Reverse hits | Corrected expected-hit delta | Ratio gap | Empirical status | Next theorem decision |
|---|---:|---|---|---:|---:|---:|---:|---|---|
| Twin-prime profile (11 ∘ 13) | 6 | `mod3:0, mod9:0` | `exact-residue-null-layer-cannot-distinguish-this-width-contrast` | 8 | 2 | 0.326824 | 0.953318 | `bounded-width-empirical-residual-contrast-not-mechanism-claim` | `replicate-before-adding-new-connector-theorem` |

### Position/Digit Contrast Pick

This is the narrowest currently selected empirical contrast inside the width micro-atlas. A concentrated row is a candidate for exact-feature inspection; a diffuse row is a replication target.

| Width | Position | Digit | Connector | Class | Forward hit | Reverse hit | Forward ratio | Reverse ratio | Row ratio gap | Width ratio gap | Aligns with width | Concentration share | Localization status |
|---:|---:|---:|---|---|---|---|---:|---:|---:|---:|---|---:|---|
| 6 | 0 | 5 | `500000` | `reverse-only-prime-hit` | false | true | 0.000000 | 3.595501 | -3.595501 | 0.953318 | false | 0.100747 | `diffuse-mixed-sign-position-digit-contrast` |

Top position/digit rows by absolute ratio gap:

| Rank | Position | Digit | Connector | Class | Forward hit | Reverse hit | Ratio gap | Absolute gap |
|---:|---:|---:|---|---|---|---|---:|---:|
| 1 | 0 | 5 | `500000` | `reverse-only-prime-hit` | false | true | -3.595501 | 3.595501 |
| 2 | 1 | 7 | `070000` | `reverse-only-prime-hit` | false | true | -3.589965 | 3.589965 |
| 3 | 0 | 8 | `800000` | `forward-only-prime-hit` | true | false | 3.572483 | 3.572483 |
| 4 | 0 | 4 | `400000` | `forward-only-prime-hit` | true | false | 3.566585 | 3.566585 |
| 5 | 1 | 4 | `040000` | `forward-only-prime-hit` | true | false | 3.561097 | 3.561097 |
| 6 | 1 | 2 | `020000` | `forward-only-prime-hit` | true | false | 3.560787 | 3.560787 |
| 7 | 2 | 8 | `008000` | `forward-only-prime-hit` | true | false | 3.560601 | 3.560601 |
| 8 | 2 | 4 | `004000` | `forward-only-prime-hit` | true | false | 3.560539 | 3.560539 |

## Residual Sweep Summary

| Pair | Negative bounds | Positive bounds | Zero bounds | Delta range | Sign-stable |
|---|---:|---:|---:|---:|---|
| Canonical pair (10301 ∘ 3007003007003) | 9 | 0 | 0 | [-0.298856, -0.200520] | true |
| Zero-padded membrane (10301 ∘ 30305070305070303) | 9 | 0 | 0 | [-0.244970, -0.170073] | true |
| Twin-prime profile (11 ∘ 13) | 0 | 9 | 0 | [0.343902, 0.405676] | true |
| Sophie Germain profile (23 ∘ 47) | 0 | 9 | 0 | [0.004418, 0.049695] | true |

## Proof Links

| Link | Lean module | Status |
|---|---|---|
| fixed-width connector residue filters | `PrimeArithmetic.Connector.ConcatenationFilters` | `proved` |
| connector family residue profiles | `PrimeArithmetic.Connector.ConcatenationFamilies` | `proved` |
| finite connector residue-survivor count null theorem | `PrimeArithmetic.Connector.ConcatenationFamilies` | `proved` |
| canonical connector residue-survivor count null theorem | `PrimeArithmetic.Connector.ConcatenationFamilies` | `proved` |
| maintained connector profile examples | `PrimeArithmetic.Connector.ConcatenationProfileExamples` | `proved` |
| twin-prime mod-3 connector blocking explanation | `PrimeArithmetic.Connector.ConcatenationProfileExamples` | `proved-null-explanation` |
| expected-hit coverage monotonicity | `PrimeArithmetic.Analysis.HardyLittlewoodShell` | `proved-transform-only` |

The Hardy-Littlewood coverage row records only a monotone transform from supplied expected-hit `lambda` to Poisson-style coverage. Residual forward/reverse gaps remain empirical.
