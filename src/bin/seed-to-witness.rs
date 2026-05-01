//! One-command seed-to-witness demo for large affine membrane witnesses.

use clap::Parser;
use primes::validation::{
    large_affine_witness::PROBABLE_PRIME_BASES,
    reporting::{write_json_pretty, write_text_file},
    seed_to_witness::{
        find_seed_to_witness, render_seed_to_witness_transcript, SeedToWitnessConfig,
        DEFAULT_MAX_STEPS, DEFAULT_VISIBLE_DIGITS,
    },
};
use std::{
    error::Error,
    path::PathBuf,
    process,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Walk an affine membrane lane from a seed origin to the first large witness"
)]
struct Args {
    #[arg(long)]
    seed: Option<u64>,

    #[arg(long, default_value_t = 10)]
    base: u32,

    #[arg(long, default_value_t = 3)]
    outer: u32,

    #[arg(long, default_value_t = 7)]
    inner: u32,

    #[arg(long, default_value = "2,1", value_parser = parse_k)]
    k: (u32, u32),

    #[arg(long, default_value_t = DEFAULT_VISIBLE_DIGITS)]
    visible_digits: usize,

    #[arg(long, default_value_t = DEFAULT_MAX_STEPS)]
    max_steps: u64,

    #[arg(long)]
    exact_seed_only: bool,

    #[arg(long)]
    json_out: Option<PathBuf>,

    #[arg(long)]
    markdown_out: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("seed-to-witness error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let seed = args.seed.unwrap_or_else(current_epoch_nanos_u64);
    let config = SeedToWitnessConfig {
        input_seed: seed,
        max_steps: args.max_steps,
        exact_seed_only: args.exact_seed_only,
        base: args.base,
        outer: args.outer,
        inner: args.inner,
        k_outer: args.k.0,
        k_inner: args.k.1,
        visible_digits: args.visible_digits,
        probable_prime_bases: PROBABLE_PRIME_BASES.to_vec(),
    };

    let result = find_seed_to_witness(config)?;
    let transcript = render_seed_to_witness_transcript(&result);
    println!("{transcript}");

    if let Some(path) = args.json_out {
        write_json_pretty(path, &result)?;
    }
    if let Some(path) = args.markdown_out {
        write_text_file(path, &transcript)?;
    }

    Ok(())
}

fn current_epoch_nanos_u64() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    u64::try_from(nanos).expect("current epoch nanoseconds do not fit in u64")
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
