//! Special-form witness comparison report.
//!
//! Compares the Mersenne-prime genre (`p -> 2^p - 1`) with the Prime Witness
//! Engine genre (`seed origin -> affine membrane lane -> witness`). The point is
//! conceptual shape, not a claim that the proof or search methods are the same.

use num_bigint::BigUint;
use num_traits::One;
use primes::validation::{
    large_affine_witness::classify_mersenne,
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
    seed_to_witness::{find_seed_to_witness, SeedToWitnessConfig},
    timestamp_seed_policy::DEFAULT_TIMESTAMP_ANCHOR_SEED,
};
use serde::Serialize;
use std::{env, path::PathBuf};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_special_form_witness_comparison";
const ARTIFACT_ID: &str = "special_form_witness_comparison_report";
const EXPORT_VERSION: u32 = 1;

#[derive(Debug, Clone)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct ComparisonRow {
    family: String,
    example_label: String,
    descriptor: String,
    descriptor_length_chars: usize,
    input_label: String,
    output_digits: usize,
    decimal_value: String,
    confirmation: String,
    is_mersenne: bool,
    mersenne_exponent: Option<u64>,
    mersenne_class: String,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    row_count: usize,
    mersenne_rows: usize,
    affine_rows: usize,
    affine_non_mersenne_rows: usize,
    strongest_line: String,
    caution_line: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    summary: ReportSummary,
    rows: Vec<ComparisonRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");
    let rows = build_rows();
    let summary = build_summary(&rows);
    let report = render_report(&options, &summary, &rows);
    let bundle = ReportBundle {
        export_version: EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        summary,
        rows: rows.clone(),
    };

    write_text_file(options.out_dir.join("report.md"), &report).expect("write report");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_csv_rows(options.out_dir.join("comparison_rows.csv"), &rows).expect("write rows");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--release".to_string(),
                "--example".to_string(),
                "special_form_witness_comparison_report".to_string(),
            ],
            upstream_inputs: vec![
                "examples/special_form_witness_comparison_report.rs".to_string(),
                "src/validation/large_affine_witness.rs".to_string(),
                "src/validation/seed_to_witness.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "comparison_rows.csv".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )
    .expect("write manifest");

    println!(
        "wrote special-form witness comparison bundle to {}",
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

fn build_rows() -> Vec<ComparisonRow> {
    let mut rows = Vec::new();
    for exponent in [89u64, 127, 521, 1279, 2203] {
        rows.push(mersenne_row(exponent));
    }

    let affine_specs = [
        (
            "affine_full_middle_timestamp_29d",
            DEFAULT_TIMESTAMP_ANCHOR_SEED,
            29usize,
            512u64,
        ),
        ("affine_seed60_128d", 60, 128, 20_000),
        ("affine_seed60_512d", 60, 512, 20_000),
    ];
    for (label, seed, visible_digits, max_steps) in affine_specs {
        rows.push(affine_row(label, seed, visible_digits, max_steps));
    }
    rows
}

fn mersenne_row(exponent: u64) -> ComparisonRow {
    let value = (BigUint::one() << exponent as usize) - BigUint::one();
    let decimal_value = value.to_str_radix(10);
    let classification = classify_mersenne(&value);
    let descriptor = format!("p={exponent}; N=2^p-1");
    ComparisonRow {
        family: "mersenne_special_form".to_string(),
        example_label: format!("M{exponent}"),
        descriptor_length_chars: descriptor.len(),
        descriptor,
        input_label: exponent.to_string(),
        output_digits: decimal_value.len(),
        decimal_value,
        confirmation: "known_mersenne_prime_exponent_static_catalog".to_string(),
        is_mersenne: classification.is_mersenne,
        mersenne_exponent: classification.mersenne_exponent,
        mersenne_class: classification.mersenne_class,
        note: "Known Mersenne-prime exponent used as a special-form comparison row.".to_string(),
    }
}

fn affine_row(label: &str, seed: u64, visible_digits: usize, max_steps: u64) -> ComparisonRow {
    let result = find_seed_to_witness(
        SeedToWitnessConfig::default_for_seed(seed)
            .with_visible_digits(visible_digits)
            .with_max_steps(max_steps),
    )
    .expect("affine comparison witness should be found");
    let descriptor = format!(
        "base={}, pair=({},{}), k=({},{}), M={}, seed_origin={}, witness_seed={}",
        result.base,
        result.outer,
        result.inner,
        result.k_outer,
        result.k_inner,
        result.middle_length,
        result.input_seed,
        result.witness_seed
    );
    ComparisonRow {
        family: "affine_membrane_witness".to_string(),
        example_label: label.to_string(),
        descriptor_length_chars: descriptor.len(),
        descriptor,
        input_label: result.input_seed.to_string(),
        output_digits: result.decimal_digits,
        decimal_value: result.decimal_value,
        confirmation: result.confirmation,
        is_mersenne: result.is_mersenne,
        mersenne_exponent: result.mersenne_exponent,
        mersenne_class: result.mersenne_class,
        note: "Prime Witness Engine row: compact descriptor, affine lane, residue funnel, non-Mersenne structured witness.".to_string(),
    }
}

fn build_summary(rows: &[ComparisonRow]) -> ReportSummary {
    let mersenne_rows = rows
        .iter()
        .filter(|row| row.family == "mersenne_special_form")
        .count();
    let affine_rows = rows
        .iter()
        .filter(|row| row.family == "affine_membrane_witness")
        .count();
    let affine_non_mersenne_rows = rows
        .iter()
        .filter(|row| row.family == "affine_membrane_witness" && !row.is_mersenne)
        .count();
    ReportSummary {
        row_count: rows.len(),
        mersenne_rows,
        affine_rows,
        affine_non_mersenne_rows,
        strongest_line:
            "Both families turn a compact descriptor into a large prime-shaped witness; the affine rows do it through digit-template lanes and are explicitly not Mersenne."
                .to_string(),
        caution_line:
            "This is a genre comparison, not an equivalence of proof methods, record-search maturity, or density theory."
                .to_string(),
    }
}

fn render_report(options: &Options, summary: &ReportSummary, rows: &[ComparisonRow]) -> String {
    let mut lines = Vec::new();
    lines.push("# Special-Form Witness Comparison Report".to_string());
    lines.push(String::new());
    lines.push("## Frame".to_string());
    lines.push(format!("- {}", summary.strongest_line));
    lines.push(format!("- {}", summary.caution_line));
    lines.push(String::new());
    lines.push("## Settings".to_string());
    lines.push(format!("- output dir: `{}`", options.out_dir.display()));
    lines.push(format!(
        "- rows: `{}` total, `{}` Mersenne rows, `{}` affine rows",
        summary.row_count, summary.mersenne_rows, summary.affine_rows
    ));
    lines.push(String::new());
    lines.push("## Comparison Rows".to_string());
    lines.push(
        "| Family | Example | Descriptor | Digits | Confirmation | Mersenne class | Value |"
            .to_string(),
    );
    lines.push("|---|---|---|---:|---|---|---|".to_string());
    for row in rows {
        lines.push(format!(
            "| `{}` | `{}` | `{}` | {} | `{}` | `{}` | `{}` |",
            row.family,
            row.example_label,
            row.descriptor,
            row.output_digits,
            row.confirmation,
            row.mersenne_class,
            abbreviate(&row.decimal_value, 56)
        ));
    }
    lines.push(String::new());
    lines.push("## Reading".to_string());
    lines.push("- Mersenne rows are binary-repunit special forms: `p -> 2^p - 1`.".to_string());
    lines.push("- Affine rows are decimal membrane special forms: `seed origin -> A + G*s` inside a fixed visible lane.".to_string());
    lines.push("- The affine rows are marked `not_mersenne` by exact shape, while still preserving a compact descriptor and large witness output.".to_string());
    lines.push("- The strongest next proof question is whether the affine witnesses can gain certificate support comparable in spirit, not method, to special-form prime workflows.".to_string());
    lines.push(String::new());
    lines.push("## Non-Claims".to_string());
    lines.push("- This does not say affine membrane witnesses are Mersenne primes.".to_string());
    lines.push(
        "- This does not claim Lucas-Lehmer-style proof machinery for the affine family."
            .to_string(),
    );
    lines.push(
        "- This does not claim a density theorem or record-prime search program.".to_string(),
    );
    lines.push(String::new());
    lines.push("## Artifacts".to_string());
    lines.push("- `comparison_rows.csv`: row-level descriptors, values, confirmation labels, and Mersenne classifications.".to_string());
    lines.push("- `summary.json`: machine-readable bundle.".to_string());
    lines.push("- `artifact_manifest.json`: reproducibility sidecar.".to_string());
    lines.join("\n")
}

fn abbreviate(value: &str, max_chars: usize) -> String {
    if value.len() <= max_chars {
        value.to_string()
    } else {
        let keep = max_chars.saturating_sub(3) / 2;
        format!("{}...{}", &value[..keep], &value[value.len() - keep..])
    }
}
