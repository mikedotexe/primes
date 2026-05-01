//! Bounded-`k` transition-curve report across a short range of middle lengths.
//!
//! This report extends the `M=2 -> M=3` transition audit into a compact curve:
//! it evaluates the same ordered unit-residue pairs across `M=min..=max`,
//! tracks the positive anomaly mass `max(best_minus_k00_pp, 0)`, and estimates
//! where that mass disappears.
//!
//! The default range stops at `M=4` because exact scans become expensive in
//! higher bases once `M >= 5`.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example m_transition_curve_report
//! cargo run --release --example m_transition_curve_report -- --full --min-middle-length 1 --max-middle-length 4 --out-dir /tmp/primes_m_transition_curve_full
//! ```

use primes::validation::{
    bounded_k::{
        evaluate_pair_row, format_k, ordered_unit_pairs, select_smoke_pairs, summarize_pair_rows,
        KDominancePairRow, KDominanceSummaryRow, DEFAULT_BOUNDED_K_GRID,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{collections::BTreeMap, env, path::PathBuf};

const BASES: &[u32] = &[6, 10, 12, 14, 30];
const DEFAULT_MIN_MIDDLE_LENGTH: usize = 1;
const DEFAULT_MAX_MIDDLE_LENGTH: usize = 4;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_m_transition_curve";
const REPORT_EXPORT_VERSION: u32 = 1;
const SMOKE_MAX_ORDERED_PAIRS_PER_BASE: usize = 8;
const SMOKE_PAIR_ANCHORS: &[(u32, u32, u32)] = &[(6, 1, 5), (10, 3, 3), (10, 3, 7), (30, 11, 7)];

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    full_catalog: bool,
    min_middle_length: usize,
    max_middle_length: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    bases: Vec<u32>,
    pair_catalog_mode: String,
    max_ordered_pairs_per_base: Option<usize>,
    min_middle_length: usize,
    max_middle_length: usize,
    k_grid: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct BaseLengthRow {
    base: u32,
    middle_length: usize,
    ordered_pair_count: usize,
    counterexample_pairs: usize,
    counterexample_share: f64,
    anomaly_mass_pp: f64,
    k00_noninferior_share: f64,
    strongest_counterexample_pair: String,
    strongest_counterexample_best_k: String,
    strongest_counterexample_margin_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct GlobalCurveRow {
    middle_length: usize,
    ordered_pair_count: usize,
    counterexample_pairs: usize,
    counterexample_share: f64,
    anomaly_mass_pp: f64,
    k00_noninferior_share: f64,
    active_bases: String,
    leading_pair: String,
    leading_best_k: String,
    leading_margin_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct PairCurveSummaryRow {
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    anomaly_count: usize,
    anomaly_lengths: String,
    first_anomalous_length: Option<usize>,
    last_anomalous_length: Option<usize>,
    collapse_length: Option<usize>,
    collapse_estimate: String,
    max_anomaly_length: Option<usize>,
    max_anomaly_mass_pp: f64,
    total_anomaly_mass_pp: f64,
    dominant_counterexample_k: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    total_pairs: usize,
    total_pair_length_rows: usize,
    middle_lengths: Vec<usize>,
    lengths_with_positive_anomaly_mass: Vec<usize>,
    first_global_zero_after_positive: Option<usize>,
    all_bases_zero_from_length: Option<usize>,
    pairs_with_any_anomaly: usize,
    pairs_with_persistent_anomaly_at_max_length: usize,
    bases_with_any_anomaly: Vec<u32>,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    pair_length_rows: Vec<KDominancePairRow>,
    length_summary_rows: Vec<KDominanceSummaryRow>,
    base_length_rows: Vec<BaseLengthRow>,
    global_curve_rows: Vec<GlobalCurveRow>,
    pair_curve_summary_rows: Vec<PairCurveSummaryRow>,
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
        min_middle_length: options.min_middle_length,
        max_middle_length: options.max_middle_length,
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format_k(config))
            .collect(),
    };

    let pair_length_rows = build_pair_length_rows(
        options.full_catalog,
        options.min_middle_length,
        options.max_middle_length,
    );
    let length_summary_rows = summarize_pair_rows(&pair_length_rows);
    let base_length_rows = build_base_length_rows(&pair_length_rows, &length_summary_rows);
    let global_curve_rows = build_global_curve_rows(&pair_length_rows, &length_summary_rows);
    let pair_curve_summary_rows = build_pair_curve_summary_rows(
        &pair_length_rows,
        options.min_middle_length,
        options.max_middle_length,
    );
    let report_summary = build_report_summary(
        &pair_length_rows,
        &base_length_rows,
        &global_curve_rows,
        &pair_curve_summary_rows,
        options.min_middle_length,
        options.max_middle_length,
    );

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        pair_length_rows,
        length_summary_rows,
        base_length_rows,
        global_curve_rows,
        pair_curve_summary_rows,
        report_summary,
    };

    write_csv_rows(
        options.out_dir.join("pair_length_rows.csv"),
        &bundle.pair_length_rows,
    )
    .expect("failed to write pair length rows");
    write_csv_rows(
        options.out_dir.join("length_summary_rows.csv"),
        &bundle.length_summary_rows,
    )
    .expect("failed to write length summary rows");
    write_csv_rows(
        options.out_dir.join("base_length_rows.csv"),
        &bundle.base_length_rows,
    )
    .expect("failed to write base length rows");
    write_csv_rows(
        options.out_dir.join("global_curve_rows.csv"),
        &bundle.global_curve_rows,
    )
    .expect("failed to write global curve rows");
    write_csv_rows(
        options.out_dir.join("pair_curve_summary_rows.csv"),
        &bundle.pair_curve_summary_rows,
    )
    .expect("failed to write pair curve summary rows");
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
    let mut min_middle_length = DEFAULT_MIN_MIDDLE_LENGTH;
    let mut max_middle_length = DEFAULT_MAX_MIDDLE_LENGTH;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir"));
            }
            "--full" => {
                full_catalog = true;
            }
            "--min-middle-length" => {
                min_middle_length = parse_next::<usize>(&mut args, "--min-middle-length");
            }
            "--max-middle-length" => {
                max_middle_length = parse_next::<usize>(&mut args, "--max-middle-length");
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

    if min_middle_length == 0 || max_middle_length == 0 {
        eprintln!("Middle lengths must be positive");
        std::process::exit(1);
    }
    if min_middle_length > max_middle_length {
        eprintln!("Expected --min-middle-length <= --max-middle-length");
        std::process::exit(1);
    }

    Options {
        out_dir,
        full_catalog,
        min_middle_length,
        max_middle_length,
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
    println!("Bounded-k transition curve report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example m_transition_curve_report -- [options]");
    println!();
    println!("Options:");
    println!(
        "  --out-dir <path>              Output directory for artifacts (default: {DEFAULT_OUT_DIR})"
    );
    println!("  --full                        Use the exhaustive ordered-pair catalog instead of the default smoke catalog");
    println!(
        "  --min-middle-length <n>       Minimum middle length to scan (default: {DEFAULT_MIN_MIDDLE_LENGTH})"
    );
    println!(
        "  --max-middle-length <n>       Maximum middle length to scan (default: {DEFAULT_MAX_MIDDLE_LENGTH})"
    );
}

fn build_pair_length_rows(
    full_catalog: bool,
    min_middle_length: usize,
    max_middle_length: usize,
) -> Vec<KDominancePairRow> {
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
            pairs.into_iter().flat_map(move |(outer, inner)| {
                (min_middle_length..=max_middle_length)
                    .map(move |middle_length| (base, middle_length, outer, inner))
            })
        })
        .collect();

    let mut rows: Vec<_> = tasks
        .par_iter()
        .map(|&(base, middle_length, outer, inner)| {
            evaluate_pair_row(base, middle_length, outer, inner, DEFAULT_BOUNDED_K_GRID)
        })
        .collect();
    rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.middle_length.cmp(&right.middle_length))
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
    });
    rows
}

fn build_base_length_rows(
    pair_rows: &[KDominancePairRow],
    summary_rows: &[KDominanceSummaryRow],
) -> Vec<BaseLengthRow> {
    let summary_by_group = summary_rows
        .iter()
        .map(|row| ((row.base, row.middle_length), row))
        .collect::<BTreeMap<_, _>>();

    let mut by_group: BTreeMap<(u32, usize), Vec<&KDominancePairRow>> = BTreeMap::new();
    for row in pair_rows {
        by_group
            .entry((row.base, row.middle_length))
            .or_default()
            .push(row);
    }

    by_group
        .into_iter()
        .map(|((base, middle_length), rows)| {
            let summary = summary_by_group[&(base, middle_length)];
            let counterexample_pairs = rows
                .iter()
                .filter(|row| row.best_minus_k00_pp > 0.0)
                .count();
            let anomaly_mass_pp = positive_anomaly(
                rows.iter()
                    .map(|row| positive_anomaly(row.best_minus_k00_pp))
                    .sum::<f64>(),
            );

            BaseLengthRow {
                base,
                middle_length,
                ordered_pair_count: rows.len(),
                counterexample_pairs,
                counterexample_share: counterexample_pairs as f64 / rows.len() as f64,
                anomaly_mass_pp,
                k00_noninferior_share: summary.k00_noninferior_share,
                strongest_counterexample_pair: summary.strongest_counterexample_pair.clone(),
                strongest_counterexample_best_k: summary.strongest_counterexample_best_k.clone(),
                strongest_counterexample_margin_pp: summary.strongest_counterexample_margin_pp,
            }
        })
        .collect()
}

fn build_global_curve_rows(
    pair_rows: &[KDominancePairRow],
    summary_rows: &[KDominanceSummaryRow],
) -> Vec<GlobalCurveRow> {
    let mut by_length: BTreeMap<usize, Vec<&KDominancePairRow>> = BTreeMap::new();
    for row in pair_rows {
        by_length.entry(row.middle_length).or_default().push(row);
    }

    let mut summary_by_length: BTreeMap<usize, Vec<&KDominanceSummaryRow>> = BTreeMap::new();
    for row in summary_rows {
        summary_by_length
            .entry(row.middle_length)
            .or_default()
            .push(row);
    }

    by_length
        .into_iter()
        .map(|(middle_length, rows)| {
            let counterexample_pairs = rows
                .iter()
                .filter(|row| row.best_minus_k00_pp > 0.0)
                .count();
            let anomaly_mass_pp = positive_anomaly(
                rows.iter()
                    .map(|row| positive_anomaly(row.best_minus_k00_pp))
                    .sum::<f64>(),
            );
            let active_bases = rows
                .iter()
                .filter(|row| row.best_minus_k00_pp > 0.0)
                .map(|row| row.base)
                .collect::<Vec<_>>();
            let leading_row = rows
                .iter()
                .max_by(|left, right| {
                    left.best_minus_k00_pp
                        .total_cmp(&right.best_minus_k00_pp)
                        .then_with(|| left.base.cmp(&right.base))
                        .then_with(|| left.pair_label.cmp(&right.pair_label))
                })
                .expect("each length should have at least one pair row");
            let k00_noninferior_share = weighted_average(
                summary_by_length
                    .remove(&middle_length)
                    .expect("summary rows should exist for every length")
                    .into_iter()
                    .map(|row| (row.k00_noninferior_share, row.ordered_pair_count)),
            );

            GlobalCurveRow {
                middle_length,
                ordered_pair_count: rows.len(),
                counterexample_pairs,
                counterexample_share: counterexample_pairs as f64 / rows.len() as f64,
                anomaly_mass_pp,
                k00_noninferior_share,
                active_bases: join_u32s(unique_sorted(active_bases)),
                leading_pair: format!("base {} {}", leading_row.base, leading_row.pair_label),
                leading_best_k: leading_row.best_k.clone(),
                leading_margin_pp: leading_row.best_minus_k00_pp,
            }
        })
        .collect()
}

fn build_pair_curve_summary_rows(
    pair_rows: &[KDominancePairRow],
    min_middle_length: usize,
    max_middle_length: usize,
) -> Vec<PairCurveSummaryRow> {
    let mut by_pair: BTreeMap<(u32, u32, u32), Vec<&KDominancePairRow>> = BTreeMap::new();
    for row in pair_rows {
        by_pair
            .entry((row.base, row.outer, row.inner))
            .or_default()
            .push(row);
    }

    by_pair
        .into_iter()
        .map(|((base, outer, inner), mut rows)| {
            rows.sort_by(|left, right| left.middle_length.cmp(&right.middle_length));
            let anomaly_rows = rows
                .iter()
                .filter(|row| row.best_minus_k00_pp > 0.0)
                .copied()
                .collect::<Vec<_>>();
            let anomaly_lengths = anomaly_rows
                .iter()
                .map(|row| row.middle_length)
                .collect::<Vec<_>>();
            let max_anomaly_row = anomaly_rows
                .iter()
                .max_by(|left, right| {
                    left.best_minus_k00_pp
                        .total_cmp(&right.best_minus_k00_pp)
                        .then_with(|| left.middle_length.cmp(&right.middle_length))
                })
                .copied();

            PairCurveSummaryRow {
                base,
                outer,
                inner,
                pair_label: rows[0].pair_label.clone(),
                anomaly_count: anomaly_rows.len(),
                anomaly_lengths: join_usizes(anomaly_lengths.clone()),
                first_anomalous_length: anomaly_lengths.first().copied(),
                last_anomalous_length: anomaly_lengths.last().copied(),
                collapse_length: first_zero_after_last_positive(
                    rows.iter()
                        .map(|row| (row.middle_length, positive_anomaly(row.best_minus_k00_pp)))
                        .collect(),
                    min_middle_length,
                    max_middle_length,
                ),
                collapse_estimate: collapse_estimate(
                    rows.iter()
                        .map(|row| (row.middle_length, positive_anomaly(row.best_minus_k00_pp)))
                        .collect(),
                    min_middle_length,
                    max_middle_length,
                ),
                max_anomaly_length: max_anomaly_row.map(|row| row.middle_length),
                max_anomaly_mass_pp: max_anomaly_row
                    .map(|row| positive_anomaly(row.best_minus_k00_pp))
                    .unwrap_or(0.0),
                total_anomaly_mass_pp: positive_anomaly(
                    anomaly_rows
                        .iter()
                        .map(|row| positive_anomaly(row.best_minus_k00_pp))
                        .sum(),
                ),
                dominant_counterexample_k: dominant_counterexample_k(&anomaly_rows),
            }
        })
        .collect()
}

fn build_report_summary(
    pair_rows: &[KDominancePairRow],
    base_length_rows: &[BaseLengthRow],
    global_curve_rows: &[GlobalCurveRow],
    pair_curve_summary_rows: &[PairCurveSummaryRow],
    min_middle_length: usize,
    max_middle_length: usize,
) -> ReportSummary {
    let lengths_with_positive_anomaly_mass = global_curve_rows
        .iter()
        .filter(|row| row.anomaly_mass_pp > 0.0)
        .map(|row| row.middle_length)
        .collect::<Vec<_>>();

    let all_bases_zero_from_length =
        first_length_all_base_rows_zero(base_length_rows, min_middle_length, max_middle_length);

    ReportSummary {
        total_pairs: pair_curve_summary_rows.len(),
        total_pair_length_rows: pair_rows.len(),
        middle_lengths: (min_middle_length..=max_middle_length).collect(),
        lengths_with_positive_anomaly_mass: lengths_with_positive_anomaly_mass.clone(),
        first_global_zero_after_positive: first_zero_after_last_positive(
            global_curve_rows
                .iter()
                .map(|row| (row.middle_length, row.anomaly_mass_pp))
                .collect(),
            min_middle_length,
            max_middle_length,
        ),
        all_bases_zero_from_length,
        pairs_with_any_anomaly: pair_curve_summary_rows
            .iter()
            .filter(|row| row.anomaly_count > 0)
            .count(),
        pairs_with_persistent_anomaly_at_max_length: pair_curve_summary_rows
            .iter()
            .filter(|row| row.last_anomalous_length == Some(max_middle_length))
            .count(),
        bases_with_any_anomaly: unique_sorted(
            base_length_rows
                .iter()
                .filter(|row| row.anomaly_mass_pp > 0.0)
                .map(|row| row.base)
                .collect(),
        ),
    }
}

fn weighted_average(values: impl Iterator<Item = (f64, usize)>) -> f64 {
    let mut weighted_sum = 0.0;
    let mut total_weight = 0usize;
    for (value, weight) in values {
        weighted_sum += value * weight as f64;
        total_weight += weight;
    }
    if total_weight == 0 {
        0.0
    } else {
        weighted_sum / total_weight as f64
    }
}

fn dominant_counterexample_k(rows: &[&KDominancePairRow]) -> String {
    if rows.is_empty() {
        return "none".to_string();
    }
    let mut by_k: BTreeMap<String, f64> = BTreeMap::new();
    for row in rows {
        *by_k.entry(row.best_k.clone()).or_default() += positive_anomaly(row.best_minus_k00_pp);
    }
    by_k.into_iter()
        .max_by(|left, right| {
            left.1
                .total_cmp(&right.1)
                .then_with(|| left.0.cmp(&right.0))
        })
        .map(|(label, _)| label)
        .unwrap_or_else(|| "none".to_string())
}

fn first_zero_after_last_positive(
    values: Vec<(usize, f64)>,
    min_middle_length: usize,
    max_middle_length: usize,
) -> Option<usize> {
    let by_length = values.into_iter().collect::<BTreeMap<_, _>>();
    find_collapse_length(&by_length, min_middle_length, max_middle_length)
}

fn find_collapse_length(
    by_length: &BTreeMap<usize, f64>,
    min_middle_length: usize,
    max_middle_length: usize,
) -> Option<usize> {
    let mut seen_positive = false;
    for middle_length in min_middle_length..=max_middle_length {
        let value = *by_length.get(&middle_length).unwrap_or(&0.0);
        if value > 0.0 {
            seen_positive = true;
            continue;
        }
        if seen_positive {
            let future_positive = ((middle_length + 1)..=max_middle_length)
                .any(|future| by_length.get(&future).copied().unwrap_or(0.0) > 0.0);
            if !future_positive {
                return Some(middle_length);
            }
        }
    }
    None
}

fn first_length_all_base_rows_zero(
    base_rows: &[BaseLengthRow],
    min_middle_length: usize,
    max_middle_length: usize,
) -> Option<usize> {
    let by_length = base_rows.iter().fold(
        BTreeMap::<usize, Vec<&BaseLengthRow>>::new(),
        |mut acc, row| {
            acc.entry(row.middle_length).or_default().push(row);
            acc
        },
    );

    let mut seen_positive = false;
    for middle_length in min_middle_length..=max_middle_length {
        let rows = by_length
            .get(&middle_length)
            .expect("base rows should exist for every scanned length");
        let any_positive_here = rows.iter().any(|row| row.anomaly_mass_pp > 0.0);
        if any_positive_here {
            seen_positive = true;
            continue;
        }
        if seen_positive {
            let future_positive = ((middle_length + 1)..=max_middle_length).any(|future| {
                by_length
                    .get(&future)
                    .expect("future base rows should exist")
                    .iter()
                    .any(|row| row.anomaly_mass_pp > 0.0)
            });
            if !future_positive {
                return Some(middle_length);
            }
        }
    }
    None
}

fn collapse_estimate(
    values: Vec<(usize, f64)>,
    min_middle_length: usize,
    max_middle_length: usize,
) -> String {
    let by_length = values.into_iter().collect::<BTreeMap<_, _>>();
    let positive_lengths = (min_middle_length..=max_middle_length)
        .filter(|middle_length| by_length.get(middle_length).copied().unwrap_or(0.0) > 0.0)
        .collect::<Vec<_>>();
    if positive_lengths.is_empty() {
        "no anomaly mass in range".to_string()
    } else if let Some(collapse_length) =
        find_collapse_length(&by_length, min_middle_length, max_middle_length)
    {
        format!("collapsed by M={collapse_length}")
    } else {
        "still positive at max length".to_string()
    }
}

fn positive_anomaly(value: f64) -> f64 {
    if value > 0.0 {
        value
    } else {
        0.0
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

fn join_usizes(values: Vec<usize>) -> String {
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

fn print_summary(bundle: &ReportBundle) {
    println!("=== M Transition Curve Report ===\n");
    println!(
        "Pair catalog: {} | M={}..{} | bases {:?} | output {}",
        bundle.settings.pair_catalog_mode,
        bundle.settings.min_middle_length,
        bundle.settings.max_middle_length,
        bundle.settings.bases,
        bundle.settings.out_dir
    );
    println!();
    println!(
        "Lengths with positive anomaly mass: {}",
        join_usizes(
            bundle
                .report_summary
                .lengths_with_positive_anomaly_mass
                .clone()
        )
    );
    println!(
        "First global zero after positive mass: {}",
        bundle
            .report_summary
            .first_global_zero_after_positive
            .map(|value| format!("M={value}"))
            .unwrap_or_else(|| "none in scanned range".to_string())
    );
    println!(
        "All bases zero from length: {}",
        bundle
            .report_summary
            .all_bases_zero_from_length
            .map(|value| format!("M={value}"))
            .unwrap_or_else(|| "none in scanned range".to_string())
    );
    println!(
        "Pairs with any anomaly: {} / {} | persistent at max length: {}",
        bundle.report_summary.pairs_with_any_anomaly,
        bundle.report_summary.total_pairs,
        bundle
            .report_summary
            .pairs_with_persistent_anomaly_at_max_length
    );
    println!(
        "Bases with any anomaly: {}",
        join_u32s(bundle.report_summary.bases_with_any_anomaly.clone())
    );
    println!();

    for row in &bundle.global_curve_rows {
        println!(
            "  - M={}: anomaly mass {:.2}pp | counterexample pairs {}/{} | active bases {} | k00 noninferior {:.1}%",
            row.middle_length,
            row.anomaly_mass_pp,
            row.counterexample_pairs,
            row.ordered_pair_count,
            row.active_bases,
            row.k00_noninferior_share * 100.0
        );
    }
}

fn render_markdown_report(bundle: &ReportBundle) -> String {
    let mut leading_pairs = bundle
        .pair_curve_summary_rows
        .iter()
        .filter(|row| row.anomaly_count > 0)
        .collect::<Vec<_>>();
    leading_pairs.sort_by(|left, right| {
        right
            .total_anomaly_mass_pp
            .total_cmp(&left.total_anomaly_mass_pp)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    leading_pairs.truncate(10);

    let mut lines = vec![
        "# M Transition Curve Report".to_string(),
        String::new(),
        "_Generated from `examples/m_transition_curve_report.rs`._".to_string(),
        String::new(),
        format!("- Generated at: `{}`", bundle.generated_at_utc),
        format!("- Bases: `{:?}`", bundle.settings.bases),
        format!(
            "- Middle lengths: `{}..={}`",
            bundle.settings.min_middle_length, bundle.settings.max_middle_length
        ),
        format!("- Pair catalog: `{}`", bundle.settings.pair_catalog_mode),
        format!("- Bounded k-grid: `{:?}`", bundle.settings.k_grid),
        String::new(),
        "## Overall".to_string(),
        String::new(),
        format!(
            "- Lengths with positive anomaly mass: `{}`",
            join_usizes(bundle.report_summary.lengths_with_positive_anomaly_mass.clone())
        ),
        format!(
            "- First global zero after positive mass: `{}`",
            bundle
                .report_summary
                .first_global_zero_after_positive
                .map(|value| format!("M={value}"))
                .unwrap_or_else(|| "none in scanned range".to_string())
        ),
        format!(
            "- All bases zero from length: `{}`",
            bundle
                .report_summary
                .all_bases_zero_from_length
                .map(|value| format!("M={value}"))
                .unwrap_or_else(|| "none in scanned range".to_string())
        ),
        format!(
            "- Pairs with any anomaly: `{}/{}; persistent at max length: {}`",
            bundle.report_summary.pairs_with_any_anomaly,
            bundle.report_summary.total_pairs,
            bundle.report_summary.pairs_with_persistent_anomaly_at_max_length
        ),
        format!(
            "- Bases with any anomaly: `{}`",
            join_u32s(bundle.report_summary.bases_with_any_anomaly.clone())
        ),
        String::new(),
        "## Global Curve".to_string(),
        String::new(),
        "| M | Counterexample pairs | Anomaly mass | Active bases | k00 noninferior share | Leading pair |".to_string(),
        "|---:|---:|---:|---|---:|---|".to_string(),
    ];

    for row in &bundle.global_curve_rows {
        lines.push(format!(
            "| `{}` | `{}/{} ({:.1}%)` | `{:.2}pp` | `{}` | `{:.1}%` | `{}` via `{}` (`{:.2}pp`) |",
            row.middle_length,
            row.counterexample_pairs,
            row.ordered_pair_count,
            row.counterexample_share * 100.0,
            row.anomaly_mass_pp,
            row.active_bases,
            row.k00_noninferior_share * 100.0,
            row.leading_pair,
            row.leading_best_k,
            row.leading_margin_pp
        ));
    }

    lines.extend([
        String::new(),
        "## Base Rows".to_string(),
        String::new(),
        "| Base | M | Counterexample pairs | Anomaly mass | k00 noninferior share | Strongest counterexample |".to_string(),
        "|---:|---:|---:|---:|---:|---|".to_string(),
    ]);

    for row in &bundle.base_length_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}/{} ({:.1}%)` | `{:.2}pp` | `{:.1}%` | `{}` via `{}` (`{:.2}pp`) |",
            row.base,
            row.middle_length,
            row.counterexample_pairs,
            row.ordered_pair_count,
            row.counterexample_share * 100.0,
            row.anomaly_mass_pp,
            row.k00_noninferior_share * 100.0,
            row.strongest_counterexample_pair,
            row.strongest_counterexample_best_k,
            row.strongest_counterexample_margin_pp
        ));
    }

    if !leading_pairs.is_empty() {
        lines.extend([
            String::new(),
            "## Leading Anomaly Pairs".to_string(),
            String::new(),
            "| Base | Pair | Lengths | Collapse | Dominant best k | Peak | Total |".to_string(),
            "|---:|---|---|---|---|---:|---:|".to_string(),
        ]);
        for row in leading_pairs {
            lines.push(format!(
                "| `{}` | {} | `{}` | `{}` | `{}` | `{:.2}pp @ M={}` | `{:.2}pp` |",
                row.base,
                row.pair_label,
                row.anomaly_lengths,
                row.collapse_estimate,
                row.dominant_counterexample_k,
                row.max_anomaly_mass_pp,
                row.max_anomaly_length
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                row.total_anomaly_mass_pp
            ));
        }
    }

    lines.join("\n")
}
