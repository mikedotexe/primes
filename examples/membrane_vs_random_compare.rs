//! Compare two archival matched-control JSON exports.
//!
//! This helper highlights pooled residual-status changes and any per-family
//! lift or q-value moves that exceed chosen materiality thresholds.
//!
//! Example:
//! `cargo run --example membrane_vs_random_compare -- before.json after.json --lift-threshold 0.25 --q-threshold 0.10 --flag-sampling-plan-drift --json-out diff.json --exit-on-flag`

use primes::validation::matched_control::{
    compare_export_bundles, format_p_like, read_json_export, summarize_comparison_audit,
    write_comparison_json_export, MatchedControlComparePolicy, MatchedControlCompareSettings,
    MatchedControlFamilyDelta, DEFAULT_COMPARE_LIFT_THRESHOLD, DEFAULT_COMPARE_Q_THRESHOLD,
    MATCHED_CONTROL_COMPARE_FLAG_EXIT_CODE,
};
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let options = parse_args();
    let before = read_json_export(&options.before).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read matched-control JSON export {}: {err}",
            options.before.display()
        );
        std::process::exit(1);
    });
    let after = read_json_export(&options.after).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read matched-control JSON export {}: {err}",
            options.after.display()
        );
        std::process::exit(1);
    });
    let comparison = compare_export_bundles(&before, &after, options.settings);
    let audit = summarize_comparison_audit(&before, &after, &comparison, options.policy);

    if let Some(path) = &options.json_out {
        write_comparison_json_export(
            path,
            &options.before,
            &options.after,
            options.settings,
            options.policy,
            &comparison,
            &audit,
        )
        .unwrap_or_else(|err| {
            eprintln!(
                "Failed to write comparison JSON export to {}: {err}",
                path.display()
            );
            std::process::exit(1);
        });
    }

    println!("Matched-Control Export Comparison");
    println!("{}", "=".repeat(112));
    println!(
        "Before: {}  |  generated {}  |  export v{}",
        options.before.display(),
        comparison.before_generated_at_utc,
        comparison.before_export_version
    );
    println!(
        "After:  {}  |  generated {}  |  export v{}",
        options.after.display(),
        comparison.after_generated_at_utc,
        comparison.after_export_version
    );
    println!(
        "Panels: before={}  |  after={}",
        format_panel_id(comparison.before_panel_id.as_deref()),
        format_panel_id(comparison.after_panel_id.as_deref())
    );
    println!(
        "Material thresholds: |delta lift| >= {:.3}x, |delta q| >= {:.3}",
        options.settings.lift_threshold, options.settings.q_threshold
    );
    println!("Audit policy: {}", format_policy(options.policy));
    println!(
        "Before sampling plan: {} samples, seed lengths {}..={}, FDR={:.3}",
        before.settings.samples,
        before.settings.min_seed_len,
        before.settings.max_seed_len,
        before.settings.fdr
    );
    println!(
        "After sampling plan:  {} samples, seed lengths {}..={}, FDR={:.3}",
        after.settings.samples,
        after.settings.min_seed_len,
        after.settings.max_seed_len,
        after.settings.fdr
    );
    if before.settings != after.settings {
        println!("Caution: the sampling plans differ across the two exports.");
    }
    if let Some(path) = &options.json_out {
        println!("Comparison JSON: {}", path.display());
    }

    println!();
    println!("Summary");
    println!("{}", "-".repeat(112));
    println!(
        "Residual criterion: {} -> {}{}",
        bool_status(comparison.residual_criterion_before),
        bool_status(comparison.residual_criterion_after),
        if comparison.residual_criterion_changed {
            " (changed)"
        } else {
            " (unchanged)"
        }
    );
    println!(
        "Pooled lift:        {} -> {} ({})",
        format_ratio(comparison.pooled_lift_before),
        format_ratio(comparison.pooled_lift_after),
        format_ratio_delta(comparison.pooled_lift_delta)
    );
    println!(
        "Positive-q families: {} -> {}",
        comparison.positive_q_before, comparison.positive_q_after
    );
    println!(
        "Negative-q families: {} -> {}",
        comparison.negative_q_before, comparison.negative_q_after
    );
    println!(
        "Families compared: {}  |  added: {}  |  removed: {}",
        comparison.families_compared,
        comparison.added_families.len(),
        comparison.removed_families.len()
    );
    println!(
        "Audit flag: {}{}",
        if audit.flagged { "set" } else { "clear" },
        if audit.reasons.is_empty() {
            String::new()
        } else {
            format!(" ({})", audit.reasons.join(", "))
        }
    );
    println!(
        "Audit conditions: residual={} material={} sampling={} added={} removed={}",
        audit
            .conditions
            .residual_criterion_changed
            .severity
            .as_str(),
        audit.conditions.material_family_change.severity.as_str(),
        audit.conditions.sampling_plan_drift.severity.as_str(),
        audit.conditions.added_families.severity.as_str(),
        audit.conditions.removed_families.severity.as_str(),
    );

    println!();
    println!("Material Family Changes");
    println!("{}", "-".repeat(112));
    if comparison.materially_changed_families.is_empty() {
        println!("No family exceeded the configured materiality thresholds.");
    } else {
        println!(
            "{:<23} {:>28} {:>24} {:>24} {:>10}",
            "family", "lift", "q", "decision", "flags"
        );
        println!("{}", "-".repeat(112));
        for delta in &comparison.materially_changed_families {
            println!(
                "{:<23} {:>28} {:>24} {:>24} {:>10}",
                delta.family_code,
                format_ratio_change(delta),
                format_q_change(delta),
                format!("{} -> {}", delta.decision_before, delta.decision_after),
                change_flags(delta),
            );
        }
    }

    if !comparison.added_families.is_empty() {
        println!();
        println!("Added Families");
        println!("{}", "-".repeat(112));
        for family in &comparison.added_families {
            println!("  + {} ({})", family.family_code, family.family_label);
        }
    }

    if !comparison.removed_families.is_empty() {
        println!();
        println!("Removed Families");
        println!("{}", "-".repeat(112));
        for family in &comparison.removed_families {
            println!("  - {} ({})", family.family_code, family.family_label);
        }
    }

    if options.exit_on_flag && audit.flagged {
        let _ = io::stdout().flush();
        eprintln!(
            "Matched-control comparison flagged audit conditions; exiting with code {}.",
            MATCHED_CONTROL_COMPARE_FLAG_EXIT_CODE
        );
        std::process::exit(MATCHED_CONTROL_COMPARE_FLAG_EXIT_CODE);
    }
}

struct CliOptions {
    before: PathBuf,
    after: PathBuf,
    settings: MatchedControlCompareSettings,
    policy: MatchedControlComparePolicy,
    json_out: Option<PathBuf>,
    exit_on_flag: bool,
}

fn parse_args() -> CliOptions {
    let mut before = None;
    let mut after = None;
    let mut settings = MatchedControlCompareSettings::default();
    let mut policy = MatchedControlComparePolicy::default();
    let mut json_out = None;
    let mut exit_on_flag = false;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--lift-threshold" => {
                settings.lift_threshold = parse_next(&mut args, "--lift-threshold");
            }
            "--q-threshold" => {
                settings.q_threshold = parse_next(&mut args, "--q-threshold");
            }
            "--flag-sampling-plan-drift" => {
                policy.flag_sampling_plan_drift = true;
            }
            "--flag-added-families" => {
                policy.flag_added_families = true;
            }
            "--flag-removed-families" => {
                policy.flag_removed_families = true;
            }
            "--json-out" => {
                json_out = Some(PathBuf::from(parse_next::<String>(&mut args, "--json-out")));
            }
            "--exit-on-flag" => exit_on_flag = true,
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ if before.is_none() => before = Some(PathBuf::from(arg)),
            _ if after.is_none() => after = Some(PathBuf::from(arg)),
            _ => {
                eprintln!("Unknown argument: {arg}");
                print_help();
                std::process::exit(2);
            }
        }
    }

    let before = before.unwrap_or_else(|| {
        eprintln!("Missing <before.json> argument");
        print_help();
        std::process::exit(2);
    });
    let after = after.unwrap_or_else(|| {
        eprintln!("Missing <after.json> argument");
        print_help();
        std::process::exit(2);
    });

    if settings.lift_threshold < 0.0 {
        eprintln!("--lift-threshold must be nonnegative");
        std::process::exit(2);
    }
    if settings.q_threshold < 0.0 {
        eprintln!("--q-threshold must be nonnegative");
        std::process::exit(2);
    }

    CliOptions {
        before,
        after,
        settings,
        policy,
        json_out,
        exit_on_flag,
    }
}

fn print_help() {
    println!("Compare two matched-control JSON exports");
    println!();
    println!("Usage:");
    println!(
        "  cargo run --example membrane_vs_random_compare -- <before.json> <after.json> [options]"
    );
    println!();
    println!("Options:");
    println!(
        "  --lift-threshold <x>  Material absolute lift delta in x-units (default: {DEFAULT_COMPARE_LIFT_THRESHOLD})"
    );
    println!(
        "  --q-threshold <x>     Material absolute q-value delta (default: {DEFAULT_COMPARE_Q_THRESHOLD})"
    );
    println!(
        "  --flag-sampling-plan-drift  Promote sampling-plan drift from informational to flagged"
    );
    println!(
        "  --flag-added-families       Promote added-family changes from informational to flagged"
    );
    println!(
        "  --flag-removed-families     Promote removed-family changes from informational to flagged"
    );
    println!("  --json-out <path>     Write machine-readable comparison JSON to the given path");
    println!(
        "  --exit-on-flag        Exit with code {MATCHED_CONTROL_COMPARE_FLAG_EXIT_CODE} when residual status flips or flagged family changes appear"
    );
}

fn parse_next<T>(args: &mut impl Iterator<Item = String>, flag: &str) -> T
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let value = args.next().unwrap_or_else(|| {
        eprintln!("Missing value for {flag}");
        std::process::exit(2);
    });
    value.parse::<T>().unwrap_or_else(|err| {
        eprintln!("Invalid value for {flag}: {value} ({err})");
        std::process::exit(2);
    })
}

fn bool_status(value: bool) -> &'static str {
    if value {
        "met"
    } else {
        "not met"
    }
}

fn format_panel_id(panel_id: Option<&str>) -> &str {
    panel_id.unwrap_or("manual")
}

fn format_ratio(value: Option<f64>) -> String {
    match value {
        Some(value) => format!("{value:.3}x"),
        None => "n/a".to_string(),
    }
}

fn format_ratio_delta(value: Option<f64>) -> String {
    match value {
        Some(value) => format!("{value:+.3}x"),
        None => "n/a".to_string(),
    }
}

fn format_ratio_change(delta: &MatchedControlFamilyDelta) -> String {
    match (delta.lift_before, delta.lift_after, delta.lift_delta) {
        (Some(before), Some(after), Some(change)) => {
            format!("{before:.3}x -> {after:.3}x ({change:+.3}x)")
        }
        _ => "n/a".to_string(),
    }
}

fn format_q_change(delta: &MatchedControlFamilyDelta) -> String {
    match (delta.q_before, delta.q_after, delta.q_delta) {
        (Some(before), Some(after), Some(change)) => format!(
            "{} -> {} ({change:+.3})",
            format_p_like(before),
            format_p_like(after)
        ),
        _ => "n/a".to_string(),
    }
}

fn change_flags(delta: &MatchedControlFamilyDelta) -> String {
    let mut flags = Vec::new();
    if delta.material_lift_change {
        flags.push("lift");
    }
    if delta.material_q_change {
        flags.push("q");
    }
    if delta.decision_changed {
        flags.push("decision");
    }
    flags.join(",")
}

fn format_policy(policy: MatchedControlComparePolicy) -> String {
    let mut parts = Vec::new();
    if policy.flag_sampling_plan_drift {
        parts.push("sampling-drift=flagged");
    }
    if policy.flag_added_families {
        parts.push("added-families=flagged");
    }
    if policy.flag_removed_families {
        parts.push("removed-families=flagged");
    }
    if parts.is_empty() {
        "default (sampling drift and family-set changes stay informational)".to_string()
    } else {
        parts.join(", ")
    }
}
