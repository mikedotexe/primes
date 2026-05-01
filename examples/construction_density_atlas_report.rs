//! Construction density atlas report.
//!
//! This report maps density drift across maintained and stress-test affine
//! membrane prime families. It is intentionally conservative: high-yield lanes
//! are treated as controlled candidate surfaces, not as a density theorem.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example construction_density_atlas_report -- --out-dir /tmp/primes_construction_density_atlas
//! ```

use plotters::prelude::*;
use primes::validation::{
    construction_density::{
        measure_construction_density_atlas, ConstructionDensitySettings, ControlAtlasRow,
        DensityAtlasRow, WitnessAtlasRow, DEFAULT_CONSTRUCTION_DENSITY_SPECS,
        DEFAULT_DENSITY_EXACT_SEED_CAP, DEFAULT_DENSITY_MAX_WITNESSES,
        DEFAULT_DENSITY_SAMPLE_COUNT, DEFAULT_DENSITY_SAMPLE_SEED,
        DEFAULT_DENSITY_WHEEL_PERIOD_CAP,
    },
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

const DEFAULT_OUT_DIR: &str = "/tmp/primes_construction_density_atlas";
const ARTIFACT_ID: &str = "construction_density_atlas_report";
const EXPORT_VERSION: u32 = 1;
type DensityMetric = (&'static str, fn(&DensityAtlasRow) -> f64);

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    settings: ConstructionDensitySettings,
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
    settings: ConstructionDensitySettings,
    summary: primes::validation::construction_density::ConstructionDensitySummary,
    density_rows: Vec<DensityAtlasRow>,
    control_rows: Vec<ControlAtlasRow>,
    witness_rows: Vec<WitnessAtlasRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output directory");

    let atlas =
        measure_construction_density_atlas(DEFAULT_CONSTRUCTION_DENSITY_SPECS, options.settings);
    let observations = build_observations(&atlas.density_rows, &atlas.control_rows);

    let density_heatmap_path = options.out_dir.join("density_heatmap.png");
    render_density_heatmap(&atlas.density_rows, &density_heatmap_path);
    let zero_run_path = options.out_dir.join("zero_run_drift.png");
    render_zero_run_drift(&atlas.density_rows, &zero_run_path);
    let funnel_path = options.out_dir.join("layered_control_funnel.png");
    render_layered_control_funnel(&atlas.density_rows, &atlas.control_rows, &funnel_path);
    let gallery_path = options.out_dir.join("construction_gallery.png");
    render_construction_gallery(&atlas.density_rows, &atlas.witness_rows, &gallery_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "density_heatmap".to_string(),
            label: "Prime density and residue-filter heatmap".to_string(),
            path: density_heatmap_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "zero_run_drift".to_string(),
            label: "Zero-run drift panel".to_string(),
            path: zero_run_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "layered_control_funnel".to_string(),
            label: "Layered control funnel".to_string(),
            path: funnel_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "construction_gallery".to_string(),
            label: "Good-vs-lousy construction gallery".to_string(),
            path: gallery_path.display().to_string(),
        },
    ];

    let bundle = ReportBundle {
        export_version: EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings: options.settings,
        summary: atlas.summary.clone(),
        density_rows: atlas.density_rows.clone(),
        control_rows: atlas.control_rows.clone(),
        witness_rows: atlas.witness_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        observations: observations.clone(),
    };

    write_csv_rows(
        options.out_dir.join("density_rows.csv"),
        &atlas.density_rows,
    )
    .expect("write density rows");
    write_csv_rows(
        options.out_dir.join("control_rows.csv"),
        &atlas.control_rows,
    )
    .expect("write control rows");
    write_csv_rows(
        options.out_dir.join("witness_rows.csv"),
        &atlas.witness_rows,
    )
    .expect("write witness rows");
    write_csv_rows(
        options.out_dir.join("image_artifact_rows.csv"),
        &image_artifact_rows,
    )
    .expect("write image rows");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_text_file(
        options.out_dir.join("report.md"),
        &render_report(&bundle, &observations),
    )
    .expect("write report");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo run --release --example construction_density_atlas_report"
                .to_string(),
            args: env::args().skip(1).collect(),
            upstream_inputs: vec![
                "src/validation/construction_density.rs".to_string(),
                "src/validation/fast_affine.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "density_rows.csv".to_string(),
                "control_rows.csv".to_string(),
                "witness_rows.csv".to_string(),
                "density_heatmap.png".to_string(),
                "zero_run_drift.png".to_string(),
                "layered_control_funnel.png".to_string(),
                "construction_gallery.png".to_string(),
            ],
        },
    )
    .expect("write manifest");

    println!(
        "wrote construction density atlas to {}",
        options.out_dir.display()
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut settings = ConstructionDensitySettings::default();
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = PathBuf::from(parse_next::<String>(&mut args, "--out-dir")),
            "--sample-count" => {
                settings.sample_count = parse_next(&mut args, "--sample-count");
            }
            "--exact-seed-cap" => {
                settings.exact_seed_cap = parse_next(&mut args, "--exact-seed-cap");
            }
            "--max-witnesses" => {
                settings.max_witnesses = parse_next(&mut args, "--max-witnesses");
            }
            "--wheel-period-cap" => {
                settings.wheel_period_cap = parse_next(&mut args, "--wheel-period-cap");
            }
            "--sample-seed" => {
                settings.sample_seed = parse_next(&mut args, "--sample-seed");
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
    println!("Construction Density Atlas Report");
    println!();
    println!("Options:");
    println!("  --out-dir <path>           Output directory (default: {DEFAULT_OUT_DIR})");
    println!(
        "  --sample-count <n>         Sampled seeds per large lane (default: {DEFAULT_DENSITY_SAMPLE_COUNT})"
    );
    println!(
        "  --exact-seed-cap <n>       Enumerate lanes up to this seed capacity (default: {DEFAULT_DENSITY_EXACT_SEED_CAP})"
    );
    println!(
        "  --max-witnesses <n>        Prime witnesses retained per lane (default: {DEFAULT_DENSITY_MAX_WITNESSES})"
    );
    println!(
        "  --wheel-period-cap <n>     Residue wheel period cap (default: {DEFAULT_DENSITY_WHEEL_PERIOD_CAP})"
    );
    println!(
        "  --sample-seed <n>          Deterministic sample seed (default: {DEFAULT_DENSITY_SAMPLE_SEED})"
    );
}

fn build_observations(
    density_rows: &[DensityAtlasRow],
    control_rows: &[ControlAtlasRow],
) -> Vec<String> {
    let mut observations = Vec::new();
    if let Some(best) = density_rows
        .iter()
        .filter(|row| row.deterministic_u64_scope)
        .max_by(|left, right| left.raw_prime_rate.total_cmp(&right.raw_prime_rate))
    {
        observations.push(format!(
            "Highest measured membrane rate: `{}` at {:.2}%.",
            best.role,
            best.raw_prime_rate * 100.0
        ));
    }
    if let Some(weakest) = density_rows
        .iter()
        .filter(|row| row.deterministic_u64_scope)
        .min_by(|left, right| left.raw_prime_rate.total_cmp(&right.raw_prime_rate))
    {
        observations.push(format!(
            "Weakest measured membrane rate: `{}` at {:.2}%.",
            weakest.role,
            weakest.raw_prime_rate * 100.0
        ));
    }
    let scaffold_available = control_rows
        .iter()
        .filter(|row| row.control_kind == "same_budget_scaffold")
        .filter(|row| row.available)
        .count();
    observations.push(format!(
        "Same-budget scaffold controls are available for {} nonzero-padding lanes.",
        scaffold_available
    ));
    observations.push(
        "The atlas separates raw density from residue survival, coprime controls, and same-slot/scaffold controls before talking about residual signal."
            .to_string(),
    );
    observations
}

fn render_report(bundle: &ReportBundle, observations: &[String]) -> String {
    let mut lines = Vec::new();
    lines.push("# Construction Density Atlas".to_string());
    lines.push(String::new());
    lines.push("This report maps density drift across controlled affine membrane prime families. A great construction here means a high-yield affine membrane surface under specified controls, not a new density theorem.".to_string());
    lines.push(String::new());
    lines.push("```text".to_string());
    lines.push("template family -> affine lane N(s)=A+G*s -> raw candidates -> residue-admissible seeds -> deterministic primality -> prime witnesses -> matched/control residual".to_string());
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push("## Run Settings".to_string());
    lines.push(format!(
        "- sample count for large lanes: `{}`",
        bundle.settings.sample_count
    ));
    lines.push(format!(
        "- exact seed cap: `{}`",
        bundle.settings.exact_seed_cap
    ));
    lines.push(format!(
        "- residue wheel period cap: `{}`",
        bundle.settings.wheel_period_cap
    ));
    lines.push(format!("- sample seed: `{}`", bundle.settings.sample_seed));
    lines.push("- primality scope: deterministic `u64` via `primal::is_prime`".to_string());
    lines.push(String::new());
    lines.push("## Summary".to_string());
    lines.push(format!(
        "- lanes: `{}` total, `{}` deterministic-u64 in scope",
        bundle.summary.lane_count, bundle.summary.in_scope_lane_count
    ));
    lines.push(format!(
        "- exact lanes: `{}`; sampled lanes: `{}`",
        bundle.summary.exact_lane_count, bundle.summary.sampled_lane_count
    ));
    lines.push(format!(
        "- best lane: `{}` at `{:.2}%` raw prime rate",
        bundle.summary.best_lane,
        bundle.summary.best_raw_prime_rate * 100.0
    ));
    lines.push(format!(
        "- weakest lane: `{}` at `{:.2}%` raw prime rate",
        bundle.summary.weakest_lane,
        bundle.summary.weakest_raw_prime_rate * 100.0
    ));
    lines.push(format!(
        "- average membrane rate: `{:.2}%`; average coprime-control rate: `{:.2}%`",
        bundle.summary.average_membrane_prime_rate * 100.0,
        bundle.summary.average_coprime_control_rate * 100.0
    ));
    lines.push(String::new());
    lines.push("## Observations".to_string());
    for observation in observations {
        lines.push(format!("- {observation}"));
    }
    lines.push(String::new());
    lines.push("## Density Rows".to_string());
    lines
        .push("| role | mode | base | k | M | raw rate | admissible | primes | note |".to_string());
    lines.push("|---|---:|---:|---|---:|---:|---:|---:|---|".to_string());
    for row in &bundle.density_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{:.2}%` | `{:.2}%` | `{}` | {} |",
            row.role,
            row.measurement_mode,
            row.base,
            row.k_label,
            row.middle_length,
            row.raw_prime_rate * 100.0,
            row.residue_admissible_share * 100.0,
            row.prime_count,
            row.note
        ));
    }
    lines.push(String::new());
    lines.push("## Prime Witnesses".to_string());
    lines.push("| role | seed | template digits | decimal value |".to_string());
    lines.push("|---|---:|---|---:|".to_string());
    for row in bundle.witness_rows.iter().take(24) {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | `{}` |",
            row.role, row.seed, row.template_digits, row.decimal_value
        ));
    }
    lines.push(String::new());
    lines.push("## Visuals".to_string());
    for image in &bundle.image_artifact_rows {
        lines.push(format!("- `{}`: `{}`", image.label, image.path));
    }
    lines.push(String::new());
    lines.push("## Claim Boundary".to_string());
    lines.push("These visuals and rows show measured density drift across specified candidate surfaces. They do not claim an asymptotic prime-density theorem or a membrane-specific residual unless that residual survives the controls in the exported rows.".to_string());
    lines.push(String::new());
    lines.join("\n")
}

fn render_density_heatmap(rows: &[DensityAtlasRow], path: &Path) {
    let width = 1500;
    let height = 900;
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE).expect("fill heatmap");
    root.draw(&Text::new(
        "Construction Density Atlas: Rate, Residue Survival, Expected Baselines",
        (36, 36),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");

    let metrics: [DensityMetric; 5] = [
        ("raw prime", |row: &DensityAtlasRow| row.raw_prime_rate),
        ("residue survivor", |row: &DensityAtlasRow| {
            row.residue_admissible_share
        }),
        ("prime/admissible", |row: &DensityAtlasRow| {
            row.prime_rate_among_admissible
        }),
        ("PNT expected", |row: &DensityAtlasRow| {
            row.pnt_expected_density
        }),
        ("coprime expected", |row: &DensityAtlasRow| {
            row.coprime_adjusted_expected_density
        }),
    ];
    let left = 330;
    let top = 95;
    let row_h = 54;
    let cell_w = 198;

    for (col, (label, _)) in metrics.iter().enumerate() {
        root.draw(&Text::new(
            *label,
            (left + col as i32 * cell_w + 12, top - 20),
            ("sans-serif", 16).into_font().style(FontStyle::Bold),
        ))
        .expect("draw metric label");
    }

    for (idx, row) in rows.iter().enumerate() {
        let y = top + idx as i32 * row_h;
        root.draw(&Text::new(
            truncate(&row.role, 31),
            (30, y + 28),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw row label");
        for (col, (_, metric)) in metrics.iter().enumerate() {
            let value = metric(row);
            let x = left + col as i32 * cell_w;
            let color = heat_color(value, if col == 1 { 1.0 } else { 0.40 });
            root.draw(&Rectangle::new(
                [(x, y), (x + cell_w - 8, y + row_h - 8)],
                color.filled(),
            ))
            .expect("draw heat cell");
            root.draw(&Text::new(
                format!("{:.2}%", value * 100.0),
                (x + 42, y + 31),
                ("sans-serif", 18).into_font().style(FontStyle::Bold),
            ))
            .expect("draw heat value");
        }
    }
    root.present().expect("present heatmap");
}

fn render_zero_run_drift(rows: &[DensityAtlasRow], path: &Path) {
    let root = BitMapBackend::new(path, (1250, 760)).into_drawing_area();
    root.fill(&WHITE).expect("fill drift");
    let max_zero = rows
        .iter()
        .map(|row| row.total_zero_budget)
        .max()
        .unwrap_or(1)
        .max(1) as f64;
    let max_rate = rows
        .iter()
        .map(|row| row.raw_prime_rate)
        .fold(0.0_f64, f64::max)
        .max(0.01);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Zero-Run Drift: Padding Budget vs Measured Prime Rate",
            ("sans-serif", 30).into_font(),
        )
        .margin(30)
        .x_label_area_size(55)
        .y_label_area_size(70)
        .build_cartesian_2d(-0.5_f64..(max_zero + 1.0), 0.0_f64..(max_rate * 1.25))
        .expect("build drift chart");

    chart
        .configure_mesh()
        .x_desc("total zero budget")
        .y_desc("raw prime rate")
        .y_label_formatter(&|value| format!("{:.1}%", value * 100.0))
        .draw()
        .expect("draw drift mesh");

    for row in rows.iter().filter(|row| row.deterministic_u64_scope) {
        let color = if row.category == "maintained" {
            BLUE.mix(0.78)
        } else if row.expected_quality == "poor" {
            RED.mix(0.78)
        } else {
            RGBColor(240, 150, 35).mix(0.78)
        };
        chart
            .draw_series(std::iter::once(Circle::new(
                (row.total_zero_budget as f64, row.raw_prime_rate),
                8 + (row.zero_lopsidedness as i32 * 2),
                color.filled(),
            )))
            .expect("draw drift point");
        chart
            .draw_series(std::iter::once(Text::new(
                truncate(&row.role, 18),
                (
                    row.total_zero_budget as f64 + 0.08,
                    row.raw_prime_rate + max_rate * 0.025,
                ),
                ("sans-serif", 13).into_font(),
            )))
            .expect("draw drift label");
    }

    root.present().expect("present drift");
}

fn render_layered_control_funnel(
    density_rows: &[DensityAtlasRow],
    control_rows: &[ControlAtlasRow],
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1300, 780)).into_drawing_area();
    root.fill(&WHITE).expect("fill funnel");
    root.draw(&Text::new(
        "Layered Controls: What Survives After Each Comparison?",
        (34, 38),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");

    let stages = [
        (
            "membrane lane",
            mean(density_rows.iter().map(|row| row.raw_prime_rate)),
            RGBColor(47, 107, 180),
        ),
        (
            "residue survivor share",
            mean(density_rows.iter().map(|row| row.residue_admissible_share)),
            RGBColor(78, 159, 120),
        ),
        (
            "raw random",
            control_mean(control_rows, "raw_random_same_digits"),
            RGBColor(180, 180, 185),
        ),
        (
            "coprime random",
            control_mean(control_rows, "coprime_random_same_digits"),
            RGBColor(125, 125, 135),
        ),
        (
            "same-slot random",
            control_mean(control_rows, "same_slot_random"),
            RGBColor(222, 156, 56),
        ),
        (
            "same-budget scaffold",
            control_mean(control_rows, "same_budget_scaffold"),
            RGBColor(170, 96, 167),
        ),
    ];
    let max_value = stages
        .iter()
        .map(|(_, value, _)| *value)
        .fold(0.0_f64, f64::max)
        .max(0.01);
    let left = 310;
    let top = 110;
    let bar_h = 64;
    let max_w = 820;

    for (idx, (label, value, color)) in stages.iter().enumerate() {
        let y = top + idx as i32 * 88;
        root.draw(&Text::new(
            *label,
            (36, y + 39),
            ("sans-serif", 22).into_font().style(FontStyle::Bold),
        ))
        .expect("draw funnel label");
        let width = ((*value / max_value) * max_w as f64).round() as i32;
        root.draw(&Rectangle::new(
            [(left, y), (left + width, y + bar_h)],
            color.filled(),
        ))
        .expect("draw funnel bar");
        root.draw(&Text::new(
            format!("{:.2}%", value * 100.0),
            (left + width + 18, y + 40),
            ("sans-serif", 22).into_font(),
        ))
        .expect("draw funnel value");
    }

    root.draw(&Text::new(
        "Residue survivor share is not a prime rate; it is the first exact gate before primality.",
        (36, 705),
        ("sans-serif", 18).into_font(),
    ))
    .expect("draw note");
    root.present().expect("present funnel");
}

fn render_construction_gallery(
    density_rows: &[DensityAtlasRow],
    witness_rows: &[WitnessAtlasRow],
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1500, 900)).into_drawing_area();
    root.fill(&WHITE).expect("fill gallery");
    root.draw(&Text::new(
        "Good, Mediocre, and Lousy Construction Surfaces",
        (36, 38),
        ("sans-serif", 30).into_font().style(FontStyle::Bold),
    ))
    .expect("draw title");

    let mut ranked = density_rows
        .iter()
        .filter(|row| row.deterministic_u64_scope)
        .collect::<Vec<_>>();
    ranked.sort_by(|left, right| right.raw_prime_rate.total_cmp(&left.raw_prime_rate));
    let mut gallery_rows = ranked.iter().take(4).copied().collect::<Vec<_>>();
    gallery_rows.extend(ranked.iter().rev().take(4).copied());

    let card_w = 690;
    let card_h = 175;
    for (idx, row) in gallery_rows.into_iter().enumerate() {
        let x = if idx % 2 == 0 { 40 } else { 770 };
        let y = 95 + (idx / 2) as i32 * 195;
        let color = if row.expected_quality == "poor" {
            RGBColor(255, 232, 232)
        } else if row.category == "maintained" {
            RGBColor(229, 241, 255)
        } else {
            RGBColor(255, 246, 229)
        };
        root.draw(&Rectangle::new(
            [(x, y), (x + card_w, y + card_h)],
            ShapeStyle::from(&color).filled(),
        ))
        .expect("draw card");
        root.draw(&Rectangle::new(
            [(x, y), (x + card_w, y + card_h)],
            ShapeStyle::from(&RGBColor(80, 80, 88)).stroke_width(1),
        ))
        .expect("draw card border");
        root.draw(&Text::new(
            truncate(&row.role, 42),
            (x + 18, y + 30),
            ("sans-serif", 21).into_font().style(FontStyle::Bold),
        ))
        .expect("draw role");
        root.draw(&Text::new(
            format!(
                "base {} {} {} M={} | zeros={} | rate {:.2}% | admissible {:.2}%",
                row.base,
                row.pair_label,
                row.k_label,
                row.middle_length,
                row.total_zero_budget,
                row.raw_prime_rate * 100.0,
                row.residue_admissible_share * 100.0
            ),
            (x + 18, y + 67),
            ("sans-serif", 17).into_font(),
        ))
        .expect("draw metrics");
        let witness = witness_rows
            .iter()
            .find(|witness| witness.role == row.role)
            .map(|witness| {
                format!(
                    "{} = {}",
                    truncate(&witness.template_digits, 38),
                    witness.decimal_value
                )
            })
            .unwrap_or_else(|| "no witness in retained sample".to_string());
        root.draw(&Text::new(
            witness,
            (x + 18, y + 106),
            ("sans-serif", 16).into_font(),
        ))
        .expect("draw witness");
        root.draw(&Text::new(
            truncate(&row.note, 72),
            (x + 18, y + 143),
            ("sans-serif", 15).into_font(),
        ))
        .expect("draw note");
    }

    root.present().expect("present gallery");
}

fn heat_color(value: f64, max_value: f64) -> RGBColor {
    let t = (value / max_value).clamp(0.0, 1.0);
    let red = (248.0 - 110.0 * t) as u8;
    let green = (242.0 - 35.0 * (1.0 - t)) as u8;
    let blue = (230.0 - 170.0 * t) as u8;
    RGBColor(red, green, blue)
}

fn mean(values: impl Iterator<Item = f64>) -> f64 {
    let mut count = 0usize;
    let mut total = 0.0;
    for value in values {
        if value.is_finite() {
            count += 1;
            total += value;
        }
    }
    if count == 0 {
        0.0
    } else {
        total / count as f64
    }
}

fn control_mean(rows: &[ControlAtlasRow], kind: &str) -> f64 {
    mean(rows.iter().filter_map(|row| {
        if row.control_kind == kind {
            row.prime_rate
        } else {
            None
        }
    }))
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
