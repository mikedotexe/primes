//! Export a deterministic proof-carrying witness policy matrix.

use clap::Parser;
use primes::validation::{
    reporting::{
        ensure_dir, write_artifact_manifest, write_json_pretty, write_text_file, ArtifactManifest,
    },
    seed_to_witness::{
        build_proof_carrying_witness_certificate_for_config,
        build_proof_carrying_witness_policy_matrix_atlas,
        build_proof_carrying_witness_policy_matrix_report,
        proof_carrying_witness_policy_matrix_row, proof_carrying_witness_policy_matrix_smoke_specs,
        render_proof_carrying_witness_policy_matrix_atlas_markdown,
        render_proof_carrying_witness_policy_matrix_markdown,
        verify_proof_carrying_witness_certificate,
    },
};
use std::{error::Error, path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export deterministic proof-carrying witness policy-matrix certificates and atlas rows"
)]
struct Args {
    /// Output directory for the matrix report and certificate candidates.
    #[arg(long, default_value = "/tmp/proof-carrying-witness-policy-matrix")]
    out_dir: PathBuf,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("export-proof-carrying-witness-policy-matrix error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let certificate_dir = args.out_dir.join("certificates");
    ensure_dir(&certificate_dir)?;

    let mut rows = Vec::new();
    let mut expected_outputs = vec![
        "witness_policy_matrix.json".to_string(),
        "witness_policy_matrix.md".to_string(),
        "witness_policy_matrix_atlas.json".to_string(),
        "witness_policy_matrix_atlas.md".to_string(),
        "artifact_manifest.json".to_string(),
    ];

    for spec in proof_carrying_witness_policy_matrix_smoke_specs() {
        let certificate = build_proof_carrying_witness_certificate_for_config(spec.config.clone())?;
        let verification = verify_proof_carrying_witness_certificate(&certificate);
        if !verification.ok {
            return Err(format!(
                "policy-matrix certificate failed verification: {}",
                verification.failures.join("; ")
            )
            .into());
        }

        let relative_certificate_path = format!("certificates/{}", spec.file_name);
        let certificate_path = args.out_dir.join(&relative_certificate_path);
        write_json_pretty(&certificate_path, &certificate)?;
        expected_outputs.push(relative_certificate_path.clone());
        rows.push(proof_carrying_witness_policy_matrix_row(
            &spec,
            &certificate,
            relative_certificate_path,
        ));
    }

    let report = build_proof_carrying_witness_policy_matrix_report(rows);
    let atlas = build_proof_carrying_witness_policy_matrix_atlas(&report);
    let markdown = render_proof_carrying_witness_policy_matrix_markdown(&report);
    let atlas_markdown = render_proof_carrying_witness_policy_matrix_atlas_markdown(&atlas);
    write_json_pretty(args.out_dir.join("witness_policy_matrix.json"), &report)?;
    write_text_file(args.out_dir.join("witness_policy_matrix.md"), &markdown)?;
    write_json_pretty(
        args.out_dir.join("witness_policy_matrix_atlas.json"),
        &atlas,
    )?;
    write_text_file(
        args.out_dir.join("witness_policy_matrix_atlas.md"),
        &atlas_markdown,
    )?;
    write_artifact_manifest(
        &args.out_dir,
        &ArtifactManifest {
            artifact_id: report.matrix_id.clone(),
            generator_cmd: "cargo run --bin export_proof_carrying_witness_policy_matrix"
                .to_string(),
            args: vec!["--out-dir".to_string(), args.out_dir.display().to_string()],
            upstream_inputs: Vec::new(),
            expected_outputs,
        },
    )?;

    println!(
        "wrote policy matrix: {}",
        args.out_dir.join("witness_policy_matrix.json").display()
    );
    println!(
        "rows={} lanes={} matrix_lean_promoted={} small_lean_candidates={} next_replay_target={}",
        report.summary.row_count,
        report.summary.lane_count,
        report.summary.matrix_lean_promoted_count,
        report.summary.small_lean_candidate_count,
        atlas.next_replay_target.reason
    );

    Ok(())
}
