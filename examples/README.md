# Examples

All curated top-level examples compile and are maintained. Run any example with:

```bash
cargo run --example <name>
# or for performance-sensitive examples:
cargo run --release --example <name>
```

Report-style examples in this repo follow a shared artifact pattern:
stdout summary first, then optional machine-readable JSON/CSV exports for reruns
and follow-up analysis.

For the large readable witness path, start with
[`../docs/PRIME_WITNESS_ENGINE.md`](../docs/PRIME_WITNESS_ENGINE.md). The demo
entrypoint is `seed-to-witness`; the measurement entrypoint is
`large_affine_witness_ladder_report`.

## Quick Start (5 minutes)

```bash
cargo run --example prime_count_smoke_test      # Validate sieve against OEIS
cargo run --example proper_membrane_generator   # Generate membrane primes
cargo run --example connector_signal_report     # Connector report with density-aware residual audit
cargo run --example comparative_signal_report   # Midpoint vs connector comparative table
cargo run --release --example cross_base_invariance_report # Cross-base smoke scorecard; add --full for exhaustive pair catalog
cargo run --release --example m_boundary_layer_report      # Short-length boundary-layer report for M=1,2,3
cargo run --release --example m2_m3_transition_report      # Same-pair transition report across M=2 and M=3
cargo run --release --example m2_m3_transfer_collapse_report # Exact transfer-vocabulary report explaining why M=2 is meaningful and M=3 collapses
cargo run --release --example m2_survivor_autopsy_report   # Full-catalog autopsy for the pairs that survive to M=2
cargo run --release --example m2_species_aggregation_report # Species-level aggregation for m1_only / m1_to_m2 / m2_only
cargo run --release --example m2_species_geometry_report   # Pair-lattice and modulus-heatmap views for the anomaly species
cargo run --release --example m2_relief_fingerprint_report # Persistent vs emergent modulus-relief fingerprint at M=2
cargo run --release --example m2_obstruction_signature_report # Composite obstruction-signature view for M=2 survivors and nearest dead neighbors
cargo run --release --example m_transition_curve_report    # Same-pair transition curve across a short M range
cargo run --release --example m_transition_phase_map_report # Grouped phase-map view of the bounded-k transition lane
cargo run --release --example chaos_threshold_translation_report # Arithmetic translation of chaos/stability metaphors into maintained M-threshold language
cargo run --release --example base14_survivor_atlas_report # Focused base-14 atlas for the M=2 boundary layer and nearby dead neighbors
cargo run --release --example base14_outlier_mechanism_report # Base-14 mechanism decomposition for the four M=2 active pairs
cargo run --release --example base14_signal_clarity_report # Downstream signal-clarity pass for the base-14 mechanism lane
cargo run --release --example base14_shared_yield_report # Shared-admissible witness report for the base-14 mechanism lane
cargo run --release --example base14_shared_digit_structure_report # Digit-structure mining inside the shared-admissible witness lane
cargo run --release --example two_p_family_report # Direct 2p-family test against foil bases for M=2 persistence and shared-yield structure
cargo run --release --example two_p_hinge_report # Focused 2p hinge report explaining why base 14 stays strong while 22 and 26 weaken
cargo run --release --example two_p_hinge_mask_report # Cross-base hinge mask atlas explaining the hinge via exact k=(0,0)->best transfer structure
cargo run --release --example two_p_hinge_discriminator_report # Exact hinge discriminator search with a rule frontier and representative atlas
cargo run --release --example two_p_hinge_atom_family_report # Atom-family depth pass ranking which hinge atoms are deepest, bridging, or diagnostic
cargo run --release --example two_p_hinge_robustness_report # Robustness matrix pass testing whether the hinge family ladder survives data-surface and threshold-vocabulary pressure
cargo run --release --example affine_hinge_classifier_report # Exploratory affine atlas for local shift / gradient / zero-seed comparison on the hinge species surface
cargo run --release --example affine_gradient_transition_report # Focused affine follow-up centered on whether gradient_only truly disappears or winners simply avoid it
cargo run --release --example affine_period_lock_report # Exploratory period-lock pass for affine gradient agreement via multiplicative order and direct lane comparisons
cargo run --release --example residue_torus_period_lock_report # Visual-intuition residue torus walkthrough for period lock with computed construction witnesses
cargo run --release --example membrane_prime_throughput_report # Deterministic u64 affine generation throughput report for visible membrane lanes
cargo run --release --example large_affine_witness_ladder_report # Large visible affine witness ladder with BigUint/u128/u64 backend scope and fair comparison rows
cargo run --release --bin seed-to-witness # Current timestamp-ns seed origin to one large readable probable-prime witness transcript
cargo run --release --bin seed-to-witness -- --seed 60 # Canonical fixed-seed 128-digit witness transcript
cargo run --release --example seed_to_witness_demo_report # Report bundle for the seed-to-witness transcript demo
cargo run --release --example timestamp_seed_policy_report # Bounded empirical timestamp-origin policy measurement
cargo run --release --example special_form_witness_comparison_report # Mersenne-style special-form comparison for affine witnesses
scripts/signal_spine.sh witness-engine # Prime Witness Engine umbrella smoke
cargo run --release --example affine_singular_series_report # Finite singular-profile scout for residue-weather residual leads
scripts/signal_spine.sh singular-series # Non-default signal-spine group for the singular-profile scout
cargo run --release --example construction_density_atlas_report # Density-drift atlas across good, mediocre, and lousy affine membrane families
cargo run --release --example base30_wheel_compact_report # Focused base-30 compact wheel report with all-pair heatmap, residue funnel, and witness gallery
cargo run --release --example base30_reversal_asymmetry_report # Ordered-pair reversal asymmetry report for compact base-30 residue phases
cargo run --release --example base30_reversal_residual_report # Decomposes base-30 reversal deltas into size, residue survival, and survivor-prime residual layers
cargo run --release --example affine_phase_residual_atlas_report # Cross-base compact reversal atlas for coherent local affine phase effects
cargo run --release --example shift_phase_signal_mining_report # Curt-ready curated follow-up for shift-phase residual leads and foils
cargo run --release --example unit_cycle_phase_signal_report # Unit-cycle normalized cross-base phase-signal report with bucket leads and M4 follow-up
cargo run --release --example unit_cycle_base_neighbor_report # Neighbor-base unit-cycle geometry scout centered on bases 56..60 and the base57/base58 teaching pair
cargo run --release --example base57_affine_codec_report # Base57 baseline transcoding plus affine residue/prime notation experiment
cargo run --features metal --release --example metal_affine_transfer_collapse_report # Metal affine transfer-collapse report: residue metadata in, survivor bitmask out
cargo run --features metal --release --example metal_affine_benchmark_report # Local benchmark of Metal affine transfer-collapse vs CPU affine and ordinary candidate baselines
cargo run --release --example prime_witness_engine_visual_atlas # Visual atlas of construction grammar, affine paths, residue weather, singular profiles, and unit-cycle geometry
cargo run --release --example affine_period_lock_species_report # Order-spectrum + shift-residual atlas exploiting the period-lock theorem on the hinge-family direct lane surface
cargo run --release --example base22_gradient_pocket_report # Focused autopsy of the base-22, k=(2,2), mod-5 residual gradient_only pocket at M=2
cargo run --release --example base10_persistence_species_report # Focused base-10 persistence-only report with nearest same-base neighbors and contrast outgroups
cargo run --release --example bounded_k_transfer_criterion_report # Direct lane-comparison theorem audit for the bounded-k transfer-collapse ladder; add --include-base-210 for wheel-track stretch audit
cargo run --release --example base_hinge_probe_report # Flexible base probe, defaulting to base 34 against the 2p hinge references
cargo run --release --example base34_boundary_species_report # Focused base-34 report comparing the three boundary-led pockets against nearby dead pairs
cargo run --release --example m3_k_dominance_report         # Focused M=3 bounded-k report
cargo run --example statistical_prime_generator # Statistical prime generation
cargo run --example prime_verification_report   # Verify all documented claims
```

## Verification and Core Tools

| Example | Description |
|---------|-------------|
| `check_prime` | Simple CLI prime checker (reads from stdin) |
| `prime_count_smoke_test` | Deterministic prime-counting tests against OEIS A000720 |
| `prime_verification_report` | Verification report for all documented membrane primes |
| `verify_prime_checker` | Validates the Miller-Rabin checker against known primes and composites |

## Membrane Generation

| Example | Description |
|---------|-------------|
| `comprehensive_base_analysis` | Systematic membrane config testing across multiple bases |
| `comparative_signal_report` | Single-table comparison of midpoint-density, insertion hit rates, and corrected connector residuals |
| `cross_base_invariance_report` | Cross-base scorecard for exact invariants, bounded `k`-dominance, and maintained matched-control coverage; default smoke catalog with optional `--full` exhaustive rerun |
| `m_boundary_layer_report` | Short-length boundary-layer report that asks which pair features predict whether an `M=1` bounded-`k` anomaly survives to `M=2` |
| `m2_m3_transition_report` | Same-pair transition report that compares bounded-`k` behavior across `M=2` and `M=3` and estimates where anomaly mass collapses |
| `m2_m3_transfer_collapse_report` | Exact `best_k` vs `k=(0,0)` transfer report across `M=2` and `M=3`, showing where nonidentity transfer buckets, signal-source diversity, and representative species meaning exist at `M=2` and collapse into identity profiles by `M=3` |
| `m2_survivor_autopsy_report` | Full-catalog autopsy for the pairs whose anomalies survive to `M=2`, with nearby `m1_only` controls and residue-level obstruction deltas |
| `m2_species_aggregation_report` | Species-level aggregation for the short-length anomaly classes, comparing geometry, winning `k`, admissible deltas, and modulus relief across `m1_only`, `m1_to_m2`, and `m2_only` |
| `m2_species_geometry_report` | Downstream geometric visualization of the anomaly species artifact, rendering pair-lattice scatter plots by base plus an `M=2` species-vs-modulus relief heatmap |
| `m2_relief_fingerprint_report` | Downstream comparison of the persistent vs emergent `M=2` residue-relief species, rendered as a two-panel fingerprint chart over small prime moduli |
| `m2_obstruction_signature_report` | Downstream composite-signature report for `M=2` survivors and nearest dead neighbors, combining admissible lift and whole-vector modulus relief into a signature plane and metric heatmap |
| `m_transition_curve_report` | Same-pair bounded-`k` curve report across a short middle-length range, with anomaly-mass collapse estimates by pair, base, and global lane |
| `m_transition_phase_map_report` | Downstream grouped phase map for the bounded-`k` transition artifact, showing which `k` lane wins for each pair across middle length `M` and how anomaly mass fades out |
| `chaos_threshold_translation_report` | Arithmetic translation pass for the repo's chaos/stability metaphors, treating the bounded-`k` transition lane as the maintained source of truth and exporting transition, decomposition, and metaphor-translation artifacts for bases `6,10,12,14,22,26,30,34` across `M=1..3` |
| `base14_survivor_atlas_report` | Exact base-14 boundary-layer atlas with a species lattice, local transition strip, and `M=2` residue-relief heatmap for the surviving pairs plus nearby `m1_only` neighbors |
| `base14_outlier_mechanism_report` | Reads the maintained base-14 atlas artifact, compares each active `M=2` pair against `k=(0,0)` and its rank-1 nearby dead control, and exports exact admissible/yield/transfer-mask decomposition tables plus a mechanism-plane visual |
| `base14_signal_clarity_report` | Downstream clarity pass for the base-14 mechanism artifact, combining transfer-source prime deltas and exact effect contributions so the shared-yield signal and the `(D,B)` stress case are easier to see |
| `base14_shared_yield_report` | Downstream witness report for the shared-admissible lane itself, comparing active base-14 pairs and their nearby dead controls and rendering a `(D,B)` stress strip over shared-admissible candidates |
| `base14_shared_digit_structure_report` | Downstream digit-structure mining for the shared-admissible lane, summarizing first/second digit deltas, sum/difference residue deltas, a `(D,B)` digit-grid, and an active-pair sum-residue heatmap |
| `two_p_family_report` | Direct `B = 2p` family test across `6,10,14,22,26` against foils `12,18,30`, comparing `M=2` persistence and shared-yield-core structure with pair-weighted and base-weighted summaries |
| `two_p_hinge_report` | Focused follow-up inside the `2p` family, separating `persistent_core`, `persistence_only`, `core_only`, and `active_neither` behavior to explain why base `14` is the strongest nontrivial hinge base while `22` and `26` weaken |
| `two_p_hinge_mask_report` | Cross-base hinge mask atlas for bases `10,14,22,26` plus base `6` appendix, using the exact `k=(0,0) -> best_k_at_M2` transfer spectrum to explain why base `14` is the only non-tiny persistent-core bridge |
| `two_p_hinge_discriminator_report` | Exact hinge-discriminator pass for the main `2p` bases, exporting a reusable cross-`M` feature surface, constrained small-rule search, and representative atlas for the hinge witnesses and near-miss species |
| `two_p_hinge_atom_family_report` | Atom-family depth pass for the main `2p` hinge surface, classifying deterministic atom families by family-only leverage, ablation leverage, mixed-rule participation, and closeness to future theorem language |
| `two_p_hinge_robustness_report` | Robustness matrix pass for the hinge atom-family story, stress-testing the family ladder under representative-drop, threshold-vocabulary, and adversarial catalog scenarios while holding bases `34` and `6` out as appendix audits |
| `affine_hinge_classifier_report` | Exploratory affine atlas for the hinge species surface, exporting exact local shift / gradient / zero-seed comparisons for `k=(0,0) -> best_k`, fixed representative heatmaps, and affine-only vs mixed rule frontiers without widening the public claim surface |
| `affine_gradient_transition_report` | Focused affine transition report for `gradient_only`, comparing the winning `k=(0,0) -> best_k` surface against the full direct `k=(0,0) -> each noncompact lane` atlas across `M=1,2,3` to test whether the observed M=2 disappearance is a true lane collapse or a winner-selection effect |
| `affine_period_lock_report` | Exploratory local-classifier report for period-locked affine gradient agreement, checking whether observed gradient equality is exactly explained by `Δposition ≡ 0 mod ord_p(base)` on the maintained direct lane surface and using the base-22 / mod-5 pocket as the anchor witness |
| `residue_torus_period_lock_report` | Visual-intuition walkthrough for the period-lock residue torus, exporting torus phase rows, a base-22/mod-5 canonical panel, and a small gallery of computed prime construction witnesses while keeping density claims conservative |
| `membrane_prime_throughput_report` | Deterministic `u64` throughput report for maintained affine membrane prime families, measuring the funnel from raw seeds through exact residue filters into deterministic prime witnesses |
| `large_affine_witness_ladder_report` | Prime Witness Engine measurement entrypoint for the primary visible decimal lane `(3,7), k=(2,1)`, measuring time-to-first witness, residue funnel efficacy, confirmation tier scope across BigUint/u128/u64, local controls, OpenSSL calibration, primesieve scope, semantic rarity, and witness gallery rows |
| `seed_to_witness_demo_report` | Prime Witness Engine transcript bundle with a canonical 128-digit probable-prime witness, a shorter teaching row, CSV/JSON exports, and copyable external verification snippets |
| `timestamp_seed_policy_report` | Prime Witness Engine policy report that samples timestamp-like seed origins and measures bounded success rates, step quantiles, residue survivors, and witness rows for full-middle and 128-digit lanes |
| `special_form_witness_comparison_report` | Prime Witness Engine comparison report placing known Mersenne-prime special forms beside affine membrane witnesses, with compact descriptors, confirmation language, and exact `not_mersenne` labels for affine rows |
| `affine_singular_series_report` | Finite affine singular-profile scout that ranks lane residuals after PNT size expectation and exact small-prime residue-weather accounting; positive rows are follow-up leads, not density theorems |
| `construction_density_atlas_report` | Density-drift atlas for maintained plus stress-test affine membrane families, exporting layered controls, witness rows, and visuals for residue survival, zero-run drift, and good-vs-lousy construction contrast |
| `base30_wheel_compact_report` | Focused base-30 compact wheel report that scans all ordered unit pairs for `M=1..3`, exports the `(B,7)` residue funnel, and frames the result as a classical wheel-compressed affine surface rather than residual density magic |
| `base30_reversal_asymmetry_report` | Focused compact base-30 report comparing each ordered unit pair against its reversal, exporting signed asymmetry heatmaps, residue-phase fingerprints, and witnesses for the strongest swapped-role cases |
| `base30_reversal_residual_report` | Follow-up compact base-30 report that decomposes each unordered reversal comparison into raw prime-rate delta, PNT size expectation, exact residue-survivor delta, and survivor-prime residual |
| `affine_phase_residual_atlas_report` | Cross-base compact reversal atlas for bases `6,10,14,22,26,30,34`, ranking local affine phase residual leads after size/PNT and residue-survivor accounting |
| `shift_phase_signal_mining_report` | Curated downstream shift-phase report that follows focus leads and foils into mature `M=4` lanes, exporting same-gradient line, residue-gate comb, survivor-yield, maturity, and lead-vs-foil visuals |
| `unit_cycle_phase_signal_report` | Unit-cycle normalized follow-up to shift-phase residuals, grouping same-gradient swaps by arc geometry and edge/complement status before ranking bucket leads, M4 follow-ups, foils, and witnesses |
| `unit_cycle_base_neighbor_report` | Neighbor-base unit-cycle geometry scout, comparing normalized bead counts, chord spacing, diameter/complement examples, and exact compact phase leads around bases `56..60`, especially base `57` versus base `58` |
| `base57_affine_codec_report` | Base57 codec experiment comparing ordinary base58/base57 transcoding and base-invariant value maps against residue-filtered and prime-witness affine notation chunks. |
| `metal_affine_transfer_collapse_report` | Feature-gated Metal report comparing CPU fast affine, legacy candidate-buffer GPU, and maintained affine transfer-collapse paths; use `--features metal` on macOS |
| `metal_affine_benchmark_report` | Feature-gated local benchmark comparing Metal affine transfer-collapse against CPU affine wheels, CPU residue rows, sequential odd scans, small-prime wheel scans, and random same-window odd candidates; includes repeated Metal dispatch setup timing, a beyond-`u64` BigUint probable-prime row, a source-grounded external comparison frame, and optional local CLI rows for tools such as OpenSSL and primesieve |
| `prime_witness_engine_visual_atlas` | Visual atlas for explaining affine membrane prime families through construction grammar, affine lines, residue gates, transfer collapse, geodesic residue paths, residue weather, singular-profile dashboards, and unit-cycle chord geometry |
| `affine_period_lock_species_report` | Downstream order-spectrum + shift-residual atlas for the period-lock lane, treating direct `k=(0,0) -> each noncompact lane` comparisons as the theorem surface, separating low-order lock mass from higher-order locked `gradient_only` side-pockets, and using base `30` as a theorem-facing control |
| `base22_gradient_pocket_report` | Focused autopsy of the base-22 `M=2` residual `gradient_only` pocket, centered on the direct lane `k=(0,0) -> (2,2)` and the exact mod-5 shift/gradient formulas that separate the pocket rows from the collapsed columns `inner=5,F` |
| `base10_persistence_species_report` | Focused base-10 follow-up on the load-bearing `persistence_only` witness `(3,3)`, comparing it against structured same-base neighbors plus tiny persistence-only and hinge outgroups and running a local exact rule search on the focused species surface |
| `bounded_k_transfer_criterion_report` | Direct theorem-audit pass for bounded-`k` transfer collapse, comparing `k=(0,0)` against each noncompact lane in the maintained grid at `M=2` and `M=3` and classifying every lane comparison by the exact ladder `profile_agreement` / `admissible_equality_only` / `no_positive_admissible_delta_only` / `fails_all_three` |
| `base_hinge_probe_report` | Flexible solution-space probe for a target base, defaulting to base `34`, with base-level hinge comparisons, target-pocket summaries, and ranked active/shared-overlap pair tables |
| `base34_boundary_species_report` | Focused base-34 follow-up that treats the three `M=2` pockets as a possible non-hinge species, comparing their exact `k=(1,0)` boundary-release signal against nearby same-base dead pairs |
| `m3_k_dominance_report` | Focused cross-base report for the stable `M=3` bounded `k` lane, with smoke/full pair catalogs and machine-readable exports |
| `membrane_palindrome_probe` | Exact structure probe: palindrome overlap and zero-layout symmetry |
| `membrane_scaffold_probe` | Exact centered-scaffold vs same-budget control probe |
| `membrane_showcase` | Demonstration of membrane prime patterns across different bases |
| `membrane_vs_random` | Canonical cross-family matched-control report with effect sizes, confidence intervals, BH-adjusted decisions, named `--panel smoke|audit` sampling plans, and optional `--json-out`/`--csv-out` archival export |
| `membrane_vs_random_compare` | Compares two matched-control JSON exports, can emit machine-readable diff JSON with structured audit severities, and supports policy flags that optionally promote sampling drift or family-set changes into nonzero audit failures |
| `membrane_vs_random_compare_batch` | Summarizes many matched-control comparison JSON exports into run/family stability rows, structured severity tallies, and optional archive artifacts |
| `membrane_vs_random_fast` | Fast single-base exploratory control run using base 30 |
| `proper_membrane_generator` | Deterministic membrane generator using seeds (not random search) |
| `solution_space_explorer` | Systematic parameter space mapping (base, M, k_outer, k_inner) |
| `statistical_prime_factory` | Production-ready prime generator using verified membrane patterns |
| `statistical_prime_generator` | Statistical prime generator using empirically-derived patterns |
| `statistical_sampling_demo` | Demonstrates proper statistical sampling of membrane configurations |

### Matched-Control Archive Workflow

Use the named panels when producing rerun archives that should be compared over
time:

```bash
cargo run --release --example membrane_vs_random -- --panel smoke --json-out /tmp/mc-smoke-a.json
cargo run --release --example membrane_vs_random -- --panel smoke --json-out /tmp/mc-smoke-b.json
cargo run --example membrane_vs_random_compare -- /tmp/mc-smoke-a.json /tmp/mc-smoke-b.json --json-out /tmp/mc-diff-ab.json
cargo run --example membrane_vs_random_compare_batch -- /tmp/mc-diff-ab.json --out-dir /tmp/mc-batch
cargo run --bin export_matched_control_atlas_manifest -- --panel smoke --out docs/atlas/matched_control_smoke_atlas_manifest.json
cargo run --bin export_matched_control_residue_masks -- --panel smoke --prime-bound 31 --out-dir /tmp/mc-residue-masks
cargo run --bin export_matched_control_residue_masks -- --panel smoke --prime-bound 31 --format theorem-queue --out docs/atlas/matched_control_theorem_queue.md
cargo run --bin export_matched_control_residue_masks -- --panel smoke --prime-bound 31 --format lean-candidate-checks
./scripts/matched_control_atlas_bridge.sh verify
```

The batch summary is a stability and regression surface for Gate A. It prepares
later forbidden-seed-class theorem work by keeping empirical drift records
machine-readable, while the proof-carrying atlas and residue-mask scanner link
maintained family codes to generated Lean lane identities and exact local
residue facts. The residue-mask scanner now emits v4 cross-modulus fingerprint
rows with exact survivor-count products, shared-overlap products, displacement
lists, optional Lean pair-certificate links, and summary-level
`pair_certified_count` / `pair_uncertified_count` / `top_theorem_candidate`
fields, ranking mask geometry rather than claiming a same-modulus survivor-count
advantage. The scanner also emits `theorem_queue.md` from the same selected
candidate summary so the human planning queue and CI gate consume the same
target surface. Neither layer is itself evidence for a new residual mechanism.
The atlas bridge now fails if the canonical smoke scanner reports any
uncertified pair fingerprint, drifts from the maintained scanner summary
counts, drifts from the tracked theorem queue, or selects a theorem candidate
whose Lean proof links no longer elaborate.

## Lagrange Points and Connectors

| Example | Description |
|---------|-------------|
| `connector_signal_report` | Reconstructs the canonical connector source case, runs the matched same-budget comparison report, and optionally exports density-audit JSON/CSV artifacts |
| `connector_utility_demo` | Demonstrates the connector concatenation API |
| `lagrange_clustering_verifier` | Verifies prime clustering around Lagrange points between prime pairs |
| `lagrange_full_verification` | Verifies entire concatenated strings for primality |
| `lagrange_mechanics` | Explores Lagrange point mechanics with position/digit analysis |
| `lagrange_verification` | Tests Lagrange point insertions between concatenated primes |
| `scan_connectors` | CLI tool to discover prime connectors between two primes |

## Hardy-Littlewood and Statistical Analysis

| Example | Description |
|---------|-------------|
| `babylonian_prime_orthogonality` | Demonstrates orthogonality of human-convenient vs prime-harmonic metrics |
| `empirical_verification_pipeline` | Joins sample/model CSVs and runs verification pipeline |
| `hardy_littlewood_validation` | Computes HL singular series and compares with empirical observations |
| `harmonic_lagrange_explorer` | Polynomial fitting for harmonic Lagrange lineout data |
| `harmonic_overtones_explorer` | Overtone spectrum computation from sample/model data |
| `orthogonality_verification` | Tests independence of spectral regularity and phase lock density |
| `prime_gap_analysis` | Prime gap distributions in coordinate constellations |
| `symmetry_breaking_explorer` | Ridge/trough analysis of symmetry-breaking patterns |

## Interactive TUI Applications

These require a terminal (will show "Device not configured" if run without one).

| Example | Description |
|---------|-------------|
| `lagrange_tui_demo` | Research-grade TUI for exploring prime connectors |
| `membrane_lab_tui` | Interactive membrane laboratory with real-time parameter tuning |
| `prime_atom_tui` | Visualizes membrane primes as atomic structures |

## Special-Purpose Tools

| Example | Description |
|---------|-------------|
| `belphegor_scanner` | Palindromic prime scanner inspired by Numberphile |
| `sandwich_prime_finder` | Finds primes with 1[zeros]meatball[zeros]1 structure |

## Historical Examples

174 exploration scripts, hypothesis tests, and one-off investigations have been
moved to `historical/examples/`. This includes the former `examples/verified/`
(25 files, 24 broken) and `examples/experimental/` (7 files, 4 broken)
subdirectories, whose compiling members were duplicates of top-level examples.
Historical examples are preserved for reference but are not maintained.
