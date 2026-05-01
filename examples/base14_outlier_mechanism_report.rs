//! Base-14 mechanism report for the `M=2` bounded-`k` outliers.
//!
//! This report is intentionally local. It does not rescan the whole search
//! space. Instead it reads the maintained base-14 atlas artifact, locks the
//! four active `M=2` pairs and their rank-1 nearby dead controls, and explains
//! each win over `k=(0,0)` with two exact pieces:
//! - admissible-set change
//! - prime-yield change among admissible candidates
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base14_outlier_mechanism_report
//! cargo run --release --example base14_outlier_mechanism_report -- --input-json /tmp/primes_base14_survivor_atlas/summary.json --out-dir /tmp/primes_base14_outlier_mechanism
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        digit_symbol, evaluate_pair_row, format_k, parse_k_label, render_divisibility_mask,
        scan_k_config_mask_profile, BoundedKConfig, KConfigCandidateMaskRow,
        KConfigMaskHistogramRow, KConfigMaskProfile, DEFAULT_BOUNDED_K_GRID,
        DEFAULT_PREFILTER_PRIMES,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};

const BASE: u32 = 14;
const MIDDLE_LENGTH: usize = 2;
const DEFAULT_INPUT_JSON: &str = "/tmp/primes_base14_survivor_atlas/summary.json";
const DEFAULT_OUT_DIR: &str = "/tmp/primes_base14_outlier_mechanism";
const REPORT_EXPORT_VERSION: u32 = 1;
const EXPECTED_ACTIVE_PAIRS: &[&str] = &["(3,1)", "(9,B)", "(D,5)", "(D,B)"];
const APPENDIX_OUTLIERS: &[(u32, u32, u32)] = &[(6, 5, 5), (10, 1, 7), (12, 11, 1)];

#[derive(Debug)]
struct Options {
    input_json: PathBuf,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct AtlasInputRow {
    pair_label: String,
    atlas_role: String,
    rank_within_anchor: usize,
    best_k_m2: String,
}

#[derive(Debug, Clone, Deserialize)]
struct AtlasNeighborRow {
    survivor_pair: String,
    control_pair: String,
    comparison_rank: usize,
    unit_distance: usize,
    same_gap_bucket: bool,
    same_same_digit: bool,
    same_best_k_m1: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct AtlasInputBundle {
    generated_at_utc: String,
    atlas_rows: Vec<AtlasInputRow>,
    neighbor_rows: Vec<AtlasNeighborRow>,
}

#[derive(Debug, Clone)]
struct ActivePairSpec {
    pair_label: String,
    atlas_role: String,
    outer: u32,
    inner: u32,
    best_k_m2: BoundedKConfig,
    control_pair: String,
    control_outer: u32,
    control_inner: u32,
    control_rank: usize,
    control_distance: usize,
    control_same_gap_bucket: bool,
    control_same_same_digit: bool,
    control_same_best_k_m1: bool,
}

#[derive(Debug, Clone)]
struct ModulusReliefRow {
    modulus: u32,
    relief_pp: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CandidateTransferCategory {
    SharedInadmissible,
    InadmissibleToAdmissible,
    AdmissibleToInadmissible,
    SharedAdmissible,
}

impl CandidateTransferCategory {
    fn label(self) -> &'static str {
        match self {
            Self::SharedInadmissible => "shared_inadmissible",
            Self::InadmissibleToAdmissible => "inadmissible_to_admissible",
            Self::AdmissibleToInadmissible => "admissible_to_inadmissible",
            Self::SharedAdmissible => "shared_admissible",
        }
    }
}

#[derive(Debug, Clone)]
struct PairMechanismAnalysis {
    k00_profile: KConfigMaskProfile,
    best_profile: KConfigMaskProfile,
    transfer_rows: Vec<CandidateTransferRow>,
    histogram_rows: Vec<MaskHistogramDeltaRow>,
    candidate_rows: Vec<CandidateMaskExportRow>,
    top_moduli_m2: String,
    explanation_label: String,
    anomaly_m2_pp: f64,
    prime_hit_delta_count: isize,
    prime_hit_delta_pp: f64,
    admissible_delta_count: isize,
    admissible_delta_pp: f64,
    zero_mask_delta_count: isize,
    zero_mask_delta_pp: f64,
    prime_yield_k00_pp: f64,
    prime_yield_best_pp: f64,
    prime_yield_delta_pp: f64,
    admissible_set_effect_pp: f64,
    prime_yield_effect_pp: f64,
    total_positive_relief_pp: f64,
    total_negative_relief_pp: f64,
    net_relief_pp: f64,
    zero_positive_singleton_relief: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    input_json: String,
    out_dir: String,
    base: u32,
    middle_length: usize,
    prefilter_moduli: Vec<u32>,
    appendix_pairs: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct MechanismRow {
    pair_label: String,
    atlas_role: String,
    best_k_m2: String,
    control_pair: String,
    control_rank: usize,
    control_distance: usize,
    control_same_gap_bucket: bool,
    control_same_same_digit: bool,
    control_same_best_k_m1: bool,
    explanation_label: String,
    stress_case: bool,
    admissible_count_k00: usize,
    admissible_count_best: usize,
    admissible_delta_count: isize,
    admissible_delta_pp: f64,
    zero_mask_delta_count: isize,
    zero_mask_delta_pp: f64,
    prime_hits_k00: usize,
    prime_hits_best: usize,
    prime_hit_delta_count: isize,
    prime_hit_delta_pp: f64,
    prime_yield_k00_pp: f64,
    prime_yield_best_pp: f64,
    prime_yield_delta_pp: f64,
    admissible_set_effect_pp: f64,
    prime_yield_effect_pp: f64,
    total_positive_relief_pp: f64,
    total_negative_relief_pp: f64,
    net_relief_pp: f64,
    zero_positive_singleton_relief: bool,
    top_moduli_m2: String,
}

#[derive(Debug, Clone, Serialize)]
struct ControlReferenceRow {
    survivor_pair: String,
    control_pair: String,
    control_rank: usize,
    unit_distance: usize,
    same_gap_bucket: bool,
    same_same_digit: bool,
    same_best_k_m1: bool,
    control_best_k_m2: String,
    control_anomaly_m2_pp: f64,
    control_admissible_delta_m2_pp: f64,
    control_prime_hits_k00: usize,
    control_prime_hits_best: usize,
}

#[derive(Debug, Clone, Serialize)]
struct CandidateTransferRow {
    pair_label: String,
    category: String,
    middle_index_count: usize,
    share_of_candidates_pp: f64,
    k00_admissible_count: usize,
    best_admissible_count: usize,
    k00_prime_count: usize,
    best_prime_count: usize,
    k00_composite_count: usize,
    best_composite_count: usize,
    prime_delta_count: isize,
}

#[derive(Debug, Clone, Serialize)]
struct MaskHistogramDeltaRow {
    pair_label: String,
    divisibility_mask: u16,
    mask_label: String,
    k00_count: usize,
    best_count: usize,
    delta_count: isize,
    k00_prime_count: usize,
    best_prime_count: usize,
    delta_prime_count: isize,
}

#[derive(Debug, Clone, Serialize)]
struct CandidateMaskExportRow {
    pair_label: String,
    lane: String,
    middle_index: u32,
    middle_digits: String,
    decimal_value: String,
    divisibility_mask: u16,
    mask_label: String,
    admissible: bool,
    prime: bool,
}

#[derive(Debug, Clone, Serialize)]
struct AppendixRow {
    base: u32,
    pair_label: String,
    best_k_m2: String,
    explanation_label: String,
    anomaly_m2_pp: f64,
    admissible_delta_pp: f64,
    zero_mask_delta_pp: f64,
    prime_yield_delta_pp: f64,
    admissible_set_effect_pp: f64,
    prime_yield_effect_pp: f64,
    total_positive_relief_pp: f64,
    total_negative_relief_pp: f64,
    net_relief_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    active_pairs: usize,
    active_pair_labels: String,
    yield_dominated_pairs: usize,
    mixed_pairs: usize,
    overlap_lift_pairs: usize,
    zero_positive_singleton_pairs: usize,
    appendix_rows: usize,
    stress_case_pair: String,
}

#[derive(Debug, Clone, Serialize)]
struct OutputBundle {
    export_version: u32,
    generated_at_utc: String,
    input_json: String,
    input_generated_at_utc: String,
    settings: ReportSettings,
    mechanism_rows: Vec<MechanismRow>,
    control_reference_rows: Vec<ControlReferenceRow>,
    candidate_transfer_rows: Vec<CandidateTransferRow>,
    mask_histogram_delta_rows: Vec<MaskHistogramDeltaRow>,
    candidate_mask_rows: Vec<CandidateMaskExportRow>,
    appendix_rows: Vec<AppendixRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let input_bundle = load_atlas_bundle(&options.input_json);
    let active_specs = build_active_specs(&input_bundle);

    let mut mechanism_rows = Vec::new();
    let mut control_reference_rows = Vec::new();
    let mut candidate_transfer_rows = Vec::new();
    let mut mask_histogram_delta_rows = Vec::new();
    let mut candidate_mask_rows = Vec::new();

    for spec in &active_specs {
        let analysis = analyze_pair(BASE, MIDDLE_LENGTH, spec.outer, spec.inner, spec.best_k_m2);
        mechanism_rows.push(MechanismRow {
            pair_label: spec.pair_label.clone(),
            atlas_role: spec.atlas_role.clone(),
            best_k_m2: format_k(spec.best_k_m2),
            control_pair: spec.control_pair.clone(),
            control_rank: spec.control_rank,
            control_distance: spec.control_distance,
            control_same_gap_bucket: spec.control_same_gap_bucket,
            control_same_same_digit: spec.control_same_same_digit,
            control_same_best_k_m1: spec.control_same_best_k_m1,
            explanation_label: analysis.explanation_label.clone(),
            stress_case: spec.pair_label == "(D,B)",
            admissible_count_k00: analysis.k00_profile.admissible_count,
            admissible_count_best: analysis.best_profile.admissible_count,
            admissible_delta_count: analysis.admissible_delta_count,
            admissible_delta_pp: analysis.admissible_delta_pp,
            zero_mask_delta_count: analysis.zero_mask_delta_count,
            zero_mask_delta_pp: analysis.zero_mask_delta_pp,
            prime_hits_k00: analysis.k00_profile.prime_hits,
            prime_hits_best: analysis.best_profile.prime_hits,
            prime_hit_delta_count: analysis.prime_hit_delta_count,
            prime_hit_delta_pp: analysis.prime_hit_delta_pp,
            prime_yield_k00_pp: analysis.prime_yield_k00_pp,
            prime_yield_best_pp: analysis.prime_yield_best_pp,
            prime_yield_delta_pp: analysis.prime_yield_delta_pp,
            admissible_set_effect_pp: analysis.admissible_set_effect_pp,
            prime_yield_effect_pp: analysis.prime_yield_effect_pp,
            total_positive_relief_pp: analysis.total_positive_relief_pp,
            total_negative_relief_pp: analysis.total_negative_relief_pp,
            net_relief_pp: analysis.net_relief_pp,
            zero_positive_singleton_relief: analysis.zero_positive_singleton_relief,
            top_moduli_m2: analysis.top_moduli_m2.clone(),
        });
        candidate_transfer_rows.extend(analysis.transfer_rows);
        mask_histogram_delta_rows.extend(analysis.histogram_rows);
        candidate_mask_rows.extend(analysis.candidate_rows);

        let control_row = evaluate_pair_row(
            BASE,
            MIDDLE_LENGTH,
            spec.control_outer,
            spec.control_inner,
            DEFAULT_BOUNDED_K_GRID,
        );
        let control_analysis = analyze_pair(
            BASE,
            MIDDLE_LENGTH,
            spec.control_outer,
            spec.control_inner,
            parse_k_label(&control_row.best_k),
        );
        control_reference_rows.push(ControlReferenceRow {
            survivor_pair: spec.pair_label.clone(),
            control_pair: spec.control_pair.clone(),
            control_rank: spec.control_rank,
            unit_distance: spec.control_distance,
            same_gap_bucket: spec.control_same_gap_bucket,
            same_same_digit: spec.control_same_same_digit,
            same_best_k_m1: spec.control_same_best_k_m1,
            control_best_k_m2: control_row.best_k,
            control_anomaly_m2_pp: control_analysis.anomaly_m2_pp,
            control_admissible_delta_m2_pp: control_analysis.admissible_delta_pp,
            control_prime_hits_k00: control_analysis.k00_profile.prime_hits,
            control_prime_hits_best: control_analysis.best_profile.prime_hits,
        });
    }

    mechanism_rows.sort_by(|left, right| left.pair_label.cmp(&right.pair_label));
    control_reference_rows.sort_by(|left, right| left.survivor_pair.cmp(&right.survivor_pair));
    candidate_transfer_rows.sort_by(|left, right| {
        left.pair_label.cmp(&right.pair_label).then_with(|| {
            transfer_sort_key(&left.category).cmp(&transfer_sort_key(&right.category))
        })
    });
    mask_histogram_delta_rows.sort_by(|left, right| {
        left.pair_label
            .cmp(&right.pair_label)
            .then_with(|| left.divisibility_mask.cmp(&right.divisibility_mask))
    });
    candidate_mask_rows.sort_by(|left, right| {
        left.pair_label
            .cmp(&right.pair_label)
            .then_with(|| left.lane.cmp(&right.lane))
            .then_with(|| left.middle_index.cmp(&right.middle_index))
    });

    let appendix_rows = build_appendix_rows();
    let plot_path = options.out_dir.join("base14_decomposition_plane.png");
    render_decomposition_plane(&mechanism_rows, &plot_path);

    let image_artifact_rows = vec![ImageArtifactRow {
        kind: "decomposition_plane".to_string(),
        label: "Base 14 admissible-delta vs prime-yield-delta plane".to_string(),
        path: plot_path.display().to_string(),
    }];
    let report_summary = build_report_summary(&mechanism_rows, &appendix_rows);
    let observations = derive_observations(&mechanism_rows, &appendix_rows);

    let bundle = OutputBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        input_json: options.input_json.display().to_string(),
        input_generated_at_utc: input_bundle.generated_at_utc,
        settings: ReportSettings {
            input_json: options.input_json.display().to_string(),
            out_dir: options.out_dir.display().to_string(),
            base: BASE,
            middle_length: MIDDLE_LENGTH,
            prefilter_moduli: DEFAULT_PREFILTER_PRIMES.to_vec(),
            appendix_pairs: APPENDIX_OUTLIERS
                .iter()
                .map(|&(base, outer, inner)| {
                    format!("base {} {}", base, format_pair_label(base, outer, inner))
                })
                .collect(),
        },
        mechanism_rows: mechanism_rows.clone(),
        control_reference_rows: control_reference_rows.clone(),
        candidate_transfer_rows: candidate_transfer_rows.clone(),
        mask_histogram_delta_rows: mask_histogram_delta_rows.clone(),
        candidate_mask_rows: candidate_mask_rows.clone(),
        appendix_rows: appendix_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(options.out_dir.join("mechanism_rows.csv"), &mechanism_rows)
        .expect("failed to write mechanism_rows.csv");
    write_csv_rows(
        options.out_dir.join("control_reference_rows.csv"),
        &control_reference_rows,
    )
    .expect("failed to write control_reference_rows.csv");
    write_csv_rows(
        options.out_dir.join("candidate_transfer_rows.csv"),
        &candidate_transfer_rows,
    )
    .expect("failed to write candidate_transfer_rows.csv");
    write_csv_rows(
        options.out_dir.join("mask_histogram_delta_rows.csv"),
        &mask_histogram_delta_rows,
    )
    .expect("failed to write mask_histogram_delta_rows.csv");
    write_csv_rows(
        options.out_dir.join("candidate_mask_rows.csv"),
        &candidate_mask_rows,
    )
    .expect("failed to write candidate_mask_rows.csv");
    write_csv_rows(options.out_dir.join("appendix_rows.csv"), &appendix_rows)
        .expect("failed to write appendix_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");

    let markdown = render_markdown(&bundle);
    write_text_file(options.out_dir.join("report.md"), &markdown)
        .expect("failed to write report.md");

    println!("Base-14 outlier mechanism report");
    println!("  input atlas: {}", options.input_json.display());
    println!("  output dir:  {}", options.out_dir.display());
    for row in &mechanism_rows {
        println!(
            "  {} -> {} | {} | anomaly {:+.2}pp | admissible {:+.2}pp | yield {:+.2}pp",
            row.pair_label,
            row.control_pair,
            row.explanation_label,
            row.prime_hit_delta_pp,
            row.admissible_delta_pp,
            row.prime_yield_delta_pp,
        );
    }
}

fn parse_args() -> Options {
    let mut input_json = PathBuf::from(DEFAULT_INPUT_JSON);
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input-json" => {
                let value = args
                    .next()
                    .unwrap_or_else(|| panic!("missing value after --input-json"));
                input_json = PathBuf::from(value);
            }
            "--out-dir" => {
                let value = args
                    .next()
                    .unwrap_or_else(|| panic!("missing value after --out-dir"));
                out_dir = PathBuf::from(value);
            }
            "--help" | "-h" => {
                print_help_and_exit();
            }
            other => panic!("unrecognized argument: {other}"),
        }
    }

    Options {
        input_json,
        out_dir,
    }
}

fn print_help_and_exit() -> ! {
    println!("Usage:");
    println!("  cargo run --release --example base14_outlier_mechanism_report -- [options]");
    println!();
    println!("Options:");
    println!("  --input-json <path>   Read atlas artifact from this JSON path");
    println!("  --out-dir <path>      Write output bundle to this directory");
    println!("  -h, --help            Show this help");
    std::process::exit(0);
}

fn load_atlas_bundle(path: &Path) -> AtlasInputBundle {
    let text = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!(
            "Failed to read atlas JSON at {}: {err}\nRun `cargo run --release --example base14_survivor_atlas_report` first or pass --input-json.",
            path.display()
        )
    });
    serde_json::from_str(&text)
        .unwrap_or_else(|err| panic!("Failed to parse atlas JSON at {}: {err}", path.display()))
}

fn build_active_specs(input: &AtlasInputBundle) -> Vec<ActivePairSpec> {
    let active_rows = input
        .atlas_rows
        .iter()
        .filter(|row| {
            row.rank_within_anchor == 0 && matches!(row.atlas_role.as_str(), "m1_to_m2" | "m2_only")
        })
        .collect::<Vec<_>>();
    let mut pair_labels = active_rows
        .iter()
        .map(|row| row.pair_label.clone())
        .collect::<Vec<_>>();
    pair_labels.sort();
    let mut expected = EXPECTED_ACTIVE_PAIRS
        .iter()
        .map(|label| (*label).to_string())
        .collect::<Vec<_>>();
    expected.sort();
    assert_eq!(
        pair_labels, expected,
        "atlas active pair set drifted: expected {:?}, found {:?}",
        expected, pair_labels
    );

    let mut specs = Vec::new();
    for row in active_rows {
        let neighbor = input
            .neighbor_rows
            .iter()
            .find(|neighbor| {
                neighbor.survivor_pair == row.pair_label && neighbor.comparison_rank == 1
            })
            .unwrap_or_else(|| {
                panic!(
                    "atlas neighbor_rows missing comparison_rank=1 control for {}",
                    row.pair_label
                )
            });
        let (outer, inner) = parse_pair_label(&row.pair_label);
        let (control_outer, control_inner) = parse_pair_label(&neighbor.control_pair);
        specs.push(ActivePairSpec {
            pair_label: row.pair_label.clone(),
            atlas_role: row.atlas_role.clone(),
            outer,
            inner,
            best_k_m2: parse_k_label(&row.best_k_m2),
            control_pair: neighbor.control_pair.clone(),
            control_outer,
            control_inner,
            control_rank: neighbor.comparison_rank,
            control_distance: neighbor.unit_distance,
            control_same_gap_bucket: neighbor.same_gap_bucket,
            control_same_same_digit: neighbor.same_same_digit,
            control_same_best_k_m1: neighbor.same_best_k_m1,
        });
    }
    specs.sort_by(|left, right| left.pair_label.cmp(&right.pair_label));
    specs
}

fn parse_pair_label(label: &str) -> (u32, u32) {
    let trimmed = label.trim_matches(|ch| ch == '(' || ch == ')');
    let mut pieces = trimmed.split(',');
    let left = pieces
        .next()
        .unwrap_or_else(|| panic!("missing outer digit in pair label: {label}"));
    let right = pieces
        .next()
        .unwrap_or_else(|| panic!("missing inner digit in pair label: {label}"));
    assert!(
        pieces.next().is_none(),
        "unexpected extra components in pair label: {label}"
    );
    (parse_digit_symbol(left), parse_digit_symbol(right))
}

fn parse_digit_symbol(symbol: &str) -> u32 {
    if symbol.len() == 1 {
        let ch = symbol.chars().next().expect("single-character symbol");
        if let Some(value) = ch.to_digit(10) {
            return value;
        }
        if ch.is_ascii_uppercase() {
            return 10 + (ch as u32 - 'A' as u32);
        }
    }
    panic!("unsupported digit symbol: {symbol}");
}

fn analyze_pair(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    best_k: BoundedKConfig,
) -> PairMechanismAnalysis {
    let k00_profile = scan_k_config_mask_profile(base, middle_length, outer, inner, (0, 0));
    let best_profile = if best_k == (0, 0) {
        k00_profile.clone()
    } else {
        scan_k_config_mask_profile(base, middle_length, outer, inner, best_k)
    };
    let candidates_per_config = k00_profile.candidates_per_config;
    let modulus_relief_rows = zip_modulus_relief_rows(
        &k00_profile.modulus_divisibility_rows,
        &best_profile.modulus_divisibility_rows,
        candidates_per_config,
    );
    let total_positive_relief_pp = modulus_relief_rows
        .iter()
        .filter(|row| row.relief_pp > 0.0)
        .map(|row| row.relief_pp)
        .sum::<f64>();
    let total_negative_relief_pp = modulus_relief_rows
        .iter()
        .filter(|row| row.relief_pp < 0.0)
        .map(|row| -row.relief_pp)
        .sum::<f64>();
    let net_relief_pp = total_positive_relief_pp - total_negative_relief_pp;

    let prime_yield_k00 = ratio(k00_profile.prime_hits, k00_profile.admissible_count);
    let prime_yield_best = ratio(best_profile.prime_hits, best_profile.admissible_count);
    let prime_yield_k00_pp = prime_yield_k00 * 100.0;
    let prime_yield_best_pp = prime_yield_best * 100.0;
    let prime_yield_delta_pp = prime_yield_best_pp - prime_yield_k00_pp;

    let admissible_delta_count =
        best_profile.admissible_count as isize - k00_profile.admissible_count as isize;
    let admissible_delta_pp = count_delta_pp(
        best_profile.admissible_count,
        k00_profile.admissible_count,
        candidates_per_config,
    );
    let zero_mask_delta_count = histogram_count(&best_profile.mask_histogram_rows, 0) as isize
        - histogram_count(&k00_profile.mask_histogram_rows, 0) as isize;
    let zero_mask_delta_pp = count_delta_pp(
        histogram_count(&best_profile.mask_histogram_rows, 0),
        histogram_count(&k00_profile.mask_histogram_rows, 0),
        candidates_per_config,
    );
    let prime_hit_delta_count = best_profile.prime_hits as isize - k00_profile.prime_hits as isize;
    let prime_hit_delta_pp = count_delta_pp(
        best_profile.prime_hits,
        k00_profile.prime_hits,
        candidates_per_config,
    );

    let admissible_share_k00 = k00_profile.admissible_count as f64 / candidates_per_config as f64;
    let admissible_share_best = best_profile.admissible_count as f64 / candidates_per_config as f64;
    let admissible_set_effect_pp =
        (admissible_share_best - admissible_share_k00) * prime_yield_k00 * 100.0;
    let prime_yield_effect_pp =
        admissible_share_best * (prime_yield_best - prime_yield_k00) * 100.0;
    let top_moduli_m2 = render_top_moduli(&modulus_relief_rows);
    let zero_positive_singleton_relief = total_positive_relief_pp <= 0.0;
    let explanation_label = classify_explanation(
        prime_hit_delta_pp,
        admissible_set_effect_pp,
        prime_yield_effect_pp,
        zero_positive_singleton_relief,
    );

    PairMechanismAnalysis {
        k00_profile: k00_profile.clone(),
        best_profile: best_profile.clone(),
        transfer_rows: build_transfer_rows(
            &format_pair_label(base, outer, inner),
            &k00_profile.candidate_rows,
            &best_profile.candidate_rows,
            candidates_per_config,
        ),
        histogram_rows: build_histogram_delta_rows(
            &format_pair_label(base, outer, inner),
            &k00_profile.mask_histogram_rows,
            &best_profile.mask_histogram_rows,
        ),
        candidate_rows: build_candidate_export_rows(
            &format_pair_label(base, outer, inner),
            &k00_profile.candidate_rows,
            &best_profile.candidate_rows,
            best_k,
        ),
        top_moduli_m2,
        explanation_label,
        anomaly_m2_pp: prime_hit_delta_pp.max(0.0),
        prime_hit_delta_count,
        prime_hit_delta_pp,
        admissible_delta_count,
        admissible_delta_pp,
        zero_mask_delta_count,
        zero_mask_delta_pp,
        prime_yield_k00_pp,
        prime_yield_best_pp,
        prime_yield_delta_pp,
        admissible_set_effect_pp,
        prime_yield_effect_pp,
        total_positive_relief_pp,
        total_negative_relief_pp,
        net_relief_pp,
        zero_positive_singleton_relief,
    }
}

fn build_transfer_rows(
    pair_label: &str,
    k00_rows: &[KConfigCandidateMaskRow],
    best_rows: &[KConfigCandidateMaskRow],
    candidates_per_config: usize,
) -> Vec<CandidateTransferRow> {
    let mut counts = BTreeMap::<&'static str, CandidateTransferRow>::new();
    for (k00_row, best_row) in k00_rows.iter().zip(best_rows) {
        assert_eq!(
            k00_row.middle_index, best_row.middle_index,
            "candidate rows should align by middle index"
        );
        let category = match (k00_row.admissible, best_row.admissible) {
            (false, false) => CandidateTransferCategory::SharedInadmissible,
            (false, true) => CandidateTransferCategory::InadmissibleToAdmissible,
            (true, false) => CandidateTransferCategory::AdmissibleToInadmissible,
            (true, true) => CandidateTransferCategory::SharedAdmissible,
        };
        let entry = counts
            .entry(category.label())
            .or_insert(CandidateTransferRow {
                pair_label: pair_label.to_string(),
                category: category.label().to_string(),
                middle_index_count: 0,
                share_of_candidates_pp: 0.0,
                k00_admissible_count: 0,
                best_admissible_count: 0,
                k00_prime_count: 0,
                best_prime_count: 0,
                k00_composite_count: 0,
                best_composite_count: 0,
                prime_delta_count: 0,
            });
        entry.middle_index_count += 1;
        entry.k00_admissible_count += usize::from(k00_row.admissible);
        entry.best_admissible_count += usize::from(best_row.admissible);
        entry.k00_prime_count += usize::from(k00_row.prime);
        entry.best_prime_count += usize::from(best_row.prime);
        entry.k00_composite_count += usize::from(!k00_row.prime);
        entry.best_composite_count += usize::from(!best_row.prime);
    }

    let mut rows = counts.into_values().collect::<Vec<_>>();
    for row in &mut rows {
        row.share_of_candidates_pp =
            row.middle_index_count as f64 * 100.0 / candidates_per_config as f64;
        row.prime_delta_count = row.best_prime_count as isize - row.k00_prime_count as isize;
    }
    rows.sort_by(|left, right| {
        transfer_sort_key(&left.category).cmp(&transfer_sort_key(&right.category))
    });
    rows
}

fn build_histogram_delta_rows(
    pair_label: &str,
    k00_rows: &[KConfigMaskHistogramRow],
    best_rows: &[KConfigMaskHistogramRow],
) -> Vec<MaskHistogramDeltaRow> {
    let mut masks = BTreeMap::<u16, (usize, usize, usize, usize)>::new();
    for row in k00_rows {
        masks.entry(row.divisibility_mask).or_insert((0, 0, 0, 0)).0 = row.count;
        masks.entry(row.divisibility_mask).or_insert((0, 0, 0, 0)).2 = row.prime_count;
    }
    for row in best_rows {
        masks.entry(row.divisibility_mask).or_insert((0, 0, 0, 0)).1 = row.count;
        masks.entry(row.divisibility_mask).or_insert((0, 0, 0, 0)).3 = row.prime_count;
    }
    masks
        .into_iter()
        .map(
            |(divisibility_mask, (k00_count, best_count, k00_prime_count, best_prime_count))| {
                MaskHistogramDeltaRow {
                    pair_label: pair_label.to_string(),
                    divisibility_mask,
                    mask_label: render_divisibility_mask(divisibility_mask),
                    k00_count,
                    best_count,
                    delta_count: best_count as isize - k00_count as isize,
                    k00_prime_count,
                    best_prime_count,
                    delta_prime_count: best_prime_count as isize - k00_prime_count as isize,
                }
            },
        )
        .collect()
}

fn build_candidate_export_rows(
    pair_label: &str,
    k00_rows: &[KConfigCandidateMaskRow],
    best_rows: &[KConfigCandidateMaskRow],
    best_k: BoundedKConfig,
) -> Vec<CandidateMaskExportRow> {
    let mut rows = Vec::with_capacity(k00_rows.len() + best_rows.len());
    rows.extend(k00_rows.iter().map(|row| CandidateMaskExportRow {
        pair_label: pair_label.to_string(),
        lane: "k=(0,0)".to_string(),
        middle_index: row.middle_index,
        middle_digits: row.middle_digits.clone(),
        decimal_value: row.decimal_value.clone(),
        divisibility_mask: row.divisibility_mask,
        mask_label: row.mask_label.clone(),
        admissible: row.admissible,
        prime: row.prime,
    }));
    rows.extend(best_rows.iter().map(|row| CandidateMaskExportRow {
        pair_label: pair_label.to_string(),
        lane: format_k(best_k),
        middle_index: row.middle_index,
        middle_digits: row.middle_digits.clone(),
        decimal_value: row.decimal_value.clone(),
        divisibility_mask: row.divisibility_mask,
        mask_label: row.mask_label.clone(),
        admissible: row.admissible,
        prime: row.prime,
    }));
    rows
}

fn build_appendix_rows() -> Vec<AppendixRow> {
    let mut rows = Vec::new();
    for &(base, outer, inner) in APPENDIX_OUTLIERS {
        let pair_row = evaluate_pair_row(base, MIDDLE_LENGTH, outer, inner, DEFAULT_BOUNDED_K_GRID);
        let analysis = analyze_pair(
            base,
            MIDDLE_LENGTH,
            outer,
            inner,
            parse_k_label(&pair_row.best_k),
        );
        rows.push(AppendixRow {
            base,
            pair_label: format_pair_label(base, outer, inner),
            best_k_m2: pair_row.best_k,
            explanation_label: analysis.explanation_label,
            anomaly_m2_pp: analysis.prime_hit_delta_pp,
            admissible_delta_pp: analysis.admissible_delta_pp,
            zero_mask_delta_pp: analysis.zero_mask_delta_pp,
            prime_yield_delta_pp: analysis.prime_yield_delta_pp,
            admissible_set_effect_pp: analysis.admissible_set_effect_pp,
            prime_yield_effect_pp: analysis.prime_yield_effect_pp,
            total_positive_relief_pp: analysis.total_positive_relief_pp,
            total_negative_relief_pp: analysis.total_negative_relief_pp,
            net_relief_pp: analysis.net_relief_pp,
        });
    }
    rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    rows
}

fn zip_modulus_relief_rows(
    k00_rows: &[primes::validation::bounded_k::KConfigModulusDivisibilityRow],
    best_rows: &[primes::validation::bounded_k::KConfigModulusDivisibilityRow],
    candidates_per_config: usize,
) -> Vec<ModulusReliefRow> {
    let mut rows = k00_rows
        .iter()
        .zip(best_rows)
        .map(|(k00_row, best_row)| ModulusReliefRow {
            modulus: k00_row.modulus,
            relief_pp: count_delta_pp(
                k00_row.divisible_count,
                best_row.divisible_count,
                candidates_per_config,
            ),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.modulus.cmp(&right.modulus));
    rows
}

fn histogram_count(rows: &[KConfigMaskHistogramRow], divisibility_mask: u16) -> usize {
    rows.iter()
        .find(|row| row.divisibility_mask == divisibility_mask)
        .map(|row| row.count)
        .unwrap_or(0)
}

fn render_top_moduli(rows: &[ModulusReliefRow]) -> String {
    let mut top_rows = rows
        .iter()
        .filter(|row| row.relief_pp > 0.0)
        .collect::<Vec<_>>();
    top_rows.sort_by(|left, right| {
        right
            .relief_pp
            .total_cmp(&left.relief_pp)
            .then_with(|| left.modulus.cmp(&right.modulus))
    });
    top_rows.truncate(3);
    if top_rows.is_empty() {
        "none".to_string()
    } else {
        top_rows
            .into_iter()
            .map(|row| format!("p{}:+{:.2}pp", row.modulus, row.relief_pp))
            .collect::<Vec<_>>()
            .join(";")
    }
}

fn classify_explanation(
    prime_hit_delta_pp: f64,
    admissible_set_effect_pp: f64,
    prime_yield_effect_pp: f64,
    zero_positive_singleton_relief: bool,
) -> String {
    const EPS: f64 = 1e-12;
    if prime_hit_delta_pp <= EPS {
        "no_m2_anomaly".to_string()
    } else if zero_positive_singleton_relief && admissible_set_effect_pp > EPS {
        "overlap_lift".to_string()
    } else if admissible_set_effect_pp > EPS && prime_yield_effect_pp > EPS {
        "mixed".to_string()
    } else if prime_yield_effect_pp > EPS {
        "yield_dominated".to_string()
    } else {
        "overlap_lift".to_string()
    }
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn count_delta_pp(best: usize, baseline: usize, total: usize) -> f64 {
    (best as f64 - baseline as f64) * 100.0 / total as f64
}

fn format_pair_label(base: u32, outer: u32, inner: u32) -> String {
    assert!(
        outer < base && inner < base,
        "pair digit should be inside base"
    );
    format!("({},{})", digit_symbol(outer), digit_symbol(inner))
}

fn transfer_sort_key(category: &str) -> usize {
    match category {
        "inadmissible_to_admissible" => 0,
        "admissible_to_inadmissible" => 1,
        "shared_admissible" => 2,
        "shared_inadmissible" => 3,
        _ => 4,
    }
}

fn render_decomposition_plane(rows: &[MechanismRow], path: &Path) {
    let x_values = rows
        .iter()
        .map(|row| row.admissible_delta_pp)
        .collect::<Vec<_>>();
    let y_values = rows
        .iter()
        .map(|row| row.prime_yield_delta_pp)
        .collect::<Vec<_>>();
    let x_min = x_values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
        .min(-1.0)
        - 0.8;
    let x_max = x_values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(1.0)
        + 0.8;
    let y_min = y_values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
        .min(-1.0)
        - 0.8;
    let y_max = y_values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(1.0)
        + 0.8;

    let root = BitMapBackend::new(path, (980, 760)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill mechanism canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base 14 M=2 Outlier Mechanism Plane  (x = admissible delta, y = prime-yield delta)",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(56)
        .y_label_area_size(64)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .expect("failed to build decomposition plane");

    chart
        .configure_mesh()
        .x_desc("admissible delta (pp)")
        .y_desc("prime-yield delta among admissible candidates (pp)")
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(214, 207, 196))
        .draw()
        .expect("failed to draw decomposition mesh");

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(0.0, y_min), (0.0, y_max)],
            ShapeStyle::from(&RGBColor(180, 173, 162)).stroke_width(1),
        )))
        .expect("failed to draw x=0 guide");
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(x_min, 0.0), (x_max, 0.0)],
            ShapeStyle::from(&RGBColor(180, 173, 162)).stroke_width(1),
        )))
        .expect("failed to draw y=0 guide");

    for row in rows {
        let color = explanation_color(&row.explanation_label);
        let radius = if row.stress_case { 10 } else { 8 };
        chart
            .draw_series(std::iter::once(Circle::new(
                (row.admissible_delta_pp, row.prime_yield_delta_pp),
                radius,
                ShapeStyle::from(&color).filled(),
            )))
            .expect("failed to draw decomposition point");
        if row.stress_case {
            chart
                .draw_series(std::iter::once(Circle::new(
                    (row.admissible_delta_pp, row.prime_yield_delta_pp),
                    radius + 2,
                    ShapeStyle::from(&BLACK).stroke_width(2),
                )))
                .expect("failed to draw stress-case outline");
        }
        chart
            .draw_series(std::iter::once(Text::new(
                row.pair_label.clone(),
                (
                    row.admissible_delta_pp + 0.16,
                    row.prime_yield_delta_pp + 0.16,
                ),
                ("sans-serif", 18).into_font().color(&BLACK),
            )))
            .expect("failed to draw point label");
    }

    root.present()
        .expect("failed to present decomposition plane image");
}

fn explanation_color(label: &str) -> RGBColor {
    match label {
        "overlap_lift" => RGBColor(33, 120, 180),
        "yield_dominated" => RGBColor(191, 82, 32),
        "mixed" => RGBColor(80, 140, 52),
        _ => RGBColor(110, 110, 110),
    }
}

fn build_report_summary(rows: &[MechanismRow], appendix_rows: &[AppendixRow]) -> ReportSummary {
    ReportSummary {
        active_pairs: rows.len(),
        active_pair_labels: rows
            .iter()
            .map(|row| row.pair_label.clone())
            .collect::<Vec<_>>()
            .join(", "),
        yield_dominated_pairs: rows
            .iter()
            .filter(|row| row.explanation_label == "yield_dominated")
            .count(),
        mixed_pairs: rows
            .iter()
            .filter(|row| row.explanation_label == "mixed")
            .count(),
        overlap_lift_pairs: rows
            .iter()
            .filter(|row| row.explanation_label == "overlap_lift")
            .count(),
        zero_positive_singleton_pairs: rows
            .iter()
            .filter(|row| row.zero_positive_singleton_relief)
            .count(),
        appendix_rows: appendix_rows.len(),
        stress_case_pair: "(D,B)".to_string(),
    }
}

fn derive_observations(rows: &[MechanismRow], appendix_rows: &[AppendixRow]) -> Vec<String> {
    let stress_case = rows
        .iter()
        .find(|row| row.pair_label == "(D,B)")
        .expect("stress-case row should exist");
    let mixed_row = rows
        .iter()
        .find(|row| row.explanation_label == "mixed")
        .expect("base-14 mechanism lane should include one mixed row");
    let appendix_overlap = appendix_rows
        .iter()
        .filter(|row| row.explanation_label == "overlap_lift")
        .map(|row| format!("base {} {}", row.base, row.pair_label))
        .collect::<Vec<_>>()
        .join(", ");

    vec![
        format!(
            "The base-14 stress case {} stays positive at M=2 despite {} admissible delta, {} total positive singleton relief, and {} top moduli; its win is therefore classified as {}.",
            stress_case.pair_label,
            format_signed_pp(stress_case.admissible_delta_pp),
            format_signed_pp(stress_case.total_positive_relief_pp),
            stress_case.top_moduli_m2,
            stress_case.explanation_label
        ),
        format!(
            "The emergent pair {} is the clean mixed case in base 14: admissible delta {} and prime-yield delta {} are both positive.",
            mixed_row.pair_label,
            format_signed_pp(mixed_row.admissible_delta_pp),
            format_signed_pp(mixed_row.prime_yield_delta_pp)
        ),
        "Zero-mask delta and admissible delta match exactly in every row because the zero-mask histogram is the admissible set.".to_string(),
        format!(
            "The broader zero-positive-signature appendix stays connected to the main lane: {} all classify as overlap_lift under the same decomposition, while (D,B) remains the yield-dominated stress case.",
            appendix_overlap
        ),
    ]
}

fn render_markdown(bundle: &OutputBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Base-14 Outlier Mechanism\n\n");
    markdown.push_str("_Generated from `examples/base14_outlier_mechanism_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Input atlas: `{}`\n- Output directory: `{}`\n- Base / middle length: `{} / {}`\n\n",
        bundle.settings.input_json,
        bundle.settings.out_dir,
        bundle.settings.base,
        bundle.settings.middle_length
    ));

    markdown.push_str("## Summary\n\n");
    markdown.push_str(
        "| Pair | Role | Best k at M=2 | Label | Anomaly | Admissible delta | Yield delta | Top moduli |\n",
    );
    markdown.push_str("|---|---|---|---|---:|---:|---:|---|\n");
    for row in &bundle.mechanism_rows {
        markdown.push_str(&format!(
            "| {} | {} | `{}` | `{}` | {:+.2}pp | {:+.2}pp | {:+.2}pp | {} |\n",
            row.pair_label,
            row.atlas_role,
            row.best_k_m2,
            row.explanation_label,
            row.prime_hit_delta_pp,
            row.admissible_delta_pp,
            row.prime_yield_delta_pp,
            row.top_moduli_m2
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Exact Decomposition\n\n");
    markdown
        .push_str("For each active pair we decompose the `M=2` win over `k=(0,0)` exactly as:\n\n");
    markdown.push_str("`delta prime-rate = admissible-set effect + prime-yield effect`\n\n");
    markdown.push_str(
        "| Pair | Admissible-set effect | Prime-yield effect | Zero-mask delta | Positive singleton relief | Net relief |\n",
    );
    markdown.push_str("|---|---:|---:|---:|---:|---:|\n");
    for row in &bundle.mechanism_rows {
        markdown.push_str(&format!(
            "| {} | {:+.2}pp | {:+.2}pp | {:+.2}pp | {:+.2}pp | {:+.2}pp |\n",
            row.pair_label,
            row.admissible_set_effect_pp,
            row.prime_yield_effect_pp,
            row.zero_mask_delta_pp,
            row.total_positive_relief_pp,
            row.net_relief_pp
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Stress Case\n\n");
    if let Some(stress_case) = bundle
        .mechanism_rows
        .iter()
        .find(|row| row.pair_label == "(D,B)")
    {
        markdown.push_str(&format!(
            "- `(D,B)` is the key stress case: anomaly `{:+.2}pp`, admissible delta `{:+.2}pp`, yield delta `{:+.2}pp`, top moduli `{}`.\n",
            stress_case.prime_hit_delta_pp,
            stress_case.admissible_delta_pp,
            stress_case.prime_yield_delta_pp,
            stress_case.top_moduli_m2
        ));
        markdown.push_str(&format!(
            "- It is classified as `{}` because the win survives despite zero positive singleton relief and a nonpositive admissible shift.\n\n",
            stress_case.explanation_label
        ));
    }

    markdown.push_str("## Visual\n\n");
    if let Some(image) = bundle.image_artifact_rows.first() {
        markdown.push_str(&format!("![{}]({})\n\n", image.label, image.path));
    }

    markdown.push_str("## Candidate Transfer Tables\n\n");
    for pair_label in EXPECTED_ACTIVE_PAIRS {
        markdown.push_str(&format!("### {}\n\n", pair_label));
        markdown.push_str(
            "| Category | Count | Share | k00 admissible | best admissible | k00 primes | best primes | Prime delta |\n",
        );
        markdown.push_str("|---|---:|---:|---:|---:|---:|---:|---:|\n");
        for row in bundle
            .candidate_transfer_rows
            .iter()
            .filter(|row| row.pair_label == *pair_label)
        {
            markdown.push_str(&format!(
                "| `{}` | {} | {:.2}% | {} | {} | {} | {} | {:+} |\n",
                row.category,
                row.middle_index_count,
                row.share_of_candidates_pp,
                row.k00_admissible_count,
                row.best_admissible_count,
                row.k00_prime_count,
                row.best_prime_count,
                row.prime_delta_count
            ));
        }
        markdown.push('\n');
    }

    markdown.push_str("## Appendix: Other Zero-Positive-Signature Outliers\n\n");
    markdown.push_str(
        "| Base | Pair | Best k at M=2 | Label | Anomaly | Admissible delta | Yield delta | Net relief |\n",
    );
    markdown.push_str("|---:|---|---|---|---:|---:|---:|---:|\n");
    for row in &bundle.appendix_rows {
        markdown.push_str(&format!(
            "| {} | {} | `{}` | `{}` | {:+.2}pp | {:+.2}pp | {:+.2}pp | {:+.2}pp |\n",
            row.base,
            row.pair_label,
            row.best_k_m2,
            row.explanation_label,
            row.anomaly_m2_pp,
            row.admissible_delta_pp,
            row.prime_yield_delta_pp,
            row.net_relief_pp
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Observations\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {}\n", observation));
    }
    markdown
}

fn format_signed_pp(value: f64) -> String {
    format!("{value:+.2}pp")
}
