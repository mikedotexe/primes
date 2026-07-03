//! Export the proof-carrying matched-control atlas manifest.

use clap::Parser;
use primes::validation::matched_control::{
    build_matched_control_atlas_manifest, write_matched_control_atlas_manifest_json,
    MatchedControlPanel,
};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export a deterministic matched-control atlas manifest as JSON"
)]
struct Args {
    /// Canonical matched-control panel to index.
    #[arg(long, default_value = "smoke")]
    panel: String,

    /// Optional JSON output path. If omitted, JSON is printed to stdout.
    #[arg(long)]
    out: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let panel = MatchedControlPanel::from_name(&args.panel)
        .ok_or("invalid --panel value; expected smoke or audit")?;

    if let Some(path) = &args.out {
        let output_path = absolute_output_path(path);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        write_matched_control_atlas_manifest_json(&output_path, panel)?;
        let manifest = build_matched_control_atlas_manifest(panel);
        println!("Wrote atlas manifest: {}", display_path(&output_path));
        println!("Panel: {} ({})", manifest.panel_id, manifest.panel);
        println!("Lanes: {}", manifest.lane_count);
    } else {
        let manifest = build_matched_control_atlas_manifest(panel);
        println!("{}", serde_json::to_string_pretty(&manifest)?);
    }

    Ok(())
}

fn absolute_output_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}

fn display_path(path: &Path) -> String {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.strip_prefix(&repo_root)
        .unwrap_or(path)
        .display()
        .to_string()
}
