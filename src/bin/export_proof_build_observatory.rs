use clap::Parser;
use primes::validation::proof_build_observatory::{
    build_proof_build_observatory_manifest, build_proof_build_observatory_report,
    read_timing_source, render_proof_build_observatory_markdown,
};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(about = "Normalize proof-build timing JSONs into one local observatory atlas")]
struct Args {
    #[arg(long)]
    witness_timing: PathBuf,

    #[arg(long)]
    matched_control_timing: PathBuf,

    #[arg(long)]
    lean_umbrella_timing: PathBuf,

    #[arg(long)]
    out_json: PathBuf,

    #[arg(long)]
    out_md: PathBuf,

    #[arg(long)]
    manifest_out: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let sources = vec![
        read_timing_source("witness-lean", &args.witness_timing)?,
        read_timing_source("matched-control-atlas", &args.matched_control_timing)?,
        read_timing_source("lean-umbrella", &args.lean_umbrella_timing)?,
    ];
    let report = build_proof_build_observatory_report(&sources)?;
    let markdown = render_proof_build_observatory_markdown(&report);
    let raw_reports = sources
        .iter()
        .map(|source| (source.suite.clone(), source.path.clone()))
        .collect::<Vec<_>>();
    let manifest =
        build_proof_build_observatory_manifest(&args.out_json, &args.out_md, &raw_reports);

    write_json(&args.out_json, &report)?;
    write_text(&args.out_md, &markdown)?;
    write_json(&args.manifest_out, &manifest)?;

    println!("proof-build observatory: {}", args.out_json.display());
    println!("proof-build observatory summary: {}", args.out_md.display());
    println!(
        "proof-build observatory manifest: {}",
        args.manifest_out.display()
    );
    Ok(())
}

fn write_json<T: serde::Serialize>(
    path: &PathBuf,
    payload: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut json = serde_json::to_string_pretty(payload)?;
    json.push('\n');
    fs::write(path, json)?;
    Ok(())
}

fn write_text(path: &PathBuf, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}
