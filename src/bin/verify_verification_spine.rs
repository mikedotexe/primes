//! Verification spine checker for maintained generated, report, and visual lanes.

use clap::{Parser, Subcommand, ValueEnum};
use primes::validation::reporting::ArtifactManifest;
use serde::Deserialize;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

const DEFAULT_CATALOG_PATH: &str = "tools/verification_spine.toml";

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Check or regenerate the maintained verification spine"
)]
struct Cli {
    #[command(subcommand)]
    command: CommandKind,

    /// Optional spine entry ids to target.
    #[arg(long = "id")]
    ids: Vec<String>,

    /// Override catalog path.
    #[arg(long, default_value = DEFAULT_CATALOG_PATH)]
    catalog: PathBuf,
}

#[derive(Subcommand, Debug)]
enum CommandKind {
    /// Validate all selected spine entries.
    Check,
    /// Regenerate the selected formal-generated entries in place.
    Regenerate,
}

#[derive(Debug, Deserialize)]
struct SpineCatalog {
    #[serde(rename = "entry")]
    entries: Vec<SpineEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct SpineEntry {
    id: String,
    tier: SpineTier,
    verify_mode: VerifyMode,
    generator_cmd: Vec<String>,
    #[serde(default)]
    tracked_outputs: Vec<String>,
    bundle_dir: Option<String>,
    #[serde(default)]
    expected_outputs: Vec<String>,
    owner_doc: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
enum SpineTier {
    FormalGenerated,
    ReportBundle,
    VisualBundle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum VerifyMode {
    Diff,
    BundleManifest,
    UpstreamOnly,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let catalog = load_catalog(&repo_root.join(&cli.catalog))?;
    let entries = select_entries(&catalog.entries, &cli.ids)?;

    match cli.command {
        CommandKind::Check => {
            for entry in entries {
                check_entry(entry, &repo_root)?;
            }
            println!("Verification spine check passed.");
        }
        CommandKind::Regenerate => {
            for entry in entries {
                regenerate_entry(entry, &repo_root)?;
            }
            println!("Verification spine regenerate passed.");
        }
    }

    Ok(())
}

fn load_catalog(path: &Path) -> Result<SpineCatalog, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    let catalog = toml::from_str::<SpineCatalog>(&text)?;
    Ok(catalog)
}

fn select_entries<'a>(
    entries: &'a [SpineEntry],
    ids: &[String],
) -> Result<Vec<&'a SpineEntry>, Box<dyn std::error::Error>> {
    if ids.is_empty() {
        return Ok(entries.iter().collect());
    }

    let mut selected = Vec::with_capacity(ids.len());
    for id in ids {
        let entry = entries
            .iter()
            .find(|entry| &entry.id == id)
            .ok_or_else(|| format!("unknown verification spine entry id: {id}"))?;
        selected.push(entry);
    }
    Ok(selected)
}

fn check_entry(entry: &SpineEntry, repo_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking spine entry: {}", entry.id);
    validate_entry_shape(entry, repo_root)?;

    match entry.verify_mode {
        VerifyMode::Diff => {
            ensure_paths_exist(repo_root, &entry.tracked_outputs, "tracked output")?;
            run_generator(entry, repo_root, Some("verify"))?;
            ensure_paths_exist(repo_root, &entry.tracked_outputs, "tracked output")?;
        }
        VerifyMode::BundleManifest => {
            run_generator(entry, repo_root, None)?;
            validate_bundle_manifest(entry, repo_root)?;
        }
        VerifyMode::UpstreamOnly => {
            run_generator(entry, repo_root, None)?;
        }
    }

    Ok(())
}

fn regenerate_entry(
    entry: &SpineEntry,
    repo_root: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Regenerating spine entry: {}", entry.id);
    validate_entry_shape(entry, repo_root)?;

    if entry.tier != SpineTier::FormalGenerated || entry.verify_mode != VerifyMode::Diff {
        return Err(format!(
            "regenerate is only supported for formal-generated diff entries, but `{}` is {:?}/{:?}",
            entry.id, entry.tier, entry.verify_mode
        )
        .into());
    }

    run_generator(entry, repo_root, Some("regenerate"))?;
    ensure_paths_exist(repo_root, &entry.tracked_outputs, "tracked output")?;
    Ok(())
}

fn validate_entry_shape(
    entry: &SpineEntry,
    repo_root: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if entry.generator_cmd.is_empty() {
        return Err(format!("spine entry `{}` has an empty generator_cmd", entry.id).into());
    }
    ensure_paths_exist(
        repo_root,
        std::slice::from_ref(&entry.owner_doc),
        "owner doc",
    )?;

    match entry.tier {
        SpineTier::FormalGenerated => {
            if entry.verify_mode != VerifyMode::Diff {
                return Err(format!(
                    "formal-generated entry `{}` must use verify_mode = diff",
                    entry.id
                )
                .into());
            }
            if entry.tracked_outputs.is_empty() {
                return Err(format!(
                    "formal-generated entry `{}` must declare tracked_outputs",
                    entry.id
                )
                .into());
            }
        }
        SpineTier::ReportBundle | SpineTier::VisualBundle => {
            if entry.verify_mode != VerifyMode::BundleManifest
                && entry.verify_mode != VerifyMode::UpstreamOnly
            {
                return Err(format!(
                    "bundle entry `{}` must use bundle_manifest or upstream_only verification",
                    entry.id
                )
                .into());
            }
            if entry.bundle_dir.is_none() {
                return Err(format!("bundle entry `{}` must declare bundle_dir", entry.id).into());
            }
            if entry.verify_mode == VerifyMode::BundleManifest && entry.expected_outputs.is_empty()
            {
                return Err(format!(
                    "bundle entry `{}` must declare expected_outputs for bundle_manifest verification",
                    entry.id
                )
                .into());
            }
        }
    }

    Ok(())
}

fn ensure_paths_exist(
    repo_root: &Path,
    paths: &[String],
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for relative in paths {
        let path = repo_root.join(relative);
        if !path.exists() {
            return Err(format!("missing {label}: {}", path.display()).into());
        }
    }
    Ok(())
}

fn run_generator(
    entry: &SpineEntry,
    repo_root: &Path,
    mode: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = build_command(&entry.generator_cmd, repo_root)?;
    if let Some(mode) = mode {
        command.arg(mode);
    }
    let status = command.status()?;
    if !status.success() {
        return Err(command_failure(entry, status).into());
    }
    Ok(())
}

fn build_command(
    generator_cmd: &[String],
    repo_root: &Path,
) -> Result<Command, Box<dyn std::error::Error>> {
    let program = generator_cmd
        .first()
        .ok_or("generator_cmd must not be empty")?;
    let mut command = if program.contains('/') || program.starts_with('.') {
        Command::new(repo_root.join(program))
    } else {
        Command::new(program)
    };
    command.current_dir(repo_root);
    for arg in generator_cmd.iter().skip(1) {
        command.arg(arg);
    }
    Ok(command)
}

fn command_failure(entry: &SpineEntry, status: ExitStatus) -> String {
    format!(
        "generator for spine entry `{}` failed with status {status}",
        entry.id
    )
}

fn validate_bundle_manifest(
    entry: &SpineEntry,
    repo_root: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let bundle_dir = PathBuf::from(
        entry
            .bundle_dir
            .as_ref()
            .expect("bundle_dir should be validated before use"),
    );
    let manifest_path = bundle_dir.join("artifact_manifest.json");
    if !manifest_path.exists() {
        return Err(format!(
            "bundle entry `{}` did not produce {}",
            entry.id,
            manifest_path.display()
        )
        .into());
    }

    let manifest = serde_json::from_str::<ArtifactManifest>(&fs::read_to_string(&manifest_path)?)?;
    if manifest.artifact_id != entry.id {
        return Err(format!(
            "bundle manifest id mismatch for `{}`: expected `{}`, found `{}`",
            entry.id, entry.id, manifest.artifact_id
        )
        .into());
    }

    let manifest_outputs = manifest
        .expected_outputs
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected_outputs = entry
        .expected_outputs
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if manifest_outputs != expected_outputs {
        return Err(format!("bundle manifest outputs mismatch for `{}`", entry.id).into());
    }

    for output in &entry.expected_outputs {
        let output_path = bundle_dir.join(output);
        if !output_path.exists() {
            return Err(format!(
                "bundle entry `{}` is missing expected output {}",
                entry.id,
                output_path.display()
            )
            .into());
        }
    }

    for upstream in &manifest.upstream_inputs {
        let upstream_path = resolve_manifest_path(repo_root, upstream);
        if !upstream_path.exists() {
            return Err(format!(
                "bundle entry `{}` is missing upstream input {}",
                entry.id,
                upstream_path.display()
            )
            .into());
        }
    }

    Ok(())
}

fn resolve_manifest_path(repo_root: &Path, raw: &str) -> PathBuf {
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        path
    } else {
        repo_root.join(path)
    }
}
