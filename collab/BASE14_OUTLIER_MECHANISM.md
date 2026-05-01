# Base-14 Outlier Mechanism Note

This note records the first local explanation tranche for the `M=2`
boundary-layer outliers.

Scope:
- base `14` only
- middle length `M=2`
- compare `best_k_at_M2` against `k=(0,0)`
- lock the active pairs and nearby dead controls from
  `/tmp/primes_base14_survivor_atlas/summary.json`

Primary rerun:

```bash
cargo run --release --example base14_survivor_atlas_report
cargo run --release --example base14_outlier_mechanism_report
```

Default artifact bundle:
- `/tmp/primes_base14_outlier_mechanism/report.md`
- `/tmp/primes_base14_outlier_mechanism/summary.json`

## Exact Facts

1. Admissible count is exactly the zero-mask count.
   The exact mask helper in
   [src/validation/bounded_k.rs](/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/src/validation/bounded_k.rs)
   records a candidate as admissible iff its divisibility mask is zero, so the
   zero-mask histogram count and admissible count are identical by definition.

2. Zero positive singleton relief does not force zero admissible delta.
   The appendix outliers below show that singleton marginal relief can stay flat
   or negative while the zero-mask count still rises. This is an overlap fact,
   not a heuristic.

3. Zero or negative admissible delta does not force zero prime anomaly.
   Base-14 pair `(D,B)` has admissible delta `-0.51pp` and still gains
   `+3.06pp` in prime hits. Base-14 pair `(3,1)` has admissible delta
   `-2.04pp` and still gains `+0.51pp`.

4. The `M=2` win admits an exact two-piece decomposition:

```text
delta prime-rate = admissible-set effect + prime-yield effect
```

This is an identity for the compared lanes, not an approximation.

## Empirical Base-14 Classifications

Current active pairs from the maintained atlas:
- `(3,1)` with best `k=(1,0)`
- `(9,B)` with best `k=(0,1)`
- `(D,5)` with best `k=(0,1)`
- `(D,B)` with best `k=(0,1)`

Current explanation labels from
[examples/base14_outlier_mechanism_report.rs](/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/examples/base14_outlier_mechanism_report.rs):
- `yield_dominated`: `(3,1)`, `(D,5)`, `(D,B)`
- `mixed`: `(9,B)`
- `overlap_lift`: none in the first base-14 tranche

Most important stress case:
- `(D,B)` stays positive with anomaly `+3.06pp`, admissible delta `-0.51pp`,
  zero positive singleton relief, and `top_moduli_m2 = none`.
- That makes it the clearest current evidence that the base-14 lane cannot be
  explained only by singleton marginal relief counts.

## Exact vs Open

Exact / settled inside this tranche:
- admissible count equals zero-mask count
- candidate-transfer categories are exact partitions by middle index
- the two-piece prime-rate decomposition is exact
- the report reproduces the four current base-14 active pairs and their rank-1
  nearby dead controls from the maintained atlas artifact

Empirical / still open:
- why `(D,B)` gains so much prime yield on the shared-admissible lane
- whether the base-14 classifications persist under wider `k` grids or longer
  prefilter lists
- whether the mixed vs yield-dominated split has a clean residue-neighborhood
  rule

The current theorem-shaped takeaway is modest:
base-14 `M=2` survivors do not form one mechanism class. At least one pair is a
mixed admissible-plus-yield case, while the strongest stress case is
yield-dominated even after singleton marginal relief goes flat.

## Appendix: Other Zero-Positive-Signature Outliers

These rows use the same decomposition metrics but no full visuals.

| Base | Pair | Best k at M=2 | Label | Anomaly | Admissible delta | Yield delta | Net relief |
|---:|---|---|---|---:|---:|---:|---:|
| 6 | `(5,5)` | `k=(1,0)` | `overlap_lift` | `+8.33pp` | `+2.78pp` | `+14.71pp` | `-2.78pp` |
| 10 | `(1,7)` | `k=(1,0)` | `overlap_lift` | `+5.00pp` | `+3.00pp` | `+10.04pp` | `+0.00pp` |
| 12 | `(B,1)` | `k=(0,1)` | `overlap_lift` | `+0.69pp` | `+0.69pp` | `+0.96pp` | `-0.69pp` |

These appendix rows are the cleanest current evidence that overlap structure can
raise the zero-mask count without showing up as positive singleton marginal
relief.
