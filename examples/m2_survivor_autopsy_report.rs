//! Autopsy report for the full-catalog pairs whose bounded-`k` anomalies are
//! present at `M=2`.
//!
//! This report isolates the survivor set, compares each survivor against nearby
//! `m1_only` controls, and audits the residue-level obstruction profile for the
//! winning `k` versus `k=(0,0)`.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example m2_survivor_autopsy_report
//! cargo run --release --example m2_survivor_autopsy_report -- --smoke --out-dir /tmp/primes_m2_survivor_autopsy_smoke
//! ```

use primes::validation::{
    bounded_k::{
        evaluate_pair_row, format_k, ordered_unit_pairs, scan_k_config_profile, select_smoke_pairs,
        unit_residues, KConfigModulusDivisibilityRow, KDominancePairRow, DEFAULT_BOUNDED_K_GRID,
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
const DEFAULT_OUT_DIR: &str = "/tmp/primes_m2_survivor_autopsy";
const REPORT_EXPORT_VERSION: u32 = 1;
const SMOKE_MAX_ORDERED_PAIRS_PER_BASE: usize = 8;
const SMOKE_PAIR_ANCHORS: &[(u32, u32, u32)] = &[(6, 1, 5), (10, 3, 3), (10, 3, 7), (30, 11, 7)];
const NEARBY_CONTROL_LIMIT: usize = 3;
const TOP_MODULUS_LIMIT: usize = 3;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    smoke_catalog: bool,
}

#[derive(Debug, Clone)]
struct BoundaryScan {
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    same_digit: bool,
    complement_pair: bool,
    unit_index_outer: usize,
    unit_index_inner: usize,
    unit_gap_bucket: String,
    row_m1: KDominancePairRow,
    row_m2: KDominancePairRow,
    row_m3: KDominancePairRow,
    boundary_class: String,
}

#[derive(Debug, Clone)]
struct ProfileDeltaSummary {
    admissible_delta_pp: f64,
    top_moduli_summary: String,
    modulus_relief_rows: Vec<ModulusReliefRow>,
}

#[derive(Debug, Clone)]
struct ModulusReliefRow {
    modulus: u32,
    k00_divisible_pp: f64,
    best_divisible_pp: f64,
    relief_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    bases: Vec<u32>,
    pair_catalog_mode: String,
    middle_lengths: Vec<usize>,
    nearby_control_limit: usize,
    top_modulus_limit: usize,
    k_grid: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct SurvivorRow {
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    same_digit: bool,
    complement_pair: bool,
    unit_gap_bucket: String,
    best_k_m1: String,
    best_k_m2: String,
    boundary_class: String,
    anomaly_m1_pp: f64,
    anomaly_m2_pp: f64,
    anomaly_m3_pp: f64,
    retention_share_m2_over_m1: Option<f64>,
    admissible_delta_m1_pp: f64,
    admissible_delta_m2_pp: f64,
    top_moduli_m1: String,
    top_moduli_m2: String,
    nearest_control_pair: String,
    nearest_control_same_base: bool,
    nearest_control_unit_distance: usize,
    nearest_control_boundary_class: String,
    nearest_control_anomaly_m1_pp: f64,
    nearest_control_anomaly_m2_pp: f64,
    nearest_control_top_moduli_m1: String,
    nearest_control_top_moduli_m2: String,
}

#[derive(Debug, Clone, Serialize)]
struct ComparisonRow {
    survivor_base: u32,
    survivor_pair: String,
    control_base: u32,
    control_pair: String,
    comparison_rank: usize,
    same_base: bool,
    same_gap_bucket: bool,
    same_same_digit: bool,
    same_best_k_m1: bool,
    unit_distance: usize,
    survivor_boundary_class: String,
    control_boundary_class: String,
    survivor_best_k_m1: String,
    survivor_best_k_m2: String,
    control_best_k_m1: String,
    control_best_k_m2: String,
    survivor_anomaly_m1_pp: f64,
    survivor_anomaly_m2_pp: f64,
    control_anomaly_m1_pp: f64,
    control_anomaly_m2_pp: f64,
    survivor_admissible_delta_m1_pp: f64,
    survivor_admissible_delta_m2_pp: f64,
    control_admissible_delta_m1_pp: f64,
    control_admissible_delta_m2_pp: f64,
    survivor_top_moduli_m1: String,
    survivor_top_moduli_m2: String,
    control_top_moduli_m1: String,
    control_top_moduli_m2: String,
}

#[derive(Debug, Clone, Serialize)]
struct ResidueComparisonRow {
    survivor_base: u32,
    survivor_pair: String,
    control_base: u32,
    control_pair: String,
    comparison_rank: usize,
    middle_length: usize,
    modulus: u32,
    survivor_k00_divisible_pp: f64,
    survivor_best_divisible_pp: f64,
    survivor_relief_pp: f64,
    control_k00_divisible_pp: f64,
    control_best_divisible_pp: f64,
    control_relief_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct SurvivorBucketRow {
    bucket_kind: String,
    bucket_value: String,
    survivor_count: usize,
    persistent_survivor_count: usize,
    emergent_survivor_count: usize,
    median_anomaly_m2_pp: f64,
    median_retention_share_m2_over_m1: Option<f64>,
    median_admissible_delta_m2_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    total_survivors: usize,
    persistent_survivors: usize,
    emergent_survivors: usize,
    total_controls: usize,
    survivor_bases: String,
    survivors_same_digit: usize,
    survivors_adjacent_gap: usize,
    survivors_same_gap: usize,
    survivors_wide_gap: usize,
    median_anomaly_m2_pp: f64,
    median_persistent_retention_share_m2_over_m1: Option<f64>,
    median_admissible_delta_m2_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    survivor_rows: Vec<SurvivorRow>,
    comparison_rows: Vec<ComparisonRow>,
    residue_comparison_rows: Vec<ResidueComparisonRow>,
    survivor_bucket_rows: Vec<SurvivorBucketRow>,
    report_summary: ReportSummary,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        bases: BASES.to_vec(),
        pair_catalog_mode: if options.smoke_catalog {
            "smoke".to_string()
        } else {
            "full".to_string()
        },
        middle_lengths: vec![M1, M2, M3],
        nearby_control_limit: NEARBY_CONTROL_LIMIT,
        top_modulus_limit: TOP_MODULUS_LIMIT,
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format_k(config))
            .collect(),
    };

    let scans = build_boundary_scans(options.smoke_catalog);
    let survivors = scans
        .iter()
        .filter(|scan| anomaly_mass(&scan.row_m2) > 0.0)
        .collect::<Vec<_>>();
    let controls = scans
        .iter()
        .filter(|scan| scan.boundary_class == "m1_only")
        .collect::<Vec<_>>();

    let (survivor_rows, comparison_rows, residue_comparison_rows) =
        build_autopsy_rows(&survivors, &controls);
    let survivor_bucket_rows = build_survivor_bucket_rows(&survivor_rows);
    let report_summary = build_report_summary(&survivor_rows, controls.len());

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        survivor_rows,
        comparison_rows,
        residue_comparison_rows,
        survivor_bucket_rows,
        report_summary,
    };

    write_csv_rows(
        options.out_dir.join("survivor_rows.csv"),
        &bundle.survivor_rows,
    )
    .expect("failed to write survivor rows");
    write_csv_rows(
        options.out_dir.join("comparison_rows.csv"),
        &bundle.comparison_rows,
    )
    .expect("failed to write comparison rows");
    write_csv_rows(
        options.out_dir.join("residue_comparison_rows.csv"),
        &bundle.residue_comparison_rows,
    )
    .expect("failed to write residue comparison rows");
    write_csv_rows(
        options.out_dir.join("survivor_bucket_rows.csv"),
        &bundle.survivor_bucket_rows,
    )
    .expect("failed to write survivor bucket rows");
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
    let mut smoke_catalog = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir"));
            }
            "--smoke" => {
                smoke_catalog = true;
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
        smoke_catalog,
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
    println!("M=2 survivor autopsy report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example m2_survivor_autopsy_report -- [options]");
    println!();
    println!("Options:");
    println!(
        "  --out-dir <path>          Output directory for artifacts (default: {DEFAULT_OUT_DIR})"
    );
    println!(
        "  --smoke                   Use the smoke catalog instead of the default full catalog"
    );
}

fn build_boundary_scans(smoke_catalog: bool) -> Vec<BoundaryScan> {
    let tasks: Vec<_> = BASES
        .iter()
        .copied()
        .flat_map(|base| {
            let pairs = if smoke_catalog {
                let anchors = SMOKE_PAIR_ANCHORS
                    .iter()
                    .filter(|&&(anchor_base, _, _)| anchor_base == base)
                    .map(|&(_, outer, inner)| (outer, inner))
                    .collect::<Vec<_>>();
                select_smoke_pairs(base, SMOKE_MAX_ORDERED_PAIRS_PER_BASE, &anchors)
            } else {
                ordered_unit_pairs(base)
            };
            pairs
                .into_iter()
                .map(move |(outer, inner)| (base, outer, inner))
        })
        .collect();

    let mut scans: Vec<_> = tasks
        .par_iter()
        .map(|&(base, outer, inner)| build_boundary_scan(base, outer, inner))
        .collect();
    scans.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
    });
    scans
}

fn build_boundary_scan(base: u32, outer: u32, inner: u32) -> BoundaryScan {
    let row_m1 = evaluate_pair_row(base, M1, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let row_m2 = evaluate_pair_row(base, M2, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let row_m3 = evaluate_pair_row(base, M3, outer, inner, DEFAULT_BOUNDED_K_GRID);
    let boundary_class = boundary_class(
        anomaly_mass_bool(&row_m1),
        anomaly_mass_bool(&row_m2),
        anomaly_mass_bool(&row_m3),
    );
    let units = unit_residues(base);
    let unit_index_outer = units
        .iter()
        .position(|&digit| digit == outer)
        .expect("outer digit should be a unit residue");
    let unit_index_inner = units
        .iter()
        .position(|&digit| digit == inner)
        .expect("inner digit should be a unit residue");

    BoundaryScan {
        base,
        outer,
        inner,
        pair_label: row_m1.pair_label.clone(),
        same_digit: outer == inner,
        complement_pair: (outer + inner).is_multiple_of(base),
        unit_index_outer,
        unit_index_inner,
        unit_gap_bucket: unit_gap_bucket(unit_index_outer.abs_diff(unit_index_inner)),
        row_m1,
        row_m2,
        row_m3,
        boundary_class,
    }
}

fn build_autopsy_rows(
    survivors: &[&BoundaryScan],
    controls: &[&BoundaryScan],
) -> (
    Vec<SurvivorRow>,
    Vec<ComparisonRow>,
    Vec<ResidueComparisonRow>,
) {
    let mut survivor_rows = Vec::new();
    let mut comparison_rows = Vec::new();
    let mut residue_rows = Vec::new();

    for survivor in survivors {
        let survivor_m1 = profile_delta_summary(survivor, M1, &survivor.row_m1);
        let survivor_m2 = profile_delta_summary(survivor, M2, &survivor.row_m2);
        let nearby_controls = nearest_controls(survivor, controls, NEARBY_CONTROL_LIMIT);
        let nearest_control = nearby_controls.first().copied();

        survivor_rows.push(SurvivorRow {
            base: survivor.base,
            outer: survivor.outer,
            inner: survivor.inner,
            pair_label: survivor.pair_label.clone(),
            same_digit: survivor.same_digit,
            complement_pair: survivor.complement_pair,
            unit_gap_bucket: survivor.unit_gap_bucket.clone(),
            best_k_m1: survivor.row_m1.best_k.clone(),
            best_k_m2: survivor.row_m2.best_k.clone(),
            boundary_class: survivor.boundary_class.clone(),
            anomaly_m1_pp: anomaly_mass(&survivor.row_m1),
            anomaly_m2_pp: anomaly_mass(&survivor.row_m2),
            anomaly_m3_pp: anomaly_mass(&survivor.row_m3),
            retention_share_m2_over_m1: if anomaly_mass(&survivor.row_m1) > 0.0 {
                Some(anomaly_mass(&survivor.row_m2) / anomaly_mass(&survivor.row_m1))
            } else {
                None
            },
            admissible_delta_m1_pp: survivor_m1.admissible_delta_pp,
            admissible_delta_m2_pp: survivor_m2.admissible_delta_pp,
            top_moduli_m1: survivor_m1.top_moduli_summary.clone(),
            top_moduli_m2: survivor_m2.top_moduli_summary.clone(),
            nearest_control_pair: nearest_control
                .map(|control| control.pair_label.clone())
                .unwrap_or_else(|| "none".to_string()),
            nearest_control_same_base: nearest_control
                .map(|control| control.base == survivor.base)
                .unwrap_or(false),
            nearest_control_unit_distance: nearest_control
                .map(|control| pair_distance(survivor, control))
                .unwrap_or(usize::MAX),
            nearest_control_boundary_class: nearest_control
                .map(|control| control.boundary_class.clone())
                .unwrap_or_else(|| "none".to_string()),
            nearest_control_anomaly_m1_pp: nearest_control
                .map(|control| anomaly_mass(&control.row_m1))
                .unwrap_or(0.0),
            nearest_control_anomaly_m2_pp: nearest_control
                .map(|control| anomaly_mass(&control.row_m2))
                .unwrap_or(0.0),
            nearest_control_top_moduli_m1: nearest_control
                .map(|control| {
                    profile_delta_summary(control, M1, &control.row_m1).top_moduli_summary
                })
                .unwrap_or_else(|| "none".to_string()),
            nearest_control_top_moduli_m2: nearest_control
                .map(|control| {
                    profile_delta_summary(control, M2, &control.row_m2).top_moduli_summary
                })
                .unwrap_or_else(|| "none".to_string()),
        });

        for (comparison_rank, control) in nearby_controls.into_iter().enumerate() {
            let control_m1 = profile_delta_summary(control, M1, &control.row_m1);
            let control_m2 = profile_delta_summary(control, M2, &control.row_m2);

            comparison_rows.push(ComparisonRow {
                survivor_base: survivor.base,
                survivor_pair: survivor.pair_label.clone(),
                control_base: control.base,
                control_pair: control.pair_label.clone(),
                comparison_rank: comparison_rank + 1,
                same_base: survivor.base == control.base,
                same_gap_bucket: survivor.unit_gap_bucket == control.unit_gap_bucket,
                same_same_digit: survivor.same_digit == control.same_digit,
                same_best_k_m1: survivor.row_m1.best_k == control.row_m1.best_k,
                unit_distance: pair_distance(survivor, control),
                survivor_boundary_class: survivor.boundary_class.clone(),
                control_boundary_class: control.boundary_class.clone(),
                survivor_best_k_m1: survivor.row_m1.best_k.clone(),
                survivor_best_k_m2: survivor.row_m2.best_k.clone(),
                control_best_k_m1: control.row_m1.best_k.clone(),
                control_best_k_m2: control.row_m2.best_k.clone(),
                survivor_anomaly_m1_pp: anomaly_mass(&survivor.row_m1),
                survivor_anomaly_m2_pp: anomaly_mass(&survivor.row_m2),
                control_anomaly_m1_pp: anomaly_mass(&control.row_m1),
                control_anomaly_m2_pp: anomaly_mass(&control.row_m2),
                survivor_admissible_delta_m1_pp: survivor_m1.admissible_delta_pp,
                survivor_admissible_delta_m2_pp: survivor_m2.admissible_delta_pp,
                control_admissible_delta_m1_pp: control_m1.admissible_delta_pp,
                control_admissible_delta_m2_pp: control_m2.admissible_delta_pp,
                survivor_top_moduli_m1: survivor_m1.top_moduli_summary.clone(),
                survivor_top_moduli_m2: survivor_m2.top_moduli_summary.clone(),
                control_top_moduli_m1: control_m1.top_moduli_summary.clone(),
                control_top_moduli_m2: control_m2.top_moduli_summary.clone(),
            });

            for middle_length in [M1, M2] {
                let survivor_summary = if middle_length == M1 {
                    &survivor_m1
                } else {
                    &survivor_m2
                };
                let control_summary = if middle_length == M1 {
                    &control_m1
                } else {
                    &control_m2
                };

                for (survivor_relief, control_relief) in survivor_summary
                    .modulus_relief_rows
                    .iter()
                    .zip(&control_summary.modulus_relief_rows)
                {
                    residue_rows.push(ResidueComparisonRow {
                        survivor_base: survivor.base,
                        survivor_pair: survivor.pair_label.clone(),
                        control_base: control.base,
                        control_pair: control.pair_label.clone(),
                        comparison_rank: comparison_rank + 1,
                        middle_length,
                        modulus: survivor_relief.modulus,
                        survivor_k00_divisible_pp: survivor_relief.k00_divisible_pp,
                        survivor_best_divisible_pp: survivor_relief.best_divisible_pp,
                        survivor_relief_pp: survivor_relief.relief_pp,
                        control_k00_divisible_pp: control_relief.k00_divisible_pp,
                        control_best_divisible_pp: control_relief.best_divisible_pp,
                        control_relief_pp: control_relief.relief_pp,
                    });
                }
            }
        }
    }

    survivor_rows.sort_by(|left, right| {
        right
            .anomaly_m2_pp
            .total_cmp(&left.anomaly_m2_pp)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    comparison_rows.sort_by(|left, right| {
        left.survivor_base
            .cmp(&right.survivor_base)
            .then_with(|| left.survivor_pair.cmp(&right.survivor_pair))
            .then_with(|| left.comparison_rank.cmp(&right.comparison_rank))
    });
    residue_rows.sort_by(|left, right| {
        left.survivor_base
            .cmp(&right.survivor_base)
            .then_with(|| left.survivor_pair.cmp(&right.survivor_pair))
            .then_with(|| left.comparison_rank.cmp(&right.comparison_rank))
            .then_with(|| left.middle_length.cmp(&right.middle_length))
            .then_with(|| left.modulus.cmp(&right.modulus))
    });

    (survivor_rows, comparison_rows, residue_rows)
}

fn nearest_controls<'a>(
    survivor: &BoundaryScan,
    controls: &'a [&BoundaryScan],
    limit: usize,
) -> Vec<&'a BoundaryScan> {
    let mut same_base = controls
        .iter()
        .copied()
        .filter(|control| control.base == survivor.base)
        .collect::<Vec<_>>();
    if same_base.is_empty() {
        same_base = controls.to_vec();
    }

    same_base.sort_by(|left, right| {
        control_rank_tuple(survivor, left).cmp(&control_rank_tuple(survivor, right))
    });
    same_base.truncate(limit);
    same_base
}

fn control_rank_tuple(
    survivor: &BoundaryScan,
    control: &BoundaryScan,
) -> (usize, usize, usize, usize, u32, String) {
    (
        usize::from(survivor.unit_gap_bucket != control.unit_gap_bucket),
        usize::from(survivor.same_digit != control.same_digit),
        usize::from(survivor.row_m1.best_k != control.row_m1.best_k),
        pair_distance(survivor, control),
        survivor.base.abs_diff(control.base),
        control.pair_label.clone(),
    )
}

fn pair_distance(left: &BoundaryScan, right: &BoundaryScan) -> usize {
    left.unit_index_outer.abs_diff(right.unit_index_outer)
        + left.unit_index_inner.abs_diff(right.unit_index_inner)
}

fn profile_delta_summary(
    scan: &BoundaryScan,
    middle_length: usize,
    row: &KDominancePairRow,
) -> ProfileDeltaSummary {
    let k00_profile =
        scan_k_config_profile(scan.base, middle_length, scan.outer, scan.inner, (0, 0));
    let best_config = parse_k_label(&row.best_k);
    let best_profile = if best_config == (0, 0) {
        k00_profile.clone()
    } else {
        scan_k_config_profile(
            scan.base,
            middle_length,
            scan.outer,
            scan.inner,
            best_config,
        )
    };

    let modulus_relief_rows = zip_modulus_rows(
        k00_profile.modulus_divisibility_rows,
        best_profile.modulus_divisibility_rows,
        best_profile.candidates_per_config,
    );
    let top_moduli_summary = render_top_moduli(&modulus_relief_rows);

    ProfileDeltaSummary {
        admissible_delta_pp: count_delta_pp(
            best_profile.admissible_count,
            k00_profile.admissible_count,
            best_profile.candidates_per_config,
        ),
        top_moduli_summary,
        modulus_relief_rows,
    }
}

fn zip_modulus_rows(
    k00_rows: Vec<KConfigModulusDivisibilityRow>,
    best_rows: Vec<KConfigModulusDivisibilityRow>,
    candidates_per_config: usize,
) -> Vec<ModulusReliefRow> {
    let mut rows = k00_rows
        .into_iter()
        .zip(best_rows)
        .map(|(k00_row, best_row)| ModulusReliefRow {
            modulus: k00_row.modulus,
            k00_divisible_pp: count_to_pp(k00_row.divisible_count, candidates_per_config),
            best_divisible_pp: count_to_pp(best_row.divisible_count, candidates_per_config),
            relief_pp: count_delta_pp(
                k00_row.divisible_count,
                best_row.divisible_count,
                candidates_per_config,
            ),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        right
            .relief_pp
            .total_cmp(&left.relief_pp)
            .then_with(|| left.modulus.cmp(&right.modulus))
    });
    rows
}

fn render_top_moduli(rows: &[ModulusReliefRow]) -> String {
    let top_rows = rows
        .iter()
        .filter(|row| row.relief_pp > 0.0)
        .take(TOP_MODULUS_LIMIT)
        .collect::<Vec<_>>();
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

fn parse_k_label(label: &str) -> (u32, u32) {
    DEFAULT_BOUNDED_K_GRID
        .iter()
        .copied()
        .find(|&config| format_k(config) == label)
        .unwrap_or_else(|| panic!("unrecognized k label: {label}"))
}

fn build_survivor_bucket_rows(rows: &[SurvivorRow]) -> Vec<SurvivorBucketRow> {
    let mut buckets: BTreeMap<(String, String), Vec<&SurvivorRow>> = BTreeMap::new();
    for row in rows {
        add_survivor_bucket(&mut buckets, "base", &row.base.to_string(), row);
        add_survivor_bucket(&mut buckets, "unit_gap_bucket", &row.unit_gap_bucket, row);
        add_survivor_bucket(
            &mut buckets,
            "same_digit",
            if row.same_digit { "true" } else { "false" },
            row,
        );
        add_survivor_bucket(&mut buckets, "best_k_m2", &row.best_k_m2, row);
    }

    let mut bucket_rows = buckets
        .into_iter()
        .map(
            |((bucket_kind, bucket_value), group_rows)| SurvivorBucketRow {
                bucket_kind,
                bucket_value,
                survivor_count: group_rows.len(),
                persistent_survivor_count: group_rows
                    .iter()
                    .filter(|row| row.retention_share_m2_over_m1.is_some())
                    .count(),
                emergent_survivor_count: group_rows
                    .iter()
                    .filter(|row| row.retention_share_m2_over_m1.is_none())
                    .count(),
                median_anomaly_m2_pp: median(
                    group_rows.iter().map(|row| row.anomaly_m2_pp).collect(),
                ),
                median_retention_share_m2_over_m1: median_option(
                    group_rows
                        .iter()
                        .filter_map(|row| row.retention_share_m2_over_m1)
                        .collect(),
                ),
                median_admissible_delta_m2_pp: median(
                    group_rows
                        .iter()
                        .map(|row| row.admissible_delta_m2_pp)
                        .collect(),
                ),
            },
        )
        .collect::<Vec<_>>();
    bucket_rows.sort_by(|left, right| {
        right
            .survivor_count
            .cmp(&left.survivor_count)
            .then_with(|| {
                right
                    .median_anomaly_m2_pp
                    .total_cmp(&left.median_anomaly_m2_pp)
            })
            .then_with(|| left.bucket_kind.cmp(&right.bucket_kind))
            .then_with(|| left.bucket_value.cmp(&right.bucket_value))
    });
    bucket_rows
}

fn add_survivor_bucket<'a>(
    buckets: &mut BTreeMap<(String, String), Vec<&'a SurvivorRow>>,
    kind: &str,
    value: &str,
    row: &'a SurvivorRow,
) {
    buckets
        .entry((kind.to_string(), value.to_string()))
        .or_default()
        .push(row);
}

fn build_report_summary(rows: &[SurvivorRow], total_controls: usize) -> ReportSummary {
    let persistent_survivors = rows
        .iter()
        .filter(|row| row.retention_share_m2_over_m1.is_some())
        .count();
    let emergent_survivors = rows.len() - persistent_survivors;
    ReportSummary {
        total_survivors: rows.len(),
        persistent_survivors,
        emergent_survivors,
        total_controls,
        survivor_bases: join_u32s(unique_sorted(rows.iter().map(|row| row.base).collect())),
        survivors_same_digit: rows.iter().filter(|row| row.same_digit).count(),
        survivors_adjacent_gap: rows
            .iter()
            .filter(|row| row.unit_gap_bucket == "adjacent")
            .count(),
        survivors_same_gap: rows
            .iter()
            .filter(|row| row.unit_gap_bucket == "same")
            .count(),
        survivors_wide_gap: rows
            .iter()
            .filter(|row| row.unit_gap_bucket == "wide")
            .count(),
        median_anomaly_m2_pp: median(rows.iter().map(|row| row.anomaly_m2_pp).collect()),
        median_persistent_retention_share_m2_over_m1: median_option(
            rows.iter()
                .filter_map(|row| row.retention_share_m2_over_m1)
                .collect(),
        ),
        median_admissible_delta_m2_pp: median(
            rows.iter().map(|row| row.admissible_delta_m2_pp).collect(),
        ),
    }
}

fn anomaly_mass(row: &KDominancePairRow) -> f64 {
    if row.best_minus_k00_pp > 0.0 {
        row.best_minus_k00_pp
    } else {
        0.0
    }
}

fn anomaly_mass_bool(row: &KDominancePairRow) -> bool {
    anomaly_mass(row) > 0.0
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

fn unit_gap_bucket(unit_gap: usize) -> String {
    match unit_gap {
        0 => "same".to_string(),
        1 => "adjacent".to_string(),
        _ => "wide".to_string(),
    }
}

fn count_to_pp(count: usize, total: usize) -> f64 {
    count as f64 * 100.0 / total as f64
}

fn count_delta_pp(left_count: usize, right_count: usize, total: usize) -> f64 {
    (left_count as f64 - right_count as f64) * 100.0 / total as f64
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
    println!("=== M=2 Survivor Autopsy Report ===\n");
    println!(
        "Pair catalog: {} | output {}",
        bundle.settings.pair_catalog_mode, bundle.settings.out_dir
    );
    println!(
        "M=2 survivors: {} | persistent {} | emergent {} | nearby m1-only controls: {} | survivor bases {}",
        bundle.report_summary.total_survivors,
        bundle.report_summary.persistent_survivors,
        bundle.report_summary.emergent_survivors,
        bundle.report_summary.total_controls,
        bundle.report_summary.survivor_bases
    );
    println!(
        "Survivor geometry: same {} | adjacent {} | wide {} | same-digit {}",
        bundle.report_summary.survivors_same_gap,
        bundle.report_summary.survivors_adjacent_gap,
        bundle.report_summary.survivors_wide_gap,
        bundle.report_summary.survivors_same_digit
    );
    println!(
        "Median M=2 anomaly {:.2}pp | median persistent retention {} | median admissible delta {:.2}pp",
        bundle.report_summary.median_anomaly_m2_pp,
        format_share(bundle.report_summary.median_persistent_retention_share_m2_over_m1),
        bundle.report_summary.median_admissible_delta_m2_pp
    );
}

fn render_markdown_report(bundle: &ReportBundle) -> String {
    let mut lines = vec![
        "# M=2 Survivor Autopsy Report".to_string(),
        String::new(),
        "_Generated from `examples/m2_survivor_autopsy_report.rs`._".to_string(),
        String::new(),
        format!("- Generated at: `{}`", bundle.generated_at_utc),
        format!("- Bases: `{:?}`", bundle.settings.bases),
        format!("- Pair catalog: `{}`", bundle.settings.pair_catalog_mode),
        format!("- Nearby controls per survivor: `{}`", bundle.settings.nearby_control_limit),
        format!("- Bounded k-grid: `{:?}`", bundle.settings.k_grid),
        String::new(),
        "## Overall".to_string(),
        String::new(),
        format!(
            "- M=2 pairs in scope: `{}` survivors = `{}` persistent from `M=1` plus `{}` emergent at `M=2`, against `{}` nearby `m1_only` controls",
            bundle.report_summary.total_survivors,
            bundle.report_summary.persistent_survivors,
            bundle.report_summary.emergent_survivors,
            bundle.report_summary.total_controls
        ),
        format!("- Survivor bases: `{}`", bundle.report_summary.survivor_bases),
        format!(
            "- Survivor geometry: `same {}`, `adjacent {}`, `wide {}`, `same-digit {}`",
            bundle.report_summary.survivors_same_gap,
            bundle.report_summary.survivors_adjacent_gap,
            bundle.report_summary.survivors_wide_gap,
            bundle.report_summary.survivors_same_digit
        ),
        format!(
            "- Median M=2 anomaly: `{:.2}pp`; median persistent retention: `{}`; median admissible delta at M=2: `{:.2}pp`",
            bundle.report_summary.median_anomaly_m2_pp,
            format_share(bundle.report_summary.median_persistent_retention_share_m2_over_m1),
            bundle.report_summary.median_admissible_delta_m2_pp
        ),
        String::new(),
        "## Survivor Buckets".to_string(),
        String::new(),
        "| Bucket | Survivors | Persistent | Emergent | Median M=2 anomaly | Median retention | Median admissible delta M2 |".to_string(),
        "|---|---:|---:|---:|---:|---:|---:|".to_string(),
    ];

    for row in &bundle.survivor_bucket_rows {
        lines.push(format!(
            "| `{}` = `{}` | `{}` | `{}` | `{}` | `{:.2}pp` | `{}` | `{:.2}pp` |",
            row.bucket_kind,
            row.bucket_value,
            row.survivor_count,
            row.persistent_survivor_count,
            row.emergent_survivor_count,
            row.median_anomaly_m2_pp,
            format_share(row.median_retention_share_m2_over_m1),
            row.median_admissible_delta_m2_pp
        ));
    }

    lines.extend([
        String::new(),
        "## Survivor Rows".to_string(),
        String::new(),
        "| Base | Pair | Class | Gap | M2 best k | M2 anomaly | Retention | M2 admissible delta | Top moduli M2 | Nearest control |".to_string(),
        "|---:|---|---|---|---|---:|---:|---:|---|---|".to_string(),
    ]);

    for row in &bundle.survivor_rows {
        let nearest_control_label = if row.nearest_control_pair == "none" {
            "none".to_string()
        } else {
            format!(
                "{} ({}, M2 {:.2}pp, {})",
                row.nearest_control_pair,
                if row.nearest_control_same_base {
                    "same base"
                } else {
                    "fallback base"
                },
                row.nearest_control_anomaly_m2_pp,
                row.nearest_control_top_moduli_m2
            )
        };
        lines.push(format!(
            "| `{}` | {} | `{}` | `{}` | `{}` | `{:.2}pp` | `{}` | `{:.2}pp` | `{}` | {} |",
            row.base,
            row.pair_label,
            row.boundary_class,
            row.unit_gap_bucket,
            row.best_k_m2,
            row.anomaly_m2_pp,
            format_share(row.retention_share_m2_over_m1),
            row.admissible_delta_m2_pp,
            row.top_moduli_m2,
            nearest_control_label
        ));
    }

    let leading_comparisons = bundle
        .comparison_rows
        .iter()
        .filter(|row| row.comparison_rank == 1)
        .collect::<Vec<_>>();
    if !leading_comparisons.is_empty() {
        lines.extend([
            String::new(),
            "## Nearest Control Comparisons".to_string(),
            String::new(),
            "| Survivor | Control | Match | Survivor M2 | Control M2 | Survivor moduli M2 | Control moduli M2 |".to_string(),
            "|---|---|---|---:|---:|---|---|".to_string(),
        ]);
        for row in leading_comparisons {
            let match_summary = format!(
                "{} / {} / {} / d={}",
                if row.same_base { "base" } else { "fallback" },
                if row.same_gap_bucket { "gap" } else { "-" },
                if row.same_same_digit {
                    "same-digit"
                } else {
                    "-"
                },
                row.unit_distance
            );
            lines.push(format!(
                "| `{} {}` | `{} {}` | `{}` | `{:.2}pp` | `{:.2}pp` | `{}` | `{}` |",
                row.survivor_base,
                row.survivor_pair,
                row.control_base,
                row.control_pair,
                match_summary,
                row.survivor_anomaly_m2_pp,
                row.control_anomaly_m2_pp,
                row.survivor_top_moduli_m2,
                row.control_top_moduli_m2
            ));
        }
    }

    lines.join("\n")
}
