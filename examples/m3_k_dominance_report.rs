//! Cross-base bounded-`k` report for the stable `M=3` membrane lane.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example m3_k_dominance_report
//! cargo run --release --example m3_k_dominance_report -- --full --out-dir /tmp/primes_m3_k_dominance_full
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
use std::{env, path::PathBuf};

const BASES: &[u32] = &[6, 10, 12, 14, 30];
const MIDDLE_LENGTH: usize = 3;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_m3_k_dominance";
const REPORT_EXPORT_VERSION: u32 = 1;
const SMOKE_MAX_ORDERED_PAIRS_PER_BASE: usize = 8;
const K00_SUPPORT_THRESHOLD: f64 = 0.75;
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
    middle_length: usize,
    k_grid: Vec<String>,
    pair_catalog_mode: String,
    max_ordered_pairs_per_base: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    supporting_bases: Vec<u32>,
    counterexample_bases: Vec<u32>,
    all_bases_supported: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    pair_rows: Vec<KDominancePairRow>,
    summary_rows: Vec<KDominanceSummaryRow>,
    report_summary: ReportSummary,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        bases: BASES.to_vec(),
        middle_length: MIDDLE_LENGTH,
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format_k(config))
            .collect(),
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
    };

    let pair_rows = build_pair_rows(options.full_catalog);
    let summary_rows = summarize_pair_rows(&pair_rows);
    let report_summary = ReportSummary {
        supporting_bases: summary_rows
            .iter()
            .filter(|row| row.k00_noninferior_share >= K00_SUPPORT_THRESHOLD)
            .map(|row| row.base)
            .collect(),
        counterexample_bases: summary_rows
            .iter()
            .filter(|row| row.strongest_counterexample_margin_pp > 0.0)
            .map(|row| row.base)
            .collect(),
        all_bases_supported: summary_rows
            .iter()
            .all(|row| row.k00_noninferior_share >= K00_SUPPORT_THRESHOLD),
    };

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        pair_rows,
        summary_rows,
        report_summary,
    };

    write_csv_rows(options.out_dir.join("pair_rows.csv"), &bundle.pair_rows)
        .expect("failed to write pair rows");
    write_csv_rows(
        options.out_dir.join("summary_rows.csv"),
        &bundle.summary_rows,
    )
    .expect("failed to write summary rows");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("failed to write json");
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
    println!("M=3 bounded-k dominance report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example m3_k_dominance_report -- [options]");
    println!();
    println!("Options:");
    println!(
        "  --out-dir <path>          Output directory for artifacts (default: {DEFAULT_OUT_DIR})"
    );
    println!("  --full                    Use the exhaustive ordered-pair catalog instead of the default smoke catalog");
}

fn build_pair_rows(full_catalog: bool) -> Vec<KDominancePairRow> {
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
        .map(|&(base, outer, inner)| {
            evaluate_pair_row(base, MIDDLE_LENGTH, outer, inner, DEFAULT_BOUNDED_K_GRID)
        })
        .collect();
    rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
    });
    rows
}

fn print_summary(bundle: &ReportBundle) {
    println!("=== M=3 Bounded-k Report ===\n");
    println!(
        "Pair catalog: {} | bases {:?} | output {}",
        bundle.settings.pair_catalog_mode, bundle.settings.bases, bundle.settings.out_dir
    );
    println!();
    for row in &bundle.summary_rows {
        println!(
            "  - base {:>2}: k00 noninferior {:.0}% | strict/tied {}/{} of {} | strongest counterexample {} via {} at +{:.2}pp",
            row.base,
            row.k00_noninferior_share * 100.0,
            row.k00_strict_best_pairs,
            row.k00_tied_best_pairs,
            row.ordered_pair_count,
            row.strongest_counterexample_pair,
            row.strongest_counterexample_best_k,
            row.strongest_counterexample_margin_pp
        );
    }
    println!();
    println!(
        "Supporting bases (threshold {:.0}%): {}",
        K00_SUPPORT_THRESHOLD * 100.0,
        format_base_list(&bundle.report_summary.supporting_bases)
    );
    println!(
        "Counterexample bases: {}",
        format_base_list(&bundle.report_summary.counterexample_bases)
    );
}

fn render_markdown_report(bundle: &ReportBundle) -> String {
    let mut counterexamples: Vec<_> = bundle
        .pair_rows
        .iter()
        .filter(|row| row.best_minus_k00_pp > 0.0)
        .collect();
    counterexamples.sort_by(|left, right| {
        right
            .best_minus_k00_pp
            .total_cmp(&left.best_minus_k00_pp)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });

    let mut lines = vec![
        "# M=3 Bounded-k Dominance Report".to_string(),
        String::new(),
        "_Generated from `examples/m3_k_dominance_report.rs`._".to_string(),
        String::new(),
        format!("- Generated at: `{}`", bundle.generated_at_utc),
        format!("- Bases: `{:?}`", bundle.settings.bases),
        format!("- Pair catalog: `{}`", bundle.settings.pair_catalog_mode),
        format!("- Bounded k-grid: `{:?}`", bundle.settings.k_grid),
        String::new(),
        "## Summary".to_string(),
        String::new(),
        format!(
            "- Supporting bases (`>= {:.0}%` k00 noninferior share): {}",
            K00_SUPPORT_THRESHOLD * 100.0,
            format_base_list(&bundle.report_summary.supporting_bases)
        ),
        format!(
            "- Counterexample bases: {}",
            format_base_list(&bundle.report_summary.counterexample_bases)
        ),
        String::new(),
        "## Base Rows".to_string(),
        String::new(),
        "| Base | Ordered pairs | k00 noninferior | strict best | tied best | strongest counterexample |".to_string(),
        "|---:|---:|---:|---:|---:|---|".to_string(),
    ];

    for row in &bundle.summary_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{:.0}%` | `{}` | `{}` | {} via {} at `{:.2}pp` |",
            row.base,
            row.ordered_pair_count,
            row.k00_noninferior_share * 100.0,
            row.k00_strict_best_pairs,
            row.k00_tied_best_pairs,
            row.strongest_counterexample_pair,
            row.strongest_counterexample_best_k,
            row.strongest_counterexample_margin_pp
        ));
    }

    if let Some(row) = counterexamples.first() {
        lines.extend([
            String::new(),
            "## Leading Counterexample".to_string(),
            String::new(),
            format!(
                "- Base `{}` pair `{}` improves from `{:.2}%` at `k=(0,0)` to `{:.2}%` at `{}`.",
                row.base,
                row.pair_label,
                row.rate_k00 * 100.0,
                row.best_rate * 100.0,
                row.best_k
            ),
        ]);
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
