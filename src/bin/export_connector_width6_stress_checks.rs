//! Export Lean declaration checks for connector width-6 stress theorem links.

use clap::Parser;
use primes::validation::{
    connector_signal::{render_connector_width6_stress_lean_checks, ConnectorWidth6StressReport},
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
    about = "Export Lean declaration checks from connector_width6_stress.json theorem links"
)]
struct Args {
    /// Connector width-6 stress JSON.
    #[arg(long, default_value = "docs/connector/connector_width6_stress.json")]
    stress: PathBuf,

    /// Optional Lean output path. If omitted, checks are printed to stdout.
    #[arg(long)]
    out: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-connector-width6-stress-checks error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let stress_path = absolute_path(&repo_root, &args.stress);
    let stress_text = fs::read_to_string(&stress_path)?;
    let report: ConnectorWidth6StressReport = serde_json::from_str(&stress_text)?;
    let checks = render_connector_width6_stress_lean_checks(&report)?;

    if let Some(path) = &args.out {
        let output_path = absolute_path(&repo_root, path);
        write_text_file(&output_path, &checks)?;
        println!(
            "Wrote connector width-6 stress Lean checks: {}",
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
