//! Export the proof-carrying witness search-policy atlas.

use clap::Parser;
use primes::validation::{
    reporting::{ensure_dir, write_json_pretty, write_text_file},
    seed_to_witness::{
        build_proof_carrying_witness_search_policy_atlas, canonical_proof_carrying_witness_specs,
        proof_carrying_witness_search_policy_coverage_row,
        render_proof_carrying_witness_search_policy_atlas_markdown,
        verify_proof_carrying_witness_certificate, ProofCarryingWitnessCertificate,
    },
};
use std::{error::Error, fs, path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export a deterministic proof-carrying witness search-policy atlas"
)]
struct Args {
    /// Directory containing the canonical proof-carrying witness certificates.
    #[arg(long, default_value = "docs/witness")]
    certificate_dir: PathBuf,

    /// Output directory for witness_search_policy_atlas.json and .md.
    #[arg(long, default_value = "docs/witness")]
    out_dir: PathBuf,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-proof-carrying-witness-search-policy-atlas error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    ensure_dir(&args.out_dir)?;

    let mut coverage_rows = Vec::new();
    for spec in canonical_proof_carrying_witness_specs() {
        let certificate_path = args.certificate_dir.join(spec.file_name);
        let certificate_text = fs::read_to_string(&certificate_path)?;
        let certificate: ProofCarryingWitnessCertificate = serde_json::from_str(&certificate_text)?;
        let verification = verify_proof_carrying_witness_certificate(&certificate);
        if !verification.ok {
            return Err(format!(
                "certificate failed verification before atlas export: {}",
                verification.failures.join("; ")
            )
            .into());
        }
        coverage_rows.push(proof_carrying_witness_search_policy_coverage_row(
            &spec,
            &certificate,
        ));
    }

    let atlas = build_proof_carrying_witness_search_policy_atlas(coverage_rows);
    let markdown = render_proof_carrying_witness_search_policy_atlas_markdown(&atlas);
    let json_path = args.out_dir.join("witness_search_policy_atlas.json");
    let markdown_path = args.out_dir.join("witness_search_policy_atlas.md");

    write_json_pretty(&json_path, &atlas)?;
    write_text_file(&markdown_path, &markdown)?;

    println!("wrote {}", json_path.display());
    println!("wrote {}", markdown_path.display());
    println!(
        "artifacts={} lanes={} max_first_accepted_distance={}",
        atlas.summary.artifact_count,
        atlas.summary.lane_count,
        atlas.summary.max_first_accepted_distance
    );

    Ok(())
}
