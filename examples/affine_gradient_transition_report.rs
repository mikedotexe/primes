//! Focused affine follow-up centered on the `gradient_only` transition.
//!
//! This pass asks a narrower question than the affine hinge atlas:
//! is the observed disappearance of `gradient_only` at `M=2`
//! - a genuine affine lane-collapse phenomenon
//! - or a winner-selection effect where the best lane avoids `gradient_only`
//!   while alternate noncompact lanes still carry it?
//!
//! The report compares two maintained surfaces across `M=1,2,3`:
//! - `best_surface`: the exact `k=(0,0) -> best_k` comparison
//! - `direct_all`: the full direct `k=(0,0) -> each noncompact lane` atlas
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example affine_gradient_transition_report
//! cargo run --release --example affine_gradient_transition_report -- --out-dir /tmp/primes_affine_gradient_transition_alt
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        analyze_best_vs_k00_feature_row, analyze_hinge_feature_row, ordered_unit_pairs,
        parse_k_label, scan_k_config_affine_lane_comparison, BestVsK00FeatureRow, BoundedKConfig,
        KConfigAffineLaneComparison, DEFAULT_BOUNDED_K_GRID, HINGE_CATEGORY_ACTIVE_NEITHER,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::BTreeSet,
    env,
    path::{Path, PathBuf},
};

const MAIN_BASES: &[u32] = &[10, 14, 22, 26];
const APPENDIX_BASES: &[u32] = &[34, 6];
const MIDDLE_LENGTHS: &[usize] = &[1, 2, 3];
const DEFAULT_OUT_DIR: &str = "/tmp/primes_affine_gradient_transition";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "affine_gradient_transition_report";

const NONCOMPACT_LANES: &[BoundedKConfig] = &[(0, 1), (1, 0), (1, 1), (2, 2)];
const SURFACE_DIRECT_ALL: &str = "direct_all";
const SURFACE_BEST: &str = "best_surface";
const SURFACE_BEST_ACTIVE: &str = "best_active_surface";

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

#[derive(Debug, Clone, Copy)]
struct RepresentativeSpec {
    role: &'static str,
    base: u32,
    outer: u32,
    inner: u32,
}

#[derive(Debug, Clone)]
struct PairPhaseBundle {
    scope: String,
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    middle_length: usize,
    m2_hinge_category: String,
    best_feature: BestVsK00FeatureRow,
    best_comparison: KConfigAffineLaneComparison,
    direct_comparisons: Vec<KConfigAffineLaneComparison>,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    main_bases: Vec<u32>,
    appendix_bases: Vec<u32>,
    middle_lengths: Vec<usize>,
    noncompact_lanes: Vec<String>,
    surfaces: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct GradientDirectLaneCsvRow {
    scope: String,
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_label: String,
    m2_hinge_category: String,
    same_digit: bool,
    gap_bucket: String,
    unit_distance: usize,
    to_k: String,
    best_k: String,
    comparison_is_best: bool,
    best_active: bool,
    best_noncompact_winner: bool,
    best_anomaly_mass_pp: f64,
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
    gradient_only_present: bool,
}

#[derive(Debug, Clone, Serialize)]
struct GradientPhaseCsvRow {
    scope: String,
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_label: String,
    m2_hinge_category: String,
    same_digit: bool,
    gap_bucket: String,
    unit_distance: usize,
    best_k: String,
    best_active: bool,
    best_noncompact_winner: bool,
    best_anomaly_mass_pp: f64,
    best_same_shift_share: f64,
    best_same_gradient_share: f64,
    best_same_zero_seed_share: f64,
    best_identity_share: f64,
    best_shift_only_share: f64,
    best_gradient_only_share: f64,
    best_shift_and_gradient_share: f64,
    max_direct_gradient_only_share: f64,
    max_direct_gradient_only_k: String,
    direct_gradient_only_lane_count: usize,
    direct_gradient_only_lane_share: f64,
    mean_direct_gradient_only_share: f64,
    mean_direct_shift_only_share: f64,
    mean_direct_identity_share: f64,
    gradient_transition_class: String,
}

#[derive(Debug, Clone, Serialize)]
struct GradientSummaryRow {
    scope: String,
    subset: String,
    middle_length: usize,
    pair_rows: usize,
    direct_lane_rows: usize,
    best_active_pair_share: f64,
    best_noncompact_winner_share: f64,
    best_gradient_only_pair_share: f64,
    any_direct_gradient_only_pair_share: f64,
    winner_gradient_only_share: f64,
    winner_avoids_gradient_only_share: f64,
    full_lane_collapse_share: f64,
    mean_best_gradient_only_share: f64,
    mean_max_direct_gradient_only_share: f64,
    mean_best_shift_only_share: f64,
    mean_best_identity_share: f64,
    mean_direct_gradient_only_share: f64,
    mean_direct_shift_only_share: f64,
}

#[derive(Debug, Clone, Serialize)]
struct GradientRelationSummaryRow {
    scope: String,
    surface_kind: String,
    middle_length: usize,
    modulus_rows: usize,
    identity_share: f64,
    shift_only_share: f64,
    gradient_only_share: f64,
    shift_and_gradient_share: f64,
}

#[derive(Debug, Clone, Serialize)]
struct GradientModulusSummaryRow {
    scope: String,
    surface_kind: String,
    middle_length: usize,
    modulus: u32,
    total_rows: usize,
    identity_share: f64,
    shift_only_share: f64,
    gradient_only_share: f64,
    shift_and_gradient_share: f64,
}

#[derive(Debug, Clone, Serialize)]
struct RepresentativeGradientRow {
    role: String,
    base: u32,
    middle_length: usize,
    pair_label: String,
    m2_hinge_category: String,
    best_k: String,
    best_active: bool,
    best_gradient_only_share: f64,
    best_shift_only_share: f64,
    best_identity_share: f64,
    max_direct_gradient_only_share: f64,
    max_direct_gradient_only_k: String,
    direct_gradient_only_lane_count: usize,
    gradient_transition_class: String,
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
    main_pair_rows: usize,
    main_direct_lane_rows: usize,
    m1_best_gradient_only_pair_share: f64,
    m2_best_gradient_only_pair_share: f64,
    m3_best_gradient_only_pair_share: f64,
    m1_direct_gradient_only_pair_share: f64,
    m2_direct_gradient_only_pair_share: f64,
    m3_direct_gradient_only_pair_share: f64,
    m2_winner_avoids_gradient_only_share: f64,
    m2_full_lane_collapse_share: f64,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    gradient_direct_lane_rows: Vec<GradientDirectLaneCsvRow>,
    gradient_phase_rows: Vec<GradientPhaseCsvRow>,
    gradient_summary_rows: Vec<GradientSummaryRow>,
    gradient_relation_summary_rows: Vec<GradientRelationSummaryRow>,
    gradient_modulus_summary_rows: Vec<GradientModulusSummaryRow>,
    representative_rows: Vec<RepresentativeGradientRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let mut main_bundles = build_pair_phase_bundles(MAIN_BASES, "main");
    let mut appendix_bundles = build_pair_phase_bundles(APPENDIX_BASES, "appendix");
    main_bundles.sort_by(bundle_sort);
    appendix_bundles.sort_by(bundle_sort);

    let mut all_bundles = main_bundles.clone();
    all_bundles.extend(appendix_bundles.clone());

    let gradient_direct_lane_rows = build_direct_lane_rows(&all_bundles);
    let gradient_phase_rows = build_phase_rows(&all_bundles);
    let gradient_summary_rows =
        build_summary_rows(&gradient_phase_rows, &gradient_direct_lane_rows);
    let gradient_relation_summary_rows = build_relation_summary_rows(&all_bundles);
    let gradient_modulus_summary_rows = build_modulus_summary_rows(&all_bundles);
    let representative_rows = build_representative_rows(&all_bundles);

    let mass_path = options.out_dir.join("gradient_only_mass_by_m.png");
    render_gradient_only_mass_by_m(&gradient_summary_rows, &mass_path);
    let stack_path = options.out_dir.join("gradient_relation_stack.png");
    render_gradient_relation_stack(&gradient_relation_summary_rows, &stack_path);
    let heatmap_path = options.out_dir.join("gradient_modulus_heatmap.png");
    render_gradient_modulus_heatmap(&gradient_modulus_summary_rows, &heatmap_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "mass_by_m".to_string(),
            label: "Gradient-only mass by middle length".to_string(),
            path: mass_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "relation_stack".to_string(),
            label: "Affine relation stack".to_string(),
            path: stack_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "modulus_heatmap".to_string(),
            label: "Gradient-only modulus heatmap".to_string(),
            path: heatmap_path.display().to_string(),
        },
    ];

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        main_bases: MAIN_BASES.to_vec(),
        appendix_bases: APPENDIX_BASES.to_vec(),
        middle_lengths: MIDDLE_LENGTHS.to_vec(),
        noncompact_lanes: NONCOMPACT_LANES.iter().map(|&k| format_k(k)).collect(),
        surfaces: vec![
            SURFACE_DIRECT_ALL.to_string(),
            SURFACE_BEST.to_string(),
            SURFACE_BEST_ACTIVE.to_string(),
        ],
    };
    let report_summary = build_report_summary(&gradient_summary_rows);
    let observations = derive_observations(
        &gradient_summary_rows,
        &gradient_phase_rows,
        &representative_rows,
        &gradient_modulus_summary_rows,
    );
    let report_text = render_report(
        &settings,
        &report_summary,
        &observations,
        &gradient_summary_rows,
    );

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        gradient_direct_lane_rows: gradient_direct_lane_rows.clone(),
        gradient_phase_rows: gradient_phase_rows.clone(),
        gradient_summary_rows: gradient_summary_rows.clone(),
        gradient_relation_summary_rows: gradient_relation_summary_rows.clone(),
        gradient_modulus_summary_rows: gradient_modulus_summary_rows.clone(),
        representative_rows: representative_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary: report_summary.clone(),
        observations: observations.clone(),
    };

    write_csv_rows(
        options.out_dir.join("gradient_direct_lane_rows.csv"),
        &gradient_direct_lane_rows,
    )
    .expect("write direct lane rows");
    write_csv_rows(
        options.out_dir.join("gradient_phase_rows.csv"),
        &gradient_phase_rows,
    )
    .expect("write phase rows");
    write_csv_rows(
        options.out_dir.join("gradient_summary_rows.csv"),
        &gradient_summary_rows,
    )
    .expect("write summary rows");
    write_csv_rows(
        options.out_dir.join("gradient_relation_summary_rows.csv"),
        &gradient_relation_summary_rows,
    )
    .expect("write relation summary rows");
    write_csv_rows(
        options.out_dir.join("gradient_modulus_summary_rows.csv"),
        &gradient_modulus_summary_rows,
    )
    .expect("write modulus summary rows");
    write_csv_rows(
        options.out_dir.join("gradient_representative_rows.csv"),
        &representative_rows,
    )
    .expect("write representative rows");
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
                "affine_gradient_transition_report".to_string(),
            ],
            upstream_inputs: vec!["src/validation/bounded_k.rs".to_string()],
            expected_outputs: vec![
                "gradient_direct_lane_rows.csv".to_string(),
                "gradient_phase_rows.csv".to_string(),
                "gradient_summary_rows.csv".to_string(),
                "gradient_relation_summary_rows.csv".to_string(),
                "gradient_modulus_summary_rows.csv".to_string(),
                "gradient_representative_rows.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
                "artifact_manifest.json".to_string(),
                "gradient_only_mass_by_m.png".to_string(),
                "gradient_relation_stack.png".to_string(),
                "gradient_modulus_heatmap.png".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote affine gradient-transition bundle to {}",
        options.out_dir.display()
    );
    println!("{}", report_summary.main_takeaway);
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

fn build_pair_phase_bundles(bases: &[u32], scope: &str) -> Vec<PairPhaseBundle> {
    let pair_catalog = bases
        .iter()
        .flat_map(|&base| {
            ordered_unit_pairs(base)
                .into_iter()
                .map(move |(outer, inner)| (base, outer, inner))
        })
        .collect::<Vec<_>>();

    pair_catalog
        .into_par_iter()
        .flat_map_iter(|(base, outer, inner)| {
            let m2_hinge_category = analyze_hinge_feature_row(base, outer, inner).hinge_category;
            MIDDLE_LENGTHS.iter().copied().map(move |middle_length| {
                let best_feature =
                    analyze_best_vs_k00_feature_row(base, middle_length, outer, inner);
                let best_comparison = scan_k_config_affine_lane_comparison(
                    base,
                    middle_length,
                    outer,
                    inner,
                    (0, 0),
                    parse_k_label(&best_feature.best_k),
                );
                let direct_comparisons = NONCOMPACT_LANES
                    .iter()
                    .copied()
                    .map(|to_k| {
                        scan_k_config_affine_lane_comparison(
                            base,
                            middle_length,
                            outer,
                            inner,
                            (0, 0),
                            to_k,
                        )
                    })
                    .collect::<Vec<_>>();
                PairPhaseBundle {
                    scope: scope.to_string(),
                    base,
                    outer,
                    inner,
                    pair_label: format!(
                        "({},{})",
                        digit_symbol(base, outer),
                        digit_symbol(base, inner)
                    ),
                    middle_length,
                    m2_hinge_category: m2_hinge_category.clone(),
                    best_feature,
                    best_comparison,
                    direct_comparisons,
                }
            })
        })
        .collect()
}

fn bundle_sort(left: &PairPhaseBundle, right: &PairPhaseBundle) -> std::cmp::Ordering {
    left.base
        .cmp(&right.base)
        .then_with(|| left.middle_length.cmp(&right.middle_length))
        .then_with(|| left.outer.cmp(&right.outer))
        .then_with(|| left.inner.cmp(&right.inner))
}

fn build_direct_lane_rows(bundles: &[PairPhaseBundle]) -> Vec<GradientDirectLaneCsvRow> {
    bundles
        .iter()
        .flat_map(|bundle| {
            bundle
                .direct_comparisons
                .iter()
                .map(move |comparison| GradientDirectLaneCsvRow {
                    scope: bundle.scope.clone(),
                    base: bundle.base,
                    middle_length: bundle.middle_length,
                    outer: bundle.outer,
                    inner: bundle.inner,
                    pair_label: bundle.pair_label.clone(),
                    m2_hinge_category: bundle.m2_hinge_category.clone(),
                    same_digit: bundle.outer == bundle.inner,
                    gap_bucket: bundle.best_feature.gap_bucket.clone(),
                    unit_distance: bundle.best_feature.unit_distance,
                    to_k: comparison.to_k.clone(),
                    best_k: bundle.best_feature.best_k.clone(),
                    comparison_is_best: comparison.to_k == bundle.best_feature.best_k,
                    best_active: bundle.best_feature.active,
                    best_noncompact_winner: bundle.best_feature.best_k != "(0,0)",
                    best_anomaly_mass_pp: bundle.best_feature.anomaly_mass_pp,
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
                    gradient_only_present: comparison.gradient_only_count > 0,
                })
        })
        .collect()
}

fn build_phase_rows(bundles: &[PairPhaseBundle]) -> Vec<GradientPhaseCsvRow> {
    bundles
        .iter()
        .map(|bundle| {
            let max_gradient_comparison = bundle
                .direct_comparisons
                .iter()
                .max_by(|left, right| {
                    left.gradient_only_share
                        .partial_cmp(&right.gradient_only_share)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| left.to_k.cmp(&right.to_k))
                })
                .expect("direct comparison list should be nonempty");
            let direct_gradient_only_lane_count = bundle
                .direct_comparisons
                .iter()
                .filter(|comparison| comparison.gradient_only_count > 0)
                .count();

            GradientPhaseCsvRow {
                scope: bundle.scope.clone(),
                base: bundle.base,
                middle_length: bundle.middle_length,
                outer: bundle.outer,
                inner: bundle.inner,
                pair_label: bundle.pair_label.clone(),
                m2_hinge_category: bundle.m2_hinge_category.clone(),
                same_digit: bundle.outer == bundle.inner,
                gap_bucket: bundle.best_feature.gap_bucket.clone(),
                unit_distance: bundle.best_feature.unit_distance,
                best_k: bundle.best_feature.best_k.clone(),
                best_active: bundle.best_feature.active,
                best_noncompact_winner: bundle.best_feature.best_k != "(0,0)",
                best_anomaly_mass_pp: bundle.best_feature.anomaly_mass_pp,
                best_same_shift_share: bundle.best_comparison.same_shift_share,
                best_same_gradient_share: bundle.best_comparison.same_gradient_share,
                best_same_zero_seed_share: bundle.best_comparison.same_zero_seed_share,
                best_identity_share: bundle.best_comparison.identity_share,
                best_shift_only_share: bundle.best_comparison.shift_only_share,
                best_gradient_only_share: bundle.best_comparison.gradient_only_share,
                best_shift_and_gradient_share: bundle.best_comparison.shift_and_gradient_share,
                max_direct_gradient_only_share: max_gradient_comparison.gradient_only_share,
                max_direct_gradient_only_k: max_gradient_comparison.to_k.clone(),
                direct_gradient_only_lane_count,
                direct_gradient_only_lane_share: ratio(
                    direct_gradient_only_lane_count,
                    bundle.direct_comparisons.len(),
                ),
                mean_direct_gradient_only_share: mean(
                    bundle
                        .direct_comparisons
                        .iter()
                        .map(|comparison| comparison.gradient_only_share),
                ),
                mean_direct_shift_only_share: mean(
                    bundle
                        .direct_comparisons
                        .iter()
                        .map(|comparison| comparison.shift_only_share),
                ),
                mean_direct_identity_share: mean(
                    bundle
                        .direct_comparisons
                        .iter()
                        .map(|comparison| comparison.identity_share),
                ),
                gradient_transition_class: gradient_transition_class(
                    bundle.best_comparison.gradient_only_count > 0,
                    direct_gradient_only_lane_count > 0,
                )
                .to_string(),
            }
        })
        .collect()
}

fn build_summary_rows(
    phase_rows: &[GradientPhaseCsvRow],
    direct_lane_rows: &[GradientDirectLaneCsvRow],
) -> Vec<GradientSummaryRow> {
    let mut rows = Vec::new();
    for scope in ["main", "appendix"] {
        for subset in ["all_pairs", "best_active_pairs"] {
            for &middle_length in MIDDLE_LENGTHS {
                let filtered_phase_rows = phase_rows
                    .iter()
                    .filter(|row| row.scope == scope && row.middle_length == middle_length)
                    .filter(|row| subset != "best_active_pairs" || row.best_active)
                    .collect::<Vec<_>>();
                if filtered_phase_rows.is_empty() {
                    continue;
                }
                let filtered_direct_rows = direct_lane_rows
                    .iter()
                    .filter(|row| row.scope == scope && row.middle_length == middle_length)
                    .filter(|row| {
                        subset != "best_active_pairs"
                            || filtered_phase_rows.iter().any(|phase_row| {
                                phase_row.base == row.base
                                    && phase_row.outer == row.outer
                                    && phase_row.inner == row.inner
                            })
                    })
                    .collect::<Vec<_>>();

                rows.push(GradientSummaryRow {
                    scope: scope.to_string(),
                    subset: subset.to_string(),
                    middle_length,
                    pair_rows: filtered_phase_rows.len(),
                    direct_lane_rows: filtered_direct_rows.len(),
                    best_active_pair_share: ratio(
                        filtered_phase_rows
                            .iter()
                            .filter(|row| row.best_active)
                            .count(),
                        filtered_phase_rows.len(),
                    ),
                    best_noncompact_winner_share: ratio(
                        filtered_phase_rows
                            .iter()
                            .filter(|row| row.best_noncompact_winner)
                            .count(),
                        filtered_phase_rows.len(),
                    ),
                    best_gradient_only_pair_share: ratio(
                        filtered_phase_rows
                            .iter()
                            .filter(|row| row.best_gradient_only_share > 0.0)
                            .count(),
                        filtered_phase_rows.len(),
                    ),
                    any_direct_gradient_only_pair_share: ratio(
                        filtered_phase_rows
                            .iter()
                            .filter(|row| row.direct_gradient_only_lane_count > 0)
                            .count(),
                        filtered_phase_rows.len(),
                    ),
                    winner_gradient_only_share: ratio(
                        filtered_phase_rows
                            .iter()
                            .filter(|row| row.gradient_transition_class == "winner_gradient_only")
                            .count(),
                        filtered_phase_rows.len(),
                    ),
                    winner_avoids_gradient_only_share: ratio(
                        filtered_phase_rows
                            .iter()
                            .filter(|row| {
                                row.gradient_transition_class == "winner_avoids_gradient_only"
                            })
                            .count(),
                        filtered_phase_rows.len(),
                    ),
                    full_lane_collapse_share: ratio(
                        filtered_phase_rows
                            .iter()
                            .filter(|row| row.gradient_transition_class == "full_lane_collapse")
                            .count(),
                        filtered_phase_rows.len(),
                    ),
                    mean_best_gradient_only_share: mean(
                        filtered_phase_rows
                            .iter()
                            .map(|row| row.best_gradient_only_share),
                    ),
                    mean_max_direct_gradient_only_share: mean(
                        filtered_phase_rows
                            .iter()
                            .map(|row| row.max_direct_gradient_only_share),
                    ),
                    mean_best_shift_only_share: mean(
                        filtered_phase_rows
                            .iter()
                            .map(|row| row.best_shift_only_share),
                    ),
                    mean_best_identity_share: mean(
                        filtered_phase_rows
                            .iter()
                            .map(|row| row.best_identity_share),
                    ),
                    mean_direct_gradient_only_share: mean(
                        filtered_direct_rows
                            .iter()
                            .map(|row| row.gradient_only_share),
                    ),
                    mean_direct_shift_only_share: mean(
                        filtered_direct_rows.iter().map(|row| row.shift_only_share),
                    ),
                });
            }
        }
    }
    rows
}

fn build_relation_summary_rows(bundles: &[PairPhaseBundle]) -> Vec<GradientRelationSummaryRow> {
    let mut rows = Vec::new();
    for scope in ["main", "appendix"] {
        for surface_kind in [SURFACE_DIRECT_ALL, SURFACE_BEST, SURFACE_BEST_ACTIVE] {
            for &middle_length in MIDDLE_LENGTHS {
                let modulus_rows =
                    collect_relation_labels(bundles, scope, middle_length, surface_kind);
                if modulus_rows.is_empty() {
                    continue;
                }
                let total_rows = modulus_rows.len();
                rows.push(GradientRelationSummaryRow {
                    scope: scope.to_string(),
                    surface_kind: surface_kind.to_string(),
                    middle_length,
                    modulus_rows: total_rows,
                    identity_share: ratio(
                        modulus_rows
                            .iter()
                            .filter(|label| **label == "identity")
                            .count(),
                        total_rows,
                    ),
                    shift_only_share: ratio(
                        modulus_rows
                            .iter()
                            .filter(|label| **label == "shift_only")
                            .count(),
                        total_rows,
                    ),
                    gradient_only_share: ratio(
                        modulus_rows
                            .iter()
                            .filter(|label| **label == "gradient_only")
                            .count(),
                        total_rows,
                    ),
                    shift_and_gradient_share: ratio(
                        modulus_rows
                            .iter()
                            .filter(|label| **label == "shift_and_gradient")
                            .count(),
                        total_rows,
                    ),
                });
            }
        }
    }
    rows
}

fn build_modulus_summary_rows(bundles: &[PairPhaseBundle]) -> Vec<GradientModulusSummaryRow> {
    let mut rows = Vec::new();
    for scope in ["main", "appendix"] {
        for surface_kind in [SURFACE_DIRECT_ALL, SURFACE_BEST] {
            for &middle_length in MIDDLE_LENGTHS {
                let modulus_set = bundles
                    .iter()
                    .filter(|bundle| bundle.scope == scope && bundle.middle_length == middle_length)
                    .flat_map(|bundle| {
                        bundle
                            .best_comparison
                            .modulus_rows
                            .iter()
                            .map(|row| row.modulus)
                    })
                    .collect::<BTreeSet<_>>();
                for modulus in modulus_set {
                    let labels = collect_modulus_relation_labels(
                        bundles,
                        scope,
                        middle_length,
                        surface_kind,
                        modulus,
                    );
                    if labels.is_empty() {
                        continue;
                    }
                    let total_rows = labels.len();
                    rows.push(GradientModulusSummaryRow {
                        scope: scope.to_string(),
                        surface_kind: surface_kind.to_string(),
                        middle_length,
                        modulus,
                        total_rows,
                        identity_share: ratio(
                            labels.iter().filter(|label| **label == "identity").count(),
                            total_rows,
                        ),
                        shift_only_share: ratio(
                            labels
                                .iter()
                                .filter(|label| **label == "shift_only")
                                .count(),
                            total_rows,
                        ),
                        gradient_only_share: ratio(
                            labels
                                .iter()
                                .filter(|label| **label == "gradient_only")
                                .count(),
                            total_rows,
                        ),
                        shift_and_gradient_share: ratio(
                            labels
                                .iter()
                                .filter(|label| **label == "shift_and_gradient")
                                .count(),
                            total_rows,
                        ),
                    });
                }
            }
        }
    }
    rows
}

fn collect_relation_labels<'a>(
    bundles: &'a [PairPhaseBundle],
    scope: &str,
    middle_length: usize,
    surface_kind: &str,
) -> Vec<&'a str> {
    let filtered_bundles = bundles
        .iter()
        .filter(|bundle| bundle.scope == scope && bundle.middle_length == middle_length)
        .filter(|bundle| surface_kind != SURFACE_BEST_ACTIVE || bundle.best_feature.active)
        .collect::<Vec<_>>();
    match surface_kind {
        SURFACE_DIRECT_ALL => filtered_bundles
            .into_iter()
            .flat_map(|bundle| {
                bundle
                    .direct_comparisons
                    .iter()
                    .flat_map(|comparison| comparison.modulus_rows.iter())
            })
            .map(|row| row.local_relation_label.as_str())
            .collect(),
        SURFACE_BEST | SURFACE_BEST_ACTIVE => filtered_bundles
            .into_iter()
            .flat_map(|bundle| bundle.best_comparison.modulus_rows.iter())
            .map(|row| row.local_relation_label.as_str())
            .collect(),
        _ => Vec::new(),
    }
}

fn collect_modulus_relation_labels<'a>(
    bundles: &'a [PairPhaseBundle],
    scope: &str,
    middle_length: usize,
    surface_kind: &str,
    modulus: u32,
) -> Vec<&'a str> {
    let filtered_bundles = bundles
        .iter()
        .filter(|bundle| bundle.scope == scope && bundle.middle_length == middle_length)
        .collect::<Vec<_>>();
    match surface_kind {
        SURFACE_DIRECT_ALL => filtered_bundles
            .into_iter()
            .flat_map(|bundle| {
                bundle
                    .direct_comparisons
                    .iter()
                    .flat_map(|comparison| comparison.modulus_rows.iter())
            })
            .filter(|row| row.modulus == modulus)
            .map(|row| row.local_relation_label.as_str())
            .collect(),
        SURFACE_BEST => filtered_bundles
            .into_iter()
            .flat_map(|bundle| bundle.best_comparison.modulus_rows.iter())
            .filter(|row| row.modulus == modulus)
            .map(|row| row.local_relation_label.as_str())
            .collect(),
        _ => Vec::new(),
    }
}

fn build_representative_rows(bundles: &[PairPhaseBundle]) -> Vec<RepresentativeGradientRow> {
    REPRESENTATIVES
        .iter()
        .flat_map(|spec| {
            MIDDLE_LENGTHS.iter().copied().map(move |middle_length| {
                let bundle = bundles
                    .iter()
                    .find(|bundle| {
                        bundle.base == spec.base
                            && bundle.outer == spec.outer
                            && bundle.inner == spec.inner
                            && bundle.middle_length == middle_length
                    })
                    .expect("representative bundle should exist");
                let max_gradient = bundle
                    .direct_comparisons
                    .iter()
                    .max_by(|left, right| {
                        left.gradient_only_share
                            .partial_cmp(&right.gradient_only_share)
                            .unwrap_or(std::cmp::Ordering::Equal)
                            .then_with(|| left.to_k.cmp(&right.to_k))
                    })
                    .expect("representative direct comparison should exist");
                let direct_gradient_only_lane_count = bundle
                    .direct_comparisons
                    .iter()
                    .filter(|comparison| comparison.gradient_only_count > 0)
                    .count();
                let transition_class = gradient_transition_class(
                    bundle.best_comparison.gradient_only_count > 0,
                    direct_gradient_only_lane_count > 0,
                );
                RepresentativeGradientRow {
                    role: spec.role.to_string(),
                    base: spec.base,
                    middle_length,
                    pair_label: bundle.pair_label.clone(),
                    m2_hinge_category: bundle.m2_hinge_category.clone(),
                    best_k: bundle.best_feature.best_k.clone(),
                    best_active: bundle.best_feature.active,
                    best_gradient_only_share: bundle.best_comparison.gradient_only_share,
                    best_shift_only_share: bundle.best_comparison.shift_only_share,
                    best_identity_share: bundle.best_comparison.identity_share,
                    max_direct_gradient_only_share: max_gradient.gradient_only_share,
                    max_direct_gradient_only_k: max_gradient.to_k.clone(),
                    direct_gradient_only_lane_count,
                    gradient_transition_class: transition_class.to_string(),
                    note: representative_note(
                        bundle,
                        max_gradient,
                        direct_gradient_only_lane_count,
                    ),
                }
            })
        })
        .collect()
}

fn representative_note(
    bundle: &PairPhaseBundle,
    max_gradient: &KConfigAffineLaneComparison,
    direct_gradient_only_lane_count: usize,
) -> String {
    match gradient_transition_class(
        bundle.best_comparison.gradient_only_count > 0,
        direct_gradient_only_lane_count > 0,
    ) {
        "winner_gradient_only" => format!(
            "The winning affine comparison still carries gradient_only at {}.",
            bundle.best_feature.best_k
        ),
        "winner_avoids_gradient_only" => format!(
            "The winning affine comparison avoids gradient_only, but {} still reaches {:.3}.",
            max_gradient.to_k, max_gradient.gradient_only_share
        ),
        "full_lane_collapse" => {
            "No noncompact lane shows gradient_only on the coprime-modulus surface.".to_string()
        }
        _ => "unclassified".to_string(),
    }
}

fn gradient_transition_class(
    best_has_gradient_only: bool,
    any_direct_gradient_only: bool,
) -> &'static str {
    match (best_has_gradient_only, any_direct_gradient_only) {
        (true, _) => "winner_gradient_only",
        (false, true) => "winner_avoids_gradient_only",
        (false, false) => "full_lane_collapse",
    }
}

fn build_report_summary(summary_rows: &[GradientSummaryRow]) -> ReportSummary {
    let main_all = select_summary_row(summary_rows, "main", "all_pairs", 1);
    let main_all_m2 = select_summary_row(summary_rows, "main", "all_pairs", 2);
    let main_all_m3 = select_summary_row(summary_rows, "main", "all_pairs", 3);
    let takeaway = if main_all_m2.best_gradient_only_pair_share == 0.0
        && main_all_m2.any_direct_gradient_only_pair_share > 0.0
    {
        format!(
            "On the maintained main surface, `gradient_only` disappears on the winning M=2 comparison but not from the full lane space: best-surface share is 0 while direct-lane share is {:.2}%.",
            main_all_m2.any_direct_gradient_only_pair_share * 100.0
        )
    } else if main_all_m2.any_direct_gradient_only_pair_share == 0.0 {
        "On the maintained main surface, M=2 shows a true lane-level collapse of gradient_only."
            .to_string()
    } else {
        format!(
            "On the maintained main surface, M=2 still allows gradient_only on both the winning and direct lane surfaces ({:.2}% / {:.2}%).",
            main_all_m2.best_gradient_only_pair_share * 100.0,
            main_all_m2.any_direct_gradient_only_pair_share * 100.0
        )
    };

    ReportSummary {
        main_pair_rows: [main_all, main_all_m2, main_all_m3]
            .iter()
            .map(|row| row.pair_rows)
            .sum(),
        main_direct_lane_rows: [main_all, main_all_m2, main_all_m3]
            .iter()
            .map(|row| row.direct_lane_rows)
            .sum(),
        m1_best_gradient_only_pair_share: main_all.best_gradient_only_pair_share,
        m2_best_gradient_only_pair_share: main_all_m2.best_gradient_only_pair_share,
        m3_best_gradient_only_pair_share: main_all_m3.best_gradient_only_pair_share,
        m1_direct_gradient_only_pair_share: main_all.any_direct_gradient_only_pair_share,
        m2_direct_gradient_only_pair_share: main_all_m2.any_direct_gradient_only_pair_share,
        m3_direct_gradient_only_pair_share: main_all_m3.any_direct_gradient_only_pair_share,
        m2_winner_avoids_gradient_only_share: main_all_m2.winner_avoids_gradient_only_share,
        m2_full_lane_collapse_share: main_all_m2.full_lane_collapse_share,
        main_takeaway: takeaway,
    }
}

fn derive_observations(
    summary_rows: &[GradientSummaryRow],
    phase_rows: &[GradientPhaseCsvRow],
    representative_rows: &[RepresentativeGradientRow],
    modulus_summary_rows: &[GradientModulusSummaryRow],
) -> Vec<String> {
    let mut observations = Vec::new();
    for &middle_length in MIDDLE_LENGTHS {
        let row = select_summary_row(summary_rows, "main", "all_pairs", middle_length);
        observations.push(format!(
            "M={} main pairs: best-surface gradient_only share {:.2}% vs direct-lane share {:.2}%; winner-avoids share {:.2}%.",
            middle_length,
            row.best_gradient_only_pair_share * 100.0,
            row.any_direct_gradient_only_pair_share * 100.0,
            row.winner_avoids_gradient_only_share * 100.0,
        ));
    }
    if let Some(base14_db_m2) = representative_rows
        .iter()
        .find(|row| row.base == 14 && row.pair_label == "(D,B)" && row.middle_length == 2)
    {
        observations.push(format!(
            "Base 14 (D,B) at M=2 lands in `{}` with best shift_only {:.3} and max alternate gradient_only {:.3}.",
            base14_db_m2.gradient_transition_class,
            base14_db_m2.best_shift_only_share,
            base14_db_m2.max_direct_gradient_only_share,
        ));
    }
    if let Some(base10_33_m2) = representative_rows
        .iter()
        .find(|row| row.base == 10 && row.pair_label == "(3,3)" && row.middle_length == 2)
    {
        observations.push(format!(
            "Base 10 (3,3) at M=2 has best gradient_only {:.3}, but the strongest alternate lane reaches {:.3}.",
            base10_33_m2.best_gradient_only_share,
            base10_33_m2.max_direct_gradient_only_share,
        ));
    }
    let m2_winner_avoids = phase_rows
        .iter()
        .filter(|row| row.scope == "main" && row.middle_length == 2)
        .filter(|row| row.gradient_transition_class == "winner_avoids_gradient_only")
        .collect::<Vec<_>>();
    if !m2_winner_avoids.is_empty()
        && m2_winner_avoids.iter().all(|row| row.base == 22)
        && m2_winner_avoids
            .iter()
            .all(|row| row.max_direct_gradient_only_k == "k=(2,2)")
        && m2_winner_avoids
            .iter()
            .all(|row| row.m2_hinge_category == HINGE_CATEGORY_ACTIVE_NEITHER)
    {
        observations.push(
            "The residual M=2 direct-lane gradient_only pocket is entirely base 22, entirely active_neither, and always peaks on the alternate lane k=(2,2)."
                .to_string(),
        );
    }
    let m3_main = select_summary_row(summary_rows, "main", "all_pairs", 3);
    if m3_main.best_gradient_only_pair_share == 0.0
        && m3_main.any_direct_gradient_only_pair_share > 0.8
    {
        observations.push(
            "By M=3 the winning surface is fully free of gradient_only, but the direct lane space rebounds strongly, so the M=2 behavior is a sparse suppression window rather than a monotone extinction."
                .to_string(),
        );
    }
    let m2_nonzero_moduli = modulus_summary_rows
        .iter()
        .filter(|row| {
            row.scope == "main"
                && row.surface_kind == SURFACE_DIRECT_ALL
                && row.middle_length == 2
                && row.gradient_only_share > 0.0
        })
        .map(|row| row.modulus)
        .collect::<Vec<_>>();
    if m2_nonzero_moduli == vec![5] {
        observations.push(
            "On the main direct M=2 surface, gradient_only is modulus-localized: among the maintained coprime prefilter moduli, only modulus 5 contributes any nonzero gradient_only mass."
                .to_string(),
        );
    }
    observations
}

fn render_report(
    settings: &ReportSettings,
    report_summary: &ReportSummary,
    observations: &[String],
    summary_rows: &[GradientSummaryRow],
) -> String {
    let mut lines = Vec::new();
    lines.push("# Affine Gradient-Only Transition".to_string());
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
        "- Middle lengths: {}",
        settings
            .middle_lengths
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    ));
    lines.push(
        "- Surfaces: best `k=(0,0)->best_k` and direct `k=(0,0)->each noncompact lane`".to_string(),
    );
    lines.push(String::new());
    lines.push("## Main Transition".to_string());
    for &middle_length in MIDDLE_LENGTHS {
        let row = select_summary_row(summary_rows, "main", "all_pairs", middle_length);
        lines.push(format!(
            "- M={}: best gradient_only {:.2}%, direct-any {:.2}%, winner-avoids {:.2}%, full-lane-collapse {:.2}%",
            middle_length,
            row.best_gradient_only_pair_share * 100.0,
            row.any_direct_gradient_only_pair_share * 100.0,
            row.winner_avoids_gradient_only_share * 100.0,
            row.full_lane_collapse_share * 100.0,
        ));
    }
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

fn render_gradient_only_mass_by_m(summary_rows: &[GradientSummaryRow], path: &Path) {
    let main_rows = MIDDLE_LENGTHS
        .iter()
        .map(|&middle_length| select_summary_row(summary_rows, "main", "all_pairs", middle_length))
        .collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (980, 720)).into_drawing_area();
    root.fill(&WHITE).expect("fill mass plot");
    let mut chart = ChartBuilder::on(&root)
        .margin(24)
        .caption(
            "Gradient-Only Pair Share by Middle Length",
            ("sans-serif", 28).into_font(),
        )
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..(MIDDLE_LENGTHS.len() as f64), 0f64..1.0)
        .expect("build mass chart");
    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("middle length")
        .y_desc("pair share")
        .x_labels(MIDDLE_LENGTHS.len())
        .x_label_formatter(&|x| {
            let index = x.floor() as usize;
            if index < MIDDLE_LENGTHS.len() {
                format!("M={}", MIDDLE_LENGTHS[index])
            } else {
                String::new()
            }
        })
        .draw()
        .expect("draw mass chart mesh");

    for (index, row) in main_rows.iter().enumerate() {
        let base_x = index as f64;
        let best_left = base_x + 0.08;
        let best_right = base_x + 0.38;
        let direct_left = base_x + 0.52;
        let direct_right = base_x + 0.82;
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [
                    (best_left, 0.0),
                    (best_right, row.best_gradient_only_pair_share),
                ],
                ShapeStyle::from(&RGBColor(31, 119, 180)).filled(),
            )))
            .expect("draw best bar");
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [
                    (direct_left, 0.0),
                    (direct_right, row.any_direct_gradient_only_pair_share),
                ],
                ShapeStyle::from(&RGBColor(214, 39, 40)).filled(),
            )))
            .expect("draw direct bar");
    }
    root.present().expect("present mass plot");
}

fn render_gradient_relation_stack(rows: &[GradientRelationSummaryRow], path: &Path) {
    let main_rows = rows
        .iter()
        .filter(|row| row.scope == "main")
        .collect::<Vec<_>>();
    let bar_count = main_rows.len();
    let root = BitMapBackend::new(path, (1200, 760)).into_drawing_area();
    root.fill(&WHITE).expect("fill relation stack");
    let mut chart = ChartBuilder::on(&root)
        .margin(24)
        .caption(
            "Affine Relation Mix by Surface and Middle Length",
            ("sans-serif", 28).into_font(),
        )
        .x_label_area_size(70)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..bar_count as f64, 0f64..1.0)
        .expect("build relation stack");
    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(bar_count)
        .x_label_formatter(&|x| {
            let index = x.floor() as usize;
            if let Some(row) = main_rows.get(index) {
                format!(
                    "{} M={}",
                    surface_short_label(&row.surface_kind),
                    row.middle_length
                )
            } else {
                String::new()
            }
        })
        .x_desc("surface")
        .y_desc("local relation share")
        .draw()
        .expect("draw relation stack mesh");

    for (index, row) in main_rows.iter().enumerate() {
        let mut y0 = 0.0;
        for (share, color) in [
            (row.identity_share, relation_color("identity")),
            (row.shift_only_share, relation_color("shift_only")),
            (row.gradient_only_share, relation_color("gradient_only")),
            (
                row.shift_and_gradient_share,
                relation_color("shift_and_gradient"),
            ),
        ] {
            let y1 = y0 + share;
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(index as f64 + 0.1, y0), (index as f64 + 0.9, y1)],
                    ShapeStyle::from(&color).filled(),
                )))
                .expect("draw relation stack bar");
            y0 = y1;
        }
    }
    root.present().expect("present relation stack");
}

fn render_gradient_modulus_heatmap(rows: &[GradientModulusSummaryRow], path: &Path) {
    let main_rows = rows
        .iter()
        .filter(|row| row.scope == "main")
        .filter(|row| row.surface_kind == SURFACE_DIRECT_ALL || row.surface_kind == SURFACE_BEST)
        .collect::<Vec<_>>();
    let moduli = main_rows
        .iter()
        .map(|row| row.modulus)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let surface_rows = ["best_surface", "direct_all"]
        .into_iter()
        .flat_map(|surface_kind| {
            MIDDLE_LENGTHS
                .iter()
                .copied()
                .map(move |middle_length| (surface_kind, middle_length))
        })
        .collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (1240, 760)).into_drawing_area();
    root.fill(&WHITE).expect("fill heatmap");
    let cell_width = 92i32;
    let cell_height = 74i32;
    let origin_x = 210i32;
    let origin_y = 100i32;
    root.draw(&Text::new(
        "Gradient-Only Share by Modulus",
        (40, 34),
        ("sans-serif", 28).into_font(),
    ))
    .expect("draw heatmap title");

    for (column, modulus) in moduli.iter().enumerate() {
        let x = origin_x + column as i32 * cell_width + cell_width / 2;
        root.draw(&Text::new(
            modulus.to_string(),
            (x, origin_y - 20),
            ("sans-serif", 18).into_font(),
        ))
        .expect("draw modulus label");
    }
    for (row_index, (surface_kind, middle_length)) in surface_rows.iter().enumerate() {
        let y = origin_y + row_index as i32 * cell_height + cell_height / 2;
        root.draw(&Text::new(
            format!("{} M={}", surface_short_label(surface_kind), middle_length),
            (30, y),
            ("sans-serif", 18).into_font(),
        ))
        .expect("draw row label");
        for (column, modulus) in moduli.iter().enumerate() {
            let x0 = origin_x + column as i32 * cell_width;
            let y0 = origin_y + row_index as i32 * cell_height;
            let cell = main_rows
                .iter()
                .find(|row| {
                    row.surface_kind == *surface_kind
                        && row.middle_length == *middle_length
                        && row.modulus == *modulus
                })
                .map(|row| row.gradient_only_share)
                .unwrap_or(0.0);
            let color = heatmap_color(cell);
            root.draw(&Rectangle::new(
                [(x0, y0), (x0 + cell_width - 6, y0 + cell_height - 6)],
                ShapeStyle::from(&color).filled(),
            ))
            .expect("draw heatmap cell");
            root.draw(&Text::new(
                format!("{:.2}", cell),
                (x0 + 18, y0 + 42),
                ("sans-serif", 16).into_font().color(&BLACK),
            ))
            .expect("draw heatmap cell text");
        }
    }
    root.present().expect("present heatmap");
}

fn select_summary_row<'a>(
    rows: &'a [GradientSummaryRow],
    scope: &str,
    subset: &str,
    middle_length: usize,
) -> &'a GradientSummaryRow {
    rows.iter()
        .find(|row| {
            row.scope == scope && row.subset == subset && row.middle_length == middle_length
        })
        .expect("summary row should exist")
}

fn relation_color(label: &str) -> RGBColor {
    match label {
        "identity" => RGBColor(76, 175, 80),
        "shift_only" => RGBColor(255, 193, 7),
        "gradient_only" => RGBColor(244, 67, 54),
        "shift_and_gradient" => RGBColor(33, 150, 243),
        _ => RGBColor(160, 160, 160),
    }
}

fn heatmap_color(value: f64) -> RGBColor {
    let clamped = value.clamp(0.0, 1.0);
    let red = 255u8;
    let green = (255.0 - clamped * 170.0).round() as u8;
    let blue = (255.0 - clamped * 220.0).round() as u8;
    RGBColor(red, green, blue)
}

fn surface_short_label(surface_kind: &str) -> &'static str {
    match surface_kind {
        SURFACE_DIRECT_ALL => "direct",
        SURFACE_BEST => "best",
        SURFACE_BEST_ACTIVE => "best-active",
        _ => "surface",
    }
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn mean(values: impl IntoIterator<Item = f64>) -> f64 {
    let mut total = 0.0;
    let mut count = 0usize;
    for value in values {
        total += value;
        count += 1;
    }
    if count == 0 {
        0.0
    } else {
        total / count as f64
    }
}

fn format_k(k: BoundedKConfig) -> String {
    let _ = DEFAULT_BOUNDED_K_GRID;
    format!("({}, {})", k.0, k.1).replace(", ", ",")
}

fn digit_symbol(base: u32, digit: u32) -> String {
    let symbol = if digit < 10 {
        (b'0' + digit as u8) as char
    } else {
        (b'A' + (digit as u8 - 10)) as char
    };
    if base <= 10 {
        digit.to_string()
    } else {
        symbol.to_string()
    }
}
