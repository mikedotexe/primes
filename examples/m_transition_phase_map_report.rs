//! Geometric phase-map view of the bounded-`k` transition lane.
//!
//! This example is intentionally downstream of `m_transition_curve_report`:
//! it reads the machine-readable transition artifact and renders a grouped
//! phase map whose rows are `(base, pair)` and whose columns are middle length
//! `M`.
//!
//! The visualization keeps two signals visible at once:
//! - hue = winning `k` lane
//! - saturation = positive anomaly mass `max(best_minus_k00_pp, 0)`
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example m_transition_phase_map_report
//! cargo run --release --example m_transition_phase_map_report -- --input-json /tmp/primes_m_transition_curve/summary.json --out-dir /tmp/primes_m_transition_phase_map
//! ```

use plotters::prelude::*;
use primes::validation::reporting::{
    ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
};

const DEFAULT_INPUT_JSON: &str = "/tmp/primes_m_transition_curve/summary.json";
const DEFAULT_OUT_DIR: &str = "/tmp/primes_m_transition_phase_map";
const REPORT_EXPORT_VERSION: u32 = 1;

#[derive(Debug)]
struct Options {
    input_json: PathBuf,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct InputSettings {
    bases: Vec<u32>,
    pair_catalog_mode: String,
    min_middle_length: usize,
    max_middle_length: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct PairLengthRow {
    base: u32,
    middle_length: usize,
    pair_label: String,
    best_k: String,
    k00_noninferior: bool,
    best_minus_k00_pp: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct InputBundle {
    generated_at_utc: String,
    settings: InputSettings,
    pair_length_rows: Vec<PairLengthRow>,
}

#[derive(Debug, Clone, Serialize)]
struct PhaseCellRow {
    base: u32,
    pair_label: String,
    row_label: String,
    middle_length: usize,
    best_k: String,
    k00_noninferior: bool,
    anomaly_mass_pp: f64,
    phase_state: String,
}

#[derive(Debug, Clone, Serialize)]
struct PairSummaryRow {
    base: u32,
    pair_label: String,
    row_label: String,
    total_anomaly_mass_pp: f64,
    active_lengths: String,
    last_active_middle_length: Option<usize>,
    strongest_middle_length: usize,
    strongest_best_k: String,
    strongest_mass_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct BaseSummaryRow {
    base: u32,
    pair_count: usize,
    anomalous_pairs: usize,
    latest_active_middle_length: Option<usize>,
    total_anomaly_mass_pp: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct OutputBundle {
    export_version: u32,
    generated_at_utc: String,
    input_json: String,
    input_generated_at_utc: String,
    pair_catalog_mode: String,
    phase_cell_rows: Vec<PhaseCellRow>,
    pair_summary_rows: Vec<PairSummaryRow>,
    base_summary_rows: Vec<BaseSummaryRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let input_bundle = load_input_bundle(&options.input_json);
    let phase_cell_rows = build_phase_cell_rows(&input_bundle);
    let pair_summary_rows = build_pair_summary_rows(&phase_cell_rows, &input_bundle.settings.bases);
    let base_summary_rows = build_base_summary_rows(&pair_summary_rows);
    let overview_path = options.out_dir.join("transition_phase_map.png");
    render_phase_map(
        &input_bundle,
        &phase_cell_rows,
        &pair_summary_rows,
        &overview_path,
        "Bounded-k Transition Phase Map  (hue = best k, saturation = anomaly mass)",
    );
    let mut image_artifact_rows = vec![ImageArtifactRow {
        kind: "phase_map".to_string(),
        label: "Bounded-k transition phase map".to_string(),
        path: overview_path.display().to_string(),
    }];
    for base_summary in &base_summary_rows {
        let base_path = options.out_dir.join(format!(
            "transition_phase_map_base_{}.png",
            base_summary.base
        ));
        let base_pair_rows = pair_summary_rows
            .iter()
            .filter(|row| row.base == base_summary.base)
            .cloned()
            .collect::<Vec<_>>();
        let base_phase_rows = phase_cell_rows
            .iter()
            .filter(|row| row.base == base_summary.base)
            .cloned()
            .collect::<Vec<_>>();
        render_phase_map(
            &input_bundle,
            &base_phase_rows,
            &base_pair_rows,
            &base_path,
            &format!(
                "Base {} Transition Phase Map  (hue = best k, saturation = anomaly mass)",
                base_summary.base
            ),
        );
        image_artifact_rows.push(ImageArtifactRow {
            kind: "phase_map_base".to_string(),
            label: format!("Base {} transition phase map", base_summary.base),
            path: base_path.display().to_string(),
        });
    }
    let observations = derive_observations(&input_bundle, &phase_cell_rows, &base_summary_rows);

    let output_bundle = OutputBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        input_json: options.input_json.display().to_string(),
        input_generated_at_utc: input_bundle.generated_at_utc.clone(),
        pair_catalog_mode: input_bundle.settings.pair_catalog_mode.clone(),
        phase_cell_rows,
        pair_summary_rows,
        base_summary_rows,
        image_artifact_rows,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("phase_cell_rows.csv"),
        &output_bundle.phase_cell_rows,
    )
    .expect("failed to write phase cell rows");
    write_csv_rows(
        options.out_dir.join("pair_summary_rows.csv"),
        &output_bundle.pair_summary_rows,
    )
    .expect("failed to write pair summary rows");
    write_csv_rows(
        options.out_dir.join("base_summary_rows.csv"),
        &output_bundle.base_summary_rows,
    )
    .expect("failed to write base summary rows");
    write_csv_rows(
        options.out_dir.join("image_artifact_rows.csv"),
        &output_bundle.image_artifact_rows,
    )
    .expect("failed to write image artifact rows");
    write_json_pretty(options.out_dir.join("summary.json"), &output_bundle)
        .expect("failed to write summary json");
    write_text_file(
        options.out_dir.join("report.md"),
        &render_markdown_report(&output_bundle),
    )
    .expect("failed to write markdown report");

    print_summary(&output_bundle);
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
    println!("Bounded-k transition phase map report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example m_transition_phase_map_report -- [options]");
    println!();
    println!("Options:");
    println!(
        "  --input-json <path>       Transition curve artifact (default: {DEFAULT_INPUT_JSON})"
    );
    println!(
        "  --out-dir <path>          Output directory for images and summary files (default: {DEFAULT_OUT_DIR})"
    );
}

fn load_input_bundle(path: &Path) -> InputBundle {
    let contents = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read input JSON at {}: {err}\nRun `cargo run --release --example m_transition_curve_report` first or pass --input-json.",
            path.display()
        );
        std::process::exit(1);
    });
    serde_json::from_str(&contents).unwrap_or_else(|err| {
        eprintln!("Failed to parse input JSON at {}: {err}", path.display());
        std::process::exit(1);
    })
}

fn build_phase_cell_rows(input_bundle: &InputBundle) -> Vec<PhaseCellRow> {
    let mut rows = input_bundle
        .pair_length_rows
        .iter()
        .map(|row| PhaseCellRow {
            base: row.base,
            pair_label: row.pair_label.clone(),
            row_label: format!("b{} {}", row.base, row.pair_label),
            middle_length: row.middle_length,
            best_k: row.best_k.clone(),
            k00_noninferior: row.k00_noninferior,
            anomaly_mass_pp: row.best_minus_k00_pp.max(0.0),
            phase_state: if row.k00_noninferior {
                "k00_noninferior".to_string()
            } else {
                row.best_k.clone()
            },
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.pair_label.cmp(&right.pair_label))
            .then_with(|| left.middle_length.cmp(&right.middle_length))
    });
    rows
}

fn build_pair_summary_rows(rows: &[PhaseCellRow], ordered_bases: &[u32]) -> Vec<PairSummaryRow> {
    let mut by_pair: BTreeMap<(u32, String), Vec<&PhaseCellRow>> = BTreeMap::new();
    for row in rows {
        by_pair
            .entry((row.base, row.pair_label.clone()))
            .or_default()
            .push(row);
    }

    let base_rank = ordered_bases
        .iter()
        .enumerate()
        .map(|(index, &base)| (base, index))
        .collect::<BTreeMap<_, _>>();

    let mut summaries = by_pair
        .into_iter()
        .map(|((base, pair_label), pair_rows)| {
            let total_anomaly_mass_pp =
                pair_rows.iter().map(|row| row.anomaly_mass_pp).sum::<f64>();
            let active_lengths = pair_rows
                .iter()
                .filter(|row| row.anomaly_mass_pp > 0.0)
                .map(|row| row.middle_length.to_string())
                .collect::<Vec<_>>();
            let strongest = pair_rows
                .iter()
                .max_by(|left, right| {
                    left.anomaly_mass_pp
                        .total_cmp(&right.anomaly_mass_pp)
                        .then_with(|| left.middle_length.cmp(&right.middle_length))
                })
                .expect("pair rows should not be empty");

            PairSummaryRow {
                base,
                row_label: format!("b{} {}", base, pair_label),
                pair_label,
                total_anomaly_mass_pp,
                active_lengths: if active_lengths.is_empty() {
                    "none".to_string()
                } else {
                    active_lengths.join(",")
                },
                last_active_middle_length: pair_rows
                    .iter()
                    .filter(|row| row.anomaly_mass_pp > 0.0)
                    .map(|row| row.middle_length)
                    .max(),
                strongest_middle_length: strongest.middle_length,
                strongest_best_k: strongest.best_k.clone(),
                strongest_mass_pp: strongest.anomaly_mass_pp,
            }
        })
        .collect::<Vec<_>>();

    summaries.sort_by(|left, right| {
        base_rank
            .get(&left.base)
            .cmp(&base_rank.get(&right.base))
            .then_with(|| {
                right
                    .total_anomaly_mass_pp
                    .total_cmp(&left.total_anomaly_mass_pp)
            })
            .then_with(|| {
                right
                    .last_active_middle_length
                    .unwrap_or(0)
                    .cmp(&left.last_active_middle_length.unwrap_or(0))
            })
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });
    summaries
}

fn build_base_summary_rows(pair_summary_rows: &[PairSummaryRow]) -> Vec<BaseSummaryRow> {
    let mut by_base: BTreeMap<u32, Vec<&PairSummaryRow>> = BTreeMap::new();
    for row in pair_summary_rows {
        by_base.entry(row.base).or_default().push(row);
    }

    by_base
        .into_iter()
        .map(|(base, rows)| BaseSummaryRow {
            base,
            pair_count: rows.len(),
            anomalous_pairs: rows
                .iter()
                .filter(|row| row.total_anomaly_mass_pp > 0.0)
                .count(),
            latest_active_middle_length: rows
                .iter()
                .filter_map(|row| row.last_active_middle_length)
                .max(),
            total_anomaly_mass_pp: rows.iter().map(|row| row.total_anomaly_mass_pp).sum(),
        })
        .collect()
}

fn render_phase_map(
    input_bundle: &InputBundle,
    phase_cell_rows: &[PhaseCellRow],
    pair_summary_rows: &[PairSummaryRow],
    path: &Path,
    title: &str,
) {
    let pair_order = pair_summary_rows
        .iter()
        .map(|row| (row.base, row.pair_label.clone(), row.row_label.clone()))
        .collect::<Vec<_>>();
    let row_count = pair_order.len();
    let middle_lengths = (input_bundle.settings.min_middle_length
        ..=input_bundle.settings.max_middle_length)
        .collect::<Vec<_>>();
    let max_mass = phase_cell_rows
        .iter()
        .map(|row| row.anomaly_mass_pp)
        .fold(0.0_f64, f64::max);
    let height = (180 + row_count as u32 * 28).max(520);
    let root = BitMapBackend::new(path, (1320, height)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill phase map canvas");

    let row_labels = pair_order
        .iter()
        .map(|(_, _, row_label)| row_label.clone())
        .collect::<Vec<_>>();
    let row_lookup = pair_order
        .iter()
        .enumerate()
        .map(|(index, (base, pair_label, _))| ((*base, pair_label.clone()), index as i32))
        .collect::<BTreeMap<_, _>>();
    let x_labels = middle_lengths
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>();
    let max_x = middle_lengths.len() as i32;
    let max_y = row_count as i32;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 28))
        .margin(24)
        .x_label_area_size(56)
        .y_label_area_size(170)
        .build_cartesian_2d(0i32..max_x, 0i32..max_y)
        .expect("failed to build phase map chart");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(middle_lengths.len())
        .y_labels(row_count)
        .x_desc("middle length M")
        .y_desc("base / ordered unit pair")
        .x_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < x_labels.len() {
                x_labels[*value as usize].clone()
            } else {
                String::new()
            }
        })
        .y_label_formatter(&move |value| {
            let row_index = (max_y - 1 - (*value).clamp(0, max_y - 1)) as usize;
            row_labels.get(row_index).cloned().unwrap_or_default()
        })
        .axis_style(RGBColor(92, 86, 78))
        .label_style(("sans-serif", 16))
        .draw()
        .expect("failed to draw phase map mesh");

    for row in phase_cell_rows {
        let x = (row.middle_length - input_bundle.settings.min_middle_length) as i32;
        let order_index = *row_lookup
            .get(&(row.base, row.pair_label.clone()))
            .expect("phase row should have pair summary");
        let y = max_y - 1 - order_index;
        let fill = phase_color(&row.best_k, row.anomaly_mass_pp, max_mass);
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x, y), (x + 1, y + 1)],
                ShapeStyle::from(&fill).filled(),
            )))
            .expect("failed to draw phase cell");

        if row.anomaly_mass_pp > 0.0 {
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(x, y), (x + 1, y + 1)],
                    ShapeStyle::from(&BLACK.mix(0.7)).stroke_width(2),
                )))
                .expect("failed to draw anomaly border");
        }
    }

    let mut seen_bases = BTreeSet::new();
    for (row_index, (base, _, _)) in pair_order.iter().enumerate() {
        if seen_bases.insert(*base) && row_index > 0 {
            let y = max_y - row_index as i32;
            chart
                .draw_series(std::iter::once(PathElement::new(
                    vec![(0, y), (max_x, y)],
                    RGBColor(150, 143, 132).stroke_width(2),
                )))
                .expect("failed to draw base separator");
        }
    }

    root.present().expect("failed to present phase map image");
}

fn phase_color(best_k: &str, anomaly_mass_pp: f64, max_mass_pp: f64) -> RGBColor {
    let normalized = if max_mass_pp > 0.0 {
        (anomaly_mass_pp / max_mass_pp).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let (start, end) = match best_k {
        "k=(0,1)" => ((228.0, 237.0, 245.0), (59.0, 99.0, 140.0)),
        "k=(1,0)" => ((250.0, 235.0, 220.0), (214.0, 108.0, 33.0)),
        "k=(1,1)" => ((224.0, 242.0, 238.0), (23.0, 133.0, 123.0)),
        "k=(2,2)" => ((243.0, 231.0, 238.0), (145.0, 68.0, 97.0)),
        _ => ((245.0, 241.0, 235.0), (213.0, 206.0, 197.0)),
    };
    let t = if best_k == "k=(0,0)" {
        0.25
    } else {
        0.25 + 0.75 * normalized
    };
    RGBColor(
        lerp(start.0, end.0, t) as u8,
        lerp(start.1, end.1, t) as u8,
        lerp(start.2, end.2, t) as u8,
    )
}

fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

fn derive_observations(
    input_bundle: &InputBundle,
    phase_cell_rows: &[PhaseCellRow],
    base_summary_rows: &[BaseSummaryRow],
) -> Vec<String> {
    let mut anomaly_counts_by_length = BTreeMap::new();
    for row in phase_cell_rows {
        if row.anomaly_mass_pp > 0.0 {
            *anomaly_counts_by_length
                .entry(row.middle_length)
                .or_insert(0usize) += 1;
        }
    }
    let anomaly_profile = (input_bundle.settings.min_middle_length
        ..=input_bundle.settings.max_middle_length)
        .map(|middle_length| {
            format!(
                "M{}:{}",
                middle_length,
                anomaly_counts_by_length
                    .get(&middle_length)
                    .copied()
                    .unwrap_or(0)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let most_persistent_base = base_summary_rows
        .iter()
        .max_by(|left, right| {
            left.latest_active_middle_length
                .unwrap_or(0)
                .cmp(&right.latest_active_middle_length.unwrap_or(0))
                .then_with(|| {
                    left.total_anomaly_mass_pp
                        .total_cmp(&right.total_anomaly_mass_pp)
                })
        })
        .expect("base summaries should not be empty");
    let last_active_length = base_summary_rows
        .iter()
        .filter_map(|row| row.latest_active_middle_length)
        .max()
        .unwrap_or(0);

    vec![
        format!(
            "The phase map should show the anomaly cloud collapsing column by column: positive cells by length are `{}`.",
            anomaly_profile
        ),
        format!(
            "Base `{}` is the latest active lane in this artifact, but even there the last colored cells stop by `M={}`.",
            most_persistent_base.base, last_active_length
        ),
        "Warm/orange cells mark `k=(1,0)` wins, cool/blue cells mark `k=(0,1)` wins, and the pale neutral background is the settled `k=(0,0)` regime.".to_string(),
    ]
}

fn render_markdown_report(bundle: &OutputBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Bounded-k Transition Phase Map Report\n\n");
    markdown.push_str("_Generated from `examples/m_transition_phase_map_report.rs`._\n\n");
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

    markdown.push_str("## Phase Legend\n\n");
    markdown.push_str("- `k=(0,0)`: pale neutral cells, compact settled regime\n");
    markdown.push_str("- `k=(0,1)`: blue cells, offset anomaly lane\n");
    markdown.push_str("- `k=(1,0)`: orange cells, offset anomaly lane\n");
    markdown.push_str("- `k=(1,1)`: teal cells, wider symmetric lane\n");
    markdown.push_str("- `k=(2,2)`: plum cells, widest maintained lane\n\n");

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

    markdown.push_str("## Base Summary\n\n");
    markdown.push_str("| Base | Pairs | Anomalous pairs | Last active M | Total anomaly mass |\n");
    markdown.push_str("|---:|---:|---:|---:|---:|\n");
    for row in &bundle.base_summary_rows {
        let last_active = row
            .latest_active_middle_length
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string());
        markdown.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{:.2}pp` |\n",
            row.base, row.pair_count, row.anomalous_pairs, last_active, row.total_anomaly_mass_pp
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Leading Pairs\n\n");
    markdown
        .push_str("| Row | Active lengths | Strongest M | Strongest best k | Strongest mass |\n");
    markdown.push_str("|---|---|---:|---|---:|\n");
    for row in bundle.pair_summary_rows.iter().take(12) {
        markdown.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{:.2}pp` |\n",
            row.row_label,
            row.active_lengths,
            row.strongest_middle_length,
            row.strongest_best_k,
            row.strongest_mass_pp
        ));
    }

    markdown
}

fn print_summary(bundle: &OutputBundle) {
    println!("=== Bounded-k Transition Phase Map Report ===");
    println!();
    println!(
        "Input {} | output {}",
        bundle.input_json, bundle.pair_catalog_mode
    );
    println!(
        "Phase cells: {} | pair rows: {} | image {}",
        bundle.phase_cell_rows.len(),
        bundle.pair_summary_rows.len(),
        bundle.image_artifact_rows[0].path
    );
    for row in &bundle.base_summary_rows {
        println!(
            "  - base {:>2}: {} pairs | anomalous {} | last active {} | total mass {:.2}pp",
            row.base,
            row.pair_count,
            row.anomalous_pairs,
            row.latest_active_middle_length
                .map(|value| format!("M{value}"))
                .unwrap_or_else(|| "none".to_string()),
            row.total_anomaly_mass_pp
        );
    }
}
