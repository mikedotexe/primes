//! Unit-cycle phase signal report.
//!
//! This report translates compact shift-phase residuals into base-normalized
//! unit-cycle geometry so cross-base signals can be compared as structural arc
//! patterns rather than base-specific digit anecdotes.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example unit_cycle_phase_signal_report -- --out-dir /tmp/primes_unit_cycle_phase_signal
//! ```

use plotters::prelude::*;
use primes::validation::{
    affine_phase_residual::{
        DEFAULT_PHASE_RESIDUAL_BASES, DEFAULT_PHASE_RESIDUAL_MAX_MIDDLE_LENGTH,
        DEFAULT_PHASE_RESIDUAL_MIN_MIDDLE_LENGTH, DEFAULT_SHIFT_PHASE_FOLLOWUP_MIDDLE_LENGTH,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
    unit_cycle_phase::{
        build_unit_cycle_phase_signal_report, normalize_unit_cycle_geometry, UnitCycleBucketRow,
        UnitCycleLeadRow, UnitCycleMaturityRow, UnitCyclePhaseRow, UnitCyclePhaseSettings,
        UnitCyclePhaseSignalReport, DEFAULT_UNIT_CYCLE_PAIR_TOP_LIMIT,
        DEFAULT_UNIT_CYCLE_REPRESENTATIVES_PER_BUCKET, DEFAULT_UNIT_CYCLE_TOP_BUCKET_LIMIT,
        DEFAULT_UNIT_CYCLE_WITNESS_LIMIT,
    },
};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_unit_cycle_phase_signal";
const ARTIFACT_ID: &str = "unit_cycle_phase_signal_report";
const EXPORT_VERSION: u32 = 1;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    settings: UnitCyclePhaseSettings,
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
    report: UnitCyclePhaseSignalReport,
    image_artifact_rows: Vec<ImageArtifactRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output directory");

    let report = build_unit_cycle_phase_signal_report(options.settings.clone());
    let arc_map_path = options.out_dir.join("unit_cycle_arc_map.png");
    render_unit_cycle_arc_map(&report.lead_rows, &arc_map_path);
    let heatmap_path = options.out_dir.join("cycle_bucket_heatmap.png");
    render_cycle_bucket_heatmap(&report.cycle_bucket_rows, &heatmap_path);
    let scatter_path = options.out_dir.join("distance_residual_scatter.png");
    render_distance_residual_scatter(
        &report.unit_cycle_phase_rows,
        options.settings.base_settings.max_middle_length,
        &scatter_path,
    );
    let edge_path = options.out_dir.join("edge_wrap_panel.png");
    render_edge_wrap_panel(
        &report.unit_cycle_phase_rows,
        options.settings.base_settings.max_middle_length,
        &edge_path,
    );
    let maturity_path = options.out_dir.join("maturity_strip.png");
    render_maturity_strip(&report.maturity_rows, &maturity_path);
    let gallery_path = options.out_dir.join("lead_gallery.png");
    render_lead_gallery(&report.lead_rows, &report.maturity_rows, &gallery_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "unit_cycle_arc_map".to_string(),
            label: "Unit-cycle arc map".to_string(),
            path: arc_map_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "cycle_bucket_heatmap".to_string(),
            label: "Cycle bucket heatmap".to_string(),
            path: heatmap_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "distance_residual_scatter".to_string(),
            label: "Distance/residual scatter".to_string(),
            path: scatter_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "edge_wrap_panel".to_string(),
            label: "Edge/wrap panel".to_string(),
            path: edge_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "maturity_strip".to_string(),
            label: "Maturity strip".to_string(),
            path: maturity_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "lead_gallery".to_string(),
            label: "Lead gallery".to_string(),
            path: gallery_path.display().to_string(),
        },
    ];

    write_csv_rows(
        options.out_dir.join("unit_cycle_phase_rows.csv"),
        &report.unit_cycle_phase_rows,
    )
    .expect("write unit-cycle phase rows");
    write_csv_rows(
        options.out_dir.join("cycle_bucket_rows.csv"),
        &report.cycle_bucket_rows,
    )
    .expect("write cycle bucket rows");
    write_csv_rows(options.out_dir.join("lead_rows.csv"), &report.lead_rows)
        .expect("write lead rows");
    write_csv_rows(
        options.out_dir.join("maturity_rows.csv"),
        &report.maturity_rows,
    )
    .expect("write maturity rows");
    write_csv_rows(options.out_dir.join("foil_rows.csv"), &report.foil_rows)
        .expect("write foil rows");
    write_csv_rows(
        options.out_dir.join("witness_rows.csv"),
        &report.witness_rows,
    )
    .expect("write witness rows");
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
            generator_cmd: "cargo run --release --example unit_cycle_phase_signal_report"
                .to_string(),
            args: env::args().skip(1).collect(),
            upstream_inputs: vec![
                "src/validation/unit_cycle_phase.rs".to_string(),
                "src/validation/affine_phase_residual.rs".to_string(),
                "src/validation/bounded_k.rs".to_string(),
                "src/validation/fast_affine.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "unit_cycle_phase_rows.csv".to_string(),
                "cycle_bucket_rows.csv".to_string(),
                "lead_rows.csv".to_string(),
                "maturity_rows.csv".to_string(),
                "foil_rows.csv".to_string(),
                "witness_rows.csv".to_string(),
                "image_artifact_rows.csv".to_string(),
                "unit_cycle_arc_map.png".to_string(),
                "cycle_bucket_heatmap.png".to_string(),
                "distance_residual_scatter.png".to_string(),
                "edge_wrap_panel.png".to_string(),
                "maturity_strip.png".to_string(),
                "lead_gallery.png".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote unit-cycle phase signal report to {}",
        options.out_dir.display()
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut settings = UnitCyclePhaseSettings::default();
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir")),
            "--bases" => {
                settings.base_settings.bases = parse_next::<String>(&mut args, "--bases")
                    .split(',')
                    .map(|part| {
                        part.parse::<u32>().unwrap_or_else(|_| {
                            eprintln!("invalid base in --bases: {part}");
                            std::process::exit(2);
                        })
                    })
                    .collect();
            }
            "--min-middle-length" => {
                settings.base_settings.min_middle_length =
                    parse_next(&mut args, "--min-middle-length");
            }
            "--max-middle-length" => {
                settings.base_settings.max_middle_length =
                    parse_next(&mut args, "--max-middle-length");
            }
            "--followup-middle-length" => {
                settings.followup_middle_length = parse_next(&mut args, "--followup-middle-length");
            }
            "--top-bucket-limit" => {
                settings.top_bucket_limit = parse_next(&mut args, "--top-bucket-limit");
            }
            "--representatives-per-bucket" => {
                settings.representatives_per_bucket =
                    parse_next(&mut args, "--representatives-per-bucket");
            }
            "--pair-top-limit" => {
                settings.pair_top_limit = parse_next(&mut args, "--pair-top-limit");
            }
            "--witness-limit" => {
                settings.witness_limit = parse_next(&mut args, "--witness-limit");
            }
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

    if settings.base_settings.bases.is_empty() {
        eprintln!("--bases must contain at least one base");
        std::process::exit(2);
    }
    if settings.base_settings.min_middle_length == 0
        || settings.base_settings.max_middle_length < settings.base_settings.min_middle_length
    {
        eprintln!("middle length range must be nonempty and start at least at 1");
        std::process::exit(2);
    }
    if settings.followup_middle_length == 0
        || settings.top_bucket_limit == 0
        || settings.representatives_per_bucket == 0
        || settings.pair_top_limit == 0
        || settings.witness_limit == 0
    {
        eprintln!("limits and follow-up middle length must be at least 1");
        std::process::exit(2);
    }

    Options { out_dir, settings }
}

fn parse_next<T: std::str::FromStr>(args: &mut impl Iterator<Item = String>, flag: &str) -> T {
    args.next()
        .unwrap_or_else(|| {
            eprintln!("{flag} requires a value");
            std::process::exit(2);
        })
        .parse::<T>()
        .unwrap_or_else(|_| {
            eprintln!("invalid value for {flag}");
            std::process::exit(2);
        })
}

fn print_help() {
    println!("Unit-Cycle Phase Signal Report");
    println!();
    println!("Options:");
    println!("  --out-dir <path>                   Output directory (default: {DEFAULT_OUT_DIR})");
    println!(
        "  --bases <csv>                      Bases to sweep (default: {})",
        DEFAULT_PHASE_RESIDUAL_BASES
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );
    println!(
        "  --min-middle-length <n>            Minimum source M (default: {DEFAULT_PHASE_RESIDUAL_MIN_MIDDLE_LENGTH})"
    );
    println!(
        "  --max-middle-length <n>            Maximum source M (default: {DEFAULT_PHASE_RESIDUAL_MAX_MIDDLE_LENGTH})"
    );
    println!(
        "  --followup-middle-length <n>       Mature follow-up M (default: {DEFAULT_SHIFT_PHASE_FOLLOWUP_MIDDLE_LENGTH})"
    );
    println!(
        "  --top-bucket-limit <n>             Top buckets to follow (default: {DEFAULT_UNIT_CYCLE_TOP_BUCKET_LIMIT})"
    );
    println!(
        "  --representatives-per-bucket <n>   Representatives per bucket (default: {DEFAULT_UNIT_CYCLE_REPRESENTATIVES_PER_BUCKET})"
    );
    println!(
        "  --pair-top-limit <n>               Pair-level lead rows (default: {DEFAULT_UNIT_CYCLE_PAIR_TOP_LIMIT})"
    );
    println!(
        "  --witness-limit <n>                Witnesses per selected direction (default: {DEFAULT_UNIT_CYCLE_WITNESS_LIMIT})"
    );
}

fn render_report(bundle: &ReportBundle) -> String {
    let report = &bundle.report;
    let summary = &report.summary;
    let mut lines = Vec::new();
    lines.push("# Unit-Cycle Phase Signal Report".to_string());
    lines.push(String::new());
    lines.push(summary.strong_line.clone());
    lines.push(summary.caution_line.clone());
    lines.push(String::new());
    lines.push("## Core Question".to_string());
    lines.push("The report takes the shift-phase comparison and removes some base-local vocabulary. Instead of only asking whether `(1,T)` is strong in base 30, it asks which unit-cycle arc shapes keep producing survivor-yield residuals across bases.".to_string());
    lines.push(String::new());
    lines.push("```text".to_string());
    lines.push("same-gradient swap -> unit-cycle arc orientation".to_string());
    lines.push("-> residue gate profile -> survivor-yield residual".to_string());
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push("## Headline".to_string());
    lines.push(format!("- Phase rows: `{}`", summary.phase_row_count));
    lines.push(format!(
        "- Buckets: `{}` total, `{}` qualifying signal buckets",
        summary.bucket_row_count, summary.qualifying_bucket_count
    ));
    lines.push(format!(
        "- Strongest bucket: `{}` at `{:.3}` pp mean absolute survivor residual",
        summary.strongest_bucket_label, summary.strongest_bucket_mean_abs_survivor_residual_pp
    ));
    lines.push(format!(
        "- Strongest mature follow-up: `{}` (`{}`) at `{:+.3}` pp",
        summary.strongest_mature_track,
        summary.strongest_mature_pair,
        summary.strongest_mature_survivor_prime_residual_pp
    ));
    lines.push(String::new());
    lines.push("## Bucket Queue".to_string());
    lines.push("| M | bucket | rows | sign share | dominant | mean abs survivor residual | strongest pair |".to_string());
    lines.push("|---:|---|---:|---:|---|---:|---|".to_string());
    for row in report
        .cycle_bucket_rows
        .iter()
        .filter(|row| row.middle_length == report.settings.base_settings.max_middle_length)
        .filter(|row| row.qualifies_signal_bucket)
        .take(24)
    {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{:.3}` | `{}` | `{:.3}` pp | base `{}` `{}` vs `{}` `{:+.3}` pp |",
            row.middle_length,
            row.cycle_bucket_label,
            row.row_count,
            row.same_sign_share,
            row.dominant_sign,
            row.mean_abs_survivor_prime_residual_delta_pp,
            row.strongest_base,
            row.strongest_pair_label,
            row.strongest_reverse_pair_label,
            row.strongest_survivor_prime_residual_delta_pp
        ));
    }
    lines.push(String::new());
    lines.push("## Lead Rows".to_string());
    lines
        .push("| reason | base | pair | bucket | raw-size | survivor residual | tag |".to_string());
    lines.push("|---|---:|---|---|---:|---:|---|".to_string());
    for row in report.lead_rows.iter().take(32) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` vs `{}` | `{}` | `{:+.3}` pp | `{:+.3}` pp | `{}` |",
            row.selection_reason,
            row.base,
            row.pair_label,
            row.reverse_pair_label,
            row.cycle_bucket_label,
            row.residual_after_size_pp,
            row.survivor_prime_residual_delta_pp,
            row.lead_tag
        ));
    }
    lines.push(String::new());
    lines.push("## Mature Follow-Up".to_string());
    lines.push("| track | kind | base | pair | bucket | source survivor | follow-up survivor | hits source | hits follow-up | label |".to_string());
    lines.push("|---|---|---:|---|---|---:|---:|---|---|---|".to_string());
    for row in report.maturity_rows.iter().take(28) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` vs `{}` | `{}` | `{:+.3}` pp | `{:+.3}` pp | `{}` | `{}` | `{}` |",
            row.track_name,
            row.track_kind,
            row.base,
            row.pair_label,
            row.reverse_pair_label,
            row.cycle_bucket_label,
            row.source_survivor_prime_residual_delta_pp,
            row.followup_survivor_prime_residual_delta_pp,
            row.source_prime_hits,
            row.followup_prime_hits,
            row.stability_label
        ));
    }
    lines.push(String::new());
    lines.push("## Foils".to_string());
    lines.push(
        "| foil | base | pair | bucket | raw-size | residue delta | survivor residual | tag |"
            .to_string(),
    );
    lines.push("|---|---:|---|---|---:|---:|---:|---|".to_string());
    for row in &report.foil_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` vs `{}` | `{}` | `{:+.3}` pp | `{:+.3}` pp | `{:+.3}` pp | `{}` |",
            row.foil_name,
            row.base,
            row.pair_label,
            row.reverse_pair_label,
            row.cycle_bucket_label,
            row.residual_after_size_pp,
            row.residue_survivor_delta_pp,
            row.survivor_prime_residual_delta_pp,
            row.lead_tag
        ));
    }
    lines.push(String::new());
    lines.push("## Prime Witnesses".to_string());
    lines.push(
        "| track | base | M | orientation | pair | seed | template | decimal value |".to_string(),
    );
    lines.push("|---|---:|---:|---|---|---:|---|---:|".to_string());
    for row in report.witness_rows.iter().take(80) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |",
            row.track_name,
            row.base,
            row.middle_length,
            row.orientation,
            row.pair_label,
            row.seed,
            row.template_digits,
            row.decimal_value
        ));
    }
    lines.push(String::new());
    lines.push("## Visuals".to_string());
    for image in &bundle.image_artifact_rows {
        lines.push(format!("- `{}`: `{}`", image.label, image.path));
    }
    lines.push(String::new());
    lines.push("## Claim Boundary".to_string());
    lines.push("The report ranks finite exact compact lanes under unit-cycle normalization. A coherent bucket is a research lead; it is not evidence for an asymptotic prime-density law until matched controls and later formal work survive.".to_string());
    lines.push(String::new());
    lines.join("\n")
}

fn render_unit_cycle_arc_map(rows: &[UnitCycleLeadRow], path: &Path) {
    let root = BitMapBackend::new(path, (1500, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill arc map");
    root.draw(&Text::new(
        "Unit-Cycle Arc Map",
        (42, 42),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");
    root.draw(&Text::new(
        "Each card normalizes unit digits onto a cycle; the chord is the low-to-high arc behind the same-gradient swap.",
        (42, 82),
        ("sans-serif", 20).into_font(),
    ))
    .expect("draw note");

    for (idx, row) in rows.iter().take(12).enumerate() {
        let x = 60 + (idx % 4) as i32 * 360;
        let y = 140 + (idx / 4) as i32 * 260;
        draw_arc_card(&root, x, y, row);
    }
    root.present().expect("present arc map");
}

fn draw_arc_card(
    root: &DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>,
    x: i32,
    y: i32,
    row: &UnitCycleLeadRow,
) {
    let geometry = normalize_unit_cycle_geometry(row.base, row.low_digit, row.high_digit);
    let center = (x + 120, y + 105);
    let radius = 70.0;
    root.draw(&Rectangle::new(
        [(x, y), (x + 310, y + 220)],
        RGBColor(239, 247, 244).filled(),
    ))
    .expect("draw arc card fill");
    root.draw(&Rectangle::new(
        [(x, y), (x + 310, y + 220)],
        ShapeStyle::from(&RGBColor(80, 88, 96)).stroke_width(1),
    ))
    .expect("draw arc card border");
    root.draw(&Text::new(
        format!(
            "b{} {} vs {}",
            row.base, row.pair_label, row.reverse_pair_label
        ),
        (x + 14, y + 28),
        ("sans-serif", 16).into_font().style(FontStyle::Bold),
    ))
    .expect("draw arc card title");
    root.draw(&Circle::new(
        center,
        radius as i32,
        ShapeStyle::from(&RGBColor(105, 112, 120)).stroke_width(2),
    ))
    .expect("draw cycle");
    for unit_idx in 0..geometry.unit_count {
        let angle = angle_for_index(unit_idx, geometry.unit_count);
        let px = center.0 + (radius * angle.cos()) as i32;
        let py = center.1 + (radius * angle.sin()) as i32;
        root.draw(&Circle::new((px, py), 3, RGBColor(115, 125, 135).filled()))
            .expect("draw unit point");
    }
    let low_angle = angle_for_index(geometry.low_unit_index, geometry.unit_count);
    let high_angle = angle_for_index(geometry.high_unit_index, geometry.unit_count);
    let low = (
        center.0 + (radius * low_angle.cos()) as i32,
        center.1 + (radius * low_angle.sin()) as i32,
    );
    let high = (
        center.0 + (radius * high_angle.cos()) as i32,
        center.1 + (radius * high_angle.sin()) as i32,
    );
    root.draw(&PathElement::new(
        vec![low, high],
        ShapeStyle::from(&signed_color(row.survivor_prime_residual_delta_pp, 6.0)).stroke_width(4),
    ))
    .expect("draw chord");
    root.draw(&Circle::new(low, 7, RGBColor(39, 111, 178).filled()))
        .expect("draw low endpoint");
    root.draw(&Circle::new(high, 7, RGBColor(205, 95, 70).filled()))
        .expect("draw high endpoint");
    root.draw(&Text::new(
        format!("{} / {}", row.distance_label, row.low_to_high_arc_label),
        (x + 14, y + 184),
        ("sans-serif", 13).into_font(),
    ))
    .expect("draw arc labels");
    root.draw(&Text::new(
        row.edge_label.clone(),
        (x + 14, y + 204),
        ("sans-serif", 13).into_font(),
    ))
    .expect("draw edge label");
    root.draw(&Text::new(
        format!("{:+.3} pp", row.survivor_prime_residual_delta_pp),
        (x + 205, y + 204),
        ("sans-serif", 13).into_font().style(FontStyle::Bold),
    ))
    .expect("draw arc residual");
}

fn render_cycle_bucket_heatmap(rows: &[UnitCycleBucketRow], path: &Path) {
    let selected = rows
        .iter()
        .filter(|row| row.qualifies_signal_bucket)
        .take(18)
        .collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (1400, 860)).into_drawing_area();
    root.fill(&WHITE).expect("fill bucket heatmap");
    let max_value = selected
        .iter()
        .map(|row| row.mean_abs_survivor_prime_residual_delta_pp)
        .fold(0.0_f64, f64::max)
        .max(0.01);
    root.draw(&Text::new(
        "Qualifying Unit-Cycle Buckets",
        (40, 42),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw heatmap title");
    for (idx, row) in selected.iter().enumerate() {
        let x = 70 + (idx % 3) as i32 * 430;
        let y = 110 + (idx / 3) as i32 * 115;
        let t = row.mean_abs_survivor_prime_residual_delta_pp / max_value;
        root.draw(&Rectangle::new(
            [(x, y), (x + 390, y + 82)],
            RGBColor(240, (240.0 - 90.0 * t) as u8, (240.0 - 115.0 * t) as u8).filled(),
        ))
        .expect("draw bucket cell");
        root.draw(&Rectangle::new(
            [(x, y), (x + 390, y + 82)],
            ShapeStyle::from(&RGBColor(80, 88, 96)).stroke_width(1),
        ))
        .expect("draw bucket border");
        root.draw(&Text::new(
            row.cycle_bucket_label.clone(),
            (x + 12, y + 25),
            ("sans-serif", 16).into_font().style(FontStyle::Bold),
        ))
        .expect("draw bucket label");
        root.draw(&Text::new(
            format!(
                "rows {} sign {} {:.0}% mean |survivor| {:.3} pp",
                row.row_count,
                row.dominant_sign,
                row.same_sign_share * 100.0,
                row.mean_abs_survivor_prime_residual_delta_pp
            ),
            (x + 12, y + 55),
            ("sans-serif", 14).into_font(),
        ))
        .expect("draw bucket stats");
    }
    root.present().expect("present bucket heatmap");
}

fn render_distance_residual_scatter(rows: &[UnitCyclePhaseRow], middle_length: usize, path: &Path) {
    let selected = rows
        .iter()
        .filter(|row| row.middle_length == middle_length)
        .collect::<Vec<_>>();
    let (y_min, y_max) = padded_range(
        selected
            .iter()
            .map(|row| row.survivor_prime_residual_delta_pp),
    );
    let root = BitMapBackend::new(path, (1280, 840)).into_drawing_area();
    root.fill(&WHITE).expect("fill scatter");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Unit-Cycle Distance vs Survivor Residual (M={middle_length})"),
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(80)
        .y_label_area_size(90)
        .build_cartesian_2d(0.0_f64..0.55_f64, y_min..y_max)
        .expect("build distance scatter");
    chart
        .configure_mesh()
        .x_desc("cyclic distance / unit count")
        .y_desc("survivor-prime residual (percentage points)")
        .x_label_formatter(&|value| format!("{value:.2}"))
        .y_label_formatter(&|value| format!("{value:+.2}"))
        .draw()
        .expect("draw distance mesh");
    chart
        .draw_series(LineSeries::new(
            vec![(0.0, 0.0), (0.55, 0.0)],
            RGBColor(135, 135, 135).stroke_width(1),
        ))
        .expect("draw zero");
    for row in selected {
        chart
            .draw_series(std::iter::once(Circle::new(
                (row.distance_fraction, row.survivor_prime_residual_delta_pp),
                if row.base_complement { 7 } else { 4 },
                edge_color(&row.edge_label).filled(),
            )))
            .expect("draw distance point");
    }
    root.present().expect("present distance scatter");
}

fn render_edge_wrap_panel(rows: &[UnitCyclePhaseRow], middle_length: usize, path: &Path) {
    let mut groups: BTreeMap<String, Vec<&UnitCyclePhaseRow>> = BTreeMap::new();
    for row in rows.iter().filter(|row| row.middle_length == middle_length) {
        groups.entry(row.edge_label.clone()).or_default().push(row);
    }
    let labels = groups.keys().cloned().collect::<Vec<_>>();
    let values = labels
        .iter()
        .map(|label| {
            groups[label]
                .iter()
                .map(|row| row.abs_survivor_prime_residual_delta_pp)
                .sum::<f64>()
                / groups[label].len() as f64
        })
        .collect::<Vec<_>>();
    let max_value = values.iter().copied().fold(0.0_f64, f64::max).max(0.01);
    let root = BitMapBackend::new(path, (1160, 760)).into_drawing_area();
    root.fill(&WHITE).expect("fill edge panel");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Edge and Complement Buckets (M={middle_length})"),
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(80)
        .y_label_area_size(90)
        .build_cartesian_2d(
            0.5_f64..labels.len() as f64 + 0.5,
            0.0_f64..(max_value * 1.25),
        )
        .expect("build edge panel");
    chart
        .configure_mesh()
        .x_labels(labels.len())
        .x_desc("edge/complement label")
        .y_desc("mean abs survivor residual (percentage points)")
        .x_label_formatter(&|value| {
            let idx = value.round() as usize;
            if idx == 0 || idx > labels.len() {
                String::new()
            } else {
                labels[idx - 1].clone()
            }
        })
        .y_label_formatter(&|value| format!("{value:.2}"))
        .draw()
        .expect("draw edge mesh");
    for (idx, value) in values.iter().enumerate() {
        let x = idx as f64 + 1.0;
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x - 0.3, 0.0), (x + 0.3, *value)],
                edge_color(&labels[idx]).filled(),
            )))
            .expect("draw edge bar");
    }
    root.present().expect("present edge panel");
}

fn render_maturity_strip(rows: &[UnitCycleMaturityRow], path: &Path) {
    let selected = rows.iter().take(18).collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (1500, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill maturity strip");
    root.draw(&Text::new(
        "Unit-Cycle Maturity Strip",
        (42, 42),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw maturity title");
    root.draw(&Text::new(
        "blue = source M residual, orange = mature follow-up residual",
        (42, 82),
        ("sans-serif", 20).into_font(),
    ))
    .expect("draw maturity note");
    let max_abs = selected
        .iter()
        .flat_map(|row| {
            [
                row.source_survivor_prime_residual_delta_pp.abs(),
                row.followup_survivor_prime_residual_delta_pp.abs(),
            ]
        })
        .fold(0.0_f64, f64::max)
        .max(0.01);
    let x_zero = 760;
    let scale = 560.0 / max_abs;
    root.draw(&PathElement::new(
        vec![(x_zero, 125), (x_zero, 930)],
        ShapeStyle::from(&RGBColor(135, 135, 135)).stroke_width(2),
    ))
    .expect("draw zero");
    for (idx, row) in selected.iter().enumerate() {
        let y = 150 + idx as i32 * 43;
        let source_x = x_zero + (row.source_survivor_prime_residual_delta_pp * scale) as i32;
        let follow_x = x_zero + (row.followup_survivor_prime_residual_delta_pp * scale) as i32;
        root.draw(&Text::new(
            format!(
                "{} b{} {} {}",
                idx + 1,
                row.base,
                row.pair_label,
                row.edge_label
            ),
            (42, y + 5),
            ("sans-serif", 14).into_font(),
        ))
        .expect("draw row label");
        root.draw(&PathElement::new(
            vec![(source_x, y), (follow_x, y)],
            ShapeStyle::from(&RGBColor(85, 94, 104)).stroke_width(2),
        ))
        .expect("draw maturity segment");
        root.draw(&Circle::new(
            (source_x, y),
            7,
            RGBColor(42, 115, 185).filled(),
        ))
        .expect("draw source dot");
        root.draw(&Circle::new(
            (follow_x, y),
            7,
            RGBColor(218, 112, 63).filled(),
        ))
        .expect("draw followup dot");
        root.draw(&Text::new(
            row.stability_label.clone(),
            (1370, y + 5),
            ("sans-serif", 14).into_font(),
        ))
        .expect("draw stability");
    }
    root.present().expect("present maturity strip");
}

fn render_lead_gallery(
    lead_rows: &[UnitCycleLeadRow],
    maturity_rows: &[UnitCycleMaturityRow],
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1500, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill gallery");
    root.draw(&Text::new(
        "Unit-Cycle Lead Gallery",
        (38, 40),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw gallery title");
    for (idx, row) in lead_rows.iter().take(9).enumerate() {
        let x = 35 + (idx % 3) as i32 * 485;
        let y = 95 + (idx / 3) as i32 * 270;
        draw_lead_card(&root, x, y, row, maturity_rows);
    }
    root.present().expect("present gallery");
}

fn draw_lead_card(
    root: &DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>,
    x: i32,
    y: i32,
    row: &UnitCycleLeadRow,
    maturity_rows: &[UnitCycleMaturityRow],
) {
    let card_w = 455;
    let card_h = 240;
    let fill = if row.selection_reason == "curated_foil" {
        RGBColor(248, 239, 226)
    } else {
        RGBColor(231, 246, 241)
    };
    root.draw(&Rectangle::new(
        [(x, y), (x + card_w, y + card_h)],
        fill.filled(),
    ))
    .expect("draw lead card");
    root.draw(&Rectangle::new(
        [(x, y), (x + card_w, y + card_h)],
        ShapeStyle::from(&RGBColor(80, 88, 96)).stroke_width(1),
    ))
    .expect("draw lead border");
    let mature = maturity_rows.iter().find(|maturity| {
        maturity.base == row.base
            && maturity.low_digit == row.low_digit
            && maturity.high_digit == row.high_digit
            && maturity.source_middle_length == row.middle_length
    });
    root.draw(&Text::new(
        format!("{} b{} {}", row.selection_reason, row.base, row.pair_label),
        (x + 16, y + 30),
        ("sans-serif", 18).into_font().style(FontStyle::Bold),
    ))
    .expect("draw lead title");
    root.draw(&Text::new(
        format!("bucket: {}", row.cycle_bucket_label),
        (x + 16, y + 63),
        ("sans-serif", 15).into_font(),
    ))
    .expect("draw bucket");
    root.draw(&Text::new(
        format!(
            "source survivor: {:+.3} pp",
            row.survivor_prime_residual_delta_pp
        ),
        (x + 16, y + 98),
        ("sans-serif", 16).into_font(),
    ))
    .expect("draw source");
    if let Some(mature) = mature {
        root.draw(&Text::new(
            format!(
                "M{} follow-up: {:+.3} pp ({})",
                mature.followup_middle_length,
                mature.followup_survivor_prime_residual_delta_pp,
                mature.stability_label
            ),
            (x + 16, y + 130),
            ("sans-serif", 16).into_font().style(FontStyle::Bold),
        ))
        .expect("draw mature");
        root.draw(&Text::new(
            format!(
                "hits: {} -> {}",
                mature.source_prime_hits, mature.followup_prime_hits
            ),
            (x + 16, y + 162),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw hits");
    } else {
        root.draw(&Text::new(
            "not selected for M4 follow-up",
            (x + 16, y + 130),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw no followup");
    }
    root.draw(&Text::new(
        row.note.clone(),
        (x + 16, y + 205),
        ("sans-serif", 14).into_font(),
    ))
    .expect("draw note");
}

fn angle_for_index(index: usize, unit_count: usize) -> f64 {
    -std::f64::consts::FRAC_PI_2 + 2.0 * std::f64::consts::PI * index as f64 / unit_count as f64
}

fn edge_color(label: &str) -> RGBColor {
    match label {
        "base_complement" => RGBColor(203, 91, 68),
        "edge_pair" => RGBColor(218, 139, 57),
        "low_edge" => RGBColor(49, 124, 181),
        _ => RGBColor(62, 151, 116),
    }
}

fn signed_color(delta_pp: f64, max_abs_pp: f64) -> RGBColor {
    if delta_pp.abs() < f64::EPSILON {
        return RGBColor(235, 235, 235);
    }
    let t = (delta_pp.abs() / max_abs_pp).clamp(0.0, 1.0);
    let low = (235.0 - 75.0 * t) as u8;
    if delta_pp > 0.0 {
        RGBColor(238, low, low)
    } else {
        RGBColor(low, low, 238)
    }
}

fn padded_range(values: impl Iterator<Item = f64>) -> (f64, f64) {
    let values = values.collect::<Vec<_>>();
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !min.is_finite() || !max.is_finite() {
        return (-1.0, 1.0);
    }
    let span = (max - min).abs();
    let pad = if span < 0.001 { 1.0 } else { span * 0.25 };
    ((min - pad).min(0.0), (max + pad).max(0.0))
}
