# Proof-Carrying Atlas

This directory holds deterministic metadata for the maintained
matched-control atlas. The atlas links empirical Gate A family identities to
generated Lean lane constants and, where available, exact arithmetic facts about
forbidden seed classes.

The canonical v1 atlas artifacts are:

```bash
cargo run --bin export_matched_control_atlas_manifest -- --panel smoke --out docs/atlas/matched_control_smoke_atlas_manifest.json
cargo run --bin export_matched_control_residue_masks -- --panel smoke --prime-bound 31 --format theorem-queue --out docs/atlas/matched_control_theorem_queue.md
```

Regenerate or verify it through the maintained manifest drift-check wrapper:

```bash
./scripts/matched_control_atlas_manifest.sh regenerate
./scripts/matched_control_atlas_manifest.sh verify
```

For CI and pre-commit checks, use the bridge gate that verifies the generated
Lean family-lane catalog, generated smoke-profile certificate module, this
atlas manifest, the generated Lean `#check` surface for smoke-profile
proof-link declarations, and a golden canonical smoke residue-mask summary
including `pair_uncertified_count == 0` and the pinned
`top_theorem_candidate` row, plus a generated Lean `#check` surface for that
selected row:

```bash
./scripts/matched_control_atlas_bridge.sh verify
```

For local proof-build performance diagnosis, the same bridge can be timed as
one input to the proof-build observatory:

```bash
./scripts/matched_control_atlas_bridge.sh timing --repeat 3 --json-out /tmp/matched_control_atlas_timing.json
./scripts/proof_build_observatory.sh timing --repeat 3
```

These timing artifacts are cache-aware engineering data, not atlas evidence or
density claims.

The broader CI-oriented proof-catalog alias includes this bridge:

```bash
./scripts/ci_proof_catalog.sh
```

The smoke-profile certificate metadata that feeds the atlas can also be emitted
directly:

```bash
cargo run --bin export_matched_control_smoke_profile_certificates -- --format json
cargo run --bin export_matched_control_smoke_profile_certificates -- --format lean-candidates
cargo run --bin export_matched_control_smoke_profile_certificates -- --format lean-module
cargo run --bin export_matched_control_smoke_profile_certificates -- --format lean-checks
cargo run --bin export_matched_control_smoke_profile_certificates -- --format lean-silent-checks
```

The bridge gate uses the silent check format as the tracked
`PrimeArithmetic.Generated.MatchedControlSmokeProfileCertificateChecks` umbrella
module, with deterministic `MatchedControlSmokeProfileCertificateChecksShardNN`
imports, so Lake can cache the declaration-link check surface instead of
re-elaborating a temporary `#check` file on every run.

Exact local residue-mask scan queues can be exported without making a density
claim:

```bash
cargo run --bin export_matched_control_residue_masks -- --panel smoke --prime-bound 31 --out-dir /tmp/mc-residue-masks
cargo run --bin export_matched_control_residue_masks -- --panel smoke --prime-bound 31 --format theorem-queue
cargo run --bin export_matched_control_residue_masks -- --panel smoke --prime-bound 31 --format lean-candidate-checks
cargo run --bin export_matched_control_residue_masks -- --panel smoke --prime-bound 31 --format lean-candidate-silent-checks
```

The residue-mask scanner emits `matched-control-residue-masks-v4` JSON. In
addition to lane/modulus rows and one-modulus pair candidates, it includes
cross-modulus `pair_fingerprint_rows` with exact survivor-count products
serialized as decimal strings and optional pair-certificate links to exact
Lean separation/equal-survivor theorems. The summary block also reports
`pair_certified_count`, `pair_uncertified_count`, and `top_theorem_candidate` so
automation can assert proof-link coverage and select the current maintained
follow-on row without scanning every row. These rows rank local mask overlap and
displacement geometry; certified fingerprint rows also expose
`forbidden_residue_set_theorem` at top level as a convenience link mirroring the
nested pair-certificate metadata. The Markdown summary mirrors the same proof
surface with a top theorem-candidate block plus seed-mask, residue-set, and
equal-survivor theorem columns. The `lean-candidate-checks` format emits a
visible Lean file that imports and `#check`s the selected candidate's theorem
links; the bridge uses `lean-candidate-silent-checks` as the tracked
`PrimeArithmetic.Generated.MatchedControlResidueTopCandidateChecks` module so
Lake can cache the declaration-link check. The tracked
`matched_control_theorem_queue.md` artifact is generated from the same summary
field, giving research planning and CI the same candidate surface. These rows
do not claim that one same-modulus lane has more first-order survivors than
another.

The JSON manifest is the source of truth. It is intentionally timestamp-free so
it can be committed, diffed, and checked by automation. Rows with exact Lean
facts include theorem links and local modulus data. Rows without such facts are
kept in the manifest with `proof_status: "lane-generated-only"`.

The canonical smoke v1 manifest currently has proof metadata for every
maintained smoke row. That completeness is a catalog status only: each proof is
an exact local residue-profile annotation, not a density theorem.

This is not a density claim and not evidence for a residual prime mechanism. It
is a proof-carrying index that keeps the maintained empirical panel aligned with
the exact arithmetic currently formalized in Lean.
