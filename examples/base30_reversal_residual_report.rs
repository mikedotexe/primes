//! Base-30 ordered-pair reversal residual report.
//!
//! This report follows the compact `k=(0,0)` base-30 reversal lead and
//! decomposes each `(low, high)` versus `(high, low)` delta into size/PNT,
//! residue-survivor, and survivor-prime layers.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base30_reversal_residual_report -- --out-dir /tmp/primes_base30_reversal_residual
//! ```

use plotters::prelude::*;
use primes::validation::{
    base30_reversal::{
        build_base30_reversal_residual_report, Base30ReversalResidualReport,
        Base30ReversalResidualRow, Base30ReversalSettings, Base30TopResidualRow,
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

const DEFAULT_OUT_DIR: &str = "/tmp/primes_base30_reversal_residual";
const ARTIFACT_ID: &str = "base30_reversal_residual_report";
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
    report: Base30ReversalResidualReport,
    image_artifact_rows: Vec<ImageArtifactRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output directory");

    let report = build_base30_reversal_residual_report(options.settings);
    let heatmap_path = options.out_dir.join("signed_residual_heatmap.png");
    render_signed_residual_heatmap(
        &report.residual_rows,
        options.settings.max_middle_length,
        &heatmap_path,
    );
    let scatter_path = options.out_dir.join("size_vs_raw_delta_scatter.png");
    render_size_vs_raw_delta_scatter(
        &report.residual_rows,
        options.settings.max_middle_length,
        &scatter_path,
    );
    let focus_path = options.out_dir.join("focus_1b_component_panel.png");
    render_focus_component_panel(&report.residual_rows, &focus_path);
    let gallery_path = options.out_dir.join("top_residual_gallery.png");
    render_top_residual_gallery(&report.top_residual_rows, &gallery_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "signed_residual_heatmap".to_string(),
            label: format!(
                "Base-30 M={} signed reversal residual heatmap",
                options.settings.max_middle_length
            ),
            path: heatmap_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "size_vs_raw_delta_scatter".to_string(),
            label: "Raw reversal delta versus PNT size-expected delta".to_string(),
            path: scatter_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "focus_1b_component_panel".to_string(),
            label: "(1,B) component panel across M=1..3".to_string(),
            path: focus_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "top_residual_gallery".to_string(),
            label: "Top reversal residual gallery".to_string(),
            path: gallery_path.display().to_string(),
        },
    ];

    write_csv_rows(
        options.out_dir.join("reversal_residual_rows.csv"),
        &report.residual_rows,
    )
    .expect("write residual rows");
    write_csv_rows(
        options.out_dir.join("residue_delta_rows.csv"),
        &report.residue_delta_rows,
    )
    .expect("write residue delta rows");
    write_csv_rows(
        options.out_dir.join("top_residual_rows.csv"),
        &report.top_residual_rows,
    )
    .expect("write top residual rows");
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
            generator_cmd: "cargo run --release --example base30_reversal_residual_report"
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
                "reversal_residual_rows.csv".to_string(),
                "residue_delta_rows.csv".to_string(),
                "top_residual_rows.csv".to_string(),
                "witness_rows.csv".to_string(),
                "image_artifact_rows.csv".to_string(),
                "signed_residual_heatmap.png".to_string(),
                "size_vs_raw_delta_scatter.png".to_string(),
                "focus_1b_component_panel.png".to_string(),
                "top_residual_gallery.png".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote base-30 reversal residual report to {}",
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
    println!("Base 30 Reversal Residual Report");
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
        "  --top-limit <n>              Top residual pairs for gallery (default: {DEFAULT_BASE30_REVERSAL_TOP_LIMIT})"
    );
}

fn render_report(bundle: &ReportBundle) -> String {
    let report = &bundle.report;
    let summary = &report.summary;
    let focus_rows = report
        .residual_rows
        .iter()
        .filter(|row| {
            row.low_high_pair_label == summary.focus_pair_label
                && row.high_low_pair_label == summary.focus_reverse_pair_label
        })
        .collect::<Vec<_>>();

    let mut lines = Vec::new();
    lines.push("# Base 30 Reversal Residual Report".to_string());
    lines.push(String::new());
    lines.push(summary.strong_line.clone());
    lines.push(summary.caution_line.clone());
    lines.push(String::new());
    lines.push("## Layered Reading".to_string());
    lines.push(
        "The report keeps the reversal question in four layers instead of one magic percentage:"
            .to_string(),
    );
    lines.push(String::new());
    lines.push("```text".to_string());
    lines.push("raw prime-rate delta".to_string());
    lines.push("-> size / PNT-expected delta".to_string());
    lines.push("-> residue-survivor delta".to_string());
    lines.push("-> prime-rate-among-survivors residual".to_string());
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push("The smaller outer digit usually makes smaller candidates, so part of its advantage is expected. The residue phase can still change because the affine shift changes while the compact gradient stays fixed. Any remaining residual is a lead, not a theorem.".to_string());
    lines.push(String::new());
    lines.push("## Focus Pair".to_string());
    lines.push(format!(
        "- `{}` versus `{}`",
        summary.focus_pair_label, summary.focus_reverse_pair_label
    ));
    lines.push(format!(
        "- M=2 raw delta: `{:+.3}` percentage points",
        summary.focus_m2_raw_delta_pp
    ));
    lines.push(format!(
        "- M=3 raw delta: `{:+.3}` pp; size-expected delta: `{:+.3}` pp; residual after size: `{:+.3}` pp",
        summary.focus_m3_raw_delta_pp,
        summary.focus_m3_size_expected_delta_pp,
        summary.focus_m3_residual_after_size_pp
    ));
    lines.push(format!(
        "- M=3 survivor-prime residual: `{:+.3}` pp",
        summary.focus_m3_survivor_prime_residual_delta_pp
    ));
    lines.push(String::new());
    lines.push("## Focus Across M".to_string());
    lines.push("| M | hits | raw delta | size expected | residual after size | survivor share | survivor-prime residual |".to_string());
    lines.push("|---:|---:|---:|---:|---:|---:|---:|".to_string());
    for row in focus_rows {
        lines.push(format!(
            "| `{}` | `{} / {}` | `{:+.3}` pp | `{:+.3}` pp | `{:+.3}` pp | `{:.2}% / {:.2}%` | `{:+.3}` pp |",
            row.middle_length,
            row.low_high_prime_hits,
            row.high_low_prime_hits,
            row.raw_delta_pp,
            row.size_expected_delta_pp,
            row.residual_after_size_pp,
            row.low_high_survivor_share * 100.0,
            row.high_low_survivor_share * 100.0,
            row.survivor_prime_residual_delta_pp
        ));
    }
    lines.push(String::new());
    lines.push("## Top M-Residual Pairs".to_string());
    lines.push("| reason | pair | reverse | hits | raw delta | size expected | residual after size | survivor-prime residual |".to_string());
    lines.push("|---|---|---|---:|---:|---:|---:|---:|".to_string());
    for row in &report.top_residual_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{} / {}` | `{:+.3}` pp | `{:+.3}` pp | `{:+.3}` pp | `{:+.3}` pp |",
            row.selection_reason,
            row.low_high_pair_label,
            row.high_low_pair_label,
            row.low_high_prime_hits,
            row.high_low_prime_hits,
            row.raw_delta_pp,
            row.size_expected_delta_pp,
            row.residual_after_size_pp,
            row.survivor_prime_residual_delta_pp
        ));
    }
    lines.push(String::new());
    lines.push("## Residue Delta Rows".to_string());
    lines.push(
        "| pair | modulus set | survivor counts | survivor delta | excluded seed classes |"
            .to_string(),
    );
    lines.push("|---|---|---:|---:|---|".to_string());
    for row in report
        .residue_delta_rows
        .iter()
        .filter(|row| row.middle_length == report.settings.max_middle_length)
        .take(36)
    {
        lines.push(format!(
            "| `{}` vs `{}` | `{}` | `{} / {}` | `{:+.3}` pp | `{}` vs `{}` |",
            row.low_high_pair_label,
            row.high_low_pair_label,
            row.modulus_set_label,
            row.low_high_survivor_count,
            row.high_low_survivor_count,
            row.survivor_delta_pp,
            row.low_high_excluded_seed_classes,
            row.high_low_excluded_seed_classes
        ));
    }
    lines.push(String::new());
    lines.push("## First Witnesses".to_string());
    lines.push("| pair | seed | template | decimal value |".to_string());
    lines.push("|---|---:|---|---:|".to_string());
    for row in report.witness_rows.iter().take(64) {
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
    lines.push("This report claims an exact local affine decomposition for compact base-30 reversal comparisons. It does not claim that reversal residuals are globally stable, asymptotic, or independent of ordinary prime-density effects.".to_string());
    lines.push(String::new());
    lines.join("\n")
}

fn render_signed_residual_heatmap(
    rows: &[Base30ReversalResidualRow],
    middle_length: usize,
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1120, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill heatmap");
    root.draw(&Text::new(
        format!("Base 30 Reversal Residual Heatmap (M={middle_length}, k=(0,0))"),
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
        .map(|row| row.abs_residual_after_size_pp)
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

    for &outer in &units {
        for &inner in &units {
            let x_idx = units.iter().position(|&digit| digit == inner).unwrap();
            let y_idx = units.iter().position(|&digit| digit == outer).unwrap();
            let x = left + x_idx as i32 * cell;
            let y = top + y_idx as i32 * cell;
            if outer == inner {
                root.draw(&Rectangle::new(
                    [(x, y), (x + cell - 8, y + cell - 8)],
                    RGBColor(235, 235, 235).filled(),
                ))
                .expect("draw diagonal");
                continue;
            }
            let low = outer.min(inner);
            let high = outer.max(inner);
            let row = m_rows
                .iter()
                .find(|row| row.low_digit == low && row.high_digit == high)
                .expect("residual row should exist");
            let delta = if outer == low {
                row.residual_after_size_pp
            } else {
                -row.residual_after_size_pp
            };
            root.draw(&Rectangle::new(
                [(x, y), (x + cell - 8, y + cell - 8)],
                signed_color(delta, max_abs).filled(),
            ))
            .expect("draw cell");
            root.draw(&Text::new(
                format!("{delta:+.2}"),
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
    }

    root.draw(&Text::new(
        "rows = outer, columns = inner; values are raw delta minus PNT size-expected delta",
        (115, 935),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw note");
    root.present().expect("present heatmap");
}

fn render_size_vs_raw_delta_scatter(
    rows: &[Base30ReversalResidualRow],
    middle_length: usize,
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1180, 840)).into_drawing_area();
    root.fill(&WHITE).expect("fill scatter");
    let m_rows = rows
        .iter()
        .filter(|row| row.middle_length == middle_length)
        .collect::<Vec<_>>();
    let (x_min, x_max) = padded_range(m_rows.iter().map(|row| row.size_expected_delta_pp));
    let (y_min, y_max) = padded_range(m_rows.iter().map(|row| row.raw_delta_pp));

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base 30 Reversal: Size Baseline vs Raw Delta",
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(85)
        .y_label_area_size(90)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .expect("build scatter");

    chart
        .configure_mesh()
        .x_desc("PNT size-expected delta (percentage points)")
        .y_desc("raw prime-rate delta (percentage points)")
        .x_label_formatter(&|value| format!("{value:+.2}"))
        .y_label_formatter(&|value| format!("{value:+.2}"))
        .draw()
        .expect("draw mesh");

    let diag_min = x_min.min(y_min);
    let diag_max = x_max.max(y_max);
    chart
        .draw_series(LineSeries::new(
            vec![(diag_min, diag_min), (diag_max, diag_max)],
            RGBColor(120, 120, 120).stroke_width(2),
        ))
        .expect("draw diagonal");

    for row in &m_rows {
        let is_focus = row.low_digit == 1 && row.high_digit == 11;
        let style = if is_focus {
            RED.filled()
        } else {
            BLUE.mix(0.65).filled()
        };
        chart
            .draw_series(std::iter::once(Circle::new(
                (row.size_expected_delta_pp, row.raw_delta_pp),
                if is_focus { 8 } else { 5 },
                style,
            )))
            .expect("draw point");
        if is_focus {
            chart
                .draw_series(std::iter::once(Text::new(
                    "(1,B)",
                    (row.size_expected_delta_pp, row.raw_delta_pp),
                    ("sans-serif", 16).into_font().style(FontStyle::Bold),
                )))
                .expect("draw focus label");
        }
    }

    root.draw(&Text::new(
        "points above the diagonal beat the size-only expectation; below it underperform that baseline",
        (85, 104),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw note");
    root.present().expect("present scatter");
}

fn render_focus_component_panel(rows: &[Base30ReversalResidualRow], path: &Path) {
    let root = BitMapBackend::new(path, (1180, 820)).into_drawing_area();
    root.fill(&WHITE).expect("fill component panel");
    let focus_rows = rows
        .iter()
        .filter(|row| row.low_digit == 1 && row.high_digit == 11)
        .collect::<Vec<_>>();
    let values = focus_rows
        .iter()
        .flat_map(|row| {
            [
                row.raw_delta_pp,
                row.size_expected_delta_pp,
                row.residual_after_size_pp,
                row.survivor_prime_residual_delta_pp,
            ]
        })
        .collect::<Vec<_>>();
    let (y_min, y_max) = padded_range(values.into_iter());
    let x_min = focus_rows
        .first()
        .map(|row| row.middle_length as f64 - 0.5)
        .unwrap_or(0.5);
    let x_max = focus_rows
        .last()
        .map(|row| row.middle_length as f64 + 0.5)
        .unwrap_or(1.5);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "(1,B) vs (B,1): Layered Reversal Components",
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(70)
        .y_label_area_size(90)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .expect("build component chart");

    chart
        .configure_mesh()
        .x_labels(focus_rows.len())
        .x_desc("middle length M")
        .y_desc("delta (percentage points)")
        .x_label_formatter(&|value| format!("{}", value.round() as usize))
        .y_label_formatter(&|value| format!("{value:+.2}"))
        .draw()
        .expect("draw mesh");

    let raw = focus_rows
        .iter()
        .map(|row| (row.middle_length as f64, row.raw_delta_pp))
        .collect::<Vec<_>>();
    let size = focus_rows
        .iter()
        .map(|row| (row.middle_length as f64, row.size_expected_delta_pp))
        .collect::<Vec<_>>();
    let residual = focus_rows
        .iter()
        .map(|row| (row.middle_length as f64, row.residual_after_size_pp))
        .collect::<Vec<_>>();
    let survivor = focus_rows
        .iter()
        .map(|row| {
            (
                row.middle_length as f64,
                row.survivor_prime_residual_delta_pp,
            )
        })
        .collect::<Vec<_>>();

    chart
        .draw_series(LineSeries::new(raw.clone(), RED.stroke_width(4)))
        .expect("draw raw line");
    chart
        .draw_series(LineSeries::new(size.clone(), BLUE.stroke_width(4)))
        .expect("draw size line");
    chart
        .draw_series(LineSeries::new(
            residual.clone(),
            RGBColor(120, 60, 180).stroke_width(4),
        ))
        .expect("draw residual line");
    chart
        .draw_series(LineSeries::new(
            survivor.clone(),
            RGBColor(30, 140, 90).stroke_width(4),
        ))
        .expect("draw survivor line");

    for (series, color) in [
        (raw, RED),
        (size, BLUE),
        (residual, RGBColor(120, 60, 180)),
        (survivor, RGBColor(30, 140, 90)),
    ] {
        chart
            .draw_series(
                series
                    .into_iter()
                    .map(|point| Circle::new(point, 7, color.filled())),
            )
            .expect("draw component points");
    }

    root.draw(&Text::new(
        "red = raw, blue = size/PNT, purple = raw-size, green = survivor-prime residual",
        (92, 104),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw legend");
    root.present().expect("present component panel");
}

fn render_top_residual_gallery(rows: &[Base30TopResidualRow], path: &Path) {
    let root = BitMapBackend::new(path, (1500, 920)).into_drawing_area();
    root.fill(&WHITE).expect("fill gallery");
    root.draw(&Text::new(
        "Base 30 Top Reversal Residual Pairs",
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
            RGBColor(232, 246, 244)
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
            format!("{} vs {}", row.low_high_pair_label, row.high_low_pair_label),
            (x + 18, y + 31),
            ("sans-serif", 22).into_font().style(FontStyle::Bold),
        ))
        .expect("draw card title");
        root.draw(&Text::new(
            format!(
                "hits: {} / {}",
                row.low_high_prime_hits, row.high_low_prime_hits
            ),
            (x + 18, y + 66),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw hits");
        root.draw(&Text::new(
            format!("raw delta: {:+.3} pp", row.raw_delta_pp),
            (x + 18, y + 101),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw raw");
        root.draw(&Text::new(
            format!("size expected: {:+.3} pp", row.size_expected_delta_pp),
            (x + 18, y + 132),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw size");
        root.draw(&Text::new(
            format!("raw - size: {:+.3} pp", row.residual_after_size_pp),
            (x + 18, y + 163),
            ("sans-serif", 16).into_font().style(FontStyle::Bold),
        ))
        .expect("draw residual");
        root.draw(&Text::new(
            format!(
                "survivor-prime residual: {:+.3} pp",
                row.survivor_prime_residual_delta_pp
            ),
            (x + 18, y + 194),
            ("sans-serif", 14).into_font(),
        ))
        .expect("draw survivor residual");
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

fn padded_range(values: impl Iterator<Item = f64>) -> (f64, f64) {
    let values = values.collect::<Vec<_>>();
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !min.is_finite() || !max.is_finite() {
        return (-1.0, 1.0);
    }
    let span = (max - min).abs();
    let pad = if span < 0.001 { 1.0 } else { span * 0.2 };
    (min - pad, max + pad)
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
