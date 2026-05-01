//! Transfer-vocabulary collapse report for the bounded-`k` lane.
//!
//! This report asks a precise version of the recent campfire question:
//! why does the transfer vocabulary become meaningful at `M=2` and then
//! collapse by `M=3`?
//!
//! The maintained interpretation here is arithmetic-first:
//! - `M=2` has a sparse but real competitive surface where `best_k != k=(0,0)`
//!   and the exact transfer buckets (`stable_zero`, `gain_zero`, `loss_zero`,
//!   `stable_nonzero`, `nonzero_churn`) still carry explanatory load.
//! - `M=3` collapses that surface: the best lane returns to `k=(0,0)`,
//!   nonidentity transfer vanishes, and the transfer grammar degenerates into
//!   an identity profile.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example m2_m3_transfer_collapse_report
//! cargo run --release --example m2_m3_transfer_collapse_report -- --out-dir /tmp/primes_m2_m3_transfer_collapse_alt
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{analyze_best_vs_k00_feature_row, ordered_unit_pairs, BestVsK00FeatureRow},
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

const BASES: &[u32] = &[6, 10, 12, 14, 22, 26, 30, 34];
const M2: usize = 2;
const M3: usize = 3;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_m2_m3_transfer_collapse";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "m2_m3_transfer_collapse_report";

const REPRESENTATIVES: &[RepresentativeSpec] = &[
    RepresentativeSpec {
        role: "persistent_core",
        base: 14,
        outer: 13,
        inner: 11,
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
        role: "boundary_release",
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

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    bases: Vec<u32>,
    middle_lengths: Vec<usize>,
    pair_catalog_mode: String,
}

#[derive(Debug, Clone, Serialize)]
struct PairTransferRow {
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    pair_label: String,
    hinge_like_role: String,
    best_k: String,
    active: bool,
    noncompact_winner: bool,
    identity_collapse: bool,
    signal_source_label: String,
    same_digit: bool,
    gap_bucket: String,
    unit_distance: usize,
    anomaly_mass_pp: f64,
    stable_zero_signal_margin_count: isize,
    stable_zero_signal_margin_pp: f64,
    stable_zero_support_ratio: f64,
    stable_zero_prime_delta_count: isize,
    boundary_prime_delta_count: isize,
    stable_zero_prime_delta_pp: f64,
    boundary_prime_delta_pp: f64,
    admissible_overlap_jaccard: f64,
    mask_stability_share: f64,
    nonidentity_transfer_share: f64,
    boundary_bucket_share: f64,
    churn_share: f64,
    transfer_bucket_richness: usize,
    stable_zero_count: usize,
    gain_zero_count: usize,
    loss_zero_count: usize,
    stable_nonzero_count: usize,
    nonzero_churn_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct PairTransitionRow {
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    role: String,
    m2_best_k: String,
    m3_best_k: String,
    m2_active: bool,
    m3_active: bool,
    m2_signal_source_label: String,
    m3_signal_source_label: String,
    m2_anomaly_mass_pp: f64,
    m3_anomaly_mass_pp: f64,
    anomaly_mass_drop_pp: f64,
    m2_nonidentity_transfer_share: f64,
    m3_nonidentity_transfer_share: f64,
    nonidentity_transfer_drop_share: f64,
    m2_transfer_bucket_richness: usize,
    m3_transfer_bucket_richness: usize,
    m2_stable_zero_signal_margin_count: isize,
    m3_stable_zero_signal_margin_count: isize,
    collapse_class: String,
}

#[derive(Debug, Clone, Serialize)]
struct BaseTransferSummaryRow {
    base: u32,
    middle_length: usize,
    ordered_pair_count: usize,
    active_pair_count: usize,
    active_pair_share: f64,
    noncompact_winner_share: f64,
    identity_collapse_share: f64,
    signal_source_diversity_active: usize,
    mean_nonidentity_transfer_share_all: f64,
    mean_transfer_bucket_richness_all: f64,
    mean_nonidentity_transfer_share_active: Option<f64>,
    mean_boundary_bucket_share_active: Option<f64>,
    mean_churn_share_active: Option<f64>,
    mean_stable_zero_signal_margin_count_active: Option<f64>,
    leading_pair: String,
    leading_signal_source_label: String,
    leading_anomaly_mass_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct VocabularySummaryRow {
    middle_length: usize,
    scope: String,
    row_count: usize,
    active_pair_share: Option<f64>,
    noncompact_winner_share: f64,
    identity_collapse_share: f64,
    mean_nonidentity_transfer_share: f64,
    mean_boundary_bucket_share: f64,
    mean_churn_share: f64,
    mean_transfer_bucket_richness: f64,
    stable_zero_led_share: f64,
    boundary_led_share: f64,
    mixed_or_flat_share: f64,
    signal_source_diversity: usize,
}

#[derive(Debug, Clone, Serialize)]
struct RepresentativeTransferRow {
    role: String,
    base: u32,
    pair_label: String,
    middle_length: usize,
    best_k: String,
    active: bool,
    identity_collapse: bool,
    signal_source_label: String,
    anomaly_mass_pp: f64,
    stable_zero_signal_margin_count: isize,
    stable_zero_prime_delta_count: isize,
    boundary_prime_delta_count: isize,
    nonidentity_transfer_share: f64,
    boundary_bucket_share: f64,
    churn_share: f64,
    transfer_bucket_richness: usize,
    mechanism_sentence: String,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    total_pairs_per_length: usize,
    m2_active_pairs: usize,
    m3_active_pairs: usize,
    m2_noncompact_winner_share: f64,
    m3_noncompact_winner_share: f64,
    m2_identity_collapse_share: f64,
    m3_identity_collapse_share: f64,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    pair_transfer_rows: Vec<PairTransferRow>,
    pair_transition_rows: Vec<PairTransitionRow>,
    base_transfer_summary_rows: Vec<BaseTransferSummaryRow>,
    vocabulary_summary_rows: Vec<VocabularySummaryRow>,
    representative_transfer_rows: Vec<RepresentativeTransferRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let pair_transfer_rows = build_pair_transfer_rows();
    let pair_transition_rows = build_pair_transition_rows(&pair_transfer_rows);
    let base_transfer_summary_rows = build_base_transfer_summary_rows(&pair_transfer_rows);
    let vocabulary_summary_rows = build_vocabulary_summary_rows(&pair_transfer_rows);
    let representative_transfer_rows = build_representative_transfer_rows(&pair_transfer_rows);

    let collapse_path = options.out_dir.join("transfer_vocabulary_collapse.png");
    render_transfer_vocabulary_collapse(&vocabulary_summary_rows, &collapse_path);
    let plane_path = options.out_dir.join("transfer_meaning_plane.png");
    render_transfer_meaning_plane(
        &pair_transfer_rows,
        &representative_transfer_rows,
        &plane_path,
    );
    let strip_path = options.out_dir.join("representative_transfer_strip.png");
    render_representative_transfer_strip(&representative_transfer_rows, &strip_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "transfer_vocabulary_collapse".to_string(),
            label: "Transfer vocabulary collapse".to_string(),
            path: collapse_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "transfer_meaning_plane".to_string(),
            label: "Transfer meaning plane".to_string(),
            path: plane_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "representative_transfer_strip".to_string(),
            label: "Representative transfer strip".to_string(),
            path: strip_path.display().to_string(),
        },
    ];

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        bases: BASES.to_vec(),
        middle_lengths: vec![M2, M3],
        pair_catalog_mode: "full".to_string(),
    };

    let report_summary = build_report_summary(&pair_transfer_rows, &vocabulary_summary_rows);
    let observations = derive_observations(
        &pair_transfer_rows,
        &pair_transition_rows,
        &base_transfer_summary_rows,
        &vocabulary_summary_rows,
        &representative_transfer_rows,
    );

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        pair_transfer_rows: pair_transfer_rows.clone(),
        pair_transition_rows: pair_transition_rows.clone(),
        base_transfer_summary_rows: base_transfer_summary_rows.clone(),
        vocabulary_summary_rows: vocabulary_summary_rows.clone(),
        representative_transfer_rows: representative_transfer_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("pair_transfer_rows.csv"),
        &pair_transfer_rows,
    )
    .expect("failed to write pair_transfer_rows.csv");
    write_csv_rows(
        options.out_dir.join("pair_transition_rows.csv"),
        &pair_transition_rows,
    )
    .expect("failed to write pair_transition_rows.csv");
    write_csv_rows(
        options.out_dir.join("base_transfer_summary_rows.csv"),
        &base_transfer_summary_rows,
    )
    .expect("failed to write base_transfer_summary_rows.csv");
    write_csv_rows(
        options.out_dir.join("vocabulary_summary_rows.csv"),
        &vocabulary_summary_rows,
    )
    .expect("failed to write vocabulary_summary_rows.csv");
    write_csv_rows(
        options.out_dir.join("representative_transfer_rows.csv"),
        &representative_transfer_rows,
    )
    .expect("failed to write representative_transfer_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");
    write_text_file(options.out_dir.join("report.md"), &render_markdown(&bundle))
        .expect("failed to write report.md");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--release".to_string(),
                "--example".to_string(),
                "m2_m3_transfer_collapse_report".to_string(),
                "--".to_string(),
                "--out-dir".to_string(),
                options.out_dir.display().to_string(),
            ],
            upstream_inputs: vec![],
            expected_outputs: vec![
                "pair_transfer_rows.csv".to_string(),
                "pair_transition_rows.csv".to_string(),
                "base_transfer_summary_rows.csv".to_string(),
                "vocabulary_summary_rows.csv".to_string(),
                "representative_transfer_rows.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
                "artifact_manifest.json".to_string(),
                "transfer_vocabulary_collapse.png".to_string(),
                "transfer_meaning_plane.png".to_string(),
                "representative_transfer_strip.png".to_string(),
            ],
        },
    )
    .expect("failed to write artifact manifest");

    println!("m2->m3 transfer collapse report");
    println!("  output dir: {}", options.out_dir.display());
    for row in &vocabulary_summary_rows {
        println!(
            "  M{} {:<6} | n {:>3} | active {} | noncompact {} | identity {} | nonidentity {:.3}",
            row.middle_length,
            row.scope,
            row.row_count,
            format_option_share(row.active_pair_share),
            format_share(row.noncompact_winner_share),
            format_share(row.identity_collapse_share),
            row.mean_nonidentity_transfer_share,
        );
    }
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                let value = args
                    .next()
                    .expect("--out-dir requires a directory argument");
                out_dir = PathBuf::from(value);
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => panic!("unrecognized argument: {arg}"),
        }
    }
    Options { out_dir }
}

fn print_help() {
    println!("Usage:");
    println!("  cargo run --release --example m2_m3_transfer_collapse_report -- [options]");
    println!();
    println!("Options:");
    println!("  --out-dir <dir>   Output directory (default: {DEFAULT_OUT_DIR})");
    println!("  -h, --help        Show this help message");
}

fn build_pair_transfer_rows() -> Vec<PairTransferRow> {
    build_pair_transfer_rows_for_bases(BASES)
}

fn build_pair_transfer_rows_for_bases(bases: &[u32]) -> Vec<PairTransferRow> {
    let tasks = bases
        .iter()
        .copied()
        .flat_map(|base| {
            ordered_unit_pairs(base)
                .into_iter()
                .flat_map(move |(outer, inner)| {
                    [(base, M2, outer, inner), (base, M3, outer, inner)]
                })
        })
        .collect::<Vec<_>>();

    let mut rows = tasks
        .par_iter()
        .map(|&(base, middle_length, outer, inner)| {
            let feature = analyze_best_vs_k00_feature_row(base, middle_length, outer, inner);
            pair_transfer_row(&feature)
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.middle_length.cmp(&right.middle_length))
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
    });
    rows
}

#[cfg(test)]
fn build_pair_transfer_rows_for_pairs(pairs: &[(u32, u32, u32)]) -> Vec<PairTransferRow> {
    let tasks = pairs
        .iter()
        .flat_map(|&(base, outer, inner)| [(base, M2, outer, inner), (base, M3, outer, inner)])
        .collect::<Vec<_>>();

    let mut rows = tasks
        .iter()
        .map(|&(base, middle_length, outer, inner)| {
            let feature = analyze_best_vs_k00_feature_row(base, middle_length, outer, inner);
            pair_transfer_row(&feature)
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.middle_length.cmp(&right.middle_length))
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
    });
    rows
}

fn pair_transfer_row(feature: &BestVsK00FeatureRow) -> PairTransferRow {
    PairTransferRow {
        base: feature.base,
        middle_length: feature.middle_length,
        outer: feature.outer,
        inner: feature.inner,
        pair_label: feature.pair_label.clone(),
        hinge_like_role: representative_role(feature.base, feature.outer, feature.inner)
            .unwrap_or("other")
            .to_string(),
        best_k: feature.best_k.clone(),
        active: feature.active,
        noncompact_winner: feature.best_k != "k=(0,0)",
        identity_collapse: feature.gain_zero_count == 0
            && feature.loss_zero_count == 0
            && feature.nonzero_churn_count == 0,
        signal_source_label: feature.signal_source_label.clone(),
        same_digit: feature.same_digit,
        gap_bucket: feature.gap_bucket.clone(),
        unit_distance: feature.unit_distance,
        anomaly_mass_pp: feature.anomaly_mass_pp,
        stable_zero_signal_margin_count: feature.stable_zero_signal_margin_count,
        stable_zero_signal_margin_pp: feature.stable_zero_signal_margin_pp,
        stable_zero_support_ratio: feature.stable_zero_support_ratio,
        stable_zero_prime_delta_count: feature.stable_zero_prime_delta_count,
        boundary_prime_delta_count: feature.boundary_prime_delta_count,
        stable_zero_prime_delta_pp: feature.stable_zero_prime_delta_pp,
        boundary_prime_delta_pp: feature.boundary_prime_delta_pp,
        admissible_overlap_jaccard: feature.admissible_overlap_jaccard,
        mask_stability_share: feature.mask_stability_share,
        nonidentity_transfer_share: ratio(
            feature.gain_zero_count + feature.loss_zero_count + feature.nonzero_churn_count,
            feature.candidates_per_config,
        ),
        boundary_bucket_share: ratio(
            feature.gain_zero_count + feature.loss_zero_count,
            feature.candidates_per_config,
        ),
        churn_share: ratio(feature.nonzero_churn_count, feature.candidates_per_config),
        transfer_bucket_richness: [
            feature.stable_zero_count,
            feature.gain_zero_count,
            feature.loss_zero_count,
            feature.stable_nonzero_count,
            feature.nonzero_churn_count,
        ]
        .into_iter()
        .filter(|&count| count > 0)
        .count(),
        stable_zero_count: feature.stable_zero_count,
        gain_zero_count: feature.gain_zero_count,
        loss_zero_count: feature.loss_zero_count,
        stable_nonzero_count: feature.stable_nonzero_count,
        nonzero_churn_count: feature.nonzero_churn_count,
    }
}

fn build_pair_transition_rows(pair_rows: &[PairTransferRow]) -> Vec<PairTransitionRow> {
    let mut by_pair = BTreeMap::<(u32, u32, u32), Vec<&PairTransferRow>>::new();
    for row in pair_rows {
        by_pair
            .entry((row.base, row.outer, row.inner))
            .or_default()
            .push(row);
    }

    let mut rows = by_pair
        .into_values()
        .map(|group| {
            let m2 = group
                .iter()
                .copied()
                .find(|row| row.middle_length == M2)
                .expect("M2 row should exist");
            let m3 = group
                .iter()
                .copied()
                .find(|row| row.middle_length == M3)
                .expect("M3 row should exist");

            PairTransitionRow {
                base: m2.base,
                outer: m2.outer,
                inner: m2.inner,
                pair_label: m2.pair_label.clone(),
                role: m2.hinge_like_role.clone(),
                m2_best_k: m2.best_k.clone(),
                m3_best_k: m3.best_k.clone(),
                m2_active: m2.active,
                m3_active: m3.active,
                m2_signal_source_label: m2.signal_source_label.clone(),
                m3_signal_source_label: m3.signal_source_label.clone(),
                m2_anomaly_mass_pp: m2.anomaly_mass_pp,
                m3_anomaly_mass_pp: m3.anomaly_mass_pp,
                anomaly_mass_drop_pp: m2.anomaly_mass_pp - m3.anomaly_mass_pp,
                m2_nonidentity_transfer_share: m2.nonidentity_transfer_share,
                m3_nonidentity_transfer_share: m3.nonidentity_transfer_share,
                nonidentity_transfer_drop_share: m2.nonidentity_transfer_share
                    - m3.nonidentity_transfer_share,
                m2_transfer_bucket_richness: m2.transfer_bucket_richness,
                m3_transfer_bucket_richness: m3.transfer_bucket_richness,
                m2_stable_zero_signal_margin_count: m2.stable_zero_signal_margin_count,
                m3_stable_zero_signal_margin_count: m3.stable_zero_signal_margin_count,
                collapse_class: collapse_class(m2, m3).to_string(),
            }
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        right
            .m2_anomaly_mass_pp
            .total_cmp(&left.m2_anomaly_mass_pp)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    rows
}

fn build_base_transfer_summary_rows(pair_rows: &[PairTransferRow]) -> Vec<BaseTransferSummaryRow> {
    let mut by_group = BTreeMap::<(u32, usize), Vec<&PairTransferRow>>::new();
    for row in pair_rows {
        by_group
            .entry((row.base, row.middle_length))
            .or_default()
            .push(row);
    }

    by_group
        .into_iter()
        .map(|((base, middle_length), rows)| {
            let active_rows = rows
                .iter()
                .copied()
                .filter(|row| row.active)
                .collect::<Vec<_>>();
            let leading = rows
                .iter()
                .copied()
                .max_by(|left, right| {
                    left.anomaly_mass_pp
                        .total_cmp(&right.anomaly_mass_pp)
                        .then_with(|| left.pair_label.cmp(&right.pair_label))
                })
                .expect("group should not be empty");
            let active_signals = active_rows
                .iter()
                .map(|row| row.signal_source_label.clone())
                .collect::<BTreeSet<_>>();
            BaseTransferSummaryRow {
                base,
                middle_length,
                ordered_pair_count: rows.len(),
                active_pair_count: active_rows.len(),
                active_pair_share: ratio(active_rows.len(), rows.len()),
                noncompact_winner_share: ratio(
                    rows.iter().filter(|row| row.noncompact_winner).count(),
                    rows.len(),
                ),
                identity_collapse_share: ratio(
                    rows.iter().filter(|row| row.identity_collapse).count(),
                    rows.len(),
                ),
                signal_source_diversity_active: active_signals.len(),
                mean_nonidentity_transfer_share_all: mean(
                    rows.iter().map(|row| row.nonidentity_transfer_share),
                ),
                mean_transfer_bucket_richness_all: mean(
                    rows.iter().map(|row| row.transfer_bucket_richness as f64),
                ),
                mean_nonidentity_transfer_share_active: mean_option(
                    active_rows.iter().map(|row| row.nonidentity_transfer_share),
                ),
                mean_boundary_bucket_share_active: mean_option(
                    active_rows.iter().map(|row| row.boundary_bucket_share),
                ),
                mean_churn_share_active: mean_option(active_rows.iter().map(|row| row.churn_share)),
                mean_stable_zero_signal_margin_count_active: mean_option(
                    active_rows
                        .iter()
                        .map(|row| row.stable_zero_signal_margin_count as f64),
                ),
                leading_pair: leading.pair_label.clone(),
                leading_signal_source_label: leading.signal_source_label.clone(),
                leading_anomaly_mass_pp: leading.anomaly_mass_pp,
            }
        })
        .collect()
}

fn build_vocabulary_summary_rows(pair_rows: &[PairTransferRow]) -> Vec<VocabularySummaryRow> {
    [M2, M3]
        .into_iter()
        .flat_map(|middle_length| {
            let rows = pair_rows
                .iter()
                .filter(|row| row.middle_length == middle_length)
                .collect::<Vec<_>>();
            let active_rows = rows
                .iter()
                .copied()
                .filter(|row| row.active)
                .collect::<Vec<_>>();
            vec![
                vocabulary_summary_row(
                    middle_length,
                    "all",
                    &rows,
                    active_rows.len(),
                    Some(rows.len()),
                ),
                vocabulary_summary_row(
                    middle_length,
                    "active",
                    &active_rows,
                    active_rows.len(),
                    Some(rows.len()),
                ),
            ]
        })
        .collect()
}

fn vocabulary_summary_row(
    middle_length: usize,
    scope: &str,
    rows: &[&PairTransferRow],
    active_pair_count: usize,
    base_total_rows: Option<usize>,
) -> VocabularySummaryRow {
    let signal_sources = rows
        .iter()
        .map(|row| row.signal_source_label.clone())
        .collect::<BTreeSet<_>>();
    VocabularySummaryRow {
        middle_length,
        scope: scope.to_string(),
        row_count: rows.len(),
        active_pair_share: base_total_rows.map(|total| ratio(active_pair_count, total)),
        noncompact_winner_share: ratio(
            rows.iter().filter(|row| row.noncompact_winner).count(),
            rows.len(),
        ),
        identity_collapse_share: ratio(
            rows.iter().filter(|row| row.identity_collapse).count(),
            rows.len(),
        ),
        mean_nonidentity_transfer_share: mean(
            rows.iter().map(|row| row.nonidentity_transfer_share),
        ),
        mean_boundary_bucket_share: mean(rows.iter().map(|row| row.boundary_bucket_share)),
        mean_churn_share: mean(rows.iter().map(|row| row.churn_share)),
        mean_transfer_bucket_richness: mean(
            rows.iter().map(|row| row.transfer_bucket_richness as f64),
        ),
        stable_zero_led_share: ratio(
            rows.iter()
                .filter(|row| row.signal_source_label == "stable_zero_led")
                .count(),
            rows.len(),
        ),
        boundary_led_share: ratio(
            rows.iter()
                .filter(|row| row.signal_source_label == "boundary_led")
                .count(),
            rows.len(),
        ),
        mixed_or_flat_share: ratio(
            rows.iter()
                .filter(|row| row.signal_source_label == "mixed_or_flat")
                .count(),
            rows.len(),
        ),
        signal_source_diversity: signal_sources.len(),
    }
}

fn build_representative_transfer_rows(
    pair_rows: &[PairTransferRow],
) -> Vec<RepresentativeTransferRow> {
    let mut rows = Vec::new();
    for spec in REPRESENTATIVES {
        for middle_length in [M2, M3] {
            let row = pair_rows
                .iter()
                .find(|row| {
                    row.base == spec.base
                        && row.outer == spec.outer
                        && row.inner == spec.inner
                        && row.middle_length == middle_length
                })
                .expect("representative row should exist");
            rows.push(RepresentativeTransferRow {
                role: spec.role.to_string(),
                base: spec.base,
                pair_label: row.pair_label.clone(),
                middle_length,
                best_k: row.best_k.clone(),
                active: row.active,
                identity_collapse: row.identity_collapse,
                signal_source_label: row.signal_source_label.clone(),
                anomaly_mass_pp: row.anomaly_mass_pp,
                stable_zero_signal_margin_count: row.stable_zero_signal_margin_count,
                stable_zero_prime_delta_count: row.stable_zero_prime_delta_count,
                boundary_prime_delta_count: row.boundary_prime_delta_count,
                nonidentity_transfer_share: row.nonidentity_transfer_share,
                boundary_bucket_share: row.boundary_bucket_share,
                churn_share: row.churn_share,
                transfer_bucket_richness: row.transfer_bucket_richness,
                mechanism_sentence: representative_mechanism_sentence(row).to_string(),
            });
        }
    }
    rows
}

fn build_report_summary(
    pair_rows: &[PairTransferRow],
    vocabulary_rows: &[VocabularySummaryRow],
) -> ReportSummary {
    let m2_all = vocabulary_rows
        .iter()
        .find(|row| row.middle_length == M2 && row.scope == "all")
        .expect("M2 all row should exist");
    let m3_all = vocabulary_rows
        .iter()
        .find(|row| row.middle_length == M3 && row.scope == "all")
        .expect("M3 all row should exist");
    let total_pairs_per_length = pair_rows
        .iter()
        .filter(|row| row.middle_length == M2)
        .count();
    ReportSummary {
        total_pairs_per_length,
        m2_active_pairs: pair_rows
            .iter()
            .filter(|row| row.middle_length == M2 && row.active)
            .count(),
        m3_active_pairs: pair_rows
            .iter()
            .filter(|row| row.middle_length == M3 && row.active)
            .count(),
        m2_noncompact_winner_share: m2_all.noncompact_winner_share,
        m3_noncompact_winner_share: m3_all.noncompact_winner_share,
        m2_identity_collapse_share: m2_all.identity_collapse_share,
        m3_identity_collapse_share: m3_all.identity_collapse_share,
        main_takeaway:
            "M=2 is the last sparse lane where noncompact winners and nonidentity transfer buckets still exist; by M=3 the competitive surface has collapsed into k=(0,0) identity profiles.".to_string(),
    }
}

fn derive_observations(
    pair_rows: &[PairTransferRow],
    pair_transition_rows: &[PairTransitionRow],
    base_rows: &[BaseTransferSummaryRow],
    vocabulary_rows: &[VocabularySummaryRow],
    representative_rows: &[RepresentativeTransferRow],
) -> Vec<String> {
    let m2_all = vocabulary_rows
        .iter()
        .find(|row| row.middle_length == M2 && row.scope == "all")
        .expect("M2 all row should exist");
    let m3_all = vocabulary_rows
        .iter()
        .find(|row| row.middle_length == M3 && row.scope == "all")
        .expect("M3 all row should exist");
    let m2_active = vocabulary_rows
        .iter()
        .find(|row| row.middle_length == M2 && row.scope == "active")
        .expect("M2 active row should exist");
    let m3_active = vocabulary_rows
        .iter()
        .find(|row| row.middle_length == M3 && row.scope == "active")
        .expect("M3 active row should exist");
    let collapsed_pairs = pair_transition_rows
        .iter()
        .filter(|row| row.collapse_class == "active_to_identity")
        .count();
    let m2_base14 = base_rows
        .iter()
        .find(|row| row.base == 14 && row.middle_length == M2)
        .expect("base 14 M2 summary should exist");
    let m2_base34 = base_rows
        .iter()
        .find(|row| row.base == 34 && row.middle_length == M2)
        .expect("base 34 M2 summary should exist");
    let anchor_14 = representative_rows
        .iter()
        .find(|row| row.base == 14 && row.pair_label == "(D,B)" && row.middle_length == M2)
        .expect("base 14 representative should exist");
    let anchor_10 = representative_rows
        .iter()
        .find(|row| row.base == 10 && row.pair_label == "(3,3)" && row.middle_length == M2)
        .expect("base 10 representative should exist");

    vec![
        format!(
            "The decisive global split is exact: `M=2` still has `{}` active pairs and noncompact-winner share `{:.2}%`, while `M=3` has `{}` active pairs and noncompact-winner share `{:.2}%`.",
            pair_rows
                .iter()
                .filter(|row| row.middle_length == M2 && row.active)
                .count(),
            m2_all.noncompact_winner_share * 100.0,
            pair_rows
                .iter()
                .filter(|row| row.middle_length == M3 && row.active)
                .count(),
            m3_all.noncompact_winner_share * 100.0,
        ),
        format!(
            "The transfer grammar itself collapses, not just the anomaly count: mean nonidentity transfer share on the full surface falls from `{:.3}` at `M=2` to `{:.3}` at `M=3`, while identity-collapse share rises from `{:.2}%` to `{:.2}%`.",
            m2_all.mean_nonidentity_transfer_share,
            m3_all.mean_nonidentity_transfer_share,
            m2_all.identity_collapse_share * 100.0,
            m3_all.identity_collapse_share * 100.0,
        ),
        format!(
            "On the active surface the vocabulary is meaningful only at `M=2`: active rows have signal-source diversity `{}` with mean boundary share `{:.3}` and churn share `{:.3}`; by `M=3` the active row count is `{}`.",
            m2_active.signal_source_diversity,
            m2_active.mean_boundary_bucket_share,
            m2_active.mean_churn_share,
            m3_active.row_count,
        ),
        format!(
            "The representative strip shows the species split before collapse: `14 (D,B)` is overlap-led with margin `{}`, `10 (3,3)` is persistence-only with margin `{}`, and all representatives return to `k=(0,0)` identity by `M=3`.",
            anchor_14.stable_zero_signal_margin_count,
            anchor_10.stable_zero_signal_margin_count,
        ),
        format!(
            "This is not just a base-14 story. At `M=2`, base `14` has active share `{:.2}%` and mean active nonidentity transfer `{:.3}`, while base `34` has `{:.2}%` and `{:.3}`; both collapse by `M=3`, but only base `14` carries overlap-positive meaning before the collapse.",
            m2_base14.active_pair_share * 100.0,
            m2_base14.mean_nonidentity_transfer_share_active.unwrap_or(0.0),
            m2_base34.active_pair_share * 100.0,
            m2_base34.mean_nonidentity_transfer_share_active.unwrap_or(0.0),
        ),
        format!(
            "The cleanest explanatory read is that `M=2` is the last length where exact gain/loss/churn vocabulary can separate species before `{}` competitive pairs collapse into identity transfer at `M=3`.",
            collapsed_pairs
        ),
    ]
}

fn render_transfer_vocabulary_collapse(rows: &[VocabularySummaryRow], path: &Path) {
    let root = BitMapBackend::new(path, (1500, 900)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill collapse figure");
    let areas = root.split_evenly((1, 3));

    let m2_all = rows
        .iter()
        .find(|row| row.middle_length == M2 && row.scope == "all")
        .expect("M2 all row should exist");
    let m3_all = rows
        .iter()
        .find(|row| row.middle_length == M3 && row.scope == "all")
        .expect("M3 all row should exist");
    let m2_active = rows
        .iter()
        .find(|row| row.middle_length == M2 && row.scope == "active")
        .expect("M2 active row should exist");
    let m3_active = rows
        .iter()
        .find(|row| row.middle_length == M3 && row.scope == "active")
        .expect("M3 active row should exist");

    let global_metrics = vec![
        (
            "noncompact",
            m2_all.noncompact_winner_share,
            m3_all.noncompact_winner_share,
        ),
        (
            "identity",
            m2_all.identity_collapse_share,
            m3_all.identity_collapse_share,
        ),
        (
            "nonidentity",
            m2_all.mean_nonidentity_transfer_share,
            m3_all.mean_nonidentity_transfer_share,
        ),
    ];
    render_metric_panel(&areas[0], "Global Surface", &global_metrics, 1.0);

    let active_metrics = vec![
        (
            "boundary",
            m2_active.mean_boundary_bucket_share,
            m3_active.mean_boundary_bucket_share,
        ),
        (
            "churn",
            m2_active.mean_churn_share,
            m3_active.mean_churn_share,
        ),
        (
            "richness",
            m2_active.mean_transfer_bucket_richness / 5.0,
            m3_active.mean_transfer_bucket_richness / 5.0,
        ),
    ];
    render_metric_panel(&areas[1], "Active Surface", &active_metrics, 1.0);

    let signal_metrics = vec![
        (
            "stable_zero_led",
            m2_active.stable_zero_led_share,
            m3_active.stable_zero_led_share,
        ),
        (
            "boundary_led",
            m2_active.boundary_led_share,
            m3_active.boundary_led_share,
        ),
        (
            "mixed_or_flat",
            m2_active.mixed_or_flat_share,
            m3_active.mixed_or_flat_share,
        ),
    ];
    render_metric_panel(&areas[2], "Signal Sources", &signal_metrics, 1.0);

    root.present().expect("failed to present collapse figure");
}

fn render_metric_panel(
    area: &DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>,
    title: &str,
    metrics: &[(&str, f64, f64)],
    y_max: f64,
) {
    let mut chart = ChartBuilder::on(area)
        .caption(title, ("sans-serif", 22))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(80)
        .build_cartesian_2d(0f64..y_max, 0usize..metrics.len())
        .expect("failed to build metric panel");

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("share / normalized mean")
        .y_labels(metrics.len())
        .y_label_formatter(&{
            let labels = metrics
                .iter()
                .map(|(label, _, _)| label.to_string())
                .collect::<Vec<_>>();
            move |value| labels.get(*value).cloned().unwrap_or_default()
        })
        .label_style(("sans-serif", 14))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw metric panel mesh");

    for (index, (_, m2, m3)) in metrics.iter().enumerate() {
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(0.0, index), (*m2, index + 1)],
                ShapeStyle::from(&RGBAColor(214, 132, 64, 0.75)).filled(),
            )))
            .expect("failed to draw M2 metric bar");
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(0.0, index), (*m3, index + 1)],
                ShapeStyle::from(&RGBAColor(82, 120, 160, 0.55)).filled(),
            )))
            .expect("failed to draw M3 metric bar");
        chart
            .draw_series(std::iter::once(Text::new(
                format!("M2 {:.2} / M3 {:.2}", m2, m3),
                (m2.max(*m3) + 0.02, index),
                ("sans-serif", 12).into_font().color(&BLACK),
            )))
            .expect("failed to draw metric label");
    }
}

fn render_transfer_meaning_plane(
    pair_rows: &[PairTransferRow],
    representatives: &[RepresentativeTransferRow],
    path: &Path,
) {
    let m2_rows = pair_rows
        .iter()
        .filter(|row| row.middle_length == M2 && row.active)
        .collect::<Vec<_>>();
    let m3_rows = pair_rows
        .iter()
        .filter(|row| row.middle_length == M3)
        .collect::<Vec<_>>();

    let root = BitMapBackend::new(path, (1200, 860)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill meaning plane");
    let mut chart = ChartBuilder::on(&root)
        .caption("Transfer Meaning Plane", ("sans-serif", 28))
        .margin(26)
        .x_label_area_size(60)
        .y_label_area_size(70)
        .build_cartesian_2d(-0.02f64..0.35f64, -8.0f64..8.5f64)
        .expect("failed to build meaning plane");

    chart
        .configure_mesh()
        .x_desc("nonidentity transfer share")
        .y_desc("stable-zero signal margin (count)")
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw meaning plane mesh");

    chart
        .draw_series(m3_rows.iter().map(|row| {
            Circle::new(
                (
                    row.nonidentity_transfer_share,
                    row.stable_zero_signal_margin_count as f64,
                ),
                2,
                ShapeStyle::from(&RGBAColor(120, 120, 120, 0.35)).filled(),
            )
        }))
        .expect("failed to draw M3 rows");

    chart
        .draw_series(m2_rows.iter().map(|row| {
            Circle::new(
                (
                    row.nonidentity_transfer_share,
                    row.stable_zero_signal_margin_count as f64,
                ),
                (5.0 + row.anomaly_mass_pp.max(0.0)) as i32,
                ShapeStyle::from(&RGBAColor(214, 132, 64, 0.75)).filled(),
            )
        }))
        .expect("failed to draw M2 rows");

    for rep in representatives.iter().filter(|row| row.middle_length == M2) {
        let pair_row = pair_rows
            .iter()
            .find(|row| {
                row.base == rep.base
                    && row.pair_label == rep.pair_label
                    && row.middle_length == rep.middle_length
            })
            .expect("representative point should exist");
        chart
            .draw_series(std::iter::once(Text::new(
                format!("{} {}", rep.pair_label, rep.role),
                (
                    pair_row.nonidentity_transfer_share + 0.01,
                    pair_row.stable_zero_signal_margin_count as f64,
                ),
                ("sans-serif", 14).into_font().color(&BLACK),
            )))
            .expect("failed to draw representative label");
    }

    root.present().expect("failed to present meaning plane");
}

fn render_representative_transfer_strip(rows: &[RepresentativeTransferRow], path: &Path) {
    let labels = rows
        .iter()
        .map(|row| format!("M{} {} {}", row.middle_length, row.pair_label, row.role))
        .collect::<Vec<_>>();
    let x_max = rows
        .iter()
        .map(|row| {
            row.anomaly_mass_pp
                .max(row.nonidentity_transfer_share * 100.0)
        })
        .fold(0.0, f64::max)
        + 5.0;

    let root = BitMapBackend::new(path, (1360, 900)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill representative strip");
    let mut chart = ChartBuilder::on(&root)
        .caption("Representative Transfer Strip", ("sans-serif", 28))
        .margin(26)
        .x_label_area_size(60)
        .y_label_area_size(260)
        .build_cartesian_2d(0.0f64..x_max, 0usize..rows.len())
        .expect("failed to build representative strip");

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_desc("anomaly mass (pp) / nonidentity share × 100")
        .y_labels(rows.len())
        .y_label_formatter(&{ move |value| labels.get(*value).cloned().unwrap_or_default() })
        .label_style(("sans-serif", 14))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw representative strip mesh");

    for (index, row) in rows.iter().enumerate() {
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(0.0, index), (row.anomaly_mass_pp, index + 1)],
                ShapeStyle::from(&RGBAColor(214, 132, 64, 0.8)).filled(),
            )))
            .expect("failed to draw anomaly bar");
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [
                    (0.0, index),
                    (row.nonidentity_transfer_share * 100.0, index + 1),
                ],
                ShapeStyle::from(&RGBAColor(82, 120, 160, 0.55)).filled(),
            )))
            .expect("failed to draw nonidentity bar");
        chart
            .draw_series(std::iter::once(Text::new(
                format!(
                    "{} | margin {} | {}",
                    row.best_k, row.stable_zero_signal_margin_count, row.signal_source_label
                ),
                (
                    row.anomaly_mass_pp
                        .max(row.nonidentity_transfer_share * 100.0)
                        + 0.5,
                    index,
                ),
                ("sans-serif", 12).into_font().color(&BLACK),
            )))
            .expect("failed to draw representative annotation");
    }

    root.present()
        .expect("failed to present representative strip");
}

fn render_markdown(bundle: &ReportBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# M2 vs M3 Transfer Collapse Report\n\n");
    markdown.push_str("_Generated from `examples/m2_m3_transfer_collapse_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Output directory: `{}`\n- Bases: `{}`\n- Middle lengths: `{}`\n- Pair catalog: `{}`\n\n",
        bundle.settings.out_dir,
        bundle
            .settings
            .bases
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        bundle
            .settings
            .middle_lengths
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        bundle.settings.pair_catalog_mode,
    ));

    markdown.push_str("## Vocabulary Summary\n\n");
    markdown.push_str("| M | Scope | Rows | Active share | Noncompact share | Identity collapse | Mean nonidentity | Mean richness | Signal diversity |\n");
    markdown.push_str("|---|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in &bundle.vocabulary_summary_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {:.2}% | {:.2}% | {:.3} | {:.2} | {} |\n",
            row.middle_length,
            row.scope,
            row.row_count,
            format_option_share(row.active_pair_share),
            row.noncompact_winner_share * 100.0,
            row.identity_collapse_share * 100.0,
            row.mean_nonidentity_transfer_share,
            row.mean_transfer_bucket_richness,
            row.signal_source_diversity,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Base Summary\n\n");
    markdown.push_str("| Base | M | Active share | Noncompact share | Identity collapse | Active mean nonidentity | Leading pair |\n");
    markdown.push_str("|---|---:|---:|---:|---:|---:|---|\n");
    for row in &bundle.base_transfer_summary_rows {
        markdown.push_str(&format!(
            "| {} | {} | {:.2}% | {:.2}% | {:.2}% | {} | {} |\n",
            row.base,
            row.middle_length,
            row.active_pair_share * 100.0,
            row.noncompact_winner_share * 100.0,
            row.identity_collapse_share * 100.0,
            format_option_float(row.mean_nonidentity_transfer_share_active),
            row.leading_pair,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Representative Strip\n\n");
    markdown.push_str("| Role | Base | Pair | M | Best k | Active | Identity | Signal | Anomaly | Margin | Nonidentity |\n");
    markdown.push_str("|---|---:|---|---:|---|---|---|---|---:|---:|---:|\n");
    for row in &bundle.representative_transfer_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {:.2}pp | {} | {:.3} |\n",
            row.role,
            row.base,
            row.pair_label,
            row.middle_length,
            row.best_k,
            yes_no(row.active),
            yes_no(row.identity_collapse),
            row.signal_source_label,
            row.anomaly_mass_pp,
            row.stable_zero_signal_margin_count,
            row.nonidentity_transfer_share,
        ));
    }
    markdown.push('\n');

    for image in &bundle.image_artifact_rows {
        markdown.push_str(&format!("![{}]({})\n\n", image.label, image.path));
    }

    markdown.push_str("## Observations\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {}\n", observation));
    }

    markdown
}

fn representative_role(base: u32, outer: u32, inner: u32) -> Option<&'static str> {
    REPRESENTATIVES
        .iter()
        .find(|spec| spec.base == base && spec.outer == outer && spec.inner == inner)
        .map(|spec| spec.role)
}

fn representative_mechanism_sentence(row: &PairTransferRow) -> &'static str {
    if row.middle_length == M3 {
        "collapsed_to_identity"
    } else if row.active && row.signal_source_label == "stable_zero_led" {
        "overlap_meaningful"
    } else if row.active && row.signal_source_label == "boundary_led" {
        "boundary_meaningful"
    } else {
        "no_competitive_signal"
    }
}

fn collapse_class(m2: &PairTransferRow, m3: &PairTransferRow) -> &'static str {
    match (m2.active, m3.active, m3.identity_collapse) {
        (true, false, true) => "active_to_identity",
        (true, false, false) => "active_to_nonidentity_dead",
        (false, false, true) => "inactive_to_identity",
        (false, false, false) => "inactive_nonidentity",
        _ => "persistent_active",
    }
}

fn mean(values: impl Iterator<Item = f64>) -> f64 {
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

fn mean_option(values: impl Iterator<Item = f64>) -> Option<f64> {
    let mut total = 0.0;
    let mut count = 0usize;
    for value in values {
        total += value;
        count += 1;
    }
    (count > 0).then_some(total / count as f64)
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn format_share(value: f64) -> String {
    format!("{:.2}%", value * 100.0)
}

fn format_option_share(value: Option<f64>) -> String {
    value.map(format_share).unwrap_or_else(|| "—".to_string())
}

fn format_option_float(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.3}"))
        .unwrap_or_else(|| "—".to_string())
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PAIRS: &[(u32, u32, u32)] = &[
        (10, 3, 3),
        (14, 13, 11),
        (22, 17, 19),
        (26, 23, 23),
        (34, 25, 9),
    ];

    #[test]
    fn vocabulary_summary_shows_m2_signal_and_m3_collapse() {
        let pair_rows = build_pair_transfer_rows_for_pairs(TEST_PAIRS);
        let summary_rows = build_vocabulary_summary_rows(&pair_rows);
        let m2_all = summary_rows
            .iter()
            .find(|row| row.middle_length == M2 && row.scope == "all")
            .expect("M2 all summary should exist");
        let m3_all = summary_rows
            .iter()
            .find(|row| row.middle_length == M3 && row.scope == "all")
            .expect("M3 all summary should exist");

        assert!(m2_all.noncompact_winner_share > 0.0);
        assert_eq!(m3_all.noncompact_winner_share, 0.0);
        assert!(m2_all.identity_collapse_share < 1.0);
        assert_eq!(m3_all.identity_collapse_share, 1.0);
    }

    #[test]
    fn representatives_collapse_to_k00_identity_by_m3() {
        let pair_rows = build_pair_transfer_rows_for_pairs(TEST_PAIRS);
        let representative_rows = build_representative_transfer_rows(&pair_rows);
        for row in representative_rows
            .iter()
            .filter(|row| row.middle_length == M3)
        {
            assert_eq!(row.best_k, "k=(0,0)");
            assert!(!row.active);
            assert!(row.identity_collapse);
        }
    }

    #[test]
    fn base14_and_base10_representatives_keep_expected_m2_signal_split() {
        let pair_rows = build_pair_transfer_rows_for_pairs(TEST_PAIRS);
        let base14 = pair_rows
            .iter()
            .find(|row| row.base == 14 && row.middle_length == M2 && row.pair_label == "(D,B)")
            .expect("base 14 representative should exist");
        let base10 = pair_rows
            .iter()
            .find(|row| row.base == 10 && row.middle_length == M2 && row.pair_label == "(3,3)")
            .expect("base 10 representative should exist");

        assert_eq!(base14.signal_source_label, "stable_zero_led");
        assert_eq!(base10.signal_source_label, "boundary_led");
        assert!(base14.stable_zero_signal_margin_count > 0);
        assert!(base10.stable_zero_signal_margin_count <= 0);
    }
}
