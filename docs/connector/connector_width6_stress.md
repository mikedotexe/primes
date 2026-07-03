# Connector Width-6 Stress Test

This deterministic stress surface fixes the selected connector microscope at width `6`, sweeps small-prime correction bounds, and compares the `(11,13)` target across a small twin-prime ladder. It is not a connector law and does not claim a prime-density mechanism.

- Schema: `connector-width6-stress-v79`
- Artifact: `connector-width6-stress`
- Width: `6`; digits: [1, 2, 3, 4, 5, 6, 7, 8, 9]; residue moduli: [3, 9]
- Small-prime correction bounds: [5, 7, 11, 13, 17, 19, 23, 29, 31]
- Claim status: `bounded-empirical-stress-test-not-connector-law`

## Stress Decision

- Selected target: `Selected twin-prime target (11 ∘ 13)`
- Target decision: `selected-width6-gap-persists-across-correction-bounds-but-microscope-is-diffuse-mixed-sign`
- Selected vs controls: `ladder-control-matches-or-exceeds-selected-width6-gap`

- Ladder pattern: `sign-changing-nonmonotone-ladder`
- Median gap monotone by pair size: `false`
- Selected median absolute gap rank: `2`
- Sign sequence: `3:-, 5:+, 11:+, 17:-, 29:+, 41:mixed`

## Ladder Peak Follow-Up

The peak follow-up compares the ladder's largest median width-6 residual gap against the same exact residue-null layer. It is a bounded feature screen, not a connector law.

- Peak pair: `Twin-prime ladder pair (3 ∘ 5)`
- Dominant top connector: `900000` at position `0` digit `9` (`reverse-only-prime-hit`)
- Exact layer decision: `exact-mod3-mod9-residue-null-layer-cannot-distinguish-peak-direction`
- Feature decision: `aligned-but-diffuse-peak-feature-candidate-needs-replication`
- Next experiment target: `stress-width6-peak-pair-3-5-connector-900000-under-broader-twin-ladder-controls`

| Bound | Small primes | Width ratio gap | Top connector | Position | Digit | Top class | Top row gap | Share of absolute gap | Aligns | Concentration | Localization |
|---:|---|---:|---|---:|---:|---|---:|---:|---|---|---|
| 5 | `[2, 3, 5]` | -0.920198 | `900000` | 0 | 9 | `reverse-only-prime-hit` | -4.771479 | 0.143683 | true | `diffuse-position-digit-contrast-replication-target` | `diffuse-aligned-position-digit-contrast` |
| 7 | `[2, 3, 5, 7]` | -0.946550 | `900000` | 0 | 9 | `reverse-only-prime-hit` | -4.089840 | 0.143683 | true | `diffuse-position-digit-contrast-replication-target` | `diffuse-aligned-position-digit-contrast` |
| 11 | `[2, 3, 5, 7, 11]` | -1.032743 | `900000` | 0 | 9 | `reverse-only-prime-hit` | -3.718036 | 0.143683 | true | `diffuse-position-digit-contrast-replication-target` | `diffuse-aligned-position-digit-contrast` |
| 13 | `[2, 3, 5, 7, 11, 13]` | -0.993062 | `900000` | 0 | 9 | `reverse-only-prime-hit` | -3.432033 | 0.143683 | true | `diffuse-position-digit-contrast-replication-target` | `diffuse-aligned-position-digit-contrast` |
| 17 | `[2, 3, 5, 7, 11, 13, 17]` | -0.934646 | `900000` | 0 | 9 | `reverse-only-prime-hit` | -3.230149 | 0.143683 | true | `diffuse-position-digit-contrast-replication-target` | `diffuse-aligned-position-digit-contrast` |
| 19 | `[2, 3, 5, 7, 11, 13, 17, 19]` | -0.965831 | `900000` | 0 | 9 | `reverse-only-prime-hit` | -3.060141 | 0.143683 | true | `diffuse-position-digit-contrast-replication-target` | `diffuse-aligned-position-digit-contrast` |
| 23 | `[2, 3, 5, 7, 11, 13, 17, 19, 23]` | -0.967885 | `900000` | 0 | 9 | `reverse-only-prime-hit` | -2.927091 | 0.143683 | true | `diffuse-position-digit-contrast-replication-target` | `diffuse-aligned-position-digit-contrast` |
| 29 | `[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]` | -0.981297 | `900000` | 0 | 9 | `reverse-only-prime-hit` | -2.826157 | 0.143683 | true | `diffuse-position-digit-contrast-replication-target` | `diffuse-aligned-position-digit-contrast` |
| 31 | `[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]` | -0.999693 | `900000` | 0 | 9 | `reverse-only-prime-hit` | -2.734991 | 0.143683 | true | `diffuse-position-digit-contrast-replication-target` | `diffuse-aligned-position-digit-contrast` |

## Peak Matched-Control Screen

This screen expands the `(3,5)` / `900000` peak into matched controls across nearby twin pairs, adjacent widths, and leading-digit/position variants. The exact residue-null layer remains direction-neutral; the rows below are empirical feature-screen rows.

- Selection rule: `rank-1-width6-ladder-peak-same-connector-adjacent-width-leading-digit-position-controls`
- Peak connector: `900000` for pair `(3, 5)`, width `6`, position `0`, digit `9`
- Exact layer decision: `exact-mod3-mod9-residue-null-layer-remains-direction-neutral-across-peak-controls`
- Exact layer Lean module: `PrimeArithmetic.Connector.ConcatenationProfileExamples`
- Exact layer Lean theorems: `twinPrimeAboveThree_decimal_connector_mod3_forward_blocked, twinPrimeAboveThree_decimal_connector_mod3_reverse_blocked`
- Screen decision: `peak-alignment-is-pair-specific-under-nearby-twin-controls`
- Aligned rows: `7` of `24`; aligned non-peak rows: `3`

- Mod-3 exception rows: `13`; theorem-blocked rows: `5`

- Next non-blocked candidate rule: `exclude-theorem-blocked;require-both-direction-residue-admissible;require-peak-gap-alignment;exclude-selected-peak-row;rank-by-absolute-ratio-gap`

### Next Non-Blocked Candidate

| Family | Pair | Width | Position | Digit | Connector | Ratio gap | Class | Mod-3 class | Residue deltas |
|---|---|---:|---:|---:|---|---:|---|---|---|
| `adjacent-width-control` | Twin-prime ladder pair (3 ∘ 5) | 7 | 0 | 9 | `9000000` | -3.453938 | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |

### Adjacent-Width Follow-Up

This follow-up keeps the `(3,5)`, position `0`, digit `9` lane fixed and compares the width-adjacent connectors under the same exact residue guardrail.

- Follow-up decision: `adjacent-width-amplification-candidate-under-exact-residue-guardrail`
- Exact layer decision: `mod3-mod9-residue-survivor-deltas-remain-zero-across-adjacent-widths`
- Strongest adjacent width: `7` via connector `9000000` with signed gap `-3.453938`

| Width | Connector | F residue | R residue | F hit | R hit | Ratio gap | Aligns | Class | Mod-3 class | Residue deltas |
|---:|---|---|---|---|---|---:|---|---|---|---|
| 5 | `90000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 6 | `900000` | true | true | false | true | -3.060141 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 7 | `9000000` | true | true | false | true | -3.453938 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |

### Width-Extension Probe

This probe keeps `(3,5)`, position `0`, digit `9` fixed and extends the width scan to `5..=9` to check whether the reverse-only pattern persists beyond the adjacent-width window.

- Persistence decision: `reverse-only-pattern-partial-width-extension-under-exact-residue-guardrail`
- Exact layer decision: `mod3-mod9-residue-survivor-deltas-remain-zero-across-width-extension`
- Strongest width: `7` via connector `9000000` with signed gap `-3.453938`

| Width | Connector | F residue | R residue | F hit | R hit | Ratio gap | Aligns | Class | Mod-3 class | Residue deltas |
|---:|---|---|---|---|---|---:|---|---|---|---|
| 5 | `90000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 6 | `900000` | true | true | false | true | -3.060141 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 7 | `9000000` | true | true | false | true | -3.453938 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 8 | `90000000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 9 | `900000000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |

### Leading-Digit Width Probe

This probe keeps `(3,5)` and position `0` fixed, then compares leading digits `1..=9` at widths `6` and `7`. It asks whether the reverse-only rows look like a width-6 row phenomenon, a digit-9 persistence phenomenon, or sparse prime-hit noise.

- Digit-pattern decision: `reverse-only-pattern-not-specific-to-leading-multiple-of-3-controls-under-exact-residue-guardrail`
- Exact layer decision: `mod3-mod9-residue-survivor-deltas-remain-zero-across-leading-digit-width-probe`
- Reverse-only rows: `4` total; multiple-of-3 digits: `3`; digit-9 rows: `2`; non-multiple-of-3 digits: `1`
- Strongest row: width `7`, digit `9`, connector `9000000` with signed gap `-3.453938`

- Top ranked hypothesis: `width-6-row-phenomenon`

#### Hypothesis Ranking

| Rank | Hypothesis | Score | Rationale |
|---:|---|---:|---|
| 1 | `width-6-row-phenomenon` | 9 | width 6 has reverse-only digits [6, 8, 9]; width 7 has [9] |
| 2 | `digit-9-persistence-phenomenon` | 6 | digits reverse-only at both widths are [9] |
| 3 | `sparse-prime-hit-noise` | 6 | digits with no reverse-only hit at either width are [1, 2, 3, 4, 5, 7] |

#### Compact Heatmap

| Digit | Width 6 connector | Width 6 class | Width 6 gap | Width 7 connector | Width 7 class | Width 7 gap | Persistence |
|---:|---|---|---:|---|---|---:|---|
| 1 | `100000` | `neither-prime-hit` | 0.000000 | `1000000` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 2 | `200000` | `neither-prime-hit` | 0.000000 | `2000000` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 3 | `300000` | `neither-prime-hit` | 0.000000 | `3000000` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 4 | `400000` | `neither-prime-hit` | 0.000000 | `4000000` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 5 | `500000` | `neither-prime-hit` | 0.000000 | `5000000` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 6 | `600000` | `reverse-only-prime-hit` | -3.051216 | `6000000` | `neither-prime-hit` | 0.000000 | `width6-only-reverse-only` |
| 7 | `700000` | `neither-prime-hit` | 0.000000 | `7000000` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 8 | `800000` | `reverse-only-prime-hit` | -3.057217 | `8000000` | `neither-prime-hit` | 0.000000 | `width6-only-reverse-only` |
| 9 | `900000` | `reverse-only-prime-hit` | -3.060141 | `9000000` | `reverse-only-prime-hit` | -3.453938 | `persistent-reverse-only` |

#### Probe Rows

| Width | Digit | Connector | F residue | R residue | F hit | R hit | Ratio gap | Aligns | Class | Mod-3 class | Residue deltas |
|---:|---:|---|---|---|---|---|---:|---|---|---|---|
| 6 | 1 | `100000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 6 | 2 | `200000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 6 | 3 | `300000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 6 | 4 | `400000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 6 | 5 | `500000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 6 | 6 | `600000` | true | true | false | true | -3.051216 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 6 | 7 | `700000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 6 | 8 | `800000` | true | true | false | true | -3.057217 | true | `reverse-only-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 6 | 9 | `900000` | true | true | false | true | -3.060141 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 7 | 1 | `1000000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 7 | 2 | `2000000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 7 | 3 | `3000000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 7 | 4 | `4000000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 7 | 5 | `5000000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 7 | 6 | `6000000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 7 | 7 | `7000000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 7 | 8 | `8000000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 7 | 9 | `9000000` | true | true | false | true | -3.453938 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |

### Width-6 Position/Digit Probe

This probe fixes `(3,5)` and width `6`, then checks positions `0..5` for leading digits `6`, `8`, and `9`. It asks whether the width-6 reverse-only row is localized at position `0` or spreads across connector positions.

- Position-pattern decision: `width6-reverse-only-spreads-across-positions-under-exact-residue-guardrail`
- Exact layer decision: `mod3-mod9-residue-survivor-deltas-remain-zero-across-width6-position-digit-probe`
- Reverse-only rows: `4` total; position-0 rows: `3`; non-position-0 rows: `1`
- Reverse-only positions: `[0, 5]`; reverse-only digits: `[6, 8, 9]`
- Strongest row: position `0`, digit `9`, connector `900000` with signed gap `-3.060141`
- Top ranked hypothesis: `width-6-position-spread-artifact`

#### Position Hypothesis Ranking

| Rank | Hypothesis | Score | Rationale |
|---:|---|---:|---|
| 1 | `width-6-position-spread-artifact` | 33 | reverse-only positions are [0, 5] |
| 2 | `sparse-prime-hit-noise` | 14 | 14 of 18 cells have no reverse-only hit |
| 3 | `position-0-localized-feature-candidate` | 3 | position 0 has 3 reverse-only rows; non-position-0 rows have 1 |

#### Position/Digit Heatmap

| Position | Digit 6 connector | Digit 6 class | Digit 6 gap | Digit 8 connector | Digit 8 class | Digit 8 gap | Digit 9 connector | Digit 9 class | Digit 9 gap | Position class |
|---:|---|---|---:|---|---|---:|---|---|---:|---|
| 0 | `600000` | `reverse-only-prime-hit` | -3.051216 | `800000` | `reverse-only-prime-hit` | -3.057217 | `900000` | `reverse-only-prime-hit` | -3.060141 | `position0-multi-digit-reverse-only` |
| 1 | `060000` | `neither-prime-hit` | 0.000000 | `080000` | `neither-prime-hit` | 0.000000 | `090000` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 2 | `006000` | `neither-prime-hit` | 0.000000 | `008000` | `neither-prime-hit` | 0.000000 | `009000` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 3 | `000600` | `neither-prime-hit` | 0.000000 | `000800` | `neither-prime-hit` | 0.000000 | `000900` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 4 | `000060` | `neither-prime-hit` | 0.000000 | `000080` | `neither-prime-hit` | 0.000000 | `000090` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 5 | `000006` | `reverse-only-prime-hit` | -3.031834 | `000008` | `neither-prime-hit` | 0.000000 | `000009` | `neither-prime-hit` | 0.000000 | `nonposition0-single-digit-reverse-only` |

#### Position/Digit Probe Rows

| Position | Digit | Connector | F residue | R residue | F hit | R hit | Ratio gap | Aligns | Class | Mod-3 class | Residue deltas |
|---:|---:|---|---|---|---|---|---:|---|---|---|---|
| 0 | 6 | `600000` | true | true | false | true | -3.051216 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 0 | 8 | `800000` | true | true | false | true | -3.057217 | true | `reverse-only-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 0 | 9 | `900000` | true | true | false | true | -3.060141 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 1 | 6 | `060000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 1 | 8 | `080000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 1 | 9 | `090000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 2 | 6 | `006000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 2 | 8 | `008000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 2 | 9 | `009000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 3 | 6 | `000600` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 3 | 8 | `000800` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 3 | 9 | `000900` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 4 | 6 | `000060` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 4 | 8 | `000080` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 4 | 9 | `000090` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 5 | 6 | `000006` | true | true | false | true | -3.031834 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |
| 5 | 8 | `000008` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | `mod3:0, mod9:0` |
| 5 | 9 | `000009` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | `mod3:0, mod9:0` |

### Width-7 Position/Digit Probe

This probe fixes `(3,5)` and width `7`, then checks positions `0..6` for leading digits `6`, `8`, and `9` under the same exact residue guardrail.

- Position-pattern decision: `width7-reverse-only-spreads-across-positions-under-exact-residue-guardrail`
- Exact layer decision: `mod3-mod9-residue-survivor-deltas-remain-zero-across-width7-position-digit-probe`
- Reverse-only rows: `2` total; position-0 rows: `1`; non-position-0 rows: `1`
- Reverse-only positions: `[0, 6]`; reverse-only digits: `[9]`
- Strongest row: position `0`, digit `9`, connector `9000000` with signed gap `-3.453938`
- Top ranked hypothesis: `width-7-position-spread-artifact`

#### Width-7 Position Hypothesis Ranking

| Rank | Hypothesis | Score | Rationale |
|---:|---|---:|---|
| 1 | `width-7-position-spread-artifact` | 33 | reverse-only positions are [0, 6] |
| 2 | `sparse-prime-hit-noise` | 23 | 19 of 21 cells have no reverse-only hit |
| 3 | `position-0-localized-feature-candidate` | 1 | position 0 has 1 reverse-only rows; non-position-0 rows have 1 |

#### Width-7 Position/Digit Heatmap

| Position | Digit 6 connector | Digit 6 class | Digit 6 gap | Digit 8 connector | Digit 8 class | Digit 8 gap | Digit 9 connector | Digit 9 class | Digit 9 gap | Position class |
|---:|---|---|---:|---|---|---:|---|---|---:|---|
| 0 | `6000000` | `neither-prime-hit` | 0.000000 | `8000000` | `neither-prime-hit` | 0.000000 | `9000000` | `reverse-only-prime-hit` | -3.453938 | `position0-single-digit-reverse-only` |
| 1 | `0600000` | `neither-prime-hit` | 0.000000 | `0800000` | `neither-prime-hit` | 0.000000 | `0900000` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 2 | `0060000` | `neither-prime-hit` | 0.000000 | `0080000` | `neither-prime-hit` | 0.000000 | `0090000` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 3 | `0006000` | `neither-prime-hit` | 0.000000 | `0008000` | `neither-prime-hit` | 0.000000 | `0009000` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 4 | `0000600` | `neither-prime-hit` | 0.000000 | `0000800` | `neither-prime-hit` | 0.000000 | `0000900` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 5 | `0000060` | `neither-prime-hit` | 0.000000 | `0000080` | `neither-prime-hit` | 0.000000 | `0000090` | `neither-prime-hit` | 0.000000 | `no-reverse-only` |
| 6 | `0000006` | `neither-prime-hit` | 0.000000 | `0000008` | `neither-prime-hit` | 0.000000 | `0000009` | `reverse-only-prime-hit` | -3.425631 | `nonposition0-single-digit-reverse-only` |

### Width Position-Spread Comparison

This block compares the width-6 and width-7 position/digit heatmaps directly. It is an empirical sparse-hit comparison under a direction-neutral residue layer.

- Comparison decision: `width6-and-width7-both-spread-across-positions-under-exact-residue-guardrail`
- Exact layer decision: `mod3-mod9-residue-survivor-deltas-remain-zero-across-width6-width7-position-comparison`
- Width 6 top hypothesis: `width-6-position-spread-artifact`; width 7 top hypothesis: `width-7-position-spread-artifact`

| Width | Reverse-only rows | Non-position-0 reverse-only rows | Reverse-only positions | Reverse-only digits | Top hypothesis | Decision |
|---:|---:|---:|---|---|---|---|
| 6 | 4 | 1 | `[0, 5]` | `[6, 8, 9]` | `width-6-position-spread-artifact` | `width6-reverse-only-spreads-across-positions-under-exact-residue-guardrail` |
| 7 | 2 | 1 | `[0, 6]` | `[9]` | `width-7-position-spread-artifact` | `width7-reverse-only-spreads-across-positions-under-exact-residue-guardrail` |

### Edge Position Probe

This probe compares leading edge position `0` with trailing edge position `width - 1` across widths `5..=9` for digits `6`, `8`, and `9`.

- Edge-pattern decision: `leading-and-trailing-edges-both-carry-reverse-only-hits-under-exact-residue-guardrail`
- Exact layer decision: `mod3-mod9-residue-survivor-deltas-remain-zero-across-edge-position-probe`
- Leading reverse-only cells: `4`; trailing reverse-only cells: `3`; both-edge cells: `2`
- Leading widths: `[6, 7]`; trailing widths: `[6, 7, 8]`; reverse-only digits: `[6, 8, 9]`
- Strongest row: `trailing` edge, width `8`, digit `6`, connector `00000006` with signed gap `-3.819429`
- Top ranked hypothesis: `leading-and-trailing-edge-spread`

#### Edge Hypothesis Ranking

| Rank | Hypothesis | Score | Rationale |
|---:|---|---:|---|
| 1 | `leading-and-trailing-edge-spread` | 51 | leading edge has 4 reverse-only cells; trailing edge has 3 |
| 2 | `sparse-prime-hit-noise` | 10 | 10 of 15 edge cells have no reverse-only hit |
| 3 | `leading-edge-only-pattern` | 4 | leading edge has 4 reverse-only cells |
| 4 | `trailing-edge-only-pattern` | 3 | trailing edge has 3 reverse-only cells |

#### Edge Heatmap

| Width | Digit | Leading connector | Leading class | Leading gap | Trailing connector | Trailing class | Trailing gap | Edge class |
|---:|---:|---|---|---:|---|---|---:|---|
| 5 | 6 | `60000` | `neither-prime-hit` | 0.000000 | `00006` | `neither-prime-hit` | 0.000000 | `no-edge-reverse-only` |
| 5 | 8 | `80000` | `neither-prime-hit` | 0.000000 | `00008` | `neither-prime-hit` | 0.000000 | `no-edge-reverse-only` |
| 5 | 9 | `90000` | `neither-prime-hit` | 0.000000 | `00009` | `neither-prime-hit` | 0.000000 | `no-edge-reverse-only` |
| 6 | 6 | `600000` | `reverse-only-prime-hit` | -3.051216 | `000006` | `reverse-only-prime-hit` | -3.031834 | `both-edges-reverse-only` |
| 6 | 8 | `800000` | `reverse-only-prime-hit` | -3.057217 | `000008` | `neither-prime-hit` | 0.000000 | `leading-edge-only-reverse-only` |
| 6 | 9 | `900000` | `reverse-only-prime-hit` | -3.060141 | `000009` | `neither-prime-hit` | 0.000000 | `leading-edge-only-reverse-only` |
| 7 | 6 | `6000000` | `neither-prime-hit` | 0.000000 | `0000006` | `neither-prime-hit` | 0.000000 | `no-edge-reverse-only` |
| 7 | 8 | `8000000` | `neither-prime-hit` | 0.000000 | `0000008` | `neither-prime-hit` | 0.000000 | `no-edge-reverse-only` |
| 7 | 9 | `9000000` | `reverse-only-prime-hit` | -3.453938 | `0000009` | `reverse-only-prime-hit` | -3.425631 | `both-edges-reverse-only` |
| 8 | 6 | `60000000` | `neither-prime-hit` | 0.000000 | `00000006` | `reverse-only-prime-hit` | -3.819429 | `trailing-edge-only-reverse-only` |
| 8 | 8 | `80000000` | `neither-prime-hit` | 0.000000 | `00000008` | `neither-prime-hit` | 0.000000 | `no-edge-reverse-only` |
| 8 | 9 | `90000000` | `neither-prime-hit` | 0.000000 | `00000009` | `neither-prime-hit` | 0.000000 | `no-edge-reverse-only` |
| 9 | 6 | `600000000` | `neither-prime-hit` | 0.000000 | `000000006` | `neither-prime-hit` | 0.000000 | `no-edge-reverse-only` |
| 9 | 8 | `800000000` | `neither-prime-hit` | 0.000000 | `000000008` | `neither-prime-hit` | 0.000000 | `no-edge-reverse-only` |
| 9 | 9 | `900000000` | `neither-prime-hit` | 0.000000 | `000000009` | `neither-prime-hit` | 0.000000 | `no-edge-reverse-only` |

### Edge Pair-Replication Probe

This probe repeats the leading/trailing edge scan across the nearby twin-prime ladder, excludes theorem-blocked mod-3 rows from the ranking, and asks whether reverse-only edge cells replicate across non-blocked pairs.

- Replication decision: `nonblocked-edge-replication-found-under-mod3-exclusion`
- Exact layer decision: `theorem-blocked-mod3-twin-pair-controls-excluded-before-edge-replication-ranking`
- Raw rows: `180`; non-blocked rows: `80`; theorem-blocked rows: `100`
- Ranked cells: `30`; replicated reverse-only cells: `2`; singleton reverse-only cells: `7`; neutral cells: `21`
- Target `00000006` (`trailing` edge, width `8`, digit `6`): `singleton-nonblocked-reverse-only-edge-cell`; reverse-only pairs: `1`; theorem-blocked pair controls: `5`; labels: `["Twin-prime ladder pair (3 ∘ 5)"]`
- Top ranked non-blocked cell: `leading` edge, width `6`, digit `8`, connector `800000` with `2` reverse-only pair(s)

#### Edge Pair-Replication Ranking

| Rank | Edge | Width | Digit | Connector | Non-blocked pairs | Theorem-blocked pairs | Reverse-only pairs | Reverse-only labels | Strongest pair | Status |
|---:|---|---:|---:|---|---:|---:|---:|---|---|---|
| 1 | `leading` | 6 | 8 | `800000` | 6 | 0 | 2 | `["Twin-prime ladder pair (3 ∘ 5)", "Twin-prime ladder pair (17 ∘ 19)"]` | `Twin-prime ladder pair (17 ∘ 19)` | `replicated-nonblocked-reverse-only-edge-cell` |
| 2 | `trailing` | 5 | 8 | `00008` | 6 | 0 | 2 | `["Selected twin-prime target (11 ∘ 13)", "Twin-prime ladder pair (29 ∘ 31)"]` | `Twin-prime ladder pair (29 ∘ 31)` | `replicated-nonblocked-reverse-only-edge-cell` |
| 3 | `leading` | 9 | 8 | `800000000` | 6 | 0 | 1 | `["Selected twin-prime target (11 ∘ 13)"]` | `Selected twin-prime target (11 ∘ 13)` | `singleton-nonblocked-reverse-only-edge-cell` |
| 4 | `trailing` | 8 | 6 | `00000006` | 1 | 5 | 1 | `["Twin-prime ladder pair (3 ∘ 5)"]` | `Twin-prime ladder pair (3 ∘ 5)` | `singleton-nonblocked-reverse-only-edge-cell` |
| 5 | `leading` | 7 | 9 | `9000000` | 1 | 5 | 1 | `["Twin-prime ladder pair (3 ∘ 5)"]` | `Twin-prime ladder pair (3 ∘ 5)` | `singleton-nonblocked-reverse-only-edge-cell` |
| 6 | `trailing` | 7 | 9 | `0000009` | 1 | 5 | 1 | `["Twin-prime ladder pair (3 ∘ 5)"]` | `Twin-prime ladder pair (3 ∘ 5)` | `singleton-nonblocked-reverse-only-edge-cell` |
| 7 | `leading` | 6 | 9 | `900000` | 1 | 5 | 1 | `["Twin-prime ladder pair (3 ∘ 5)"]` | `Twin-prime ladder pair (3 ∘ 5)` | `singleton-nonblocked-reverse-only-edge-cell` |
| 8 | `leading` | 6 | 6 | `600000` | 1 | 5 | 1 | `["Twin-prime ladder pair (3 ∘ 5)"]` | `Twin-prime ladder pair (3 ∘ 5)` | `singleton-nonblocked-reverse-only-edge-cell` |
| 9 | `trailing` | 6 | 6 | `000006` | 1 | 5 | 1 | `["Twin-prime ladder pair (3 ∘ 5)"]` | `Twin-prime ladder pair (3 ∘ 5)` | `singleton-nonblocked-reverse-only-edge-cell` |
| 10 | `trailing` | 9 | 8 | `000000008` | 6 | 0 | 0 | `[]` | `Twin-prime ladder pair (17 ∘ 19)` | `no-nonblocked-reverse-only-replication` |
| 11 | `trailing` | 8 | 8 | `00000008` | 6 | 0 | 0 | `[]` | `Twin-prime ladder pair (41 ∘ 43)` | `no-nonblocked-reverse-only-replication` |
| 12 | `leading` | 8 | 8 | `80000000` | 6 | 0 | 0 | `[]` | `Twin-prime ladder pair (17 ∘ 19)` | `no-nonblocked-reverse-only-replication` |

### Digit-8 Edge Zoom Probe

This probe zooms into the replicated digit-8 edge cells, widening the twin-prime ladder and comparing leading/trailing edges across widths `5..=7` while excluding theorem-blocked mod-3 rows from the ranking.

- Zoom decision: `digit8-anchor-edge-cells-replicate-across-widened-nonblocked-ladder`
- Exact layer decision: `digit8-connectors-are-not-theorem-blocked-by-the-decimal-multiple-of-3-null-layer`
- Widened pair count: `12`; raw rows: `72`; non-blocked rows: `72`; theorem-blocked rows: `0`
- Ranked cells: `6`; replicated reverse-only cells: `3`; singleton reverse-only cells: `2`; neutral cells: `1`
- Top ranked digit-8 edge cell: `trailing` edge, width `5`, connector `00008` with `4` reverse-only pair(s)

#### Digit-8 Focus Status

| Anchor connector | Edge | Anchor width | Reverse-only pairs | Labels | Status |
|---|---|---:|---:|---|---|
| `800000` | `leading` | 6 | 3 | `["Twin-prime ladder pair (3 ∘ 5)", "Twin-prime ladder pair (17 ∘ 19)", "Twin-prime ladder pair (71 ∘ 73)"]` | `replicated-nonblocked-reverse-only-edge-cell` |
| `00008` | `trailing` | 5 | 4 | `["Selected twin-prime target (11 ∘ 13)", "Twin-prime ladder pair (29 ∘ 31)", "Twin-prime ladder pair (101 ∘ 103)", "Twin-prime ladder pair (107 ∘ 109)"]` | `replicated-nonblocked-reverse-only-edge-cell` |

#### Digit-8 Edge Ranking

| Rank | Edge | Width | Connector | Non-blocked pairs | Reverse-only pairs | Reverse-only labels | Strongest pair | Status |
|---:|---|---:|---|---:|---:|---|---|---|
| 1 | `trailing` | 5 | `00008` | 12 | 4 | `["Selected twin-prime target (11 ∘ 13)", "Twin-prime ladder pair (29 ∘ 31)", "Twin-prime ladder pair (101 ∘ 103)", "Twin-prime ladder pair (107 ∘ 109)"]` | `Twin-prime ladder pair (107 ∘ 109)` | `replicated-nonblocked-reverse-only-edge-cell` |
| 2 | `trailing` | 6 | `000008` | 12 | 3 | `["Twin-prime ladder pair (101 ∘ 103)", "Twin-prime ladder pair (137 ∘ 139)", "Twin-prime ladder pair (149 ∘ 151)"]` | `Twin-prime ladder pair (149 ∘ 151)` | `replicated-nonblocked-reverse-only-edge-cell` |
| 3 | `leading` | 6 | `800000` | 12 | 3 | `["Twin-prime ladder pair (3 ∘ 5)", "Twin-prime ladder pair (17 ∘ 19)", "Twin-prime ladder pair (71 ∘ 73)"]` | `Twin-prime ladder pair (71 ∘ 73)` | `replicated-nonblocked-reverse-only-edge-cell` |
| 4 | `trailing` | 7 | `0000008` | 12 | 1 | `["Twin-prime ladder pair (149 ∘ 151)"]` | `Twin-prime ladder pair (149 ∘ 151)` | `singleton-nonblocked-reverse-only-edge-cell` |
| 5 | `leading` | 5 | `80000` | 12 | 1 | `["Twin-prime ladder pair (101 ∘ 103)"]` | `Twin-prime ladder pair (101 ∘ 103)` | `singleton-nonblocked-reverse-only-edge-cell` |
| 6 | `leading` | 7 | `8000000` | 12 | 0 | `[]` | `Twin-prime ladder pair (101 ∘ 103)` | `no-nonblocked-reverse-only-replication` |

#### Digit-8 Residue-Class Profile

This profile compares reverse-only rows with non-reverse-only rows by the left twin-prime residue `p mod q` for small prime moduli. Most rows remain theorem candidates; rows with Lean metadata are finite bounded classifiers for this stress artifact only, not connector laws.

- Profile decision: `small-prime-residue-separators-cover-every-replicated-digit8-edge-cell`
- Profiled cells: `3`; exact separator cells: `3`; moduli: `[3, 5, 7, 11, 13, 17, 19, 23, 29, 31]`
- Best-separator theorem coverage: backed `3`; unbacked `0`
- Best separator: `leading` edge width `6` connector `800000` mod `17` reverse-only residues `[0, 3]`; proof status `finite-bounded-classifier-theorem-backed` via `digit8LeadingWidth6_reverseOnly_mem_iff_mod17`

- Next unclassified exact separator: `none`

##### Multi-Modulus Theorem-Backed Summaries

These summaries group repeated finite classifiers for the same digit-8 edge cell. They are compact stress-artifact metadata only; they do not promote the separators to a connector law.

| Edge | Width | Connector | Theorem-backed moduli | Reverse-only pairs | Comparison pairs | Status | Summary theorem | Theorems |
|---|---:|---|---|---:|---:|---|---|---|
| `leading` | 6 | `800000` | `[17, 23, 29, 31]` | 3 | 9 | `multi-modulus-finite-classifier-summary` | `digit8LeadingWidth6_reverseOnly_multiModulusClassifier` | `["digit8LeadingWidth6_reverseOnly_mem_iff_mod17", "digit8LeadingWidth6_reverseOnly_mem_iff_mod23", "digit8LeadingWidth6_reverseOnly_mem_iff_mod29", "digit8LeadingWidth6_reverseOnly_mem_iff_mod31"]` |
| `trailing` | 6 | `000008` | `[17, 19, 29, 31]` | 3 | 9 | `multi-modulus-finite-classifier-summary` | `digit8TrailingWidth6_reverseOnly_multiModulusClassifier` | `["digit8TrailingWidth6_reverseOnly_mem_iff_mod17", "digit8TrailingWidth6_reverseOnly_mem_iff_mod19", "digit8TrailingWidth6_reverseOnly_mem_iff_mod29", "digit8TrailingWidth6_reverseOnly_mem_iff_mod31"]` |
| `trailing` | 5 | `00008` | `[19, 29, 31]` | 4 | 8 | `multi-modulus-finite-classifier-summary` | `digit8TrailingWidth5_reverseOnly_multiModulusClassifier` | `["digit8TrailingWidth5_reverseOnly_mem_iff_mod19", "digit8TrailingWidth5_reverseOnly_mem_iff_mod29", "digit8TrailingWidth5_reverseOnly_mem_iff_mod31"]` |

| Edge | Width | Connector | Reverse-only pairs | Comparison pairs | Best separator | Reverse-only residues |
|---|---:|---|---:|---:|---|---|
| `leading` | 6 | `800000` | 3 | 9 | `17` | `[0, 3]` |
| `trailing` | 5 | `00008` | 4 | 8 | `19` | `[6, 10, 11, 12]` |
| `trailing` | 6 | `000008` | 3 | 9 | `17` | `[1, 13, 16]` |

##### Exact Separator Candidates

| Edge | Width | Connector | Modulus | Reverse-only residues | Comparison residues | Proof status | Lean theorem |
|---|---:|---|---:|---|---|---|---|
| `leading` | 6 | `800000` | 17 | `[0, 3]` | `[1, 5, 7, 8, 11, 12, 13, 16]` | `finite-bounded-classifier-theorem-backed` | `digit8LeadingWidth6_reverseOnly_mem_iff_mod17` |
| `leading` | 6 | `800000` | 23 | `[2, 3, 17]` | `[5, 6, 9, 11, 13, 15, 18, 22]` | `finite-bounded-classifier-theorem-backed` | `digit8LeadingWidth6_reverseOnly_mem_iff_mod23` |
| `leading` | 6 | `800000` | 29 | `[3, 13, 17]` | `[0, 1, 4, 5, 11, 12, 14, 20, 21]` | `finite-bounded-classifier-theorem-backed` | `digit8LeadingWidth6_reverseOnly_mem_iff_mod29` |
| `leading` | 6 | `800000` | 31 | `[3, 9, 17]` | `[5, 8, 10, 11, 13, 14, 25, 28, 29]` | `finite-bounded-classifier-theorem-backed` | `digit8LeadingWidth6_reverseOnly_mem_iff_mod31` |
| `trailing` | 5 | `00008` | 19 | `[6, 10, 11, 12]` | `[2, 3, 4, 5, 14, 16, 17]` | `finite-bounded-classifier-theorem-backed` | `digit8TrailingWidth5_reverseOnly_mem_iff_mod19` |
| `trailing` | 5 | `00008` | 29 | `[0, 11, 14, 20]` | `[1, 3, 4, 5, 12, 13, 17, 21]` | `finite-bounded-classifier-theorem-backed` | `digit8TrailingWidth5_reverseOnly_mem_iff_mod29` |
| `trailing` | 5 | `00008` | 31 | `[8, 11, 14, 29]` | `[3, 5, 9, 10, 13, 17, 25, 28]` | `finite-bounded-classifier-theorem-backed` | `digit8TrailingWidth5_reverseOnly_mem_iff_mod31` |
| `trailing` | 6 | `000008` | 17 | `[1, 13, 16]` | `[0, 3, 5, 7, 8, 11, 12]` | `finite-bounded-classifier-theorem-backed` | `digit8TrailingWidth6_reverseOnly_mem_iff_mod17` |
| `trailing` | 6 | `000008` | 19 | `[4, 6, 16]` | `[2, 3, 5, 10, 11, 12, 14, 17]` | `finite-bounded-classifier-theorem-backed` | `digit8TrailingWidth6_reverseOnly_mem_iff_mod19` |
| `trailing` | 6 | `000008` | 29 | `[4, 14, 21]` | `[0, 1, 3, 5, 11, 12, 13, 17, 20]` | `finite-bounded-classifier-theorem-backed` | `digit8TrailingWidth6_reverseOnly_mem_iff_mod29` |
| `trailing` | 6 | `000008` | 31 | `[8, 13, 25]` | `[3, 5, 9, 10, 11, 14, 17, 28, 29]` | `finite-bounded-classifier-theorem-backed` | `digit8TrailingWidth6_reverseOnly_mem_iff_mod31` |

#### Digit-8 Classifier Family Replication

This block tests the three theorem-backed digit-8 classifier cells on the next twelve twin-prime pairs only. `Retained` means the source residue mask remains exact; `split` means it stays separated but changes residue classes; `collapsed` means overlap or no reverse-only row appears. This is a bounded replication screen, not a connector law.

- Replication decision: `digit8-classifier-family-partly-collapses-on-outside-ladder`
- Selection rule: `theorem-backed-digit8-classifier-cells-tested-on-next-twelve-twin-prime-pairs-only`
- Baseline pairs: `12`; widened pairs: `24`; added outside-ladder pairs: `12`
- Tested cells: `3`; retained: `0`; split: `0`; collapsed: `3`

| Edge | Width | Connector | Source moduli | Outside reverse-only pairs | Retained moduli | Split moduli | Collapsed moduli | Status | Summary theorem |
|---|---:|---|---|---:|---:|---:|---:|---|---|
| `leading` | 6 | `800000` | `[17, 23, 29, 31]` | 0 | 0 | 0 | 4 | `collapsed-at-some-source-modulus-outside-ladder` | `digit8LeadingWidth6_reverseOnly_multiModulusClassifier` |
| `trailing` | 6 | `000008` | `[17, 19, 29, 31]` | 0 | 0 | 0 | 4 | `collapsed-at-some-source-modulus-outside-ladder` | `digit8TrailingWidth6_reverseOnly_multiModulusClassifier` |
| `trailing` | 5 | `00008` | `[19, 29, 31]` | 2 | 0 | 2 | 1 | `collapsed-at-some-source-modulus-outside-ladder` | `digit8TrailingWidth5_reverseOnly_multiModulusClassifier` |

##### Outside-Ladder Modulus Results

| Edge | Width | Connector | Modulus | Source residues | Outside reverse-only residues | Shared residues | Status |
|---|---:|---|---:|---|---|---|---|
| `leading` | 6 | `800000` | 17 | `[0, 3]` | `[]` | `[]` | `collapsed-no-reverse-only-outside-ladder` |
| `leading` | 6 | `800000` | 23 | `[2, 3, 17]` | `[]` | `[]` | `collapsed-no-reverse-only-outside-ladder` |
| `leading` | 6 | `800000` | 29 | `[3, 13, 17]` | `[]` | `[]` | `collapsed-no-reverse-only-outside-ladder` |
| `leading` | 6 | `800000` | 31 | `[3, 9, 17]` | `[]` | `[]` | `collapsed-no-reverse-only-outside-ladder` |
| `trailing` | 6 | `000008` | 17 | `[1, 13, 16]` | `[]` | `[]` | `collapsed-no-reverse-only-outside-ladder` |
| `trailing` | 6 | `000008` | 19 | `[4, 6, 16]` | `[]` | `[]` | `collapsed-no-reverse-only-outside-ladder` |
| `trailing` | 6 | `000008` | 29 | `[4, 14, 21]` | `[]` | `[]` | `collapsed-no-reverse-only-outside-ladder` |
| `trailing` | 6 | `000008` | 31 | `[8, 13, 25]` | `[]` | `[]` | `collapsed-no-reverse-only-outside-ladder` |
| `trailing` | 5 | `00008` | 19 | `[6, 10, 11, 12]` | `[3, 5]` | `[5]` | `collapsed-overlapping-residue-classes-outside-ladder` |
| `trailing` | 5 | `00008` | 29 | `[0, 11, 14, 20]` | `[8, 28]` | `[]` | `split-exact-separator-outside-ladder` |
| `trailing` | 5 | `00008` | 31 | `[8, 11, 14, 29]` | `[6, 21]` | `[]` | `split-exact-separator-outside-ladder` |

##### Split-Exact Separator Follow-Up

This block ignores the collapsed family story and follows only first outside-ladder rows that remained exact separators with changed residue classes. It tests those split rows on the next twelve twin-prime pairs before proposing any new Lean classifier.

- Follow-up decision: `split-signal-partly-collapses-on-second-outside-ladder`
- Selection rule: `outside-ladder-split-exact-separator-rows-tested-on-next-twelve-twin-prime-pairs-only`
- Source split rows: `2`; tested rows: `2`; follow-up pairs: `12`
- Stabilized: `0`; split again: `0`; collapsed: `2`

| Edge | Width | Connector | Modulus | First outside residues | Follow-up reverse-only residues | Shared residues | Reverse-only pairs | Status |
|---|---:|---|---:|---|---|---|---:|---|
| `trailing` | 5 | `00008` | 29 | `[8, 28]` | `[]` | `[]` | 0 | `collapsed-no-reverse-only-on-second-outside-ladder` |
| `trailing` | 5 | `00008` | 31 | `[6, 21]` | `[]` | `[]` | 0 | `collapsed-no-reverse-only-on-second-outside-ladder` |

### Independent Branch Replication

This block tests the branch picker's selected singleton branch on the next disjoint twin-prime ladder. The outcome is interpreted through the existing exact mod-3 guardrail; it is not a connector law and does not claim a prime-density mechanism.

- Source branch: `trailing-edge-width8-digit6-connector-00000006` status `needs-independent-replication`
- Target cell: `trailing` edge, width `8`, position `7`, digit `6`, connector `00000006`
- Fresh ladder pairs: `12`; theorem-blocked rows `12`; nonblocked rows `0`; reverse-only rows `0`
- Exact layer decision: `decimal-connector-multiple-of-3-null-layer-blocks-every-fresh-twin-pair-above-3`
- Replication decision: `retired-all-fresh-independent-rows-theorem-blocked-by-mod3-null-layer`
- Next experiment target: `select-new-non-mod3-connector-stress-family-after-00000006-retirement`

| Pair | Mod-3 blocked | Contrast | Forward prime | Reverse prime |
|---|---:|---|---:|---:|
| `Twin-prime ladder pair (1031 ∘ 1033)` | true | `neither-prime-hit` | false | false |
| `Twin-prime ladder pair (1049 ∘ 1051)` | true | `neither-prime-hit` | false | false |
| `Twin-prime ladder pair (1061 ∘ 1063)` | true | `neither-prime-hit` | false | false |
| `Twin-prime ladder pair (1091 ∘ 1093)` | true | `neither-prime-hit` | false | false |
| `Twin-prime ladder pair (1151 ∘ 1153)` | true | `neither-prime-hit` | false | false |
| `Twin-prime ladder pair (1229 ∘ 1231)` | true | `neither-prime-hit` | false | false |
| `Twin-prime ladder pair (1277 ∘ 1279)` | true | `neither-prime-hit` | false | false |
| `Twin-prime ladder pair (1289 ∘ 1291)` | true | `neither-prime-hit` | false | false |
| `Twin-prime ladder pair (1301 ∘ 1303)` | true | `neither-prime-hit` | false | false |
| `Twin-prime ladder pair (1319 ∘ 1321)` | true | `neither-prime-hit` | false | false |
| `Twin-prime ladder pair (1427 ∘ 1429)` | true | `neither-prime-hit` | false | false |
| `Twin-prime ladder pair (1451 ∘ 1453)` | true | `neither-prime-hit` | false | false |

### Non-Mod3 Candidate Picker

This block selects a fresh connector stress family only after the previous singleton branch is retired by the mod-3 guardrail. It excludes decimal connectors divisible by `3` and the stopped digit-8 classifier branch, then ranks remaining edge cells by fresh-ladder reverse-only evidence. It is an empirical routing surface, not a connector law.

- Picker decision: `fresh-nonmod3-candidate-selected-after-retiring-collapsed-separator`
- Fresh pairs: `12`; rows `600`; nonblocked rows `600`; theorem-blocked rows `0`
- Ranked cells: `50`; cells with reverse-only hits `29`
- Retired candidates: `1` `["trailing-edge-width7-digit7-connector-0000007"]`
- Selected non-mod3 candidate: `trailing` edge width `7` digit `1` connector `0000001` reverse-only pairs `4` target `independently-replicate-nonmod3-0000001-trailing-edge-width7-digit1`

| Retired candidate | Source modulus | Source residues | Third-ladder reverse-only residues | Shared residues | Decision | Reason |
|---|---:|---|---|---|---|---|
| `trailing-edge-width7-digit7-connector-0000007` | 19 | `[1, 12]` | `[2]` | `[2]` | `mod19-residue-separator-collapsed-keep-empirical` | third disjoint ladder overlapped the candidate reverse-only residue with comparison rows |

| Rank | Edge | Width | Digit | Connector | Nonblocked pairs | Reverse-only pairs | Forward-only pairs | Both-hit pairs | Neither-hit pairs | Strongest pair | Status |
|---:|---|---:|---:|---|---:|---:|---:|---:|---:|---|---|

### Non-Mod3 Second Independent Replication

This block tests the selected non-mod3 candidate on the next disjoint twin-prime ladder before any residue profiler or Lean classifier is added. Survival here is still empirical routing, not a connector law.

- Source candidate: `trailing-edge-width7-digit1-connector-0000001` rank `2`
- Target cell: `trailing` edge, width `7`, position `6`, digit `1`, connector `0000001`
- Fresh ladder pairs: `12`; rows `12`; nonblocked rows `12`; theorem-blocked rows `0`
- Hit classes: reverse-only `1`; forward-only `1`; both `0`; neither `10`
- Replication decision: `survived-second-independent-ladder-residue-profiler-next`
- Next experiment target: `residue-profile-nonmod3-0000001-trailing-edge-width7-digit1`

| Pair | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---|---:|---:|---:|
| `Twin-prime ladder pair (1481 ∘ 1483)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (1487 ∘ 1489)` | `reverse-only-prime-hit` | false | true | -5.581249 |
| `Twin-prime ladder pair (1607 ∘ 1609)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (1619 ∘ 1621)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (1667 ∘ 1669)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (1697 ∘ 1699)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (1721 ∘ 1723)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (1787 ∘ 1789)` | `forward-only-prime-hit` | true | false | 5.612449 |
| `Twin-prime ladder pair (1871 ∘ 1873)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (1877 ∘ 1879)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (1931 ∘ 1933)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (1949 ∘ 1951)` | `neither-prime-hit` | false | false | 0.000000 |

### Non-Mod3 Residue-Class Profile

This block profiles the second-ladder survivor by small-prime residue classes. It is a theorem-candidate screen only after independent survival; it is not a connector law and does not add Lean metadata in this tranche.

- Source candidate: `trailing-edge-width7-digit1-connector-0000001`
- Profile decision: `small-prime-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `1`; comparison pairs `11`; exact separators `5`
- Best separator: mod `17` reverse-only residues `[8]` target `replicate-nonmod3-0000001-mod17-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[2]` | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[2]` | `[1, 2, 4]` | `[2]` | `overlapping-residue-classes` |
| 7 | `[3]` | `[1, 2, 3, 4, 6]` | `[3]` | `overlapping-residue-classes` |
| 11 | `[2]` | `[1, 2, 3, 5, 6, 7]` | `[2]` | `overlapping-residue-classes` |
| 13 | `[5]` | `[3, 5, 6, 7, 8, 12]` | `[5]` | `overlapping-residue-classes` |
| 17 | `[8]` | `[1, 2, 4, 7, 9, 10, 11, 14]` | `[]` | `exact-residue-separator` |
| 19 | `[5]` | `[1, 4, 6, 9, 11, 12, 14, 15, 18]` | `[]` | `exact-residue-separator` |
| 23 | `[15]` | `[8, 9, 11, 14, 16, 17, 18, 19, 20, 22]` | `[]` | `exact-residue-separator` |
| 29 | `[8]` | `[2, 6, 10, 12, 14, 15, 17, 18, 21, 24]` | `[]` | `exact-residue-separator` |
| 31 | `[30]` | `[7, 9, 11, 16, 17, 20, 23, 24, 26, 27]` | `[]` | `exact-residue-separator` |

### Non-Mod3 Residue-Separator Replication

This block tests the best second-ladder residue separator on a third disjoint twin-prime ladder. A retained separator can become a finite Lean classifier candidate; a split or collapsed separator stays empirical and routes onward without theorem metadata.

- Source candidate: `trailing-edge-width7-digit1-connector-0000001`
- Tested separator: mod `17` source reverse-only residues `[8]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `3`; comparison rows `9`
- Third-ladder reverse-only residues: `[1, 10]`; comparison residues `[3, 4, 6, 7, 8, 13, 14, 16]`; shared residues `[]`
- Separator status: `split-exact-residue-separator-on-third-ladder`
- Replication decision: `mod17-residue-separator-split-keep-empirical`
- Next experiment target: `replicate-mutated-nonmod3-0000001-mod17-residue-separator-before-lean`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---:|---|---:|---:|---:|
| `Twin-prime ladder pair (1997 ∘ 1999)` | 8 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2027 ∘ 2029)` | 4 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2081 ∘ 2083)` | 7 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2087 ∘ 2089)` | 13 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2111 ∘ 2113)` | 3 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2129 ∘ 2131)` | 4 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2141 ∘ 2143)` | 16 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2237 ∘ 2239)` | 10 | `reverse-only-prime-hit` | false | true | -5.651013 |
| `Twin-prime ladder pair (2267 ∘ 2269)` | 6 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2309 ∘ 2311)` | 14 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2339 ∘ 2341)` | 10 | `reverse-only-prime-hit` | false | true | -5.658632 |
| `Twin-prime ladder pair (2381 ∘ 2383)` | 1 | `reverse-only-prime-hit` | false | true | -5.661674 |

### Non-Mod3 Mutated Residue-Separator Replication

This block tests a split third-ladder residue separator on a fourth disjoint twin-prime ladder. Stabilization would justify a finite classifier candidate; another split or collapse retires the branch and keeps the signal empirical.

- Source candidate: `trailing-edge-width7-digit1-connector-0000001`
- Mutated separator under test: mod `17` reverse-only residues `[1, 10]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `0`; comparison rows `12`
- Fourth-ladder reverse-only residues: `[]`; comparison residues `[1, 5, 7, 8, 9, 11, 13, 16]`; shared residues `[]`
- Separator status: `collapsed-no-reverse-only-on-fourth-ladder`
- Replication decision: `mod17-mutated-residue-separator-collapsed-retire-branch`
- Next experiment target: `select-next-nonmod3-connector-stress-family-after-0000001-retirement`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---:|---|---:|---:|---:|
| `Twin-prime ladder pair (2549 ∘ 2551)` | 16 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2591 ∘ 2593)` | 7 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2657 ∘ 2659)` | 5 | `forward-only-prime-hit` | true | false | 5.680287 |
| `Twin-prime ladder pair (2687 ∘ 2689)` | 1 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2711 ∘ 2713)` | 8 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2729 ∘ 2731)` | 9 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2789 ∘ 2791)` | 1 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2801 ∘ 2803)` | 13 | `both-prime-hit` | true | true | -0.000122 |
| `Twin-prime ladder pair (2969 ∘ 2971)` | 11 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (2999 ∘ 3001)` | 7 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (3119 ∘ 3121)` | 8 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (3167 ∘ 3169)` | 5 | `neither-prime-hit` | false | false | 0.000000 |

### Non-Mod3 Next Candidate Picker

This block runs the same non-mod3 picker after retiring the latest branch. It names the next empirical family to independently replicate before any residue profiler or Lean classifier work.

- Picker decision: `fresh-nonmod3-candidate-selected-after-retiring-collapsed-separator`
- Retired candidates: `2` `["trailing-edge-width7-digit7-connector-0000007", "trailing-edge-width7-digit1-connector-0000001"]`
- Selected next non-mod3 candidate: `trailing` edge width `8` digit `5` connector `00000005` reverse-only pairs `3` target `independently-replicate-nonmod3-00000005-trailing-edge-width8-digit5`

| Rank | Edge | Width | Digit | Connector | Reverse-only pairs | Forward-only pairs | Both-hit pairs | Neither-hit pairs | Status |
|---:|---|---:|---:|---|---:|---:|---:|---:|---|

### Non-Mod3 Next Candidate Independent Replication

This block tests the selected next non-mod3 candidate on a fresh disjoint twin-prime ladder. A survivor can be residue-profiled later; a collapse retires the branch without Lean metadata.

- Source candidate: `trailing-edge-width8-digit5-connector-00000005` rank `3`
- Target cell: `trailing` edge, width `8`, position `7`, digit `5`, connector `00000005`
- Fresh ladder pairs: `12`; rows `12`; nonblocked rows `12`; theorem-blocked rows `0`
- Hit classes: reverse-only `0`; forward-only `1`; both `1`; neither `10`
- Replication decision: `collapsed-on-second-independent-ladder-retire-without-lean`
- Next experiment target: `retire-nonmod3-00000005-trailing-edge-width8-digit5`

| Pair | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---|---:|---:|---:|
| `Twin-prime ladder pair (3251 ∘ 3253)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (3257 ∘ 3259)` | `forward-only-prime-hit` | true | false | 6.108907 |
| `Twin-prime ladder pair (3299 ∘ 3301)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (3329 ∘ 3331)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (3359 ∘ 3361)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (3371 ∘ 3373)` | `both-prime-hit` | true | true | -0.000101 |
| `Twin-prime ladder pair (3389 ∘ 3391)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (3461 ∘ 3463)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (3467 ∘ 3469)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (3527 ∘ 3529)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (3539 ∘ 3541)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (3557 ∘ 3559)` | `neither-prime-hit` | false | false | 0.000000 |

### Non-Mod3 Retirement Summary

This block records edge/singleton non-mod3 branches that were retired before further theorem work. It keeps collapsed empirical paths visible instead of silently deleting them.

| Candidate | Edge | Width | Position | Digit | Connector | Decision | Reason |
|---|---|---:|---:|---:|---|---|---|
| `trailing-edge-width7-digit1-connector-0000001` | `trailing` | 7 | 6 | 1 | `0000001` | `mod17-mutated-residue-separator-collapsed-retire-branch` | mutated separator failed to stabilize on the next disjoint twin-prime ladder |
| `trailing-edge-width7-digit7-connector-0000007` | `trailing` | 7 | 6 | 7 | `0000007` | `mod19-residue-separator-collapsed-keep-empirical` | third disjoint ladder overlapped the candidate reverse-only residue with comparison rows |
| `trailing-edge-width8-digit5-connector-00000005` | `trailing` | 8 | 7 | 5 | `00000005` | `collapsed-on-second-independent-ladder-retire-without-lean` | second independent ladder produced no reverse-only rows; branch retired before residue profiling or Lean metadata |

### Interior Non-Mod3 Family Picker

This block leaves the collapsed edge-singleton surface and scans interior single-digit connector positions on a fresh disjoint twin-prime ladder. A selected family must still survive independent replication before residue profiling or Lean work.

- Picker decision: `interior-nonmod3-family-selected-for-independent-replication`
- Source pairs: `12`; rows `1500`; nonblocked rows `1500`; ranked cells `125`; reverse-only cells `82`
- Retired candidates excluded: `0` `[]`
- Selected interior candidate: width `5` position `3` digit `1` connector `00010` reverse-only pairs `5` target `independently-replicate-interior-nonmod3-00010-width5-position3-digit1`

| Rank | Width | Position | Digit | Connector | Reverse-only pairs | Forward-only pairs | Both-hit pairs | Neither-hit pairs | Status |
|---:|---:|---:|---:|---|---:|---:|---:|---:|---|

### Interior Non-Mod3 Independent Replication

This block tests the selected interior family on the next disjoint twin-prime ladder. Survival only opens a residue-profiler target; it is not promoted to a Lean classifier here.

- Source candidate: `interior-width5-position3-digit1-connector-00010` rank `1`
- Selection rule: `selected-interior-nonmod3-family-on-next-twelve-disjoint-twin-prime-pairs-after-4127-4129`
- Target cell: width `5`, position `3`, digit `1`, connector `00010`
- Fresh ladder pairs: `12`; rows `12`; nonblocked rows `12`; theorem-blocked rows `0`
- Hit classes: reverse-only `2`; forward-only `0`; both `0`; neither `10`
- Replication decision: `survived-interior-independent-ladder-residue-profiler-next`
- Next experiment target: `residue-profile-interior-nonmod3-00010-width5-position3-digit1`

| Pair | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---|---:|---:|---:|
| `Twin-prime ladder pair (4157 ∘ 4159)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (4217 ∘ 4219)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (4229 ∘ 4231)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (4241 ∘ 4243)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (4259 ∘ 4261)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (4271 ∘ 4273)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (4337 ∘ 4339)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (4421 ∘ 4423)` | `reverse-only-prime-hit` | false | true | -4.979850 |
| `Twin-prime ladder pair (4481 ∘ 4483)` | `reverse-only-prime-hit` | false | true | -4.982154 |
| `Twin-prime ladder pair (4517 ∘ 4519)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (4547 ∘ 4549)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (4637 ∘ 4639)` | `neither-prime-hit` | false | false | 0.000000 |

### Interior Non-Mod3 Residue-Class Profile

This block profiles the survived interior `00010` branch by small-prime residue classes. It is a theorem-candidate screen only if the best separator survives one more disjoint ladder unchanged.

- Source candidate: `interior-width5-position3-digit1-connector-00010`
- Profile decision: `small-prime-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `2`; comparison pairs `10`; exact separators `4`
- Best separator: mod `19` reverse-only residues `[13, 16]` target `replicate-interior-nonmod3-00010-mod19-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[2]` | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[1]` | `[1, 2, 4]` | `[1]` | `overlapping-residue-classes` |
| 7 | `[1, 4]` | `[1, 2, 3, 4, 6]` | `[1, 4]` | `overlapping-residue-classes` |
| 11 | `[4, 10]` | `[2, 3, 4, 5, 6, 7, 10]` | `[4, 10]` | `overlapping-residue-classes` |
| 13 | `[1, 9]` | `[3, 4, 5, 6, 7, 8, 9, 10]` | `[9]` | `overlapping-residue-classes` |
| 17 | `[1, 10]` | `[1, 2, 4, 8, 9, 12, 13]` | `[1]` | `overlapping-residue-classes` |
| 19 | `[13, 16]` | `[1, 3, 4, 5, 6, 11, 14, 15, 18]` | `[]` | `exact-residue-separator` |
| 23 | `[5, 19]` | `[4, 8, 9, 13, 14, 16, 17, 20]` | `[]` | `exact-residue-separator` |
| 29 | `[13, 15]` | `[7, 8, 10, 12, 16, 22, 23, 24, 25, 26]` | `[]` | `exact-residue-separator` |
| 31 | `[17, 19]` | `[1, 3, 12, 13, 18, 21, 22, 24, 25, 28]` | `[]` | `exact-residue-separator` |

### Interior Non-Mod3 Residue-Separator Replication

This block tests the best interior residue separator on one more disjoint twin-prime ladder. A retained separator becomes Lean-candidate material; a mutated or collapsed separator retires the branch without theorem metadata.

- Source candidate: `interior-width5-position3-digit1-connector-00010`
- Tested separator: mod `19` source reverse-only residues `[13, 16]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `4`; comparison rows `8`
- Fresh-ladder reverse-only residues: `[2, 10, 13, 18]`; comparison residues `[5, 6, 7, 8, 9, 11, 12, 16]`; shared residues `[]`
- Separator status: `split-exact-residue-separator-on-interior-separator-ladder`; retained residues `1`; split residues `3`; collapsed source residues `1`
- Replication decision: `mod19-interior-residue-separator-mutated-retire-branch`
- Next experiment target: `retire-interior-nonmod3-00010-mod19-residue-separator-after-mutation`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---:|---|---:|---:|---:|
| `Twin-prime ladder pair (4649 ∘ 4651)` | 13 | `reverse-only-prime-hit` | false | true | -4.988446 |
| `Twin-prime ladder pair (4721 ∘ 4723)` | 9 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (4787 ∘ 4789)` | 18 | `reverse-only-prime-hit` | false | true | -4.993447 |
| `Twin-prime ladder pair (4799 ∘ 4801)` | 11 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (4931 ∘ 4933)` | 10 | `reverse-only-prime-hit` | false | true | -4.998514 |
| `Twin-prime ladder pair (4967 ∘ 4969)` | 8 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5009 ∘ 5011)` | 12 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5021 ∘ 5023)` | 5 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5099 ∘ 5101)` | 7 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5231 ∘ 5233)` | 6 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5279 ∘ 5281)` | 16 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5417 ∘ 5419)` | 2 | `reverse-only-prime-hit` | false | true | -5.014584 |

### Interior Non-Mod3 Retirement Summary

This block records interior non-mod3 branches retired after the replicate-before-Lean guardrail. Retired candidates are excluded from the next interior picker rather than silently reconsidered.

| Candidate | Width | Position | Digit | Connector | Decision | Reason |
|---|---:|---:|---:|---|---|---|
| `interior-width5-position1-digit4-connector-04000` | 5 | 1 | 4 | `04000` | `mod17-interior-residue-separator-mutated-retire-branch` | interior residue separator failed to stabilize on the next disjoint twin-prime ladder; branch retired before Lean metadata |
| `interior-width5-position1-digit5-connector-05000` | 5 | 1 | 5 | `05000` | `mod11-interior-residue-separator-collapsed-retire-branch` | interior residue separator failed to stabilize on the next disjoint twin-prime ladder; branch retired before Lean metadata |
| `interior-width5-position3-digit1-connector-00010` | 5 | 3 | 1 | `00010` | `mod19-interior-residue-separator-mutated-retire-branch` | interior residue separator failed to stabilize on the next disjoint twin-prime ladder; branch retired before Lean metadata |
| `interior-width7-position5-digit7-connector-0000070` | 7 | 5 | 7 | `0000070` | `mod17-interior-residue-separator-collapsed-retire-branch` | interior residue separator failed to stabilize on the next disjoint twin-prime ladder; branch retired before Lean metadata |
| `interior-width9-position4-digit5-connector-000050000` | 9 | 4 | 5 | `000050000` | `mod13-interior-residue-separator-collapsed-retire-branch` | interior residue separator failed to stabilize on the next disjoint twin-prime ladder; branch retired before Lean metadata |
| `interior-width9-position5-digit7-connector-000007000` | 9 | 5 | 7 | `000007000` | `collapsed-interior-independent-ladder-retire-without-lean` | interior candidate failed fresh-ladder survival; branch retired before residue profiling or Lean metadata |
| `interior-width9-position7-digit7-connector-000000070` | 9 | 7 | 7 | `000000070` | `collapsed-interior-independent-ladder-retire-without-lean` | interior candidate failed fresh-ladder survival; branch retired before residue profiling or Lean metadata |

### Interior Non-Mod3 Next Family Picker

This block reruns the interior picker after excluding retired interior candidates. The selected row still must survive an independent ladder before residue profiling or Lean work.

- Picker decision: `interior-nonmod3-family-selected-for-independent-replication`
- Selection rule: `after-interior-residue-separator-retirement;exclude-retired-interior-candidates;widths-5-through-9;interior-positions-only;digits-1-2-4-5-7;rank-by-reverse-only-count-gap`
- Retired candidates excluded: `1` `["interior-width5-position3-digit1-connector-00010"]`
- Source pairs: `12`; rows `1488`; nonblocked rows `1488`; ranked cells `124`; reverse-only cells `81`
- Selected next interior candidate: width `7` position `5` digit `7` connector `0000070` reverse-only pairs `4` target `independently-replicate-interior-nonmod3-0000070-width7-position5-digit7`

| Rank | Width | Position | Digit | Connector | Reverse-only pairs | Forward-only pairs | Both-hit pairs | Neither-hit pairs | Status |
|---:|---:|---:|---:|---|---:|---:|---:|---:|---|

### Interior Non-Mod3 Next Independent Replication

This block tests the selected next interior family on a fresh disjoint twin-prime ladder. Survival only opens the next residue-profiler target; it is not theorem metadata.

- Source candidate: `interior-width7-position5-digit7-connector-0000070` rank `1`
- Selection rule: `selected-next-interior-nonmod3-family-on-next-twelve-disjoint-twin-prime-pairs-after-5417-5419`
- Target cell: width `7`, position `5`, digit `7`, connector `0000070`
- Fresh ladder pairs: `12`; rows `12`; nonblocked rows `12`; theorem-blocked rows `0`
- Hit classes: reverse-only `3`; forward-only `3`; both `0`; neither `6`
- Replication decision: `survived-interior-independent-ladder-residue-profiler-next`
- Next experiment target: `residue-profile-interior-nonmod3-0000070-width7-position5-digit7`

| Pair | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---|---:|---:|---:|
| `Twin-prime ladder pair (5441 ∘ 5443)` | `reverse-only-prime-hit` | false | true | -5.802934 |
| `Twin-prime ladder pair (5477 ∘ 5479)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5501 ∘ 5503)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5519 ∘ 5521)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5639 ∘ 5641)` | `forward-only-prime-hit` | true | false | 5.808985 |
| `Twin-prime ladder pair (5651 ∘ 5653)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5657 ∘ 5659)` | `reverse-only-prime-hit` | false | true | -5.809590 |
| `Twin-prime ladder pair (5741 ∘ 5743)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5849 ∘ 5851)` | `reverse-only-prime-hit` | false | true | -5.815296 |
| `Twin-prime ladder pair (5867 ∘ 5869)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (5879 ∘ 5881)` | `forward-only-prime-hit` | true | false | 5.816113 |
| `Twin-prime ladder pair (6089 ∘ 6091)` | `forward-only-prime-hit` | true | false | 5.822115 |

### Interior Non-Mod3 Next Residue-Class Profile

This block profiles the survived next interior branch by small-prime residue classes. It remains theorem-candidate material only if the best separator survives one more disjoint ladder unchanged.

- Source candidate: `interior-width7-position5-digit7-connector-0000070`
- Profile decision: `small-prime-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `3`; comparison pairs `9`; exact separators `2`
- Best separator: mod `17` reverse-only residues `[1, 13]` target `replicate-interior-nonmod3-0000070-mod17-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[2]` | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[1, 2, 4]` | `[1, 2, 4]` | `[1, 2, 4]` | `overlapping-residue-classes` |
| 7 | `[1, 2, 4]` | `[1, 2, 3, 4, 6]` | `[1, 2, 4]` | `overlapping-residue-classes` |
| 11 | `[3, 7, 8]` | `[1, 4, 5, 6, 7, 8, 10]` | `[7, 8]` | `overlapping-residue-classes` |
| 13 | `[2, 7, 12]` | `[2, 3, 4, 5, 7, 8, 9, 10]` | `[2, 7]` | `overlapping-residue-classes` |
| 17 | `[1, 13]` | `[2, 3, 7, 10, 11, 12, 14]` | `[]` | `exact-residue-separator` |
| 19 | `[7, 14, 16]` | `[3, 5, 8, 9, 10, 15]` | `[]` | `exact-residue-separator` |
| 23 | `[7, 13, 22]` | `[2, 3, 4, 14, 16, 17, 22]` | `[22]` | `overlapping-residue-classes` |
| 29 | `[2, 18, 20]` | `[9, 13, 20, 21, 25, 28]` | `[20]` | `overlapping-residue-classes` |
| 31 | `[15, 16, 21]` | `[1, 6, 8, 9, 13, 14, 20, 21, 28]` | `[21]` | `overlapping-residue-classes` |

### Interior Non-Mod3 Next Residue-Separator Replication

This block tests the best next-interior residue separator on one more disjoint twin-prime ladder. A retained separator becomes Lean-candidate material; mutation or overlap retires the branch.

- Source candidate: `interior-width7-position5-digit7-connector-0000070`
- Selection rule: `test-best-next-interior-mod17-separator-on-next-disjoint-twelve-twin-prime-pairs`
- Tested separator: mod `17` source reverse-only residues `[1, 13]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `4`; comparison rows `8`
- Fresh-ladder reverse-only residues: `[8, 9, 11, 12]`; comparison residues `[1, 3, 6, 7, 9, 12, 13]`; shared residues `[9, 12]`
- Separator status: `collapsed-overlapping-residue-classes-on-interior-separator-ladder`; retained residues `0`; split residues `4`; collapsed source residues `2`
- Replication decision: `mod17-interior-residue-separator-collapsed-retire-branch`
- Next experiment target: `retire-interior-nonmod3-0000070-mod17-residue-separator-after-mutation`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---:|---|---:|---:|---:|
| `Twin-prime ladder pair (6131 ∘ 6133)` | 11 | `reverse-only-prime-hit` | false | true | -5.823347 |
| `Twin-prime ladder pair (6197 ∘ 6199)` | 9 | `reverse-only-prime-hit` | false | true | -5.825177 |
| `Twin-prime ladder pair (6269 ∘ 6271)` | 13 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6299 ∘ 6301)` | 9 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6359 ∘ 6361)` | 1 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6449 ∘ 6451)` | 6 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6551 ∘ 6553)` | 6 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6569 ∘ 6571)` | 7 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6659 ∘ 6661)` | 12 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6689 ∘ 6691)` | 8 | `reverse-only-prime-hit` | false | true | -5.838239 |
| `Twin-prime ladder pair (6701 ∘ 6703)` | 3 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6761 ∘ 6763)` | 12 | `reverse-only-prime-hit` | false | true | -5.840070 |

### Interior Non-Mod3 Post-Retirement Family Picker

This block reruns the interior picker after retiring both `00010` and `0000070`. The selected family must survive a fresh ladder before residue profiling; no theorem work is attached here.

- Picker decision: `interior-nonmod3-family-selected-for-independent-replication`
- Selection rule: `after-second-interior-residue-separator-retirement;exclude-retired-interior-candidates;widths-5-through-9;interior-positions-only;digits-1-2-4-5-7;rank-by-reverse-only-count-gap`
- Retired candidates excluded: `2` `["interior-width5-position3-digit1-connector-00010", "interior-width7-position5-digit7-connector-0000070"]`
- Source pairs: `12`; rows `1476`; nonblocked rows `1476`; ranked cells `123`; reverse-only cells `80`
- Selected post-retirement interior candidate: width `5` position `1` digit `4` connector `04000` reverse-only pairs `4` target `independently-replicate-interior-nonmod3-04000-width5-position1-digit4`

| Rank | Width | Position | Digit | Connector | Reverse-only pairs | Forward-only pairs | Both-hit pairs | Neither-hit pairs | Status |
|---:|---:|---:|---:|---|---:|---:|---:|---:|---|

### Interior Non-Mod3 Post-Retirement Independent Replication

This block tests the post-retirement interior family on the next disjoint twin-prime ladder. Survival opens a residue-profiler target only; it is not Lean metadata and not a connector law.

- Source candidate: `interior-width5-position1-digit4-connector-04000` rank `1`
- Selection rule: `selected-post-retirement-interior-nonmod3-family-on-next-twelve-disjoint-twin-prime-pairs-after-6761-6763`
- Target cell: width `5`, position `1`, digit `4`, connector `04000`
- Fresh ladder pairs: `12`; rows `12`; nonblocked rows `12`; theorem-blocked rows `0`
- Hit classes: reverse-only `1`; forward-only `0`; both `0`; neither `11`
- Replication decision: `survived-interior-independent-ladder-residue-profiler-next`
- Next experiment target: `residue-profile-interior-nonmod3-04000-width5-position1-digit4`

| Pair | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---|---:|---:|---:|
| `Twin-prime ladder pair (6779 ∘ 6781)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6791 ∘ 6793)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6827 ∘ 6829)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6869 ∘ 6871)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6947 ∘ 6949)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (6959 ∘ 6961)` | `reverse-only-prime-hit` | false | true | -5.057411 |
| `Twin-prime ladder pair (7127 ∘ 7129)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (7211 ∘ 7213)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (7307 ∘ 7309)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (7331 ∘ 7333)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (7349 ∘ 7351)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (7457 ∘ 7459)` | `neither-prime-hit` | false | false | 0.000000 |

### Interior Non-Mod3 Post-Retirement Residue-Class Profile

This block profiles the survived post-retirement interior branch by small-prime residue classes. It remains theorem-candidate material only if the best separator survives one more disjoint ladder unchanged.

- Source candidate: `interior-width5-position1-digit4-connector-04000`
- Profile decision: `small-prime-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `1`; comparison pairs `11`; exact separators `3`
- Best separator: mod `17` reverse-only residues `[6]` target `replicate-interior-nonmod3-04000-mod17-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[2]` | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[4]` | `[1, 2, 4]` | `[4]` | `overlapping-residue-classes` |
| 7 | `[1]` | `[1, 2, 3, 6]` | `[1]` | `overlapping-residue-classes` |
| 11 | `[7]` | `[1, 3, 4, 5, 6, 7, 10]` | `[7]` | `overlapping-residue-classes` |
| 13 | `[4]` | `[1, 2, 3, 4, 5, 6, 8, 9, 12]` | `[4]` | `overlapping-residue-classes` |
| 17 | `[6]` | `[1, 3, 4, 5, 8, 10, 11, 13, 14]` | `[]` | `exact-residue-separator` |
| 19 | `[5]` | `[2, 6, 8, 9, 10, 11, 12, 15, 16]` | `[]` | `exact-residue-separator` |
| 23 | `[13]` | `[1, 5, 6, 12, 15, 16, 17, 19, 20]` | `[]` | `exact-residue-separator` |
| 29 | `[28]` | `[4, 5, 12, 16, 19, 22, 23, 25, 28]` | `[28]` | `overlapping-residue-classes` |
| 31 | `[15]` | `[2, 3, 7, 15, 17, 18, 19, 21, 22, 28]` | `[15]` | `overlapping-residue-classes` |

### Interior Non-Mod3 Post-Retirement Residue-Separator Replication

This block tests the best post-retirement residue separator on one more disjoint twin-prime ladder. A retained separator becomes theorem-candidate material; mutation or overlap retires the branch.

- Source candidate: `interior-width5-position1-digit4-connector-04000`
- Selection rule: `test-best-post-retirement-interior-residue-separator-on-next-disjoint-twelve-twin-prime-pairs`
- Tested separator: mod `17` source reverse-only residues `[6]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `3`; comparison rows `9`
- Fresh-ladder reverse-only residues: `[3, 5, 11]`; comparison residues `[2, 6, 7, 8, 10, 12, 16]`; shared residues `[]`
- Separator status: `split-exact-residue-separator-on-interior-separator-ladder`; retained residues `0`; split residues `3`; collapsed source residues `1`
- Replication decision: `mod17-interior-residue-separator-mutated-retire-branch`
- Next experiment target: `retire-interior-nonmod3-04000-mod17-residue-separator-after-mutation`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---:|---|---:|---:|---:|
| `Twin-prime ladder pair (7487 ∘ 7489)` | 7 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (7547 ∘ 7549)` | 16 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (7559 ∘ 7561)` | 11 | `reverse-only-prime-hit` | false | true | -5.071551 |
| `Twin-prime ladder pair (7589 ∘ 7591)` | 7 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (7757 ∘ 7759)` | 5 | `reverse-only-prime-hit` | false | true | -5.075972 |
| `Twin-prime ladder pair (7877 ∘ 7879)` | 6 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (7949 ∘ 7951)` | 10 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8009 ∘ 8011)` | 2 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8087 ∘ 8089)` | 12 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8219 ∘ 8221)` | 8 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8231 ∘ 8233)` | 3 | `reverse-only-prime-hit` | false | true | -5.086114 |
| `Twin-prime ladder pair (8291 ∘ 8293)` | 12 | `neither-prime-hit` | false | false | 0.000000 |

### Interior Non-Mod3 After Third-Retirement Family Picker

This block reruns the interior picker after retiring `04000`, `00010`, and `0000070`. The selected family is tested only for fresh-ladder survival in this tranche; residue profiling waits for a subsequent step.

- Picker decision: `interior-nonmod3-family-selected-for-independent-replication`
- Selection rule: `after-third-interior-residue-separator-retirement;exclude-retired-interior-candidates;widths-5-through-9;interior-positions-only;digits-1-2-4-5-7;rank-by-reverse-only-count-gap`
- Retired candidates excluded: `3` `["interior-width5-position1-digit4-connector-04000", "interior-width5-position3-digit1-connector-00010", "interior-width7-position5-digit7-connector-0000070"]`
- Source pairs: `12`; rows `1464`; nonblocked rows `1464`; ranked cells `122`; reverse-only cells `79`
- Selected after-third-retirement interior candidate: width `5` position `1` digit `5` connector `05000` reverse-only pairs `4` target `independently-replicate-interior-nonmod3-05000-width5-position1-digit5`

| Rank | Width | Position | Digit | Connector | Reverse-only pairs | Forward-only pairs | Both-hit pairs | Neither-hit pairs | Status |
|---:|---:|---:|---:|---|---:|---:|---:|---:|---|

### Interior Non-Mod3 After Third-Retirement Independent Replication

This block tests the after-third-retirement interior family on the next disjoint twin-prime ladder. Survival opens a future residue-profiler target only; no residue classifier or Lean theorem is added here.

- Source candidate: `interior-width5-position1-digit5-connector-05000` rank `1`
- Selection rule: `selected-after-third-retirement-interior-nonmod3-family-on-next-twelve-disjoint-twin-prime-pairs-after-8291-8293`
- Target cell: width `5`, position `1`, digit `5`, connector `05000`
- Fresh ladder pairs: `12`; rows `12`; nonblocked rows `12`; theorem-blocked rows `0`
- Hit classes: reverse-only `1`; forward-only `0`; both `0`; neither `11`
- Replication decision: `survived-interior-independent-ladder-residue-profiler-next`
- Next experiment target: `residue-profile-interior-nonmod3-05000-width5-position1-digit5`

| Pair | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---|---:|---:|---:|
| `Twin-prime ladder pair (8387 ∘ 8389)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8429 ∘ 8431)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8537 ∘ 8539)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8597 ∘ 8599)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8627 ∘ 8629)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8819 ∘ 8821)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8837 ∘ 8839)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8861 ∘ 8863)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8969 ∘ 8971)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (8999 ∘ 9001)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9011 ∘ 9013)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9041 ∘ 9043)` | `reverse-only-prime-hit` | false | true | -5.102163 |

### Interior Non-Mod3 After Third-Retirement Residue-Class Profile

This block profiles the survived `05000` interior branch by small-prime residue classes. It remains theorem-candidate material only if the best separator survives one more disjoint ladder unchanged.

- Source candidate: `interior-width5-position1-digit5-connector-05000`
- Profile decision: `small-prime-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `1`; comparison pairs `11`; exact separators `5`
- Best separator: mod `11` reverse-only residues `[10]` target `replicate-interior-nonmod3-05000-mod11-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[2]` | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[1]` | `[1, 2, 4]` | `[1]` | `overlapping-residue-classes` |
| 7 | `[4]` | `[1, 2, 3, 4, 6]` | `[4]` | `overlapping-residue-classes` |
| 11 | `[10]` | `[1, 2, 3, 4, 5, 6, 8]` | `[]` | `exact-residue-separator` |
| 13 | `[6]` | `[2, 3, 4, 5, 8, 9, 10, 12]` | `[]` | `exact-residue-separator` |
| 17 | `[14]` | `[1, 3, 4, 6, 8, 10, 12, 13, 14]` | `[14]` | `overlapping-residue-classes` |
| 19 | `[16]` | `[1, 2, 3, 5, 6, 7, 8, 9, 12]` | `[]` | `exact-residue-separator` |
| 23 | `[2]` | `[2, 4, 5, 6, 10, 11, 15, 18, 22]` | `[2]` | `overlapping-residue-classes` |
| 29 | `[22]` | `[3, 6, 8, 9, 11, 13, 14, 16, 19, 21]` | `[]` | `exact-residue-separator` |
| 31 | `[20]` | `[2, 9, 10, 12, 15, 17, 21, 26, 28]` | `[]` | `exact-residue-separator` |

### Interior Non-Mod3 After Third-Retirement Residue-Separator Replication

This block tests the best `05000` residue separator on one more disjoint twin-prime ladder. A retained separator becomes theorem-candidate material; mutation or overlap retires the branch.

- Source candidate: `interior-width5-position1-digit5-connector-05000`
- Selection rule: `test-best-after-third-retirement-interior-residue-separator-on-next-disjoint-twelve-twin-prime-pairs`
- Tested separator: mod `11` source reverse-only residues `[10]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `0`; comparison rows `12`
- Fresh-ladder reverse-only residues: `[]`; comparison residues `[1, 2, 3, 4, 6, 8, 10]`; shared residues `[]`
- Separator status: `collapsed-no-reverse-only-on-interior-separator-ladder`; retained residues `0`; split residues `0`; collapsed source residues `1`
- Replication decision: `mod11-interior-residue-separator-collapsed-retire-branch`
- Next experiment target: `retire-interior-nonmod3-05000-mod11-residue-separator-after-mutation`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---:|---|---:|---:|---:|
| `Twin-prime ladder pair (9239 ∘ 9241)` | 10 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9281 ∘ 9283)` | 8 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9341 ∘ 9343)` | 2 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9419 ∘ 9421)` | 3 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9431 ∘ 9433)` | 4 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9437 ∘ 9439)` | 10 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9461 ∘ 9463)` | 1 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9629 ∘ 9631)` | 4 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9677 ∘ 9679)` | 8 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9719 ∘ 9721)` | 6 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9767 ∘ 9769)` | 10 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (9857 ∘ 9859)` | 1 | `neither-prime-hit` | false | false | 0.000000 |

### Interior Non-Mod3 After Fourth-Retirement Family Picker

This block reruns the interior picker after retiring `04000`, `05000`, `00010`, and `0000070`. The selected family is tested only for fresh-ladder survival here; residue profiling waits for a later tranche if it survives.

- Picker decision: `interior-nonmod3-family-selected-for-independent-replication`
- Selection rule: `after-fourth-interior-residue-separator-retirement;exclude-retired-interior-candidates;widths-5-through-9;interior-positions-only;digits-1-2-4-5-7;rank-by-reverse-only-count-gap`
- Retired candidates excluded: `4` `["interior-width5-position1-digit4-connector-04000", "interior-width5-position1-digit5-connector-05000", "interior-width5-position3-digit1-connector-00010", "interior-width7-position5-digit7-connector-0000070"]`
- Source pairs: `12`; rows `1452`; nonblocked rows `1452`; ranked cells `121`; reverse-only cells `78`
- Selected after-fourth-retirement interior candidate: width `9` position `4` digit `5` connector `000050000` reverse-only pairs `3` target `independently-replicate-interior-nonmod3-000050000-width9-position4-digit5`

| Rank | Width | Position | Digit | Connector | Reverse-only pairs | Forward-only pairs | Both-hit pairs | Neither-hit pairs | Status |
|---:|---:|---:|---:|---|---:|---:|---:|---:|---|

### Interior Non-Mod3 After Fourth-Retirement Independent Replication

This block tests the after-fourth-retirement interior family on the next disjoint twin-prime ladder. Survival opens a future residue-profiler target only; collapse retires the branch before theorem work.

- Source candidate: `interior-width9-position4-digit5-connector-000050000` rank `1`
- Selection rule: `selected-after-fourth-retirement-interior-nonmod3-family-on-next-twelve-disjoint-twin-prime-pairs-after-9857-9859`
- Target cell: width `9`, position `4`, digit `5`, connector `000050000`
- Fresh ladder pairs: `12`; rows `12`; nonblocked rows `12`; theorem-blocked rows `0`
- Hit classes: reverse-only `3`; forward-only `0`; both `0`; neither `9`
- Replication decision: `survived-interior-independent-ladder-residue-profiler-next`
- Next experiment target: `residue-profile-interior-nonmod3-000050000-width9-position4-digit5`

| Pair | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---|---:|---:|---:|
| `Twin-prime ladder pair (9929 ∘ 9931)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (10007 ∘ 10009)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (10037 ∘ 10039)` | `reverse-only-prime-hit` | false | true | -7.089018 |
| `Twin-prime ladder pair (10067 ∘ 10069)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (10091 ∘ 10093)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (10139 ∘ 10141)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (10271 ∘ 10273)` | `reverse-only-prime-hit` | false | true | -7.092959 |
| `Twin-prime ladder pair (10301 ∘ 10303)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (10331 ∘ 10333)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (10427 ∘ 10429)` | `reverse-only-prime-hit` | false | true | -7.095536 |
| `Twin-prime ladder pair (10457 ∘ 10459)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (10499 ∘ 10501)` | `neither-prime-hit` | false | false | 0.000000 |

### Interior Non-Mod3 After Fourth-Retirement Residue-Class Profile

This block profiles the survived `000050000` interior branch by small-prime residue classes. It remains theorem-candidate material only if the best separator survives one more disjoint ladder unchanged.

- Source candidate: `interior-width9-position4-digit5-connector-000050000`
- Profile decision: `small-prime-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `3`; comparison pairs `9`; exact separators `3`
- Best separator: mod `13` reverse-only residues `[1]` target `replicate-interior-nonmod3-000050000-mod13-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[2]` | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[1, 2]` | `[1, 2, 4]` | `[1, 2]` | `overlapping-residue-classes` |
| 7 | `[2, 4, 6]` | `[1, 3, 4, 6]` | `[4, 6]` | `overlapping-residue-classes` |
| 11 | `[5, 8, 10]` | `[2, 4, 5, 7, 8]` | `[5, 8]` | `overlapping-residue-classes` |
| 13 | `[1]` | `[3, 5, 8, 9, 10, 12]` | `[]` | `exact-residue-separator` |
| 17 | `[3, 6, 7]` | `[1, 2, 3, 7, 10, 11, 12, 16]` | `[3, 7]` | `overlapping-residue-classes` |
| 19 | `[5, 11, 15]` | `[2, 3, 7, 11, 12, 13, 14, 16]` | `[11]` | `overlapping-residue-classes` |
| 23 | `[8, 9, 13]` | `[2, 4, 11, 15, 16, 17, 19, 20]` | `[]` | `exact-residue-separator` |
| 29 | `[3, 5, 16]` | `[1, 2, 4, 6, 7, 11, 17, 18, 28]` | `[]` | `exact-residue-separator` |
| 31 | `[10, 11, 24]` | `[2, 8, 9, 10, 16, 21, 23, 25]` | `[10]` | `overlapping-residue-classes` |

### Interior Non-Mod3 After Fourth-Retirement Residue-Separator Replication

This block tests the best `000050000` residue separator on one more disjoint twin-prime ladder. A retained separator becomes theorem-candidate material; mutation or overlap retires the branch.

- Source candidate: `interior-width9-position4-digit5-connector-000050000`
- Selection rule: `test-best-after-fourth-retirement-interior-residue-separator-on-next-disjoint-twelve-twin-prime-pairs`
- Tested separator: mod `13` source reverse-only residues `[1]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `0`; comparison rows `12`
- Fresh-ladder reverse-only residues: `[]`; comparison residues `[2, 4, 5, 6, 7, 8, 10, 12]`; shared residues `[]`
- Separator status: `collapsed-no-reverse-only-on-interior-separator-ladder`; retained residues `0`; split residues `0`; collapsed source residues `1`
- Replication decision: `mod13-interior-residue-separator-collapsed-retire-branch`
- Next experiment target: `retire-interior-nonmod3-000050000-mod13-residue-separator-after-mutation`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---:|---|---:|---:|---:|
| `Twin-prime ladder pair (10529 ∘ 10531)` | 12 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (10709 ∘ 10711)` | 10 | `both-prime-hit` | true | true | -0.000032 |
| `Twin-prime ladder pair (10859 ∘ 10861)` | 4 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (10889 ∘ 10891)` | 8 | `forward-only-prime-hit` | true | false | 7.102918 |
| `Twin-prime ladder pair (10937 ∘ 10939)` | 4 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11057 ∘ 11059)` | 7 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11069 ∘ 11071)` | 6 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11117 ∘ 11119)` | 2 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11159 ∘ 11161)` | 5 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11171 ∘ 11173)` | 4 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11351 ∘ 11353)` | 2 | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11489 ∘ 11491)` | 10 | `neither-prime-hit` | false | false | 0.000000 |

### Interior Non-Mod3 After Fifth-Retirement Family Picker

This block reruns the interior picker after retiring `04000`, `05000`, `00010`, `0000070`, and `000050000`. The selected family is tested only for fresh-ladder survival here; residue profiling waits for a later tranche if it survives.

- Picker decision: `interior-nonmod3-family-selected-for-independent-replication`
- Selection rule: `after-fifth-interior-residue-separator-retirement;exclude-retired-interior-candidates;widths-5-through-9;interior-positions-only;digits-1-2-4-5-7;rank-by-reverse-only-count-gap`
- Retired candidates excluded: `5` `["interior-width5-position1-digit4-connector-04000", "interior-width5-position1-digit5-connector-05000", "interior-width5-position3-digit1-connector-00010", "interior-width7-position5-digit7-connector-0000070", "interior-width9-position4-digit5-connector-000050000"]`
- Source pairs: `12`; rows `1440`; nonblocked rows `1440`; ranked cells `120`; reverse-only cells `77`
- Selected after-fifth-retirement interior candidate: width `9` position `5` digit `7` connector `000007000` reverse-only pairs `3` target `independently-replicate-interior-nonmod3-000007000-width9-position5-digit7`

| Rank | Width | Position | Digit | Connector | Reverse-only pairs | Forward-only pairs | Both-hit pairs | Neither-hit pairs | Status |
|---:|---:|---:|---:|---|---:|---:|---:|---:|---|

### Interior Non-Mod3 After Fifth-Retirement Independent Replication

This block tests the after-fifth-retirement interior family on the next disjoint twin-prime ladder. Survival opens a future residue-profiler target only; collapse retires the branch before theorem work.

- Source candidate: `interior-width9-position5-digit7-connector-000007000` rank `1`
- Selection rule: `selected-after-fifth-retirement-interior-nonmod3-family-on-next-twelve-disjoint-twin-prime-pairs-after-11489-11491`
- Target cell: width `9`, position `5`, digit `7`, connector `000007000`
- Fresh ladder pairs: `12`; rows `12`; nonblocked rows `12`; theorem-blocked rows `0`
- Hit classes: reverse-only `0`; forward-only `0`; both `0`; neither `12`
- Replication decision: `collapsed-interior-independent-ladder-retire-without-lean`
- Next experiment target: `retire-interior-nonmod3-000007000-width9-position5-digit7`

| Pair | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---|---:|---:|---:|
| `Twin-prime ladder pair (11549 ∘ 11551)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11699 ∘ 11701)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11717 ∘ 11719)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11777 ∘ 11779)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11831 ∘ 11833)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11939 ∘ 11941)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (11969 ∘ 11971)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (12041 ∘ 12043)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (12071 ∘ 12073)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (12107 ∘ 12109)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (12161 ∘ 12163)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (12239 ∘ 12241)` | `neither-prime-hit` | false | false | 0.000000 |

### Interior Non-Mod3 After Sixth-Retirement Family Picker

This block reruns the interior picker after six retired interior branches. The selected family is tested only for fresh-ladder survival here; residue profiling waits for a later tranche if it survives.

- Picker decision: `interior-nonmod3-family-selected-for-independent-replication`
- Selection rule: `after-sixth-interior-family-retirement;exclude-retired-interior-candidates;widths-5-through-9;interior-positions-only;digits-1-2-4-5-7;rank-by-reverse-only-count-gap`
- Retired candidates excluded: `6` `["interior-width5-position1-digit4-connector-04000", "interior-width5-position1-digit5-connector-05000", "interior-width5-position3-digit1-connector-00010", "interior-width7-position5-digit7-connector-0000070", "interior-width9-position4-digit5-connector-000050000", "interior-width9-position5-digit7-connector-000007000"]`
- Source pairs: `12`; rows `1428`; nonblocked rows `1428`; ranked cells `119`; reverse-only cells `76`
- Selected after-sixth-retirement interior candidate: width `9` position `7` digit `7` connector `000000070` reverse-only pairs `3` target `independently-replicate-interior-nonmod3-000000070-width9-position7-digit7`

| Rank | Width | Position | Digit | Connector | Reverse-only pairs | Forward-only pairs | Both-hit pairs | Neither-hit pairs | Status |
|---:|---:|---:|---:|---|---:|---:|---:|---:|---|

### Interior Non-Mod3 After Sixth-Retirement Independent Replication

This block tests the after-sixth-retirement interior family on the next disjoint twin-prime ladder. Survival opens a future residue-profiler target only; collapse retires the branch before theorem work.

- Source candidate: `interior-width9-position7-digit7-connector-000000070` rank `1`
- Selection rule: `selected-after-sixth-retirement-interior-nonmod3-family-on-next-twelve-disjoint-twin-prime-pairs-after-12239-12241`
- Target cell: width `9`, position `7`, digit `7`, connector `000000070`
- Fresh ladder pairs: `12`; rows `12`; nonblocked rows `12`; theorem-blocked rows `0`
- Hit classes: reverse-only `0`; forward-only `2`; both `0`; neither `10`
- Replication decision: `collapsed-interior-independent-ladder-retire-without-lean`
- Next experiment target: `retire-interior-nonmod3-000000070-width9-position7-digit7`

| Pair | Contrast | Forward prime | Reverse prime | Signed ratio gap |
|---|---|---:|---:|---:|
| `Twin-prime ladder pair (12251 ∘ 12253)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (12377 ∘ 12379)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (12539 ∘ 12541)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (12611 ∘ 12613)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (12821 ∘ 12823)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (12917 ∘ 12919)` | `forward-only-prime-hit` | true | false | 7.132128 |
| `Twin-prime ladder pair (13001 ∘ 13003)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (13007 ∘ 13009)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (13217 ∘ 13219)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (13337 ∘ 13339)` | `neither-prime-hit` | false | false | 0.000000 |
| `Twin-prime ladder pair (13397 ∘ 13399)` | `forward-only-prime-hit` | true | false | 7.138368 |
| `Twin-prime ladder pair (13679 ∘ 13681)` | `neither-prime-hit` | false | false | 0.000000 |

### Single-Digit Interior Pivot

This block records the routing decision after repeated single-digit interior branches collapsed on fresh ladders. It does not prove a connector law; it moves the stress surface to adjacent two-digit interior motifs so future residue profiling is not row-by-row singleton cargo cult.

- Pivot decision: `pivot-away-from-single-digit-interior-family-after-repeated-fresh-ladder-collapse`

### Multi-Digit Motif Family Picker

This block scans adjacent two-digit interior connector motifs after excluding immediate mod-3 theorem-blocked motifs. A selected motif must survive a fresh disjoint ladder before residue profiling or Lean work.

- Picker decision: `multi-digit-motif-selected-for-independent-replication`
- Selection rule: `after-single-digit-interior-collapse;pivot-to-adjacent-two-digit-interior-motifs;widths-5-through-9;digits-1-2-4-5-7;exclude-mod3-blocked;rank-by-reverse-only-count`
- Source pairs: `12`; rows `3120`; nonblocked rows `3120`; ranked motifs `260`; reverse-only motifs `187`
- Selected motif: width `5` start `1` digits `[1, 1]` connector `01100` reverse-only pairs `4` target `independently-replicate-multidigit-motif-01100-width5-start1-digits11`

| Rank | Width | Start | Digits | Connector | Reverse-only pairs | Forward-only pairs | Both-hit pairs | Neither-hit pairs | Status |
|---:|---:|---:|---|---|---:|---:|---:|---:|---|

### Multi-Digit Motif Independent Replication

This block tests the selected adjacent two-digit motif on a fresh disjoint twin-prime ladder. Survival opens a residue-profiler target only; collapse retires the motif without theorem metadata.

- Source motif: `multidigit-motif-width5-start1-digits11-connector-01100` rank `1`
- Target motif: width `5`, start `1`, digits `[1, 1]`, connector `01100`
- Fresh ladder pairs: `12`; rows `12`; nonblocked rows `12`; theorem-blocked rows `0`
- Hit classes: reverse-only `2`; forward-only `4`; both `0`; neither `6`
- Replication decision: `survived-multidigit-motif-independent-ladder-residue-profiler-next`
- Next experiment target: `residue-profile-multidigit-motif-01100-width5-start1-digits11`

| Pair | Contrast | Forward prime | Reverse prime | Connector |
|---|---|---:|---:|---|
| `Twin-prime ladder pair (13691 ∘ 13693)` | `reverse-only-prime-hit` | false | true | `01100` |
| `Twin-prime ladder pair (13709 ∘ 13711)` | `forward-only-prime-hit` | true | false | `01100` |
| `Twin-prime ladder pair (13721 ∘ 13723)` | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (13757 ∘ 13759)` | `reverse-only-prime-hit` | false | true | `01100` |
| `Twin-prime ladder pair (13829 ∘ 13831)` | `forward-only-prime-hit` | true | false | `01100` |
| `Twin-prime ladder pair (13877 ∘ 13879)` | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (13901 ∘ 13903)` | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (13931 ∘ 13933)` | `forward-only-prime-hit` | true | false | `01100` |
| `Twin-prime ladder pair (13997 ∘ 13999)` | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (14009 ∘ 14011)` | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (14081 ∘ 14083)` | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (14249 ∘ 14251)` | `forward-only-prime-hit` | true | false | `01100` |

### Multi-Digit Motif Residue-Class Profile

This block profiles the survived adjacent two-digit motif by small-prime residue classes. It is theorem-candidate material only if the best separator survives one more disjoint ladder unchanged.

- Source motif: `multidigit-motif-width5-start1-digits11-connector-01100`
- Profile decision: `small-prime-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `2`; comparison pairs `10`; exact separators `2`
- Best separator: mod `11` reverse-only residues `[7]` target `replicate-multidigit-motif-01100-mod11-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[2]` | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[1, 2]` | `[1, 2, 4]` | `[1, 2]` | `overlapping-residue-classes` |
| 7 | `[2, 6]` | `[1, 2, 3, 4, 6]` | `[2, 6]` | `overlapping-residue-classes` |
| 11 | `[7]` | `[1, 2, 3, 4, 5, 6, 8]` | `[]` | `exact-residue-separator` |
| 13 | `[2, 3]` | `[1, 2, 4, 6, 7, 8, 9, 10]` | `[2]` | `overlapping-residue-classes` |
| 17 | `[4, 6]` | `[1, 2, 3, 5, 6, 7, 8, 12]` | `[6]` | `overlapping-residue-classes` |
| 19 | `[1, 11]` | `[2, 3, 4, 6, 7, 10, 12, 13, 16, 18]` | `[]` | `exact-residue-separator` |
| 23 | `[3, 6]` | `[1, 2, 5, 6, 8, 9, 12, 13, 16]` | `[6]` | `overlapping-residue-classes` |
| 29 | `[3, 11]` | `[2, 4, 10, 11, 15, 16, 19, 21, 25]` | `[11]` | `overlapping-residue-classes` |
| 31 | `[20, 24]` | `[3, 7, 12, 13, 16, 19, 20, 28]` | `[20]` | `overlapping-residue-classes` |

### Multi-Digit Motif Residue-Separator Replication

This block tests the best adjacent two-digit motif residue separator on one more disjoint twin-prime ladder. Retention opens a finite-classifier theorem candidate; split or collapse retires the motif without Lean.

- Source motif: `multidigit-motif-width5-start1-digits11-connector-01100`
- Tested separator: mod `11` source reverse-only residues `[7]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `0`; comparison rows `12`
- Fresh-ladder reverse-only residues: `[]`; comparison residues `[1, 4, 5, 6, 7, 8, 10]`; shared residues `[]`
- Separator status: `collapsed-no-reverse-only-on-multidigit-separator-ladder`; retained residues `0`; split residues `0`; collapsed source residues `1`
- Replication decision: `mod11-multidigit-residue-separator-collapsed-retire-branch`
- Next experiment target: `retire-multidigit-motif-01100-mod11-residue-separator-after-mutation`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Twin-prime ladder pair (14321 ∘ 14323)` | 10 | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (14387 ∘ 14389)` | 10 | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (14447 ∘ 14449)` | 4 | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (14549 ∘ 14551)` | 7 | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (14561 ∘ 14563)` | 8 | `forward-only-prime-hit` | true | false | `01100` |
| `Twin-prime ladder pair (14591 ∘ 14593)` | 5 | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (14627 ∘ 14629)` | 8 | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (14867 ∘ 14869)` | 6 | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (15137 ∘ 15139)` | 1 | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (15269 ∘ 15271)` | 1 | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (15287 ∘ 15289)` | 8 | `neither-prime-hit` | false | false | `01100` |
| `Twin-prime ladder pair (15329 ∘ 15331)` | 6 | `forward-only-prime-hit` | true | false | `01100` |

### Multi-Digit Motif Retirement Summary

This block formally retires adjacent two-digit motifs whose residue separator collapsed on a disjoint ladder. Retirement is a routing decision, not a density claim.

| Motif | Width | Start | Digits | Source modulus | Decision | Reason |
|---|---:|---:|---|---:|---|---|
| `01100` | 5 | 1 | `[1, 1]` | 11 | `mod11-multidigit-residue-separator-collapsed-retire-branch` | second disjoint separator ladder had zero reverse-only rows; retire before Lean |

### Orthogonal Pair-Family Retirement Summary

This block formally retires orthogonal pair-family branches whose residue separator failed a disjoint separator ladder. Retired branches are excluded from the next source/fresh picker; this is a routing guardrail, not a density claim.

| Branch | Family | Connector | Source modulus | Decision | Reason |
|---|---|---|---:|---|---|
| `orthogonal-gap6-width6-start2-digits52-connector-005200` | `sexy-prime-gap6` | `005200` | 23 | `mod23-orthogonal-residue-separator-collapsed-retire-branch` | disjoint separator ladder mutated the source mod-23 separator; retire before Lean |
| `orthogonal-gap6-width6-start3-digits41-connector-000410` | `sexy-prime-gap6` | `000410` | 23 | `mod23-orthogonal-residue-separator-collapsed-retire-branch` | disjoint separator ladder had zero reverse-only rows; retire before Lean |

### Orthogonal Pair-Family Control

This block pivots away from the collapsed twin-prime motif branch by scanning adjacent two-digit interior motifs over non-twin prime-pair controls. It is an empirical routing surface only: theorem-candidate language requires survival across source, fresh, and separator ladders with an unchanged exact separator.

- Matrix decision: `orthogonal-pair-family-control-survived-residue-profiler-next`
- Selection rule: `after-01100-multidigit-collapse;exclude-retired-orthogonal-branches;scan-gap4-and-gap6-non-twin-pair-controls;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `6240`; fresh rows `24`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected orthogonal branch: `orthogonal-gap4-width5-start1-digits47-connector-04700` family `cousin-prime-gap4` connector `04700` fresh reverse-only `1` target `residue-profile-orthogonal-gap4-multidigit-motif-04700-width5-start1-digits47`

| Family | Gap | Source selected connector | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `04700` | 4 | 1 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-gap4-multidigit-motif-04700-width5-start1-digits47` |
| `sexy-prime-gap6` | 6 | `0740000` | 2 | 0 | 0 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-gap6-multidigit-motif-0740000-width7-start1-digits74` |

### Orthogonal Pair-Family Residue-Class Profile

This block profiles the survived non-twin pair-family branch by small-prime residue classes. It remains empirical routing only unless the best separator survives one more disjoint ladder unchanged.

- Source branch: `orthogonal-gap4-width5-start1-digits47-connector-04700` family `cousin-prime-gap4` connector `04700`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `1`; comparison pairs `11`; exact separators `3`
- Best separator: mod `13` reverse-only residues `[12]` target `replicate-orthogonal-gap4-multidigit-motif-04700-mod13-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[4]` | `[2, 3, 4]` | `[4]` | `overlapping-residue-classes` |
| 7 | `[2]` | `[1, 2, 4, 5, 6]` | `[2]` | `overlapping-residue-classes` |
| 11 | `[2]` | `[1, 2, 3, 4, 5, 8, 9, 10]` | `[2]` | `overlapping-residue-classes` |
| 13 | `[12]` | `[1, 2, 4, 6, 7, 8, 11]` | `[]` | `exact-residue-separator` |
| 17 | `[14]` | `[4, 5, 8, 11, 12, 14, 15, 16]` | `[14]` | `overlapping-residue-classes` |
| 19 | `[4]` | `[1, 4, 8, 12, 13, 14, 16, 17]` | `[4]` | `overlapping-residue-classes` |
| 23 | `[11]` | `[1, 2, 5, 9, 10, 14, 15, 18, 20]` | `[]` | `exact-residue-separator` |
| 29 | `[9]` | `[1, 3, 4, 5, 7, 11, 15, 21, 24, 26]` | `[]` | `exact-residue-separator` |
| 31 | `[5]` | `[1, 5, 7, 10, 11, 16, 17, 18, 21, 29]` | `[5]` | `overlapping-residue-classes` |

### Orthogonal Pair-Family Residue-Separator Replication

This block tests the best orthogonal pair-family residue separator on one more disjoint gap-6 prime-pair ladder. Retention would make a finite-classifier candidate; split or collapse retires the branch without Lean.

- Source branch: `orthogonal-gap4-width5-start1-digits47-connector-04700` family `cousin-prime-gap4` connector `04700`
- Tested separator: mod `13` source reverse-only residues `[12]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `1`; comparison rows `11`
- Fresh-ladder reverse-only residues: `[2]`; comparison residues `[1, 2, 4, 5, 6, 8, 11, 12]`; shared residues `[2]`
- Separator status: `collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder`; retained residues `0`; split residues `1`; collapsed source residues `1`
- Replication decision: `mod13-orthogonal-residue-separator-collapsed-retire-branch`
- Next experiment target: `pivot-away-from-orthogonal-adjacent-two-digit-motifs-after-repeated-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 2 | `reverse-only-prime-hit` | false | true | `04700` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 5 | `neither-prime-hit` | false | false | `04700` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 1 | `neither-prime-hit` | false | false | `04700` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 12 | `neither-prime-hit` | false | false | `04700` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 4 | `forward-only-prime-hit` | true | false | `04700` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 1 | `neither-prime-hit` | false | false | `04700` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 2 | `forward-only-prime-hit` | true | false | `04700` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 8 | `neither-prime-hit` | false | false | `04700` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 8 | `neither-prime-hit` | false | false | `04700` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 6 | `neither-prime-hit` | false | false | `04700` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 8 | `neither-prime-hit` | false | false | `04700` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 11 | `neither-prime-hit` | false | false | `04700` |

### Orthogonal Compact Three-Digit Control

This block pivots away from the collapsed adjacent two-digit orthogonal branch by scanning compact adjacent three-digit interior motifs over the same non-twin prime-pair controls. It is still an empirical routing surface: theorem-candidate language requires source, fresh, and separator-ladder survival with an unchanged exact separator.

- Matrix decision: `orthogonal-compact-three-digit-control-survived-residue-profiler-next`
- Selection rule: `after-adjacent-orthogonal-collapse;pivot-to-compact-three-digit-interior-motifs;scan-gap4-and-gap6-non-twin-pair-controls;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `32400`; fresh rows `24`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected compact branch: `orthogonal-compact3-gap4-width5-start1-digits251-connector-02510` family `cousin-prime-gap4` connector `02510` fresh reverse-only `3` target `residue-profile-orthogonal-gap4-multidigit-motif-02510-width5-start1-digits251`

| Family | Gap | Source selected connector | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `02510` | 6 | 3 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-gap4-multidigit-motif-02510-width5-start1-digits251` |
| `sexy-prime-gap6` | 6 | `0045700` | 4 | 0 | 2 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-gap6-multidigit-motif-0045700-width7-start2-digits457` |

### Orthogonal Compact Three-Digit Residue-Class Profile

This block profiles the survived compact three-digit branch by small-prime residue classes. It remains a candidate-routing screen unless the best separator survives one more disjoint ladder unchanged.

- Source branch: `orthogonal-compact3-gap4-width5-start1-digits251-connector-02510` family `cousin-prime-gap4` connector `02510`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `3`; comparison pairs `9`; exact separators `1`
- Best separator: mod `29` reverse-only residues `[1, 9, 15]` target `replicate-orthogonal-gap4-multidigit-motif-02510-mod29-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[3, 4]` | `[2, 3, 4]` | `[3, 4]` | `overlapping-residue-classes` |
| 7 | `[2, 6]` | `[1, 2, 4, 5, 6]` | `[2, 6]` | `overlapping-residue-classes` |
| 11 | `[2, 4]` | `[1, 2, 3, 5, 8, 9, 10]` | `[2]` | `overlapping-residue-classes` |
| 13 | `[2, 11, 12]` | `[1, 2, 4, 6, 7, 8, 11]` | `[2, 11]` | `overlapping-residue-classes` |
| 17 | `[11, 14, 16]` | `[4, 5, 8, 12, 14, 15]` | `[14]` | `overlapping-residue-classes` |
| 19 | `[4, 14, 16]` | `[1, 4, 8, 12, 13, 14, 17]` | `[4, 14]` | `overlapping-residue-classes` |
| 23 | `[11, 14, 20]` | `[1, 2, 5, 9, 10, 15, 18, 20]` | `[20]` | `overlapping-residue-classes` |
| 29 | `[1, 9, 15]` | `[3, 4, 5, 7, 11, 21, 24, 26]` | `[]` | `exact-residue-separator` |
| 31 | `[1, 5, 18]` | `[5, 7, 10, 11, 16, 17, 18, 21, 29]` | `[5, 18]` | `overlapping-residue-classes` |

### Orthogonal Compact Three-Digit Residue-Separator Replication

This block tests the compact three-digit branch's best residue separator on one more disjoint non-twin prime-pair ladder. Split or overlap means the branch is retired before any finite classifier theorem is proposed.

- Source branch: `orthogonal-compact3-gap4-width5-start1-digits251-connector-02510` family `cousin-prime-gap4` connector `02510`
- Tested separator: mod `29` source reverse-only residues `[1, 9, 15]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `2`; comparison rows `10`
- Fresh-ladder reverse-only residues: `[4, 26]`; comparison residues `[1, 6, 9, 12, 18, 20, 24, 26]`; shared residues `[26]`
- Separator status: `collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder`; retained residues `0`; split residues `2`; collapsed source residues `3`
- Replication decision: `mod29-orthogonal-residue-separator-collapsed-retire-branch`
- Next experiment target: `select-next-orthogonal-nonadjacent-two-digit-motif-family-after-compact-three-digit-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 26 | `reverse-only-prime-hit` | false | true | `02510` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 1 | `neither-prime-hit` | false | false | `02510` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 20 | `neither-prime-hit` | false | false | `02510` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 6 | `neither-prime-hit` | false | false | `02510` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 6 | `neither-prime-hit` | false | false | `02510` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 4 | `reverse-only-prime-hit` | false | true | `02510` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 12 | `neither-prime-hit` | false | false | `02510` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 18 | `forward-only-prime-hit` | true | false | `02510` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 9 | `neither-prime-hit` | false | false | `02510` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 24 | `forward-only-prime-hit` | true | false | `02510` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 20 | `neither-prime-hit` | false | false | `02510` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 26 | `neither-prime-hit` | false | false | `02510` |

### Orthogonal Non-Adjacent Two-Digit Control

This block follows the compact-three-digit collapse by scanning non-adjacent two-digit interior motifs over the same non-twin prime-pair controls. It keeps the same three-ladder rule: source survival, fresh survival, and unchanged separator-ladder retention are required before any theorem-candidate language.

- Matrix decision: `orthogonal-nonadjacent-two-digit-control-survived-residue-profiler-next`
- Selection rule: `after-compact-three-digit-collapse;pivot-to-nonadjacent-two-digit-interior-motifs;scan-gap4-and-gap6-non-twin-pair-controls;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `10920`; fresh rows `24`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected non-adjacent branch: `orthogonal-nonadjacent2-gap4-width7-pos25-digits52-connector-0050020` family `cousin-prime-gap4` connector `0050020` positions `[2, 5]` fresh reverse-only `1` target `residue-profile-orthogonal-nonadjacent2-gap4-multidigit-motif-0050020-width7-pos25-digits52`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `0050020` | `[2, 5]` | 5 | 1 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-nonadjacent2-gap4-multidigit-motif-0050020-width7-pos25-digits52` |
| `sexy-prime-gap6` | 6 | `0002020` | `[3, 5]` | 4 | 0 | 0 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-nonadjacent2-gap6-multidigit-motif-0002020-width7-pos35-digits22` |

### Orthogonal Non-Adjacent Two-Digit Residue-Class Profile

This block profiles the survived non-adjacent two-digit branch by small-prime residue classes. It remains empirical routing unless the best separator survives one more disjoint ladder unchanged.

- Source branch: `orthogonal-nonadjacent2-gap4-width7-pos25-digits52-connector-0050020` family `cousin-prime-gap4` connector `0050020` positions `[2, 5]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `1`; comparison pairs `11`; exact separators `4`
- Best separator: mod `11` reverse-only residues `[4]` target `replicate-orthogonal-nonadjacent2-gap4-multidigit-motif-0050020-pos25-mod11-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[3]` | `[2, 3, 4]` | `[3]` | `overlapping-residue-classes` |
| 7 | `[6]` | `[1, 2, 4, 5, 6]` | `[6]` | `overlapping-residue-classes` |
| 11 | `[4]` | `[1, 2, 3, 5, 8, 9, 10]` | `[]` | `exact-residue-separator` |
| 13 | `[2]` | `[1, 2, 4, 6, 7, 8, 11, 12]` | `[2]` | `overlapping-residue-classes` |
| 17 | `[11]` | `[4, 5, 8, 12, 14, 15, 16]` | `[]` | `exact-residue-separator` |
| 19 | `[16]` | `[1, 4, 8, 12, 13, 14, 17]` | `[]` | `exact-residue-separator` |
| 23 | `[20]` | `[1, 2, 5, 9, 10, 11, 14, 15, 18, 20]` | `[20]` | `overlapping-residue-classes` |
| 29 | `[15]` | `[1, 3, 4, 5, 7, 9, 11, 21, 24, 26]` | `[]` | `exact-residue-separator` |
| 31 | `[18]` | `[1, 5, 7, 10, 11, 16, 17, 18, 21, 29]` | `[18]` | `overlapping-residue-classes` |

### Orthogonal Non-Adjacent Two-Digit Residue-Separator Replication

This block tests the non-adjacent branch's best residue separator on one more disjoint non-twin prime-pair ladder. Mutation or collapse retires the small-digit motif path before Lean theorem growth.

- Source branch: `orthogonal-nonadjacent2-gap4-width7-pos25-digits52-connector-0050020` family `cousin-prime-gap4` connector `0050020` positions `[2, 5]`
- Tested separator: mod `11` source reverse-only residues `[4]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `1`; comparison rows `11`
- Fresh-ladder reverse-only residues: `[6]`; comparison residues `[1, 2, 3, 5, 9, 10]`; shared residues `[]`
- Separator status: `split-exact-residue-separator-on-orthogonal-separator-ladder`; retained residues `0`; split residues `1`; collapsed source residues `1`
- Replication decision: `mod11-orthogonal-residue-separator-mutated-retire-branch`
- Next experiment target: `pivot-away-from-small-digit-orthogonal-motifs-after-nonadjacent-two-digit-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 6 | `reverse-only-prime-hit` | false | true | `0050020` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 5 | `neither-prime-hit` | false | false | `0050020` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 9 | `neither-prime-hit` | false | false | `0050020` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 1 | `neither-prime-hit` | false | false | `0050020` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 10 | `neither-prime-hit` | false | false | `0050020` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 3 | `neither-prime-hit` | false | false | `0050020` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 3 | `neither-prime-hit` | false | false | `0050020` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 9 | `neither-prime-hit` | false | false | `0050020` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 10 | `neither-prime-hit` | false | false | `0050020` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 2 | `neither-prime-hit` | false | false | `0050020` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 1 | `neither-prime-hit` | false | false | `0050020` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 3 | `neither-prime-hit` | false | false | `0050020` |

### Orthogonal Edge-Plus-Interior Control

This block pivots away from interior-only small digit motifs by scanning two-digit motifs with one connector digit on an edge and one digit in the interior. It keeps the same source/fresh/separator rule; theorem-candidate language requires unchanged separator retention on the third ladder.

- Matrix decision: `orthogonal-edge-plus-interior-control-survived-residue-profiler-next`
- Selection rule: `after-small-digit-orthogonal-collapse;pivot-to-edge-plus-interior-two-digit-motifs;scan-gap4-and-gap6-non-twin-pair-controls;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `15600`; fresh rows `24`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected edge-plus branch: `orthogonal-edgeplus2-gap4-width5-pos34-digits22-connector-00022` family `cousin-prime-gap4` connector `00022` positions `[3, 4]` fresh reverse-only `1` target `residue-profile-orthogonal-edgeplus2-gap4-motif-00022-width5-pos34-digits22`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `00022` | `[3, 4]` | 5 | 1 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-edgeplus2-gap4-motif-00022-width5-pos34-digits22` |
| `sexy-prime-gap6` | 6 | `00107` | `[2, 4]` | 3 | 1 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-edgeplus2-gap6-motif-00107-width5-pos24-digits17` |

### Orthogonal Edge-Plus-Interior Residue-Class Profile

This block profiles the survived edge-plus branch by small-prime residue classes. It remains empirical routing unless the best separator survives one more disjoint ladder unchanged.

- Source branch: `orthogonal-edgeplus2-gap4-width5-pos34-digits22-connector-00022` family `cousin-prime-gap4` connector `00022` positions `[3, 4]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `1`; comparison pairs `11`; exact separators `3`
- Best separator: mod `13` reverse-only residues `[12]` target `replicate-orthogonal-edgeplus2-gap4-motif-00022-pos34-mod13-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[4]` | `[2, 3, 4]` | `[4]` | `overlapping-residue-classes` |
| 7 | `[2]` | `[1, 2, 4, 5, 6]` | `[2]` | `overlapping-residue-classes` |
| 11 | `[2]` | `[1, 2, 3, 4, 5, 8, 9, 10]` | `[2]` | `overlapping-residue-classes` |
| 13 | `[12]` | `[1, 2, 4, 6, 7, 8, 11]` | `[]` | `exact-residue-separator` |
| 17 | `[14]` | `[4, 5, 8, 11, 12, 14, 15, 16]` | `[14]` | `overlapping-residue-classes` |
| 19 | `[4]` | `[1, 4, 8, 12, 13, 14, 16, 17]` | `[4]` | `overlapping-residue-classes` |
| 23 | `[11]` | `[1, 2, 5, 9, 10, 14, 15, 18, 20]` | `[]` | `exact-residue-separator` |
| 29 | `[9]` | `[1, 3, 4, 5, 7, 11, 15, 21, 24, 26]` | `[]` | `exact-residue-separator` |
| 31 | `[5]` | `[1, 5, 7, 10, 11, 16, 17, 18, 21, 29]` | `[5]` | `overlapping-residue-classes` |

### Orthogonal Edge-Plus-Interior Residue-Separator Replication

This block tests the edge-plus branch's best residue separator on one more disjoint non-twin prime-pair ladder. Collapse retires this family class before theorem growth.

- Source branch: `orthogonal-edgeplus2-gap4-width5-pos34-digits22-connector-00022` family `cousin-prime-gap4` connector `00022` positions `[3, 4]`
- Tested separator: mod `13` source reverse-only residues `[12]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `0`; comparison rows `12`
- Fresh-ladder reverse-only residues: `[]`; comparison residues `[1, 2, 4, 5, 6, 8, 11, 12]`; shared residues `[]`
- Separator status: `collapsed-no-reverse-only-on-orthogonal-separator-ladder`; retained residues `0`; split residues `0`; collapsed source residues `1`
- Replication decision: `mod13-orthogonal-residue-separator-collapsed-retire-branch`
- Next experiment target: `pivot-away-from-edge-plus-interior-orthogonal-motifs-after-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 2 | `neither-prime-hit` | false | false | `00022` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 5 | `neither-prime-hit` | false | false | `00022` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 1 | `neither-prime-hit` | false | false | `00022` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 12 | `neither-prime-hit` | false | false | `00022` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 4 | `forward-only-prime-hit` | true | false | `00022` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 1 | `neither-prime-hit` | false | false | `00022` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 2 | `neither-prime-hit` | false | false | `00022` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 8 | `neither-prime-hit` | false | false | `00022` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 8 | `neither-prime-hit` | false | false | `00022` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 6 | `neither-prime-hit` | false | false | `00022` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 8 | `neither-prime-hit` | false | false | `00022` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 11 | `neither-prime-hit` | false | false | `00022` |

### Repeated-Block Orthogonal Control

This block stops the small digit-motif branch and scans gapped repeated two-digit blocks `ab...ab` over the same non-twin prime-pair controls. It remains empirical routing only: theorem-candidate language requires unchanged separator retention on the third ladder.

- Matrix decision: `orthogonal-repeated-block-control-survived-residue-profiler-next`
- Selection rule: `after-edge-plus-interior-collapse;pivot-to-gapped-repeated-two-digit-blocks;scan-gap4-and-gap6-non-twin-pair-controls;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `15120`; fresh rows `24`; widths `[5, 6, 7, 8, 9]`; digits `[1, 2, 4, 5, 7, 8]`
- Position scope: `gapped-repeated-two-digit-blocks-ab-gap-ab;positions-[i,i+1,j,j+1]-with-j>=i+3`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected repeated-block branch: `orthogonal-repeatblock-gap4-width9-pos3467-digits5555-connector-000550550` family `cousin-prime-gap4` connector `000550550` positions `[3, 4, 6, 7]` fresh reverse-only `4` target `residue-profile-orthogonal-repeatblock-gap4-motif-000550550-width9-pos3467-digits5555`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `000550550` | `[3, 4, 6, 7]` | 5 | 4 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-repeatblock-gap4-motif-000550550-width9-pos3467-digits5555` |
| `sexy-prime-gap6` | 6 | `0440044` | `[1, 2, 5, 6]` | 4 | 0 | 0 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-repeatblock-gap6-motif-0440044-width7-pos1256-digits4444` |

### Repeated-Block Orthogonal Residue-Class Profile

This block profiles the survived repeated-block branch by small-prime residue classes. It does not promote a connector law or density mechanism; it only decides whether a third-ladder separator test is warranted.

- Source branch: `orthogonal-repeatblock-gap4-width9-pos3467-digits5555-connector-000550550` family `cousin-prime-gap4` connector `000550550` positions `[3, 4, 6, 7]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `4`; comparison pairs `8`; exact separators `1`
- Best separator: mod `29` reverse-only residues `[7, 15, 21, 26]` target `replicate-orthogonal-repeatblock-gap4-motif-000550550-pos3467-mod29-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[2, 3, 4]` | `[2, 3, 4]` | `[2, 3, 4]` | `overlapping-residue-classes` |
| 7 | `[1, 5, 6]` | `[2, 4, 6]` | `[6]` | `overlapping-residue-classes` |
| 11 | `[3, 4, 5, 8]` | `[1, 2, 3, 5, 9, 10]` | `[3, 5]` | `overlapping-residue-classes` |
| 13 | `[2, 4, 6, 8]` | `[1, 2, 4, 6, 7, 11, 12]` | `[2, 4, 6]` | `overlapping-residue-classes` |
| 17 | `[5, 11, 15]` | `[4, 5, 8, 12, 14, 16]` | `[5]` | `overlapping-residue-classes` |
| 19 | `[1, 8, 13, 16]` | `[1, 4, 12, 13, 14, 17]` | `[1, 13]` | `overlapping-residue-classes` |
| 23 | `[9, 10, 20]` | `[1, 2, 5, 9, 11, 14, 15, 18]` | `[9]` | `overlapping-residue-classes` |
| 29 | `[7, 15, 21, 26]` | `[1, 3, 4, 5, 9, 11, 24]` | `[]` | `exact-residue-separator` |
| 31 | `[5, 7, 18, 29]` | `[1, 5, 10, 11, 16, 17, 18, 21]` | `[5, 18]` | `overlapping-residue-classes` |

### Repeated-Block Orthogonal Residue-Separator Replication

This block tests the repeated-block branch's best residue separator on one more disjoint non-twin prime-pair ladder. Mutation or collapse routes away from digit-pattern motifs toward arithmetic connector families before any Lean classifier work.

- Source branch: `orthogonal-repeatblock-gap4-width9-pos3467-digits5555-connector-000550550` family `cousin-prime-gap4` connector `000550550` positions `[3, 4, 6, 7]`
- Tested separator: mod `29` source reverse-only residues `[7, 15, 21, 26]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `1`; comparison rows `11`
- Fresh-ladder reverse-only residues: `[26]`; comparison residues `[1, 4, 6, 9, 12, 18, 20, 24, 26]`; shared residues `[26]`
- Separator status: `collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder`; retained residues `1`; split residues `0`; collapsed source residues `3`
- Replication decision: `mod29-orthogonal-residue-separator-collapsed-retire-branch`
- Next experiment target: `pivot-to-arithmetic-connector-families-after-repeated-block-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 26 | `reverse-only-prime-hit` | false | true | `000550550` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 1 | `neither-prime-hit` | false | false | `000550550` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 20 | `neither-prime-hit` | false | false | `000550550` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 6 | `neither-prime-hit` | false | false | `000550550` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 6 | `neither-prime-hit` | false | false | `000550550` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 4 | `neither-prime-hit` | false | false | `000550550` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 12 | `neither-prime-hit` | false | false | `000550550` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 18 | `neither-prime-hit` | false | false | `000550550` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 9 | `neither-prime-hit` | false | false | `000550550` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 24 | `neither-prime-hit` | false | false | `000550550` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 20 | `neither-prime-hit` | false | false | `000550550` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 26 | `neither-prime-hit` | false | false | `000550550` |

### Arithmetic Connector Control

This block pivots away from digit-pattern motifs by scanning connector values generated from bounded square and triangular sequences, then zero-padding those values to each fixed width. The nonzero digit positions are derived after the arithmetic value is selected; this remains empirical routing only.

- Matrix decision: `orthogonal-arithmetic-connector-control-survived-residue-profiler-next`
- Selection rule: `after-repeated-block-collapse;pivot-to-arithmetic-connector-values;scan-square-and-triangular-connectors-index-1-to-96;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `11280`; fresh rows `24`; widths `[5, 6, 7, 8, 9]`
- Position scope: `arithmetic connector values from n^2 and n*(n+1)/2, zero-padded to fixed width; motif positions are derived from nonzero decimal digits after value selection`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected arithmetic branch: `orthogonal-arithmetic-gap4-width5-connector-04900` family `cousin-prime-gap4` connector `04900` positions `[1, 2]` fresh reverse-only `3` target `residue-profile-orthogonal-arithmetic-gap4-connector-04900-width5`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `04900` | `[1, 2]` | 4 | 3 | 1 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-arithmetic-gap4-connector-04900-width5` |
| `sexy-prime-gap6` | 6 | `003025` | `[2, 4, 5]` | 3 | 1 | 1 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-arithmetic-gap6-connector-003025-width6` |

### Arithmetic Connector Residue-Class Profile

This block profiles the survived arithmetic connector branch by small-prime residue classes. It is not a connector law or density mechanism; it only decides whether a third-ladder separator test is warranted.

- Source branch: `orthogonal-arithmetic-gap4-width5-connector-04900` family `cousin-prime-gap4` connector `04900` positions `[1, 2]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `3`; comparison pairs `9`; exact separators `2`
- Best separator: mod `23` reverse-only residues `[1, 2, 15]` target `replicate-orthogonal-arithmetic-gap4-connector-04900-mod23-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[2, 4]` | `[2, 3, 4]` | `[2, 4]` | `overlapping-residue-classes` |
| 7 | `[4]` | `[1, 2, 4, 5, 6]` | `[4]` | `overlapping-residue-classes` |
| 11 | `[1, 2, 9]` | `[2, 3, 4, 5, 8, 10]` | `[2]` | `overlapping-residue-classes` |
| 13 | `[1, 2, 7]` | `[2, 4, 6, 8, 11, 12]` | `[2]` | `overlapping-residue-classes` |
| 17 | `[4, 5, 8]` | `[4, 5, 11, 12, 14, 15, 16]` | `[4, 5]` | `overlapping-residue-classes` |
| 19 | `[12, 14, 17]` | `[1, 4, 8, 13, 14, 16]` | `[14]` | `overlapping-residue-classes` |
| 23 | `[1, 2, 15]` | `[5, 9, 10, 11, 14, 18, 20]` | `[]` | `exact-residue-separator` |
| 29 | `[3, 4, 24]` | `[1, 3, 5, 7, 9, 11, 15, 21, 26]` | `[3]` | `overlapping-residue-classes` |
| 31 | `[11, 17, 21]` | `[1, 5, 7, 10, 16, 18, 29]` | `[]` | `exact-residue-separator` |

### Arithmetic Connector Residue-Separator Replication

This block tests the arithmetic connector branch's best residue separator on one more disjoint non-twin prime-pair ladder. Mutation or collapse keeps the result empirical and routes to the next arithmetic family before any Lean classifier work.

- Source branch: `orthogonal-arithmetic-gap4-width5-connector-04900` family `cousin-prime-gap4` connector `04900` positions `[1, 2]`
- Tested separator: mod `23` source reverse-only residues `[1, 2, 15]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `1`; comparison rows `11`
- Fresh-ladder reverse-only residues: `[20]`; comparison residues `[1, 5, 9, 10, 14, 15, 18, 20, 21, 22]`; shared residues `[20]`
- Separator status: `collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder`; retained residues `0`; split residues `1`; collapsed source residues `3`
- Replication decision: `mod23-orthogonal-residue-separator-collapsed-retire-branch`
- Next experiment target: `select-next-arithmetic-connector-family-after-square-triangular-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 15 | `neither-prime-hit` | false | false | `04900` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 20 | `reverse-only-prime-hit` | false | true | `04900` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 22 | `neither-prime-hit` | false | false | `04900` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 9 | `forward-only-prime-hit` | true | false | `04900` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 22 | `neither-prime-hit` | false | false | `04900` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 21 | `neither-prime-hit` | false | false | `04900` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 18 | `neither-prime-hit` | false | false | `04900` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 1 | `neither-prime-hit` | false | false | `04900` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 10 | `neither-prime-hit` | false | false | `04900` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 20 | `neither-prime-hit` | false | false | `04900` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 5 | `both-prime-hit` | true | true | `04900` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 14 | `forward-only-prime-hit` | true | false | `04900` |

### Residue-Lattice Connector Control

This block pivots from square/triangular arithmetic values to bounded residue-lattice connector values `q*m+r` for small prime moduli. Values are selected first and zero-padded to each fixed width; nonzero digit positions are derived afterward. It remains empirical routing only, not a connector law or density mechanism.

- Matrix decision: `orthogonal-residue-lattice-connector-control-survived-residue-profiler-next`
- Selection rule: `after-square-triangular-collapse;pivot-to-residue-lattice-connector-values;scan-q*m+r-for-small-prime-moduli-and-residues-coprime-to-30;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `27960`; fresh rows `24`; widths `[5, 6, 7, 8, 9]`
- Position scope: `residue-lattice connector values q*m+r with m in [7,11,13,17,19,23], q in 0..=24, and residue r coprime to 30; nonzero digit positions are derived after value selection`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected residue-lattice branch: `orthogonal-residuelattice-gap4-width6-connector-000122` family `cousin-prime-gap4` connector `000122` positions `[3, 4, 5]` fresh reverse-only `2` target `residue-profile-orthogonal-residuelattice-gap4-connector-000122-width6`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `000122` | `[3, 4, 5]` | 5 | 2 | 2 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-residuelattice-gap4-connector-000122-width6` |
| `sexy-prime-gap6` | 6 | `00292` | `[2, 3, 4]` | 4 | 0 | 1 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-residuelattice-gap6-connector-00292-width5` |

### Residue-Lattice Connector Residue-Class Profile

This block profiles the survived residue-lattice connector branch by small-prime residue classes. A separator here is only a third-ladder test candidate; no Lean classifier is proposed unless it survives unchanged.

- Source branch: `orthogonal-residuelattice-gap4-width6-connector-000122` family `cousin-prime-gap4` connector `000122` positions `[3, 4, 5]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `2`; comparison pairs `10`; exact separators `3`
- Best separator: mod `23` reverse-only residues `[2, 5]` target `replicate-orthogonal-residuelattice-gap4-connector-000122-mod23-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[2]` | `[2, 3, 4]` | `[2]` | `overlapping-residue-classes` |
| 7 | `[4]` | `[1, 2, 4, 5, 6]` | `[4]` | `overlapping-residue-classes` |
| 11 | `[2, 3]` | `[1, 2, 3, 4, 5, 8, 9, 10]` | `[2, 3]` | `overlapping-residue-classes` |
| 13 | `[2, 4]` | `[1, 2, 4, 6, 7, 8, 11, 12]` | `[2, 4]` | `overlapping-residue-classes` |
| 17 | `[8, 14]` | `[4, 5, 11, 12, 14, 15, 16]` | `[14]` | `overlapping-residue-classes` |
| 19 | `[12, 13]` | `[1, 4, 8, 13, 14, 16, 17]` | `[13]` | `overlapping-residue-classes` |
| 23 | `[2, 5]` | `[1, 9, 10, 11, 14, 15, 18, 20]` | `[]` | `exact-residue-separator` |
| 29 | `[4, 11]` | `[1, 3, 5, 7, 9, 15, 21, 24, 26]` | `[]` | `exact-residue-separator` |
| 31 | `[10, 17]` | `[1, 5, 7, 11, 16, 18, 21, 29]` | `[]` | `exact-residue-separator` |

### Residue-Lattice Connector Residue-Separator Replication

This block tests the residue-lattice branch's best residue separator on one more disjoint non-twin prime-pair ladder. Mutation or collapse keeps the result empirical and routes onward before any Lean classifier work.

- Source branch: `orthogonal-residuelattice-gap4-width6-connector-000122` family `cousin-prime-gap4` connector `000122` positions `[3, 4, 5]`
- Tested separator: mod `23` source reverse-only residues `[2, 5]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `1`; comparison rows `11`
- Fresh-ladder reverse-only residues: `[10]`; comparison residues `[1, 5, 9, 14, 15, 18, 20, 21, 22]`; shared residues `[]`
- Separator status: `split-exact-residue-separator-on-orthogonal-separator-ladder`; retained residues `0`; split residues `1`; collapsed source residues `2`
- Replication decision: `mod23-orthogonal-residue-separator-mutated-retire-branch`
- Next experiment target: `select-next-arithmetic-connector-family-after-residue-lattice-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 15 | `neither-prime-hit` | false | false | `000122` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 20 | `neither-prime-hit` | false | false | `000122` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 22 | `neither-prime-hit` | false | false | `000122` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 9 | `neither-prime-hit` | false | false | `000122` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 22 | `neither-prime-hit` | false | false | `000122` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 21 | `neither-prime-hit` | false | false | `000122` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 18 | `forward-only-prime-hit` | true | false | `000122` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 1 | `neither-prime-hit` | false | false | `000122` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 10 | `reverse-only-prime-hit` | false | true | `000122` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 20 | `forward-only-prime-hit` | true | false | `000122` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 5 | `neither-prime-hit` | false | false | `000122` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 14 | `neither-prime-hit` | false | false | `000122` |

### Modular-Walk Connector Control

This block pivots from residue-lattice connectors to bounded modular-walk traces outside the prior quotient window. It samples structured low-degree residue walks, then zero-pads the resulting values to each width. It remains empirical routing only, not a connector law or density mechanism.

- Matrix decision: `orthogonal-modular-walk-connector-control-survived-residue-profiler-next`
- Selection rule: `after-residue-lattice-collapse;pivot-to-modular-walk-connector-values-outside-prior-quotient-window;scan-q*m+f(q)-for-small-prime-moduli-and-fixed-affine-quadratic-traces;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `66000`; fresh rows `24`; widths `[5, 6, 7, 8, 9]`
- Position scope: `modular-walk connector values q*m+f(q) with m in [7,11,13,17,19,23], q in 25..=72, and f(q)=a2*q^2+a1*q+a0 over a fixed trace basis; nonzero digit positions are derived after value selection`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected modular-walk branch: `orthogonal-modularwalk-gap4-width6-connector-001139` family `cousin-prime-gap4` connector `001139` positions `[2, 3, 4, 5]` fresh reverse-only `5` target `residue-profile-orthogonal-modularwalk-gap4-connector-001139-width6`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `001139` | `[2, 3, 4, 5]` | 7 | 5 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-modularwalk-gap4-connector-001139-width6` |
| `sexy-prime-gap6` | 6 | `001054` | `[2, 4, 5]` | 4 | 0 | 0 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-modularwalk-gap6-connector-001054-width6` |

### Modular-Walk Connector Residue-Class Profile

This block profiles the survived modular-walk connector branch by small-prime residue classes. A separator here is only a third-ladder test candidate; no Lean classifier is proposed unless it survives unchanged.

- Source branch: `orthogonal-modularwalk-gap4-width6-connector-001139` family `cousin-prime-gap4` connector `001139` positions `[2, 3, 4, 5]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `5`; comparison pairs `7`; exact separators `1`
- Best separator: mod `29` reverse-only residues `[3, 7, 24, 26]` target `replicate-orthogonal-modularwalk-gap4-connector-001139-mod29-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[2, 3, 4]` | `[2, 3, 4]` | `[2, 3, 4]` | `overlapping-residue-classes` |
| 7 | `[2, 4, 5]` | `[1, 2, 4, 6]` | `[2, 4]` | `overlapping-residue-classes` |
| 11 | `[1, 3, 5, 8, 9]` | `[2, 3, 4, 5, 10]` | `[3, 5]` | `overlapping-residue-classes` |
| 13 | `[1, 4, 7, 8, 11]` | `[2, 4, 6, 11, 12]` | `[4, 11]` | `overlapping-residue-classes` |
| 17 | `[4, 5, 12, 15]` | `[4, 5, 8, 11, 14, 16]` | `[4, 5]` | `overlapping-residue-classes` |
| 19 | `[1, 8, 14, 17]` | `[4, 12, 13, 14, 16]` | `[14]` | `overlapping-residue-classes` |
| 23 | `[1, 9, 15, 18, 20]` | `[2, 5, 9, 10, 11, 14, 20]` | `[9, 20]` | `overlapping-residue-classes` |
| 29 | `[3, 7, 24, 26]` | `[1, 4, 5, 9, 11, 15, 21]` | `[]` | `exact-residue-separator` |
| 31 | `[5, 7, 11, 18, 21]` | `[1, 5, 10, 16, 17, 18, 29]` | `[5, 18]` | `overlapping-residue-classes` |

### Modular-Walk Connector Residue-Separator Replication

This block tests the modular-walk branch's best residue separator on one more disjoint non-twin prime-pair ladder. Mutation or collapse keeps the result empirical and routes onward before any Lean classifier work.

- Source branch: `orthogonal-modularwalk-gap4-width6-connector-001139` family `cousin-prime-gap4` connector `001139` positions `[2, 3, 4, 5]`
- Tested separator: mod `29` source reverse-only residues `[3, 7, 24, 26]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `1`; comparison rows `11`
- Fresh-ladder reverse-only residues: `[18]`; comparison residues `[1, 4, 6, 9, 12, 20, 24, 26]`; shared residues `[]`
- Separator status: `split-exact-residue-separator-on-orthogonal-separator-ladder`; retained residues `0`; split residues `1`; collapsed source residues `4`
- Replication decision: `mod29-orthogonal-residue-separator-mutated-retire-branch`
- Next experiment target: `select-next-arithmetic-connector-family-after-modular-walk-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 26 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 1 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 20 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 6 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 6 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 4 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 12 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 18 | `reverse-only-prime-hit` | false | true | `001139` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 9 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 24 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 20 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 26 | `neither-prime-hit` | false | false | `001139` |

### Arithmetic Connector Family Registry

This block records arithmetic connector families that already failed the source/fresh/separator gate before selecting another arithmetic surface. It is a routing and exclusion ledger, not evidence for or against a connector law.

- Registry decision: `retired-arithmetic-families-recorded-select-base-mixed-connectors-next`
- Retired families: `8`; active families: `1`; selected next family: `base-mixed` target `scan-orthogonal-base-mixed-connectors-under-source-fresh-separator-gate`

| Family | Class | Status | Evidence branch | Modulus | Decision | Target | Rationale |
|---|---|---|---|---:|---|---|---|
| `square-triangular` | `low-degree-integer-sequence` | `retired` | `orthogonal-arithmetic-gap4-width5-connector-04900` | `23` | `mod23-orthogonal-residue-separator-collapsed-retire-branch` | `select-next-arithmetic-connector-family-after-square-triangular-three-ladder-collapse` | square/triangular branch collapsed on the separator ladder; keep as retired arithmetic baseline |
| `residue-lattice` | `residue-lattice` | `retired` | `orthogonal-residuelattice-gap4-width6-connector-000122` | `23` | `mod23-orthogonal-residue-separator-mutated-retire-branch` | `select-next-arithmetic-connector-family-after-residue-lattice-three-ladder-collapse` | residue-lattice branch mutated on the separator ladder; do not reselect without a new design |
| `modular-walk` | `modular-walk` | `retired` | `orthogonal-modularwalk-gap4-width6-connector-001139` | `29` | `mod29-orthogonal-residue-separator-mutated-retire-branch` | `select-next-arithmetic-connector-family-after-modular-walk-three-ladder-collapse` | modular-walk branch split on the separator ladder; route to a genuinely different arithmetic family |
| `crt-paired` | `paired-congruence-system` | `retired` | `orthogonal-crtpaired-gap4-width6-connector-003727` | `23` | `mod23-orthogonal-residue-separator-collapsed-retire-branch` | `select-next-arithmetic-connector-family-after-crt-paired-three-ladder-collapse` | CRT-paired branch collapsed on the separator ladder; route to a non-CRT arithmetic family |
| `multiplicative-order` | `unit-order-orbit-family` | `retired` | `orthogonal-multorder-gap4-width6-connector-001139` | `29` | `mod29-orthogonal-residue-separator-mutated-retire-branch` | `select-next-arithmetic-connector-family-after-multiplicative-order-three-ladder-collapse` | multiplicative-order branch mutated on the separator ladder; keep excluded before inventing another arithmetic connector family |
| `automorphic-repunit` | `base10-fixed-point-perturbation-family` | `retired` | `orthogonal-automorphic-repunit-gap4-width5-connector-91736` | `29` | `mod29-orthogonal-residue-separator-collapsed-retire-branch` | `select-next-arithmetic-connector-family-after-automorphic-repunit-three-ladder-collapse` | automorphic/repunit branch failed the separator ladder; keep excluded before selecting another arithmetic connector family |
| `cyclic-reptend` | `decimal-repetend-window-family` | `retired` | `orthogonal-cyclic-reptend-gap6-width5-connector-53191` | `13` | `mod13-orthogonal-residue-separator-mutated-retire-branch` | `select-next-arithmetic-connector-family-after-cyclic-reptend-three-ladder-collapse` | cyclic-reptend branch failed the separator ladder; keep excluded before selecting another arithmetic connector family |
| `carry-chain` | `decimal-carry-propagation-family` | `retired` | `orthogonal-carry-chain-gap4-width9-connector-900020000` | `7` | `mod7-orthogonal-residue-separator-collapsed-retire-branch` | `select-next-arithmetic-connector-family-after-carry-chain-three-ladder-collapse` | carry-chain branch failed the separator ladder; keep excluded before selecting another arithmetic connector family |
| `base-mixed` | `base-mixed-maintained-lane-family` | `selected-next-family` | `none` | `none` | `none` | `scan-orthogonal-base-mixed-connectors-under-source-fresh-separator-gate` | next connector surface after decimal arithmetic-family collapse; interprets maintained base-6/base-12/base-30 lane words as decimal connector values |

### CRT-Paired Connector Control

This block pivots from modular-walk traces to paired CRT constraints over two small coprime primes. Values are still tested only by the empirical source/fresh/separator rule; no residue profiler result is theorem-candidate material unless it survives the separator ladder unchanged.

- Matrix decision: `orthogonal-crt-paired-connector-control-survived-residue-profiler-next`
- Selection rule: `after-modular-walk-collapse;pivot-to-crt-paired-connector-values;scan-x-constrained-by-two-small-prime-residue-equations-and-lifted-by-their-product;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `38880`; fresh rows `24`; widths `[5, 6, 7, 8, 9]`
- Position scope: `CRT-paired connector values x = lift*(m*n)+crt(r_m,r_n), with (m,n) in [(7,11),(7,13),(11,17),(13,19),(17,23)], lift in 16..=31, and residues generated from a fixed seed list; nonzero digit positions are derived after value selection`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected CRT-paired branch: `orthogonal-crtpaired-gap4-width6-connector-003727` family `cousin-prime-gap4` connector `003727` positions `[2, 3, 4, 5]` fresh reverse-only `2` target `residue-profile-orthogonal-crtpaired-gap4-connector-003727-width6`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `003727` | `[2, 3, 4, 5]` | 5 | 2 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-crtpaired-gap4-connector-003727-width6` |
| `sexy-prime-gap6` | 6 | `002495` | `[2, 3, 4, 5]` | 4 | 0 | 0 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-crtpaired-gap6-connector-002495-width6` |

### CRT-Paired Connector Residue-Class Profile

This block profiles the survived CRT-paired branch by small-prime residue classes. Separators here are third-ladder tests only, not connector laws.

- Source branch: `orthogonal-crtpaired-gap4-width6-connector-003727` family `cousin-prime-gap4` connector `003727` positions `[2, 3, 4, 5]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `2`; comparison pairs `10`; exact separators `1`
- Best separator: mod `23` reverse-only residues `[1, 18]` target `replicate-orthogonal-crtpaired-gap4-connector-003727-mod23-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[2, 4]` | `[2, 3, 4]` | `[2, 4]` | `overlapping-residue-classes` |
| 7 | `[2, 4]` | `[1, 2, 4, 5, 6]` | `[2, 4]` | `overlapping-residue-classes` |
| 11 | `[1, 5]` | `[2, 3, 4, 5, 8, 9, 10]` | `[5]` | `overlapping-residue-classes` |
| 13 | `[7, 11]` | `[1, 2, 4, 6, 8, 11, 12]` | `[11]` | `overlapping-residue-classes` |
| 17 | `[5, 12]` | `[4, 5, 8, 11, 14, 15, 16]` | `[5]` | `overlapping-residue-classes` |
| 19 | `[1, 17]` | `[1, 4, 8, 12, 13, 14, 16]` | `[1]` | `overlapping-residue-classes` |
| 23 | `[1, 18]` | `[2, 5, 9, 10, 11, 14, 15, 20]` | `[]` | `exact-residue-separator` |
| 29 | `[3, 24]` | `[1, 3, 4, 5, 7, 9, 11, 15, 21, 26]` | `[3]` | `overlapping-residue-classes` |
| 31 | `[18, 21]` | `[1, 5, 7, 10, 11, 16, 17, 18, 29]` | `[18]` | `overlapping-residue-classes` |

### CRT-Paired Connector Residue-Separator Replication

This block tests the CRT-paired branch's best residue separator on one more disjoint non-twin prime-pair ladder. Mutation or collapse keeps the result empirical and routes onward before any Lean classifier work.

- Source branch: `orthogonal-crtpaired-gap4-width6-connector-003727` family `cousin-prime-gap4` connector `003727` positions `[2, 3, 4, 5]`
- Tested separator: mod `23` source reverse-only residues `[1, 18]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `1`; comparison rows `11`
- Fresh-ladder reverse-only residues: `[22]`; comparison residues `[1, 5, 9, 10, 14, 15, 18, 20, 21, 22]`; shared residues `[22]`
- Separator status: `collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder`; retained residues `0`; split residues `1`; collapsed source residues `2`
- Replication decision: `mod23-orthogonal-residue-separator-collapsed-retire-branch`
- Next experiment target: `select-next-arithmetic-connector-family-after-crt-paired-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 15 | `neither-prime-hit` | false | false | `003727` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 20 | `neither-prime-hit` | false | false | `003727` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 22 | `neither-prime-hit` | false | false | `003727` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 9 | `neither-prime-hit` | false | false | `003727` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 22 | `reverse-only-prime-hit` | false | true | `003727` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 21 | `neither-prime-hit` | false | false | `003727` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 18 | `neither-prime-hit` | false | false | `003727` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 1 | `neither-prime-hit` | false | false | `003727` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 10 | `neither-prime-hit` | false | false | `003727` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 20 | `neither-prime-hit` | false | false | `003727` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 5 | `neither-prime-hit` | false | false | `003727` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 14 | `neither-prime-hit` | false | false | `003727` |

### Multiplicative-Order Connector Control

This block pivots away from CRT-paired connectors into small-prime unit-group power orbits lifted into connector values. The source/fresh gate may expose empirical signal, but no theorem-candidate language is used unless the separator ladder retains the same exact residue separator.

- Matrix decision: `orthogonal-multiplicative-order-connector-control-survived-residue-profiler-next`
- Selection rule: `after-crt-paired-collapse;pivot-to-multiplicative-order-connector-values;scan-small-prime-unit-power-orbits-lifted-outside-prior-quotient-windows;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `254160`; fresh rows `24`; widths `[5, 6, 7, 8, 9]`
- Position scope: `multiplicative-order connector values q*m+g^e mod m, with m in [7,11,13,17,19,23,29,31], generators [2,3,5,10] where coprime to m, e in 1..m-1, and q in 73..=120; nonzero digit positions are derived after value selection`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected multiplicative-order branch: `orthogonal-multorder-gap4-width6-connector-001139` family `cousin-prime-gap4` connector `001139` positions `[2, 3, 4, 5]` fresh reverse-only `5` target `residue-profile-orthogonal-multorder-gap4-connector-001139-width6`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `001139` | `[2, 3, 4, 5]` | 7 | 5 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-multorder-gap4-connector-001139-width6` |
| `sexy-prime-gap6` | 6 | `002495` | `[2, 3, 4, 5]` | 4 | 0 | 0 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-multorder-gap6-connector-002495-width6` |

### Multiplicative-Order Connector Residue-Class Profile

This block profiles the survived multiplicative-order branch by small-prime residue classes. Separators here are still empirical routing targets until a disjoint separator ladder retains them unchanged.

- Source branch: `orthogonal-multorder-gap4-width6-connector-001139` family `cousin-prime-gap4` connector `001139` positions `[2, 3, 4, 5]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `5`; comparison pairs `7`; exact separators `1`
- Best separator: mod `29` reverse-only residues `[3, 7, 24, 26]` target `replicate-orthogonal-multorder-gap4-connector-001139-mod29-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[2, 3, 4]` | `[2, 3, 4]` | `[2, 3, 4]` | `overlapping-residue-classes` |
| 7 | `[2, 4, 5]` | `[1, 2, 4, 6]` | `[2, 4]` | `overlapping-residue-classes` |
| 11 | `[1, 3, 5, 8, 9]` | `[2, 3, 4, 5, 10]` | `[3, 5]` | `overlapping-residue-classes` |
| 13 | `[1, 4, 7, 8, 11]` | `[2, 4, 6, 11, 12]` | `[4, 11]` | `overlapping-residue-classes` |
| 17 | `[4, 5, 12, 15]` | `[4, 5, 8, 11, 14, 16]` | `[4, 5]` | `overlapping-residue-classes` |
| 19 | `[1, 8, 14, 17]` | `[4, 12, 13, 14, 16]` | `[14]` | `overlapping-residue-classes` |
| 23 | `[1, 9, 15, 18, 20]` | `[2, 5, 9, 10, 11, 14, 20]` | `[9, 20]` | `overlapping-residue-classes` |
| 29 | `[3, 7, 24, 26]` | `[1, 4, 5, 9, 11, 15, 21]` | `[]` | `exact-residue-separator` |
| 31 | `[5, 7, 11, 18, 21]` | `[1, 5, 10, 16, 17, 18, 29]` | `[5, 18]` | `overlapping-residue-classes` |

### Multiplicative-Order Connector Residue-Separator Replication

This block tests the multiplicative-order branch's best residue separator on one more disjoint non-twin prime-pair ladder. Mutation or collapse retires the branch before any finite-classifier or connector-law language.

- Source branch: `orthogonal-multorder-gap4-width6-connector-001139` family `cousin-prime-gap4` connector `001139` positions `[2, 3, 4, 5]`
- Tested separator: mod `29` source reverse-only residues `[3, 7, 24, 26]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `1`; comparison rows `11`
- Fresh-ladder reverse-only residues: `[18]`; comparison residues `[1, 4, 6, 9, 12, 20, 24, 26]`; shared residues `[]`
- Separator status: `split-exact-residue-separator-on-orthogonal-separator-ladder`; retained residues `0`; split residues `1`; collapsed source residues `4`
- Replication decision: `mod29-orthogonal-residue-separator-mutated-retire-branch`
- Next experiment target: `select-next-arithmetic-connector-family-after-multiplicative-order-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 26 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 1 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 20 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 6 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 6 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 4 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 12 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 18 | `reverse-only-prime-hit` | false | true | `001139` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 9 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 24 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 20 | `neither-prime-hit` | false | false | `001139` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 26 | `neither-prime-hit` | false | false | `001139` |

### Automorphic/Repunit Connector Control

This block pivots from unit-order orbits to base-10 automorphic fixed-point residues and local repunit-block perturbations. The source/fresh gate may expose empirical signal, but theorem-candidate language still requires unchanged separator-ladder retention.

- Matrix decision: `orthogonal-automorphic-repunit-connector-control-survived-residue-profiler-next`
- Selection rule: `after-multiplicative-order-collapse;pivot-to-base10-automorphic-residues-and-local-repunit-block-perturbations;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `7128`; fresh rows `24`; widths `[5, 6, 7, 8, 9]`
- Position scope: `automorphic/repunit connector values: the two nontrivial base-10 automorphic residues modulo 10^width plus +/- R_len*10^shift perturbations with len in 1..=min(4,width); nonzero digit positions are derived after value selection`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected automorphic/repunit branch: `orthogonal-automorphic-repunit-gap4-width5-connector-91736` family `cousin-prime-gap4` connector `91736` positions `[0, 1, 2, 3, 4]` fresh reverse-only `2` target `residue-profile-orthogonal-automorphic-repunit-gap4-connector-91736-width5`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `91736` | `[0, 1, 2, 3, 4]` | 4 | 2 | 1 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-automorphic-repunit-gap4-connector-91736-width5` |
| `sexy-prime-gap6` | 6 | `79525` | `[0, 1, 2, 3, 4]` | 3 | 0 | 0 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-automorphic-repunit-gap6-connector-79525-width5` |

### Automorphic/Repunit Connector Residue-Class Profile

This block profiles the survived automorphic/repunit branch by small-prime residue classes. Separators remain empirical routing targets until a disjoint separator ladder retains them unchanged.

- Source branch: `orthogonal-automorphic-repunit-gap4-width5-connector-91736` family `cousin-prime-gap4` connector `91736` positions `[0, 1, 2, 3, 4]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `2`; comparison pairs `10`; exact separators `2`
- Best separator: mod `29` reverse-only residues `[5, 11]` target `replicate-orthogonal-automorphic-repunit-gap4-connector-91736-mod29-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[2]` | `[2, 3, 4]` | `[2]` | `overlapping-residue-classes` |
| 7 | `[4, 6]` | `[1, 2, 4, 5, 6]` | `[4, 6]` | `overlapping-residue-classes` |
| 11 | `[3, 10]` | `[1, 2, 3, 4, 5, 8, 9]` | `[3]` | `overlapping-residue-classes` |
| 13 | `[4, 6]` | `[1, 2, 4, 6, 7, 8, 11, 12]` | `[4, 6]` | `overlapping-residue-classes` |
| 17 | `[4, 14]` | `[4, 5, 8, 11, 12, 14, 15, 16]` | `[4, 14]` | `overlapping-residue-classes` |
| 19 | `[4, 13]` | `[1, 4, 8, 12, 13, 14, 16, 17]` | `[4, 13]` | `overlapping-residue-classes` |
| 23 | `[5, 9]` | `[1, 2, 9, 10, 11, 14, 15, 18, 20]` | `[9]` | `overlapping-residue-classes` |
| 29 | `[5, 11]` | `[1, 3, 4, 7, 9, 15, 21, 24, 26]` | `[]` | `exact-residue-separator` |
| 31 | `[10, 16]` | `[1, 5, 7, 11, 17, 18, 21, 29]` | `[]` | `exact-residue-separator` |

### Automorphic/Repunit Connector Residue-Separator Replication

This block tests the automorphic/repunit branch's best residue separator on one more disjoint non-twin prime-pair ladder. Mutation or collapse retires the branch before any finite-classifier or connector-law language.

- Source branch: `orthogonal-automorphic-repunit-gap4-width5-connector-91736` family `cousin-prime-gap4` connector `91736` positions `[0, 1, 2, 3, 4]`
- Tested separator: mod `29` source reverse-only residues `[5, 11]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `2`; comparison rows `10`
- Fresh-ladder reverse-only residues: `[24, 26]`; comparison residues `[1, 4, 6, 9, 12, 18, 20, 26]`; shared residues `[26]`
- Separator status: `collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder`; retained residues `0`; split residues `2`; collapsed source residues `2`
- Replication decision: `mod29-orthogonal-residue-separator-collapsed-retire-branch`
- Next experiment target: `select-next-arithmetic-connector-family-after-automorphic-repunit-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 26 | `neither-prime-hit` | false | false | `91736` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 1 | `neither-prime-hit` | false | false | `91736` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 20 | `neither-prime-hit` | false | false | `91736` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 6 | `neither-prime-hit` | false | false | `91736` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 6 | `neither-prime-hit` | false | false | `91736` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 4 | `neither-prime-hit` | false | false | `91736` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 12 | `neither-prime-hit` | false | false | `91736` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 18 | `neither-prime-hit` | false | false | `91736` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 9 | `neither-prime-hit` | false | false | `91736` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 24 | `reverse-only-prime-hit` | false | true | `91736` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 20 | `neither-prime-hit` | false | false | `91736` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 26 | `reverse-only-prime-hit` | false | true | `91736` |

### Cyclic-Reptend Connector Control

This block pivots from base-10 fixed points to cyclic windows of decimal repetends. The source/fresh gate may expose empirical signal, but theorem-candidate language still requires unchanged separator-ladder retention.

- Matrix decision: `orthogonal-cyclic-reptend-connector-control-survived-residue-profiler-next`
- Selection rule: `after-automorphic-repunit-collapse;pivot-to-cyclic-windows-of-decimal-repetends;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `11376`; fresh rows `24`; widths `[5, 6, 7, 8, 9]`
- Position scope: `cyclic-reptend connector values: width-length cyclic windows from decimal repetends of 1/p for small primes p coprime to 10; nonzero digit positions are derived after value selection`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected cyclic-reptend branch: `orthogonal-cyclic-reptend-gap6-width5-connector-53191` family `sexy-prime-gap6` connector `53191` positions `[0, 1, 2, 3, 4]` fresh reverse-only `1` target `residue-profile-orthogonal-cyclic-reptend-gap6-connector-53191-width5`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `42105263` | `[0, 1, 2, 4, 5, 6, 7]` | 4 | 1 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-cyclic-reptend-gap4-connector-42105263-width8` |
| `sexy-prime-gap6` | 6 | `53191` | `[0, 1, 2, 3, 4]` | 5 | 1 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-cyclic-reptend-gap6-connector-53191-width5` |

### Cyclic-Reptend Connector Residue-Class Profile

This block profiles the survived cyclic-reptend branch by small-prime residue classes. Separators remain empirical routing targets until a disjoint separator ladder retains them unchanged.

- Source branch: `orthogonal-cyclic-reptend-gap6-width5-connector-53191` family `sexy-prime-gap6` connector `53191` positions `[0, 1, 2, 3, 4]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `1`; comparison pairs `11`; exact separators `6`
- Best separator: mod `13` reverse-only residues `[6]` target `replicate-orthogonal-cyclic-reptend-gap6-connector-53191-mod13-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[2]` | `[1, 2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[1]` | `[1, 2, 3]` | `[1]` | `overlapping-residue-classes` |
| 7 | `[3]` | `[2, 3, 4, 5, 6]` | `[3]` | `overlapping-residue-classes` |
| 11 | `[1]` | `[1, 3, 6, 7, 8, 10]` | `[1]` | `overlapping-residue-classes` |
| 13 | `[6]` | `[1, 2, 8, 9, 11]` | `[]` | `exact-residue-separator` |
| 17 | `[13]` | `[1, 3, 4, 6, 7, 9, 10, 12, 15]` | `[]` | `exact-residue-separator` |
| 19 | `[6]` | `[2, 3, 4, 8, 9, 10, 14, 16, 17]` | `[]` | `exact-residue-separator` |
| 23 | `[7]` | `[2, 3, 8, 9, 11, 13, 14, 16, 19, 20]` | `[]` | `exact-residue-separator` |
| 29 | `[24]` | `[9, 12, 15, 18, 20, 21, 22, 26, 27]` | `[]` | `exact-residue-separator` |
| 31 | `[3]` | `[4, 5, 7, 9, 10, 12, 18, 24, 29, 30]` | `[]` | `exact-residue-separator` |

### Cyclic-Reptend Connector Residue-Separator Replication

This block tests the cyclic-reptend branch's best residue separator on one more disjoint non-twin prime-pair ladder. Mutation or collapse retires the branch before any finite-classifier or connector-law language.

- Source branch: `orthogonal-cyclic-reptend-gap6-width5-connector-53191` family `sexy-prime-gap6` connector `53191` positions `[0, 1, 2, 3, 4]`
- Tested separator: mod `13` source reverse-only residues `[6]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `1`; comparison rows `11`
- Fresh-ladder reverse-only residues: `[10]`; comparison residues `[2, 3, 4, 5, 6, 8, 9, 11, 12]`; shared residues `[]`
- Separator status: `split-exact-residue-separator-on-orthogonal-separator-ladder`; retained residues `0`; split residues `1`; collapsed source residues `1`
- Replication decision: `mod13-orthogonal-residue-separator-mutated-retire-branch`
- Next experiment target: `select-next-arithmetic-connector-family-after-cyclic-reptend-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Sexy-prime separator control pair (16091 ∘ 16097)` | 10 | `reverse-only-prime-hit` | false | true | `53191` |
| `Sexy-prime separator control pair (16097 ∘ 16103)` | 3 | `neither-prime-hit` | false | false | `53191` |
| `Sexy-prime separator control pair (16183 ∘ 16189)` | 11 | `neither-prime-hit` | false | false | `53191` |
| `Sexy-prime separator control pair (16187 ∘ 16193)` | 2 | `neither-prime-hit` | false | false | `53191` |
| `Sexy-prime separator control pair (16217 ∘ 16223)` | 6 | `neither-prime-hit` | false | false | `53191` |
| `Sexy-prime separator control pair (16223 ∘ 16229)` | 12 | `neither-prime-hit` | false | false | `53191` |
| `Sexy-prime separator control pair (16267 ∘ 16273)` | 4 | `neither-prime-hit` | false | false | `53191` |
| `Sexy-prime separator control pair (16333 ∘ 16339)` | 5 | `neither-prime-hit` | false | false | `53191` |
| `Sexy-prime separator control pair (16363 ∘ 16369)` | 9 | `neither-prime-hit` | false | false | `53191` |
| `Sexy-prime separator control pair (16411 ∘ 16417)` | 5 | `neither-prime-hit` | false | false | `53191` |
| `Sexy-prime separator control pair (16421 ∘ 16427)` | 2 | `neither-prime-hit` | false | false | `53191` |
| `Sexy-prime separator control pair (16427 ∘ 16433)` | 8 | `neither-prime-hit` | false | false | `53191` |

### Carry-Chain Connector Control

This block pivots from repetend windows to decimal carry-propagation geometry: trigger digits next to 9-runs and their borrow-chain complements. The source/fresh gate may expose empirical signal, but theorem-candidate language still requires unchanged separator-ladder retention.

- Matrix decision: `orthogonal-carry-chain-connector-control-survived-residue-profiler-next`
- Selection rule: `after-cyclic-reptend-collapse;pivot-to-decimal-carry-chain-and-borrow-chain-connectors;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `30240`; fresh rows `24`; widths `[5, 6, 7, 8, 9]`
- Position scope: `carry-chain connector values: a trigger digit adjacent to a run of 9s, plus width-complement borrow-chain partners; nonzero digit positions are derived after value selection`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected carry-chain branch: `orthogonal-carry-chain-gap4-width9-connector-900020000` family `cousin-prime-gap4` connector `900020000` positions `[0, 4]` fresh reverse-only `1` target `residue-profile-orthogonal-carry-chain-gap4-connector-900020000-width9`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `900020000` | `[0, 4]` | 5 | 1 | 1 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-orthogonal-carry-chain-gap4-connector-900020000-width9` |
| `sexy-prime-gap6` | 6 | `00999994` | `[2, 3, 4, 5, 6, 7]` | 4 | 0 | 0 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-carry-chain-gap6-connector-00999994-width8` |

### Carry-Chain Connector Residue-Class Profile

This block profiles the survived carry-chain branch by small-prime residue classes. Separators remain empirical routing targets until a disjoint separator ladder retains them unchanged.

- Source branch: `orthogonal-carry-chain-gap4-width9-connector-900020000` family `cousin-prime-gap4` connector `900020000` positions `[0, 4]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Reverse-only pairs: `1`; comparison pairs `11`; exact separators `4`
- Best separator: mod `7` reverse-only residues `[1]` target `replicate-orthogonal-carry-chain-gap4-connector-900020000-mod7-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[2]` | `[2, 3, 4]` | `[2]` | `overlapping-residue-classes` |
| 7 | `[1]` | `[2, 4, 5, 6]` | `[]` | `exact-residue-separator` |
| 11 | `[5]` | `[1, 2, 3, 4, 5, 8, 9, 10]` | `[5]` | `overlapping-residue-classes` |
| 13 | `[6]` | `[1, 2, 4, 6, 7, 8, 11, 12]` | `[6]` | `overlapping-residue-classes` |
| 17 | `[5]` | `[4, 5, 8, 11, 12, 14, 15, 16]` | `[5]` | `overlapping-residue-classes` |
| 19 | `[13]` | `[1, 4, 8, 12, 13, 14, 16, 17]` | `[13]` | `overlapping-residue-classes` |
| 23 | `[10]` | `[1, 2, 5, 9, 11, 14, 15, 18, 20]` | `[]` | `exact-residue-separator` |
| 29 | `[21]` | `[1, 3, 4, 5, 7, 9, 11, 15, 24, 26]` | `[]` | `exact-residue-separator` |
| 31 | `[29]` | `[1, 5, 7, 10, 11, 16, 17, 18, 21]` | `[]` | `exact-residue-separator` |

### Carry-Chain Connector Residue-Separator Replication

This block tests the carry-chain branch's best residue separator on one more disjoint non-twin prime-pair ladder. Mutation or collapse retires the branch before any finite-classifier or connector-law language.

- Source branch: `orthogonal-carry-chain-gap4-width9-connector-900020000` family `cousin-prime-gap4` connector `900020000` positions `[0, 4]`
- Tested separator: mod `7` source reverse-only residues `[1]`
- Fresh ladder pairs: `12`; rows `12`; reverse-only rows `0`; comparison rows `12`
- Fresh-ladder reverse-only residues: `[]`; comparison residues `[1, 2, 4, 5, 6]`; shared residues `[]`
- Separator status: `collapsed-no-reverse-only-on-orthogonal-separator-ladder`; retained residues `0`; split residues `0`; collapsed source residues `1`
- Replication decision: `mod7-orthogonal-residue-separator-collapsed-retire-branch`
- Next experiment target: `select-next-arithmetic-connector-family-after-carry-chain-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Cousin-prime separator control pair (16759 ∘ 16763)` | 1 | `neither-prime-hit` | false | false | `900020000` |
| `Cousin-prime separator control pair (16879 ∘ 16883)` | 2 | `neither-prime-hit` | false | false | `900020000` |
| `Cousin-prime separator control pair (16927 ∘ 16931)` | 1 | `neither-prime-hit` | false | false | `900020000` |
| `Cousin-prime separator control pair (17029 ∘ 17033)` | 5 | `neither-prime-hit` | false | false | `900020000` |
| `Cousin-prime separator control pair (17203 ∘ 17207)` | 4 | `neither-prime-hit` | false | false | `900020000` |
| `Cousin-prime separator control pair (17317 ∘ 17321)` | 6 | `neither-prime-hit` | false | false | `900020000` |
| `Cousin-prime separator control pair (17383 ∘ 17387)` | 2 | `neither-prime-hit` | false | false | `900020000` |
| `Cousin-prime separator control pair (17389 ∘ 17393)` | 1 | `neither-prime-hit` | false | false | `900020000` |
| `Cousin-prime separator control pair (17467 ∘ 17471)` | 2 | `neither-prime-hit` | false | false | `900020000` |
| `Cousin-prime separator control pair (17569 ∘ 17573)` | 6 | `neither-prime-hit` | false | false | `900020000` |
| `Cousin-prime separator control pair (17623 ∘ 17627)` | 4 | `both-prime-hit` | true | true | `900020000` |
| `Cousin-prime separator control pair (17977 ∘ 17981)` | 1 | `forward-only-prime-hit` | true | false | `900020000` |

### Base-Mixed Connector Control

This block pivots away from decimal-only arithmetic connector families. It evaluates maintained base-6/base-12/base-30 symmetric lane words as decimal connector values, then applies the same source/fresh gate. Collapse here is an empirical routing result, not a density claim.

- Matrix decision: `orthogonal-base-mixed-connector-control-collapsed-retire-without-lean`
- Selection rule: `after-decimal-arithmetic-family-collapse;pivot-to-non-decimal-maintained-lane-template-values;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `2`; source rows `23040`; fresh rows `24`; widths `[5, 6, 7, 8, 9]`
- Position scope: `base-mixed connector values: maintained base-6/base-12/base-30 symmetric lane template values for middle widths 1..=2, scanning the first 120 fixed-width seed values per lane and converting each template value to a decimal connector integer`; excluded connector rule: `exclude connector values divisible by 3 as exact-arithmetic nuisance filter; no non-twin theorem claim attached`
- Selected base-mixed branch: `none`; next target `select-next-connector-surface-after-base-mixed-source-fresh-collapse`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `cousin-prime-gap4` | 4 | `276061` | `[0, 1, 2, 4, 5]` | 6 | 0 | 2 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-base-mixed-gap4-connector-276061-width6` |
| `sexy-prime-gap6` | 6 | `0286717` | `[1, 2, 3, 4, 5, 6]` | 4 | 0 | 1 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-orthogonal-base-mixed-gap6-connector-0286717-width7` |

### Connector Stress Meta-Atlas

This block summarizes retired connector stress branch classes and selects the next empirical surface. It is a routing atlas, not a density mechanism or connector law.

- Atlas decision: `retired-connector-value-branches-recorded-select-pair-family-gap-portfolio`
- Branch classes: retired `18`; active `1`; theorem candidates `0`
- Selected surface: `pair-family-gap-portfolio` target `scan-pair-family-gap-portfolio-over-retired-connector-heads`

| Branch class | Surface | Status | Evidence | Evidence status | Decision | Target | Rationale |
|---|---|---|---|---|---|---|---|
| `digit8-edge-classifier-family` | `small-digit-edge` | `retired` | `digit8-edge-classifier-family` | `collapsed` | `none-digit8-branch-stopped-after-second-stage-collapse` | `stop-digit8-edge-classifier-family-after-outside-ladder-collapse` | the theorem-backed digit-8 ladder did not survive outside-ladder replication |
| `edge-singleton-nonmod3-family` | `small-digit-edge` | `retired` | `edge-singleton-retirement-summary` | `collapsed` | `collapsed-after-independent-ladders` | `pivot-away-from-edge-singleton-connectors` | 0000007, 0000001, and 00000005 failed independent-ladder promotion |
| `single-digit-interior-nonmod3-family` | `small-digit-interior` | `retired` | `interior-nonmod3-retirement-summary` | `collapsed` | `repeated-source-selected-branches-collapsed` | `pivot-away-from-single-digit-interior-connectors` | six interior single-digit candidates collapsed or mutated before theorem material |
| `multi-digit-motif-01100` | `multi-digit-motif` | `retired` | `01100` | `collapsed` | `mod11-multidigit-residue-separator-collapsed-retire-branch` | `pivot-to-orthogonal-pair-family-controls-after-01100-collapse` | the 01100 separator failed on the second disjoint separator ladder |
| `orthogonal-adjacent-two-digit-motifs` | `orthogonal-motif` | `retired` | `orthogonal-gap4-width5-start1-digits47-connector-04700` | `collapsed` | `mod13-orthogonal-residue-separator-collapsed-retire-branch` | `pivot-away-from-orthogonal-adjacent-two-digit-motifs-after-repeated-three-ladder-collapse` | adjacent two-digit orthogonal motifs failed the three-ladder gate |
| `orthogonal-compact-three-digit-motifs` | `orthogonal-motif` | `retired` | `orthogonal-compact3-gap4-width5-start1-digits251-connector-02510` | `collapsed` | `mod29-orthogonal-residue-separator-collapsed-retire-branch` | `select-next-orthogonal-nonadjacent-two-digit-motif-family-after-compact-three-digit-collapse` | compact three-digit motifs failed the separator ladder |
| `orthogonal-nonadjacent-two-digit-motifs` | `orthogonal-motif` | `retired` | `orthogonal-nonadjacent2-gap4-width7-pos25-digits52-connector-0050020` | `mutated` | `mod11-orthogonal-residue-separator-mutated-retire-branch` | `pivot-away-from-small-digit-orthogonal-motifs-after-nonadjacent-two-digit-three-ladder-collapse` | non-adjacent two-digit motifs mutated on the separator ladder |
| `orthogonal-edge-plus-interior-motifs` | `orthogonal-motif` | `retired` | `orthogonal-edgeplus2-gap4-width5-pos34-digits22-connector-00022` | `collapsed` | `mod13-orthogonal-residue-separator-collapsed-retire-branch` | `pivot-away-from-edge-plus-interior-orthogonal-motifs-after-three-ladder-collapse` | edge-plus-interior motifs failed the separator ladder |
| `orthogonal-repeated-block-motifs` | `orthogonal-motif` | `retired` | `orthogonal-repeatblock-gap4-width9-pos3467-digits5555-connector-000550550` | `collapsed` | `mod29-orthogonal-residue-separator-collapsed-retire-branch` | `pivot-to-arithmetic-connector-families-after-repeated-block-three-ladder-collapse` | repeated-block motifs failed the separator ladder |
| `square-triangular` | `arithmetic-connector-family` | `retired` | `orthogonal-arithmetic-gap4-width5-connector-04900` | `collapsed` | `mod23-orthogonal-residue-separator-collapsed-retire-branch` | `retired-square-triangular-connector-family` | arithmetic connector family failed the source/fresh/separator guardrail |
| `residue-lattice` | `arithmetic-connector-family` | `retired` | `orthogonal-residuelattice-gap4-width6-connector-000122` | `mutated` | `mod23-orthogonal-residue-separator-mutated-retire-branch` | `retired-residue-lattice-connector-family` | arithmetic connector family failed the source/fresh/separator guardrail |
| `modular-walk` | `arithmetic-connector-family` | `retired` | `orthogonal-modularwalk-gap4-width6-connector-001139` | `mutated` | `mod29-orthogonal-residue-separator-mutated-retire-branch` | `retired-modular-walk-connector-family` | arithmetic connector family failed the source/fresh/separator guardrail |
| `crt-paired` | `arithmetic-connector-family` | `retired` | `orthogonal-crtpaired-gap4-width6-connector-003727` | `collapsed` | `mod23-orthogonal-residue-separator-collapsed-retire-branch` | `retired-crt-paired-connector-family` | arithmetic connector family failed the source/fresh/separator guardrail |
| `multiplicative-order` | `arithmetic-connector-family` | `retired` | `orthogonal-multorder-gap4-width6-connector-001139` | `mutated` | `mod29-orthogonal-residue-separator-mutated-retire-branch` | `retired-multiplicative-order-connector-family` | arithmetic connector family failed the source/fresh/separator guardrail |
| `automorphic-repunit` | `arithmetic-connector-family` | `retired` | `orthogonal-automorphic-repunit-gap4-width5-connector-91736` | `collapsed` | `mod29-orthogonal-residue-separator-collapsed-retire-branch` | `retired-automorphic-repunit-connector-family` | arithmetic connector family failed the source/fresh/separator guardrail |
| `cyclic-reptend` | `arithmetic-connector-family` | `retired` | `orthogonal-cyclic-reptend-gap6-width5-connector-53191` | `mutated` | `mod13-orthogonal-residue-separator-mutated-retire-branch` | `retired-cyclic-reptend-connector-family` | arithmetic connector family failed the source/fresh/separator guardrail |
| `carry-chain` | `arithmetic-connector-family` | `retired` | `orthogonal-carry-chain-gap4-width9-connector-900020000` | `collapsed` | `mod7-orthogonal-residue-separator-collapsed-retire-branch` | `retired-carry-chain-connector-family` | arithmetic connector family failed the source/fresh/separator guardrail |
| `base-mixed` | `base-mixed-maintained-lane-family` | `retired` | `orthogonal-base-mixed-source-fresh-control` | `collapsed-before-residue-profile` | `orthogonal-base-mixed-connector-control-collapsed-retire-without-lean` | `select-next-connector-surface-after-base-mixed-source-fresh-collapse` | base-mixed maintained-lane connector heads found source signal but no fresh reverse-only row |
| `pair-family-gap-portfolio` | `pair-family-side-control` | `selected-next-surface` | `none` | `none` | `none` | `scan-pair-family-gap-portfolio-over-retired-connector-heads` | connector-value family auditions repeatedly collapsed; vary pair-family gaps while freezing representative connector heads |

### Pair-Family Gap Portfolio Control

This block freezes representative connector heads from retired branches and varies the prime-pair gap family instead. Source/fresh/separator survival is still required before residue-profiler or Lean theorem work.

- Matrix decision: `pair-family-gap-portfolio-control-survived-residue-profiler-next`
- Selection rule: `after-retired-connector-value-branch-meta-atlas;freeze-retired-connector-heads;vary-prime-pair-gap-families-through-source-fresh-separator-gate`
- Pair families: `3`; source rows `504`; fresh rows `36`; widths `[5, 6, 7, 9]`
- Position scope: `frozen connector portfolio from retired branch heads; pair-family side varies across gap 8, gap 10, and gap 12 prime pairs`; excluded connector rule: `reuse connector-value row builder and existing mod-3 nuisance filter; no new residue theorem claim attached`
- Selected pair-family branch: `pair-family-gap-portfolio-gap8-width6-connector-003727` family `prime-gap8` connector `003727` positions `[2, 3, 4, 5]` fresh reverse-only `3` target `residue-profile-pair-family-gap-portfolio-gap8-connector-003727-width6`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `prime-gap8` | 8 | `003727` | `[2, 3, 4, 5]` | 3 | 3 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-pair-family-gap-portfolio-gap8-connector-003727-width6` |
| `prime-gap10` | 10 | `0286717` | `[1, 2, 3, 4, 5, 6]` | 2 | 1 | 3 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-pair-family-gap-portfolio-gap10-connector-0286717-width7` |
| `prime-gap12` | 12 | `02510` | `[1, 2, 3]` | 2 | 0 | 0 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-pair-family-gap-portfolio-gap12-connector-02510-width5` |

### Pair-Family Gap Portfolio Residue-Class Profile

This block profiles a survived pair-family gap portfolio branch by small-prime residue classes. It remains empirical unless the separator ladder retains the same separator unchanged.

- Source branch: `pair-family-gap-portfolio-gap8-width6-connector-003727` family `prime-gap8` connector `003727` positions `[2, 3, 4, 5]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Best separator: mod `23` reverse-only residues `[10, 12, 20]` target `replicate-pair-family-gap-portfolio-gap8-connector-003727-mod23-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[2]` | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[1, 4]` | `[1, 3, 4]` | `[1, 4]` | `overlapping-residue-classes` |
| 7 | `[2, 5]` | `[1, 2, 3, 4, 5]` | `[2, 5]` | `overlapping-residue-classes` |
| 11 | `[6, 7, 9]` | `[4, 5, 6, 8, 9, 10]` | `[6, 9]` | `overlapping-residue-classes` |
| 13 | `[9, 11]` | `[1, 2, 6, 10, 11, 12]` | `[11]` | `overlapping-residue-classes` |
| 17 | `[1, 16]` | `[1, 2, 3, 6, 13, 14, 16]` | `[1, 16]` | `overlapping-residue-classes` |
| 19 | `[2, 3, 10]` | `[1, 2, 4, 7, 8, 13]` | `[2]` | `overlapping-residue-classes` |
| 23 | `[10, 12, 20]` | `[1, 9, 13, 14, 16, 17, 18, 19]` | `[]` | `exact-residue-separator` |
| 29 | `[5, 8, 20]` | `[2, 3, 5, 9, 10, 12, 14, 22, 26]` | `[5]` | `overlapping-residue-classes` |
| 31 | `[7, 12, 21]` | `[3, 6, 7, 8, 10, 24, 27]` | `[7]` | `overlapping-residue-classes` |

### Pair-Family Gap Portfolio Residue-Separator Replication

This block tests a pair-family gap portfolio separator on a disjoint separator ladder. Mutation or collapse retires the branch before any finite-classifier language.

- Source branch: `pair-family-gap-portfolio-gap8-width6-connector-003727` family `prime-gap8` connector `003727` positions `[2, 3, 4, 5]`
- Separator status: `split-exact-residue-separator-on-orthogonal-separator-ladder`; replication decision `mod23-orthogonal-residue-separator-mutated-retire-branch`; next target `select-new-pair-family-side-control-after-gap-portfolio-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Gap-8 separator control pair (22271 ∘ 22279)` | 7 | `neither-prime-hit` | false | false | `003727` |
| `Gap-8 separator control pair (22283 ∘ 22291)` | 19 | `neither-prime-hit` | false | false | `003727` |
| `Gap-8 separator control pair (22433 ∘ 22441)` | 8 | `forward-only-prime-hit` | true | false | `003727` |
| `Gap-8 separator control pair (22541 ∘ 22549)` | 1 | `neither-prime-hit` | false | false | `003727` |
| `Gap-8 separator control pair (22613 ∘ 22621)` | 4 | `neither-prime-hit` | false | false | `003727` |
| `Gap-8 separator control pair (22643 ∘ 22651)` | 11 | `neither-prime-hit` | false | false | `003727` |
| `Gap-8 separator control pair (22691 ∘ 22699)` | 13 | `reverse-only-prime-hit` | false | true | `003727` |
| `Gap-8 separator control pair (22709 ∘ 22717)` | 8 | `neither-prime-hit` | false | false | `003727` |
| `Gap-8 separator control pair (22769 ∘ 22777)` | 22 | `neither-prime-hit` | false | false | `003727` |
| `Gap-8 separator control pair (22853 ∘ 22861)` | 14 | `neither-prime-hit` | false | false | `003727` |
| `Gap-8 separator control pair (23003 ∘ 23011)` | 3 | `neither-prime-hit` | false | false | `003727` |
| `Gap-8 separator control pair (23021 ∘ 23029)` | 21 | `neither-prime-hit` | false | false | `003727` |

### Pair-Family Gap Extension Control

This block follows the gap-portfolio collapse by keeping the retired connector-head portfolio fixed and extending the pair-family side to prime gaps 14, 16, and 18. Source/fresh/separator survival is still required before residue-profiler or Lean theorem work.

- Matrix decision: `pair-family-gap-extension-control-survived-residue-profiler-next`
- Selection rule: `after-gap-portfolio-separator-mutation;reuse-retired-connector-head-portfolio;extend-prime-pair-gap-families-to-14-16-18;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `3`; source rows `504`; fresh rows `36`; widths `[5, 6, 7, 9]`
- Position scope: `frozen connector portfolio from retired branch heads; pair-family side extends beyond gap 8/10/12 to gap 14, gap 16, and gap 18 prime pairs`; excluded connector rule: `reuse connector-value row builder and existing mod-3 nuisance filter; no new residue theorem claim attached`
- Selected pair-family branch: `pair-family-gap-extension-gap16-width6-connector-276061` family `prime-gap16` connector `276061` positions `[0, 1, 2, 4, 5]` fresh reverse-only `2` target `residue-profile-pair-family-gap-extension-gap16-connector-276061-width6`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `prime-gap14` | 14 | `91736` | `[0, 1, 2, 3, 4]` | 5 | 0 | 2 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-pair-family-gap-extension-gap14-connector-91736-width5` |
| `prime-gap16` | 16 | `276061` | `[0, 1, 2, 4, 5]` | 3 | 2 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-pair-family-gap-extension-gap16-connector-276061-width6` |
| `prime-gap18` | 18 | `003727` | `[2, 3, 4, 5]` | 3 | 0 | 0 | `collapsed-orthogonal-pair-family-independent-ladder-retire-without-lean` | `retire-pair-family-gap-extension-gap18-connector-003727-width6` |

### Pair-Family Gap Extension Residue-Class Profile

This block profiles a survived gap-extension branch by small-prime residue classes. It remains empirical unless the separator ladder retains the same separator unchanged.

- Source branch: `pair-family-gap-extension-gap16-width6-connector-276061` family `prime-gap16` connector `276061` positions `[0, 1, 2, 4, 5]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Best separator: mod `19` reverse-only residues `[11, 17]` target `replicate-pair-family-gap-extension-gap16-connector-276061-mod19-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[1]` | `[1]` | `[1]` | `overlapping-residue-classes` |
| 5 | `[1, 3]` | `[1, 2, 3]` | `[1, 3]` | `overlapping-residue-classes` |
| 7 | `[3, 4]` | `[1, 2, 3, 4, 6]` | `[3, 4]` | `overlapping-residue-classes` |
| 11 | `[3, 7]` | `[1, 2, 3, 4, 5, 7, 8, 9]` | `[3, 7]` | `overlapping-residue-classes` |
| 13 | `[5, 7]` | `[1, 2, 3, 5, 6, 8, 9, 11]` | `[5]` | `overlapping-residue-classes` |
| 17 | `[14, 16]` | `[2, 5, 8, 10, 12, 13, 14]` | `[14]` | `overlapping-residue-classes` |
| 19 | `[11, 17]` | `[2, 6, 9, 10, 16, 18]` | `[]` | `exact-residue-separator` |
| 23 | `[1, 12]` | `[2, 5, 6, 8, 9, 10, 15, 18]` | `[]` | `exact-residue-separator` |
| 29 | `[3, 21]` | `[1, 2, 16, 17, 18, 19, 22, 24, 25, 27]` | `[]` | `exact-residue-separator` |
| 31 | `[19, 23]` | `[1, 2, 4, 5, 14, 17, 22, 26, 27]` | `[]` | `exact-residue-separator` |

### Pair-Family Gap Extension Residue-Separator Replication

This block tests a gap-extension separator on a disjoint separator ladder. Mutation or collapse retires the branch before any finite-classifier language.

- Source branch: `pair-family-gap-extension-gap16-width6-connector-276061` family `prime-gap16` connector `276061` positions `[0, 1, 2, 4, 5]`
- Separator status: `split-exact-residue-separator-on-orthogonal-separator-ladder`; replication decision `mod19-orthogonal-residue-separator-mutated-retire-branch`; next target `select-new-pair-family-side-control-after-gap-extension-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Gap-16 separator control pair (35983 ∘ 35999)` | 16 | `neither-prime-hit` | false | false | `276061` |
| `Gap-16 separator control pair (36067 ∘ 36083)` | 5 | `neither-prime-hit` | false | false | `276061` |
| `Gap-16 separator control pair (36277 ∘ 36293)` | 6 | `neither-prime-hit` | false | false | `276061` |
| `Gap-16 separator control pair (36373 ∘ 36389)` | 7 | `neither-prime-hit` | false | false | `276061` |
| `Gap-16 separator control pair (36451 ∘ 36467)` | 9 | `neither-prime-hit` | false | false | `276061` |
| `Gap-16 separator control pair (36457 ∘ 36473)` | 15 | `neither-prime-hit` | false | false | `276061` |
| `Gap-16 separator control pair (36571 ∘ 36587)` | 15 | `neither-prime-hit` | false | false | `276061` |
| `Gap-16 separator control pair (36583 ∘ 36599)` | 8 | `neither-prime-hit` | false | false | `276061` |
| `Gap-16 separator control pair (36637 ∘ 36653)` | 5 | `neither-prime-hit` | false | false | `276061` |
| `Gap-16 separator control pair (36697 ∘ 36713)` | 8 | `neither-prime-hit` | false | false | `276061` |
| `Gap-16 separator control pair (36793 ∘ 36809)` | 9 | `neither-prime-hit` | false | false | `276061` |
| `Gap-16 separator control pair (36871 ∘ 36887)` | 11 | `reverse-only-prime-hit` | false | true | `276061` |

### Pair-Family Size-Band Control

This block follows the gap-extension collapse by holding gap 8 fixed, keeping the retired connector-head portfolio fixed, and moving through disjoint prime-pair size bands. Source/fresh/separator survival is still required before residue-profiler or Lean theorem work.

- Matrix decision: `pair-family-size-band-control-survived-residue-profiler-next`
- Selection rule: `after-gap-extension-separator-mutation;reuse-retired-connector-head-portfolio;hold-gap8-fixed-and-vary-prime-pair-size-bands-40k-80k-120k;source-fresh-separator-survival-before-theorem-candidate`
- Pair families: `3`; source rows `504`; fresh rows `36`; widths `[5, 6, 7, 9]`
- Position scope: `frozen connector portfolio from retired branch heads; pair-family side holds gap 8 fixed while source/fresh/separator ladders move through disjoint 40k, 80k, and 120k size bands`; excluded connector rule: `reuse connector-value row builder and existing mod-3 nuisance filter; no new residue theorem claim attached`
- Selected pair-family branch: `pair-family-size-band-prime-gap8-size120k-gap8-width5-connector-91736` family `prime-gap8-size120k` connector `91736` positions `[0, 1, 2, 3, 4]` fresh reverse-only `1` target `residue-profile-pair-family-size-band-prime-gap8-size120k-connector-91736-width5`

| Family | Gap | Source selected connector | Positions | Source reverse-only | Fresh reverse-only | Fresh forward-only | Decision | Target |
|---|---:|---|---|---:|---:|---:|---|---|
| `prime-gap8-size40k` | 8 | `91736` | `[0, 1, 2, 3, 4]` | 3 | 1 | 1 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-pair-family-size-band-prime-gap8-size40k-connector-91736-width5` |
| `prime-gap8-size80k` | 8 | `0050020` | `[2, 5]` | 4 | 1 | 0 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-pair-family-size-band-prime-gap8-size80k-connector-0050020-width7` |
| `prime-gap8-size120k` | 8 | `91736` | `[0, 1, 2, 3, 4]` | 4 | 1 | 2 | `survived-orthogonal-pair-family-independent-ladder-residue-profiler-next` | `residue-profile-pair-family-size-band-prime-gap8-size120k-connector-91736-width5` |

### Pair-Family Size-Band Residue-Class Profile

This block profiles a survived size-band branch by small-prime residue classes. It remains empirical unless the separator ladder retains the same separator unchanged.

- Source branch: `pair-family-size-band-prime-gap8-size120k-gap8-width5-connector-91736` family `prime-gap8-size120k` connector `91736` positions `[0, 1, 2, 3, 4]`
- Profile decision: `small-prime-orthogonal-residue-separator-found-replicate-before-lean`
- Best separator: mod `11` reverse-only residues `[6]` target `replicate-pair-family-size-band-prime-gap8-size120k-connector-91736-mod11-residue-separator-before-lean`

| Modulus | Reverse-only residues | Comparison residues | Shared residues | Status |
|---:|---|---|---|---|
| 3 | `[2]` | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[3]` | `[1, 3, 4]` | `[3]` | `overlapping-residue-classes` |
| 7 | `[3]` | `[1, 2, 3, 4, 5]` | `[3]` | `overlapping-residue-classes` |
| 11 | `[6]` | `[2, 4, 5, 7, 8, 9, 10]` | `[]` | `exact-residue-separator` |
| 13 | `[12]` | `[1, 2, 3, 6, 7, 8, 10, 11]` | `[]` | `exact-residue-separator` |
| 17 | `[7]` | `[1, 3, 4, 5, 7, 8, 10, 11, 12, 14]` | `[7]` | `overlapping-residue-classes` |
| 19 | `[18]` | `[3, 7, 9, 10, 12, 14, 15, 17]` | `[]` | `exact-residue-separator` |
| 23 | `[14]` | `[1, 2, 6, 7, 10, 11, 14, 16, 18, 22]` | `[14]` | `overlapping-residue-classes` |
| 29 | `[13]` | `[1, 3, 6, 7, 12, 13, 16, 18, 22, 24]` | `[13]` | `overlapping-residue-classes` |
| 31 | `[3]` | `[4, 5, 8, 9, 10, 12, 15, 17, 20, 30]` | `[]` | `exact-residue-separator` |

### Pair-Family Size-Band Residue-Separator Replication

This block tests a size-band separator on a disjoint separator ladder. Mutation or collapse retires the branch before any finite-classifier language.

- Source branch: `pair-family-size-band-prime-gap8-size120k-gap8-width5-connector-91736` family `prime-gap8-size120k` connector `91736` positions `[0, 1, 2, 3, 4]`
- Separator status: `collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder`; replication decision `mod11-orthogonal-residue-separator-collapsed-retire-branch`; next target `select-new-pair-family-side-control-after-size-band-three-ladder-collapse`

| Pair | Left mod source | Contrast | Forward prime | Reverse prime | Connector |
|---|---:|---|---:|---:|---|
| `Gap-8 size-120k separator control pair (122501 ∘ 122509)` | 5 | `neither-prime-hit` | false | false | `91736` |
| `Gap-8 size-120k separator control pair (122693 ∘ 122701)` | 10 | `neither-prime-hit` | false | false | `91736` |
| `Gap-8 size-120k separator control pair (122753 ∘ 122761)` | 4 | `reverse-only-prime-hit` | false | true | `91736` |
| `Gap-8 size-120k separator control pair (122819 ∘ 122827)` | 4 | `both-prime-hit` | true | true | `91736` |
| `Gap-8 size-120k separator control pair (122861 ∘ 122869)` | 2 | `neither-prime-hit` | false | false | `91736` |
| `Gap-8 size-120k separator control pair (122921 ∘ 122929)` | 7 | `neither-prime-hit` | false | false | `91736` |
| `Gap-8 size-120k separator control pair (122963 ∘ 122971)` | 5 | `forward-only-prime-hit` | true | false | `91736` |
| `Gap-8 size-120k separator control pair (123083 ∘ 123091)` | 4 | `neither-prime-hit` | false | false | `91736` |
| `Gap-8 size-120k separator control pair (123113 ∘ 123121)` | 1 | `neither-prime-hit` | false | false | `91736` |
| `Gap-8 size-120k separator control pair (123209 ∘ 123217)` | 9 | `neither-prime-hit` | false | false | `91736` |
| `Gap-8 size-120k separator control pair (123419 ∘ 123427)` | 10 | `neither-prime-hit` | false | false | `91736` |
| `Gap-8 size-120k separator control pair (123449 ∘ 123457)` | 7 | `neither-prime-hit` | false | false | `91736` |

### Replication Null Atlas

This compact accounting block summarizes branch-level source/fresh/profile/separator outcomes. It is a falsification and routing surface, not a connector law or density mechanism.

- Stability status: `not-stable-under-current-three-ladder-gate`
- Rows: `50` total; class-level `19`; theorem candidates `0`
- Separator outcomes retained/split/collapsed: `0` / `11` / `16`
- Current next target: `select-new-cohort-invariant-surface-after-surface-agnostic-ensemble-picker-collapse`

| Branch | Class | Surface | Profile | Separator | Readiness | Target |
|---|---|---|---|---|---|---|
| `automorphic-repunit` | `automorphic-repunit` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `retired-automorphic-repunit-connector-family` |
| `base-mixed` | `base-mixed` | `connector-stress-meta-atlas` | `not-run` | `not-run` | `empirical-only` | `select-next-connector-surface-after-base-mixed-source-fresh-collapse` |
| `carry-chain` | `carry-chain` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `retired-carry-chain-connector-family` |
| `crt-paired` | `crt-paired` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `retired-crt-paired-connector-family` |
| `cyclic-reptend` | `cyclic-reptend` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `split-exact-residue-separator-on-prior-branch` | `empirical-only` | `retired-cyclic-reptend-connector-family` |
| `digit8-edge-classifier-family` | `digit8-edge-classifier-family` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `stop-digit8-edge-classifier-family-after-outside-ladder-collapse` |
| `edge-singleton-nonmod3-family` | `edge-singleton-nonmod3-family` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `pivot-away-from-edge-singleton-connectors` |
| `modular-walk` | `modular-walk` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `split-exact-residue-separator-on-prior-branch` | `empirical-only` | `retired-modular-walk-connector-family` |
| `multi-digit-motif-01100` | `multi-digit-motif-01100` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `pivot-to-orthogonal-pair-family-controls-after-01100-collapse` |
| `multiplicative-order` | `multiplicative-order` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `split-exact-residue-separator-on-prior-branch` | `empirical-only` | `retired-multiplicative-order-connector-family` |
| `orthogonal-adjacent-two-digit-motifs` | `orthogonal-adjacent-two-digit-motifs` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `pivot-away-from-orthogonal-adjacent-two-digit-motifs-after-repeated-three-ladder-collapse` |
| `orthogonal-compact-three-digit-motifs` | `orthogonal-compact-three-digit-motifs` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `select-next-orthogonal-nonadjacent-two-digit-motif-family-after-compact-three-digit-collapse` |
| `orthogonal-edge-plus-interior-motifs` | `orthogonal-edge-plus-interior-motifs` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `pivot-away-from-edge-plus-interior-orthogonal-motifs-after-three-ladder-collapse` |
| `orthogonal-nonadjacent-two-digit-motifs` | `orthogonal-nonadjacent-two-digit-motifs` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `split-exact-residue-separator-on-prior-branch` | `empirical-only` | `pivot-away-from-small-digit-orthogonal-motifs-after-nonadjacent-two-digit-three-ladder-collapse` |
| `orthogonal-repeated-block-motifs` | `orthogonal-repeated-block-motifs` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `pivot-to-arithmetic-connector-families-after-repeated-block-three-ladder-collapse` |
| `pair-family-cohort-width5-connector-91736` | `pair-family-cohort` | `cohort-retention-control` | `no-exact-small-prime-cohort-separator` | `not-run` | `empirical-only` | `select-new-family-level-replication-surface-after-cohort-91736-profile-no-small-prime-separator` |
| `cohort-invariant-next-picker-after-profiled-residue-null` | `pair-family-cohort-invariant-next-picker` | `gap-size-cohort-invariant-routing` | `not-residue-profiled` | `cohort-invariant-next-picker-selected-unprofiled-stable-ratio-geometry` | `empirical-only` | `residue-profile-cohort-invariant-width6-connector-276061-direction-reverse` |
| `frozen-portfolio-cohort-invariant-picker` | `pair-family-cohort-invariant-picker` | `frozen-portfolio-gap-size-ratio-picker` | `not-residue-profiled` | `frozen-portfolio-cohort-invariant-picker-selected-stable-ratio-geometry` | `empirical-only` | `residue-profile-cohort-invariant-width6-connector-003727-direction-reverse` |
| `cohort-invariant-post-two-null-picker` | `pair-family-cohort-invariant-post-two-null-picker` | `gap-size-cohort-invariant-routing` | `not-residue-profiled` | `cohort-invariant-post-two-null-picker-selected-unprofiled-stable-ratio-geometry` | `empirical-only` | `residue-profile-cohort-invariant-width5-connector-91736-direction-reverse` |
| `cohort-invariant-residue-profile-width5-connector-91736-direction-reverse` | `pair-family-cohort-invariant-residue-profile` | `gap-size-cohort-invariant-residue-profile` | `no-small-prime-cohort-invariant-residue-separator` | `no-small-prime-cohort-invariant-residue-separator` | `empirical-only` | `select-new-cohort-invariant-after-91736-residue-profile-no-coherent-separator` |
| `cohort-invariant-residue-profile-width6-connector-003727-direction-reverse` | `pair-family-cohort-invariant-residue-profile` | `gap-size-cohort-invariant-residue-profile` | `no-small-prime-cohort-invariant-residue-separator` | `no-small-prime-cohort-invariant-residue-separator` | `empirical-only` | `select-new-cohort-invariant-after-003727-residue-profile-no-coherent-separator` |
| `cohort-invariant-residue-profile-width6-connector-276061-direction-reverse` | `pair-family-cohort-invariant-residue-profile` | `gap-size-cohort-invariant-residue-profile` | `no-small-prime-cohort-invariant-residue-separator` | `no-small-prime-cohort-invariant-residue-separator` | `empirical-only` | `select-new-cohort-invariant-after-276061-residue-profile-no-coherent-separator` |
| `cohort-invariant-residue-profile-width9-connector-900020000-direction-forward` | `pair-family-cohort-invariant-residue-profile` | `gap-size-cohort-invariant-residue-profile` | `no-small-prime-cohort-invariant-residue-separator` | `no-small-prime-cohort-invariant-residue-separator` | `empirical-only` | `select-new-cohort-invariant-after-900020000-residue-profile-no-coherent-separator` |
| `cohort-invariant-forward-null-conclusion` | `pair-family-cohort-invariant-route-conclusion` | `gap-size-cohort-invariant-routing` | `forward-stable-ratio-cohort-residue-route-collapsed-under-small-prime-exact-mask-rule` | `forward-stable-ratio-cohort-residue-route-collapsed-under-small-prime-exact-mask-rule` | `empirical-only` | `select-new-cohort-invariant-surface-after-forward-route-small-prime-exact-mask-null` |
| `cohort-invariant-three-reverse-null-conclusion` | `pair-family-cohort-invariant-route-conclusion` | `gap-size-cohort-invariant-routing` | `reverse-stable-ratio-cohort-residue-route-collapsed-under-small-prime-exact-mask-rule` | `reverse-stable-ratio-cohort-residue-route-collapsed-under-small-prime-exact-mask-rule` | `empirical-only` | `residue-profile-cohort-invariant-width9-connector-900020000-direction-forward` |
| `pair-family-gap-cohort-sign-persistence-picker` | `pair-family-cohort-sign-persistence-picker` | `gap-size-sign-persistence-routing` | `not-residue-profiled` | `sign-persistence-picker-selected-low-volatility-cohort-invariant` | `empirical-only` | `stress-test-sign-persistence-cohort-invariant-width6-connector-003727-direction-reverse` |
| `pair-family-gap-cohort-sign-persistence-stress` | `pair-family-cohort-sign-persistence-stress` | `fresh-gap-size-sign-persistence-stress` | `not-residue-profiled` | `sign-persistence-split-on-fresh-surfaces` | `empirical-only` | `select-new-cohort-invariant-surface-after-sign-persistence-fresh-split` |
| `pair-family-gap-cohort-surface-agnostic-ensemble-picker` | `pair-family-cohort-surface-agnostic-ensemble-picker` | `mixed-gap-size-surface-agnostic-ensemble-picker` | `not-residue-profiled` | `surface-agnostic-ensemble-picker-found-no-stable-mixed-surface-cohort` | `empirical-only` | `select-new-cohort-invariant-surface-after-surface-agnostic-ensemble-picker-collapse` |
| `pair-family-gap-cohort-surface-family-contrast-anatomy` | `pair-family-cohort-surface-family-contrast-anatomy` | `gap-vs-size-surface-family-contrast-driver-anatomy` | `not-residue-profiled` | `distributed-full-driver-cohort` | `empirical-only` | `stress-test-surface-family-contrast-driver-cohort-distributed-gap-family-forward-vs-size-family-opposite` |
| `pair-family-gap-cohort-surface-family-contrast-picker` | `pair-family-cohort-surface-family-contrast-picker` | `gap-vs-size-surface-family-contrast-routing` | `not-residue-profiled` | `surface-family-directional-contrast-found` | `empirical-only` | `stress-test-surface-family-contrast-gap-family-forward-vs-size-family-opposite` |
| `pair-family-gap-cohort-surface-family-contrast-stress` | `pair-family-cohort-surface-family-contrast-stress` | `fresh-gap-vs-size-surface-family-contrast-stress` | `not-residue-profiled` | `surface-family-contrast-retained-on-fresh-surfaces` | `empirical-only` | `analyze-surface-family-contrast-anatomy-gap-family-forward-vs-size-family-opposite` |
| `pair-family-gap-cohort-surface-family-driver-cohort-stress` | `pair-family-cohort-surface-family-driver-cohort-stress` | `fresh-gap-vs-size-driver-cohort-stress` | `not-residue-profiled` | `driver-cohort-contrast-split-on-fresh-surfaces` | `empirical-only` | `select-new-cohort-invariant-surface-after-driver-cohort-contrast-fresh-split` |
| `pair-family-gap-cohort-surface-family-matched-nondriver-control-stress` | `pair-family-cohort-surface-family-matched-nondriver-control-stress` | `fresh-gap-vs-size-matched-nondriver-control-stress` | `not-residue-profiled` | `matched-nondriver-control-split-on-fresh-surfaces` | `empirical-only` | `select-new-cohort-invariant-surface-after-driver-and-nondriver-fresh-split` |
| `pair-family-gap-cohort-volatility-ensemble-picker` | `pair-family-cohort-volatility-ensemble-picker` | `gap-size-volatility-ensemble-routing` | `not-residue-profiled` | `volatility-ensemble-picker-selected-shared-direction-cohort` | `empirical-only` | `stress-test-volatility-ensemble-cohort-direction-forward-connector-count-14` |
| `pair-family-gap-cohort-volatility-ensemble-stress` | `pair-family-cohort-volatility-ensemble-stress` | `fresh-gap-size-volatility-ensemble-stress` | `not-residue-profiled` | `volatility-ensemble-split-on-fresh-surfaces` | `empirical-only` | `select-new-cohort-invariant-surface-after-volatility-ensemble-fresh-split` |
| `pair-family-gap-cohort-window-consensus-stress` | `pair-family-cohort-window-consensus-stress` | `heldout-gap-size-window-consensus-stress` | `not-residue-profiled` | `window-consensus-collapsed-on-heldout-gap-size-surfaces` | `empirical-only` | `select-new-cohort-invariant-surface-after-window-consensus-heldout-collapse` |
| `pair-family-gap-cohort-window-consensus-surface` | `pair-family-cohort-window-consensus-surface` | `gap-size-window-consensus-routing` | `not-residue-profiled` | `window-consensus-surface-selected-shared-stage-complete-invariant` | `empirical-only` | `stress-test-window-consensus-cohort-invariant-width6-connector-003727-direction-reverse` |
| `pair-family-gap-extension-gap16-width6-connector-276061` | `pair-family-gap-extension` | `pair-family-gap-extension-control` | `exact-residue-separator-found` | `split-exact-residue-separator-on-orthogonal-separator-ladder` | `empirical-only` | `select-new-pair-family-side-control-after-gap-extension-three-ladder-collapse` |
| `pair-family-gap-portfolio` | `pair-family-gap-portfolio` | `connector-stress-meta-atlas` | `not-run` | `not-run` | `none` | `scan-pair-family-gap-portfolio-over-retired-connector-heads` |
| `pair-family-gap-portfolio-gap8-width6-connector-003727` | `pair-family-gap-portfolio` | `pair-family-gap-portfolio-control` | `exact-residue-separator-found` | `split-exact-residue-separator-on-orthogonal-separator-ladder` | `empirical-only` | `select-new-pair-family-side-control-after-gap-portfolio-three-ladder-collapse` |
| `pair-family-gap-cohort-geometry-gap20-22-24` | `pair-family-geometry-cohort` | `gap-cohort-geometry-control` | `no-exact-small-prime-gap-cohort-separator` | `not-run` | `empirical-only` | `select-new-family-level-replication-surface-after-gap-cohort-0286717-profile-no-small-prime-separator` |
| `pair-family-gap-cohort-ratio-geometry-atlas-001139-size-gap` | `pair-family-ratio-geometry-atlas` | `gap-size-ratio-geometry-atlas` | `balanced-split-correction-bound-geometry-across-default-bounds` | `ratio-geometry-001139-not-stable-across-size-and-gap-bands` | `empirical-only` | `select-new-cohort-invariant-after-ratio-geometry-001139-size-gap-atlas-collapse` |
| `pair-family-gap-cohort-ratio-geometry-gap20-22-24` | `pair-family-ratio-geometry-cohort` | `gap-cohort-ratio-geometry-control` | `not-a-residue-profile` | `retained-source-ratio-geometry-on-gap-cohort-separator-ladders` | `empirical-only` | `expand-gap-cohort-ratio-geometry-width6-connector-001139-bias-reverse-on-new-gap-ladders` |
| `pair-family-gap-cohort-ratio-geometry-expansion-gap26-28-30` | `pair-family-ratio-geometry-expansion` | `gap-cohort-ratio-geometry-expansion` | `stable-reverse-correction-bound-geometry-across-default-bounds` | `not-a-residue-separator` | `empirical-only` | `expand-ratio-geometry-001139-to-size-band-and-gap-band-controls` |
| `pair-family-size-band-prime-gap8-size120k-gap8-width5-connector-91736` | `pair-family-size-band` | `pair-family-size-band-control` | `exact-residue-separator-found` | `collapsed-overlapping-residue-classes-on-orthogonal-separator-ladder` | `empirical-only` | `select-new-pair-family-side-control-after-size-band-three-ladder-collapse` |
| `pair-family-size-band` | `pair-family-surface` | `family-level-surface-control` | `no-exact-small-prime-family-surface-separator` | `not-run` | `empirical-only` | `select-new-family-level-replication-surface-after-pair-family-size-band-profile-no-small-prime-separator` |
| `topn-pair-family-size-band` | `pair-family-topn-surface` | `topn-family-level-surface-control` | `no-exact-small-prime-topn-family-surface-separator` | `not-run` | `empirical-only` | `select-new-family-level-replication-surface-after-topn-pair-family-size-band-profile-no-small-prime-separator` |
| `residue-lattice` | `residue-lattice` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `split-exact-residue-separator-on-prior-branch` | `empirical-only` | `retired-residue-lattice-connector-family` |
| `single-digit-interior-nonmod3-family` | `single-digit-interior-nonmod3-family` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `pivot-away-from-single-digit-interior-connectors` |
| `square-triangular` | `square-triangular` | `connector-stress-meta-atlas` | `prior-exact-residue-separator-found` | `collapsed-exact-residue-separator-on-prior-branch` | `empirical-only` | `retired-square-triangular-connector-family` |

### Pair-Family Cohort Retention Picker

This block uses the replication null atlas to stop chasing single branches. It groups related pair-family branches by connector identity and requires cohort-level fresh-ladder retention before any residue-profile or Lean theorem candidate is surfaced.

- Picker decision: `single-branch-separators-not-stable-select-cohort-level-pair-family-side-control`
- Source surfaces: `3`; candidate cohorts `6`; cohort-ready `1`
- Selected cohort: `pair-family-cohort-width5-connector-91736` connector `91736` width `5` fresh survivors `2` target `cohort-residue-profile-width5-connector-91736-across-related-pair-family-ladders`

| Cohort | Width | Connector | Families | Source selected | Fresh survivors | Fresh reverse-only | Separator retained/split/collapsed | Status | Target |
|---|---:|---|---|---:|---:|---:|---|---|---|
| `pair-family-cohort-width5-connector-91736` | 5 | `91736` | `["prime-gap14", "prime-gap8-size120k", "prime-gap8-size40k"]` | 3 | 2 | 2 | `0/0/1` | `fresh-cohort-survived-needs-cohort-residue-profile` | `cohort-residue-profile-width5-connector-91736-across-related-pair-family-ladders` |
| `pair-family-cohort-width6-connector-003727` | 6 | `003727` | `["prime-gap18", "prime-gap8"]` | 2 | 1 | 3 | `0/1/0` | `single-branch-survived-not-cohort-ready` | `do-not-profile-single-branch-width6-connector-003727-without-cohort-retention` |
| `pair-family-cohort-width6-connector-276061` | 6 | `276061` | `["prime-gap16"]` | 1 | 1 | 2 | `0/1/0` | `single-branch-survived-not-cohort-ready` | `do-not-profile-single-branch-width6-connector-276061-without-cohort-retention` |
| `pair-family-cohort-width7-connector-0050020` | 7 | `0050020` | `["prime-gap8-size80k"]` | 1 | 1 | 1 | `0/0/0` | `single-branch-survived-not-cohort-ready` | `do-not-profile-single-branch-width7-connector-0050020-without-cohort-retention` |
| `pair-family-cohort-width7-connector-0286717` | 7 | `0286717` | `["prime-gap10"]` | 1 | 1 | 1 | `0/0/0` | `single-branch-survived-not-cohort-ready` | `do-not-profile-single-branch-width7-connector-0286717-without-cohort-retention` |
| `pair-family-cohort-width5-connector-02510` | 5 | `02510` | `["prime-gap12"]` | 1 | 0 | 0 | `0/0/0` | `cohort-collapsed-on-fresh-ladders` | `do-not-profile-single-branch-width5-connector-02510-without-cohort-retention` |

### Pair-Family Cohort Residue Profile

This block profiles the selected cohort as a cohort, not as another single branch. A Lean candidate is surfaced only if a small-prime residue separator is found at the cohort level and then retained on the related separator ladders; otherwise the result remains empirical routing evidence.

- Cohort: `pair-family-cohort-width5-connector-91736` connector `91736` width `5` families `["prime-gap14", "prime-gap8-size120k", "prime-gap8-size40k"]`
- Fresh rows `36`; reverse-only `2`; comparison `34`; fresh-surviving branches `2`
- Exact separator count `0`; decision `no-small-prime-cohort-residue-separator-found`; target `select-new-family-level-replication-surface-after-cohort-91736-profile-no-small-prime-separator`

| Modulus | Reverse-only residues | Shared residues | Status |
|---:|---|---|---|
| 3 | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[1, 3]` | `[1, 3]` | `overlapping-residue-classes` |
| 7 | `[3, 4]` | `[3, 4]` | `overlapping-residue-classes` |
| 11 | `[6, 10]` | `[6, 10]` | `overlapping-residue-classes` |
| 13 | `[2, 12]` | `[2, 12]` | `overlapping-residue-classes` |
| 17 | `[3, 7]` | `[3, 7]` | `overlapping-residue-classes` |
| 19 | `[18]` | `[18]` | `overlapping-residue-classes` |
| 23 | `[4, 14]` | `[4, 14]` | `overlapping-residue-classes` |
| 29 | `[13, 27]` | `[13, 27]` | `overlapping-residue-classes` |
| 31 | `[3, 27]` | `[3]` | `overlapping-residue-classes` |

### Pair-Family Surface Picker

This block pivots after the connector-identity cohort fails to produce a small-prime separator. It ranks whole pair-family control surfaces, requiring multiple fresh-surviving selected branches before a surface-level residue profile is considered.

- Picker decision: `cohort-connector-profile-failed-select-family-level-pair-surface`
- Source surfaces: `3`; candidate surfaces `3`; surface-ready `2`
- Selected surface: `pair-family-size-band` label `gap-8 size-band controls` fresh survivors `3` target `surface-residue-profile-pair-family-size-band-across-selected-branches`

| Surface | Families | Source selected | Fresh survivors | Fresh reverse-only | Status | Target |
|---|---|---:|---:|---:|---|---|
| `pair-family-size-band` | `["prime-gap8-size120k", "prime-gap8-size40k", "prime-gap8-size80k"]` | 3 | 3 | 3 | `fresh-family-surface-survived-needs-surface-residue-profile` | `surface-residue-profile-pair-family-size-band-across-selected-branches` |
| `pair-family-gap-portfolio` | `["prime-gap10", "prime-gap12", "prime-gap8"]` | 3 | 2 | 4 | `fresh-family-surface-survived-needs-surface-residue-profile` | `surface-residue-profile-pair-family-gap-portfolio-across-selected-branches` |
| `pair-family-gap-extension` | `["prime-gap14", "prime-gap16", "prime-gap18"]` | 3 | 1 | 2 | `single-branch-survived-not-family-surface-ready` | `do-not-profile-pair-family-gap-extension-without-family-level-retention` |

### Pair-Family Surface Residue Profile

This block profiles the selected family surface as a surface, not as a connector-specific cohort. No Lean candidate is proposed unless an exact small-prime separator appears at this level.

- Surface: `pair-family-size-band` label `gap-8 size-band controls` connectors `["0050020", "91736"]` families `["prime-gap8-size120k", "prime-gap8-size40k", "prime-gap8-size80k"]`
- Fresh rows `36`; reverse-only `3`; comparison `33`; fresh-surviving branches `3`
- Exact separator count `0`; decision `no-small-prime-family-surface-residue-separator-found`; target `select-new-family-level-replication-surface-after-pair-family-size-band-profile-no-small-prime-separator`

| Modulus | Reverse-only residues | Shared residues | Status |
|---:|---|---|---|
| 3 | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[1, 3]` | `[1, 3]` | `overlapping-residue-classes` |
| 7 | `[3, 4, 5]` | `[3, 4, 5]` | `overlapping-residue-classes` |
| 11 | `[4, 6, 10]` | `[4, 6, 10]` | `overlapping-residue-classes` |
| 13 | `[1, 2, 12]` | `[1, 2, 12]` | `overlapping-residue-classes` |
| 17 | `[3, 6, 7]` | `[3, 6, 7]` | `overlapping-residue-classes` |
| 19 | `[8, 18]` | `[8, 18]` | `overlapping-residue-classes` |
| 23 | `[4, 13, 14]` | `[4, 14]` | `overlapping-residue-classes` |
| 29 | `[13, 19, 27]` | `[13, 19, 27]` | `overlapping-residue-classes` |
| 31 | `[3, 5, 27]` | `[3, 5, 27]` | `overlapping-residue-classes` |

### Pair-Family Top-N Motif Surface Profile

This block widens the family-level test from one selected branch per pair family to the top-N source motifs per pair family. It is still a guardrail surface: theorem work waits for exact residue structure, not sparse prime-hit excitement.

- Source surface: `pair-family-size-band` label `gap-8 size-band controls`; top-N `3`; pair families `["prime-gap8-size120k", "prime-gap8-size40k", "prime-gap8-size80k"]`
- Motifs `9`; fresh-surviving motifs `8`; fresh rows `108`; reverse-only `13`; comparison `95`
- Exact separator count `0`; decision `no-small-prime-topn-family-surface-residue-separator-found`; target `select-new-family-level-replication-surface-after-topn-pair-family-size-band-profile-no-small-prime-separator`

| Family | Rank | Width | Connector | Source reverse-only | Fresh reverse-only | Fresh forward-only | Status |
|---|---:|---:|---|---:|---:|---:|---|
| `prime-gap8-size120k` | 1 | 5 | `91736` | 4 | 1 | 2 | `fresh-reverse-only-survived` |
| `prime-gap8-size120k` | 2 | 5 | `53191` | 3 | 1 | 1 | `fresh-reverse-only-survived` |
| `prime-gap8-size120k` | 3 | 9 | `900020000` | 3 | 3 | 1 | `fresh-reverse-only-survived` |
| `prime-gap8-size40k` | 1 | 5 | `91736` | 3 | 1 | 1 | `fresh-reverse-only-survived` |
| `prime-gap8-size40k` | 2 | 6 | `003727` | 3 | 3 | 0 | `fresh-reverse-only-survived` |
| `prime-gap8-size40k` | 3 | 5 | `04700` | 2 | 2 | 0 | `fresh-reverse-only-survived` |
| `prime-gap8-size80k` | 1 | 7 | `0050020` | 4 | 1 | 0 | `fresh-reverse-only-survived` |
| `prime-gap8-size80k` | 2 | 5 | `53191` | 3 | 0 | 2 | `fresh-reverse-only-collapsed` |
| `prime-gap8-size80k` | 3 | 5 | `04700` | 1 | 1 | 0 | `fresh-reverse-only-survived` |

| Modulus | Reverse-only residues | Shared residues | Status |
|---:|---|---|---|
| 3 | `[2]` | `[2]` | `overlapping-residue-classes` |
| 5 | `[1, 3, 4]` | `[1, 3, 4]` | `overlapping-residue-classes` |
| 7 | `[1, 2, 3, 4, 5]` | `[1, 2, 3, 4, 5]` | `overlapping-residue-classes` |
| 11 | `[1, 2, 4, 5, 6, 8, 9, 10]` | `[1, 2, 4, 5, 6, 8, 9, 10]` | `overlapping-residue-classes` |
| 13 | `[1, 2, 3, 4, 6, 7, 8, 9, 10, 12]` | `[1, 2, 3, 4, 6, 7, 8, 9, 10, 12]` | `overlapping-residue-classes` |
| 17 | `[2, 3, 6, 7, 11, 12, 15]` | `[2, 3, 6, 7, 11, 12, 15]` | `overlapping-residue-classes` |
| 19 | `[1, 3, 5, 6, 7, 8, 15, 18]` | `[1, 3, 5, 6, 7, 8, 15, 18]` | `overlapping-residue-classes` |
| 23 | `[1, 4, 6, 7, 10, 13, 14, 20]` | `[1, 4, 6, 7, 10, 13, 14, 20]` | `overlapping-residue-classes` |
| 29 | `[5, 12, 13, 18, 19, 25, 27]` | `[5, 12, 13, 18, 19, 25, 27]` | `overlapping-residue-classes` |
| 31 | `[3, 4, 5, 13, 15, 19, 22, 24, 27]` | `[3, 4, 5, 13, 15, 19, 22, 24, 27]` | `overlapping-residue-classes` |

### Pair-Family Gap Cohort Geometry Control

This block moves away from single selected branches and tests whether shared source/fresh hit geometry persists across new disjoint gap-20, gap-22, and gap-24 pair-family ladders. It is empirical routing only: residue profiling and Lean theorem work wait for retained cohort geometry.

- Selection rule: `after-topn-family-surface-profile-finds-no-small-prime-separator;scan-top-10-frozen-connector motifs across new gap20-gap22-gap24 pair-family ladders and require shared fresh hit geometry before residue profiling`
- Pair families: `["prime-gap20", "prime-gap22", "prime-gap24"]`; top-N `10`; source motifs `21`; fresh-surviving motifs `11`
- Geometry rows `12`; retained geometry rows `2`; decision `pair-family-gap-cohort-geometry-retained-residue-profiler-next`; target `residue-profile-gap-cohort-width7-connector-0286717-across-gap20-22-24`

- Selected geometry: width `7` connector `0286717` fresh survivor families `2` fresh reverse-only `5` target `residue-profile-gap-cohort-width7-connector-0286717-across-gap20-22-24`

| Width | Connector | Source families | Fresh survivor families | Fresh reverse-only | Fresh forward-only | Status | Target |
|---:|---|---:|---:|---:|---:|---|---|
| 7 | `0286717` | 2 | 2 | 5 | 1 | `geometry-retained-across-gap-families` | `residue-profile-gap-cohort-width7-connector-0286717-across-gap20-22-24` |
| 6 | `001139` | 3 | 2 | 4 | 0 | `geometry-retained-across-gap-families` | `residue-profile-gap-cohort-width6-connector-001139-across-gap20-22-24` |
| 9 | `000550550` | 1 | 1 | 3 | 1 | `single-family-or-fresh-collapsed` | `do-not-profile-gap-cohort-width9-connector-000550550-without-shared-fresh-retention` |
| 5 | `04700` | 2 | 1 | 2 | 0 | `single-family-or-fresh-collapsed` | `do-not-profile-gap-cohort-width5-connector-04700-without-shared-fresh-retention` |
| 6 | `003727` | 3 | 1 | 1 | 0 | `single-family-or-fresh-collapsed` | `do-not-profile-gap-cohort-width6-connector-003727-without-shared-fresh-retention` |
| 5 | `02510` | 1 | 1 | 1 | 0 | `single-family-or-fresh-collapsed` | `do-not-profile-gap-cohort-width5-connector-02510-without-shared-fresh-retention` |
| 5 | `91736` | 1 | 1 | 1 | 0 | `single-family-or-fresh-collapsed` | `do-not-profile-gap-cohort-width5-connector-91736-without-shared-fresh-retention` |
| 7 | `0050020` | 1 | 1 | 1 | 0 | `single-family-or-fresh-collapsed` | `do-not-profile-gap-cohort-width7-connector-0050020-without-shared-fresh-retention` |
| 6 | `000122` | 1 | 1 | 1 | 1 | `single-family-or-fresh-collapsed` | `do-not-profile-gap-cohort-width6-connector-000122-without-shared-fresh-retention` |
| 5 | `04900` | 2 | 0 | 0 | 1 | `single-family-or-fresh-collapsed` | `do-not-profile-gap-cohort-width5-connector-04900-without-shared-fresh-retention` |
| 5 | `53191` | 2 | 0 | 0 | 1 | `single-family-or-fresh-collapsed` | `do-not-profile-gap-cohort-width5-connector-53191-without-shared-fresh-retention` |
| 6 | `276061` | 2 | 0 | 0 | 1 | `single-family-or-fresh-collapsed` | `do-not-profile-gap-cohort-width6-connector-276061-without-shared-fresh-retention` |

### Pair-Family Gap Cohort Residue Profile

This block profiles the selected retained gap-cohort geometry across small prime residues. It is still a guardrail layer: an exact separator must survive the separator ladders before any Lean classifier is proposed.

- Cohort: `pair-family-gap-cohort-width7-connector-0286717` width `7` connector `0286717` families `["prime-gap20", "prime-gap22", "prime-gap24"]`
- Fresh rows `36`; reverse-only `5`; comparison `31`; fresh-surviving branches `2`
- Exact separator count `0`; decision `no-small-prime-gap-cohort-residue-separator-found`; target `select-new-family-level-replication-surface-after-gap-cohort-0286717-profile-no-small-prime-separator`

| Modulus | Reverse-only residues | Shared residues | Status |
|---:|---|---|---|
| 3 | `[1, 2]` | `[1, 2]` | `overlapping-residue-classes` |
| 5 | `[2, 3, 4]` | `[2, 3, 4]` | `overlapping-residue-classes` |
| 7 | `[2, 3, 6]` | `[2, 3, 6]` | `overlapping-residue-classes` |
| 11 | `[1, 5, 6, 8, 10]` | `[1, 5, 6, 8, 10]` | `overlapping-residue-classes` |
| 13 | `[2, 7, 9, 10, 12]` | `[7, 10, 12]` | `overlapping-residue-classes` |
| 17 | `[3, 6, 9, 11]` | `[3, 9, 11]` | `overlapping-residue-classes` |
| 19 | `[4, 6, 9, 12, 16]` | `[4, 6, 9, 12]` | `overlapping-residue-classes` |
| 23 | `[2, 7, 11, 15, 16]` | `[2, 7, 11, 15, 16]` | `overlapping-residue-classes` |
| 29 | `[4, 10, 14, 16]` | `[4, 10, 16]` | `overlapping-residue-classes` |
| 31 | `[4, 17, 19, 22, 27]` | `[4, 17, 19, 22, 27]` | `overlapping-residue-classes` |

### Pair-Family Gap Cohort Ratio Geometry Control

This block pivots from small-prime residue separation to shared forward/reverse hit-ratio geometry across gap-20, gap-22, and gap-24 families. It is empirical routing only, not an exact residue theorem or connector law.

- Source profile: `pair-family-gap-cohort-width7-connector-0286717`; decision `pair-family-gap-cohort-ratio-geometry-retained-replicate-next`; target `replicate-gap-cohort-ratio-geometry-width6-connector-001139-bias-reverse-on-separator-ladders`
- Family rows `21`; geometry rows `12`; retained geometry rows `1`

- Selected ratio geometry: width `6` connector `001139` bias `reverse` retained families `2` signed delta `4` target `replicate-gap-cohort-ratio-geometry-width6-connector-001139-bias-reverse-on-separator-ladders`

| Width | Connector | Reverse-biased families | Forward-biased families | Neutral families | Bias | Signed delta | Status | Target |
|---:|---|---:|---:|---:|---|---:|---|---|
| 6 | `001139` | 2 | 0 | 1 | `reverse` | 4 | `ratio-geometry-retained-across-gap-families` | `replicate-gap-cohort-ratio-geometry-width6-connector-001139-bias-reverse-on-separator-ladders` |
| 7 | `0286717` | 1 | 0 | 1 | `reverse` | 4 | `ratio-geometry-not-retained-across-gap-families` | `do-not-replicate-gap-cohort-ratio-geometry-width7-connector-0286717-without-shared-direction` |
| 5 | `04700` | 1 | 0 | 1 | `reverse` | 2 | `ratio-geometry-not-retained-across-gap-families` | `do-not-replicate-gap-cohort-ratio-geometry-width5-connector-04700-without-shared-direction` |
| 9 | `000550550` | 1 | 0 | 0 | `reverse` | 2 | `ratio-geometry-not-retained-across-gap-families` | `do-not-replicate-gap-cohort-ratio-geometry-width9-connector-000550550-without-shared-direction` |
| 6 | `003727` | 1 | 0 | 2 | `reverse` | 1 | `ratio-geometry-not-retained-across-gap-families` | `do-not-replicate-gap-cohort-ratio-geometry-width6-connector-003727-without-shared-direction` |
| 5 | `04900` | 0 | 1 | 1 | `forward` | -1 | `ratio-geometry-not-retained-across-gap-families` | `do-not-replicate-gap-cohort-ratio-geometry-width5-connector-04900-without-shared-direction` |
| 5 | `53191` | 0 | 1 | 1 | `forward` | -1 | `ratio-geometry-not-retained-across-gap-families` | `do-not-replicate-gap-cohort-ratio-geometry-width5-connector-53191-without-shared-direction` |
| 6 | `276061` | 0 | 1 | 1 | `forward` | -1 | `ratio-geometry-not-retained-across-gap-families` | `do-not-replicate-gap-cohort-ratio-geometry-width6-connector-276061-without-shared-direction` |
| 5 | `02510` | 1 | 0 | 0 | `reverse` | 1 | `ratio-geometry-not-retained-across-gap-families` | `do-not-replicate-gap-cohort-ratio-geometry-width5-connector-02510-without-shared-direction` |
| 5 | `91736` | 1 | 0 | 0 | `reverse` | 1 | `ratio-geometry-not-retained-across-gap-families` | `do-not-replicate-gap-cohort-ratio-geometry-width5-connector-91736-without-shared-direction` |
| 7 | `0050020` | 1 | 0 | 0 | `reverse` | 1 | `ratio-geometry-not-retained-across-gap-families` | `do-not-replicate-gap-cohort-ratio-geometry-width7-connector-0050020-without-shared-direction` |
| 6 | `000122` | 0 | 0 | 1 | `reverse` | 0 | `ratio-geometry-not-retained-across-gap-families` | `do-not-replicate-gap-cohort-ratio-geometry-width6-connector-000122-without-shared-direction` |

### Pair-Family Gap Cohort Ratio Geometry Replication

This block tests the selected ratio geometry on the disjoint separator ladders. Retention keeps it as an empirical expansion target; it still does not create Lean theorem material.

- Source: width `6` connector `001139` bias `reverse` retained source families `2`
- Separator rows `36`; retained direction families `2`; split direction families `0`; status `retained-source-ratio-geometry-on-gap-cohort-separator-ladders`
- Decision `gap-cohort-ratio-geometry-retained-expand-empirical-surface`; target `expand-gap-cohort-ratio-geometry-width6-connector-001139-bias-reverse-on-new-gap-ladders`

| Family | Bias | Reverse-only | Forward-only | Both | Neither | Signed delta |
|---|---|---:|---:|---:|---:|---:|
| `prime-gap20` | `reverse` | 1 | 0 | 0 | 11 | 1 |
| `prime-gap22` | `neutral` | 1 | 1 | 0 | 10 | 0 |
| `prime-gap24` | `reverse` | 2 | 0 | 0 | 10 | 2 |

### Pair-Family Gap Cohort Ratio Geometry Expansion

This block expands the retained `001139` ratio geometry to generated gap-26, gap-28, and gap-30 ladders. Mixed retention is allowed to proceed to correction-bound stability, but it remains empirical routing rather than theorem-candidate material.

- Source: width `6` connector `001139` bias `reverse`
- Generated pairs `108` across `3` families and `9` windows; retained windows `5`; split windows `2`; status `mixed-retained-ratio-geometry-on-new-gap-ladders`
- Decision `gap-cohort-ratio-geometry-mixed-retained-test-correction-bounds-next`; target `test-correction-bound-stability-gap-cohort-ratio-geometry-width6-connector-001139`

| Family | Stage | Bias | Reverse-only | Forward-only | Both | Neither | Signed delta |
|---|---|---|---:|---:|---:|---:|---:|
| `prime-gap26` | `source` | `forward` | 0 | 1 | 1 | 10 | -1 |
| `prime-gap26` | `fresh` | `reverse` | 2 | 1 | 0 | 9 | 1 |
| `prime-gap26` | `separator` | `neutral` | 1 | 1 | 0 | 10 | 0 |
| `prime-gap28` | `source` | `forward` | 0 | 1 | 1 | 10 | -1 |
| `prime-gap28` | `fresh` | `reverse` | 2 | 1 | 0 | 9 | 1 |
| `prime-gap28` | `separator` | `reverse` | 2 | 1 | 0 | 9 | 1 |
| `prime-gap30` | `source` | `reverse` | 2 | 0 | 0 | 10 | 2 |
| `prime-gap30` | `fresh` | `neutral` | 0 | 0 | 0 | 12 | 0 |
| `prime-gap30` | `separator` | `reverse` | 1 | 0 | 0 | 11 | 1 |

### Pair-Family Gap Cohort Ratio Correction-Bound Stability

This block normalizes the expanded `001139` hit-ratio geometry by small-prime correction bounds. Stable reverse geometry is a next empirical surface, not a connector law or density mechanism.

- Bounds `[5, 7, 11, 13, 17, 19, 23, 29, 31]`; stable bounds `9` of `9`; status `stable-reverse-correction-bound-geometry-across-default-bounds`
- Decision `correction-bound-stability-retained-expand-ratio-geometry-atlas-next`; target `expand-ratio-geometry-001139-to-size-band-and-gap-band-controls`

| Bound | Reverse-positive families | Forward-positive families | Neutral families | Signed corrected-ratio gap | Status |
|---:|---:|---:|---:|---:|---|
| 5 | 3 | 0 | 0 | 42.791543 | `reverse-positive-correction-bound` |
| 7 | 3 | 0 | 0 | 36.678465 | `reverse-positive-correction-bound` |
| 11 | 3 | 0 | 0 | 33.344059 | `reverse-positive-correction-bound` |
| 13 | 3 | 0 | 0 | 30.779132 | `reverse-positive-correction-bound` |
| 17 | 3 | 0 | 0 | 28.968594 | `reverse-positive-correction-bound` |
| 19 | 3 | 0 | 0 | 27.443932 | `reverse-positive-correction-bound` |
| 23 | 3 | 0 | 0 | 26.250717 | `reverse-positive-correction-bound` |
| 29 | 3 | 0 | 0 | 25.345520 | `reverse-positive-correction-bound` |
| 31 | 3 | 0 | 0 | 24.527923 | `reverse-positive-correction-bound` |

### Pair-Family Gap Cohort Ratio Geometry Atlas

This atlas projects the fixed width-6 connector `001139` across gap-band and size-band controls. It is a high-quality falsification surface: local ratio geometry remains interesting, but the strong correction-stable family-level invariant does not survive both bands. This is not a density mechanism, connector law, or Lean theorem candidate.

- Source: width `6` connector `001139` bias `reverse`
- Status `ratio-geometry-001139-not-stable-across-size-and-gap-bands`; decision `ratio-geometry-001139-collapsed-as-family-level-invariant-record-falsification`; target `select-new-cohort-invariant-after-ratio-geometry-001139-size-gap-atlas-collapse`

| Surface | Families | Windows | Reverse windows | Forward windows | Neutral windows | Correction status | Stable bounds | Balanced bounds | Concentrated bounds |
|---|---|---:|---:|---:|---:|---|---:|---:|---:|
| `gap-band` | `prime-gap14, prime-gap16, prime-gap18` | 9 | 3 | 2 | 4 | `balanced-split-correction-bound-geometry-across-default-bounds` | 0 | 9 | 0 |
| `size-band` | `prime-gap8-size40k, prime-gap8-size80k, prime-gap8-size120k` | 9 | 3 | 5 | 1 | `aggregate-positive-but-family-concentrated-across-default-bounds` | 0 | 0 | 9 |

| Surface | Bound | Reverse-positive families | Forward-positive families | Neutral families | Signed corrected-ratio gap | Status |
|---|---:|---:|---:|---:|---:|---|
| `gap-band` | 5 | 2 | 1 | 0 | -0.000960 | `balanced-split-correction-bound` |
| `gap-band` | 7 | 2 | 1 | 0 | -0.000823 | `balanced-split-correction-bound` |
| `gap-band` | 11 | 2 | 1 | 0 | -0.000748 | `balanced-split-correction-bound` |
| `gap-band` | 13 | 2 | 1 | 0 | -0.000690 | `balanced-split-correction-bound` |
| `gap-band` | 17 | 2 | 1 | 0 | -0.000650 | `balanced-split-correction-bound` |
| `gap-band` | 19 | 2 | 1 | 0 | -0.000616 | `balanced-split-correction-bound` |
| `gap-band` | 23 | 2 | 1 | 0 | -0.000589 | `balanced-split-correction-bound` |
| `gap-band` | 29 | 2 | 1 | 0 | -0.000569 | `balanced-split-correction-bound` |
| `gap-band` | 31 | 2 | 1 | 0 | -0.000550 | `balanced-split-correction-bound` |
| `size-band` | 5 | 1 | 2 | 0 | 0.919424 | `aggregate-positive-family-concentrated-correction-bound` |
| `size-band` | 7 | 1 | 2 | 0 | 0.788077 | `aggregate-positive-family-concentrated-correction-bound` |
| `size-band` | 11 | 1 | 2 | 0 | 0.716434 | `aggregate-positive-family-concentrated-correction-bound` |
| `size-band` | 13 | 1 | 2 | 0 | 0.661324 | `aggregate-positive-family-concentrated-correction-bound` |
| `size-band` | 17 | 1 | 2 | 0 | 0.622422 | `aggregate-positive-family-concentrated-correction-bound` |
| `size-band` | 19 | 1 | 2 | 0 | 0.589663 | `aggregate-positive-family-concentrated-correction-bound` |
| `size-band` | 23 | 1 | 2 | 0 | 0.564026 | `aggregate-positive-family-concentrated-correction-bound` |
| `size-band` | 29 | 1 | 2 | 0 | 0.544577 | `aggregate-positive-family-concentrated-correction-bound` |
| `size-band` | 31 | 1 | 2 | 0 | 0.527010 | `aggregate-positive-family-concentrated-correction-bound` |

### Frozen Portfolio Cohort-Invariant Picker

This picker scans the frozen retired-head connector portfolio for direction-agnostic correction-stable family-level ratio geometry across the same gap-band and size-band controls. It is empirical routing only: selected rows go to residue profiling, not directly to Lean theorem work or density-mechanism language.

- Candidates `14`; stable candidates `6`; decision `frozen-portfolio-cohort-invariant-picker-selected-stable-ratio-geometry`; target `residue-profile-cohort-invariant-width6-connector-003727-direction-reverse`

- Selected invariant candidate: width `6` connector `003727` direction `reverse` stable bounds `18` target `residue-profile-cohort-invariant-width6-connector-003727-direction-reverse`

| Rank | Width | Connector | Status | Direction | Stable surfaces | Stable bounds | Gap-band | Size-band | Target |
|---:|---:|---|---|---|---:|---:|---|---|---|
| 1 | 6 | `003727` | `stable-family-level-ratio-geometry` | `reverse` | 2 | 18 | `stable-reverse-correction-bound-geometry-across-default-bounds` | `stable-reverse-correction-bound-geometry-across-default-bounds` | `residue-profile-cohort-invariant-width6-connector-003727-direction-reverse` |
| 2 | 6 | `276061` | `stable-family-level-ratio-geometry` | `reverse` | 2 | 18 | `stable-reverse-correction-bound-geometry-across-default-bounds` | `stable-reverse-correction-bound-geometry-across-default-bounds` | `residue-profile-cohort-invariant-width6-connector-276061-direction-reverse` |
| 3 | 5 | `91736` | `stable-family-level-ratio-geometry` | `reverse` | 2 | 18 | `stable-reverse-correction-bound-geometry-across-default-bounds` | `stable-reverse-correction-bound-geometry-across-default-bounds` | `residue-profile-cohort-invariant-width5-connector-91736-direction-reverse` |
| 4 | 9 | `900020000` | `stable-family-level-ratio-geometry` | `forward` | 2 | 18 | `stable-forward-correction-bound-geometry-across-default-bounds` | `stable-forward-correction-bound-geometry-across-default-bounds` | `residue-profile-cohort-invariant-width9-connector-900020000-direction-forward` |
| 5 | 5 | `00022` | `stable-family-level-ratio-geometry` | `forward` | 2 | 18 | `stable-forward-correction-bound-geometry-across-default-bounds` | `stable-forward-correction-bound-geometry-across-default-bounds` | `residue-profile-cohort-invariant-width5-connector-00022-direction-forward` |
| 6 | 5 | `53191` | `stable-family-level-ratio-geometry` | `forward` | 2 | 18 | `stable-forward-correction-bound-geometry-across-default-bounds` | `stable-forward-correction-bound-geometry-across-default-bounds` | `residue-profile-cohort-invariant-width5-connector-53191-direction-forward` |
| 7 | 7 | `0050020` | `surface-split-or-concentrated` | `none` | 2 | 18 | `stable-forward-correction-bound-geometry-across-default-bounds` | `stable-reverse-correction-bound-geometry-across-default-bounds` | `do-not-promote-frozen-portfolio-width7-connector-0050020-ratio-geometry-without-shared-stable-direction` |
| 8 | 5 | `04700` | `surface-split-or-concentrated` | `none` | 2 | 18 | `stable-forward-correction-bound-geometry-across-default-bounds` | `stable-reverse-correction-bound-geometry-across-default-bounds` | `do-not-promote-frozen-portfolio-width5-connector-04700-ratio-geometry-without-shared-stable-direction` |
| 9 | 5 | `04900` | `surface-split-or-concentrated` | `none` | 2 | 18 | `stable-forward-correction-bound-geometry-across-default-bounds` | `stable-reverse-correction-bound-geometry-across-default-bounds` | `do-not-promote-frozen-portfolio-width5-connector-04900-ratio-geometry-without-shared-stable-direction` |
| 10 | 7 | `0286717` | `surface-split-or-concentrated` | `none` | 1 | 9 | `family-concentrated-correction-bound-geometry-across-default-bounds` | `stable-forward-correction-bound-geometry-across-default-bounds` | `do-not-promote-frozen-portfolio-width7-connector-0286717-ratio-geometry-without-shared-stable-direction` |
| 11 | 5 | `02510` | `surface-split-or-concentrated` | `none` | 1 | 9 | `stable-forward-correction-bound-geometry-across-default-bounds` | `family-concentrated-correction-bound-geometry-across-default-bounds` | `do-not-promote-frozen-portfolio-width5-connector-02510-ratio-geometry-without-shared-stable-direction` |
| 12 | 9 | `000550550` | `surface-split-or-concentrated` | `none` | 1 | 9 | `stable-reverse-correction-bound-geometry-across-default-bounds` | `family-concentrated-correction-bound-geometry-across-default-bounds` | `do-not-promote-frozen-portfolio-width9-connector-000550550-ratio-geometry-without-shared-stable-direction` |
| 13 | 6 | `000122` | `surface-split-or-concentrated` | `none` | 1 | 9 | `stable-forward-correction-bound-geometry-across-default-bounds` | `family-concentrated-correction-bound-geometry-across-default-bounds` | `do-not-promote-frozen-portfolio-width6-connector-000122-ratio-geometry-without-shared-stable-direction` |
| 14 | 6 | `001139` | `surface-split-or-concentrated` | `none` | 0 | 0 | `balanced-split-correction-bound-geometry-across-default-bounds` | `family-concentrated-correction-bound-geometry-across-default-bounds` | `do-not-promote-frozen-portfolio-width6-connector-001139-ratio-geometry-without-shared-stable-direction` |

### Cohort-Invariant Residue Profile

This block profiles the selected frozen-portfolio cohort invariant by small-prime residue classes across the same gap-band and size-band surfaces. A theorem candidate is surfaced only if the same exact residue mask is retained on both surfaces; otherwise the result remains empirical falsification/accounting.

- Source: width `6` connector `003727` direction `reverse` surfaces `["gap-band", "size-band"]`
- Rows `216`; target-direction rows `29`; comparison rows `187`; exact separators `0`; coherent separators `0`
- Status `no-small-prime-cohort-invariant-residue-separator`; decision `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification`; target `select-new-cohort-invariant-after-003727-residue-profile-no-coherent-separator`

- Best coherent separator: `none`

| Surface | Rows | Target rows | Comparison rows | Exact separators | Best modulus | Best residues | Status |
|---|---:|---:|---:|---:|---|---|---|
| `gap-band` | 108 | 14 | 94 | 0 | `none` | `[]` | `surface-has-no-small-prime-exact-separator` |
| `size-band` | 108 | 15 | 93 | 0 | `none` | `[]` | `surface-has-no-small-prime-exact-separator` |

| Surface | Modulus | Target residues | Shared residues | Status |
|---|---:|---|---|---|
| `gap-band` | 3 | `[1, 2]` | `[1, 2]` | `overlapping-residue-classes` |
| `gap-band` | 5 | `[1, 2, 3, 4]` | `[1, 2, 3, 4]` | `overlapping-residue-classes` |
| `gap-band` | 7 | `[1, 2, 3, 4, 6]` | `[1, 2, 3, 4, 6]` | `overlapping-residue-classes` |
| `gap-band` | 11 | `[1, 2, 3, 5, 7, 9, 10]` | `[1, 2, 3, 5, 7, 9, 10]` | `overlapping-residue-classes` |
| `gap-band` | 13 | `[3, 5, 7, 8, 9, 10, 11, 12]` | `[3, 5, 7, 8, 9, 10, 11, 12]` | `overlapping-residue-classes` |
| `gap-band` | 17 | `[1, 2, 3, 4, 5, 7, 8, 11, 16]` | `[1, 2, 3, 4, 5, 7, 8, 11, 16]` | `overlapping-residue-classes` |
| `gap-band` | 19 | `[6, 7, 9, 10, 11, 12, 13, 15, 16, 17, 18]` | `[6, 7, 9, 10, 11, 12, 13, 15, 16, 17, 18]` | `overlapping-residue-classes` |
| `gap-band` | 23 | `[2, 4, 5, 6, 8, 9, 11, 12, 14, 16, 18, 21]` | `[2, 4, 5, 6, 8, 9, 11, 12, 14, 16, 18, 21]` | `overlapping-residue-classes` |
| `gap-band` | 29 | `[2, 6, 8, 15, 17, 18, 23, 25, 26, 28]` | `[2, 6, 8, 17, 18, 23, 25, 26, 28]` | `overlapping-residue-classes` |
| `gap-band` | 31 | `[2, 3, 6, 10, 11, 15, 18, 22, 23, 26, 27, 28]` | `[3, 6, 10, 11, 15, 18, 22, 23, 26, 27, 28]` | `overlapping-residue-classes` |
| `size-band` | 3 | `[2]` | `[2]` | `overlapping-residue-classes` |
| `size-band` | 5 | `[1, 3, 4]` | `[1, 3, 4]` | `overlapping-residue-classes` |
| `size-band` | 7 | `[1, 2, 3, 4, 5]` | `[1, 2, 3, 4, 5]` | `overlapping-residue-classes` |
| `size-band` | 11 | `[1, 4, 5, 6, 7, 9, 10]` | `[1, 4, 5, 6, 7, 9, 10]` | `overlapping-residue-classes` |
| `size-band` | 13 | `[1, 3, 4, 6, 9, 10, 12]` | `[1, 3, 4, 6, 9, 10, 12]` | `overlapping-residue-classes` |
| `size-band` | 17 | `[1, 2, 4, 8, 11, 12, 13, 14, 15, 16]` | `[1, 2, 4, 8, 11, 12, 13, 14, 15, 16]` | `overlapping-residue-classes` |
| `size-band` | 19 | `[1, 3, 4, 5, 6, 7, 8, 12, 14, 15, 18]` | `[1, 3, 4, 5, 6, 7, 8, 12, 14, 15, 18]` | `overlapping-residue-classes` |
| `size-band` | 23 | `[5, 6, 7, 8, 10, 11, 13, 17, 18, 19, 20]` | `[5, 6, 7, 8, 10, 11, 13, 17, 18, 19, 20]` | `overlapping-residue-classes` |
| `size-band` | 29 | `[2, 3, 6, 8, 10, 12, 13, 16, 18, 19, 27]` | `[2, 3, 6, 8, 10, 12, 13, 16, 18, 19, 27]` | `overlapping-residue-classes` |
| `size-band` | 31 | `[1, 3, 4, 5, 12, 13, 14, 15, 17, 22]` | `[1, 3, 4, 5, 12, 13, 14, 15, 17]` | `overlapping-residue-classes` |

### Next Cohort-Invariant Picker

This picker follows a residue-profile null result by excluding already-profiled cohort invariants and selecting the next correction-stable ratio-geometry surface. It is empirical routing only; selected rows still require profiling or replication before theorem work.

- Excluded profiles `1`; remaining candidates `13`; stable candidates `5`; decision `cohort-invariant-next-picker-selected-unprofiled-stable-ratio-geometry`; target `residue-profile-cohort-invariant-width6-connector-276061-direction-reverse`

- Selected next invariant: width `6` connector `276061` direction `reverse` stable bounds `18` target `residue-profile-cohort-invariant-width6-connector-276061-direction-reverse`

| Excluded width | Excluded connector | Direction | Profile status | Profile decision |
|---:|---|---|---|---|
| 6 | `003727` | `reverse` | `no-small-prime-cohort-invariant-residue-separator` | `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification` |

| Rank | Width | Connector | Status | Direction | Stable surfaces | Stable bounds | Target |
|---:|---:|---|---|---|---:|---:|---|
| 1 | 6 | `276061` | `stable-family-level-ratio-geometry` | `reverse` | 2 | 18 | `residue-profile-cohort-invariant-width6-connector-276061-direction-reverse` |
| 2 | 5 | `91736` | `stable-family-level-ratio-geometry` | `reverse` | 2 | 18 | `residue-profile-cohort-invariant-width5-connector-91736-direction-reverse` |
| 3 | 9 | `900020000` | `stable-family-level-ratio-geometry` | `forward` | 2 | 18 | `residue-profile-cohort-invariant-width9-connector-900020000-direction-forward` |
| 4 | 5 | `00022` | `stable-family-level-ratio-geometry` | `forward` | 2 | 18 | `residue-profile-cohort-invariant-width5-connector-00022-direction-forward` |
| 5 | 5 | `53191` | `stable-family-level-ratio-geometry` | `forward` | 2 | 18 | `residue-profile-cohort-invariant-width5-connector-53191-direction-forward` |
| 6 | 7 | `0050020` | `surface-split-or-concentrated` | `none` | 2 | 18 | `do-not-promote-frozen-portfolio-width7-connector-0050020-ratio-geometry-without-shared-stable-direction` |
| 7 | 5 | `04700` | `surface-split-or-concentrated` | `none` | 2 | 18 | `do-not-promote-frozen-portfolio-width5-connector-04700-ratio-geometry-without-shared-stable-direction` |
| 8 | 5 | `04900` | `surface-split-or-concentrated` | `none` | 2 | 18 | `do-not-promote-frozen-portfolio-width5-connector-04900-ratio-geometry-without-shared-stable-direction` |

### Next Cohort-Invariant Residue Profile

This block profiles the next unprofiled frozen-portfolio cohort invariant by small-prime residue classes. It uses the same coherent-separator rule as the first profile: no Lean theorem candidate appears unless one exact mask is retained across both gap-band and size-band surfaces.

- Source: width `6` connector `276061` direction `reverse` surfaces `["gap-band", "size-band"]`
- Rows `216`; target-direction rows `15`; comparison rows `201`; exact separators `0`; coherent separators `0`
- Status `no-small-prime-cohort-invariant-residue-separator`; decision `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification`; target `select-new-cohort-invariant-after-276061-residue-profile-no-coherent-separator`

- Best coherent separator: `none`

| Surface | Rows | Target rows | Comparison rows | Exact separators | Best modulus | Best residues | Status |
|---|---:|---:|---:|---:|---|---|---|
| `gap-band` | 108 | 8 | 100 | 0 | `none` | `[]` | `surface-has-no-small-prime-exact-separator` |
| `size-band` | 108 | 7 | 101 | 0 | `none` | `[]` | `surface-has-no-small-prime-exact-separator` |

| Surface | Modulus | Target residues | Shared residues | Status |
|---|---:|---|---|---|
| `gap-band` | 3 | `[1, 2]` | `[1, 2]` | `overlapping-residue-classes` |
| `gap-band` | 5 | `[1, 2, 3, 4]` | `[1, 2, 3, 4]` | `overlapping-residue-classes` |
| `gap-band` | 7 | `[1, 2, 3, 4, 5]` | `[1, 2, 3, 4, 5]` | `overlapping-residue-classes` |
| `gap-band` | 11 | `[1, 2, 3, 4, 7, 9, 10]` | `[1, 2, 3, 4, 7, 9, 10]` | `overlapping-residue-classes` |
| `gap-band` | 13 | `[2, 3, 5, 6, 7]` | `[2, 3, 5, 6, 7]` | `overlapping-residue-classes` |
| `gap-band` | 17 | `[3, 5, 7, 12, 14, 15, 16]` | `[3, 5, 7, 12, 14, 15, 16]` | `overlapping-residue-classes` |
| `gap-band` | 19 | `[7, 11, 12, 14, 17]` | `[7, 11, 12, 14, 17]` | `overlapping-residue-classes` |
| `gap-band` | 23 | `[1, 2, 3, 6, 12, 13, 20]` | `[1, 2, 3, 6, 12, 13, 20]` | `overlapping-residue-classes` |
| `gap-band` | 29 | `[2, 3, 10, 12, 14, 21, 25]` | `[2, 3, 10, 12, 14, 21, 25]` | `overlapping-residue-classes` |
| `gap-band` | 31 | `[10, 11, 12, 19, 23, 24, 29]` | `[10, 11, 12, 19, 23, 24, 29]` | `overlapping-residue-classes` |
| `size-band` | 3 | `[2]` | `[2]` | `overlapping-residue-classes` |
| `size-band` | 5 | `[1, 3, 4]` | `[1, 3, 4]` | `overlapping-residue-classes` |
| `size-band` | 7 | `[1, 3, 4, 5]` | `[1, 3, 4, 5]` | `overlapping-residue-classes` |
| `size-band` | 11 | `[1, 5, 6, 8, 9]` | `[1, 5, 6, 8, 9]` | `overlapping-residue-classes` |
| `size-band` | 13 | `[1, 6, 7, 8]` | `[1, 6, 7, 8]` | `overlapping-residue-classes` |
| `size-band` | 17 | `[3, 5, 7, 11, 15]` | `[3, 5, 7, 11, 15]` | `overlapping-residue-classes` |
| `size-band` | 19 | `[1, 5, 7, 10, 15]` | `[1, 5, 7, 10, 15]` | `overlapping-residue-classes` |
| `size-band` | 23 | `[1, 3, 4, 7, 10, 16, 22]` | `[1, 3, 4, 7, 10, 16, 22]` | `overlapping-residue-classes` |
| `size-band` | 29 | `[9, 10, 11, 12, 13, 27]` | `[9, 10, 11, 12, 13, 27]` | `overlapping-residue-classes` |
| `size-band` | 31 | `[4, 13, 14, 15, 16, 18, 26]` | `[4, 13, 14, 15, 18, 26]` | `overlapping-residue-classes` |

### Post-Two-Null Cohort-Invariant Picker

This picker follows two residue-profile null results by excluding both profiled cohort invariants and selecting the next correction-stable ratio-geometry surface. It is still empirical routing only; selected rows require their own residue profile before theorem work.

- Excluded profiles `2`; remaining candidates `12`; stable candidates `4`; decision `cohort-invariant-post-two-null-picker-selected-unprofiled-stable-ratio-geometry`; target `residue-profile-cohort-invariant-width5-connector-91736-direction-reverse`

- Selected next invariant: width `5` connector `91736` direction `reverse` stable bounds `18` target `residue-profile-cohort-invariant-width5-connector-91736-direction-reverse`

| Excluded width | Excluded connector | Direction | Profile status | Profile decision |
|---:|---|---|---|---|
| 6 | `003727` | `reverse` | `no-small-prime-cohort-invariant-residue-separator` | `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification` |
| 6 | `276061` | `reverse` | `no-small-prime-cohort-invariant-residue-separator` | `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification` |

| Rank | Width | Connector | Status | Direction | Stable surfaces | Stable bounds | Target |
|---:|---:|---|---|---|---:|---:|---|
| 1 | 5 | `91736` | `stable-family-level-ratio-geometry` | `reverse` | 2 | 18 | `residue-profile-cohort-invariant-width5-connector-91736-direction-reverse` |
| 2 | 9 | `900020000` | `stable-family-level-ratio-geometry` | `forward` | 2 | 18 | `residue-profile-cohort-invariant-width9-connector-900020000-direction-forward` |
| 3 | 5 | `00022` | `stable-family-level-ratio-geometry` | `forward` | 2 | 18 | `residue-profile-cohort-invariant-width5-connector-00022-direction-forward` |
| 4 | 5 | `53191` | `stable-family-level-ratio-geometry` | `forward` | 2 | 18 | `residue-profile-cohort-invariant-width5-connector-53191-direction-forward` |
| 5 | 7 | `0050020` | `surface-split-or-concentrated` | `none` | 2 | 18 | `do-not-promote-frozen-portfolio-width7-connector-0050020-ratio-geometry-without-shared-stable-direction` |
| 6 | 5 | `04700` | `surface-split-or-concentrated` | `none` | 2 | 18 | `do-not-promote-frozen-portfolio-width5-connector-04700-ratio-geometry-without-shared-stable-direction` |
| 7 | 5 | `04900` | `surface-split-or-concentrated` | `none` | 2 | 18 | `do-not-promote-frozen-portfolio-width5-connector-04900-ratio-geometry-without-shared-stable-direction` |
| 8 | 7 | `0286717` | `surface-split-or-concentrated` | `none` | 1 | 9 | `do-not-promote-frozen-portfolio-width7-connector-0286717-ratio-geometry-without-shared-stable-direction` |

### Post-Two-Null Cohort-Invariant Residue Profile

This block profiles the post-two-null selected cohort invariant by small-prime residue classes. The theorem gate remains unchanged: a candidate advances only if one exact residue mask is coherent across both gap-band and size-band surfaces.

- Source: width `5` connector `91736` direction `reverse` surfaces `["gap-band", "size-band"]`
- Rows `216`; target-direction rows `26`; comparison rows `190`; exact separators `0`; coherent separators `0`
- Status `no-small-prime-cohort-invariant-residue-separator`; decision `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification`; target `select-new-cohort-invariant-after-91736-residue-profile-no-coherent-separator`

- Best coherent separator: `none`

| Surface | Rows | Target rows | Comparison rows | Exact separators | Best modulus | Best residues | Status |
|---|---:|---:|---:|---:|---|---|---|
| `gap-band` | 108 | 13 | 95 | 0 | `none` | `[]` | `surface-has-no-small-prime-exact-separator` |
| `size-band` | 108 | 13 | 95 | 0 | `none` | `[]` | `surface-has-no-small-prime-exact-separator` |

| Surface | Modulus | Target residues | Shared residues | Status |
|---|---:|---|---|---|
| `gap-band` | 3 | `[1, 2]` | `[1, 2]` | `overlapping-residue-classes` |
| `gap-band` | 5 | `[1, 2, 3, 4]` | `[1, 2, 3, 4]` | `overlapping-residue-classes` |
| `gap-band` | 7 | `[1, 2, 3, 4, 5]` | `[1, 2, 3, 4, 5]` | `overlapping-residue-classes` |
| `gap-band` | 11 | `[1, 3, 5, 6, 8, 9]` | `[1, 3, 5, 6, 8, 9]` | `overlapping-residue-classes` |
| `gap-band` | 13 | `[1, 2, 3, 4, 5, 6, 7, 10, 11, 12]` | `[1, 2, 3, 4, 5, 6, 7, 10, 11, 12]` | `overlapping-residue-classes` |
| `gap-band` | 17 | `[1, 3, 7, 9, 12, 15, 16]` | `[1, 3, 7, 9, 12, 15, 16]` | `overlapping-residue-classes` |
| `gap-band` | 19 | `[2, 4, 6, 8, 9, 11, 14, 15, 17, 18]` | `[2, 4, 6, 8, 9, 11, 14, 15, 17, 18]` | `overlapping-residue-classes` |
| `gap-band` | 23 | `[2, 4, 5, 6, 12, 13, 18, 19, 20]` | `[2, 4, 5, 6, 12, 13, 18, 19, 20]` | `overlapping-residue-classes` |
| `gap-band` | 29 | `[2, 4, 5, 12, 14, 17, 19, 22, 24, 27, 28]` | `[2, 4, 5, 12, 14, 17, 19, 22, 24, 27, 28]` | `overlapping-residue-classes` |
| `gap-band` | 31 | `[1, 3, 4, 11, 12, 14, 15, 18, 20, 25, 26, 27, 30]` | `[1, 3, 4, 11, 12, 14, 15, 18, 25, 26, 27, 30]` | `overlapping-residue-classes` |
| `size-band` | 3 | `[2]` | `[2]` | `overlapping-residue-classes` |
| `size-band` | 5 | `[1, 3, 4]` | `[1, 3, 4]` | `overlapping-residue-classes` |
| `size-band` | 7 | `[1, 2, 3, 4, 5]` | `[1, 2, 3, 4, 5]` | `overlapping-residue-classes` |
| `size-band` | 11 | `[1, 2, 4, 6, 8, 9, 10]` | `[1, 2, 4, 6, 8, 9, 10]` | `overlapping-residue-classes` |
| `size-band` | 13 | `[2, 3, 4, 6, 7, 8, 9, 10, 12]` | `[2, 3, 4, 6, 7, 8, 9, 10, 12]` | `overlapping-residue-classes` |
| `size-band` | 17 | `[1, 2, 3, 5, 6, 7, 13, 14]` | `[1, 2, 3, 5, 6, 7, 13, 14]` | `overlapping-residue-classes` |
| `size-band` | 19 | `[2, 6, 8, 13, 14, 15, 17, 18]` | `[2, 6, 8, 13, 14, 15, 17, 18]` | `overlapping-residue-classes` |
| `size-band` | 23 | `[1, 2, 3, 4, 5, 7, 10, 11, 14, 17]` | `[1, 2, 3, 4, 5, 7, 10, 11, 14, 17]` | `overlapping-residue-classes` |
| `size-band` | 29 | `[1, 4, 5, 8, 11, 13, 16, 18, 19, 25, 27]` | `[1, 4, 5, 8, 11, 13, 16, 18, 19, 25, 27]` | `overlapping-residue-classes` |
| `size-band` | 31 | `[2, 3, 4, 10, 11, 13, 20, 24, 27, 29, 30]` | `[2, 3, 4, 10, 11, 13, 20, 24, 27, 29, 30]` | `overlapping-residue-classes` |

### Three-Null Cohort-Invariant Conclusion

This block stops row-by-row reverse-candidate residue profiling after three correction-stable reverse cohort invariants fail the same coherent small-prime exact-mask gate. It marks that reverse route as collapsed under the current rule and pivots to the remaining forward stable candidates. This is routing/accounting only, not a theorem or density claim.

- Collapsed direction `reverse` across `3` profiled rows; status `reverse-stable-ratio-cohort-residue-route-collapsed-under-small-prime-exact-mask-rule`; decision `pivot-to-forward-stable-ratio-cohort-candidates-after-three-reverse-residue-nulls`; target `residue-profile-cohort-invariant-width9-connector-900020000-direction-forward`

- Selected forward invariant: width `9` connector `900020000` direction `forward` stable bounds `18` target `residue-profile-cohort-invariant-width9-connector-900020000-direction-forward`

| Collapsed width | Collapsed connector | Direction | Profile status | Profile decision |
|---:|---|---|---|---|
| 6 | `003727` | `reverse` | `no-small-prime-cohort-invariant-residue-separator` | `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification` |
| 6 | `276061` | `reverse` | `no-small-prime-cohort-invariant-residue-separator` | `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification` |
| 5 | `91736` | `reverse` | `no-small-prime-cohort-invariant-residue-separator` | `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification` |

| Rank | Width | Connector | Direction | Stable surfaces | Stable bounds | Target |
|---:|---:|---|---|---:|---:|---|
| 1 | 9 | `900020000` | `forward` | 2 | 18 | `residue-profile-cohort-invariant-width9-connector-900020000-direction-forward` |
| 2 | 5 | `00022` | `forward` | 2 | 18 | `residue-profile-cohort-invariant-width5-connector-00022-direction-forward` |
| 3 | 5 | `53191` | `forward` | 2 | 18 | `residue-profile-cohort-invariant-width5-connector-53191-direction-forward` |

### Forward Cohort-Invariant Residue Profile

This block profiles the forward stable-ratio candidate selected after the reverse route collapsed. The gate is unchanged: a candidate advances toward theorem work only if one exact residue mask is coherent across both gap-band and size-band surfaces.

- Source: width `9` connector `900020000` direction `forward` surfaces `["gap-band", "size-band"]`
- Rows `216`; target-direction rows `23`; comparison rows `193`; exact separators `0`; coherent separators `0`
- Status `no-small-prime-cohort-invariant-residue-separator`; decision `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification`; target `select-new-cohort-invariant-after-900020000-residue-profile-no-coherent-separator`

- Best coherent separator: `none`

| Surface | Rows | Target rows | Comparison rows | Exact separators | Best modulus | Best residues | Status |
|---|---:|---:|---:|---:|---|---|---|
| `gap-band` | 108 | 9 | 99 | 0 | `none` | `[]` | `surface-has-no-small-prime-exact-separator` |
| `size-band` | 108 | 14 | 94 | 0 | `none` | `[]` | `surface-has-no-small-prime-exact-separator` |

| Surface | Modulus | Target residues | Shared residues | Status |
|---|---:|---|---|---|
| `gap-band` | 3 | `[1, 2]` | `[1, 2]` | `overlapping-residue-classes` |
| `gap-band` | 5 | `[1, 2, 3]` | `[1, 2, 3]` | `overlapping-residue-classes` |
| `gap-band` | 7 | `[1, 2, 3, 6]` | `[1, 2, 3, 6]` | `overlapping-residue-classes` |
| `gap-band` | 11 | `[1, 2, 7, 8, 9]` | `[1, 2, 7, 8, 9]` | `overlapping-residue-classes` |
| `gap-band` | 13 | `[1, 3, 8, 11]` | `[1, 3, 8, 11]` | `overlapping-residue-classes` |
| `gap-band` | 17 | `[5, 6, 10, 12, 13, 16]` | `[5, 6, 10, 12, 13, 16]` | `overlapping-residue-classes` |
| `gap-band` | 19 | `[5, 6, 8, 9, 10, 14, 15, 17]` | `[5, 6, 8, 9, 10, 14, 15, 17]` | `overlapping-residue-classes` |
| `gap-band` | 23 | `[4, 9, 10, 16, 17, 18, 19, 21]` | `[4, 9, 10, 16, 17, 18, 19, 21]` | `overlapping-residue-classes` |
| `gap-band` | 29 | `[4, 5, 8, 16, 19, 21, 27]` | `[4, 5, 8, 16, 19, 21, 27]` | `overlapping-residue-classes` |
| `gap-band` | 31 | `[3, 5, 10, 21, 23, 27, 28, 30]` | `[3, 5, 10, 21, 23, 27, 28, 30]` | `overlapping-residue-classes` |
| `size-band` | 3 | `[2]` | `[2]` | `overlapping-residue-classes` |
| `size-band` | 5 | `[1, 3, 4]` | `[1, 3, 4]` | `overlapping-residue-classes` |
| `size-band` | 7 | `[1, 2, 4, 5]` | `[1, 2, 4, 5]` | `overlapping-residue-classes` |
| `size-band` | 11 | `[2, 4, 5, 6, 7, 8, 9, 10]` | `[2, 4, 5, 6, 7, 8, 9, 10]` | `overlapping-residue-classes` |
| `size-band` | 13 | `[1, 3, 4, 6, 7, 8, 9, 10, 11]` | `[1, 3, 4, 6, 7, 8, 9, 10, 11]` | `overlapping-residue-classes` |
| `size-band` | 17 | `[2, 3, 5, 8, 10, 11, 15, 16]` | `[2, 3, 5, 8, 10, 11, 15, 16]` | `overlapping-residue-classes` |
| `size-band` | 19 | `[5, 6, 7, 8, 10, 13, 14, 16, 18]` | `[5, 6, 7, 8, 10, 13, 14, 18]` | `overlapping-residue-classes` |
| `size-band` | 23 | `[1, 3, 7, 8, 9, 10, 18, 20, 21, 22]` | `[1, 3, 7, 8, 9, 10, 18, 20, 21, 22]` | `overlapping-residue-classes` |
| `size-band` | 29 | `[5, 8, 11, 12, 13, 14, 16, 17, 19, 24, 27, 28]` | `[5, 8, 11, 12, 13, 14, 16, 17, 19, 24, 27, 28]` | `overlapping-residue-classes` |
| `size-band` | 31 | `[3, 4, 5, 6, 8, 10, 11, 16, 19, 22, 29]` | `[3, 4, 5, 6, 8, 10, 11, 19, 29]` | `overlapping-residue-classes` |

### Forward-Route Cohort-Invariant Conclusion

This block stops row-by-row forward-candidate residue profiling after the selected forward stable-ratio invariant fails the coherent small-prime exact-mask gate. It records the remaining forward rows as audit context, then pivots to a new cohort-invariant surface. This is routing/accounting only, not a theorem or density claim.

- Collapsed direction `forward` across `1` profiled rows; status `forward-stable-ratio-cohort-residue-route-collapsed-under-small-prime-exact-mask-rule`; decision `pivot-to-new-cohort-invariant-surface-after-forward-residue-null`; remaining stable candidates `2`; target `select-new-cohort-invariant-surface-after-forward-route-small-prime-exact-mask-null`

| Collapsed width | Collapsed connector | Direction | Profile status | Profile decision |
|---:|---|---|---|---|
| 9 | `900020000` | `forward` | `no-small-prime-cohort-invariant-residue-separator` | `cohort-invariant-residue-profile-no-small-prime-separator-record-falsification` |

| Rank | Width | Connector | Direction | Stable surfaces | Stable bounds | Target |
|---:|---:|---|---|---:|---:|---|
| 1 | 5 | `00022` | `forward` | 2 | 18 | `residue-profile-cohort-invariant-width5-connector-00022-direction-forward` |
| 2 | 5 | `53191` | `forward` | 2 | 18 | `residue-profile-cohort-invariant-width5-connector-53191-direction-forward` |

### Pair-Family Gap Cohort Window-Consensus Surface

This surface follows the reverse/forward exact-mask nulls by asking a broader cohort question: does any frozen connector keep the same source/fresh/separator window-bias direction across both gap-band and size-band controls? It is empirical routing only; selected rows require independent stress testing before residue profiling or theorem work.

- Candidates `14`; surfaces `["gap-band", "size-band"]`; windows `252`; decision `window-consensus-surface-selected-shared-stage-complete-invariant`; target `stress-test-window-consensus-cohort-invariant-width6-connector-003727-direction-reverse`

- Selected window-consensus invariant: width `6` connector `003727` direction `reverse` consensus windows `10` opposite windows `2` target `stress-test-window-consensus-cohort-invariant-width6-connector-003727-direction-reverse`

| Rank | Width | Connector | Status | Direction | Consensus surfaces | Consensus windows | Opposite windows | Target |
|---:|---:|---|---|---|---:|---:|---:|---|
| 1 | 6 | `003727` | `shared-stage-complete-window-consensus` | `reverse` | 2 | 10 | 2 | `stress-test-window-consensus-cohort-invariant-width6-connector-003727-direction-reverse` |
| 2 | 5 | `04700` | `partial-window-consensus` | `none` | 2 | 0 | 0 | `do-not-promote-window-consensus-width5-connector-04700-without-shared-stage-complete-consensus` |
| 3 | 5 | `00022` | `partial-window-consensus` | `none` | 1 | 0 | 0 | `do-not-promote-window-consensus-width5-connector-00022-without-shared-stage-complete-consensus` |
| 4 | 7 | `0050020` | `partial-window-consensus` | `none` | 1 | 0 | 0 | `do-not-promote-window-consensus-width7-connector-0050020-without-shared-stage-complete-consensus` |
| 5 | 5 | `02510` | `partial-window-consensus` | `none` | 1 | 0 | 0 | `do-not-promote-window-consensus-width5-connector-02510-without-shared-stage-complete-consensus` |
| 6 | 6 | `001139` | `partial-window-consensus` | `none` | 1 | 0 | 0 | `do-not-promote-window-consensus-width6-connector-001139-without-shared-stage-complete-consensus` |
| 7 | 9 | `900020000` | `partial-window-consensus` | `none` | 1 | 0 | 0 | `do-not-promote-window-consensus-width9-connector-900020000-without-shared-stage-complete-consensus` |
| 8 | 5 | `04900` | `partial-window-consensus` | `none` | 1 | 0 | 0 | `do-not-promote-window-consensus-width5-connector-04900-without-shared-stage-complete-consensus` |

| Width | Connector | Surface | Status | Direction | Reverse windows | Forward windows | Neutral windows | Source/Fresh/Separator consensus |
|---:|---|---|---|---|---:|---:|---:|---|
| 6 | `003727` | `gap-band` | `reverse-stage-complete-window-consensus` | `reverse` | 5 | 1 | 3 | `3/1/1` |
| 6 | `003727` | `size-band` | `reverse-stage-complete-window-consensus` | `reverse` | 5 | 1 | 3 | `1/2/2` |
| 6 | `276061` | `gap-band` | `split-window-consensus` | `reverse` | 4 | 0 | 5 | `1/2/1` |
| 6 | `276061` | `size-band` | `split-window-consensus` | `reverse` | 3 | 2 | 4 | `1/2/0` |
| 5 | `91736` | `gap-band` | `split-window-consensus` | `none` | 3 | 3 | 3 | `0/0/0` |
| 5 | `91736` | `size-band` | `split-window-consensus` | `reverse` | 2 | 1 | 6 | `2/0/0` |
| 9 | `900020000` | `gap-band` | `split-window-consensus` | `forward` | 1 | 4 | 4 | `2/2/0` |
| 9 | `900020000` | `size-band` | `forward-stage-complete-window-consensus` | `forward` | 2 | 5 | 2 | `1/2/2` |
| 5 | `00022` | `gap-band` | `forward-stage-complete-window-consensus` | `forward` | 3 | 6 | 0 | `2/2/2` |
| 5 | `00022` | `size-band` | `split-window-consensus` | `forward` | 3 | 4 | 2 | `1/2/1` |
| 5 | `53191` | `gap-band` | `split-window-consensus` | `none` | 2 | 2 | 5 | `0/0/0` |
| 5 | `53191` | `size-band` | `split-window-consensus` | `forward` | 2 | 3 | 4 | `0/1/2` |

### Pair-Family Gap Cohort Window-Consensus Stress

This block stress-tests the selected window-consensus invariant on held-out gap20/22/24 and generated gap8 size160k/200k/240k source/fresh/separator windows. It remains empirical routing only: retained stress geometry may be expanded before residue profiling, while split/collapsed geometry is recorded as falsification.

- Source: width `6` connector `003727` direction `reverse` target `stress-test-window-consensus-cohort-invariant-width6-connector-003727-direction-reverse`
- Surfaces `["heldout-gap-band", "generated-gap8-size-band"]`; pair families `6`; windows `18`; retained/split/collapsed `0`/`0`/`2`
- Status `window-consensus-collapsed-on-heldout-gap-size-surfaces`; decision `window-consensus-heldout-stress-collapsed-record-falsification`; target `select-new-cohort-invariant-surface-after-window-consensus-heldout-collapse`

| Surface | Status | Direction | Reverse windows | Forward windows | Neutral windows | Source/Fresh/Separator consensus |
|---|---|---|---:|---:|---:|---|
| `heldout-gap-band` | `split-window-consensus` | `reverse` | 2 | 1 | 6 | `1/1/0` |
| `generated-gap8-size-band` | `split-window-consensus` | `reverse` | 4 | 3 | 2 | `1/1/2` |

| Pair family | Stage | Reverse-only | Forward-only | Both-hit | Neither-hit | Bias |
|---|---|---:|---:|---:|---:|---|
| `prime-gap20` | `source` | 1 | 0 | 0 | 11 | `reverse` |
| `prime-gap20` | `fresh` | 0 | 0 | 0 | 12 | `neutral` |
| `prime-gap20` | `separator` | 2 | 2 | 1 | 7 | `neutral` |
| `prime-gap22` | `source` | 1 | 1 | 0 | 10 | `neutral` |
| `prime-gap22` | `fresh` | 0 | 0 | 1 | 11 | `neutral` |
| `prime-gap22` | `separator` | 0 | 4 | 0 | 8 | `forward` |
| `prime-gap24` | `source` | 1 | 1 | 0 | 10 | `neutral` |
| `prime-gap24` | `fresh` | 1 | 0 | 0 | 11 | `reverse` |
| `prime-gap24` | `separator` | 0 | 0 | 1 | 11 | `neutral` |
| `prime-gap8-size160k` | `source` | 1 | 1 | 0 | 10 | `neutral` |
| `prime-gap8-size160k` | `fresh` | 0 | 2 | 1 | 9 | `forward` |
| `prime-gap8-size160k` | `separator` | 2 | 0 | 0 | 10 | `reverse` |
| `prime-gap8-size200k` | `source` | 1 | 1 | 0 | 10 | `neutral` |
| `prime-gap8-size200k` | `fresh` | 2 | 1 | 0 | 9 | `reverse` |
| `prime-gap8-size200k` | `separator` | 1 | 0 | 0 | 11 | `reverse` |
| `prime-gap8-size240k` | `source` | 2 | 1 | 1 | 8 | `reverse` |
| `prime-gap8-size240k` | `fresh` | 0 | 1 | 0 | 11 | `forward` |
| `prime-gap8-size240k` | `separator` | 0 | 2 | 0 | 10 | `forward` |

### Pair-Family Gap Cohort Sign-Persistence Picker

After the held-out window-consensus surface collapses, this block scans the same frozen connector portfolio for lower-bar but still guarded sign persistence across the original gap/size surfaces and held-out gap/size surfaces. It remains empirical routing only: sign persistence is not a residue theorem, connector law, or density mechanism.

- Candidates `14`; surfaces `["gap-band", "size-band", "heldout-gap-band", "generated-gap8-size-band"]`; pair families `12`; windows `504`; persistent candidates `4`
- Decision `sign-persistence-picker-selected-low-volatility-cohort-invariant`; target `stress-test-sign-persistence-cohort-invariant-width6-connector-003727-direction-reverse`

- Selected sign-persistence candidate: width `6` connector `003727` direction `reverse` surfaces `4` volatility `6` target `stress-test-sign-persistence-cohort-invariant-width6-connector-003727-direction-reverse`

| Rank | Width | Connector | Status | Direction | Surfaces | Retained/Opposite/Neutral windows | Volatility | Target |
|---:|---:|---|---|---|---:|---|---:|---|
| 1 | 6 | `003727` | `surface-sign-persistent-cohort-invariant` | `reverse` | 4 | `16/6/14` | 6 | `stress-test-sign-persistence-cohort-invariant-width6-connector-003727-direction-reverse` |
| 2 | 6 | `000122` | `surface-sign-persistent-cohort-invariant` | `forward` | 4 | `14/7/15` | 7 | `stress-test-sign-persistence-cohort-invariant-width6-connector-000122-direction-forward` |
| 3 | 5 | `00022` | `surface-sign-persistent-cohort-invariant` | `forward` | 4 | `18/8/10` | 8 | `stress-test-sign-persistence-cohort-invariant-width5-connector-00022-direction-forward` |
| 4 | 5 | `53191` | `surface-sign-persistent-cohort-invariant` | `forward` | 3 | `13/7/16` | 32 | `stress-test-sign-persistence-cohort-invariant-width5-connector-53191-direction-forward` |
| 5 | 5 | `04700` | `mixed-sign-persistence` | `reverse` | 3 | `21/5/10` | 105 | `do-not-promote-sign-persistence-width5-connector-04700-without-low-volatility-surface-retention` |
| 6 | 6 | `276061` | `mixed-sign-persistence` | `reverse` | 3 | `11/7/18` | 107 | `do-not-promote-sign-persistence-width6-connector-276061-without-low-volatility-surface-retention` |
| 7 | 7 | `0050020` | `mixed-sign-persistence` | `reverse` | 3 | `17/8/11` | 108 | `do-not-promote-sign-persistence-width7-connector-0050020-without-low-volatility-surface-retention` |
| 8 | 5 | `04900` | `mixed-sign-persistence` | `reverse` | 3 | `11/12/13` | 112 | `do-not-promote-sign-persistence-width5-connector-04900-without-low-volatility-surface-retention` |

| Surface | Direction | Status | Reverse windows | Forward windows | Neutral windows | Signed delta |
|---|---|---|---:|---:|---:|---:|
| `gap-band` | `forward` | `forward-sign-persistent-surface` | 3 | 6 | 0 | -5 |
| `size-band` | `forward` | `forward-sign-persistent-surface` | 3 | 4 | 2 | -4 |
| `heldout-gap-band` | `forward` | `forward-sign-persistent-surface` | 1 | 3 | 5 | -5 |
| `generated-gap8-size-band` | `forward` | `forward-sign-persistent-surface` | 1 | 5 | 3 | -7 |
| `gap-band` | `none` | `neutral-or-split-sign-surface` | 3 | 3 | 3 | -5 |
| `size-band` | `forward` | `forward-sign-persistent-surface` | 2 | 5 | 2 | -3 |
| `heldout-gap-band` | `forward` | `forward-sign-persistent-surface` | 3 | 4 | 2 | -3 |
| `generated-gap8-size-band` | `reverse` | `reverse-sign-persistent-surface` | 7 | 1 | 1 | 7 |
| `gap-band` | `forward` | `forward-sign-persistent-surface` | 3 | 5 | 1 | -2 |
| `size-band` | `reverse` | `reverse-sign-persistent-surface` | 8 | 0 | 1 | 12 |
| `heldout-gap-band` | `reverse` | `reverse-sign-persistent-surface` | 3 | 0 | 6 | 4 |
| `generated-gap8-size-band` | `reverse` | `reverse-sign-persistent-surface` | 7 | 0 | 2 | 9 |

### Pair-Family Gap Cohort Sign-Persistence Stress

This block stress-tests the selected low-volatility sign-persistence candidate on disjoint generated gap and size surfaces. A retained result can route to residue profiling; a split or collapse remains empirical falsification, not a connector law, density mechanism, or theorem candidate.

- Source: width `6` connector `003727` direction `reverse` target `stress-test-sign-persistence-cohort-invariant-width6-connector-003727-direction-reverse`
- Surfaces `["fresh-generated-gap-band", "fresh-generated-gap8-size-band"]`; pair families `6`; windows `18`
- Retained/split/neutral surfaces `0/2/0`; retained/opposite/neutral windows `6/8/4`
- Status `sign-persistence-split-on-fresh-surfaces`; decision `sign-persistence-fresh-stress-split-record-falsification`; target `select-new-cohort-invariant-surface-after-sign-persistence-fresh-split`

| Surface | Direction | Status | Reverse windows | Forward windows | Neutral windows | Signed delta | Absolute delta |
|---|---|---|---:|---:|---:|---:|---:|
| `fresh-generated-gap-band` | `forward` | `forward-sign-persistent-surface` | 3 | 4 | 2 | 1 | 9 |
| `fresh-generated-gap8-size-band` | `forward` | `forward-sign-persistent-surface` | 3 | 4 | 2 | 1 | 9 |

| Pair family | Stage | Reverse-only | Forward-only | Both-hit | Neither-hit | Bias |
|---|---|---:|---:|---:|---:|---|
| `prime-gap32` | `source` | 2 | 3 | 1 | 6 | `forward` |
| `prime-gap32` | `fresh` | 4 | 1 | 0 | 7 | `reverse` |
| `prime-gap32` | `separator` | 1 | 0 | 1 | 10 | `reverse` |
| `prime-gap34` | `source` | 2 | 2 | 0 | 8 | `neutral` |
| `prime-gap34` | `fresh` | 1 | 2 | 0 | 9 | `forward` |
| `prime-gap34` | `separator` | 1 | 2 | 0 | 9 | `forward` |
| `prime-gap36` | `source` | 0 | 0 | 0 | 12 | `neutral` |
| `prime-gap36` | `fresh` | 1 | 0 | 0 | 11 | `reverse` |
| `prime-gap36` | `separator` | 1 | 2 | 0 | 9 | `forward` |
| `prime-gap8-size280k` | `source` | 1 | 0 | 0 | 11 | `reverse` |
| `prime-gap8-size280k` | `fresh` | 2 | 0 | 0 | 10 | `reverse` |
| `prime-gap8-size280k` | `separator` | 1 | 2 | 1 | 8 | `forward` |
| `prime-gap8-size320k` | `source` | 0 | 0 | 0 | 12 | `neutral` |
| `prime-gap8-size320k` | `fresh` | 2 | 0 | 0 | 10 | `reverse` |
| `prime-gap8-size320k` | `separator` | 0 | 1 | 2 | 9 | `forward` |
| `prime-gap8-size360k` | `source` | 0 | 1 | 0 | 11 | `forward` |
| `prime-gap8-size360k` | `fresh` | 1 | 1 | 1 | 9 | `neutral` |
| `prime-gap8-size360k` | `separator` | 1 | 2 | 0 | 9 | `forward` |

### Pair-Family Gap Cohort Volatility/Ensemble Picker

After the singleton sign-persistence route splits, this block asks a stricter cohort question: do multiple frozen-portfolio connectors share the same directional geometry across the maintained gap and size surfaces? This remains empirical routing only, not a residue theorem, connector law, or density mechanism.

- Candidates `14`; surfaces `["gap-band", "size-band", "heldout-gap-band", "generated-gap8-size-band", "fresh-generated-gap-band", "fresh-generated-gap8-size-band"]`; pair families `18`; windows `756`
- Ensembles `2`; qualifying `2`; decision `volatility-ensemble-picker-selected-shared-direction-cohort`; target `stress-test-volatility-ensemble-cohort-direction-forward-connector-count-14`

- Selected ensemble: direction `forward` surfaces `6` connectors `14` target `stress-test-volatility-ensemble-cohort-direction-forward-connector-count-14`
- Selected connectors: `["width5:00022", "width5:02510", "width5:04700", "width5:04900", "width5:53191", "width5:91736", "width6:000122", "width6:001139", "width6:003727", "width6:276061", "width7:0050020", "width7:0286717", "width9:000550550", "width9:900020000"]`

| Rank | Direction | Status | Surfaces | Connectors | Retained/Opposite/Neutral windows | Absolute delta | Target |
|---:|---|---|---:|---:|---|---:|---|
| 1 | `forward` | `volatility-ensemble-retained-across-surfaces` | 6 | 14 | `156/67/110` | 341 | `stress-test-volatility-ensemble-cohort-direction-forward-connector-count-14` |
| 2 | `reverse` | `volatility-ensemble-retained-across-surfaces` | 6 | 13 | `181/61/127` | 377 | `stress-test-volatility-ensemble-cohort-direction-reverse-connector-count-13` |

| Rank | Width | Connector | Direction | Surfaces retained/opposite/neutral | Windows retained/opposite/neutral | Volatility | Status |
|---:|---:|---|---|---|---|---:|---|
| 1 | 5 | `04700` | `reverse` | `5/1/0` | `30/6/18` | 106 | `connector-direction-retained-across-ensemble-surfaces` |
| 2 | 6 | `000122` | `forward` | `5/1/0` | `20/12/22` | 112 | `connector-direction-retained-across-ensemble-surfaces` |
| 3 | 5 | `00022` | `forward` | `5/1/0` | `25/13/16` | 113 | `connector-direction-retained-across-ensemble-surfaces` |
| 4 | 5 | `04900` | `reverse` | `5/1/0` | `20/15/19` | 115 | `connector-direction-retained-across-ensemble-surfaces` |
| 5 | 5 | `53191` | `forward` | `4/0/2` | `20/13/21` | 63 | `connector-direction-retained-across-ensemble-surfaces` |
| 6 | 7 | `0050020` | `reverse` | `4/2/0` | `23/12/19` | 212 | `connector-direction-retained-across-ensemble-surfaces` |
| 7 | 6 | `003727` | `reverse` | `4/2/0` | `22/14/18` | 214 | `connector-direction-retained-across-ensemble-surfaces` |
| 8 | 6 | `276061` | `reverse` | `4/2/0` | `17/14/23` | 214 | `connector-direction-retained-across-ensemble-surfaces` |
| 9 | 9 | `900020000` | `forward` | `3/2/1` | `18/15/21` | 240 | `connector-direction-mixed-across-ensemble-surfaces` |
| 10 | 5 | `02510` | `reverse` | `3/2/1` | `23/17/14` | 242 | `connector-direction-mixed-across-ensemble-surfaces` |

### Pair-Family Gap Cohort Volatility/Ensemble Stress

This block stress-tests the selected volatility/ensemble cohort on fresh generated gap and size surfaces. A retained result routes to ensemble anatomy and correction-bound analysis; a split or collapse remains empirical falsification, not a residue theorem, connector law, density mechanism, or Lean theorem candidate.

- Source direction `forward`; connectors `14`; source surfaces `6`; target `stress-test-volatility-ensemble-cohort-direction-forward-connector-count-14`
- Fresh surfaces `["fresh-volatility-generated-gap-band", "fresh-volatility-generated-gap8-size-band"]`; pair families `6`; connector-surface rows `28`; family-window rows `252`
- Retained/mixed/split/collapsed surfaces `0/0/2/0`; retained/opposite/neutral connectors `13/12/3`; retained/opposite/neutral windows `86/102/64`
- Status `volatility-ensemble-split-on-fresh-surfaces`; decision `volatility-ensemble-fresh-stress-split-record-falsification`; target `select-new-cohort-invariant-surface-after-volatility-ensemble-fresh-split`

| Surface | Status | Connectors retained/opposite/neutral | Windows retained/opposite/neutral | Absolute delta |
|---|---|---|---|---:|
| `fresh-volatility-generated-gap-band` | `volatility-ensemble-split-fresh-surface` | `7/6/1` | `44/45/37` | 131 |
| `fresh-volatility-generated-gap8-size-band` | `volatility-ensemble-split-fresh-surface` | `6/6/2` | `42/57/27` | 144 |

| Surface | Width | Connector | Direction | Windows reverse/forward/neutral | Absolute delta | Status |
|---|---:|---|---|---|---:|---|
| `fresh-volatility-generated-gap-band` | 5 | `00022` | `forward` | `2/5/2` | 10 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 5 | `02510` | `none` | `3/3/3` | 8 | `neutral-or-split-sign-surface` |
| `fresh-volatility-generated-gap-band` | 5 | `04700` | `reverse` | `4/3/2` | 10 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 5 | `04900` | `reverse` | `5/1/3` | 12 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 5 | `53191` | `reverse` | `6/2/1` | 12 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 5 | `91736` | `forward` | `1/5/3` | 8 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 6 | `000122` | `reverse` | `5/2/2` | 9 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 6 | `001139` | `forward` | `3/4/2` | 9 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 6 | `003727` | `forward` | `1/6/2` | 10 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 6 | `276061` | `forward` | `3/4/2` | 15 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 7 | `0050020` | `reverse` | `4/1/4` | 8 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 7 | `0286717` | `reverse` | `3/1/5` | 4 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 9 | `000550550` | `forward` | `2/3/4` | 6 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap-band` | 9 | `900020000` | `forward` | `3/4/2` | 10 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 5 | `00022` | `reverse` | `7/2/0` | 12 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 5 | `02510` | `forward` | `4/5/0` | 15 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 5 | `04700` | `reverse` | `8/0/1` | 16 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 5 | `04900` | `forward` | `3/5/1` | 11 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 5 | `53191` | `forward` | `2/4/3` | 9 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 5 | `91736` | `reverse` | `7/1/1` | 10 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 6 | `000122` | `forward` | `1/5/3` | 10 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 6 | `001139` | `reverse` | `4/0/5` | 5 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 6 | `003727` | `none` | `4/4/1` | 10 | `neutral-or-split-sign-surface` |
| `fresh-volatility-generated-gap8-size-band` | 6 | `276061` | `forward` | `2/5/2` | 9 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 7 | `0050020` | `reverse` | `7/0/2` | 9 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 7 | `0286717` | `forward` | `2/6/1` | 10 | `forward-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 9 | `000550550` | `reverse` | `3/2/4` | 7 | `reverse-sign-persistent-surface` |
| `fresh-volatility-generated-gap8-size-band` | 9 | `900020000` | `none` | `3/3/3` | 11 | `neutral-or-split-sign-surface` |

### Pair-Family Gap Cohort Surface-Family Contrast Picker

After the volatility/ensemble fresh split, this picker asks whether the selected direction failed uniformly or split by surface family. It groups the selected-ensemble evidence into gap-family and size-family surfaces before any residue profiling or Lean theorem work.

- Source status `volatility-ensemble-split-on-fresh-surfaces`; direction `forward`; connectors `14`; surfaces `8`; connector-surface rows `112`
- Families retained/split/mixed `1/1/0`; contrast `surface-family-directional-contrast-found`; decision `surface-family-contrast-picker-selected-directional-gap-size-contrast`; target `stress-test-surface-family-contrast-gap-family-forward-vs-size-family-opposite`

| Surface family | Status | Surfaces | Connectors retained/opposite/neutral | Windows retained/opposite/neutral | Absolute delta |
|---|---|---|---|---|---:|
| `gap-family` | `selected-direction-retained-surface-family` | `["fresh-generated-gap-band", "fresh-volatility-generated-gap-band", "gap-band", "heldout-gap-band"]` | `26/24/6` | `165/162/177` | 496 |
| `size-family` | `opposite-direction-split-surface-family` | `["fresh-generated-gap8-size-band", "fresh-volatility-generated-gap8-size-band", "generated-gap8-size-band", "size-band"]` | `24/29/3` | `154/204/146` | 548 |

| Surface | Family | Status | Connectors retained/opposite/neutral | Windows retained/opposite/neutral | Absolute delta |
|---|---|---|---|---|---:|
| `fresh-generated-gap-band` | `gap-family` | `selected-direction-retained-surface` | `7/6/1` | `48/40/38` | 137 |
| `fresh-generated-gap8-size-band` | `size-family` | `opposite-direction-split-surface` | `5/9/0` | `31/52/43` | 131 |
| `fresh-volatility-generated-gap-band` | `gap-family` | `mixed-surface-family-contrast` | `7/6/1` | `44/45/37` | 131 |
| `fresh-volatility-generated-gap8-size-band` | `size-family` | `mixed-surface-family-contrast` | `6/6/2` | `42/57/27` | 144 |
| `gap-band` | `gap-family` | `selected-direction-retained-surface` | `7/4/3` | `43/36/47` | 122 |
| `generated-gap8-size-band` | `size-family` | `opposite-direction-split-surface` | `6/7/1` | `41/51/34` | 144 |
| `heldout-gap-band` | `gap-family` | `opposite-direction-split-surface` | `5/8/1` | `30/41/55` | 106 |
| `size-band` | `size-family` | `mixed-surface-family-contrast` | `7/7/0` | `40/44/42` | 129 |

### Pair-Family Gap Cohort Surface-Family Contrast Stress

This block directly stress-tests the selected gap-family versus size-family contrast on disjoint generated surfaces. A retained result routes to anatomy analysis; a partial split or collapse remains empirical falsification, not a residue theorem, connector law, density mechanism, or Lean theorem candidate.

- Source contrast `surface-family-directional-contrast-found`; direction `forward`; selected family `gap-family`; opposite family `size-family`; connectors `14`; source target `stress-test-surface-family-contrast-gap-family-forward-vs-size-family-opposite`
- Fresh surfaces `2`; pair families `6`; connector-surface rows `28`; family-window rows `252`
- Families retained/split/mixed `1/1/0`; surfaces retained/split/mixed `1/1/0`; retained/opposite/neutral windows `82/86/84`
- Status `surface-family-contrast-retained-on-fresh-surfaces`; decision `surface-family-contrast-fresh-stress-retained-anatomy-next`; target `analyze-surface-family-contrast-anatomy-gap-family-forward-vs-size-family-opposite`

| Surface family | Status | Surfaces | Connectors retained/opposite/neutral | Windows retained/opposite/neutral | Absolute delta |
|---|---|---|---|---|---:|
| `gap-family` | `selected-direction-retained-surface-family` | `["fresh-contrast-generated-gap-family"]` | `6/5/3` | `43/36/47` | 128 |
| `size-family` | `opposite-direction-split-surface-family` | `["fresh-contrast-generated-gap8-size-family"]` | `5/8/1` | `39/50/37` | 145 |

| Surface | Family | Status | Connectors retained/opposite/neutral | Windows retained/opposite/neutral | Absolute delta |
|---|---|---|---|---|---:|
| `fresh-contrast-generated-gap-family` | `gap-family` | `selected-direction-retained-surface` | `6/5/3` | `43/36/47` | 128 |
| `fresh-contrast-generated-gap8-size-family` | `size-family` | `opposite-direction-split-surface` | `5/8/1` | `39/50/37` | 145 |

### Pair-Family Gap Cohort Surface-Family Contrast Anatomy

This block decomposes the retained gap-family versus size-family contrast by connector. Driver scores are window-count margins, not density estimates; the purpose is to decide whether the retained contrast is concentrated in a few connectors or distributed across the ensemble before any residue profiling or Lean theorem work.

- Source status `surface-family-contrast-retained-on-fresh-surfaces`; direction `forward`; selected family `gap-family`; opposite family `size-family`; connectors `14`; source target `analyze-surface-family-contrast-anatomy-gap-family-forward-vs-size-family-opposite`
- Driver rows `14`; full/gap-only/size-only/neutral `4/2/4/4`; score total `44`; top score `7`; top share bp `1591`
- Concentration `distributed-full-driver-cohort`; decision `surface-family-contrast-anatomy-found-distributed-driver-cohort`; target `stress-test-surface-family-contrast-driver-cohort-distributed-gap-family-forward-vs-size-family-opposite`

| Rank | Width | Connector | Role | Gap dir | Size dir | Gap retained/opposite | Size retained/opposite | Score |
|---:|---:|---|---|---|---|---|---|---:|
| 1 | 7 | `0286717` | `full-gap-retained-size-opposed-driver` | `forward` | `reverse` | `5/0` | `1/3` | 7 |
| 2 | 5 | `04700` | `full-gap-retained-size-opposed-driver` | `forward` | `reverse` | `5/2` | `0/3` | 6 |
| 3 | 9 | `900020000` | `full-gap-retained-size-opposed-driver` | `forward` | `reverse` | `3/1` | `0/4` | 6 |
| 4 | 5 | `04900` | `full-gap-retained-size-opposed-driver` | `forward` | `reverse` | `4/3` | `4/5` | 2 |
| 5 | 5 | `91736` | `size-family-opposition-driver` | `none` | `reverse` | `3/3` | `1/6` | 5 |
| 6 | 6 | `001139` | `size-family-opposition-driver` | `reverse` | `reverse` | `2/4` | `2/7` | 5 |
| 7 | 7 | `0050020` | `size-family-opposition-driver` | `reverse` | `reverse` | `1/4` | `0/5` | 5 |
| 8 | 9 | `000550550` | `gap-family-retention-driver` | `forward` | `none` | `4/0` | `4/4` | 4 |
| 9 | 6 | `003727` | `gap-family-retention-driver` | `forward` | `forward` | `5/3` | `4/2` | 2 |
| 10 | 5 | `00022` | `size-family-opposition-driver` | `reverse` | `reverse` | `2/4` | `2/4` | 2 |

### Pair-Family Gap Cohort Surface-Family Driver-Cohort Stress

This block stress-tests the four distributed full contrast drivers on fresh disjoint generated gap and size surfaces. A split is recorded as holdout falsification of this driver-cohort contrast, not as a residue theorem, connector law, density mechanism, or Lean theorem candidate.

- Source status `distributed-full-driver-cohort`; direction `forward`; selected family `gap-family`; opposite family `size-family`; source target `stress-test-surface-family-contrast-driver-cohort-distributed-gap-family-forward-vs-size-family-opposite`
- Driver connectors `["width7:0286717", "width5:04700", "width9:900020000", "width5:04900"]`; fresh surfaces `2`; pair families `6`; connector-surface rows `8`; family-window rows `72`
- Families retained/split/mixed `0/2/0`; surfaces retained/split/mixed `0/2/0`; retained/opposite/neutral windows `22/34/16`
- Status `driver-cohort-contrast-split-on-fresh-surfaces`; decision `driver-cohort-contrast-fresh-stress-split-record-falsification`; target `select-new-cohort-invariant-surface-after-driver-cohort-contrast-fresh-split`

| Surface family | Status | Surfaces | Connectors retained/opposite/neutral | Windows retained/opposite/neutral | Absolute delta |
|---|---|---|---|---|---:|
| `gap-family` | `opposite-direction-split-surface-family` | `["fresh-driver-generated-gap-family"]` | `1/2/1` | `13/16/7` | 47 |
| `size-family` | `opposite-direction-split-surface-family` | `["fresh-driver-generated-gap8-size-family"]` | `1/2/1` | `9/18/9` | 43 |

| Surface | Family | Status | Connectors retained/opposite/neutral | Windows retained/opposite/neutral | Absolute delta |
|---|---|---|---|---|---:|
| `fresh-driver-generated-gap-family` | `gap-family` | `opposite-direction-split-surface` | `1/2/1` | `13/16/7` | 47 |
| `fresh-driver-generated-gap8-size-family` | `size-family` | `opposite-direction-split-surface` | `1/2/1` | `9/18/9` | 43 |

### Pair-Family Gap Cohort Surface-Family Matched Non-Driver Control Stress

This block matches each full contrast driver to a same-width non-driver control where possible, then tests those controls on the same fresh generated gap and size surfaces. It asks whether the driver-cohort split is specific or broad; either outcome remains empirical routing, not a residue theorem, connector law, density mechanism, or Lean theorem candidate.

- Source driver stress `driver-cohort-contrast-split-on-fresh-surfaces`; decision `driver-cohort-contrast-fresh-stress-split-record-falsification`; direction `forward`; source target `select-new-cohort-invariant-surface-after-driver-cohort-contrast-fresh-split`
- Driver connectors `["width7:0286717", "width5:04700", "width9:900020000", "width5:04900"]`; matched controls `["width7:0050020", "width5:91736", "width9:000550550", "width5:00022"]`; fresh surfaces `2`; pair families `6`; connector-surface rows `8`; family-window rows `72`
- Families retained/split/mixed `0/1/1`; surfaces retained/split/mixed `0/1/1`; retained/opposite/neutral windows `19/27/26`
- Status `matched-nondriver-control-split-on-fresh-surfaces`; decision `matched-nondriver-control-also-split-record-broad-surface-falsification`; target `select-new-cohort-invariant-surface-after-driver-and-nondriver-fresh-split`

| Surface family | Status | Surfaces | Connectors retained/opposite/neutral | Windows retained/opposite/neutral | Absolute delta |
|---|---|---|---|---|---:|
| `gap-family` | `opposite-direction-split-surface-family` | `["fresh-driver-generated-gap-family"]` | `1/2/1` | `7/11/18` | 30 |
| `size-family` | `mixed-surface-family-contrast` | `["fresh-driver-generated-gap8-size-family"]` | `2/2/0` | `12/16/8` | 44 |

| Surface | Family | Status | Connectors retained/opposite/neutral | Windows retained/opposite/neutral | Absolute delta |
|---|---|---|---|---|---:|
| `fresh-driver-generated-gap-family` | `gap-family` | `opposite-direction-split-surface` | `1/2/1` | `7/11/18` | 30 |
| `fresh-driver-generated-gap8-size-family` | `size-family` | `mixed-surface-family-contrast` | `2/2/0` | `12/16/8` | 44 |

### Pair-Family Gap Cohort Surface-Agnostic Ensemble Picker

This block drops the gap-family versus size-family contrast assumption and scans the frozen connector portfolio across twelve mixed surfaces. It is cohort-level empirical routing only: no residue theorem, connector law, density mechanism, or Lean theorem candidate is promoted here.

- Source status `matched-nondriver-control-split-on-fresh-surfaces`; source target `select-new-cohort-invariant-surface-after-driver-and-nondriver-fresh-split`
- Candidates `14`; stable connectors `4`; surfaces `12`; pair families `36`; windows `1512`
- Ensembles `2`; qualifying `0`; decision `surface-agnostic-ensemble-picker-found-no-stable-mixed-surface-cohort`; target `select-new-cohort-invariant-surface-after-surface-agnostic-ensemble-picker-collapse`
- Selected ensemble: `none`

| Rank | Direction | Connectors | Stable-surface total | Retained/opposite/neutral windows | Absolute delta | Status | Target |
|---:|---|---:|---:|---|---:|---|---|
| 1 | `reverse` | 2 | 20 | `116/31/69` | 225 | `surface-agnostic-ensemble-below-three-connector-threshold` | `do-not-promote-surface-agnostic-ensemble-direction-reverse-without-three-stable-connectors` |
| 2 | `forward` | 2 | 16 | `84/60/72` | 226 | `surface-agnostic-ensemble-below-three-connector-threshold` | `do-not-promote-surface-agnostic-ensemble-direction-forward-without-three-stable-connectors` |

| Rank | Connector | Direction | Surfaces R/F/N | Retained/opposite surfaces | Retained/opposite/neutral windows | Volatility | Status |
|---:|---|---|---|---|---|---:|---|
| 1 | `width7:0050020` | `reverse` | `10/2/0` | `10/2` | `55/14/39` | 214 | `stable-surface-agnostic-direction` |
| 2 | `width5:04700` | `reverse` | `10/2/0` | `10/2` | `61/17/30` | 217 | `stable-surface-agnostic-direction` |
| 3 | `width6:000122` | `forward` | `2/8/2` | `8/2` | `40/29/39` | 279 | `stable-surface-agnostic-direction` |
| 4 | `width5:53191` | `forward` | `2/8/2` | `8/2` | `44/31/33` | 281 | `stable-surface-agnostic-direction` |
| 5 | `width6:276061` | `reverse` | `7/5/0` | `7/5` | `32/35/41` | 535 | `mixed-surface-agnostic-direction` |
| 6 | `width5:04900` | `reverse` | `7/5/0` | `7/5` | `41/39/28` | 539 | `mixed-surface-agnostic-direction` |
| 7 | `width5:91736` | `reverse` | `6/4/2` | `6/4` | `41/33/34` | 483 | `mixed-surface-agnostic-direction` |
| 8 | `width6:003727` | `reverse` | `6/5/1` | `6/5` | `39/37/32` | 562 | `mixed-surface-agnostic-direction` |
| 9 | `width5:02510` | `forward` | `3/6/3` | `6/3` | `42/37/29` | 412 | `mixed-surface-agnostic-direction` |
| 10 | `width9:000550550` | `forward` | `4/6/2` | `6/4` | `39/32/37` | 482 | `mixed-surface-agnostic-direction` |
| 11 | `width5:00022` | `forward` | `5/6/1` | `6/5` | `41/37/30` | 562 | `mixed-surface-agnostic-direction` |
| 12 | `width6:001139` | `none` | `6/6/0` | `0/0` | `0/0/31` | 0 | `no-surface-agnostic-direction` |

### Branch Status Picker

This block stops collapsed branches before theorem growth. `live` means replicated nonblocked reverse-only evidence remains; `needs-independent-replication` means a singleton branch can be tested but is not theorem-ready; `collapsed` means the branch is not followed further in this stress artifact.

- Picker decision: `all-branches-collapsed-after-independent-mod3-guardrail`
- Branches: `21` total; live `0`; needs independent replication `0`; collapsed `21`
- Selected next branch: `none`

| Rank | Branch | Source | Edge | Width | Digit | Connector | Reverse-only pairs | Status | Reason | Target |
|---:|---|---|---|---:|---:|---|---:|---|---|---|
| 1 | `digit8-edge-classifier-family` | `digit8-edge-zoom/classifier-family-replication` | `mixed` | 0 | 8 | `digit8-edge-family` | 3 | `collapsed` | digit-8 classifier family collapsed in outside-ladder replication, and split-only follow-up also collapsed | `none-digit8-branch-stopped-after-second-stage-collapse` |
| 2 | `trailing-edge-width8-digit6-connector-00000006` | `edge-pair-replication` | `trailing` | 8 | 6 | `00000006` | 1 | `collapsed` | fresh independent twin-prime ladder is theorem-blocked by mod-3 null layer | `none-independent-mod3-guardrail-blocked-00000006-trailing-edge-width8-digit6` |
| 3 | `leading-edge-width7-digit9-connector-9000000` | `edge-pair-replication` | `leading` | 7 | 9 | `9000000` | 1 | `collapsed` | fresh independent twin-prime ladder is theorem-blocked by mod-3 null layer | `none-independent-mod3-guardrail-blocked-9000000-leading-edge-width7-digit9` |
| 4 | `trailing-edge-width7-digit9-connector-0000009` | `edge-pair-replication` | `trailing` | 7 | 9 | `0000009` | 1 | `collapsed` | fresh independent twin-prime ladder is theorem-blocked by mod-3 null layer | `none-independent-mod3-guardrail-blocked-0000009-trailing-edge-width7-digit9` |
| 5 | `leading-edge-width6-digit9-connector-900000` | `edge-pair-replication` | `leading` | 6 | 9 | `900000` | 1 | `collapsed` | fresh independent twin-prime ladder is theorem-blocked by mod-3 null layer | `none-independent-mod3-guardrail-blocked-900000-leading-edge-width6-digit9` |
| 6 | `leading-edge-width6-digit6-connector-600000` | `edge-pair-replication` | `leading` | 6 | 6 | `600000` | 1 | `collapsed` | fresh independent twin-prime ladder is theorem-blocked by mod-3 null layer | `none-independent-mod3-guardrail-blocked-600000-leading-edge-width6-digit6` |
| 7 | `trailing-edge-width6-digit6-connector-000006` | `edge-pair-replication` | `trailing` | 6 | 6 | `000006` | 1 | `collapsed` | fresh independent twin-prime ladder is theorem-blocked by mod-3 null layer | `none-independent-mod3-guardrail-blocked-000006-trailing-edge-width6-digit6` |
| 8 | `leading-edge-width5-digit6-connector-60000` | `edge-pair-replication` | `leading` | 5 | 6 | `60000` | 0 | `collapsed` | fresh independent twin-prime ladder is theorem-blocked by mod-3 null layer | `none-independent-mod3-guardrail-blocked-60000-leading-edge-width5-digit6` |
| 9 | `leading-edge-width5-digit9-connector-90000` | `edge-pair-replication` | `leading` | 5 | 9 | `90000` | 0 | `collapsed` | fresh independent twin-prime ladder is theorem-blocked by mod-3 null layer | `none-independent-mod3-guardrail-blocked-90000-leading-edge-width5-digit9` |
| 10 | `leading-edge-width7-digit6-connector-6000000` | `edge-pair-replication` | `leading` | 7 | 6 | `6000000` | 0 | `collapsed` | fresh independent twin-prime ladder is theorem-blocked by mod-3 null layer | `none-independent-mod3-guardrail-blocked-6000000-leading-edge-width7-digit6` |
| 11 | `leading-edge-width8-digit6-connector-60000000` | `edge-pair-replication` | `leading` | 8 | 6 | `60000000` | 0 | `collapsed` | fresh independent twin-prime ladder is theorem-blocked by mod-3 null layer | `none-independent-mod3-guardrail-blocked-60000000-leading-edge-width8-digit6` |
| 12 | `leading-edge-width8-digit9-connector-90000000` | `edge-pair-replication` | `leading` | 8 | 9 | `90000000` | 0 | `collapsed` | fresh independent twin-prime ladder is theorem-blocked by mod-3 null layer | `none-independent-mod3-guardrail-blocked-90000000-leading-edge-width8-digit9` |

### Control Summary

| Control family | Rows | Aligned | Selected peak rows | Both residue-admissible | Forward-only hits | Reverse-only hits | Both hits | Neither hits | Mod-3 exceptions | Theorem-blocked |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `pair-control` | 6 | 1 | 1 | 1 | 0 | 1 | 0 | 5 | 1 | 5 |
| `adjacent-width-control` | 3 | 2 | 1 | 3 | 0 | 2 | 0 | 1 | 3 | 0 |
| `leading-digit-control` | 9 | 3 | 1 | 6 | 0 | 3 | 0 | 6 | 3 | 0 |
| `position-control` | 6 | 1 | 1 | 6 | 0 | 1 | 0 | 5 | 6 | 0 |

### Control Rows

| Family | Pair | Width | Position | Digit | Connector | F residue | R residue | F hit | R hit | Ratio gap | Aligns | Class | Mod-3 class | Theorem-blocked | Residue deltas |
|---|---|---:|---:|---:|---|---|---|---|---|---:|---|---|---|---|---|
| `pair-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 9 | `900000` | true | true | false | true | -3.060141 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `pair-control` | Twin-prime ladder pair (5 ∘ 7) | 6 | 0 | 9 | `900000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `theorem-blocked-twin-pair-above-three-connector-multiple-of-3` | true | `mod3:0, mod9:0` |
| `pair-control` | Selected twin-prime target (11 ∘ 13) | 6 | 0 | 9 | `900000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `theorem-blocked-twin-pair-above-three-connector-multiple-of-3` | true | `mod3:0, mod9:0` |
| `pair-control` | Twin-prime ladder pair (17 ∘ 19) | 6 | 0 | 9 | `900000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `theorem-blocked-twin-pair-above-three-connector-multiple-of-3` | true | `mod3:0, mod9:0` |
| `pair-control` | Twin-prime ladder pair (29 ∘ 31) | 6 | 0 | 9 | `900000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `theorem-blocked-twin-pair-above-three-connector-multiple-of-3` | true | `mod3:0, mod9:0` |
| `pair-control` | Twin-prime ladder pair (41 ∘ 43) | 6 | 0 | 9 | `900000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `theorem-blocked-twin-pair-above-three-connector-multiple-of-3` | true | `mod3:0, mod9:0` |
| `adjacent-width-control` | Twin-prime ladder pair (3 ∘ 5) | 5 | 0 | 9 | `90000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `adjacent-width-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 9 | `900000` | true | true | false | true | -3.060141 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `adjacent-width-control` | Twin-prime ladder pair (3 ∘ 5) | 7 | 0 | 9 | `9000000` | true | true | false | true | -3.453938 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `leading-digit-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 1 | `100000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | false | `mod3:0, mod9:0` |
| `leading-digit-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 2 | `200000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | false | `mod3:0, mod9:0` |
| `leading-digit-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 3 | `300000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `leading-digit-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 4 | `400000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | false | `mod3:0, mod9:0` |
| `leading-digit-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 5 | `500000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | false | `mod3:0, mod9:0` |
| `leading-digit-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 6 | `600000` | true | true | false | true | -3.051216 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `leading-digit-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 7 | `700000` | false | false | false | false | 0.000000 | false | `neither-prime-hit` | `not-connector-multiple-of-3` | false | `mod3:0, mod9:0` |
| `leading-digit-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 8 | `800000` | true | true | false | true | -3.057217 | true | `reverse-only-prime-hit` | `not-connector-multiple-of-3` | false | `mod3:0, mod9:0` |
| `leading-digit-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 9 | `900000` | true | true | false | true | -3.060141 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `position-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 0 | 9 | `900000` | true | true | false | true | -3.060141 | true | `reverse-only-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `position-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 1 | 9 | `090000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `position-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 2 | 9 | `009000` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `position-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 3 | 9 | `000900` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `position-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 4 | 9 | `000090` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |
| `position-control` | Twin-prime ladder pair (3 ∘ 5) | 6 | 5 | 9 | `000009` | true | true | false | false | 0.000000 | false | `neither-prime-hit` | `exceptional-small-twin-pair-mod3-lane` | false | `mod3:0, mod9:0` |

## Pair Summaries

| Pair | Ladder index | Median gap rank | Positive bounds | Negative bounds | Sign-stable | Signed gap range | Median absolute gap | Localization statuses | Top connectors | Interpretation |
|---|---:|---:|---:|---:|---|---:|---:|---|---|---|
| Twin-prime ladder pair (3 ∘ 5) | 1 | 1 | 0 | 9 | true | [-1.032743, -0.920198] | 0.967885 | `diffuse-aligned-position-digit-contrast` | `900000` | `width-gap-sign-stable-position-digit-localization-needs-review` |
| Twin-prime ladder pair (5 ∘ 7) | 2 | 3 | 9 | 0 | true | [0.825354, 0.934134] | 0.890115 | `diffuse-aligned-position-digit-contrast` | `200000` | `width-gap-sign-stable-position-digit-localization-needs-review` |
| Selected twin-prime target (11 ∘ 13) | 3 | 2 | 9 | 0 | true | [0.923140, 1.037663] | 0.959786 | `diffuse-mixed-sign-position-digit-contrast` | `500000` | `width-gap-sign-stable-but-position-digit-signal-diffuse-mixed-sign` |
| Twin-prime ladder pair (17 ∘ 19) | 4 | 4 | 0 | 9 | true | [-0.693514, -0.417573] | 0.597966 | `diffuse-aligned-position-digit-contrast` | `800000` | `width-gap-sign-stable-position-digit-localization-needs-review` |
| Twin-prime ladder pair (29 ∘ 31) | 5 | 5 | 9 | 0 | true | [0.569067, 0.674527] | 0.594934 | `diffuse-mixed-sign-position-digit-contrast` | `000400` | `width-gap-sign-stable-but-position-digit-signal-diffuse-mixed-sign` |
| Twin-prime ladder pair (41 ∘ 43) | 6 | 6 | 1 | 8 | false | [-0.106159, 0.054775] | 0.002643 | `diffuse-aligned-position-digit-contrast, diffuse-mixed-sign-position-digit-contrast` | `500000` | `width-gap-sign-not-stable-under-small-prime-bound-stress` |

## Selected Target Rows

| Bound | Small primes | Forward ratio | Reverse ratio | Ratio gap | Top connector | Top class | Aligns with width | Localization |
|---:|---|---:|---:|---:|---|---|---|---|
| 5 | `[2, 3, 5]` | 1.234131 | 0.310991 | 0.923140 | `500000` | `reverse-only-prime-hit` | false | `diffuse-mixed-sign-position-digit-contrast` |
| 7 | `[2, 3, 5, 7]` | 1.269343 | 0.309556 | 0.959786 | `500000` | `reverse-only-prime-hit` | false | `diffuse-mixed-sign-position-digit-contrast` |
| 11 | `[2, 3, 5, 7, 11]` | 1.282168 | 0.300821 | 0.981346 | `500000` | `reverse-only-prime-hit` | false | `diffuse-mixed-sign-position-digit-contrast` |
| 13 | `[2, 3, 5, 7, 11, 13]` | 1.331523 | 0.322087 | 1.009436 | `500000` | `reverse-only-prime-hit` | false | `diffuse-mixed-sign-position-digit-contrast` |
| 17 | `[2, 3, 5, 7, 11, 13, 17]` | 1.367168 | 0.329505 | 1.037663 | `500000` | `reverse-only-prime-hit` | false | `diffuse-mixed-sign-position-digit-contrast` |
| 19 | `[2, 3, 5, 7, 11, 13, 17, 19]` | 1.295212 | 0.341894 | 0.953318 | `500000` | `reverse-only-prime-hit` | false | `diffuse-mixed-sign-position-digit-contrast` |
| 23 | `[2, 3, 5, 7, 11, 13, 17, 19, 23]` | 1.297917 | 0.343384 | 0.954533 | `500000` | `reverse-only-prime-hit` | false | `diffuse-mixed-sign-position-digit-contrast` |
| 29 | `[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]` | 1.315823 | 0.331543 | 0.984279 | `500000` | `reverse-only-prime-hit` | false | `diffuse-mixed-sign-position-digit-contrast` |
| 31 | `[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]` | 1.273377 | 0.320848 | 0.952528 | `500000` | `reverse-only-prime-hit` | false | `diffuse-mixed-sign-position-digit-contrast` |

## All Stress Rows

| Pair | Bound | Forward hits | Reverse hits | Forward ratio | Reverse ratio | Ratio gap | Residue survivor deltas | Top connector | Localization |
|---|---:|---:|---:|---:|---:|---:|---|---|---|
| Twin-prime ladder pair (3 ∘ 5) | 5 | 0 | 7 | 0.000000 | 0.920198 | -0.920198 | `mod3:0, mod9:0` | `900000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (3 ∘ 5) | 7 | 0 | 7 | 0.000000 | 0.946550 | -0.946550 | `mod3:0, mod9:0` | `900000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (3 ∘ 5) | 11 | 0 | 7 | 0.000000 | 1.032743 | -1.032743 | `mod3:0, mod9:0` | `900000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (3 ∘ 5) | 13 | 0 | 7 | 0.000000 | 0.993062 | -0.993062 | `mod3:0, mod9:0` | `900000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (3 ∘ 5) | 17 | 0 | 7 | 0.000000 | 0.934646 | -0.934646 | `mod3:0, mod9:0` | `900000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (3 ∘ 5) | 19 | 0 | 7 | 0.000000 | 0.965831 | -0.965831 | `mod3:0, mod9:0` | `900000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (3 ∘ 5) | 23 | 0 | 7 | 0.000000 | 0.967885 | -0.967885 | `mod3:0, mod9:0` | `900000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (3 ∘ 5) | 29 | 0 | 7 | 0.000000 | 0.981297 | -0.981297 | `mod3:0, mod9:0` | `900000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (3 ∘ 5) | 31 | 0 | 7 | 0.000000 | 0.999693 | -0.999693 | `mod3:0, mod9:0` | `900000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (5 ∘ 7) | 5 | 7 | 0 | 0.920023 | 0.000000 | 0.920023 | `mod3:0, mod9:0` | `200000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (5 ∘ 7) | 7 | 7 | 0 | 0.915900 | 0.000000 | 0.915900 | `mod3:0, mod9:0` | `200000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (5 ∘ 7) | 11 | 7 | 0 | 0.890115 | 0.000000 | 0.890115 | `mod3:0, mod9:0` | `200000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (5 ∘ 7) | 13 | 7 | 0 | 0.916268 | 0.000000 | 0.916268 | `mod3:0, mod9:0` | `200000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (5 ∘ 7) | 17 | 7 | 0 | 0.934134 | 0.000000 | 0.934134 | `mod3:0, mod9:0` | `200000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (5 ∘ 7) | 19 | 7 | 0 | 0.884969 | 0.000000 | 0.884969 | `mod3:0, mod9:0` | `200000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (5 ∘ 7) | 23 | 7 | 0 | 0.846492 | 0.000000 | 0.846492 | `mod3:0, mod9:0` | `200000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (5 ∘ 7) | 29 | 7 | 0 | 0.852866 | 0.000000 | 0.852866 | `mod3:0, mod9:0` | `200000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (5 ∘ 7) | 31 | 7 | 0 | 0.825354 | 0.000000 | 0.825354 | `mod3:0, mod9:0` | `200000` | `diffuse-aligned-position-digit-contrast` |
| Selected twin-prime target (11 ∘ 13) | 5 | 8 | 2 | 1.234131 | 0.310991 | 0.923140 | `mod3:0, mod9:0` | `500000` | `diffuse-mixed-sign-position-digit-contrast` |
| Selected twin-prime target (11 ∘ 13) | 7 | 8 | 2 | 1.269343 | 0.309556 | 0.959786 | `mod3:0, mod9:0` | `500000` | `diffuse-mixed-sign-position-digit-contrast` |
| Selected twin-prime target (11 ∘ 13) | 11 | 8 | 2 | 1.282168 | 0.300821 | 0.981346 | `mod3:0, mod9:0` | `500000` | `diffuse-mixed-sign-position-digit-contrast` |
| Selected twin-prime target (11 ∘ 13) | 13 | 8 | 2 | 1.331523 | 0.322087 | 1.009436 | `mod3:0, mod9:0` | `500000` | `diffuse-mixed-sign-position-digit-contrast` |
| Selected twin-prime target (11 ∘ 13) | 17 | 8 | 2 | 1.367168 | 0.329505 | 1.037663 | `mod3:0, mod9:0` | `500000` | `diffuse-mixed-sign-position-digit-contrast` |
| Selected twin-prime target (11 ∘ 13) | 19 | 8 | 2 | 1.295212 | 0.341894 | 0.953318 | `mod3:0, mod9:0` | `500000` | `diffuse-mixed-sign-position-digit-contrast` |
| Selected twin-prime target (11 ∘ 13) | 23 | 8 | 2 | 1.297917 | 0.343384 | 0.954533 | `mod3:0, mod9:0` | `500000` | `diffuse-mixed-sign-position-digit-contrast` |
| Selected twin-prime target (11 ∘ 13) | 29 | 8 | 2 | 1.315823 | 0.331543 | 0.984279 | `mod3:0, mod9:0` | `500000` | `diffuse-mixed-sign-position-digit-contrast` |
| Selected twin-prime target (11 ∘ 13) | 31 | 8 | 2 | 1.273377 | 0.320848 | 0.952528 | `mod3:0, mod9:0` | `500000` | `diffuse-mixed-sign-position-digit-contrast` |
| Twin-prime ladder pair (17 ∘ 19) | 5 | 9 | 12 | 1.417248 | 1.899506 | -0.482258 | `mod3:0, mod9:0` | `800000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (17 ∘ 19) | 7 | 9 | 12 | 1.410657 | 1.890726 | -0.480069 | `mod3:0, mod9:0` | `800000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (17 ∘ 19) | 11 | 9 | 12 | 1.419834 | 1.837407 | -0.417573 | `mod3:0, mod9:0` | `800000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (17 ∘ 19) | 13 | 9 | 12 | 1.359167 | 1.891763 | -0.532596 | `mod3:0, mod9:0` | `800000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (17 ∘ 19) | 17 | 9 | 12 | 1.328427 | 1.928880 | -0.600453 | `mod3:0, mod9:0` | `800000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (17 ∘ 19) | 19 | 9 | 12 | 1.308861 | 1.906827 | -0.597966 | `mod3:0, mod9:0` | `800000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (17 ∘ 19) | 23 | 9 | 12 | 1.304130 | 1.997645 | -0.693514 | `mod3:0, mod9:0` | `800000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (17 ∘ 19) | 29 | 9 | 12 | 1.259160 | 1.928760 | -0.669600 | `mod3:0, mod9:0` | `800000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (17 ∘ 19) | 31 | 9 | 12 | 1.218542 | 1.866542 | -0.648000 | `mod3:0, mod9:0` | `800000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (29 ∘ 31) | 5 | 7 | 3 | 1.129894 | 0.485718 | 0.644176 | `mod3:0, mod9:0` | `000400` | `diffuse-mixed-sign-position-digit-contrast` |
| Twin-prime ladder pair (29 ∘ 31) | 7 | 7 | 3 | 1.124710 | 0.499589 | 0.625121 | `mod3:0, mod9:0` | `000400` | `diffuse-mixed-sign-position-digit-contrast` |
| Twin-prime ladder pair (29 ∘ 31) | 11 | 7 | 3 | 1.092960 | 0.504628 | 0.588332 | `mod3:0, mod9:0` | `000400` | `diffuse-mixed-sign-position-digit-contrast` |
| Twin-prime ladder pair (29 ∘ 31) | 13 | 7 | 3 | 1.083612 | 0.503078 | 0.580534 | `mod3:0, mod9:0` | `000400` | `diffuse-mixed-sign-position-digit-contrast` |
| Twin-prime ladder pair (29 ∘ 31) | 17 | 7 | 3 | 1.059101 | 0.473485 | 0.585616 | `mod3:0, mod9:0` | `000400` | `diffuse-mixed-sign-position-digit-contrast` |
| Twin-prime ladder pair (29 ∘ 31) | 19 | 7 | 3 | 1.043499 | 0.448565 | 0.594934 | `mod3:0, mod9:0` | `000400` | `diffuse-mixed-sign-position-digit-contrast` |
| Twin-prime ladder pair (29 ∘ 31) | 23 | 7 | 3 | 0.998129 | 0.429062 | 0.569067 | `mod3:0, mod9:0` | `000400` | `diffuse-mixed-sign-position-digit-contrast` |
| Twin-prime ladder pair (29 ∘ 31) | 29 | 7 | 3 | 1.047524 | 0.414267 | 0.633257 | `mod3:0, mod9:0` | `000400` | `diffuse-mixed-sign-position-digit-contrast` |
| Twin-prime ladder pair (29 ∘ 31) | 31 | 7 | 3 | 1.110295 | 0.435768 | 0.674527 | `mod3:0, mod9:0` | `000400` | `diffuse-mixed-sign-position-digit-contrast` |
| Twin-prime ladder pair (41 ∘ 43) | 5 | 7 | 7 | 1.147807 | 1.150271 | -0.002465 | `mod3:0, mod9:0` | `500000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (41 ∘ 43) | 7 | 7 | 7 | 1.142510 | 1.144979 | -0.002469 | `mod3:0, mod9:0` | `500000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (41 ∘ 43) | 11 | 7 | 7 | 1.238367 | 1.290692 | -0.052326 | `mod3:0, mod9:0` | `500000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (41 ∘ 43) | 13 | 7 | 7 | 1.188835 | 1.294994 | -0.106159 | `mod3:0, mod9:0` | `500000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (41 ∘ 43) | 17 | 7 | 7 | 1.216207 | 1.274223 | -0.058016 | `mod3:0, mod9:0` | `500000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (41 ∘ 43) | 19 | 7 | 7 | 1.261934 | 1.207159 | 0.054775 | `mod3:0, mod9:0` | `500000` | `diffuse-mixed-sign-position-digit-contrast` |
| Twin-prime ladder pair (41 ∘ 43) | 23 | 7 | 7 | 1.207068 | 1.209663 | -0.002595 | `mod3:0, mod9:0` | `500000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (41 ∘ 43) | 29 | 7 | 7 | 1.223708 | 1.226351 | -0.002643 | `mod3:0, mod9:0` | `500000` | `diffuse-aligned-position-digit-contrast` |
| Twin-prime ladder pair (41 ∘ 43) | 31 | 7 | 7 | 1.184234 | 1.186792 | -0.002558 | `mod3:0, mod9:0` | `500000` | `diffuse-aligned-position-digit-contrast` |

Interpretation: a stable width-level gap with diffuse or mixed-sign position/digit localization is a replication target, not immediate evidence for a new exact connector feature.
