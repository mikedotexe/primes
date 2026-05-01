//! Affine singular-series scout report.
//!
//! This report estimates how much of an affine membrane lane's observed yield
//! is explained by finite small-prime residue weather before treating any
//! residual as density signal.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example affine_singular_series_report -- --out-dir /tmp/primes_affine_singular_series
//! ```

use plotters::prelude::*;
use primes::validation::{
    affine_singular_series::{
        build_affine_singular_series_report, AffineSingularLaneRow, AffineSingularModulusRow,
        AffineSingularRankRow, AffineSingularSeriesReport, AffineSingularSeriesSettings,
        AFFINE_SINGULAR_SERIES_EXPECTED_OUTPUTS, DEFAULT_SINGULAR_EXACT_SEED_CAP,
        DEFAULT_SINGULAR_MAX_WITNESSES, DEFAULT_SINGULAR_PRIME_BOUND,
        DEFAULT_SINGULAR_SAMPLE_COUNT, DEFAULT_SINGULAR_SAMPLE_SEED, DEFAULT_SINGULAR_TOP_LIMIT,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use serde::Serialize;
use std::{
    collections::BTreeSet,
    env,
    path::{Path, PathBuf},
};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_affine_singular_series";
const ARTIFACT_ID: &str = "affine_singular_series_report";
const EXPORT_VERSION: u32 = 1;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    settings: AffineSingularSeriesSettings,
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
    report: AffineSingularSeriesReport,
    image_artifact_rows: Vec<ImageArtifactRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output directory");

    let report = build_affine_singular_series_report(options.settings);
    let residual_ranking_path = options.out_dir.join("residual_ranking.png");
    render_residual_ranking(&report.lane_rows, &residual_ranking_path);
    let multiplier_path = options.out_dir.join("multiplier_decomposition.png");
    render_multiplier_decomposition(&report.lane_rows, &multiplier_path);
    let heatmap_path = options.out_dir.join("modulus_gate_heatmap.png");
    render_modulus_gate_heatmap(&report.modulus_rows, &heatmap_path);
    let gallery_path = options.out_dir.join("lead_gallery.png");
    render_lead_gallery(&report.residual_rank_rows, &gallery_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "residual_ranking".to_string(),
            label: "Residual ranking after finite residue expectation".to_string(),
            path: residual_ranking_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "multiplier_decomposition".to_string(),
            label: "Base-coprime and finite residue multiplier decomposition".to_string(),
            path: multiplier_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "modulus_gate_heatmap".to_string(),
            label: "Small-prime modulus gate classification heatmap".to_string(),
            path: heatmap_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "lead_gallery".to_string(),
            label: "Positive leads and absolute foils gallery".to_string(),
            path: gallery_path.display().to_string(),
        },
    ];

    write_csv_rows(options.out_dir.join("lane_rows.csv"), &report.lane_rows)
        .expect("write lane rows");
    write_csv_rows(
        options.out_dir.join("modulus_rows.csv"),
        &report.modulus_rows,
    )
    .expect("write modulus rows");
    write_csv_rows(
        options.out_dir.join("residual_rank_rows.csv"),
        &report.residual_rank_rows,
    )
    .expect("write residual rank rows");
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
        .expect("write report");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo run --release --example affine_singular_series_report"
                .to_string(),
            args: env::args().skip(1).collect(),
            upstream_inputs: vec![
                "src/validation/affine_singular_series.rs".to_string(),
                "src/validation/fast_affine.rs".to_string(),
                "src/validation/construction_density.rs".to_string(),
                "src/validation/affine_phase_residual.rs".to_string(),
            ],
            expected_outputs: AFFINE_SINGULAR_SERIES_EXPECTED_OUTPUTS
                .iter()
                .map(|item| item.to_string())
                .collect(),
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote affine singular-series scout report to {}",
        options.out_dir.display()
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut settings = AffineSingularSeriesSettings::default();
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir")),
            "--prime-bound" => settings.prime_bound = parse_next(&mut args, "--prime-bound"),
            "--exact-seed-cap" => {
                settings.exact_seed_cap = parse_next(&mut args, "--exact-seed-cap");
            }
            "--sample-count" => settings.sample_count = parse_next(&mut args, "--sample-count"),
            "--sample-seed" => settings.sample_seed = parse_next(&mut args, "--sample-seed"),
            "--max-witnesses" => {
                settings.max_witnesses = parse_next(&mut args, "--max-witnesses");
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

    if settings.prime_bound < 2 {
        eprintln!("--prime-bound must be at least 2");
        std::process::exit(2);
    }
    if settings.exact_seed_cap == 0 || settings.sample_count == 0 || settings.top_limit == 0 {
        eprintln!("seed caps, sample count, and top limit must be positive");
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
    println!("Affine Singular-Series Scout Report");
    println!();
    println!("Options:");
    println!("  --out-dir <path>          Output directory (default: {DEFAULT_OUT_DIR})");
    println!(
        "  --prime-bound <n>         Gate primes <= n (default: {DEFAULT_SINGULAR_PRIME_BOUND})"
    );
    println!("  --exact-seed-cap <n>      Exact enumeration cap (default: {DEFAULT_SINGULAR_EXACT_SEED_CAP})");
    println!("  --sample-count <n>        Deterministic sample count (default: {DEFAULT_SINGULAR_SAMPLE_COUNT})");
    println!("  --sample-seed <n>         Deterministic sample seed (default: {DEFAULT_SINGULAR_SAMPLE_SEED})");
    println!("  --max-witnesses <n>       Witnesses per lane (default: {DEFAULT_SINGULAR_MAX_WITNESSES})");
    println!(
        "  --top-limit <n>           Rows per rank queue (default: {DEFAULT_SINGULAR_TOP_LIMIT})"
    );
}

fn render_report(bundle: &ReportBundle) -> String {
    let report = &bundle.report;
    let summary = &report.summary;
    let mut lines = Vec::new();
    lines.push("# Affine Singular-Series Scout Report".to_string());
    lines.push(String::new());
    lines.push(summary.strong_line.clone());
    lines.push(summary.caution_line.clone());
    lines.push(String::new());
    lines.push("## Settings".to_string());
    lines.push(format!(
        "- prime gate bound: `{}`",
        report.settings.prime_bound
    ));
    lines.push(format!(
        "- exact seed cap: `{}`; sample count: `{}`; sample seed: `{}`",
        report.settings.exact_seed_cap, report.settings.sample_count, report.settings.sample_seed
    ));
    lines.push(format!(
        "- lanes: `{}`; modulus rows: `{}`; witnesses: `{}`",
        summary.lane_count, summary.modulus_row_count, summary.witness_count
    ));
    lines.push(String::new());
    lines.push("## Headline Leads".to_string());
    lines.push(format!(
        "- strongest positive residual: `{}` at `{:.3}` pp",
        summary.strongest_positive_role, summary.strongest_positive_residual_pp
    ));
    lines.push(format!(
        "- strongest absolute residual: `{}` at `{:.3}` pp",
        summary.strongest_absolute_role, summary.strongest_absolute_residual_pp
    ));
    lines.push(format!(
        "- structurally blocked lanes: `{}`",
        summary.structurally_blocked_lane_count
    ));
    lines.push(String::new());
    lines.push("## How To Read The Prediction".to_string());
    lines.push("The report estimates `mean(1/ln(N(s)))`, then multiplies by a finite residue-weather factor: observed survivor share divided by the random expectation `prod(1 - 1/p)` for the included small primes.".to_string());
    lines.push("That multiplier includes base-safe primes, active seed gates, inactive-safe gates, and structurally blocked cases. It is a finite control layer, not an asymptotic singular-series theorem.".to_string());
    lines.push(String::new());
    lines.push("## Ranked Residuals".to_string());
    lines.push(
        "| reason | rank | role | lane | observed | residue expected | residual | tag |"
            .to_string(),
    );
    lines.push("|---|---:|---|---|---:|---:|---:|---|".to_string());
    for row in &report.residual_rank_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | base `{}` `{}` M=`{}` `{}` | `{:.3}%` | `{:.3}%` | `{:.3}` pp | `{}` |",
            row.selection_reason,
            row.rank,
            row.role,
            row.base,
            row.pair_label,
            row.middle_length,
            row.k_label,
            row.observed_prime_rate * 100.0,
            row.residue_adjusted_expected_density * 100.0,
            row.residual_vs_residue_expected_pp,
            row.residual_tag
        ));
    }
    lines.push(String::new());
    lines.push("## Representative Lanes".to_string());
    lines.push("| role | observed | PNT | residue expected | finite multiplier | survivor share | first witness |".to_string());
    lines.push("|---|---:|---:|---:|---:|---:|---:|".to_string());
    for row in report.lane_rows.iter().take(18) {
        lines.push(format!(
            "| `{}` | `{:.3}%` | `{:.3}%` | `{:.3}%` | `{:.3}` | `{:.3}%` | `{}` |",
            row.role,
            row.observed_prime_rate * 100.0,
            row.pnt_expected_density * 100.0,
            row.residue_adjusted_expected_density * 100.0,
            row.finite_residue_multiplier,
            row.survivor_share * 100.0,
            empty_dash(&row.first_witness_value)
        ));
    }
    lines.push(String::new());
    lines.push("## Visuals".to_string());
    for image in &bundle.image_artifact_rows {
        lines.push(format!("- `{}`: `{}`", image.label, image.path));
    }
    lines.push(String::new());
    lines.push("## Claim Boundary".to_string());
    lines.push("A positive row is a lead queue entry for later witness-ladder expansion. It is not a density theorem, and it does not replace matched controls.".to_string());
    lines.push(String::new());
    lines.join("\n")
}

fn render_residual_ranking(rows: &[AffineSingularLaneRow], path: &Path) {
    let mut ranked = rows.iter().collect::<Vec<_>>();
    ranked.sort_by(|left, right| {
        right
            .abs_residual_vs_residue_expected_pp
            .total_cmp(&left.abs_residual_vs_residue_expected_pp)
    });
    ranked.truncate(16);
    let root = BitMapBackend::new(path, (1280, 820)).into_drawing_area();
    root.fill(&WHITE).expect("fill residual ranking");
    root.draw(&Text::new(
        "Residual After Finite Residue Expectation",
        (35, 38),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");

    let min_y = ranked
        .iter()
        .map(|row| row.residual_vs_residue_expected_pp)
        .fold(0.0_f64, f64::min)
        .min(-1.0);
    let max_y = ranked
        .iter()
        .map(|row| row.residual_vs_residue_expected_pp)
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let chart_area = root.margin(70, 35, 80, 280);
    let mut chart = ChartBuilder::on(&chart_area)
        .caption("percentage points", ("sans-serif", 18))
        .x_label_area_size(30)
        .y_label_area_size(65)
        .build_cartesian_2d(0..ranked.len(), min_y * 1.15..max_y * 1.15)
        .expect("build chart");
    chart
        .configure_mesh()
        .disable_x_mesh()
        .y_desc("observed - residue expected")
        .x_labels(1)
        .draw()
        .expect("draw mesh");
    chart
        .draw_series(ranked.iter().enumerate().map(|(index, row)| {
            let color = if row.residual_vs_residue_expected_pp >= 0.0 {
                GREEN.mix(0.72)
            } else {
                RED.mix(0.72)
            };
            Rectangle::new(
                [
                    (index, 0.0),
                    (index + 1, row.residual_vs_residue_expected_pp),
                ],
                color.filled(),
            )
        }))
        .expect("draw bars");
    for (index, row) in ranked.iter().enumerate() {
        root.draw(&Text::new(
            format!(
                "{}  {:.2} pp",
                row.role, row.residual_vs_residue_expected_pp
            ),
            (55, 125 + index as i32 * 37),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw label");
    }
}

fn render_multiplier_decomposition(rows: &[AffineSingularLaneRow], path: &Path) {
    let mut ranked = rows.iter().collect::<Vec<_>>();
    ranked.sort_by(|left, right| {
        right
            .finite_residue_multiplier
            .total_cmp(&left.finite_residue_multiplier)
    });
    ranked.truncate(14);
    let root = BitMapBackend::new(path, (1280, 820)).into_drawing_area();
    root.fill(&WHITE).expect("fill multiplier chart");
    root.draw(&Text::new(
        "Base-Coprime And Finite Residue Multipliers",
        (35, 38),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");
    let max_y = ranked
        .iter()
        .flat_map(|row| [row.base_coprime_multiplier, row.finite_residue_multiplier])
        .fold(1.0_f64, f64::max);
    let chart_area = root.margin(80, 45, 210, 70);
    let mut chart = ChartBuilder::on(&chart_area)
        .x_label_area_size(35)
        .y_label_area_size(70)
        .build_cartesian_2d(0..ranked.len() * 2, 0.0..max_y * 1.2)
        .expect("build chart");
    chart
        .configure_mesh()
        .disable_x_mesh()
        .x_labels(1)
        .y_desc("multiplier")
        .draw()
        .expect("draw mesh");
    chart
        .draw_series(ranked.iter().enumerate().flat_map(|(index, row)| {
            let x = index * 2;
            [
                Rectangle::new(
                    [(x, 0.0), (x + 1, row.base_coprime_multiplier)],
                    BLUE.filled(),
                ),
                Rectangle::new(
                    [(x + 1, 0.0), (x + 2, row.finite_residue_multiplier)],
                    MAGENTA.filled(),
                ),
            ]
        }))
        .expect("draw bars");
    for (index, row) in ranked.iter().enumerate() {
        root.draw(&Text::new(
            format!(
                "{}: base {:.2}, finite {:.2}",
                row.role, row.base_coprime_multiplier, row.finite_residue_multiplier
            ),
            (55, 595 + index as i32 * 16),
            ("sans-serif", 14).into_font(),
        ))
        .expect("draw labels");
    }
    root.draw(&Text::new(
        "blue = base-coprime multiplier; magenta = finite residue multiplier",
        (760, 92),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw legend");
}

fn render_modulus_gate_heatmap(rows: &[AffineSingularModulusRow], path: &Path) {
    let mut lane_keys = rows
        .iter()
        .map(|row| row.role.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .take(28)
        .collect::<Vec<_>>();
    lane_keys.sort();
    let moduli = rows
        .iter()
        .map(|row| row.modulus)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (1320, 940)).into_drawing_area();
    root.fill(&WHITE).expect("fill heatmap");
    root.draw(&Text::new(
        "Finite Small-Prime Gate Classes",
        (35, 38),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");
    let left = 270i32;
    let top = 92i32;
    let cell_w = 34i32;
    let cell_h = 25i32;
    for (col, modulus) in moduli.iter().enumerate() {
        root.draw(&Text::new(
            modulus.to_string(),
            (left + col as i32 * cell_w + 6, top - 13),
            ("sans-serif", 12).into_font(),
        ))
        .expect("draw modulus");
    }
    for (row_index, role) in lane_keys.iter().enumerate() {
        let y = top + row_index as i32 * cell_h;
        root.draw(&Text::new(
            role.clone(),
            (25, y + 16),
            ("sans-serif", 13).into_font(),
        ))
        .expect("draw role");
        for (col, modulus) in moduli.iter().enumerate() {
            let class = rows
                .iter()
                .find(|row| row.role == *role && row.modulus == *modulus)
                .map(|row| row.classification.as_str())
                .unwrap_or("missing");
            let color = match class {
                "base_safe" => BLUE.mix(0.75),
                "active_seed_gate" => GREEN.mix(0.75),
                "inactive_safe" => CYAN.mix(0.55),
                "structurally_blocked" => RED.mix(0.8),
                _ => BLACK.mix(0.2),
            };
            let x = left + col as i32 * cell_w;
            root.draw(&Rectangle::new(
                [(x, y), (x + cell_w - 3, y + cell_h - 3)],
                color.filled(),
            ))
            .expect("draw cell");
        }
    }
    root.draw(&Text::new(
        "blue base_safe; green active_seed_gate; cyan inactive_safe; red structurally_blocked",
        (35, 875),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw legend");
}

fn render_lead_gallery(rows: &[AffineSingularRankRow], path: &Path) {
    let root = BitMapBackend::new(path, (1280, 820)).into_drawing_area();
    root.fill(&WHITE).expect("fill gallery");
    root.draw(&Text::new(
        "Finite Singular-Profile Lead Queue",
        (35, 40),
        ("sans-serif", 32).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");
    for (index, row) in rows.iter().take(16).enumerate() {
        let y = 95 + index as i32 * 42;
        let color = if row.residual_vs_residue_expected_pp >= 0.0 {
            GREEN.mix(0.75)
        } else {
            RED.mix(0.75)
        };
        root.draw(&Rectangle::new([(35, y - 18), (55, y + 8)], color.filled()))
            .expect("draw swatch");
        root.draw(&Text::new(
            format!(
                "{} #{} {} base {} {} M={} residual {:.3} pp; witness {}",
                row.selection_reason,
                row.rank,
                row.role,
                row.base,
                row.pair_label,
                row.middle_length,
                row.residual_vs_residue_expected_pp,
                empty_dash(&row.first_witness_value)
            ),
            (70, y),
            ("sans-serif", 18).into_font(),
        ))
        .expect("draw row");
    }
    root.draw(&Text::new(
        "Lead means follow-up target, not theorem.",
        (35, 785),
        ("sans-serif", 20).into_font().style(FontStyle::Bold),
    ))
    .expect("draw footer");
}

fn empty_dash(value: &str) -> &str {
    if value.is_empty() {
        "-"
    } else {
        value
    }
}
