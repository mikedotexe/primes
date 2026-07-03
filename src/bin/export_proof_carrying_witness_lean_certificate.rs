//! Export a proof-carrying witness certificate as a generated Lean module.

use clap::Parser;
use primes::validation::{
    reporting::{write_json_pretty, write_text_file},
    seed_to_witness::{
        build_proof_carrying_witness_lean_catalog_manifest,
        build_proof_carrying_witness_policy_matrix_lean_catalog_manifest,
        canonical_proof_carrying_witness_specs, proof_carrying_witness_lean_catalog_artifact,
        proof_carrying_witness_policy_matrix_lean_catalog_artifact,
        proof_carrying_witness_policy_matrix_lean_module_name,
        proof_carrying_witness_policy_matrix_promoted_specs,
        render_proof_carrying_witness_lean_module, verify_proof_carrying_witness_certificate,
        ProofCarryingWitnessCertificate, ProofCarryingWitnessPolicyMatrixSpec,
    },
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
    about = "Export proof-carrying witness certificate arithmetic as a Lean module"
)]
struct Args {
    /// Export generated Lean modules for every canonical witness manifest artifact.
    #[arg(long)]
    catalog: bool,

    /// Export generated Lean modules for promoted policy-matrix artifacts.
    #[arg(long)]
    policy_matrix_catalog: bool,

    /// Certificate JSON to turn into Lean arithmetic facts.
    #[arg(
        long,
        default_value = "docs/witness/teaching38_proof_carrying_witness.json"
    )]
    certificate: PathBuf,

    /// Directory containing canonical certificate JSON files in catalog mode.
    #[arg(long, default_value = "docs/witness")]
    certificate_dir: PathBuf,

    /// Output path under lean-proofs/PrimeArithmetic/Generated/.
    #[arg(
        long,
        default_value = "lean-proofs/PrimeArithmetic/Generated/Witness/Teaching38.lean"
    )]
    out: PathBuf,

    /// Output directory under lean-proofs/PrimeArithmetic/Generated/ in catalog mode.
    #[arg(long, default_value = "lean-proofs/PrimeArithmetic/Generated/Witness")]
    out_dir: PathBuf,

    /// Optional Lean catalog manifest JSON output path in catalog mode.
    #[arg(long)]
    manifest_out: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-proof-carrying-witness-lean-certificate error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    if args.catalog && args.policy_matrix_catalog {
        return Err("--catalog and --policy-matrix-catalog are mutually exclusive".into());
    }

    if args.catalog {
        return export_catalog(&repo_root, &args);
    }
    if args.policy_matrix_catalog {
        return export_policy_matrix_catalog(&repo_root, &args);
    }

    export_one(&repo_root, &args.certificate, &args.out).map(|_| ())
}

fn export_catalog(repo_root: &Path, args: &Args) -> Result<(), Box<dyn Error>> {
    let certificate_dir = absolute_path(repo_root, &args.certificate_dir);
    let out_dir = resolve_output_dir(repo_root, &args.out_dir)?;
    let mut manifest_artifacts = Vec::new();

    for spec in canonical_proof_carrying_witness_specs() {
        let certificate_path = certificate_dir.join(spec.file_name);
        let output_path = out_dir.join(format!("{}.lean", spec.lean_module_stem));
        let certificate = export_one(repo_root, &certificate_path, &output_path)?;
        manifest_artifacts.push(proof_carrying_witness_lean_catalog_artifact(
            &spec,
            &certificate,
        ));
    }

    if let Some(path) = &args.manifest_out {
        let output_path = absolute_path(repo_root, path);
        let manifest = build_proof_carrying_witness_lean_catalog_manifest(manifest_artifacts);
        write_json_pretty(&output_path, &manifest)?;
        println!(
            "Wrote Lean witness catalog manifest: {}",
            display_path(repo_root, &output_path)
        );
    }

    Ok(())
}

fn export_policy_matrix_catalog(repo_root: &Path, args: &Args) -> Result<(), Box<dyn Error>> {
    let certificate_dir = absolute_path(repo_root, &args.certificate_dir);
    let out_dir = resolve_output_dir(repo_root, &args.out_dir)?;
    let mut manifest_artifacts = Vec::new();

    for spec in proof_carrying_witness_policy_matrix_promoted_specs() {
        let certificate_path = certificate_dir.join(spec.file_name);
        let output_path = out_dir.join(format!(
            "{}.lean",
            proof_carrying_witness_policy_matrix_module_stem_for_export(&spec)?
        ));
        let certificate = export_one(repo_root, &certificate_path, &output_path)?;
        manifest_artifacts.push(
            proof_carrying_witness_policy_matrix_lean_catalog_artifact(&spec, &certificate)
                .expect("promoted matrix spec has Lean metadata"),
        );
    }

    if let Some(path) = &args.manifest_out {
        let output_path = absolute_path(repo_root, path);
        let manifest =
            build_proof_carrying_witness_policy_matrix_lean_catalog_manifest(manifest_artifacts);
        write_json_pretty(&output_path, &manifest)?;
        println!(
            "Wrote policy-matrix Lean witness catalog manifest: {}",
            display_path(repo_root, &output_path)
        );
    }

    Ok(())
}

fn export_one(
    repo_root: &Path,
    certificate: &Path,
    out: &Path,
) -> Result<ProofCarryingWitnessCertificate, Box<dyn Error>> {
    let certificate_path = absolute_path(repo_root, certificate);
    let output_path = resolve_output_path(repo_root, out)?;
    let module_name = lean_module_name(repo_root, &output_path)?;
    let source_certificate_path = display_path(repo_root, &certificate_path);
    let generated_by_command = format!(
        "cargo run --bin export_proof_carrying_witness_lean_certificate -- --certificate {} --out {}",
        display_path(repo_root, &certificate_path),
        display_path(repo_root, &output_path)
    );

    let certificate_text = fs::read_to_string(&certificate_path)?;
    let certificate: ProofCarryingWitnessCertificate = serde_json::from_str(&certificate_text)?;
    let verification = verify_proof_carrying_witness_certificate(&certificate);
    if !verification.ok {
        return Err(format!(
            "certificate failed verification before Lean export: {}",
            verification.failures.join("; ")
        )
        .into());
    }

    let lean = render_proof_carrying_witness_lean_module(
        &certificate,
        &module_name,
        &source_certificate_path,
        &generated_by_command,
    )?;
    write_text_file(&output_path, &lean)?;

    println!(
        "Wrote Lean witness certificate: {}",
        display_path(repo_root, &output_path)
    );
    println!("Module: {module_name}");
    println!("Source certificate: {source_certificate_path}");

    Ok(certificate)
}

fn proof_carrying_witness_policy_matrix_module_stem_for_export(
    spec: &ProofCarryingWitnessPolicyMatrixSpec,
) -> Result<String, Box<dyn Error>> {
    let module = proof_carrying_witness_policy_matrix_lean_module_name(spec)
        .ok_or("policy-matrix spec is not Lean-promoted")?;
    module
        .rsplit('.')
        .next()
        .map(str::to_string)
        .ok_or_else(|| "could not derive policy-matrix Lean module stem".into())
}

fn absolute_path(repo_root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        repo_root.join(path)
    }
}

fn resolve_output_path(repo_root: &Path, path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let absolute = absolute_path(repo_root, path);
    let generated_root = repo_root
        .join("lean-proofs")
        .join("PrimeArithmetic")
        .join("Generated");
    if !absolute.starts_with(&generated_root) {
        return Err(format!("output path must live under {}", generated_root.display()).into());
    }
    if absolute.extension().and_then(|ext| ext.to_str()) != Some("lean") {
        return Err("output path must end with .lean".into());
    }
    Ok(absolute)
}

fn resolve_output_dir(repo_root: &Path, path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let absolute = absolute_path(repo_root, path);
    let generated_root = repo_root
        .join("lean-proofs")
        .join("PrimeArithmetic")
        .join("Generated");
    if !absolute.starts_with(&generated_root) {
        return Err(format!(
            "output directory must live under {}",
            generated_root.display()
        )
        .into());
    }
    Ok(absolute)
}

fn lean_module_name(repo_root: &Path, path: &Path) -> Result<String, Box<dyn Error>> {
    let lean_root = repo_root.join("lean-proofs");
    let relative = path
        .strip_prefix(&lean_root)
        .map_err(|_| "output path is not inside lean-proofs")?;
    let without_ext = relative.with_extension("");
    let components: Vec<String> = without_ext
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect();
    if components.is_empty() {
        return Err("could not derive a Lean module name from the output path".into());
    }
    Ok(components.join("."))
}

fn display_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .display()
        .to_string()
}
