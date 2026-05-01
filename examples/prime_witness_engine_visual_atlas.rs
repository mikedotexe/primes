//! Visual atlas for the structured prime-witness engine.
//!
//! This is a collaborator-facing visual report. It renders several candidate
//! explanations of one affine membrane prime family: a fixed symmetric
//! zero-run template lane where the middle seed varies. Individual primes found
//! in the family are prime witnesses. The panels compare which visual metaphors
//! actually help: construction strip, affine line, residue gate matrix,
//! throughput funnel, and candidate-transfer collapse.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example prime_witness_engine_visual_atlas -- --out-dir /tmp/primes_prime_witness_engine_visual_atlas
//! ```

use plotters::coord::Shift;
use plotters::prelude::*;
use primes::validation::{
    affine_singular_series::{
        build_affine_singular_series_report, AffineSingularLaneRow, AffineSingularSeriesReport,
    },
    bounded_k::{digit_symbol, unit_residues},
    fast_affine::{build_fast_affine_lane, FastAffineLane, FastLaneConfig},
    metal_affine::{
        build_metal_affine_residue_rows, default_metal_affine_moduli,
        residue_rows_allow_local_seed, MetalAffineResidueRow,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use serde::Serialize;
use std::{
    env,
    f64::consts::PI,
    path::{Path, PathBuf},
};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_prime_witness_engine_visual_atlas";
const ARTIFACT_ID: &str = "prime_witness_engine_visual_atlas";
const REPORT_EXPORT_VERSION: u32 = 1;
const DEFAULT_SEED_COUNT: u64 = 20_000;
const MATRIX_SEEDS: u64 = 140;

const PAPER: RGBColor = RGBColor(249, 248, 244);
const INK: RGBColor = RGBColor(35, 42, 50);
const MUTED: RGBColor = RGBColor(103, 112, 121);
const BLUE: RGBColor = RGBColor(42, 105, 170);
const TEAL: RGBColor = RGBColor(20, 137, 125);
const GREEN: RGBColor = RGBColor(63, 145, 89);
const GOLD: RGBColor = RGBColor(202, 145, 42);
const RED: RGBColor = RGBColor(192, 76, 62);
const PALE: RGBColor = RGBColor(232, 235, 234);

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    seed_count: u64,
}

#[derive(Debug, Clone)]
struct SeedStatus {
    seed: u64,
    value: u64,
    survivor: bool,
    prime: bool,
}

#[derive(Debug)]
struct VisualData {
    lane: FastAffineLane,
    residue_rows: Vec<MetalAffineResidueRow>,
    moduli: Vec<u32>,
    statuses: Vec<SeedStatus>,
    singular_report: AffineSingularSeriesReport,
}

#[derive(Debug, Clone, Serialize)]
struct VisualArtifactRow {
    variation: String,
    path: String,
    role: String,
    what_to_notice: String,
}

#[derive(Debug, Clone, Serialize)]
struct WitnessRow {
    seed: u64,
    middle_digits: String,
    template_digits: String,
    value: u64,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    base: u32,
    pair_label: String,
    k_label: String,
    middle_length: usize,
    visual_seed_count: u64,
    survivor_count: u64,
    prime_count: u64,
    survivor_share: f64,
    prime_share_of_raw: f64,
    prime_share_of_survivors: f64,
    residue_moduli: Vec<u32>,
    transfer_metadata_bytes: u64,
    transfer_bitmask_bytes: u64,
    avoided_candidate_value_bytes_u64: u64,
    first_prime: u64,
    first_prime_template: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    summary: ReportSummary,
    visual_artifact_rows: Vec<VisualArtifactRow>,
    witness_rows: Vec<WitnessRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output dir");

    let data = build_visual_data(options.seed_count);
    let summary = build_summary(&data);
    let witness_rows = build_witness_rows(&data, 12);

    let visuals = render_visuals(&options.out_dir, &data, &summary);
    let report = render_report(&summary, &visuals);
    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        summary,
        visual_artifact_rows: visuals.clone(),
        witness_rows: witness_rows.clone(),
    };

    write_text_file(options.out_dir.join("report.md"), &report).expect("write report");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary");
    write_csv_rows(options.out_dir.join("visual_artifact_rows.csv"), &visuals)
        .expect("write visual rows");
    write_csv_rows(options.out_dir.join("witness_rows.csv"), &witness_rows)
        .expect("write witness rows");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--release".to_string(),
                "--example".to_string(),
                "prime_witness_engine_visual_atlas".to_string(),
            ],
            upstream_inputs: vec![
                "src/validation/fast_affine.rs".to_string(),
                "src/validation/metal_affine.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "visual_artifact_rows.csv".to_string(),
                "witness_rows.csv".to_string(),
                "construction_strip.png".to_string(),
                "affine_line_witnesses.png".to_string(),
                "residue_gate_matrix.png".to_string(),
                "throughput_funnel.png".to_string(),
                "transfer_collapse.png".to_string(),
                "geodesic_residue_path.png".to_string(),
                "residue_weather_map.png".to_string(),
                "singular_profile_dashboard.png".to_string(),
                "unit_cycle_chord_map.png".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )
    .expect("write manifest");

    println!(
        "wrote prime witness engine visual atlas to {}",
        options.out_dir.display()
    );
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut seed_count = DEFAULT_SEED_COUNT;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = PathBuf::from(args.next().expect("missing --out-dir value")),
            "--seed-count" => {
                seed_count = args
                    .next()
                    .expect("missing --seed-count value")
                    .parse()
                    .expect("invalid --seed-count")
            }
            _ => panic!("unrecognized argument: {arg}"),
        }
    }
    Options {
        out_dir,
        seed_count,
    }
}

fn build_visual_data(seed_count: u64) -> VisualData {
    let lane = build_fast_affine_lane(FastLaneConfig::new(10, 3, 7, 9, (2, 1)))
        .expect("visual lane should fit in u64");
    let moduli = default_metal_affine_moduli(&lane);
    let residue_rows = build_metal_affine_residue_rows(&lane, 0, &moduli).expect("residue rows");
    let scanned = seed_count.min(lane.seed_capacity);
    let statuses = (0..scanned)
        .map(|seed| {
            let value = lane.candidate_value(seed).expect("candidate value");
            let survivor = residue_rows_allow_local_seed(&residue_rows, seed);
            let prime = survivor && primal::is_prime(value);
            SeedStatus {
                seed,
                value,
                survivor,
                prime,
            }
        })
        .collect();
    let singular_report = build_affine_singular_series_report(Default::default());
    VisualData {
        lane,
        residue_rows,
        moduli,
        statuses,
        singular_report,
    }
}

fn build_summary(data: &VisualData) -> ReportSummary {
    let raw = data.statuses.len() as u64;
    let survivor_count = data.statuses.iter().filter(|row| row.survivor).count() as u64;
    let prime_count = data.statuses.iter().filter(|row| row.prime).count() as u64;
    let first_prime = data
        .statuses
        .iter()
        .find(|row| row.prime)
        .expect("visual seed window should contain a prime");
    let metadata_bytes = data.residue_rows.len() as u64 * 16 + 16;
    let bitmask_bytes = raw.div_ceil(32) * 4;
    ReportSummary {
        base: data.lane.config.base,
        pair_label: data.lane.config.pair_label(),
        k_label: data.lane.config.k_label(),
        middle_length: data.lane.config.middle_length,
        visual_seed_count: raw,
        survivor_count,
        prime_count,
        survivor_share: ratio(survivor_count, raw),
        prime_share_of_raw: ratio(prime_count, raw),
        prime_share_of_survivors: ratio(prime_count, survivor_count),
        residue_moduli: data.moduli.clone(),
        transfer_metadata_bytes: metadata_bytes,
        transfer_bitmask_bytes: bitmask_bytes,
        avoided_candidate_value_bytes_u64: raw * 8,
        first_prime: first_prime.value,
        first_prime_template: data.lane.template_digits(first_prime.seed),
    }
}

fn build_witness_rows(data: &VisualData, limit: usize) -> Vec<WitnessRow> {
    data.statuses
        .iter()
        .filter(|row| row.prime)
        .take(limit)
        .map(|row| WitnessRow {
            seed: row.seed,
            middle_digits: data.lane.middle_digits(row.seed),
            template_digits: data.lane.template_digits(row.seed),
            value: row.value,
        })
        .collect()
}

fn render_visuals(
    out_dir: &Path,
    data: &VisualData,
    summary: &ReportSummary,
) -> Vec<VisualArtifactRow> {
    let construction_path = out_dir.join("construction_strip.png");
    render_construction_strip(data, summary, &construction_path);
    let affine_path = out_dir.join("affine_line_witnesses.png");
    render_affine_line(data, &affine_path);
    let matrix_path = out_dir.join("residue_gate_matrix.png");
    render_residue_gate_matrix(data, &matrix_path);
    let funnel_path = out_dir.join("throughput_funnel.png");
    render_throughput_funnel(summary, &funnel_path);
    let transfer_path = out_dir.join("transfer_collapse.png");
    render_transfer_collapse(summary, &transfer_path);
    let geodesic_path = out_dir.join("geodesic_residue_path.png");
    render_geodesic_residue_path(data, &geodesic_path);
    let weather_path = out_dir.join("residue_weather_map.png");
    render_residue_weather_map(data, &weather_path);
    let singular_path = out_dir.join("singular_profile_dashboard.png");
    render_singular_profile_dashboard(&data.singular_report.lane_rows, &singular_path);
    let chord_path = out_dir.join("unit_cycle_chord_map.png");
    render_unit_cycle_chord_map(&data.singular_report.lane_rows, &chord_path);

    vec![
        VisualArtifactRow {
            variation: "A".to_string(),
            path: construction_path.display().to_string(),
            role: "construction strip".to_string(),
            what_to_notice:
                "The symmetric zero-padded grammar stays fixed while the middle seed varies."
                    .to_string(),
        },
        VisualArtifactRow {
            variation: "B".to_string(),
            path: affine_path.display().to_string(),
            role: "affine line with witnesses".to_string(),
            what_to_notice:
                "The seed walk is an arithmetic line; prime witnesses are points on that line."
                    .to_string(),
        },
        VisualArtifactRow {
            variation: "C".to_string(),
            path: matrix_path.display().to_string(),
            role: "residue gate matrix".to_string(),
            what_to_notice:
                "Each small modulus blocks exact seed classes before primality testing.".to_string(),
        },
        VisualArtifactRow {
            variation: "D".to_string(),
            path: funnel_path.display().to_string(),
            role: "throughput funnel".to_string(),
            what_to_notice:
                "Raw seeds collapse to residue survivors, then to confirmed prime witnesses."
                    .to_string(),
        },
        VisualArtifactRow {
            variation: "E".to_string(),
            path: transfer_path.display().to_string(),
            role: "candidate-transfer collapse".to_string(),
            what_to_notice:
                "The GPU path receives lane metadata and returns a bitmask, not candidate values."
                    .to_string(),
        },
        VisualArtifactRow {
            variation: "F".to_string(),
            path: geodesic_path.display().to_string(),
            role: "geodesic residue path".to_string(),
            what_to_notice:
                "The affine seed walk becomes a winding path through residue phase space."
                    .to_string(),
        },
        VisualArtifactRow {
            variation: "G".to_string(),
            path: weather_path.display().to_string(),
            role: "residue weather map".to_string(),
            what_to_notice:
                "Composite weather is local and layered: some seeds hit several exact gates, while clearings become survivor candidates."
                    .to_string(),
        },
        VisualArtifactRow {
            variation: "H".to_string(),
            path: singular_path.display().to_string(),
            role: "singular-profile dashboard".to_string(),
            what_to_notice:
                "Observed yield is compared with PNT and finite residue-weather expectation before we call anything signal."
                    .to_string(),
        },
        VisualArtifactRow {
            variation: "I".to_string(),
            path: chord_path.display().to_string(),
            role: "unit-cycle chord map".to_string(),
            what_to_notice:
                "Digit-pair geometry becomes a base-normalized chord system for phase residual leads."
                    .to_string(),
        },
    ]
}

fn render_construction_strip(data: &VisualData, summary: &ReportSummary, path: &Path) {
    let root = BitMapBackend::new(path, (1500, 620)).into_drawing_area();
    fill_root(&root);
    title(
        &root,
        "Variation A: visible construction grammar",
        "A fixed symmetric frame becomes a family when the middle seed changes.",
    );

    let first_seed = data
        .statuses
        .iter()
        .find(|row| row.prime)
        .expect("prime witness")
        .seed;
    let middle = data.lane.middle_digits(first_seed);
    let segments = [
        ("outer", "3", BLUE),
        ("k outer", "00", PALE),
        ("inner", "7", TEAL),
        ("k inner", "0", PALE),
        ("seed", middle.as_str(), GOLD),
        ("k inner", "0", PALE),
        ("inner", "7", TEAL),
        ("k outer", "00", PALE),
        ("outer", "3", BLUE),
    ];

    let mut x = 80;
    let y = 210;
    for (label, value, color) in segments {
        let width = 68 + value.len() as i32 * 30;
        root.draw(&Rectangle::new(
            [(x, y), (x + width, y + 120)],
            color.mix(0.23).filled(),
        ))
        .unwrap();
        root.draw(&Rectangle::new(
            [(x, y), (x + width, y + 120)],
            color.stroke_width(3),
        ))
        .unwrap();
        root.draw(&Text::new(
            value.to_string(),
            (x + 22, y + 70),
            ("monospace", 36).into_font().color(&INK),
        ))
        .unwrap();
        root.draw(&Text::new(
            label.to_string(),
            (x + 8, y + 150),
            ("sans-serif", 18).into_font().color(&MUTED),
        ))
        .unwrap();
        x += width + 8;
    }

    let formula = format!("N(s) = {} + {} * s", data.lane.shift, data.lane.gradient);
    root.draw(&Text::new(
        formula,
        (90, 455),
        ("monospace", 28).into_font().color(&INK),
    ))
    .unwrap();
    root.draw(&Text::new(
        format!(
            "First witness in this window: {} = {}",
            summary.first_prime_template, summary.first_prime
        ),
        (90, 505),
        ("sans-serif", 25).into_font().color(&GREEN),
    ))
    .unwrap();
}

fn render_affine_line(data: &VisualData, path: &Path) {
    let root = BitMapBackend::new(path, (1500, 760)).into_drawing_area();
    fill_root(&root);
    title(
        &root,
        "Variation B: affine line with prime witnesses",
        "The engine walks seed-space; every candidate lies on N(s)=A+G*s.",
    );

    let chart_area = root.margin(80, 60, 80, 70);
    let max_seed = data.statuses.len().min(800) as u64;
    let y0 = 0.0;
    let y1 = (max_seed.saturating_sub(1) * data.lane.gradient) as f64;
    let mut chart = ChartBuilder::on(&chart_area)
        .margin(10)
        .x_label_area_size(44)
        .y_label_area_size(70)
        .build_cartesian_2d(0u64..max_seed, y0..y1)
        .unwrap();
    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("seed s")
        .y_desc("candidate offset N(s)-A")
        .y_label_formatter(&|value| format!("{:.0}M", value / 1_000_000.0))
        .label_style(("sans-serif", 18).into_font().color(&MUTED))
        .axis_style(MUTED)
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..max_seed).map(|seed| (seed, (seed * data.lane.gradient) as f64)),
            BLUE.stroke_width(4),
        ))
        .unwrap();

    chart
        .draw_series(
            data.statuses
                .iter()
                .take(max_seed as usize)
                .filter(|row| row.survivor && !row.prime)
                .map(|row| {
                    Circle::new(
                        (row.seed, (row.seed * data.lane.gradient) as f64),
                        2,
                        TEAL.mix(0.45).filled(),
                    )
                }),
        )
        .unwrap();
    chart
        .draw_series(
            data.statuses
                .iter()
                .take(max_seed as usize)
                .filter(|row| row.prime)
                .map(|row| {
                    Circle::new(
                        (row.seed, (row.seed * data.lane.gradient) as f64),
                        5,
                        GOLD.filled(),
                    )
                }),
        )
        .unwrap();
}

fn render_residue_gate_matrix(data: &VisualData, path: &Path) {
    let root = BitMapBackend::new(path, (1500, 760)).into_drawing_area();
    fill_root(&root);
    title(
        &root,
        "Variation C: exact residue gates",
        "Rows are small moduli; columns are seeds. Red means this modulus blocks the seed.",
    );

    let x0 = 140;
    let y0 = 150;
    let cell_w = 7;
    let cell_h = 42;
    let seeds = MATRIX_SEEDS.min(data.statuses.len() as u64);
    for (row_index, modulus) in data.moduli.iter().enumerate() {
        let y = y0 + row_index as i32 * (cell_h + 8);
        root.draw(&Text::new(
            format!("mod {modulus}"),
            (52, y + 28),
            ("sans-serif", 18).into_font().color(&INK),
        ))
        .unwrap();
        let residue_row = data
            .residue_rows
            .iter()
            .find(|row| row.p == *modulus)
            .expect("residue row");
        for seed in 0..seeds {
            let x = x0 + seed as i32 * cell_w;
            let p = residue_row.p as u64;
            let value = (residue_row.a as u64 + ((seed % p) * residue_row.g as u64) % p) % p;
            let status = &data.statuses[seed as usize];
            let color = if value == 0 {
                RED
            } else if status.prime {
                GOLD
            } else if status.survivor {
                GREEN
            } else {
                PALE
            };
            root.draw(&Rectangle::new(
                [(x, y), (x + cell_w - 1, y + cell_h)],
                color.mix(0.86).filled(),
            ))
            .unwrap();
        }
    }

    legend(
        &root,
        1160,
        180,
        &[
            ("blocked by this modulus", RED),
            ("survives all gates", GREEN),
            ("prime witness", GOLD),
            ("not blocked here", PALE),
        ],
    );
}

fn render_throughput_funnel(summary: &ReportSummary, path: &Path) {
    let root = BitMapBackend::new(path, (1500, 760)).into_drawing_area();
    fill_root(&root);
    title(
        &root,
        "Variation D: throughput funnel",
        "The engine removes exact residue obstructions before primality confirmation.",
    );

    let raw = summary.visual_seed_count;
    let survivor = summary.survivor_count;
    let primes = summary.prime_count;
    let bars = [
        ("raw seeds", raw, BLUE),
        ("residue survivors", survivor, TEAL),
        ("prime witnesses", primes, GOLD),
    ];
    let max_width = 1080.0;
    for (idx, (label, value, color)) in bars.iter().enumerate() {
        let y = 190 + idx as i32 * 145;
        let width = ((*value as f64 / raw as f64) * max_width).max(18.0) as i32;
        root.draw(&Rectangle::new(
            [(300, y), (300 + width, y + 82)],
            color.mix(0.78).filled(),
        ))
        .unwrap();
        root.draw(&Text::new(
            label.to_string(),
            (80, y + 52),
            ("sans-serif", 26).into_font().color(&INK),
        ))
        .unwrap();
        let text = format!("{} ({:.2}%)", value, 100.0 * *value as f64 / raw as f64);
        let inside = width > 860;
        let label_x = if inside {
            300 + width - 265
        } else {
            320 + width
        };
        let label_color = if inside { PAPER } else { INK };
        root.draw(&Text::new(
            text,
            (label_x, y + 52),
            ("sans-serif", 25).into_font().color(&label_color),
        ))
        .unwrap();
    }

    root.draw(&Text::new(
        format!(
            "prime share among survivors: {:.2}%",
            summary.prime_share_of_survivors * 100.0
        ),
        (300, 645),
        ("sans-serif", 26).into_font().color(&GREEN),
    ))
    .unwrap();
}

fn render_transfer_collapse(summary: &ReportSummary, path: &Path) {
    let root = BitMapBackend::new(path, (1500, 760)).into_drawing_area();
    fill_root(&root);
    title(
        &root,
        "Variation E: candidate-transfer collapse",
        "The GPU does not receive candidate values; it receives residue metadata.",
    );

    draw_pipeline_box(&root, 100, 210, 390, 110, "candidate array", RED);
    draw_pipeline_box(&root, 560, 210, 260, 110, "GPU sieve", BLUE);
    draw_pipeline_box(&root, 980, 210, 330, 110, "survivor bitmask", TEAL);
    draw_arrow(&root, 495, 265, 545, 265, RED);
    draw_arrow(&root, 830, 265, 970, 265, TEAL);
    root.draw(&Text::new(
        format!(
            "naive transfer: {} bytes",
            summary.avoided_candidate_value_bytes_u64
        ),
        (100, 370),
        ("sans-serif", 24).into_font().color(&RED),
    ))
    .unwrap();

    draw_pipeline_box(&root, 100, 500, 390, 110, "residue metadata", GREEN);
    draw_pipeline_box(&root, 560, 500, 260, 110, "GPU sieve", BLUE);
    draw_pipeline_box(&root, 980, 500, 330, 110, "survivor bitmask", TEAL);
    draw_arrow(&root, 495, 555, 545, 555, GREEN);
    draw_arrow(&root, 830, 555, 970, 555, TEAL);
    root.draw(&Text::new(
        format!(
            "ours: {} bytes in, {} bytes out",
            summary.transfer_metadata_bytes, summary.transfer_bitmask_bytes
        ),
        (100, 660),
        ("sans-serif", 24).into_font().color(&GREEN),
    ))
    .unwrap();
}

fn render_geodesic_residue_path(data: &VisualData, path: &Path) {
    let root = BitMapBackend::new(path, (1500, 760)).into_drawing_area();
    fill_root(&root);
    title(
        &root,
        "Variation F: geodesic residue path",
        "Project the affine seed walk onto two residue cycles; witnesses appear as rare clear points.",
    );

    let mod_x = 7u64;
    let mod_y = 11u64;
    let limit = data.statuses.len().min(420);
    let phase = |row: &SeedStatus| {
        (
            (row.value % mod_x) as f64 / mod_x as f64,
            (row.value % mod_y) as f64 / mod_y as f64,
        )
    };

    let chart_area = root.margin(95, 70, 90, 90);
    let mut chart = ChartBuilder::on(&chart_area)
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..1.0, 0.0..1.0)
        .unwrap();
    chart
        .configure_mesh()
        .x_desc("phase mod 7")
        .y_desc("phase mod 11")
        .label_style(("sans-serif", 18).into_font().color(&MUTED))
        .axis_style(MUTED)
        .light_line_style(PALE)
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            data.statuses.iter().take(limit).map(phase),
            BLUE.mix(0.25).stroke_width(2),
        ))
        .unwrap();
    chart
        .draw_series(data.statuses.iter().take(limit).map(|row| {
            let color = if row.prime {
                GOLD
            } else if row.survivor {
                GREEN
            } else {
                RED
            };
            Circle::new(
                phase(row),
                if row.prime { 6 } else { 3 },
                color.mix(0.72).filled(),
            )
        }))
        .unwrap();

    root.draw(&Text::new(
        "The line is straight in seed-space, but it winds when seen through modular cycles.",
        (105, 690),
        ("sans-serif", 22).into_font().color(&INK),
    ))
    .unwrap();
    legend(
        &root,
        1120,
        160,
        &[
            ("blocked seed", RED),
            ("residue survivor", GREEN),
            ("prime witness", GOLD),
        ],
    );
}

fn render_residue_weather_map(data: &VisualData, path: &Path) {
    let root = BitMapBackend::new(path, (1500, 760)).into_drawing_area();
    fill_root(&root);
    title(
        &root,
        "Variation G: residue weather map",
        "Each seed has local weather: how many exact small-prime gates block it?",
    );

    let limit = data.statuses.len().min(620);
    let block_counts = data
        .statuses
        .iter()
        .take(limit)
        .map(|row| seed_block_count(&data.residue_rows, row.seed))
        .collect::<Vec<_>>();
    let max_blocks = block_counts.iter().copied().max().unwrap_or(1).max(1);
    let chart_area = root.margin(95, 70, 85, 70);
    let mut chart = ChartBuilder::on(&chart_area)
        .margin(10)
        .x_label_area_size(44)
        .y_label_area_size(70)
        .build_cartesian_2d(0u64..limit as u64, 0u32..max_blocks + 1)
        .unwrap();
    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("seed s")
        .y_desc("blocking gates")
        .label_style(("sans-serif", 18).into_font().color(&MUTED))
        .axis_style(MUTED)
        .draw()
        .unwrap();

    chart
        .draw_series(
            data.statuses
                .iter()
                .take(limit)
                .zip(block_counts.iter())
                .map(|(row, &blocks)| {
                    let color = if row.prime {
                        GOLD
                    } else if blocks == 0 {
                        GREEN
                    } else {
                        RED
                    };
                    let height = blocks.max(1);
                    Rectangle::new(
                        [(row.seed, 0), (row.seed + 1, height)],
                        color.mix(if blocks == 0 { 0.55 } else { 0.72 }).filled(),
                    )
                }),
        )
        .unwrap();

    root.draw(&Text::new(
        "Zero blocks means a seed survives the finite gate profile; gold bars are confirmed prime witnesses.",
        (105, 690),
        ("sans-serif", 22).into_font().color(&INK),
    ))
    .unwrap();
}

fn render_singular_profile_dashboard(rows: &[AffineSingularLaneRow], path: &Path) {
    let root = BitMapBackend::new(path, (1500, 860)).into_drawing_area();
    fill_root(&root);
    title(
        &root,
        "Variation H: finite singular-profile dashboard",
        "Observed yield, PNT size expectation, and residue-weather expectation share one lane card.",
    );

    let mut ranked = rows.iter().collect::<Vec<_>>();
    ranked.sort_by(|left, right| {
        right
            .abs_residual_vs_residue_expected_pp
            .total_cmp(&left.abs_residual_vs_residue_expected_pp)
    });
    ranked.truncate(9);
    let max_rate = ranked
        .iter()
        .flat_map(|row| {
            [
                row.observed_prime_rate,
                row.pnt_expected_density,
                row.residue_adjusted_expected_density,
            ]
        })
        .fold(0.01_f64, f64::max);
    for (index, row) in ranked.iter().enumerate() {
        let y = 145 + index as i32 * 72;
        root.draw(&Text::new(
            format!(
                "{}  base {} {} {}",
                row.role, row.base, row.pair_label, row.k_label
            ),
            (60, y + 20),
            ("sans-serif", 17).into_font().color(&INK),
        ))
        .unwrap();
        draw_rate_bar(
            &root,
            500,
            y,
            row.pnt_expected_density,
            max_rate,
            PALE,
            "PNT",
        );
        draw_rate_bar(
            &root,
            760,
            y,
            row.residue_adjusted_expected_density,
            max_rate,
            TEAL,
            "residue",
        );
        draw_rate_bar(
            &root,
            1020,
            y,
            row.observed_prime_rate,
            max_rate,
            GOLD,
            "observed",
        );
        let residual_color = if row.residual_vs_residue_expected_pp >= 0.0 {
            GREEN
        } else {
            RED
        };
        root.draw(&Text::new(
            format!("{:+.2} pp", row.residual_vs_residue_expected_pp),
            (1290, y + 22),
            ("sans-serif", 20).into_font().color(&residual_color),
        ))
        .unwrap();
    }
}

fn render_unit_cycle_chord_map(rows: &[AffineSingularLaneRow], path: &Path) {
    let root = BitMapBackend::new(path, (1500, 860)).into_drawing_area();
    fill_root(&root);
    title(
        &root,
        "Variation I: unit-cycle chord map",
        "Base-30 role swaps as chords on the unit-residue cycle, colored by finite residual.",
    );

    let units = unit_residues(30);
    let center = (500, 440);
    let radius = 275;
    root.draw(&Circle::new(center, radius, MUTED.stroke_width(2)))
        .unwrap();
    for (index, digit) in units.iter().enumerate() {
        let (x, y) = unit_cycle_point(index, units.len(), center, radius);
        root.draw(&Circle::new((x, y), 8, BLUE.filled())).unwrap();
        root.draw(&Text::new(
            digit_symbol(*digit),
            (x - 10, y - 18),
            ("sans-serif", 18).into_font().color(&INK),
        ))
        .unwrap();
    }

    let focus_rows = rows
        .iter()
        .filter(|row| row.base == 30 && row.outer == 1 && row.k_label == "k=(0,0)")
        .take(6)
        .collect::<Vec<_>>();
    for (index, row) in focus_rows.iter().enumerate() {
        let Some(outer_index) = units.iter().position(|digit| *digit == row.outer) else {
            continue;
        };
        let Some(inner_index) = units.iter().position(|digit| *digit == row.inner) else {
            continue;
        };
        let from = unit_cycle_point(outer_index, units.len(), center, radius);
        let to = unit_cycle_point(inner_index, units.len(), center, radius);
        let color = if row.residual_vs_residue_expected_pp >= 0.0 {
            GREEN
        } else {
            RED
        };
        root.draw(&PathElement::new(
            vec![from, to],
            color.mix(0.68).stroke_width(5),
        ))
        .unwrap();
        root.draw(&Text::new(
            format!(
                "{} -> {}  {:+.2} pp",
                digit_symbol(row.outer),
                digit_symbol(row.inner),
                row.residual_vs_residue_expected_pp
            ),
            (900, 190 + index as i32 * 58),
            ("sans-serif", 24).into_font().color(&color),
        ))
        .unwrap();
    }
    root.draw(&Text::new(
        "Same circle radius; bases differ by bead count and unit positions. Chords are a way to see role geometry.",
        (80, 790),
        ("sans-serif", 22).into_font().color(&INK),
    ))
    .unwrap();
}

fn render_report(summary: &ReportSummary, visuals: &[VisualArtifactRow]) -> String {
    let mut lines = Vec::new();
    lines.push("# Prime Witness Engine Visual Atlas".to_string());
    lines.push(String::new());
    lines.push("## Thesis".to_string());
    lines.push("This atlas tries several visual explanations of one affine membrane prime family: a visible symmetric zero-run template becomes an affine seed line, exact residue gates remove impossible seeds, and prime witnesses are confirmed only after the funnel.".to_string());
    lines.push(String::new());
    lines.push("## Family Language".to_string());
    lines.push("Use `affine membrane prime family` for the whole seed-varying lane, `symmetric zero-run template prime` as the accessible gloss, and `prime witness` for an individual prime found inside that family.".to_string());
    lines.push(String::new());
    lines.push("## Teaching Lane".to_string());
    lines.push(format!(
        "- base `{}`, pair `{}`, `{}`, middle length `{}`",
        summary.base, summary.pair_label, summary.k_label, summary.middle_length
    ));
    lines.push(format!(
        "- sample: `{}` seeds -> `{}` residue survivors -> `{}` prime witnesses",
        summary.visual_seed_count, summary.survivor_count, summary.prime_count
    ));
    lines.push(format!(
        "- first witness: `{}` = `{}`",
        summary.first_prime_template, summary.first_prime
    ));
    lines.push(String::new());
    lines.push("## Variations".to_string());
    for visual in visuals {
        lines.push(format!(
            "- Variation {}: `{}` - {}",
            visual.variation, visual.role, visual.what_to_notice
        ));
    }
    lines.push(String::new());
    lines.push("## Current Favorite".to_string());
    lines.push("The geodesic residue path plus the singular-profile dashboard feel like the strongest new pair: one lets the imagination see the affine walk, while the other keeps the claim honest against finite residue-weather accounting.".to_string());
    lines.push(String::new());
    lines.push("## Guardrail".to_string());
    lines.push("These visuals show a structured candidate funnel and real prime witnesses. They do not claim a global prime-density theorem.".to_string());
    lines.join("\n")
}

fn fill_root(root: &DrawingArea<BitMapBackend<'_>, Shift>) {
    root.fill(&PAPER).unwrap();
}

fn title(root: &DrawingArea<BitMapBackend<'_>, Shift>, heading: &str, subheading: &str) {
    root.draw(&Text::new(
        heading.to_string(),
        (60, 62),
        ("sans-serif", 35).into_font().color(&INK),
    ))
    .unwrap();
    root.draw(&Text::new(
        subheading.to_string(),
        (60, 105),
        ("sans-serif", 22).into_font().color(&MUTED),
    ))
    .unwrap();
}

fn legend(
    root: &DrawingArea<BitMapBackend<'_>, Shift>,
    x: i32,
    y: i32,
    items: &[(&str, RGBColor)],
) {
    for (idx, (label, color)) in items.iter().enumerate() {
        let y = y + idx as i32 * 46;
        root.draw(&Rectangle::new(
            [(x, y), (x + 28, y + 28)],
            color.mix(0.86).filled(),
        ))
        .unwrap();
        root.draw(&Text::new(
            label.to_string(),
            (x + 42, y + 22),
            ("sans-serif", 18).into_font().color(&INK),
        ))
        .unwrap();
    }
}

fn draw_pipeline_box(
    root: &DrawingArea<BitMapBackend<'_>, Shift>,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    label: &str,
    color: RGBColor,
) {
    root.draw(&Rectangle::new(
        [(x, y), (x + w, y + h)],
        color.mix(0.18).filled(),
    ))
    .unwrap();
    root.draw(&Rectangle::new(
        [(x, y), (x + w, y + h)],
        color.stroke_width(3),
    ))
    .unwrap();
    root.draw(&Text::new(
        label.to_string(),
        (x + 24, y + 66),
        ("sans-serif", 27).into_font().color(&INK),
    ))
    .unwrap();
}

fn draw_arrow(
    root: &DrawingArea<BitMapBackend<'_>, Shift>,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: RGBColor,
) {
    root.draw(&PathElement::new(
        vec![(x0, y0), (x1, y1)],
        color.stroke_width(4),
    ))
    .unwrap();
    root.draw(&Polygon::new(
        vec![(x1, y1), (x1 - 14, y1 - 8), (x1 - 14, y1 + 8)],
        color.filled(),
    ))
    .unwrap();
}

fn draw_rate_bar(
    root: &DrawingArea<BitMapBackend<'_>, Shift>,
    x: i32,
    y: i32,
    rate: f64,
    max_rate: f64,
    color: RGBColor,
    label: &str,
) {
    let width = ((rate / max_rate) * 190.0).max(5.0) as i32;
    root.draw(&Rectangle::new(
        [(x, y), (x + width, y + 26)],
        color.mix(0.78).filled(),
    ))
    .unwrap();
    root.draw(&Text::new(
        format!("{label} {:.2}%", rate * 100.0),
        (x, y + 50),
        ("sans-serif", 14).into_font().color(&MUTED),
    ))
    .unwrap();
}

fn seed_block_count(rows: &[MetalAffineResidueRow], seed: u64) -> u32 {
    rows.iter()
        .filter(|row| {
            let p = row.p as u64;
            (row.a as u64 + ((seed % p) * row.g as u64) % p).is_multiple_of(p)
        })
        .count() as u32
}

fn unit_cycle_point(index: usize, count: usize, center: (i32, i32), radius: i32) -> (i32, i32) {
    let angle = -PI / 2.0 + 2.0 * PI * index as f64 / count as f64;
    (
        center.0 + (radius as f64 * angle.cos()).round() as i32,
        center.1 + (radius as f64 * angle.sin()).round() as i32,
    )
}

fn ratio(num: u64, den: u64) -> f64 {
    if den == 0 {
        0.0
    } else {
        num as f64 / den as f64
    }
}
