//! Seed-to-witness demo report.
//!
//! Writes a compact bundle with the canonical 128-digit seed-to-witness
//! transcript plus a shorter teaching row.

use primes::validation::{
    large_affine_witness::PROBABLE_PRIME_BASES,
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
    seed_to_witness::{
        build_proof_carrying_witness_certificate, find_seed_to_witness,
        render_seed_to_witness_transcript, SeedToWitnessConfig, SeedToWitnessResult,
    },
};
use serde::Serialize;
use std::{env, path::PathBuf};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_seed_to_witness_demo";
const ARTIFACT_ID: &str = "seed_to_witness_demo_report";
const EXPORT_VERSION: u32 = 1;

#[derive(Debug, Clone)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    canonical_seed: u64,
    canonical_visible_digits: usize,
    teaching_seed: u64,
    teaching_visible_digits: usize,
}

#[derive(Debug, Clone, Serialize)]
struct WitnessCsvRow {
    role: String,
    input_seed: u64,
    witness_seed: u64,
    steps_to_witness: u64,
    visible_digits: usize,
    middle_length: usize,
    scanned_seed_count: u64,
    residue_survivor_count: u64,
    probable_prime_tests: u64,
    elapsed_seconds: f64,
    confirmation: String,
    is_mersenne: bool,
    mersenne_exponent: Option<u64>,
    mersenne_class: String,
    template_digits: String,
    decimal_value: String,
    compact_description: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    witness_count: usize,
    largest_visible_digits: usize,
    canonical_witness_seed: u64,
    canonical_steps_to_witness: u64,
    strongest_line: String,
    caution_line: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    summary: ReportSummary,
    witnesses: Vec<SeedToWitnessResult>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let canonical = find_seed_to_witness(SeedToWitnessConfig::default_for_seed(60))
        .expect("canonical seed-to-witness demo should find a witness");
    let canonical_certificate =
        build_proof_carrying_witness_certificate(&canonical, PROBABLE_PRIME_BASES);
    let teaching = find_seed_to_witness(
        SeedToWitnessConfig::default_for_seed(0)
            .with_visible_digits(38)
            .with_max_steps(100),
    )
    .expect("teaching seed-to-witness demo should find a witness");
    let witnesses = vec![canonical, teaching];
    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        canonical_seed: 60,
        canonical_visible_digits: 128,
        teaching_seed: 0,
        teaching_visible_digits: 38,
    };
    let summary = build_summary(&witnesses);
    let report = render_report(&settings, &summary, &witnesses);
    let transcript = render_transcript_bundle(&witnesses);
    let rows = build_witness_rows(&witnesses);
    let bundle = ReportBundle {
        export_version: EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        summary,
        witnesses,
    };

    write_text_file(options.out_dir.join("report.md"), &report).expect("write report");
    write_text_file(options.out_dir.join("transcript.md"), &transcript).expect("write transcript");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_json_pretty(
        options.out_dir.join("canonical_certificate.json"),
        &canonical_certificate,
    )
    .expect("write canonical certificate json");
    write_csv_rows(options.out_dir.join("witness_rows.csv"), &rows).expect("write witness rows");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--release".to_string(),
                "--example".to_string(),
                "seed_to_witness_demo_report".to_string(),
            ],
            upstream_inputs: vec![
                "examples/seed_to_witness_demo_report.rs".to_string(),
                "src/bin/seed-to-witness.rs".to_string(),
                "src/validation/seed_to_witness.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "canonical_certificate.json".to_string(),
                "transcript.md".to_string(),
                "witness_rows.csv".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote seed-to-witness demo bundle to {}",
        options.out_dir.display()
    );
    println!("{}", bundle.summary.strongest_line);
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = PathBuf::from(args.next().expect("missing --out-dir value")),
            _ => panic!("unrecognized argument: {arg}"),
        }
    }
    Options { out_dir }
}

fn build_summary(witnesses: &[SeedToWitnessResult]) -> ReportSummary {
    let canonical = witnesses
        .iter()
        .find(|witness| witness.visible_digits == 128)
        .expect("canonical witness");
    ReportSummary {
        witness_count: witnesses.len(),
        largest_visible_digits: witnesses
            .iter()
            .map(|witness| witness.visible_digits)
            .max()
            .unwrap_or_default(),
        canonical_witness_seed: canonical.witness_seed,
        canonical_steps_to_witness: canonical.steps_to_witness,
        strongest_line:
            "One seed origin, one named construction family, one large readable probable-prime witness."
                .to_string(),
        caution_line:
            "The seed is a start point, not a guarantee; above u64, the repo says probable-prime witness."
                .to_string(),
    }
}

fn build_witness_rows(witnesses: &[SeedToWitnessResult]) -> Vec<WitnessCsvRow> {
    witnesses
        .iter()
        .map(|witness| WitnessCsvRow {
            role: format!("{}d_seed_{}", witness.visible_digits, witness.input_seed),
            input_seed: witness.input_seed,
            witness_seed: witness.witness_seed,
            steps_to_witness: witness.steps_to_witness,
            visible_digits: witness.visible_digits,
            middle_length: witness.middle_length,
            scanned_seed_count: witness.scanned_seed_count,
            residue_survivor_count: witness.residue_survivor_count,
            probable_prime_tests: witness.probable_prime_tests,
            elapsed_seconds: witness.elapsed_seconds,
            confirmation: witness.confirmation.clone(),
            is_mersenne: witness.is_mersenne,
            mersenne_exponent: witness.mersenne_exponent,
            mersenne_class: witness.mersenne_class.clone(),
            template_digits: witness.template_digits.clone(),
            decimal_value: witness.decimal_value.clone(),
            compact_description: witness.compact_description.clone(),
        })
        .collect()
}

fn render_report(
    settings: &ReportSettings,
    summary: &ReportSummary,
    witnesses: &[SeedToWitnessResult],
) -> String {
    let mut lines = Vec::new();
    lines.push("# Seed To Witness Demo Report".to_string());
    lines.push(String::new());
    lines.push("## Frame".to_string());
    lines.push(format!("- {}", summary.strongest_line));
    lines.push(format!("- {}", summary.caution_line));
    lines.push(String::new());
    lines.push("## Settings".to_string());
    lines.push(format!("- output dir: `{}`", settings.out_dir));
    lines.push(format!(
        "- canonical: seed `{}`, visible digits `{}`",
        settings.canonical_seed, settings.canonical_visible_digits
    ));
    lines.push(format!(
        "- teaching row: seed `{}`, visible digits `{}`",
        settings.teaching_seed, settings.teaching_visible_digits
    ));
    lines.push(String::new());
    lines.push("## Witness Rows".to_string());
    lines.push("| Role | Input seed | Witness seed | Steps | Visible digits | Scanned | Survivors | Confirmation | Mersenne class |".to_string());
    lines.push("|---|---:|---:|---:|---:|---:|---:|---|---|".to_string());
    for witness in witnesses {
        lines.push(format!(
            "| `{}d_seed_{}` | {} | {} | {} | {} | {} | {} | `{}` | `{}` |",
            witness.visible_digits,
            witness.input_seed,
            witness.input_seed,
            witness.witness_seed,
            witness.steps_to_witness,
            witness.visible_digits,
            witness.scanned_seed_count,
            witness.residue_survivor_count,
            witness.confirmation,
            witness.mersenne_class
        ));
    }
    lines.push(String::new());
    lines.push("## Canonical Transcript".to_string());
    lines.push("See `transcript.md` for the full copyable transcript, including WolframAlpha, Mathematica, PARI/GP, and Sage checks.".to_string());
    lines.push(String::new());
    lines.push("## Artifacts".to_string());
    lines.push("- `transcript.md`: human-facing seed-to-witness transcript.".to_string());
    lines.push("- `canonical_certificate.json`: deterministic construction and residue-funnel certificate for the canonical seed-60 witness.".to_string());
    lines.push(
        "- `witness_rows.csv`: compact row export for the canonical and teaching witnesses."
            .to_string(),
    );
    lines.push("- `summary.json`: machine-readable bundle.".to_string());
    lines.push("- `artifact_manifest.json`: reproducibility sidecar.".to_string());
    lines.join("\n")
}

fn render_transcript_bundle(witnesses: &[SeedToWitnessResult]) -> String {
    witnesses
        .iter()
        .map(render_seed_to_witness_transcript)
        .collect::<Vec<_>>()
        .join("\n\n---\n\n")
}
