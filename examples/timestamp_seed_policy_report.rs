//! Timestamp seed policy report.
//!
//! Defines and measures bounded policies for turning timestamp-like seed
//! origins into nearby affine membrane probable-prime witnesses.

use primes::validation::{
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
    timestamp_seed_policy::{
        build_timestamp_seed_policy_report, default_timestamp_policy_settings,
        TimestampPolicySummaryRow, TimestampPolicyTrialRow,
    },
};
use serde::Serialize;
use std::{env, path::PathBuf};

const DEFAULT_OUT_DIR: &str = "/tmp/primes_timestamp_seed_policy";
const ARTIFACT_ID: &str = "timestamp_seed_policy_report";
const EXPORT_VERSION: u32 = 1;

#[derive(Debug, Clone)]
struct Options {
    out_dir: PathBuf,
    profile: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    profile: String,
    policy_rows: Vec<TimestampPolicySummaryRow>,
    trial_rows: Vec<TimestampPolicyTrialRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");
    let settings = default_timestamp_policy_settings(&options.profile);
    let data = build_timestamp_seed_policy_report(settings);
    let report = render_report(&options, &data.policy_rows);
    let bundle = ReportBundle {
        export_version: EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        profile: options.profile.clone(),
        policy_rows: data.policy_rows.clone(),
        trial_rows: data.trial_rows.clone(),
    };

    write_text_file(options.out_dir.join("report.md"), &report).expect("write report");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle).expect("write summary json");
    write_csv_rows(options.out_dir.join("policy_rows.csv"), &data.policy_rows)
        .expect("write policy rows");
    write_csv_rows(options.out_dir.join("trial_rows.csv"), &data.trial_rows)
        .expect("write trial rows");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--release".to_string(),
                "--example".to_string(),
                "timestamp_seed_policy_report".to_string(),
            ],
            upstream_inputs: vec![
                "examples/timestamp_seed_policy_report.rs".to_string(),
                "src/validation/timestamp_seed_policy.rs".to_string(),
                "src/validation/seed_to_witness.rs".to_string(),
            ],
            expected_outputs: vec![
                "report.md".to_string(),
                "summary.json".to_string(),
                "policy_rows.csv".to_string(),
                "trial_rows.csv".to_string(),
                "artifact_manifest.json".to_string(),
            ],
        },
    )
    .expect("write artifact manifest");

    println!(
        "wrote timestamp seed policy bundle to {}",
        options.out_dir.display()
    );
    for row in &data.policy_rows {
        println!(
            "{}: {} successes / {} trials within {} steps (p95={}, max={})",
            row.policy_label,
            row.successes,
            row.sample_count,
            row.max_steps,
            row.p95_steps,
            row.max_steps_observed
        );
    }
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut profile = "release".to_string();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = PathBuf::from(args.next().expect("missing --out-dir value")),
            "--profile" => profile = args.next().expect("missing --profile value"),
            _ => panic!("unrecognized argument: {arg}"),
        }
    }
    assert!(
        profile == "smoke" || profile == "release",
        "--profile must be smoke or release"
    );
    Options { out_dir, profile }
}

fn render_report(options: &Options, rows: &[TimestampPolicySummaryRow]) -> String {
    let mut lines = Vec::new();
    lines.push("# Timestamp Seed Policy Report".to_string());
    lines.push(String::new());
    lines.push("## Policy".to_string());
    lines.push(
        "- Treat a nanosecond timestamp as a seed origin, not as a guaranteed prime seed."
            .to_string(),
    );
    lines.push("- Walk forward on the default decimal affine membrane lane: `base=10, pair=(3,7), k=(2,1)`.".to_string());
    lines.push(
        "- Apply exact small-prime residue filters before probable-prime confirmation.".to_string(),
    );
    lines.push(
        "- Declare success only if a witness appears within the policy's max-step budget."
            .to_string(),
    );
    lines.push(
        "- This is a bounded empirical statement, not a theorem and not a density claim."
            .to_string(),
    );
    lines.push(String::new());
    lines.push("## Settings".to_string());
    lines.push(format!("- profile: `{}`", options.profile));
    lines.push(format!("- output dir: `{}`", options.out_dir.display()));
    lines.push(String::new());
    lines.push("## Results".to_string());
    lines.push("| Policy | Visible digits | Max steps | Trials | Successes | Success rate | Median steps | P95 steps | P99 steps | Max observed | Mean tests |".to_string());
    lines.push("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|".to_string());
    for row in rows {
        lines.push(format!(
            "| `{}` | {} | {} | {} | {} | {:.2}% | {} | {} | {} | {} | {:.1} |",
            row.policy_label,
            row.visible_digits,
            row.max_steps,
            row.sample_count,
            row.successes,
            row.success_rate * 100.0,
            row.median_steps,
            row.p95_steps,
            row.p99_steps,
            row.max_steps_observed,
            row.mean_probable_prime_tests
        ));
    }
    lines.push(String::new());
    lines.push("## Bounded Statements".to_string());
    for row in rows {
        lines.push(format!(
            "- `{}`: {}",
            row.policy_label, row.bounded_statement
        ));
    }
    lines.push(String::new());
    lines.push("## Reading".to_string());
    lines.push("- For 29 visible digits, the timestamp-scale seed fills the whole middle slot, so the witness visibly carries the timestamp-like center.".to_string());
    lines.push("- For 128 visible digits, the timestamp is a seed origin inside a much larger chamber; this tests the large-witness demo behavior rather than the full-middle visual.".to_string());
    lines.push("- A miss would be useful signal: it would tell us the step budget is too tight for that policy surface.".to_string());
    lines.push(String::new());
    lines.push("## Artifacts".to_string());
    lines.push("- `policy_rows.csv`: one row per bounded policy.".to_string());
    lines.push("- `trial_rows.csv`: one row per sampled timestamp-like seed origin.".to_string());
    lines.push("- `summary.json`: complete machine-readable bundle.".to_string());
    lines.push("- `artifact_manifest.json`: reproducibility sidecar.".to_string());
    lines.join("\n")
}
