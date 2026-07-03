# Proof-Carrying Witness Artifacts

This directory holds deterministic witness certificates for the Prime Witness
Engine. These artifacts carry exact construction, residue-funnel evidence,
bounded search-replay rows, and small-prime rejection examples for named
witnesses; they are not primality proof certificates.

The canonical v1 bundle is indexed by:

```bash
docs/witness/witness_certificate_manifest.json
docs/witness/witness_lean_catalog_manifest.json
docs/witness/witness_policy_matrix_lean_catalog_manifest.json
docs/witness/witness_search_policy_atlas.json
docs/witness/witness_search_policy_atlas.md
docs/witness/policy_matrix/matrix_decimal_readable_22d_seed0.json
docs/witness/policy_matrix/matrix_decimal_classic_22d_seed0.json
docs/witness/policy_matrix/matrix_decimal_breathing_22d_seed0.json
docs/witness/policy_matrix/matrix_decimal_readable_64d_seed0.json
docs/witness/policy_matrix/matrix_decimal_readable_96d_seed0.json
docs/witness/policy_matrix/matrix_decimal_classic_64d_seed0.json
docs/witness/policy_matrix/matrix_decimal_breathing_64d_seed0.json
docs/witness/policy_matrix/matrix_decimal_classic_96d_seed0.json
docs/witness/policy_matrix/matrix_decimal_breathing_96d_seed0.json
docs/witness/policy_matrix/matrix_base30_wheel_64d_seed0.json
docs/witness/policy_matrix/matrix_base30_wheel_96d_seed0.json
docs/witness/policy_matrix/matrix_base6_compact_18d_seed0.json
docs/witness/policy_matrix/matrix_base12_compact_18d_seed0.json
docs/witness/policy_matrix/matrix_base6_compact_64d_seed0.json
docs/witness/policy_matrix/matrix_base6_compact_96d_seed0.json
docs/witness/policy_matrix/matrix_base12_compact_64d_seed0.json
docs/witness/policy_matrix/matrix_base12_compact_96d_seed0.json
docs/witness/policy_matrix/matrix_base30_wheel_18d_seed0.json
```

The maintained artifacts are:

```bash
docs/witness/seed60_proof_carrying_witness.json
docs/witness/teaching38_proof_carrying_witness.json
docs/witness/timestamp_policy_29d_trial0_proof_carrying_witness.json
```

Regenerate or verify the bundle with:

```bash
scripts/ci_witness_certificate.sh
scripts/proof_carrying_witness.sh regenerate
scripts/proof_carrying_witness.sh verify
scripts/lean_proof_carrying_witness_certificate.sh verify
scripts/lean_proof_carrying_witness_certificate.sh timing --repeat 3 --json-out /tmp/witness_lean_timing.json
scripts/proof_build_observatory.sh timing --repeat 3
cargo run --bin export_proof_carrying_witness_search_policy_atlas -- --certificate-dir docs/witness --out-dir docs/witness
cargo run --bin export_proof_carrying_witness_policy_matrix -- --out-dir /tmp/proof-carrying-witness-policy-matrix
```

The CI-facing gate also regenerates the canonical smoke policy matrix and fails
unless `unpromoted_replay_candidate_count` and
`atlas_only_large_candidate_count` are both zero. That keeps the proof-carrying
witness atlas in the same state asserted by the generated Lean catalog.

You can also verify any certificate directly without rerunning the witness
search:

```bash
cargo run --bin verify-proof-carrying-witness -- docs/witness/seed60_proof_carrying_witness.json
cargo run --bin verify-proof-carrying-witness -- docs/witness/teaching38_proof_carrying_witness.json
cargo run --bin verify-proof-carrying-witness -- docs/witness/timestamp_policy_29d_trial0_proof_carrying_witness.json
```

Each v1 certificate records the affine lane, witness seed, decimal value,
template digits, per-modulus residue checks, the bounded walk from input seed to
the first accepted residue survivor, nearby seeds rejected by exact small-prime
residue gates, Mersenne-shape classification, and the fixed Miller-Rabin
probable-prime method metadata. Its
`primality_proof_status` is intentionally
`probable-prime-not-proof-certified`.

The Lean-facing witness catalog is generated from all three certificate JSON
artifacts into `PrimeArithmetic/Generated/Witness/*.lean`, with
`witness_lean_catalog_manifest.json` mapping each JSON artifact to its generated
Lean module and theorem names. The maintained Lean witness gate also regenerates
the tracked `PrimeArithmetic/Generated/Witness/CatalogChecks.lean` silent
declaration-check umbrella from that manifest, fails on drift, and builds it so
stale theorem strings fail CI. The umbrella imports deterministic
`CatalogChecksShardNN.lean` files, one per canonical witness artifact. Each
generated module now contains a generic
`SearchReplayCertificate` object, replay soundness theorem, exact theorem that
the survivor list is the residue-survivor subset of the finite replay window,
a replay accounting theorem showing that residue-rejected rows and survivor rows
form a disjoint partition with counts matching the certificate metadata, and a
first-accepted-survivor theorem showing that pre-witness residue survivors are
explicitly non-accepted while the witness seed is the first accepted residue
survivor.
The wrapper `PrimeArithmetic/Witness/TeachingSeedCertificate.lean` is
intentionally kept only for the small teaching artifact, forwarding compact
human-facing theorem names for construction, residue-funnel survival, replay
soundness, exact survivor-list, first-accepted-survivor, and rejection-example
facts only.
The witness Lean gate writes generated Lean and catalog JSON artifacts
content-stably: unchanged rendered outputs are left untouched so Lake cache
mtimes remain valid across no-drift verification runs.

The search-policy atlas is derived from the same certificate bundle. It gives
automation and reviewers one compact surface for lane identity, visible digit
length, seed-origin policy, rejection geometry, residue-survivor counts,
first-accepted distance, and Lean replay theorem links. It remains a
search-policy/residue replay atlas, not a prime-density claim and not a
primality proof.

The policy-matrix exporter is the broader exploratory front door. It runs a
deterministic smoke matrix across the canonical decimal lane, decimal comparison
lanes, compact base-6/base-12/base-30 lanes, a complete 64-digit non-small
tranche, and a 96-digit tranche across the same six matrix lanes. It emits certificate candidates under the
chosen output directory and marks each row as curated-catalog generated Lean,
policy-matrix generated Lean, or atlas-only future work. Each run also emits
`witness_policy_matrix_atlas.json` and `.md`, summarizing the matrix by lane,
first-accepted distance, rejection geometry, Lean replay coverage, and a
structured next replay target. After promoting the atlas-selected
`matrix-decimal-classic-64d-seed0`,
`matrix-base30-wheel-64d-seed0`,
`matrix-decimal-breathing-64d-seed0`,
`matrix-base12-compact-64d-seed0`,
`matrix-base6-compact-64d-seed0`,
`matrix-decimal-readable-64d-seed0`, and
`matrix-decimal-classic-96d-seed0`, and
`matrix-base30-wheel-96d-seed0`, and
`matrix-base6-compact-96d-seed0`, and
`matrix-decimal-breathing-96d-seed0`, and
`matrix-decimal-readable-96d-seed0`, and
`matrix-base12-compact-96d-seed0` rows, the current smoke policy matrix has no
atlas-only large replay target.

The promoted policy-matrix tranche tracks eighteen matrix certificates under
`docs/witness/policy_matrix/`: the six small matrix rows, the complete six-row
64-digit matrix, and the complete six-row 96-digit matrix. They generate matching Lean replay modules under
`PrimeArithmetic/Generated/Witness/Matrix*`; large replay windows intentionally
expose compact aggregate replay theorems instead of per-seed convenience
wrappers. The atlas also includes `promoted_large_replay_geometry_rows` to
compare promoted large rows across decimal, base6, base12, and base30 lanes by
replay distance, survivor count, and rejection geometry without treating
probable-prime status as a proof. The modules are indexed by
`witness_policy_matrix_lean_catalog_manifest.json`. The same gate regenerates
and builds `PrimeArithmetic/Generated/Witness/MatrixCatalogChecks.lean`, so the
matrix theorem-link surface has the same machine-readable silent declaration
drift guard as the canonical witness catalog. The matrix check file is a small
umbrella over deterministic `MatrixCatalogChecksShardNN.lean` files, each generated from
fixed-size manifest chunks. The Lean gate builds the catalog-check modules
directly; those shard imports cover all generated witness modules and avoid both
the old monolithic check file and a redundant per-module Lake build loop. These rows remain
construction/search-replay certificates; large-output primality is still
recorded only as probable-prime metadata.

For performance work, `scripts/lean_proof_carrying_witness_certificate.sh
timing --repeat 3 --json-out /tmp/witness_lean_timing.json` follows the verify
path and reports raw per-run timings plus min/median/max summaries for each
exporter and Lake target. Treat those timings as local cache-aware measurements,
not deterministic tracked artifacts. For cross-gate proof-build diagnosis, use
`scripts/proof_build_observatory.sh timing --repeat 3`; it archives witness,
matched-control atlas, and umbrella Lean timings into one local atlas under
`reports/proof-build-observatory/`.
