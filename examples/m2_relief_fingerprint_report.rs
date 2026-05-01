//! Residue-relief fingerprint comparison for the `M=2` survivor species.
//!
//! This example is intentionally downstream of `m2_species_aggregation_report`:
//! it reads the species artifact and renders a side-by-side fingerprint for the
//! two live `M=2` species:
//! - `m1_to_m2` persistent survivors
//! - `m2_only` emergent cases
//!
//! The visualization focuses on moduli because that is where the species split
//! became clearest in the earlier reports.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example m2_relief_fingerprint_report
//! cargo run --release --example m2_relief_fingerprint_report -- --input-json /tmp/primes_m2_species_aggregation/summary.json --out-dir /tmp/primes_m2_relief_fingerprint
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

const DEFAULT_INPUT_JSON: &str = "/tmp/primes_m2_species_aggregation/summary.json";
const DEFAULT_OUT_DIR: &str = "/tmp/primes_m2_relief_fingerprint";
const REPORT_EXPORT_VERSION: u32 = 1;
const TARGET_MIDDLE_LENGTH: usize = 2;
const PERSISTENT_SPECIES: &str = "m1_to_m2";
const EMERGENT_SPECIES: &str = "m2_only";

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
struct SpeciesSummaryRow {
    species: String,
    pair_count: usize,
    top_moduli_m2: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SpeciesModulusRow {
    species: String,
    middle_length: usize,
    modulus: u32,
    positive_relief_share: f64,
    mean_relief_pp: f64,
    median_relief_pp: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct InputBundle {
    generated_at_utc: String,
    settings: InputSettings,
    species_summary_rows: Vec<SpeciesSummaryRow>,
    species_modulus_rows: Vec<SpeciesModulusRow>,
}

#[derive(Debug, Clone, Serialize)]
struct FingerprintRow {
    modulus: u32,
    persistent_share: f64,
    emergent_share: f64,
    share_gap_pp: f64,
    persistent_mean_relief_pp: f64,
    emergent_mean_relief_pp: f64,
    mean_gap_pp: f64,
    persistent_median_relief_pp: f64,
    emergent_median_relief_pp: f64,
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
    fingerprint_rows: Vec<FingerprintRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let input_bundle = load_input_bundle(&options.input_json);
    let fingerprint_rows = build_fingerprint_rows(&input_bundle);
    let fingerprint_path = options.out_dir.join("relief_fingerprint_m2.png");
    render_fingerprint_chart(&fingerprint_rows, &fingerprint_path);
    let image_artifact_rows = vec![ImageArtifactRow {
        kind: "fingerprint_chart".to_string(),
        label: "Persistent vs emergent M=2 relief fingerprint".to_string(),
        path: fingerprint_path.display().to_string(),
    }];
    let observations = derive_observations(&input_bundle, &fingerprint_rows);

    let output_bundle = OutputBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        input_json: options.input_json.display().to_string(),
        input_generated_at_utc: input_bundle.generated_at_utc.clone(),
        pair_catalog_mode: input_bundle.settings.pair_catalog_mode.clone(),
        fingerprint_rows,
        image_artifact_rows,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("fingerprint_rows.csv"),
        &output_bundle.fingerprint_rows,
    )
    .expect("failed to write fingerprint rows");
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
    println!("M=2 relief fingerprint report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example m2_relief_fingerprint_report -- [options]");
    println!();
    println!("Options:");
    println!(
        "  --input-json <path>       Species aggregation artifact (default: {DEFAULT_INPUT_JSON})"
    );
    println!(
        "  --out-dir <path>          Output directory for images and summary files (default: {DEFAULT_OUT_DIR})"
    );
}

fn load_input_bundle(path: &Path) -> InputBundle {
    let contents = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read input JSON at {}: {err}\nRun `cargo run --release --example m2_species_aggregation_report` first or pass --input-json.",
            path.display()
        );
        std::process::exit(1);
    });
    serde_json::from_str(&contents).unwrap_or_else(|err| {
        eprintln!("Failed to parse input JSON at {}: {err}", path.display());
        std::process::exit(1);
    })
}

fn build_fingerprint_rows(input_bundle: &InputBundle) -> Vec<FingerprintRow> {
    let relevant_rows = input_bundle
        .species_modulus_rows
        .iter()
        .filter(|row| row.middle_length == TARGET_MIDDLE_LENGTH)
        .filter(|row| row.species == PERSISTENT_SPECIES || row.species == EMERGENT_SPECIES)
        .collect::<Vec<_>>();
    let mut by_modulus: BTreeMap<u32, Vec<&SpeciesModulusRow>> = BTreeMap::new();
    for row in relevant_rows {
        by_modulus.entry(row.modulus).or_default().push(row);
    }

    let mut rows = by_modulus
        .into_iter()
        .map(|(modulus, entries)| {
            let persistent = entries
                .iter()
                .find(|row| row.species == PERSISTENT_SPECIES)
                .expect("persistent fingerprint row should exist");
            let emergent = entries
                .iter()
                .find(|row| row.species == EMERGENT_SPECIES)
                .expect("emergent fingerprint row should exist");

            FingerprintRow {
                modulus,
                persistent_share: persistent.positive_relief_share,
                emergent_share: emergent.positive_relief_share,
                share_gap_pp: (emergent.positive_relief_share - persistent.positive_relief_share)
                    * 100.0,
                persistent_mean_relief_pp: persistent.mean_relief_pp,
                emergent_mean_relief_pp: emergent.mean_relief_pp,
                mean_gap_pp: emergent.mean_relief_pp - persistent.mean_relief_pp,
                persistent_median_relief_pp: persistent.median_relief_pp,
                emergent_median_relief_pp: emergent.median_relief_pp,
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.modulus.cmp(&right.modulus));
    rows
}

fn render_fingerprint_chart(rows: &[FingerprintRow], path: &Path) {
    let root = BitMapBackend::new(path, (1280, 880)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill fingerprint canvas");
    let panels = root.split_evenly((2, 1));

    let moduli = rows.iter().map(|row| row.modulus).collect::<Vec<_>>();
    let x_labels = moduli.iter().map(u32::to_string).collect::<Vec<_>>();
    let max_x = moduli.len() as i32;
    let share_max = rows
        .iter()
        .map(|row| row.persistent_share.max(row.emergent_share))
        .fold(0.0_f64, f64::max)
        .max(0.05);
    let min_mean = rows
        .iter()
        .map(|row| {
            row.persistent_mean_relief_pp
                .min(row.emergent_mean_relief_pp)
        })
        .fold(0.0_f64, f64::min);
    let max_mean = rows
        .iter()
        .map(|row| {
            row.persistent_mean_relief_pp
                .max(row.emergent_mean_relief_pp)
        })
        .fold(0.0_f64, f64::max);
    let mean_padding = 0.15;

    let persistent_color = RGBColor(210, 99, 34);
    let emergent_color = RGBColor(23, 133, 123);

    let mut share_chart = ChartBuilder::on(&panels[0])
        .caption(
            "M=2 Relief Fingerprint  (positive relief share by modulus)",
            ("sans-serif", 24),
        )
        .margin(22)
        .x_label_area_size(45)
        .y_label_area_size(70)
        .build_cartesian_2d(0i32..max_x, 0.0f64..(share_max * 1.15))
        .expect("failed to build share chart");
    share_chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(moduli.len())
        .y_desc("positive relief share")
        .x_desc("small prime modulus")
        .x_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < x_labels.len() {
                x_labels[*value as usize].clone()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw share mesh");

    let persistent_share_points = rows
        .iter()
        .enumerate()
        .map(|(index, row)| (index as i32, row.persistent_share))
        .collect::<Vec<_>>();
    let emergent_share_points = rows
        .iter()
        .enumerate()
        .map(|(index, row)| (index as i32, row.emergent_share))
        .collect::<Vec<_>>();
    share_chart
        .draw_series(LineSeries::new(
            persistent_share_points.iter().copied(),
            persistent_color.stroke_width(3),
        ))
        .expect("failed to draw persistent share series");
    share_chart
        .draw_series(LineSeries::new(
            emergent_share_points.iter().copied(),
            emergent_color.stroke_width(3),
        ))
        .expect("failed to draw emergent share series");
    share_chart
        .draw_series(
            persistent_share_points
                .iter()
                .copied()
                .map(|point| Circle::new(point, 5, ShapeStyle::from(&persistent_color).filled())),
        )
        .expect("failed to draw persistent share points");
    share_chart
        .draw_series(
            emergent_share_points
                .iter()
                .copied()
                .map(|point| Circle::new(point, 5, ShapeStyle::from(&emergent_color).filled())),
        )
        .expect("failed to draw emergent share points");

    let x_labels = moduli.iter().map(u32::to_string).collect::<Vec<_>>();
    let mut mean_chart = ChartBuilder::on(&panels[1])
        .caption(
            "M=2 Relief Fingerprint  (mean relief in percentage points)",
            ("sans-serif", 24),
        )
        .margin(22)
        .x_label_area_size(55)
        .y_label_area_size(70)
        .build_cartesian_2d(
            0i32..max_x,
            (min_mean - mean_padding)..(max_mean + mean_padding),
        )
        .expect("failed to build mean-relief chart");
    mean_chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(moduli.len())
        .y_desc("mean relief (pp)")
        .x_desc("small prime modulus")
        .x_label_formatter(&move |value| {
            if *value >= 0 && (*value as usize) < x_labels.len() {
                x_labels[*value as usize].clone()
            } else {
                String::new()
            }
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw mean-relief mesh");

    mean_chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(0, 0.0), (max_x, 0.0)],
            RGBColor(150, 143, 132).stroke_width(2),
        )))
        .expect("failed to draw zero baseline");

    let persistent_mean_points = rows
        .iter()
        .enumerate()
        .map(|(index, row)| (index as i32, row.persistent_mean_relief_pp))
        .collect::<Vec<_>>();
    let emergent_mean_points = rows
        .iter()
        .enumerate()
        .map(|(index, row)| (index as i32, row.emergent_mean_relief_pp))
        .collect::<Vec<_>>();
    mean_chart
        .draw_series(LineSeries::new(
            persistent_mean_points.iter().copied(),
            persistent_color.stroke_width(3),
        ))
        .expect("failed to draw persistent mean series");
    mean_chart
        .draw_series(LineSeries::new(
            emergent_mean_points.iter().copied(),
            emergent_color.stroke_width(3),
        ))
        .expect("failed to draw emergent mean series");
    mean_chart
        .draw_series(
            persistent_mean_points
                .iter()
                .copied()
                .map(|point| Circle::new(point, 5, ShapeStyle::from(&persistent_color).filled())),
        )
        .expect("failed to draw persistent mean points");
    mean_chart
        .draw_series(
            emergent_mean_points
                .iter()
                .copied()
                .map(|point| Circle::new(point, 5, ShapeStyle::from(&emergent_color).filled())),
        )
        .expect("failed to draw emergent mean points");

    panels[0]
        .draw(&Text::new(
            "orange = persistent m1_to_m2, teal = emergent m2_only",
            (28, 28),
            ("sans-serif", 18).into_font().color(&RGBColor(72, 66, 58)),
        ))
        .expect("failed to draw fingerprint legend");
    root.present().expect("failed to present fingerprint image");
}

fn derive_observations(input_bundle: &InputBundle, rows: &[FingerprintRow]) -> Vec<String> {
    let summary_lookup = input_bundle
        .species_summary_rows
        .iter()
        .map(|row| (row.species.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    let persistent_summary = summary_lookup
        .get(PERSISTENT_SPECIES)
        .expect("persistent summary should exist");
    let emergent_summary = summary_lookup
        .get(EMERGENT_SPECIES)
        .expect("emergent summary should exist");
    let biggest_share_gap = rows
        .iter()
        .max_by(|left, right| left.share_gap_pp.total_cmp(&right.share_gap_pp))
        .expect("fingerprint rows should not be empty");
    let biggest_mean_gap = rows
        .iter()
        .max_by(|left, right| left.mean_gap_pp.total_cmp(&right.mean_gap_pp))
        .expect("fingerprint rows should not be empty");
    let strongest_persistent_mean = rows
        .iter()
        .max_by(|left, right| {
            left.persistent_mean_relief_pp
                .total_cmp(&right.persistent_mean_relief_pp)
        })
        .expect("fingerprint rows should not be empty");

    vec![
        format!(
            "The emergent species is the more shared relief fingerprint: `{}` pairs vs `{}` persistent pairs, with the biggest share gap at modulus `{}` (`{:.1}pp`).",
            emergent_summary.pair_count,
            persistent_summary.pair_count,
            biggest_share_gap.modulus,
            biggest_share_gap.share_gap_pp
        ),
        format!(
            "The biggest mean-relief separation is at modulus `{}` (`{:.2}pp` emergent minus persistent), while the persistent lane's strongest mean relief is at modulus `{}`.",
            biggest_mean_gap.modulus,
            biggest_mean_gap.mean_gap_pp,
            strongest_persistent_mean.modulus
        ),
        format!(
            "The text summaries agree with the chart: persistent top moduli are `{}`, while emergent top moduli are `{}`.",
            persistent_summary.top_moduli_m2, emergent_summary.top_moduli_m2
        ),
    ]
}

fn render_markdown_report(bundle: &OutputBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# M=2 Relief Fingerprint Report\n\n");
    markdown.push_str("_Generated from `examples/m2_relief_fingerprint_report.rs`._\n\n");
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

    markdown.push_str("## Fingerprint Rows\n\n");
    markdown.push_str("| Modulus | Persistent share | Emergent share | Share gap | Persistent mean | Emergent mean | Mean gap |\n");
    markdown.push_str("|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in &bundle.fingerprint_rows {
        markdown.push_str(&format!(
            "| `{}` | `{:.0}%` | `{:.0}%` | `{:+.1}pp` | `{:+.2}pp` | `{:+.2}pp` | `{:+.2}pp` |\n",
            row.modulus,
            row.persistent_share * 100.0,
            row.emergent_share * 100.0,
            row.share_gap_pp,
            row.persistent_mean_relief_pp,
            row.emergent_mean_relief_pp,
            row.mean_gap_pp
        ));
    }

    markdown
}

fn print_summary(bundle: &OutputBundle) {
    println!("=== M=2 Relief Fingerprint Report ===");
    println!();
    println!(
        "Input {} | output {}",
        bundle.input_json, bundle.pair_catalog_mode
    );
    println!(
        "Fingerprint rows: {} | image {}",
        bundle.fingerprint_rows.len(),
        bundle.image_artifact_rows[0].path
    );
    for row in bundle
        .fingerprint_rows
        .iter()
        .filter(|row| row.share_gap_pp.abs() >= 10.0)
    {
        println!(
            "  - modulus {:>2}: persistent {:.0}% | emergent {:.0}% | share gap {:+.1}pp | mean gap {:+.2}pp",
            row.modulus,
            row.persistent_share * 100.0,
            row.emergent_share * 100.0,
            row.share_gap_pp,
            row.mean_gap_pp
        );
    }
}
