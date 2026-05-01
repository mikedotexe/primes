//! Canonical matched-control report for maintained membrane families.
//!
//! This example compares documented and maintained symmetric digit-template
//! families against random decimal integers that are:
//! - matched on exact decimal digit count
//! - filtered to be coprime to the same base
//!
//! The reporting math lives in `src/validation/matched_control.rs`; this file is
//! now only the CLI and presentation shell.
//!
//! Recommended audit run:
//! `cargo run --release --example membrane_vs_random -- --samples 5000 --max-seed-len 4 --json-out matched-control.json --csv-out matched-control.csv`

use primes::validation::matched_control::{
    format_p_like, run_cross_family_report_with_progress, summarize_reports, write_csv_export,
    write_json_export, MatchedControlDecision, MatchedControlRunSettings, DEFAULT_FDR,
    DEFAULT_MAX_SEED_LEN, DEFAULT_MIN_SEED_LEN, DEFAULT_SAMPLES,
    MAINTAINED_MATCHED_CONTROL_FAMILIES,
};
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let options = parse_args();
    let settings = options.settings;
    eprintln!(
        "Running matched-control report: {} samples, seed lengths {}..={} across {} families",
        settings.samples,
        settings.min_seed_len,
        settings.max_seed_len,
        MAINTAINED_MATCHED_CONTROL_FAMILIES.len()
    );
    let reports =
        run_cross_family_report_with_progress(&MAINTAINED_MATCHED_CONTROL_FAMILIES, settings, true);
    let summary = summarize_reports(&reports, settings);
    let mut archived_outputs = Vec::new();

    if let Some(path) = options.json_out {
        write_json_export(&path, &reports, &summary, settings).unwrap_or_else(|err| {
            eprintln!("Failed to write JSON export to {}: {err}", path.display());
            std::process::exit(1);
        });
        archived_outputs.push(format!("JSON: {}", path.display()));
    }

    if let Some(path) = options.csv_out {
        write_csv_export(&path, &reports, settings).unwrap_or_else(|err| {
            eprintln!("Failed to write CSV export to {}: {err}", path.display());
            std::process::exit(1);
        });
        archived_outputs.push(format!("CSV: {}", path.display()));
    }

    println!("Matched-Control Membrane Report");
    println!("{}", "=".repeat(112));
    println!("Maintained families:");
    for family in MAINTAINED_MATCHED_CONTROL_FAMILIES {
        println!(
            "  - {}: base {} ({}, {}) k=({}, {})",
            family.label, family.base, family.outer, family.inner, family.k_outer, family.k_inner
        );
    }
    println!();
    println!(
        "Sampling plan: {} samples per family, seed lengths {}..={}, FDR={:.3}",
        settings.samples, settings.min_seed_len, settings.max_seed_len, settings.fdr
    );
    println!("Control design: exact decimal digit matching + coprime-to-base filtering.");
    println!("Predeclared residual criterion:");
    println!("  pooled lift CI lower > 1.0, no BH-significant negative families,");
    println!("  and at least two distinct bases with BH-significant positive families.");
    println!();

    println!(
        "{:<23} {:>7} {:>21} {:>21} {:>21} {:>20} {:>8} {:>8} {:>11}",
        "family",
        "digits",
        "membrane rate",
        "control rate",
        "diff pp",
        "lift",
        "g",
        "p",
        "decision"
    );
    println!("{}", "-".repeat(112));

    for report in &reports {
        println!(
            "{:<23} {:>7.1} {:>6.2}% [{:>5.2},{:>5.2}] {:>6.2}% [{:>5.2},{:>5.2}] {:+6.2} [{:+5.2},{:+5.2}] {:>5.2}x [{:>4.2},{:>4.2}] {:>+8.3} {:>8} {:>11}",
            report.family.code(report.seed_len),
            report.mean_digits,
            report.membrane.rate * 100.0,
            report.membrane.ci.0 * 100.0,
            report.membrane.ci.1 * 100.0,
            report.control.rate * 100.0,
            report.control.ci.0 * 100.0,
            report.control.ci.1 * 100.0,
            report.diff * 100.0,
            report.diff_ci.0 * 100.0,
            report.diff_ci.1 * 100.0,
            report.lift,
            report.lift_ci.0,
            report.lift_ci.1,
            report.hedges_g,
            format_p_like(report.p_value),
            format!(
                "{} q={}",
                report.decision.as_str(),
                format_p_like(report.q_value)
            ),
        );
    }

    println!();
    println!("Aggregate by base");
    println!("{}", "-".repeat(112));
    println!(
        "{:<10} {:>9} {:>23} {:>23} {:>18} {:>10}",
        "base", "families", "membrane pooled", "control pooled", "lift", "q-signals"
    );
    println!("{}", "-".repeat(112));

    for base_summary in &summary.base_summaries {
        println!(
            "{:<10} {:>9} {:>6.2}% [{:>5.2},{:>5.2}] {:>6.2}% [{:>5.2},{:>5.2}] {:>5.2}x [{:>4.2},{:>4.2}] {:>4}/{:<4}",
            format!("base {}", base_summary.base),
            base_summary.families,
            base_summary.membrane.rate * 100.0,
            base_summary.membrane.ci.0 * 100.0,
            base_summary.membrane.ci.1 * 100.0,
            base_summary.control.rate * 100.0,
            base_summary.control.ci.0 * 100.0,
            base_summary.control.ci.1 * 100.0,
            base_summary.lift,
            base_summary.lift_ci.0,
            base_summary.lift_ci.1,
            base_summary.positive_q_families,
            base_summary.families,
        );
    }

    println!();
    println!("Overall summary");
    println!("{}", "-".repeat(112));
    println!(
        "Families tested: {}  |  positive-q: {}  |  negative-q: {}  |  positive-raw: {}  |  negative-raw: {}",
        summary.total_families,
        summary.positive_q,
        summary.negative_q,
        summary.positive_raw,
        summary.negative_raw
    );
    println!(
        "Distinct bases with positive-q families: {}",
        summary.positive_q_bases.len()
    );
    println!(
        "Pooled membrane rate: {:.2}% [{:.2}, {:.2}]",
        summary.pooled_membrane.rate * 100.0,
        summary.pooled_membrane.ci.0 * 100.0,
        summary.pooled_membrane.ci.1 * 100.0
    );
    println!(
        "Pooled control rate:  {:.2}% [{:.2}, {:.2}]",
        summary.pooled_control.rate * 100.0,
        summary.pooled_control.ci.0 * 100.0,
        summary.pooled_control.ci.1 * 100.0
    );
    println!(
        "Pooled lift:          {:.3}x [{:.3}, {:.3}]",
        summary.pooled_lift, summary.pooled_lift_ci.0, summary.pooled_lift_ci.1
    );

    println!();
    if summary.residual_criterion_met {
        println!("Residual criterion: met");
        println!(
            "Interpretation: this run supports a stable positive residual under matched controls."
        );
    } else if summary.positive_q > 0 {
        println!("Residual criterion: not met");
        println!(
            "Interpretation: some families stay positive after BH correction, but the cross-base residual is not yet stable enough."
        );
    } else if summary.positive_raw > 0 {
        println!("Residual criterion: not met");
        println!(
            "Interpretation: some positive families appear before multiplicity correction, but no multiplicity-robust residual survives this report."
        );
    } else {
        println!("Residual criterion: not met");
        println!(
            "Interpretation: this run does not detect a stable positive residual beyond matched coprime controls."
        );
    }

    if reports
        .iter()
        .any(|report| report.decision == MatchedControlDecision::NegativeQ)
    {
        println!("Caution: at least one family is significantly *worse* than the matched control after BH adjustment.");
    }
    println!("Use Gate B only in alignment with this output; do not treat raw density alone as mechanism evidence.");
    println!("Configured FDR threshold: {:.3}", settings.fdr);

    if !archived_outputs.is_empty() {
        println!("Archived outputs:");
        for output in archived_outputs {
            println!("  - {output}");
        }
    }
}

struct CliOptions {
    settings: MatchedControlRunSettings,
    json_out: Option<PathBuf>,
    csv_out: Option<PathBuf>,
}

fn parse_args() -> CliOptions {
    let mut settings = MatchedControlRunSettings::default();
    let mut json_out = None;
    let mut csv_out = None;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--samples" => settings.samples = parse_next(&mut args, "--samples"),
            "--min-seed-len" => {
                settings.min_seed_len = parse_next(&mut args, "--min-seed-len");
            }
            "--max-seed-len" => {
                settings.max_seed_len = parse_next(&mut args, "--max-seed-len");
            }
            "--fdr" => settings.fdr = parse_next(&mut args, "--fdr"),
            "--json-out" => {
                json_out = Some(PathBuf::from(parse_next::<String>(&mut args, "--json-out")));
            }
            "--csv-out" => {
                csv_out = Some(PathBuf::from(parse_next::<String>(&mut args, "--csv-out")));
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {arg}");
                print_help();
                std::process::exit(2);
            }
        }
    }

    if settings.samples < 2 {
        eprintln!("--samples must be at least 2");
        std::process::exit(2);
    }
    if settings.min_seed_len == 0 {
        eprintln!("--min-seed-len must be at least 1");
        std::process::exit(2);
    }
    if settings.min_seed_len > settings.max_seed_len {
        eprintln!("--min-seed-len must be <= --max-seed-len");
        std::process::exit(2);
    }
    if !(0.0..=1.0).contains(&settings.fdr) {
        eprintln!("--fdr must be in [0, 1]");
        std::process::exit(2);
    }

    CliOptions {
        settings,
        json_out,
        csv_out,
    }
}

fn print_help() {
    println!("Canonical cross-family matched-control report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example membrane_vs_random -- [options]");
    println!();
    println!("Options:");
    println!("  --samples <n>         Samples per family (default: {DEFAULT_SAMPLES})");
    println!(
        "  --min-seed-len <n>    Minimum base-digit seed length (default: {DEFAULT_MIN_SEED_LEN})"
    );
    println!(
        "  --max-seed-len <n>    Maximum base-digit seed length (default: {DEFAULT_MAX_SEED_LEN})"
    );
    println!("  --fdr <x>             Benjamini-Hochberg FDR threshold (default: {DEFAULT_FDR})");
    println!("  --json-out <path>     Write archival JSON bundle to the given path");
    println!("  --csv-out <path>      Write one-row-per-family CSV export to the given path");
}

fn parse_next<T>(args: &mut impl Iterator<Item = String>, flag: &str) -> T
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let value = args.next().unwrap_or_else(|| {
        eprintln!("Missing value for {flag}");
        std::process::exit(2);
    });
    value.parse::<T>().unwrap_or_else(|err| {
        eprintln!("Invalid value for {flag}: {value} ({err})");
        std::process::exit(2);
    })
}
