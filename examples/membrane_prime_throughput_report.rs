//! Maintained throughput report for fast affine membrane prime generation.
//!
//! Terminology: a fixed symmetric zero-run lane with a varying middle seed is
//! an affine membrane prime family. Individual primes found in that family are
//! prime witnesses.
//!
//! The report measures deterministic `u64` lanes only. It uses the maintained
//! bounded-`k` grammar and the fast affine engine rather than the older
//! optimization-sketch binaries.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example membrane_prime_throughput_report -- --out-dir /tmp/primes_fast_generation
//! ```

use plotters::prelude::*;
use primes::validation::{
    fast_affine::{scan_fast_prime_lane, FastLaneConfig, FastPrimeRun},
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

const DEFAULT_OUT_DIR: &str = "/tmp/primes_fast_generation";
const ARTIFACT_ID: &str = "membrane_prime_throughput_report";
const REPORT_EXPORT_VERSION: u32 = 1;
const DEFAULT_SEED_COUNT: u64 = 10_000;
const DEFAULT_MAX_PRIMES: usize = 20;
const DEFAULT_WHEEL_PERIOD_CAP: u64 = 1_000_000;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    seed_count: u64,
    max_primes: usize,
    wheel_period_cap: u64,
}

#[derive(Debug, Clone, Copy)]
struct LaneSpec {
    role: &'static str,
    base: u32,
    outer: u32,
    inner: u32,
    middle_length: usize,
    k: (u32, u32),
    note: &'static str,
}

const LANE_SPECS: &[LaneSpec] = &[
    LaneSpec {
        role: "decimal_visible_zero_run_k21",
        base: 10,
        outer: 3,
        inner: 7,
        middle_length: 2,
        k: (2, 1),
        note: "visible decimal teaching lane with mirrored zero runs",
    },
    LaneSpec {
        role: "decimal_visible_zero_run_k11",
        base: 10,
        outer: 3,
        inner: 7,
        middle_length: 2,
        k: (1, 1),
        note: "compact decimal teaching lane",
    },
    LaneSpec {
        role: "decimal_deep_zero_run_k22",
        base: 10,
        outer: 1,
        inner: 7,
        middle_length: 2,
        k: (2, 2),
        note: "deep decimal zero-run teaching lane",
    },
    LaneSpec {
        role: "decimal_m3_nonpal_center",
        base: 10,
        outer: 3,
        inner: 1,
        middle_length: 3,
        k: (2, 2),
        note: "M=3 decimal lane with non-palindromic center witness",
    },
    LaneSpec {
        role: "base_6_base_aware_witness",
        base: 6,
        outer: 1,
        inner: 5,
        middle_length: 1,
        k: (0, 0),
        note: "small base-aware bridge witness",
    },
    LaneSpec {
        role: "base_22_compact_pocket",
        base: 22,
        outer: 17,
        inner: 19,
        middle_length: 2,
        k: (0, 0),
        note: "compact side of the base-22/mod-5 pocket",
    },
    LaneSpec {
        role: "base_22_side_pocket",
        base: 22,
        outer: 17,
        inner: 19,
        middle_length: 2,
        k: (2, 2),
        note: "higher-order base-22 side pocket",
    },
];

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    requested_seed_count: u64,
    max_primes: usize,
    wheel_period_cap: u64,
    deterministic_scope: String,
}

#[derive(Debug, Clone, Serialize)]
struct ThroughputRow {
    role: String,
    base: u32,
    pair_label: String,
    k_label: String,
    middle_length: usize,
    shift: u64,
    gradient: u64,
    seed_capacity: u64,
    requested_seed_count: u64,
    scanned_seed_count: u64,
    capped_to_seed_capacity: bool,
    wheel_period: u64,
    wheel_moduli_label: String,
    wheel_admissible_residue_count: usize,
    admissible_seed_count: u64,
    admissible_share: f64,
    primality_tests: u64,
    primes_found: u64,
    prime_share_of_raw: f64,
    elapsed_seconds: f64,
    seeds_per_second: f64,
    primality_tests_per_second: f64,
    primes_per_second: f64,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct PrimeWitnessRow {
    role: String,
    base: u32,
    pair_label: String,
    k_label: String,
    middle_length: usize,
    seed: u64,
    middle_digits: String,
    template_digits: String,
    value: u64,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    lane_count: usize,
    total_scanned_seeds: u64,
    total_admissible_seeds: u64,
    total_primality_tests: u64,
    total_primes_found: u64,
    fastest_seed_lane: String,
    fastest_seed_rate: f64,
    fastest_prime_lane: String,
    fastest_prime_rate: f64,
    exact_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    report_summary: ReportSummary,
    throughput_rows: Vec<ThroughputRow>,
    prime_witness_rows: Vec<PrimeWitnessRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let runs = LANE_SPECS
        .iter()
        .map(|spec| run_lane(spec, &options))
        .collect::<Vec<_>>();
    let throughput_rows = build_throughput_rows(&runs);
    let witness_rows = build_witness_rows(&runs);
    let summary = build_summary(&throughput_rows);
    let observations = build_observations(&throughput_rows);

    let speed_funnel_path = options.out_dir.join("speed_funnel.png");
    render_speed_funnel(&summary, &speed_funnel_path);
    let throughput_path = options.out_dir.join("throughput_by_lane.png");
    render_throughput_by_lane(&throughput_rows, &throughput_path);
    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "speed_funnel".to_string(),
            label: "Raw seeds to prime witnesses funnel".to_string(),
            path: speed_funnel_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "throughput_by_lane".to_string(),
            label: "Seed throughput by maintained lane".to_string(),
            path: throughput_path.display().to_string(),
        },
    ];

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        requested_seed_count: options.seed_count,
        max_primes: options.max_primes,
        wheel_period_cap: options.wheel_period_cap,
        deterministic_scope: "u64 candidates with primal::is_prime".to_string(),
    };
    let report_text = render_report(&settings, &summary, &throughput_rows, &observations);
    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        report_summary: summary.clone(),
        throughput_rows: throughput_rows.clone(),
        prime_witness_rows: witness_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        observations,
    };

    write_csv_rows(
        options.out_dir.join("throughput_rows.csv"),
        &throughput_rows,
    )
    .expect("write throughput rows");
    write_csv_rows(
        options.out_dir.join("prime_witness_rows.csv"),
        &witness_rows,
    )
    .expect("write witness rows");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_text_file(options.out_dir.join("report.md"), &report_text).expect("write report");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--release".to_string(),
                "--example".to_string(),
                "membrane_prime_throughput_report".to_string(),
            ],
            upstream_inputs: vec![
                "src/validation/fast_affine.rs".to_string(),
                "src/validation/bounded_k.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "throughput_rows.csv".to_string(),
                "prime_witness_rows.csv".to_string(),
                "speed_funnel.png".to_string(),
                "throughput_by_lane.png".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote membrane prime throughput bundle to {}",
        options.out_dir.display()
    );
    println!("{}", summary.exact_takeaway);
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut seed_count = DEFAULT_SEED_COUNT;
    let mut max_primes = DEFAULT_MAX_PRIMES;
    let mut wheel_period_cap = DEFAULT_WHEEL_PERIOD_CAP;
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
            "--max-primes" => {
                max_primes = args
                    .next()
                    .expect("missing --max-primes value")
                    .parse()
                    .expect("invalid --max-primes")
            }
            "--wheel-period-cap" => {
                wheel_period_cap = args
                    .next()
                    .expect("missing --wheel-period-cap value")
                    .parse()
                    .expect("invalid --wheel-period-cap")
            }
            _ => panic!("unrecognized argument: {arg}"),
        }
    }
    Options {
        out_dir,
        seed_count,
        max_primes,
        wheel_period_cap,
    }
}

fn run_lane(spec: &LaneSpec, options: &Options) -> (LaneSpec, FastPrimeRun) {
    let config = FastLaneConfig::new(
        spec.base,
        spec.outer,
        spec.inner,
        spec.middle_length,
        spec.k,
    );
    let run = scan_fast_prime_lane(
        config,
        options.seed_count,
        options.max_primes,
        options.wheel_period_cap,
    )
    .unwrap_or_else(|err| panic!("failed to scan {}: {err}", spec.role));
    (*spec, run)
}

fn build_throughput_rows(runs: &[(LaneSpec, FastPrimeRun)]) -> Vec<ThroughputRow> {
    runs.iter()
        .map(|(spec, run)| ThroughputRow {
            role: spec.role.to_string(),
            base: run.config.base,
            pair_label: run.pair_label.clone(),
            k_label: run.k_label.clone(),
            middle_length: run.config.middle_length,
            shift: run.shift,
            gradient: run.gradient,
            seed_capacity: run.seed_capacity,
            requested_seed_count: run.requested_seed_count,
            scanned_seed_count: run.scanned_seed_count,
            capped_to_seed_capacity: run.capped_to_seed_capacity,
            wheel_period: run.wheel_period,
            wheel_moduli_label: run
                .wheel_moduli
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("|"),
            wheel_admissible_residue_count: run.wheel_admissible_residue_count,
            admissible_seed_count: run.admissible_seed_count,
            admissible_share: ratio(run.admissible_seed_count, run.scanned_seed_count),
            primality_tests: run.primality_tests,
            primes_found: run.primes_found,
            prime_share_of_raw: ratio(run.primes_found, run.scanned_seed_count),
            elapsed_seconds: run.elapsed_seconds,
            seeds_per_second: run.seeds_per_second,
            primality_tests_per_second: run.primality_tests_per_second,
            primes_per_second: run.primes_per_second,
            note: spec.note.to_string(),
        })
        .collect()
}

fn build_witness_rows(runs: &[(LaneSpec, FastPrimeRun)]) -> Vec<PrimeWitnessRow> {
    runs.iter()
        .flat_map(|(spec, run)| {
            run.witnesses.iter().map(|witness| PrimeWitnessRow {
                role: spec.role.to_string(),
                base: run.config.base,
                pair_label: run.pair_label.clone(),
                k_label: run.k_label.clone(),
                middle_length: run.config.middle_length,
                seed: witness.seed,
                middle_digits: witness.middle_digits.clone(),
                template_digits: witness.template_digits.clone(),
                value: witness.value,
            })
        })
        .collect()
}

fn build_summary(rows: &[ThroughputRow]) -> ReportSummary {
    let fastest_seed = rows
        .iter()
        .max_by(|left, right| {
            left.seeds_per_second
                .partial_cmp(&right.seeds_per_second)
                .unwrap()
        })
        .expect("at least one lane");
    let fastest_prime = rows
        .iter()
        .max_by(|left, right| {
            left.primes_per_second
                .partial_cmp(&right.primes_per_second)
                .unwrap()
        })
        .expect("at least one lane");
    let total_scanned_seeds = rows.iter().map(|row| row.scanned_seed_count).sum();
    let total_admissible_seeds = rows.iter().map(|row| row.admissible_seed_count).sum();
    let total_primality_tests = rows.iter().map(|row| row.primality_tests).sum();
    let total_primes_found = rows.iter().map(|row| row.primes_found).sum();

    ReportSummary {
        lane_count: rows.len(),
        total_scanned_seeds,
        total_admissible_seeds,
        total_primality_tests,
        total_primes_found,
        fastest_seed_lane: fastest_seed.role.clone(),
        fastest_seed_rate: fastest_seed.seeds_per_second,
        fastest_prime_lane: fastest_prime.role.clone(),
        fastest_prime_rate: fastest_prime.primes_per_second,
        exact_takeaway:
            "fast generation is a throughput funnel: affine construction is cheap, residue filters skip deterministic obstructions, and u64 primality confirms survivors"
                .to_string(),
    }
}

fn build_observations(rows: &[ThroughputRow]) -> Vec<String> {
    let capped = rows
        .iter()
        .filter(|row| row.capped_to_seed_capacity)
        .count();
    let best = rows
        .iter()
        .max_by(|left, right| {
            left.primes_per_second
                .partial_cmp(&right.primes_per_second)
                .unwrap()
        })
        .expect("at least one lane");
    vec![
        format!(
            "{} of {} lanes cap the requested seed count to the finite `base^M` seed space; this avoids duplicate candidates.",
            capped,
            rows.len()
        ),
        format!(
            "The fastest observed prime-throughput lane in this run is `{}` at {:.2} primes/s.",
            best.role, best.primes_per_second
        ),
        "The old `membrane-prime-optimized` and `membrane-prime-ultra` binaries remain optimization sketches and are not used for these maintained claims.".to_string(),
    ]
}

fn render_report(
    settings: &ReportSettings,
    summary: &ReportSummary,
    rows: &[ThroughputRow],
    observations: &[String],
) -> String {
    let mut lines = Vec::new();
    lines.push("# Membrane Prime Throughput Report".to_string());
    lines.push(String::new());
    lines.push("## Settings".to_string());
    lines.push(format!("- output dir: `{}`", settings.out_dir));
    lines.push(format!(
        "- requested seed count: `{}`",
        settings.requested_seed_count
    ));
    lines.push(format!(
        "- max witnesses per lane: `{}`",
        settings.max_primes
    ));
    lines.push(format!(
        "- wheel period cap: `{}`",
        settings.wheel_period_cap
    ));
    lines.push(format!(
        "- deterministic scope: `{}`",
        settings.deterministic_scope
    ));
    lines.push(String::new());
    lines.push("## Throughput Funnel".to_string());
    lines.push("- Fixed lane: precompute `N(s) = shift + gradient*s`.".to_string());
    lines.push(
        "- Residue wheel: skip seed classes that are exactly divisible by small coprime moduli."
            .to_string(),
    );
    lines.push(
        "- Deterministic confirmation: run `primal::is_prime(u64)` only on survivors.".to_string(),
    );
    lines.push(
        "- This is a speed claim for specified lanes, not a global density theorem.".to_string(),
    );
    lines.push(String::new());
    lines.push("## Headline".to_string());
    lines.push(format!("- {}", summary.exact_takeaway));
    lines.push(format!(
        "- lanes: {}, raw seeds scanned: {}, admissible/primality tests: {}, primes found: {}",
        summary.lane_count,
        summary.total_scanned_seeds,
        summary.total_primality_tests,
        summary.total_primes_found
    ));
    lines.push(format!(
        "- fastest seed lane: `{}` ({:.0} seeds/s)",
        summary.fastest_seed_lane, summary.fastest_seed_rate
    ));
    lines.push(format!(
        "- fastest prime lane: `{}` ({:.2} primes/s)",
        summary.fastest_prime_lane, summary.fastest_prime_rate
    ));
    lines.push(String::new());
    lines.push("## Lane Rows".to_string());
    lines.push("| Role | Lane | Scanned | Admissible | Primes | Seeds/s | Primes/s |".to_string());
    lines.push("|---|---|---:|---:|---:|---:|---:|".to_string());
    for row in rows {
        lines.push(format!(
            "| `{}` | base {} {} M={} {} | {} | {} | {} | {:.0} | {:.2} |",
            row.role,
            row.base,
            row.pair_label,
            row.middle_length,
            row.k_label,
            row.scanned_seed_count,
            row.admissible_seed_count,
            row.primes_found,
            row.seeds_per_second,
            row.primes_per_second
        ));
    }
    lines.push(String::new());
    lines.push("## Observations".to_string());
    for observation in observations {
        lines.push(format!("- {}", observation));
    }
    lines.push(String::new());
    lines.push("## Artifacts".to_string());
    lines.push("- `throughput_rows.csv`: per-lane throughput and funnel metrics.".to_string());
    lines.push("- `prime_witness_rows.csv`: first prime witnesses found per lane.".to_string());
    lines.push("- `speed_funnel.png`: aggregate raw/admissible/test/prime funnel.".to_string());
    lines.push("- `throughput_by_lane.png`: per-lane seed throughput.".to_string());
    lines.join("\n")
}

fn render_speed_funnel(summary: &ReportSummary, path: &Path) {
    let root = BitMapBackend::new(path, (1200, 720)).into_drawing_area();
    root.fill(&RGBColor(250, 249, 246)).unwrap();
    root.draw(&Text::new(
        "Fast generation throughput funnel",
        (60, 70),
        ("sans-serif", 34).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    let bars = [
        (
            "raw seeds",
            summary.total_scanned_seeds,
            RGBColor(47, 103, 168),
        ),
        (
            "admissible",
            summary.total_admissible_seeds,
            RGBColor(24, 118, 117),
        ),
        (
            "primality tests",
            summary.total_primality_tests,
            RGBColor(92, 107, 122),
        ),
        (
            "prime witnesses",
            summary.total_primes_found,
            RGBColor(191, 61, 56),
        ),
    ];
    let max_value = bars.iter().map(|(_, value, _)| *value).max().unwrap_or(1) as f64;
    let mut y = 160;
    for (label, value, color) in bars {
        let width = ((value as f64 / max_value) * 720.0).max(2.0) as i32;
        root.draw(&Text::new(
            label.to_string(),
            (80, y + 28),
            ("sans-serif", 22).into_font().color(&RGBColor(38, 44, 51)),
        ))
        .unwrap();
        root.draw(&Rectangle::new(
            [(310, y), (310 + width, y + 48)],
            ShapeStyle::from(&color).filled(),
        ))
        .unwrap();
        root.draw(&Text::new(
            compact_count(value),
            (330 + width, y + 31),
            ("sans-serif", 20).into_font().color(&RGBColor(82, 88, 96)),
        ))
        .unwrap();
        y += 95;
    }
    root.present().unwrap();
}

fn render_throughput_by_lane(rows: &[ThroughputRow], path: &Path) {
    let root = BitMapBackend::new(path, (1500, 860)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    root.draw(&Text::new(
        "Seed throughput by maintained lane",
        (60, 70),
        ("sans-serif", 34).into_font().color(&RGBColor(38, 44, 51)),
    ))
    .unwrap();
    let max_rate = rows
        .iter()
        .map(|row| row.seeds_per_second)
        .fold(1.0_f64, f64::max);
    let mut y = 145;
    for row in rows {
        let width = ((row.seeds_per_second / max_rate) * 650.0).max(2.0) as i32;
        root.draw(&Text::new(
            row.role.replace('_', " "),
            (70, y + 28),
            ("sans-serif", 19).into_font().color(&RGBColor(38, 44, 51)),
        ))
        .unwrap();
        root.draw(&Rectangle::new(
            [(485, y), (485 + width, y + 38)],
            ShapeStyle::from(&RGBColor(47, 103, 168)).filled(),
        ))
        .unwrap();
        root.draw(&Text::new(
            format!(
                "{}/s seeds, {}/s primes",
                compact_rate(row.seeds_per_second),
                compact_rate(row.primes_per_second)
            ),
            (505 + width, y + 26),
            ("sans-serif", 17).into_font().color(&RGBColor(82, 88, 96)),
        ))
        .unwrap();
        y += 80;
    }
    root.present().unwrap();
}

fn ratio(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn compact_count(value: u64) -> String {
    if value >= 1_000_000 {
        format!("{:.2}M", value as f64 / 1_000_000.0)
    } else if value >= 1_000 {
        format!("{:.1}K", value as f64 / 1_000.0)
    } else {
        value.to_string()
    }
}

fn compact_rate(value: f64) -> String {
    if value >= 1_000_000.0 {
        format!("{:.2}M", value / 1_000_000.0)
    } else if value >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else {
        format!("{value:.1}")
    }
}
