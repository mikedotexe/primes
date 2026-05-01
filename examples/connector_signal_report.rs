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

use primes::connector::{
    canonical_source_hits, scan_single_digit_hits, small_primes_up_to, ConcatenationSystem,
    Direction, DirectionSignalStats, PairScanSummary, PairSignalAudit, PositionSignalRow,
    CANONICAL_DOCUMENTED_FORWARD_HITS, CANONICAL_WIDTH5_HITS,
};
use primes::validation::reporting::{export_timestamp_utc, write_csv_rows, write_json_pretty};
use serde::Serialize;
use std::{collections::BTreeSet, env, path::PathBuf};

const WIDTHS: &[u32] = &[5, 6, 7];
const DIGITS: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
const RESIDUE_MODULI: &[u32] = &[3, 9];
const DEFAULT_SMALL_PRIME_BOUND: u32 = 19;
const DEFAULT_SWEEP_BOUNDS: &[u32] = &[5, 7, 11, 13, 17, 19, 23, 29, 31];
const CONNECTOR_SIGNAL_EXPORT_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy)]
struct PairCase {
    name: &'static str,
    pair: ConcatenationSystem,
}

#[derive(Debug)]
struct SignalReportOptions {
    json_out: Option<PathBuf>,
    csv_out: Option<PathBuf>,
    sweep_csv_out: Option<PathBuf>,
    small_prime_bound: u32,
}

#[derive(Debug, Clone)]
struct NamedPairReport {
    name: String,
    summary: PairScanSummary,
    audit: PairSignalAudit,
    sweep: Vec<ResidualSweepRow>,
}

#[derive(Debug, Clone, Serialize)]
struct SignalReportSettings {
    widths: Vec<u32>,
    digits: Vec<u8>,
    residue_moduli: Vec<u32>,
    small_primes: Vec<u32>,
}

#[derive(Debug, Clone, Serialize)]
struct CanonicalSourceCaseRow {
    width: u32,
    position: u32,
    digit: u8,
    direction: Direction,
    connector: String,
    value: String,
    source_class: String,
    matched_scan_prime_hit: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedPairReport {
    name: String,
    summary: PairScanSummary,
    audit: PairSignalAudit,
    residual_sweep: Vec<ResidualSweepRow>,
}

#[derive(Debug, Clone, Serialize)]
struct ComparisonRow {
    name: String,
    raw_hit_delta: isize,
    raw_rate_delta_pp: f64,
    corrected_expected_hit_delta: f64,
    corrected_residual_ratio_delta: f64,
    forward_corrected_ratio: f64,
    reverse_corrected_ratio: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ComparisonVerdict {
    raw_broader_law_survives: bool,
    corrected_broader_law_survives: bool,
    rows: Vec<ComparisonRow>,
}

#[derive(Debug, Clone, Serialize)]
struct ResidualSweepRow {
    pair_label: String,
    bound: u32,
    small_primes: Vec<u32>,
    forward_corrected_ratio: f64,
    reverse_corrected_ratio: f64,
    corrected_residual_ratio_delta: f64,
    corrected_expected_hit_delta: f64,
    forward_corrected_expected_hits: f64,
    reverse_corrected_expected_hits: f64,
    forward_corrected_poisson_z: f64,
    reverse_corrected_poisson_z: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ResidualSweepExportRow {
    pair_label: String,
    bound: u32,
    small_primes: String,
    forward_corrected_ratio: f64,
    reverse_corrected_ratio: f64,
    corrected_residual_ratio_delta: f64,
    corrected_expected_hit_delta: f64,
    forward_corrected_expected_hits: f64,
    reverse_corrected_expected_hits: f64,
    forward_corrected_poisson_z: f64,
    reverse_corrected_poisson_z: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ResidualSweepSummary {
    pair_label: String,
    negative_bounds: usize,
    positive_bounds: usize,
    zero_bounds: usize,
    min_delta: f64,
    max_delta: f64,
    sign_stable: bool,
}

#[derive(Debug, Clone, Serialize)]
struct SignalReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: SignalReportSettings,
    canonical_source_cases: Vec<CanonicalSourceCaseRow>,
    comparison: ComparisonVerdict,
    residual_sweep_summary: Vec<ResidualSweepSummary>,
    pairs: Vec<ExportedPairReport>,
}

#[derive(Debug, Clone, Serialize)]
struct PositionExportRow {
    pair_label: String,
    left: u128,
    right: u128,
    direction: Direction,
    width: u32,
    position: u32,
    residue_admissible_candidates: usize,
    prime_hits: usize,
    working_digits: String,
    naive_expected_hits: f64,
    small_prime_corrected_expected_hits: f64,
    observed_to_corrected_ratio: f64,
    direction_prime_hits: usize,
    direction_naive_expected_hits: f64,
    direction_corrected_expected_hits: f64,
    direction_observed_to_corrected_ratio: f64,
    direction_corrected_poisson_residual_z: f64,
    raw_hit_delta: isize,
    raw_rate_delta_pp: f64,
    corrected_expected_hit_delta: f64,
    corrected_residual_ratio_delta: f64,
}

fn main() {
    let options = parse_args();
    let small_primes = small_primes_up_to(options.small_prime_bound);
    let settings = SignalReportSettings {
        widths: WIDTHS.to_vec(),
        digits: DIGITS.to_vec(),
        residue_moduli: RESIDUE_MODULI.to_vec(),
        small_primes: small_primes.clone(),
    };

    println!("=== Connector Signal Report ===\n");
    println!("Exact layer: base-10 fixed-pair, zero-padded single-digit scans");
    println!(
        "Matched budget: widths {:?}, digits {:?}, residue moduli {:?}",
        WIDTHS, DIGITS, RESIDUE_MODULI
    );
    println!(
        "Density-aware follow-up: exact small-prime conditioning through {:?}\n",
        small_primes
    );

    let pair_cases = [
        PairCase {
            name: "Canonical pair (10301 ∘ 3007003007003)",
            pair: ConcatenationSystem::new(10301, 3007003007003),
        },
        PairCase {
            name: "Zero-padded membrane (10301 ∘ 30305070305070303)",
            pair: ConcatenationSystem::new(10301, 30305070305070303),
        },
        PairCase {
            name: "Twin-prime profile (11 ∘ 13)",
            pair: ConcatenationSystem::new(11, 13),
        },
        PairCase {
            name: "Sophie Germain profile (23 ∘ 47)",
            pair: ConcatenationSystem::new(23, 47),
        },
    ];

    let reports: Vec<_> = pair_cases
        .iter()
        .map(|case| {
            let summary = scan_single_digit_hits(case.pair, WIDTHS, DIGITS, RESIDUE_MODULI);
            let audit = summary.signal_audit(&small_primes);
            let sweep = build_residual_sweep(case.name, &summary);
            NamedPairReport {
                name: case.name.to_string(),
                summary,
                audit,
                sweep,
            }
        })
        .collect();

    let canonical = reports.first().expect("canonical report should exist");
    assert_canonical_source_case(&canonical.summary);
    let canonical_source_cases = build_canonical_source_rows(&canonical.summary);
    print_canonical_source_case(&canonical_source_cases);

    for report in &reports {
        print_pair_summary(&report.name, &report.summary, &report.audit);
    }

    let comparison = build_comparison_verdict(&reports);
    print_same_budget_comparison(&comparison);
    print_residual_sweep_summary(&reports);

    if let Some(path) = &options.json_out {
        let bundle = build_export_bundle(&settings, &canonical_source_cases, &comparison, &reports);
        write_json_pretty(path, &bundle).unwrap_or_else(|err| {
            panic!("failed to write JSON export to {}: {err}", path.display())
        });
        println!("Wrote JSON export to {}", path.display());
    }

    if let Some(path) = &options.csv_out {
        let rows = build_position_export_rows(&reports);
        write_csv_rows(path, &rows).unwrap_or_else(|err| {
            panic!("failed to write CSV export to {}: {err}", path.display())
        });
        println!("Wrote CSV export to {}", path.display());
    }

    if let Some(path) = &options.sweep_csv_out {
        let rows = build_sweep_export_rows(&reports);
        write_csv_rows(path, &rows).unwrap_or_else(|err| {
            panic!(
                "failed to write sweep CSV export to {}: {err}",
                path.display()
            )
        });
        println!("Wrote sweep CSV export to {}", path.display());
    }
}

fn assert_canonical_source_case(summary: &PairScanSummary) {
    let mod3 = summary
        .residue_profiles
        .iter()
        .find(|profile| profile.modulus == 3)
        .expect("missing mod-3 residue profile");
    let mod9 = summary
        .residue_profiles
        .iter()
        .find(|profile| profile.modulus == 9)
        .expect("missing mod-9 residue profile");

    assert_eq!(
        mod3.pair_residue, 1,
        "unexpected canonical pair residue mod 3"
    );
    assert_eq!(
        mod3.blocked_connector_residue, 2,
        "unexpected blocked connector class mod 3"
    );
    assert_eq!(
        mod9.pair_residue, 1,
        "unexpected canonical pair residue mod 9"
    );
    assert_eq!(
        mod9.blocked_connector_residue, 8,
        "unexpected blocked connector class mod 9"
    );
}

fn build_canonical_source_rows(summary: &PairScanSummary) -> Vec<CanonicalSourceCaseRow> {
    let observed_forward: BTreeSet<_> = summary
        .forward
        .hit_cases()
        .into_iter()
        .map(|hit| (hit.width, hit.position, hit.digit))
        .collect();

    canonical_source_hits()
        .into_iter()
        .map(|hit| {
            let connector = hit
                .connector_string()
                .unwrap_or_else(|| "<overflow>".to_string());
            let value = hit
                .concatenated_value()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "<overflow>".to_string());
            let documented =
                CANONICAL_DOCUMENTED_FORWARD_HITS
                    .iter()
                    .any(|&(width, position, digit)| {
                        width == hit.width && position == hit.position && digit == hit.digit
                    });

            CanonicalSourceCaseRow {
                width: hit.width,
                position: hit.position,
                digit: hit.digit,
                direction: hit.direction,
                connector,
                value,
                source_class: if documented {
                    "documented forward case".to_string()
                } else {
                    "shell-only source case".to_string()
                },
                matched_scan_prime_hit: observed_forward.contains(&(
                    hit.width,
                    hit.position,
                    hit.digit,
                )),
            }
        })
        .collect()
}

fn print_canonical_source_case(rows: &[CanonicalSourceCaseRow]) {
    println!("Canonical source case");
    println!("---------------------");
    println!("Agda width-5 shell cases:");
    for &(width, position, digit) in CANONICAL_WIDTH5_HITS {
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

fn build_comparison_verdict(reports: &[NamedPairReport]) -> ComparisonVerdict {
    let canonical = reports
        .first()
        .expect("comparison needs a canonical pair report");
    let canonical_raw = canonical.summary.directional_asymmetry();
    let canonical_corrected_sign = canonical.audit.corrected_residual_ratio_delta().signum();

    let rows: Vec<_> = reports
        .iter()
        .map(|report| {
            let asymmetry = report.summary.directional_asymmetry();
            ComparisonRow {
                name: report.name.clone(),
                raw_hit_delta: asymmetry.hit_delta(),
                raw_rate_delta_pp: asymmetry.post_filter_rate_delta() * 100.0,
                corrected_expected_hit_delta: report.audit.corrected_expected_hit_delta(),
                corrected_residual_ratio_delta: report.audit.corrected_residual_ratio_delta(),
                forward_corrected_ratio: report.audit.forward.observed_to_corrected_ratio,
                reverse_corrected_ratio: report.audit.reverse.observed_to_corrected_ratio,
            }
        })
        .collect();

    let raw_broader_law_survives = canonical_raw.hit_delta() != 0
        && canonical_raw.post_filter_rate_delta() != 0.0
        && reports.iter().skip(1).all(|report| {
            let asymmetry = report.summary.directional_asymmetry();
            asymmetry.hit_delta().signum() == canonical_raw.hit_delta().signum()
                && asymmetry.post_filter_rate_delta().signum()
                    == canonical_raw.post_filter_rate_delta().signum()
                && asymmetry.hit_delta() != 0
                && asymmetry.post_filter_rate_delta() != 0.0
        });

    let corrected_broader_law_survives = canonical_corrected_sign != 0.0
        && reports.iter().skip(1).all(|report| {
            let delta = report.audit.corrected_residual_ratio_delta();
            delta != 0.0 && delta.signum() == canonical_corrected_sign
        });

    ComparisonVerdict {
        raw_broader_law_survives,
        corrected_broader_law_survives,
        rows,
    }
}

fn print_same_budget_comparison(comparison: &ComparisonVerdict) {
    println!("Same-budget comparison verdict");
    println!("-----------------------------");
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

fn build_residual_sweep(pair_label: &str, summary: &PairScanSummary) -> Vec<ResidualSweepRow> {
    DEFAULT_SWEEP_BOUNDS
        .iter()
        .copied()
        .map(|bound| {
            let small_primes = small_primes_up_to(bound);
            let audit = summary.signal_audit(&small_primes);
            ResidualSweepRow {
                pair_label: pair_label.to_string(),
                bound,
                small_primes,
                forward_corrected_ratio: audit.forward.observed_to_corrected_ratio,
                reverse_corrected_ratio: audit.reverse.observed_to_corrected_ratio,
                corrected_residual_ratio_delta: audit.corrected_residual_ratio_delta(),
                corrected_expected_hit_delta: audit.corrected_expected_hit_delta(),
                forward_corrected_expected_hits: audit.forward.small_prime_corrected_expected_hits,
                reverse_corrected_expected_hits: audit.reverse.small_prime_corrected_expected_hits,
                forward_corrected_poisson_z: audit.forward.corrected_poisson_residual_z,
                reverse_corrected_poisson_z: audit.reverse.corrected_poisson_residual_z,
            }
        })
        .collect()
}

fn summarize_residual_sweep(report: &NamedPairReport) -> ResidualSweepSummary {
    let negative_bounds = report
        .sweep
        .iter()
        .filter(|row| row.corrected_residual_ratio_delta < 0.0)
        .count();
    let positive_bounds = report
        .sweep
        .iter()
        .filter(|row| row.corrected_residual_ratio_delta > 0.0)
        .count();
    let zero_bounds = report.sweep.len() - negative_bounds - positive_bounds;
    let min_delta = report
        .sweep
        .iter()
        .map(|row| row.corrected_residual_ratio_delta)
        .fold(f64::INFINITY, f64::min);
    let max_delta = report
        .sweep
        .iter()
        .map(|row| row.corrected_residual_ratio_delta)
        .fold(f64::NEG_INFINITY, f64::max);

    ResidualSweepSummary {
        pair_label: report.name.clone(),
        negative_bounds,
        positive_bounds,
        zero_bounds,
        min_delta,
        max_delta,
        sign_stable: (negative_bounds == report.sweep.len()
            || positive_bounds == report.sweep.len())
            && zero_bounds == 0,
    }
}

fn print_residual_sweep_summary(reports: &[NamedPairReport]) {
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

fn build_export_bundle(
    settings: &SignalReportSettings,
    canonical_source_cases: &[CanonicalSourceCaseRow],
    comparison: &ComparisonVerdict,
    reports: &[NamedPairReport],
) -> SignalReportBundle {
    SignalReportBundle {
        export_version: CONNECTOR_SIGNAL_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings: settings.clone(),
        canonical_source_cases: canonical_source_cases.to_vec(),
        comparison: comparison.clone(),
        residual_sweep_summary: reports.iter().map(summarize_residual_sweep).collect(),
        pairs: reports
            .iter()
            .map(|report| ExportedPairReport {
                name: report.name.clone(),
                summary: report.summary.clone(),
                audit: report.audit.clone(),
                residual_sweep: report.sweep.clone(),
            })
            .collect(),
    }
}

fn build_position_export_rows(reports: &[NamedPairReport]) -> Vec<PositionExportRow> {
    let mut rows = Vec::new();

    for report in reports {
        let asymmetry = report.summary.directional_asymmetry();

        for row in report
            .audit
            .forward_positions
            .iter()
            .chain(report.audit.reverse_positions.iter())
        {
            let direction_stats = match row.direction {
                Direction::Forward => &report.audit.forward,
                Direction::Reverse => &report.audit.reverse,
            };
            rows.push(position_export_row(
                report,
                row,
                direction_stats,
                &asymmetry,
            ));
        }
    }

    rows
}

fn build_sweep_export_rows(reports: &[NamedPairReport]) -> Vec<ResidualSweepExportRow> {
    reports
        .iter()
        .flat_map(|report| {
            report
                .sweep
                .iter()
                .map(|row| ResidualSweepExportRow {
                    pair_label: row.pair_label.clone(),
                    bound: row.bound,
                    small_primes: format!("{:?}", row.small_primes),
                    forward_corrected_ratio: row.forward_corrected_ratio,
                    reverse_corrected_ratio: row.reverse_corrected_ratio,
                    corrected_residual_ratio_delta: row.corrected_residual_ratio_delta,
                    corrected_expected_hit_delta: row.corrected_expected_hit_delta,
                    forward_corrected_expected_hits: row.forward_corrected_expected_hits,
                    reverse_corrected_expected_hits: row.reverse_corrected_expected_hits,
                    forward_corrected_poisson_z: row.forward_corrected_poisson_z,
                    reverse_corrected_poisson_z: row.reverse_corrected_poisson_z,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn position_export_row(
    report: &NamedPairReport,
    row: &PositionSignalRow,
    direction_stats: &DirectionSignalStats,
    asymmetry: &primes::connector::DirectionalAsymmetry,
) -> PositionExportRow {
    PositionExportRow {
        pair_label: report.name.clone(),
        left: report.summary.pair.left,
        right: report.summary.pair.right,
        direction: row.direction,
        width: row.width,
        position: row.position,
        residue_admissible_candidates: row.residue_admissible_candidates,
        prime_hits: row.prime_hits,
        working_digits: format!("{:?}", row.working_digits),
        naive_expected_hits: row.naive_expected_hits,
        small_prime_corrected_expected_hits: row.small_prime_corrected_expected_hits,
        observed_to_corrected_ratio: row.observed_to_corrected_ratio,
        direction_prime_hits: direction_stats.prime_hits,
        direction_naive_expected_hits: direction_stats.naive_expected_hits,
        direction_corrected_expected_hits: direction_stats.small_prime_corrected_expected_hits,
        direction_observed_to_corrected_ratio: direction_stats.observed_to_corrected_ratio,
        direction_corrected_poisson_residual_z: direction_stats.corrected_poisson_residual_z,
        raw_hit_delta: asymmetry.hit_delta(),
        raw_rate_delta_pp: asymmetry.post_filter_rate_delta() * 100.0,
        corrected_expected_hit_delta: report.audit.corrected_expected_hit_delta(),
        corrected_residual_ratio_delta: report.audit.corrected_residual_ratio_delta(),
    }
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
