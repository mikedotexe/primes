//! Cross-base hinge mask atlas for the `B = 2p` lane.
//!
//! This report treats the hinge as a mask-spectrum problem: why does the
//! overlap of persistence and shared-yield core appear in base 14 while bases
//! 10, 22, and 26 split into weaker sub-regimes?
//!
//! The exact object is the transfer profile from `k=(0,0)` to the best `M=2`
//! lane for each active ordered pair.

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        digit_symbol, evaluate_pair_row, ordered_unit_pairs, parse_k_label,
        scan_k_config_mask_profile, scan_k_config_transfer_profile, DEFAULT_BOUNDED_K_GRID,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
};

const MAIN_BASES: &[u32] = &[10, 14, 22, 26];
const APPENDIX_BASES: &[u32] = &[6];
const ALL_BASES: &[u32] = &[10, 14, 22, 26, 6];
const M1: usize = 1;
const M2: usize = 2;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_two_p_hinge_mask";
const REPORT_EXPORT_VERSION: u32 = 1;
const TRANSFER_BUCKETS: &[&str] = &[
    "stable_zero",
    "gain_zero",
    "loss_zero",
    "stable_nonzero",
    "nonzero_churn",
];

const CATEGORY_PERSISTENT_CORE: &str = "persistent_core";
const CATEGORY_PERSISTENCE_ONLY: &str = "persistence_only";
const CATEGORY_CORE_ONLY: &str = "core_only";
const CATEGORY_ACTIVE_NEITHER: &str = "active_neither";
const CATEGORIES: &[&str] = &[
    CATEGORY_PERSISTENT_CORE,
    CATEGORY_PERSISTENCE_ONLY,
    CATEGORY_CORE_ONLY,
    CATEGORY_ACTIVE_NEITHER,
];

const REPRESENTATIVES: &[(u32, u32, u32, &str)] = &[
    (14, 13, 11, CATEGORY_PERSISTENT_CORE),
    (10, 3, 3, CATEGORY_PERSISTENCE_ONLY),
    (26, 23, 23, CATEGORY_CORE_ONLY),
    (22, 17, 19, CATEGORY_ACTIVE_NEITHER),
];

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    main_bases: Vec<u32>,
    appendix_bases: Vec<u32>,
    middle_length: usize,
    baseline_k: String,
}

#[derive(Debug, Clone, Serialize)]
struct PairMaskRow {
    scope: String,
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    hinge_category: String,
    best_k_m2: String,
    anomaly_m1_pp: f64,
    anomaly_m2_pp: f64,
    m2_persistent: bool,
    m2_emergent: bool,
    mask_stability_share: f64,
    admissible_overlap_jaccard: f64,
    zero_mask_net_transfer_pp: f64,
    stable_zero_prime_delta_count: isize,
    stable_zero_prime_delta_pp: f64,
    boundary_prime_delta_pp: f64,
    stable_zero_signal_margin_pp: f64,
    stable_zero_support_ratio: f64,
    nonzero_churn_share: f64,
    signal_source_label: String,
    dominant_count_transition_label: String,
    dominant_count_transition_share: f64,
    stable_zero_count: usize,
    gain_zero_count: usize,
    loss_zero_count: usize,
    stable_nonzero_count: usize,
    nonzero_churn_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct BaseMaskSummaryRow {
    scope: String,
    base: u32,
    active_pair_count: usize,
    persistent_pair_count: usize,
    persistent_core_pairs: usize,
    persistence_only_pairs: usize,
    core_only_pairs: usize,
    active_neither_pairs: usize,
    persistent_stable_zero_led_pairs: usize,
    persistent_stable_zero_led_share: Option<f64>,
    mean_persistent_stable_zero_support_ratio: Option<f64>,
    mean_persistent_signal_margin_pp: Option<f64>,
    persistent_signal_label: String,
    mean_mask_stability_share: f64,
    mean_admissible_overlap_jaccard: f64,
    mean_zero_mask_net_transfer_pp: f64,
    mean_stable_zero_prime_delta_pp: f64,
    mean_boundary_prime_delta_pp: f64,
    mean_nonzero_churn_share: f64,
    signal_source_label: String,
    dominant_count_transition_label: String,
    dominant_count_transition_share: f64,
    base_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct CategoryMaskSummaryRow {
    scope: String,
    hinge_category: String,
    active_pair_count: usize,
    bases: String,
    mean_mask_stability_share: f64,
    mean_admissible_overlap_jaccard: f64,
    mean_zero_mask_net_transfer_pp: f64,
    mean_stable_zero_prime_delta_pp: f64,
    mean_boundary_prime_delta_pp: f64,
    mean_nonzero_churn_share: f64,
    signal_source_label: String,
    dominant_count_transition_label: String,
    dominant_count_transition_share: f64,
}

#[derive(Debug, Clone, Serialize)]
struct RepresentativeTransitionRow {
    base: u32,
    pair_label: String,
    hinge_category: String,
    best_k_m2: String,
    transfer_bucket: String,
    count: usize,
    share: f64,
    prime_delta_count: isize,
    prime_delta_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    active_pair_count_main: usize,
    active_pair_count_appendix: usize,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    pair_mask_rows: Vec<PairMaskRow>,
    base_mask_summary_rows: Vec<BaseMaskSummaryRow>,
    category_mask_summary_rows: Vec<CategoryMaskSummaryRow>,
    representative_transition_rows: Vec<RepresentativeTransitionRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

#[derive(Debug, Clone)]
struct PairAnalysis {
    pair_row: PairMaskRow,
    representative_rows: Vec<RepresentativeTransitionRow>,
}

#[derive(Debug, Clone, Default)]
struct TransferBucketStats {
    count: usize,
    prime_delta_count: isize,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        main_bases: MAIN_BASES.to_vec(),
        appendix_bases: APPENDIX_BASES.to_vec(),
        middle_length: M2,
        baseline_k: "k=(0,0)".to_string(),
    };

    let analyses = build_pair_analyses();
    let pair_mask_rows = analyses
        .iter()
        .map(|analysis| analysis.pair_row.clone())
        .collect::<Vec<_>>();
    let representative_transition_rows = analyses
        .iter()
        .flat_map(|analysis| analysis.representative_rows.clone())
        .collect::<Vec<_>>();
    let base_mask_summary_rows = build_base_mask_summary_rows(&pair_mask_rows);
    let category_mask_summary_rows = build_category_mask_summary_rows(&pair_mask_rows);

    let hinge_mask_plane_path = options.out_dir.join("hinge_mask_plane.png");
    render_hinge_mask_plane(&pair_mask_rows, &hinge_mask_plane_path);
    let representative_bars_path = options.out_dir.join("representative_transition_bars.png");
    render_representative_transition_bars(
        &representative_transition_rows,
        &representative_bars_path,
    );
    let persistent_support_ratio_path = options.out_dir.join("persistent_support_ratio_bars.png");
    render_persistent_support_ratio_bars(&pair_mask_rows, &persistent_support_ratio_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "hinge_mask_plane".to_string(),
            label: "Hinge mask plane".to_string(),
            path: hinge_mask_plane_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "representative_transition_bars".to_string(),
            label: "Representative transition bars".to_string(),
            path: representative_bars_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "persistent_support_ratio_bars".to_string(),
            label: "Persistent support ratio bars".to_string(),
            path: persistent_support_ratio_path.display().to_string(),
        },
    ];

    let report_summary = build_report_summary(&pair_mask_rows, &base_mask_summary_rows);
    let observations = derive_observations(&base_mask_summary_rows, &category_mask_summary_rows);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        pair_mask_rows: pair_mask_rows.clone(),
        base_mask_summary_rows: base_mask_summary_rows.clone(),
        category_mask_summary_rows: category_mask_summary_rows.clone(),
        representative_transition_rows: representative_transition_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(options.out_dir.join("pair_mask_rows.csv"), &pair_mask_rows)
        .expect("failed to write pair_mask_rows.csv");
    write_csv_rows(
        options.out_dir.join("base_mask_summary_rows.csv"),
        &base_mask_summary_rows,
    )
    .expect("failed to write base_mask_summary_rows.csv");
    write_csv_rows(
        options.out_dir.join("category_mask_summary_rows.csv"),
        &category_mask_summary_rows,
    )
    .expect("failed to write category_mask_summary_rows.csv");
    write_csv_rows(
        options.out_dir.join("representative_transition_rows.csv"),
        &representative_transition_rows,
    )
    .expect("failed to write representative_transition_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");
    write_text_file(options.out_dir.join("report.md"), &render_markdown(&bundle))
        .expect("failed to write report.md");

    println!("2p hinge mask report");
    println!("  output dir: {}", options.out_dir.display());
    for row in &base_mask_summary_rows {
        println!(
            "  base {:>2} | scope {:<8} | label {:<16} | source {:<15} | persistent_source {:<18} | stable_zero_delta {:>6.2}pp | boundary_delta {:>6.2}pp",
            row.base,
            row.scope,
            row.base_label,
            row.signal_source_label,
            row.persistent_signal_label,
            row.mean_stable_zero_prime_delta_pp,
            row.mean_boundary_prime_delta_pp,
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
    println!("  cargo run --release --example two_p_hinge_mask_report -- [options]");
    println!();
    println!("Options:");
    println!("  --out-dir <dir>   Output directory (default: {DEFAULT_OUT_DIR})");
    println!("  -h, --help        Show this help message");
}

fn build_pair_analyses() -> Vec<PairAnalysis> {
    ALL_BASES
        .par_iter()
        .copied()
        .flat_map(|base| {
            ordered_unit_pairs(base)
                .into_par_iter()
                .filter_map(move |(outer, inner)| analyze_active_pair(base, outer, inner))
        })
        .collect()
}

fn analyze_active_pair(base: u32, outer: u32, inner: u32) -> Option<PairAnalysis> {
    let row_m1 = evaluate_pair_row(base, M1, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let row_m2 = evaluate_pair_row(base, M2, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let anomaly_m1 = anomaly_mass(&row_m1);
    let anomaly_m2 = anomaly_mass(&row_m2);
    if anomaly_m2 <= 0.0 {
        return None;
    }

    let best_k_m2 = parse_k_label(&row_m2.best_k);
    let from_profile = scan_k_config_mask_profile(base, M2, outer, inner, (0, 0));
    let to_profile = if best_k_m2 == (0, 0) {
        from_profile.clone()
    } else {
        scan_k_config_mask_profile(base, M2, outer, inner, best_k_m2)
    };
    let transfer_profile =
        scan_k_config_transfer_profile(base, M2, outer, inner, (0, 0), best_k_m2);

    let mut bucket_stats = TRANSFER_BUCKETS
        .iter()
        .map(|&bucket| (bucket.to_string(), TransferBucketStats::default()))
        .collect::<BTreeMap<_, _>>();
    for row in &transfer_profile.candidate_rows {
        let entry = bucket_stats
            .get_mut(&row.transfer_bucket)
            .expect("transfer bucket should be initialized");
        entry.count += 1;
        entry.prime_delta_count += row.prime_to as isize - row.prime_from as isize;
    }

    let total = transfer_profile.candidates_per_config as f64;
    let stable_zero_count = bucket_count(&bucket_stats, "stable_zero");
    let gain_zero_count = bucket_count(&bucket_stats, "gain_zero");
    let loss_zero_count = bucket_count(&bucket_stats, "loss_zero");
    let stable_nonzero_count = bucket_count(&bucket_stats, "stable_nonzero");
    let nonzero_churn_count = bucket_count(&bucket_stats, "nonzero_churn");
    let same_mask_count = stable_zero_count + stable_nonzero_count;

    let stable_zero_prime_delta_count = bucket_prime_delta(&bucket_stats, "stable_zero");
    let boundary_prime_delta_count = bucket_prime_delta(&bucket_stats, "gain_zero")
        + bucket_prime_delta(&bucket_stats, "loss_zero");
    let zero_union_count = stable_zero_count + gain_zero_count + loss_zero_count;

    let admissible_share_from = ratio(
        from_profile.admissible_count,
        from_profile.candidates_per_config,
    );
    let admissible_share_to = ratio(
        to_profile.admissible_count,
        to_profile.candidates_per_config,
    );
    let prime_yield_from = ratio(from_profile.prime_hits, from_profile.admissible_count);
    let prime_yield_to = ratio(to_profile.prime_hits, to_profile.admissible_count);
    let admissible_set_effect_pp =
        (admissible_share_to - admissible_share_from) * prime_yield_from * 100.0;
    let prime_yield_effect_pp = admissible_share_to * (prime_yield_to - prime_yield_from) * 100.0;

    let m2_persistent = anomaly_m1 > 0.0;
    let shared_yield_core = stable_zero_prime_delta_count > boundary_prime_delta_count.abs()
        && stable_zero_prime_delta_count > 0
        && prime_yield_effect_pp.abs() > admissible_set_effect_pp.abs();
    let hinge_category = hinge_category(m2_persistent, shared_yield_core).to_string();
    let signal_source_label = signal_source_label(
        stable_zero_prime_delta_count as f64 * 100.0 / total,
        boundary_prime_delta_count as f64 * 100.0 / total,
    )
    .to_string();
    let dominant_count_transition_label =
        dominant_count_transition_label(&bucket_stats).to_string();
    let dominant_count_transition_share =
        bucket_count(&bucket_stats, &dominant_count_transition_label) as f64 / total;

    let pair_label = format!("({},{})", digit_symbol(outer), digit_symbol(inner));
    let pair_row = PairMaskRow {
        scope: scope_label(base).to_string(),
        base,
        outer,
        inner,
        pair_label: pair_label.clone(),
        hinge_category: hinge_category.clone(),
        best_k_m2: row_m2.best_k.clone(),
        anomaly_m1_pp: anomaly_m1,
        anomaly_m2_pp: anomaly_m2,
        m2_persistent,
        m2_emergent: !m2_persistent,
        mask_stability_share: same_mask_count as f64 / total,
        admissible_overlap_jaccard: ratio(stable_zero_count, zero_union_count),
        zero_mask_net_transfer_pp: (gain_zero_count as f64 - loss_zero_count as f64) * 100.0
            / total,
        stable_zero_prime_delta_count,
        stable_zero_prime_delta_pp: stable_zero_prime_delta_count as f64 * 100.0 / total,
        boundary_prime_delta_pp: boundary_prime_delta_count as f64 * 100.0 / total,
        stable_zero_signal_margin_pp: stable_zero_prime_delta_count as f64 * 100.0 / total
            - (boundary_prime_delta_count as f64 * 100.0 / total).abs(),
        stable_zero_support_ratio: if anomaly_m2 > 0.0 {
            (stable_zero_prime_delta_count as f64 * 100.0 / total) / anomaly_m2
        } else {
            0.0
        },
        nonzero_churn_share: nonzero_churn_count as f64 / total,
        signal_source_label,
        dominant_count_transition_label: dominant_count_transition_label.clone(),
        dominant_count_transition_share,
        stable_zero_count,
        gain_zero_count,
        loss_zero_count,
        stable_nonzero_count,
        nonzero_churn_count,
    };

    let representative_rows = if is_representative(base, outer, inner) {
        TRANSFER_BUCKETS
            .iter()
            .map(|&bucket| RepresentativeTransitionRow {
                base,
                pair_label: pair_label.clone(),
                hinge_category: hinge_category.clone(),
                best_k_m2: row_m2.best_k.clone(),
                transfer_bucket: bucket.to_string(),
                count: bucket_count(&bucket_stats, bucket),
                share: bucket_count(&bucket_stats, bucket) as f64 / total,
                prime_delta_count: bucket_prime_delta(&bucket_stats, bucket),
                prime_delta_pp: bucket_prime_delta(&bucket_stats, bucket) as f64 * 100.0 / total,
            })
            .collect()
    } else {
        Vec::new()
    };

    Some(PairAnalysis {
        pair_row,
        representative_rows,
    })
}

fn build_base_mask_summary_rows(rows: &[PairMaskRow]) -> Vec<BaseMaskSummaryRow> {
    let mut by_base = BTreeMap::<u32, Vec<&PairMaskRow>>::new();
    for row in rows {
        by_base.entry(row.base).or_default().push(row);
    }

    ALL_BASES
        .iter()
        .copied()
        .filter_map(|base| {
            let group = by_base.get(&base)?;
            let persistent_pair_count = group.iter().filter(|row| row.m2_persistent).count();
            let persistent_core_pairs = group
                .iter()
                .filter(|row| row.hinge_category == CATEGORY_PERSISTENT_CORE)
                .count();
            let persistence_only_pairs = group
                .iter()
                .filter(|row| row.hinge_category == CATEGORY_PERSISTENCE_ONLY)
                .count();
            let core_only_pairs = group
                .iter()
                .filter(|row| row.hinge_category == CATEGORY_CORE_ONLY)
                .count();
            let active_neither_pairs = group
                .iter()
                .filter(|row| row.hinge_category == CATEGORY_ACTIVE_NEITHER)
                .count();
            let persistent_stable_zero_led_pairs = group
                .iter()
                .filter(|row| row.m2_persistent && row.signal_source_label == "stable_zero_led")
                .count();
            let base_label = if APPENDIX_BASES.contains(&base) {
                "tiny_witness"
            } else if persistent_core_pairs > 0 {
                "hinge_bridge"
            } else if persistence_only_pairs > 0 && core_only_pairs == 0 {
                "persistence_only"
            } else if core_only_pairs > 0 && persistence_only_pairs == 0 {
                "core_only"
            } else {
                "active_neither"
            };

            Some(BaseMaskSummaryRow {
                scope: scope_label(base).to_string(),
                base,
                active_pair_count: group.len(),
                persistent_pair_count,
                persistent_core_pairs,
                persistence_only_pairs,
                core_only_pairs,
                active_neither_pairs,
                persistent_stable_zero_led_pairs,
                persistent_stable_zero_led_share: ratio_option(
                    persistent_stable_zero_led_pairs,
                    persistent_pair_count,
                ),
                mean_persistent_stable_zero_support_ratio: mean_option(
                    group
                        .iter()
                        .filter(|row| row.m2_persistent)
                        .map(|row| row.stable_zero_support_ratio),
                ),
                mean_persistent_signal_margin_pp: mean_option(
                    group
                        .iter()
                        .filter(|row| row.m2_persistent)
                        .map(|row| row.stable_zero_signal_margin_pp),
                ),
                persistent_signal_label: persistent_signal_label(group.iter().copied()),
                mean_mask_stability_share: mean(group.iter().map(|row| row.mask_stability_share)),
                mean_admissible_overlap_jaccard: mean(
                    group.iter().map(|row| row.admissible_overlap_jaccard),
                ),
                mean_zero_mask_net_transfer_pp: mean(
                    group.iter().map(|row| row.zero_mask_net_transfer_pp),
                ),
                mean_stable_zero_prime_delta_pp: mean(
                    group.iter().map(|row| row.stable_zero_prime_delta_pp),
                ),
                mean_boundary_prime_delta_pp: mean(
                    group.iter().map(|row| row.boundary_prime_delta_pp),
                ),
                mean_nonzero_churn_share: mean(group.iter().map(|row| row.nonzero_churn_share)),
                signal_source_label: signal_source_label(
                    mean(group.iter().map(|row| row.stable_zero_prime_delta_pp)),
                    mean(group.iter().map(|row| row.boundary_prime_delta_pp)),
                )
                .to_string(),
                dominant_count_transition_label: most_common_label(
                    group
                        .iter()
                        .map(|row| row.dominant_count_transition_label.clone()),
                ),
                dominant_count_transition_share: dominant_pair_label_share(
                    group
                        .iter()
                        .map(|row| row.dominant_count_transition_label.clone()),
                    group.len(),
                ),
                base_label: base_label.to_string(),
            })
        })
        .collect()
}

fn build_category_mask_summary_rows(rows: &[PairMaskRow]) -> Vec<CategoryMaskSummaryRow> {
    let mut grouped = BTreeMap::<(String, String), Vec<&PairMaskRow>>::new();
    for row in rows {
        grouped
            .entry((row.scope.clone(), row.hinge_category.clone()))
            .or_default()
            .push(row);
    }

    let mut summary_rows = Vec::new();
    for scope in ["main", "appendix"] {
        for &category in CATEGORIES {
            let Some(group) = grouped.get(&(scope.to_string(), category.to_string())) else {
                continue;
            };
            let mut bases = group.iter().map(|row| row.base).collect::<Vec<_>>();
            bases.sort_unstable();
            bases.dedup();
            summary_rows.push(CategoryMaskSummaryRow {
                scope: scope.to_string(),
                hinge_category: category.to_string(),
                active_pair_count: group.len(),
                bases: bases
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(", "),
                mean_mask_stability_share: mean(group.iter().map(|row| row.mask_stability_share)),
                mean_admissible_overlap_jaccard: mean(
                    group.iter().map(|row| row.admissible_overlap_jaccard),
                ),
                mean_zero_mask_net_transfer_pp: mean(
                    group.iter().map(|row| row.zero_mask_net_transfer_pp),
                ),
                mean_stable_zero_prime_delta_pp: mean(
                    group.iter().map(|row| row.stable_zero_prime_delta_pp),
                ),
                mean_boundary_prime_delta_pp: mean(
                    group.iter().map(|row| row.boundary_prime_delta_pp),
                ),
                mean_nonzero_churn_share: mean(group.iter().map(|row| row.nonzero_churn_share)),
                signal_source_label: signal_source_label(
                    mean(group.iter().map(|row| row.stable_zero_prime_delta_pp)),
                    mean(group.iter().map(|row| row.boundary_prime_delta_pp)),
                )
                .to_string(),
                dominant_count_transition_label: most_common_label(
                    group
                        .iter()
                        .map(|row| row.dominant_count_transition_label.clone()),
                ),
                dominant_count_transition_share: dominant_pair_label_share(
                    group
                        .iter()
                        .map(|row| row.dominant_count_transition_label.clone()),
                    group.len(),
                ),
            });
        }
    }
    summary_rows
}

fn render_hinge_mask_plane(rows: &[PairMaskRow], path: &Path) {
    let main_rows = rows
        .iter()
        .filter(|row| row.scope == "main")
        .collect::<Vec<_>>();
    let min_y = main_rows
        .iter()
        .map(|row| row.stable_zero_prime_delta_pp)
        .fold(0.0_f64, f64::min)
        .min(-0.2);
    let max_y = main_rows
        .iter()
        .map(|row| row.stable_zero_prime_delta_pp)
        .fold(0.0_f64, f64::max)
        .max(0.2);

    let root = BitMapBackend::new(path, (1180, 760)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill hinge mask plane canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Cross-Base Hinge Mask Plane  (x = admissible overlap, y = stable-zero prime delta)",
            ("sans-serif", 26),
        )
        .margin(28)
        .x_label_area_size(64)
        .y_label_area_size(82)
        .build_cartesian_2d(0.0f64..1.02f64, (min_y - 0.2)..(max_y + 0.2))
        .expect("failed to build hinge mask plane");

    chart
        .configure_mesh()
        .x_desc("admissible overlap jaccard")
        .y_desc("stable_zero prime delta (pp)")
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(222, 216, 207))
        .draw()
        .expect("failed to draw hinge mask plane mesh");

    for row in &main_rows {
        chart
            .draw_series(std::iter::once(Circle::new(
                (
                    row.admissible_overlap_jaccard,
                    row.stable_zero_prime_delta_pp,
                ),
                7,
                ShapeStyle::from(&hinge_color(&row.hinge_category)).filled(),
            )))
            .expect("failed to draw hinge mask point");

        if is_representative(row.base, row.outer, row.inner) {
            chart
                .draw_series(std::iter::once(Text::new(
                    format!("{} {}", row.base, row.pair_label),
                    (
                        row.admissible_overlap_jaccard + 0.02,
                        row.stable_zero_prime_delta_pp + 0.08,
                    ),
                    ("sans-serif", 15).into_font().color(&BLACK),
                )))
                .expect("failed to draw hinge mask representative label");
        }
    }

    root.present().expect("failed to present hinge mask plane");
}

fn render_representative_transition_bars(rows: &[RepresentativeTransitionRow], path: &Path) {
    let mut pair_keys = REPRESENTATIVES
        .iter()
        .map(|&(base, outer, inner, _)| {
            (
                base,
                format!("({},{})", digit_symbol(outer), digit_symbol(inner)),
            )
        })
        .collect::<Vec<_>>();
    pair_keys.retain(|(base, pair_label)| {
        rows.iter()
            .any(|row| row.base == *base && row.pair_label == *pair_label)
    });

    let min_y = rows
        .iter()
        .map(|row| row.prime_delta_pp)
        .fold(0.0_f64, f64::min)
        .min(-1.0);
    let max_y = rows
        .iter()
        .map(|row| row.prime_delta_pp)
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let root = BitMapBackend::new(path, (1280, 760)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill representative bars canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Representative Transition Prime Delta by Bucket",
            ("sans-serif", 26),
        )
        .margin(28)
        .x_label_area_size(92)
        .y_label_area_size(82)
        .build_cartesian_2d(0.0f64..pair_keys.len() as f64, (min_y - 0.4)..(max_y + 0.4))
        .expect("failed to build representative bar chart");

    let pair_labels = pair_keys
        .iter()
        .map(|(base, pair_label)| format!("{base} {pair_label}"))
        .collect::<Vec<_>>();
    chart
        .configure_mesh()
        .x_desc("representative pairs")
        .y_desc("prime delta by transfer bucket (pp)")
        .x_labels(pair_labels.len())
        .x_label_formatter(&move |value| {
            let index = value.floor() as usize;
            if index < pair_labels.len() {
                pair_labels[index].clone()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 15))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(222, 216, 207))
        .draw()
        .expect("failed to draw representative bar mesh");

    let bar_width = 0.15f64;
    let offsets = [-0.34, -0.17, 0.0, 0.17, 0.34];
    for (pair_index, (base, pair_label)) in pair_keys.iter().enumerate() {
        for (bucket_index, bucket) in TRANSFER_BUCKETS.iter().enumerate() {
            let row = rows
                .iter()
                .find(|row| {
                    row.base == *base
                        && row.pair_label == *pair_label
                        && row.transfer_bucket == *bucket
                })
                .expect("representative transfer row should exist");
            let x = pair_index as f64 + 0.5;
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [
                        (x + offsets[bucket_index] - bar_width / 2.0, 0.0),
                        (
                            x + offsets[bucket_index] + bar_width / 2.0,
                            row.prime_delta_pp,
                        ),
                    ],
                    ShapeStyle::from(&transfer_bucket_color(bucket)).filled(),
                )))
                .expect("failed to draw representative transition bar");
        }
    }

    root.present()
        .expect("failed to present representative bar chart");
}

fn render_persistent_support_ratio_bars(rows: &[PairMaskRow], path: &Path) {
    let persistent_rows = rows
        .iter()
        .filter(|row| row.m2_persistent)
        .collect::<Vec<_>>();
    if persistent_rows.is_empty() {
        return;
    }

    let labels = persistent_rows
        .iter()
        .map(|row| format!("{} {}", row.base, row.pair_label))
        .collect::<Vec<_>>();
    let max_y = persistent_rows
        .iter()
        .map(|row| row.stable_zero_support_ratio)
        .fold(1.0_f64, f64::max)
        .max(1.0);
    let min_y = persistent_rows
        .iter()
        .map(|row| row.stable_zero_support_ratio)
        .fold(0.0_f64, f64::min)
        .min(-0.1);

    let root = BitMapBackend::new(path, (1080, 620)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill persistent support ratio canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Persistent Pair Stable-Zero Support Ratio",
            ("sans-serif", 26),
        )
        .margin(28)
        .x_label_area_size(72)
        .y_label_area_size(82)
        .build_cartesian_2d(
            0.0f64..persistent_rows.len() as f64,
            (min_y - 0.1)..(max_y + 0.3),
        )
        .expect("failed to build persistent support ratio chart");

    chart
        .configure_mesh()
        .x_desc("persistent pairs")
        .y_desc("stable-zero support ratio")
        .x_labels(labels.len())
        .x_label_formatter(&move |value| {
            let index = value.floor() as usize;
            if index < labels.len() {
                labels[index].clone()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 15))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(222, 216, 207))
        .draw()
        .expect("failed to draw persistent support ratio mesh");

    chart
        .draw_series(LineSeries::new(
            vec![(0.0, 1.0), (persistent_rows.len() as f64, 1.0)],
            ShapeStyle::from(&RGBColor(120, 120, 120)).stroke_width(2),
        ))
        .expect("failed to draw support ratio reference line");

    for (index, row) in persistent_rows.iter().enumerate() {
        let x = index as f64 + 0.5;
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x - 0.22, 0.0), (x + 0.22, row.stable_zero_support_ratio)],
                ShapeStyle::from(&hinge_color(&row.hinge_category)).filled(),
            )))
            .expect("failed to draw persistent support ratio bar");
        chart
            .draw_series(std::iter::once(Text::new(
                format!("{:.2}", row.stable_zero_support_ratio),
                (x - 0.12, row.stable_zero_support_ratio + 0.04),
                ("sans-serif", 14).into_font().color(&BLACK),
            )))
            .expect("failed to draw persistent support ratio label");
    }

    root.present()
        .expect("failed to present persistent support ratio chart");
}

fn build_report_summary(
    pair_rows: &[PairMaskRow],
    base_rows: &[BaseMaskSummaryRow],
) -> ReportSummary {
    let base14 = base_rows
        .iter()
        .find(|row| row.base == 14)
        .expect("base 14 summary row should exist");
    let base10 = base_rows
        .iter()
        .find(|row| row.base == 10)
        .expect("base 10 summary row should exist");
    let base26 = base_rows
        .iter()
        .find(|row| row.base == 26)
        .expect("base 26 summary row should exist");
    let base22 = base_rows
        .iter()
        .find(|row| row.base == 22)
        .expect("base 22 summary row should exist");

    ReportSummary {
        active_pair_count_main: pair_rows.iter().filter(|row| row.scope == "main").count(),
        active_pair_count_appendix: pair_rows.iter().filter(|row| row.scope == "appendix").count(),
        main_takeaway: format!(
            "Base 14 is the only non-tiny base where persistence is fully stable-zero-led: persistent share `{}` and mean persistent support ratio `{}`, while base 10 is `{}`, base 26 is `{}`, and base 22 is `{}`.",
            format_option_share(base14.persistent_stable_zero_led_share),
            format_option_float(base14.mean_persistent_stable_zero_support_ratio),
            base10.base_label,
            base26.base_label,
            base22.base_label
        ),
    }
}

fn derive_observations(
    base_rows: &[BaseMaskSummaryRow],
    category_rows: &[CategoryMaskSummaryRow],
) -> Vec<String> {
    let base10 = base_rows
        .iter()
        .find(|row| row.base == 10)
        .expect("base 10");
    let base14 = base_rows
        .iter()
        .find(|row| row.base == 14)
        .expect("base 14");
    let base22 = base_rows
        .iter()
        .find(|row| row.base == 22)
        .expect("base 22");
    let base26 = base_rows
        .iter()
        .find(|row| row.base == 26)
        .expect("base 26");
    let persistent_core = category_rows
        .iter()
        .find(|row| row.scope == "main" && row.hinge_category == CATEGORY_PERSISTENT_CORE)
        .expect("persistent_core category should exist");
    let persistence_only = category_rows
        .iter()
        .find(|row| row.scope == "main" && row.hinge_category == CATEGORY_PERSISTENCE_ONLY)
        .expect("persistence_only category should exist");
    let core_only = category_rows
        .iter()
        .find(|row| row.scope == "main" && row.hinge_category == CATEGORY_CORE_ONLY)
        .expect("core_only category should exist");

    vec![
        format!(
            "Base 14 is the unique non-tiny hinge bridge: its mean stable-zero prime delta is `{:.2}pp` with `{}` persistent-core pairs, while base 10 is `{:.2}pp`, base 26 is `{:.2}pp`, and base 22 is `{:.2}pp`.",
            base14.mean_stable_zero_prime_delta_pp,
            base14.persistent_core_pairs,
            base10.mean_stable_zero_prime_delta_pp,
            base26.mean_stable_zero_prime_delta_pp,
            base22.mean_stable_zero_prime_delta_pp,
        ),
        format!(
            "The split between the weakening bases is exact in mask language: base 10 stays `{}` and is `{}`, base 26 stays `{}` and is `{}`, and base 22 stays `{}` and is `{}`.",
            base10.base_label,
            base10.persistent_signal_label,
            base26.base_label,
            base26.persistent_signal_label,
            base22.base_label,
            base22.persistent_signal_label
        ),
        format!(
            "Persistent-core pairs as a class are the most overlap-preserving and shared-yield-positive main species here: mean admissible-overlap jaccard `{:.2}`, mean stable-zero prime delta `{:.2}pp`, signal source `{}`, and count-dominant transition `{}`.",
            persistent_core.mean_admissible_overlap_jaccard,
            persistent_core.mean_stable_zero_prime_delta_pp,
            persistent_core.signal_source_label,
            persistent_core.dominant_count_transition_label,
        ),
        format!(
            "The strongest persistence-conditioned statement is now sharper: base 14 has persistent stable-zero-led share `{}` and mean persistent support ratio `{}`, while base 10 has `{}` and `{}`. The hinge is where persistence is fully carried by the shared-overlap lane.",
            format_option_share(base14.persistent_stable_zero_led_share),
            format_option_float(base14.mean_persistent_stable_zero_support_ratio),
            format_option_share(base10.persistent_stable_zero_led_share),
            format_option_float(base10.mean_persistent_stable_zero_support_ratio),
        ),
        format!(
            "The contrast with the split species is what matters: `persistence_only` has mean stable-zero prime delta `{:.2}pp`, while `core_only` has mean stable-zero prime delta `{:.2}pp` but no persistence. The hinge is where those two stories meet, not where either one gets large on its own.",
            persistence_only.mean_stable_zero_prime_delta_pp,
            core_only.mean_stable_zero_prime_delta_pp,
        ),
    ]
}

fn render_markdown(bundle: &ReportBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Cross-Base Hinge Mask Atlas\n\n");
    markdown.push_str("_Generated from `examples/two_p_hinge_mask_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Output directory: `{}`\n- Main bases: `{}`\n- Appendix base: `{}`\n- Middle length: `M={}`\n\n",
        bundle.settings.out_dir,
        MAIN_BASES
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        APPENDIX_BASES[0],
        bundle.settings.middle_length
    ));

    markdown.push_str("## Base Summary\n\n");
    markdown.push_str("| Scope | Base | Label | Source | Persistent source | Active pairs | Persistent-core | Stable-zero delta | Boundary delta |\n");
    markdown.push_str("|---|---:|---|---|---|---:|---:|---:|---:|\n");
    for row in &bundle.base_mask_summary_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {:.2}pp | {:.2}pp |\n",
            row.scope,
            row.base,
            row.base_label,
            row.signal_source_label,
            row.persistent_signal_label,
            row.active_pair_count,
            row.persistent_core_pairs,
            row.mean_stable_zero_prime_delta_pp,
            row.mean_boundary_prime_delta_pp,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Hinge Categories\n\n");
    markdown.push_str(
        "| Scope | Category | Source | Pairs | Bases | Mean overlap jaccard | Mean stable-zero delta |\n",
    );
    markdown.push_str("|---|---|---|---:|---|---:|---:|\n");
    for row in &bundle.category_mask_summary_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {:.2} | {:.2}pp |\n",
            row.scope,
            row.hinge_category,
            row.signal_source_label,
            row.active_pair_count,
            row.bases,
            row.mean_admissible_overlap_jaccard,
            row.mean_stable_zero_prime_delta_pp,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Active Pairs\n\n");
    markdown.push_str("| Scope | Base | Pair | Category | Source | Best k | Overlap jaccard | Stable-zero delta | Boundary delta | Support ratio |\n");
    markdown.push_str("|---|---:|---|---|---|---|---:|---:|---:|---:|\n");
    let mut active_rows = bundle.pair_mask_rows.iter().collect::<Vec<_>>();
    active_rows.sort_by(|left, right| {
        left.scope
            .cmp(&right.scope)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| {
                right
                    .stable_zero_prime_delta_pp
                    .total_cmp(&left.stable_zero_prime_delta_pp)
            })
    });
    for row in active_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {:.2} | {:.2}pp | {:.2}pp | {:.2} |\n",
            row.scope,
            row.base,
            row.pair_label,
            row.hinge_category,
            row.signal_source_label,
            row.best_k_m2,
            row.admissible_overlap_jaccard,
            row.stable_zero_prime_delta_pp,
            row.boundary_prime_delta_pp,
            row.stable_zero_support_ratio,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Persistent Pairs\n\n");
    markdown.push_str("| Base | Pair | Category | Source | Signal margin | Support ratio |\n");
    markdown.push_str("|---:|---|---|---|---:|---:|\n");
    let mut persistent_rows = bundle
        .pair_mask_rows
        .iter()
        .filter(|row| row.m2_persistent)
        .collect::<Vec<_>>();
    persistent_rows.sort_by(|left, right| {
        left.base.cmp(&right.base).then_with(|| {
            right
                .stable_zero_support_ratio
                .total_cmp(&left.stable_zero_support_ratio)
        })
    });
    for row in persistent_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {:.2}pp | {:.2} |\n",
            row.base,
            row.pair_label,
            row.hinge_category,
            row.signal_source_label,
            row.stable_zero_signal_margin_pp,
            row.stable_zero_support_ratio,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Representative Buckets\n\n");
    markdown.push_str("| Base | Pair | Category | Bucket | Prime delta |\n");
    markdown.push_str("|---:|---|---|---|---:|\n");
    for row in &bundle.representative_transition_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {:.2}pp |\n",
            row.base, row.pair_label, row.hinge_category, row.transfer_bucket, row.prime_delta_pp
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

fn anomaly_mass(row: &primes::validation::bounded_k::KDominancePairRow) -> f64 {
    row.best_minus_k00_pp.max(0.0)
}

fn hinge_category(m2_persistent: bool, shared_yield_core: bool) -> &'static str {
    match (m2_persistent, shared_yield_core) {
        (true, true) => CATEGORY_PERSISTENT_CORE,
        (true, false) => CATEGORY_PERSISTENCE_ONLY,
        (false, true) => CATEGORY_CORE_ONLY,
        (false, false) => CATEGORY_ACTIVE_NEITHER,
    }
}

fn scope_label(base: u32) -> &'static str {
    if MAIN_BASES.contains(&base) {
        "main"
    } else if APPENDIX_BASES.contains(&base) {
        "appendix"
    } else {
        panic!("base {base} not classified");
    }
}

fn is_representative(base: u32, outer: u32, inner: u32) -> bool {
    REPRESENTATIVES
        .iter()
        .any(|&(rep_base, rep_outer, rep_inner, _)| {
            rep_base == base && rep_outer == outer && rep_inner == inner
        })
}

fn bucket_count(stats: &BTreeMap<String, TransferBucketStats>, bucket: &str) -> usize {
    stats.get(bucket).map(|entry| entry.count).unwrap_or(0)
}

fn bucket_prime_delta(stats: &BTreeMap<String, TransferBucketStats>, bucket: &str) -> isize {
    stats
        .get(bucket)
        .map(|entry| entry.prime_delta_count)
        .unwrap_or(0)
}

fn dominant_count_transition_label(stats: &BTreeMap<String, TransferBucketStats>) -> &'static str {
    let mut best_bucket = TRANSFER_BUCKETS[0];
    let mut best_count = bucket_count(stats, best_bucket);
    for &bucket in &TRANSFER_BUCKETS[1..] {
        let count = bucket_count(stats, bucket);
        if count > best_count {
            best_bucket = bucket;
            best_count = count;
        }
    }
    best_bucket
}

fn signal_source_label(
    stable_zero_prime_delta_pp: f64,
    boundary_prime_delta_pp: f64,
) -> &'static str {
    const EPS: f64 = 1e-9;
    let stable_abs = stable_zero_prime_delta_pp.abs();
    let boundary_abs = boundary_prime_delta_pp.abs();
    if stable_zero_prime_delta_pp > 0.0 && stable_abs > boundary_abs + EPS {
        "stable_zero_led"
    } else if boundary_abs > stable_abs + EPS {
        "boundary_led"
    } else {
        "mixed_or_flat"
    }
}

fn persistent_signal_label<'a>(rows: impl Iterator<Item = &'a PairMaskRow>) -> String {
    let persistent_rows = rows.filter(|row| row.m2_persistent).collect::<Vec<_>>();
    if persistent_rows.is_empty() {
        return "no_persistence".to_string();
    }
    let stable_zero_led_pairs = persistent_rows
        .iter()
        .filter(|row| row.signal_source_label == "stable_zero_led")
        .count();
    let boundary_led_pairs = persistent_rows
        .iter()
        .filter(|row| row.signal_source_label == "boundary_led")
        .count();
    if stable_zero_led_pairs == persistent_rows.len() {
        "all_stable_zero_led".to_string()
    } else if boundary_led_pairs == persistent_rows.len() {
        "all_boundary_led".to_string()
    } else {
        "mixed_persistent".to_string()
    }
}

fn mean(values: impl Iterator<Item = f64>) -> f64 {
    let values = values.collect::<Vec<_>>();
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn mean_option(values: impl Iterator<Item = f64>) -> Option<f64> {
    let values = values.collect::<Vec<_>>();
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn ratio_option(numerator: usize, denominator: usize) -> Option<f64> {
    if denominator == 0 {
        None
    } else {
        Some(ratio(numerator, denominator))
    }
}

fn most_common_label(labels: impl Iterator<Item = String>) -> String {
    let mut counts = BTreeMap::<String, usize>::new();
    for label in labels {
        *counts.entry(label).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by(|left, right| left.1.cmp(&right.1).then_with(|| right.0.cmp(&left.0)))
        .map(|(label, _)| label)
        .unwrap_or_else(|| "n/a".to_string())
}

fn dominant_pair_label_share(labels: impl Iterator<Item = String>, total: usize) -> f64 {
    if total == 0 {
        return 0.0;
    }
    let mut counts = BTreeMap::<String, usize>::new();
    for label in labels {
        *counts.entry(label).or_insert(0) += 1;
    }
    counts.into_values().max().unwrap_or(0) as f64 / total as f64
}

fn format_option_share(value: Option<f64>) -> String {
    value
        .map(|value| format!("{:.2}%", value * 100.0))
        .unwrap_or_else(|| "n/a".to_string())
}

fn format_option_float(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.2}"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn hinge_color(category: &str) -> RGBColor {
    match category {
        CATEGORY_PERSISTENT_CORE => RGBColor(48, 119, 142),
        CATEGORY_PERSISTENCE_ONLY => RGBColor(218, 143, 53),
        CATEGORY_CORE_ONLY => RGBColor(181, 76, 64),
        CATEGORY_ACTIVE_NEITHER => RGBColor(122, 122, 122),
        _ => RGBColor(122, 122, 122),
    }
}

fn transfer_bucket_color(bucket: &str) -> RGBColor {
    match bucket {
        "stable_zero" => RGBColor(48, 119, 142),
        "gain_zero" => RGBColor(86, 166, 75),
        "loss_zero" => RGBColor(206, 88, 65),
        "stable_nonzero" => RGBColor(141, 110, 99),
        "nonzero_churn" => RGBColor(126, 87, 194),
        _ => RGBColor(122, 122, 122),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primes::validation::bounded_k::scan_k_config_transfer_profile;

    #[test]
    fn representative_pairs_reproduce_expected_sign_pattern() {
        let pair = analyze_active_pair(14, 13, 11).expect("base 14 (D,B) should be active");
        assert!(pair.pair_row.stable_zero_prime_delta_pp > 0.0);
        assert_eq!(pair.pair_row.signal_source_label, "stable_zero_led");

        let pair = analyze_active_pair(10, 3, 3).expect("base 10 (3,3) should be active");
        assert!(pair.pair_row.stable_zero_prime_delta_pp <= 0.0);
        assert_eq!(pair.pair_row.signal_source_label, "boundary_led");

        let pair = analyze_active_pair(26, 23, 23).expect("base 26 (N,N) should be active");
        assert!(pair.pair_row.stable_zero_prime_delta_pp > 0.0);
        assert_eq!(pair.pair_row.hinge_category, CATEGORY_CORE_ONLY);
        assert_eq!(pair.pair_row.signal_source_label, "stable_zero_led");

        let pair = analyze_active_pair(22, 17, 19).expect("base 22 (H,J) should be active");
        assert!(pair.pair_row.stable_zero_prime_delta_pp <= 0.0);
        assert_eq!(pair.pair_row.signal_source_label, "boundary_led");
    }

    #[test]
    fn transfer_histogram_representative_rows_cover_all_buckets() {
        let transfer = scan_k_config_transfer_profile(14, 2, 13, 11, (0, 0), (0, 1));
        let histogram_count = transfer
            .transfer_histogram_rows
            .iter()
            .map(|row| row.count)
            .sum::<usize>();
        assert_eq!(histogram_count, transfer.candidates_per_config);
    }

    #[test]
    fn base_signal_source_labels_match_mask_story() {
        let analyses = build_pair_analyses();
        let rows = analyses
            .iter()
            .map(|analysis| analysis.pair_row.clone())
            .collect::<Vec<_>>();
        let summaries = build_base_mask_summary_rows(&rows);

        let source = |base| {
            summaries
                .iter()
                .find(|row| row.base == base)
                .map(|row| row.signal_source_label.as_str())
                .expect("base summary should exist")
        };

        assert_eq!(source(14), "stable_zero_led");
        assert_eq!(source(10), "boundary_led");
        assert_eq!(source(22), "boundary_led");
        assert_eq!(source(26), "boundary_led");
    }

    #[test]
    fn persistence_conditioned_hinge_statement_is_sharp() {
        let analyses = build_pair_analyses();
        let rows = analyses
            .iter()
            .map(|analysis| analysis.pair_row.clone())
            .collect::<Vec<_>>();
        let summaries = build_base_mask_summary_rows(&rows);

        let base = |n| {
            summaries
                .iter()
                .find(|row| row.base == n)
                .expect("base summary")
        };

        let base14 = base(14);
        assert_eq!(base14.persistent_signal_label, "all_stable_zero_led");
        assert_eq!(base14.persistent_stable_zero_led_share, Some(1.0));
        assert!(
            base14
                .mean_persistent_stable_zero_support_ratio
                .unwrap_or(0.0)
                >= 1.0
        );

        let base10 = base(10);
        assert_eq!(base10.persistent_signal_label, "all_boundary_led");
        assert_eq!(base10.persistent_stable_zero_led_share, Some(0.0));
        assert_eq!(base10.mean_persistent_stable_zero_support_ratio, Some(0.0));
    }
}
