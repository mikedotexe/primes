//! Verify a proof-carrying witness certificate without rerunning witness search.

use clap::Parser;
use primes::validation::{
    reporting::write_json_pretty,
    seed_to_witness::{verify_proof_carrying_witness_certificate, ProofCarryingWitnessCertificate},
};
use std::{error::Error, fs, path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Verify a proof-carrying witness certificate's affine and residue evidence"
)]
struct Args {
    certificate_json: PathBuf,

    #[arg(long)]
    json_out: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("verify-proof-carrying-witness error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let text = fs::read_to_string(&args.certificate_json)?;
    let certificate: ProofCarryingWitnessCertificate = serde_json::from_str(&text)?;
    let report = verify_proof_carrying_witness_certificate(&certificate);

    if let Some(path) = args.json_out {
        write_json_pretty(path, &report)?;
    }

    if report.ok {
        println!(
            "verified proof-carrying witness certificate: {} ({} residue rows)",
            args.certificate_json.display(),
            report.checked_residue_row_count
        );
        Ok(())
    } else {
        eprintln!(
            "proof-carrying witness certificate failed verification: {}",
            args.certificate_json.display()
        );
        for failure in &report.failures {
            eprintln!("- {failure}");
        }
        process::exit(1);
    }
}
