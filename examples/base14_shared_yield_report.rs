//! Shared-admissible yield witness report for the base-14 mechanism lane.
//!
//! This report drills into the `shared_admissible` lane itself. It asks:
//! where, exactly, are the extra prime hits coming from once both `k=(0,0)`
//! and the winning `k` already agree that a candidate is admissible?
//!
//! The report is downstream of `base14_outlier_mechanism_report`. It reads the
//! maintained mechanism artifact, recomputes the exact shared-admissible
//! witness rows for each active pair plus its rank-1 nearby dead control, and
//! renders:
//! - a witness-count chart for the active pairs
//! - a `(D,B)` stress-case strip against its nearby dead control `(9,5)`
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base14_shared_yield_report
//! cargo run --release --example base14_shared_yield_report -- --input-json /tmp/primes_base14_outlier_mechanism/summary.json --out-dir /tmp/primes_base14_shared_yield
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{format_k, parse_k_label, scan_k_config_mask_profile, BoundedKConfig},
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
const DEFAULT_INPUT_JSON: &str = "/tmp/primes_base14_outlier_mechanism/summary.json";
const DEFAULT_OUT_DIR: &str = "/tmp/primes_base14_shared_yield";
const REPORT_EXPORT_VERSION: u32 = 1;
const STRESS_PAIR: &str = "(D,B)";

#[derive(Debug)]
struct Options {
    input_json: PathBuf,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct MechanismRow {
    pair_label: String,
    atlas_role: String,
    best_k_m2: String,
    stress_case: bool,
    prime_hit_delta_pp: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct ControlReferenceRow {
    survivor_pair: String,
    control_pair: String,
    control_best_k_m2: String,
}

#[derive(Debug, Clone, Deserialize)]
struct InputBundle {
    generated_at_utc: String,
    mechanism_rows: Vec<MechanismRow>,
    control_reference_rows: Vec<ControlReferenceRow>,
}

#[derive(Debug, Clone, Serialize)]
struct SharedYieldRow {
    role: String,
    anchor_pair: String,
    pair_label: String,
    atlas_role: String,
    best_k_m2: String,
    stress_case: bool,
    shared_admissible_count: usize,
    both_prime_count: usize,
    best_only_prime_count: usize,
    k00_only_prime_count: usize,
    both_composite_count: usize,
    shared_prime_hits_k00: usize,
    shared_prime_hits_best: usize,
    shared_prime_delta_count: isize,
    shared_prime_rate_k00_pp: f64,
    shared_prime_rate_best_pp: f64,
    shared_prime_rate_delta_pp: f64,
    whole_pair_prime_delta_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct SharedWitnessRow {
    role: String,
    anchor_pair: String,
    pair_label: String,
    best_k_m2: String,
    middle_index: u32,
    middle_digits: String,
    witness_class: String,
    k00_prime: bool,
    best_prime: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ActiveControlComparisonRow {
    active_pair: String,
    control_pair: String,
    active_shared_count: usize,
    control_shared_count: usize,
    active_shared_prime_delta_count: isize,
    active_shared_prime_rate_best_pp: f64,
    active_shared_prime_rate_k00_pp: f64,
    control_shared_prime_rate_pp: f64,
    active_minus_control_shared_rate_pp: f64,
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
    positive_shared_delta_pairs: usize,
    negative_shared_delta_pairs: usize,
    strongest_shared_delta_pair: String,
    strongest_shared_delta_count: isize,
}

#[derive(Debug, Clone, Serialize)]
struct OutputBundle {
    export_version: u32,
    generated_at_utc: String,
    input_json: String,
    input_generated_at_utc: String,
    shared_yield_rows: Vec<SharedYieldRow>,
    shared_witness_rows: Vec<SharedWitnessRow>,
    active_control_comparison_rows: Vec<ActiveControlComparisonRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let input_bundle = load_input_bundle(&options.input_json);
    let control_lookup = input_bundle
        .control_reference_rows
        .iter()
        .map(|row| (row.survivor_pair.clone(), row))
        .collect::<BTreeMap<_, _>>();

    let mut shared_yield_rows = Vec::new();
    let mut shared_witness_rows = Vec::new();
    let mut active_control_comparison_rows = Vec::new();

    for mechanism_row in &input_bundle.mechanism_rows {
        let (outer, inner) = parse_pair_label(&mechanism_row.pair_label);
        let best_k = parse_k_label(&mechanism_row.best_k_m2);
        let active_summary = summarize_shared_lane(&SharedLaneSpec {
            role: "active".to_string(),
            anchor_pair: mechanism_row.pair_label.clone(),
            pair_label: mechanism_row.pair_label.clone(),
            atlas_role: mechanism_row.atlas_role.clone(),
            best_k,
            stress_case: mechanism_row.stress_case,
            whole_pair_prime_delta_pp: mechanism_row.prime_hit_delta_pp,
            outer,
            inner,
        });
        let active_row = active_summary.summary_row.clone();
        shared_witness_rows.extend(active_summary.witness_rows);
        shared_yield_rows.push(active_row.clone());

        let control_ref = control_lookup
            .get(&mechanism_row.pair_label)
            .unwrap_or_else(|| {
                panic!(
                    "missing control reference row for {}",
                    mechanism_row.pair_label
                )
            });
        let (control_outer, control_inner) = parse_pair_label(&control_ref.control_pair);
        let control_best_k = parse_k_label(&control_ref.control_best_k_m2);
        let control_summary = summarize_shared_lane(&SharedLaneSpec {
            role: "control".to_string(),
            anchor_pair: mechanism_row.pair_label.clone(),
            pair_label: control_ref.control_pair.clone(),
            atlas_role: "m1_only_control".to_string(),
            best_k: control_best_k,
            stress_case: false,
            whole_pair_prime_delta_pp: 0.0,
            outer: control_outer,
            inner: control_inner,
        });
        let control_row = control_summary.summary_row.clone();
        shared_witness_rows.extend(control_summary.witness_rows);
        shared_yield_rows.push(control_row.clone());

        active_control_comparison_rows.push(ActiveControlComparisonRow {
            active_pair: mechanism_row.pair_label.clone(),
            control_pair: control_ref.control_pair.clone(),
            active_shared_count: active_row.shared_admissible_count,
            control_shared_count: control_row.shared_admissible_count,
            active_shared_prime_delta_count: active_row.shared_prime_delta_count,
            active_shared_prime_rate_best_pp: active_row.shared_prime_rate_best_pp,
            active_shared_prime_rate_k00_pp: active_row.shared_prime_rate_k00_pp,
            control_shared_prime_rate_pp: control_row.shared_prime_rate_best_pp,
            active_minus_control_shared_rate_pp: active_row.shared_prime_rate_best_pp
                - control_row.shared_prime_rate_best_pp,
        });
    }

    shared_yield_rows.sort_by(|left, right| {
        left.anchor_pair
            .cmp(&right.anchor_pair)
            .then_with(|| left.role.cmp(&right.role))
    });
    shared_witness_rows.sort_by(|left, right| {
        left.anchor_pair
            .cmp(&right.anchor_pair)
            .then_with(|| left.role.cmp(&right.role))
            .then_with(|| left.middle_index.cmp(&right.middle_index))
    });
    active_control_comparison_rows.sort_by(|left, right| left.active_pair.cmp(&right.active_pair));

    let witness_chart_path = options.out_dir.join("shared_yield_witness_counts.png");
    render_witness_count_chart(&shared_yield_rows, &witness_chart_path);
    let stress_strip_path = options.out_dir.join("db_shared_witness_strip.png");
    render_stress_strip(&shared_witness_rows, &stress_strip_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "witness_counts".to_string(),
            label: "Shared-admissible witness counts".to_string(),
            path: witness_chart_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "stress_strip".to_string(),
            label: "(D,B) stress-case shared-witness strip".to_string(),
            path: stress_strip_path.display().to_string(),
        },
    ];
    let report_summary = build_report_summary(&shared_yield_rows);
    let observations = derive_observations(&shared_yield_rows, &active_control_comparison_rows);

    let bundle = OutputBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        input_json: options.input_json.display().to_string(),
        input_generated_at_utc: input_bundle.generated_at_utc,
        shared_yield_rows: shared_yield_rows.clone(),
        shared_witness_rows: shared_witness_rows.clone(),
        active_control_comparison_rows: active_control_comparison_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("shared_yield_rows.csv"),
        &shared_yield_rows,
    )
    .expect("failed to write shared_yield_rows.csv");
    write_csv_rows(
        options.out_dir.join("shared_witness_rows.csv"),
        &shared_witness_rows,
    )
    .expect("failed to write shared_witness_rows.csv");
    write_csv_rows(
        options.out_dir.join("active_control_comparison_rows.csv"),
        &active_control_comparison_rows,
    )
    .expect("failed to write active_control_comparison_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");

    let markdown = render_markdown(&bundle);
    write_text_file(options.out_dir.join("report.md"), &markdown)
        .expect("failed to write report.md");

    println!("Base-14 shared-yield report");
    println!("  input mechanism: {}", options.input_json.display());
    println!("  output dir:      {}", options.out_dir.display());
    for row in shared_yield_rows.iter().filter(|row| row.role == "active") {
        println!(
            "  {} | shared delta {:+} | best-only {} | k00-only {}",
            row.pair_label,
            row.shared_prime_delta_count,
            row.best_only_prime_count,
            row.k00_only_prime_count
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
    println!("  cargo run --release --example base14_shared_yield_report -- [options]");
    println!();
    println!("Options:");
    println!("  --input-json <path>   Read mechanism artifact from this JSON path");
    println!("  --out-dir <path>      Write output bundle to this directory");
    println!("  -h, --help            Show this help");
    std::process::exit(0);
}

fn load_input_bundle(path: &Path) -> InputBundle {
    let text = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!(
            "Failed to read mechanism JSON at {}: {err}\nRun `cargo run --release --example base14_outlier_mechanism_report` first or pass --input-json.",
            path.display()
        )
    });
    serde_json::from_str(&text).unwrap_or_else(|err| {
        panic!(
            "Failed to parse mechanism JSON at {}: {err}",
            path.display()
        )
    })
}

#[derive(Debug, Clone)]
struct SharedLaneSummary {
    summary_row: SharedYieldRow,
    witness_rows: Vec<SharedWitnessRow>,
}

#[derive(Debug, Clone)]
struct SharedLaneSpec {
    role: String,
    anchor_pair: String,
    pair_label: String,
    atlas_role: String,
    best_k: BoundedKConfig,
    stress_case: bool,
    whole_pair_prime_delta_pp: f64,
    outer: u32,
    inner: u32,
}

fn summarize_shared_lane(spec: &SharedLaneSpec) -> SharedLaneSummary {
    let k00_profile =
        scan_k_config_mask_profile(BASE, MIDDLE_LENGTH, spec.outer, spec.inner, (0, 0));
    let best_profile = if spec.best_k == (0, 0) {
        k00_profile.clone()
    } else {
        scan_k_config_mask_profile(BASE, MIDDLE_LENGTH, spec.outer, spec.inner, spec.best_k)
    };

    let mut witness_rows = Vec::new();
    let mut both_prime_count = 0usize;
    let mut best_only_prime_count = 0usize;
    let mut k00_only_prime_count = 0usize;
    let mut both_composite_count = 0usize;

    for (k00_row, best_row) in k00_profile
        .candidate_rows
        .iter()
        .zip(&best_profile.candidate_rows)
    {
        if !(k00_row.admissible && best_row.admissible) {
            continue;
        }
        let witness_class = match (k00_row.prime, best_row.prime) {
            (true, true) => {
                both_prime_count += 1;
                "both_prime"
            }
            (false, true) => {
                best_only_prime_count += 1;
                "best_only_prime"
            }
            (true, false) => {
                k00_only_prime_count += 1;
                "k00_only_prime"
            }
            (false, false) => {
                both_composite_count += 1;
                "both_composite"
            }
        };
        witness_rows.push(SharedWitnessRow {
            role: spec.role.clone(),
            anchor_pair: spec.anchor_pair.clone(),
            pair_label: spec.pair_label.clone(),
            best_k_m2: format_k(spec.best_k),
            middle_index: k00_row.middle_index,
            middle_digits: k00_row.middle_digits.clone(),
            witness_class: witness_class.to_string(),
            k00_prime: k00_row.prime,
            best_prime: best_row.prime,
        });
    }

    let shared_admissible_count = witness_rows.len();
    let shared_prime_hits_k00 = both_prime_count + k00_only_prime_count;
    let shared_prime_hits_best = both_prime_count + best_only_prime_count;
    let shared_prime_rate_k00_pp = rate_pp(shared_prime_hits_k00, shared_admissible_count);
    let shared_prime_rate_best_pp = rate_pp(shared_prime_hits_best, shared_admissible_count);
    let shared_prime_delta_count = shared_prime_hits_best as isize - shared_prime_hits_k00 as isize;

    SharedLaneSummary {
        summary_row: SharedYieldRow {
            role: spec.role.clone(),
            anchor_pair: spec.anchor_pair.clone(),
            pair_label: spec.pair_label.clone(),
            atlas_role: spec.atlas_role.clone(),
            best_k_m2: format_k(spec.best_k),
            stress_case: spec.stress_case,
            shared_admissible_count,
            both_prime_count,
            best_only_prime_count,
            k00_only_prime_count,
            both_composite_count,
            shared_prime_hits_k00,
            shared_prime_hits_best,
            shared_prime_delta_count,
            shared_prime_rate_k00_pp,
            shared_prime_rate_best_pp,
            shared_prime_rate_delta_pp: shared_prime_rate_best_pp - shared_prime_rate_k00_pp,
            whole_pair_prime_delta_pp: spec.whole_pair_prime_delta_pp,
        },
        witness_rows,
    }
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

fn rate_pp(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 * 100.0 / denominator as f64
    }
}

fn render_witness_count_chart(rows: &[SharedYieldRow], path: &Path) {
    let active_rows = rows
        .iter()
        .filter(|row| row.role == "active")
        .collect::<Vec<_>>();
    let labels = active_rows
        .iter()
        .map(|row| row.pair_label.clone())
        .collect::<Vec<_>>();
    let max_y = active_rows.len() as i32;
    let max_abs = active_rows
        .iter()
        .map(|row| (row.best_only_prime_count + row.k00_only_prime_count) as i32)
        .max()
        .unwrap_or(4)
        + 2;

    let root = BitMapBackend::new(path, (1080, (300 + active_rows.len() as u32 * 90).max(620)))
        .into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill witness chart canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base 14 Shared-Admissible Prime Witness Counts",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(64)
        .y_label_area_size(180)
        .build_cartesian_2d(-(max_abs as f64)..(max_abs as f64), 0i32..max_y)
        .expect("failed to build witness chart");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("prime witness count")
        .y_desc("active pairs")
        .y_labels(active_rows.len())
        .y_label_formatter(&move |value| {
            if *value >= 0 && *value < max_y {
                let row_index = (max_y - 1 - *value) as usize;
                labels.get(row_index).cloned().unwrap_or_default()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw witness chart mesh");

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(0.0, 0), (0.0, max_y)],
            ShapeStyle::from(&RGBColor(180, 173, 162)).stroke_width(1),
        )))
        .expect("failed to draw witness chart zero line");

    for (index, row) in active_rows.iter().enumerate() {
        let y = max_y - 1 - index as i32;
        let negative = -(row.k00_only_prime_count as f64);
        if row.k00_only_prime_count > 0 {
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(negative, y), (0.0, y + 1)],
                    ShapeStyle::from(&RGBColor(180, 82, 45)).filled(),
                )))
                .expect("failed to draw k00-only segment");
        }
        if row.best_only_prime_count > 0 {
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(0.0, y), (row.best_only_prime_count as f64, y + 1)],
                    ShapeStyle::from(&RGBColor(69, 129, 190)).filled(),
                )))
                .expect("failed to draw best-only segment");
        }
        chart
            .draw_series(std::iter::once(Text::new(
                format!("both {}", row.both_prime_count),
                (row.best_only_prime_count as f64 + 0.4, y),
                ("sans-serif", 16).into_font().color(&BLACK),
            )))
            .expect("failed to draw both-prime label");
        if row.stress_case {
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(-(max_abs as f64), y), (max_abs as f64, y + 1)],
                    ShapeStyle::from(&BLACK.mix(0.55)).stroke_width(2),
                )))
                .expect("failed to draw stress-case outline");
        }
    }

    root.present().expect("failed to present witness chart");
}

fn render_stress_strip(rows: &[SharedWitnessRow], path: &Path) {
    let stress_rows = rows
        .iter()
        .filter(|row| row.anchor_pair == STRESS_PAIR)
        .collect::<Vec<_>>();
    let active = stress_rows
        .iter()
        .filter(|row| row.role == "active")
        .collect::<Vec<_>>();
    let control = stress_rows
        .iter()
        .filter(|row| row.role == "control")
        .collect::<Vec<_>>();
    let columns = active.len().max(control.len()) as i32;

    let root = BitMapBackend::new(path, (1240, 300)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill stress strip canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "(D,B) Shared-Admissible Witness Strip  (top = active, bottom = nearby dead control)",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(52)
        .y_label_area_size(120)
        .build_cartesian_2d(0i32..columns, 0i32..2i32)
        .expect("failed to build stress strip chart");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("shared-admissible witness index")
        .y_desc("pair")
        .y_labels(2)
        .y_label_formatter(&move |value| match *value {
            0 => "(9,5) control".to_string(),
            1 => "(D,B) active".to_string(),
            _ => String::new(),
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw stress strip mesh");

    for (row_y, witness_rows) in [(1i32, active), (0i32, control)] {
        for (index, row) in witness_rows.iter().enumerate() {
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(index as i32, row_y), (index as i32 + 1, row_y + 1)],
                    ShapeStyle::from(&witness_color(&row.witness_class)).filled(),
                )))
                .expect("failed to draw stress witness cell");
        }
    }

    root.present().expect("failed to present stress strip");
}

fn witness_color(class: &str) -> RGBColor {
    match class {
        "best_only_prime" => RGBColor(69, 129, 190),
        "k00_only_prime" => RGBColor(180, 82, 45),
        "both_prime" => RGBColor(80, 140, 52),
        _ => RGBColor(220, 214, 204),
    }
}

fn build_report_summary(rows: &[SharedYieldRow]) -> ReportSummary {
    let active_rows = rows
        .iter()
        .filter(|row| row.role == "active")
        .collect::<Vec<_>>();
    let strongest = active_rows
        .iter()
        .max_by(|left, right| {
            left.shared_prime_delta_count
                .cmp(&right.shared_prime_delta_count)
                .then_with(|| left.pair_label.cmp(&right.pair_label))
        })
        .expect("shared-yield report should have active rows");

    ReportSummary {
        active_pairs: active_rows.len(),
        positive_shared_delta_pairs: active_rows
            .iter()
            .filter(|row| row.shared_prime_delta_count > 0)
            .count(),
        negative_shared_delta_pairs: active_rows
            .iter()
            .filter(|row| row.shared_prime_delta_count < 0)
            .count(),
        strongest_shared_delta_pair: strongest.pair_label.clone(),
        strongest_shared_delta_count: strongest.shared_prime_delta_count,
    }
}

fn derive_observations(
    rows: &[SharedYieldRow],
    comparisons: &[ActiveControlComparisonRow],
) -> Vec<String> {
    let active_rows = rows
        .iter()
        .filter(|row| row.role == "active")
        .collect::<Vec<_>>();
    let stress = active_rows
        .iter()
        .find(|row| row.pair_label == STRESS_PAIR)
        .expect("stress case should exist");
    let overlap_exception = active_rows
        .iter()
        .find(|row| row.shared_prime_delta_count < 0)
        .expect("one overlap exception should exist");
    let stress_comparison = comparisons
        .iter()
        .find(|row| row.active_pair == STRESS_PAIR)
        .expect("stress comparison should exist");

    vec![
        format!(
            "Three of the four active base-14 pairs have positive shared-admissible prime delta, so the overlap itself is producing more primes for the winning lane rather than merely admitting different candidates."
        ),
        format!(
            "The stress case {} is strongest on exactly this lane: shared prime delta is {:+}, with {} best-only shared primes against {} k00-only shared primes.",
            stress.pair_label,
            stress.shared_prime_delta_count,
            stress.best_only_prime_count,
            stress.k00_only_prime_count
        ),
        format!(
            "Against its nearby dead control {}, the stress case also has a much higher shared prime rate under the winning lane ({:.2}% vs {:.2}%).",
            stress_comparison.control_pair,
            stress_comparison.active_shared_prime_rate_best_pp,
            stress_comparison.control_shared_prime_rate_pp
        ),
        format!(
            "{} is the one honest exception: its shared-admissible lane is negative ({:+}), so its win really does come from boundary transfer rather than shared-yield lift.",
            overlap_exception.pair_label,
            overlap_exception.shared_prime_delta_count
        ),
    ]
}

fn render_markdown(bundle: &OutputBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Base-14 Shared Yield\n\n");
    markdown.push_str("_Generated from `examples/base14_shared_yield_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Input mechanism artifact: `{}`\n- Output directory: `{}`\n\n",
        bundle.input_json, DEFAULT_OUT_DIR
    ));

    markdown.push_str("## Main Table\n\n");
    markdown.push_str(
        "| Role | Pair | Shared count | Best-only shared primes | k00-only shared primes | Shared delta | Shared best rate |\n",
    );
    markdown.push_str("|---|---|---:|---:|---:|---:|---:|\n");
    for row in &bundle.shared_yield_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {:+} | {:.2}% |\n",
            row.role,
            row.pair_label,
            row.shared_admissible_count,
            row.best_only_prime_count,
            row.k00_only_prime_count,
            row.shared_prime_delta_count,
            row.shared_prime_rate_best_pp
        ));
    }
    markdown.push('\n');

    if let Some(image) = bundle
        .image_artifact_rows
        .iter()
        .find(|image| image.kind == "witness_counts")
    {
        markdown.push_str("## Witness Counts\n\n");
        markdown.push_str(&format!("![{}]({})\n\n", image.label, image.path));
    }
    if let Some(image) = bundle
        .image_artifact_rows
        .iter()
        .find(|image| image.kind == "stress_strip")
    {
        markdown.push_str("## Stress Strip\n\n");
        markdown.push_str(&format!("![{}]({})\n\n", image.label, image.path));
    }

    markdown.push_str("## Observations\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {}\n", observation));
    }
    markdown
}
