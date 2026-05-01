//! Comparative report joining midpoint-density and connector-scan data.
//!
//! This example puts three layers into one row-per-pair table:
//! 1. midpoint-window prime density around the arithmetic midpoint,
//! 2. insertion hit rates from the maintained connector scan,
//! 3. density-corrected residual ratios from the connector audit.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example comparative_signal_report
//! cargo run --example comparative_signal_report -- --json-out comparative_signal.json --csv-out comparative_signal.csv
//! ```

use num_bigint::BigUint;
use primes::{
    connector::{scan_single_digit_hits, small_primes_up_to, ConcatenationSystem},
    is_prime,
    validation::reporting::{export_timestamp_utc, write_csv_rows, write_json_pretty},
};
use serde::Serialize;
use std::{env, path::PathBuf};

const WIDTHS: &[u32] = &[5, 6, 7];
const DIGITS: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
const RESIDUE_MODULI: &[u32] = &[3, 9];
const DEFAULT_SMALL_PRIME_BOUND: u32 = 19;
const DEFAULT_MIDPOINT_RADIUS: u128 = 1000;
const COMPARATIVE_REPORT_EXPORT_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy)]
struct PairCase {
    short_label: &'static str,
    pair_label: &'static str,
    pair: ConcatenationSystem,
}

#[derive(Debug)]
struct ComparativeReportOptions {
    json_out: Option<PathBuf>,
    csv_out: Option<PathBuf>,
    small_prime_bound: u32,
    midpoint_radius: u128,
}

#[derive(Debug, Clone, Serialize)]
struct ComparativeReportSettings {
    widths: Vec<u32>,
    digits: Vec<u8>,
    residue_moduli: Vec<u32>,
    small_primes: Vec<u32>,
    midpoint_radius: u128,
}

#[derive(Debug, Clone, Serialize)]
struct ComparativeSignalRow {
    short_label: String,
    pair_label: String,
    left: u128,
    right: u128,
    left_is_prime: bool,
    right_is_prime: bool,
    left_digits: usize,
    right_digits: usize,
    fair_midpoint_candidate: bool,
    fair_midpoint_reason: String,
    midpoint: u128,
    midpoint_radius: u128,
    left_window_primes: usize,
    midpoint_window_primes: usize,
    right_window_primes: usize,
    endpoint_avg_window_primes: f64,
    midpoint_density: f64,
    endpoint_avg_density: f64,
    midpoint_lift_pct: f64,
    residue_admissible_candidates_per_direction: usize,
    forward_prime_hits: usize,
    reverse_prime_hits: usize,
    forward_post_filter_rate: f64,
    reverse_post_filter_rate: f64,
    forward_corrected_expected_hits: f64,
    reverse_corrected_expected_hits: f64,
    forward_corrected_ratio: f64,
    reverse_corrected_ratio: f64,
    corrected_expected_hit_delta: f64,
    corrected_residual_ratio_delta: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ComparativeReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ComparativeReportSettings,
    rows: Vec<ComparativeSignalRow>,
}

fn main() {
    let options = parse_args();
    let small_primes = small_primes_up_to(options.small_prime_bound);
    let settings = ComparativeReportSettings {
        widths: WIDTHS.to_vec(),
        digits: DIGITS.to_vec(),
        residue_moduli: RESIDUE_MODULI.to_vec(),
        small_primes: small_primes.clone(),
        midpoint_radius: options.midpoint_radius,
    };

    let pair_cases = [
        PairCase {
            short_label: "Midpoint membrane",
            pair_label: "Midpoint membrane (303050303 ∘ 307050703)",
            pair: ConcatenationSystem::new(303050303, 307050703),
        },
        PairCase {
            short_label: "Canonical connector",
            pair_label: "Canonical connector (10301 ∘ 3007003007003)",
            pair: ConcatenationSystem::new(10301, 3007003007003),
        },
        PairCase {
            short_label: "Zero-padded membrane",
            pair_label: "Zero-padded membrane (10301 ∘ 30305070305070303)",
            pair: ConcatenationSystem::new(10301, 30305070305070303),
        },
        PairCase {
            short_label: "Twin profile",
            pair_label: "Twin-prime profile (11 ∘ 13)",
            pair: ConcatenationSystem::new(11, 13),
        },
        PairCase {
            short_label: "Sophie profile",
            pair_label: "Sophie Germain profile (23 ∘ 47)",
            pair: ConcatenationSystem::new(23, 47),
        },
    ];

    let rows: Vec<_> = pair_cases
        .iter()
        .map(|case| build_row(case, &small_primes, options.midpoint_radius))
        .collect();

    println!("=== Comparative Signal Report ===\n");
    println!(
        "Midpoint window radius: {} | Connector widths {:?} | digits {:?} | residue moduli {:?} | small primes <= {}",
        options.midpoint_radius,
        WIDTHS,
        DIGITS,
        RESIDUE_MODULI,
        options.small_prime_bound
    );
    println!();

    render_table(&rows);
    let fair_rows: Vec<_> = rows
        .iter()
        .filter(|row| row.fair_midpoint_candidate)
        .cloned()
        .collect();
    println!();
    println!("Fair midpoint candidates");
    println!("------------------------");
    if fair_rows.is_empty() {
        println!("No rows meet the maintained fairness rule.");
    } else {
        render_table(&fair_rows);
    }
    let excluded_rows: Vec<_> = rows
        .iter()
        .filter(|row| !row.fair_midpoint_candidate)
        .collect();
    if !excluded_rows.is_empty() {
        println!();
        println!("Excluded from fair midpoint set:");
        for row in excluded_rows {
            println!("  - {}: {}", row.short_label, row.fair_midpoint_reason);
        }
    }
    println!();
    println!("Notes:");
    println!(
        "  midpoint lift compares midpoint-window density against the average of the left and right endpoint windows"
    );
    println!(
        "  midpoint lift is most meaningful for same-scale endpoint pairs; on highly disparate magnitudes it mostly reflects ordinary prime thinning"
    );
    println!(
        "  insertion rates come from the matched width-5..7 zero-padded single-digit scan after the exact mod-3/mod-9 filter"
    );
    println!(
        "  corrected ratios are observed hits divided by the density-corrected expectation after local small-prime conditioning"
    );
    println!(
        "  fair midpoint candidates require both endpoints to be prime and to have the same decimal digit length"
    );

    if let Some(path) = &options.json_out {
        let bundle = ComparativeReportBundle {
            export_version: COMPARATIVE_REPORT_EXPORT_VERSION,
            generated_at_utc: export_timestamp_utc(),
            settings: settings.clone(),
            rows: rows.clone(),
        };
        write_json_pretty(path, &bundle).unwrap_or_else(|err| {
            panic!("failed to write JSON export to {}: {err}", path.display())
        });
        println!("Wrote JSON export to {}", path.display());
    }

    if let Some(path) = &options.csv_out {
        write_csv_rows(path, &rows).unwrap_or_else(|err| {
            panic!("failed to write CSV export to {}: {err}", path.display())
        });
        println!("Wrote CSV export to {}", path.display());
    }
}

fn build_row(case: &PairCase, small_primes: &[u32], midpoint_radius: u128) -> ComparativeSignalRow {
    let summary = scan_single_digit_hits(case.pair, WIDTHS, DIGITS, RESIDUE_MODULI);
    let audit = summary.signal_audit(small_primes);
    let midpoint_stats = midpoint_density_stats(case.pair, midpoint_radius);
    let left_digits = decimal_digits(case.pair.left);
    let right_digits = decimal_digits(case.pair.right);
    let fair_midpoint_candidate = midpoint_stats.left_is_prime
        && midpoint_stats.right_is_prime
        && left_digits == right_digits;
    let fair_midpoint_reason = if !midpoint_stats.left_is_prime || !midpoint_stats.right_is_prime {
        "one endpoint is not prime".to_string()
    } else if left_digits != right_digits {
        format!("digit-length mismatch: {} vs {}", left_digits, right_digits)
    } else {
        "same-scale prime pair".to_string()
    };

    ComparativeSignalRow {
        short_label: case.short_label.to_string(),
        pair_label: case.pair_label.to_string(),
        left: case.pair.left,
        right: case.pair.right,
        left_is_prime: midpoint_stats.left_is_prime,
        right_is_prime: midpoint_stats.right_is_prime,
        left_digits,
        right_digits,
        fair_midpoint_candidate,
        fair_midpoint_reason,
        midpoint: midpoint_stats.midpoint,
        midpoint_radius,
        left_window_primes: midpoint_stats.left_window_primes,
        midpoint_window_primes: midpoint_stats.midpoint_window_primes,
        right_window_primes: midpoint_stats.right_window_primes,
        endpoint_avg_window_primes: midpoint_stats.endpoint_avg_window_primes,
        midpoint_density: midpoint_stats.midpoint_density,
        endpoint_avg_density: midpoint_stats.endpoint_avg_density,
        midpoint_lift_pct: midpoint_stats.midpoint_lift_pct,
        residue_admissible_candidates_per_direction: summary
            .forward
            .residue_admissible_candidates(),
        forward_prime_hits: summary.forward.prime_hits(),
        reverse_prime_hits: summary.reverse.prime_hits(),
        forward_post_filter_rate: summary.forward.post_filter_prime_rate(),
        reverse_post_filter_rate: summary.reverse.post_filter_prime_rate(),
        forward_corrected_expected_hits: audit.forward.small_prime_corrected_expected_hits,
        reverse_corrected_expected_hits: audit.reverse.small_prime_corrected_expected_hits,
        forward_corrected_ratio: audit.forward.observed_to_corrected_ratio,
        reverse_corrected_ratio: audit.reverse.observed_to_corrected_ratio,
        corrected_expected_hit_delta: audit.corrected_expected_hit_delta(),
        corrected_residual_ratio_delta: audit.corrected_residual_ratio_delta(),
    }
}

struct MidpointDensityStats {
    left_is_prime: bool,
    right_is_prime: bool,
    midpoint: u128,
    left_window_primes: usize,
    midpoint_window_primes: usize,
    right_window_primes: usize,
    endpoint_avg_window_primes: f64,
    midpoint_density: f64,
    endpoint_avg_density: f64,
    midpoint_lift_pct: f64,
}

fn midpoint_density_stats(pair: ConcatenationSystem, radius: u128) -> MidpointDensityStats {
    let left_is_prime = is_prime_u128(pair.left);
    let right_is_prime = is_prime_u128(pair.right);
    let midpoint = midpoint_u128(pair.left, pair.right);
    let left_window_primes = count_primes_in_window(pair.left, radius);
    let midpoint_window_primes = count_primes_in_window(midpoint, radius);
    let right_window_primes = count_primes_in_window(pair.right, radius);
    let window_size = (radius * 2 + 1) as f64;
    let left_density = left_window_primes as f64 / window_size;
    let midpoint_density = midpoint_window_primes as f64 / window_size;
    let right_density = right_window_primes as f64 / window_size;
    let endpoint_avg_density = (left_density + right_density) / 2.0;
    let endpoint_avg_window_primes = (left_window_primes as f64 + right_window_primes as f64) / 2.0;
    let midpoint_lift_pct = if endpoint_avg_density > 0.0 {
        (midpoint_density / endpoint_avg_density - 1.0) * 100.0
    } else {
        0.0
    };

    MidpointDensityStats {
        left_is_prime,
        right_is_prime,
        midpoint,
        left_window_primes,
        midpoint_window_primes,
        right_window_primes,
        endpoint_avg_window_primes,
        midpoint_density,
        endpoint_avg_density,
        midpoint_lift_pct,
    }
}

fn count_primes_in_window(center: u128, radius: u128) -> usize {
    let start = center.saturating_sub(radius);
    let end = center.saturating_add(radius);
    let mut count = 0usize;

    for value in start..=end {
        if is_prime_u128(value) {
            count += 1;
        }
    }

    count
}

fn is_prime_u128(value: u128) -> bool {
    is_prime(&BigUint::from(value))
}

fn midpoint_u128(left: u128, right: u128) -> u128 {
    left / 2 + right / 2 + ((left % 2 + right % 2) / 2)
}

fn decimal_digits(value: u128) -> usize {
    value.to_string().len()
}

fn render_table(rows: &[ComparativeSignalRow]) {
    let headers = [
        "Pair",
        "Ends",
        "Mid L/M/R",
        "Mid lift",
        "Ins F/R",
        "Rates F/R",
        "Corr F/R",
        "Corr d",
    ];

    let table_rows: Vec<[String; 8]> = rows
        .iter()
        .map(|row| {
            [
                row.short_label.clone(),
                format!("{}/{}", yn(row.left_is_prime), yn(row.right_is_prime)),
                format!(
                    "{}/{}/{}",
                    row.left_window_primes, row.midpoint_window_primes, row.right_window_primes
                ),
                format!("{:+.1}%", row.midpoint_lift_pct),
                format!("{}/{}", row.forward_prime_hits, row.reverse_prime_hits),
                format!(
                    "{:.2}%/{:.2}%",
                    row.forward_post_filter_rate * 100.0,
                    row.reverse_post_filter_rate * 100.0
                ),
                format!(
                    "{:.3}/{:.3}",
                    row.forward_corrected_ratio, row.reverse_corrected_ratio
                ),
                format!("{:+.3}", row.corrected_residual_ratio_delta),
            ]
        })
        .collect();

    let mut widths = headers.map(str::len);
    for row in &table_rows {
        for (index, cell) in row.iter().enumerate() {
            widths[index] = widths[index].max(cell.len());
        }
    }

    print_row(&headers, &widths);
    print_separator(&widths);
    for row in &table_rows {
        print_row(row, &widths);
    }
}

fn print_row<const N: usize>(cells: &[impl AsRef<str>; N], widths: &[usize; N]) {
    let rendered = cells
        .iter()
        .zip(widths.iter())
        .map(|(cell, width)| format!("{:<width$}", cell.as_ref(), width = width))
        .collect::<Vec<_>>()
        .join(" | ");
    println!("{rendered}");
}

fn print_separator<const N: usize>(widths: &[usize; N]) {
    let rendered = widths
        .iter()
        .map(|width| "-".repeat(*width))
        .collect::<Vec<_>>()
        .join("-+-");
    println!("{rendered}");
}

fn yn(value: bool) -> &'static str {
    if value {
        "Y"
    } else {
        "N"
    }
}

fn parse_args() -> ComparativeReportOptions {
    let mut args = env::args().skip(1);
    let mut json_out = None;
    let mut csv_out = None;
    let mut small_prime_bound = DEFAULT_SMALL_PRIME_BOUND;
    let mut midpoint_radius = DEFAULT_MIDPOINT_RADIUS;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--json-out" => {
                json_out = Some(PathBuf::from(parse_next::<String>(&mut args, "--json-out")));
            }
            "--csv-out" => {
                csv_out = Some(PathBuf::from(parse_next::<String>(&mut args, "--csv-out")));
            }
            "--small-prime-bound" => {
                small_prime_bound = parse_next::<u32>(&mut args, "--small-prime-bound");
            }
            "--midpoint-radius" => {
                midpoint_radius = parse_next::<u128>(&mut args, "--midpoint-radius");
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

    ComparativeReportOptions {
        json_out,
        csv_out,
        small_prime_bound,
        midpoint_radius,
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
    println!("Comparative midpoint/connector signal report");
    println!();
    println!("Usage:");
    println!("  cargo run --example comparative_signal_report -- [options]");
    println!();
    println!("Options:");
    println!("  --json-out <path>           Write archival JSON bundle to the given path");
    println!("  --csv-out <path>            Write one-row-per-pair CSV export to the given path");
    println!(
        "  --small-prime-bound <n>     Use all primes <= n in the density-correction layer (default: {DEFAULT_SMALL_PRIME_BOUND})"
    );
    println!(
        "  --midpoint-radius <n>       Use radius n for midpoint/end-window density counts (default: {DEFAULT_MIDPOINT_RADIUS})"
    );
}
