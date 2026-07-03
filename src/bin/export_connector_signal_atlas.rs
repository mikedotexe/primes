//! Export the deterministic connector signal atlas.

use clap::Parser;
use primes::validation::{
    connector_signal::{build_connector_signal_atlas, render_connector_signal_atlas_markdown},
    reporting::{
        ensure_dir, write_artifact_manifest, write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use std::{error::Error, path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export the deterministic connector signal atlas"
)]
struct Args {
    /// Output directory for connector_signal_atlas.json and .md.
    #[arg(long, default_value = "docs/connector")]
    out_dir: PathBuf,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-connector-signal-atlas error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    ensure_dir(&args.out_dir)?;

    let atlas = build_connector_signal_atlas();
    let markdown = render_connector_signal_atlas_markdown(&atlas);
    let json_path = args.out_dir.join("connector_signal_atlas.json");
    let markdown_path = args.out_dir.join("connector_signal_atlas.md");

    write_json_pretty(&json_path, &atlas)?;
    write_text_file(&markdown_path, &markdown)?;
    write_artifact_manifest(
        &args.out_dir,
        &ArtifactManifest {
            artifact_id: atlas.artifact_id.clone(),
            generator_cmd: "cargo run --bin export_connector_signal_atlas".to_string(),
            args: vec!["--out-dir".to_string(), "docs/connector".to_string()],
            upstream_inputs: vec![
                "src/validation/connector_signal.rs".to_string(),
                "src/connector/analysis.rs".to_string(),
                "lean-proofs/PrimeArithmetic/Connector/ConcatenationFilters.lean".to_string(),
                "lean-proofs/PrimeArithmetic/Connector/ConcatenationFamilies.lean".to_string(),
                "lean-proofs/PrimeArithmetic/Connector/ConcatenationProfileExamples.lean"
                    .to_string(),
                "lean-proofs/PrimeArithmetic/Analysis/HardyLittlewoodShell.lean".to_string(),
            ],
            expected_outputs: vec![
                "connector_signal_atlas.json".to_string(),
                "connector_signal_atlas.md".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )?;

    println!("wrote {}", json_path.display());
    println!("wrote {}", markdown_path.display());
    println!(
        "pairs={} source_cases={} proof_links={}",
        atlas.maintained_pairs.len(),
        atlas.canonical_source_cases.len(),
        atlas.proof_links.len()
    );

    Ok(())
}
