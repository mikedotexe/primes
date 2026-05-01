//! Maintained fast membrane prime generator for deterministic u64 lanes.

use clap::Parser;
use primes::validation::{
    fast_affine::{scan_fast_prime_lane, FastLaneConfig, FastPrimeWitness},
    reporting::{write_csv_rows, write_json_pretty},
};
use serde::Serialize;
use std::{error::Error, path::PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Fast deterministic affine membrane prime generator"
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

    #[arg(long, default_value_t = 20)]
    max_primes: usize,

    #[arg(long, default_value_t = 1_000_000)]
    wheel_period_cap: u64,

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
    let run = scan_fast_prime_lane(
        config,
        args.seed_count,
        args.max_primes,
        args.wheel_period_cap,
    )?;

    println!("Fast affine membrane prime scan");
    println!("  base: {}", run.config.base);
    println!("  pair: {}", run.pair_label);
    println!("  lane: {}", run.k_label);
    println!("  affine: N(s) = {} + {}*s", run.shift, run.gradient);
    println!(
        "  seeds: {} scanned of {} requested{}",
        run.scanned_seed_count,
        run.requested_seed_count,
        if run.capped_to_seed_capacity {
            " (capped to finite seed space)"
        } else {
            ""
        }
    );
    println!(
        "  wheel: period {}, moduli {:?}, admissible residues {}",
        run.wheel_period, run.wheel_moduli, run.wheel_admissible_residue_count
    );
    println!(
        "  funnel: {} raw -> {} admissible/primality tests -> {} primes",
        run.scanned_seed_count, run.admissible_seed_count, run.primes_found
    );
    println!(
        "  throughput: {:.0} seeds/s, {:.0} tests/s, {:.2} primes/s",
        run.seeds_per_second, run.primality_tests_per_second, run.primes_per_second
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
