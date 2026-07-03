# Generated Lean Artifacts

This directory is the landing zone for runtime-exported Lean artifacts that
instantiate maintained proof shells.

Layout:

- `Examples/`: tracked sample artifacts that are kept in the repo and checked
  as part of the Lean package
- `MatchedControlFamilyLanes.lean`: tracked family-code lane constants exported
  from the Rust maintained matched-control catalog, with generated key lookup
  and duplicate-free key facts
- `MatchedControlSmokeProfileCertificates.lean`: tracked smoke-profile
  certificate declarations exported from Rust metadata, importing the
  hand-written zero-seed-class support layer
- `MatchedControlSmokeProfileCertificateChecks.lean`: tracked silent
  declaration-check umbrella for smoke-profile certificate metadata and pair
  proof-link strings
- `MatchedControlSmokeProfileCertificateChecksShardNN.lean`: deterministic
  shard modules imported by the smoke-profile declaration-check umbrella
- `MatchedControlResidueTopCandidateChecks.lean`: tracked silent
  declaration-check module for the current residue-mask top theorem candidate
- `Runtime/`: default output location for ad hoc exported artifacts generated
  from the Rust runtime side
- `Witness/`: tracked proof-carrying witness certificate arithmetic emitted
  from deterministic witness JSON artifacts

Current tracked examples:

- `Examples/WindowP3Base6Span5.lean`
- `Examples/WindowP5Base10Span5.lean`
- `Examples/WindowP5Base12Span17.lean`
- `Examples/WindowP11Base30Span5.lean`
- `Examples/WindowP101Base30Span29.lean`
- `Examples/WindowP163Base30Span35.lean`
- `Examples/WindowP41Base210Span5.lean`
- `MatchedControlFamilyLanes.lean`
- `MatchedControlSmokeProfileCertificates.lean`
- `MatchedControlSmokeProfileCertificateChecks.lean`
- `MatchedControlSmokeProfileCertificateChecksShard01.lean`
- `MatchedControlSmokeProfileCertificateChecksShard02.lean`
- `MatchedControlSmokeProfileCertificateChecksShard03.lean`
- `MatchedControlSmokeProfileCertificateChecksShard04.lean`
- `MatchedControlSmokeProfileCertificateChecksShard05.lean`
- `MatchedControlResidueTopCandidateChecks.lean`
- `Witness/CatalogChecks.lean`
- `Witness/CatalogChecksShard01.lean`
- `Witness/CatalogChecksShard02.lean`
- `Witness/CatalogChecksShard03.lean`
- `Witness/MatrixCatalogChecks.lean`
- `Witness/MatrixCatalogChecksShard01.lean`
- `Witness/MatrixCatalogChecksShard02.lean`
- `Witness/MatrixCatalogChecksShard03.lean`
- `Witness/MatrixCatalogChecksShard04.lean`
- `Witness/MatrixCatalogChecksShard05.lean`
- `Witness/MatrixCatalogChecksShard06.lean`
- `Witness/Seed60.lean`
- `Witness/Teaching38.lean`
- `Witness/TimestampPolicy29Trial0.lean`
- `Witness/MatrixDecimalReadable22.lean`
- `Witness/MatrixDecimalClassic22.lean`
- `Witness/MatrixDecimalBreathing22.lean`
- `Witness/MatrixDecimalReadable64.lean`
- `Witness/MatrixDecimalClassic64.lean`
- `Witness/MatrixDecimalBreathing64.lean`
- `Witness/MatrixBase30Wheel64.lean`
- `Witness/MatrixBase6Compact18.lean`
- `Witness/MatrixBase12Compact18.lean`
- `Witness/MatrixBase6Compact64.lean`
- `Witness/MatrixBase12Compact64.lean`
- `Witness/MatrixBase30Wheel18.lean`

Catalog workflow:

- `./scripts/lean_generated_catalog.sh verify`: regenerate each tracked example
  in place, compare it to a backup, and restore the tracked file on success
- `./scripts/lean_generated_catalog.sh regenerate`: rewrite the tracked example
  files in place from the exporter
- `./scripts/lean_matched_control_family_lanes.sh verify`: regenerate the
  matched-control lane catalog into a temporary file and fail on drift without
  touching the tracked Lean file during no-drift verifies
- `./scripts/lean_matched_control_smoke_profile_certificates.sh verify`:
  regenerate the tracked smoke-profile certificate module and fail on drift
  after building the generated Lean module. The atlas bridge calls this script
  with `--skip-build` because it immediately builds the cached
  `MatchedControlSmokeProfileCertificateChecks` module, which imports the same
  generated certificate module.
- `./scripts/lean_proof_carrying_witness_certificate.sh verify`: regenerate
  the tracked proof-carrying witness Lean catalogs from `docs/witness/*.json`
  and `docs/witness/policy_matrix/*.json`, fail on drift across the generated
  modules, manifests, and declaration-check surfaces, and build the generated
  modules plus the small theorem-facing teaching wrapper
- `./scripts/matched_control_atlas_bridge.sh verify`: verify the generated
  matched-control Lean lane catalog, generated smoke-profile certificates, and
  the proof-carrying atlas manifest as a single bridge gate, including the
  cached silent smoke-profile declaration-check umbrella plus deterministic
  shard modules, canonical smoke residue-mask pair proof coverage, the tracked
  residue-mask theorem queue, and the cached silent declaration-check module for
  the current residue-mask top theorem candidate
- `./scripts/ci_proof_catalog.sh`: run the CI-facing proof-catalog group,
  including the generated Lean/Agda catalogs and the matched-control atlas
  bridge

The canonical exporter is the Rust binary:

```bash
cargo run --bin export_window_certificate -- \
  --p 5 \
  --base 10 \
  --window-span 5 \
  --exclude-radius 1 \
  --out lean-proofs/PrimeArithmetic/Generated/Examples/WindowP5Base10Span5.lean
```

This writes a Lean module under `PrimeArithmetic/Generated/...` that:

- defines a concrete `GeneratedWindowPayload`
- derives support counts automatically
- supplies balanced-count, fixed-point-exclusion, and pointwise-safety evidence
- rebuilds a `GeneratedDualEvidence` bundle and the corresponding dual
  certificate

Check the resulting artifact locally with:

```bash
cd lean-proofs
lake env lean PrimeArithmetic/Generated/Examples/WindowP5Base10Span5.lean
```

Verify the whole tracked catalog from the repo root with:

```bash
./scripts/lean_generated_catalog.sh verify
```

The proof-carrying witness certificate catalog exporter is:

```bash
cargo run --bin export_proof_carrying_witness_lean_certificate -- \
  --catalog \
  --certificate-dir docs/witness \
  --out-dir lean-proofs/PrimeArithmetic/Generated/Witness \
  --manifest-out docs/witness/witness_lean_catalog_manifest.json
```

Those generated modules define each witness config, seed, residue-modulus list,
exact row facts, rejection examples, bounded search-replay residue facts, and
aggregate residue-funnel survival theorem. The companion
`docs/witness/witness_lean_catalog_manifest.json` maps
each JSON artifact to its generated Lean module and theorem names. The checker

```bash
cargo run --bin export_proof_carrying_witness_lean_catalog_checks -- \
  --manifest docs/witness/witness_lean_catalog_manifest.json \
  --out lean-proofs/PrimeArithmetic/Generated/Witness/CatalogChecks.lean \
  --shard-size 1 \
  --module-prefix PrimeArithmetic.Generated.Witness
```

emits the tracked `PrimeArithmetic.Generated.Witness.CatalogChecks` umbrella
plus deterministic `CatalogChecksShardNN` modules for those theorem strings;
the maintained `scripts/lean_proof_carrying_witness_certificate.sh verify` gate
regenerates them, fails on drift, and builds the umbrella target. The hand-written
theorem-facing wrapper is intentionally kept only for the small teaching
artifact at `PrimeArithmetic/Witness/TeachingSeedCertificate.lean`.

The same witness Lean gate also tracks the promoted policy-matrix replay
modules:

```bash
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalReadable22.lean
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalClassic22.lean
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalBreathing22.lean
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalReadable64.lean
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalClassic64.lean
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixDecimalBreathing64.lean
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase30Wheel64.lean
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase6Compact18.lean
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase12Compact18.lean
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase6Compact64.lean
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase12Compact64.lean
lean-proofs/PrimeArithmetic/Generated/Witness/MatrixBase30Wheel18.lean
```

Those modules are generated from stable source certificates under
`docs/witness/policy_matrix/`. Large replay-window modules intentionally expose
aggregate replay certificate/accounting/first-accepted theorem links without
per-seed replay convenience wrappers. They are indexed by
`docs/witness/witness_policy_matrix_lean_catalog_manifest.json`, and the
checker

```bash
cargo run --bin export_proof_carrying_witness_lean_catalog_checks -- \
  --manifest docs/witness/witness_policy_matrix_lean_catalog_manifest.json \
  --out lean-proofs/PrimeArithmetic/Generated/Witness/MatrixCatalogChecks.lean \
  --shard-size 3 \
  --module-prefix PrimeArithmetic.Generated.Witness
```

emits the tracked `PrimeArithmetic.Generated.Witness.MatrixCatalogChecks`
umbrella plus deterministic `MatrixCatalogChecksShardNN` modules, so every
matrix theorem link resolves in Lean without forcing the gate through one giant
declaration-check file.
