//! Focused reproducer for bounded-`k` counterexamples.
//!
//! Defaults to the strongest current smoke-lane anomaly:
//! base `6`, `M=2`, pair `(5,5)`, where `k=(1,0)` beats `k=(0,0)`.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example base_specific_k_counterexample
//! cargo run --example base_specific_k_counterexample -- --base 10 --middle-length 2 --outer 3 --inner 3
//! ```

use primes::validation::{
    bounded_k::{format_k, scan_k_config_examples, KConfigPrimeExample, DEFAULT_BOUNDED_K_GRID},
    reporting::{
        ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
    },
};
use serde::Serialize;
use std::{env, path::PathBuf};

const DEFAULT_BASE: u32 = 6;
const DEFAULT_MIDDLE_LENGTH: usize = 2;
const DEFAULT_OUTER: u32 = 5;
const DEFAULT_INNER: u32 = 5;
const DEFAULT_EXAMPLE_LIMIT: usize = 8;
const DEFAULT_OUT_DIR: &str = "/tmp/primes_k_counterexample";
const REPORT_EXPORT_VERSION: u32 = 1;

#[derive(Debug)]
struct Options {
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    example_limit: usize,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    example_limit: usize,
    out_dir: String,
}

#[derive(Debug, Clone, Serialize)]
struct KConfigRow {
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    k_label: String,
    candidates_per_config: usize,
    prime_hits: usize,
    rate: f64,
    delta_vs_k00_pp: f64,
    prime_examples: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    counterexample_found: bool,
    best_k: String,
    best_prime_hits: usize,
    rows: Vec<KConfigRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let candidates_per_config = (options.base as usize).pow(options.middle_length as u32);
    let mut rows = DEFAULT_BOUNDED_K_GRID
        .iter()
        .copied()
        .map(|config| build_row(&options, config, candidates_per_config))
        .collect::<Vec<_>>();
    let k00_hits = rows
        .iter()
        .find(|row| row.k_outer == 0 && row.k_inner == 0)
        .expect("missing k00 row")
        .prime_hits;
    for row in &mut rows {
        row.delta_vs_k00_pp =
            (row.prime_hits as f64 - k00_hits as f64) * 100.0 / candidates_per_config as f64;
    }
    rows.sort_by(|left, right| {
        right
            .prime_hits
            .cmp(&left.prime_hits)
            .then_with(|| left.k_outer.cmp(&right.k_outer))
            .then_with(|| left.k_inner.cmp(&right.k_inner))
    });

    let best_row = rows.first().expect("bounded k-grid should not be empty");

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings: ReportSettings {
            base: options.base,
            middle_length: options.middle_length,
            outer: options.outer,
            inner: options.inner,
            example_limit: options.example_limit,
            out_dir: options.out_dir.display().to_string(),
        },
        counterexample_found: best_row.prime_hits > k00_hits,
        best_k: best_row.k_label.clone(),
        best_prime_hits: best_row.prime_hits,
        rows,
    };

    write_csv_rows(options.out_dir.join("k_config_rows.csv"), &bundle.rows)
        .expect("failed to write k config rows");
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
    let mut options = Options {
        base: DEFAULT_BASE,
        middle_length: DEFAULT_MIDDLE_LENGTH,
        outer: DEFAULT_OUTER,
        inner: DEFAULT_INNER,
        example_limit: DEFAULT_EXAMPLE_LIMIT,
        out_dir: PathBuf::from(DEFAULT_OUT_DIR),
    };

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--base" => options.base = parse_next(&mut args, "--base"),
            "--middle-length" => options.middle_length = parse_next(&mut args, "--middle-length"),
            "--outer" => options.outer = parse_next(&mut args, "--outer"),
            "--inner" => options.inner = parse_next(&mut args, "--inner"),
            "--example-limit" => options.example_limit = parse_next(&mut args, "--example-limit"),
            "--out-dir" => {
                options.out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir"))
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

    options
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
    println!("Bounded-k counterexample reproducer");
    println!();
    println!("Usage:");
    println!("  cargo run --example base_specific_k_counterexample -- [options]");
    println!();
    println!("Options:");
    println!("  --base <n>               Base to scan (default: {DEFAULT_BASE})");
    println!("  --middle-length <n>      Middle length M (default: {DEFAULT_MIDDLE_LENGTH})");
    println!("  --outer <digit>          Outer boundary digit (default: {DEFAULT_OUTER})");
    println!("  --inner <digit>          Inner boundary digit (default: {DEFAULT_INNER})");
    println!("  --example-limit <n>      Prime examples to record per k row (default: {DEFAULT_EXAMPLE_LIMIT})");
    println!(
        "  --out-dir <path>         Output directory for artifacts (default: {DEFAULT_OUT_DIR})"
    );
}

fn build_row(options: &Options, config: (u32, u32), candidates_per_config: usize) -> KConfigRow {
    let (prime_hits, examples) = scan_k_config_examples(
        options.base,
        options.middle_length,
        options.outer,
        options.inner,
        config,
        options.example_limit,
    );
    KConfigRow {
        base: options.base,
        middle_length: options.middle_length,
        outer: options.outer,
        inner: options.inner,
        k_outer: config.0,
        k_inner: config.1,
        k_label: format_k(config),
        candidates_per_config,
        prime_hits,
        rate: prime_hits as f64 / candidates_per_config as f64,
        delta_vs_k00_pp: 0.0,
        prime_examples: format_examples(&examples),
    }
}

fn format_examples(examples: &[KConfigPrimeExample]) -> String {
    if examples.is_empty() {
        "none".to_string()
    } else {
        examples
            .iter()
            .map(|example| format!("{}=>{}", example.middle_digits, example.decimal_value))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

fn print_summary(bundle: &ReportBundle) {
    println!("=== Bounded-k Counterexample Reproducer ===\n");
    println!(
        "base {} | M={} | pair ({},{}) | output {}",
        bundle.settings.base,
        bundle.settings.middle_length,
        bundle.settings.outer,
        bundle.settings.inner,
        bundle.settings.out_dir
    );
    println!();
    for row in &bundle.rows {
        println!(
            "  - {:>6}: hits {:>3}/{:<3} ({:>6.2}%) | delta vs k00 {:+.2}pp | examples {}",
            row.k_label,
            row.prime_hits,
            row.candidates_per_config,
            row.rate * 100.0,
            row.delta_vs_k00_pp,
            row.prime_examples
        );
    }
    println!();
    println!(
        "Counterexample found: {} | best k: {}",
        if bundle.counterexample_found {
            "yes"
        } else {
            "no"
        },
        bundle.best_k
    );
}

fn render_markdown_report(bundle: &ReportBundle) -> String {
    let mut lines = vec![
        "# Bounded-k Counterexample Reproducer".to_string(),
        String::new(),
        "_Generated from `historical/examples/base_specific_k_counterexample.rs`._".to_string(),
        String::new(),
        format!("- Generated at: `{}`", bundle.generated_at_utc),
        format!("- Base: `{}`", bundle.settings.base),
        format!("- Middle length: `{}`", bundle.settings.middle_length),
        format!(
            "- Pair: `({}, {})`",
            bundle.settings.outer, bundle.settings.inner
        ),
        format!("- Counterexample found: `{}`", bundle.counterexample_found),
        String::new(),
        "| k | Prime hits | Rate | Delta vs k00 | Prime examples |".to_string(),
        "|---|---:|---:|---:|---|".to_string(),
    ];
    for row in &bundle.rows {
        lines.push(format!(
            "| `{}` | `{}` | `{:.2}%` | `{:+.2}pp` | {} |",
            row.k_label,
            row.prime_hits,
            row.rate * 100.0,
            row.delta_vs_k00_pp,
            row.prime_examples
        ));
    }
    lines.join("\n")
}
