//! Verify the lightweight top-level signal catalog.

use clap::Parser;
use primes::validation::signal_catalog::{
    verify_signal_catalog, verify_signal_catalog_deep, SignalCatalog, SignalCatalogVerification,
};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process,
    time::Duration,
};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Verify that signal catalog rows point at known artifacts and drift gates"
)]
struct Args {
    /// Signal catalog JSON.
    #[arg(long, default_value = "docs/signal_catalog/signal_catalog.json")]
    catalog: PathBuf,

    /// Repository root used to resolve relative artifact paths.
    #[arg(long)]
    repo_root: Option<PathBuf>,

    /// Optional JSON output path for the verification result.
    #[arg(long)]
    json_out: Option<PathBuf>,

    /// Run each row's maintained drift gate after shallow verification passes.
    #[arg(long)]
    deep: bool,

    /// Per-row timeout for --deep drift gates.
    #[arg(long, default_value_t = 120)]
    timeout_seconds: u64,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("verify-signal-catalog error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let default_repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = args.repo_root.as_deref().unwrap_or(&default_repo_root);
    let catalog_path = absolute_path(repo_root, &args.catalog);
    let catalog_text = fs::read_to_string(&catalog_path)?;
    let catalog: SignalCatalog = serde_json::from_str(&catalog_text)?;
    if args.timeout_seconds == 0 {
        return Err("--timeout-seconds must be positive".into());
    }
    let verification = if args.deep {
        verify_signal_catalog_deep(
            &catalog,
            repo_root,
            Duration::from_secs(args.timeout_seconds),
        )
    } else {
        verify_signal_catalog(&catalog, repo_root)
    };

    if let Some(path) = &args.json_out {
        let output_path = absolute_path(repo_root, path);
        primes::validation::reporting::write_json_pretty(&output_path, &verification)?;
    }

    print_result(&verification);
    if verification.ok {
        Ok(())
    } else {
        let issue_count = verification.failures.len()
            + verification.gate_results.iter().filter(|r| !r.ok).count();
        Err(format!(
            "signal catalog verification failed with {} issue(s)",
            issue_count
        )
        .into())
    }
}

fn print_result(verification: &SignalCatalogVerification) {
    if verification.ok {
        println!(
            "Signal catalog verification passed: checked {} row(s)",
            verification.checked_rows
        );
        if !verification.gate_results.is_empty() {
            println!(
                "Deep drift gates passed: {} row gate(s)",
                verification.gate_results.len()
            );
        }
    } else {
        let failed_gate_count = verification
            .gate_results
            .iter()
            .filter(|result| !result.ok)
            .count();
        eprintln!(
            "Signal catalog verification failed: checked {} row(s), {} shallow issue(s), {} gate issue(s)",
            verification.checked_rows,
            verification.failures.len(),
            failed_gate_count
        );
        for failure in &verification.failures {
            eprintln!(
                "  - {}.{} = {:?}: {}",
                failure.signal_id, failure.field, failure.value, failure.message
            );
        }
        for result in &verification.gate_results {
            if !result.ok {
                eprintln!(
                    "  - {} gate {:?}: status={}, exit={:?}, timed_out={}, duration_ms={}, error={:?}",
                    result.signal_id,
                    result.drift_check_command,
                    result.status,
                    result.exit_code,
                    result.timed_out,
                    result.duration_ms,
                    result.error_message
                );
            }
        }
    }
}

fn absolute_path(repo_root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        repo_root.join(path)
    }
}
