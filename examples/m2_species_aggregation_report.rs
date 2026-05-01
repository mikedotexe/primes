//! Species-level aggregation report for the short-length bounded-`k` anomaly
//! lane.
//!
//! This report groups anomalous pairs by boundary species:
//! - `m1_only`
//! - `m1_to_m2`
//! - `m2_only`
//!
//! It then aggregates geometry, winning-`k` choices, admissible-count deltas,
//! and modulus-level relief so we can compare persistence and emergence as
//! species rather than as isolated examples.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example m2_species_aggregation_report
//! cargo run --release --example m2_species_aggregation_report -- --smoke --out-dir /tmp/primes_m2_species_aggregation_smoke
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
const DEFAULT_OUT_DIR: &str = "/tmp/primes_m2_species_aggregation";
const REPORT_EXPORT_VERSION: u32 = 1;
const SMOKE_MAX_ORDERED_PAIRS_PER_BASE: usize = 8;
const SMOKE_PAIR_ANCHORS: &[(u32, u32, u32)] = &[(6, 1, 5), (10, 3, 3), (10, 3, 7), (30, 11, 7)];
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
    unit_gap_bucket: String,
    row_m1: KDominancePairRow,
    row_m2: KDominancePairRow,
    row_m3: KDominancePairRow,
    boundary_class: String,
}

#[derive(Debug, Clone)]
struct ProfileDeltaSummary {
    admissible_delta_pp: f64,
    modulus_relief_rows: Vec<ModulusReliefRow>,
}

#[derive(Debug, Clone)]
struct ModulusReliefRow {
    modulus: u32,
    relief_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    bases: Vec<u32>,
    pair_catalog_mode: String,
    middle_lengths: Vec<usize>,
    top_modulus_limit: usize,
    k_grid: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct SpeciesPairRow {
    species: String,
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    same_digit: bool,
    complement_pair: bool,
    unit_gap_bucket: String,
    best_k_m1: String,
    best_k_m2: String,
    anomaly_m1_pp: f64,
    anomaly_m2_pp: f64,
    anomaly_m3_pp: f64,
    admissible_delta_m1_pp: f64,
    admissible_delta_m2_pp: f64,
    positive_relief_moduli_m1: usize,
    positive_relief_moduli_m2: usize,
    top_moduli_m1: String,
    top_moduli_m2: String,
}

#[derive(Debug, Clone, Serialize)]
struct SpeciesSummaryRow {
    species: String,
    pair_count: usize,
    bases: String,
    same_digit_count: usize,
    complement_pair_count: usize,
    adjacent_gap_count: usize,
    same_gap_count: usize,
    wide_gap_count: usize,
    dominant_best_k_m1: String,
    dominant_best_k_m2: String,
    median_anomaly_m1_pp: f64,
    median_anomaly_m2_pp: f64,
    median_admissible_delta_m1_pp: f64,
    median_admissible_delta_m2_pp: f64,
    median_positive_relief_moduli_m1: f64,
    median_positive_relief_moduli_m2: f64,
    top_moduli_m1: String,
    top_moduli_m2: String,
}

#[derive(Debug, Clone, Serialize)]
struct SpeciesBaseRow {
    species: String,
    base: u32,
    pair_count: usize,
    same_digit_count: usize,
    adjacent_gap_count: usize,
    same_gap_count: usize,
    wide_gap_count: usize,
    dominant_best_k_m2: String,
    median_anomaly_m1_pp: f64,
    median_anomaly_m2_pp: f64,
    median_admissible_delta_m2_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct SpeciesModulusRow {
    species: String,
    middle_length: usize,
    modulus: u32,
    pair_count: usize,
    positive_relief_pairs: usize,
    positive_relief_share: f64,
    median_relief_pp: f64,
    mean_relief_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    total_pairs: usize,
    never_anomalous_pairs: usize,
    anomaly_species_pairs: usize,
    species_labels: Vec<String>,
    m1_only_pairs: usize,
    m1_to_m2_pairs: usize,
    m2_only_pairs: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    species_pair_rows: Vec<SpeciesPairRow>,
    species_summary_rows: Vec<SpeciesSummaryRow>,
    species_base_rows: Vec<SpeciesBaseRow>,
    species_modulus_rows: Vec<SpeciesModulusRow>,
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
        top_modulus_limit: TOP_MODULUS_LIMIT,
        k_grid: DEFAULT_BOUNDED_K_GRID
            .iter()
            .map(|&config| format_k(config))
            .collect(),
    };

    let scans = build_boundary_scans(options.smoke_catalog);
    let species_pair_rows = build_species_pair_rows(&scans);
    let species_modulus_rows = build_species_modulus_rows(&species_pair_rows);
    let species_summary_rows =
        build_species_summary_rows(&species_pair_rows, &species_modulus_rows);
    let species_base_rows = build_species_base_rows(&species_pair_rows);
    let report_summary = build_report_summary(&scans, &species_pair_rows);

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        species_pair_rows,
        species_summary_rows,
        species_base_rows,
        species_modulus_rows,
        report_summary,
    };

    write_csv_rows(
        options.out_dir.join("species_pair_rows.csv"),
        &bundle.species_pair_rows,
    )
    .expect("failed to write species pair rows");
    write_csv_rows(
        options.out_dir.join("species_summary_rows.csv"),
        &bundle.species_summary_rows,
    )
    .expect("failed to write species summary rows");
    write_csv_rows(
        options.out_dir.join("species_base_rows.csv"),
        &bundle.species_base_rows,
    )
    .expect("failed to write species base rows");
    write_csv_rows(
        options.out_dir.join("species_modulus_rows.csv"),
        &bundle.species_modulus_rows,
    )
    .expect("failed to write species modulus rows");
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
    println!("M=2 species aggregation report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example m2_species_aggregation_report -- [options]");
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
        unit_gap_bucket: unit_gap_bucket(unit_index_outer.abs_diff(unit_index_inner)),
        row_m1,
        row_m2,
        row_m3,
        boundary_class,
    }
}

fn build_species_pair_rows(scans: &[BoundaryScan]) -> Vec<SpeciesPairRow> {
    let anomaly_scans = scans
        .iter()
        .filter(|scan| {
            matches!(
                scan.boundary_class.as_str(),
                "m1_only" | "m1_to_m2" | "m2_only"
            )
        })
        .collect::<Vec<_>>();

    let mut rows = anomaly_scans
        .into_par_iter()
        .map(|scan| {
            let summary_m1 = profile_delta_summary(scan, M1, &scan.row_m1);
            let summary_m2 = profile_delta_summary(scan, M2, &scan.row_m2);
            SpeciesPairRow {
                species: scan.boundary_class.clone(),
                base: scan.base,
                outer: scan.outer,
                inner: scan.inner,
                pair_label: scan.pair_label.clone(),
                same_digit: scan.same_digit,
                complement_pair: scan.complement_pair,
                unit_gap_bucket: scan.unit_gap_bucket.clone(),
                best_k_m1: scan.row_m1.best_k.clone(),
                best_k_m2: scan.row_m2.best_k.clone(),
                anomaly_m1_pp: anomaly_mass(&scan.row_m1),
                anomaly_m2_pp: anomaly_mass(&scan.row_m2),
                anomaly_m3_pp: anomaly_mass(&scan.row_m3),
                admissible_delta_m1_pp: summary_m1.admissible_delta_pp,
                admissible_delta_m2_pp: summary_m2.admissible_delta_pp,
                positive_relief_moduli_m1: summary_m1
                    .modulus_relief_rows
                    .iter()
                    .filter(|row| row.relief_pp > 0.0)
                    .count(),
                positive_relief_moduli_m2: summary_m2
                    .modulus_relief_rows
                    .iter()
                    .filter(|row| row.relief_pp > 0.0)
                    .count(),
                top_moduli_m1: render_top_moduli(&summary_m1.modulus_relief_rows),
                top_moduli_m2: render_top_moduli(&summary_m2.modulus_relief_rows),
            }
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        left.species
            .cmp(&right.species)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    rows
}

fn build_species_summary_rows(
    rows: &[SpeciesPairRow],
    species_modulus_rows: &[SpeciesModulusRow],
) -> Vec<SpeciesSummaryRow> {
    let mut by_species: BTreeMap<&str, Vec<&SpeciesPairRow>> = BTreeMap::new();
    for row in rows {
        by_species.entry(&row.species).or_default().push(row);
    }

    by_species
        .into_iter()
        .map(|(species, group_rows)| SpeciesSummaryRow {
            species: species.to_string(),
            pair_count: group_rows.len(),
            bases: join_u32s(unique_sorted(
                group_rows.iter().map(|row| row.base).collect(),
            )),
            same_digit_count: group_rows.iter().filter(|row| row.same_digit).count(),
            complement_pair_count: group_rows.iter().filter(|row| row.complement_pair).count(),
            adjacent_gap_count: group_rows
                .iter()
                .filter(|row| row.unit_gap_bucket == "adjacent")
                .count(),
            same_gap_count: group_rows
                .iter()
                .filter(|row| row.unit_gap_bucket == "same")
                .count(),
            wide_gap_count: group_rows
                .iter()
                .filter(|row| row.unit_gap_bucket == "wide")
                .count(),
            dominant_best_k_m1: dominant_label(group_rows.iter().map(|row| row.best_k_m1.as_str())),
            dominant_best_k_m2: dominant_label(group_rows.iter().map(|row| row.best_k_m2.as_str())),
            median_anomaly_m1_pp: median(group_rows.iter().map(|row| row.anomaly_m1_pp).collect()),
            median_anomaly_m2_pp: median(group_rows.iter().map(|row| row.anomaly_m2_pp).collect()),
            median_admissible_delta_m1_pp: median(
                group_rows
                    .iter()
                    .map(|row| row.admissible_delta_m1_pp)
                    .collect(),
            ),
            median_admissible_delta_m2_pp: median(
                group_rows
                    .iter()
                    .map(|row| row.admissible_delta_m2_pp)
                    .collect(),
            ),
            median_positive_relief_moduli_m1: median(
                group_rows
                    .iter()
                    .map(|row| row.positive_relief_moduli_m1 as f64)
                    .collect(),
            ),
            median_positive_relief_moduli_m2: median(
                group_rows
                    .iter()
                    .map(|row| row.positive_relief_moduli_m2 as f64)
                    .collect(),
            ),
            top_moduli_m1: aggregate_top_moduli(species_modulus_rows, species, M1),
            top_moduli_m2: aggregate_top_moduli(species_modulus_rows, species, M2),
        })
        .collect()
}

fn build_species_base_rows(rows: &[SpeciesPairRow]) -> Vec<SpeciesBaseRow> {
    let mut by_species_base: BTreeMap<(&str, u32), Vec<&SpeciesPairRow>> = BTreeMap::new();
    for row in rows {
        by_species_base
            .entry((&row.species, row.base))
            .or_default()
            .push(row);
    }

    by_species_base
        .into_iter()
        .map(|((species, base), group_rows)| SpeciesBaseRow {
            species: species.to_string(),
            base,
            pair_count: group_rows.len(),
            same_digit_count: group_rows.iter().filter(|row| row.same_digit).count(),
            adjacent_gap_count: group_rows
                .iter()
                .filter(|row| row.unit_gap_bucket == "adjacent")
                .count(),
            same_gap_count: group_rows
                .iter()
                .filter(|row| row.unit_gap_bucket == "same")
                .count(),
            wide_gap_count: group_rows
                .iter()
                .filter(|row| row.unit_gap_bucket == "wide")
                .count(),
            dominant_best_k_m2: dominant_label(group_rows.iter().map(|row| row.best_k_m2.as_str())),
            median_anomaly_m1_pp: median(group_rows.iter().map(|row| row.anomaly_m1_pp).collect()),
            median_anomaly_m2_pp: median(group_rows.iter().map(|row| row.anomaly_m2_pp).collect()),
            median_admissible_delta_m2_pp: median(
                group_rows
                    .iter()
                    .map(|row| row.admissible_delta_m2_pp)
                    .collect(),
            ),
        })
        .collect()
}

fn build_species_modulus_rows(rows: &[SpeciesPairRow]) -> Vec<SpeciesModulusRow> {
    let mut by_key: BTreeMap<(String, usize, u32), Vec<f64>> = BTreeMap::new();

    let scans = rows
        .iter()
        .map(|row| {
            let summary_m1 = species_row_profile_summary(row, M1);
            let summary_m2 = species_row_profile_summary(row, M2);
            (row, summary_m1, summary_m2)
        })
        .collect::<Vec<_>>();

    for (row, summary_m1, summary_m2) in scans {
        for (middle_length, summary) in [(M1, summary_m1), (M2, summary_m2)] {
            for relief_row in summary.modulus_relief_rows {
                by_key
                    .entry((row.species.clone(), middle_length, relief_row.modulus))
                    .or_default()
                    .push(relief_row.relief_pp);
            }
        }
    }

    let mut modulus_rows = by_key
        .into_iter()
        .map(|((species, middle_length, modulus), reliefs)| {
            let positive_relief_pairs = reliefs.iter().filter(|&&value| value > 0.0).count();
            let pair_count = reliefs.len();
            SpeciesModulusRow {
                species,
                middle_length,
                modulus,
                pair_count,
                positive_relief_pairs,
                positive_relief_share: positive_relief_pairs as f64 / pair_count as f64,
                median_relief_pp: median(reliefs.clone()),
                mean_relief_pp: reliefs.iter().sum::<f64>() / pair_count as f64,
            }
        })
        .collect::<Vec<_>>();

    modulus_rows.sort_by(|left, right| {
        left.species
            .cmp(&right.species)
            .then_with(|| left.middle_length.cmp(&right.middle_length))
            .then_with(|| {
                right
                    .positive_relief_share
                    .total_cmp(&left.positive_relief_share)
            })
            .then_with(|| right.mean_relief_pp.total_cmp(&left.mean_relief_pp))
            .then_with(|| left.modulus.cmp(&right.modulus))
    });
    modulus_rows
}

fn species_row_profile_summary(row: &SpeciesPairRow, middle_length: usize) -> ProfileDeltaSummary {
    let best_k = if middle_length == M1 {
        parse_k_label(&row.best_k_m1)
    } else {
        parse_k_label(&row.best_k_m2)
    };
    let k00_profile = scan_k_config_profile(row.base, middle_length, row.outer, row.inner, (0, 0));
    let best_profile = if best_k == (0, 0) {
        k00_profile.clone()
    } else {
        scan_k_config_profile(row.base, middle_length, row.outer, row.inner, best_k)
    };

    let modulus_relief_rows = zip_modulus_rows(
        k00_profile.modulus_divisibility_rows,
        best_profile.modulus_divisibility_rows,
        best_profile.candidates_per_config,
    );

    ProfileDeltaSummary {
        admissible_delta_pp: count_delta_pp(
            best_profile.admissible_count,
            k00_profile.admissible_count,
            best_profile.candidates_per_config,
        ),
        modulus_relief_rows,
    }
}

fn profile_delta_summary(
    scan: &BoundaryScan,
    middle_length: usize,
    row: &KDominancePairRow,
) -> ProfileDeltaSummary {
    let k00_profile =
        scan_k_config_profile(scan.base, middle_length, scan.outer, scan.inner, (0, 0));
    let best_k = parse_k_label(&row.best_k);
    let best_profile = if best_k == (0, 0) {
        k00_profile.clone()
    } else {
        scan_k_config_profile(scan.base, middle_length, scan.outer, scan.inner, best_k)
    };

    ProfileDeltaSummary {
        admissible_delta_pp: count_delta_pp(
            best_profile.admissible_count,
            k00_profile.admissible_count,
            best_profile.candidates_per_config,
        ),
        modulus_relief_rows: zip_modulus_rows(
            k00_profile.modulus_divisibility_rows,
            best_profile.modulus_divisibility_rows,
            best_profile.candidates_per_config,
        ),
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

fn aggregate_top_moduli(
    species_modulus_rows: &[SpeciesModulusRow],
    species: &str,
    middle_length: usize,
) -> String {
    let modulus_rows = species_modulus_rows
        .iter()
        .filter(|row| row.species == species && row.middle_length == middle_length)
        .filter(|row| row.positive_relief_share > 0.0)
        .take(TOP_MODULUS_LIMIT)
        .collect::<Vec<_>>();
    if modulus_rows.is_empty() {
        "none".to_string()
    } else {
        modulus_rows
            .into_iter()
            .map(|row| {
                format!(
                    "p{}:{:.0}%/{:.2}pp",
                    row.modulus,
                    row.positive_relief_share * 100.0,
                    row.mean_relief_pp
                )
            })
            .collect::<Vec<_>>()
            .join(";")
    }
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

fn build_report_summary(scans: &[BoundaryScan], rows: &[SpeciesPairRow]) -> ReportSummary {
    ReportSummary {
        total_pairs: scans.len(),
        never_anomalous_pairs: scans
            .iter()
            .filter(|scan| scan.boundary_class == "never_anomalous")
            .count(),
        anomaly_species_pairs: rows.len(),
        species_labels: vec![
            "m1_only".to_string(),
            "m1_to_m2".to_string(),
            "m2_only".to_string(),
        ],
        m1_only_pairs: rows.iter().filter(|row| row.species == "m1_only").count(),
        m1_to_m2_pairs: rows.iter().filter(|row| row.species == "m1_to_m2").count(),
        m2_only_pairs: rows.iter().filter(|row| row.species == "m2_only").count(),
    }
}

fn parse_k_label(label: &str) -> (u32, u32) {
    DEFAULT_BOUNDED_K_GRID
        .iter()
        .copied()
        .find(|&config| format_k(config) == label)
        .unwrap_or_else(|| panic!("unrecognized k label: {label}"))
}

fn dominant_label<'a>(labels: impl Iterator<Item = &'a str>) -> String {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for label in labels {
        *counts.entry(label).or_default() += 1;
    }
    counts
        .into_iter()
        .max_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(right.0)))
        .map(|(label, _)| label.to_string())
        .unwrap_or_else(|| "none".to_string())
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

fn print_summary(bundle: &ReportBundle) {
    println!("=== M=2 Species Aggregation Report ===\n");
    println!(
        "Pair catalog: {} | output {}",
        bundle.settings.pair_catalog_mode, bundle.settings.out_dir
    );
    println!(
        "Total pairs {} | anomaly species {} | never anomalous {}",
        bundle.report_summary.total_pairs,
        bundle.report_summary.anomaly_species_pairs,
        bundle.report_summary.never_anomalous_pairs
    );
    println!(
        "Species counts: m1_only {} | m1_to_m2 {} | m2_only {}",
        bundle.report_summary.m1_only_pairs,
        bundle.report_summary.m1_to_m2_pairs,
        bundle.report_summary.m2_only_pairs
    );
}

fn render_markdown_report(bundle: &ReportBundle) -> String {
    let mut lines = vec![
        "# M=2 Species Aggregation Report".to_string(),
        String::new(),
        "_Generated from `examples/m2_species_aggregation_report.rs`._".to_string(),
        String::new(),
        format!("- Generated at: `{}`", bundle.generated_at_utc),
        format!("- Bases: `{:?}`", bundle.settings.bases),
        format!("- Pair catalog: `{}`", bundle.settings.pair_catalog_mode),
        format!("- Bounded k-grid: `{:?}`", bundle.settings.k_grid),
        String::new(),
        "## Overall".to_string(),
        String::new(),
        format!(
            "- Total pairs: `{}`; anomaly-species pairs: `{}`; never anomalous: `{}`",
            bundle.report_summary.total_pairs,
            bundle.report_summary.anomaly_species_pairs,
            bundle.report_summary.never_anomalous_pairs
        ),
        format!(
            "- Species counts: `m1_only {}`, `m1_to_m2 {}`, `m2_only {}`",
            bundle.report_summary.m1_only_pairs,
            bundle.report_summary.m1_to_m2_pairs,
            bundle.report_summary.m2_only_pairs
        ),
        String::new(),
        "## Species Summary".to_string(),
        String::new(),
        "| Species | Pairs | Bases | Gap mix (same/adj/wide) | Dominant k at M1 | Dominant k at M2 | Median anomalies (M1/M2) | Median admissible deltas (M1/M2) | Top moduli M2 |".to_string(),
        "|---|---:|---|---|---|---|---|---|---|".to_string(),
    ];

    for row in &bundle.species_summary_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}/{}/{}` | `{}` | `{}` | `{:.2}pp / {:.2}pp` | `{:.2}pp / {:.2}pp` | `{}` |",
            row.species,
            row.pair_count,
            row.bases,
            row.same_gap_count,
            row.adjacent_gap_count,
            row.wide_gap_count,
            row.dominant_best_k_m1,
            row.dominant_best_k_m2,
            row.median_anomaly_m1_pp,
            row.median_anomaly_m2_pp,
            row.median_admissible_delta_m1_pp,
            row.median_admissible_delta_m2_pp,
            row.top_moduli_m2
        ));
    }

    lines.extend([
        String::new(),
        "## Species Base Rows".to_string(),
        String::new(),
        "| Species | Base | Pairs | Gap mix (same/adj/wide) | Dominant k at M2 | Median M2 anomaly | Median M2 admissible delta |".to_string(),
        "|---|---:|---:|---|---|---:|---:|".to_string(),
    ]);

    for row in &bundle.species_base_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}/{}/{}` | `{}` | `{:.2}pp` | `{:.2}pp` |",
            row.species,
            row.base,
            row.pair_count,
            row.same_gap_count,
            row.adjacent_gap_count,
            row.wide_gap_count,
            row.dominant_best_k_m2,
            row.median_anomaly_m2_pp,
            row.median_admissible_delta_m2_pp
        ));
    }

    let top_modulus_rows = bundle
        .species_modulus_rows
        .iter()
        .filter(|row| row.middle_length == M2 && row.positive_relief_share > 0.0)
        .take(12)
        .collect::<Vec<_>>();
    if !top_modulus_rows.is_empty() {
        lines.extend([
            String::new(),
            "## Top Modulus Rows".to_string(),
            String::new(),
            "| Species | M | Modulus | Positive relief share | Mean relief | Median relief |"
                .to_string(),
            "|---|---:|---:|---:|---:|---:|".to_string(),
        ]);
        for row in top_modulus_rows {
            lines.push(format!(
                "| `{}` | `{}` | `{}` | `{:.0}%` | `{:.2}pp` | `{:.2}pp` |",
                row.species,
                row.middle_length,
                row.modulus,
                row.positive_relief_share * 100.0,
                row.mean_relief_pp,
                row.median_relief_pp
            ));
        }
    }

    lines.join("\n")
}
