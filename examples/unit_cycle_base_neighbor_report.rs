//! Unit-cycle base-neighbor report.
//!
//! This report compares nearby bases as normalized unit-cycle geometries and
//! uses base 57 versus base 58 as the teaching pair.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example unit_cycle_base_neighbor_report -- --out-dir /tmp/primes_unit_cycle_base_neighbor
//! ```

use plotters::coord::Shift;
use plotters::prelude::*;
use primes::validation::{
    bounded_k::{digit_symbol, unit_residues},
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
    unit_cycle_neighbors::{
        build_unit_cycle_base_neighbor_report, UnitCycleBaseGeometryRow,
        UnitCycleBaseNeighborReport, UnitCycleBaseNeighborSettings, UnitCycleNeighborPhaseRow,
        DEFAULT_UNIT_CYCLE_NEIGHBOR_FOCUS_MIDDLE_LENGTH,
        DEFAULT_UNIT_CYCLE_NEIGHBOR_SCAN_MIDDLE_LENGTH, DEFAULT_UNIT_CYCLE_NEIGHBOR_TOP_LIMIT,
    },
    unit_cycle_phase::normalize_unit_cycle_geometry,
};
use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_unit_cycle_base_neighbor";
const ARTIFACT_ID: &str = "unit_cycle_base_neighbor_report";
const EXPORT_VERSION: u32 = 1;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    settings: UnitCycleBaseNeighborSettings,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    report: UnitCycleBaseNeighborReport,
    image_artifact_rows: Vec<ImageArtifactRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output directory");

    let report = build_unit_cycle_base_neighbor_report(options.settings.clone());
    let cycle_path = options.out_dir.join("base57_base58_unit_cycles.png");
    render_base57_base58_cycles(&cycle_path);
    let density_path = options.out_dir.join("unit_count_delta_strip.png");
    render_unit_count_delta_strip(&report.base_geometry_rows, &density_path);
    let lead_path = options.out_dir.join("top_neighbor_phase_leads.png");
    render_top_neighbor_phase_leads(&report.top_phase_rows, &lead_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "base57_base58_unit_cycles".to_string(),
            label: "Base 57 / base 58 unit-cycle comparison".to_string(),
            path: cycle_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "unit_count_delta_strip".to_string(),
            label: "Neighbor unit-count and adjacent-chord strip".to_string(),
            path: density_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "top_neighbor_phase_leads".to_string(),
            label: "Top exact neighbor phase leads".to_string(),
            path: lead_path.display().to_string(),
        },
    ];

    write_csv_rows(
        options.out_dir.join("base_geometry_rows.csv"),
        &report.base_geometry_rows,
    )
    .expect("write base geometry rows");
    write_csv_rows(
        options.out_dir.join("neighbor_delta_rows.csv"),
        &report.neighbor_delta_rows,
    )
    .expect("write neighbor delta rows");
    write_csv_rows(
        options.out_dir.join("scan_phase_rows.csv"),
        &report.scan_phase_rows,
    )
    .expect("write scan phase rows");
    write_csv_rows(
        options.out_dir.join("top_phase_rows.csv"),
        &report.top_phase_rows,
    )
    .expect("write top phase rows");
    write_csv_rows(
        options.out_dir.join("focus_phase_rows.csv"),
        &report.focus_phase_rows,
    )
    .expect("write focus phase rows");
    write_csv_rows(
        options.out_dir.join("image_artifact_rows.csv"),
        &image_artifact_rows,
    )
    .expect("write image artifact rows");

    let bundle = ReportBundle {
        export_version: EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        report,
        image_artifact_rows,
    };
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_text_file(options.out_dir.join("report.md"), &render_report(&bundle))
        .expect("write report markdown");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo run --release --example unit_cycle_base_neighbor_report"
                .to_string(),
            args: env::args().skip(1).collect(),
            upstream_inputs: vec![
                "src/validation/unit_cycle_neighbors.rs".to_string(),
                "src/validation/unit_cycle_phase.rs".to_string(),
                "src/validation/affine_phase_residual.rs".to_string(),
                "src/validation/bounded_k.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "base_geometry_rows.csv".to_string(),
                "neighbor_delta_rows.csv".to_string(),
                "scan_phase_rows.csv".to_string(),
                "top_phase_rows.csv".to_string(),
                "focus_phase_rows.csv".to_string(),
                "image_artifact_rows.csv".to_string(),
                "base57_base58_unit_cycles.png".to_string(),
                "unit_count_delta_strip.png".to_string(),
                "top_neighbor_phase_leads.png".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote unit-cycle base-neighbor report to {}",
        options.out_dir.display()
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut settings = UnitCycleBaseNeighborSettings::default();
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir")),
            "--bases" => {
                settings.bases = parse_next::<String>(&mut args, "--bases")
                    .split(',')
                    .map(|part| {
                        part.parse::<u32>().unwrap_or_else(|_| {
                            eprintln!("invalid base in --bases: {part}");
                            std::process::exit(2);
                        })
                    })
                    .collect();
            }
            "--scan-middle-length" => {
                settings.scan_middle_length = parse_next(&mut args, "--scan-middle-length");
            }
            "--focus-middle-length" => {
                settings.focus_middle_length = parse_next(&mut args, "--focus-middle-length");
            }
            "--top-limit" => settings.top_limit = parse_next(&mut args, "--top-limit"),
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                eprintln!("unknown argument: {arg}");
                print_help();
                std::process::exit(2);
            }
        }
    }

    if settings.bases.is_empty() {
        eprintln!("--bases must contain at least one base");
        std::process::exit(2);
    }
    if settings.scan_middle_length == 0 || settings.focus_middle_length == 0 {
        eprintln!("middle lengths must be positive");
        std::process::exit(2);
    }
    if settings.top_limit == 0 {
        eprintln!("--top-limit must be positive");
        std::process::exit(2);
    }

    Options { out_dir, settings }
}

fn parse_next<T: std::str::FromStr>(args: &mut impl Iterator<Item = String>, flag: &str) -> T {
    args.next()
        .unwrap_or_else(|| {
            eprintln!("missing value for {flag}");
            std::process::exit(2);
        })
        .parse::<T>()
        .unwrap_or_else(|_| {
            eprintln!("invalid value for {flag}");
            std::process::exit(2);
        })
}

fn print_help() {
    println!(
        "Usage: cargo run --release --example unit_cycle_base_neighbor_report -- \\
  --out-dir {DEFAULT_OUT_DIR} \\
  [--bases 56,57,58,59,60] \\
  [--scan-middle-length {DEFAULT_UNIT_CYCLE_NEIGHBOR_SCAN_MIDDLE_LENGTH}] \\
  [--focus-middle-length {DEFAULT_UNIT_CYCLE_NEIGHBOR_FOCUS_MIDDLE_LENGTH}] \\
  [--top-limit {DEFAULT_UNIT_CYCLE_NEIGHBOR_TOP_LIMIT}]"
    );
}

fn render_report(bundle: &ReportBundle) -> String {
    let report = &bundle.report;
    let summary = &report.summary;
    let mut lines = Vec::new();
    lines.push("# Unit-Cycle Base-Neighbor Report".to_string());
    lines.push(String::new());
    lines.push(summary.strong_line.clone());
    lines.push(summary.caution_line.clone());
    lines.push(String::new());
    lines.push("## Campfire Setup".to_string());
    lines.push("The drawing uses a normalized radius-1 circle. The circumference is always `2*pi`; what changes from base to base is how many unit digits, or coprime boundary digits, get placed around that same circle.".to_string());
    lines.push(String::new());
    lines.push("That means base `57` and base `58` are not different-sized circles in this report. They are same-sized explanatory circles with different bead counts, different bead spacing, and different diameter pairs.".to_string());
    lines.push(String::new());
    lines.push("```text".to_string());
    lines.push("base -> unit digits -> normalized circle beads -> diameter/complement geometry -> shift-phase residue weather".to_string());
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push("## Headline".to_string());
    lines.push(format!(
        "- Bases scanned: `{}`",
        report
            .settings
            .bases
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    ));
    lines.push(format!(
        "- Broad exact phase scan: `M={}` with `{}` unordered rows",
        summary.scan_middle_length, summary.scan_phase_row_count
    ));
    lines.push(format!(
        "- Focus exact teaching rows: `M={}` with `{}` diameter/complement rows",
        summary.focus_middle_length, summary.focus_phase_row_count
    ));
    lines.push(format!(
        "- Base 57 has `{}` unit beads; base 58 has `{}` unit beads",
        summary.base57_unit_count, summary.base58_unit_count
    ));
    lines.push(format!(
        "- Adjacent chord length: base 57 `{:.6}`, base 58 `{:.6}`",
        summary.base57_adjacent_chord_length, summary.base58_adjacent_chord_length
    ));
    lines.push(format!(
        "- Strongest broad survivor residual: base `{}` `{}` vs `{}` at `{:+.3}` pp",
        summary.strongest_scan_base,
        summary.strongest_scan_pair,
        summary.strongest_scan_reverse_pair,
        summary.strongest_scan_survivor_residual_pp
    ));
    lines.push(String::new());
    lines.push("## Base Geometry".to_string());
    lines.push("| base | factors | units | unit arc | adjacent chord | diameter samples | complement samples |".to_string());
    lines.push("|---:|---|---:|---:|---:|---|---|".to_string());
    for row in &report.base_geometry_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{:.4}` | `{:.4}` | `{}` | `{}` |",
            row.base,
            row.factorization_label,
            row.unit_count,
            row.unit_arc_length,
            row.adjacent_chord_length,
            row.sample_diameter_pairs_label,
            row.sample_complement_pairs_label
        ));
    }
    lines.push(String::new());
    lines.push("## Neighbor Deltas".to_string());
    lines.push("| neighbor | unit delta | chord delta | interpretation |".to_string());
    lines.push("|---|---:|---:|---|".to_string());
    for row in &report.neighbor_delta_rows {
        lines.push(format!(
            "| `{} -> {}` | `{:+}` | `{:+.6}` | {} |",
            row.left_base,
            row.right_base,
            row.unit_count_delta,
            row.adjacent_chord_delta,
            row.interpretation
        ));
    }
    lines.push(String::new());
    lines.push("## Focus Geometry Rows".to_string());
    lines.push(
        "| reason | base | pair | bucket | survivor residual | hits | witnesses |".to_string(),
    );
    lines.push("|---|---:|---|---|---:|---|---|".to_string());
    for row in &report.focus_phase_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` vs `{}` | `{}/{}/{}` | `{:+.3}` pp | `{}/{}` | `{}` / `{}` |",
            row.selection_reason,
            row.base,
            row.pair_label,
            row.reverse_pair_label,
            row.distance_label,
            row.low_to_high_arc_label,
            row.edge_label,
            row.survivor_prime_residual_delta_pp,
            row.low_high_prime_hits,
            row.high_low_prime_hits,
            row.first_low_high_witness,
            row.first_high_low_witness
        ));
    }
    lines.push(String::new());
    lines.push("## Top Broad Phase Leads".to_string());
    lines
        .push("| base | pair | bucket | survivor residual | raw-size residual | tag |".to_string());
    lines.push("|---:|---|---|---:|---:|---|".to_string());
    for row in &report.top_phase_rows {
        lines.push(format!(
            "| `{}` | `{}` vs `{}` | `{}/{}/{}` | `{:+.3}` pp | `{:+.3}` pp | `{}` |",
            row.base,
            row.pair_label,
            row.reverse_pair_label,
            row.distance_label,
            row.low_to_high_arc_label,
            row.edge_label,
            row.survivor_prime_residual_delta_pp,
            row.residual_after_size_pp,
            row.lead_tag
        ));
    }
    lines.push(String::new());
    lines.push("## Base58 Payload Side Note".to_string());
    lines.push(summary.payload_note.clone());
    lines.push("A lossless arbitrary payload conversion from base 58 to base 57 still has to preserve the underlying integer or byte string. A streaming transducer could hide the decode/re-encode boundary by carrying remainders forward, but mathematically it is still changing radix. The genuinely different move would be to generate identifiers directly inside a base-57 affine/residue grammar so the candidate is never an arbitrary base58 payload in the first place.".to_string());
    lines.push(String::new());
    lines.push("## Visuals".to_string());
    for image in &bundle.image_artifact_rows {
        lines.push(format!("- `{}`: `{}`", image.label, image.path));
    }
    lines.push(String::new());
    lines.push("## Claim Boundary".to_string());
    lines.push("This report treats unit-cycle geometry as a search projection. It can point to useful phase leads, but it is not a density theorem and it is not a shortcut for arbitrary base-encoded payload conversion.".to_string());
    lines.push(String::new());
    lines.join("\n")
}

fn render_base57_base58_cycles(path: &Path) {
    let root = BitMapBackend::new(path, (1500, 760)).into_drawing_area();
    root.fill(&WHITE).expect("fill base cycles");
    root.draw(&Text::new(
        "Base 57 vs Base 58: Same Radius, Different Unit Beads",
        (42, 46),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");
    root.draw(&Text::new(
        "blue chord = first diameter pair, orange chord = base-complement edge pair",
        (42, 84),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw subtitle");
    draw_cycle_card(&root, 95, 130, 57, (1, 29), (1, 56));
    draw_cycle_card(&root, 805, 130, 58, (1, 31), (1, 57));
    root.present().expect("present base cycles");
}

fn draw_cycle_card(
    root: &DrawingArea<BitMapBackend<'_>, Shift>,
    x: i32,
    y: i32,
    base: u32,
    diameter_pair: (u32, u32),
    complement_pair: (u32, u32),
) {
    let units = unit_residues(base);
    let center = (x + 300, y + 270);
    let radius = 190.0;
    root.draw(&Rectangle::new(
        [(x, y), (x + 600, y + 540)],
        RGBColor(239, 247, 244).filled(),
    ))
    .expect("draw card fill");
    root.draw(&Rectangle::new(
        [(x, y), (x + 600, y + 540)],
        ShapeStyle::from(&RGBColor(80, 88, 96)).stroke_width(1),
    ))
    .expect("draw card border");
    root.draw(&Text::new(
        format!(
            "base {base}: {} unit digits, adjacent chord {:.4}",
            units.len(),
            2.0 * (std::f64::consts::PI / units.len() as f64).sin()
        ),
        (x + 24, y + 36),
        ("sans-serif", 20).into_font().style(FontStyle::Bold),
    ))
    .expect("draw card title");
    root.draw(&Circle::new(
        center,
        radius as i32,
        ShapeStyle::from(&RGBColor(98, 105, 112)).stroke_width(2),
    ))
    .expect("draw cycle");
    for (idx, &unit) in units.iter().enumerate() {
        let angle = angle_for_index(idx, units.len());
        let point = (
            center.0 + (radius * angle.cos()) as i32,
            center.1 + (radius * angle.sin()) as i32,
        );
        let dot_radius = if unit == 1 || unit == base - 1 { 5 } else { 3 };
        root.draw(&Circle::new(
            point,
            dot_radius,
            RGBColor(105, 115, 125).filled(),
        ))
        .expect("draw unit dot");
    }
    draw_pair_chord(
        root,
        center,
        radius,
        base,
        diameter_pair,
        RGBColor(39, 111, 178),
    );
    draw_pair_chord(
        root,
        center,
        radius,
        base,
        complement_pair,
        RGBColor(205, 95, 70),
    );
    let diameter_geometry = normalize_unit_cycle_geometry(base, diameter_pair.0, diameter_pair.1);
    let complement_geometry =
        normalize_unit_cycle_geometry(base, complement_pair.0, complement_pair.1);
    root.draw(&Text::new(
        format!(
            "diameter {}: {} unit steps",
            pair_label(diameter_pair),
            diameter_geometry.cyclic_distance
        ),
        (x + 24, y + 485),
        ("sans-serif", 17).into_font(),
    ))
    .expect("draw diameter label");
    root.draw(&Text::new(
        format!(
            "complement {}: {} / {}",
            pair_label(complement_pair),
            complement_geometry.distance_label,
            complement_geometry.low_to_high_arc_label
        ),
        (x + 300, y + 485),
        ("sans-serif", 17).into_font(),
    ))
    .expect("draw complement label");
}

fn draw_pair_chord(
    root: &DrawingArea<BitMapBackend<'_>, Shift>,
    center: (i32, i32),
    radius: f64,
    base: u32,
    pair: (u32, u32),
    color: RGBColor,
) {
    let units = unit_residues(base);
    let left_idx = units
        .iter()
        .position(|&unit| unit == pair.0)
        .expect("pair left should be a unit");
    let right_idx = units
        .iter()
        .position(|&unit| unit == pair.1)
        .expect("pair right should be a unit");
    let left_angle = angle_for_index(left_idx, units.len());
    let right_angle = angle_for_index(right_idx, units.len());
    let left = (
        center.0 + (radius * left_angle.cos()) as i32,
        center.1 + (radius * left_angle.sin()) as i32,
    );
    let right = (
        center.0 + (radius * right_angle.cos()) as i32,
        center.1 + (radius * right_angle.sin()) as i32,
    );
    root.draw(&PathElement::new(
        vec![left, right],
        ShapeStyle::from(&color.mix(0.58)).stroke_width(5),
    ))
    .expect("draw chord");
    root.draw(&Circle::new(left, 8, color.filled()))
        .expect("draw left endpoint");
    root.draw(&Circle::new(right, 8, color.filled()))
        .expect("draw right endpoint");
}

fn render_unit_count_delta_strip(rows: &[UnitCycleBaseGeometryRow], path: &Path) {
    let root = BitMapBackend::new(path, (1400, 780)).into_drawing_area();
    root.fill(&WHITE).expect("fill unit strip");
    let max_units = rows.iter().map(|row| row.unit_count).max().unwrap_or(1) as f64;
    root.draw(&Text::new(
        "Neighbor Bases: Unit Beads and Adjacent Chord Length",
        (42, 46),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw strip title");
    for (idx, row) in rows.iter().enumerate() {
        let y = 120 + idx as i32 * 115;
        let bar_w = (760.0 * row.unit_count as f64 / max_units) as i32;
        root.draw(&Text::new(
            format!("base {} = {}", row.base, row.factorization_label),
            (58, y + 26),
            ("sans-serif", 18).into_font().style(FontStyle::Bold),
        ))
        .expect("draw base label");
        root.draw(&Rectangle::new(
            [(300, y), (300 + bar_w, y + 38)],
            RGBColor(66, 132, 178).filled(),
        ))
        .expect("draw unit bar");
        root.draw(&Text::new(
            format!(
                "{} units, arc {:.4}, chord {:.4}",
                row.unit_count, row.unit_arc_length, row.adjacent_chord_length
            ),
            (300 + bar_w + 18, y + 26),
            ("sans-serif", 17).into_font(),
        ))
        .expect("draw unit stats");
        root.draw(&Text::new(
            format!("diameter samples: {}", row.sample_diameter_pairs_label),
            (300, y + 72),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw diameter samples");
    }
    root.present().expect("present unit strip");
}

fn render_top_neighbor_phase_leads(rows: &[UnitCycleNeighborPhaseRow], path: &Path) {
    let selected = rows.iter().take(12).collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (1500, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill lead chart");
    root.draw(&Text::new(
        "Top Exact Neighbor Phase Leads",
        (42, 46),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw lead title");
    root.draw(&Text::new(
        "exact compact M=2 scan; signed bars show survivor-prime residual",
        (42, 84),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw lead subtitle");
    let max_abs = selected
        .iter()
        .map(|row| row.survivor_prime_residual_delta_pp.abs())
        .fold(0.0_f64, f64::max)
        .max(0.01);
    let x_zero = 760;
    let scale = 560.0 / max_abs;
    root.draw(&PathElement::new(
        vec![(x_zero, 130), (x_zero, 900)],
        ShapeStyle::from(&RGBColor(135, 135, 135)).stroke_width(2),
    ))
    .expect("draw zero axis");
    for (idx, row) in selected.iter().enumerate() {
        let y = 155 + idx as i32 * 60;
        let x = x_zero + (row.survivor_prime_residual_delta_pp * scale) as i32;
        root.draw(&Text::new(
            format!("b{} {} {}", row.base, row.pair_label, row.edge_label),
            (44, y + 5),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw lead label");
        root.draw(&PathElement::new(
            vec![(x_zero, y), (x, y)],
            ShapeStyle::from(&signed_color(row.survivor_prime_residual_delta_pp)).stroke_width(7),
        ))
        .expect("draw lead bar");
        root.draw(&Circle::new(
            (x, y),
            8,
            signed_color(row.survivor_prime_residual_delta_pp).filled(),
        ))
        .expect("draw lead point");
        root.draw(&Text::new(
            format!(
                "{:+.3} pp  hits {}/{}",
                row.survivor_prime_residual_delta_pp,
                row.low_high_prime_hits,
                row.high_low_prime_hits
            ),
            (1120, y + 5),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw lead value");
    }
    root.present().expect("present lead chart");
}

fn angle_for_index(index: usize, unit_count: usize) -> f64 {
    -std::f64::consts::FRAC_PI_2 + 2.0 * std::f64::consts::PI * index as f64 / unit_count as f64
}

fn pair_label(pair: (u32, u32)) -> String {
    format!(
        "({},{})",
        readable_digit_label(pair.0),
        readable_digit_label(pair.1)
    )
}

fn readable_digit_label(digit: u32) -> String {
    if digit < 36 {
        digit_symbol(digit)
    } else {
        format!("[{digit}]")
    }
}

fn signed_color(value: f64) -> RGBColor {
    if value >= 0.0 {
        RGBColor(205, 95, 70)
    } else {
        RGBColor(55, 115, 185)
    }
}
