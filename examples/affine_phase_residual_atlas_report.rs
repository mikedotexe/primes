//! Cross-base affine phase residual atlas report.
//!
//! This report generalizes the compact base-30 reversal residual into a
//! cross-base signal-discovery atlas. It compares `(low, high)` with
//! `(high, low)` across compact `k=(0,0)` lanes and keeps the claim boundary
//! conservative: ranked local affine phase leads, not density theorems.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example affine_phase_residual_atlas_report -- --out-dir /tmp/primes_affine_phase_residual_atlas
//! ```

use plotters::prelude::*;
use primes::validation::{
    affine_phase_residual::{
        build_affine_phase_residual_atlas, AffinePhaseBaseSummaryRow, AffinePhaseResidualAtlas,
        AffinePhaseResidualRow, AffinePhaseResidualSettings, AffinePhaseTopSignalRow,
        DEFAULT_PHASE_RESIDUAL_MAX_MIDDLE_LENGTH, DEFAULT_PHASE_RESIDUAL_MIN_MIDDLE_LENGTH,
        DEFAULT_PHASE_RESIDUAL_TOP_LIMIT, DEFAULT_PHASE_RESIDUAL_WITNESS_LIMIT,
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

const DEFAULT_OUT_DIR: &str = "/tmp/primes_affine_phase_residual_atlas";
const ARTIFACT_ID: &str = "affine_phase_residual_atlas_report";
const EXPORT_VERSION: u32 = 1;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    settings: AffinePhaseResidualSettings,
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
    atlas: AffinePhaseResidualAtlas,
    image_artifact_rows: Vec<ImageArtifactRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output directory");

    let atlas = build_affine_phase_residual_atlas(options.settings.clone());
    let overview_path = options.out_dir.join("cross_base_residual_overview.png");
    render_cross_base_overview(
        &atlas.base_summary_rows,
        options.settings.max_middle_length,
        &overview_path,
    );
    let raw_scatter_path = options.out_dir.join("raw_vs_size_scatter.png");
    render_raw_vs_size_scatter(
        &atlas.phase_residual_rows,
        options.settings.max_middle_length,
        &raw_scatter_path,
    );
    let survivor_scatter_path = options.out_dir.join("survivor_prime_residual_scatter.png");
    render_survivor_prime_scatter(
        &atlas.phase_residual_rows,
        options.settings.max_middle_length,
        &survivor_scatter_path,
    );
    let base30_path = options.out_dir.join("base30_comparison_panel.png");
    render_base30_panel(
        &atlas.phase_residual_rows,
        options.settings.max_middle_length,
        &base30_path,
    );
    let gallery_path = options.out_dir.join("top_signal_gallery.png");
    render_top_signal_gallery(&atlas.top_signal_rows, &gallery_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "cross_base_residual_overview".to_string(),
            label: "Cross-base residual overview".to_string(),
            path: overview_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "raw_vs_size_scatter".to_string(),
            label: "Raw delta versus PNT size-expected delta".to_string(),
            path: raw_scatter_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "survivor_prime_residual_scatter".to_string(),
            label: "Residual-after-size versus survivor-prime residual".to_string(),
            path: survivor_scatter_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "base30_comparison_panel".to_string(),
            label: "Base-30 anchor comparison panel".to_string(),
            path: base30_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "top_signal_gallery".to_string(),
            label: "Top affine phase residual lead queue".to_string(),
            path: gallery_path.display().to_string(),
        },
    ];

    write_csv_rows(
        options.out_dir.join("phase_residual_rows.csv"),
        &atlas.phase_residual_rows,
    )
    .expect("write phase residual rows");
    write_csv_rows(
        options.out_dir.join("phase_modulus_rows.csv"),
        &atlas.phase_modulus_rows,
    )
    .expect("write phase modulus rows");
    write_csv_rows(
        options.out_dir.join("base_summary_rows.csv"),
        &atlas.base_summary_rows,
    )
    .expect("write base summary rows");
    write_csv_rows(
        options.out_dir.join("top_signal_rows.csv"),
        &atlas.top_signal_rows,
    )
    .expect("write top signal rows");
    write_csv_rows(
        options.out_dir.join("witness_rows.csv"),
        &atlas.witness_rows,
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
        atlas,
        image_artifact_rows,
    };
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_text_file(options.out_dir.join("report.md"), &render_report(&bundle))
        .expect("write report markdown");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo run --release --example affine_phase_residual_atlas_report"
                .to_string(),
            args: env::args().skip(1).collect(),
            upstream_inputs: vec![
                "src/validation/affine_phase_residual.rs".to_string(),
                "src/validation/fast_affine.rs".to_string(),
                "src/validation/bounded_k.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "phase_residual_rows.csv".to_string(),
                "phase_modulus_rows.csv".to_string(),
                "base_summary_rows.csv".to_string(),
                "top_signal_rows.csv".to_string(),
                "witness_rows.csv".to_string(),
                "image_artifact_rows.csv".to_string(),
                "cross_base_residual_overview.png".to_string(),
                "raw_vs_size_scatter.png".to_string(),
                "survivor_prime_residual_scatter.png".to_string(),
                "base30_comparison_panel.png".to_string(),
                "top_signal_gallery.png".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote affine phase residual atlas to {}",
        options.out_dir.display()
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut settings = AffinePhaseResidualSettings::default();
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
            "--min-middle-length" => {
                settings.min_middle_length = parse_next(&mut args, "--min-middle-length");
            }
            "--max-middle-length" => {
                settings.max_middle_length = parse_next(&mut args, "--max-middle-length");
            }
            "--top-limit" => settings.top_limit = parse_next(&mut args, "--top-limit"),
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

    if settings.bases.is_empty() {
        eprintln!("--bases must contain at least one base");
        std::process::exit(2);
    }
    if settings.min_middle_length == 0 || settings.max_middle_length < settings.min_middle_length {
        eprintln!("middle length range must be nonempty and start at least at 1");
        std::process::exit(2);
    }
    if settings.top_limit == 0 || settings.witness_limit == 0 {
        eprintln!("--top-limit and --witness-limit must be at least 1");
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
    println!("Affine Phase Residual Atlas Report");
    println!();
    println!("Options:");
    println!("  --out-dir <path>             Output directory (default: {DEFAULT_OUT_DIR})");
    println!("  --bases <csv>                Bases to sweep (default: 6,10,14,22,26,30,34)");
    println!(
        "  --min-middle-length <n>      Minimum M (default: {DEFAULT_PHASE_RESIDUAL_MIN_MIDDLE_LENGTH})"
    );
    println!(
        "  --max-middle-length <n>      Maximum M (default: {DEFAULT_PHASE_RESIDUAL_MAX_MIDDLE_LENGTH})"
    );
    println!(
        "  --top-limit <n>              Rows per top ranking surface (default: {DEFAULT_PHASE_RESIDUAL_TOP_LIMIT})"
    );
    println!(
        "  --witness-limit <n>          Witnesses per selected direction (default: {DEFAULT_PHASE_RESIDUAL_WITNESS_LIMIT})"
    );
}

fn render_report(bundle: &ReportBundle) -> String {
    let atlas = &bundle.atlas;
    let summary = &atlas.summary;
    let mut lines = Vec::new();
    lines.push("# Affine Phase Residual Atlas".to_string());
    lines.push(String::new());
    lines.push(summary.strong_line.clone());
    lines.push(summary.caution_line.clone());
    lines.push(String::new());
    lines.push("## What This Atlas Is Asking".to_string());
    lines.push("For each compact lane, the atlas compares `(low, high)` with `(high, low)`. The grammar and gradient are shared; the affine shift changes. That makes this a controlled local phase question.".to_string());
    lines.push(String::new());
    lines.push("```text".to_string());
    lines.push("raw prime-rate delta".to_string());
    lines.push("-> size / PNT-expected delta".to_string());
    lines.push("-> exact small-prime residue survival".to_string());
    lines.push("-> survivor-prime residual".to_string());
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push("## Headline Leads".to_string());
    lines.push(format!(
        "- Rows: `{}` residual rows and `{}` modulus-phase rows across `{}` bases",
        summary.residual_row_count, summary.phase_modulus_row_count, summary.base_count
    ));
    lines.push(format!(
        "- Base-30 anchor `{}` vs `{}` at M=3: raw `{:+.3}` pp, residual-after-size `{:+.3}` pp, survivor-prime residual `{:+.3}` pp",
        summary.anchor_pair_label,
        summary.anchor_reverse_pair_label,
        summary.anchor_m3_raw_delta_pp,
        summary.anchor_m3_residual_after_size_pp,
        summary.anchor_m3_survivor_prime_residual_delta_pp
    ));
    lines.push(format!(
        "- Strongest size residual: base `{}`, M=`{}`, `{}` vs `{}` at `{:+.3}` pp",
        summary.strongest_size_residual_base,
        summary.strongest_size_residual_m,
        summary.strongest_size_residual_pair,
        summary.strongest_size_residual_reverse_pair,
        summary.strongest_size_residual_pp
    ));
    lines.push(format!(
        "- Strongest survivor-prime residual: base `{}`, M=`{}`, `{}` vs `{}` at `{:+.3}` pp",
        summary.strongest_survivor_prime_base,
        summary.strongest_survivor_prime_m,
        summary.strongest_survivor_prime_pair,
        summary.strongest_survivor_prime_reverse_pair,
        summary.strongest_survivor_prime_residual_pp
    ));
    lines.push(String::new());
    lines.push("## Base Summary".to_string());
    lines.push("| base | M | pairs | mean abs raw | mean abs size residual | mean abs survivor-prime residual | strongest size residual |".to_string());
    lines.push("|---:|---:|---:|---:|---:|---:|---|".to_string());
    for row in atlas
        .base_summary_rows
        .iter()
        .filter(|row| row.middle_length == atlas.settings.max_middle_length)
    {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{:.3}` pp | `{:.3}` pp | `{:.3}` pp | `{}` vs `{}` `{:+.3}` pp |",
            row.base,
            row.middle_length,
            row.unordered_pair_count,
            row.mean_abs_raw_delta_pp,
            row.mean_abs_residual_after_size_pp,
            row.mean_abs_survivor_prime_residual_delta_pp,
            row.strongest_size_residual_pair,
            row.strongest_size_residual_reverse_pair,
            row.strongest_size_residual_pp
        ));
    }
    lines.push(String::new());
    lines.push("## Lead Queue".to_string());
    lines.push("| reason | base | M | pair | raw | size expected | raw-size | survivor delta | survivor-prime residual | tag |".to_string());
    lines.push("|---|---:|---:|---|---:|---:|---:|---:|---:|---|".to_string());
    for row in atlas.top_signal_rows.iter().take(32) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` vs `{}` | `{:+.3}` pp | `{:+.3}` pp | `{:+.3}` pp | `{:+.3}` pp | `{:+.3}` pp | `{}` |",
            row.selection_reason,
            row.base,
            row.middle_length,
            row.low_high_pair_label,
            row.high_low_pair_label,
            row.raw_delta_pp,
            row.size_expected_delta_pp,
            row.residual_after_size_pp,
            row.residue_survivor_delta_pp,
            row.survivor_prime_residual_delta_pp,
            row.lead_tag
        ));
    }
    lines.push(String::new());
    lines.push(format!(
        "## Mature M={} Lead Queue",
        atlas.settings.max_middle_length
    ));
    lines.push("Short `M=1` lanes are intentionally included in the scout surface, but the mature view is usually a better follow-up queue because the seed space is larger.".to_string());
    lines.push(String::new());
    lines.push("| reason | base | pair | raw-size | survivor-prime residual | tag |".to_string());
    lines.push("|---|---:|---|---:|---:|---|".to_string());
    for row in atlas
        .top_signal_rows
        .iter()
        .filter(|row| row.selection_reason.starts_with("top_mature"))
        .take(24)
    {
        lines.push(format!(
            "| `{}` | `{}` | `{}` vs `{}` | `{:+.3}` pp | `{:+.3}` pp | `{}` |",
            row.selection_reason,
            row.base,
            row.low_high_pair_label,
            row.high_low_pair_label,
            row.residual_after_size_pp,
            row.survivor_prime_residual_delta_pp,
            row.lead_tag
        ));
    }
    lines.push(String::new());
    lines.push("## First Witnesses".to_string());
    lines.push("| base | M | pair | seed | template | decimal value |".to_string());
    lines.push("|---:|---:|---|---:|---|---:|".to_string());
    for row in atlas.witness_rows.iter().take(64) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |",
            row.base,
            row.middle_length,
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
    lines.push("This atlas ranks exact finite compact reversal lanes. It does not claim asymptotic density lift, and it does not say survivor-prime residuals are independent of controls that have not yet been run.".to_string());
    lines.push(String::new());
    lines.join("\n")
}

fn render_cross_base_overview(
    rows: &[AffinePhaseBaseSummaryRow],
    middle_length: usize,
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1180, 820)).into_drawing_area();
    root.fill(&WHITE).expect("fill overview");
    let m_rows = rows
        .iter()
        .filter(|row| row.middle_length == middle_length)
        .collect::<Vec<_>>();
    let max_y = m_rows
        .iter()
        .flat_map(|row| {
            [
                row.mean_abs_residual_after_size_pp,
                row.mean_abs_survivor_prime_residual_delta_pp,
                row.strongest_size_residual_pp.abs(),
            ]
        })
        .fold(0.0_f64, f64::max)
        .max(0.01);
    let x_max = m_rows.len() as f64 + 0.5;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Affine Phase Residual Overview (M={middle_length})"),
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(70)
        .y_label_area_size(90)
        .build_cartesian_2d(0.5_f64..x_max, 0.0_f64..(max_y * 1.25))
        .expect("build overview chart");

    chart
        .configure_mesh()
        .x_labels(m_rows.len())
        .x_desc("base")
        .y_desc("absolute delta (percentage points)")
        .x_label_formatter(&|value| {
            let idx = value.round() as usize;
            if idx == 0 || idx > m_rows.len() {
                String::new()
            } else {
                m_rows[idx - 1].base.to_string()
            }
        })
        .y_label_formatter(&|value| format!("{value:.2}"))
        .draw()
        .expect("draw overview mesh");

    for (idx, row) in m_rows.iter().enumerate() {
        let x = idx as f64 + 1.0;
        chart
            .draw_series(std::iter::once(Circle::new(
                (x, row.mean_abs_residual_after_size_pp),
                7,
                BLUE.filled(),
            )))
            .expect("draw mean residual point");
        chart
            .draw_series(std::iter::once(TriangleMarker::new(
                (x, row.mean_abs_survivor_prime_residual_delta_pp),
                8,
                RGBColor(30, 140, 90).filled(),
            )))
            .expect("draw survivor point");
        chart
            .draw_series(std::iter::once(Cross::new(
                (x, row.strongest_size_residual_pp.abs()),
                10,
                RED.stroke_width(3),
            )))
            .expect("draw max point");
    }

    root.draw(&Text::new(
        "blue = mean |raw-size|, green = mean |survivor-prime residual|, red = strongest |raw-size|",
        (85, 104),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw overview legend");
    root.present().expect("present overview");
}

fn render_raw_vs_size_scatter(rows: &[AffinePhaseResidualRow], middle_length: usize, path: &Path) {
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
            "Raw Delta vs PNT Size Baseline",
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(90)
        .y_label_area_size(90)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .expect("build raw scatter");

    chart
        .configure_mesh()
        .x_desc("PNT size-expected delta (percentage points)")
        .y_desc("raw prime-rate delta (percentage points)")
        .x_label_formatter(&|value| format!("{value:+.2}"))
        .y_label_formatter(&|value| format!("{value:+.2}"))
        .draw()
        .expect("draw scatter mesh");

    let diag_min = x_min.min(y_min);
    let diag_max = x_max.max(y_max);
    chart
        .draw_series(LineSeries::new(
            vec![(diag_min, diag_min), (diag_max, diag_max)],
            RGBColor(120, 120, 120).stroke_width(2),
        ))
        .expect("draw diagonal");

    for row in &m_rows {
        chart
            .draw_series(std::iter::once(Circle::new(
                (row.size_expected_delta_pp, row.raw_delta_pp),
                if is_base30_anchor(row) { 8 } else { 4 },
                base_color(row.base).filled(),
            )))
            .expect("draw raw scatter point");
    }

    root.draw(&Text::new(
        "above diagonal = raw reversal delta beats the size-only expectation",
        (85, 104),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw scatter note");
    root.present().expect("present raw scatter");
}

fn render_survivor_prime_scatter(
    rows: &[AffinePhaseResidualRow],
    middle_length: usize,
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1180, 840)).into_drawing_area();
    root.fill(&WHITE).expect("fill survivor scatter");
    let m_rows = rows
        .iter()
        .filter(|row| row.middle_length == middle_length)
        .collect::<Vec<_>>();
    let (x_min, x_max) = padded_range(m_rows.iter().map(|row| row.residual_after_size_pp));
    let (y_min, y_max) = padded_range(
        m_rows
            .iter()
            .map(|row| row.survivor_prime_residual_delta_pp),
    );

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Residual After Size vs Survivor-Prime Residual",
            ("sans-serif", 30).into_font(),
        )
        .margin(35)
        .x_label_area_size(90)
        .y_label_area_size(90)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .expect("build survivor scatter");

    chart
        .configure_mesh()
        .x_desc("raw minus size expectation (percentage points)")
        .y_desc("prime rate among survivors delta (percentage points)")
        .x_label_formatter(&|value| format!("{value:+.2}"))
        .y_label_formatter(&|value| format!("{value:+.2}"))
        .draw()
        .expect("draw survivor mesh");

    chart
        .draw_series(LineSeries::new(
            vec![(x_min, 0.0), (x_max, 0.0)],
            RGBColor(150, 150, 150).stroke_width(1),
        ))
        .expect("draw horizontal zero");
    chart
        .draw_series(LineSeries::new(
            vec![(0.0, y_min), (0.0, y_max)],
            RGBColor(150, 150, 150).stroke_width(1),
        ))
        .expect("draw vertical zero");

    for row in &m_rows {
        chart
            .draw_series(std::iter::once(Circle::new(
                (
                    row.residual_after_size_pp,
                    row.survivor_prime_residual_delta_pp,
                ),
                if is_base30_anchor(row) { 8 } else { 4 },
                base_color(row.base).filled(),
            )))
            .expect("draw survivor scatter point");
    }

    root.draw(&Text::new(
        "same direction in both axes is the cleanest next lead; opposite signs are useful foils",
        (85, 104),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw survivor note");
    root.present().expect("present survivor scatter");
}

fn render_base30_panel(rows: &[AffinePhaseResidualRow], middle_length: usize, path: &Path) {
    let root = BitMapBackend::new(path, (1120, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill base30 panel");
    root.draw(&Text::new(
        format!("Base 30 Residual-After-Size Panel (M={middle_length})"),
        (40, 40),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");

    let units = unit_residues(30);
    let m_rows = rows
        .iter()
        .filter(|row| row.base == 30 && row.middle_length == middle_length)
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
                .expect("base30 row should exist");
            let delta = if outer == low {
                row.residual_after_size_pp
            } else {
                -row.residual_after_size_pp
            };
            root.draw(&Rectangle::new(
                [(x, y), (x + cell - 8, y + cell - 8)],
                signed_color(delta, max_abs).filled(),
            ))
            .expect("draw base30 cell");
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
            .expect("draw pp");
        }
    }

    root.draw(&Text::new(
        "rows = outer, columns = inner; values are raw prime-rate delta minus size expectation",
        (115, 935),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw base30 note");
    root.present().expect("present base30 panel");
}

fn render_top_signal_gallery(rows: &[AffinePhaseTopSignalRow], path: &Path) {
    let root = BitMapBackend::new(path, (1500, 980)).into_drawing_area();
    root.fill(&WHITE).expect("fill gallery");
    root.draw(&Text::new(
        "Top Affine Phase Residual Leads",
        (36, 38),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw gallery title");

    let card_w = 460;
    let card_h = 250;
    for (idx, row) in rows.iter().take(9).enumerate() {
        let x = 35 + (idx % 3) as i32 * 485;
        let y = 95 + (idx / 3) as i32 * 275;
        let fill = if row.selection_reason == "base30_anchor" {
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
            format!(
                "base {} M{}: {} vs {}",
                row.base, row.middle_length, row.low_high_pair_label, row.high_low_pair_label
            ),
            (x + 18, y + 31),
            ("sans-serif", 20).into_font().style(FontStyle::Bold),
        ))
        .expect("draw card title");
        root.draw(&Text::new(
            format!("reason: {} / {}", row.selection_reason, row.lead_tag),
            (x + 18, y + 66),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw reason");
        root.draw(&Text::new(
            format!(
                "hits: {} / {}",
                row.low_high_prime_hits, row.high_low_prime_hits
            ),
            (x + 18, y + 96),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw hits");
        root.draw(&Text::new(
            format!("raw delta: {:+.3} pp", row.raw_delta_pp),
            (x + 18, y + 126),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw raw");
        root.draw(&Text::new(
            format!("size expected: {:+.3} pp", row.size_expected_delta_pp),
            (x + 18, y + 156),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw size");
        root.draw(&Text::new(
            format!("raw - size: {:+.3} pp", row.residual_after_size_pp),
            (x + 18, y + 186),
            ("sans-serif", 15).into_font().style(FontStyle::Bold),
        ))
        .expect("draw residual");
        root.draw(&Text::new(
            format!(
                "survivor-prime: {:+.3} pp",
                row.survivor_prime_residual_delta_pp
            ),
            (x + 18, y + 216),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw survivor");
    }

    root.present().expect("present gallery");
}

fn is_base30_anchor(row: &AffinePhaseResidualRow) -> bool {
    row.base == 30 && row.middle_length == 3 && row.low_digit == 1 && row.high_digit == 11
}

fn base_color(base: u32) -> RGBColor {
    match base {
        6 => RGBColor(52, 111, 184),
        10 => RGBColor(212, 95, 73),
        14 => RGBColor(54, 153, 112),
        22 => RGBColor(132, 90, 190),
        26 => RGBColor(226, 159, 71),
        30 => RGBColor(46, 139, 170),
        34 => RGBColor(165, 70, 118),
        _ => RGBColor(90, 90, 90),
    }
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
            .expect("base digit")
            .to_string()
    }
}
