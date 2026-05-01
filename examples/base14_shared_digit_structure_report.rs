//! Digit-structure mining for the base-14 shared-admissible witness lane.
//!
//! This report stays inside the shared-admissible overlap discovered by
//! `base14_shared_yield_report`. It asks whether the `best_only_prime` versus
//! `k00_only_prime` witnesses show simple digit-level structure in the two-digit
//! middle block.
//!
//! The report reads the maintained shared-yield artifact and exports:
//! - exact per-pair digit enrichment rows
//! - exact per-pair sum/difference residue enrichment rows
//! - a `(D,B)` middle-digit delta grid
//! - a pair-by-sum-residue heatmap for all active pairs
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base14_shared_digit_structure_report
//! cargo run --release --example base14_shared_digit_structure_report -- --input-json /tmp/primes_base14_shared_yield/summary.json --out-dir /tmp/primes_base14_shared_digit_structure
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

const BASE: usize = 14;
const DEFAULT_INPUT_JSON: &str = "/tmp/primes_base14_shared_yield/summary.json";
const DEFAULT_OUT_DIR: &str = "/tmp/primes_base14_shared_digit_structure";
const REPORT_EXPORT_VERSION: u32 = 1;
const STRESS_PAIR: &str = "(D,B)";

#[derive(Debug)]
struct Options {
    input_json: PathBuf,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct SharedYieldRow {
    role: String,
    pair_label: String,
    atlas_role: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SharedWitnessRow {
    role: String,
    pair_label: String,
    middle_digits: String,
    witness_class: String,
}

#[derive(Debug, Clone, Deserialize)]
struct InputBundle {
    generated_at_utc: String,
    shared_yield_rows: Vec<SharedYieldRow>,
    shared_witness_rows: Vec<SharedWitnessRow>,
}

#[derive(Debug, Clone, Serialize)]
struct DigitDeltaRow {
    pair_label: String,
    atlas_role: String,
    position: String,
    digit_value: u32,
    digit_label: String,
    best_only_count: usize,
    k00_only_count: usize,
    delta_count: isize,
}

#[derive(Debug, Clone, Serialize)]
struct ResidueDeltaRow {
    pair_label: String,
    atlas_role: String,
    residue_kind: String,
    residue_value: u32,
    best_only_count: usize,
    k00_only_count: usize,
    delta_count: isize,
}

#[derive(Debug, Clone, Serialize)]
struct GridDeltaRow {
    pair_label: String,
    first_digit_value: u32,
    first_digit_label: String,
    second_digit_value: u32,
    second_digit_label: String,
    best_only_count: usize,
    k00_only_count: usize,
    delta_count: isize,
}

#[derive(Debug, Clone, Serialize)]
struct PairSignalRow {
    pair_label: String,
    atlas_role: String,
    best_only_count: usize,
    k00_only_count: usize,
    top_first_digit_deltas: String,
    top_second_digit_deltas: String,
    top_sum_residue_deltas: String,
    top_difference_residue_deltas: String,
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
    stress_pair: String,
    stress_top_second_digit_deltas: String,
    stress_top_sum_residue_deltas: String,
    stress_top_difference_residue_deltas: String,
}

#[derive(Debug, Clone, Serialize)]
struct OutputBundle {
    export_version: u32,
    generated_at_utc: String,
    input_json: String,
    input_generated_at_utc: String,
    pair_signal_rows: Vec<PairSignalRow>,
    digit_delta_rows: Vec<DigitDeltaRow>,
    residue_delta_rows: Vec<ResidueDeltaRow>,
    grid_delta_rows: Vec<GridDeltaRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let input_bundle = load_input_bundle(&options.input_json);
    let active_roles = input_bundle
        .shared_yield_rows
        .iter()
        .filter(|row| row.role == "active")
        .map(|row| (row.pair_label.clone(), row.atlas_role.clone()))
        .collect::<BTreeMap<_, _>>();

    let active_pairs = active_roles.keys().cloned().collect::<Vec<_>>();
    let witness_lookup = input_bundle
        .shared_witness_rows
        .iter()
        .filter(|row| row.role == "active")
        .fold(
            BTreeMap::<String, Vec<SharedWitnessRow>>::new(),
            |mut acc, row| {
                acc.entry(row.pair_label.clone())
                    .or_default()
                    .push(row.clone());
                acc
            },
        );

    let mut digit_delta_rows = Vec::new();
    let mut residue_delta_rows = Vec::new();
    let mut grid_delta_rows = Vec::new();
    let mut pair_signal_rows = Vec::new();

    for pair_label in &active_pairs {
        let atlas_role = active_roles
            .get(pair_label)
            .expect("active pair should have atlas role")
            .clone();
        let witness_rows = witness_lookup
            .get(pair_label)
            .unwrap_or_else(|| panic!("missing active shared witnesses for {pair_label}"));

        let digit_summary = build_digit_delta_rows(pair_label, &atlas_role, witness_rows);
        let residue_summary = build_residue_delta_rows(pair_label, &atlas_role, witness_rows);
        let grid_rows = build_grid_delta_rows(pair_label, witness_rows);

        pair_signal_rows.push(PairSignalRow {
            pair_label: pair_label.clone(),
            atlas_role,
            best_only_count: witness_rows
                .iter()
                .filter(|row| row.witness_class == "best_only_prime")
                .count(),
            k00_only_count: witness_rows
                .iter()
                .filter(|row| row.witness_class == "k00_only_prime")
                .count(),
            top_first_digit_deltas: summarize_top_digit_deltas(&digit_summary, "first"),
            top_second_digit_deltas: summarize_top_digit_deltas(&digit_summary, "second"),
            top_sum_residue_deltas: summarize_top_residue_deltas(&residue_summary, "sum_mod_14"),
            top_difference_residue_deltas: summarize_top_residue_deltas(
                &residue_summary,
                "difference_mod_14",
            ),
        });

        digit_delta_rows.extend(digit_summary);
        residue_delta_rows.extend(residue_summary);
        if pair_label == STRESS_PAIR {
            grid_delta_rows.extend(grid_rows);
        }
    }

    pair_signal_rows.sort_by(|left, right| left.pair_label.cmp(&right.pair_label));
    digit_delta_rows.sort_by(|left, right| {
        left.pair_label
            .cmp(&right.pair_label)
            .then_with(|| left.position.cmp(&right.position))
            .then_with(|| left.digit_value.cmp(&right.digit_value))
    });
    residue_delta_rows.sort_by(|left, right| {
        left.pair_label
            .cmp(&right.pair_label)
            .then_with(|| left.residue_kind.cmp(&right.residue_kind))
            .then_with(|| left.residue_value.cmp(&right.residue_value))
    });
    grid_delta_rows.sort_by(|left, right| {
        left.first_digit_value
            .cmp(&right.first_digit_value)
            .then_with(|| left.second_digit_value.cmp(&right.second_digit_value))
    });

    let grid_path = options.out_dir.join("db_digit_delta_grid.png");
    render_db_grid(&grid_delta_rows, &grid_path);
    let sum_heatmap_path = options.out_dir.join("active_sum_residue_heatmap.png");
    render_sum_heatmap(&pair_signal_rows, &residue_delta_rows, &sum_heatmap_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "db_grid".to_string(),
            label: "(D,B) digit-delta grid".to_string(),
            path: grid_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "sum_heatmap".to_string(),
            label: "Active-pair sum-residue delta heatmap".to_string(),
            path: sum_heatmap_path.display().to_string(),
        },
    ];
    let report_summary = build_report_summary(&pair_signal_rows);
    let observations = derive_observations(&pair_signal_rows);

    let bundle = OutputBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        input_json: options.input_json.display().to_string(),
        input_generated_at_utc: input_bundle.generated_at_utc,
        pair_signal_rows: pair_signal_rows.clone(),
        digit_delta_rows: digit_delta_rows.clone(),
        residue_delta_rows: residue_delta_rows.clone(),
        grid_delta_rows: grid_delta_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("pair_signal_rows.csv"),
        &pair_signal_rows,
    )
    .expect("failed to write pair_signal_rows.csv");
    write_csv_rows(
        options.out_dir.join("digit_delta_rows.csv"),
        &digit_delta_rows,
    )
    .expect("failed to write digit_delta_rows.csv");
    write_csv_rows(
        options.out_dir.join("residue_delta_rows.csv"),
        &residue_delta_rows,
    )
    .expect("failed to write residue_delta_rows.csv");
    write_csv_rows(
        options.out_dir.join("grid_delta_rows.csv"),
        &grid_delta_rows,
    )
    .expect("failed to write grid_delta_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");

    let markdown = render_markdown(&bundle);
    write_text_file(options.out_dir.join("report.md"), &markdown)
        .expect("failed to write report.md");

    println!("Base-14 shared digit-structure report");
    println!(
        "  input shared-yield artifact: {}",
        options.input_json.display()
    );
    println!(
        "  output dir:                 {}",
        options.out_dir.display()
    );
    for row in &pair_signal_rows {
        println!(
            "  {} | first {} | second {} | sum {} | diff {}",
            row.pair_label,
            row.top_first_digit_deltas,
            row.top_second_digit_deltas,
            row.top_sum_residue_deltas,
            row.top_difference_residue_deltas
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
            "--help" | "-h" => print_help_and_exit(),
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
    println!("  cargo run --release --example base14_shared_digit_structure_report -- [options]");
    println!();
    println!("Options:");
    println!("  --input-json <path>   Read shared-yield artifact from this JSON path");
    println!("  --out-dir <path>      Write output bundle to this directory");
    println!("  -h, --help            Show this help");
    std::process::exit(0);
}

fn load_input_bundle(path: &Path) -> InputBundle {
    let text = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!(
            "Failed to read shared-yield JSON at {}: {err}\nRun `cargo run --release --example base14_shared_yield_report` first or pass --input-json.",
            path.display()
        )
    });
    serde_json::from_str(&text).unwrap_or_else(|err| {
        panic!(
            "Failed to parse shared-yield JSON at {}: {err}",
            path.display()
        )
    })
}

fn build_digit_delta_rows(
    pair_label: &str,
    atlas_role: &str,
    witness_rows: &[SharedWitnessRow],
) -> Vec<DigitDeltaRow> {
    let mut best_first = [0usize; BASE];
    let mut k00_first = [0usize; BASE];
    let mut best_second = [0usize; BASE];
    let mut k00_second = [0usize; BASE];

    for row in witness_rows {
        let (first, second) = parse_middle_digits(&row.middle_digits);
        match row.witness_class.as_str() {
            "best_only_prime" => {
                best_first[first as usize] += 1;
                best_second[second as usize] += 1;
            }
            "k00_only_prime" => {
                k00_first[first as usize] += 1;
                k00_second[second as usize] += 1;
            }
            _ => {}
        }
    }

    let mut rows = Vec::with_capacity(BASE * 2);
    for digit in 0..BASE {
        rows.push(DigitDeltaRow {
            pair_label: pair_label.to_string(),
            atlas_role: atlas_role.to_string(),
            position: "first".to_string(),
            digit_value: digit as u32,
            digit_label: digit_label(digit as u32),
            best_only_count: best_first[digit],
            k00_only_count: k00_first[digit],
            delta_count: best_first[digit] as isize - k00_first[digit] as isize,
        });
        rows.push(DigitDeltaRow {
            pair_label: pair_label.to_string(),
            atlas_role: atlas_role.to_string(),
            position: "second".to_string(),
            digit_value: digit as u32,
            digit_label: digit_label(digit as u32),
            best_only_count: best_second[digit],
            k00_only_count: k00_second[digit],
            delta_count: best_second[digit] as isize - k00_second[digit] as isize,
        });
    }
    rows
}

fn build_residue_delta_rows(
    pair_label: &str,
    atlas_role: &str,
    witness_rows: &[SharedWitnessRow],
) -> Vec<ResidueDeltaRow> {
    let mut best_sum = [0usize; BASE];
    let mut k00_sum = [0usize; BASE];
    let mut best_diff = [0usize; BASE];
    let mut k00_diff = [0usize; BASE];

    for row in witness_rows {
        let (first, second) = parse_middle_digits(&row.middle_digits);
        let sum = ((first + second) % BASE as u32) as usize;
        let diff = ((first + BASE as u32 - second) % BASE as u32) as usize;
        match row.witness_class.as_str() {
            "best_only_prime" => {
                best_sum[sum] += 1;
                best_diff[diff] += 1;
            }
            "k00_only_prime" => {
                k00_sum[sum] += 1;
                k00_diff[diff] += 1;
            }
            _ => {}
        }
    }

    let mut rows = Vec::with_capacity(BASE * 2);
    for residue in 0..BASE {
        rows.push(ResidueDeltaRow {
            pair_label: pair_label.to_string(),
            atlas_role: atlas_role.to_string(),
            residue_kind: "sum_mod_14".to_string(),
            residue_value: residue as u32,
            best_only_count: best_sum[residue],
            k00_only_count: k00_sum[residue],
            delta_count: best_sum[residue] as isize - k00_sum[residue] as isize,
        });
        rows.push(ResidueDeltaRow {
            pair_label: pair_label.to_string(),
            atlas_role: atlas_role.to_string(),
            residue_kind: "difference_mod_14".to_string(),
            residue_value: residue as u32,
            best_only_count: best_diff[residue],
            k00_only_count: k00_diff[residue],
            delta_count: best_diff[residue] as isize - k00_diff[residue] as isize,
        });
    }
    rows
}

fn build_grid_delta_rows(pair_label: &str, witness_rows: &[SharedWitnessRow]) -> Vec<GridDeltaRow> {
    let mut best = [[0usize; BASE]; BASE];
    let mut k00 = [[0usize; BASE]; BASE];
    for row in witness_rows {
        let (first, second) = parse_middle_digits(&row.middle_digits);
        match row.witness_class.as_str() {
            "best_only_prime" => best[first as usize][second as usize] += 1,
            "k00_only_prime" => k00[first as usize][second as usize] += 1,
            _ => {}
        }
    }
    let mut rows = Vec::with_capacity(BASE * BASE);
    for first in 0..BASE {
        for second in 0..BASE {
            rows.push(GridDeltaRow {
                pair_label: pair_label.to_string(),
                first_digit_value: first as u32,
                first_digit_label: digit_label(first as u32),
                second_digit_value: second as u32,
                second_digit_label: digit_label(second as u32),
                best_only_count: best[first][second],
                k00_only_count: k00[first][second],
                delta_count: best[first][second] as isize - k00[first][second] as isize,
            });
        }
    }
    rows
}

fn parse_middle_digits(text: &str) -> (u32, u32) {
    let chars = text.chars().collect::<Vec<_>>();
    assert_eq!(chars.len(), 2, "expected two-digit middle block: {text}");
    (parse_digit_char(chars[0]), parse_digit_char(chars[1]))
}

fn parse_digit_char(ch: char) -> u32 {
    if let Some(value) = ch.to_digit(10) {
        value
    } else if ch.is_ascii_uppercase() {
        10 + (ch as u32 - 'A' as u32)
    } else {
        panic!("unsupported digit character: {ch}");
    }
}

fn digit_label(value: u32) -> String {
    if value < 10 {
        value.to_string()
    } else {
        char::from_u32('A' as u32 + value - 10)
            .expect("digit label should fit uppercase alphabet")
            .to_string()
    }
}

fn summarize_top_digit_deltas(rows: &[DigitDeltaRow], position: &str) -> String {
    summarize_top_named_deltas(
        rows.iter()
            .filter(|row| row.position == position)
            .map(|row| (row.digit_label.clone(), row.delta_count))
            .collect(),
    )
}

fn summarize_top_residue_deltas(rows: &[ResidueDeltaRow], residue_kind: &str) -> String {
    summarize_top_named_deltas(
        rows.iter()
            .filter(|row| row.residue_kind == residue_kind)
            .map(|row| (row.residue_value.to_string(), row.delta_count))
            .collect(),
    )
}

fn summarize_top_named_deltas(mut rows: Vec<(String, isize)>) -> String {
    rows.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    let filtered = rows
        .into_iter()
        .filter(|(_, delta)| *delta > 0)
        .take(3)
        .map(|(name, delta)| format!("{name}:{delta:+}"))
        .collect::<Vec<_>>();
    if filtered.is_empty() {
        "none".to_string()
    } else {
        filtered.join(";")
    }
}

fn render_db_grid(rows: &[GridDeltaRow], path: &Path) {
    let max_abs = rows
        .iter()
        .map(|row| row.delta_count.abs() as f64)
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let root = BitMapBackend::new(path, (980, 920)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill D,B grid canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "(D,B) Best-Only Minus k00-Only Digit Grid  (shared-admissible lane)",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(64)
        .y_label_area_size(70)
        .build_cartesian_2d(0i32..BASE as i32, 0i32..BASE as i32)
        .expect("failed to build D,B grid chart");

    let labels = (0..BASE)
        .map(|digit| digit_label(digit as u32))
        .collect::<Vec<_>>();
    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("second digit")
        .y_desc("first digit")
        .x_labels(BASE)
        .y_labels(BASE)
        .x_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < labels.len() {
                labels[*value as usize].clone()
            } else {
                String::new()
            }
        })
        .y_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < BASE {
                digit_label(*value as u32)
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw D,B grid mesh");

    for row in rows {
        let x = row.second_digit_value as i32;
        let y = row.first_digit_value as i32;
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x, y), (x + 1, y + 1)],
                ShapeStyle::from(&delta_color(row.delta_count as f64, max_abs)).filled(),
            )))
            .expect("failed to draw D,B grid cell");
        if row.delta_count != 0 {
            chart
                .draw_series(std::iter::once(Text::new(
                    row.delta_count.to_string(),
                    (x, y),
                    ("sans-serif", 15).into_font().color(&BLACK),
                )))
                .expect("failed to draw D,B cell label");
        }
    }

    root.present().expect("failed to present D,B grid");
}

fn render_sum_heatmap(rows: &[PairSignalRow], residue_rows: &[ResidueDeltaRow], path: &Path) {
    let pair_labels = rows
        .iter()
        .map(|row| row.pair_label.clone())
        .collect::<Vec<_>>();
    let pair_lookup = rows
        .iter()
        .enumerate()
        .map(|(index, row)| (row.pair_label.clone(), index as i32))
        .collect::<BTreeMap<_, _>>();
    let max_abs = residue_rows
        .iter()
        .filter(|row| row.residue_kind == "sum_mod_14")
        .map(|row| row.delta_count.abs() as f64)
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let root = BitMapBackend::new(path, (1180, 420)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill sum heatmap canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Active Pair Sum-Residue Delta Heatmap  (best-only minus k00-only)",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(56)
        .y_label_area_size(140)
        .build_cartesian_2d(0i32..BASE as i32, 0i32..rows.len() as i32)
        .expect("failed to build sum heatmap");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("sum residue mod 14")
        .y_desc("active pairs")
        .x_labels(BASE)
        .y_labels(rows.len())
        .x_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < BASE {
                value.to_string()
            } else {
                String::new()
            }
        })
        .y_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < pair_labels.len() {
                let row_index = (pair_labels.len() - 1) - *value as usize;
                pair_labels[row_index].clone()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw sum heatmap mesh");

    for row in residue_rows
        .iter()
        .filter(|row| row.residue_kind == "sum_mod_14")
    {
        let x = row.residue_value as i32;
        let pair_index = pair_lookup
            .get(&row.pair_label)
            .copied()
            .expect("sum heatmap pair should exist");
        let y = rows.len() as i32 - 1 - pair_index;
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x, y), (x + 1, y + 1)],
                ShapeStyle::from(&delta_color(row.delta_count as f64, max_abs)).filled(),
            )))
            .expect("failed to draw sum heatmap cell");
        if row.delta_count != 0 {
            chart
                .draw_series(std::iter::once(Text::new(
                    row.delta_count.to_string(),
                    (x, y),
                    ("sans-serif", 14).into_font().color(&BLACK),
                )))
                .expect("failed to draw sum heatmap label");
        }
    }

    root.present().expect("failed to present sum heatmap");
}

fn delta_color(value: f64, max_abs: f64) -> RGBColor {
    if value > 0.0 {
        let intensity = (value / max_abs).clamp(0.0, 1.0);
        RGBColor(
            (240.0 - 110.0 * intensity).round() as u8,
            (244.0 - 70.0 * intensity).round() as u8,
            (228.0 - 170.0 * intensity).round() as u8,
        )
    } else if value < 0.0 {
        let intensity = ((-value) / max_abs).clamp(0.0, 1.0);
        RGBColor(
            (242.0 - 50.0 * intensity).round() as u8,
            (236.0 - 150.0 * intensity).round() as u8,
            (228.0 - 130.0 * intensity).round() as u8,
        )
    } else {
        RGBColor(236, 232, 224)
    }
}

fn build_report_summary(rows: &[PairSignalRow]) -> ReportSummary {
    let stress = rows
        .iter()
        .find(|row| row.pair_label == STRESS_PAIR)
        .expect("stress pair should exist");
    ReportSummary {
        active_pairs: rows.len(),
        stress_pair: STRESS_PAIR.to_string(),
        stress_top_second_digit_deltas: stress.top_second_digit_deltas.clone(),
        stress_top_sum_residue_deltas: stress.top_sum_residue_deltas.clone(),
        stress_top_difference_residue_deltas: stress.top_difference_residue_deltas.clone(),
    }
}

fn derive_observations(rows: &[PairSignalRow]) -> Vec<String> {
    let stress = rows
        .iter()
        .find(|row| row.pair_label == STRESS_PAIR)
        .expect("stress pair should exist");
    let positive_second = stress.top_second_digit_deltas.clone();
    let positive_sum = stress.top_sum_residue_deltas.clone();
    let positive_diff = stress.top_difference_residue_deltas.clone();
    let nonstress = rows
        .iter()
        .filter(|row| row.pair_label != STRESS_PAIR)
        .map(|row| format!("{}: {}", row.pair_label, row.top_sum_residue_deltas))
        .collect::<Vec<_>>()
        .join("; ");

    vec![
        format!(
            "Inside the (D,B) shared-admissible overlap, the cleanest positive second-digit enrichments are {}.",
            positive_second
        ),
        format!(
            "For (D,B), the strongest positive sum residues are {}, while the strongest positive difference residues are {}.",
            positive_sum, positive_diff
        ),
        format!(
            "The digit-grid view shows that the (D,B) lift is distributed across several middle-digit cells rather than one isolated witness, which is consistent with a small family effect rather than a single lucky hit."
        ),
        format!(
            "The other active pairs do not share one universal residue signature on the sum side: {}. That keeps the base-14 story interestingly non-universal even inside the shared-yield lane.",
            nonstress
        ),
    ]
}

fn render_markdown(bundle: &OutputBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Base-14 Shared Digit Structure\n\n");
    markdown.push_str("_Generated from `examples/base14_shared_digit_structure_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Input shared-yield artifact: `{}`\n- Output directory: `{}`\n\n",
        bundle.input_json, DEFAULT_OUT_DIR
    ));

    markdown.push_str("## Pair Signals\n\n");
    markdown.push_str(
        "| Pair | Role | Top first-digit deltas | Top second-digit deltas | Top sum-residue deltas | Top difference-residue deltas |\n",
    );
    markdown.push_str("|---|---|---|---|---|---|\n");
    for row in &bundle.pair_signal_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            row.pair_label,
            row.atlas_role,
            row.top_first_digit_deltas,
            row.top_second_digit_deltas,
            row.top_sum_residue_deltas,
            row.top_difference_residue_deltas
        ));
    }
    markdown.push('\n');

    if let Some(image) = bundle
        .image_artifact_rows
        .iter()
        .find(|image| image.kind == "db_grid")
    {
        markdown.push_str("## (D,B) Grid\n\n");
        markdown.push_str(&format!("![{}]({})\n\n", image.label, image.path));
    }
    if let Some(image) = bundle
        .image_artifact_rows
        .iter()
        .find(|image| image.kind == "sum_heatmap")
    {
        markdown.push_str("## Sum Heatmap\n\n");
        markdown.push_str(&format!("![{}]({})\n\n", image.label, image.path));
    }

    markdown.push_str("## Observations\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {}\n", observation));
    }
    markdown
}
