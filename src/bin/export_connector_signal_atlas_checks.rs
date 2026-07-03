//! Export Lean import checks for connector signal atlas proof links.

use clap::Parser;
use primes::validation::{
    connector_signal::{render_connector_signal_atlas_lean_checks, ConnectorSignalAtlas},
    reporting::write_text_file,
};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process,
};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export Lean import checks from connector_signal_atlas.json proof links"
)]
struct Args {
    /// Connector signal atlas JSON.
    #[arg(long, default_value = "docs/connector/connector_signal_atlas.json")]
    atlas: PathBuf,

    /// Optional Lean output path. If omitted, checks are printed to stdout.
    #[arg(long)]
    out: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-connector-signal-atlas-checks error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let atlas_path = absolute_path(&repo_root, &args.atlas);
    let atlas_text = fs::read_to_string(&atlas_path)?;
    let atlas: ConnectorSignalAtlas = serde_json::from_str(&atlas_text)?;
    let checks = render_connector_signal_atlas_lean_checks(&atlas)?;

    if let Some(path) = &args.out {
        let output_path = absolute_path(&repo_root, path);
        write_text_file(&output_path, &checks)?;
        println!(
            "Wrote connector signal atlas Lean checks: {}",
            display_path(&repo_root, &output_path)
        );
    } else {
        println!("{checks}");
    }

    Ok(())
}

fn absolute_path(repo_root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        repo_root.join(path)
    }
}

fn display_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .display()
        .to_string()
}
