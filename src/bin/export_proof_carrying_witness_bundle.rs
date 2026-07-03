//! Export the canonical proof-carrying witness certificate bundle.

use clap::Parser;
use primes::validation::{
    reporting::{ensure_dir, write_json_pretty},
    seed_to_witness::{
        build_proof_carrying_witness_certificate_for_config, build_proof_carrying_witness_manifest,
        canonical_proof_carrying_witness_specs, proof_carrying_witness_manifest_artifact,
    },
};
use std::{error::Error, path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export deterministic proof-carrying witness certificates and manifest"
)]
struct Args {
    #[arg(long, default_value = "docs/witness")]
    out_dir: PathBuf,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-proof-carrying-witness-bundle error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    ensure_dir(&args.out_dir)?;

    let mut manifest_artifacts = Vec::new();
    for spec in canonical_proof_carrying_witness_specs() {
        let certificate = build_proof_carrying_witness_certificate_for_config(spec.config.clone())?;
        write_json_pretty(args.out_dir.join(spec.file_name), &certificate)?;
        manifest_artifacts.push(proof_carrying_witness_manifest_artifact(
            &spec,
            &certificate,
        ));
        println!("wrote {}", args.out_dir.join(spec.file_name).display());
    }

    let manifest = build_proof_carrying_witness_manifest(manifest_artifacts);
    write_json_pretty(
        args.out_dir.join("witness_certificate_manifest.json"),
        &manifest,
    )?;
    println!(
        "wrote {}",
        args.out_dir
            .join("witness_certificate_manifest.json")
            .display()
    );

    Ok(())
}
