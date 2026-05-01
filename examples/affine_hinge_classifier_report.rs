//! Exploratory affine hinge atlas for the maintained hinge species surface.
//!
//! This pass is classifier discovery rather than claim widening. It asks
//! whether the local affine comparison language:
//! - shift equality
//! - gradient equality
//! - zero-seed-class equality
//!
//! helps reveal why the hinge species separate the way they do.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example affine_hinge_classifier_report
//! cargo run --release --example affine_hinge_classifier_report -- --out-dir /tmp/primes_affine_hinge_classifier_alt
//! ```

use plotters::prelude::*;
use primes::validation::{
    affine_hinge::{
        build_affine_atom_specs, build_affine_hinge_search_problems, run_affine_rule_search,
        AffineRuleCandidate, AffineSearchMode,
    },
    bounded_k::{
        analyze_affine_hinge_feature_row, ordered_unit_pairs, parse_k_label,
        scan_k_config_affine_lane_comparison, AffineHingeFeatureRow, HINGE_CATEGORY_ACTIVE_NEITHER,
        HINGE_CATEGORY_CORE_ONLY, HINGE_CATEGORY_PERSISTENCE_ONLY, HINGE_CATEGORY_PERSISTENT_CORE,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    path::{Path, PathBuf},
};

const MAIN_BASES: &[u32] = &[10, 14, 22, 26];
const APPENDIX_BASES: &[u32] = &[34, 6];
const DEFAULT_OUT_DIR: &str = "/tmp/primes_affine_hinge_classifier";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "affine_hinge_classifier_report";
const MAX_RULE_ATOMS: usize = 3;
const EXPORTED_RULE_FRONTIER: usize = 40;
const BEST_RULES_PER_SEARCH: usize = 5;

const REPRESENTATIVES: &[RepresentativeSpec] = &[
    RepresentativeSpec {
        role: "persistent_core",
        base: 14,
        outer: 13,
        inner: 11,
    },
    RepresentativeSpec {
        role: "persistent_core",
        base: 14,
        outer: 3,
        inner: 1,
    },
    RepresentativeSpec {
        role: "persistence_only",
        base: 10,
        outer: 3,
        inner: 3,
    },
    RepresentativeSpec {
        role: "core_only",
        base: 26,
        outer: 23,
        inner: 23,
    },
    RepresentativeSpec {
        role: "active_neither",
        base: 22,
        outer: 17,
        inner: 19,
    },
    RepresentativeSpec {
        role: "appendix_outgroup",
        base: 34,
        outer: 25,
        inner: 9,
    },
    RepresentativeSpec {
        role: "same_base_dead_control",
        base: 10,
        outer: 9,
        inner: 9,
    },
];

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone)]
struct RowBundle {
    scope: String,
    affine_row: AffineHingeFeatureRow,
    m1_comparison: primes::validation::bounded_k::KConfigAffineLaneComparison,
    m2_comparison: primes::validation::bounded_k::KConfigAffineLaneComparison,
}

#[derive(Debug, Clone, Copy)]
struct RepresentativeSpec {
    role: &'static str,
    base: u32,
    outer: u32,
    inner: u32,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    main_bases: Vec<u32>,
    appendix_bases: Vec<u32>,
    middle_lengths: Vec<usize>,
    classifier_surface: String,
    search_modes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct AffineModulusCsvRow {
    scope: String,
    phase: String,
    base: u32,
    pair_label: String,
    hinge_category: String,
    from_k: String,
    to_k: String,
    modulus: u32,
    shift_modulus_from: u32,
    shift_modulus_to: u32,
    gradient_modulus_from: u32,
    gradient_modulus_to: u32,
    zero_seed_class_from: u32,
    zero_seed_class_to: u32,
    shift_equal: bool,
    gradient_equal: bool,
    zero_seed_equal: bool,
    local_relation_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct AffineLaneSummaryRow {
    scope: String,
    phase: String,
    base: u32,
    pair_label: String,
    hinge_category: String,
    from_k: String,
    to_k: String,
    compared_moduli_count: usize,
    same_shift_count: usize,
    same_gradient_count: usize,
    same_zero_seed_count: usize,
    identity_count: usize,
    shift_only_count: usize,
    gradient_only_count: usize,
    shift_and_gradient_count: usize,
    same_shift_share: f64,
    same_gradient_share: f64,
    same_zero_seed_share: f64,
    identity_share: f64,
    shift_only_share: f64,
    gradient_only_share: f64,
    shift_and_gradient_share: f64,
}

#[derive(Debug, Clone, Serialize)]
struct AffineHingeFeatureCsvRow {
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    same_digit: bool,
    unit_distance: usize,
    gap_bucket: String,
    m1_active: bool,
    m2_active: bool,
    m1_to_m2_persistent: bool,
    m1_best_k: String,
    m2_best_k: String,
    m1_anomaly_mass_pp: f64,
    m2_anomaly_mass_pp: f64,
    m1_admissible_delta_pp: f64,
    m2_admissible_delta_pp: f64,
    m1_stable_zero_prime_delta_count: isize,
    m2_stable_zero_prime_delta_count: isize,
    m1_boundary_prime_delta_count: isize,
    m2_boundary_prime_delta_count: isize,
    m1_stable_zero_prime_delta_pp: f64,
    m2_stable_zero_prime_delta_pp: f64,
    m1_boundary_prime_delta_pp: f64,
    m2_boundary_prime_delta_pp: f64,
    m1_shared_prime_rate_delta_pp: f64,
    m2_shared_prime_rate_delta_pp: f64,
    m1_stable_zero_signal_margin_count: isize,
    m2_stable_zero_signal_margin_count: isize,
    m1_stable_zero_signal_margin_pp: f64,
    m2_stable_zero_signal_margin_pp: f64,
    m1_stable_zero_support_ratio: f64,
    m2_stable_zero_support_ratio: f64,
    m1_mask_stability_share: f64,
    m2_mask_stability_share: f64,
    m1_admissible_overlap_jaccard: f64,
    m2_admissible_overlap_jaccard: f64,
    m1_nonzero_churn_share: f64,
    m2_nonzero_churn_share: f64,
    m2_stable_zero_count: usize,
    m2_gain_zero_count: usize,
    m2_loss_zero_count: usize,
    m2_stable_nonzero_count: usize,
    m2_nonzero_churn_count: usize,
    m1_signal_source_label: String,
    m2_signal_source_label: String,
    shared_yield_core: bool,
    hinge_category: String,
    m1_affine_compared_moduli_count: usize,
    m1_affine_same_shift_count: usize,
    m1_affine_same_gradient_count: usize,
    m1_affine_same_zero_seed_count: usize,
    m1_affine_identity_count: usize,
    m1_affine_shift_only_count: usize,
    m1_affine_gradient_only_count: usize,
    m1_affine_shift_and_gradient_count: usize,
    m1_affine_same_shift_share: f64,
    m1_affine_same_gradient_share: f64,
    m1_affine_same_zero_seed_share: f64,
    m1_affine_identity_share: f64,
    m1_affine_shift_only_share: f64,
    m1_affine_gradient_only_share: f64,
    m1_affine_shift_and_gradient_share: f64,
    m2_affine_compared_moduli_count: usize,
    m2_affine_same_shift_count: usize,
    m2_affine_same_gradient_count: usize,
    m2_affine_same_zero_seed_count: usize,
    m2_affine_identity_count: usize,
    m2_affine_shift_only_count: usize,
    m2_affine_gradient_only_count: usize,
    m2_affine_shift_and_gradient_count: usize,
    m2_affine_same_shift_share: f64,
    m2_affine_same_gradient_share: f64,
    m2_affine_same_zero_seed_share: f64,
    m2_affine_identity_share: f64,
    m2_affine_shift_only_share: f64,
    m2_affine_gradient_only_share: f64,
    m2_affine_shift_and_gradient_share: f64,
}

#[derive(Debug, Clone, Serialize)]
struct RepresentativeAffineRow {
    role: String,
    base: u32,
    pair_label: String,
    hinge_category: String,
    m1_best_k: String,
    m2_best_k: String,
    m1_same_gradient_share: f64,
    m1_same_zero_seed_share: f64,
    m1_identity_share: f64,
    m1_shift_only_share: f64,
    m1_gradient_only_share: f64,
    m2_same_gradient_share: f64,
    m2_same_zero_seed_share: f64,
    m2_identity_share: f64,
    m2_shift_only_share: f64,
    m2_gradient_only_share: f64,
    m2_shift_and_gradient_share: f64,
    m1_anomaly_mass_pp: f64,
    m2_anomaly_mass_pp: f64,
    local_note: String,
}

#[derive(Debug, Clone, Serialize)]
struct AffineRuleCandidateCsvRow {
    search_id: String,
    search_label: String,
    search_mode: String,
    atom_count: usize,
    rule_label: String,
    exact_match: bool,
    total_errors: usize,
    true_positive: usize,
    false_positive: usize,
    false_negative: usize,
    true_negative: usize,
    precision: f64,
    recall: f64,
    f1: f64,
    positive_support: usize,
    complexity_score: usize,
    threshold_free: bool,
    interpretability_rank: usize,
    atom_labels: String,
}

#[derive(Debug, Clone, Serialize)]
struct SearchSummaryRow {
    search_id: String,
    search_label: String,
    search_mode: String,
    dataset_rows: usize,
    positive_rows: usize,
    atom_pool_size: usize,
    searched_rule_count: usize,
    any_exact_rule: bool,
    best_rule_label: String,
    best_rule_status: String,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    main_rows: usize,
    main_active_rows: usize,
    affine_only_primary_status: String,
    affine_only_primary_rule: String,
    mixed_primary_status: String,
    mixed_primary_rule: String,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    affine_hinge_feature_rows: Vec<AffineHingeFeatureRow>,
    search_summary_rows: Vec<SearchSummaryRow>,
    best_rules: Vec<AffineRuleCandidate>,
    representative_rows: Vec<RepresentativeAffineRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let mut main_rows = build_row_bundles(MAIN_BASES, "main");
    let mut appendix_rows = build_row_bundles(APPENDIX_BASES, "appendix");
    main_rows.sort_by(row_bundle_sort);
    appendix_rows.sort_by(row_bundle_sort);

    let mut all_rows = main_rows.clone();
    all_rows.extend(appendix_rows.clone());

    let affine_hinge_feature_rows = all_rows
        .iter()
        .map(|bundle| bundle.affine_row.clone())
        .collect::<Vec<_>>();
    let affine_hinge_feature_csv_rows = affine_hinge_feature_rows
        .iter()
        .map(flatten_affine_hinge_feature_row)
        .collect::<Vec<_>>();
    let affine_modulus_rows = build_affine_modulus_rows(&all_rows);
    let affine_lane_summary_rows = build_affine_lane_summary_rows(&all_rows);

    let search_input_rows = main_rows
        .iter()
        .map(|bundle| bundle.affine_row.clone())
        .collect::<Vec<_>>();
    let search_outcomes = build_search_outcomes(&search_input_rows);
    let search_summary_rows = search_outcomes
        .iter()
        .map(|outcome| SearchSummaryRow {
            search_id: outcome.summary.search_id.clone(),
            search_label: outcome.summary.search_label.clone(),
            search_mode: outcome.summary.search_mode.clone(),
            dataset_rows: outcome.summary.dataset_rows,
            positive_rows: outcome.summary.positive_rows,
            atom_pool_size: outcome.summary.atom_pool_size,
            searched_rule_count: outcome.summary.searched_rule_count,
            any_exact_rule: outcome.summary.any_exact_rule,
            best_rule_label: outcome.summary.best_rule_label.clone(),
            best_rule_status: outcome.summary.best_rule_status.clone(),
        })
        .collect::<Vec<_>>();
    let affine_rule_candidate_rows = search_outcomes
        .iter()
        .flat_map(|outcome| outcome.candidate_rows.clone())
        .collect::<Vec<_>>();
    let affine_best_rules = search_outcomes
        .iter()
        .flat_map(|outcome| outcome.best_rows.clone())
        .collect::<Vec<_>>();
    let affine_rule_candidate_csv_rows = affine_rule_candidate_rows
        .iter()
        .map(flatten_rule_candidate)
        .collect::<Vec<_>>();
    let affine_best_rule_csv_rows = affine_best_rules
        .iter()
        .map(flatten_rule_candidate)
        .collect::<Vec<_>>();
    let representative_rows = build_representative_rows(&all_rows);

    let plane_path = options.out_dir.join("affine_classifier_plane.png");
    render_classifier_plane(&main_rows, &plane_path);
    let heatmap_path = options.out_dir.join("affine_relation_heatmap.png");
    render_relation_heatmap(&representative_rows, &all_rows, &heatmap_path);
    let strip_path = options.out_dir.join("affine_seed_class_strip.png");
    render_seed_class_strip(&representative_rows, &all_rows, &strip_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "classifier_plane".to_string(),
            label: "Affine classifier plane".to_string(),
            path: plane_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "relation_heatmap".to_string(),
            label: "Affine relation heatmap".to_string(),
            path: heatmap_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "seed_class_strip".to_string(),
            label: "Affine zero-seed strip".to_string(),
            path: strip_path.display().to_string(),
        },
    ];

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        main_bases: MAIN_BASES.to_vec(),
        appendix_bases: APPENDIX_BASES.to_vec(),
        middle_lengths: vec![1, 2],
        classifier_surface: "main M=2 active hinge species".to_string(),
        search_modes: vec![
            AffineSearchMode::AffineOnly.as_str().to_string(),
            AffineSearchMode::MixedExisting.as_str().to_string(),
        ],
    };
    let report_summary = build_report_summary(&main_rows, &search_outcomes);
    let observations = derive_observations(&representative_rows, &search_outcomes);
    let report_text = render_report(&settings, &report_summary, &observations, &search_outcomes);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        affine_hinge_feature_rows: affine_hinge_feature_rows.clone(),
        search_summary_rows,
        best_rules: affine_best_rules.clone(),
        representative_rows: representative_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary: report_summary.clone(),
        observations: observations.clone(),
    };

    write_csv_rows(
        options.out_dir.join("affine_modulus_rows.csv"),
        &affine_modulus_rows,
    )
    .expect("write affine modulus rows");
    write_csv_rows(
        options.out_dir.join("affine_lane_summary_rows.csv"),
        &affine_lane_summary_rows,
    )
    .expect("write affine lane summary rows");
    write_csv_rows(
        options.out_dir.join("affine_hinge_feature_rows.csv"),
        &affine_hinge_feature_csv_rows,
    )
    .expect("write affine hinge feature rows");
    write_csv_rows(
        options.out_dir.join("affine_rule_candidate_rows.csv"),
        &affine_rule_candidate_csv_rows,
    )
    .expect("write affine rule candidate rows");
    write_csv_rows(
        options.out_dir.join("affine_best_rules.csv"),
        &affine_best_rule_csv_rows,
    )
    .expect("write affine best rules");
    write_csv_rows(
        options.out_dir.join("representative_affine_rows.csv"),
        &representative_rows,
    )
    .expect("write representative affine rows");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_text_file(options.out_dir.join("report.md"), &report_text).expect("write report");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--release".to_string(),
                "--example".to_string(),
                "affine_hinge_classifier_report".to_string(),
            ],
            upstream_inputs: vec![
                "src/validation/bounded_k.rs".to_string(),
                "src/validation/affine_hinge.rs".to_string(),
            ],
            expected_outputs: vec![
                "affine_modulus_rows.csv".to_string(),
                "affine_lane_summary_rows.csv".to_string(),
                "affine_hinge_feature_rows.csv".to_string(),
                "affine_rule_candidate_rows.csv".to_string(),
                "affine_best_rules.csv".to_string(),
                "representative_affine_rows.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
                "artifact_manifest.json".to_string(),
                "affine_classifier_plane.png".to_string(),
                "affine_relation_heatmap.png".to_string(),
                "affine_seed_class_strip.png".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote affine hinge classifier bundle to {}",
        options.out_dir.display()
    );
    println!(
        "affine-only primary: {} ({})",
        report_summary.affine_only_primary_rule, report_summary.affine_only_primary_status
    );
    println!(
        "mixed primary: {} ({})",
        report_summary.mixed_primary_rule, report_summary.mixed_primary_status
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--out-dir" {
            let value = args.next().expect("missing value after --out-dir");
            out_dir = PathBuf::from(value);
        } else {
            panic!("unrecognized argument: {arg}");
        }
    }
    Options { out_dir }
}

fn build_row_bundles(bases: &[u32], scope: &str) -> Vec<RowBundle> {
    bases
        .par_iter()
        .flat_map_iter(|&base| {
            ordered_unit_pairs(base)
                .into_iter()
                .map(move |(outer, inner)| (base, outer, inner))
        })
        .map(|(base, outer, inner)| {
            let affine_row = analyze_affine_hinge_feature_row(base, outer, inner);
            let m1_comparison = scan_k_config_affine_lane_comparison(
                base,
                1,
                outer,
                inner,
                (0, 0),
                parse_k_label(&affine_row.hinge_row.m1_best_k),
            );
            let m2_comparison = scan_k_config_affine_lane_comparison(
                base,
                2,
                outer,
                inner,
                (0, 0),
                parse_k_label(&affine_row.hinge_row.m2_best_k),
            );
            RowBundle {
                scope: scope.to_string(),
                affine_row,
                m1_comparison,
                m2_comparison,
            }
        })
        .collect()
}

fn row_bundle_sort(left: &RowBundle, right: &RowBundle) -> std::cmp::Ordering {
    left.affine_row
        .hinge_row
        .base
        .cmp(&right.affine_row.hinge_row.base)
        .then_with(|| {
            left.affine_row
                .hinge_row
                .outer
                .cmp(&right.affine_row.hinge_row.outer)
        })
        .then_with(|| {
            left.affine_row
                .hinge_row
                .inner
                .cmp(&right.affine_row.hinge_row.inner)
        })
}

fn build_affine_modulus_rows(rows: &[RowBundle]) -> Vec<AffineModulusCsvRow> {
    rows.iter()
        .flat_map(|bundle| {
            [("M1", &bundle.m1_comparison), ("M2", &bundle.m2_comparison)]
                .into_iter()
                .flat_map(move |(phase, comparison)| {
                    comparison
                        .modulus_rows
                        .iter()
                        .map(move |row| AffineModulusCsvRow {
                            scope: bundle.scope.clone(),
                            phase: phase.to_string(),
                            base: bundle.affine_row.hinge_row.base,
                            pair_label: bundle.affine_row.hinge_row.pair_label.clone(),
                            hinge_category: bundle.affine_row.hinge_row.hinge_category.clone(),
                            from_k: comparison.from_k.clone(),
                            to_k: comparison.to_k.clone(),
                            modulus: row.modulus,
                            shift_modulus_from: row.shift_modulus_from,
                            shift_modulus_to: row.shift_modulus_to,
                            gradient_modulus_from: row.gradient_modulus_from,
                            gradient_modulus_to: row.gradient_modulus_to,
                            zero_seed_class_from: row.zero_seed_class_from,
                            zero_seed_class_to: row.zero_seed_class_to,
                            shift_equal: row.shift_equal,
                            gradient_equal: row.gradient_equal,
                            zero_seed_equal: row.zero_seed_equal,
                            local_relation_label: row.local_relation_label.clone(),
                        })
                })
        })
        .collect()
}

fn build_affine_lane_summary_rows(rows: &[RowBundle]) -> Vec<AffineLaneSummaryRow> {
    rows.iter()
        .flat_map(|bundle| {
            [("M1", &bundle.m1_comparison), ("M2", &bundle.m2_comparison)]
                .into_iter()
                .map(move |(phase, comparison)| AffineLaneSummaryRow {
                    scope: bundle.scope.clone(),
                    phase: phase.to_string(),
                    base: bundle.affine_row.hinge_row.base,
                    pair_label: bundle.affine_row.hinge_row.pair_label.clone(),
                    hinge_category: bundle.affine_row.hinge_row.hinge_category.clone(),
                    from_k: comparison.from_k.clone(),
                    to_k: comparison.to_k.clone(),
                    compared_moduli_count: comparison.compared_moduli_count,
                    same_shift_count: comparison.same_shift_count,
                    same_gradient_count: comparison.same_gradient_count,
                    same_zero_seed_count: comparison.same_zero_seed_count,
                    identity_count: comparison.identity_count,
                    shift_only_count: comparison.shift_only_count,
                    gradient_only_count: comparison.gradient_only_count,
                    shift_and_gradient_count: comparison.shift_and_gradient_count,
                    same_shift_share: comparison.same_shift_share,
                    same_gradient_share: comparison.same_gradient_share,
                    same_zero_seed_share: comparison.same_zero_seed_share,
                    identity_share: comparison.identity_share,
                    shift_only_share: comparison.shift_only_share,
                    gradient_only_share: comparison.gradient_only_share,
                    shift_and_gradient_share: comparison.shift_and_gradient_share,
                })
        })
        .collect()
}

fn flatten_affine_hinge_feature_row(row: &AffineHingeFeatureRow) -> AffineHingeFeatureCsvRow {
    AffineHingeFeatureCsvRow {
        base: row.hinge_row.base,
        outer: row.hinge_row.outer,
        inner: row.hinge_row.inner,
        pair_label: row.hinge_row.pair_label.clone(),
        same_digit: row.hinge_row.same_digit,
        unit_distance: row.hinge_row.unit_distance,
        gap_bucket: row.hinge_row.gap_bucket.clone(),
        m1_active: row.hinge_row.m1_active,
        m2_active: row.hinge_row.m2_active,
        m1_to_m2_persistent: row.hinge_row.m1_to_m2_persistent,
        m1_best_k: row.hinge_row.m1_best_k.clone(),
        m2_best_k: row.hinge_row.m2_best_k.clone(),
        m1_anomaly_mass_pp: row.hinge_row.m1_anomaly_mass_pp,
        m2_anomaly_mass_pp: row.hinge_row.m2_anomaly_mass_pp,
        m1_admissible_delta_pp: row.hinge_row.m1_admissible_delta_pp,
        m2_admissible_delta_pp: row.hinge_row.m2_admissible_delta_pp,
        m1_stable_zero_prime_delta_count: row.hinge_row.m1_stable_zero_prime_delta_count,
        m2_stable_zero_prime_delta_count: row.hinge_row.m2_stable_zero_prime_delta_count,
        m1_boundary_prime_delta_count: row.hinge_row.m1_boundary_prime_delta_count,
        m2_boundary_prime_delta_count: row.hinge_row.m2_boundary_prime_delta_count,
        m1_stable_zero_prime_delta_pp: row.hinge_row.m1_stable_zero_prime_delta_pp,
        m2_stable_zero_prime_delta_pp: row.hinge_row.m2_stable_zero_prime_delta_pp,
        m1_boundary_prime_delta_pp: row.hinge_row.m1_boundary_prime_delta_pp,
        m2_boundary_prime_delta_pp: row.hinge_row.m2_boundary_prime_delta_pp,
        m1_shared_prime_rate_delta_pp: row.hinge_row.m1_shared_prime_rate_delta_pp,
        m2_shared_prime_rate_delta_pp: row.hinge_row.m2_shared_prime_rate_delta_pp,
        m1_stable_zero_signal_margin_count: row.hinge_row.m1_stable_zero_signal_margin_count,
        m2_stable_zero_signal_margin_count: row.hinge_row.m2_stable_zero_signal_margin_count,
        m1_stable_zero_signal_margin_pp: row.hinge_row.m1_stable_zero_signal_margin_pp,
        m2_stable_zero_signal_margin_pp: row.hinge_row.m2_stable_zero_signal_margin_pp,
        m1_stable_zero_support_ratio: row.hinge_row.m1_stable_zero_support_ratio,
        m2_stable_zero_support_ratio: row.hinge_row.m2_stable_zero_support_ratio,
        m1_mask_stability_share: row.hinge_row.m1_mask_stability_share,
        m2_mask_stability_share: row.hinge_row.m2_mask_stability_share,
        m1_admissible_overlap_jaccard: row.hinge_row.m1_admissible_overlap_jaccard,
        m2_admissible_overlap_jaccard: row.hinge_row.m2_admissible_overlap_jaccard,
        m1_nonzero_churn_share: row.hinge_row.m1_nonzero_churn_share,
        m2_nonzero_churn_share: row.hinge_row.m2_nonzero_churn_share,
        m2_stable_zero_count: row.hinge_row.m2_stable_zero_count,
        m2_gain_zero_count: row.hinge_row.m2_gain_zero_count,
        m2_loss_zero_count: row.hinge_row.m2_loss_zero_count,
        m2_stable_nonzero_count: row.hinge_row.m2_stable_nonzero_count,
        m2_nonzero_churn_count: row.hinge_row.m2_nonzero_churn_count,
        m1_signal_source_label: row.hinge_row.m1_signal_source_label.clone(),
        m2_signal_source_label: row.hinge_row.m2_signal_source_label.clone(),
        shared_yield_core: row.hinge_row.shared_yield_core,
        hinge_category: row.hinge_row.hinge_category.clone(),
        m1_affine_compared_moduli_count: row.m1_affine_compared_moduli_count,
        m1_affine_same_shift_count: row.m1_affine_same_shift_count,
        m1_affine_same_gradient_count: row.m1_affine_same_gradient_count,
        m1_affine_same_zero_seed_count: row.m1_affine_same_zero_seed_count,
        m1_affine_identity_count: row.m1_affine_identity_count,
        m1_affine_shift_only_count: row.m1_affine_shift_only_count,
        m1_affine_gradient_only_count: row.m1_affine_gradient_only_count,
        m1_affine_shift_and_gradient_count: row.m1_affine_shift_and_gradient_count,
        m1_affine_same_shift_share: row.m1_affine_same_shift_share,
        m1_affine_same_gradient_share: row.m1_affine_same_gradient_share,
        m1_affine_same_zero_seed_share: row.m1_affine_same_zero_seed_share,
        m1_affine_identity_share: row.m1_affine_identity_share,
        m1_affine_shift_only_share: row.m1_affine_shift_only_share,
        m1_affine_gradient_only_share: row.m1_affine_gradient_only_share,
        m1_affine_shift_and_gradient_share: row.m1_affine_shift_and_gradient_share,
        m2_affine_compared_moduli_count: row.m2_affine_compared_moduli_count,
        m2_affine_same_shift_count: row.m2_affine_same_shift_count,
        m2_affine_same_gradient_count: row.m2_affine_same_gradient_count,
        m2_affine_same_zero_seed_count: row.m2_affine_same_zero_seed_count,
        m2_affine_identity_count: row.m2_affine_identity_count,
        m2_affine_shift_only_count: row.m2_affine_shift_only_count,
        m2_affine_gradient_only_count: row.m2_affine_gradient_only_count,
        m2_affine_shift_and_gradient_count: row.m2_affine_shift_and_gradient_count,
        m2_affine_same_shift_share: row.m2_affine_same_shift_share,
        m2_affine_same_gradient_share: row.m2_affine_same_gradient_share,
        m2_affine_same_zero_seed_share: row.m2_affine_same_zero_seed_share,
        m2_affine_identity_share: row.m2_affine_identity_share,
        m2_affine_shift_only_share: row.m2_affine_shift_only_share,
        m2_affine_gradient_only_share: row.m2_affine_gradient_only_share,
        m2_affine_shift_and_gradient_share: row.m2_affine_shift_and_gradient_share,
    }
}

fn flatten_rule_candidate(row: &AffineRuleCandidate) -> AffineRuleCandidateCsvRow {
    AffineRuleCandidateCsvRow {
        search_id: row.search_id.clone(),
        search_label: row.search_label.clone(),
        search_mode: row.search_mode.clone(),
        atom_count: row.atom_count,
        rule_label: row.rule_label.clone(),
        exact_match: row.exact_match,
        total_errors: row.total_errors,
        true_positive: row.true_positive,
        false_positive: row.false_positive,
        false_negative: row.false_negative,
        true_negative: row.true_negative,
        precision: row.precision,
        recall: row.recall,
        f1: row.f1,
        positive_support: row.positive_support,
        complexity_score: row.complexity_score,
        threshold_free: row.threshold_free,
        interpretability_rank: row.interpretability_rank,
        atom_labels: row.atom_labels.join(" | "),
    }
}

fn build_search_outcomes(
    rows: &[AffineHingeFeatureRow],
) -> Vec<primes::validation::affine_hinge::AffineSearchOutcome> {
    let problems = build_affine_hinge_search_problems(rows);
    let mut outcomes = Vec::new();
    for problem in &problems {
        for mode in [
            AffineSearchMode::AffineOnly,
            AffineSearchMode::MixedExisting,
        ] {
            let atoms = build_affine_atom_specs(problem, mode);
            outcomes.push(run_affine_rule_search(
                problem,
                mode,
                &atoms,
                MAX_RULE_ATOMS,
                EXPORTED_RULE_FRONTIER,
                BEST_RULES_PER_SEARCH,
            ));
        }
    }
    outcomes
}

fn build_representative_rows(all_rows: &[RowBundle]) -> Vec<RepresentativeAffineRow> {
    REPRESENTATIVES
        .iter()
        .map(|spec| {
            let bundle = all_rows
                .iter()
                .find(|bundle| {
                    bundle.affine_row.hinge_row.base == spec.base
                        && bundle.affine_row.hinge_row.outer == spec.outer
                        && bundle.affine_row.hinge_row.inner == spec.inner
                })
                .expect("representative row should exist");
            RepresentativeAffineRow {
                role: spec.role.to_string(),
                base: spec.base,
                pair_label: bundle.affine_row.hinge_row.pair_label.clone(),
                hinge_category: bundle.affine_row.hinge_row.hinge_category.clone(),
                m1_best_k: bundle.affine_row.hinge_row.m1_best_k.clone(),
                m2_best_k: bundle.affine_row.hinge_row.m2_best_k.clone(),
                m1_same_gradient_share: bundle.affine_row.m1_affine_same_gradient_share,
                m1_same_zero_seed_share: bundle.affine_row.m1_affine_same_zero_seed_share,
                m1_identity_share: bundle.affine_row.m1_affine_identity_share,
                m1_shift_only_share: bundle.affine_row.m1_affine_shift_only_share,
                m1_gradient_only_share: bundle.affine_row.m1_affine_gradient_only_share,
                m2_same_gradient_share: bundle.affine_row.m2_affine_same_gradient_share,
                m2_same_zero_seed_share: bundle.affine_row.m2_affine_same_zero_seed_share,
                m2_identity_share: bundle.affine_row.m2_affine_identity_share,
                m2_shift_only_share: bundle.affine_row.m2_affine_shift_only_share,
                m2_gradient_only_share: bundle.affine_row.m2_affine_gradient_only_share,
                m2_shift_and_gradient_share: bundle.affine_row.m2_affine_shift_and_gradient_share,
                m1_anomaly_mass_pp: bundle.affine_row.hinge_row.m1_anomaly_mass_pp,
                m2_anomaly_mass_pp: bundle.affine_row.hinge_row.m2_anomaly_mass_pp,
                local_note: representative_note(&bundle.affine_row),
            }
        })
        .collect()
}

fn representative_note(row: &AffineHingeFeatureRow) -> String {
    match row.hinge_row.hinge_category.as_str() {
        HINGE_CATEGORY_PERSISTENT_CORE => "persists into M=2 and stays overlap-led".to_string(),
        HINGE_CATEGORY_PERSISTENCE_ONLY => "persists into M=2 but remains boundary-led".to_string(),
        HINGE_CATEGORY_CORE_ONLY => {
            "shows M=2 overlap structure without M=1 carry-through".to_string()
        }
        HINGE_CATEGORY_ACTIVE_NEITHER => "misses both persistence and overlap core".to_string(),
        _ => "unclassified".to_string(),
    }
}

fn build_report_summary(
    main_rows: &[RowBundle],
    search_outcomes: &[primes::validation::affine_hinge::AffineSearchOutcome],
) -> ReportSummary {
    let main_active_rows = main_rows
        .iter()
        .filter(|bundle| bundle.affine_row.hinge_row.m2_active)
        .count();
    let affine_only_primary = search_outcomes
        .iter()
        .find(|outcome| {
            outcome.summary.search_id == "primary_persistent_core"
                && outcome.summary.search_mode == AffineSearchMode::AffineOnly.as_str()
        })
        .expect("affine-only primary outcome should exist");
    let mixed_primary = search_outcomes
        .iter()
        .find(|outcome| {
            outcome.summary.search_id == "primary_persistent_core"
                && outcome.summary.search_mode == AffineSearchMode::MixedExisting.as_str()
        })
        .expect("mixed primary outcome should exist");

    let main_takeaway = if affine_only_primary.summary.any_exact_rule {
        format!(
            "The affine-only primary surface admits an exact separator: `{}`.",
            affine_only_primary.summary.best_rule_label
        )
    } else {
        format!(
            "No exact affine-only primary separator appeared; the best frontier rule is `{}` while the mixed surface gives `{}`.",
            affine_only_primary.summary.best_rule_label, mixed_primary.summary.best_rule_label
        )
    };

    ReportSummary {
        main_rows: main_rows.len(),
        main_active_rows,
        affine_only_primary_status: affine_only_primary.summary.best_rule_status.clone(),
        affine_only_primary_rule: affine_only_primary.summary.best_rule_label.clone(),
        mixed_primary_status: mixed_primary.summary.best_rule_status.clone(),
        mixed_primary_rule: mixed_primary.summary.best_rule_label.clone(),
        main_takeaway,
    }
}

fn derive_observations(
    representative_rows: &[RepresentativeAffineRow],
    search_outcomes: &[primes::validation::affine_hinge::AffineSearchOutcome],
) -> Vec<String> {
    let mut observations = Vec::new();
    if let Some(base14_db) = representative_rows
        .iter()
        .find(|row| row.base == 14 && row.pair_label == "(D,B)")
    {
        observations.push(format!(
            "Base 14 (D,B) keeps high M2 zero-seed agreement ({:.3}) while staying hinge-like.",
            base14_db.m2_same_zero_seed_share
        ));
    }
    if let Some(base10_33) = representative_rows
        .iter()
        .find(|row| row.base == 10 && row.pair_label == "(3,3)")
    {
        observations.push(format!(
            "Base 10 (3,3) persists with M2 shift-only share {:.3}, but its note remains persistence-only rather than hinge-like.",
            base10_33.m2_shift_only_share
        ));
    }
    if let Some(outcome) = search_outcomes.iter().find(|outcome| {
        outcome.summary.search_id == "primary_persistent_core"
            && outcome.summary.search_mode == AffineSearchMode::AffineOnly.as_str()
    }) {
        observations.push(format!(
            "Affine-only primary search status: {} ({})",
            outcome.summary.best_rule_status, outcome.summary.best_rule_label
        ));
    }
    observations
}

fn render_report(
    settings: &ReportSettings,
    report_summary: &ReportSummary,
    observations: &[String],
    search_outcomes: &[primes::validation::affine_hinge::AffineSearchOutcome],
) -> String {
    let affine_only_primary = search_outcomes
        .iter()
        .find(|outcome| {
            outcome.summary.search_id == "primary_persistent_core"
                && outcome.summary.search_mode == AffineSearchMode::AffineOnly.as_str()
        })
        .expect("affine-only primary outcome should exist");
    let mixed_primary = search_outcomes
        .iter()
        .find(|outcome| {
            outcome.summary.search_id == "primary_persistent_core"
                && outcome.summary.search_mode == AffineSearchMode::MixedExisting.as_str()
        })
        .expect("mixed primary outcome should exist");

    let mut lines = Vec::new();
    lines.push("# Affine Hinge Classifier".to_string());
    lines.push(String::new());
    lines.push("## Scope".to_string());
    lines.push(format!(
        "- Main bases: {}",
        settings
            .main_bases
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    ));
    lines.push(format!(
        "- Appendix bases: {}",
        settings
            .appendix_bases
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    ));
    lines.push(format!(
        "- Primary surface: {} ({} main rows, {} active M2 rows)",
        settings.classifier_surface, report_summary.main_rows, report_summary.main_active_rows
    ));
    lines.push(String::new());
    lines.push("## Primary Search".to_string());
    if affine_only_primary.summary.any_exact_rule {
        lines.push(format!(
            "- Affine-only exact rule: `{}`",
            affine_only_primary.summary.best_rule_label
        ));
    } else {
        lines.push(format!(
            "- Affine-only exact rule: no_exact_rule; frontier starts at `{}`",
            affine_only_primary.summary.best_rule_label
        ));
    }
    lines.push(format!(
        "- Mixed affine+existing rule: `{}` ({})",
        mixed_primary.summary.best_rule_label, mixed_primary.summary.best_rule_status
    ));
    lines.push(String::new());
    lines.push("## Takeaway".to_string());
    lines.push(format!("- {}", report_summary.main_takeaway));
    lines.push(String::new());
    lines.push("## Observations".to_string());
    for observation in observations {
        lines.push(format!("- {observation}"));
    }
    lines.join("\n")
}

fn render_classifier_plane(rows: &[RowBundle], path: &Path) {
    let active_rows = rows
        .iter()
        .filter(|bundle| bundle.affine_row.hinge_row.m2_active)
        .collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (980, 920)).into_drawing_area();
    root.fill(&WHITE).expect("fill classifier plane");
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption(
            "Affine Hinge Classifier Plane (M2 active rows)",
            ("sans-serif", 28).into_font(),
        )
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..1.02f64, 0f64..1.02f64)
        .expect("build classifier plane");
    chart
        .configure_mesh()
        .x_desc("m2 same_gradient_share")
        .y_desc("m2 same_zero_seed_share")
        .draw()
        .expect("draw classifier plane mesh");

    for bundle in active_rows {
        let row = &bundle.affine_row;
        let radius = (5.0 + row.hinge_row.m1_anomaly_mass_pp.abs() * 1.5).round() as i32;
        let color = hinge_category_color(&row.hinge_row.hinge_category);
        chart
            .draw_series(std::iter::once(Circle::new(
                (
                    row.m2_affine_same_gradient_share,
                    row.m2_affine_same_zero_seed_share,
                ),
                radius,
                ShapeStyle::from(&color).filled(),
            )))
            .expect("draw classifier plane point");
    }
    root.present().expect("present classifier plane");
}

fn render_relation_heatmap(
    representative_rows: &[RepresentativeAffineRow],
    all_rows: &[RowBundle],
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1320, 760)).into_drawing_area();
    root.fill(&WHITE).expect("fill relation heatmap");
    let representative_bundles = representative_rows
        .iter()
        .map(|row| {
            all_rows
                .iter()
                .find(|bundle| {
                    bundle.affine_row.hinge_row.base == row.base
                        && bundle.affine_row.hinge_row.pair_label == row.pair_label
                })
                .expect("representative bundle should exist")
        })
        .collect::<Vec<_>>();
    let moduli = representative_bundles
        .iter()
        .flat_map(|bundle| {
            bundle
                .m2_comparison
                .modulus_rows
                .iter()
                .map(|row| row.modulus)
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let row_height = 60i32;
    let column_width = 84i32;
    let origin_x = 180i32;
    let origin_y = 80i32;

    root.draw(&Text::new(
        "Affine Relation Heatmap (M2)",
        (30, 30),
        ("sans-serif", 28).into_font(),
    ))
    .expect("title");

    for (column, modulus) in moduli.iter().enumerate() {
        let x = origin_x + column as i32 * column_width + column_width / 2;
        root.draw(&Text::new(
            format!("p{modulus}"),
            (x - 18, origin_y - 20),
            ("sans-serif", 18).into_font(),
        ))
        .expect("modulus label");
    }

    for (row_index, bundle) in representative_bundles.iter().enumerate() {
        let y = origin_y + row_index as i32 * row_height;
        root.draw(&Text::new(
            format!(
                "{} {}:{}",
                representative_rows[row_index].role,
                bundle.affine_row.hinge_row.base,
                bundle.affine_row.hinge_row.pair_label
            ),
            (20, y + 25),
            ("sans-serif", 18).into_font(),
        ))
        .expect("row label");
        let relation_by_modulus = bundle
            .m2_comparison
            .modulus_rows
            .iter()
            .map(|row| (row.modulus, row.local_relation_label.as_str()))
            .collect::<BTreeMap<_, _>>();
        for (column, modulus) in moduli.iter().enumerate() {
            let x0 = origin_x + column as i32 * column_width;
            let y0 = y;
            let relation = relation_by_modulus
                .get(modulus)
                .copied()
                .unwrap_or("missing");
            root.draw(&Rectangle::new(
                [(x0, y0), (x0 + column_width - 4, y0 + row_height - 8)],
                ShapeStyle::from(&relation_color(relation)).filled(),
            ))
            .expect("draw relation heatmap cell");
            root.draw(&Text::new(
                relation_label_short(relation),
                (x0 + 8, y0 + 30),
                ("sans-serif", 15).into_font().color(&BLACK),
            ))
            .expect("draw relation heatmap text");
        }
    }

    root.present().expect("present relation heatmap");
}

fn render_seed_class_strip(
    representative_rows: &[RepresentativeAffineRow],
    all_rows: &[RowBundle],
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1380, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill seed class strip");
    let representative_bundles = representative_rows
        .iter()
        .map(|row| {
            all_rows
                .iter()
                .find(|bundle| {
                    bundle.affine_row.hinge_row.base == row.base
                        && bundle.affine_row.hinge_row.pair_label == row.pair_label
                })
                .expect("representative bundle should exist")
        })
        .collect::<Vec<_>>();
    let moduli = representative_bundles
        .iter()
        .flat_map(|bundle| {
            bundle
                .m1_comparison
                .modulus_rows
                .iter()
                .chain(bundle.m2_comparison.modulus_rows.iter())
                .map(|row| row.modulus)
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let row_height = 52i32;
    let column_width = 88i32;
    let origin_x = 230i32;
    let origin_y = 80i32;

    root.draw(&Text::new(
        "Affine Zero-Seed Strip (M1 and M2)",
        (30, 30),
        ("sans-serif", 28).into_font(),
    ))
    .expect("title");
    for (column, modulus) in moduli.iter().enumerate() {
        let x = origin_x + column as i32 * column_width + column_width / 2;
        root.draw(&Text::new(
            format!("p{modulus}"),
            (x - 18, origin_y - 20),
            ("sans-serif", 18).into_font(),
        ))
        .expect("modulus label");
    }

    let mut row_cursor = 0usize;
    for bundle in &representative_bundles {
        for (phase, comparison) in [("M1", &bundle.m1_comparison), ("M2", &bundle.m2_comparison)] {
            let y = origin_y + row_cursor as i32 * row_height;
            row_cursor += 1;
            root.draw(&Text::new(
                format!(
                    "{} {}:{}",
                    phase, bundle.affine_row.hinge_row.base, bundle.affine_row.hinge_row.pair_label
                ),
                (20, y + 22),
                ("sans-serif", 17).into_font(),
            ))
            .expect("row label");
            let modulus_rows = comparison
                .modulus_rows
                .iter()
                .map(|row| (row.modulus, row))
                .collect::<BTreeMap<_, _>>();
            for (column, modulus) in moduli.iter().enumerate() {
                let x0 = origin_x + column as i32 * column_width;
                let y0 = y;
                if let Some(row) = modulus_rows.get(modulus) {
                    root.draw(&Rectangle::new(
                        [(x0, y0), (x0 + column_width - 4, y0 + row_height - 6)],
                        ShapeStyle::from(&relation_color(&row.local_relation_label)).filled(),
                    ))
                    .expect("draw strip cell");
                    root.draw(&Text::new(
                        format!("{}→{}", row.zero_seed_class_from, row.zero_seed_class_to),
                        (x0 + 8, y0 + 18),
                        ("sans-serif", 14).into_font(),
                    ))
                    .expect("draw seed class text");
                    root.draw(&Text::new(
                        format!("s{} g{}", row.shift_equal as u8, row.gradient_equal as u8),
                        (x0 + 8, y0 + 36),
                        ("sans-serif", 12).into_font(),
                    ))
                    .expect("draw equality flags");
                } else {
                    root.draw(&Rectangle::new(
                        [(x0, y0), (x0 + column_width - 4, y0 + row_height - 6)],
                        ShapeStyle::from(&RGBColor(235, 235, 235)).filled(),
                    ))
                    .expect("draw missing cell");
                }
            }
        }
    }

    root.present().expect("present seed class strip");
}

fn hinge_category_color(category: &str) -> RGBColor {
    match category {
        HINGE_CATEGORY_PERSISTENT_CORE => RGBColor(32, 120, 74),
        HINGE_CATEGORY_PERSISTENCE_ONLY => RGBColor(221, 142, 44),
        HINGE_CATEGORY_CORE_ONLY => RGBColor(62, 95, 173),
        HINGE_CATEGORY_ACTIVE_NEITHER => RGBColor(179, 67, 67),
        _ => RGBColor(120, 120, 120),
    }
}

fn relation_color(label: &str) -> RGBColor {
    match label {
        "identity" => RGBColor(45, 143, 90),
        "shift_only" => RGBColor(214, 153, 54),
        "gradient_only" => RGBColor(78, 114, 191),
        "shift_and_gradient" => RGBColor(183, 72, 72),
        _ => RGBColor(220, 220, 220),
    }
}

fn relation_label_short(label: &str) -> &'static str {
    match label {
        "identity" => "id",
        "shift_only" => "shift",
        "gradient_only" => "grad",
        "shift_and_gradient" => "both",
        _ => "na",
    }
}
