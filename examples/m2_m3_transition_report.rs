//! Cross-base transition report for the bounded-`k` membrane boundary between
//! `M=2` and `M=3`.
//!
//! This report compares the exact same ordered unit-residue pairs at both
//! lengths and tracks where the positive anomaly mass
//! `max(best_minus_k00_pp, 0)` disappears.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example m2_m3_transition_report
//! cargo run --release --example m2_m3_transition_report -- --full --out-dir /tmp/primes_m2_m3_transition_full
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
const M2: usize = 2;
const M3: usize = 3;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_m2_m3_transition";
const REPORT_EXPORT_VERSION: u32 = 1;
const SMOKE_MAX_ORDERED_PAIRS_PER_BASE: usize = 8;
const SMOKE_PAIR_ANCHORS: &[(u32, u32, u32)] = &[(6, 1, 5), (10, 3, 3), (10, 3, 7), (30, 11, 7)];

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
    k_grid: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct PairTransitionRow {
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    best_k_m2: String,
    best_k_m3: String,
    prime_hits_k00_m2: usize,
    prime_hits_best_m2: usize,
    prime_hits_k00_m3: usize,
    prime_hits_best_m3: usize,
    rate_k00_m2: f64,
    best_rate_m2: f64,
    rate_k00_m3: f64,
    best_rate_m3: f64,
    best_minus_k00_m2_pp: f64,
    best_minus_k00_m3_pp: f64,
    anomaly_mass_m2_pp: f64,
    anomaly_mass_m3_pp: f64,
    anomaly_mass_drop_pp: f64,
    k00_noninferior_m2: bool,
    k00_noninferior_m3: bool,
    transition_class: String,
}

#[derive(Debug, Clone, Serialize)]
struct BaseTransitionRow {
    base: u32,
    ordered_pair_count: usize,
    counterexample_pairs_m2: usize,
    counterexample_pairs_m3: usize,
    counterexample_share_m2: f64,
    counterexample_share_m3: f64,
    anomaly_mass_m2_pp: f64,
    anomaly_mass_m3_pp: f64,
    anomaly_mass_collapse_share: Option<f64>,
    collapsed_pairs: usize,
    persistent_pairs: usize,
    new_pairs_at_m3: usize,
    strongest_m2_pair: String,
    strongest_m2_best_k: String,
    strongest_m2_margin_pp: f64,
    strongest_m3_pair: String,
    strongest_m3_best_k: String,
    strongest_m3_margin_pp: f64,
    collapse_estimate: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    total_pairs: usize,
    total_counterexample_pairs_m2: usize,
    total_counterexample_pairs_m3: usize,
    total_anomaly_mass_m2_pp: f64,
    total_anomaly_mass_m3_pp: f64,
    total_anomaly_mass_collapse_share: Option<f64>,
    bases_collapsed_by_m3: Vec<u32>,
    bases_with_persistent_mass: Vec<u32>,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    pair_transition_rows: Vec<PairTransitionRow>,
    base_transition_rows: Vec<BaseTransitionRow>,
    summary_rows_m2: Vec<KDominanceSummaryRow>,
    summary_rows_m3: Vec<KDominanceSummaryRow>,
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
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format_k(config))
            .collect(),
    };

    let (rows_m2, rows_m3, pair_transition_rows) = build_pair_transition_rows(options.full_catalog);
    let summary_rows_m2 = summarize_pair_rows(&rows_m2);
    let summary_rows_m3 = summarize_pair_rows(&rows_m3);
    let base_transition_rows =
        build_base_transition_rows(&pair_transition_rows, &summary_rows_m2, &summary_rows_m3);
    let report_summary = build_report_summary(&pair_transition_rows, &base_transition_rows);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        pair_transition_rows,
        base_transition_rows,
        summary_rows_m2,
        summary_rows_m3,
        report_summary,
    };

    write_csv_rows(
        options.out_dir.join("pair_transition_rows.csv"),
        &bundle.pair_transition_rows,
    )
    .expect("failed to write pair transition rows");
    write_csv_rows(
        options.out_dir.join("base_transition_rows.csv"),
        &bundle.base_transition_rows,
    )
    .expect("failed to write base transition rows");
    write_csv_rows(
        options.out_dir.join("summary_rows_m2.csv"),
        &bundle.summary_rows_m2,
    )
    .expect("failed to write M2 summary rows");
    write_csv_rows(
        options.out_dir.join("summary_rows_m3.csv"),
        &bundle.summary_rows_m3,
    )
    .expect("failed to write M3 summary rows");
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
    println!("M=2 vs M=3 bounded-k transition report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example m2_m3_transition_report -- [options]");
    println!();
    println!("Options:");
    println!(
        "  --out-dir <path>          Output directory for artifacts (default: {DEFAULT_OUT_DIR})"
    );
    println!("  --full                    Use the exhaustive ordered-pair catalog instead of the default smoke catalog");
}

fn build_pair_transition_rows(
    full_catalog: bool,
) -> (
    Vec<KDominancePairRow>,
    Vec<KDominancePairRow>,
    Vec<PairTransitionRow>,
) {
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

    let mut triples: Vec<_> = tasks
        .par_iter()
        .map(|&(base, outer, inner)| {
            let row_m2 = evaluate_pair_row(base, M2, outer, inner, DEFAULT_BOUNDED_K_GRID);
            let row_m3 = evaluate_pair_row(base, M3, outer, inner, DEFAULT_BOUNDED_K_GRID);
            let transition = build_pair_transition_row(&row_m2, &row_m3);
            (row_m2, row_m3, transition)
        })
        .collect();
    triples.sort_by(|left, right| {
        left.0
            .base
            .cmp(&right.0.base)
            .then_with(|| left.0.outer.cmp(&right.0.outer))
            .then_with(|| left.0.inner.cmp(&right.0.inner))
    });

    let rows_m2 = triples.iter().map(|(m2, _, _)| m2.clone()).collect();
    let rows_m3 = triples.iter().map(|(_, m3, _)| m3.clone()).collect();
    let transition_rows = triples.into_iter().map(|(_, _, row)| row).collect();
    (rows_m2, rows_m3, transition_rows)
}

fn build_pair_transition_row(
    row_m2: &KDominancePairRow,
    row_m3: &KDominancePairRow,
) -> PairTransitionRow {
    let anomaly_mass_m2_pp = row_m2.best_minus_k00_pp.max(0.0);
    let anomaly_mass_m3_pp = row_m3.best_minus_k00_pp.max(0.0);
    let transition_class = match (anomaly_mass_m2_pp > 0.0, anomaly_mass_m3_pp > 0.0) {
        (true, false) => "collapsed_by_m3",
        (true, true) => "persistent_counterexample",
        (false, true) => "new_at_m3",
        (false, false) => "stable_or_tied",
    }
    .to_string();

    PairTransitionRow {
        base: row_m2.base,
        outer: row_m2.outer,
        inner: row_m2.inner,
        pair_label: row_m2.pair_label.clone(),
        best_k_m2: row_m2.best_k.clone(),
        best_k_m3: row_m3.best_k.clone(),
        prime_hits_k00_m2: row_m2.prime_hits_k00,
        prime_hits_best_m2: row_m2.best_prime_hits,
        prime_hits_k00_m3: row_m3.prime_hits_k00,
        prime_hits_best_m3: row_m3.best_prime_hits,
        rate_k00_m2: row_m2.rate_k00,
        best_rate_m2: row_m2.best_rate,
        rate_k00_m3: row_m3.rate_k00,
        best_rate_m3: row_m3.best_rate,
        best_minus_k00_m2_pp: row_m2.best_minus_k00_pp,
        best_minus_k00_m3_pp: row_m3.best_minus_k00_pp,
        anomaly_mass_m2_pp,
        anomaly_mass_m3_pp,
        anomaly_mass_drop_pp: anomaly_mass_m2_pp - anomaly_mass_m3_pp,
        k00_noninferior_m2: row_m2.k00_noninferior,
        k00_noninferior_m3: row_m3.k00_noninferior,
        transition_class,
    }
}

fn build_base_transition_rows(
    pair_rows: &[PairTransitionRow],
    summary_rows_m2: &[KDominanceSummaryRow],
    summary_rows_m3: &[KDominanceSummaryRow],
) -> Vec<BaseTransitionRow> {
    let summary_m2_by_base = summary_rows_m2
        .iter()
        .map(|row| (row.base, row))
        .collect::<BTreeMap<_, _>>();
    let summary_m3_by_base = summary_rows_m3
        .iter()
        .map(|row| (row.base, row))
        .collect::<BTreeMap<_, _>>();

    let mut by_base: BTreeMap<u32, Vec<&PairTransitionRow>> = BTreeMap::new();
    for row in pair_rows {
        by_base.entry(row.base).or_default().push(row);
    }

    by_base
        .into_iter()
        .map(|(base, rows)| {
            let ordered_pair_count = rows.len();
            let counterexample_pairs_m2 = rows
                .iter()
                .filter(|row| row.anomaly_mass_m2_pp > 0.0)
                .count();
            let counterexample_pairs_m3 = rows
                .iter()
                .filter(|row| row.anomaly_mass_m3_pp > 0.0)
                .count();
            let anomaly_mass_m2_pp: f64 = rows.iter().map(|row| row.anomaly_mass_m2_pp).sum();
            let anomaly_mass_m3_pp: f64 = rows.iter().map(|row| row.anomaly_mass_m3_pp).sum();
            let collapsed_pairs = rows
                .iter()
                .filter(|row| row.transition_class == "collapsed_by_m3")
                .count();
            let persistent_pairs = rows
                .iter()
                .filter(|row| row.transition_class == "persistent_counterexample")
                .count();
            let new_pairs_at_m3 = rows
                .iter()
                .filter(|row| row.transition_class == "new_at_m3")
                .count();
            let summary_m2 = summary_m2_by_base[&base];
            let summary_m3 = summary_m3_by_base[&base];

            BaseTransitionRow {
                base,
                ordered_pair_count,
                counterexample_pairs_m2,
                counterexample_pairs_m3,
                counterexample_share_m2: counterexample_pairs_m2 as f64 / ordered_pair_count as f64,
                counterexample_share_m3: counterexample_pairs_m3 as f64 / ordered_pair_count as f64,
                anomaly_mass_m2_pp,
                anomaly_mass_m3_pp,
                anomaly_mass_collapse_share: if anomaly_mass_m2_pp > 0.0 {
                    Some(1.0 - anomaly_mass_m3_pp / anomaly_mass_m2_pp)
                } else {
                    None
                },
                collapsed_pairs,
                persistent_pairs,
                new_pairs_at_m3,
                strongest_m2_pair: summary_m2.strongest_counterexample_pair.clone(),
                strongest_m2_best_k: summary_m2.strongest_counterexample_best_k.clone(),
                strongest_m2_margin_pp: summary_m2.strongest_counterexample_margin_pp,
                strongest_m3_pair: summary_m3.strongest_counterexample_pair.clone(),
                strongest_m3_best_k: summary_m3.strongest_counterexample_best_k.clone(),
                strongest_m3_margin_pp: summary_m3.strongest_counterexample_margin_pp,
                collapse_estimate: collapse_estimate(anomaly_mass_m2_pp, anomaly_mass_m3_pp),
            }
        })
        .collect()
}

fn build_report_summary(
    pair_rows: &[PairTransitionRow],
    base_rows: &[BaseTransitionRow],
) -> ReportSummary {
    let total_pairs = pair_rows.len();
    let total_counterexample_pairs_m2 = pair_rows
        .iter()
        .filter(|row| row.anomaly_mass_m2_pp > 0.0)
        .count();
    let total_counterexample_pairs_m3 = pair_rows
        .iter()
        .filter(|row| row.anomaly_mass_m3_pp > 0.0)
        .count();
    let total_anomaly_mass_m2_pp: f64 = pair_rows.iter().map(|row| row.anomaly_mass_m2_pp).sum();
    let total_anomaly_mass_m3_pp: f64 = pair_rows.iter().map(|row| row.anomaly_mass_m3_pp).sum();

    ReportSummary {
        total_pairs,
        total_counterexample_pairs_m2,
        total_counterexample_pairs_m3,
        total_anomaly_mass_m2_pp,
        total_anomaly_mass_m3_pp,
        total_anomaly_mass_collapse_share: if total_anomaly_mass_m2_pp > 0.0 {
            Some(1.0 - total_anomaly_mass_m3_pp / total_anomaly_mass_m2_pp)
        } else {
            None
        },
        bases_collapsed_by_m3: base_rows
            .iter()
            .filter(|row| row.anomaly_mass_m2_pp > 0.0 && row.anomaly_mass_m3_pp == 0.0)
            .map(|row| row.base)
            .collect(),
        bases_with_persistent_mass: base_rows
            .iter()
            .filter(|row| row.anomaly_mass_m3_pp > 0.0)
            .map(|row| row.base)
            .collect(),
    }
}

fn collapse_estimate(anomaly_mass_m2_pp: f64, anomaly_mass_m3_pp: f64) -> String {
    if anomaly_mass_m2_pp == 0.0 && anomaly_mass_m3_pp == 0.0 {
        "no anomaly mass at M=2".to_string()
    } else if anomaly_mass_m2_pp > 0.0 && anomaly_mass_m3_pp == 0.0 {
        "collapsed in (2,3]".to_string()
    } else if anomaly_mass_m2_pp > 0.0 && anomaly_mass_m3_pp < anomaly_mass_m2_pp {
        "partial collapse by 3".to_string()
    } else if anomaly_mass_m3_pp >= anomaly_mass_m2_pp && anomaly_mass_m3_pp > 0.0 {
        "no collapse by 3".to_string()
    } else {
        "indeterminate".to_string()
    }
}

fn print_summary(bundle: &ReportBundle) {
    println!("=== M=2 vs M=3 Transition Report ===\n");
    println!(
        "Pair catalog: {} | bases {:?} | output {}",
        bundle.settings.pair_catalog_mode, bundle.settings.bases, bundle.settings.out_dir
    );
    println!();
    println!(
        "Overall anomaly mass: M=2 {:.2}pp -> M=3 {:.2}pp | collapse {}",
        bundle.report_summary.total_anomaly_mass_m2_pp,
        bundle.report_summary.total_anomaly_mass_m3_pp,
        format_collapse_share(bundle.report_summary.total_anomaly_mass_collapse_share)
    );
    println!(
        "Counterexample pairs: M=2 {} / {} | M=3 {} / {}",
        bundle.report_summary.total_counterexample_pairs_m2,
        bundle.report_summary.total_pairs,
        bundle.report_summary.total_counterexample_pairs_m3,
        bundle.report_summary.total_pairs
    );
    println!(
        "Bases collapsed by M=3: {}",
        format_base_list(&bundle.report_summary.bases_collapsed_by_m3)
    );
    println!(
        "Bases with persistent mass at M=3: {}",
        format_base_list(&bundle.report_summary.bases_with_persistent_mass)
    );
    println!();

    for row in &bundle.base_transition_rows {
        println!(
            "  - base {:>2}: anomaly mass {:.2}pp -> {:.2}pp | counterexample pairs {}/{} -> {}/{} | {}",
            row.base,
            row.anomaly_mass_m2_pp,
            row.anomaly_mass_m3_pp,
            row.counterexample_pairs_m2,
            row.ordered_pair_count,
            row.counterexample_pairs_m3,
            row.ordered_pair_count,
            row.collapse_estimate
        );
    }
}

fn render_markdown_report(bundle: &ReportBundle) -> String {
    let mut leading_rows: Vec<_> = bundle
        .pair_transition_rows
        .iter()
        .filter(|row| row.anomaly_mass_m2_pp > 0.0 || row.anomaly_mass_m3_pp > 0.0)
        .collect();
    leading_rows.sort_by(|left, right| {
        right
            .anomaly_mass_drop_pp
            .total_cmp(&left.anomaly_mass_drop_pp)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    leading_rows.truncate(10);

    let mut lines = vec![
        "# M=2 vs M=3 Transition Report".to_string(),
        String::new(),
        "_Generated from `examples/m2_m3_transition_report.rs`._".to_string(),
        String::new(),
        format!("- Generated at: `{}`", bundle.generated_at_utc),
        format!("- Bases: `{:?}`", bundle.settings.bases),
        format!("- Pair catalog: `{}`", bundle.settings.pair_catalog_mode),
        format!("- Bounded k-grid: `{:?}`", bundle.settings.k_grid),
        String::new(),
        "## Overall".to_string(),
        String::new(),
        format!(
            "- Counterexample pairs: `M=2 {}/{} -> M=3 {}/{}`",
            bundle.report_summary.total_counterexample_pairs_m2,
            bundle.report_summary.total_pairs,
            bundle.report_summary.total_counterexample_pairs_m3,
            bundle.report_summary.total_pairs
        ),
        format!(
            "- Anomaly mass: `M=2 {:.2}pp -> M=3 {:.2}pp`, collapse `{}`",
            bundle.report_summary.total_anomaly_mass_m2_pp,
            bundle.report_summary.total_anomaly_mass_m3_pp,
            format_collapse_share(bundle.report_summary.total_anomaly_mass_collapse_share)
        ),
        format!(
            "- Bases collapsed by M=3: {}",
            format_base_list(&bundle.report_summary.bases_collapsed_by_m3)
        ),
        format!(
            "- Bases with persistent mass at M=3: {}",
            format_base_list(&bundle.report_summary.bases_with_persistent_mass)
        ),
        String::new(),
        "## Base Rows".to_string(),
        String::new(),
        "| Base | Pair count | Counterexample pairs M2 | Counterexample pairs M3 | Anomaly mass M2 | Anomaly mass M3 | Collapse | Estimate |".to_string(),
        "|---:|---:|---:|---:|---:|---:|---:|---|".to_string(),
    ];

    for row in &bundle.base_transition_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{:.2}pp` | `{:.2}pp` | `{}` | {} |",
            row.base,
            row.ordered_pair_count,
            row.counterexample_pairs_m2,
            row.counterexample_pairs_m3,
            row.anomaly_mass_m2_pp,
            row.anomaly_mass_m3_pp,
            format_collapse_share(row.anomaly_mass_collapse_share),
            row.collapse_estimate
        ));
    }

    if !leading_rows.is_empty() {
        lines.extend([
            String::new(),
            "## Largest Drops".to_string(),
            String::new(),
            "| Base | Pair | M2 best k | M2 margin | M3 best k | M3 margin | Drop | Class |"
                .to_string(),
            "|---:|---|---|---:|---|---:|---:|---|".to_string(),
        ]);
        for row in leading_rows {
            lines.push(format!(
                "| `{}` | {} | `{}` | `{:.2}pp` | `{}` | `{:.2}pp` | `{:.2}pp` | `{}` |",
                row.base,
                row.pair_label,
                row.best_k_m2,
                row.best_minus_k00_m2_pp,
                row.best_k_m3,
                row.best_minus_k00_m3_pp,
                row.anomaly_mass_drop_pp,
                row.transition_class
            ));
        }
    }

    lines.join("\n")
}

fn format_base_list(bases: &[u32]) -> String {
    if bases.is_empty() {
        "none".to_string()
    } else {
        bases
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn format_collapse_share(value: Option<f64>) -> String {
    value
        .map(|share| format!("{:.0}%", share * 100.0))
        .unwrap_or_else(|| "n/a".to_string())
}
