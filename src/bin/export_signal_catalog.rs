//! Export the lightweight top-level signal catalog.

use clap::Parser;
use primes::validation::{
    reporting::{
        ensure_dir, write_artifact_manifest, write_json_pretty, write_text_file, ArtifactManifest,
    },
    signal_catalog::{build_signal_catalog, render_signal_catalog_markdown},
};
use std::{error::Error, path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export the lightweight matched-control/witness/connector signal catalog"
)]
struct Args {
    /// Output directory for signal_catalog.json and .md.
    #[arg(long, default_value = "docs/signal_catalog")]
    out_dir: PathBuf,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-signal-catalog error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    ensure_dir(&args.out_dir)?;

    let catalog = build_signal_catalog();
    let markdown = render_signal_catalog_markdown(&catalog);
    let json_path = args.out_dir.join("signal_catalog.json");
    let markdown_path = args.out_dir.join("signal_catalog.md");

    write_json_pretty(&json_path, &catalog)?;
    write_text_file(&markdown_path, &markdown)?;
    write_artifact_manifest(
        &args.out_dir,
        &ArtifactManifest {
            artifact_id: catalog.artifact_id.clone(),
            generator_cmd: "cargo run --bin export_signal_catalog".to_string(),
            args: vec!["--out-dir".to_string(), "docs/signal_catalog".to_string()],
            upstream_inputs: vec![
                "docs/atlas/matched_control_smoke_atlas_manifest.json".to_string(),
                "docs/witness/witness_search_policy_atlas.json".to_string(),
                "docs/witness/witness_lean_catalog_manifest.json".to_string(),
                "docs/witness/witness_policy_matrix_lean_catalog_manifest.json".to_string(),
                "docs/connector/connector_signal_atlas.json".to_string(),
                "docs/connector/connector_width6_stress.json".to_string(),
                "docs/connector/connector_replication_null_atlas.json".to_string(),
            ],
            expected_outputs: vec![
                "signal_catalog.json".to_string(),
                "signal_catalog.md".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )?;

    println!("wrote {}", json_path.display());
    println!("wrote {}", markdown_path.display());
    println!(
        "rows={} matched_control={} witness={} connector={}",
        catalog.summary.row_count,
        catalog.summary.matched_control_rows,
        catalog.summary.witness_rows,
        catalog.summary.connector_rows
    );

    Ok(())
}
