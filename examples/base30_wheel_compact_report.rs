//! Focused base-30 compact wheel report.
//!
//! This report explains the base-30 compact lane as a clean classical wheel
//! surface: strong, useful, and visually compelling, but not a residual density
//! theorem.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example base30_wheel_compact_report -- --out-dir /tmp/primes_base30_wheel_compact
//! ```

use plotters::prelude::*;
use primes::validation::{
    base30_wheel::{
        build_base30_wheel_report, Base30LengthSummaryRow, Base30PairHeatmapRow,
        Base30ResidueFunnelRow, Base30TopPairRow, Base30WheelReport, Base30WheelSettings,
        Base30WitnessRow, BASE30_TARGET_INNER, BASE30_TARGET_OUTER,
        DEFAULT_BASE30_MAX_MIDDLE_LENGTH, DEFAULT_BASE30_MIN_MIDDLE_LENGTH,
        DEFAULT_BASE30_WITNESS_LIMIT,
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

const DEFAULT_OUT_DIR: &str = "/tmp/primes_base30_wheel_compact";
const ARTIFACT_ID: &str = "base30_wheel_compact_report";
const EXPORT_VERSION: u32 = 1;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    settings: Base30WheelSettings,
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
    report: Base30WheelReport,
    image_artifact_rows: Vec<ImageArtifactRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output directory");

    let report = build_base30_wheel_report(options.settings);
    let pair_heatmap_path = options.out_dir.join("pair_heatmap_m2.png");
    render_pair_heatmap(&report.pair_heatmap_rows, &pair_heatmap_path);
    let rate_strip_path = options.out_dir.join("compact_rate_strip_m1_m3.png");
    render_rate_strip(&report.length_summary_rows, &rate_strip_path);
    let funnel_path = options.out_dir.join("b7_residue_funnel.png");
    render_residue_funnel(&report.residue_funnel_rows, &funnel_path);
    let gallery_path = options.out_dir.join("top_pair_witness_gallery.png");
    render_witness_gallery(&report.top_pair_rows, &report.witness_rows, &gallery_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "pair_heatmap_m2".to_string(),
            label: "All-pair base-30 M=2 compact heatmap".to_string(),
            path: pair_heatmap_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "compact_rate_strip_m1_m3".to_string(),
            label: "Compact k=(0,0) rate strip across M=1..3".to_string(),
            path: rate_strip_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "b7_residue_funnel".to_string(),
            label: "(B,7) residue funnel".to_string(),
            path: funnel_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "top_pair_witness_gallery".to_string(),
            label: "Top-pair witness gallery".to_string(),
            path: gallery_path.display().to_string(),
        },
    ];

    write_csv_rows(
        options.out_dir.join("pair_heatmap_rows.csv"),
        &report.pair_heatmap_rows,
    )
    .expect("write pair heatmap rows");
    write_csv_rows(
        options.out_dir.join("length_summary_rows.csv"),
        &report.length_summary_rows,
    )
    .expect("write length summary rows");
    write_csv_rows(
        options.out_dir.join("residue_funnel_rows.csv"),
        &report.residue_funnel_rows,
    )
    .expect("write residue funnel rows");
    write_csv_rows(
        options.out_dir.join("top_pair_rows.csv"),
        &report.top_pair_rows,
    )
    .expect("write top pair rows");
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
            generator_cmd: "cargo run --release --example base30_wheel_compact_report".to_string(),
            args: env::args().skip(1).collect(),
            upstream_inputs: vec![
                "src/validation/base30_wheel.rs".to_string(),
                "src/validation/bounded_k.rs".to_string(),
                "src/validation/fast_affine.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "pair_heatmap_rows.csv".to_string(),
                "length_summary_rows.csv".to_string(),
                "residue_funnel_rows.csv".to_string(),
                "top_pair_rows.csv".to_string(),
                "witness_rows.csv".to_string(),
                "image_artifact_rows.csv".to_string(),
                "pair_heatmap_m2.png".to_string(),
                "compact_rate_strip_m1_m3.png".to_string(),
                "b7_residue_funnel.png".to_string(),
                "top_pair_witness_gallery.png".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote base-30 wheel compact report to {}",
        options.out_dir.display()
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut settings = Base30WheelSettings::default();
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
    if settings.witness_limit == 0 {
        eprintln!("--witness-limit must be at least 1");
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
    println!("Base 30 Wheel Compact Report");
    println!();
    println!("Options:");
    println!("  --out-dir <path>             Output directory (default: {DEFAULT_OUT_DIR})");
    println!(
        "  --min-middle-length <n>      Minimum M (default: {DEFAULT_BASE30_MIN_MIDDLE_LENGTH})"
    );
    println!(
        "  --max-middle-length <n>      Maximum M (default: {DEFAULT_BASE30_MAX_MIDDLE_LENGTH})"
    );
    println!(
        "  --witness-limit <n>          Witnesses per selected pair (default: {DEFAULT_BASE30_WITNESS_LIMIT})"
    );
}

fn render_report(bundle: &ReportBundle) -> String {
    let report = &bundle.report;
    let summary = &report.summary;
    let mut lines = Vec::new();
    lines.push("# Base 30 Wheel Compact Report".to_string());
    lines.push(String::new());
    lines.push("base 30 is a clean wheel-compressed affine candidate surface.".to_string());
    lines.push(
        "this is a gorgeous classical wheel effect, not yet residual density magic.".to_string(),
    );
    lines.push(String::new());
    lines.push("## Canonical Walkthrough".to_string());
    lines.push(format!(
        "- target pair: `{}` = ({},{})",
        summary.target_pair_label, BASE30_TARGET_OUTER, BASE30_TARGET_INNER
    ));
    lines.push(format!(
        "- affine lane: `N(s) = {} + {}*s`, `s = 0..{}`",
        summary.target_shift,
        summary.target_gradient,
        summary.target_seed_capacity - 1
    ));
    lines.push(format!(
        "- `(B,7)` M=2 compact rate: `{:.2}%`, rank `{}/{}` among base-30 unit pairs",
        summary.target_m2_rate_k00 * 100.0,
        summary.target_m2_rank,
        64
    ));
    lines.push(format!(
        "- top M=2 compact pair: `{}` at `{:.2}%`",
        summary.top_m2_pair,
        summary.top_m2_rate_k00 * 100.0
    ));
    lines.push(String::new());
    lines.push("## Compact Rate Summary".to_string());
    lines.push("| M | mean | median | min | max | (B,7) rank | noncompact winners |".to_string());
    lines.push("|---:|---:|---:|---:|---:|---:|---:|".to_string());
    for row in &report.length_summary_rows {
        lines.push(format!(
            "| `{}` | `{:.2}%` | `{:.2}%` | `{:.2}%` | `{:.2}%` | `{}` | `{}` |",
            row.middle_length,
            row.mean_k00_rate * 100.0,
            row.median_k00_rate * 100.0,
            row.min_k00_rate * 100.0,
            row.max_k00_rate * 100.0,
            row.target_pair_rank,
            row.noncompact_counterexample_count
        ));
    }
    lines.push(String::new());
    lines.push("## Residue Funnel".to_string());
    lines.push("| moduli | excluded class for added modulus | survivors | primes | prime rate among survivors |".to_string());
    lines.push("|---|---:|---:|---:|---:|".to_string());
    for row in &report.residue_funnel_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{:.2}%` |",
            row.modulus_set_label,
            row.added_excluded_seed_classes,
            row.survivor_count,
            row.prime_count,
            row.prime_rate_among_survivors * 100.0
        ));
    }
    lines.push(String::new());
    lines.push("## Top Pair Gallery".to_string());
    lines.push("| reason | rank | pair | compact rate | best k |".to_string());
    lines.push("|---|---:|---|---:|---|".to_string());
    for row in &report.top_pair_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{:.2}%` | `{}` |",
            row.selection_reason,
            row.k00_rank,
            row.pair_label,
            row.rate_k00 * 100.0,
            row.best_k
        ));
    }
    lines.push(String::new());
    lines.push("## First Witnesses".to_string());
    lines.push("| pair | seed | base-30 template | decimal value |".to_string());
    lines.push("|---|---:|---|---:|".to_string());
    for row in report.witness_rows.iter().take(32) {
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
    lines.push("The report explains why base 30 is a strong compact wheel surface. It does not claim that `(B,7)` is uniquely exceptional, and it does not claim residual density magic beyond the controls already maintained in the density atlas.".to_string());
    lines.push(String::new());
    lines.join("\n")
}

fn render_pair_heatmap(rows: &[Base30PairHeatmapRow], path: &Path) {
    let root = BitMapBackend::new(path, (1120, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill heatmap");
    root.draw(&Text::new(
        "Base 30 Compact Pair Heatmap (M=2, k=(0,0))",
        (40, 40),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");

    let units = unit_residues(30);
    let m2_rows = rows
        .iter()
        .filter(|row| row.middle_length == 2)
        .collect::<Vec<_>>();
    let max_rate = m2_rows
        .iter()
        .map(|row| row.rate_k00)
        .fold(0.0_f64, f64::max)
        .max(0.01);
    let left = 125;
    let top = 110;
    let cell = 100;

    for (idx, digit) in units.iter().enumerate() {
        root.draw(&Text::new(
            digit_label(*digit),
            (left + idx as i32 * cell + 38, top - 28),
            ("sans-serif", 18).into_font().style(FontStyle::Bold),
        ))
        .expect("draw x label");
        root.draw(&Text::new(
            digit_label(*digit),
            (left - 55, top + idx as i32 * cell + 58),
            ("sans-serif", 18).into_font().style(FontStyle::Bold),
        ))
        .expect("draw y label");
    }

    for row in m2_rows {
        let x_idx = units.iter().position(|&digit| digit == row.inner).unwrap();
        let y_idx = units.iter().position(|&digit| digit == row.outer).unwrap();
        let x = left + x_idx as i32 * cell;
        let y = top + y_idx as i32 * cell;
        let color = heat_color(row.rate_k00, max_rate);
        root.draw(&Rectangle::new(
            [(x, y), (x + cell - 8, y + cell - 8)],
            color.filled(),
        ))
        .expect("draw heat cell");
        if row.outer == BASE30_TARGET_OUTER && row.inner == BASE30_TARGET_INNER {
            root.draw(&Rectangle::new(
                [(x + 3, y + 3), (x + cell - 11, y + cell - 11)],
                ShapeStyle::from(&BLACK).stroke_width(3),
            ))
            .expect("draw target outline");
        }
        root.draw(&Text::new(
            format!("{:.1}%", row.rate_k00 * 100.0),
            (x + 18, y + 39),
            ("sans-serif", 17).into_font().style(FontStyle::Bold),
        ))
        .expect("draw rate");
        root.draw(&Text::new(
            format!("#{}", row.k00_rank),
            (x + 31, y + 66),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw rank");
    }

    root.draw(&Text::new(
        "rows = outer digit, columns = inner digit; outlined cell is (B,7)",
        (135, 935),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw note");
    root.present().expect("present heatmap");
}

fn render_rate_strip(rows: &[Base30LengthSummaryRow], path: &Path) {
    let root = BitMapBackend::new(path, (1180, 830)).into_drawing_area();
    root.fill(&WHITE).expect("fill strip");
    let max_rate = rows
        .iter()
        .map(|row| row.max_k00_rate)
        .fold(0.0_f64, f64::max)
        .max(0.01);
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Base 30 Compact Rate Strip Across M",
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(70)
        .y_label_area_size(70)
        .build_cartesian_2d(
            0.5_f64..(rows.len() as f64 + 0.5),
            0.0_f64..(max_rate * 1.20),
        )
        .expect("build rate strip");

    chart
        .configure_mesh()
        .x_desc("middle length M")
        .y_desc("k=(0,0) prime rate")
        .x_label_formatter(&|value| format!("{}", value.round() as usize))
        .y_label_formatter(&|value| format!("{:.1}%", value * 100.0))
        .draw()
        .expect("draw mesh");

    for row in rows {
        let x = row.middle_length as f64;
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(x, row.min_k00_rate), (x, row.max_k00_rate)],
                BLACK,
            )))
            .expect("draw min max");
        chart
            .draw_series(std::iter::once(Circle::new(
                (x, row.mean_k00_rate),
                7,
                BLUE.filled(),
            )))
            .expect("draw mean");
        chart
            .draw_series(std::iter::once(TriangleMarker::new(
                (x, row.median_k00_rate),
                8,
                RGBColor(40, 150, 90).filled(),
            )))
            .expect("draw median");
        chart
            .draw_series(std::iter::once(Cross::new(
                (x, row.target_pair_rate_k00),
                10,
                RED.stroke_width(3),
            )))
            .expect("draw target");
    }

    root.draw(&Rectangle::new(
        [(112, 84), (920, 116)],
        ShapeStyle::from(&WHITE.mix(0.92)).filled(),
    ))
    .expect("draw legend background");
    root.draw(&Text::new(
        "blue = mean, green = median, red cross = (B,7), black line = min/max across 64 pairs",
        (125, 105),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw legend");
    root.present().expect("present rate strip");
}

fn render_residue_funnel(rows: &[Base30ResidueFunnelRow], path: &Path) {
    let root = BitMapBackend::new(path, (1550, 820)).into_drawing_area();
    root.fill(&WHITE).expect("fill funnel");
    root.draw(&Text::new(
        "(B,7) Residue Funnel: Raw Seeds -> Survivors -> Prime Witnesses",
        (34, 38),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");

    let left = 330;
    let top = 95;
    let bar_h = 44;
    let row_gap = 70;
    let max_survivors = rows.iter().map(|row| row.survivor_count).max().unwrap_or(1) as f64;
    let max_width = 820.0;

    for (idx, row) in rows.iter().enumerate() {
        let y = top + idx as i32 * row_gap;
        root.draw(&Text::new(
            truncate(&row.modulus_set_label, 27),
            (35, y + 30),
            ("sans-serif", 17).into_font().style(FontStyle::Bold),
        ))
        .expect("draw label");
        let survivor_width = (row.survivor_count as f64 / max_survivors * max_width) as i32;
        root.draw(&Rectangle::new(
            [(left, y), (left + survivor_width, y + bar_h)],
            RGBColor(76, 150, 108).filled(),
        ))
        .expect("draw survivor bar");
        let prime_width = (row.prime_count as f64 / max_survivors * max_width) as i32;
        root.draw(&Rectangle::new(
            [(left, y + bar_h + 4), (left + prime_width, y + bar_h + 18)],
            RGBColor(48, 88, 170).filled(),
        ))
        .expect("draw prime bar");
        root.draw(&Text::new(
            format!(
                "{} survivors, {} primes, {:.2}% prime/survivor",
                row.survivor_count,
                row.prime_count,
                row.prime_rate_among_survivors * 100.0
            ),
            (left + survivor_width + 18, y + 29),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw values");
    }

    root.draw(&Text::new(
        "green = residue survivors, blue = prime witnesses retained by the same funnel",
        (35, 780),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw note");
    root.present().expect("present funnel");
}

fn render_witness_gallery(
    top_pair_rows: &[Base30TopPairRow],
    witness_rows: &[Base30WitnessRow],
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1500, 930)).into_drawing_area();
    root.fill(&WHITE).expect("fill gallery");
    root.draw(&Text::new(
        "Base 30 Top Compact Pairs: First Witnesses",
        (36, 38),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");

    let card_w = 460;
    let card_h = 240;
    for (idx, pair) in top_pair_rows.iter().enumerate() {
        let x = 35 + (idx % 3) as i32 * 485;
        let y = 95 + (idx / 3) as i32 * 265;
        let is_target = pair.outer == BASE30_TARGET_OUTER && pair.inner == BASE30_TARGET_INNER;
        let fill = if is_target {
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
            format!("{}  rank #{}", pair.pair_label, pair.k00_rank),
            (x + 18, y + 31),
            ("sans-serif", 22).into_font().style(FontStyle::Bold),
        ))
        .expect("draw pair");
        root.draw(&Text::new(
            format!(
                "k00 rate {:.2}% | hits {} / 900",
                pair.rate_k00 * 100.0,
                pair.prime_hits_k00
            ),
            (x + 18, y + 65),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw metrics");
        for (line_idx, witness) in witness_rows
            .iter()
            .filter(|witness| witness.outer == pair.outer && witness.inner == pair.inner)
            .take(4)
            .enumerate()
        {
            root.draw(&Text::new(
                format!(
                    "{} = {}",
                    truncate(&witness.template_digits, 18),
                    witness.decimal_value
                ),
                (x + 18, y + 105 + line_idx as i32 * 29),
                ("sans-serif", 15).into_font(),
            ))
            .expect("draw witness");
        }
    }

    root.present().expect("present gallery");
}

fn heat_color(value: f64, max_value: f64) -> RGBColor {
    let t = (value / max_value).clamp(0.0, 1.0);
    let red = (246.0 - 120.0 * t) as u8;
    let green = (235.0 - 10.0 * (1.0 - t)) as u8;
    let blue = (220.0 - 160.0 * t) as u8;
    RGBColor(red, green, blue)
}

fn digit_label(digit: u32) -> String {
    if digit < 10 {
        digit.to_string()
    } else {
        char::from_u32('A' as u32 + digit - 10)
            .expect("digit should render")
            .to_string()
    }
}

fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    let mut out = value
        .chars()
        .take(max_chars.saturating_sub(3))
        .collect::<String>();
    out.push_str("...");
    out
}
