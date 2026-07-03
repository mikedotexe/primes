//! Summarize many matched-control comparison JSON exports.
//!
//! Inputs are comparison exports produced by `membrane_vs_random_compare`, not
//! raw matched-control report exports.
//!
//! Example:
//! `cargo run --example membrane_vs_random_compare_batch -- diff-a.json diff-b.json --out-dir /tmp/matched-control-batch`

use primes::validation::{
    matched_control::{
        read_comparison_json_export, summarize_comparison_batch, MatchedControlAuditSeverityTally,
        MatchedControlBatchFamilyStatus, MatchedControlComparisonBatchFamilyRow,
        MatchedControlComparisonBatchInput, MatchedControlComparisonBatchSummary,
    },
    reporting::{
        ensure_dir, write_artifact_manifest, write_csv_rows, write_json_pretty, write_text_file,
        ArtifactManifest,
    },
};
use std::{cmp::Ordering, env, path::PathBuf};

fn main() {
    let options = parse_args();
    let inputs = options
        .input_paths
        .iter()
        .map(|path| {
            let bundle = read_comparison_json_export(path).unwrap_or_else(|err| {
                eprintln!("Failed to read comparison JSON {}: {err}", path.display());
                std::process::exit(1);
            });
            MatchedControlComparisonBatchInput {
                source_path: path.display().to_string(),
                bundle,
            }
        })
        .collect::<Vec<_>>();
    let summary = summarize_comparison_batch(&inputs).unwrap_or_else(|err| {
        eprintln!("Failed to summarize comparison batch: {err}");
        std::process::exit(1);
    });

    println!("Matched-Control Comparison Batch");
    println!("{}", "=".repeat(88));
    println!("Panel: {}", format_panel(summary.panel_id.as_deref()));
    println!("Runs: {}", summary.run_count);
    println!("Flagged runs: {}", summary.flagged_run_count);
    println!(
        "Residual criterion flips: {}",
        summary.residual_criterion_flip_count
    );
    println!(
        "Audit severities: residual={} material={} sampling={} added={} removed={}",
        format_tally(summary.condition_tallies.residual_criterion_changed),
        format_tally(summary.condition_tallies.material_family_change),
        format_tally(summary.condition_tallies.sampling_plan_drift),
        format_tally(summary.condition_tallies.added_families),
        format_tally(summary.condition_tallies.removed_families),
    );
    println!(
        "Families: {} stable, {} drifting",
        summary.stable_family_count, summary.drifting_family_count
    );

    let mut drifting = summary
        .family_rows
        .iter()
        .filter(|row| row.status == MatchedControlBatchFamilyStatus::Drifting)
        .collect::<Vec<_>>();
    drifting.sort_by(compare_family_rows_for_rank);

    println!();
    println!("Ranked Drifting Families");
    println!("{}", "-".repeat(88));
    if drifting.is_empty() {
        println!("No family drifted across the supplied comparison exports.");
    } else {
        println!(
            "{:<28} {:>9} {:>9} {:>12} {:>12}",
            "family", "material", "decision", "max lift", "max q"
        );
        println!("{}", "-".repeat(88));
        for row in drifting.iter().take(10) {
            println!(
                "{:<28} {:>9} {:>9} {:>12} {:>12}",
                row.family_code,
                row.material_change_count,
                row.decision_change_count,
                format_optional_delta(row.max_abs_lift_delta),
                format_optional_delta(row.max_abs_q_delta),
            );
        }
    }

    if let Some(out_dir) = &options.out_dir {
        ensure_dir(out_dir).unwrap_or_else(|err| {
            eprintln!(
                "Failed to create output directory {}: {err}",
                out_dir.display()
            );
            std::process::exit(1);
        });
        write_json_pretty(out_dir.join("summary.json"), &summary).unwrap_or_else(|err| {
            eprintln!("Failed to write summary JSON: {err}");
            std::process::exit(1);
        });
        write_csv_rows(out_dir.join("run_rows.csv"), &summary.run_rows).unwrap_or_else(|err| {
            eprintln!("Failed to write run rows CSV: {err}");
            std::process::exit(1);
        });
        write_csv_rows(out_dir.join("family_rows.csv"), &summary.family_rows).unwrap_or_else(
            |err| {
                eprintln!("Failed to write family rows CSV: {err}");
                std::process::exit(1);
            },
        );
        write_text_file(
            out_dir.join("summary.md"),
            &render_summary_markdown(&summary),
        )
        .unwrap_or_else(|err| {
            eprintln!("Failed to write Markdown summary: {err}");
            std::process::exit(1);
        });
        write_artifact_manifest(
            out_dir,
            &ArtifactManifest {
                artifact_id: "matched_control_comparison_batch".to_string(),
                generator_cmd: "cargo run --example membrane_vs_random_compare_batch".to_string(),
                args: options.args.clone(),
                upstream_inputs: options
                    .input_paths
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect(),
                expected_outputs: vec![
                    "summary.json".to_string(),
                    "run_rows.csv".to_string(),
                    "family_rows.csv".to_string(),
                    "summary.md".to_string(),
                    "artifact_manifest.json".to_string(),
                ],
            },
        )
        .unwrap_or_else(|err| {
            eprintln!("Failed to write artifact manifest: {err}");
            std::process::exit(1);
        });
        println!();
        println!("Batch artifacts: {}", out_dir.display());
    }
}

struct CliOptions {
    input_paths: Vec<PathBuf>,
    out_dir: Option<PathBuf>,
    args: Vec<String>,
}

fn parse_args() -> CliOptions {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let mut input_paths = Vec::new();
    let mut out_dir = None;
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--out-dir" => {
                let value = iter.next().unwrap_or_else(|| {
                    eprintln!("Missing value for --out-dir");
                    std::process::exit(2);
                });
                out_dir = Some(PathBuf::from(value));
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ if arg.starts_with("--") => {
                eprintln!("Unknown argument: {arg}");
                print_help();
                std::process::exit(2);
            }
            _ => input_paths.push(PathBuf::from(arg)),
        }
    }

    if input_paths.is_empty() {
        eprintln!("Provide at least one comparison JSON export");
        print_help();
        std::process::exit(2);
    }

    CliOptions {
        input_paths,
        out_dir,
        args,
    }
}

fn print_help() {
    println!("Summarize matched-control comparison JSON exports");
    println!();
    println!("Usage:");
    println!(
        "  cargo run --example membrane_vs_random_compare_batch -- <comparison.json>... [options]"
    );
    println!();
    println!("Options:");
    println!("  --out-dir <path>      Write summary.json, CSV rows, summary.md, and manifest");
}

fn format_panel(panel_id: Option<&str>) -> &str {
    panel_id.unwrap_or("manual")
}

fn format_tally(tally: MatchedControlAuditSeverityTally) -> String {
    format!(
        "clear:{},info:{},error:{}",
        tally.clear, tally.info, tally.error
    )
}

fn format_optional_delta(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.3}"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn compare_family_rows_for_rank(
    left: &&MatchedControlComparisonBatchFamilyRow,
    right: &&MatchedControlComparisonBatchFamilyRow,
) -> Ordering {
    right
        .material_change_count
        .cmp(&left.material_change_count)
        .then_with(|| right.decision_change_count.cmp(&left.decision_change_count))
        .then_with(|| {
            right
                .max_abs_lift_delta
                .partial_cmp(&left.max_abs_lift_delta)
                .unwrap_or(Ordering::Equal)
        })
        .then_with(|| left.family_code.cmp(&right.family_code))
}

fn render_summary_markdown(summary: &MatchedControlComparisonBatchSummary) -> String {
    let mut drifting = summary
        .family_rows
        .iter()
        .filter(|row| row.status == MatchedControlBatchFamilyStatus::Drifting)
        .collect::<Vec<_>>();
    drifting.sort_by(compare_family_rows_for_rank);

    let mut text = String::new();
    text.push_str("# Matched-Control Comparison Batch\n\n");
    text.push_str(&format!(
        "- Panel: {}\n",
        format_panel(summary.panel_id.as_deref())
    ));
    text.push_str(&format!("- Runs: {}\n", summary.run_count));
    text.push_str(&format!("- Flagged runs: {}\n", summary.flagged_run_count));
    text.push_str(&format!(
        "- Residual criterion flips: {}\n",
        summary.residual_criterion_flip_count
    ));
    text.push_str(&format!(
        "- Families: {} stable, {} drifting\n\n",
        summary.stable_family_count, summary.drifting_family_count
    ));
    text.push_str("## Audit Severities\n\n");
    text.push_str(&format!(
        "- Residual criterion changed: {}\n",
        format_tally(summary.condition_tallies.residual_criterion_changed)
    ));
    text.push_str(&format!(
        "- Material family change: {}\n",
        format_tally(summary.condition_tallies.material_family_change)
    ));
    text.push_str(&format!(
        "- Sampling plan drift: {}\n",
        format_tally(summary.condition_tallies.sampling_plan_drift)
    ));
    text.push_str(&format!(
        "- Added families: {}\n",
        format_tally(summary.condition_tallies.added_families)
    ));
    text.push_str(&format!(
        "- Removed families: {}\n\n",
        format_tally(summary.condition_tallies.removed_families)
    ));
    text.push_str("## Ranked Drifting Families\n\n");
    if drifting.is_empty() {
        text.push_str("No family drifted across the supplied comparison exports.\n");
    } else {
        text.push_str("| Family | Material Changes | Decision Changes | Max Abs Lift Delta | Max Abs Q Delta |\n");
        text.push_str("|---|---:|---:|---:|---:|\n");
        for row in drifting.iter().take(10) {
            text.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                row.family_code,
                row.material_change_count,
                row.decision_change_count,
                format_optional_delta(row.max_abs_lift_delta),
                format_optional_delta(row.max_abs_q_delta)
            ));
        }
    }
    text
}
