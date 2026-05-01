//! Report for the maintained Metal affine transfer-collapse prototype.
//!
//! Run with:
//!
//! ```bash
//! cargo run --features metal --release --example metal_affine_transfer_collapse_report -- --out-dir /tmp/primes_metal_affine_transfer
//! ```

use primes::validation::{
    fast_affine::{build_fast_affine_lane, scan_fast_prime_lane, FastAffineLane, FastLaneConfig},
    metal_affine::scan_metal_affine_lane,
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
    time::Instant,
};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_metal_affine_transfer";
const ARTIFACT_ID: &str = "metal_affine_transfer_collapse_report";
const REPORT_EXPORT_VERSION: u32 = 1;
const DEFAULT_SEED_COUNT: u64 = 10_000;
const DEFAULT_MAX_PRIMES: usize = 20;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
    seed_count: u64,
    max_primes: usize,
}

#[derive(Debug, Clone, Copy)]
struct LaneSpec {
    role: &'static str,
    base: u32,
    outer: u32,
    inner: u32,
    middle_length: usize,
    k: (u32, u32),
    include_legacy_candidate_gpu: bool,
    note: &'static str,
}

const LANE_SPECS: &[LaneSpec] = &[
    LaneSpec {
        role: "base6_legacy_comparison_lane",
        base: 6,
        outer: 1,
        inner: 5,
        middle_length: 6,
        k: (0, 0),
        include_legacy_candidate_gpu: true,
        note: "u32-safe maintained lane used to compare candidate-buffer GPU transfer against affine metadata transfer",
    },
    LaneSpec {
        role: "decimal_visible_zero_run_k21",
        base: 10,
        outer: 3,
        inner: 7,
        middle_length: 2,
        k: (2, 1),
        include_legacy_candidate_gpu: false,
        note: "visible decimal teaching lane; legacy u32 candidate-buffer GPU is out of scope",
    },
    LaneSpec {
        role: "base22_side_pocket",
        base: 22,
        outer: 17,
        inner: 19,
        middle_length: 2,
        k: (2, 2),
        include_legacy_candidate_gpu: false,
        note: "base-22 pocket lane whose full values are large but residue metadata remains tiny",
    },
];

#[derive(Debug, Clone, Serialize)]
struct TransferComparisonRow {
    role: String,
    path: String,
    status: String,
    base: u32,
    pair_label: String,
    k_label: String,
    middle_length: usize,
    scanned_seed_count: u64,
    candidate_value_buffer_bytes: u64,
    input_metadata_bytes: u64,
    output_bytes: u64,
    avoided_candidate_value_bytes_u64: u64,
    gpu_sieve_seconds: f64,
    cpu_confirm_seconds: f64,
    total_seconds: f64,
    survivor_count: u64,
    primes_found: u64,
    first_witness: String,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct PrimeWitnessRow {
    role: String,
    path: String,
    seed: u64,
    middle_digits: String,
    template_digits: String,
    value: u64,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    requested_seed_count: u64,
    max_primes: usize,
    deterministic_scope: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    lane_count: usize,
    comparison_row_count: usize,
    metal_rows: usize,
    total_candidate_value_bytes_avoided: u64,
    exact_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    summary: ReportSummary,
    transfer_rows: Vec<TransferComparisonRow>,
    prime_witness_rows: Vec<PrimeWitnessRow>,
    observations: Vec<String>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("create output dir");

    let mut transfer_rows = Vec::new();
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
        append_cpu_row(
            spec,
            &options,
            config.clone(),
            &mut transfer_rows,
            &mut witness_rows,
        );
        append_metal_row(
            spec,
            &options,
            config,
            &mut transfer_rows,
            &mut witness_rows,
        );
        if spec.include_legacy_candidate_gpu {
            append_legacy_candidate_gpu_row(
                spec,
                &options,
                &lane,
                &mut transfer_rows,
                &mut witness_rows,
            );
        }
    }

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        requested_seed_count: options.seed_count,
        max_primes: options.max_primes,
        deterministic_scope: "u64 maintained affine lanes with CPU deterministic confirmation"
            .to_string(),
    };
    let summary = build_summary(&transfer_rows);
    let observations = build_observations(&transfer_rows);
    let report = render_report(&settings, &summary, &transfer_rows, &observations);
    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        summary: summary.clone(),
        transfer_rows: transfer_rows.clone(),
        prime_witness_rows: witness_rows.clone(),
        observations,
    };

    write_csv_rows(options.out_dir.join("transfer_rows.csv"), &transfer_rows)
        .expect("write transfer rows");
    write_csv_rows(
        options.out_dir.join("prime_witness_rows.csv"),
        &witness_rows,
    )
    .expect("write witness rows");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary");
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
                "metal_affine_transfer_collapse_report".to_string(),
            ],
            upstream_inputs: vec![
                "src/validation/metal_affine.rs".to_string(),
                "src/validation/fast_affine.rs".to_string(),
                "shaders/sieve_affine.metal".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "transfer_rows.csv".to_string(),
                "prime_witness_rows.csv".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )
    .expect("write manifest");

    println!(
        "wrote Metal affine transfer-collapse bundle to {}",
        options.out_dir.display()
    );
    println!("{}", summary.exact_takeaway);
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut seed_count = DEFAULT_SEED_COUNT;
    let mut max_primes = DEFAULT_MAX_PRIMES;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = PathBuf::from(args.next().expect("missing --out-dir")),
            "--seed-count" => {
                seed_count = args
                    .next()
                    .expect("missing --seed-count")
                    .parse()
                    .expect("invalid --seed-count")
            }
            "--max-primes" => {
                max_primes = args
                    .next()
                    .expect("missing --max-primes")
                    .parse()
                    .expect("invalid --max-primes")
            }
            _ => panic!("unrecognized argument: {arg}"),
        }
    }
    Options {
        out_dir,
        seed_count,
        max_primes,
    }
}

fn append_cpu_row(
    spec: &LaneSpec,
    options: &Options,
    config: FastLaneConfig,
    transfer_rows: &mut Vec<TransferComparisonRow>,
    witness_rows: &mut Vec<PrimeWitnessRow>,
) {
    let run = scan_fast_prime_lane(config, options.seed_count, options.max_primes, 1_000_000)
        .unwrap_or_else(|err| panic!("CPU fast path failed for {}: {err}", spec.role));
    for witness in &run.witnesses {
        witness_rows.push(PrimeWitnessRow {
            role: spec.role.to_string(),
            path: "cpu_fast_affine_wheel".to_string(),
            seed: witness.seed,
            middle_digits: witness.middle_digits.clone(),
            template_digits: witness.template_digits.clone(),
            value: witness.value,
        });
    }
    transfer_rows.push(TransferComparisonRow {
        role: spec.role.to_string(),
        path: "cpu_fast_affine_wheel".to_string(),
        status: "ok".to_string(),
        base: run.config.base,
        pair_label: run.pair_label,
        k_label: run.k_label,
        middle_length: run.config.middle_length,
        scanned_seed_count: run.scanned_seed_count,
        candidate_value_buffer_bytes: 0,
        input_metadata_bytes: 0,
        output_bytes: 0,
        avoided_candidate_value_bytes_u64: 0,
        gpu_sieve_seconds: 0.0,
        cpu_confirm_seconds: run.elapsed_seconds,
        total_seconds: run.elapsed_seconds,
        survivor_count: run.admissible_seed_count,
        primes_found: run.primes_found,
        first_witness: run
            .witnesses
            .first()
            .map(|witness| witness.value.to_string())
            .unwrap_or_default(),
        note: "CPU maintained baseline; no GPU transfer occurs".to_string(),
    });
}

fn append_metal_row(
    spec: &LaneSpec,
    options: &Options,
    config: FastLaneConfig,
    transfer_rows: &mut Vec<TransferComparisonRow>,
    witness_rows: &mut Vec<PrimeWitnessRow>,
) {
    match scan_metal_affine_lane(config, options.seed_count, options.max_primes, 0, None) {
        Ok(run) => {
            for witness in &run.witnesses {
                witness_rows.push(PrimeWitnessRow {
                    role: spec.role.to_string(),
                    path: "metal_affine_transfer_collapse".to_string(),
                    seed: witness.seed,
                    middle_digits: witness.middle_digits.clone(),
                    template_digits: witness.template_digits.clone(),
                    value: witness.value,
                });
            }
            transfer_rows.push(TransferComparisonRow {
                role: spec.role.to_string(),
                path: "metal_affine_transfer_collapse".to_string(),
                status: "ok".to_string(),
                base: run.config.base,
                pair_label: run.pair_label,
                k_label: run.k_label,
                middle_length: run.config.middle_length,
                scanned_seed_count: run.scanned_seed_count,
                candidate_value_buffer_bytes: 0,
                input_metadata_bytes: run.metrics.input_metadata_bytes,
                output_bytes: run.metrics.output_bitmask_bytes,
                avoided_candidate_value_bytes_u64: run.metrics.avoided_candidate_value_bytes_u64,
                gpu_sieve_seconds: run.metrics.gpu_sieve_seconds,
                cpu_confirm_seconds: run.metrics.cpu_confirm_seconds,
                total_seconds: run.metrics.total_seconds,
                survivor_count: run.survivor_seed_count,
                primes_found: run.primes_found,
                first_witness: run
                    .witnesses
                    .first()
                    .map(|witness| witness.value.to_string())
                    .unwrap_or_default(),
                note: spec.note.to_string(),
            });
        }
        Err(err) => transfer_rows.push(TransferComparisonRow {
            role: spec.role.to_string(),
            path: "metal_affine_transfer_collapse".to_string(),
            status: format!("unavailable: {err}"),
            base: spec.base,
            pair_label: format!("({},{})", spec.outer, spec.inner),
            k_label: format!("k=({},{})", spec.k.0, spec.k.1),
            middle_length: spec.middle_length,
            scanned_seed_count: 0,
            candidate_value_buffer_bytes: 0,
            input_metadata_bytes: 0,
            output_bytes: 0,
            avoided_candidate_value_bytes_u64: 0,
            gpu_sieve_seconds: 0.0,
            cpu_confirm_seconds: 0.0,
            total_seconds: 0.0,
            survivor_count: 0,
            primes_found: 0,
            first_witness: String::new(),
            note: "Build on macOS with --features metal to run this path".to_string(),
        }),
    }
}

fn append_legacy_candidate_gpu_row(
    spec: &LaneSpec,
    options: &Options,
    lane: &FastAffineLane,
    transfer_rows: &mut Vec<TransferComparisonRow>,
    witness_rows: &mut Vec<PrimeWitnessRow>,
) {
    match run_legacy_candidate_buffer_gpu(lane, options.seed_count, options.max_primes) {
        Ok(row) => {
            for witness in row.witnesses {
                witness_rows.push(PrimeWitnessRow {
                    role: spec.role.to_string(),
                    path: "legacy_candidate_buffer_gpu".to_string(),
                    seed: witness.seed,
                    middle_digits: witness.middle_digits,
                    template_digits: witness.template_digits,
                    value: witness.value,
                });
            }
            transfer_rows.push(row.row);
        }
        Err(err) => transfer_rows.push(TransferComparisonRow {
            role: spec.role.to_string(),
            path: "legacy_candidate_buffer_gpu".to_string(),
            status: format!("unavailable: {err}"),
            base: spec.base,
            pair_label: lane.config.pair_label(),
            k_label: lane.config.k_label(),
            middle_length: spec.middle_length,
            scanned_seed_count: 0,
            candidate_value_buffer_bytes: 0,
            input_metadata_bytes: 0,
            output_bytes: 0,
            avoided_candidate_value_bytes_u64: 0,
            gpu_sieve_seconds: 0.0,
            cpu_confirm_seconds: 0.0,
            total_seconds: 0.0,
            survivor_count: 0,
            primes_found: 0,
            first_witness: String::new(),
            note: "Legacy path requires u32 candidate values and --features metal".to_string(),
        }),
    }
}

struct LegacyGpuBundle {
    row: TransferComparisonRow,
    witnesses: Vec<LegacyWitness>,
}

struct LegacyWitness {
    seed: u64,
    middle_digits: String,
    template_digits: String,
    value: u64,
}

#[cfg(all(feature = "metal", target_os = "macos"))]
fn run_legacy_candidate_buffer_gpu(
    lane: &FastAffineLane,
    requested_seed_count: u64,
    max_witnesses: usize,
) -> Result<LegacyGpuBundle, String> {
    use primes::gpu::GpuSieve;

    let scanned_seed_count = requested_seed_count.min(lane.seed_capacity);
    if scanned_seed_count > u32::MAX as u64 {
        return Err("seed count exceeds u32 legacy GPU range".to_string());
    }
    let mut values = Vec::with_capacity(scanned_seed_count as usize);
    for seed in 0..scanned_seed_count {
        let value = lane
            .candidate_value(seed)
            .ok_or_else(|| format!("candidate value overflow at seed {seed}"))?;
        values
            .push(u32::try_from(value).map_err(|_| {
                format!("candidate value {value} does not fit u32 legacy GPU path")
            })?);
    }
    let gpu = GpuSieve::new().map_err(|err| format!("GPU init: {err}"))?;
    let total_start = Instant::now();
    let gpu_start = Instant::now();
    let survivor_indices = gpu
        .sieve(&values, lane.config.base)
        .map_err(|err| format!("GPU sieve: {err}"))?;
    let gpu_sieve_seconds = gpu_start.elapsed().as_secs_f64();

    let confirm_start = Instant::now();
    let mut primes_found = 0u64;
    let mut witnesses = Vec::new();
    for &idx in &survivor_indices {
        let seed = idx as u64;
        let value = values[idx as usize] as u64;
        if primal::is_prime(value) {
            primes_found += 1;
            if witnesses.len() < max_witnesses {
                witnesses.push(LegacyWitness {
                    seed,
                    middle_digits: lane.middle_digits(seed),
                    template_digits: lane.template_digits(seed),
                    value,
                });
            }
        }
    }
    let cpu_confirm_seconds = confirm_start.elapsed().as_secs_f64();
    let total_seconds = total_start.elapsed().as_secs_f64();
    let first_witness = witnesses
        .first()
        .map(|witness| witness.value.to_string())
        .unwrap_or_default();
    Ok(LegacyGpuBundle {
        row: TransferComparisonRow {
            role: "base6_legacy_comparison_lane".to_string(),
            path: "legacy_candidate_buffer_gpu".to_string(),
            status: "ok".to_string(),
            base: lane.config.base,
            pair_label: lane.config.pair_label(),
            k_label: lane.config.k_label(),
            middle_length: lane.config.middle_length,
            scanned_seed_count,
            candidate_value_buffer_bytes: scanned_seed_count * std::mem::size_of::<u32>() as u64,
            input_metadata_bytes: 0,
            output_bytes: survivor_indices.len() as u64 * std::mem::size_of::<u32>() as u64,
            avoided_candidate_value_bytes_u64: 0,
            gpu_sieve_seconds,
            cpu_confirm_seconds,
            total_seconds,
            survivor_count: survivor_indices.len() as u64,
            primes_found,
            first_witness,
            note: "Current legacy GPU surface: transfers precomputed u32 candidate values before GPU filtering".to_string(),
        },
        witnesses,
    })
}

#[cfg(not(all(feature = "metal", target_os = "macos")))]
fn run_legacy_candidate_buffer_gpu(
    _lane: &FastAffineLane,
    _requested_seed_count: u64,
    _max_witnesses: usize,
) -> Result<LegacyGpuBundle, String> {
    Err("unavailable without macOS --features metal".to_string())
}

fn build_summary(rows: &[TransferComparisonRow]) -> ReportSummary {
    let metal_rows = rows
        .iter()
        .filter(|row| row.path == "metal_affine_transfer_collapse" && row.status == "ok")
        .count();
    let total_candidate_value_bytes_avoided = rows
        .iter()
        .filter(|row| row.path == "metal_affine_transfer_collapse")
        .map(|row| row.avoided_candidate_value_bytes_u64)
        .sum();
    ReportSummary {
        lane_count: LANE_SPECS.len(),
        comparison_row_count: rows.len(),
        metal_rows,
        total_candidate_value_bytes_avoided,
        exact_takeaway:
            "The maintained Metal path transfers affine residue metadata and a survivor bitmask, not full candidate values."
                .to_string(),
    }
}

fn build_observations(rows: &[TransferComparisonRow]) -> Vec<String> {
    let legacy = rows
        .iter()
        .find(|row| row.path == "legacy_candidate_buffer_gpu");
    let metal = rows.iter().find(|row| {
        row.role == "base6_legacy_comparison_lane" && row.path == "metal_affine_transfer_collapse"
    });
    let mut observations = vec![
        "This is a candidate-transfer-collapse claim, not total zero-copy: params, residue rows, and output masks are still shared Metal buffers.".to_string(),
        "The affine residue loop is implemented in the dedicated `.metal` kernel `shaders/sieve_affine.metal::sieve_affine_lane`; Rust's `metal` crate is the host/dispatch layer.".to_string(),
        "Prime witnesses remain CPU-confirmed with deterministic `primal::is_prime(u64)`.".to_string(),
        "The old `membrane-prime-ultra` GPU path is not used here; its optimized kernel lookup is currently runtime-fragile.".to_string(),
    ];
    if let (Some(legacy), Some(metal)) = (legacy, metal) {
        observations.push(format!(
            "On the shared base-6 comparison lane, legacy candidate-buffer input is {} bytes while the maintained Metal affine metadata input is {} bytes.",
            legacy.candidate_value_buffer_bytes, metal.input_metadata_bytes
        ));
    }
    observations
}

fn render_report(
    settings: &ReportSettings,
    summary: &ReportSummary,
    rows: &[TransferComparisonRow],
    observations: &[String],
) -> String {
    let mut lines = Vec::new();
    lines.push("# Metal Affine Transfer-Collapse Report".to_string());
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
        "- deterministic scope: `{}`",
        settings.deterministic_scope
    ));
    lines.push(String::new());
    lines.push("## Transfer Claim".to_string());
    lines.push(
        "- The maintained Metal path sends lane residue metadata, not a candidate-value array."
            .to_string(),
    );
    lines.push(
        "- The residue sieve itself runs in a dedicated `.metal` shader; Rust hosts the Metal pipeline and buffers."
            .to_string(),
    );
    lines.push(
        "- GPU output is a compact survivor bitmask; CPU reconstructs and confirms only survivors."
            .to_string(),
    );
    lines.push("- This should be described as `candidate-transfer collapse` or `zero candidate transfer`, not total zero-copy.".to_string());
    lines.push(String::new());
    lines.push("## Headline".to_string());
    lines.push(format!("- {}", summary.exact_takeaway));
    lines.push(format!(
        "- Metal rows completed: `{}`; u64 candidate-value bytes avoided: `{}`",
        summary.metal_rows, summary.total_candidate_value_bytes_avoided
    ));
    lines.push(String::new());
    lines.push("## Comparison Rows".to_string());
    lines.push("| Role | Path | Status | Scanned | Candidate Buffer Bytes | Metadata In | Output | Survivors | Primes |".to_string());
    lines.push("|---|---|---|---:|---:|---:|---:|---:|---:|".to_string());
    for row in rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | {} | {} | {} | {} | {} | {} |",
            row.role,
            row.path,
            row.status,
            row.scanned_seed_count,
            row.candidate_value_buffer_bytes,
            row.input_metadata_bytes,
            row.output_bytes,
            row.survivor_count,
            row.primes_found
        ));
    }
    lines.push(String::new());
    lines.push("## Observations".to_string());
    for observation in observations {
        lines.push(format!("- {}", observation));
    }
    lines.push(String::new());
    lines.push("## Artifacts".to_string());
    lines.push("- `transfer_rows.csv`: CPU, maintained Metal, and legacy candidate-buffer comparison rows.".to_string());
    lines.push("- `prime_witness_rows.csv`: first CPU-confirmed prime witnesses.".to_string());
    lines.push("- `summary.json`: machine-readable settings, rows, and observations.".to_string());
    lines.join("\n")
}

#[allow(dead_code)]
fn path_label(path: &Path) -> String {
    path.display().to_string()
}
