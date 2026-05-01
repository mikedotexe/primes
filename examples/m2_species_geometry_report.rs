//! Geometric visualization report for the short-length anomaly species.
//!
//! This example is intentionally data-first: it reads the maintained artifact
//! produced by `m2_species_aggregation_report` and renders geometric views on
//! top of that measured surface.
//!
//! Outputs:
//! - one pair-lattice PNG per base
//! - one species-vs-modulus heatmap PNG for `M=2`
//! - a markdown summary explaining what to notice
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example m2_species_geometry_report
//! cargo run --release --example m2_species_geometry_report -- --input-json /tmp/primes_m2_species_aggregation/summary.json --out-dir /tmp/primes_m2_species_geometry
//! ```

use plotters::prelude::*;
use primes::{
    validation::bounded_k::{digit_symbol, unit_residues},
    validation::reporting::{
        ensure_dir, export_timestamp_utc, write_csv_rows, write_json_pretty, write_text_file,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};

const DEFAULT_INPUT_JSON: &str = "/tmp/primes_m2_species_aggregation/summary.json";
const DEFAULT_OUT_DIR: &str = "/tmp/primes_m2_species_geometry";
const REPORT_EXPORT_VERSION: u32 = 1;
const HEATMAP_MIDDLE_LENGTH: usize = 2;
const SPECIES_ORDER: &[&str] = &["m1_only", "m1_to_m2", "m2_only"];

#[derive(Debug)]
struct Options {
    input_json: PathBuf,
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct InputSettings {
    bases: Vec<u32>,
    pair_catalog_mode: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SpeciesPairRow {
    species: String,
    base: u32,
    outer: u32,
    inner: u32,
    pair_label: String,
    same_digit: bool,
    anomaly_m1_pp: f64,
    anomaly_m2_pp: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct SpeciesSummaryRow {
    species: String,
    pair_count: usize,
    adjacent_gap_count: usize,
    same_gap_count: usize,
    wide_gap_count: usize,
    top_moduli_m2: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SpeciesModulusRow {
    species: String,
    middle_length: usize,
    modulus: u32,
    positive_relief_share: f64,
    median_relief_pp: f64,
    mean_relief_pp: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct InputBundle {
    generated_at_utc: String,
    settings: InputSettings,
    species_pair_rows: Vec<SpeciesPairRow>,
    species_summary_rows: Vec<SpeciesSummaryRow>,
    species_modulus_rows: Vec<SpeciesModulusRow>,
}

#[derive(Debug, Clone, Serialize)]
struct LatticeBaseSummaryRow {
    base: u32,
    unit_count: usize,
    pair_count: usize,
    m1_only_pairs: usize,
    m1_to_m2_pairs: usize,
    m2_only_pairs: usize,
    strongest_pair: String,
    strongest_display_mass_pp: f64,
    image_path: String,
}

#[derive(Debug, Clone, Serialize)]
struct HeatmapCellRow {
    species: String,
    modulus: u32,
    positive_relief_share: f64,
    mean_relief_pp: f64,
    median_relief_pp: f64,
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
    lattice_base_rows: Vec<LatticeBaseSummaryRow>,
    heatmap_rows: Vec<HeatmapCellRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let input_bundle = load_input_bundle(&options.input_json);
    let lattice_base_rows = render_pair_lattice_images(&input_bundle, &options.out_dir);
    let heatmap_rows = collect_heatmap_rows(&input_bundle);
    let heatmap_path = options.out_dir.join("species_modulus_heatmap_m2.png");
    render_species_modulus_heatmap(&heatmap_rows, &heatmap_path);
    let observations = derive_observations(&input_bundle);

    let mut image_artifact_rows = lattice_base_rows
        .iter()
        .map(|row| ImageArtifactRow {
            kind: "pair_lattice".to_string(),
            label: format!("Base {} pair lattice", row.base),
            path: row.image_path.clone(),
        })
        .collect::<Vec<_>>();
    image_artifact_rows.push(ImageArtifactRow {
        kind: "species_modulus_heatmap".to_string(),
        label: "M=2 species-modulus heatmap".to_string(),
        path: heatmap_path.display().to_string(),
    });

    let output_bundle = OutputBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        input_json: options.input_json.display().to_string(),
        input_generated_at_utc: input_bundle.generated_at_utc.clone(),
        pair_catalog_mode: input_bundle.settings.pair_catalog_mode.clone(),
        lattice_base_rows,
        heatmap_rows,
        image_artifact_rows,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("lattice_base_rows.csv"),
        &output_bundle.lattice_base_rows,
    )
    .expect("failed to write lattice base rows");
    write_csv_rows(
        options.out_dir.join("heatmap_rows.csv"),
        &output_bundle.heatmap_rows,
    )
    .expect("failed to write heatmap rows");
    write_csv_rows(
        options.out_dir.join("image_artifact_rows.csv"),
        &output_bundle.image_artifact_rows,
    )
    .expect("failed to write image artifact rows");
    write_json_pretty(options.out_dir.join("summary.json"), &output_bundle)
        .expect("failed to write geometry summary json");
    write_text_file(
        options.out_dir.join("report.md"),
        &render_markdown_report(&output_bundle),
    )
    .expect("failed to write geometry report markdown");

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
    println!("M=2 species geometry report");
    println!();
    println!("Usage:");
    println!("  cargo run --release --example m2_species_geometry_report -- [options]");
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

fn render_pair_lattice_images(
    input_bundle: &InputBundle,
    out_dir: &Path,
) -> Vec<LatticeBaseSummaryRow> {
    let mut rows = Vec::new();
    for &base in &input_bundle.settings.bases {
        let unit_digits = unit_residues(base);
        let unit_symbols = unit_digits
            .iter()
            .copied()
            .map(digit_symbol)
            .collect::<Vec<_>>();
        let base_rows = input_bundle
            .species_pair_rows
            .iter()
            .filter(|row| row.base == base)
            .cloned()
            .collect::<Vec<_>>();
        let image_path = out_dir.join(format!("pair_lattice_base_{base}.png"));
        render_pair_lattice_base(base, &unit_digits, &unit_symbols, &base_rows, &image_path);

        let strongest = base_rows
            .iter()
            .max_by(|left, right| {
                display_mass(left)
                    .total_cmp(&display_mass(right))
                    .then_with(|| left.pair_label.cmp(&right.pair_label))
            })
            .expect("base rows should not be empty");

        rows.push(LatticeBaseSummaryRow {
            base,
            unit_count: unit_digits.len(),
            pair_count: base_rows.len(),
            m1_only_pairs: base_rows
                .iter()
                .filter(|row| row.species == "m1_only")
                .count(),
            m1_to_m2_pairs: base_rows
                .iter()
                .filter(|row| row.species == "m1_to_m2")
                .count(),
            m2_only_pairs: base_rows
                .iter()
                .filter(|row| row.species == "m2_only")
                .count(),
            strongest_pair: strongest.pair_label.clone(),
            strongest_display_mass_pp: display_mass(strongest),
            image_path: image_path.display().to_string(),
        });
    }
    rows
}

fn render_pair_lattice_base(
    base: u32,
    unit_digits: &[u32],
    unit_symbols: &[String],
    rows: &[SpeciesPairRow],
    path: &Path,
) {
    let size = (900, 900);
    let root = BitMapBackend::new(path, size).into_drawing_area();
    root.fill(&RGBColor(250, 248, 244))
        .expect("failed to fill pair lattice canvas");

    let max_index = unit_digits.len() as i32;
    let x_labels = unit_symbols.to_vec();
    let y_labels = unit_symbols.to_vec();
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Base {base} Pair Lattice  (species color, size = active anomaly mass)"),
            ("sans-serif", 26),
        )
        .margin(24)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0i32..max_index, 0i32..max_index)
        .expect("failed to build pair lattice chart");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(unit_symbols.len())
        .y_labels(unit_symbols.len())
        .x_desc("outer residue")
        .y_desc("inner residue")
        .x_label_formatter(&move |value| {
            x_labels
                .get((*value).clamp(0, max_index - 1) as usize)
                .cloned()
                .unwrap_or_default()
        })
        .y_label_formatter(&move |value| {
            y_labels
                .get((*value).clamp(0, max_index - 1) as usize)
                .cloned()
                .unwrap_or_default()
        })
        .axis_style(RGBColor(90, 84, 76))
        .label_style(("sans-serif", 18))
        .draw()
        .expect("failed to draw pair lattice mesh");

    chart
        .draw_series(
            (0..max_index)
                .flat_map(|x| (0..max_index).map(move |y| (x, y)))
                .map(|(x, y)| {
                    Circle::new(
                        (x, y),
                        3,
                        ShapeStyle::from(&RGBColor(224, 219, 212)).filled(),
                    )
                }),
        )
        .expect("failed to draw pair lattice background");

    for row in rows {
        let outer_index = unit_digits
            .iter()
            .position(|&digit| digit == row.outer)
            .expect("outer digit should be in unit residues") as i32;
        let inner_index = unit_digits
            .iter()
            .position(|&digit| digit == row.inner)
            .expect("inner digit should be in unit residues") as i32;
        let radius = (4.0 + (display_mass(row) / 1.5).min(12.0)).round() as i32;
        let point_color = species_color(&row.species);
        let fill_style = ShapeStyle {
            color: point_color.to_rgba(),
            filled: true,
            stroke_width: 1,
        };

        chart
            .draw_series(std::iter::once(Circle::new(
                (outer_index, inner_index),
                radius,
                fill_style,
            )))
            .expect("failed to draw lattice point");

        if row.same_digit {
            chart
                .draw_series(std::iter::once(Circle::new(
                    (outer_index, inner_index),
                    radius + 2,
                    ShapeStyle::from(&BLACK).stroke_width(2),
                )))
                .expect("failed to draw same-digit outline");
        }
    }

    root.present()
        .expect("failed to present pair lattice image");
}

fn collect_heatmap_rows(input_bundle: &InputBundle) -> Vec<HeatmapCellRow> {
    let mut rows = input_bundle
        .species_modulus_rows
        .iter()
        .filter(|row| row.middle_length == HEATMAP_MIDDLE_LENGTH)
        .filter(|row| SPECIES_ORDER.contains(&row.species.as_str()))
        .map(|row| HeatmapCellRow {
            species: row.species.clone(),
            modulus: row.modulus,
            positive_relief_share: row.positive_relief_share,
            mean_relief_pp: row.mean_relief_pp,
            median_relief_pp: row.median_relief_pp,
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        species_order_index(&left.species)
            .cmp(&species_order_index(&right.species))
            .then_with(|| left.modulus.cmp(&right.modulus))
    });
    rows
}

fn render_species_modulus_heatmap(rows: &[HeatmapCellRow], path: &Path) {
    let moduli = unique_sorted_u32(rows.iter().map(|row| row.modulus).collect());
    let root = BitMapBackend::new(path, (1100, 420)).into_drawing_area();
    root.fill(&RGBColor(250, 248, 244))
        .expect("failed to fill heatmap canvas");

    let x_labels = moduli.iter().map(u32::to_string).collect::<Vec<_>>();
    let y_labels = SPECIES_ORDER
        .iter()
        .map(|&label| label.to_string())
        .collect::<Vec<_>>();
    let max_x = moduli.len() as i32;
    let max_y = SPECIES_ORDER.len() as i32;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "M=2 Species-Modulus Relief Heatmap  (color = positive relief share)",
            ("sans-serif", 26),
        )
        .margin(24)
        .x_label_area_size(50)
        .y_label_area_size(110)
        .build_cartesian_2d(0i32..max_x, 0i32..max_y)
        .expect("failed to build heatmap chart");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(moduli.len())
        .y_labels(SPECIES_ORDER.len())
        .x_desc("small prime modulus")
        .y_desc("species")
        .x_label_formatter(&move |value| {
            x_labels
                .get((*value).clamp(0, max_x - 1) as usize)
                .cloned()
                .unwrap_or_default()
        })
        .y_label_formatter(&move |value| {
            y_labels
                .get((*value).clamp(0, max_y - 1) as usize)
                .cloned()
                .unwrap_or_default()
        })
        .axis_style(RGBColor(90, 84, 76))
        .label_style(("sans-serif", 18))
        .draw()
        .expect("failed to draw heatmap mesh");

    let mut row_lookup = BTreeMap::new();
    for row in rows {
        row_lookup.insert((row.species.clone(), row.modulus), row);
    }

    for (y_index, &species) in SPECIES_ORDER.iter().enumerate() {
        for (x_index, &modulus) in moduli.iter().enumerate() {
            let row = row_lookup
                .get(&(species.to_string(), modulus))
                .expect("expected full species-modulus grid");
            let cell_color = heatmap_color(row.positive_relief_share);
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [
                        (x_index as i32, y_index as i32),
                        (x_index as i32 + 1, y_index as i32 + 1),
                    ],
                    ShapeStyle::from(&cell_color).filled(),
                )))
                .expect("failed to draw heatmap cell");

            let label = if row.positive_relief_share > 0.0 {
                format!("{:.0}%", row.positive_relief_share * 100.0)
            } else {
                "0".to_string()
            };
            chart
                .draw_series(std::iter::once(Text::new(
                    label,
                    (x_index as i32, y_index as i32),
                    ("sans-serif", 18).into_font().color(&BLACK),
                )))
                .expect("failed to draw heatmap label");
        }
    }

    root.present().expect("failed to present heatmap image");
}

fn species_order_index(species: &str) -> usize {
    SPECIES_ORDER
        .iter()
        .position(|&value| value == species)
        .unwrap_or(SPECIES_ORDER.len())
}

fn display_mass(row: &SpeciesPairRow) -> f64 {
    match row.species.as_str() {
        "m1_only" => row.anomaly_m1_pp,
        _ => row.anomaly_m2_pp,
    }
}

fn species_color(species: &str) -> RGBColor {
    match species {
        "m1_only" => RGBColor(117, 122, 140),
        "m1_to_m2" => RGBColor(210, 99, 34),
        "m2_only" => RGBColor(23, 133, 123),
        _ => RGBColor(80, 80, 80),
    }
}

fn heatmap_color(share: f64) -> RGBColor {
    let share = share.clamp(0.0, 1.0);
    let start = (244.0, 240.0, 230.0);
    let end = (31.0, 76.0, 122.0);
    RGBColor(
        lerp(start.0, end.0, share) as u8,
        lerp(start.1, end.1, share) as u8,
        lerp(start.2, end.2, share) as u8,
    )
}

fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

fn derive_observations(input_bundle: &InputBundle) -> Vec<String> {
    let summary_by_species = input_bundle
        .species_summary_rows
        .iter()
        .map(|row| (row.species.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    let persistent = summary_by_species
        .get("m1_to_m2")
        .expect("m1_to_m2 summary should exist");
    let emergent = summary_by_species
        .get("m2_only")
        .expect("m2_only summary should exist");
    let m1_only = summary_by_species
        .get("m1_only")
        .expect("m1_only summary should exist");

    vec![
        format!(
            "The pair lattices should show the persistent species hugging the tight part of pair-space: `m1_to_m2` has gap mix {}/{}/{} (same/adjacent/wide), while `m2_only` has {}/{}/{}.",
            persistent.same_gap_count,
            persistent.adjacent_gap_count,
            persistent.wide_gap_count,
            emergent.same_gap_count,
            emergent.adjacent_gap_count,
            emergent.wide_gap_count
        ),
        format!(
            "The heatmap should show `m2_only` as the more shared M=2 residue-relief species: its top M=2 moduli are `{}`, while `m1_to_m2` is sparser at `{}`.",
            emergent.top_moduli_m2, persistent.top_moduli_m2
        ),
        format!(
            "The faded background species `m1_only` is still the biggest cloud ({} pairs), but geometrically it should look more diffuse and wider than the compact persistent lane.",
            m1_only.pair_count
        ),
    ]
}

fn unique_sorted_u32(mut values: Vec<u32>) -> Vec<u32> {
    values.sort_unstable();
    values.dedup();
    values
}

fn print_summary(bundle: &OutputBundle) {
    println!("=== M=2 Species Geometry Report ===\n");
    println!(
        "Input {} | output {}",
        bundle.input_json, bundle.pair_catalog_mode
    );
    println!(
        "Pair lattice images: {} | heatmap rows: {}",
        bundle.lattice_base_rows.len(),
        bundle.heatmap_rows.len()
    );
    for row in &bundle.lattice_base_rows {
        println!(
            "  - base {:>2}: {} points | species {} / {} / {} | image {}",
            row.base,
            row.pair_count,
            row.m1_only_pairs,
            row.m1_to_m2_pairs,
            row.m2_only_pairs,
            row.image_path
        );
    }
    println!(
        "  - heatmap: {}",
        bundle
            .image_artifact_rows
            .iter()
            .find(|row| row.kind == "species_modulus_heatmap")
            .map(|row| row.path.clone())
            .unwrap_or_else(|| "missing".to_string())
    );
}

fn render_markdown_report(bundle: &OutputBundle) -> String {
    let mut lines = vec![
        "# M=2 Species Geometry Report".to_string(),
        String::new(),
        "_Generated from `examples/m2_species_geometry_report.rs`._".to_string(),
        String::new(),
        format!("- Generated at: `{}`", bundle.generated_at_utc),
        format!("- Input JSON: `{}`", bundle.input_json),
        format!("- Input generated at: `{}`", bundle.input_generated_at_utc),
        format!("- Pair catalog mode: `{}`", bundle.pair_catalog_mode),
        String::new(),
        "## What To Notice".to_string(),
        String::new(),
    ];
    for observation in &bundle.observations {
        lines.push(format!("- {}", observation));
    }

    lines.extend([
        String::new(),
        "## Image Artifacts".to_string(),
        String::new(),
        "| Kind | Label | Path |".to_string(),
        "|---|---|---|".to_string(),
    ]);
    for row in &bundle.image_artifact_rows {
        lines.push(format!(
            "| `{}` | {} | `{}` |",
            row.kind, row.label, row.path
        ));
    }

    lines.extend([
        String::new(),
        "## Base Lattice Rows".to_string(),
        String::new(),
        "| Base | Pairs | m1_only | m1_to_m2 | m2_only | Strongest pair | Strongest display mass |"
            .to_string(),
        "|---:|---:|---:|---:|---:|---|---:|".to_string(),
    ]);
    for row in &bundle.lattice_base_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{:.2}pp` |",
            row.base,
            row.pair_count,
            row.m1_only_pairs,
            row.m1_to_m2_pairs,
            row.m2_only_pairs,
            row.strongest_pair,
            row.strongest_display_mass_pp
        ));
    }

    lines.join("\n")
}
