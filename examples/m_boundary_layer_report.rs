//! Boundary-layer report for the bounded-`k` anomaly regime at short middle
//! lengths.
//!
//! This report treats `M=1` and `M=2` as the short-length boundary layer and
//! asks a narrower question:
//! which pair features seem to predict whether an `M=1` anomaly survives one
//! step longer to `M=2`?
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example m_boundary_layer_report
//! cargo run --release --example m_boundary_layer_report -- --full --out-dir /tmp/primes_m_boundary_layer_full
//! ```

use primes::validation::{
    bounded_k::{
        evaluate_pair_row, format_k, ordered_unit_pairs, select_smoke_pairs, unit_residues,
        KDominancePairRow, DEFAULT_BOUNDED_K_GRID,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{collections::BTreeMap, env, path::PathBuf};

const BASES: &[u32] = &[6, 10, 12, 14, 30];
const M1: usize = 1;
const M2: usize = 2;
const M3: usize = 3;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_m_boundary_layer";
const REPORT_EXPORT_VERSION: u32 = 1;
const SMOKE_MAX_ORDERED_PAIRS_PER_BASE: usize = 8;
const SMOKE_PAIR_ANCHORS: &[(u32, u32, u32)] = &[(6, 1, 5), (10, 3, 3), (10, 3, 7), (30, 11, 7)];
const MIN_BUCKET_SUPPORT: usize = 2;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    full_catalog: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    bases: Vec<u32>,
    pair_catalog_mode: String,
    max_ordered_pairs_per_base: Option<usize>,
    middle_lengths: Vec<usize>,
    k_grid: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct BoundaryPairRow {
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    same_digit: bool,
    complement_pair: bool,
    outer_gt_inner: bool,
    unit_index_outer: usize,
    unit_index_inner: usize,
    unit_gap: usize,
    unit_gap_bucket: String,
    best_k_m1: String,
    best_k_m2: String,
    best_k_m3: String,
    anomaly_m1_pp: f64,
    anomaly_m2_pp: f64,
    anomaly_m3_pp: f64,
    m1_anomalous: bool,
    persists_to_m2: bool,
    persists_to_m3: bool,
    retention_share_m2_over_m1: Option<f64>,
    boundary_class: String,
    collapse_estimate: String,
}

#[derive(Debug, Clone, Serialize)]
struct BaseBoundarySummaryRow {
    base: u32,
    total_pairs: usize,
    m1_anomalous_pairs: usize,
    m2_persistent_pairs: usize,
    m3_persistent_pairs: usize,
    m1_anomaly_mass_pp: f64,
    m2_anomaly_mass_pp: f64,
    m3_anomaly_mass_pp: f64,
    retention_share_m2_over_m1: Option<f64>,
    dominant_persistent_pair: String,
    dominant_persistent_best_k: String,
    dominant_persistent_m2_margin_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct BucketSummaryRow {
    bucket_kind: String,
    bucket_value: String,
    total_pairs: usize,
    m1_anomalous_pairs: usize,
    m2_persistent_pairs: usize,
    m3_persistent_pairs: usize,
    persistence_rate_given_m1: Option<f64>,
    median_m1_anomaly_pp: f64,
    median_retention_share_m2_over_m1: Option<f64>,
    dominant_boundary_class: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    total_pairs: usize,
    m1_anomalous_pairs: usize,
    m2_persistent_pairs: usize,
    m3_persistent_pairs: usize,
    m1_anomaly_mass_pp: f64,
    m2_anomaly_mass_pp: f64,
    m3_anomaly_mass_pp: f64,
    retention_share_m2_over_m1: Option<f64>,
    retention_share_m3_over_m1: Option<f64>,
    collapse_by_m2_pairs: usize,
    collapse_by_m3_pairs: usize,
    active_bases_at_m1: String,
    active_bases_at_m2: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    boundary_pair_rows: Vec<BoundaryPairRow>,
    base_summary_rows: Vec<BaseBoundarySummaryRow>,
    bucket_summary_rows: Vec<BucketSummaryRow>,
    report_summary: ReportSummary,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        bases: BASES.to_vec(),
        pair_catalog_mode: if options.full_catalog {
            "full".to_string()
        } else {
            "smoke".to_string()
        },
        max_ordered_pairs_per_base: if options.full_catalog {
            None
        } else {
            Some(SMOKE_MAX_ORDERED_PAIRS_PER_BASE)
        },
        middle_lengths: vec![M1, M2, M3],
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format_k(config))
            .collect(),
    };

    let boundary_pair_rows = build_boundary_pair_rows(options.full_catalog);
    let base_summary_rows = build_base_summary_rows(&boundary_pair_rows);
    let bucket_summary_rows = build_bucket_summary_rows(&boundary_pair_rows);
    let report_summary = build_report_summary(&boundary_pair_rows);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        boundary_pair_rows,
        base_summary_rows,
        bucket_summary_rows,
        report_summary,
    };

    write_csv_rows(
        options.out_dir.join("boundary_pair_rows.csv"),
        &bundle.boundary_pair_rows,
    )
    .expect("failed to write boundary pair rows");
    write_csv_rows(
        options.out_dir.join("base_summary_rows.csv"),
        &bundle.base_summary_rows,
    )
    .expect("failed to write base summary rows");
    write_csv_rows(
        options.out_dir.join("bucket_summary_rows.csv"),
        &bundle.bucket_summary_rows,
    )
    .expect("failed to write bucket summary rows");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary json");
    write_text_file(
        options.out_dir.join("report.md"),
        &render_markdown_report(&bundle),
    )
    .expect("failed to write markdown report");

    print_summary(&bundle);
}

fn parse_args() -> Options {
    let mut args = env::args().skip(1);
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut full_catalog = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir"));
            }
            "--full" => {
                full_catalog = true;
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            other => {
                eprintln!("Unrecognized argument: {other}");
                print_usage();
                std::process::exit(1);
            }
        }
    }

    Options {
        out_dir,
        full_catalog,
    }
}

fn parse_next<T>(args: &mut impl Iterator<Item = String>, flag: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let raw = args.next().unwrap_or_else(|| {
        eprintln!("Missing value for {flag}");
        print_usage();
        std::process::exit(1);
    });
    raw.parse::<T>().unwrap_or_else(|err| {
        eprintln!("Invalid value for {flag}: {err}");
        print_usage();
        std::process::exit(1);
    })
}

fn print_usage() {
    println!("Bounded-k boundary-layer report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example m_boundary_layer_report -- [options]");
    println!();
    println!("Options:");
    println!(
        "  --out-dir <path>          Output directory for artifacts (default: {DEFAULT_OUT_DIR})"
    );
    println!("  --full                    Use the exhaustive ordered-pair catalog instead of the default smoke catalog");
}

fn build_boundary_pair_rows(full_catalog: bool) -> Vec<BoundaryPairRow> {
    let tasks: Vec<_> = BASES
        .iter()
        .copied()
        .flat_map(|base| {
            let pairs = if full_catalog {
                ordered_unit_pairs(base)
            } else {
                let anchors = SMOKE_PAIR_ANCHORS
                    .iter()
                    .filter(|&&(anchor_base, _, _)| anchor_base == base)
                    .map(|&(_, outer, inner)| (outer, inner))
                    .collect::<Vec<_>>();
                select_smoke_pairs(base, SMOKE_MAX_ORDERED_PAIRS_PER_BASE, &anchors)
            };
            pairs
                .into_iter()
                .map(move |(outer, inner)| (base, outer, inner))
        })
        .collect();

    let mut rows: Vec<_> = tasks
        .par_iter()
        .map(|&(base, outer, inner)| build_boundary_pair_row(base, outer, inner))
        .collect();
    rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
    });
    rows
}

fn build_boundary_pair_row(base: u32, outer: u32, inner: u32) -> BoundaryPairRow {
    let row_m1 = evaluate_pair_row(base, M1, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let row_m2 = evaluate_pair_row(base, M2, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let row_m3 = evaluate_pair_row(base, M3, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let units = unit_residues(base);
    let unit_index_outer = units
        .iter()
        .position(|&digit| digit == outer)
        .expect("outer digit should be a unit residue");
    let unit_index_inner = units
        .iter()
        .position(|&digit| digit == inner)
        .expect("inner digit should be a unit residue");
    let unit_gap = unit_index_outer.abs_diff(unit_index_inner);
    let anomaly_m1_pp = anomaly_mass(&row_m1);
    let anomaly_m2_pp = anomaly_mass(&row_m2);
    let anomaly_m3_pp = anomaly_mass(&row_m3);
    let m1_anomalous = anomaly_m1_pp > 0.0;
    let persists_to_m2 = anomaly_m2_pp > 0.0;
    let persists_to_m3 = anomaly_m3_pp > 0.0;

    BoundaryPairRow {
        base,
        outer,
        inner,
        pair_label: row_m1.pair_label.clone(),
        same_digit: outer == inner,
        complement_pair: (outer + inner).is_multiple_of(base),
        outer_gt_inner: outer > inner,
        unit_index_outer,
        unit_index_inner,
        unit_gap,
        unit_gap_bucket: unit_gap_bucket(unit_gap),
        best_k_m1: row_m1.best_k.clone(),
        best_k_m2: row_m2.best_k.clone(),
        best_k_m3: row_m3.best_k.clone(),
        anomaly_m1_pp,
        anomaly_m2_pp,
        anomaly_m3_pp,
        m1_anomalous,
        persists_to_m2,
        persists_to_m3,
        retention_share_m2_over_m1: if anomaly_m1_pp > 0.0 {
            Some(anomaly_m2_pp / anomaly_m1_pp)
        } else {
            None
        },
        boundary_class: boundary_class(m1_anomalous, persists_to_m2, persists_to_m3),
        collapse_estimate: collapse_estimate(m1_anomalous, persists_to_m2, persists_to_m3),
    }
}

fn build_base_summary_rows(rows: &[BoundaryPairRow]) -> Vec<BaseBoundarySummaryRow> {
    let mut by_base: BTreeMap<u32, Vec<&BoundaryPairRow>> = BTreeMap::new();
    for row in rows {
        by_base.entry(row.base).or_default().push(row);
    }

    by_base
        .into_iter()
        .map(|(base, group_rows)| {
            let total_pairs = group_rows.len();
            let m1_anomalous_pairs = group_rows.iter().filter(|row| row.m1_anomalous).count();
            let m2_persistent_pairs = group_rows.iter().filter(|row| row.persists_to_m2).count();
            let m3_persistent_pairs = group_rows.iter().filter(|row| row.persists_to_m3).count();
            let m1_anomaly_mass_pp = positive_sum(group_rows.iter().map(|row| row.anomaly_m1_pp));
            let m2_anomaly_mass_pp = positive_sum(group_rows.iter().map(|row| row.anomaly_m2_pp));
            let m3_anomaly_mass_pp = positive_sum(group_rows.iter().map(|row| row.anomaly_m3_pp));
            let dominant_persistent =
                group_rows
                    .iter()
                    .filter(|row| row.persists_to_m2)
                    .max_by(|left, right| {
                        left.anomaly_m2_pp
                            .total_cmp(&right.anomaly_m2_pp)
                            .then_with(|| left.pair_label.cmp(&right.pair_label))
                    });

            BaseBoundarySummaryRow {
                base,
                total_pairs,
                m1_anomalous_pairs,
                m2_persistent_pairs,
                m3_persistent_pairs,
                m1_anomaly_mass_pp,
                m2_anomaly_mass_pp,
                m3_anomaly_mass_pp,
                retention_share_m2_over_m1: if m1_anomaly_mass_pp > 0.0 {
                    Some(m2_anomaly_mass_pp / m1_anomaly_mass_pp)
                } else {
                    None
                },
                dominant_persistent_pair: dominant_persistent
                    .map(|row| row.pair_label.clone())
                    .unwrap_or_else(|| "none".to_string()),
                dominant_persistent_best_k: dominant_persistent
                    .map(|row| row.best_k_m2.clone())
                    .unwrap_or_else(|| "none".to_string()),
                dominant_persistent_m2_margin_pp: dominant_persistent
                    .map(|row| row.anomaly_m2_pp)
                    .unwrap_or(0.0),
            }
        })
        .collect()
}

fn build_bucket_summary_rows(rows: &[BoundaryPairRow]) -> Vec<BucketSummaryRow> {
    let mut buckets: BTreeMap<(String, String), Vec<&BoundaryPairRow>> = BTreeMap::new();
    for row in rows {
        add_bucket(&mut buckets, "base", &row.base.to_string(), row);
        add_bucket(
            &mut buckets,
            "same_digit",
            if row.same_digit { "true" } else { "false" },
            row,
        );
        add_bucket(
            &mut buckets,
            "complement_pair",
            if row.complement_pair { "true" } else { "false" },
            row,
        );
        add_bucket(
            &mut buckets,
            "outer_gt_inner",
            if row.outer_gt_inner { "true" } else { "false" },
            row,
        );
        add_bucket(&mut buckets, "unit_gap_bucket", &row.unit_gap_bucket, row);
        add_bucket(&mut buckets, "best_k_m1", &row.best_k_m1, row);
        add_bucket(
            &mut buckets,
            "base_x_unit_gap_bucket",
            &format!("{}:{}", row.base, row.unit_gap_bucket),
            row,
        );
        add_bucket(
            &mut buckets,
            "base_x_same_digit",
            &format!("{}:{}", row.base, row.same_digit),
            row,
        );
        add_bucket(
            &mut buckets,
            "same_digit_x_unit_gap_bucket",
            &format!("{}:{}", row.same_digit, row.unit_gap_bucket),
            row,
        );
    }

    let mut summary_rows: Vec<_> = buckets
        .into_iter()
        .map(|((bucket_kind, bucket_value), group_rows)| {
            let total_pairs = group_rows.len();
            let m1_rows = group_rows
                .iter()
                .filter(|row| row.m1_anomalous)
                .copied()
                .collect::<Vec<_>>();
            let m1_anomalous_pairs = m1_rows.len();
            let m2_persistent_pairs = m1_rows.iter().filter(|row| row.persists_to_m2).count();
            let m3_persistent_pairs = m1_rows.iter().filter(|row| row.persists_to_m3).count();

            BucketSummaryRow {
                bucket_kind,
                bucket_value,
                total_pairs,
                m1_anomalous_pairs,
                m2_persistent_pairs,
                m3_persistent_pairs,
                persistence_rate_given_m1: if m1_anomalous_pairs > 0 {
                    Some(m2_persistent_pairs as f64 / m1_anomalous_pairs as f64)
                } else {
                    None
                },
                median_m1_anomaly_pp: median(m1_rows.iter().map(|row| row.anomaly_m1_pp).collect()),
                median_retention_share_m2_over_m1: median_option(
                    m1_rows
                        .iter()
                        .filter_map(|row| row.retention_share_m2_over_m1)
                        .collect(),
                ),
                dominant_boundary_class: dominant_boundary_class(&m1_rows),
            }
        })
        .collect();

    summary_rows.sort_by(|left, right| {
        right
            .persistence_rate_given_m1
            .unwrap_or(-1.0)
            .total_cmp(&left.persistence_rate_given_m1.unwrap_or(-1.0))
            .then_with(|| right.m1_anomalous_pairs.cmp(&left.m1_anomalous_pairs))
            .then_with(|| left.bucket_kind.cmp(&right.bucket_kind))
            .then_with(|| left.bucket_value.cmp(&right.bucket_value))
    });
    summary_rows
}

fn add_bucket<'a>(
    buckets: &mut BTreeMap<(String, String), Vec<&'a BoundaryPairRow>>,
    kind: &str,
    value: &str,
    row: &'a BoundaryPairRow,
) {
    buckets
        .entry((kind.to_string(), value.to_string()))
        .or_default()
        .push(row);
}

fn build_report_summary(rows: &[BoundaryPairRow]) -> ReportSummary {
    let m1_anomalous_pairs = rows.iter().filter(|row| row.m1_anomalous).count();
    let m2_persistent_pairs = rows.iter().filter(|row| row.persists_to_m2).count();
    let m3_persistent_pairs = rows.iter().filter(|row| row.persists_to_m3).count();
    let m1_anomaly_mass_pp = positive_sum(rows.iter().map(|row| row.anomaly_m1_pp));
    let m2_anomaly_mass_pp = positive_sum(rows.iter().map(|row| row.anomaly_m2_pp));
    let m3_anomaly_mass_pp = positive_sum(rows.iter().map(|row| row.anomaly_m3_pp));

    ReportSummary {
        total_pairs: rows.len(),
        m1_anomalous_pairs,
        m2_persistent_pairs,
        m3_persistent_pairs,
        m1_anomaly_mass_pp,
        m2_anomaly_mass_pp,
        m3_anomaly_mass_pp,
        retention_share_m2_over_m1: if m1_anomaly_mass_pp > 0.0 {
            Some(m2_anomaly_mass_pp / m1_anomaly_mass_pp)
        } else {
            None
        },
        retention_share_m3_over_m1: if m1_anomaly_mass_pp > 0.0 {
            Some(m3_anomaly_mass_pp / m1_anomaly_mass_pp)
        } else {
            None
        },
        collapse_by_m2_pairs: rows
            .iter()
            .filter(|row| row.boundary_class == "m1_only")
            .count(),
        collapse_by_m3_pairs: rows
            .iter()
            .filter(|row| row.boundary_class == "m1_to_m2")
            .count(),
        active_bases_at_m1: join_u32s(unique_sorted(
            rows.iter()
                .filter(|row| row.m1_anomalous)
                .map(|row| row.base)
                .collect(),
        )),
        active_bases_at_m2: join_u32s(unique_sorted(
            rows.iter()
                .filter(|row| row.persists_to_m2)
                .map(|row| row.base)
                .collect(),
        )),
    }
}

fn anomaly_mass(row: &KDominancePairRow) -> f64 {
    if row.best_minus_k00_pp > 0.0 {
        row.best_minus_k00_pp
    } else {
        0.0
    }
}

fn unit_gap_bucket(unit_gap: usize) -> String {
    match unit_gap {
        0 => "same".to_string(),
        1 => "adjacent".to_string(),
        _ => "wide".to_string(),
    }
}

fn boundary_class(m1: bool, m2: bool, m3: bool) -> String {
    match (m1, m2, m3) {
        (false, false, false) => "never_anomalous",
        (true, false, false) => "m1_only",
        (true, true, false) => "m1_to_m2",
        (true, true, true) => "m1_to_m2_to_m3",
        (false, true, false) => "m2_only",
        (false, true, true) => "m2_to_m3",
        (false, false, true) => "m3_only",
        (true, false, true) => "non_monotone",
    }
    .to_string()
}

fn collapse_estimate(m1: bool, m2: bool, m3: bool) -> String {
    match (m1, m2, m3) {
        (false, false, false) => "no anomaly in boundary layer",
        (true, false, false) => "collapsed by M=2",
        (true, true, false) => "collapsed by M=3",
        (true, true, true) => "still anomalous at M=3",
        (false, true, false) => "emerges at M=2, collapses by M=3",
        (false, true, true) => "emerges at M=2, persists at M=3",
        (false, false, true) => "emerges at M=3",
        (true, false, true) => "non-monotone boundary pattern",
    }
    .to_string()
}

fn dominant_boundary_class(rows: &[&BoundaryPairRow]) -> String {
    if rows.is_empty() {
        return "none".to_string();
    }
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for row in rows {
        *counts.entry(&row.boundary_class).or_default() += 1;
    }
    counts
        .into_iter()
        .max_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(right.0)))
        .map(|(label, _)| label.to_string())
        .unwrap_or_else(|| "none".to_string())
}

fn positive_sum(values: impl Iterator<Item = f64>) -> f64 {
    let sum: f64 = values.sum();
    if sum > 0.0 {
        sum
    } else {
        0.0
    }
}

fn median(mut values: Vec<f64>) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(|left, right| left.total_cmp(right));
    let middle = values.len() / 2;
    if values.len() % 2 == 1 {
        values[middle]
    } else {
        (values[middle - 1] + values[middle]) / 2.0
    }
}

fn median_option(values: Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(median(values))
    }
}

fn unique_sorted(mut values: Vec<u32>) -> Vec<u32> {
    values.sort_unstable();
    values.dedup();
    values
}

fn join_u32s(values: Vec<u32>) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn format_share(value: Option<f64>) -> String {
    value
        .map(|share| format!("{:.1}%", share * 100.0))
        .unwrap_or_else(|| "n/a".to_string())
}

fn print_summary(bundle: &ReportBundle) {
    println!("=== M Boundary Layer Report ===\n");
    println!(
        "Pair catalog: {} | bases {:?} | output {}",
        bundle.settings.pair_catalog_mode, bundle.settings.bases, bundle.settings.out_dir
    );
    println!();
    println!(
        "Boundary anomalies: M=1 {}/{} pairs, M=2 {}/{} pairs, M=3 {}/{} pairs",
        bundle.report_summary.m1_anomalous_pairs,
        bundle.report_summary.total_pairs,
        bundle.report_summary.m2_persistent_pairs,
        bundle.report_summary.total_pairs,
        bundle.report_summary.m3_persistent_pairs,
        bundle.report_summary.total_pairs
    );
    println!(
        "Anomaly mass retention: M=1 {:.2}pp -> M=2 {:.2}pp ({}) -> M=3 {:.2}pp ({})",
        bundle.report_summary.m1_anomaly_mass_pp,
        bundle.report_summary.m2_anomaly_mass_pp,
        format_share(bundle.report_summary.retention_share_m2_over_m1),
        bundle.report_summary.m3_anomaly_mass_pp,
        format_share(bundle.report_summary.retention_share_m3_over_m1)
    );
    println!(
        "Collapse counts: by M=2 {} | by M=3 {}",
        bundle.report_summary.collapse_by_m2_pairs, bundle.report_summary.collapse_by_m3_pairs
    );
    println!(
        "Active bases: M=1 {} | M=2 {}",
        bundle.report_summary.active_bases_at_m1, bundle.report_summary.active_bases_at_m2
    );
}

fn render_markdown_report(bundle: &ReportBundle) -> String {
    let mut predictor_rows = bundle
        .bucket_summary_rows
        .iter()
        .filter(|row| row.m1_anomalous_pairs >= MIN_BUCKET_SUPPORT)
        .collect::<Vec<_>>();
    predictor_rows.sort_by(|left, right| {
        right
            .persistence_rate_given_m1
            .unwrap_or(-1.0)
            .total_cmp(&left.persistence_rate_given_m1.unwrap_or(-1.0))
            .then_with(|| right.m1_anomalous_pairs.cmp(&left.m1_anomalous_pairs))
            .then_with(|| left.bucket_kind.cmp(&right.bucket_kind))
            .then_with(|| left.bucket_value.cmp(&right.bucket_value))
    });
    predictor_rows.truncate(12);

    let mut anomaly_rows = bundle
        .boundary_pair_rows
        .iter()
        .filter(|row| row.m1_anomalous)
        .collect::<Vec<_>>();
    anomaly_rows.sort_by(|left, right| {
        right
            .anomaly_m1_pp
            .total_cmp(&left.anomaly_m1_pp)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });

    let mut lines = vec![
        "# M Boundary Layer Report".to_string(),
        String::new(),
        "_Generated from `examples/m_boundary_layer_report.rs`._".to_string(),
        String::new(),
        format!("- Generated at: `{}`", bundle.generated_at_utc),
        format!("- Bases: `{:?}`", bundle.settings.bases),
        format!("- Pair catalog: `{}`", bundle.settings.pair_catalog_mode),
        format!("- Boundary lengths: `{:?}`", bundle.settings.middle_lengths),
        format!("- Bounded k-grid: `{:?}`", bundle.settings.k_grid),
        String::new(),
        "## Overall".to_string(),
        String::new(),
        format!(
            "- M=1 anomalous pairs: `{}/{}`
- M=2 persisting pairs: `{}/{}`
- M=3 persisting pairs: `{}/{}`
- Anomaly mass retention: `M=1 {:.2}pp -> M=2 {:.2}pp ({}) -> M=3 {:.2}pp ({})`
- Collapse counts: `by M=2 {}` and `by M=3 {}`
- Active bases: `M=1 {}` and `M=2 {}`",
            bundle.report_summary.m1_anomalous_pairs,
            bundle.report_summary.total_pairs,
            bundle.report_summary.m2_persistent_pairs,
            bundle.report_summary.total_pairs,
            bundle.report_summary.m3_persistent_pairs,
            bundle.report_summary.total_pairs,
            bundle.report_summary.m1_anomaly_mass_pp,
            bundle.report_summary.m2_anomaly_mass_pp,
            format_share(bundle.report_summary.retention_share_m2_over_m1),
            bundle.report_summary.m3_anomaly_mass_pp,
            format_share(bundle.report_summary.retention_share_m3_over_m1),
            bundle.report_summary.collapse_by_m2_pairs,
            bundle.report_summary.collapse_by_m3_pairs,
            bundle.report_summary.active_bases_at_m1,
            bundle.report_summary.active_bases_at_m2
        ),
        String::new(),
        "## Base Rows".to_string(),
        String::new(),
        "| Base | M=1 anomalous pairs | M=2 persisting pairs | M=1 mass | M=2 mass | Retention | Dominant persister |".to_string(),
        "|---:|---:|---:|---:|---:|---:|---|".to_string(),
    ];

    for row in &bundle.base_summary_rows {
        lines.push(format!(
            "| `{}` | `{}/{} ({:.1}%)` | `{}/{} ({:.1}%)` | `{:.2}pp` | `{:.2}pp` | `{}` | `{}` via `{}` (`{:.2}pp`) |",
            row.base,
            row.m1_anomalous_pairs,
            row.total_pairs,
            row.m1_anomalous_pairs as f64 * 100.0 / row.total_pairs as f64,
            row.m2_persistent_pairs,
            row.total_pairs,
            row.m2_persistent_pairs as f64 * 100.0 / row.total_pairs as f64,
            row.m1_anomaly_mass_pp,
            row.m2_anomaly_mass_pp,
            format_share(row.retention_share_m2_over_m1),
            row.dominant_persistent_pair,
            row.dominant_persistent_best_k,
            row.dominant_persistent_m2_margin_pp
        ));
    }

    if !predictor_rows.is_empty() {
        lines.extend([
            String::new(),
            "## Candidate Predictors".to_string(),
            String::new(),
            "| Bucket | M=1 anomalies | Persist to M=2 | Persistence rate | Median M=1 mass | Median retention | Dominant class |".to_string(),
            "|---|---:|---:|---:|---:|---:|---|".to_string(),
        ]);
        for row in predictor_rows {
            lines.push(format!(
                "| `{}` = `{}` | `{}` | `{}` | `{}` | `{:.2}pp` | `{}` | `{}` |",
                row.bucket_kind,
                row.bucket_value,
                row.m1_anomalous_pairs,
                row.m2_persistent_pairs,
                format_share(row.persistence_rate_given_m1),
                row.median_m1_anomaly_pp,
                format_share(row.median_retention_share_m2_over_m1),
                row.dominant_boundary_class
            ));
        }
    }

    if !anomaly_rows.is_empty() {
        lines.extend([
            String::new(),
            "## Boundary Pairs".to_string(),
            String::new(),
            "| Base | Pair | Gap bucket | M1 best k | M1 mass | M2 mass | Class | Collapse |"
                .to_string(),
            "|---:|---|---|---|---:|---:|---|---|".to_string(),
        ]);
        for row in anomaly_rows {
            lines.push(format!(
                "| `{}` | {} | `{}` | `{}` | `{:.2}pp` | `{:.2}pp` | `{}` | `{}` |",
                row.base,
                row.pair_label,
                row.unit_gap_bucket,
                row.best_k_m1,
                row.anomaly_m1_pp,
                row.anomaly_m2_pp,
                row.boundary_class,
                row.collapse_estimate
            ));
        }
    }

    lines.join("\n")
}
