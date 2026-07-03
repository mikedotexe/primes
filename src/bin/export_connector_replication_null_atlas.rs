//! Export the deterministic connector replication null atlas.

use clap::Parser;
use primes::validation::{
    connector_signal::{
        build_connector_replication_null_atlas, render_connector_replication_null_atlas_markdown,
    },
    reporting::{
        ensure_dir, write_artifact_manifest, write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use std::{error::Error, path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export the deterministic connector replication null atlas"
)]
struct Args {
    /// Output directory for connector_replication_null_atlas.json and .md.
    #[arg(long, default_value = "docs/connector")]
    out_dir: PathBuf,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-connector-replication-null-atlas error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    ensure_dir(&args.out_dir)?;

    let atlas = build_connector_replication_null_atlas();
    let markdown = render_connector_replication_null_atlas_markdown(&atlas);
    let json_path = args.out_dir.join("connector_replication_null_atlas.json");
    let markdown_path = args.out_dir.join("connector_replication_null_atlas.md");
    let manifest_dir = args
        .out_dir
        .join("connector_replication_null_atlas_manifest");

    write_json_pretty(&json_path, &atlas)?;
    write_text_file(&markdown_path, &markdown)?;
    ensure_dir(&manifest_dir)?;
    write_artifact_manifest(
        &manifest_dir,
        &ArtifactManifest {
            artifact_id: atlas.artifact_id.clone(),
            generator_cmd: "cargo run --bin export_connector_replication_null_atlas".to_string(),
            args: vec!["--out-dir".to_string(), "docs/connector".to_string()],
            upstream_inputs: vec![
                "src/validation/connector_signal.rs".to_string(),
                "src/validation/connector_signal_impl.rs".to_string(),
                "docs/connector/connector_width6_stress.json".to_string(),
            ],
            expected_outputs: vec![
                "connector_replication_null_atlas.json".to_string(),
                "connector_replication_null_atlas.md".to_string(),
                "connector_replication_null_atlas_manifest/artifact_manifest.json".to_string(),
            ],
        },
    )?;

    println!("wrote {}", json_path.display());
    println!("wrote {}", markdown_path.display());
    println!(
        "rows={} theorem_candidates={} status={}",
        atlas.summary.branch_row_count,
        atlas.summary.theorem_candidate_count,
        atlas.summary.single_branch_separator_stability_status
    );

    Ok(())
}
