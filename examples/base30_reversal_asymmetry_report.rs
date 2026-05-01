//! Focused base-30 ordered-pair reversal asymmetry report.
//!
//! This report compares compact `k=(0,0)` lanes `(outer, inner)` against their
//! reversals `(inner, outer)` and treats any difference as a local affine
//! residue-phase signal, not as a density theorem.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base30_reversal_asymmetry_report -- --out-dir /tmp/primes_base30_reversal_asymmetry
//! ```

use plotters::prelude::*;
use primes::validation::{
    base30_reversal::{
        build_base30_reversal_report, Base30ReversalLengthSummaryRow, Base30ReversalPairRow,
        Base30ReversalReport, Base30ReversalSettings, Base30TopAsymmetryRow,
        DEFAULT_BASE30_REVERSAL_MAX_MIDDLE_LENGTH, DEFAULT_BASE30_REVERSAL_MIN_MIDDLE_LENGTH,
        DEFAULT_BASE30_REVERSAL_TOP_LIMIT, DEFAULT_BASE30_REVERSAL_WITNESS_LIMIT,
    },
    bounded_k::unit_residues,
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_base30_reversal_asymmetry";
const ARTIFACT_ID: &str = "base30_reversal_asymmetry_report";
const EXPORT_VERSION: u32 = 1;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    settings: Base30ReversalSettings,
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
    report: Base30ReversalReport,
    image_artifact_rows: Vec<ImageArtifactRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output directory");

    let report = build_base30_reversal_report(options.settings);
    let heatmap_path = options.out_dir.join("m3_reversal_delta_heatmap.png");
    render_reversal_heatmap(
        &report.reversal_pair_rows,
        options.settings.max_middle_length,
        &heatmap_path,
    );
    let strip_path = options.out_dir.join("asymmetry_by_m_strip.png");
    render_asymmetry_strip(&report.length_summary_rows, &strip_path);
    let focus_path = options.out_dir.join("focus_pair_reversal_strip.png");
    render_focus_pair_strip(&report.length_summary_rows, &focus_path);
    let gallery_path = options.out_dir.join("top_reversal_gallery.png");
    render_top_gallery(&report.top_asymmetry_rows, &gallery_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "m3_reversal_delta_heatmap".to_string(),
            label: format!(
                "Base-30 M={} signed reversal delta heatmap",
                options.settings.max_middle_length
            ),
            path: heatmap_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "asymmetry_by_m_strip".to_string(),
            label: "Mean/max reversal asymmetry by middle length".to_string(),
            path: strip_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "focus_pair_reversal_strip".to_string(),
            label: "(1,B) versus (B,1) exact compact rates".to_string(),
            path: focus_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "top_reversal_gallery".to_string(),
            label: "Top reversal asymmetry gallery".to_string(),
            path: gallery_path.display().to_string(),
        },
    ];

    write_csv_rows(
        options.out_dir.join("ordered_pair_rows.csv"),
        &report.ordered_pair_rows,
    )
    .expect("write ordered pair rows");
    write_csv_rows(
        options.out_dir.join("reversal_pair_rows.csv"),
        &report.reversal_pair_rows,
    )
    .expect("write reversal pair rows");
    write_csv_rows(
        options.out_dir.join("length_summary_rows.csv"),
        &report.length_summary_rows,
    )
    .expect("write length summary rows");
    write_csv_rows(
        options.out_dir.join("top_asymmetry_rows.csv"),
        &report.top_asymmetry_rows,
    )
    .expect("write top asymmetry rows");
    write_csv_rows(
        options.out_dir.join("residue_phase_rows.csv"),
        &report.residue_phase_rows,
    )
    .expect("write residue phase rows");
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
            generator_cmd: "cargo run --release --example base30_reversal_asymmetry_report"
                .to_string(),
            args: env::args().skip(1).collect(),
            upstream_inputs: vec![
                "src/validation/base30_reversal.rs".to_string(),
                "src/validation/base30_wheel.rs".to_string(),
                "src/validation/fast_affine.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "ordered_pair_rows.csv".to_string(),
                "reversal_pair_rows.csv".to_string(),
                "length_summary_rows.csv".to_string(),
                "top_asymmetry_rows.csv".to_string(),
                "residue_phase_rows.csv".to_string(),
                "witness_rows.csv".to_string(),
                "image_artifact_rows.csv".to_string(),
                "m3_reversal_delta_heatmap.png".to_string(),
                "asymmetry_by_m_strip.png".to_string(),
                "focus_pair_reversal_strip.png".to_string(),
                "top_reversal_gallery.png".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote base-30 reversal asymmetry report to {}",
        options.out_dir.display()
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut settings = Base30ReversalSettings::default();
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir")),
            "--min-middle-length" => {
                settings.min_middle_length = parse_next(&mut args, "--min-middle-length");
            }
            "--max-middle-length" => {
                settings.max_middle_length = parse_next(&mut args, "--max-middle-length");
            }
            "--witness-limit" => {
                settings.witness_limit = parse_next(&mut args, "--witness-limit");
            }
            "--top-limit" => {
                settings.top_limit = parse_next(&mut args, "--top-limit");
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

    if settings.min_middle_length == 0 || settings.max_middle_length < settings.min_middle_length {
        eprintln!("middle length range must be nonempty and start at least at 1");
        std::process::exit(2);
    }
    if settings.witness_limit == 0 || settings.top_limit == 0 {
        eprintln!("--witness-limit and --top-limit must be at least 1");
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
    println!("Base 30 Reversal Asymmetry Report");
    println!();
    println!("Options:");
    println!("  --out-dir <path>             Output directory (default: {DEFAULT_OUT_DIR})");
    println!(
        "  --min-middle-length <n>      Minimum M (default: {DEFAULT_BASE30_REVERSAL_MIN_MIDDLE_LENGTH})"
    );
    println!(
        "  --max-middle-length <n>      Maximum M (default: {DEFAULT_BASE30_REVERSAL_MAX_MIDDLE_LENGTH})"
    );
    println!(
        "  --witness-limit <n>          Witnesses per selected direction (default: {DEFAULT_BASE30_REVERSAL_WITNESS_LIMIT})"
    );
    println!(
        "  --top-limit <n>              Top asymmetry pairs for gallery (default: {DEFAULT_BASE30_REVERSAL_TOP_LIMIT})"
    );
}

fn render_report(bundle: &ReportBundle) -> String {
    let report = &bundle.report;
    let summary = &report.summary;
    let mut lines = Vec::new();
    lines.push("# Base 30 Reversal Asymmetry Report".to_string());
    lines.push(String::new());
    lines.push(summary.strong_line.clone());
    lines.push(summary.caution_line.clone());
    lines.push(String::new());
    lines.push("## Why Reversal Can Matter".to_string());
    lines.push("The compact template is symmetric after the ordered roles are chosen, but `(outer, inner)` and `(inner, outer)` are different affine shifts with the same gradient. They therefore walk through later residue gates in different phases.".to_string());
    lines.push(String::new());
    lines.push("For compact base-30 lanes:".to_string());
    lines.push(String::new());
    lines.push("```text".to_string());
    lines.push("N_{o,i}(s) - N_{i,o}(s) = (o-i)(30-1)(30^(M+2)-1)".to_string());
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push("## Focus Pair".to_string());
    lines.push(format!(
        "- `{}` versus `{}`",
        summary.focus_pair_label, summary.focus_reverse_pair_label
    ));
    lines.push(format!(
        "- M=2 delta: `{:+.3}` percentage points",
        summary.focus_m2_delta_pp
    ));
    lines.push(format!(
        "- M=3 delta: `{:+.3}` percentage points (`{}` vs `{}` hits)",
        summary.focus_m3_delta_pp, summary.focus_m3_forward_hits, summary.focus_m3_reverse_hits
    ));
    lines.push(String::new());
    lines.push("## Length Summary".to_string());
    lines.push("| M | mean abs delta | median abs delta | max abs delta | strongest direction | focus delta |".to_string());
    lines.push("|---:|---:|---:|---:|---|---:|".to_string());
    for row in &report.length_summary_rows {
        lines.push(format!(
            "| `{}` | `{:.3}` pp | `{:.3}` pp | `{:.3}` pp | `{}` over `{}` | `{:+.3}` pp |",
            row.middle_length,
            row.mean_abs_delta_pp,
            row.median_abs_delta_pp,
            row.max_abs_delta_pp,
            row.strongest_dominant_pair,
            row.strongest_weaker_pair,
            row.focus_delta_pp
        ));
    }
    lines.push(String::new());
    lines.push("## Top M-Reversal Pairs".to_string());
    lines.push("| reason | pair | reverse | hits | rates | delta | dominant |".to_string());
    lines.push("|---|---|---|---:|---:|---:|---|".to_string());
    for row in &report.top_asymmetry_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{} / {}` | `{:.3}% / {:.3}%` | `{:+.3}` pp | `{}` |",
            row.selection_reason,
            row.pair_label,
            row.reverse_pair_label,
            row.forward_hits,
            row.reverse_hits,
            row.forward_rate * 100.0,
            row.reverse_rate * 100.0,
            row.rate_delta_pp,
            row.dominant_pair_label
        ));
    }
    lines.push(String::new());
    lines.push("## Residue Phase Fingerprints".to_string());
    lines.push(
        "| pair | mod | shift | gradient | excluded seed class | survivor share |".to_string(),
    );
    lines.push("|---|---:|---:|---:|---|---:|".to_string());
    for row in report.residue_phase_rows.iter().take(48) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{:.2}%` |",
            row.pair_label,
            row.modulus,
            row.shift_modulus,
            row.gradient_modulus,
            row.excluded_seed_classes,
            row.survivor_share * 100.0
        ));
    }
    lines.push(String::new());
    lines.push("## First Witnesses".to_string());
    lines.push("| pair | seed | template | decimal value |".to_string());
    lines.push("|---|---:|---|---:|".to_string());
    for row in report.witness_rows.iter().take(48) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` |",
            row.pair_label, row.seed, row.template_digits, row.decimal_value
        ));
    }
    lines.push(String::new());
    lines.push("## Visuals".to_string());
    for image in &bundle.image_artifact_rows {
        lines.push(format!("- `{}`: `{}`", image.label, image.path));
    }
    lines.push(String::new());
    lines.push("## Claim Boundary".to_string());
    lines.push("The report shows ordered-pair phase asymmetry on the compact base-30 wheel surface. It does not claim that reversal asymmetry predicts primes globally, and it does not replace matched density controls.".to_string());
    lines.push(String::new());
    lines.join("\n")
}

fn render_reversal_heatmap(rows: &[Base30ReversalPairRow], middle_length: usize, path: &Path) {
    let root = BitMapBackend::new(path, (1120, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill heatmap");
    root.draw(&Text::new(
        format!("Base 30 Reversal Delta Heatmap (M={middle_length}, k=(0,0))"),
        (40, 40),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");

    let units = unit_residues(30);
    let m_rows = rows
        .iter()
        .filter(|row| row.middle_length == middle_length)
        .collect::<Vec<_>>();
    let max_abs = m_rows
        .iter()
        .map(|row| row.abs_rate_delta_pp)
        .fold(0.0_f64, f64::max)
        .max(0.01);
    let left = 125;
    let top = 110;
    let cell = 100;

    for (idx, digit) in units.iter().enumerate() {
        let label = digit_label(*digit);
        root.draw(&Text::new(
            label.clone(),
            (left + idx as i32 * cell + 38, top - 28),
            ("sans-serif", 18).into_font().style(FontStyle::Bold),
        ))
        .expect("draw x label");
        root.draw(&Text::new(
            label,
            (left - 55, top + idx as i32 * cell + 58),
            ("sans-serif", 18).into_font().style(FontStyle::Bold),
        ))
        .expect("draw y label");
    }

    for row in m_rows {
        let x_idx = units.iter().position(|&digit| digit == row.inner).unwrap();
        let y_idx = units.iter().position(|&digit| digit == row.outer).unwrap();
        let x = left + x_idx as i32 * cell;
        let y = top + y_idx as i32 * cell;
        root.draw(&Rectangle::new(
            [(x, y), (x + cell - 8, y + cell - 8)],
            signed_color(row.rate_delta_pp, max_abs).filled(),
        ))
        .expect("draw cell");
        root.draw(&Text::new(
            format!("{:+.2}", row.rate_delta_pp),
            (x + 16, y + 39),
            ("sans-serif", 16).into_font().style(FontStyle::Bold),
        ))
        .expect("draw delta");
        root.draw(&Text::new(
            "pp",
            (x + 39, y + 66),
            ("sans-serif", 14).into_font(),
        ))
        .expect("draw unit");
    }

    root.draw(&Text::new(
        "rows = outer, columns = inner; positive means this ordered pair beats its reversal",
        (115, 935),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw note");
    root.present().expect("present heatmap");
}

fn render_asymmetry_strip(rows: &[Base30ReversalLengthSummaryRow], path: &Path) {
    let root = BitMapBackend::new(path, (1180, 820)).into_drawing_area();
    root.fill(&WHITE).expect("fill strip");
    let max_delta = rows
        .iter()
        .map(|row| row.max_abs_delta_pp)
        .fold(0.0_f64, f64::max)
        .max(0.01);
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base 30 Reversal Asymmetry by Middle Length",
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(70)
        .y_label_area_size(85)
        .build_cartesian_2d(
            0.5_f64..(rows.len() as f64 + 0.5),
            0.0_f64..(max_delta * 1.25),
        )
        .expect("build chart");

    chart
        .configure_mesh()
        .x_labels(rows.len())
        .x_desc("middle length M")
        .y_desc("absolute reversal delta (percentage points)")
        .x_label_formatter(&|value| format!("{}", value.round() as usize))
        .y_label_formatter(&|value| format!("{value:.2}"))
        .draw()
        .expect("draw mesh");

    for row in rows {
        let x = row.middle_length as f64;
        chart
            .draw_series(std::iter::once(Circle::new(
                (x, row.mean_abs_delta_pp),
                7,
                BLUE.filled(),
            )))
            .expect("draw mean");
        chart
            .draw_series(std::iter::once(TriangleMarker::new(
                (x, row.median_abs_delta_pp),
                8,
                RGBColor(40, 150, 90).filled(),
            )))
            .expect("draw median");
        chart
            .draw_series(std::iter::once(Cross::new(
                (x, row.max_abs_delta_pp),
                10,
                RED.stroke_width(3),
            )))
            .expect("draw max");
        chart
            .draw_series(std::iter::once(Circle::new(
                (x, row.focus_delta_pp.abs()),
                7,
                RGBColor(120, 60, 180).filled(),
            )))
            .expect("draw focus");
    }

    root.draw(&Text::new(
        "blue = mean, green = median, red cross = max, purple = |(1,B)-(B,1)|",
        (92, 104),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw legend");
    root.present().expect("present strip");
}

fn render_focus_pair_strip(rows: &[Base30ReversalLengthSummaryRow], path: &Path) {
    let root = BitMapBackend::new(path, (1180, 780)).into_drawing_area();
    root.fill(&WHITE).expect("fill focus");
    let max_rate = rows
        .iter()
        .flat_map(|row| [row.focus_forward_rate, row.focus_reverse_rate])
        .fold(0.0_f64, f64::max)
        .max(0.01);
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "(1,B) vs (B,1): Compact Base-30 Reversal",
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(70)
        .y_label_area_size(85)
        .build_cartesian_2d(
            0.5_f64..(rows.len() as f64 + 0.5),
            0.0_f64..(max_rate * 1.25),
        )
        .expect("build focus chart");

    chart
        .configure_mesh()
        .x_labels(rows.len())
        .x_desc("middle length M")
        .y_desc("compact prime rate")
        .x_label_formatter(&|value| format!("{}", value.round() as usize))
        .y_label_formatter(&|value| format!("{:.1}%", value * 100.0))
        .draw()
        .expect("draw mesh");

    let forward = rows
        .iter()
        .map(|row| (row.middle_length as f64, row.focus_forward_rate))
        .collect::<Vec<_>>();
    let reverse = rows
        .iter()
        .map(|row| (row.middle_length as f64, row.focus_reverse_rate))
        .collect::<Vec<_>>();
    chart
        .draw_series(LineSeries::new(forward.clone(), BLUE.stroke_width(4)))
        .expect("draw forward line");
    chart
        .draw_series(LineSeries::new(reverse.clone(), RED.stroke_width(4)))
        .expect("draw reverse line");
    chart
        .draw_series(
            forward
                .into_iter()
                .map(|point| Circle::new(point, 7, BLUE.filled())),
        )
        .expect("draw forward points");
    chart
        .draw_series(
            reverse
                .into_iter()
                .map(|point| Circle::new(point, 7, RED.filled())),
        )
        .expect("draw reverse points");

    root.draw(&Text::new(
        "blue = (1,B), red = (B,1)",
        (92, 104),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw legend");
    root.present().expect("present focus");
}

fn render_top_gallery(rows: &[Base30TopAsymmetryRow], path: &Path) {
    let root = BitMapBackend::new(path, (1500, 920)).into_drawing_area();
    root.fill(&WHITE).expect("fill gallery");
    root.draw(&Text::new(
        "Base 30 Top Reversal Asymmetry Pairs",
        (36, 38),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");

    let card_w = 460;
    let card_h = 235;
    for (idx, row) in rows.iter().take(9).enumerate() {
        let x = 35 + (idx % 3) as i32 * 485;
        let y = 95 + (idx / 3) as i32 * 260;
        let fill = if row.selection_reason == "focus_pair" {
            RGBColor(255, 245, 220)
        } else {
            RGBColor(230, 241, 255)
        };
        root.draw(&Rectangle::new(
            [(x, y), (x + card_w, y + card_h)],
            fill.filled(),
        ))
        .expect("draw card");
        root.draw(&Rectangle::new(
            [(x, y), (x + card_w, y + card_h)],
            ShapeStyle::from(&RGBColor(80, 80, 88)).stroke_width(1),
        ))
        .expect("draw card border");
        root.draw(&Text::new(
            format!("{} vs {}", row.pair_label, row.reverse_pair_label),
            (x + 18, y + 31),
            ("sans-serif", 22).into_font().style(FontStyle::Bold),
        ))
        .expect("draw title");
        root.draw(&Text::new(
            format!("dominant: {}", row.dominant_pair_label),
            (x + 18, y + 66),
            ("sans-serif", 17).into_font(),
        ))
        .expect("draw dominant");
        root.draw(&Text::new(
            format!("hits: {} / {}", row.forward_hits, row.reverse_hits),
            (x + 18, y + 101),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw hits");
        root.draw(&Text::new(
            format!(
                "rates: {:.3}% / {:.3}%",
                row.forward_rate * 100.0,
                row.reverse_rate * 100.0
            ),
            (x + 18, y + 132),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw rates");
        root.draw(&Text::new(
            format!("delta: {:+.3} pp", row.rate_delta_pp),
            (x + 18, y + 163),
            ("sans-serif", 16).into_font().style(FontStyle::Bold),
        ))
        .expect("draw delta");
        root.draw(&Text::new(
            format!("shift delta: {}", row.shift_delta),
            (x + 18, y + 194),
            ("sans-serif", 14).into_font(),
        ))
        .expect("draw shift");
    }

    root.present().expect("present gallery");
}

fn signed_color(delta_pp: f64, max_abs_pp: f64) -> RGBColor {
    if delta_pp.abs() < f64::EPSILON {
        return RGBColor(235, 235, 235);
    }
    let t = (delta_pp.abs() / max_abs_pp).clamp(0.0, 1.0);
    let low = (235.0 - 70.0 * t) as u8;
    if delta_pp > 0.0 {
        RGBColor(255, low, low)
    } else {
        RGBColor(low, low, 255)
    }
}

fn digit_label(digit: u32) -> String {
    if digit < 10 {
        digit.to_string()
    } else {
        char::from_u32('A' as u32 + digit - 10)
            .expect("base-30 digit")
            .to_string()
    }
}
