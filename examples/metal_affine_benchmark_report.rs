//! Benchmark report for Metal affine candidate-transfer collapse.
//!
//! This report compares maintained affine generation against simple local
//! baseline prime-candidate strategies. It is not a claim about global prime
//! density; it measures specified deterministic `u64` windows.
//!
//! Run with:
//!
//! ```bash
//! cargo run --features metal --release --example metal_affine_benchmark_report -- --out-dir /tmp/primes_metal_affine_benchmark
//! ```

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use plotters::prelude::*;
use primes::miller_rabin_test;
use primes::validation::{
    bounded_k::{digit_symbol, DEFAULT_PREFILTER_PRIMES},
    fast_affine::{build_fast_affine_lane, scan_fast_prime_lane, FastAffineLane, FastLaneConfig},
    metal_affine::{
        build_metal_affine_residue_row_batches, build_metal_affine_residue_rows,
        cpu_affine_survivor_seeds, default_metal_affine_moduli, residue_rows_allow_local_seed,
        scan_metal_affine_lane, sieve_metal_affine_residue_batches, MetalAffineResidueRow,
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
    process::Command,
    time::Instant,
};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_metal_affine_benchmark";
const ARTIFACT_ID: &str = "metal_affine_benchmark_report";
const REPORT_EXPORT_VERSION: u32 = 1;
const DEFAULT_SEED_COUNT: u64 = 1_000_000;
const DEFAULT_MAX_PRIMES: usize = 10;
const DEFAULT_WHEEL_PERIOD_CAP: u64 = 1_000_000;
const DEFAULT_EXTERNAL_INTERVAL_DISTANCE: u64 = 1_000_000;
const MAX_EXTERNAL_OPENSSL_GENERATIONS: usize = 20;
const DEFAULT_METAL_BATCH_SEED_COUNT: u64 = 1_000_000;
const DEFAULT_BIGUINT_SEED_COUNT: u64 = 20_000;
const DEFAULT_BIGUINT_MILLER_RABIN_ROUNDS: usize = 20;
const DEFAULT_BIGUINT_MIDDLE_LENGTHS: &[usize] = &[12, 15, 18];
const DEFAULT_U128_SEED_COUNT: u64 = 20_000;
const DEFAULT_U128_MIDDLE_LENGTHS: &[usize] = &[12, 15, 18, 21, 24, 27, 28];
const U128_MILLER_RABIN_BASES: &[u128] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    seed_count: u64,
    max_primes: usize,
    wheel_period_cap: u64,
    metal_batch_seed_count: u64,
    biguint_seed_count: u64,
    biguint_miller_rabin_rounds: usize,
    biguint_middle_lengths: Vec<usize>,
    u128_seed_count: u64,
    u128_middle_lengths: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
struct LaneSpec {
    role: &'static str,
    base: u32,
    outer: u32,
    inner: u32,
    middle_length: usize,
    k: (u32, u32),
    baseline_limit: u64,
    note: &'static str,
}

const LANE_SPECS: &[LaneSpec] = &[
    LaneSpec {
        role: "decimal_16_digit_visible_lane",
        base: 10,
        outer: 3,
        inner: 7,
        middle_length: 6,
        k: (2, 1),
        baseline_limit: 200_000,
        note: "16-digit visible decimal lane with one million finite seeds",
    },
    LaneSpec {
        role: "decimal_18_digit_visible_lane",
        base: 10,
        outer: 3,
        inner: 7,
        middle_length: 8,
        k: (2, 1),
        baseline_limit: 200_000,
        note: "18-digit visible decimal lane; scanned as a prefix of its larger seed space",
    },
    LaneSpec {
        role: "decimal_19_digit_visible_lane",
        base: 10,
        outer: 3,
        inner: 7,
        middle_length: 9,
        k: (2, 1),
        baseline_limit: 200_000,
        note: "19-digit visible decimal lane near the u64 ceiling; scanned as a prefix of its billion-seed space",
    },
    LaneSpec {
        role: "base22_19_digit_side_pocket",
        base: 22,
        outer: 17,
        inner: 19,
        middle_length: 2,
        k: (2, 2),
        baseline_limit: 50_000,
        note: "small base-22 pocket with 19-digit decimal values and compact finite seed space",
    },
];

#[derive(Debug, Clone, Serialize)]
struct BenchmarkRow {
    role: String,
    path: String,
    status: String,
    base: u32,
    pair_label: String,
    k_label: String,
    middle_length: usize,
    visible_template_digits: usize,
    decimal_digits_min: usize,
    decimal_digits_max: usize,
    scanned_count: u64,
    survivor_count: u64,
    primality_tests: u64,
    primes_found: u64,
    candidate_value_buffer_bytes: u64,
    input_metadata_bytes: u64,
    output_bytes: u64,
    avoided_candidate_value_bytes_u64: u64,
    sieve_seconds: f64,
    confirm_seconds: f64,
    total_seconds: f64,
    warm_path_seconds: f64,
    raw_per_second_total: f64,
    raw_per_second_warm: f64,
    tests_per_second: f64,
    primes_per_second_total: f64,
    primes_per_second_warm: f64,
    first_prime: String,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct PrimeWitnessRow {
    role: String,
    path: String,
    seed_or_index: u64,
    value: u64,
    template_digits: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    requested_seed_count: u64,
    max_primes: usize,
    wheel_period_cap: u64,
    metal_batch_seed_count: u64,
    biguint_seed_count: u64,
    biguint_miller_rabin_rounds: usize,
    biguint_middle_lengths: Vec<usize>,
    u128_seed_count: u64,
    u128_middle_lengths: Vec<usize>,
    u128_miller_rabin_bases: Vec<u128>,
    metal_host_surface: String,
    metal_kernel_surface: String,
    external_comparison_status: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    lane_count: usize,
    benchmark_row_count: usize,
    fastest_prime_path: String,
    fastest_prime_rate: f64,
    largest_decimal_digits: usize,
    total_u64_candidate_value_bytes_avoided_by_metal: u64,
    exact_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExternalComparisonRow {
    family: String,
    representative_source: String,
    established_shape: String,
    comparison_to_affine_transfer_collapse: String,
    citation_url: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExternalBenchmarkRow {
    family: String,
    path: String,
    status: String,
    command: String,
    problem_shape: String,
    requested_work: u64,
    values_found: u64,
    elapsed_seconds: f64,
    work_per_second: f64,
    values_per_second: f64,
    first_value: String,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct MetalBatchDispatchRow {
    role: String,
    path: String,
    status: String,
    scanned_count: u64,
    batch_seed_count: u64,
    batch_count: usize,
    survivor_count: u64,
    setup_seconds: f64,
    buffer_prepare_seconds: f64,
    gpu_sieve_seconds: f64,
    unpack_seconds: f64,
    total_seconds: f64,
    raw_per_second_dispatch_only: f64,
    raw_per_second_total: f64,
    input_metadata_bytes: u64,
    output_bitmask_bytes: u64,
    avoided_candidate_value_bytes_u64: u64,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct BigUintProbablePrimeRow {
    role: String,
    path: String,
    status: String,
    base: u32,
    pair_label: String,
    k_label: String,
    middle_length: usize,
    visible_template_digits: usize,
    decimal_digits_min: usize,
    scanned_count: u64,
    survivor_count: u64,
    probable_prime_tests: u64,
    probable_primes_found: u64,
    survivor_share: f64,
    probable_prime_share_of_raw: f64,
    probable_prime_share_of_survivors: f64,
    seeds_per_probable_prime: f64,
    residue_sieve_seconds: f64,
    probable_prime_seconds: f64,
    total_seconds: f64,
    raw_per_second: f64,
    tests_per_second: f64,
    probable_primes_per_second: f64,
    first_probable_prime: String,
    first_template_digits: String,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct U128ProbablePrimeRow {
    role: String,
    path: String,
    status: String,
    base: u32,
    pair_label: String,
    k_label: String,
    middle_length: usize,
    visible_template_digits: usize,
    decimal_digits_min: usize,
    scanned_count: u64,
    survivor_count: u64,
    probable_prime_tests: u64,
    probable_primes_found: u64,
    survivor_share: f64,
    probable_prime_share_of_raw: f64,
    probable_prime_share_of_survivors: f64,
    seeds_per_probable_prime: f64,
    residue_sieve_seconds: f64,
    probable_prime_seconds: f64,
    total_seconds: f64,
    raw_per_second: f64,
    tests_per_second: f64,
    probable_primes_per_second: f64,
    first_probable_prime: String,
    first_template_digits: String,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    summary: ReportSummary,
    benchmark_rows: Vec<BenchmarkRow>,
    prime_witness_rows: Vec<PrimeWitnessRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    metal_batch_dispatch_rows: Vec<MetalBatchDispatchRow>,
    biguint_probable_prime_rows: Vec<BigUintProbablePrimeRow>,
    u128_probable_prime_rows: Vec<U128ProbablePrimeRow>,
    external_benchmark_rows: Vec<ExternalBenchmarkRow>,
    external_comparison_rows: Vec<ExternalComparisonRow>,
    observations: Vec<String>,
}

struct ReportTables<'a> {
    benchmark_rows: &'a [BenchmarkRow],
    metal_batch_rows: &'a [MetalBatchDispatchRow],
    biguint_rows: &'a [BigUintProbablePrimeRow],
    u128_rows: &'a [U128ProbablePrimeRow],
    external_benchmark_rows: &'a [ExternalBenchmarkRow],
    external_rows: &'a [ExternalComparisonRow],
    observations: &'a [String],
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output dir");

    let mut benchmark_rows = Vec::new();
    let mut witness_rows = Vec::new();
    for spec in LANE_SPECS {
        let config = FastLaneConfig::new(
            spec.base,
            spec.outer,
            spec.inner,
            spec.middle_length,
            spec.k,
        );
        let lane = build_fast_affine_lane(config.clone())
            .unwrap_or_else(|err| panic!("failed to build lane {}: {err}", spec.role));
        append_cpu_fast_affine_rows(
            spec,
            &options,
            config.clone(),
            &mut benchmark_rows,
            &mut witness_rows,
        );
        append_cpu_residue_row(
            spec,
            &options,
            &lane,
            &mut benchmark_rows,
            &mut witness_rows,
        );
        append_metal_affine_row(
            spec,
            &options,
            config,
            &mut benchmark_rows,
            &mut witness_rows,
        );
        append_baseline_rows(
            spec,
            &options,
            &lane,
            &mut benchmark_rows,
            &mut witness_rows,
        );
    }

    let throughput_path = options.out_dir.join("prime_throughput_by_path.png");
    render_prime_throughput_chart(&benchmark_rows, &throughput_path);
    let transfer_path = options.out_dir.join("candidate_transfer_bytes.png");
    render_transfer_chart(&benchmark_rows, &transfer_path);
    let image_rows = vec![
        ImageArtifactRow {
            kind: "prime_throughput_by_path".to_string(),
            path: throughput_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "candidate_transfer_bytes".to_string(),
            path: transfer_path.display().to_string(),
        },
    ];
    let metal_batch_dispatch_rows = build_metal_batch_dispatch_rows(&options);
    let biguint_probable_prime_rows = build_biguint_probable_prime_rows(&options);
    let u128_probable_prime_rows = build_u128_probable_prime_rows(&options);
    let external_benchmark_rows = build_external_benchmark_rows(&options);

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        requested_seed_count: options.seed_count,
        max_primes: options.max_primes,
        wheel_period_cap: options.wheel_period_cap,
        metal_batch_seed_count: options.metal_batch_seed_count,
        biguint_seed_count: options.biguint_seed_count,
        biguint_miller_rabin_rounds: options.biguint_miller_rabin_rounds,
        biguint_middle_lengths: options.biguint_middle_lengths.clone(),
        u128_seed_count: options.u128_seed_count,
        u128_middle_lengths: options.u128_middle_lengths.clone(),
        u128_miller_rabin_bases: U128_MILLER_RABIN_BASES.to_vec(),
        metal_host_surface:
            "Rust `metal` crate host API: pipeline loading, shared buffers, dispatch, readback"
                .to_string(),
        metal_kernel_surface:
            "Dedicated `.metal` shader: `shaders/sieve_affine.metal::sieve_affine_lane`".to_string(),
        external_comparison_status:
            "primary-source comparison frame captured; optional local CLI adapters measured when available"
                .to_string(),
    };
    let summary = build_summary(
        &benchmark_rows,
        &biguint_probable_prime_rows,
        &u128_probable_prime_rows,
    );
    let observations = build_observations(
        &benchmark_rows,
        &metal_batch_dispatch_rows,
        &biguint_probable_prime_rows,
        &u128_probable_prime_rows,
        &external_benchmark_rows,
    );
    let external_rows = build_external_comparison_rows();
    let report = render_report(
        &settings,
        &summary,
        ReportTables {
            benchmark_rows: &benchmark_rows,
            metal_batch_rows: &metal_batch_dispatch_rows,
            biguint_rows: &biguint_probable_prime_rows,
            u128_rows: &u128_probable_prime_rows,
            external_benchmark_rows: &external_benchmark_rows,
            external_rows: &external_rows,
            observations: &observations,
        },
    );
    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        summary: summary.clone(),
        benchmark_rows: benchmark_rows.clone(),
        prime_witness_rows: witness_rows.clone(),
        image_artifact_rows: image_rows,
        metal_batch_dispatch_rows: metal_batch_dispatch_rows.clone(),
        biguint_probable_prime_rows: biguint_probable_prime_rows.clone(),
        u128_probable_prime_rows: u128_probable_prime_rows.clone(),
        external_benchmark_rows: external_benchmark_rows.clone(),
        external_comparison_rows: external_rows,
        observations,
    };

    write_csv_rows(options.out_dir.join("benchmark_rows.csv"), &benchmark_rows)
        .expect("write benchmark rows");
    write_csv_rows(
        options.out_dir.join("prime_witness_rows.csv"),
        &witness_rows,
    )
    .expect("write witness rows");
    write_csv_rows(
        options.out_dir.join("external_benchmark_rows.csv"),
        &external_benchmark_rows,
    )
    .expect("write external benchmark rows");
    write_csv_rows(
        options.out_dir.join("metal_batch_dispatch_rows.csv"),
        &metal_batch_dispatch_rows,
    )
    .expect("write metal batch dispatch rows");
    write_csv_rows(
        options.out_dir.join("biguint_probable_prime_rows.csv"),
        &biguint_probable_prime_rows,
    )
    .expect("write BigUint probable prime rows");
    write_csv_rows(
        options.out_dir.join("u128_probable_prime_rows.csv"),
        &u128_probable_prime_rows,
    )
    .expect("write u128 probable prime rows");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_text_file(options.out_dir.join("report.md"), &report).expect("write report");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--features".to_string(),
                "metal".to_string(),
                "--release".to_string(),
                "--example".to_string(),
                "metal_affine_benchmark_report".to_string(),
            ],
            upstream_inputs: vec![
                "src/validation/metal_affine.rs".to_string(),
                "src/validation/fast_affine.rs".to_string(),
                "shaders/sieve_affine.metal".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "benchmark_rows.csv".to_string(),
                "prime_witness_rows.csv".to_string(),
                "external_benchmark_rows.csv".to_string(),
                "metal_batch_dispatch_rows.csv".to_string(),
                "biguint_probable_prime_rows.csv".to_string(),
                "u128_probable_prime_rows.csv".to_string(),
                "prime_throughput_by_path.png".to_string(),
                "candidate_transfer_bytes.png".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote Metal affine benchmark bundle to {}",
        options.out_dir.display()
    );
    println!("{}", summary.exact_takeaway);
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut seed_count = DEFAULT_SEED_COUNT;
    let mut max_primes = DEFAULT_MAX_PRIMES;
    let mut wheel_period_cap = DEFAULT_WHEEL_PERIOD_CAP;
    let mut metal_batch_seed_count = DEFAULT_METAL_BATCH_SEED_COUNT;
    let mut biguint_seed_count = DEFAULT_BIGUINT_SEED_COUNT;
    let mut biguint_miller_rabin_rounds = DEFAULT_BIGUINT_MILLER_RABIN_ROUNDS;
    let mut biguint_middle_lengths = DEFAULT_BIGUINT_MIDDLE_LENGTHS.to_vec();
    let mut u128_seed_count = DEFAULT_U128_SEED_COUNT;
    let mut u128_middle_lengths = DEFAULT_U128_MIDDLE_LENGTHS.to_vec();
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
            "--metal-batch-seed-count" => {
                metal_batch_seed_count = args
                    .next()
                    .expect("missing --metal-batch-seed-count value")
                    .parse()
                    .expect("invalid --metal-batch-seed-count")
            }
            "--biguint-seed-count" => {
                biguint_seed_count = args
                    .next()
                    .expect("missing --biguint-seed-count value")
                    .parse()
                    .expect("invalid --biguint-seed-count")
            }
            "--biguint-miller-rabin-rounds" => {
                biguint_miller_rabin_rounds = args
                    .next()
                    .expect("missing --biguint-miller-rabin-rounds value")
                    .parse()
                    .expect("invalid --biguint-miller-rabin-rounds")
            }
            "--biguint-middle-lengths" => {
                biguint_middle_lengths =
                    parse_usize_list(&args.next().expect("missing --biguint-middle-lengths value"))
                        .expect("invalid --biguint-middle-lengths");
            }
            "--u128-seed-count" => {
                u128_seed_count = args
                    .next()
                    .expect("missing --u128-seed-count value")
                    .parse()
                    .expect("invalid --u128-seed-count")
            }
            "--u128-middle-lengths" => {
                u128_middle_lengths =
                    parse_usize_list(&args.next().expect("missing --u128-middle-lengths value"))
                        .expect("invalid --u128-middle-lengths");
            }
            _ => panic!("unrecognized argument: {arg}"),
        }
    }
    Options {
        out_dir,
        seed_count,
        max_primes,
        wheel_period_cap,
        metal_batch_seed_count,
        biguint_seed_count,
        biguint_miller_rabin_rounds,
        biguint_middle_lengths,
        u128_seed_count,
        u128_middle_lengths,
    }
}

fn parse_usize_list(value: &str) -> Result<Vec<usize>, String> {
    let parsed = value
        .split(',')
        .filter(|part| !part.trim().is_empty())
        .map(|part| {
            part.trim()
                .parse::<usize>()
                .map_err(|err| format!("invalid integer `{part}`: {err}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if parsed.is_empty() {
        Err("list must contain at least one middle length".to_string())
    } else {
        Ok(parsed)
    }
}

fn append_cpu_fast_affine_rows(
    spec: &LaneSpec,
    options: &Options,
    config: FastLaneConfig,
    rows: &mut Vec<BenchmarkRow>,
    witnesses: &mut Vec<PrimeWitnessRow>,
) {
    let run = scan_fast_prime_lane(
        config,
        options.seed_count,
        options.max_primes,
        options.wheel_period_cap,
    )
    .unwrap_or_else(|err| panic!("CPU fast affine failed for {}: {err}", spec.role));
    let lane = build_fast_affine_lane(run.config.clone()).expect("lane should rebuild");
    for witness in &run.witnesses {
        witnesses.push(PrimeWitnessRow {
            role: spec.role.to_string(),
            path: "cpu_affine_wheel".to_string(),
            seed_or_index: witness.seed,
            value: witness.value,
            template_digits: witness.template_digits.clone(),
        });
    }
    rows.push(BenchmarkRow {
        role: spec.role.to_string(),
        path: "cpu_affine_wheel".to_string(),
        status: "ok".to_string(),
        base: run.config.base,
        pair_label: run.pair_label,
        k_label: run.k_label,
        middle_length: run.config.middle_length,
        visible_template_digits: lane.template_digits(0).len(),
        decimal_digits_min: lane.shift.to_string().len(),
        decimal_digits_max: lane.max_candidate.to_string().len(),
        scanned_count: run.scanned_seed_count,
        survivor_count: run.admissible_seed_count,
        primality_tests: run.primality_tests,
        primes_found: run.primes_found,
        candidate_value_buffer_bytes: 0,
        input_metadata_bytes: 0,
        output_bytes: 0,
        avoided_candidate_value_bytes_u64: 0,
        sieve_seconds: 0.0,
        confirm_seconds: run.elapsed_seconds,
        total_seconds: run.elapsed_seconds,
        warm_path_seconds: run.elapsed_seconds,
        raw_per_second_total: ratio_f64(run.scanned_seed_count, run.elapsed_seconds),
        raw_per_second_warm: ratio_f64(run.scanned_seed_count, run.elapsed_seconds),
        tests_per_second: ratio_f64(run.primality_tests, run.elapsed_seconds),
        primes_per_second_total: ratio_f64(run.primes_found, run.elapsed_seconds),
        primes_per_second_warm: ratio_f64(run.primes_found, run.elapsed_seconds),
        first_prime: run
            .witnesses
            .first()
            .map(|witness| witness.value.to_string())
            .unwrap_or_default(),
        note: spec.note.to_string(),
    });
}

fn append_cpu_residue_row(
    spec: &LaneSpec,
    options: &Options,
    lane: &FastAffineLane,
    rows: &mut Vec<BenchmarkRow>,
    witnesses: &mut Vec<PrimeWitnessRow>,
) {
    let scanned = options.seed_count.min(lane.seed_capacity);
    let moduli = default_metal_affine_moduli(lane);
    let residue_rows = build_metal_affine_residue_rows(lane, 0, &moduli).expect("residue rows");
    let sieve_start = Instant::now();
    let survivor_seeds =
        cpu_affine_survivor_seeds(lane, 0, scanned, &residue_rows).expect("cpu residues");
    let sieve_seconds = sieve_start.elapsed().as_secs_f64();

    let confirm_start = Instant::now();
    let mut primes_found = 0u64;
    let mut first_prime = String::new();
    for &seed in &survivor_seeds {
        let value = lane.candidate_value(seed).expect("candidate value");
        if primal::is_prime(value) {
            primes_found += 1;
            if first_prime.is_empty() {
                first_prime = value.to_string();
            }
            if witnesses
                .iter()
                .filter(|row| row.role == spec.role && row.path == "cpu_affine_residue_rows")
                .count()
                < options.max_primes
            {
                witnesses.push(PrimeWitnessRow {
                    role: spec.role.to_string(),
                    path: "cpu_affine_residue_rows".to_string(),
                    seed_or_index: seed,
                    value,
                    template_digits: lane.template_digits(seed),
                });
            }
        }
    }
    let confirm_seconds = confirm_start.elapsed().as_secs_f64();
    let total_seconds = sieve_seconds + confirm_seconds;
    rows.push(BenchmarkRow {
        role: spec.role.to_string(),
        path: "cpu_affine_residue_rows".to_string(),
        status: "ok".to_string(),
        base: lane.config.base,
        pair_label: lane.config.pair_label(),
        k_label: lane.config.k_label(),
        middle_length: lane.config.middle_length,
        visible_template_digits: lane.template_digits(0).len(),
        decimal_digits_min: lane.shift.to_string().len(),
        decimal_digits_max: lane.max_candidate.to_string().len(),
        scanned_count: scanned,
        survivor_count: survivor_seeds.len() as u64,
        primality_tests: survivor_seeds.len() as u64,
        primes_found,
        candidate_value_buffer_bytes: 0,
        input_metadata_bytes: residue_rows.len() as u64 * 16 + 16,
        output_bytes: 0,
        avoided_candidate_value_bytes_u64: 0,
        sieve_seconds,
        confirm_seconds,
        total_seconds,
        warm_path_seconds: total_seconds,
        raw_per_second_total: ratio_f64(scanned, total_seconds),
        raw_per_second_warm: ratio_f64(scanned, total_seconds),
        tests_per_second: ratio_f64(survivor_seeds.len() as u64, confirm_seconds),
        primes_per_second_total: ratio_f64(primes_found, total_seconds),
        primes_per_second_warm: ratio_f64(primes_found, total_seconds),
        first_prime,
        note: "CPU baseline using the same residue rows as the Metal kernel".to_string(),
    });
}

fn append_metal_affine_row(
    spec: &LaneSpec,
    options: &Options,
    config: FastLaneConfig,
    rows: &mut Vec<BenchmarkRow>,
    witnesses: &mut Vec<PrimeWitnessRow>,
) {
    match scan_metal_affine_lane(config, options.seed_count, options.max_primes, 0, None) {
        Ok(run) => {
            let lane = build_fast_affine_lane(run.config.clone()).expect("lane should rebuild");
            for witness in &run.witnesses {
                witnesses.push(PrimeWitnessRow {
                    role: spec.role.to_string(),
                    path: "metal_affine_transfer_collapse".to_string(),
                    seed_or_index: witness.seed,
                    value: witness.value,
                    template_digits: witness.template_digits.clone(),
                });
            }
            let warm_path_seconds = run.metrics.gpu_sieve_seconds + run.metrics.cpu_confirm_seconds;
            rows.push(BenchmarkRow {
                role: spec.role.to_string(),
                path: "metal_affine_transfer_collapse".to_string(),
                status: "ok".to_string(),
                base: run.config.base,
                pair_label: run.pair_label,
                k_label: run.k_label,
                middle_length: run.config.middle_length,
                visible_template_digits: lane.template_digits(0).len(),
                decimal_digits_min: lane.shift.to_string().len(),
                decimal_digits_max: lane.max_candidate.to_string().len(),
                scanned_count: run.scanned_seed_count,
                survivor_count: run.survivor_seed_count,
                primality_tests: run.primality_tests,
                primes_found: run.primes_found,
                candidate_value_buffer_bytes: 0,
                input_metadata_bytes: run.metrics.input_metadata_bytes,
                output_bytes: run.metrics.output_bitmask_bytes,
                avoided_candidate_value_bytes_u64: run.metrics.avoided_candidate_value_bytes_u64,
                sieve_seconds: run.metrics.gpu_sieve_seconds,
                confirm_seconds: run.metrics.cpu_confirm_seconds,
                total_seconds: run.metrics.total_seconds,
                warm_path_seconds,
                raw_per_second_total: ratio_f64(run.scanned_seed_count, run.metrics.total_seconds),
                raw_per_second_warm: ratio_f64(run.scanned_seed_count, warm_path_seconds),
                tests_per_second: ratio_f64(run.primality_tests, run.metrics.cpu_confirm_seconds),
                primes_per_second_total: ratio_f64(run.primes_found, run.metrics.total_seconds),
                primes_per_second_warm: ratio_f64(run.primes_found, warm_path_seconds),
                first_prime: run
                    .witnesses
                    .first()
                    .map(|witness| witness.value.to_string())
                    .unwrap_or_default(),
                note: "Metal path sends residue metadata and receives a survivor bitmask; candidate values are not transferred".to_string(),
            });
        }
        Err(err) => rows.push(unavailable_row(
            spec,
            "metal_affine_transfer_collapse",
            &format!("unavailable: {err}"),
        )),
    }
}

fn append_baseline_rows(
    spec: &LaneSpec,
    options: &Options,
    lane: &FastAffineLane,
    rows: &mut Vec<BenchmarkRow>,
    witnesses: &mut Vec<PrimeWitnessRow>,
) {
    let baseline_count = options
        .seed_count
        .min(spec.baseline_limit)
        .min(lane.seed_capacity.max(1));
    append_odd_baseline(
        spec,
        lane,
        baseline_count,
        options.max_primes,
        rows,
        witnesses,
    );
    append_wheel_baseline(
        spec,
        lane,
        baseline_count,
        options.max_primes,
        rows,
        witnesses,
    );
    append_random_odd_baseline(
        spec,
        lane,
        baseline_count,
        options.max_primes,
        rows,
        witnesses,
    );
}

fn append_odd_baseline(
    spec: &LaneSpec,
    lane: &FastAffineLane,
    count: u64,
    max_primes: usize,
    rows: &mut Vec<BenchmarkRow>,
    witnesses: &mut Vec<PrimeWitnessRow>,
) {
    let start_value = if lane.shift.is_multiple_of(2) {
        lane.shift + 1
    } else {
        lane.shift
    };
    let start = Instant::now();
    let mut primes_found = 0u64;
    let mut first_prime = String::new();
    for index in 0..count {
        let value = start_value.saturating_add(index.saturating_mul(2));
        if primal::is_prime(value) {
            primes_found += 1;
            if first_prime.is_empty() {
                first_prime = value.to_string();
            }
            if witnesses
                .iter()
                .filter(|row| row.role == spec.role && row.path == "sequential_odd_baseline")
                .count()
                < max_primes
            {
                witnesses.push(PrimeWitnessRow {
                    role: spec.role.to_string(),
                    path: "sequential_odd_baseline".to_string(),
                    seed_or_index: index,
                    value,
                    template_digits: String::new(),
                });
            }
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    rows.push(BenchmarkRow {
        role: spec.role.to_string(),
        path: "sequential_odd_baseline".to_string(),
        status: "ok".to_string(),
        base: lane.config.base,
        pair_label: lane.config.pair_label(),
        k_label: lane.config.k_label(),
        middle_length: lane.config.middle_length,
        visible_template_digits: lane.template_digits(0).len(),
        decimal_digits_min: start_value.to_string().len(),
        decimal_digits_max: start_value
            .saturating_add(count.saturating_mul(2))
            .to_string()
            .len(),
        scanned_count: count,
        survivor_count: count,
        primality_tests: count,
        primes_found,
        candidate_value_buffer_bytes: 0,
        input_metadata_bytes: 0,
        output_bytes: 0,
        avoided_candidate_value_bytes_u64: 0,
        sieve_seconds: 0.0,
        confirm_seconds: elapsed,
        total_seconds: elapsed,
        warm_path_seconds: elapsed,
        raw_per_second_total: ratio_f64(count, elapsed),
        raw_per_second_warm: ratio_f64(count, elapsed),
        tests_per_second: ratio_f64(count, elapsed),
        primes_per_second_total: ratio_f64(primes_found, elapsed),
        primes_per_second_warm: ratio_f64(primes_found, elapsed),
        first_prime,
        note: "Established simple baseline: scan odd numbers near the same magnitude".to_string(),
    });
}

fn append_wheel_baseline(
    spec: &LaneSpec,
    lane: &FastAffineLane,
    count: u64,
    max_primes: usize,
    rows: &mut Vec<BenchmarkRow>,
    witnesses: &mut Vec<PrimeWitnessRow>,
) {
    let start_value = if lane.shift.is_multiple_of(2) {
        lane.shift + 1
    } else {
        lane.shift
    };
    let small_primes = [3u64, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    let start = Instant::now();
    let mut survivor_count = 0u64;
    let mut primes_found = 0u64;
    let mut first_prime = String::new();
    for index in 0..count {
        let value = start_value.saturating_add(index.saturating_mul(2));
        if small_primes
            .iter()
            .any(|&prime| value != prime && value.is_multiple_of(prime))
        {
            continue;
        }
        survivor_count += 1;
        if primal::is_prime(value) {
            primes_found += 1;
            if first_prime.is_empty() {
                first_prime = value.to_string();
            }
            if witnesses
                .iter()
                .filter(|row| row.role == spec.role && row.path == "sequential_small_prime_wheel")
                .count()
                < max_primes
            {
                witnesses.push(PrimeWitnessRow {
                    role: spec.role.to_string(),
                    path: "sequential_small_prime_wheel".to_string(),
                    seed_or_index: index,
                    value,
                    template_digits: String::new(),
                });
            }
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    rows.push(BenchmarkRow {
        role: spec.role.to_string(),
        path: "sequential_small_prime_wheel".to_string(),
        status: "ok".to_string(),
        base: lane.config.base,
        pair_label: lane.config.pair_label(),
        k_label: lane.config.k_label(),
        middle_length: lane.config.middle_length,
        visible_template_digits: lane.template_digits(0).len(),
        decimal_digits_min: start_value.to_string().len(),
        decimal_digits_max: start_value
            .saturating_add(count.saturating_mul(2))
            .to_string()
            .len(),
        scanned_count: count,
        survivor_count,
        primality_tests: survivor_count,
        primes_found,
        candidate_value_buffer_bytes: 0,
        input_metadata_bytes: 0,
        output_bytes: 0,
        avoided_candidate_value_bytes_u64: 0,
        sieve_seconds: 0.0,
        confirm_seconds: elapsed,
        total_seconds: elapsed,
        warm_path_seconds: elapsed,
        raw_per_second_total: ratio_f64(count, elapsed),
        raw_per_second_warm: ratio_f64(count, elapsed),
        tests_per_second: ratio_f64(survivor_count, elapsed),
        primes_per_second_total: ratio_f64(primes_found, elapsed),
        primes_per_second_warm: ratio_f64(primes_found, elapsed),
        first_prime,
        note: "Established baseline: ordinary odd scan with small-prime prefilter".to_string(),
    });
}

fn append_random_odd_baseline(
    spec: &LaneSpec,
    lane: &FastAffineLane,
    count: u64,
    max_primes: usize,
    rows: &mut Vec<BenchmarkRow>,
    witnesses: &mut Vec<PrimeWitnessRow>,
) {
    let lower = lane.shift;
    let upper = lane
        .candidate_value(count.min(lane.seed_capacity).saturating_sub(1))
        .unwrap_or(lane.max_candidate)
        .max(lower + 1);
    let span = upper - lower + 1;
    let start = Instant::now();
    let mut rng = XorShift64::new(lane.shift ^ lane.gradient ^ count);
    let mut primes_found = 0u64;
    let mut first_prime = String::new();
    for index in 0..count {
        let mut value = lower + (rng.next() % span);
        if value.is_multiple_of(2) {
            value = if value < upper { value + 1 } else { value - 1 };
        }
        if primal::is_prime(value) {
            primes_found += 1;
            if first_prime.is_empty() {
                first_prime = value.to_string();
            }
            if witnesses
                .iter()
                .filter(|row| row.role == spec.role && row.path == "random_odd_same_window")
                .count()
                < max_primes
            {
                witnesses.push(PrimeWitnessRow {
                    role: spec.role.to_string(),
                    path: "random_odd_same_window".to_string(),
                    seed_or_index: index,
                    value,
                    template_digits: String::new(),
                });
            }
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    rows.push(BenchmarkRow {
        role: spec.role.to_string(),
        path: "random_odd_same_window".to_string(),
        status: "ok".to_string(),
        base: lane.config.base,
        pair_label: lane.config.pair_label(),
        k_label: lane.config.k_label(),
        middle_length: lane.config.middle_length,
        visible_template_digits: lane.template_digits(0).len(),
        decimal_digits_min: lower.to_string().len(),
        decimal_digits_max: upper.to_string().len(),
        scanned_count: count,
        survivor_count: count,
        primality_tests: count,
        primes_found,
        candidate_value_buffer_bytes: 0,
        input_metadata_bytes: 0,
        output_bytes: 0,
        avoided_candidate_value_bytes_u64: 0,
        sieve_seconds: 0.0,
        confirm_seconds: elapsed,
        total_seconds: elapsed,
        warm_path_seconds: elapsed,
        raw_per_second_total: ratio_f64(count, elapsed),
        raw_per_second_warm: ratio_f64(count, elapsed),
        tests_per_second: ratio_f64(count, elapsed),
        primes_per_second_total: ratio_f64(primes_found, elapsed),
        primes_per_second_warm: ratio_f64(primes_found, elapsed),
        first_prime,
        note: "Established baseline: deterministic random odd candidates in the same value window"
            .to_string(),
    });
}

fn unavailable_row(spec: &LaneSpec, path: &str, status: &str) -> BenchmarkRow {
    BenchmarkRow {
        role: spec.role.to_string(),
        path: path.to_string(),
        status: status.to_string(),
        base: spec.base,
        pair_label: format!("({},{})", spec.outer, spec.inner),
        k_label: format!("k=({},{})", spec.k.0, spec.k.1),
        middle_length: spec.middle_length,
        visible_template_digits: 0,
        decimal_digits_min: 0,
        decimal_digits_max: 0,
        scanned_count: 0,
        survivor_count: 0,
        primality_tests: 0,
        primes_found: 0,
        candidate_value_buffer_bytes: 0,
        input_metadata_bytes: 0,
        output_bytes: 0,
        avoided_candidate_value_bytes_u64: 0,
        sieve_seconds: 0.0,
        confirm_seconds: 0.0,
        total_seconds: 0.0,
        warm_path_seconds: 0.0,
        raw_per_second_total: 0.0,
        raw_per_second_warm: 0.0,
        tests_per_second: 0.0,
        primes_per_second_total: 0.0,
        primes_per_second_warm: 0.0,
        first_prime: String::new(),
        note: "Path unavailable in this build or platform".to_string(),
    }
}

fn build_summary(
    rows: &[BenchmarkRow],
    biguint_rows: &[BigUintProbablePrimeRow],
    u128_rows: &[U128ProbablePrimeRow],
) -> ReportSummary {
    let fastest_prime = rows
        .iter()
        .filter(|row| row.status == "ok")
        .max_by(|left, right| {
            left.primes_per_second_warm
                .partial_cmp(&right.primes_per_second_warm)
                .unwrap()
        })
        .expect("at least one successful row");
    let largest_decimal_digits = rows
        .iter()
        .map(|row| row.decimal_digits_max)
        .chain(
            biguint_rows
                .iter()
                .filter(|row| row.status == "ok")
                .map(|row| row.decimal_digits_min),
        )
        .chain(
            u128_rows
                .iter()
                .filter(|row| row.status == "ok")
                .map(|row| row.decimal_digits_min),
        )
        .max()
        .unwrap_or(0);
    let total_u64_candidate_value_bytes_avoided_by_metal = rows
        .iter()
        .filter(|row| row.path == "metal_affine_transfer_collapse")
        .map(|row| row.avoided_candidate_value_bytes_u64)
        .sum();
    ReportSummary {
        lane_count: LANE_SPECS.len(),
        benchmark_row_count: rows.len(),
        fastest_prime_path: format!("{} / {}", fastest_prime.role, fastest_prime.path),
        fastest_prime_rate: fastest_prime.primes_per_second_warm,
        largest_decimal_digits,
        total_u64_candidate_value_bytes_avoided_by_metal,
        exact_takeaway:
            "Local benchmarks separate transfer shape from density: Metal affine avoids candidate-value transfer, while CPU small-prime wheels remain a strong baseline at these sizes."
                .to_string(),
    }
}

fn build_observations(
    rows: &[BenchmarkRow],
    metal_batch_rows: &[MetalBatchDispatchRow],
    biguint_rows: &[BigUintProbablePrimeRow],
    u128_rows: &[U128ProbablePrimeRow],
    external_rows: &[ExternalBenchmarkRow],
) -> Vec<String> {
    let mut observations = vec![
        "External CLI tools are measured when available locally; their rows include CLI/process overhead and are problem-shape comparisons, not membrane-construction replacements.".to_string(),
        "The benchmarked Metal computation uses a dedicated `.metal` kernel for the affine residue loop; Rust's `metal` crate is the host/dispatch layer, not the computational kernel.".to_string(),
        "Metal throughput is reported both cold (`total_seconds`) and warm-ish (`gpu_sieve_seconds + cpu_confirm_seconds`) because one-shot pipeline setup is not the same question as generator dispatch throughput.".to_string(),
        "The comparison baselines are ordinary odd scans, ordinary small-prime prefilter scans, random odd same-window candidates, CPU affine wheels, and Metal affine residue sieving.".to_string(),
    ];
    let external_ok = external_rows
        .iter()
        .filter(|row| row.status == "ok")
        .count();
    let external_unavailable = external_rows.len().saturating_sub(external_ok);
    observations.push(format!(
        "Optional external adapters produced {} measured row(s) and {} unavailable/skipped row(s) on this machine.",
        external_ok, external_unavailable
    ));
    if let Some(row) = metal_batch_rows.iter().find(|row| row.status == "ok") {
        observations.push(format!(
            "The repeated-dispatch Metal row scans {} seeds across {} batches with {:.6}s setup and {:.6}s GPU dispatch time.",
            row.scanned_count, row.batch_count, row.setup_seconds, row.gpu_sieve_seconds
        ));
    }
    if let Some(row) = biguint_rows.iter().find(|row| row.status == "ok") {
        let largest_digits = biguint_rows
            .iter()
            .filter(|row| row.status == "ok")
            .map(|row| row.decimal_digits_min)
            .max()
            .unwrap_or(row.decimal_digits_min);
        let total_scanned: u64 = biguint_rows
            .iter()
            .filter(|row| row.status == "ok")
            .map(|row| row.scanned_count)
            .sum();
        let total_probable_primes: u64 = biguint_rows
            .iter()
            .filter(|row| row.status == "ok")
            .map(|row| row.probable_primes_found)
            .sum();
        observations.push(format!(
            "The beyond-u64 BigUint rows scan {} visible candidates up to {} digits and find {} probable-prime witnesses after exact residue filtering.",
            total_scanned, largest_digits, total_probable_primes
        ));
    }
    if let Some(row) = u128_rows.iter().find(|row| row.status == "ok") {
        let largest_digits = u128_rows
            .iter()
            .filter(|row| row.status == "ok")
            .map(|row| row.decimal_digits_min)
            .max()
            .unwrap_or(row.decimal_digits_min);
        let total_scanned: u64 = u128_rows
            .iter()
            .filter(|row| row.status == "ok")
            .map(|row| row.scanned_count)
            .sum();
        let total_probable_primes: u64 = u128_rows
            .iter()
            .filter(|row| row.status == "ok")
            .map(|row| row.probable_primes_found)
            .sum();
        observations.push(format!(
            "The u128 fixed-width rows scan {} visible candidates up to {} digits and find {} probable-prime witnesses before falling back to arbitrary precision.",
            total_scanned, largest_digits, total_probable_primes
        ));
        if let Some(widest) = u128_rows
            .iter()
            .filter(|row| row.status == "ok")
            .max_by_key(|row| row.decimal_digits_min)
        {
            observations.push(format!(
                "At the widest u128 row, the raw witness share is {:.2}% (about {:.1} seeds per probable-prime witness), which is the efficacy metric to keep separate from primality-backend speed.",
                widest.probable_prime_share_of_raw * 100.0,
                widest.seeds_per_probable_prime
            ));
        }
    }
    if let Some(row) = rows
        .iter()
        .filter(|row| row.path == "metal_affine_transfer_collapse")
        .max_by_key(|row| row.scanned_count)
    {
        observations.push(format!(
            "The largest maintained Metal batch in this run scans {} seeds at {} decimal digits while transferring {} metadata bytes and {} bitmask bytes.",
            row.scanned_count, row.decimal_digits_max, row.input_metadata_bytes, row.output_bytes
        ));
    }
    if let Some(row) = rows
        .iter()
        .filter(|row| row.path == "metal_affine_transfer_collapse")
        .max_by_key(|row| row.avoided_candidate_value_bytes_u64)
    {
        observations.push(format!(
            "The strongest transfer-collapse row avoids {} u64 candidate-value bytes for `{}`.",
            row.avoided_candidate_value_bytes_u64, row.role
        ));
    }
    observations
}

fn build_metal_batch_dispatch_rows(options: &Options) -> Vec<MetalBatchDispatchRow> {
    let role = "decimal_19_digit_visible_lane";
    let path = "metal_affine_repeated_dispatch";
    let lane = match build_fast_affine_lane(FastLaneConfig::new(10, 3, 7, 9, (2, 1))) {
        Ok(lane) => lane,
        Err(err) => {
            return vec![unavailable_metal_batch_row(
                role,
                path,
                &format!("lane unavailable: {err}"),
            )];
        }
    };
    let scanned = options.seed_count.min(lane.seed_capacity);
    let moduli = default_metal_affine_moduli(&lane);
    let row_batches = match build_metal_affine_residue_row_batches(
        &lane,
        0,
        scanned,
        options.metal_batch_seed_count,
        &moduli,
    ) {
        Ok(row_batches) => row_batches,
        Err(err) => {
            return vec![unavailable_metal_batch_row(
                role,
                path,
                &format!("batch rows unavailable: {err}"),
            )];
        }
    };
    match sieve_metal_affine_residue_batches(
        0,
        scanned,
        options.metal_batch_seed_count,
        &row_batches,
    ) {
        Ok(run) => vec![MetalBatchDispatchRow {
            role: role.to_string(),
            path: path.to_string(),
            status: "ok".to_string(),
            scanned_count: run.seed_count,
            batch_seed_count: run.batch_seed_count,
            batch_count: run.batch_count,
            survivor_count: run.survivor_seeds.len() as u64,
            setup_seconds: run.setup_seconds,
            buffer_prepare_seconds: run.buffer_prepare_seconds,
            gpu_sieve_seconds: run.gpu_sieve_seconds,
            unpack_seconds: run.unpack_seconds,
            total_seconds: run.total_seconds,
            raw_per_second_dispatch_only: ratio_f64(run.seed_count, run.gpu_sieve_seconds),
            raw_per_second_total: ratio_f64(run.seed_count, run.total_seconds),
            input_metadata_bytes: run.input_metadata_bytes,
            output_bitmask_bytes: run.output_bitmask_bytes,
            avoided_candidate_value_bytes_u64: run.avoided_candidate_value_bytes_u64,
            note: "Repeated dispatch reuses one Metal pipeline setup and excludes CPU primality confirmation".to_string(),
        }],
        Err(err) => vec![unavailable_metal_batch_row(
            role,
            path,
            &format!("unavailable: {err}"),
        )],
    }
}

fn unavailable_metal_batch_row(role: &str, path: &str, status: &str) -> MetalBatchDispatchRow {
    MetalBatchDispatchRow {
        role: role.to_string(),
        path: path.to_string(),
        status: status.to_string(),
        scanned_count: 0,
        batch_seed_count: 0,
        batch_count: 0,
        survivor_count: 0,
        setup_seconds: 0.0,
        buffer_prepare_seconds: 0.0,
        gpu_sieve_seconds: 0.0,
        unpack_seconds: 0.0,
        total_seconds: 0.0,
        raw_per_second_dispatch_only: 0.0,
        raw_per_second_total: 0.0,
        input_metadata_bytes: 0,
        output_bitmask_bytes: 0,
        avoided_candidate_value_bytes_u64: 0,
        note: "Repeated Metal dispatch row unavailable in this build or platform".to_string(),
    }
}

fn build_biguint_probable_prime_rows(options: &Options) -> Vec<BigUintProbablePrimeRow> {
    options
        .biguint_middle_lengths
        .iter()
        .copied()
        .map(|middle_length| build_biguint_probable_prime_row(options, middle_length))
        .collect()
}

fn build_biguint_probable_prime_row(
    options: &Options,
    middle_length: usize,
) -> BigUintProbablePrimeRow {
    let config = FastLaneConfig::new(10, 3, 7, middle_length, (2, 1));
    let lane = match build_biguint_affine_lane(config) {
        Ok(lane) => lane,
        Err(err) => {
            return unavailable_biguint_row(
                &format!("biguint_decimal_m{middle_length}_visible_lane"),
                "biguint_affine_residue_probable_prime",
                &format!("lane unavailable: {err}"),
            );
        }
    };
    let scanned = options.biguint_seed_count.min(lane.seed_capacity);
    let moduli = default_biguint_affine_moduli(lane.config.base);
    let residue_rows = build_biguint_residue_rows(&lane, 0, &moduli);

    let sieve_start = Instant::now();
    let survivor_seeds = (0..scanned)
        .filter(|&seed| residue_rows_allow_local_seed(&residue_rows, seed))
        .collect::<Vec<_>>();
    let residue_sieve_seconds = sieve_start.elapsed().as_secs_f64();

    let confirm_start = Instant::now();
    let mut probable_primes_found = 0u64;
    let mut first_probable_prime = String::new();
    let mut first_template_digits = String::new();
    for &seed in &survivor_seeds {
        let value = biguint_candidate_value(&lane, seed);
        if miller_rabin_test(&value, options.biguint_miller_rabin_rounds) {
            probable_primes_found += 1;
            if first_probable_prime.is_empty() {
                first_probable_prime = value.to_str_radix(10);
                first_template_digits = biguint_template_digits(&lane.config, seed);
            }
        }
    }
    let probable_prime_seconds = confirm_start.elapsed().as_secs_f64();
    let total_seconds = residue_sieve_seconds + probable_prime_seconds;
    let visible_digits = biguint_template_digits(&lane.config, 0).len();
    BigUintProbablePrimeRow {
        role: format!("biguint_decimal_{visible_digits}_digit_visible_lane"),
        path: "biguint_affine_residue_probable_prime".to_string(),
        status: "ok".to_string(),
        base: lane.config.base,
        pair_label: lane.config.pair_label(),
        k_label: lane.config.k_label(),
        middle_length: lane.config.middle_length,
        visible_template_digits: visible_digits,
        decimal_digits_min: lane.shift.to_str_radix(10).len(),
        scanned_count: scanned,
        survivor_count: survivor_seeds.len() as u64,
        probable_prime_tests: survivor_seeds.len() as u64,
        probable_primes_found,
        survivor_share: ratio_count(survivor_seeds.len() as u64, scanned),
        probable_prime_share_of_raw: ratio_count(probable_primes_found, scanned),
        probable_prime_share_of_survivors: ratio_count(
            probable_primes_found,
            survivor_seeds.len() as u64,
        ),
        seeds_per_probable_prime: ratio_count(scanned, probable_primes_found),
        residue_sieve_seconds,
        probable_prime_seconds,
        total_seconds,
        raw_per_second: ratio_f64(scanned, total_seconds),
        tests_per_second: ratio_f64(survivor_seeds.len() as u64, probable_prime_seconds),
        probable_primes_per_second: ratio_f64(probable_primes_found, total_seconds),
        first_probable_prime,
        first_template_digits,
        note: format!(
            "Beyond-u64 row using {} Miller-Rabin rounds after the same exact residue funnel",
            options.biguint_miller_rabin_rounds
        ),
    }
}

fn unavailable_biguint_row(role: &str, path: &str, status: &str) -> BigUintProbablePrimeRow {
    BigUintProbablePrimeRow {
        role: role.to_string(),
        path: path.to_string(),
        status: status.to_string(),
        base: 0,
        pair_label: String::new(),
        k_label: String::new(),
        middle_length: 0,
        visible_template_digits: 0,
        decimal_digits_min: 0,
        scanned_count: 0,
        survivor_count: 0,
        probable_prime_tests: 0,
        probable_primes_found: 0,
        survivor_share: 0.0,
        probable_prime_share_of_raw: 0.0,
        probable_prime_share_of_survivors: 0.0,
        seeds_per_probable_prime: 0.0,
        residue_sieve_seconds: 0.0,
        probable_prime_seconds: 0.0,
        total_seconds: 0.0,
        raw_per_second: 0.0,
        tests_per_second: 0.0,
        probable_primes_per_second: 0.0,
        first_probable_prime: String::new(),
        first_template_digits: String::new(),
        note: "BigUint probable-prime row unavailable".to_string(),
    }
}

fn build_u128_probable_prime_rows(options: &Options) -> Vec<U128ProbablePrimeRow> {
    options
        .u128_middle_lengths
        .iter()
        .copied()
        .map(|middle_length| build_u128_probable_prime_row(options, middle_length))
        .collect()
}

fn build_u128_probable_prime_row(options: &Options, middle_length: usize) -> U128ProbablePrimeRow {
    let config = FastLaneConfig::new(10, 3, 7, middle_length, (2, 1));
    let lane = match build_u128_affine_lane(config) {
        Ok(lane) => lane,
        Err(err) => {
            return unavailable_u128_row(
                &format!("u128_decimal_m{middle_length}_visible_lane"),
                "u128_affine_residue_probable_prime",
                &format!("lane unavailable: {err}"),
            );
        }
    };
    let scanned = options.u128_seed_count.min(lane.seed_capacity);
    let moduli = default_biguint_affine_moduli(lane.config.base);
    let residue_rows = build_u128_residue_rows(&lane, 0, &moduli);

    let sieve_start = Instant::now();
    let survivor_seeds = (0..scanned)
        .filter(|&seed| residue_rows_allow_local_seed(&residue_rows, seed))
        .collect::<Vec<_>>();
    let residue_sieve_seconds = sieve_start.elapsed().as_secs_f64();

    let confirm_start = Instant::now();
    let mut probable_primes_found = 0u64;
    let mut first_probable_prime = String::new();
    let mut first_template_digits = String::new();
    for &seed in &survivor_seeds {
        let Some(value) = u128_candidate_value(&lane, seed) else {
            continue;
        };
        if is_probable_prime_u128(value) {
            probable_primes_found += 1;
            if first_probable_prime.is_empty() {
                first_probable_prime = value.to_string();
                first_template_digits = biguint_template_digits(&lane.config, seed);
            }
        }
    }
    let probable_prime_seconds = confirm_start.elapsed().as_secs_f64();
    let total_seconds = residue_sieve_seconds + probable_prime_seconds;
    let visible_digits = biguint_template_digits(&lane.config, 0).len();
    U128ProbablePrimeRow {
        role: format!("u128_decimal_{visible_digits}_digit_visible_lane"),
        path: "u128_affine_residue_probable_prime".to_string(),
        status: "ok".to_string(),
        base: lane.config.base,
        pair_label: lane.config.pair_label(),
        k_label: lane.config.k_label(),
        middle_length: lane.config.middle_length,
        visible_template_digits: visible_digits,
        decimal_digits_min: lane.shift.to_string().len(),
        scanned_count: scanned,
        survivor_count: survivor_seeds.len() as u64,
        probable_prime_tests: survivor_seeds.len() as u64,
        probable_primes_found,
        survivor_share: ratio_count(survivor_seeds.len() as u64, scanned),
        probable_prime_share_of_raw: ratio_count(probable_primes_found, scanned),
        probable_prime_share_of_survivors: ratio_count(
            probable_primes_found,
            survivor_seeds.len() as u64,
        ),
        seeds_per_probable_prime: ratio_count(scanned, probable_primes_found),
        residue_sieve_seconds,
        probable_prime_seconds,
        total_seconds,
        raw_per_second: ratio_f64(scanned, total_seconds),
        tests_per_second: ratio_f64(survivor_seeds.len() as u64, probable_prime_seconds),
        probable_primes_per_second: ratio_f64(probable_primes_found, total_seconds),
        first_probable_prime,
        first_template_digits,
        note: format!(
            "Fixed-width u128 Miller-Rabin with {} bases after the same exact residue funnel",
            U128_MILLER_RABIN_BASES.len()
        ),
    }
}

fn unavailable_u128_row(role: &str, path: &str, status: &str) -> U128ProbablePrimeRow {
    U128ProbablePrimeRow {
        role: role.to_string(),
        path: path.to_string(),
        status: status.to_string(),
        base: 0,
        pair_label: String::new(),
        k_label: String::new(),
        middle_length: 0,
        visible_template_digits: 0,
        decimal_digits_min: 0,
        scanned_count: 0,
        survivor_count: 0,
        probable_prime_tests: 0,
        probable_primes_found: 0,
        survivor_share: 0.0,
        probable_prime_share_of_raw: 0.0,
        probable_prime_share_of_survivors: 0.0,
        seeds_per_probable_prime: 0.0,
        residue_sieve_seconds: 0.0,
        probable_prime_seconds: 0.0,
        total_seconds: 0.0,
        raw_per_second: 0.0,
        tests_per_second: 0.0,
        probable_primes_per_second: 0.0,
        first_probable_prime: String::new(),
        first_template_digits: String::new(),
        note: "u128 probable-prime row unavailable".to_string(),
    }
}

fn build_external_benchmark_rows(options: &Options) -> Vec<ExternalBenchmarkRow> {
    let anchor_lane = build_fast_affine_lane(FastLaneConfig::new(10, 3, 7, 9, (2, 1)))
        .expect("anchor lane should build");
    vec![
        benchmark_primesieve_interval(&anchor_lane),
        benchmark_openssl_prime_generation(options),
    ]
}

fn benchmark_primesieve_interval(anchor_lane: &FastAffineLane) -> ExternalBenchmarkRow {
    let start_value = anchor_lane.shift;
    let distance = DEFAULT_EXTERNAL_INTERVAL_DISTANCE;
    let command_text = format!("primesieve {} --dist={} --time", start_value, distance);
    let start = Instant::now();
    let output = Command::new("primesieve")
        .arg(start_value.to_string())
        .arg(format!("--dist={distance}"))
        .arg("--time")
        .output();
    let elapsed = start.elapsed().as_secs_f64();

    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let values_found = parse_primesieve_count(&stdout).unwrap_or(0);
            ExternalBenchmarkRow {
                family: "ordinary prime enumeration".to_string(),
                path: "primesieve_interval_count_cli".to_string(),
                status: if values_found > 0 {
                    "ok".to_string()
                } else {
                    "ok_unparsed_count".to_string()
                },
                command: command_text,
                problem_shape:
                    "count all primes in an ordinary interval near the 19-digit affine lane"
                        .to_string(),
                requested_work: distance,
                values_found,
                elapsed_seconds: elapsed,
                work_per_second: ratio_f64(distance, elapsed),
                values_per_second: ratio_f64(values_found, elapsed),
                first_value: String::new(),
                note: "External CLI baseline; not a structured membrane witness generator"
                    .to_string(),
            }
        }
        Ok(output) => ExternalBenchmarkRow {
            family: "ordinary prime enumeration".to_string(),
            path: "primesieve_interval_count_cli".to_string(),
            status: format!(
                "unavailable: exit {}",
                output
                    .status
                    .code()
                    .map(|code| code.to_string())
                    .unwrap_or_else(|| "signal".to_string())
            ),
            command: command_text,
            problem_shape: "count all primes in an ordinary interval near the 19-digit affine lane"
                .to_string(),
            requested_work: distance,
            values_found: 0,
            elapsed_seconds: elapsed,
            work_per_second: 0.0,
            values_per_second: 0.0,
            first_value: String::new(),
            note: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        },
        Err(err) => ExternalBenchmarkRow {
            family: "ordinary prime enumeration".to_string(),
            path: "primesieve_interval_count_cli".to_string(),
            status: format!("unavailable: {err}"),
            command: command_text,
            problem_shape: "count all primes in an ordinary interval near the 19-digit affine lane"
                .to_string(),
            requested_work: distance,
            values_found: 0,
            elapsed_seconds: 0.0,
            work_per_second: 0.0,
            values_per_second: 0.0,
            first_value: String::new(),
            note: "Install primesieve to enable this measured external row".to_string(),
        },
    }
}

fn benchmark_openssl_prime_generation(options: &Options) -> ExternalBenchmarkRow {
    let iterations = options
        .max_primes
        .clamp(1, MAX_EXTERNAL_OPENSSL_GENERATIONS);
    let command_text = format!("openssl prime -generate -bits 64 repeated {iterations}x");
    let start = Instant::now();
    let mut first_value = String::new();
    let mut values_found = 0u64;
    for _ in 0..iterations {
        let output = match Command::new("openssl")
            .args(["prime", "-generate", "-bits", "64"])
            .output()
        {
            Ok(output) => output,
            Err(err) => {
                return ExternalBenchmarkRow {
                    family: "cryptographic random prime generation".to_string(),
                    path: "openssl_generate_prime_cli".to_string(),
                    status: format!("unavailable: {err}"),
                    command: command_text,
                    problem_shape: "generate independent random 64-bit probable primes".to_string(),
                    requested_work: iterations as u64,
                    values_found,
                    elapsed_seconds: start.elapsed().as_secs_f64(),
                    work_per_second: 0.0,
                    values_per_second: 0.0,
                    first_value,
                    note: "Install OpenSSL to enable this measured external row".to_string(),
                };
            }
        };
        if !output.status.success() {
            return ExternalBenchmarkRow {
                family: "cryptographic random prime generation".to_string(),
                path: "openssl_generate_prime_cli".to_string(),
                status: format!(
                    "unavailable: exit {}",
                    output
                        .status
                        .code()
                        .map(|code| code.to_string())
                        .unwrap_or_else(|| "signal".to_string())
                ),
                command: command_text,
                problem_shape: "generate independent random 64-bit probable primes".to_string(),
                requested_work: iterations as u64,
                values_found,
                elapsed_seconds: start.elapsed().as_secs_f64(),
                work_per_second: 0.0,
                values_per_second: 0.0,
                first_value,
                note: String::from_utf8_lossy(&output.stderr).trim().to_string(),
            };
        }
        let candidate = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if first_value.is_empty() {
            first_value = candidate;
        }
        values_found += 1;
    }
    let elapsed = start.elapsed().as_secs_f64();
    ExternalBenchmarkRow {
        family: "cryptographic random prime generation".to_string(),
        path: "openssl_generate_prime_cli".to_string(),
        status: "ok".to_string(),
        command: command_text,
        problem_shape: "generate independent random 64-bit probable primes".to_string(),
        requested_work: iterations as u64,
        values_found,
        elapsed_seconds: elapsed,
        work_per_second: ratio_f64(iterations as u64, elapsed),
        values_per_second: ratio_f64(values_found, elapsed),
        first_value,
        note:
            "External CLI baseline with process overhead included; not a visible template generator"
                .to_string(),
    }
}

fn parse_primesieve_count(stdout: &str) -> Option<u64> {
    for line in stdout.lines() {
        let lower = line.to_ascii_lowercase();
        if lower.contains("prime") || lower.contains("pi(") {
            if let Some(value) = parse_last_u64(line) {
                return Some(value);
            }
        }
    }
    parse_last_u64(stdout)
}

fn parse_last_u64(text: &str) -> Option<u64> {
    let mut last = None;
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_ascii_digit() {
            current.push(ch);
        } else if !current.is_empty() {
            last = current.parse::<u64>().ok();
            current.clear();
        }
    }
    if !current.is_empty() {
        last = current.parse::<u64>().ok();
    }
    last
}

#[derive(Debug, Clone)]
struct BigUintAffineLane {
    config: FastLaneConfig,
    shift: BigUint,
    gradient: BigUint,
    seed_capacity: u64,
}

#[derive(Debug, Clone)]
struct U128AffineLane {
    config: FastLaneConfig,
    shift: u128,
    gradient: u128,
    seed_capacity: u64,
}

fn build_biguint_affine_lane(config: FastLaneConfig) -> Result<BigUintAffineLane, String> {
    if config.base < 2 {
        return Err(format!("base must be at least 2, got {}", config.base));
    }
    if config.outer >= config.base || config.inner >= config.base {
        return Err("boundary digit outside configured base".to_string());
    }

    let prefix_digits = {
        let mut digits = Vec::with_capacity((2 + config.k_outer + config.k_inner) as usize);
        digits.push(config.outer);
        digits.extend(std::iter::repeat_n(0, config.k_outer as usize));
        digits.push(config.inner);
        digits.extend(std::iter::repeat_n(0, config.k_inner as usize));
        digits
    };
    let suffix_digits = {
        let mut digits = Vec::with_capacity((2 + config.k_outer + config.k_inner) as usize);
        digits.extend(std::iter::repeat_n(0, config.k_inner as usize));
        digits.push(config.inner);
        digits.extend(std::iter::repeat_n(0, config.k_outer as usize));
        digits.push(config.outer);
        digits
    };
    let suffix_len = suffix_digits.len() as u32;
    let seed_capacity =
        checked_pow_u64_local(config.base, config.middle_length).unwrap_or(u64::MAX);
    let base_big = BigUint::from(config.base);
    let gradient = base_big.pow(suffix_len);
    let prefix_shift = base_big.pow(config.middle_length as u32 + suffix_len);
    let prefix_value = digits_to_biguint_local(config.base, &prefix_digits);
    let suffix_value = digits_to_biguint_local(config.base, &suffix_digits);
    let shift = prefix_value * prefix_shift + suffix_value;

    Ok(BigUintAffineLane {
        config,
        shift,
        gradient,
        seed_capacity,
    })
}

fn build_u128_affine_lane(config: FastLaneConfig) -> Result<U128AffineLane, String> {
    if config.base < 2 {
        return Err(format!("base must be at least 2, got {}", config.base));
    }
    if config.outer >= config.base || config.inner >= config.base {
        return Err("boundary digit outside configured base".to_string());
    }

    let prefix_digits = {
        let mut digits = Vec::with_capacity((2 + config.k_outer + config.k_inner) as usize);
        digits.push(config.outer);
        digits.extend(std::iter::repeat_n(0, config.k_outer as usize));
        digits.push(config.inner);
        digits.extend(std::iter::repeat_n(0, config.k_inner as usize));
        digits
    };
    let suffix_digits = {
        let mut digits = Vec::with_capacity((2 + config.k_outer + config.k_inner) as usize);
        digits.extend(std::iter::repeat_n(0, config.k_inner as usize));
        digits.push(config.inner);
        digits.extend(std::iter::repeat_n(0, config.k_outer as usize));
        digits.push(config.outer);
        digits
    };
    let suffix_len = suffix_digits.len() as u32;
    let seed_capacity =
        checked_pow_u64_local(config.base, config.middle_length).unwrap_or(u64::MAX);
    let base = config.base as u128;
    let gradient = checked_pow_u128_local(base, suffix_len as usize)
        .ok_or_else(|| "suffix gradient does not fit u128 for this report".to_string())?;
    let prefix_shift = checked_pow_u128_local(base, config.middle_length + suffix_len as usize)
        .ok_or_else(|| "prefix shift does not fit u128 for this report".to_string())?;
    let prefix_value = digits_to_u128_local(config.base, &prefix_digits)?;
    let suffix_value = digits_to_u128_local(config.base, &suffix_digits)?;
    let shift = prefix_value
        .checked_mul(prefix_shift)
        .and_then(|value| value.checked_add(suffix_value))
        .ok_or_else(|| "lane shift does not fit u128 for this report".to_string())?;

    Ok(U128AffineLane {
        config,
        shift,
        gradient,
        seed_capacity,
    })
}

fn biguint_candidate_value(lane: &BigUintAffineLane, seed: u64) -> BigUint {
    &lane.shift + &lane.gradient * BigUint::from(seed)
}

fn u128_candidate_value(lane: &U128AffineLane, seed: u64) -> Option<u128> {
    if seed >= lane.seed_capacity {
        return None;
    }
    lane.shift
        .checked_add(lane.gradient.checked_mul(seed as u128)?)
}

fn build_biguint_residue_rows(
    lane: &BigUintAffineLane,
    seed_offset: u64,
    moduli: &[u32],
) -> Vec<MetalAffineResidueRow> {
    moduli
        .iter()
        .copied()
        .map(|modulus| {
            let p = modulus as u64;
            let shift_mod = biguint_mod_u32(&lane.shift, modulus) as u64;
            let gradient_mod = biguint_mod_u32(&lane.gradient, modulus) as u64;
            MetalAffineResidueRow {
                a: ((shift_mod + (gradient_mod * (seed_offset % p)) % p) % p) as u32,
                g: gradient_mod as u32,
                p: modulus,
                pad: 0,
            }
        })
        .collect()
}

fn build_u128_residue_rows(
    lane: &U128AffineLane,
    seed_offset: u64,
    moduli: &[u32],
) -> Vec<MetalAffineResidueRow> {
    moduli
        .iter()
        .copied()
        .map(|modulus| {
            let p = modulus as u64;
            let shift_mod = (lane.shift % modulus as u128) as u64;
            let gradient_mod = (lane.gradient % modulus as u128) as u64;
            MetalAffineResidueRow {
                a: ((shift_mod + (gradient_mod * (seed_offset % p)) % p) % p) as u32,
                g: gradient_mod as u32,
                p: modulus,
                pad: 0,
            }
        })
        .collect()
}

fn default_biguint_affine_moduli(base: u32) -> Vec<u32> {
    DEFAULT_PREFILTER_PRIMES
        .iter()
        .copied()
        .filter(|&modulus| gcd_u32_local(base, modulus) == 1)
        .collect()
}

fn biguint_template_digits(config: &FastLaneConfig, seed: u64) -> String {
    let mut digits = String::new();
    digits.push_str(&digit_symbol(config.outer));
    digits.extend(std::iter::repeat_n('0', config.k_outer as usize));
    digits.push_str(&digit_symbol(config.inner));
    digits.extend(std::iter::repeat_n('0', config.k_inner as usize));
    digits.push_str(&biguint_middle_digits(config, seed));
    digits.extend(std::iter::repeat_n('0', config.k_inner as usize));
    digits.push_str(&digit_symbol(config.inner));
    digits.extend(std::iter::repeat_n('0', config.k_outer as usize));
    digits.push_str(&digit_symbol(config.outer));
    digits
}

fn biguint_middle_digits(config: &FastLaneConfig, mut seed: u64) -> String {
    let mut digits = vec!['0'; config.middle_length];
    for digit in digits.iter_mut().rev() {
        let value = seed % config.base as u64;
        *digit = digit_char_local(value as u32);
        seed /= config.base as u64;
    }
    digits.into_iter().collect()
}

fn digits_to_biguint_local(base: u32, digits: &[u32]) -> BigUint {
    let base_big = BigUint::from(base);
    let mut value = BigUint::from(0u32);
    for &digit in digits {
        value *= &base_big;
        value += digit;
    }
    value
}

fn digits_to_u128_local(base: u32, digits: &[u32]) -> Result<u128, String> {
    let mut value = 0u128;
    for &digit in digits {
        if digit >= base {
            return Err(format!("digit {digit} outside base {base}"));
        }
        value = value
            .checked_mul(base as u128)
            .and_then(|value| value.checked_add(digit as u128))
            .ok_or_else(|| "digit accumulation does not fit u128".to_string())?;
    }
    Ok(value)
}

fn biguint_mod_u32(value: &BigUint, modulus: u32) -> u32 {
    (value % BigUint::from(modulus))
        .to_u32()
        .expect("remainder should fit u32")
}

fn checked_pow_u128_local(base: u128, exp: usize) -> Option<u128> {
    let mut value = 1u128;
    for _ in 0..exp {
        value = value.checked_mul(base)?;
    }
    Some(value)
}

fn checked_pow_u64_local(base: u32, exp: usize) -> Option<u64> {
    let mut value = 1u64;
    for _ in 0..exp {
        value = value.checked_mul(base as u64)?;
    }
    Some(value)
}

fn digit_char_local(digit: u32) -> char {
    if digit < 10 {
        char::from_digit(digit, 10).expect("decimal digit")
    } else {
        char::from_u32('A' as u32 + digit - 10).expect("uppercase digit")
    }
}

fn is_probable_prime_u128(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    for &prime in U128_MILLER_RABIN_BASES {
        if n == prime {
            return true;
        }
        if n.is_multiple_of(prime) {
            return false;
        }
    }

    let mut d = n - 1;
    let mut s = 0u32;
    while d.is_multiple_of(2) {
        d /= 2;
        s += 1;
    }

    'bases: for &base in U128_MILLER_RABIN_BASES {
        if base >= n - 1 {
            continue;
        }
        let mut x = mod_pow_u128(base, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mod_mul_u128(x, x, n);
            if x == n - 1 {
                continue 'bases;
            }
        }
        return false;
    }
    true
}

fn mod_pow_u128(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut acc = 1u128;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = mod_mul_u128(acc, base, modulus);
        }
        exp >>= 1;
        if exp > 0 {
            base = mod_mul_u128(base, base, modulus);
        }
    }
    acc
}

fn mod_mul_u128(mut left: u128, mut right: u128, modulus: u128) -> u128 {
    if let Some(product) = left.checked_mul(right) {
        return product % modulus;
    }

    left %= modulus;
    let mut acc = 0u128;
    while right > 0 {
        if right & 1 == 1 {
            acc = add_mod_u128(acc, left, modulus);
        }
        right >>= 1;
        if right > 0 {
            left = add_mod_u128(left, left, modulus);
        }
    }
    acc
}

fn add_mod_u128(left: u128, right: u128, modulus: u128) -> u128 {
    if left >= modulus - right {
        left - (modulus - right)
    } else {
        left + right
    }
}

fn gcd_u32_local(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let tmp = left % right;
        left = right;
        right = tmp;
    }
    left
}

fn build_external_comparison_rows() -> Vec<ExternalComparisonRow> {
    vec![
        ExternalComparisonRow {
            family: "ordinary prime enumeration".to_string(),
            representative_source: "primesieve".to_string(),
            established_shape: "cache-aware segmented sieve of Eratosthenes with wheel factorization, bucket sieve, and multithreading up to 2^64".to_string(),
            comparison_to_affine_transfer_collapse: "The affine path is not a general interval enumerator; it searches structured lanes and should compare against primesieve only for range-enumeration baselines, not for template witness semantics.".to_string(),
            citation_url: "https://github.com/kimwalisch/primesieve".to_string(),
        },
        ExternalComparisonRow {
            family: "arbitrary-precision next-prime/probable-prime".to_string(),
            representative_source: "GNU MP".to_string(),
            established_shape: "trial division, Baillie-PSW probable-prime testing, and Miller-Rabin rounds for arbitrary precision integers".to_string(),
            comparison_to_affine_transfer_collapse: "GMP is the natural future comparison for BigUint survivor confirmation; our maintained benchmark is currently deterministic u64 and measures candidate funnel efficiency first.".to_string(),
            citation_url: "https://gmplib.org/manual/Number-Theoretic-Functions".to_string(),
        },
        ExternalComparisonRow {
            family: "cryptographic random prime generation".to_string(),
            representative_source: "OpenSSL BN_generate_prime_ex".to_string(),
            established_shape: "pseudo-random bit-length prime generation with optional congruence constraints, small-prime trial division, and Miller-Rabin error bounds".to_string(),
            comparison_to_affine_transfer_collapse: "OpenSSL is the right foil for random/probable prime generation, but not for visible symmetric-template constructions or density claims.".to_string(),
            citation_url: "https://docs.openssl.org/master/man3/BN_generate_prime/".to_string(),
        },
        ExternalComparisonRow {
            family: "machine-word deterministic primality".to_string(),
            representative_source: "Forisek-Jancina / Jaeschke-Sinclair Miller-Rabin bases".to_string(),
            established_shape: "deterministic Miller-Rabin-style classification for bounded 32-bit and 64-bit integers using fixed or hashed bases".to_string(),
            comparison_to_affine_transfer_collapse: "This is our current confirmation regime: the question is how much work the affine residue funnel can remove before deterministic u64 primality testing.".to_string(),
            citation_url: "https://ceur-ws.org/Vol-1326/020-Forisek.pdf".to_string(),
        },
        ExternalComparisonRow {
            family: "GPU segmented sieving".to_string(),
            representative_source: "CUDASieve".to_string(),
            established_shape: "Nvidia CUDA segmented sieve of Eratosthenes for counting and generating all primes in ranges".to_string(),
            comparison_to_affine_transfer_collapse: "CUDASieve is range-wide GPU sieving; our Metal kernel is lane-wide affine residue sieving with zero candidate-value transfer.".to_string(),
            citation_url: "https://github.com/curtisseizert/CUDASieve".to_string(),
        },
        ExternalComparisonRow {
            family: "special-form huge prime searches".to_string(),
            representative_source: "GIMPS / Prime95 / PRPLL-NTT".to_string(),
            established_shape: "trial factoring, P-1/ECM where useful, PRP tests, and Lucas-Lehmer verification for Mersenne-form candidates".to_string(),
            comparison_to_affine_transfer_collapse: "These systems optimize very large special forms over hours or days; they are conceptually relevant to GPU primality pipelines but not numerically comparable to 64-bit lane throughput.".to_string(),
            citation_url: "https://www.mersenne.org/various/works.php".to_string(),
        },
    ]
}

fn render_report(
    settings: &ReportSettings,
    summary: &ReportSummary,
    tables: ReportTables<'_>,
) -> String {
    let mut lines = Vec::new();
    lines.push("# Metal Affine Benchmark Report".to_string());
    lines.push(String::new());
    lines.push("## Settings".to_string());
    lines.push(format!("- output dir: `{}`", settings.out_dir));
    lines.push(format!(
        "- requested seed count: `{}`",
        settings.requested_seed_count
    ));
    lines.push(format!(
        "- max witnesses per path: `{}`",
        settings.max_primes
    ));
    lines.push(format!(
        "- wheel period cap: `{}`",
        settings.wheel_period_cap
    ));
    lines.push(format!(
        "- repeated Metal batch seed count: `{}`",
        settings.metal_batch_seed_count
    ));
    lines.push(format!(
        "- BigUint seed count: `{}`",
        settings.biguint_seed_count
    ));
    lines.push(format!(
        "- BigUint Miller-Rabin rounds: `{}`",
        settings.biguint_miller_rabin_rounds
    ));
    lines.push(format!(
        "- BigUint middle lengths: `{}`",
        settings
            .biguint_middle_lengths
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
    ));
    lines.push(format!("- u128 seed count: `{}`", settings.u128_seed_count));
    lines.push(format!(
        "- u128 middle lengths: `{}`",
        settings
            .u128_middle_lengths
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
    ));
    lines.push(format!(
        "- u128 Miller-Rabin bases: `{}`",
        settings
            .u128_miller_rabin_bases
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
    ));
    lines.push(format!(
        "- Metal host surface: `{}`",
        settings.metal_host_surface
    ));
    lines.push(format!(
        "- Metal kernel surface: `{}`",
        settings.metal_kernel_surface
    ));
    lines.push(format!(
        "- external comparison: `{}`",
        settings.external_comparison_status
    ));
    lines.push(String::new());
    lines.push("## Headline".to_string());
    lines.push(format!("- {}", summary.exact_takeaway));
    lines.push(format!(
        "- largest decimal width scanned: `{}` digits",
        summary.largest_decimal_digits
    ));
    lines.push(format!(
        "- u64 candidate-value bytes avoided by Metal rows: `{}`",
        summary.total_u64_candidate_value_bytes_avoided_by_metal
    ));
    lines.push(format!(
        "- fastest warm prime path: `{}` at {:.2} primes/s",
        summary.fastest_prime_path, summary.fastest_prime_rate
    ));
    lines.push(String::new());
    lines.push("## Benchmark Rows".to_string());
    lines.push("| Role | Path | Digits | Scanned | Survivors | Primes | Warm raw/s | Warm primes/s | Metadata In | Output |".to_string());
    lines.push("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|".to_string());
    for row in tables.benchmark_rows {
        lines.push(format!(
            "| `{}` | `{}` | {} | {} | {} | {} | {:.0} | {:.2} | {} | {} |",
            row.role,
            row.path,
            row.decimal_digits_max,
            row.scanned_count,
            row.survivor_count,
            row.primes_found,
            row.raw_per_second_warm,
            row.primes_per_second_warm,
            row.input_metadata_bytes,
            row.output_bytes
        ));
    }
    lines.push(String::new());
    lines.push("## Repeated Metal Dispatch Rows".to_string());
    lines.push("| Role | Path | Status | Scanned | Batch | Batches | Survivors | Setup s | GPU s | Total s | Dispatch raw/s | Total raw/s | Output |".to_string());
    lines.push("|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|".to_string());
    for row in tables.metal_batch_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | {} | {} | {} | {} | {:.6} | {:.6} | {:.6} | {:.0} | {:.0} | {} |",
            row.role,
            row.path,
            row.status,
            row.scanned_count,
            row.batch_seed_count,
            row.batch_count,
            row.survivor_count,
            row.setup_seconds,
            row.gpu_sieve_seconds,
            row.total_seconds,
            row.raw_per_second_dispatch_only,
            row.raw_per_second_total,
            row.output_bitmask_bytes
        ));
    }
    lines.push(String::new());
    lines.push("## BigUint Probable-Prime Rows".to_string());
    lines.push("| Role | Path | Status | Digits | Scanned | Survivors | Probable Primes | Survivor % | Raw Hit % | Survivor Hit % | Seeds/Witness | Probable primes/s | First Template |".to_string());
    lines.push("|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|".to_string());
    for row in tables.biguint_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | {} | {} | {} | {} | {:.2} | {:.2} | {:.2} | {:.1} | {:.2} | `{}` |",
            row.role,
            row.path,
            row.status,
            row.decimal_digits_min,
            row.scanned_count,
            row.survivor_count,
            row.probable_primes_found,
            row.survivor_share * 100.0,
            row.probable_prime_share_of_raw * 100.0,
            row.probable_prime_share_of_survivors * 100.0,
            row.seeds_per_probable_prime,
            row.probable_primes_per_second,
            row.first_template_digits
        ));
    }
    lines.push(String::new());
    lines.push("## u128 Probable-Prime Rows".to_string());
    lines.push("| Role | Path | Status | Digits | Scanned | Survivors | Probable Primes | Survivor % | Raw Hit % | Survivor Hit % | Seeds/Witness | Probable primes/s | First Template |".to_string());
    lines.push("|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|".to_string());
    for row in tables.u128_rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | {} | {} | {} | {} | {:.2} | {:.2} | {:.2} | {:.1} | {:.2} | `{}` |",
            row.role,
            row.path,
            row.status,
            row.decimal_digits_min,
            row.scanned_count,
            row.survivor_count,
            row.probable_primes_found,
            row.survivor_share * 100.0,
            row.probable_prime_share_of_raw * 100.0,
            row.probable_prime_share_of_survivors * 100.0,
            row.seeds_per_probable_prime,
            row.probable_primes_per_second,
            row.first_template_digits
        ));
    }
    lines.push(String::new());
    lines.push("## External Benchmark Rows".to_string());
    lines.push(
        "| Family | Path | Status | Work | Values | Work/s | Values/s | First Value | Note |"
            .to_string(),
    );
    lines.push("|---|---|---|---:|---:|---:|---:|---|---|".to_string());
    for row in tables.external_benchmark_rows {
        lines.push(format!(
            "| {} | `{}` | `{}` | {} | {} | {:.2} | {:.2} | `{}` | {} |",
            row.family,
            row.path,
            row.status,
            row.requested_work,
            row.values_found,
            row.work_per_second,
            row.values_per_second,
            row.first_value,
            row.note
        ));
    }
    lines.push(String::new());
    lines.push("## External Comparison Frame".to_string());
    lines.push("| Family | Representative | Established Shape | Comparison Boundary |".to_string());
    lines.push("|---|---|---|---|".to_string());
    for row in tables.external_rows {
        lines.push(format!(
            "| {} | [{}]({}) | {} | {} |",
            row.family,
            row.representative_source,
            row.citation_url,
            row.established_shape,
            row.comparison_to_affine_transfer_collapse
        ));
    }
    lines.push(String::new());
    lines.push("## Observations".to_string());
    for observation in tables.observations {
        lines.push(format!("- {}", observation));
    }
    lines.push(String::new());
    lines.push("## Read This Carefully".to_string());
    lines.push("- Fast candidate production is not the same as a new density theorem.".to_string());
    lines.push("- The benchmarked GPU math is in a dedicated `.metal` kernel; Rust hosts the pipeline and buffers.".to_string());
    lines.push("- The Metal path is most interesting as a transfer architecture: lane metadata replaces a full candidate buffer.".to_string());
    lines.push("- Ordinary CPU wheels are strong at these medium sizes; the next question is scale, batching, and larger probable-prime regimes.".to_string());
    lines.push(String::new());
    lines.push("## Artifacts".to_string());
    lines.push("- `benchmark_rows.csv`: all benchmark rows and throughput metrics.".to_string());
    lines.push("- `prime_witness_rows.csv`: first witnesses for each path.".to_string());
    lines.push(
        "- `metal_batch_dispatch_rows.csv`: repeated Metal dispatch rows with setup separated."
            .to_string(),
    );
    lines.push(
        "- `biguint_probable_prime_rows.csv`: beyond-u64 residue funnel plus Miller-Rabin confirmation rows."
            .to_string(),
    );
    lines.push(
        "- `u128_probable_prime_rows.csv`: fixed-width u128 affine residue funnel plus Miller-Rabin confirmation rows."
            .to_string(),
    );
    lines.push(
        "- `external_benchmark_rows.csv`: optional local CLI measurements when tools are installed."
            .to_string(),
    );
    lines.push(
        "- `summary.json`: includes the source-grounded external comparison rows.".to_string(),
    );
    lines.push("- `prime_throughput_by_path.png`: warm prime-throughput bars.".to_string());
    lines.push(
        "- `candidate_transfer_bytes.png`: candidate-buffer/input/output byte comparison."
            .to_string(),
    );
    lines.join("\n")
}

fn render_prime_throughput_chart(rows: &[BenchmarkRow], path: &Path) {
    let rows = rows
        .iter()
        .filter(|row| row.status == "ok")
        .collect::<Vec<_>>();
    let height = (160 + rows.len() as u32 * 50).max(900);
    let root = BitMapBackend::new(path, (1600, height)).into_drawing_area();
    root.fill(&RGBColor(250, 249, 246)).unwrap();
    root.draw(&Text::new(
        "Warm prime throughput by local path",
        (50, 60),
        ("sans-serif", 34).into_font().color(&RGBColor(35, 42, 50)),
    ))
    .unwrap();
    let max_value = rows
        .iter()
        .map(|row| row.primes_per_second_warm)
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let mut y = 120;
    for row in rows {
        let width = ((row.primes_per_second_warm / max_value) * 860.0).max(2.0) as i32;
        let label = format!("{} / {}", short_role(&row.role), row.path);
        root.draw(&Text::new(
            label,
            (50, y + 22),
            ("sans-serif", 18).into_font().color(&RGBColor(35, 42, 50)),
        ))
        .unwrap();
        root.draw(&Rectangle::new(
            [(520, y), (520 + width, y + 34)],
            ShapeStyle::from(&path_color(&row.path)).filled(),
        ))
        .unwrap();
        root.draw(&Text::new(
            format!("{:.1} primes/s", row.primes_per_second_warm),
            (535 + width, y + 23),
            ("sans-serif", 16).into_font().color(&RGBColor(82, 88, 96)),
        ))
        .unwrap();
        y += 50;
    }
}

fn render_transfer_chart(rows: &[BenchmarkRow], path: &Path) {
    let rows = rows
        .iter()
        .filter(|row| row.path == "metal_affine_transfer_collapse")
        .collect::<Vec<_>>();
    let root = BitMapBackend::new(path, (1300, 760)).into_drawing_area();
    root.fill(&RGBColor(250, 249, 246)).unwrap();
    root.draw(&Text::new(
        "Metal affine transfer bytes",
        (50, 60),
        ("sans-serif", 34).into_font().color(&RGBColor(35, 42, 50)),
    ))
    .unwrap();
    let max_value = rows
        .iter()
        .map(|row| row.avoided_candidate_value_bytes_u64.max(row.output_bytes))
        .max()
        .unwrap_or(1) as f64;
    let mut y = 145;
    for row in rows {
        root.draw(&Text::new(
            short_role(&row.role),
            (50, y + 22),
            ("sans-serif", 20).into_font().color(&RGBColor(35, 42, 50)),
        ))
        .unwrap();
        draw_byte_bar(
            &root,
            y,
            "avoided u64 candidate buffer",
            row.avoided_candidate_value_bytes_u64,
            max_value,
            RGBColor(191, 61, 56),
        );
        draw_byte_bar(
            &root,
            y + 42,
            "metadata in + bitmask out",
            row.input_metadata_bytes + row.output_bytes,
            max_value,
            RGBColor(24, 118, 117),
        );
        y += 130;
    }
}

fn draw_byte_bar(
    root: &DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>,
    y: i32,
    label: &str,
    value: u64,
    max_value: f64,
    color: RGBColor,
) {
    let width = ((value as f64 / max_value) * 620.0).max(2.0) as i32;
    root.draw(&Text::new(
        label.to_string(),
        (300, y + 22),
        ("sans-serif", 16).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();
    root.draw(&Rectangle::new(
        [(535, y), (535 + width, y + 30)],
        ShapeStyle::from(&color).filled(),
    ))
    .unwrap();
    root.draw(&Text::new(
        value.to_string(),
        (550 + width, y + 21),
        ("sans-serif", 15).into_font().color(&RGBColor(82, 88, 96)),
    ))
    .unwrap();
}

fn path_color(path: &str) -> RGBColor {
    match path {
        "metal_affine_transfer_collapse" => RGBColor(24, 118, 117),
        "cpu_affine_wheel" => RGBColor(47, 103, 168),
        "cpu_affine_residue_rows" => RGBColor(92, 107, 122),
        "sequential_small_prime_wheel" => RGBColor(106, 91, 154),
        "sequential_odd_baseline" => RGBColor(191, 61, 56),
        _ => RGBColor(150, 111, 51),
    }
}

fn short_role(role: &str) -> String {
    role.replace("decimal_", "dec_")
        .replace("_visible_lane", "")
        .replace("_side_pocket", "")
}

fn ratio_f64(numerator: u64, denominator_seconds: f64) -> f64 {
    if denominator_seconds <= 0.0 {
        0.0
    } else {
        numerator as f64 / denominator_seconds
    }
}

fn ratio_count(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}
