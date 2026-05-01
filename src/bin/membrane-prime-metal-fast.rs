//! Metal transfer-collapse membrane prime generator for deterministic u64 lanes.

use clap::Parser;
use primes::validation::{
    fast_affine::{FastLaneConfig, FastPrimeWitness},
    metal_affine::scan_metal_affine_lane,
    reporting::{write_csv_rows, write_json_pretty},
};
use serde::Serialize;
use std::{error::Error, path::PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Metal affine membrane generator with zero candidate-value transfer"
)]
struct Args {
    #[arg(long, default_value_t = 10)]
    base: u32,

    #[arg(long)]
    outer: u32,

    #[arg(long)]
    inner: u32,

    #[arg(long, value_parser = parse_k)]
    k: (u32, u32),

    #[arg(long)]
    middle_length: usize,

    #[arg(long, default_value_t = 10_000)]
    seed_count: u64,

    #[arg(long, default_value_t = 0)]
    seed_offset: u64,

    #[arg(long, default_value_t = 20)]
    max_primes: usize,

    #[arg(long)]
    residue_row_limit: Option<usize>,

    #[arg(long)]
    json_out: Option<PathBuf>,

    #[arg(long)]
    csv_out: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
struct WitnessCsvRow {
    seed: u64,
    middle_digits: String,
    template_digits: String,
    value: u64,
}

impl From<&FastPrimeWitness> for WitnessCsvRow {
    fn from(value: &FastPrimeWitness) -> Self {
        Self {
            seed: value.seed,
            middle_digits: value.middle_digits.clone(),
            template_digits: value.template_digits.clone(),
            value: value.value,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config = FastLaneConfig::new(
        args.base,
        args.outer,
        args.inner,
        args.middle_length,
        args.k,
    );
    let run = scan_metal_affine_lane(
        config,
        args.seed_count,
        args.max_primes,
        args.seed_offset,
        args.residue_row_limit,
    )?;

    println!("Metal affine transfer-collapse scan");
    println!("  base: {}", run.config.base);
    println!("  pair: {}", run.pair_label);
    println!("  lane: {}", run.k_label);
    println!("  affine: N(s) = {} + {}*s", run.shift, run.gradient);
    println!(
        "  seeds: {} scanned from offset {} of {} requested{}",
        run.scanned_seed_count,
        run.seed_offset,
        run.requested_seed_count,
        if run.capped_to_seed_capacity {
            " (capped to finite seed space)"
        } else {
            ""
        }
    );
    println!(
        "  residue rows: {} small-prime filters",
        run.residue_rows.len()
    );
    println!(
        "  transfer: {} metadata bytes in, {} bitmask bytes out, {} u64 candidate-value bytes avoided",
        run.metrics.input_metadata_bytes,
        run.metrics.output_bitmask_bytes,
        run.metrics.avoided_candidate_value_bytes_u64
    );
    println!(
        "  funnel: {} raw -> {} GPU survivors/primality tests -> {} primes",
        run.scanned_seed_count, run.survivor_seed_count, run.primes_found
    );
    println!(
        "  timing: {:.6}s GPU sieve, {:.6}s CPU confirmation, {:.6}s total",
        run.metrics.gpu_sieve_seconds, run.metrics.cpu_confirm_seconds, run.metrics.total_seconds
    );
    for witness in &run.witnesses {
        println!(
            "  prime seed {} [{}]: {} = {}",
            witness.seed, witness.middle_digits, witness.template_digits, witness.value
        );
    }

    if let Some(path) = args.json_out {
        write_json_pretty(path, &run)?;
    }
    if let Some(path) = args.csv_out {
        let rows = run
            .witnesses
            .iter()
            .map(WitnessCsvRow::from)
            .collect::<Vec<_>>();
        write_csv_rows(path, &rows)?;
    }

    Ok(())
}

fn parse_k(value: &str) -> Result<(u32, u32), String> {
    let parts = value
        .split(',')
        .map(str::trim)
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| format!("invalid k value: {err}"))?;
    if parts.len() != 2 {
        return Err("k must be formatted as outer,inner, for example 2,1".to_string());
    }
    Ok((parts[0], parts[1]))
}
