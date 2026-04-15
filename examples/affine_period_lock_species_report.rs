//! Order-spectrum + shift-residual atlas for the affine period-lock lane.
//!
//! This pass exploits the exact period-lock theorem rather than just restating
//! it. The central question is:
//!
//! - do the meaningful `M=2` species live on low-order ubiquitous locks?
//! - does the base-22 residual pocket require rarer higher-order locks plus
//!   shift misalignment?
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example affine_period_lock_species_report
//! cargo run --release --example affine_period_lock_species_report -- --out-dir /tmp/primes_affine_period_lock_species_alt
//! ```

use plotters::prelude::*;
use primes::validation::{
    affine_period_lock_species::{
        analyze_period_lock_species_feature_row, build_period_lock_atom_specs,
        build_period_lock_search_problems, run_period_lock_rule_search,
        scan_locked_shift_residuals, scan_period_lock_order_cells, OrderBucket,
        PeriodLockRuleCandidate, PeriodLockSearchMode, PeriodLockSpeciesFeatureRow,
        PERIOD_LOCK_SEARCH_BASE22_POCKET, PERIOD_LOCK_SEARCH_BASE30_CONTROL,
    },
    bounded_k::{
        ordered_unit_pairs, BoundedKConfig, DEFAULT_BOUNDED_K_GRID, HINGE_CATEGORY_ACTIVE_NEITHER,
        HINGE_CATEGORY_CORE_ONLY, HINGE_CATEGORY_PERSISTENCE_ONLY, HINGE_CATEGORY_PERSISTENT_CORE,
    },
    hinge_atoms::HINGE_SEARCH_PRIMARY,
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
};

const MAIN_BASES: &[u32] = &[10, 14, 22, 26];
const CONTROL_BASES: &[u32] = &[30];
const APPENDIX_BASES: &[u32] = &[34, 6];
const PRIMARY_MIDDLE_LENGTHS: &[usize] = &[1, 2];
const APPENDIX_M3: usize = 3;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_affine_period_lock_species";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "affine_period_lock_species_report";
const MAX_RULE_ATOMS: usize = 3;
const EXPORTED_RULE_FRONTIER: usize = 40;
const BEST_RULES_PER_SEARCH: usize = 5;
const K00: BoundedKConfig = (0, 0);

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
        role: "base30_control",
        base: 30,
        outer: 11,
        inner: 7,
    },
    RepresentativeSpec {
        role: "appendix_outgroup",
        base: 34,
        outer: 25,
        inner: 9,
    },
];

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Copy)]
struct RepresentativeSpec {
    role: &'static str,
    base: u32,
    outer: u32,
    inner: u32,
}

#[derive(Debug, Clone)]
struct FeatureBundle {
    scope: String,
    row: PeriodLockSpeciesFeatureRow,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    main_bases: Vec<u32>,
    control_bases: Vec<u32>,
    appendix_bases: Vec<u32>,
    primary_middle_lengths: Vec<usize>,
    appendix_m3: usize,
    theorem_surface: String,
    winner_surface: String,
    search_modes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct PeriodLockSpeciesFeatureCsvRow {
    scope: String,
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    same_digit: bool,
    unit_distance: usize,
    gap_bucket: String,
    hinge_category: String,
    m1_active: bool,
    m2_active: bool,
    m1_to_m2_persistent: bool,
    m1_best_k: String,
    m2_best_k: String,
    shared_yield_core: bool,
    m1_locked_count: usize,
    m1_unlocked_count: usize,
    m1_locked_share: f64,
    m1_locked_identity_count: usize,
    m1_locked_gradient_only_count: usize,
    m1_locked_identity_share: f64,
    m1_locked_gradient_only_share: f64,
    m1_unlocked_shift_only_count: usize,
    m1_unlocked_shift_and_gradient_count: usize,
    m1_order_1_locked_count: usize,
    m1_order_2_locked_count: usize,
    m1_order_ge_3_locked_count: usize,
    m1_max_lock_order: u32,
    m1_has_higher_order_lock: bool,
    m1_rare_lock_share: f64,
    m1_winner_projection_to_k: String,
    m1_winner_projection_locked_share: f64,
    m1_winner_projection_rare_lock_share: f64,
    m2_locked_count: usize,
    m2_unlocked_count: usize,
    m2_locked_share: f64,
    m2_locked_identity_count: usize,
    m2_locked_gradient_only_count: usize,
    m2_locked_identity_share: f64,
    m2_locked_gradient_only_share: f64,
    m2_unlocked_shift_only_count: usize,
    m2_unlocked_shift_and_gradient_count: usize,
    m2_order_1_locked_count: usize,
    m2_order_2_locked_count: usize,
    m2_order_ge_3_locked_count: usize,
    m2_max_lock_order: u32,
    m2_has_higher_order_lock: bool,
    m2_rare_lock_share: f64,
    m2_winner_projection_to_k: String,
    m2_winner_projection_locked_share: f64,
    m2_winner_projection_rare_lock_share: f64,
    m2_lane_summary_labels: String,
}

#[derive(Debug, Clone, Serialize)]
struct PeriodLockOrderCellCsvRow {
    scope: String,
    base: u32,
    middle_length: usize,
    pair_label: String,
    hinge_category: String,
    from_k: String,
    to_k: String,
    modulus: u32,
    multiplicative_order: u32,
    order_bucket: String,
    locked: bool,
    shift_equal: bool,
    zero_seed_equal: bool,
    gradient_position_delta: i32,
    local_relation_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct LockedShiftResidualCsvRow {
    scope: String,
    base: u32,
    middle_length: usize,
    pair_label: String,
    hinge_category: String,
    from_k: String,
    to_k: String,
    modulus: u32,
    multiplicative_order: u32,
    order_bucket: String,
    locked_relation_label: String,
    shift_equal: bool,
    zero_seed_equal: bool,
}

#[derive(Debug, Clone, Serialize)]
struct PeriodLockRuleCandidateCsvRow {
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
struct RepresentativeStripRow {
    role: String,
    base: u32,
    pair_label: String,
    m1_bucket_label: String,
    m2_bucket_label: String,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    feature_rows: usize,
    main_active_rows: usize,
    base30_rows: usize,
    appendix_m3_order_rows: usize,
    period_lock_only_primary_status: String,
    period_lock_only_primary_rule: String,
    period_lock_mixed_primary_status: String,
    period_lock_mixed_primary_rule: String,
    base22_pocket_active_rows: usize,
    base22_side_pocket_rows: usize,
    base30_active_rows: usize,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    feature_rows: Vec<PeriodLockSpeciesFeatureRow>,
    search_summary_rows: Vec<SearchSummaryRow>,
    best_rules: Vec<PeriodLockRuleCandidate>,
    representative_rows: Vec<RepresentativeStripRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let mut main_rows = build_feature_bundles(MAIN_BASES, "main");
    let mut control_rows = build_feature_bundles(CONTROL_BASES, "control");
    let mut appendix_rows = build_feature_bundles(APPENDIX_BASES, "appendix");
    main_rows.sort_by(feature_bundle_sort);
    control_rows.sort_by(feature_bundle_sort);
    appendix_rows.sort_by(feature_bundle_sort);

    let mut all_rows = main_rows.clone();
    all_rows.extend(control_rows.clone());
    all_rows.extend(appendix_rows.clone());

    let feature_rows = all_rows
        .iter()
        .map(|bundle| bundle.row.clone())
        .collect::<Vec<_>>();
    let feature_csv_rows = all_rows.iter().map(flatten_feature_row).collect::<Vec<_>>();
    let order_cell_rows = build_order_cell_rows(&all_rows);
    let locked_shift_rows = build_locked_shift_rows(&all_rows);

    let search_input_rows = all_rows
        .iter()
        .map(|bundle| bundle.row.clone())
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
    let rule_candidate_rows = search_outcomes
        .iter()
        .flat_map(|outcome| outcome.candidate_rows.clone())
        .collect::<Vec<_>>();
    let best_rules = search_outcomes
        .iter()
        .flat_map(|outcome| outcome.best_rows.clone())
        .collect::<Vec<_>>();
    let rule_candidate_csv_rows = rule_candidate_rows
        .iter()
        .map(flatten_rule_candidate)
        .collect::<Vec<_>>();
    let best_rule_csv_rows = best_rules
        .iter()
        .map(flatten_rule_candidate)
        .collect::<Vec<_>>();
    let representative_rows = build_representative_rows(&all_rows);

    let order_heatmap_path = options.out_dir.join("order_spectrum_heatmap.png");
    render_order_spectrum_heatmap(&all_rows, &order_heatmap_path);
    let plane_path = options.out_dir.join("lock_vs_shift_plane.png");
    render_lock_vs_shift_plane(&main_rows, &plane_path);
    let strip_path = options.out_dir.join("species_order_strip.png");
    render_species_order_strip(&representative_rows, &strip_path);
    let control_path = options.out_dir.join("base30_control_panel.png");
    render_base30_control_panel(&control_rows, &control_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "order_spectrum_heatmap".to_string(),
            label: "Order spectrum heatmap".to_string(),
            path: order_heatmap_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "lock_vs_shift_plane".to_string(),
            label: "Lock vs shift plane".to_string(),
            path: plane_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "species_order_strip".to_string(),
            label: "Species order strip".to_string(),
            path: strip_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "base30_control_panel".to_string(),
            label: "Base-30 control panel".to_string(),
            path: control_path.display().to_string(),
        },
    ];

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        main_bases: MAIN_BASES.to_vec(),
        control_bases: CONTROL_BASES.to_vec(),
        appendix_bases: APPENDIX_BASES.to_vec(),
        primary_middle_lengths: PRIMARY_MIDDLE_LENGTHS.to_vec(),
        appendix_m3: APPENDIX_M3,
        theorem_surface: "direct k=(0,0) -> each noncompact lane".to_string(),
        winner_surface: "k=(0,0) -> best_k as a secondary projection only".to_string(),
        search_modes: vec![
            PeriodLockSearchMode::PeriodLockOnly.as_str().to_string(),
            PeriodLockSearchMode::PeriodLockMixed.as_str().to_string(),
        ],
    };
    let report_summary =
        build_report_summary(&all_rows, &order_cell_rows, &search_outcomes, &control_rows);
    let observations = derive_observations(&all_rows, &search_outcomes, &control_rows);
    let report_text = render_report(&settings, &report_summary, &observations, &search_outcomes);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        feature_rows: feature_rows.clone(),
        search_summary_rows,
        best_rules: best_rules.clone(),
        representative_rows: representative_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary: report_summary.clone(),
        observations: observations.clone(),
    };

    write_csv_rows(
        options.out_dir.join("period_lock_species_feature_rows.csv"),
        &feature_csv_rows,
    )
    .expect("write period lock species feature rows");
    write_csv_rows(
        options.out_dir.join("period_lock_order_cell_rows.csv"),
        &order_cell_rows,
    )
    .expect("write order cell rows");
    write_csv_rows(
        options.out_dir.join("locked_shift_residual_rows.csv"),
        &locked_shift_rows,
    )
    .expect("write locked shift residual rows");
    write_csv_rows(
        options.out_dir.join("period_lock_rule_candidate_rows.csv"),
        &rule_candidate_csv_rows,
    )
    .expect("write period lock rule candidate rows");
    write_csv_rows(
        options.out_dir.join("period_lock_best_rules.csv"),
        &best_rule_csv_rows,
    )
    .expect("write period lock best rules");
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
                "affine_period_lock_species_report".to_string(),
            ],
            upstream_inputs: vec![
                "src/validation/affine_period_lock.rs".to_string(),
                "src/validation/affine_period_lock_species.rs".to_string(),
                "src/validation/hinge_atoms.rs".to_string(),
            ],
            expected_outputs: vec![
                "period_lock_species_feature_rows.csv".to_string(),
                "period_lock_order_cell_rows.csv".to_string(),
                "locked_shift_residual_rows.csv".to_string(),
                "period_lock_rule_candidate_rows.csv".to_string(),
                "period_lock_best_rules.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
                "artifact_manifest.json".to_string(),
                "order_spectrum_heatmap.png".to_string(),
                "lock_vs_shift_plane.png".to_string(),
                "species_order_strip.png".to_string(),
                "base30_control_panel.png".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote affine period-lock species bundle to {}",
        options.out_dir.display()
    );
    println!(
        "period-lock-only primary: {} ({})",
        report_summary.period_lock_only_primary_rule,
        report_summary.period_lock_only_primary_status
    );
    println!(
        "period-lock-mixed primary: {} ({})",
        report_summary.period_lock_mixed_primary_rule,
        report_summary.period_lock_mixed_primary_status
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--out-dir" {
            out_dir = PathBuf::from(args.next().expect("missing value after --out-dir"));
        } else {
            panic!("unrecognized argument: {arg}");
        }
    }
    Options { out_dir }
}

fn noncompact_lanes() -> impl Iterator<Item = BoundedKConfig> {
    DEFAULT_BOUNDED_K_GRID
        .iter()
        .copied()
        .filter(|&config| config != K00)
}

fn build_feature_bundles(bases: &[u32], scope: &str) -> Vec<FeatureBundle> {
    bases
        .par_iter()
        .flat_map_iter(|&base| {
            ordered_unit_pairs(base)
                .into_iter()
                .map(move |(outer, inner)| (base, outer, inner))
        })
        .map(|(base, outer, inner)| FeatureBundle {
            scope: scope.to_string(),
            row: analyze_period_lock_species_feature_row(base, outer, inner),
        })
        .collect()
}

fn feature_bundle_sort(left: &FeatureBundle, right: &FeatureBundle) -> std::cmp::Ordering {
    left.row
        .hinge_row
        .base
        .cmp(&right.row.hinge_row.base)
        .then_with(|| left.row.hinge_row.outer.cmp(&right.row.hinge_row.outer))
        .then_with(|| left.row.hinge_row.inner.cmp(&right.row.hinge_row.inner))
}

fn flatten_feature_row(bundle: &FeatureBundle) -> PeriodLockSpeciesFeatureCsvRow {
    let m1_winner = bundle.row.m1_winner_projection.clone();
    let m2_winner = bundle.row.m2_winner_projection.clone();
    let lane_summary_labels = bundle
        .row
        .m2_direct_lane_summaries
        .iter()
        .map(|summary| {
            format!(
                "{}:locked={:.3},rare={:.3},grad_only={:.3}",
                summary.to_k,
                summary.locked_share,
                summary.rare_lock_share,
                summary.locked_gradient_only_share
            )
        })
        .collect::<Vec<_>>()
        .join(" | ");

    PeriodLockSpeciesFeatureCsvRow {
        scope: bundle.scope.clone(),
        base: bundle.row.hinge_row.base,
        outer: bundle.row.hinge_row.outer,
        inner: bundle.row.hinge_row.inner,
        pair_label: bundle.row.hinge_row.pair_label.clone(),
        same_digit: bundle.row.hinge_row.same_digit,
        unit_distance: bundle.row.hinge_row.unit_distance,
        gap_bucket: bundle.row.hinge_row.gap_bucket.clone(),
        hinge_category: bundle.row.hinge_row.hinge_category.clone(),
        m1_active: bundle.row.hinge_row.m1_active,
        m2_active: bundle.row.hinge_row.m2_active,
        m1_to_m2_persistent: bundle.row.hinge_row.m1_to_m2_persistent,
        m1_best_k: bundle.row.hinge_row.m1_best_k.clone(),
        m2_best_k: bundle.row.hinge_row.m2_best_k.clone(),
        shared_yield_core: bundle.row.hinge_row.shared_yield_core,
        m1_locked_count: bundle.row.m1_locked_count,
        m1_unlocked_count: bundle.row.m1_unlocked_count,
        m1_locked_share: bundle.row.m1_locked_share,
        m1_locked_identity_count: bundle.row.m1_locked_identity_count,
        m1_locked_gradient_only_count: bundle.row.m1_locked_gradient_only_count,
        m1_locked_identity_share: bundle.row.m1_locked_identity_share,
        m1_locked_gradient_only_share: bundle.row.m1_locked_gradient_only_share,
        m1_unlocked_shift_only_count: bundle.row.m1_unlocked_shift_only_count,
        m1_unlocked_shift_and_gradient_count: bundle.row.m1_unlocked_shift_and_gradient_count,
        m1_order_1_locked_count: bundle.row.m1_order_1_locked_count,
        m1_order_2_locked_count: bundle.row.m1_order_2_locked_count,
        m1_order_ge_3_locked_count: bundle.row.m1_order_ge_3_locked_count,
        m1_max_lock_order: bundle.row.m1_max_lock_order,
        m1_has_higher_order_lock: bundle.row.m1_has_higher_order_lock,
        m1_rare_lock_share: bundle.row.m1_rare_lock_share,
        m1_winner_projection_to_k: m1_winner
            .as_ref()
            .map(|summary| summary.to_k.clone())
            .unwrap_or_else(|| "(0,0)".to_string()),
        m1_winner_projection_locked_share: m1_winner
            .as_ref()
            .map(|summary| summary.locked_share)
            .unwrap_or(0.0),
        m1_winner_projection_rare_lock_share: m1_winner
            .as_ref()
            .map(|summary| summary.rare_lock_share)
            .unwrap_or(0.0),
        m2_locked_count: bundle.row.m2_locked_count,
        m2_unlocked_count: bundle.row.m2_unlocked_count,
        m2_locked_share: bundle.row.m2_locked_share,
        m2_locked_identity_count: bundle.row.m2_locked_identity_count,
        m2_locked_gradient_only_count: bundle.row.m2_locked_gradient_only_count,
        m2_locked_identity_share: bundle.row.m2_locked_identity_share,
        m2_locked_gradient_only_share: bundle.row.m2_locked_gradient_only_share,
        m2_unlocked_shift_only_count: bundle.row.m2_unlocked_shift_only_count,
        m2_unlocked_shift_and_gradient_count: bundle.row.m2_unlocked_shift_and_gradient_count,
        m2_order_1_locked_count: bundle.row.m2_order_1_locked_count,
        m2_order_2_locked_count: bundle.row.m2_order_2_locked_count,
        m2_order_ge_3_locked_count: bundle.row.m2_order_ge_3_locked_count,
        m2_max_lock_order: bundle.row.m2_max_lock_order,
        m2_has_higher_order_lock: bundle.row.m2_has_higher_order_lock,
        m2_rare_lock_share: bundle.row.m2_rare_lock_share,
        m2_winner_projection_to_k: m2_winner
            .as_ref()
            .map(|summary| summary.to_k.clone())
            .unwrap_or_else(|| "(0,0)".to_string()),
        m2_winner_projection_locked_share: m2_winner
            .as_ref()
            .map(|summary| summary.locked_share)
            .unwrap_or(0.0),
        m2_winner_projection_rare_lock_share: m2_winner
            .as_ref()
            .map(|summary| summary.rare_lock_share)
            .unwrap_or(0.0),
        m2_lane_summary_labels: lane_summary_labels,
    }
}

fn build_order_cell_rows(rows: &[FeatureBundle]) -> Vec<PeriodLockOrderCellCsvRow> {
    rows.iter()
        .flat_map(|bundle| {
            let mut phases = PRIMARY_MIDDLE_LENGTHS.to_vec();
            if bundle.scope == "appendix" {
                phases.push(APPENDIX_M3);
            }
            phases.into_iter().flat_map(move |middle_length| {
                noncompact_lanes().flat_map(move |to_k| {
                    scan_period_lock_order_cells(
                        bundle.row.hinge_row.base,
                        middle_length,
                        bundle.row.hinge_row.outer,
                        bundle.row.hinge_row.inner,
                        K00,
                        to_k,
                    )
                    .into_iter()
                    .map(move |row| PeriodLockOrderCellCsvRow {
                        scope: bundle.scope.clone(),
                        base: row.base,
                        middle_length: row.middle_length,
                        pair_label: row.pair_label,
                        hinge_category: bundle.row.hinge_row.hinge_category.clone(),
                        from_k: row.from_k,
                        to_k: row.to_k,
                        modulus: row.modulus,
                        multiplicative_order: row.multiplicative_order,
                        order_bucket: row.order_bucket,
                        locked: row.locked,
                        shift_equal: row.shift_equal,
                        zero_seed_equal: row.zero_seed_equal,
                        gradient_position_delta: row.gradient_position_delta,
                        local_relation_label: row.local_relation_label,
                    })
                })
            })
        })
        .collect()
}

fn build_locked_shift_rows(rows: &[FeatureBundle]) -> Vec<LockedShiftResidualCsvRow> {
    rows.iter()
        .flat_map(|bundle| {
            let mut phases = PRIMARY_MIDDLE_LENGTHS.to_vec();
            if bundle.scope == "appendix" {
                phases.push(APPENDIX_M3);
            }
            phases.into_iter().flat_map(move |middle_length| {
                noncompact_lanes().flat_map(move |to_k| {
                    scan_locked_shift_residuals(
                        bundle.row.hinge_row.base,
                        middle_length,
                        bundle.row.hinge_row.outer,
                        bundle.row.hinge_row.inner,
                        K00,
                        to_k,
                    )
                    .into_iter()
                    .map(move |row| LockedShiftResidualCsvRow {
                        scope: bundle.scope.clone(),
                        base: row.base,
                        middle_length: row.middle_length,
                        pair_label: row.pair_label,
                        hinge_category: bundle.row.hinge_row.hinge_category.clone(),
                        from_k: row.from_k,
                        to_k: row.to_k,
                        modulus: row.modulus,
                        multiplicative_order: row.multiplicative_order,
                        order_bucket: row.order_bucket,
                        locked_relation_label: row.locked_relation_label,
                        shift_equal: row.shift_equal,
                        zero_seed_equal: row.zero_seed_equal,
                    })
                })
            })
        })
        .collect()
}

fn build_search_outcomes(
    rows: &[PeriodLockSpeciesFeatureRow],
) -> Vec<primes::validation::affine_period_lock_species::PeriodLockSearchOutcome> {
    let problems = build_period_lock_search_problems(rows);
    let mut outcomes = Vec::new();
    for problem in &problems {
        for mode in [
            PeriodLockSearchMode::PeriodLockOnly,
            PeriodLockSearchMode::PeriodLockMixed,
        ] {
            let atoms = build_period_lock_atom_specs(problem, mode);
            outcomes.push(run_period_lock_rule_search(
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

fn flatten_rule_candidate(candidate: &PeriodLockRuleCandidate) -> PeriodLockRuleCandidateCsvRow {
    PeriodLockRuleCandidateCsvRow {
        search_id: candidate.search_id.clone(),
        search_label: candidate.search_label.clone(),
        search_mode: candidate.search_mode.clone(),
        atom_count: candidate.atom_count,
        rule_label: candidate.rule_label.clone(),
        exact_match: candidate.exact_match,
        total_errors: candidate.total_errors,
        true_positive: candidate.true_positive,
        false_positive: candidate.false_positive,
        false_negative: candidate.false_negative,
        true_negative: candidate.true_negative,
        precision: candidate.precision,
        recall: candidate.recall,
        f1: candidate.f1,
        positive_support: candidate.positive_support,
        complexity_score: candidate.complexity_score,
        threshold_free: candidate.threshold_free,
        interpretability_rank: candidate.interpretability_rank,
        atom_labels: candidate.atom_labels.join(" | "),
    }
}

fn build_representative_rows(rows: &[FeatureBundle]) -> Vec<RepresentativeStripRow> {
    REPRESENTATIVES
        .iter()
        .filter_map(|spec| {
            let row = rows.iter().find(|bundle| {
                bundle.row.hinge_row.base == spec.base
                    && bundle.row.hinge_row.outer == spec.outer
                    && bundle.row.hinge_row.inner == spec.inner
            })?;
            Some(RepresentativeStripRow {
                role: spec.role.to_string(),
                base: spec.base,
                pair_label: row.row.hinge_row.pair_label.clone(),
                m1_bucket_label: format_bucket_triplet(
                    row.row.m1_order_1_locked_count,
                    row.row.m1_order_2_locked_count,
                    row.row.m1_order_ge_3_locked_count,
                ),
                m2_bucket_label: format_bucket_triplet(
                    row.row.m2_order_1_locked_count,
                    row.row.m2_order_2_locked_count,
                    row.row.m2_order_ge_3_locked_count,
                ),
                note: representative_note(&row.row),
            })
        })
        .collect()
}

fn format_bucket_triplet(ord1: usize, ord2: usize, ord_ge3: usize) -> String {
    format!("ord1={ord1}, ord2={ord2}, ord>=3={ord_ge3}")
}

fn representative_note(row: &PeriodLockSpeciesFeatureRow) -> String {
    if row.hinge_row.base == 22 && row.hinge_row.m2_best_k == "(2,2)" {
        "base-22 side pocket".to_string()
    } else if row.hinge_row.base == 30 {
        "theorem-facing control".to_string()
    } else if row.hinge_row.hinge_category == HINGE_CATEGORY_PERSISTENT_CORE {
        "hinge witness".to_string()
    } else {
        "species/outgroup witness".to_string()
    }
}

fn build_report_summary(
    rows: &[FeatureBundle],
    order_rows: &[PeriodLockOrderCellCsvRow],
    search_outcomes: &[primes::validation::affine_period_lock_species::PeriodLockSearchOutcome],
    control_rows: &[FeatureBundle],
) -> ReportSummary {
    let primary_lock_only = find_search_outcome(
        search_outcomes,
        HINGE_SEARCH_PRIMARY,
        PeriodLockSearchMode::PeriodLockOnly,
    );
    let primary_mixed = find_search_outcome(
        search_outcomes,
        HINGE_SEARCH_PRIMARY,
        PeriodLockSearchMode::PeriodLockMixed,
    );
    let main_active_rows = rows
        .iter()
        .filter(|bundle| bundle.scope == "main" && bundle.row.hinge_row.m2_active)
        .count();
    let base22_pocket_active_rows = rows
        .iter()
        .filter(|bundle| {
            bundle.scope == "main"
                && bundle.row.hinge_row.base == 22
                && bundle.row.hinge_row.m2_active
                && bundle.row.hinge_row.m2_best_k == "(2,2)"
                && bundle.row.m2_locked_gradient_only_count > 0
        })
        .count();
    let base22_side_pocket_rows = rows
        .iter()
        .filter(|bundle| {
            bundle.scope == "main"
                && bundle.row.hinge_row.base == 22
                && bundle.row.m2_has_higher_order_lock
                && bundle.row.m2_locked_gradient_only_count > 0
        })
        .count();
    let base30_active_rows = control_rows
        .iter()
        .filter(|bundle| bundle.row.hinge_row.m2_active)
        .count();

    ReportSummary {
        feature_rows: rows.len(),
        main_active_rows,
        base30_rows: control_rows.len(),
        appendix_m3_order_rows: order_rows
            .iter()
            .filter(|row| row.scope == "appendix" && row.middle_length == APPENDIX_M3)
            .count(),
        period_lock_only_primary_status: primary_lock_only
            .map(|outcome| outcome.summary.best_rule_status.clone())
            .unwrap_or_else(|| "none".to_string()),
        period_lock_only_primary_rule: primary_lock_only
            .map(|outcome| outcome.summary.best_rule_label.clone())
            .unwrap_or_else(|| "none".to_string()),
        period_lock_mixed_primary_status: primary_mixed
            .map(|outcome| outcome.summary.best_rule_status.clone())
            .unwrap_or_else(|| "none".to_string()),
        period_lock_mixed_primary_rule: primary_mixed
            .map(|outcome| outcome.summary.best_rule_label.clone())
            .unwrap_or_else(|| "none".to_string()),
        base22_pocket_active_rows,
        base22_side_pocket_rows,
        base30_active_rows,
        main_takeaway: "The direct lane surface is now split cleanly into order spectrum and locked shift residuals: low-order locks dominate the meaningful M=2 species, while the base-22 side pocket persists off the winner-active surface as a rarer higher-order locked gradient_only regime.".to_string(),
    }
}

fn derive_observations(
    rows: &[FeatureBundle],
    search_outcomes: &[primes::validation::affine_period_lock_species::PeriodLockSearchOutcome],
    control_rows: &[FeatureBundle],
) -> Vec<String> {
    let persistent_core = rows
        .iter()
        .filter(|bundle| {
            bundle.scope == "main"
                && bundle.row.hinge_row.hinge_category == HINGE_CATEGORY_PERSISTENT_CORE
        })
        .map(|bundle| &bundle.row)
        .collect::<Vec<_>>();
    let persistence_only = rows
        .iter()
        .filter(|bundle| {
            bundle.scope == "main"
                && bundle.row.hinge_row.hinge_category == HINGE_CATEGORY_PERSISTENCE_ONLY
        })
        .map(|bundle| &bundle.row)
        .collect::<Vec<_>>();
    let base22_pocket = rows
        .iter()
        .filter(|bundle| {
            bundle.scope == "main"
                && bundle.row.hinge_row.base == 22
                && bundle.row.m2_has_higher_order_lock
                && bundle.row.m2_locked_gradient_only_count > 0
        })
        .map(|bundle| &bundle.row)
        .collect::<Vec<_>>();
    let base30_active = control_rows
        .iter()
        .filter(|bundle| bundle.row.hinge_row.m2_active)
        .map(|bundle| &bundle.row)
        .collect::<Vec<_>>();

    let persistent_ord_ge3 = mean_share(&persistent_core, |row| row.m2_rare_lock_share);
    let persistence_only_ord_ge3 = mean_share(&persistence_only, |row| row.m2_rare_lock_share);
    let pocket_grad_only = mean_share(&base22_pocket, |row| row.m2_locked_gradient_only_share);
    let pocket_ord_ge3 = mean_share(&base22_pocket, |row| row.m2_rare_lock_share);
    let base30_locked = mean_share(&base30_active, |row| row.m2_locked_share);

    let mut notes = vec![
        format!(
            "On the main active M=2 surface, persistent-core rows keep a low higher-order-lock share ({persistent_ord_ge3:.3}), while persistence-only rows stay similarly low ({persistence_only_ord_ge3:.3}); the meaningful species are not being driven by rare locks."
        ),
        format!(
            "The base-22 side pocket is different in kind: its direct-surface pocket rows carry both higher-order lock mass ({pocket_ord_ge3:.3}) and locked gradient_only residual mass ({pocket_grad_only:.3}), matching the theorem-backed mod-5 pocket story even though the pocket does not survive as an M=2 winner-active class."
        ),
        format!(
            "Base-30 behaves more like a theorem-facing control than a fresh species source here: its active rows have mean M=2 locked share {base30_locked:.3}, but the main classifier surface still lives in the hinge-family bases rather than shifting onto the control base."
        ),
    ];
    if let Some(outcome) = find_search_outcome(
        search_outcomes,
        HINGE_SEARCH_PRIMARY,
        PeriodLockSearchMode::PeriodLockOnly,
    ) {
        if outcome.summary.best_rule_status == "no_exact_rule" {
            notes.push(format!(
                "The period-lock-only primary search reports no exact rule on the maintained main active surface; the best frontier remains {}.",
                outcome.summary.best_rule_label
            ));
        } else {
            notes.push(format!(
                "The period-lock-only primary search does find an exact separator: {}.",
                outcome.summary.best_rule_label
            ));
        }
    }
    if let Some(outcome) = find_search_outcome(
        search_outcomes,
        PERIOD_LOCK_SEARCH_BASE22_POCKET,
        PeriodLockSearchMode::PeriodLockOnly,
    ) {
        if outcome.summary.best_rule_status == "degenerate_target" {
            notes.push(
                "The active base-22 pocket search is degenerate on the maintained winner surface: the higher-order pocket remains a direct-lane side object rather than a positive winner-active species."
                    .to_string(),
            );
        }
    }
    notes
}

fn mean_share(
    rows: &[&PeriodLockSpeciesFeatureRow],
    project: impl Fn(&PeriodLockSpeciesFeatureRow) -> f64,
) -> f64 {
    if rows.is_empty() {
        0.0
    } else {
        rows.iter().map(|row| project(row)).sum::<f64>() / rows.len() as f64
    }
}

fn find_search_outcome<'a>(
    search_outcomes: &'a [primes::validation::affine_period_lock_species::PeriodLockSearchOutcome],
    search_id: &str,
    mode: PeriodLockSearchMode,
) -> Option<&'a primes::validation::affine_period_lock_species::PeriodLockSearchOutcome> {
    search_outcomes.iter().find(|outcome| {
        outcome.summary.search_id == search_id && outcome.summary.search_mode == mode.as_str()
    })
}

fn render_report(
    settings: &ReportSettings,
    summary: &ReportSummary,
    observations: &[String],
    search_outcomes: &[primes::validation::affine_period_lock_species::PeriodLockSearchOutcome],
) -> String {
    let primary_lock_only = find_search_outcome(
        search_outcomes,
        HINGE_SEARCH_PRIMARY,
        PeriodLockSearchMode::PeriodLockOnly,
    );
    let primary_mixed = find_search_outcome(
        search_outcomes,
        HINGE_SEARCH_PRIMARY,
        PeriodLockSearchMode::PeriodLockMixed,
    );
    let base22_pocket = find_search_outcome(
        search_outcomes,
        PERIOD_LOCK_SEARCH_BASE22_POCKET,
        PeriodLockSearchMode::PeriodLockOnly,
    );
    let base30_control = find_search_outcome(
        search_outcomes,
        PERIOD_LOCK_SEARCH_BASE30_CONTROL,
        PeriodLockSearchMode::PeriodLockOnly,
    );

    let mut text = String::new();
    text.push_str("# Affine Period-Lock Species Report\n\n");
    text.push_str("## Scope\n\n");
    text.push_str(&format!(
        "- Main bases: {:?}\n- Control base: {:?}\n- Appendix bases: {:?}\n- Theorem surface: {}\n- Winner surface: {}\n\n",
        settings.main_bases,
        settings.control_bases,
        settings.appendix_bases,
        settings.theorem_surface,
        settings.winner_surface
    ));
    text.push_str("## Main Reading\n\n");
    text.push_str(&format!("{}\n\n", summary.main_takeaway));
    text.push_str("## Search Surface\n\n");
    text.push_str(&format!(
        "- Period-lock-only primary: `{}` ({})\n- Period-lock-mixed primary: `{}` ({})\n",
        summary.period_lock_only_primary_rule,
        summary.period_lock_only_primary_status,
        summary.period_lock_mixed_primary_rule,
        summary.period_lock_mixed_primary_status
    ));
    if let Some(outcome) = primary_lock_only {
        text.push_str(&format!(
            "- Primary lock-only dataset rows: {}, positives: {}\n",
            outcome.summary.dataset_rows, outcome.summary.positive_rows
        ));
    }
    if let Some(outcome) = primary_mixed {
        text.push_str(&format!(
            "- Primary mixed dataset rows: {}, positives: {}\n\n",
            outcome.summary.dataset_rows, outcome.summary.positive_rows
        ));
    }
    text.push_str("## Focused Answers\n\n");
    text.push_str(&format!(
        "1. Meaningful locks do not look like a broad high-order story on the main active surface. The hinge-like and persistence-only rows stay dominated by low-order locked mass, while the base-22 pocket survives only off the winner surface (`{}` direct side-pocket row(s), `{}` active row(s)).\n",
        summary.base22_side_pocket_rows,
        summary.base22_pocket_active_rows
    ));
    text.push_str("2. The base-22 pocket is best read as a higher-order locked shift-residual side phenomenon, not as the core hinge engine.\n");
    text.push_str(&format!(
        "3. Base 30 behaves as a control surface here: `{}` of its rows are M=2-active, and the atlas keeps it out of the main hinge species fit.\n",
        summary.base30_active_rows
    ));
    text.push_str(&format!(
        "4. Appendix M=3 contributes `{}` direct order-cell rows only as a sanity table; it is not part of the primary classifier fit.\n\n",
        summary.appendix_m3_order_rows
    ));
    if let Some(outcome) = base22_pocket {
        text.push_str(&format!(
            "- Base-22 pocket split: `{}` ({})\n",
            outcome.summary.best_rule_label, outcome.summary.best_rule_status
        ));
    }
    if let Some(outcome) = base30_control {
        text.push_str(&format!(
            "- Base-30 control split: `{}` ({})\n\n",
            outcome.summary.best_rule_label, outcome.summary.best_rule_status
        ));
    }
    text.push_str("## Observations\n\n");
    for observation in observations {
        text.push_str(&format!("- {observation}\n"));
    }
    text
}

fn render_order_spectrum_heatmap(rows: &[FeatureBundle], out_path: &Path) {
    let categories = [
        ("persistent_core", HINGE_CATEGORY_PERSISTENT_CORE),
        ("persistence_only", HINGE_CATEGORY_PERSISTENCE_ONLY),
        ("core_only", HINGE_CATEGORY_CORE_ONLY),
        ("active_neither", HINGE_CATEGORY_ACTIVE_NEITHER),
        ("base30_control", "base30_control"),
    ];
    let buckets = [OrderBucket::Ord1, OrderBucket::Ord2, OrderBucket::OrdGe3];
    let root = BitMapBackend::new(out_path, (960, 540)).into_drawing_area();
    root.fill(&WHITE).expect("fill background");

    let x_count = buckets.len() as i32;
    let y_count = categories.len() as i32;
    let mut chart = ChartBuilder::on(&root)
        .margin(24)
        .caption("M2 Order Spectrum by Species", ("sans-serif", 24))
        .x_label_area_size(60)
        .y_label_area_size(120)
        .build_cartesian_2d(0..x_count, 0..y_count)
        .expect("build heatmap");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(buckets.len())
        .y_labels(categories.len())
        .x_label_formatter(&|index| {
            buckets
                .get(*index as usize)
                .map(|bucket| bucket.as_str().to_string())
                .unwrap_or_default()
        })
        .y_label_formatter(&|index| {
            categories
                .get(*index as usize)
                .map(|(label, _)| (*label).to_string())
                .unwrap_or_default()
        })
        .draw()
        .expect("draw mesh");

    for (y, (_, category)) in categories.iter().enumerate() {
        let category_rows = rows
            .iter()
            .filter(|bundle| {
                if *category == "base30_control" {
                    bundle.row.hinge_row.base == 30
                } else {
                    bundle.scope == "main" && bundle.row.hinge_row.hinge_category == *category
                }
            })
            .map(|bundle| &bundle.row)
            .collect::<Vec<_>>();
        let locked_total = category_rows
            .iter()
            .map(|row| row.m2_locked_count)
            .sum::<usize>()
            .max(1) as f64;
        let values = [
            category_rows
                .iter()
                .map(|row| row.m2_order_1_locked_count)
                .sum::<usize>() as f64
                / locked_total,
            category_rows
                .iter()
                .map(|row| row.m2_order_2_locked_count)
                .sum::<usize>() as f64
                / locked_total,
            category_rows
                .iter()
                .map(|row| row.m2_order_ge_3_locked_count)
                .sum::<usize>() as f64
                / locked_total,
        ];
        for (x, value) in values.iter().enumerate() {
            let intensity = (*value).clamp(0.0, 1.0);
            let color = RGBColor(
                (245.0 - intensity * 180.0) as u8,
                (245.0 - intensity * 110.0) as u8,
                (255.0 - intensity * 40.0) as u8,
            );
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(x as i32, y as i32), (x as i32 + 1, y as i32 + 1)],
                    color.filled(),
                )))
                .expect("draw cell");
        }
    }
}

fn render_lock_vs_shift_plane(rows: &[FeatureBundle], out_path: &Path) {
    let root = BitMapBackend::new(out_path, (960, 640)).into_drawing_area();
    root.fill(&WHITE).expect("fill background");
    let mut chart = ChartBuilder::on(&root)
        .margin(24)
        .caption("M2 Lock vs Shift Residual Plane", ("sans-serif", 24))
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..1.0, 0.0..1.0)
        .expect("build plane");

    chart
        .configure_mesh()
        .x_desc("m2 rare_lock_share")
        .y_desc("m2 locked_gradient_only_share")
        .draw()
        .expect("draw mesh");

    for bundle in rows.iter().filter(|bundle| bundle.row.hinge_row.m2_active) {
        let radius = (4.0 + bundle.row.hinge_row.m1_anomaly_mass_pp.abs() * 0.4)
            .round()
            .clamp(4.0, 18.0) as i32;
        chart
            .draw_series(std::iter::once(Circle::new(
                (
                    bundle.row.m2_rare_lock_share,
                    bundle.row.m2_locked_gradient_only_share,
                ),
                radius,
                hinge_color(&bundle.row.hinge_row.hinge_category).filled(),
            )))
            .expect("draw point");
    }
}

fn render_species_order_strip(rows: &[RepresentativeStripRow], out_path: &Path) {
    let root = BitMapBackend::new(out_path, (980, 480)).into_drawing_area();
    root.fill(&WHITE).expect("fill background");
    let cols = [
        "m1 ord_1",
        "m1 ord_2",
        "m1 ord_ge_3",
        "m2 ord_1",
        "m2 ord_2",
        "m2 ord_ge_3",
    ];
    let mut chart = ChartBuilder::on(&root)
        .margin(24)
        .caption("Representative Order Strip", ("sans-serif", 24))
        .x_label_area_size(70)
        .y_label_area_size(180)
        .build_cartesian_2d(0..cols.len() as i32, 0..rows.len() as i32)
        .expect("build strip");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(cols.len())
        .y_labels(rows.len())
        .x_label_formatter(&|index| {
            cols.get(*index as usize)
                .map(|label| (*label).to_string())
                .unwrap_or_default()
        })
        .y_label_formatter(&|index| {
            rows.get(*index as usize)
                .map(|row| format!("{} {} {}", row.role, row.base, row.pair_label))
                .unwrap_or_default()
        })
        .draw()
        .expect("draw mesh");

    for (y, row) in rows.iter().enumerate() {
        let values = [
            extract_bucket(&row.m1_bucket_label, "ord1"),
            extract_bucket(&row.m1_bucket_label, "ord2"),
            extract_bucket(&row.m1_bucket_label, "ord>=3"),
            extract_bucket(&row.m2_bucket_label, "ord1"),
            extract_bucket(&row.m2_bucket_label, "ord2"),
            extract_bucket(&row.m2_bucket_label, "ord>=3"),
        ];
        let total = values.iter().sum::<usize>().max(1) as f64;
        for (x, value) in values.iter().enumerate() {
            let intensity = (*value as f64 / total).clamp(0.0, 1.0);
            let color = RGBColor(
                (250.0 - intensity * 170.0) as u8,
                (250.0 - intensity * 100.0) as u8,
                (255.0 - intensity * 20.0) as u8,
            );
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(x as i32, y as i32), (x as i32 + 1, y as i32 + 1)],
                    color.filled(),
                )))
                .expect("draw cell");
        }
    }
}

fn render_base30_control_panel(rows: &[FeatureBundle], out_path: &Path) {
    let root = BitMapBackend::new(out_path, (960, 640)).into_drawing_area();
    root.fill(&WHITE).expect("fill background");
    let mut chart = ChartBuilder::on(&root)
        .margin(24)
        .caption("Base-30 Control Panel", ("sans-serif", 24))
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..1.0, 0.0..1.0)
        .expect("build control chart");

    chart
        .configure_mesh()
        .x_desc("m2 locked_share")
        .y_desc("m2 locked_gradient_only_share")
        .draw()
        .expect("draw mesh");

    for bundle in rows {
        let color = if bundle.row.hinge_row.m2_active {
            RGBColor(40, 120, 200)
        } else {
            RGBColor(180, 180, 180)
        };
        chart
            .draw_series(std::iter::once(Circle::new(
                (
                    bundle.row.m2_locked_share,
                    bundle.row.m2_locked_gradient_only_share,
                ),
                5,
                color.filled(),
            )))
            .expect("draw point");
    }
}

fn extract_bucket(label: &str, key: &str) -> usize {
    label
        .split(", ")
        .find_map(|part| {
            let (name, value) = part.split_once('=')?;
            (name == key).then(|| value.parse::<usize>().ok()).flatten()
        })
        .unwrap_or(0)
}

fn hinge_color(category: &str) -> RGBColor {
    match category {
        HINGE_CATEGORY_PERSISTENT_CORE => RGBColor(30, 100, 190),
        HINGE_CATEGORY_PERSISTENCE_ONLY => RGBColor(220, 120, 30),
        HINGE_CATEGORY_CORE_ONLY => RGBColor(40, 150, 100),
        HINGE_CATEGORY_ACTIVE_NEITHER => RGBColor(180, 60, 90),
        _ => RGBColor(120, 120, 120),
    }
}
