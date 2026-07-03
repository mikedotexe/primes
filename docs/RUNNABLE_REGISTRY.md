# Runnable Registry

This registry classifies maintained runnable entrypoints by side-effect shape.
Use it before broad sweeps so report generation, tracked regeneration, and
interactive tools do not get mixed together.

## Classes

| Class | Meaning |
|---|---|
| `stdout-only` | Prints to stdout and writes only normal build/test artifacts. |
| `writes-/tmp` | Writes report bundles to `/tmp/primes_*` by default or with `--out-dir`. |
| `repo-artifact` | Intended to write a curated repo artifact when an output path is provided. |
| `regenerates-tracked` | Can rewrite tracked generated proof/data files. Use `verify` unless intentionally regenerating. |
| `interactive` | Requires a real terminal or long-running UI/server session. |
| `release/package` | Packaging, release, broad build matrix, or external tool install behavior. |
| `external-input` | Requires caller-provided CSV/JSON/input files or stdin. |

## Signal Spine

| Runnable | Class | Notes |
|---|---|---|
| `scripts/signal_spine.sh` | `repo-artifact` | Captures the maintained signal spine under `reports/signal-spine/<run-id>/`. |
| `scripts/signal_spine.sh witness-engine` | `repo-artifact` | Prime Witness Engine umbrella smoke: verifies the proof-carrying witness certificate bundle and runs seed-to-witness / large-witness report bundles under one group. |
| `scripts/signal_spine.sh singular-series` | `repo-artifact` | Non-default finite affine singular-profile scout for small-prime residue-weather residuals. |

## Scripts

| Runnable | Class | Notes |
|---|---|---|
| `scripts/quick-ci.sh` | `stdout-only` | Format, clippy, build, and tests. |
| `scripts/ci-preflight.sh` | `stdout-only` | Local CI-style preflight, including formal checks when configured. |
| `scripts/ci_witness_certificate.sh` | `stdout-only` | CI-facing alias for the proof-carrying witness certificate bundle gate. |
| `scripts/test_all_examples.sh` | `stdout-only` | Builds/runs examples with short timeouts; TUI runs are expected to timeout. |
| `scripts/verification_spine.sh check` | `stdout-only` | Checks Rust verification spine, Lean build, and Agda clean spine. |
| `scripts/verification_spine.sh regenerate` | `regenerates-tracked` | Regenerates maintained verification-spine artifacts. |
| `scripts/lean_generated_catalog.sh verify` | `stdout-only` | Verifies generated Lean window certificates against tracked files. |
| `scripts/lean_generated_catalog.sh regenerate` | `regenerates-tracked` | Rewrites tracked Lean generated window certificates. |
| `scripts/lean_bounded_k_catalog.sh verify` | `stdout-only` | Verifies generated Lean bounded-k witnesses. |
| `scripts/lean_bounded_k_catalog.sh regenerate` | `regenerates-tracked` | Rewrites tracked Lean bounded-k witnesses. |
| `scripts/agda_generated_catalog.sh verify` | `stdout-only` | Verifies generated Agda bounded-k catalog. |
| `scripts/agda_generated_catalog.sh regenerate` | `regenerates-tracked` | Rewrites tracked Agda generated catalog. |
| `scripts/lean_proof_carrying_witness_certificate.sh verify` | `stdout-only` | Verifies the generated proof-carrying witness Lean catalogs plus promoted policy-matrix witness modules against tracked JSON sources, then builds the catalog-check modules and teaching wrapper. The matrix checks are deterministic shards imported by an umbrella module, and the checks import every generated witness module and validate every theorem string. |
| `scripts/lean_proof_carrying_witness_certificate.sh regenerate` | `regenerates-tracked` | Rewrites the generated proof-carrying witness Lean catalogs, catalog-check modules, and promoted policy-matrix witness modules from tracked JSON sources. |
| `scripts/lean_proof_carrying_witness_certificate.sh timing --repeat 3 --json-out /tmp/witness_lean_timing.json` | `writes-/tmp` | Follows the generated witness Lean catalog verify path and reports raw repeated timings plus min/median/max summaries for each exporter and Lake target. |
| `scripts/matched_control_atlas_bridge.sh timing --repeat 3 --json-out /tmp/matched_control_atlas_timing.json` | `writes-/tmp` | Follows the matched-control atlas bridge verify path and reports raw repeated timings plus min/median/max summaries for each bridge substep. |
| `scripts/lean_umbrella_build_timing.sh timing --repeat 3 --json-out /tmp/lean_umbrella_timing.json` | `writes-/tmp` | Times selected Lean catalog and umbrella targets under warm-cache conditions by default; `--cold-lean` runs `lake clean` before each repeat for heavier local baselines. |
| `scripts/proof_build_observatory.sh timing --repeat 3` | `repo-artifact` | Builds a local proof-build performance atlas under `reports/proof-build-observatory/<run-id>/` from witness, matched-control, and umbrella Lean timing JSONs. Timing artifacts are cache-aware engineering data, not tracked benchmark claims. |
| `scripts/proof_carrying_witness.sh verify` | `stdout-only` | Verifies the tracked canonical proof-carrying witness certificate bundle, derived search-policy atlas, promoted policy-matrix certificate sources, and zero-unpromoted smoke policy-matrix replay coverage against regeneration. |
| `scripts/proof_carrying_witness.sh regenerate` | `regenerates-tracked` | Rewrites the tracked canonical proof-carrying witness certificate bundle, manifest, search-policy atlas, and promoted policy-matrix certificate sources. |
| `scripts/connector_signal_atlas.sh verify` | `stdout-only` | Verifies the deterministic connector signal atlas and width-6 stress artifacts against regeneration, then checks that every atlas/stress Lean proof-link declaration resolves. |
| `scripts/connector_signal_atlas.sh regenerate` | `regenerates-tracked` | Rewrites the tracked connector signal atlas and width-6 stress artifacts under `docs/connector/`. |
| `scripts/signal_catalog.sh verify` | `stdout-only` | Verifies the lightweight top-level signal catalog against regeneration and checks that every row points at an existing artifact and known drift gate; add `--deep --timeout-seconds <n>` to run row drift gates. |
| `scripts/signal_catalog.sh regenerate` | `regenerates-tracked` | Rewrites the tracked matched-control/witness/connector signal catalog under `docs/signal_catalog/`. |
| `scripts/fix-agda-imports.sh` | `regenerates-tracked` | Rewrites Agda imports and creates `.bak` files. |
| `agda-proofs/scripts/fix-agda-imports.sh` | `regenerates-tracked` | Same purpose scoped under `agda-proofs`. |
| `agda-proofs/scripts/verify-clean-spine.sh` | `stdout-only` | Type-checks maintained clean-local Agda spine. |
| `scripts/analyze_fingerprints.py` | `external-input` | Requires fingerprint CSV. |
| `scripts/plot_fingerprints.py` | `external-input` | Requires fingerprint CSV and writes plots. |
| `scripts/analyze_hl_features.py` | `external-input` | Requires HL feature CSV and writes plots. |
| `scripts/compare_connector_patterns.py` | `external-input` | Requires fingerprint CSV and writes plots. |
| `scripts/plot_chaos_threshold_translation.py` | `writes-/tmp` | Reads chaos-threshold CSV artifacts and writes matplotlib report bundle. |
| `scripts/gen_formula.rb` | `stdout-only` | Formula helper. |
| `scripts/matrix.sh` | `release/package` | Broad feature/target matrix. |
| `scripts/build-quick-check.sh` | `release/package` | Broad release-oriented build checks. |
| `scripts/build-everything.sh` | `release/package` | Broad build sweep. |
| `scripts/build-wasm-tui.sh` | `release/package` | Can install/use `wasm-pack`, writes `pkg/`, starts web server. |
| `scripts/comprehensive-build.sh` | `release/package` | Release build and packaging audit. |
| `scripts/package-wasm.sh` | `release/package` | Creates WASM export package. |
| `scripts/release-smoke.sh` | `release/package` | Release smoke checks. |
| `scripts/release-package.sh` | `release/package` | Release package creation. |
| `scripts/release-package-lite.sh` | `release/package` | Lite release package creation. |
| `scripts/release-package-complete.sh` | `release/package` | Full release package creation. |
| `scripts/demo_for_researchers.sh` | `interactive` | Human-facing demo flow with pauses/long commands. |
| `scripts/generate_elbow_animation.sh` | `external-input` | Requires local CSV artifacts; writes animation/storyboard outputs. |
| `scripts/render_base15_elbow.sh` | `external-input` | Requires elbow artifacts. |
| `scripts/verify_optimizations.sh` | `release/package` | Heavy performance/optimization checks. |

## Binaries

| Runnable | Class | Notes |
|---|---|---|
| `cargo run --bin verify_verification_spine -- check` | `stdout-only` | Verification spine checker. |
| `cargo run --bin verify_verification_spine -- regenerate` | `regenerates-tracked` | Rewrites maintained generated artifacts. |
| `cargo run --bin export_window_certificate -- ... --out <path>` | `regenerates-tracked` | Output must live under `lean-proofs/PrimeArithmetic/Generated/`. |
| `cargo run --bin export_bounded_k_transfer_witness -- ... --out <path>` | `regenerates-tracked` | Output must live under Lean generated directory. |
| `cargo run --bin export_bounded_k_profile_witness -- ... --out <path>` | `regenerates-tracked` | Output must live under Lean generated directory. |
| `cargo run --bin export_bounded_k_transfer_agda_summary -- --out <path>` | `regenerates-tracked` | Output must live under Agda generated directory. |
| `cargo run --bin membrane-prime-fast -- ...` | `repo-artifact` | Maintained deterministic `u64` affine membrane prime family generator; optional JSON/CSV exports. |
| `cargo run --bin seed-to-witness -- [--seed <n>]` | `stdout-only` / `repo-artifact` | Prime Witness Engine demo entrypoint: seed origin to large readable probable-prime witness transcript; defaults to current epoch nanoseconds when `--seed` is omitted; optional JSON/Markdown and proof-carrying certificate exports. |
| `cargo run --bin verify-proof-carrying-witness -- <certificate.json>` | `stdout-only` | Independently verifies a proof-carrying witness certificate's affine construction and residue rows without rerunning witness search; optional verification JSON export. |
| `cargo run --bin export_proof_carrying_witness_bundle -- --out-dir docs/witness` | `regenerates-tracked` | Rewrites the canonical multi-witness certificate bundle and `witness_certificate_manifest.json`. |
| `cargo run --bin export_proof_carrying_witness_lean_certificate -- --catalog --certificate-dir docs/witness --out-dir lean-proofs/PrimeArithmetic/Generated/Witness --manifest-out docs/witness/witness_lean_catalog_manifest.json` | `regenerates-tracked` | Rewrites the generated Lean arithmetic catalog and Lean catalog manifest for the canonical proof-carrying witness certificate bundle. |
| `cargo run --bin export_proof_carrying_witness_lean_certificate -- --policy-matrix-catalog --certificate-dir docs/witness/policy_matrix --out-dir lean-proofs/PrimeArithmetic/Generated/Witness --manifest-out docs/witness/witness_policy_matrix_lean_catalog_manifest.json` | `regenerates-tracked` | Rewrites the generated Lean replay modules and Lean catalog manifest for promoted proof-carrying witness policy-matrix rows. |
| `cargo run --bin export_proof_carrying_witness_lean_catalog_checks -- --manifest docs/witness/witness_lean_catalog_manifest.json --out lean-proofs/PrimeArithmetic/Generated/Witness/CatalogChecks.lean --shard-size 1 --module-prefix PrimeArithmetic.Generated.Witness` | `regenerates-tracked` | Rewrites the tracked silent Lean declaration-check umbrella and deterministic shard files proving every theorem string in the witness Lean catalog manifest resolves. |
| `cargo run --bin export_proof_carrying_witness_lean_catalog_checks -- --manifest docs/witness/witness_policy_matrix_lean_catalog_manifest.json --out lean-proofs/PrimeArithmetic/Generated/Witness/MatrixCatalogChecks.lean --shard-size 3 --module-prefix PrimeArithmetic.Generated.Witness` | `regenerates-tracked` | Rewrites the tracked silent Lean declaration-check umbrella and deterministic shard files proving every theorem string in the promoted policy-matrix witness Lean catalog manifest resolves. |
| `cargo run --bin export_proof_carrying_witness_search_policy_atlas -- --certificate-dir docs/witness --out-dir docs/witness` | `regenerates-tracked` | Rewrites the deterministic witness search-policy atlas JSON and Markdown derived from the canonical proof-carrying witness bundle. |
| `cargo run --bin export_proof_carrying_witness_policy_matrix -- --out-dir /tmp/proof-carrying-witness-policy-matrix` | `writes-/tmp` | Runs the deterministic multi-lane witness policy matrix and emits certificate candidates, matrix JSON/Markdown, policy-matrix atlas JSON/Markdown, and an artifact manifest under the chosen output directory. |
| `cargo run --bin export_connector_signal_atlas -- --out-dir docs/connector` | `regenerates-tracked` | Rewrites the deterministic connector signal atlas JSON, Markdown, and artifact manifest. |
| `cargo run --bin export_connector_signal_atlas_checks -- --atlas docs/connector/connector_signal_atlas.json --out /tmp/ConnectorSignalAtlasChecks.lean` | `writes-/tmp` | Emits a Lean import-check surface for every proof-link module named by the connector signal atlas. |
| `cargo run --bin export_connector_width6_stress_checks -- --stress docs/connector/connector_width6_stress.json --out /tmp/ConnectorWidth6StressChecks.lean` | `writes-/tmp` | Emits a Lean declaration-check surface for theorem links named by the connector width-6 stress artifact. |
| `cargo run --bin export_signal_catalog -- --out-dir docs/signal_catalog` | `regenerates-tracked` | Rewrites the lightweight top-level signal catalog over matched-control, witness, and connector atlas artifacts. |
| `cargo run --bin verify_signal_catalog -- --catalog docs/signal_catalog/signal_catalog.json` | `stdout-only` | Shallow-verifies signal catalog row artifact paths and maintained drift-gate command strings without running the row gates. |
| `cargo run --bin verify_signal_catalog -- --catalog docs/signal_catalog/signal_catalog.json --deep --timeout-seconds 300 --json-out /tmp/signal_catalog_deep.json` | `writes-/tmp` | Runs every known signal-catalog row drift gate with a per-row timeout and writes machine-readable gate results. |
| `cargo run --features metal --bin membrane-prime-metal-fast -- ...` | `repo-artifact` | macOS/Metal affine transfer-collapse prototype; sends residue metadata rather than candidate values, optional JSON/CSV exports. |
| `cargo run --bin base57-affine-codec -- ...` | `stdout-only` | Baseline base58/base57 transcoding, base-invariant value maps, and framed affine base57 residue/prime notation encode/decode. |
| `cargo run --bin membrane-prime*` | `repo-artifact` | Legacy/experimental prime-search binaries; `membrane-prime` can write `lattice_watermark.png`. |
| `cargo run --bin orthogonal_landscape -- ...` | `repo-artifact` | Writes CSV when `--csv` is provided. |

## Examples

| Example | Class | Notes |
|---|---|---|
| `prime_count_smoke_test` | `stdout-only` | Deterministic sieve smoke test. |
| `prime_verification_report` | `stdout-only` | Maintained prime anchors plus composite audit cases. |
| `verify_prime_checker` | `stdout-only` | Primality checker audit, including known composite `300700300703`. |
| `proper_membrane_generator` | `stdout-only` | Deterministic base-aware membrane witnesses. |
| `membrane_palindrome_probe` | `stdout-only` | Exact palindrome overlap probe. |
| `membrane_scaffold_probe` | `stdout-only` | Exact same-budget scaffold control probe. |
| `membrane_showcase` | `stdout-only` | Demonstration output. |
| `membrane_vs_random` | `stdout-only` / `repo-artifact` | Stdout by default; `--json-out`/`--csv-out` writes artifacts. |
| `membrane_vs_random_fast` | `stdout-only` | Fast exploratory matched-control sketch. |
| `membrane_vs_random_compare` | `external-input` / `repo-artifact` | Requires two JSON exports; optional diff JSON. |
| `connector_signal_report` | `stdout-only` / `repo-artifact` | Optional JSON/CSV exports. |
| `comparative_signal_report` | `stdout-only` / `repo-artifact` | Optional JSON/CSV exports. |
| `connector_utility_demo` | `stdout-only` | Connector API demo. |
| `scan_connectors` | `stdout-only` | Connector scan CLI. |
| `cross_base_invariance_report` | `writes-/tmp` | Writes report bundle by default. |
| `m_boundary_layer_report` | `writes-/tmp` | Writes report bundle by default. |
| `m2_m3_transition_report` | `writes-/tmp` | Writes report bundle by default. |
| `m2_m3_transfer_collapse_report` | `writes-/tmp` | Writes report bundle by default. |
| `m2_survivor_autopsy_report` | `writes-/tmp` | Writes report bundle by default. |
| `m2_species_aggregation_report` | `writes-/tmp` | Writes report bundle by default. |
| `m2_species_geometry_report` | `external-input` / `writes-/tmp` | Requires species aggregation JSON unless provided. |
| `m2_relief_fingerprint_report` | `external-input` / `writes-/tmp` | Requires species aggregation JSON unless provided. |
| `m2_obstruction_signature_report` | `writes-/tmp` | Writes report bundle by default. |
| `m3_k_dominance_report` | `writes-/tmp` | Writes report bundle by default. |
| `m_transition_curve_report` | `writes-/tmp` | Writes report bundle by default. |
| `m_transition_phase_map_report` | `external-input` / `writes-/tmp` | Requires transition curve JSON unless provided. |
| `chaos_threshold_translation_report` | `writes-/tmp` | Writes report bundle by default. |
| `base14_survivor_atlas_report` | `writes-/tmp` | Writes report bundle by default. |
| `base14_outlier_mechanism_report` | `external-input` / `writes-/tmp` | Reads atlas artifact unless provided. |
| `base14_signal_clarity_report` | `external-input` / `writes-/tmp` | Reads mechanism artifact unless provided. |
| `base14_shared_yield_report` | `external-input` / `writes-/tmp` | Reads mechanism artifact unless provided. |
| `base14_shared_digit_structure_report` | `external-input` / `writes-/tmp` | Reads shared-yield artifact unless provided. |
| `two_p_family_report` | `writes-/tmp` | Writes report bundle by default. |
| `two_p_hinge_report` | `writes-/tmp` | Writes report bundle by default. |
| `two_p_hinge_mask_report` | `writes-/tmp` | Writes report bundle by default. |
| `two_p_hinge_discriminator_report` | `writes-/tmp` | Writes report bundle by default. |
| `two_p_hinge_atom_family_report` | `writes-/tmp` | Writes report bundle by default. |
| `two_p_hinge_robustness_report` | `writes-/tmp` | Writes report bundle by default. |
| `affine_hinge_classifier_report` | `writes-/tmp` | Writes report bundle by default. |
| `affine_gradient_transition_report` | `writes-/tmp` | Writes report bundle by default. |
| `affine_period_lock_report` | `writes-/tmp` | Writes report bundle by default. |
| `residue_torus_period_lock_report` | `writes-/tmp` | Writes visual-intuition residue torus bundle by default. |
| `membrane_prime_throughput_report` | `writes-/tmp` | Writes deterministic fast-generation throughput bundle by default. |
| `large_affine_witness_ladder_report` | `writes-/tmp` | Prime Witness Engine measurement entrypoint; writes large visible affine witness ladder bundle with confirmation tiers, local controls, OpenSSL calibration, primesieve scope rows, semantic rarity, and PNG panels. |
| `seed_to_witness_demo_report` | `writes-/tmp` | Prime Witness Engine transcript bundle; writes one-seed-to-large-witness demo output with CSV/JSON rows, canonical construction/residue certificate JSON, and copyable WolframAlpha/Mathematica/PARI/Sage verification snippets. |
| `timestamp_seed_policy_report` | `writes-/tmp` | Prime Witness Engine policy report; measures bounded timestamp-like seed-origin success rates and step quantiles for full-middle and 128-digit lanes. |
| `special_form_witness_comparison_report` | `writes-/tmp` | Prime Witness Engine special-form comparison report; places Mersenne-prime examples beside affine membrane witnesses with exact Mersenne classification. |
| `affine_singular_series_report` | `writes-/tmp` | Finite affine singular-profile scout comparing observed lane yield against PNT plus exact small-prime residue-weather expectation. |
| `construction_density_atlas_report` | `writes-/tmp` | Writes density-drift atlas bundle for maintained plus stress-test affine membrane families, including layered controls and visual panels. |
| `base30_wheel_compact_report` | `writes-/tmp` | Writes focused base-30 compact wheel bundle with all-pair heatmap, `(B,7)` residue funnel, and witness gallery. |
| `base30_reversal_asymmetry_report` | `writes-/tmp` | Writes compact base-30 ordered-pair reversal bundle with signed delta heatmap, residue-phase rows, and witness gallery. |
| `base30_reversal_residual_report` | `writes-/tmp` | Writes compact base-30 reversal residual bundle that separates raw delta, PNT size expectation, exact residue survival, and survivor-prime residual. |
| `affine_phase_residual_atlas_report` | `writes-/tmp` | Writes cross-base compact reversal atlas ranking local affine phase residual leads after size/PNT and residue-survivor accounting. |
| `shift_phase_signal_mining_report` | `writes-/tmp` | Writes curated shift-phase signal-mining bundle with mature lead follow-up, foils, residue gate profiles, witnesses, and Curt-ready visuals. |
| `unit_cycle_phase_signal_report` | `writes-/tmp` | Writes unit-cycle normalized phase-signal bundle with arc geometry buckets, lead ranking, M4 follow-up, foils, witnesses, and visuals. |
| `unit_cycle_base_neighbor_report` | `writes-/tmp` | Writes neighbor-base unit-cycle geometry bundle centered on bases `56..60`, including base57/base58 circle visuals, exact compact phase leads, and a payload-transcoding caution note. |
| `base57_affine_codec_report` | `writes-/tmp` | Writes base57 codec experiment bundle comparing ordinary radix transcoding with residue-filtered and prime-witness affine notation chunks. |
| `metal_affine_transfer_collapse_report` | `writes-/tmp` | Feature-gated macOS/Metal report for candidate-transfer collapse; use `--features metal`. |
| `metal_affine_benchmark_report` | `writes-/tmp` | Feature-gated local benchmark of Metal affine transfer-collapse against CPU affine and ordinary candidate baselines; includes repeated Metal dispatch setup timing, a beyond-`u64` BigUint probable-prime row, a source-grounded external comparison frame, and optional local CLI rows for tools such as OpenSSL and primesieve. |
| `prime_witness_engine_visual_atlas` | `writes-/tmp` | Visual atlas for affine membrane prime families: construction strip, affine line, residue gates, funnel, transfer collapse, geodesic residue path, singular-profile dashboard, and unit-cycle chords. |
| `affine_period_lock_species_report` | `writes-/tmp` | Writes report bundle by default. |
| `base22_gradient_pocket_report` | `writes-/tmp` | Writes report bundle by default. |
| `base10_persistence_species_report` | `writes-/tmp` | Writes report bundle by default. |
| `bounded_k_transfer_criterion_report` | `writes-/tmp` | Writes report bundle by default. |
| `base_hinge_probe_report` | `writes-/tmp` | Writes report bundle by default. |
| `base34_boundary_species_report` | `writes-/tmp` | Writes report bundle by default. |
| `comprehensive_base_analysis` | `stdout-only` | Systematic base analysis. |
| `solution_space_explorer` | `repo-artifact` | Writes `solution_space_complete.csv` in cwd. |
| `statistical_prime_factory` | `stdout-only` | Prime generator demo. |
| `statistical_prime_generator` | `stdout-only` | Prime generator demo. |
| `statistical_sampling_demo` | `stdout-only` | Sampling demo. |
| `belphegor_scanner` | `stdout-only` | Palindromic-prime scanner. |
| `sandwich_prime_finder` | `stdout-only` | Structured prime finder. |
| `check_prime` | `external-input` | Reads user input/stdin. |
| `prime_gap_analysis` | `stdout-only` | Gap analysis demo. |
| `hardy_littlewood_validation` | `stdout-only` | HL validation report. |
| `empirical_verification_pipeline` | `external-input` | Joins sample/model CSVs. |
| `babylonian_prime_orthogonality` | `stdout-only` | Orthogonality demo. |
| `orthogonality_verification` | `stdout-only` | Orthogonality checks. |
| `symmetry_breaking_explorer` | `stdout-only` | Symmetry-breaking exploration. |
| `harmonic_lagrange_explorer` | `stdout-only` | Harmonic Lagrange exploration. |
| `harmonic_overtones_explorer` | `stdout-only` | Overtone exploration. |
| `lagrange_clustering_verifier` | `stdout-only` | Lagrange clustering verifier. |
| `lagrange_full_verification` | `stdout-only` | Lagrange full-string verifier. |
| `lagrange_mechanics` | `stdout-only` | Lagrange mechanics demo. |
| `lagrange_verification` | `stdout-only` | Lagrange insertion verifier. |
| `lagrange_tui_demo` | `interactive` | Terminal UI; can export local CSV/MD from the UI. |
| `membrane_lab_tui` | `interactive` | Terminal UI. |
| `prime_atom_tui` | `interactive` | Terminal UI. |
