//! Flexible base probe for the bounded-`k` hinge lane.
//!
//! This example is meant for exploratory "search the solution space" work
//! without rewriting the maintained `2p` reports every time a new base looks
//! interesting.
//!
//! By default it probes base `34 = 2 * 17` against the current `2p` hinge
//! reference bases `10, 14, 22, 26`.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base_hinge_probe_report
//! cargo run --release --example base_hinge_probe_report -- --base 34 --reference-bases 10,14,22,26 --out-dir /tmp/primes_base34_probe
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        digit_symbol, evaluate_pair_row, ordered_unit_pairs, parse_k_label,
        scan_k_config_mask_profile, unit_residues, KDominancePairRow, DEFAULT_BOUNDED_K_GRID,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    path::{Path, PathBuf},
};

const DEFAULT_TARGET_BASE: u32 = 34;
const DEFAULT_REFERENCE_BASES: &[u32] = &[10, 14, 22, 26];
const DEFAULT_OUT_DIR: &str = "/tmp/primes_base34_probe";
const REPORT_EXPORT_VERSION: u32 = 1;
const M1: usize = 1;
const M2: usize = 2;
const M3: usize = 3;
const DEFAULT_TOP_ROWS: usize = 12;

const CATEGORY_PERSISTENT_CORE: &str = "persistent_core";
const CATEGORY_PERSISTENCE_ONLY: &str = "persistence_only";
const CATEGORY_CORE_ONLY: &str = "core_only";
const CATEGORY_ACTIVE_NEITHER: &str = "active_neither";

#[derive(Debug)]
struct Options {
    target_base: u32,
    reference_bases: Vec<u32>,
    top_rows: usize,
    max_middle_length: usize,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    target_base: u32,
    reference_bases: Vec<u32>,
    bases_analyzed: Vec<u32>,
    middle_lengths: Vec<usize>,
    k_grid: Vec<String>,
    top_rows: usize,
    out_dir: String,
}

#[derive(Debug, Clone, Serialize)]
struct PairSignalRow {
    scope: String,
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    unit_distance: usize,
    gap_bucket: String,
    same_digit: bool,
    best_k_m1: String,
    best_k_m2: String,
    best_k_m3: String,
    anomaly_m1_pp: f64,
    anomaly_m2_pp: f64,
    anomaly_m3_pp: f64,
    m1_anomalous: bool,
    m2_active: bool,
    m2_persistent: bool,
    m2_emergent: bool,
    positive_shared_yield: bool,
    shared_yield_core: bool,
    hinge_category: String,
    stable_zero_prime_delta_count: isize,
    stable_zero_prime_delta_pp: f64,
    boundary_prime_delta_count: isize,
    boundary_prime_delta_pp: f64,
    admissible_set_effect_pp: f64,
    prime_yield_effect_pp: f64,
    shared_admissible_count: usize,
    shared_prime_delta_count: isize,
    overlap_prime_delta_count: isize,
    shared_prime_rate_k00_pp: f64,
    shared_prime_rate_best_pp: f64,
    shared_prime_rate_delta_pp: f64,
    signal_source_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct BaseComparisonRow {
    scope: String,
    base: u32,
    unit_count: usize,
    ordered_pair_count: usize,
    m1_anomalous_pairs: usize,
    m2_active_pairs: usize,
    m2_persistent_pairs: usize,
    m2_emergent_pairs: usize,
    m3_active_pairs: usize,
    positive_shared_yield_pairs: usize,
    shared_yield_core_pairs: usize,
    persistent_core_pairs: usize,
    m1_anomaly_mass_pp: f64,
    m2_anomaly_mass_pp: f64,
    m3_anomaly_mass_pp: f64,
    persistence_rate_given_m1: Option<f64>,
    active_rate: f64,
    shared_yield_core_share_given_m2: Option<f64>,
    persistent_core_share_given_m2: Option<f64>,
    mean_anomaly_m2_pp_given_active: Option<f64>,
    mean_stable_zero_prime_delta_pp_given_active: Option<f64>,
    mean_boundary_prime_delta_pp_given_active: Option<f64>,
    mean_shared_prime_rate_delta_pp_given_active: Option<f64>,
    base_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct TargetPocketRow {
    gap_bucket: String,
    ordered_pair_count: usize,
    same_digit_pairs: usize,
    m1_anomalous_pairs: usize,
    m2_active_pairs: usize,
    m2_persistent_pairs: usize,
    shared_yield_core_pairs: usize,
    persistence_rate_given_m1: Option<f64>,
    shared_yield_core_share_given_m2: Option<f64>,
    mean_anomaly_m2_pp_given_active: Option<f64>,
    mean_stable_zero_prime_delta_pp_given_active: Option<f64>,
    mean_boundary_prime_delta_pp_given_active: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    target_active_pairs: usize,
    target_persistent_pairs: usize,
    target_shared_yield_core_pairs: usize,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    all_pair_rows: Vec<PairSignalRow>,
    base_comparison_rows: Vec<BaseComparisonRow>,
    target_pocket_rows: Vec<TargetPocketRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

#[derive(Debug, Clone)]
struct SharedYieldMetrics {
    shared_admissible_count: usize,
    shared_prime_delta_count: isize,
    overlap_prime_delta_count: isize,
    stable_zero_prime_delta_count: isize,
    boundary_prime_delta_count: isize,
    shared_prime_rate_k00_pp: f64,
    shared_prime_rate_best_pp: f64,
    shared_prime_rate_delta_pp: f64,
    admissible_set_effect_pp: f64,
    prime_yield_effect_pp: f64,
    positive_shared_yield: bool,
    shared_yield_core: bool,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let bases = ordered_unique_bases(options.target_base, &options.reference_bases);
    let settings = ReportSettings {
        target_base: options.target_base,
        reference_bases: options.reference_bases.clone(),
        bases_analyzed: bases.clone(),
        middle_lengths: (M1..=options.max_middle_length).collect(),
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format!("k=({},{})", config.0, config.1))
            .collect(),
        top_rows: options.top_rows,
        out_dir: options.out_dir.display().to_string(),
    };

    let all_pair_rows = build_pair_rows(&bases, options.target_base, options.max_middle_length);
    let base_comparison_rows = build_base_comparison_rows(&all_pair_rows, options.target_base);
    let target_pocket_rows = build_target_pocket_rows(&all_pair_rows, options.target_base);

    let comparison_scatter_path = options.out_dir.join("base_hinge_scatter.png");
    render_base_hinge_scatter(
        &base_comparison_rows,
        options.target_base,
        &comparison_scatter_path,
    );
    let target_signal_plane_path = options.out_dir.join("target_signal_plane.png");
    render_target_signal_plane(
        &all_pair_rows,
        options.target_base,
        &target_signal_plane_path,
    );

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "base_hinge_scatter".to_string(),
            label: "Target base against hinge references".to_string(),
            path: comparison_scatter_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "target_signal_plane".to_string(),
            label: "Target-base active pair signal plane".to_string(),
            path: target_signal_plane_path.display().to_string(),
        },
    ];

    let report_summary =
        build_report_summary(&all_pair_rows, &base_comparison_rows, options.target_base);
    let observations = derive_observations(
        &all_pair_rows,
        &base_comparison_rows,
        &target_pocket_rows,
        options.target_base,
    );

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        all_pair_rows: all_pair_rows.clone(),
        base_comparison_rows: base_comparison_rows.clone(),
        target_pocket_rows: target_pocket_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(options.out_dir.join("all_pair_rows.csv"), &all_pair_rows)
        .expect("failed to write all_pair_rows.csv");
    write_csv_rows(
        options.out_dir.join("base_comparison_rows.csv"),
        &base_comparison_rows,
    )
    .expect("failed to write base_comparison_rows.csv");
    write_csv_rows(
        options.out_dir.join("target_pocket_rows.csv"),
        &target_pocket_rows,
    )
    .expect("failed to write target_pocket_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");
    write_text_file(
        options.out_dir.join("report.md"),
        &render_markdown(&bundle, options.target_base, options.top_rows),
    )
    .expect("failed to write report.md");

    println!("base hinge probe report");
    println!("  target base: {}", options.target_base);
    println!("  output dir: {}", options.out_dir.display());
    for row in &base_comparison_rows {
        println!(
            "  base {:>2} | {:<10} | persistence {} | core {} | active {} | label {}",
            row.base,
            row.scope,
            format_option_share(row.persistence_rate_given_m1),
            format_option_share(row.shared_yield_core_share_given_m2),
            row.m2_active_pairs,
            row.base_label,
        );
    }
}

fn parse_args() -> Options {
    let mut target_base = DEFAULT_TARGET_BASE;
    let mut reference_bases = DEFAULT_REFERENCE_BASES.to_vec();
    let mut top_rows = DEFAULT_TOP_ROWS;
    let mut max_middle_length = M2;
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--base" => {
                let value = args.next().expect("--base requires a numeric argument");
                target_base = value.parse().expect("--base must be a positive integer");
            }
            "--reference-bases" => {
                let value = args
                    .next()
                    .expect("--reference-bases requires a comma-separated value");
                reference_bases = parse_base_list(&value);
            }
            "--top" => {
                let value = args.next().expect("--top requires a numeric argument");
                top_rows = value.parse().expect("--top must be a positive integer");
            }
            "--max-middle-length" => {
                let value = args
                    .next()
                    .expect("--max-middle-length requires a numeric argument");
                max_middle_length = value
                    .parse()
                    .expect("--max-middle-length must be a positive integer");
                assert!(
                    (M2..=M3).contains(&max_middle_length),
                    "--max-middle-length must be 2 or 3"
                );
            }
            "--out-dir" => {
                let value = args.next().expect("--out-dir requires a directory");
                out_dir = PathBuf::from(value);
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => panic!("unrecognized argument: {arg}"),
        }
    }

    reference_bases.retain(|&base| base != target_base);
    if reference_bases.is_empty() {
        reference_bases = DEFAULT_REFERENCE_BASES
            .iter()
            .copied()
            .filter(|&base| base != target_base)
            .collect();
    }

    Options {
        target_base,
        reference_bases,
        top_rows,
        max_middle_length,
        out_dir,
    }
}

fn print_help() {
    println!("Usage:");
    println!("  cargo run --release --example base_hinge_probe_report -- [options]");
    println!();
    println!("Options:");
    println!("  --base <n>                 Target base to probe (default: {DEFAULT_TARGET_BASE})");
    println!(
        "  --reference-bases <list>   Comma-separated comparison bases (default: 10,14,22,26)"
    );
    println!("  --top <n>                  Number of top target-base rows to show (default: {DEFAULT_TOP_ROWS})");
    println!("  --max-middle-length <n>    Probe through M=n (default: 2, allowed: 2 or 3)");
    println!("  --out-dir <dir>            Output directory (default: {DEFAULT_OUT_DIR})");
    println!("  -h, --help                 Show this help");
}

fn parse_base_list(raw: &str) -> Vec<u32> {
    raw.split(',')
        .filter(|item| !item.trim().is_empty())
        .map(|item| {
            item.trim()
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("invalid base in list: {item}"))
        })
        .collect()
}

fn ordered_unique_bases(target_base: u32, reference_bases: &[u32]) -> Vec<u32> {
    let mut bases = vec![target_base];
    let mut seen = BTreeSet::from([target_base]);
    for &base in reference_bases {
        if seen.insert(base) {
            bases.push(base);
        }
    }
    bases
}

fn build_pair_rows(
    bases: &[u32],
    target_base: u32,
    max_middle_length: usize,
) -> Vec<PairSignalRow> {
    bases
        .par_iter()
        .copied()
        .flat_map_iter(|base| {
            ordered_unit_pairs(base)
                .into_iter()
                .map(move |(outer, inner)| {
                    build_pair_row(base, outer, inner, target_base, max_middle_length)
                })
        })
        .collect()
}

fn build_pair_row(
    base: u32,
    outer: u32,
    inner: u32,
    target_base: u32,
    max_middle_length: usize,
) -> PairSignalRow {
    let row_m1 = evaluate_pair_row(base, M1, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let row_m2 = evaluate_pair_row(base, M2, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let row_m3 = if max_middle_length >= M3 {
        Some(evaluate_pair_row(
            base,
            M3,
            outer,
            inner,
            DEFAULT_BOUNDED_K_GRID,
        ))
    } else {
        None
    };
    let anomaly_m1 = anomaly_mass(&row_m1);
    let anomaly_m2 = anomaly_mass(&row_m2);
    let anomaly_m3 = row_m3.as_ref().map(anomaly_mass).unwrap_or(0.0);
    let m1_anomalous = anomaly_m1 > 0.0;
    let m2_active = anomaly_m2 > 0.0;
    let m2_persistent = m1_anomalous && m2_active;
    let m2_emergent = !m1_anomalous && m2_active;
    let metrics = if m2_active {
        shared_yield_metrics(base, outer, inner, parse_k_label(&row_m2.best_k))
    } else {
        SharedYieldMetrics {
            shared_admissible_count: 0,
            shared_prime_delta_count: 0,
            overlap_prime_delta_count: 0,
            stable_zero_prime_delta_count: 0,
            boundary_prime_delta_count: 0,
            shared_prime_rate_k00_pp: 0.0,
            shared_prime_rate_best_pp: 0.0,
            shared_prime_rate_delta_pp: 0.0,
            admissible_set_effect_pp: 0.0,
            prime_yield_effect_pp: 0.0,
            positive_shared_yield: false,
            shared_yield_core: false,
        }
    };

    PairSignalRow {
        scope: if base == target_base {
            "target".to_string()
        } else {
            "reference".to_string()
        },
        base,
        outer,
        inner,
        pair_label: format!("({},{})", digit_symbol(outer), digit_symbol(inner)),
        unit_distance: cyclic_unit_distance(base, outer, inner),
        gap_bucket: gap_bucket(base, outer, inner).to_string(),
        same_digit: outer == inner,
        best_k_m1: row_m1.best_k,
        best_k_m2: row_m2.best_k,
        best_k_m3: row_m3
            .as_ref()
            .map(|row| row.best_k.clone())
            .unwrap_or_else(|| "-".to_string()),
        anomaly_m1_pp: anomaly_m1,
        anomaly_m2_pp: anomaly_m2,
        anomaly_m3_pp: anomaly_m3,
        m1_anomalous,
        m2_active,
        m2_persistent,
        m2_emergent,
        positive_shared_yield: metrics.positive_shared_yield,
        shared_yield_core: metrics.shared_yield_core,
        hinge_category: hinge_category(m2_persistent, metrics.shared_yield_core).to_string(),
        stable_zero_prime_delta_count: metrics.stable_zero_prime_delta_count,
        stable_zero_prime_delta_pp: metrics.stable_zero_prime_delta_count as f64 * 100.0
            / (base as usize).pow(M2 as u32) as f64,
        boundary_prime_delta_count: metrics.boundary_prime_delta_count,
        boundary_prime_delta_pp: metrics.boundary_prime_delta_count as f64 * 100.0
            / (base as usize).pow(M2 as u32) as f64,
        admissible_set_effect_pp: metrics.admissible_set_effect_pp,
        prime_yield_effect_pp: metrics.prime_yield_effect_pp,
        shared_admissible_count: metrics.shared_admissible_count,
        shared_prime_delta_count: metrics.shared_prime_delta_count,
        overlap_prime_delta_count: metrics.overlap_prime_delta_count,
        shared_prime_rate_k00_pp: metrics.shared_prime_rate_k00_pp,
        shared_prime_rate_best_pp: metrics.shared_prime_rate_best_pp,
        shared_prime_rate_delta_pp: metrics.shared_prime_rate_delta_pp,
        signal_source_label: signal_source_label(
            metrics.stable_zero_prime_delta_count as f64 * 100.0
                / (base as usize).pow(M2 as u32) as f64,
            metrics.boundary_prime_delta_count as f64 * 100.0
                / (base as usize).pow(M2 as u32) as f64,
        )
        .to_string(),
    }
}

fn shared_yield_metrics(
    base: u32,
    outer: u32,
    inner: u32,
    best_k: (u32, u32),
) -> SharedYieldMetrics {
    let k00_profile = scan_k_config_mask_profile(base, M2, outer, inner, (0, 0));
    let best_profile = if best_k == (0, 0) {
        k00_profile.clone()
    } else {
        scan_k_config_mask_profile(base, M2, outer, inner, best_k)
    };

    let mut shared_prime_delta_count = 0isize;
    let mut overlap_prime_delta_count = 0isize;
    let mut shared_admissible_count = 0usize;
    let mut shared_prime_hits_k00 = 0usize;
    let mut shared_prime_hits_best = 0usize;
    let mut stable_zero_prime_delta_count = 0isize;
    let mut boundary_prime_delta_count = 0isize;

    for (k00_row, best_row) in k00_profile
        .candidate_rows
        .iter()
        .zip(&best_profile.candidate_rows)
    {
        match (k00_row.admissible, best_row.admissible) {
            (true, true) => {
                shared_admissible_count += 1;
                if k00_row.prime {
                    shared_prime_hits_k00 += 1;
                    shared_prime_delta_count -= 1;
                    stable_zero_prime_delta_count -= 1;
                }
                if best_row.prime {
                    shared_prime_hits_best += 1;
                    shared_prime_delta_count += 1;
                    stable_zero_prime_delta_count += 1;
                }
            }
            (false, true) => {
                if best_row.prime {
                    overlap_prime_delta_count += 1;
                    boundary_prime_delta_count += 1;
                }
            }
            (true, false) => {
                if k00_row.prime {
                    overlap_prime_delta_count -= 1;
                    boundary_prime_delta_count -= 1;
                }
            }
            (false, false) => {}
        }
    }

    let admissible_share_k00 = ratio(
        k00_profile.admissible_count,
        k00_profile.candidates_per_config,
    );
    let admissible_share_best = ratio(
        best_profile.admissible_count,
        best_profile.candidates_per_config,
    );
    let prime_yield_k00 = ratio(k00_profile.prime_hits, k00_profile.admissible_count);
    let prime_yield_best = ratio(best_profile.prime_hits, best_profile.admissible_count);
    let admissible_set_effect_pp =
        (admissible_share_best - admissible_share_k00) * prime_yield_k00 * 100.0;
    let prime_yield_effect_pp =
        admissible_share_best * (prime_yield_best - prime_yield_k00) * 100.0;

    SharedYieldMetrics {
        shared_admissible_count,
        shared_prime_delta_count,
        overlap_prime_delta_count,
        stable_zero_prime_delta_count,
        boundary_prime_delta_count,
        shared_prime_rate_k00_pp: ratio(shared_prime_hits_k00, shared_admissible_count) * 100.0,
        shared_prime_rate_best_pp: ratio(shared_prime_hits_best, shared_admissible_count) * 100.0,
        shared_prime_rate_delta_pp: (ratio(shared_prime_hits_best, shared_admissible_count)
            - ratio(shared_prime_hits_k00, shared_admissible_count))
            * 100.0,
        admissible_set_effect_pp,
        prime_yield_effect_pp,
        positive_shared_yield: shared_prime_delta_count > 0,
        shared_yield_core: stable_zero_prime_delta_count > boundary_prime_delta_count.abs()
            && stable_zero_prime_delta_count > 0
            && prime_yield_effect_pp.abs() > admissible_set_effect_pp.abs(),
    }
}

fn build_base_comparison_rows(rows: &[PairSignalRow], target_base: u32) -> Vec<BaseComparisonRow> {
    let mut by_base = BTreeMap::<u32, Vec<&PairSignalRow>>::new();
    for row in rows {
        by_base.entry(row.base).or_default().push(row);
    }

    by_base
        .into_iter()
        .map(|(base, group)| {
            let ordered_pair_count = group.len();
            let m1_anomalous_pairs = group.iter().filter(|row| row.m1_anomalous).count();
            let m2_active_pairs = group.iter().filter(|row| row.m2_active).count();
            let m2_persistent_pairs = group.iter().filter(|row| row.m2_persistent).count();
            let m2_emergent_pairs = group.iter().filter(|row| row.m2_emergent).count();
            let m3_active_pairs = group.iter().filter(|row| row.anomaly_m3_pp > 0.0).count();
            let positive_shared_yield_pairs =
                group.iter().filter(|row| row.positive_shared_yield).count();
            let shared_yield_core_pairs = group.iter().filter(|row| row.shared_yield_core).count();
            let persistent_core_pairs = group
                .iter()
                .filter(|row| row.hinge_category == CATEGORY_PERSISTENT_CORE)
                .count();
            let active_rows = group
                .iter()
                .filter(|row| row.m2_active)
                .copied()
                .collect::<Vec<_>>();

            BaseComparisonRow {
                scope: if base == target_base {
                    "target".to_string()
                } else {
                    "reference".to_string()
                },
                base,
                unit_count: unit_residues(base).len(),
                ordered_pair_count,
                m1_anomalous_pairs,
                m2_active_pairs,
                m2_persistent_pairs,
                m2_emergent_pairs,
                m3_active_pairs,
                positive_shared_yield_pairs,
                shared_yield_core_pairs,
                persistent_core_pairs,
                m1_anomaly_mass_pp: group.iter().map(|row| row.anomaly_m1_pp).sum(),
                m2_anomaly_mass_pp: group.iter().map(|row| row.anomaly_m2_pp).sum(),
                m3_anomaly_mass_pp: group.iter().map(|row| row.anomaly_m3_pp).sum(),
                persistence_rate_given_m1: ratio_option(m2_persistent_pairs, m1_anomalous_pairs),
                active_rate: ratio(m2_active_pairs, ordered_pair_count),
                shared_yield_core_share_given_m2: ratio_option(
                    shared_yield_core_pairs,
                    m2_active_pairs,
                ),
                persistent_core_share_given_m2: ratio_option(
                    persistent_core_pairs,
                    m2_active_pairs,
                ),
                mean_anomaly_m2_pp_given_active: mean(
                    &active_rows
                        .iter()
                        .map(|row| row.anomaly_m2_pp)
                        .collect::<Vec<_>>(),
                ),
                mean_stable_zero_prime_delta_pp_given_active: mean(
                    &active_rows
                        .iter()
                        .map(|row| row.stable_zero_prime_delta_pp)
                        .collect::<Vec<_>>(),
                ),
                mean_boundary_prime_delta_pp_given_active: mean(
                    &active_rows
                        .iter()
                        .map(|row| row.boundary_prime_delta_pp)
                        .collect::<Vec<_>>(),
                ),
                mean_shared_prime_rate_delta_pp_given_active: mean(
                    &active_rows
                        .iter()
                        .map(|row| row.shared_prime_rate_delta_pp)
                        .collect::<Vec<_>>(),
                ),
                base_label: base_label(
                    m2_active_pairs,
                    m2_persistent_pairs,
                    persistent_core_pairs,
                    shared_yield_core_pairs,
                )
                .to_string(),
            }
        })
        .collect()
}

fn build_target_pocket_rows(rows: &[PairSignalRow], target_base: u32) -> Vec<TargetPocketRow> {
    let target_rows = rows
        .iter()
        .filter(|row| row.base == target_base)
        .collect::<Vec<_>>();
    ["same", "adjacent", "wide"]
        .into_iter()
        .map(|bucket| {
            let group = target_rows
                .iter()
                .copied()
                .filter(|row| row.gap_bucket == bucket)
                .collect::<Vec<_>>();
            let active_rows = group
                .iter()
                .copied()
                .filter(|row| row.m2_active)
                .collect::<Vec<_>>();
            TargetPocketRow {
                gap_bucket: bucket.to_string(),
                ordered_pair_count: group.len(),
                same_digit_pairs: group.iter().filter(|row| row.same_digit).count(),
                m1_anomalous_pairs: group.iter().filter(|row| row.m1_anomalous).count(),
                m2_active_pairs: active_rows.len(),
                m2_persistent_pairs: group.iter().filter(|row| row.m2_persistent).count(),
                shared_yield_core_pairs: group.iter().filter(|row| row.shared_yield_core).count(),
                persistence_rate_given_m1: ratio_option(
                    group.iter().filter(|row| row.m2_persistent).count(),
                    group.iter().filter(|row| row.m1_anomalous).count(),
                ),
                shared_yield_core_share_given_m2: ratio_option(
                    group.iter().filter(|row| row.shared_yield_core).count(),
                    active_rows.len(),
                ),
                mean_anomaly_m2_pp_given_active: mean(
                    &active_rows
                        .iter()
                        .map(|row| row.anomaly_m2_pp)
                        .collect::<Vec<_>>(),
                ),
                mean_stable_zero_prime_delta_pp_given_active: mean(
                    &active_rows
                        .iter()
                        .map(|row| row.stable_zero_prime_delta_pp)
                        .collect::<Vec<_>>(),
                ),
                mean_boundary_prime_delta_pp_given_active: mean(
                    &active_rows
                        .iter()
                        .map(|row| row.boundary_prime_delta_pp)
                        .collect::<Vec<_>>(),
                ),
            }
        })
        .collect()
}

fn render_base_hinge_scatter(rows: &[BaseComparisonRow], target_base: u32, path: &Path) {
    let root = BitMapBackend::new(path, (1120, 760)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill base hinge scatter canvas");

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base Probe  (x = persistence given M=1, y = shared-yield-core given M=2)",
            ("sans-serif", 28),
        )
        .margin(28)
        .x_label_area_size(64)
        .y_label_area_size(80)
        .build_cartesian_2d(0.0f64..1.05f64, 0.0f64..1.05f64)
        .expect("failed to build base hinge scatter");

    chart
        .configure_mesh()
        .x_desc("M=2 persistence rate given M=1")
        .y_desc("shared-yield-core share given M=2")
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(222, 216, 207))
        .draw()
        .expect("failed to draw base hinge scatter mesh");

    for row in rows {
        let x = row.persistence_rate_given_m1.unwrap_or(0.0);
        let y = row.shared_yield_core_share_given_m2.unwrap_or(0.0);
        let color = if row.base == target_base {
            RGBColor(196, 94, 49)
        } else {
            RGBColor(60, 110, 113)
        };
        let radius = if row.base == target_base { 11 } else { 8 };

        chart
            .draw_series(std::iter::once(Circle::new(
                (x, y),
                radius,
                ShapeStyle::from(&color).filled(),
            )))
            .expect("failed to draw scatter point");
        chart
            .draw_series(std::iter::once(Text::new(
                format!("{} ({})", row.base, row.base_label),
                (x + 0.015, y + 0.02),
                ("sans-serif", 16).into_font().color(&BLACK),
            )))
            .expect("failed to draw scatter label");
    }

    root.present()
        .expect("failed to present base hinge scatter");
}

fn render_target_signal_plane(rows: &[PairSignalRow], target_base: u32, path: &Path) {
    let active_rows = rows
        .iter()
        .filter(|row| row.base == target_base && row.m2_active)
        .collect::<Vec<_>>();

    let root = BitMapBackend::new(path, (1180, 760)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill target signal plane canvas");

    if active_rows.is_empty() {
        root.draw(&Text::new(
            format!("Base {target_base} has no M=2 active pairs"),
            (220, 360),
            ("sans-serif", 32).into_font().color(&BLACK),
        ))
        .expect("failed to draw empty signal message");
        root.present()
            .expect("failed to present empty signal plane");
        return;
    }

    let x_min = active_rows
        .iter()
        .map(|row| row.stable_zero_prime_delta_pp)
        .fold(f64::INFINITY, f64::min)
        .min(0.0)
        - 0.5;
    let x_max = active_rows
        .iter()
        .map(|row| row.stable_zero_prime_delta_pp)
        .fold(f64::NEG_INFINITY, f64::max)
        .max(0.0)
        + 0.5;
    let y_max = active_rows
        .iter()
        .map(|row| row.anomaly_m2_pp)
        .fold(0.0, f64::max)
        + 0.5;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!(
                "Base {target_base} Active Pair Plane  (x = stable-zero delta, y = M=2 anomaly)"
            ),
            ("sans-serif", 28),
        )
        .margin(28)
        .x_label_area_size(72)
        .y_label_area_size(84)
        .build_cartesian_2d(x_min..x_max, 0.0f64..y_max)
        .expect("failed to build target signal plane");

    chart
        .configure_mesh()
        .x_desc("stable-zero prime delta (pp)")
        .y_desc("M=2 anomaly mass (pp)")
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(222, 216, 207))
        .draw()
        .expect("failed to draw target signal plane mesh");

    for row in &active_rows {
        let color = hinge_color(&row.hinge_category);
        chart
            .draw_series(std::iter::once(Circle::new(
                (row.stable_zero_prime_delta_pp, row.anomaly_m2_pp),
                8,
                ShapeStyle::from(&color).filled(),
            )))
            .expect("failed to draw active signal point");
    }

    let mut top_rows = active_rows.clone();
    top_rows.sort_by(|left, right| right.anomaly_m2_pp.total_cmp(&left.anomaly_m2_pp));
    for row in top_rows.into_iter().take(8) {
        chart
            .draw_series(std::iter::once(Text::new(
                row.pair_label.clone(),
                (
                    row.stable_zero_prime_delta_pp + 0.08,
                    row.anomaly_m2_pp + 0.05,
                ),
                ("sans-serif", 15).into_font().color(&BLACK),
            )))
            .expect("failed to draw active signal label");
    }

    root.present()
        .expect("failed to present target signal plane");
}

fn build_report_summary(
    _rows: &[PairSignalRow],
    base_rows: &[BaseComparisonRow],
    target_base: u32,
) -> ReportSummary {
    let target_base_row = base_rows
        .iter()
        .find(|row| row.base == target_base)
        .expect("target base row should exist");
    ReportSummary {
        target_active_pairs: target_base_row.m2_active_pairs,
        target_persistent_pairs: target_base_row.m2_persistent_pairs,
        target_shared_yield_core_pairs: target_base_row.shared_yield_core_pairs,
        main_takeaway: format!(
            "Base {} lands as `{}` with {} M=2 active pairs, {} persistent pairs, and {} shared-yield-core pairs.",
            target_base,
            target_base_row.base_label,
            target_base_row.m2_active_pairs,
            target_base_row.m2_persistent_pairs,
            target_base_row.shared_yield_core_pairs,
        ),
    }
}

fn derive_observations(
    rows: &[PairSignalRow],
    base_rows: &[BaseComparisonRow],
    pocket_rows: &[TargetPocketRow],
    target_base: u32,
) -> Vec<String> {
    let target_base_row = base_rows
        .iter()
        .find(|row| row.base == target_base)
        .expect("target base row should exist");
    let strongest_reference = base_rows
        .iter()
        .filter(|row| row.base != target_base)
        .max_by(|left, right| {
            left.persistent_core_pairs
                .cmp(&right.persistent_core_pairs)
                .then_with(|| {
                    left.shared_yield_core_share_given_m2
                        .unwrap_or(0.0)
                        .total_cmp(&right.shared_yield_core_share_given_m2.unwrap_or(0.0))
                })
        })
        .expect("at least one reference base should exist");

    let target_active_rows = rows
        .iter()
        .filter(|row| row.base == target_base && row.m2_active)
        .collect::<Vec<_>>();
    let top_active = target_active_rows
        .iter()
        .max_by(|left, right| left.anomaly_m2_pp.total_cmp(&right.anomaly_m2_pp));
    let top_stable_zero = target_active_rows.iter().max_by(|left, right| {
        left.stable_zero_prime_delta_pp
            .total_cmp(&right.stable_zero_prime_delta_pp)
    });
    let best_pocket = pocket_rows
        .iter()
        .max_by(|left, right| {
            left.mean_anomaly_m2_pp_given_active
                .unwrap_or(0.0)
                .total_cmp(&right.mean_anomaly_m2_pp_given_active.unwrap_or(0.0))
        })
        .expect("target pocket rows should exist");

    let mut observations = vec![format!(
        "Base {} does not automatically inherit the base-14 hinge. It comes out as `{}` versus base {} at `{}`.",
        target_base, target_base_row.base_label, strongest_reference.base, strongest_reference.base_label
    )];

    observations.push(format!(
        "At the base-summary level, base {} has persistence {} and shared-yield-core share {}, compared with base {} at {} and {}.",
        target_base,
        format_option_share(target_base_row.persistence_rate_given_m1),
        format_option_share(target_base_row.shared_yield_core_share_given_m2),
        strongest_reference.base,
        format_option_share(strongest_reference.persistence_rate_given_m1),
        format_option_share(strongest_reference.shared_yield_core_share_given_m2),
    ));

    if let Some(row) = top_active {
        observations.push(format!(
            "The strongest base-{} M=2 pocket is {} with anomaly {:.2}pp, best lane {}, signal source `{}`, and hinge category `{}`.",
            target_base,
            row.pair_label,
            row.anomaly_m2_pp,
            row.best_k_m2,
            row.signal_source_label,
            row.hinge_category,
        ));
    }

    if let Some(row) = top_stable_zero {
        observations.push(format!(
            "The strongest shared-overlap witness inside base {} is {} with stable-zero delta {:.2}pp and shared prime-rate delta {:.2}pp.",
            target_base,
            row.pair_label,
            row.stable_zero_prime_delta_pp,
            row.shared_prime_rate_delta_pp,
        ));
    }

    observations.push(format!(
        "The best local pocket in base {} is the `{}` gap bucket: {} active pairs, persistence {}, shared-yield-core share {}, mean M=2 anomaly {:.2}pp.",
        target_base,
        best_pocket.gap_bucket,
        best_pocket.m2_active_pairs,
        format_option_share(best_pocket.persistence_rate_given_m1),
        format_option_share(best_pocket.shared_yield_core_share_given_m2),
        best_pocket.mean_anomaly_m2_pp_given_active.unwrap_or(0.0),
    ));

    observations
}

fn render_markdown(bundle: &ReportBundle, target_base: u32, top_rows: usize) -> String {
    let target_base_row = bundle
        .base_comparison_rows
        .iter()
        .find(|row| row.base == target_base)
        .expect("target base row should exist");
    let mut target_active_rows = bundle
        .all_pair_rows
        .iter()
        .filter(|row| row.base == target_base && row.m2_active)
        .cloned()
        .collect::<Vec<_>>();
    target_active_rows.sort_by(|left, right| right.anomaly_m2_pp.total_cmp(&left.anomaly_m2_pp));

    let mut target_shared_rows = target_active_rows.clone();
    target_shared_rows.sort_by(|left, right| {
        right
            .stable_zero_prime_delta_pp
            .total_cmp(&left.stable_zero_prime_delta_pp)
            .then_with(|| right.anomaly_m2_pp.total_cmp(&left.anomaly_m2_pp))
    });

    let mut markdown = String::new();
    markdown.push_str("# Base Hinge Probe\n\n");
    markdown.push_str("_Generated from `examples/base_hinge_probe_report.rs`._\n\n");
    markdown.push_str(&format!(
        "This probe asks a deliberately exploratory question: what signal, if any, is hiding in base `{}` once we look at the same bounded-`k` hinge machinery used for bases `10, 14, 22, 26`?\n\n",
        target_base
    ));

    markdown.push_str("## Summary\n\n");
    markdown.push_str(&format!(
        "- Main takeaway: {}\n",
        bundle.report_summary.main_takeaway
    ));
    markdown.push_str(&format!(
        "- Target visuals: ![Base hinge scatter]({}) and ![Target signal plane]({})\n\n",
        bundle.image_artifact_rows[0].path, bundle.image_artifact_rows[1].path
    ));

    markdown.push_str("## Base Comparison\n\n");
    markdown.push_str("| Base | Scope | Label | Persistence | Core share | M2 active | M2 persistent | Persistent core | Mean stable-zero delta |\n");
    markdown.push_str("|---|---|---|---:|---:|---:|---:|---:|---:|\n");
    for row in &bundle.base_comparison_rows {
        markdown.push_str(&format!(
            "| {} | {} | `{}` | {} | {} | {} | {} | {} | {} |\n",
            row.base,
            row.scope,
            row.base_label,
            format_option_share(row.persistence_rate_given_m1),
            format_option_share(row.shared_yield_core_share_given_m2),
            row.m2_active_pairs,
            row.m2_persistent_pairs,
            row.persistent_core_pairs,
            format_option_float(row.mean_stable_zero_prime_delta_pp_given_active),
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Target Pockets\n\n");
    markdown.push_str("| Gap bucket | Ordered pairs | M2 active | M2 persistent | Core pairs | Persistence | Core share | Mean anomaly |\n");
    markdown.push_str("|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in &bundle.target_pocket_rows {
        markdown.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} | {} | {} |\n",
            row.gap_bucket,
            row.ordered_pair_count,
            row.m2_active_pairs,
            row.m2_persistent_pairs,
            row.shared_yield_core_pairs,
            format_option_share(row.persistence_rate_given_m1),
            format_option_share(row.shared_yield_core_share_given_m2),
            format_option_float(row.mean_anomaly_m2_pp_given_active),
        ));
    }
    markdown.push('\n');

    markdown.push_str(&format!("## Top Base {} M=2 Pairs\n\n", target_base));
    markdown.push_str(
        "| Pair | Gap | Best k | Anomaly | Stable-zero delta | Boundary delta | Source | Hinge |\n",
    );
    markdown.push_str("|---|---|---|---:|---:|---:|---|---|\n");
    for row in target_active_rows.iter().take(top_rows) {
        markdown.push_str(&format!(
            "| `{}` | `{}` | `{}` | {:.2}pp | {:.2}pp | {:.2}pp | `{}` | `{}` |\n",
            row.pair_label,
            row.gap_bucket,
            row.best_k_m2,
            row.anomaly_m2_pp,
            row.stable_zero_prime_delta_pp,
            row.boundary_prime_delta_pp,
            row.signal_source_label,
            row.hinge_category,
        ));
    }
    markdown.push('\n');

    markdown.push_str(&format!(
        "## Strongest Shared-Overlap Witnesses In Base {}\n\n",
        target_base
    ));
    markdown.push_str(
        "| Pair | Stable-zero delta | Shared rate delta | Best k | Signal source | Hinge |\n",
    );
    markdown.push_str("|---|---:|---:|---|---|---|\n");
    for row in target_shared_rows.iter().take(top_rows) {
        markdown.push_str(&format!(
            "| `{}` | {:.2}pp | {:.2}pp | `{}` | `{}` | `{}` |\n",
            row.pair_label,
            row.stable_zero_prime_delta_pp,
            row.shared_prime_rate_delta_pp,
            row.best_k_m2,
            row.signal_source_label,
            row.hinge_category,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Observations\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {}\n", observation));
    }
    markdown.push('\n');

    markdown.push_str("## Signal Read\n\n");
    markdown.push_str(&format!(
        "Base `{}` currently looks like `{}` rather than a clean new base-14-style hinge witness. That does not make it empty; it just means the useful signal is more likely in a few local pockets than in the base-wide summary.\n",
        target_base,
        target_base_row.base_label
    ));

    markdown
}

fn anomaly_mass(row: &KDominancePairRow) -> f64 {
    row.best_minus_k00_pp.max(0.0)
}

fn cyclic_unit_distance(base: u32, outer: u32, inner: u32) -> usize {
    let units = unit_residues(base);
    let outer_index = units
        .iter()
        .position(|&digit| digit == outer)
        .expect("outer digit should be a unit");
    let inner_index = units
        .iter()
        .position(|&digit| digit == inner)
        .expect("inner digit should be a unit");
    let direct = outer_index.abs_diff(inner_index);
    direct.min(units.len() - direct)
}

fn gap_bucket(base: u32, outer: u32, inner: u32) -> &'static str {
    match cyclic_unit_distance(base, outer, inner) {
        0 => "same",
        1 => "adjacent",
        _ => "wide",
    }
}

fn hinge_category(m2_persistent: bool, shared_yield_core: bool) -> &'static str {
    match (m2_persistent, shared_yield_core) {
        (true, true) => CATEGORY_PERSISTENT_CORE,
        (true, false) => CATEGORY_PERSISTENCE_ONLY,
        (false, true) => CATEGORY_CORE_ONLY,
        (false, false) => CATEGORY_ACTIVE_NEITHER,
    }
}

fn base_label(
    m2_active_pairs: usize,
    m2_persistent_pairs: usize,
    persistent_core_pairs: usize,
    shared_yield_core_pairs: usize,
) -> &'static str {
    if persistent_core_pairs > 0 {
        "hinge_bridge"
    } else if m2_persistent_pairs > 0 {
        "persistence_only"
    } else if shared_yield_core_pairs > 0 {
        "core_only"
    } else if m2_active_pairs > 0 {
        "active_neither"
    } else {
        "quiet_by_m2"
    }
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

fn hinge_color(category: &str) -> RGBColor {
    match category {
        CATEGORY_PERSISTENT_CORE => RGBColor(31, 119, 180),
        CATEGORY_PERSISTENCE_ONLY => RGBColor(255, 127, 14),
        CATEGORY_CORE_ONLY => RGBColor(44, 160, 44),
        CATEGORY_ACTIVE_NEITHER => RGBColor(148, 103, 189),
        _ => RGBColor(127, 127, 127),
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
        Some(numerator as f64 / denominator as f64)
    }
}

fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

fn format_option_share(value: Option<f64>) -> String {
    value
        .map(|ratio| format!("{:.2}%", ratio * 100.0))
        .unwrap_or_else(|| "-".to_string())
}

fn format_option_float(value: Option<f64>) -> String {
    value
        .map(|number| format!("{number:.2}pp"))
        .unwrap_or_else(|| "-".to_string())
}
