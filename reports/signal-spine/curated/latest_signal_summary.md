# Signal Spine Summary

- Run id: `20260501T163258Z`
- Groups: `witness-engine`
- Commands: `4` total, `0` failed
- Total command duration: `17s`

## Steelman Reading

- Symmetric digit templates are strongest as affine seed-search surfaces: fixed layout gives `candidate = shift + gradient * seed`.
- Exact residue filters explain much of the generator's usefulness; observed lift over naive random baselines must still be checked against coprime, same-size controls.
- Affine lane signals such as period lock and gradient-only pockets are useful research lenses, not public density theorems.

## Command Status

- `PASS` `witness-engine/seed_to_witness_demo` (12s) -> `reports/signal-spine/20260501T163258Z/stdout/witness-engine__seed_to_witness_demo.log`
- `PASS` `witness-engine/large_affine_witness_ladder` (2s) -> `reports/signal-spine/20260501T163258Z/stdout/witness-engine__large_affine_witness_ladder.log`
- `PASS` `witness-engine/timestamp_seed_policy` (1s) -> `reports/signal-spine/20260501T163258Z/stdout/witness-engine__timestamp_seed_policy.log`
- `PASS` `witness-engine/special_form_witness_comparison` (2s) -> `reports/signal-spine/20260501T163258Z/stdout/witness-engine__special_form_witness_comparison.log`

## Key Extracts

### `witness-engine/seed_to_witness_demo/report.md`
- - One seed origin, one named construction family, one large readable probable-prime witness.
- - The seed is a start point, not a guarantee; above u64, the repo says probable-prime witness.
- - canonical: seed `60`, visible digits `128`
- - teaching row: seed `0`, visible digits `38`
- - `transcript.md`: human-facing seed-to-witness transcript.
- - `witness_rows.csv`: compact row export for the canonical and teaching witnesses.

### `witness-engine/large_affine_witness_ladder/report.md`
- - This engine generates large, human-readable prime witnesses by compiling symmetric digit constructions into affine search lanes with cheap residue filtering.
- - The comparison claim is not that this beats general-purpose prime generators; it targets a named readable construction family that ordinary tools do not preserve.
- - Above `u64`, `prime` in this report means fixed-base Miller-Rabin probable-prime witness unless explicitly labeled deterministic.
- - profile: `smoke`
- - visible digit ladder: `[22, 38]`
- - seed count per rung: `2000`

### `witness-engine/timestamp_seed_policy/report.md`
- - Treat a nanosecond timestamp as a seed origin, not as a guaranteed prime seed.
- - Walk forward on the default decimal affine membrane lane: `base=10, pair=(3,7), k=(2,1)`.
- - Apply exact small-prime residue filters before probable-prime confirmation.
- - Declare success only if a witness appears within the policy's max-step budget.
- - This is a bounded empirical statement, not a theorem and not a density claim.
- - profile: `smoke`

### `witness-engine/special_form_witness_comparison/report.md`
- - Both families turn a compact descriptor into a large prime-shaped witness; the affine rows do it through digit-template lanes and are explicitly not Mersenne.
- - This is a genre comparison, not an equivalence of proof methods, record-search maturity, or density theory.
- - rows: `8` total, `5` Mersenne rows, `3` affine rows
- - Mersenne rows are binary-repunit special forms: `p -> 2^p - 1`.
- - Affine rows are decimal membrane special forms: `seed origin -> A + G*s` inside a fixed visible lane.
- - The affine rows are marked `not_mersenne` by exact shape, while still preserving a compact descriptor and large witness output.

## Failures

- None.
