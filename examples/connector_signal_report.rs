//! Connector signal report for the maintained arithmetic-first lane.
//!
//! This report does four things:
//! 1. reconstructs the canonical source cases from the current Rust and Agda
//!    shells,
//! 2. prints the exact direction-independent residue layer used by the matched
//!    scan,
//! 3. audits observed hits against density-aware expectations,
//! 4. optionally exports JSON and CSV artifacts for constructive follow-up.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example connector_signal_report
//! cargo run --example connector_signal_report -- --json-out connector_signal.json --csv-out connector_signal_positions.csv
//! ```

use primes::connector::{DirectionSignalStats, PairScanSummary, PairSignalAudit};
use primes::validation::{
    connector_signal::{
        build_connector_signal_analysis, build_position_export_rows, build_signal_report_bundle,
        build_sweep_export_rows, canonical_width5_hits, summarize_residual_sweep,
        CanonicalSourceCaseRow, ComparisonVerdict, DEFAULT_SMALL_PRIME_BOUND, DEFAULT_SWEEP_BOUNDS,
        DIGITS, RESIDUE_MODULI, WIDTHS,
    },
    reporting::{write_csv_rows, write_json_pretty},
};
use std::{env, path::PathBuf};

#[derive(Debug)]
struct SignalReportOptions {
    json_out: Option<PathBuf>,
    csv_out: Option<PathBuf>,
    sweep_csv_out: Option<PathBuf>,
    small_prime_bound: u32,
}

fn main() {
    let options = parse_args();
    let analysis = build_connector_signal_analysis(options.small_prime_bound);

    println!("=== Connector Signal Report ===\n");
    println!("Exact layer: base-10 fixed-pair, zero-padded single-digit scans");
    println!(
        "Matched budget: widths {:?}, digits {:?}, residue moduli {:?}",
        WIDTHS, DIGITS, RESIDUE_MODULI
    );
    println!(
        "Density-aware follow-up: exact small-prime conditioning through {:?}\n",
        analysis.settings.small_primes
    );

    print_canonical_source_case(&analysis.canonical_source_cases);

    for report in &analysis.reports {
        print_pair_summary(&report.name, &report.summary, &report.audit);
    }

    print_same_budget_comparison(&analysis.comparison);
    print_residual_sweep_summary(&analysis.reports);

    if let Some(path) = &options.json_out {
        let bundle = build_signal_report_bundle(&analysis);
        write_json_pretty(path, &bundle).unwrap_or_else(|err| {
            panic!("failed to write JSON export to {}: {err}", path.display())
        });
        println!("Wrote JSON export to {}", path.display());
    }

    if let Some(path) = &options.csv_out {
        let rows = build_position_export_rows(&analysis.reports);
        write_csv_rows(path, &rows).unwrap_or_else(|err| {
            panic!("failed to write CSV export to {}: {err}", path.display())
        });
        println!("Wrote CSV export to {}", path.display());
    }

    if let Some(path) = &options.sweep_csv_out {
        let rows = build_sweep_export_rows(&analysis.reports);
        write_csv_rows(path, &rows).unwrap_or_else(|err| {
            panic!(
                "failed to write sweep CSV export to {}: {err}",
                path.display()
            )
        });
        println!("Wrote sweep CSV export to {}", path.display());
    }
}

fn print_canonical_source_case(rows: &[CanonicalSourceCaseRow]) {
    println!("Canonical source case");
    println!("---------------------");
    println!("Agda width-5 shell cases:");
    for &(width, position, digit) in canonical_width5_hits() {
        println!("  - width {} position {} digit {}", width, position, digit);
    }
    println!("Documented forward source cases:");
    for row in rows {
        println!(
            "  - width {} position {} digit {} direction {} => {} => {} [{}; {}]",
            row.width,
            row.position,
            row.digit,
            row.direction,
            row.connector,
            row.value,
            row.source_class,
            if row.matched_scan_prime_hit {
                "matched scan prime hit"
            } else {
                "not a matched scan prime hit"
            }
        );
    }
    println!();
}

fn print_pair_summary(name: &str, summary: &PairScanSummary, audit: &PairSignalAudit) {
    println!("{name}");
    println!("{}", "-".repeat(name.len()));

    print!("Residue profiles:");
    for profile in &summary.residue_profiles {
        print!(
            " mod {} => pair residue {}, blocked connector class {}",
            profile.modulus, profile.pair_residue, profile.blocked_connector_residue
        );
    }
    println!();

    println!(
        "  forward: raw {:>3}, admissible {:>3}, prime hits {:>3}, post-filter rate {:>6.2}%",
        summary.forward.raw_candidates(),
        summary.forward.residue_admissible_candidates(),
        summary.forward.prime_hits(),
        summary.forward.post_filter_prime_rate() * 100.0
    );
    print_direction_audit("forward", &audit.forward);

    println!(
        "  reverse: raw {:>3}, admissible {:>3}, prime hits {:>3}, post-filter rate {:>6.2}%",
        summary.reverse.raw_candidates(),
        summary.reverse.residue_admissible_candidates(),
        summary.reverse.prime_hits(),
        summary.reverse.post_filter_prime_rate() * 100.0
    );
    print_direction_audit("reverse", &audit.reverse);

    let asymmetry = summary.directional_asymmetry();
    println!(
        "  asymmetry: hit delta {:>3}, post-filter rate delta {:>6.2}pp",
        asymmetry.hit_delta(),
        asymmetry.post_filter_rate_delta() * 100.0
    );
    println!(
        "  residual asymmetry: corrected expected-hit delta {:>7.3}, corrected ratio delta {:>7.3}",
        audit.corrected_expected_hit_delta(),
        audit.corrected_residual_ratio_delta()
    );

    let resonance = summary.resonance_positions();
    if resonance.is_empty() {
        println!("  resonance positions: none in this matched scan");
    } else {
        println!("  resonance positions:");
        for position in resonance {
            println!(
                "    - {} width {} position {} digits {:?}",
                position.direction, position.width, position.position, position.digits
            );
        }
    }

    println!();
}

fn print_direction_audit(label: &str, stats: &DirectionSignalStats) {
    println!(
        "    {label} density baseline: naive expected {:>7.3}, corrected expected {:>7.3}",
        stats.naive_expected_hits, stats.small_prime_corrected_expected_hits
    );
    println!(
        "    {label} residuals: observed/naive {:>6.3}, observed/corrected {:>6.3}, poisson z {:>6.2}",
        stats.observed_to_naive_ratio,
        stats.observed_to_corrected_ratio,
        stats.corrected_poisson_residual_z
    );
    println!(
        "    {label} local sieve: survivors {:>3}/{:>3} ({:>6.2}%), random reference {:>6.2}%, factor {:>6.3}",
        stats.joint_small_prime_survivor_count,
        stats.residue_admissible_candidates,
        stats.joint_small_prime_survival_share * 100.0,
        stats.random_joint_survival_share * 100.0,
        stats.local_correction_factor
    );

    let mut blockers = stats.small_prime_profiles.clone();
    blockers.sort_by(|left, right| {
        right
            .blocked_share
            .total_cmp(&left.blocked_share)
            .then(left.prime.cmp(&right.prime))
    });
    let top_blockers = blockers
        .into_iter()
        .take(3)
        .map(|profile| format!("p{}={:.2}%", profile.prime, profile.blocked_share * 100.0))
        .collect::<Vec<_>>()
        .join(", ");
    println!("    {label} top blockers: {}", top_blockers);
}

fn print_same_budget_comparison(comparison: &ComparisonVerdict) {
    println!("Same-budget comparison verdict");
    println!("-----------------------------");
    println!(
        "Analytic guardrail: λ-style expected-hit baselines map monotonically to Poisson coverage; residual direction gaps remain empirical, not mechanism claims."
    );
    if comparison.raw_broader_law_survives {
        println!(
            "Raw-count broader-law candidate survives this first comparison on hit and post-filter-rate signs."
        );
    } else {
        println!(
            "Raw-count broader-law candidate does not survive this first comparison; treat it as an open heuristic."
        );
    }
    if comparison.corrected_broader_law_survives {
        println!(
            "Density-corrected residual asymmetry also survives this first comparison on sign."
        );
    } else {
        println!(
            "Density-corrected residual asymmetry does not survive this first comparison; the constructive-disproof baseline remains stronger than a general law."
        );
    }
    println!();
    println!("Per-pair asymmetry deltas:");
    for row in &comparison.rows {
        println!(
            "  - {}: raw hit delta {}, raw rate delta {:.2}pp, corrected expected-hit delta {:.3}, corrected ratio delta {:.3}",
            row.name,
            row.raw_hit_delta,
            row.raw_rate_delta_pp,
            row.corrected_expected_hit_delta,
            row.corrected_residual_ratio_delta
        );
    }
    println!();
}

fn print_residual_sweep_summary(reports: &[primes::validation::connector_signal::NamedPairReport]) {
    println!("Residual stability sweep");
    println!("------------------------");
    println!("Small-prime bounds checked: {:?}", DEFAULT_SWEEP_BOUNDS);
    for report in reports {
        let summary = summarize_residual_sweep(report);
        println!(
            "  - {}: neg {}, pos {}, zero {}, delta range [{:.3}, {:.3}]{}",
            summary.pair_label,
            summary.negative_bounds,
            summary.positive_bounds,
            summary.zero_bounds,
            summary.min_delta,
            summary.max_delta,
            if summary.sign_stable {
                ", sign-stable"
            } else {
                ""
            }
        );
    }
    println!();
}

fn parse_args() -> SignalReportOptions {
    let mut args = env::args().skip(1);
    let mut json_out = None;
    let mut csv_out = None;
    let mut sweep_csv_out = None;
    let mut small_prime_bound = DEFAULT_SMALL_PRIME_BOUND;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--json-out" => {
                json_out = Some(PathBuf::from(parse_next::<String>(&mut args, "--json-out")));
            }
            "--csv-out" => {
                csv_out = Some(PathBuf::from(parse_next::<String>(&mut args, "--csv-out")));
            }
            "--sweep-csv-out" => {
                sweep_csv_out = Some(PathBuf::from(parse_next::<String>(
                    &mut args,
                    "--sweep-csv-out",
                )));
            }
            "--small-prime-bound" => {
                small_prime_bound = parse_next::<u32>(&mut args, "--small-prime-bound");
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

    SignalReportOptions {
        json_out,
        csv_out,
        sweep_csv_out,
        small_prime_bound,
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
    println!("Connector signal report");
    println!();
    println!("Usage:");
    println!("  cargo run --example connector_signal_report -- [options]");
    println!();
    println!("Options:");
    println!("  --json-out <path>           Write archival JSON bundle to the given path");
    println!(
        "  --csv-out <path>            Write one-row-per-position CSV export to the given path"
    );
    println!("  --sweep-csv-out <path>      Write one-row-per-pair-per-bound residual sweep CSV");
    println!(
        "  --small-prime-bound <n>     Use all primes <= n in the density-correction layer (default: {DEFAULT_SMALL_PRIME_BOUND})"
    );
}
