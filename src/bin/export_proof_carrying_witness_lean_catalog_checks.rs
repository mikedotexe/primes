//! Export Lean declaration checks for the proof-carrying witness Lean catalog.

use clap::Parser;
use primes::validation::{
    reporting::write_text_file,
    seed_to_witness::{
        render_proof_carrying_witness_lean_catalog_check_shards,
        render_proof_carrying_witness_lean_catalog_checks, ProofCarryingWitnessLeanCatalogManifest,
    },
};
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
    process,
};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export silent Lean declaration checks from a witness Lean catalog manifest"
)]
struct Args {
    /// Witness Lean catalog manifest JSON.
    #[arg(
        long,
        default_value = "docs/witness/witness_lean_catalog_manifest.json"
    )]
    manifest: PathBuf,

    /// Optional Lean output path. If omitted, checks are printed to stdout.
    #[arg(long)]
    out: Option<PathBuf>,

    /// Optional number of manifest artifacts per generated check shard.
    #[arg(long)]
    shard_size: Option<usize>,

    /// Lean module prefix used by generated shard imports.
    #[arg(long, default_value = "PrimeArithmetic.Generated.Witness")]
    module_prefix: String,

    /// Optional directory for shard files. Defaults to the parent of --out.
    #[arg(long)]
    shard_out_dir: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-proof-carrying-witness-lean-catalog-checks error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = absolute_path(&repo_root, &args.manifest);
    let manifest_text = fs::read_to_string(&manifest_path)?;
    let manifest: ProofCarryingWitnessLeanCatalogManifest = serde_json::from_str(&manifest_text)?;

    if let Some(shard_size) = args.shard_size {
        let Some(path) = &args.out else {
            return Err("--shard-size requires --out".into());
        };
        let output_path = absolute_path(&repo_root, path);
        let umbrella_stem = output_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "--out must have a UTF-8 file stem when --shard-size is set",
                )
            })?;
        let shard_out_dir = args
            .shard_out_dir
            .as_deref()
            .map(|path| absolute_path(&repo_root, path))
            .unwrap_or_else(|| {
                output_path
                    .parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|| repo_root.clone())
            });
        let bundle = render_proof_carrying_witness_lean_catalog_check_shards(
            &manifest,
            &args.module_prefix,
            umbrella_stem,
            shard_size,
        )
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        write_text_file(&output_path, &bundle.umbrella_contents)?;
        for shard in &bundle.shards {
            write_text_file(shard_out_dir.join(&shard.file_name), &shard.contents)?;
        }
        println!(
            "Wrote witness Lean catalog check umbrella: {}",
            display_path(&repo_root, &output_path)
        );
        println!(
            "Wrote {} witness Lean catalog check shard(s) under {}",
            bundle.shards.len(),
            display_path(&repo_root, &shard_out_dir)
        );
    } else {
        let checks = render_proof_carrying_witness_lean_catalog_checks(&manifest);
        if let Some(path) = &args.out {
            let output_path = absolute_path(&repo_root, path);
            write_text_file(&output_path, &checks)?;
            println!(
                "Wrote witness Lean catalog checks: {}",
                display_path(&repo_root, &output_path)
            );
        } else {
            println!("{checks}");
        }
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
