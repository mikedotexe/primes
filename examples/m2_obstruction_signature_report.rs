//! Composite obstruction-signature report for the `M=2` boundary survivors.
//!
//! This report is downstream of `m2_survivor_autopsy_report`: it reads the
//! full survivor/control artifact, reconstructs each survivor's exact `M=2`
//! modulus-relief vector, and summarizes that vector with a small family of
//! composite signature metrics.
//!
//! The goal is to ask a sharper question than "which top prime helped?".
//! We track whether a survivor is supported by:
//! - admissible-count lift
//! - broad positive relief across several moduli
//! - concentrated relief at one modulus
//! - or, strikingly, none of the above
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example m2_obstruction_signature_report
//! cargo run --release --example m2_obstruction_signature_report -- --input-json /tmp/primes_m2_survivor_autopsy/summary.json --out-dir /tmp/primes_m2_obstruction_signature
//! ```

use plotters::prelude::*;
use primes::validation::reporting::{
    ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};

const DEFAULT_INPUT_JSON: &str = "/tmp/primes_m2_survivor_autopsy/summary.json";
const DEFAULT_OUT_DIR: &str = "/tmp/primes_m2_obstruction_signature";
const REPORT_EXPORT_VERSION: u32 = 1;
const TARGET_MIDDLE_LENGTH: usize = 2;

#[derive(Debug)]
struct Options {
    input_json: PathBuf,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct InputSettings {
    pair_catalog_mode: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SurvivorRow {
    base: u32,
    pair_label: String,
    boundary_class: String,
    anomaly_m2_pp: f64,
    admissible_delta_m2_pp: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct ComparisonRow {
    survivor_base: u32,
    survivor_pair: String,
    control_base: u32,
    control_pair: String,
    comparison_rank: usize,
    same_base: bool,
    unit_distance: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct ResidueComparisonRow {
    survivor_base: u32,
    survivor_pair: String,
    control_base: u32,
    control_pair: String,
    comparison_rank: usize,
    middle_length: usize,
    modulus: u32,
    survivor_relief_pp: f64,
    control_relief_pp: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct InputBundle {
    generated_at_utc: String,
    settings: InputSettings,
    survivor_rows: Vec<SurvivorRow>,
    comparison_rows: Vec<ComparisonRow>,
    residue_comparison_rows: Vec<ResidueComparisonRow>,
}

#[derive(Debug, Clone, Serialize)]
struct SignatureRow {
    role: String,
    row_label: String,
    anchor_survivor: String,
    base: u32,
    pair_label: String,
    anomaly_m2_pp: f64,
    admissible_delta_m2_pp: f64,
    positive_relief_count: usize,
    negative_relief_count: usize,
    total_positive_relief_pp: f64,
    total_negative_relief_pp: f64,
    net_relief_pp: f64,
    total_abs_relief_pp: f64,
    peak_positive_relief_pp: f64,
    most_negative_relief_pp: f64,
    positive_relief_concentration: f64,
    weighted_positive_modulus_mean: Option<f64>,
    top_positive_modulus: Option<u32>,
    zero_positive_signature: bool,
    same_base_as_anchor: bool,
    unit_distance_from_anchor: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ComparisonDeltaRow {
    anchor_survivor: String,
    survivor_role: String,
    survivor_base: u32,
    survivor_pair: String,
    control_base: u32,
    control_pair: String,
    same_base: bool,
    unit_distance: usize,
    survivor_anomaly_m2_pp: f64,
    control_anomaly_m2_pp: f64,
    delta_anomaly_m2_pp: f64,
    survivor_admissible_delta_m2_pp: f64,
    control_admissible_delta_m2_pp: f64,
    delta_admissible_delta_m2_pp: f64,
    survivor_total_positive_relief_pp: f64,
    control_total_positive_relief_pp: f64,
    delta_total_positive_relief_pp: f64,
    survivor_total_negative_relief_pp: f64,
    control_total_negative_relief_pp: f64,
    delta_total_negative_relief_pp: f64,
    survivor_net_relief_pp: f64,
    control_net_relief_pp: f64,
    delta_net_relief_pp: f64,
    survivor_zero_positive_signature: bool,
    control_zero_positive_signature: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ClassSummaryRow {
    role: String,
    row_count: usize,
    median_anomaly_m2_pp: f64,
    median_admissible_delta_m2_pp: f64,
    median_total_positive_relief_pp: f64,
    median_total_negative_relief_pp: f64,
    median_net_relief_pp: f64,
    median_total_abs_relief_pp: f64,
    median_positive_relief_count: f64,
    median_positive_relief_concentration: f64,
    zero_positive_signature_rows: usize,
}

#[derive(Debug, Clone, Serialize)]
struct CorrelationRow {
    metric_name: String,
    pearson_r_with_anomaly_m2: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    survivor_rows: usize,
    persistent_survivors: usize,
    emergent_survivors: usize,
    control_rows: usize,
    zero_positive_signature_survivors: usize,
    strongest_anomaly_pair: String,
    strongest_anomaly_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct OutputBundle {
    export_version: u32,
    generated_at_utc: String,
    input_json: String,
    input_generated_at_utc: String,
    pair_catalog_mode: String,
    signature_rows: Vec<SignatureRow>,
    comparison_delta_rows: Vec<ComparisonDeltaRow>,
    class_summary_rows: Vec<ClassSummaryRow>,
    correlation_rows: Vec<CorrelationRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

struct SignatureSeed<'a> {
    role: &'a str,
    row_label: &'a str,
    anchor_survivor: String,
    pair_label: String,
    base: u32,
    anomaly_m2_pp: f64,
    admissible_delta_m2_pp: f64,
    same_base_as_anchor: bool,
    unit_distance_from_anchor: usize,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let input_bundle = load_input_bundle(&options.input_json);
    let (signature_rows, comparison_delta_rows) = build_signature_rows(&input_bundle);
    let class_summary_rows = build_class_summary_rows(&signature_rows);
    let correlation_rows = build_correlation_rows(&signature_rows);
    let report_summary = build_report_summary(&signature_rows);

    let signature_plane_path = options.out_dir.join("signature_plane.png");
    render_signature_plane(
        &signature_rows,
        &comparison_delta_rows,
        &signature_plane_path,
    );
    let heatmap_path = options.out_dir.join("signature_metric_heatmap.png");
    render_signature_heatmap(&signature_rows, &heatmap_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "signature_plane".to_string(),
            label: "M=2 obstruction signature plane".to_string(),
            path: signature_plane_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "signature_heatmap".to_string(),
            label: "M=2 obstruction signature metric heatmap".to_string(),
            path: heatmap_path.display().to_string(),
        },
    ];
    let observations = derive_observations(&signature_rows, &class_summary_rows, &correlation_rows);

    let bundle = OutputBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        input_json: options.input_json.display().to_string(),
        input_generated_at_utc: input_bundle.generated_at_utc.clone(),
        pair_catalog_mode: input_bundle.settings.pair_catalog_mode.clone(),
        signature_rows,
        comparison_delta_rows,
        class_summary_rows,
        correlation_rows,
        image_artifact_rows,
        report_summary,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("signature_rows.csv"),
        &bundle.signature_rows,
    )
    .expect("failed to write signature rows");
    write_csv_rows(
        options.out_dir.join("comparison_delta_rows.csv"),
        &bundle.comparison_delta_rows,
    )
    .expect("failed to write comparison delta rows");
    write_csv_rows(
        options.out_dir.join("class_summary_rows.csv"),
        &bundle.class_summary_rows,
    )
    .expect("failed to write class summary rows");
    write_csv_rows(
        options.out_dir.join("correlation_rows.csv"),
        &bundle.correlation_rows,
    )
    .expect("failed to write correlation rows");
    write_csv_rows(
        options.out_dir.join("image_artifact_rows.csv"),
        &bundle.image_artifact_rows,
    )
    .expect("failed to write image artifact rows");
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
    let mut input_json = PathBuf::from(DEFAULT_INPUT_JSON);
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input-json" => {
                input_json = PathBuf::from(parse_next::<String>(&mut args, "--input-json"));
            }
            "--out-dir" => {
                out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir"));
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
        input_json,
        out_dir,
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
    println!("M=2 obstruction signature report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example m2_obstruction_signature_report -- [options]");
    println!();
    println!("Options:");
    println!(
        "  --input-json <path>       Survivor autopsy artifact (default: {DEFAULT_INPUT_JSON})"
    );
    println!(
        "  --out-dir <path>          Output directory for images and summary files (default: {DEFAULT_OUT_DIR})"
    );
}

fn load_input_bundle(path: &Path) -> InputBundle {
    let contents = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read input JSON at {}: {err}\nRun `cargo run --release --example m2_survivor_autopsy_report` first or pass --input-json.",
            path.display()
        );
        std::process::exit(1);
    });
    serde_json::from_str(&contents).unwrap_or_else(|err| {
        eprintln!("Failed to parse input JSON at {}: {err}", path.display());
        std::process::exit(1);
    })
}

fn build_signature_rows(
    input_bundle: &InputBundle,
) -> (Vec<SignatureRow>, Vec<ComparisonDeltaRow>) {
    let survivor_lookup = input_bundle
        .survivor_rows
        .iter()
        .map(|row| ((row.base, row.pair_label.clone()), row))
        .collect::<BTreeMap<_, _>>();
    let rank1_comparisons = input_bundle
        .comparison_rows
        .iter()
        .filter(|row| row.comparison_rank == 1)
        .collect::<Vec<_>>();
    let survivor_order = sorted_survivors(&input_bundle.survivor_rows);

    let mut signature_rows = Vec::new();
    let mut comparison_delta_rows = Vec::new();

    for survivor in survivor_order {
        let comparison = rank1_comparisons
            .iter()
            .find(|row| {
                row.survivor_base == survivor.base && row.survivor_pair == survivor.pair_label
            })
            .expect("each survivor should have a rank-1 comparison");
        let survivor_relief_rows = collect_relief_rows(
            &input_bundle.residue_comparison_rows,
            (survivor.base, survivor.pair_label.clone()),
            Some((
                comparison.control_base,
                comparison.control_pair.clone(),
                comparison.comparison_rank,
            )),
            false,
        );
        let control_relief_rows = collect_relief_rows(
            &input_bundle.residue_comparison_rows,
            (survivor.base, survivor.pair_label.clone()),
            Some((
                comparison.control_base,
                comparison.control_pair.clone(),
                comparison.comparison_rank,
            )),
            true,
        );
        let survivor_label = format!(
            "{} {}",
            short_role_label(&survivor.boundary_class),
            survivor.pair_label
        );
        let control_label = format!(
            "ctrl {} <- {}",
            comparison.control_pair, survivor.pair_label
        );
        let survivor_signature = build_signature(
            SignatureSeed {
                role: if survivor.boundary_class == "m1_to_m2" {
                    "persistent_survivor"
                } else {
                    "emergent_survivor"
                },
                row_label: &survivor_label,
                anchor_survivor: survivor.pair_label.clone(),
                pair_label: survivor.pair_label.clone(),
                base: survivor.base,
                anomaly_m2_pp: survivor.anomaly_m2_pp,
                admissible_delta_m2_pp: survivor.admissible_delta_m2_pp,
                same_base_as_anchor: true,
                unit_distance_from_anchor: 0,
            },
            &survivor_relief_rows,
        );
        let control_signature = build_signature(
            SignatureSeed {
                role: "nearest_dead_control",
                row_label: &control_label,
                anchor_survivor: survivor.pair_label.clone(),
                pair_label: comparison.control_pair.clone(),
                base: comparison.control_base,
                anomaly_m2_pp: 0.0,
                admissible_delta_m2_pp: 0.0,
                same_base_as_anchor: comparison.same_base,
                unit_distance_from_anchor: comparison.unit_distance,
            },
            &control_relief_rows,
        );

        comparison_delta_rows.push(ComparisonDeltaRow {
            anchor_survivor: survivor.pair_label.clone(),
            survivor_role: survivor_signature.role.clone(),
            survivor_base: survivor.base,
            survivor_pair: survivor.pair_label.clone(),
            control_base: comparison.control_base,
            control_pair: comparison.control_pair.clone(),
            same_base: comparison.same_base,
            unit_distance: comparison.unit_distance,
            survivor_anomaly_m2_pp: survivor_signature.anomaly_m2_pp,
            control_anomaly_m2_pp: control_signature.anomaly_m2_pp,
            delta_anomaly_m2_pp: survivor_signature.anomaly_m2_pp - control_signature.anomaly_m2_pp,
            survivor_admissible_delta_m2_pp: survivor_signature.admissible_delta_m2_pp,
            control_admissible_delta_m2_pp: control_signature.admissible_delta_m2_pp,
            delta_admissible_delta_m2_pp: survivor_signature.admissible_delta_m2_pp
                - control_signature.admissible_delta_m2_pp,
            survivor_total_positive_relief_pp: survivor_signature.total_positive_relief_pp,
            control_total_positive_relief_pp: control_signature.total_positive_relief_pp,
            delta_total_positive_relief_pp: survivor_signature.total_positive_relief_pp
                - control_signature.total_positive_relief_pp,
            survivor_total_negative_relief_pp: survivor_signature.total_negative_relief_pp,
            control_total_negative_relief_pp: control_signature.total_negative_relief_pp,
            delta_total_negative_relief_pp: survivor_signature.total_negative_relief_pp
                - control_signature.total_negative_relief_pp,
            survivor_net_relief_pp: survivor_signature.net_relief_pp,
            control_net_relief_pp: control_signature.net_relief_pp,
            delta_net_relief_pp: survivor_signature.net_relief_pp - control_signature.net_relief_pp,
            survivor_zero_positive_signature: survivor_signature.zero_positive_signature,
            control_zero_positive_signature: control_signature.zero_positive_signature,
        });

        signature_rows.push(control_signature);
        signature_rows.push(survivor_signature);
    }

    signature_rows.sort_by(|left, right| {
        left.anchor_survivor
            .cmp(&right.anchor_survivor)
            .then_with(|| row_group_rank(&left.role).cmp(&row_group_rank(&right.role)))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    comparison_delta_rows.sort_by(|left, right| {
        role_sort_rank(&left.survivor_role)
            .cmp(&role_sort_rank(&right.survivor_role))
            .then_with(|| {
                right
                    .delta_anomaly_m2_pp
                    .total_cmp(&left.delta_anomaly_m2_pp)
            })
            .then_with(|| left.anchor_survivor.cmp(&right.anchor_survivor))
    });

    for row in &signature_rows {
        if row.role != "nearest_dead_control" {
            let _ = survivor_lookup
                .get(&(row.base, row.pair_label.clone()))
                .expect("survivor signature should map back to survivor row");
        }
    }

    (signature_rows, comparison_delta_rows)
}

fn sorted_survivors(rows: &[SurvivorRow]) -> Vec<&SurvivorRow> {
    let mut sorted = rows.iter().collect::<Vec<_>>();
    sorted.sort_by(|left, right| {
        role_sort_rank(if left.boundary_class == "m1_to_m2" {
            "persistent_survivor"
        } else {
            "emergent_survivor"
        })
        .cmp(&role_sort_rank(if right.boundary_class == "m1_to_m2" {
            "persistent_survivor"
        } else {
            "emergent_survivor"
        }))
        .then_with(|| right.anomaly_m2_pp.total_cmp(&left.anomaly_m2_pp))
        .then_with(|| left.base.cmp(&right.base))
        .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    sorted
}

fn collect_relief_rows(
    rows: &[ResidueComparisonRow],
    survivor_key: (u32, String),
    control_key: Option<(u32, String, usize)>,
    use_control_relief: bool,
) -> Vec<(u32, f64)> {
    let mut by_modulus = BTreeMap::new();
    for row in rows.iter().filter(|row| {
        row.middle_length == TARGET_MIDDLE_LENGTH
            && row.survivor_base == survivor_key.0
            && row.survivor_pair == survivor_key.1
    }) {
        if let Some((control_base, ref control_pair, comparison_rank)) = control_key {
            if row.control_base != control_base
                || row.control_pair != *control_pair
                || row.comparison_rank != comparison_rank
            {
                continue;
            }
        }
        by_modulus.insert(
            row.modulus,
            if use_control_relief {
                row.control_relief_pp
            } else {
                row.survivor_relief_pp
            },
        );
    }
    by_modulus.into_iter().collect()
}

fn build_signature(seed: SignatureSeed<'_>, relief_rows: &[(u32, f64)]) -> SignatureRow {
    let positive_rows = relief_rows
        .iter()
        .copied()
        .filter(|(_, value)| *value > 0.0)
        .collect::<Vec<_>>();
    let negative_rows = relief_rows
        .iter()
        .copied()
        .filter(|(_, value)| *value < 0.0)
        .collect::<Vec<_>>();
    let total_positive_relief_pp = positive_rows.iter().map(|(_, value)| *value).sum::<f64>();
    let total_negative_relief_pp = negative_rows
        .iter()
        .map(|(_, value)| value.abs())
        .sum::<f64>();
    let net_relief_pp = relief_rows.iter().map(|(_, value)| *value).sum::<f64>();
    let total_abs_relief_pp = relief_rows
        .iter()
        .map(|(_, value)| value.abs())
        .sum::<f64>();
    let peak_positive = positive_rows
        .iter()
        .max_by(|left, right| left.1.total_cmp(&right.1))
        .copied();
    let most_negative = negative_rows
        .iter()
        .min_by(|left, right| left.1.total_cmp(&right.1))
        .copied();

    SignatureRow {
        role: seed.role.to_string(),
        row_label: seed.row_label.to_string(),
        anchor_survivor: seed.anchor_survivor,
        base: seed.base,
        pair_label: seed.pair_label,
        anomaly_m2_pp: seed.anomaly_m2_pp,
        admissible_delta_m2_pp: seed.admissible_delta_m2_pp,
        positive_relief_count: positive_rows.len(),
        negative_relief_count: negative_rows.len(),
        total_positive_relief_pp,
        total_negative_relief_pp,
        net_relief_pp,
        total_abs_relief_pp,
        peak_positive_relief_pp: peak_positive.map(|(_, value)| value).unwrap_or(0.0),
        most_negative_relief_pp: most_negative.map(|(_, value)| value).unwrap_or(0.0),
        positive_relief_concentration: if total_positive_relief_pp > 0.0 {
            peak_positive
                .map(|(_, value)| value / total_positive_relief_pp)
                .unwrap_or(0.0)
        } else {
            0.0
        },
        weighted_positive_modulus_mean: if total_positive_relief_pp > 0.0 {
            Some(
                positive_rows
                    .iter()
                    .map(|(modulus, value)| *modulus as f64 * *value)
                    .sum::<f64>()
                    / total_positive_relief_pp,
            )
        } else {
            None
        },
        top_positive_modulus: peak_positive.map(|(modulus, _)| modulus),
        zero_positive_signature: total_positive_relief_pp == 0.0,
        same_base_as_anchor: seed.same_base_as_anchor,
        unit_distance_from_anchor: seed.unit_distance_from_anchor,
    }
}

fn build_class_summary_rows(rows: &[SignatureRow]) -> Vec<ClassSummaryRow> {
    let mut by_role: BTreeMap<String, Vec<&SignatureRow>> = BTreeMap::new();
    for row in rows {
        by_role.entry(row.role.clone()).or_default().push(row);
    }

    by_role
        .into_iter()
        .map(|(role, group_rows)| ClassSummaryRow {
            role,
            row_count: group_rows.len(),
            median_anomaly_m2_pp: median(group_rows.iter().map(|row| row.anomaly_m2_pp).collect()),
            median_admissible_delta_m2_pp: median(
                group_rows
                    .iter()
                    .map(|row| row.admissible_delta_m2_pp)
                    .collect(),
            ),
            median_total_positive_relief_pp: median(
                group_rows
                    .iter()
                    .map(|row| row.total_positive_relief_pp)
                    .collect(),
            ),
            median_total_negative_relief_pp: median(
                group_rows
                    .iter()
                    .map(|row| row.total_negative_relief_pp)
                    .collect(),
            ),
            median_net_relief_pp: median(group_rows.iter().map(|row| row.net_relief_pp).collect()),
            median_total_abs_relief_pp: median(
                group_rows
                    .iter()
                    .map(|row| row.total_abs_relief_pp)
                    .collect(),
            ),
            median_positive_relief_count: median(
                group_rows
                    .iter()
                    .map(|row| row.positive_relief_count as f64)
                    .collect(),
            ),
            median_positive_relief_concentration: median(
                group_rows
                    .iter()
                    .map(|row| row.positive_relief_concentration)
                    .collect(),
            ),
            zero_positive_signature_rows: group_rows
                .iter()
                .filter(|row| row.zero_positive_signature)
                .count(),
        })
        .collect()
}

fn build_correlation_rows(rows: &[SignatureRow]) -> Vec<CorrelationRow> {
    let survivors = rows
        .iter()
        .filter(|row| row.role != "nearest_dead_control")
        .collect::<Vec<_>>();
    vec![
        CorrelationRow {
            metric_name: "admissible_delta_m2_pp".to_string(),
            pearson_r_with_anomaly_m2: pearson_correlation(
                survivors
                    .iter()
                    .map(|row| row.admissible_delta_m2_pp)
                    .collect(),
                survivors.iter().map(|row| row.anomaly_m2_pp).collect(),
            ),
        },
        CorrelationRow {
            metric_name: "total_positive_relief_pp".to_string(),
            pearson_r_with_anomaly_m2: pearson_correlation(
                survivors
                    .iter()
                    .map(|row| row.total_positive_relief_pp)
                    .collect(),
                survivors.iter().map(|row| row.anomaly_m2_pp).collect(),
            ),
        },
        CorrelationRow {
            metric_name: "total_abs_relief_pp".to_string(),
            pearson_r_with_anomaly_m2: pearson_correlation(
                survivors
                    .iter()
                    .map(|row| row.total_abs_relief_pp)
                    .collect(),
                survivors.iter().map(|row| row.anomaly_m2_pp).collect(),
            ),
        },
        CorrelationRow {
            metric_name: "net_relief_pp".to_string(),
            pearson_r_with_anomaly_m2: pearson_correlation(
                survivors.iter().map(|row| row.net_relief_pp).collect(),
                survivors.iter().map(|row| row.anomaly_m2_pp).collect(),
            ),
        },
    ]
}

fn build_report_summary(rows: &[SignatureRow]) -> ReportSummary {
    let survivors = rows
        .iter()
        .filter(|row| row.role != "nearest_dead_control")
        .collect::<Vec<_>>();
    let controls = rows
        .iter()
        .filter(|row| row.role == "nearest_dead_control")
        .count();
    let strongest = survivors
        .iter()
        .max_by(|left, right| left.anomaly_m2_pp.total_cmp(&right.anomaly_m2_pp))
        .expect("survivor signatures should exist");

    ReportSummary {
        survivor_rows: survivors.len(),
        persistent_survivors: survivors
            .iter()
            .filter(|row| row.role == "persistent_survivor")
            .count(),
        emergent_survivors: survivors
            .iter()
            .filter(|row| row.role == "emergent_survivor")
            .count(),
        control_rows: controls,
        zero_positive_signature_survivors: survivors
            .iter()
            .filter(|row| row.zero_positive_signature)
            .count(),
        strongest_anomaly_pair: strongest.pair_label.clone(),
        strongest_anomaly_pp: strongest.anomaly_m2_pp,
    }
}

fn render_signature_plane(rows: &[SignatureRow], comparisons: &[ComparisonDeltaRow], path: &Path) {
    let min_x = rows
        .iter()
        .map(|row| row.admissible_delta_m2_pp)
        .fold(0.0_f64, f64::min)
        - 0.75;
    let max_x = rows
        .iter()
        .map(|row| row.admissible_delta_m2_pp)
        .fold(0.0_f64, f64::max)
        + 0.75;
    let min_y = rows
        .iter()
        .map(|row| row.net_relief_pp)
        .fold(0.0_f64, f64::min)
        - 0.75;
    let max_y = rows
        .iter()
        .map(|row| row.net_relief_pp)
        .fold(0.0_f64, f64::max)
        + 0.75;
    let max_anomaly = rows
        .iter()
        .map(|row| row.anomaly_m2_pp)
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let root = BitMapBackend::new(path, (1080, 860)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill signature plane canvas");

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "M=2 Obstruction Signature Plane  (x = admissible lift, y = net modulus relief)",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(55)
        .y_label_area_size(70)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)
        .expect("failed to build signature plane");

    chart
        .configure_mesh()
        .x_desc("admissible delta at M=2 (pp)")
        .y_desc("net modulus relief at M=2 (pp)")
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(222, 215, 205))
        .draw()
        .expect("failed to draw signature plane mesh");

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(0.0, min_y), (0.0, max_y)],
            RGBColor(150, 143, 132).stroke_width(2),
        )))
        .expect("failed to draw vertical baseline");
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(min_x, 0.0), (max_x, 0.0)],
            RGBColor(150, 143, 132).stroke_width(2),
        )))
        .expect("failed to draw horizontal baseline");

    let row_lookup = rows
        .iter()
        .map(|row| ((row.anchor_survivor.clone(), row.role.clone()), row))
        .collect::<BTreeMap<_, _>>();
    for comparison in comparisons {
        let survivor = row_lookup
            .get(&(
                comparison.anchor_survivor.clone(),
                comparison.survivor_role.clone(),
            ))
            .expect("survivor should be in signature rows");
        let control = row_lookup
            .get(&(
                comparison.anchor_survivor.clone(),
                "nearest_dead_control".to_string(),
            ))
            .expect("control should be in signature rows");
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![
                    (control.admissible_delta_m2_pp, control.net_relief_pp),
                    (survivor.admissible_delta_m2_pp, survivor.net_relief_pp),
                ],
                RGBColor(140, 134, 124).stroke_width(2),
            )))
            .expect("failed to draw control-to-survivor segment");
    }

    for row in rows.iter().filter(|row| row.role == "nearest_dead_control") {
        chart
            .draw_series(std::iter::once(Circle::new(
                (row.admissible_delta_m2_pp, row.net_relief_pp),
                5,
                ShapeStyle::from(&RGBColor(122, 126, 136)).filled(),
            )))
            .expect("failed to draw control point");
    }

    for row in rows.iter().filter(|row| row.role != "nearest_dead_control") {
        let radius = (5.0 + (row.anomaly_m2_pp / max_anomaly) * 13.0).round() as i32;
        let color = role_color(&row.role);
        chart
            .draw_series(std::iter::once(Circle::new(
                (row.admissible_delta_m2_pp, row.net_relief_pp),
                radius,
                ShapeStyle::from(&color).filled(),
            )))
            .expect("failed to draw survivor point");
        chart
            .draw_series(std::iter::once(Text::new(
                row.pair_label.clone(),
                (row.admissible_delta_m2_pp + 0.08, row.net_relief_pp + 0.08),
                ("sans-serif", 15).into_font().color(&RGBColor(68, 62, 56)),
            )))
            .expect("failed to draw survivor label");
    }

    root.present().expect("failed to present signature plane");
}

fn render_signature_heatmap(rows: &[SignatureRow], path: &Path) {
    let metric_specs = metric_specs();
    let row_labels = rows
        .iter()
        .map(|row| row.row_label.clone())
        .collect::<Vec<_>>();
    let max_y = rows.len() as i32;
    let root = BitMapBackend::new(path, (1260, (260 + rows.len() as u32 * 34).max(620)))
        .into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill signature heatmap canvas");

    let x_labels = metric_specs
        .iter()
        .map(|spec| spec.display_name.to_string())
        .collect::<Vec<_>>();
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "M=2 Obstruction Signature Heatmap  (rows grouped by control -> survivor)",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(70)
        .y_label_area_size(200)
        .build_cartesian_2d(0i32..metric_specs.len() as i32, 0i32..max_y)
        .expect("failed to build signature heatmap");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(metric_specs.len())
        .y_labels(rows.len())
        .x_desc("signature metrics")
        .y_desc("control / survivor rows")
        .x_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < x_labels.len() {
                x_labels[*value as usize].clone()
            } else {
                String::new()
            }
        })
        .y_label_formatter(&move |value| {
            if *value >= 0 && *value < max_y {
                let row_index = (max_y - 1 - *value) as usize;
                row_labels.get(row_index).cloned().unwrap_or_default()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 15))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw signature heatmap mesh");

    for (x_index, spec) in metric_specs.iter().enumerate() {
        let max_abs = rows
            .iter()
            .map(|row| (spec.extractor)(row).abs())
            .fold(0.0_f64, f64::max)
            .max(1.0);
        for (row_index, row) in rows.iter().enumerate() {
            let y = max_y - 1 - row_index as i32;
            let raw = (spec.extractor)(row);
            let color = if spec.signed {
                diverging_color(raw / max_abs)
            } else {
                sequential_color((raw / max_abs).clamp(0.0, 1.0))
            };
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(x_index as i32, y), (x_index as i32 + 1, y + 1)],
                    ShapeStyle::from(&color).filled(),
                )))
                .expect("failed to draw heatmap cell");
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(x_index as i32, y), (x_index as i32 + 1, y + 1)],
                    ShapeStyle::from(&RGBColor(198, 191, 182)).stroke_width(1),
                )))
                .expect("failed to draw heatmap outline");
        }
    }

    for row_index in 1..rows.len() {
        if rows[row_index - 1].anchor_survivor != rows[row_index].anchor_survivor {
            let y = max_y - row_index as i32;
            chart
                .draw_series(std::iter::once(PathElement::new(
                    vec![(0, y), (metric_specs.len() as i32, y)],
                    RGBColor(138, 132, 122).stroke_width(2),
                )))
                .expect("failed to draw anchor separator");
        }
    }

    root.present().expect("failed to present signature heatmap");
}

#[derive(Clone, Copy)]
struct MetricSpec {
    display_name: &'static str,
    signed: bool,
    extractor: fn(&SignatureRow) -> f64,
}

fn metric_specs() -> Vec<MetricSpec> {
    vec![
        MetricSpec {
            display_name: "anom",
            signed: false,
            extractor: |row| row.anomaly_m2_pp,
        },
        MetricSpec {
            display_name: "adm",
            signed: true,
            extractor: |row| row.admissible_delta_m2_pp,
        },
        MetricSpec {
            display_name: "pos+",
            signed: false,
            extractor: |row| row.total_positive_relief_pp,
        },
        MetricSpec {
            display_name: "neg-",
            signed: false,
            extractor: |row| row.total_negative_relief_pp,
        },
        MetricSpec {
            display_name: "net",
            signed: true,
            extractor: |row| row.net_relief_pp,
        },
        MetricSpec {
            display_name: "abs",
            signed: false,
            extractor: |row| row.total_abs_relief_pp,
        },
        MetricSpec {
            display_name: "n+",
            signed: false,
            extractor: |row| row.positive_relief_count as f64,
        },
        MetricSpec {
            display_name: "conc",
            signed: false,
            extractor: |row| row.positive_relief_concentration,
        },
    ]
}

fn role_sort_rank(role: &str) -> usize {
    match role {
        "persistent_survivor" => 0,
        "emergent_survivor" => 1,
        "nearest_dead_control" => 2,
        _ => 3,
    }
}

fn row_group_rank(role: &str) -> usize {
    match role {
        "nearest_dead_control" => 0,
        "persistent_survivor" => 1,
        "emergent_survivor" => 2,
        _ => 3,
    }
}

fn short_role_label(boundary_class: &str) -> &'static str {
    match boundary_class {
        "m1_to_m2" => "persist",
        "m2_only" => "emerge",
        _ => "row",
    }
}

fn role_color(role: &str) -> RGBColor {
    match role {
        "persistent_survivor" => RGBColor(210, 99, 34),
        "emergent_survivor" => RGBColor(23, 133, 123),
        _ => RGBColor(120, 124, 135),
    }
}

fn sequential_color(t: f64) -> RGBColor {
    let start = (244.0, 239.0, 230.0);
    let end = (39.0, 120.0, 112.0);
    let t = t.clamp(0.0, 1.0);
    RGBColor(
        lerp(start.0, end.0, t) as u8,
        lerp(start.1, end.1, t) as u8,
        lerp(start.2, end.2, t) as u8,
    )
}

fn diverging_color(t: f64) -> RGBColor {
    let t = t.clamp(-1.0, 1.0);
    if t >= 0.0 {
        let start = (245.0, 241.0, 235.0);
        let end = (39.0, 120.0, 112.0);
        RGBColor(
            lerp(start.0, end.0, t) as u8,
            lerp(start.1, end.1, t) as u8,
            lerp(start.2, end.2, t) as u8,
        )
    } else {
        let t = -t;
        let start = (245.0, 241.0, 235.0);
        let end = (202.0, 105.0, 52.0);
        RGBColor(
            lerp(start.0, end.0, t) as u8,
            lerp(start.1, end.1, t) as u8,
            lerp(start.2, end.2, t) as u8,
        )
    }
}

fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

fn derive_observations(
    rows: &[SignatureRow],
    class_summary_rows: &[ClassSummaryRow],
    correlation_rows: &[CorrelationRow],
) -> Vec<String> {
    let survivors = rows
        .iter()
        .filter(|row| row.role != "nearest_dead_control")
        .collect::<Vec<_>>();
    let persistent_summary = class_summary_rows
        .iter()
        .find(|row| row.role == "persistent_survivor")
        .expect("persistent summary should exist");
    let emergent_summary = class_summary_rows
        .iter()
        .find(|row| row.role == "emergent_survivor")
        .expect("emergent summary should exist");
    let strongest_corr = correlation_rows
        .iter()
        .max_by(|left, right| {
            left.pearson_r_with_anomaly_m2
                .abs()
                .total_cmp(&right.pearson_r_with_anomaly_m2.abs())
        })
        .expect("correlation rows should exist");
    let zero_signature_survivors = survivors
        .iter()
        .filter(|row| row.zero_positive_signature)
        .map(|row| row.pair_label.clone())
        .collect::<Vec<_>>();
    let strongest_outlier = survivors
        .iter()
        .filter(|row| row.zero_positive_signature)
        .max_by(|left, right| left.anomaly_m2_pp.total_cmp(&right.anomaly_m2_pp))
        .expect("there should be at least one zero-signature survivor");

    vec![
        format!(
            "On the survivor set, the strongest simple predictor here is `{}` with Pearson `r = {:.2}` against `M=2` anomaly, stronger than the raw positive-relief mass.",
            strongest_corr.metric_name, strongest_corr.pearson_r_with_anomaly_m2
        ),
        format!(
            "Emergent survivors lean more positive in the signature plane than persistent ones: both classes have median admissible lift `{:.2}pp`, but emergent survivors have median net relief `{:.2}pp` versus `{:.2}pp` for persistent.",
            emergent_summary.median_admissible_delta_m2_pp,
            emergent_summary.median_net_relief_pp,
            persistent_summary.median_net_relief_pp
        ),
        format!(
            "Four survivors still have positive `M=2` anomaly with zero positive-relief signature: `{}`. The strongest of them is `{}` at `{:.2}pp`, which is the clearest sign that modulus relief alone is not the whole mechanism.",
            zero_signature_survivors.join(", "),
            strongest_outlier.pair_label,
            strongest_outlier.anomaly_m2_pp
        ),
    ]
}

fn render_markdown_report(bundle: &OutputBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# M=2 Obstruction Signature Report\n\n");
    markdown.push_str("_Generated from `examples/m2_obstruction_signature_report.rs`._\n\n");
    markdown.push_str(&format!("- Generated at: `{}`\n", bundle.generated_at_utc));
    markdown.push_str(&format!("- Input JSON: `{}`\n", bundle.input_json));
    markdown.push_str(&format!(
        "- Input generated at: `{}`\n",
        bundle.input_generated_at_utc
    ));
    markdown.push_str(&format!(
        "- Pair catalog mode: `{}`\n\n",
        bundle.pair_catalog_mode
    ));

    markdown.push_str("## What To Notice\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {observation}\n"));
    }
    markdown.push('\n');

    markdown.push_str("## Image Artifacts\n\n");
    markdown.push_str("| Kind | Label | Path |\n");
    markdown.push_str("|---|---|---|\n");
    for row in &bundle.image_artifact_rows {
        markdown.push_str(&format!(
            "| `{}` | {} | `{}` |\n",
            row.kind, row.label, row.path
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Summary\n\n");
    markdown.push_str(&format!(
        "- Survivors: `{}` (`{}` persistent, `{}` emergent)\n",
        bundle.report_summary.survivor_rows,
        bundle.report_summary.persistent_survivors,
        bundle.report_summary.emergent_survivors
    ));
    markdown.push_str(&format!(
        "- Rank-1 control rows: `{}`\n",
        bundle.report_summary.control_rows
    ));
    markdown.push_str(&format!(
        "- Survivors with zero positive-relief signature: `{}`\n",
        bundle.report_summary.zero_positive_signature_survivors
    ));
    markdown.push_str(&format!(
        "- Strongest anomaly pair: `{}` at `{:.2}pp`\n\n",
        bundle.report_summary.strongest_anomaly_pair, bundle.report_summary.strongest_anomaly_pp
    ));

    markdown.push_str("## Class Summaries\n\n");
    markdown.push_str("| Role | Count | Median anomaly | Median admissible | Median pos+ | Median net | Zero-signature rows |\n");
    markdown.push_str("|---|---:|---:|---:|---:|---:|---:|\n");
    for row in &bundle.class_summary_rows {
        markdown.push_str(&format!(
            "| `{}` | `{}` | `{:.2}pp` | `{:.2}pp` | `{:.2}pp` | `{:.2}pp` | `{}` |\n",
            row.role,
            row.row_count,
            row.median_anomaly_m2_pp,
            row.median_admissible_delta_m2_pp,
            row.median_total_positive_relief_pp,
            row.median_net_relief_pp,
            row.zero_positive_signature_rows
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Correlations\n\n");
    markdown.push_str("| Metric | Pearson r with M=2 anomaly |\n");
    markdown.push_str("|---|---:|\n");
    for row in &bundle.correlation_rows {
        markdown.push_str(&format!(
            "| `{}` | `{:.2}` |\n",
            row.metric_name, row.pearson_r_with_anomaly_m2
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Leading Signature Rows\n\n");
    markdown.push_str("| Row | adm | pos+ | neg- | net | abs | anomaly |\n");
    markdown.push_str("|---|---:|---:|---:|---:|---:|---:|\n");
    for row in bundle
        .signature_rows
        .iter()
        .filter(|row| row.role != "nearest_dead_control")
        .take(12)
    {
        markdown.push_str(&format!(
            "| `{}` | `{:+.2}pp` | `{:.2}pp` | `{:.2}pp` | `{:+.2}pp` | `{:.2}pp` | `{:.2}pp` |\n",
            row.row_label,
            row.admissible_delta_m2_pp,
            row.total_positive_relief_pp,
            row.total_negative_relief_pp,
            row.net_relief_pp,
            row.total_abs_relief_pp,
            row.anomaly_m2_pp
        ));
    }

    markdown
}

fn print_summary(bundle: &OutputBundle) {
    println!("=== M=2 Obstruction Signature Report ===");
    println!();
    println!(
        "Input {} | output {}",
        bundle.input_json, bundle.pair_catalog_mode
    );
    println!(
        "Rows: {} survivors + {} controls | strongest anomaly {} ({:.2}pp)",
        bundle.report_summary.survivor_rows,
        bundle.report_summary.control_rows,
        bundle.report_summary.strongest_anomaly_pair,
        bundle.report_summary.strongest_anomaly_pp
    );
    for row in &bundle.correlation_rows {
        println!(
            "  - corr({}, anomaly) = {:+.2}",
            row.metric_name, row.pearson_r_with_anomaly_m2
        );
    }
    let zero_signature_rows = bundle
        .signature_rows
        .iter()
        .filter(|row| row.role != "nearest_dead_control" && row.zero_positive_signature)
        .collect::<Vec<_>>();
    println!(
        "Zero-positive-relief survivors: {}",
        zero_signature_rows
            .iter()
            .map(|row| row.pair_label.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
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

fn pearson_correlation(xs: Vec<f64>, ys: Vec<f64>) -> f64 {
    if xs.len() != ys.len() || xs.is_empty() {
        return 0.0;
    }
    let mean_x = xs.iter().sum::<f64>() / xs.len() as f64;
    let mean_y = ys.iter().sum::<f64>() / ys.len() as f64;
    let numerator = xs
        .iter()
        .zip(&ys)
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum::<f64>();
    let denominator_x = xs.iter().map(|x| (x - mean_x).powi(2)).sum::<f64>();
    let denominator_y = ys.iter().map(|y| (y - mean_y).powi(2)).sum::<f64>();
    let denominator = (denominator_x * denominator_y).sqrt();
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}
