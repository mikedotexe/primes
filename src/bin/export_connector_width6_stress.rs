//! Export the deterministic connector width-6 stress report.

use clap::Parser;
use primes::validation::{
    connector_signal::{
        build_connector_width6_stress_report, render_connector_width6_stress_markdown,
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
    about = "Export the deterministic connector width-6 stress report"
)]
struct Args {
    /// Output directory for connector_width6_stress.json and .md.
    #[arg(long, default_value = "docs/connector")]
    out_dir: PathBuf,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-connector-width6-stress error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    ensure_dir(&args.out_dir)?;

    let report = build_connector_width6_stress_report();
    let markdown = render_connector_width6_stress_markdown(&report);
    let json_path = args.out_dir.join("connector_width6_stress.json");
    let markdown_path = args.out_dir.join("connector_width6_stress.md");
    let manifest_dir = args.out_dir.join("connector_width6_stress_manifest");

    write_json_pretty(&json_path, &report)?;
    write_text_file(&markdown_path, &markdown)?;
    ensure_dir(&manifest_dir)?;
    write_artifact_manifest(
        &manifest_dir,
        &ArtifactManifest {
            artifact_id: report.artifact_id.clone(),
            generator_cmd: "cargo run --bin export_connector_width6_stress".to_string(),
            args: vec!["--out-dir".to_string(), "docs/connector".to_string()],
            upstream_inputs: vec![
                "src/validation/connector_signal.rs".to_string(),
                "src/validation/connector_signal_impl.rs".to_string(),
                "src/connector/analysis.rs".to_string(),
                "docs/connector/connector_signal_atlas.json".to_string(),
            ],
            expected_outputs: vec![
                "connector_width6_stress.json".to_string(),
                "connector_width6_stress.md".to_string(),
                "connector_width6_stress_manifest/artifact_manifest.json".to_string(),
            ],
        },
    )?;

    println!("wrote {}", json_path.display());
    println!("wrote {}", markdown_path.display());
    println!(
        "rows={} summaries={} decision={}",
        report.rows.len(),
        report.pair_summaries.len(),
        report.target_decision
    );

    Ok(())
}
