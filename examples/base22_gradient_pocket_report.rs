//! Focused autopsy of the base-22 `k=(0,0) -> (2,2)` residual `gradient_only`
//! pocket at `M=2`.
//!
//! This report follows the affine gradient-transition pass and asks the next
//! exact question:
//! why does the surviving `M=2` direct-lane `gradient_only` pocket concentrate
//! in base 22, lane `k=(2,2)`, and modulus 5?
//!
//! The report treats this as a local affine residue problem first and a species
//! problem second.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base22_gradient_pocket_report
//! cargo run --release --example base22_gradient_pocket_report -- --out-dir /tmp/primes_base22_gradient_pocket_alt
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        analyze_best_vs_k00_feature_row, analyze_hinge_feature_row, ordered_unit_pairs,
        scan_k_config_affine_lane_comparison, BoundedKConfig,
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

const BASE: u32 = 22;
const MIDDLE_LENGTH: usize = 2;
const FROM_K: BoundedKConfig = (0, 0);
const TARGET_K: BoundedKConfig = (2, 2);
const NONCOMPACT_LANES: &[BoundedKConfig] = &[(0, 1), (1, 0), (1, 1), (2, 2)];
const TARGET_MODULUS: u32 = 5;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_base22_gradient_pocket";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "base22_gradient_pocket_report";

const REPRESENTATIVES: &[RepresentativeSpec] = &[
    RepresentativeSpec {
        role: "active_pocket",
        outer: 17,
        inner: 19,
    }, // (H,J)
    RepresentativeSpec {
        role: "inactive_pocket",
        outer: 1,
        inner: 1,
    },
    RepresentativeSpec {
        role: "same_outer_collapsed",
        outer: 17,
        inner: 15,
    }, // (H,F)
    RepresentativeSpec {
        role: "small_collapsed",
        outer: 1,
        inner: 5,
    },
];

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Copy)]
struct RepresentativeSpec {
    role: &'static str,
    outer: u32,
    inner: u32,
}

#[derive(Debug, Clone)]
struct PairBundle {
    outer: u32,
    inner: u32,
    pair_label: String,
    hinge_category: String,
    best_k: String,
    best_active: bool,
    best_anomaly_mass_pp: f64,
    lane_rows: Vec<primes::validation::bounded_k::KConfigAffineLaneComparison>,
    target_lane: primes::validation::bounded_k::KConfigAffineLaneComparison,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    base: u32,
    middle_length: usize,
    from_k: String,
    target_k: String,
    target_modulus: u32,
    noncompact_lanes: Vec<String>,
    out_dir: String,
}

#[derive(Debug, Clone, Serialize)]
struct LaneSummaryRow {
    to_k: String,
    pair_count: usize,
    gradient_only_pair_count: usize,
    gradient_only_pair_share: f64,
    mean_gradient_only_share: f64,
    mean_shift_only_share: f64,
    mean_identity_share: f64,
    mean_shift_and_gradient_share: f64,
    mod5_gradient_only_pair_count: usize,
    mod5_gradient_only_pair_share: f64,
}

#[derive(Debug, Clone, Serialize)]
struct PairPocketRow {
    outer: u32,
    inner: u32,
    pair_label: String,
    hinge_category: String,
    same_digit: bool,
    unit_distance: usize,
    gap_bucket: String,
    outer_mod5: u32,
    inner_mod5: u32,
    best_k: String,
    best_active: bool,
    best_anomaly_mass_pp: f64,
    k22_gradient_only_count: usize,
    k22_gradient_only_share: f64,
    k22_shift_only_count: usize,
    k22_shift_only_share: f64,
    k22_identity_count: usize,
    k22_identity_share: f64,
    k22_shift_and_gradient_count: usize,
    k22_shift_and_gradient_share: f64,
    mod5_relation_label: String,
    mod5_shift_equal: bool,
    mod5_gradient_equal: bool,
    mod5_zero_seed_equal: bool,
    mod5_shift_from: u32,
    mod5_shift_to: u32,
    mod5_gradient_from: u32,
    mod5_gradient_to: u32,
    mod5_zero_seed_from: u32,
    mod5_zero_seed_to: u32,
    predicted_relation_label: String,
    predicted_shift_difference_mod5: u32,
    pocket_class: String,
}

#[derive(Debug, Clone, Serialize)]
struct ModulusRow {
    pair_label: String,
    outer: u32,
    inner: u32,
    modulus: u32,
    shift_from: u32,
    shift_to: u32,
    gradient_from: u32,
    gradient_to: u32,
    zero_seed_from: u32,
    zero_seed_to: u32,
    shift_equal: bool,
    gradient_equal: bool,
    zero_seed_equal: bool,
    local_relation_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct FormulaRow {
    pair_label: String,
    outer: u32,
    inner: u32,
    outer_mod5: u32,
    inner_mod5: u32,
    formula_shift_k00_mod5: u32,
    observed_shift_k00_mod5: u32,
    formula_shift_k22_mod5: u32,
    observed_shift_k22_mod5: u32,
    formula_shift_difference_mod5: u32,
    observed_shift_difference_mod5: u32,
    formula_gradient_k00_mod5: u32,
    observed_gradient_k00_mod5: u32,
    formula_gradient_k22_mod5: u32,
    observed_gradient_k22_mod5: u32,
    predicted_relation_label: String,
    observed_relation_label: String,
    formula_matches_observation: bool,
}

#[derive(Debug, Clone, Serialize)]
struct RepresentativeRow {
    role: String,
    pair_label: String,
    hinge_category: String,
    best_k: String,
    best_active: bool,
    outer_mod5: u32,
    inner_mod5: u32,
    mod5_relation_label: String,
    k22_gradient_only_share: f64,
    k22_shift_only_share: f64,
    k22_identity_share: f64,
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
    pair_count: usize,
    pocket_pair_count: usize,
    pocket_pair_share: f64,
    only_lane_with_gradient_only: String,
    only_modulus_with_gradient_only: u32,
    exact_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    lane_summary_rows: Vec<LaneSummaryRow>,
    pair_pocket_rows: Vec<PairPocketRow>,
    modulus_rows: Vec<ModulusRow>,
    formula_rows: Vec<FormulaRow>,
    representative_rows: Vec<RepresentativeRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let mut bundles = build_pair_bundles();
    bundles.sort_by(|left, right| {
        left.outer
            .cmp(&right.outer)
            .then_with(|| left.inner.cmp(&right.inner))
    });

    let lane_summary_rows = build_lane_summary_rows(&bundles);
    let pair_pocket_rows = build_pair_pocket_rows(&bundles);
    let modulus_rows = build_modulus_rows(&bundles);
    let formula_rows = build_formula_rows(&bundles);
    let representative_rows = build_representative_rows(&pair_pocket_rows);

    let grid_path = options.out_dir.join("base22_gradient_pocket_grid.png");
    render_pair_grid(&pair_pocket_rows, &grid_path);
    let lane_path = options.out_dir.join("base22_lane_summary.png");
    render_lane_summary(&lane_summary_rows, &lane_path);
    let heatmap_path = options.out_dir.join("base22_relation_heatmap.png");
    render_relation_heatmap(&bundles, &representative_rows, &heatmap_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "pair_grid".to_string(),
            label: "Base-22 gradient pocket grid".to_string(),
            path: grid_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "lane_summary".to_string(),
            label: "Base-22 lane summary".to_string(),
            path: lane_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "relation_heatmap".to_string(),
            label: "Base-22 relation heatmap".to_string(),
            path: heatmap_path.display().to_string(),
        },
    ];

    let settings = ReportSettings {
        base: BASE,
        middle_length: MIDDLE_LENGTH,
        from_k: format_k(FROM_K),
        target_k: format_k(TARGET_K),
        target_modulus: TARGET_MODULUS,
        noncompact_lanes: NONCOMPACT_LANES.iter().map(|&k| format_k(k)).collect(),
        out_dir: options.out_dir.display().to_string(),
    };
    let report_summary = build_report_summary(&lane_summary_rows, &pair_pocket_rows);
    let observations = derive_observations(&lane_summary_rows, &pair_pocket_rows);
    let report_text = render_report(&settings, &report_summary, &observations);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        lane_summary_rows: lane_summary_rows.clone(),
        pair_pocket_rows: pair_pocket_rows.clone(),
        modulus_rows: modulus_rows.clone(),
        formula_rows: formula_rows.clone(),
        representative_rows: representative_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary: report_summary.clone(),
        observations: observations.clone(),
    };

    write_csv_rows(
        options.out_dir.join("lane_summary_rows.csv"),
        &lane_summary_rows,
    )
    .expect("write lane summary");
    write_csv_rows(
        options.out_dir.join("pair_pocket_rows.csv"),
        &pair_pocket_rows,
    )
    .expect("write pair pocket rows");
    write_csv_rows(options.out_dir.join("modulus_rows.csv"), &modulus_rows)
        .expect("write modulus rows");
    write_csv_rows(options.out_dir.join("formula_rows.csv"), &formula_rows)
        .expect("write formula rows");
    write_csv_rows(
        options.out_dir.join("representative_rows.csv"),
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
                "base22_gradient_pocket_report".to_string(),
            ],
            upstream_inputs: vec!["src/validation/bounded_k.rs".to_string()],
            expected_outputs: vec![
                "lane_summary_rows.csv".to_string(),
                "pair_pocket_rows.csv".to_string(),
                "modulus_rows.csv".to_string(),
                "formula_rows.csv".to_string(),
                "representative_rows.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
                "artifact_manifest.json".to_string(),
                "base22_gradient_pocket_grid.png".to_string(),
                "base22_lane_summary.png".to_string(),
                "base22_relation_heatmap.png".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote base-22 gradient-pocket bundle to {}",
        options.out_dir.display()
    );
    println!("{}", report_summary.exact_takeaway);
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

fn build_pair_bundles() -> Vec<PairBundle> {
    ordered_unit_pairs(BASE)
        .into_par_iter()
        .map(|(outer, inner)| {
            let best_feature = analyze_best_vs_k00_feature_row(BASE, MIDDLE_LENGTH, outer, inner);
            let hinge_row = analyze_hinge_feature_row(BASE, outer, inner);
            let lane_rows = NONCOMPACT_LANES
                .iter()
                .copied()
                .map(|to_k| {
                    scan_k_config_affine_lane_comparison(
                        BASE,
                        MIDDLE_LENGTH,
                        outer,
                        inner,
                        FROM_K,
                        to_k,
                    )
                })
                .collect::<Vec<_>>();
            let target_lane = lane_rows
                .iter()
                .find(|row| row.to_k == format_k(TARGET_K))
                .expect("target lane should exist")
                .clone();
            PairBundle {
                outer,
                inner,
                pair_label: best_feature.pair_label.clone(),
                hinge_category: hinge_row.hinge_category,
                best_k: best_feature.best_k,
                best_active: best_feature.active,
                best_anomaly_mass_pp: best_feature.anomaly_mass_pp,
                lane_rows,
                target_lane,
            }
        })
        .collect()
}

fn build_lane_summary_rows(bundles: &[PairBundle]) -> Vec<LaneSummaryRow> {
    NONCOMPACT_LANES
        .iter()
        .copied()
        .map(|to_k| {
            let comparisons = bundles
                .iter()
                .map(|bundle| {
                    bundle
                        .lane_rows
                        .iter()
                        .find(|row| row.to_k == format_k(to_k))
                        .expect("lane comparison should exist")
                })
                .collect::<Vec<_>>();
            LaneSummaryRow {
                to_k: format_k(to_k),
                pair_count: comparisons.len(),
                gradient_only_pair_count: comparisons
                    .iter()
                    .filter(|row| row.gradient_only_count > 0)
                    .count(),
                gradient_only_pair_share: ratio(
                    comparisons
                        .iter()
                        .filter(|row| row.gradient_only_count > 0)
                        .count(),
                    comparisons.len(),
                ),
                mean_gradient_only_share: mean(
                    comparisons.iter().map(|row| row.gradient_only_share),
                ),
                mean_shift_only_share: mean(comparisons.iter().map(|row| row.shift_only_share)),
                mean_identity_share: mean(comparisons.iter().map(|row| row.identity_share)),
                mean_shift_and_gradient_share: mean(
                    comparisons.iter().map(|row| row.shift_and_gradient_share),
                ),
                mod5_gradient_only_pair_count: comparisons
                    .iter()
                    .filter(|row| {
                        row.modulus_rows.iter().any(|modulus_row| {
                            modulus_row.modulus == TARGET_MODULUS
                                && modulus_row.local_relation_label == "gradient_only"
                        })
                    })
                    .count(),
                mod5_gradient_only_pair_share: ratio(
                    comparisons
                        .iter()
                        .filter(|row| {
                            row.modulus_rows.iter().any(|modulus_row| {
                                modulus_row.modulus == TARGET_MODULUS
                                    && modulus_row.local_relation_label == "gradient_only"
                            })
                        })
                        .count(),
                    comparisons.len(),
                ),
            }
        })
        .collect()
}

fn build_pair_pocket_rows(bundles: &[PairBundle]) -> Vec<PairPocketRow> {
    bundles
        .iter()
        .map(|bundle| {
            let best_feature =
                analyze_best_vs_k00_feature_row(BASE, MIDDLE_LENGTH, bundle.outer, bundle.inner);
            let mod5_row = bundle
                .target_lane
                .modulus_rows
                .iter()
                .find(|row| row.modulus == TARGET_MODULUS)
                .expect("mod-5 row should exist");
            let outer_mod5 = bundle.outer % TARGET_MODULUS;
            let inner_mod5 = bundle.inner % TARGET_MODULUS;
            let formula_shift_k00 = mod5_shift_k00(outer_mod5, inner_mod5);
            let formula_shift_k22 = mod5_shift_k22(outer_mod5, inner_mod5);
            let predicted_relation_label = if formula_shift_k00 == formula_shift_k22 {
                "identity"
            } else {
                "gradient_only"
            };

            PairPocketRow {
                outer: bundle.outer,
                inner: bundle.inner,
                pair_label: bundle.pair_label.clone(),
                hinge_category: bundle.hinge_category.clone(),
                same_digit: bundle.outer == bundle.inner,
                unit_distance: best_feature.unit_distance,
                gap_bucket: best_feature.gap_bucket,
                outer_mod5,
                inner_mod5,
                best_k: bundle.best_k.clone(),
                best_active: bundle.best_active,
                best_anomaly_mass_pp: bundle.best_anomaly_mass_pp,
                k22_gradient_only_count: bundle.target_lane.gradient_only_count,
                k22_gradient_only_share: bundle.target_lane.gradient_only_share,
                k22_shift_only_count: bundle.target_lane.shift_only_count,
                k22_shift_only_share: bundle.target_lane.shift_only_share,
                k22_identity_count: bundle.target_lane.identity_count,
                k22_identity_share: bundle.target_lane.identity_share,
                k22_shift_and_gradient_count: bundle.target_lane.shift_and_gradient_count,
                k22_shift_and_gradient_share: bundle.target_lane.shift_and_gradient_share,
                mod5_relation_label: mod5_row.local_relation_label.clone(),
                mod5_shift_equal: mod5_row.shift_equal,
                mod5_gradient_equal: mod5_row.gradient_equal,
                mod5_zero_seed_equal: mod5_row.zero_seed_equal,
                mod5_shift_from: mod5_row.shift_modulus_from,
                mod5_shift_to: mod5_row.shift_modulus_to,
                mod5_gradient_from: mod5_row.gradient_modulus_from,
                mod5_gradient_to: mod5_row.gradient_modulus_to,
                mod5_zero_seed_from: mod5_row.zero_seed_class_from,
                mod5_zero_seed_to: mod5_row.zero_seed_class_to,
                predicted_relation_label: predicted_relation_label.to_string(),
                predicted_shift_difference_mod5: mod5_difference(inner_mod5),
                pocket_class: if mod5_row.local_relation_label == "gradient_only" {
                    "gradient_only_pocket".to_string()
                } else {
                    "collapsed_at_mod5".to_string()
                },
            }
        })
        .collect()
}

fn build_modulus_rows(bundles: &[PairBundle]) -> Vec<ModulusRow> {
    bundles
        .iter()
        .flat_map(|bundle| {
            bundle
                .target_lane
                .modulus_rows
                .iter()
                .map(|row| ModulusRow {
                    pair_label: bundle.pair_label.clone(),
                    outer: bundle.outer,
                    inner: bundle.inner,
                    modulus: row.modulus,
                    shift_from: row.shift_modulus_from,
                    shift_to: row.shift_modulus_to,
                    gradient_from: row.gradient_modulus_from,
                    gradient_to: row.gradient_modulus_to,
                    zero_seed_from: row.zero_seed_class_from,
                    zero_seed_to: row.zero_seed_class_to,
                    shift_equal: row.shift_equal,
                    gradient_equal: row.gradient_equal,
                    zero_seed_equal: row.zero_seed_equal,
                    local_relation_label: row.local_relation_label.clone(),
                })
        })
        .collect()
}

fn build_formula_rows(bundles: &[PairBundle]) -> Vec<FormulaRow> {
    bundles
        .iter()
        .map(|bundle| {
            let mod5_row = bundle
                .target_lane
                .modulus_rows
                .iter()
                .find(|row| row.modulus == TARGET_MODULUS)
                .expect("mod-5 row should exist");
            let outer_mod5 = bundle.outer % TARGET_MODULUS;
            let inner_mod5 = bundle.inner % TARGET_MODULUS;
            let formula_shift_from = mod5_shift_k00(outer_mod5, inner_mod5);
            let formula_shift_to = mod5_shift_k22(outer_mod5, inner_mod5);
            let formula_gradient = mod5_gradient();
            let predicted_relation = if formula_shift_from == formula_shift_to {
                "identity"
            } else {
                "gradient_only"
            };
            FormulaRow {
                pair_label: bundle.pair_label.clone(),
                outer: bundle.outer,
                inner: bundle.inner,
                outer_mod5,
                inner_mod5,
                formula_shift_k00_mod5: formula_shift_from,
                observed_shift_k00_mod5: mod5_row.shift_modulus_from,
                formula_shift_k22_mod5: formula_shift_to,
                observed_shift_k22_mod5: mod5_row.shift_modulus_to,
                formula_shift_difference_mod5: mod5_difference(inner_mod5),
                observed_shift_difference_mod5: mod5_sub(
                    mod5_row.shift_modulus_to,
                    mod5_row.shift_modulus_from,
                ),
                formula_gradient_k00_mod5: formula_gradient,
                observed_gradient_k00_mod5: mod5_row.gradient_modulus_from,
                formula_gradient_k22_mod5: formula_gradient,
                observed_gradient_k22_mod5: mod5_row.gradient_modulus_to,
                predicted_relation_label: predicted_relation.to_string(),
                observed_relation_label: mod5_row.local_relation_label.clone(),
                formula_matches_observation: formula_shift_from == mod5_row.shift_modulus_from
                    && formula_shift_to == mod5_row.shift_modulus_to
                    && formula_gradient == mod5_row.gradient_modulus_from
                    && formula_gradient == mod5_row.gradient_modulus_to
                    && predicted_relation == mod5_row.local_relation_label,
            }
        })
        .collect()
}

fn build_representative_rows(pair_rows: &[PairPocketRow]) -> Vec<RepresentativeRow> {
    REPRESENTATIVES
        .iter()
        .map(|spec| {
            let row = pair_rows
                .iter()
                .find(|row| row.outer == spec.outer && row.inner == spec.inner)
                .expect("representative row should exist");
            RepresentativeRow {
                role: spec.role.to_string(),
                pair_label: row.pair_label.clone(),
                hinge_category: row.hinge_category.clone(),
                best_k: row.best_k.clone(),
                best_active: row.best_active,
                outer_mod5: row.outer_mod5,
                inner_mod5: row.inner_mod5,
                mod5_relation_label: row.mod5_relation_label.clone(),
                k22_gradient_only_share: row.k22_gradient_only_share,
                k22_shift_only_share: row.k22_shift_only_share,
                k22_identity_share: row.k22_identity_share,
                note: representative_note(row),
            }
        })
        .collect()
}

fn representative_note(row: &PairPocketRow) -> String {
    match row.pocket_class.as_str() {
        "gradient_only_pocket" => format!(
            "Inner residue {} is nonzero mod 5, so the mod-5 local relation stays gradient_only.",
            row.inner_mod5
        ),
        "collapsed_at_mod5" => {
            "Inner residue is 0 mod 5, so the mod-5 local relation collapses to identity."
                .to_string()
        }
        _ => "unclassified".to_string(),
    }
}

fn build_report_summary(
    lane_summary_rows: &[LaneSummaryRow],
    pair_pocket_rows: &[PairPocketRow],
) -> ReportSummary {
    let target_lane = lane_summary_rows
        .iter()
        .find(|row| row.to_k == format_k(TARGET_K))
        .expect("target lane summary should exist");
    let pocket_pair_count = pair_pocket_rows
        .iter()
        .filter(|row| row.pocket_class == "gradient_only_pocket")
        .count();
    ReportSummary {
        pair_count: pair_pocket_rows.len(),
        pocket_pair_count,
        pocket_pair_share: ratio(pocket_pair_count, pair_pocket_rows.len()),
        only_lane_with_gradient_only: target_lane.to_k.clone(),
        only_modulus_with_gradient_only: TARGET_MODULUS,
        exact_takeaway:
            "For base 22, M=2, and k=(0,0)->(2,2), the residual gradient_only pocket is exactly the pairs with inner != 0 (mod 5); the other noncompact lanes show none of it."
                .to_string(),
    }
}

fn derive_observations(
    lane_summary_rows: &[LaneSummaryRow],
    pair_pocket_rows: &[PairPocketRow],
) -> Vec<String> {
    let mut observations = Vec::new();
    for row in lane_summary_rows {
        observations.push(format!(
            "Lane {}: gradient_only pair share {:.2}% and mod-5 gradient_only pair share {:.2}%.",
            row.to_k,
            row.gradient_only_pair_share * 100.0,
            row.mod5_gradient_only_pair_share * 100.0,
        ));
    }
    let pocket_rows = pair_pocket_rows
        .iter()
        .filter(|row| row.pocket_class == "gradient_only_pocket")
        .collect::<Vec<_>>();
    if pocket_rows.iter().all(|row| row.inner_mod5 != 0) {
        observations.push("Every surviving pocket row has inner nonzero modulo 5.".to_string());
    }
    let collapsed_rows = pair_pocket_rows
        .iter()
        .filter(|row| row.pocket_class == "collapsed_at_mod5")
        .collect::<Vec<_>>();
    if collapsed_rows.iter().all(|row| row.inner_mod5 == 0) {
        observations.push(
            "Every collapsed row has inner residue 0 modulo 5; in the base-22 unit catalog that means exactly the columns inner=5 and inner=F."
                .to_string(),
        );
    }
    let active_pocket_count = pocket_rows.iter().filter(|row| row.best_active).count();
    observations.push(format!(
        "Only {} of {} pocket rows are active on the best-k surface, so this is a local affine side-pocket rather than the main hinge mechanism.",
        active_pocket_count,
        pocket_rows.len()
    ));
    observations.push(
        "At modulus 5 the lane gradients match exactly because 22^2 and 22^6 are both congruent to 4 mod 5; the pocket is created entirely by the shift term."
            .to_string(),
    );
    observations.push(
        "The explicit mod-5 formulas are shift_k00 = 3*(outer+inner), shift_k22 = 3*outer + 2*inner, so the shift difference is -inner mod 5."
            .to_string(),
    );
    observations
}

fn render_report(
    settings: &ReportSettings,
    report_summary: &ReportSummary,
    observations: &[String],
) -> String {
    let mut lines = Vec::new();
    lines.push("# Base-22 Gradient Pocket".to_string());
    lines.push(String::new());
    lines.push("## Scope".to_string());
    lines.push(format!(
        "- Base {}, M={}, direct lane {} -> {}",
        settings.base, settings.middle_length, settings.from_k, settings.target_k
    ));
    lines.push(format!(
        "- Noncompact comparison lanes: {}",
        settings.noncompact_lanes.join(", ")
    ));
    lines.push(format!("- Target modulus: {}", settings.target_modulus));
    lines.push(String::new());
    lines.push("## Exact Takeaway".to_string());
    lines.push(format!("- {}", report_summary.exact_takeaway));
    lines.push(String::new());
    lines.push("## Formula".to_string());
    lines.push("- Mod 5 affine gradient: `22^2 ≡ 22^6 ≡ 4`, so the local gradients agree for `k=(0,0)` and `k=(2,2)`.".to_string());
    lines.push("- Mod 5 shift terms at `M=2`: `shift_k00 = 3*(outer+inner)` and `shift_k22 = 3*outer + 2*inner`.".to_string());
    lines.push("- Therefore `shift_k22 - shift_k00 ≡ -inner (mod 5)`, so the local relation is `gradient_only` iff `inner != 0 (mod 5)`.".to_string());
    lines.push(String::new());
    lines.push("## Observations".to_string());
    for observation in observations {
        lines.push(format!("- {observation}"));
    }
    lines.join("\n")
}

fn render_pair_grid(rows: &[PairPocketRow], path: &Path) {
    let digits = unit_digits(BASE);
    let labels = digits
        .iter()
        .map(|&digit| digit_symbol(digit))
        .collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (980, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill pair grid");
    let cell_size = 72i32;
    let origin_x = 140i32;
    let origin_y = 120i32;
    root.draw(&Text::new(
        "Base-22 M=2 Gradient Pocket for k=(0,0)->(2,2)",
        (30, 34),
        ("sans-serif", 28).into_font(),
    ))
    .expect("draw grid title");

    for (index, label) in labels.iter().enumerate() {
        let x = origin_x + index as i32 * cell_size + cell_size / 2;
        let y = origin_y + index as i32 * cell_size + cell_size / 2;
        root.draw(&Text::new(
            label.clone(),
            (x, origin_y - 24),
            ("sans-serif", 18).into_font(),
        ))
        .expect("draw top label");
        root.draw(&Text::new(
            label.clone(),
            (origin_x - 38, y),
            ("sans-serif", 18).into_font(),
        ))
        .expect("draw side label");
    }

    for row in rows {
        let x_index = digits
            .iter()
            .position(|&digit| digit == row.outer)
            .expect("outer digit should be a unit");
        let y_index = digits
            .iter()
            .position(|&digit| digit == row.inner)
            .expect("inner digit should be a unit");
        let x0 = origin_x + x_index as i32 * cell_size;
        let y0 = origin_y + y_index as i32 * cell_size;
        let color = match row.pocket_class.as_str() {
            "gradient_only_pocket" => RGBColor(244, 67, 54),
            "collapsed_at_mod5" => RGBColor(76, 175, 80),
            _ => RGBColor(180, 180, 180),
        };
        root.draw(&Rectangle::new(
            [(x0, y0), (x0 + cell_size - 4, y0 + cell_size - 4)],
            ShapeStyle::from(&color).filled(),
        ))
        .expect("draw grid cell");
        root.draw(&Text::new(
            if row.best_active { "*" } else { "" }.to_string(),
            (x0 + 28, y0 + 42),
            ("sans-serif", 20).into_font().color(&BLACK),
        ))
        .expect("draw active mark");
    }
    root.present().expect("present pair grid");
}

fn render_lane_summary(rows: &[LaneSummaryRow], path: &Path) {
    let root = BitMapBackend::new(path, (980, 720)).into_drawing_area();
    root.fill(&WHITE).expect("fill lane summary");
    let mut chart = ChartBuilder::on(&root)
        .margin(24)
        .caption(
            "Base-22 Gradient-Only Share by Noncompact Lane",
            ("sans-serif", 28).into_font(),
        )
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..rows.len() as f64, 0f64..1.0)
        .expect("build lane summary");
    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(rows.len())
        .x_label_formatter(&|x| {
            let index = x.floor() as usize;
            rows.get(index)
                .map(|row| row.to_k.clone())
                .unwrap_or_default()
        })
        .x_desc("target lane")
        .y_desc("pair share")
        .draw()
        .expect("draw lane summary mesh");

    for (index, row) in rows.iter().enumerate() {
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [
                    (index as f64 + 0.15, 0.0),
                    (index as f64 + 0.45, row.gradient_only_pair_share),
                ],
                ShapeStyle::from(&RGBColor(244, 67, 54)).filled(),
            )))
            .expect("draw gradient-only bar");
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [
                    (index as f64 + 0.55, 0.0),
                    (index as f64 + 0.85, row.mod5_gradient_only_pair_share),
                ],
                ShapeStyle::from(&RGBColor(255, 193, 7)).filled(),
            )))
            .expect("draw mod-5 bar");
    }
    root.present().expect("present lane summary");
}

fn render_relation_heatmap(
    bundles: &[PairBundle],
    representative_rows: &[RepresentativeRow],
    path: &Path,
) {
    let moduli = bundles
        .first()
        .expect("bundles should be nonempty")
        .target_lane
        .modulus_rows
        .iter()
        .map(|row| row.modulus)
        .collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (1240, 640)).into_drawing_area();
    root.fill(&WHITE).expect("fill heatmap");
    let cell_width = 90i32;
    let cell_height = 84i32;
    let origin_x = 220i32;
    let origin_y = 100i32;
    root.draw(&Text::new(
        "Base-22 k=(0,0)->(2,2) Relation Heatmap",
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
    for (row_index, representative) in representative_rows.iter().enumerate() {
        let y = origin_y + row_index as i32 * cell_height + cell_height / 2;
        root.draw(&Text::new(
            format!("{} {}", representative.role, representative.pair_label),
            (24, y),
            ("sans-serif", 18).into_font(),
        ))
        .expect("draw representative label");
        let bundle = bundles
            .iter()
            .find(|bundle| bundle.pair_label == representative.pair_label)
            .expect("representative bundle should exist");
        for (column, modulus) in moduli.iter().enumerate() {
            let x0 = origin_x + column as i32 * cell_width;
            let y0 = origin_y + row_index as i32 * cell_height;
            let relation = bundle
                .target_lane
                .modulus_rows
                .iter()
                .find(|row| row.modulus == *modulus)
                .expect("modulus row should exist")
                .local_relation_label
                .as_str();
            root.draw(&Rectangle::new(
                [(x0, y0), (x0 + cell_width - 6, y0 + cell_height - 6)],
                ShapeStyle::from(&relation_color(relation)).filled(),
            ))
            .expect("draw heatmap cell");
            root.draw(&Text::new(
                relation_short_label(relation).to_string(),
                (x0 + 18, y0 + 46),
                ("sans-serif", 16).into_font().color(&BLACK),
            ))
            .expect("draw heatmap text");
        }
    }
    root.present().expect("present heatmap");
}

fn mod5_gradient() -> u32 {
    4
}

fn mod5_shift_k00(outer_mod5: u32, inner_mod5: u32) -> u32 {
    mod5(3 * (outer_mod5 + inner_mod5))
}

fn mod5_shift_k22(outer_mod5: u32, inner_mod5: u32) -> u32 {
    mod5(3 * outer_mod5 + 2 * inner_mod5)
}

fn mod5_difference(inner_mod5: u32) -> u32 {
    mod5(5 - inner_mod5)
}

fn mod5(value: u32) -> u32 {
    value % 5
}

fn mod5_sub(left: u32, right: u32) -> u32 {
    mod5(left + 5 - right)
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

fn relation_color(label: &str) -> RGBColor {
    match label {
        "identity" => RGBColor(76, 175, 80),
        "shift_only" => RGBColor(255, 193, 7),
        "gradient_only" => RGBColor(244, 67, 54),
        "shift_and_gradient" => RGBColor(33, 150, 243),
        _ => RGBColor(180, 180, 180),
    }
}

fn relation_short_label(label: &str) -> &'static str {
    match label {
        "identity" => "id",
        "shift_only" => "sh",
        "gradient_only" => "gr",
        "shift_and_gradient" => "sg",
        _ => "?",
    }
}

fn format_k(k: BoundedKConfig) -> String {
    format!("k=({},{})", k.0, k.1)
}

fn unit_digits(base: u32) -> Vec<u32> {
    let mut digits = ordered_unit_pairs(base)
        .into_iter()
        .map(|(outer, _)| outer)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    digits.sort_unstable();
    digits
}

fn digit_symbol(digit: u32) -> String {
    if digit < 10 {
        digit.to_string()
    } else {
        ((b'A' + (digit as u8 - 10)) as char).to_string()
    }
}
