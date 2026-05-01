//! Hinge robustness matrix pass.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example two_p_hinge_robustness_report
//! cargo run --release --example two_p_hinge_robustness_report -- --out-dir /tmp/primes_two_p_hinge_robustness_alt
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        analyze_hinge_feature_row, digit_symbol, ordered_unit_pairs, HingeFeatureRow,
        HINGE_CATEGORY_PERSISTENT_CORE,
    },
    hinge_atoms::{reevaluate_hinge_rule_candidate, HingeAtomFamily, HINGE_FAMILIES},
    hinge_robustness::{
        default_hinge_robustness_scenarios, run_hinge_robustness_scenario, HingeFamilyDepthLabel,
        HingeFamilyDepthRow, HingeRobustnessRun, HingeRobustnessScenario, HingeScenarioGroup,
        HingeScenarioKind, HingeTheoremLanguageLabel,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
};

const MAIN_BASES: &[u32] = &[10, 14, 22, 26];
const APPENDIX_BASES: &[u32] = &[34, 6];
const DEFAULT_OUT_DIR: &str = "/tmp/primes_two_p_hinge_robustness";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "two_p_hinge_robustness_report";
const MAX_RULE_ATOMS: usize = 3;
const EXPORTED_RULE_FRONTIER: usize = 60;
const BEST_RULES_PER_SEARCH: usize = 5;

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    main_bases: Vec<u32>,
    appendix_bases: Vec<u32>,
    pair_catalog_mode: String,
    success_target: String,
    max_rule_atoms: usize,
}

#[derive(Debug, Clone, Serialize)]
struct RobustnessScenarioRow {
    scenario_id: String,
    scenario_label: String,
    scenario_group: HingeScenarioGroup,
    scenario_kind: HingeScenarioKind,
    counted_in_stability_tally: bool,
    threshold_policy: String,
    included_families: String,
    primary_dataset_rows: usize,
    primary_positive_rows: usize,
    any_exact_primary_rule: bool,
    primary_rule_label: String,
    primary_rule_unchanged: bool,
    family_ladder_stable: bool,
    overlap_boundary_deepest: bool,
    overlap_boundary_closest_to_theorem: bool,
    carry_through_bridge: bool,
    threshold_shape_bridge: bool,
    geometry_diagnostic: bool,
    template_choice_diagnostic: bool,
    scenario_status: String,
}

#[derive(Debug, Clone, Serialize)]
struct RobustnessPrimaryRuleRow {
    scenario_id: String,
    scenario_label: String,
    scenario_kind: HingeScenarioKind,
    any_exact_primary_rule: bool,
    primary_rule_label: String,
    primary_rule_unchanged: bool,
    primary_rule_drift_label: String,
    primary_rule_theorem_class: Option<String>,
    threshold_free: bool,
    true_positive: usize,
    false_positive: usize,
    false_negative: usize,
}

#[derive(Debug, Clone, Serialize)]
struct RobustnessFamilySnapshotRow {
    scenario_id: String,
    scenario_label: String,
    family: HingeAtomFamily,
    theorem_class: String,
    depth_label: String,
    theorem_language_label: String,
    primary_family_only_exact: bool,
    persistent_family_only_exact: bool,
    core_family_only_exact: bool,
    primary_ablation_breaks_exact: bool,
    primary_exact_mixed_rule_count: usize,
    primary_top_frontier_presence: bool,
    rationale: String,
}

#[derive(Debug, Clone, Serialize)]
struct RobustnessAppendixAuditRow {
    scenario_id: String,
    scenario_label: String,
    appendix_base: u32,
    active_rows: usize,
    positive_rows: usize,
    true_positive: usize,
    false_positive: usize,
    false_negative: usize,
    true_negative: usize,
    audited_rule_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct RepresentativeStressRow {
    scenario_id: String,
    scenario_label: String,
    role: String,
    base: u32,
    pair_label: String,
    included_in_scenario_surface: bool,
    actual_primary_positive: bool,
    predicted_primary_positive: bool,
    false_positive: bool,
    false_negative: bool,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    stability_strong_pass_count: usize,
    stability_ladder_pass_count: usize,
    stability_weakened_count: usize,
    stability_fail_count: usize,
    baseline_primary_rule: String,
    baseline_family_ladder: Vec<String>,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    robustness_scenario_rows: Vec<RobustnessScenarioRow>,
    robustness_primary_rule_rows: Vec<RobustnessPrimaryRuleRow>,
    robustness_family_snapshot_rows: Vec<RobustnessFamilySnapshotRow>,
    robustness_appendix_audit_rows: Vec<RobustnessAppendixAuditRow>,
    representative_stress_rows: Vec<RepresentativeStressRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

#[derive(Debug, Clone)]
struct ScenarioRunBundle {
    scenario: HingeRobustnessScenario,
    run: HingeRobustnessRun,
    scenario_row: RobustnessScenarioRow,
}

#[derive(Debug, Clone, Copy)]
struct RepresentativeSpec {
    role: &'static str,
    base: u32,
    outer: u32,
    inner: u32,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        main_bases: MAIN_BASES.to_vec(),
        appendix_bases: APPENDIX_BASES.to_vec(),
        pair_catalog_mode: "full".to_string(),
        success_target: "family_ladder_stable".to_string(),
        max_rule_atoms: MAX_RULE_ATOMS,
    };

    let mut main_rows = build_hinge_feature_rows(MAIN_BASES);
    main_rows.sort_by(row_sort_key);
    let mut appendix_rows = build_hinge_feature_rows(APPENDIX_BASES);
    appendix_rows.sort_by(row_sort_key);

    let scenarios = default_hinge_robustness_scenarios();
    let baseline_scenario = scenarios
        .iter()
        .find(|scenario| scenario.id == "baseline_main")
        .expect("baseline scenario should exist")
        .clone();
    let baseline_run = run_hinge_robustness_scenario(
        &main_rows,
        &baseline_scenario,
        MAX_RULE_ATOMS,
        EXPORTED_RULE_FRONTIER,
        BEST_RULES_PER_SEARCH,
    );
    let baseline_rule_label = baseline_run
        .best_primary_rule
        .as_ref()
        .map(|row| row.rule_label.clone())
        .unwrap_or_else(|| "none".to_string());
    let baseline_signature = baseline_run.family_ladder_signature.clone();

    let scenario_bundles = scenarios
        .iter()
        .cloned()
        .map(|scenario| {
            let run = run_hinge_robustness_scenario(
                &main_rows,
                &scenario,
                MAX_RULE_ATOMS,
                EXPORTED_RULE_FRONTIER,
                BEST_RULES_PER_SEARCH,
            );
            let scenario_row =
                build_scenario_row(&scenario, &run, &baseline_signature, &baseline_rule_label);
            ScenarioRunBundle {
                scenario,
                run,
                scenario_row,
            }
        })
        .collect::<Vec<_>>();

    let mut scenario_rows = scenario_bundles
        .iter()
        .map(|bundle| bundle.scenario_row.clone())
        .collect::<Vec<_>>();
    scenario_rows.sort_by(|left, right| {
        scenario_group_rank(left.scenario_group)
            .cmp(&scenario_group_rank(right.scenario_group))
            .then_with(|| left.scenario_id.cmp(&right.scenario_id))
    });

    let mut primary_rule_rows = scenario_bundles
        .iter()
        .map(|bundle| build_primary_rule_row(bundle, &baseline_rule_label))
        .collect::<Vec<_>>();
    primary_rule_rows.sort_by(|left, right| left.scenario_id.cmp(&right.scenario_id));

    let mut family_snapshot_rows = scenario_bundles
        .iter()
        .flat_map(build_family_snapshot_rows)
        .collect::<Vec<_>>();
    family_snapshot_rows.sort_by(|left, right| {
        left.scenario_id
            .cmp(&right.scenario_id)
            .then_with(|| left.family.cmp(&right.family))
    });

    let mut appendix_audit_rows = scenario_bundles
        .iter()
        .filter(|bundle| bundle.scenario.scenario_kind == HingeScenarioKind::Stability)
        .flat_map(|bundle| build_appendix_audit_rows(bundle, &appendix_rows))
        .collect::<Vec<_>>();
    appendix_audit_rows.sort_by(|left, right| {
        left.scenario_id
            .cmp(&right.scenario_id)
            .then_with(|| left.appendix_base.cmp(&right.appendix_base))
    });

    let mut representative_stress_rows =
        build_representative_stress_rows(&scenario_bundles, &main_rows, &appendix_rows);
    representative_stress_rows.sort_by(|left, right| {
        left.scenario_id
            .cmp(&right.scenario_id)
            .then_with(|| left.base.cmp(&right.base))
            .then_with(|| left.pair_label.cmp(&right.pair_label))
    });

    let robustness_matrix_path = options.out_dir.join("robustness_matrix.png");
    render_robustness_matrix(&scenario_rows, &robustness_matrix_path);
    let primary_rule_drift_path = options.out_dir.join("primary_rule_drift_strip.png");
    render_primary_rule_drift_strip(&primary_rule_rows, &primary_rule_drift_path);
    let family_ladder_stability_path = options.out_dir.join("family_ladder_stability.png");
    render_family_ladder_stability(&family_snapshot_rows, &family_ladder_stability_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "robustness_matrix".to_string(),
            label: "Robustness matrix".to_string(),
            path: robustness_matrix_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "primary_rule_drift_strip".to_string(),
            label: "Primary rule drift strip".to_string(),
            path: primary_rule_drift_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "family_ladder_stability".to_string(),
            label: "Family ladder stability".to_string(),
            path: family_ladder_stability_path.display().to_string(),
        },
    ];

    let report_summary =
        build_report_summary(&scenario_rows, &baseline_rule_label, &baseline_signature);
    let observations = derive_observations(
        &scenario_rows,
        &primary_rule_rows,
        &appendix_audit_rows,
        &representative_stress_rows,
    );

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        robustness_scenario_rows: scenario_rows.clone(),
        robustness_primary_rule_rows: primary_rule_rows.clone(),
        robustness_family_snapshot_rows: family_snapshot_rows.clone(),
        robustness_appendix_audit_rows: appendix_audit_rows.clone(),
        representative_stress_rows: representative_stress_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("robustness_scenario_rows.csv"),
        &scenario_rows,
    )
    .expect("failed to write robustness_scenario_rows.csv");
    write_csv_rows(
        options.out_dir.join("robustness_primary_rule_rows.csv"),
        &primary_rule_rows,
    )
    .expect("failed to write robustness_primary_rule_rows.csv");
    write_csv_rows(
        options.out_dir.join("robustness_family_snapshot_rows.csv"),
        &family_snapshot_rows,
    )
    .expect("failed to write robustness_family_snapshot_rows.csv");
    write_csv_rows(
        options.out_dir.join("robustness_appendix_audit_rows.csv"),
        &appendix_audit_rows,
    )
    .expect("failed to write robustness_appendix_audit_rows.csv");
    write_csv_rows(
        options.out_dir.join("representative_stress_rows.csv"),
        &representative_stress_rows,
    )
    .expect("failed to write representative_stress_rows.csv");
    write_json_pretty(options.out_dir.join("summary.json"), &bundle)
        .expect("failed to write summary.json");
    write_text_file(options.out_dir.join("report.md"), &render_markdown(&bundle))
        .expect("failed to write report.md");
    write_artifact_manifest(
        &options.out_dir,
        &ArtifactManifest {
            artifact_id: ARTIFACT_ID.to_string(),
            generator_cmd: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "--release".to_string(),
                "--example".to_string(),
                "two_p_hinge_robustness_report".to_string(),
                "--".to_string(),
                "--out-dir".to_string(),
                options.out_dir.display().to_string(),
            ],
            upstream_inputs: vec![],
            expected_outputs: vec![
                "robustness_scenario_rows.csv".to_string(),
                "robustness_primary_rule_rows.csv".to_string(),
                "robustness_family_snapshot_rows.csv".to_string(),
                "robustness_appendix_audit_rows.csv".to_string(),
                "representative_stress_rows.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
                "artifact_manifest.json".to_string(),
                "robustness_matrix.png".to_string(),
                "primary_rule_drift_strip.png".to_string(),
                "family_ladder_stability.png".to_string(),
            ],
        },
    )
    .expect("failed to write artifact manifest");

    println!("2p hinge robustness report");
    println!("  output dir: {}", options.out_dir.display());
    for row in &scenario_rows {
        println!(
            "  {:<24} | {:<11} | exact {:<3} | unchanged {:<3} | ladder {:<3} | status {}",
            row.scenario_id,
            row.scenario_group.as_str(),
            yes_no(row.any_exact_primary_rule),
            yes_no(row.primary_rule_unchanged),
            yes_no(row.family_ladder_stable),
            row.scenario_status,
        );
    }
}

fn parse_args() -> Options {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                let value = args
                    .next()
                    .expect("--out-dir requires a directory argument");
                out_dir = PathBuf::from(value);
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => panic!("unrecognized argument: {arg}"),
        }
    }
    Options { out_dir }
}

fn print_help() {
    println!("Usage:");
    println!("  cargo run --release --example two_p_hinge_robustness_report -- [options]");
    println!();
    println!("Options:");
    println!("  --out-dir <dir>   Output directory (default: {DEFAULT_OUT_DIR})");
    println!("  -h, --help        Show this help message");
}

fn build_hinge_feature_rows(bases: &[u32]) -> Vec<HingeFeatureRow> {
    bases
        .par_iter()
        .copied()
        .flat_map(|base| {
            ordered_unit_pairs(base)
                .into_par_iter()
                .map(move |(outer, inner)| analyze_hinge_feature_row(base, outer, inner))
        })
        .collect()
}

fn row_sort_key(left: &HingeFeatureRow, right: &HingeFeatureRow) -> std::cmp::Ordering {
    left.base
        .cmp(&right.base)
        .then_with(|| left.outer.cmp(&right.outer))
        .then_with(|| left.inner.cmp(&right.inner))
}

fn build_scenario_row(
    scenario: &HingeRobustnessScenario,
    run: &HingeRobustnessRun,
    baseline_signature: &[String],
    baseline_rule_label: &str,
) -> RobustnessScenarioRow {
    let overlap = family_snapshot(run, HingeAtomFamily::OverlapBoundary);
    let carry = family_snapshot(run, HingeAtomFamily::CarryThrough);
    let threshold = family_snapshot(run, HingeAtomFamily::ThresholdShape);
    let geometry = family_snapshot(run, HingeAtomFamily::Geometry);
    let template = family_snapshot(run, HingeAtomFamily::TemplateChoice);
    let primary_rule_label = run
        .best_primary_rule
        .as_ref()
        .map(|row| row.rule_label.clone())
        .unwrap_or_else(|| "no_exact_rule".to_string());
    let primary_rule_unchanged =
        run.any_exact_primary_rule && primary_rule_label == baseline_rule_label;
    let family_ladder_stable = run.family_ladder_signature == baseline_signature;

    RobustnessScenarioRow {
        scenario_id: scenario.id.to_string(),
        scenario_label: scenario.label.to_string(),
        scenario_group: scenario.group,
        scenario_kind: scenario.scenario_kind,
        counted_in_stability_tally: scenario.scenario_kind == HingeScenarioKind::Stability,
        threshold_policy: scenario
            .atom_catalog_policy
            .threshold_policy
            .as_str()
            .to_string(),
        included_families: scenario
            .atom_catalog_policy
            .included_families
            .iter()
            .map(|family| family.as_str().to_string())
            .collect::<Vec<_>>()
            .join(","),
        primary_dataset_rows: run.dataset_rows,
        primary_positive_rows: run.positive_rows,
        any_exact_primary_rule: run.any_exact_primary_rule,
        primary_rule_label,
        primary_rule_unchanged,
        family_ladder_stable,
        overlap_boundary_deepest: overlap.depth_label == HingeFamilyDepthLabel::Deepest,
        overlap_boundary_closest_to_theorem: overlap.theorem_language_label
            == HingeTheoremLanguageLabel::ClosestToTheorem,
        carry_through_bridge: carry.depth_label == HingeFamilyDepthLabel::Bridge,
        threshold_shape_bridge: threshold.depth_label == HingeFamilyDepthLabel::Bridge,
        geometry_diagnostic: geometry.depth_label == HingeFamilyDepthLabel::Diagnostic,
        template_choice_diagnostic: template.depth_label == HingeFamilyDepthLabel::Diagnostic,
        scenario_status: classify_scenario_status(
            family_ladder_stable,
            overlap.depth_label == HingeFamilyDepthLabel::Deepest
                && overlap.theorem_language_label == HingeTheoremLanguageLabel::ClosestToTheorem,
            carry.depth_label == HingeFamilyDepthLabel::Bridge
                && threshold.depth_label == HingeFamilyDepthLabel::Bridge,
            geometry.depth_label == HingeFamilyDepthLabel::Diagnostic
                && template.depth_label == HingeFamilyDepthLabel::Diagnostic,
            run.any_exact_primary_rule,
            primary_rule_unchanged,
        )
        .to_string(),
    }
}

fn build_primary_rule_row(
    bundle: &ScenarioRunBundle,
    baseline_rule_label: &str,
) -> RobustnessPrimaryRuleRow {
    let best = bundle.run.best_primary_rule.as_ref();
    let primary_rule_label = best
        .map(|row| row.rule_label.clone())
        .unwrap_or_else(|| "no_exact_rule".to_string());
    let primary_rule_unchanged =
        bundle.run.any_exact_primary_rule && primary_rule_label == baseline_rule_label;
    let primary_rule_drift_label = if !bundle.run.any_exact_primary_rule {
        "no_exact_rule".to_string()
    } else if primary_rule_unchanged {
        "unchanged".to_string()
    } else {
        "drifted".to_string()
    };

    RobustnessPrimaryRuleRow {
        scenario_id: bundle.scenario.id.to_string(),
        scenario_label: bundle.scenario.label.to_string(),
        scenario_kind: bundle.scenario.scenario_kind,
        any_exact_primary_rule: bundle.run.any_exact_primary_rule,
        primary_rule_label,
        primary_rule_unchanged,
        primary_rule_drift_label,
        primary_rule_theorem_class: best.map(|row| row.rule_theorem_class.as_str().to_string()),
        threshold_free: best.map(|row| row.threshold_free).unwrap_or(false),
        true_positive: best.map(|row| row.true_positive).unwrap_or(0),
        false_positive: best.map(|row| row.false_positive).unwrap_or(0),
        false_negative: best.map(|row| row.false_negative).unwrap_or(0),
    }
}

fn build_family_snapshot_rows(bundle: &ScenarioRunBundle) -> Vec<RobustnessFamilySnapshotRow> {
    bundle
        .run
        .family_depth_rows
        .iter()
        .map(|row| RobustnessFamilySnapshotRow {
            scenario_id: bundle.scenario.id.to_string(),
            scenario_label: bundle.scenario.label.to_string(),
            family: row.family,
            theorem_class: row.theorem_class.as_str().to_string(),
            depth_label: row.depth_label.as_str().to_string(),
            theorem_language_label: row.theorem_language_label.as_str().to_string(),
            primary_family_only_exact: row.primary_family_only_exact,
            persistent_family_only_exact: row.persistent_family_only_exact,
            core_family_only_exact: row.core_family_only_exact,
            primary_ablation_breaks_exact: row.primary_ablation_breaks_exact,
            primary_exact_mixed_rule_count: row.primary_exact_mixed_rule_count,
            primary_top_frontier_presence: row.primary_top_frontier_presence,
            rationale: row.rationale.clone(),
        })
        .collect()
}

fn build_appendix_audit_rows(
    bundle: &ScenarioRunBundle,
    appendix_rows: &[HingeFeatureRow],
) -> Vec<RobustnessAppendixAuditRow> {
    APPENDIX_BASES
        .iter()
        .copied()
        .map(|base| {
            let active_rows = appendix_rows
                .iter()
                .filter(|row| row.base == base && row.m2_active)
                .collect::<Vec<_>>();
            let problem = primes::validation::hinge_atoms::HingeSearchProblem {
                id: "appendix_primary_audit",
                label: "Appendix primary audit",
                rows: active_rows.clone(),
                target: active_rows
                    .iter()
                    .map(|row| row.hinge_category == HINGE_CATEGORY_PERSISTENT_CORE)
                    .collect(),
            };
            let evaluation = bundle
                .run
                .best_primary_rule
                .as_ref()
                .map(|candidate| reevaluate_hinge_rule_candidate(&problem, candidate));
            RobustnessAppendixAuditRow {
                scenario_id: bundle.scenario.id.to_string(),
                scenario_label: bundle.scenario.label.to_string(),
                appendix_base: base,
                active_rows: active_rows.len(),
                positive_rows: problem.target.iter().filter(|&&value| value).count(),
                true_positive: evaluation
                    .as_ref()
                    .map(|row| row.true_positive)
                    .unwrap_or(0),
                false_positive: evaluation
                    .as_ref()
                    .map(|row| row.false_positive)
                    .unwrap_or(0),
                false_negative: evaluation
                    .as_ref()
                    .map(|row| row.false_negative)
                    .unwrap_or(0),
                true_negative: evaluation
                    .as_ref()
                    .map(|row| row.true_negative)
                    .unwrap_or(active_rows.len()),
                audited_rule_label: bundle
                    .run
                    .best_primary_rule
                    .as_ref()
                    .map(|row| row.rule_label.clone())
                    .unwrap_or_else(|| "no_exact_rule".to_string()),
            }
        })
        .collect()
}

fn build_representative_stress_rows(
    bundles: &[ScenarioRunBundle],
    main_rows: &[HingeFeatureRow],
    appendix_rows: &[HingeFeatureRow],
) -> Vec<RepresentativeStressRow> {
    let mut specs = vec![
        RepresentativeSpec {
            role: "persistent_core",
            base: 14,
            outer: 13,
            inner: 11,
        },
        RepresentativeSpec {
            role: "persistent_core",
            base: 14,
            outer: 3,
            inner: 1,
        },
        RepresentativeSpec {
            role: "persistence_only",
            base: 10,
            outer: 3,
            inner: 3,
        },
        RepresentativeSpec {
            role: "core_only",
            base: 26,
            outer: 23,
            inner: 23,
        },
        RepresentativeSpec {
            role: "active_neither",
            base: 22,
            outer: 17,
            inner: 19,
        },
        RepresentativeSpec {
            role: "appendix_outgroup",
            base: 34,
            outer: 25,
            inner: 9,
        },
    ];
    specs.extend(
        appendix_rows
            .iter()
            .filter(|row| row.base == 6 && row.m2_active)
            .map(|row| RepresentativeSpec {
                role: "appendix_active",
                base: row.base,
                outer: row.outer,
                inner: row.inner,
            }),
    );

    let mut row_map = BTreeMap::<(u32, u32, u32), HingeFeatureRow>::new();
    for row in main_rows.iter().chain(appendix_rows) {
        row_map.insert((row.base, row.outer, row.inner), row.clone());
    }

    bundles
        .iter()
        .flat_map(|bundle| {
            specs.iter().map(|spec| {
                let row = row_map
                    .get(&(spec.base, spec.outer, spec.inner))
                    .expect("representative row should exist");
                let included = bundle.run.filtered_rows.iter().any(|filtered| {
                    filtered.base == spec.base
                        && filtered.outer == spec.outer
                        && filtered.inner == spec.inner
                });
                let predicted_positive = bundle
                    .run
                    .best_primary_rule
                    .as_ref()
                    .map(|candidate| {
                        candidate
                            .atom_predicates
                            .iter()
                            .all(|predicate| predicate.evaluate(row))
                    })
                    .unwrap_or(false);
                let actual_primary_positive = row.hinge_category == HINGE_CATEGORY_PERSISTENT_CORE;
                RepresentativeStressRow {
                    scenario_id: bundle.scenario.id.to_string(),
                    scenario_label: bundle.scenario.label.to_string(),
                    role: spec.role.to_string(),
                    base: spec.base,
                    pair_label: format!(
                        "({},{})",
                        digit_symbol(spec.outer),
                        digit_symbol(spec.inner)
                    ),
                    included_in_scenario_surface: included,
                    actual_primary_positive,
                    predicted_primary_positive: predicted_positive,
                    false_positive: predicted_positive && !actual_primary_positive,
                    false_negative: !predicted_positive && actual_primary_positive,
                    note: if included {
                        representative_mechanism_sentence(row).to_string()
                    } else {
                        "dropped_from_surface".to_string()
                    },
                }
            })
        })
        .collect()
}

fn build_report_summary(
    scenario_rows: &[RobustnessScenarioRow],
    baseline_rule_label: &str,
    baseline_signature: &[String],
) -> ReportSummary {
    let stability_rows = scenario_rows
        .iter()
        .filter(|row| row.counted_in_stability_tally)
        .collect::<Vec<_>>();
    ReportSummary {
        stability_strong_pass_count: stability_rows
            .iter()
            .filter(|row| row.scenario_status == "strong_pass")
            .count(),
        stability_ladder_pass_count: stability_rows
            .iter()
            .filter(|row| row.scenario_status == "ladder_pass")
            .count(),
        stability_weakened_count: stability_rows
            .iter()
            .filter(|row| row.scenario_status == "weakened")
            .count(),
        stability_fail_count: stability_rows
            .iter()
            .filter(|row| row.scenario_status == "fail")
            .count(),
        baseline_primary_rule: baseline_rule_label.to_string(),
        baseline_family_ladder: baseline_signature.to_vec(),
        main_takeaway: if stability_rows
            .iter()
            .all(|row| row.overlap_boundary_deepest && row.overlap_boundary_closest_to_theorem)
        {
            "Across the stability surface, overlap_boundary stays deepest and theorem-adjacent even when the primary rule string is pressured.".to_string()
        } else {
            "At least one stability scenario demotes overlap_boundary; the hinge explanation weakens on the current robustness surface.".to_string()
        },
    }
}

fn derive_observations(
    scenario_rows: &[RobustnessScenarioRow],
    primary_rule_rows: &[RobustnessPrimaryRuleRow],
    appendix_rows: &[RobustnessAppendixAuditRow],
    representative_rows: &[RepresentativeStressRow],
) -> Vec<String> {
    let stability_rows = scenario_rows
        .iter()
        .filter(|row| row.counted_in_stability_tally)
        .collect::<Vec<_>>();
    let threshold_pressure = stability_rows
        .iter()
        .filter(|row| row.scenario_group == HingeScenarioGroup::ThresholdVocabulary)
        .collect::<Vec<_>>();
    let appendix34 = appendix_rows
        .iter()
        .filter(|row| row.appendix_base == 34)
        .collect::<Vec<_>>();

    vec![
        format!(
            "Stability scenarios split as strong `{}`, ladder `{}`, weakened `{}`, fail `{}`.",
            stability_rows
                .iter()
                .filter(|row| row.scenario_status == "strong_pass")
                .count(),
            stability_rows
                .iter()
                .filter(|row| row.scenario_status == "ladder_pass")
                .count(),
            stability_rows
                .iter()
                .filter(|row| row.scenario_status == "weakened")
                .count(),
            stability_rows
                .iter()
                .filter(|row| row.scenario_status == "fail")
                .count(),
        ),
        format!(
            "The primary rule string drifts in `{}` of the counted stability scenarios, but the family ladder stays stable in `{}` of them.",
            primary_rule_rows
                .iter()
                .filter(|row| row.scenario_kind == HingeScenarioKind::Stability && !row.primary_rule_unchanged)
                .count(),
            stability_rows.iter().filter(|row| row.family_ladder_stable).count(),
        ),
        format!(
            "Threshold-vocabulary pressure stays informative rather than destructive: `{}`.",
            threshold_pressure
                .iter()
                .map(|row| format!("{}→{}", row.scenario_id, row.scenario_status))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        format!(
            "Base 34 remains a held-out non-hinge control under the stability audits: `{}`.",
            appendix34
                .iter()
                .map(|row| format!("{} fp={} fn={}", row.scenario_id, row.false_positive, row.false_negative))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        format!(
            "Representative stress rows keep the hinge witnesses narrow: `{}`.",
            representative_rows
                .iter()
                .filter(|row| row.scenario_id == "baseline_main")
                .map(|row| format!("{} {} pred={}", row.base, row.pair_label, yes_no(row.predicted_primary_positive)))
                .collect::<Vec<_>>()
                .join("; ")
        ),
    ]
}

fn render_robustness_matrix(rows: &[RobustnessScenarioRow], path: &Path) {
    let root = BitMapBackend::new(path, (1520, 900)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill robustness matrix");
    let mut chart = ChartBuilder::on(&root)
        .caption("Hinge Robustness Matrix", ("sans-serif", 28))
        .margin(26)
        .x_label_area_size(90)
        .y_label_area_size(260)
        .build_cartesian_2d(0..6, 0..rows.len())
        .expect("failed to build robustness matrix");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(6)
        .x_label_formatter(&|value| match *value {
            0 => "exact".to_string(),
            1 => "unchanged".to_string(),
            2 => "ladder".to_string(),
            3 => "overlap".to_string(),
            4 => "carry".to_string(),
            5 => "threshold".to_string(),
            _ => String::new(),
        })
        .y_labels(rows.len())
        .y_label_formatter(&{
            let labels = rows
                .iter()
                .map(|row| row.scenario_id.clone())
                .collect::<Vec<_>>();
            move |value| labels.get(*value).cloned().unwrap_or_default()
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw robustness matrix mesh");

    for (row_index, row) in rows.iter().enumerate() {
        let cells = [
            row.any_exact_primary_rule,
            row.primary_rule_unchanged,
            row.family_ladder_stable,
            row.overlap_boundary_deepest && row.overlap_boundary_closest_to_theorem,
            row.carry_through_bridge,
            row.threshold_shape_bridge,
        ];
        for (column_index, value) in cells.into_iter().enumerate() {
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [
                        (column_index as i32, row_index),
                        (column_index as i32 + 1, row_index + 1),
                    ],
                    ShapeStyle::from(&bool_color(value)).filled(),
                )))
                .expect("failed to draw robustness cell");
        }
    }

    root.present().expect("failed to present robustness matrix");
}

fn render_primary_rule_drift_strip(rows: &[RobustnessPrimaryRuleRow], path: &Path) {
    let root = BitMapBackend::new(path, (1480, 900)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill rule drift strip");
    let mut chart = ChartBuilder::on(&root)
        .caption("Primary Rule Drift Strip", ("sans-serif", 26))
        .margin(28)
        .x_label_area_size(48)
        .y_label_area_size(300)
        .build_cartesian_2d(0.0f64..1.02f64, 0usize..rows.len())
        .expect("failed to build rule drift strip");

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("F1-equivalent status band")
        .y_labels(rows.len())
        .y_label_formatter(&{
            let labels = rows
                .iter()
                .map(|row| row.scenario_id.clone())
                .collect::<Vec<_>>();
            move |value| labels.get(*value).cloned().unwrap_or_default()
        })
        .label_style(("sans-serif", 14))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw rule drift strip mesh");

    for (index, row) in rows.iter().enumerate() {
        let width = if row.any_exact_primary_rule {
            1.0
        } else {
            0.45
        };
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(0.0, index), (width, index + 1)],
                ShapeStyle::from(&drift_color(&row.primary_rule_drift_label)).filled(),
            )))
            .expect("failed to draw rule drift bar");
        chart
            .draw_series(std::iter::once(Text::new(
                truncate_label(&row.primary_rule_label, 72),
                (0.02, index),
                ("sans-serif", 12).into_font().color(&BLACK),
            )))
            .expect("failed to draw rule drift label");
    }

    root.present()
        .expect("failed to present primary rule drift strip");
}

fn render_family_ladder_stability(rows: &[RobustnessFamilySnapshotRow], path: &Path) {
    let scenario_ids = rows
        .iter()
        .map(|row| row.scenario_id.clone())
        .collect::<Vec<_>>();
    let scenario_ids = unique_preserve_order(scenario_ids);
    let y_axis_labels = scenario_ids.clone();

    let root = BitMapBackend::new(path, (1520, 920)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill family ladder stability");
    let mut chart = ChartBuilder::on(&root)
        .caption("Family Ladder Stability", ("sans-serif", 28))
        .margin(26)
        .x_label_area_size(80)
        .y_label_area_size(300)
        .build_cartesian_2d(0..HINGE_FAMILIES.len(), 0..scenario_ids.len())
        .expect("failed to build family ladder stability");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(HINGE_FAMILIES.len())
        .x_label_formatter(&|value| {
            HINGE_FAMILIES
                .get(*value)
                .map(|family| family.as_str().to_string())
                .unwrap_or_default()
        })
        .y_labels(scenario_ids.len())
        .y_label_formatter(&{ move |value| y_axis_labels.get(*value).cloned().unwrap_or_default() })
        .label_style(("sans-serif", 15))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw family ladder stability mesh");

    for (row_index, scenario_id) in scenario_ids.iter().enumerate() {
        for (column_index, family) in HINGE_FAMILIES.iter().copied().enumerate() {
            let snapshot = rows
                .iter()
                .find(|row| row.scenario_id == *scenario_id && row.family == family)
                .expect("family snapshot should exist");
            let score = match snapshot.depth_label.as_str() {
                "deepest" => 3.0,
                "bridge" => 2.0,
                _ => 1.0,
            };
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(column_index, row_index), (column_index + 1, row_index + 1)],
                    ShapeStyle::from(&depth_color(score)).filled(),
                )))
                .expect("failed to draw ladder stability cell");
        }
    }

    root.present()
        .expect("failed to present family ladder stability");
}

fn render_markdown(bundle: &ReportBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Hinge Robustness Matrix Pass\n\n");
    markdown.push_str("_Generated from `examples/two_p_hinge_robustness_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Output directory: `{}`\n- Main bases: `{}`\n- Appendix bases: `{}`\n- Success target: `{}`\n- Max rule atoms: `{}`\n\n",
        bundle.settings.out_dir,
        bundle.settings.main_bases.iter().map(u32::to_string).collect::<Vec<_>>().join(", "),
        bundle.settings.appendix_bases.iter().map(u32::to_string).collect::<Vec<_>>().join(", "),
        bundle.settings.success_target,
        bundle.settings.max_rule_atoms
    ));

    markdown.push_str("## Scenario Matrix\n\n");
    markdown.push_str(
        "| Scenario | Group | Kind | Exact | Unchanged | Ladder stable | Status | Best rule |\n",
    );
    markdown.push_str("|---|---|---|---|---|---|---|---|\n");
    for row in &bundle.robustness_scenario_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.scenario_id,
            row.scenario_group.as_str(),
            row.scenario_kind.as_str(),
            yes_no(row.any_exact_primary_rule),
            yes_no(row.primary_rule_unchanged),
            yes_no(row.family_ladder_stable),
            row.scenario_status,
            row.primary_rule_label,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Family Snapshot\n\n");
    markdown.push_str("| Scenario | Family | Depth | Theorem language | Primary exact | Persistent exact | Core exact |\n");
    markdown.push_str("|---|---|---|---|---|---|---|\n");
    for row in &bundle.robustness_family_snapshot_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            row.scenario_id,
            row.family.as_str(),
            row.depth_label,
            row.theorem_language_label,
            yes_no(row.primary_family_only_exact),
            yes_no(row.persistent_family_only_exact),
            yes_no(row.core_family_only_exact),
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Appendix Audit\n\n");
    markdown.push_str("| Scenario | Base | Active rows | Positives | tp/fp/fn/tn | Rule |\n");
    markdown.push_str("|---|---:|---:|---:|---|---|\n");
    for row in &bundle.robustness_appendix_audit_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {}/{}/{}/{} | {} |\n",
            row.scenario_id,
            row.appendix_base,
            row.active_rows,
            row.positive_rows,
            row.true_positive,
            row.false_positive,
            row.false_negative,
            row.true_negative,
            row.audited_rule_label,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Representative Stress\n\n");
    markdown.push_str("| Scenario | Role | Base | Pair | Included | Actual positive | Predicted positive | Note |\n");
    markdown.push_str("|---|---|---:|---|---|---|---|---|\n");
    for row in &bundle.representative_stress_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.scenario_id,
            row.role,
            row.base,
            row.pair_label,
            yes_no(row.included_in_scenario_surface),
            yes_no(row.actual_primary_positive),
            yes_no(row.predicted_primary_positive),
            row.note,
        ));
    }
    markdown.push('\n');

    for image in &bundle.image_artifact_rows {
        markdown.push_str(&format!("![{}]({})\n\n", image.label, image.path));
    }

    markdown.push_str("## Observations\n\n");
    for observation in &bundle.observations {
        markdown.push_str(&format!("- {}\n", observation));
    }

    markdown
}

fn classify_scenario_status(
    family_ladder_stable: bool,
    overlap_ok: bool,
    bridge_families_ok: bool,
    diagnostic_families_ok: bool,
    any_exact_primary_rule: bool,
    primary_rule_unchanged: bool,
) -> &'static str {
    if !overlap_ok {
        "fail"
    } else if !family_ladder_stable && (!bridge_families_ok || !diagnostic_families_ok) {
        "weakened"
    } else if family_ladder_stable && any_exact_primary_rule && primary_rule_unchanged {
        "strong_pass"
    } else {
        "ladder_pass"
    }
}

fn family_snapshot(run: &HingeRobustnessRun, family: HingeAtomFamily) -> &HingeFamilyDepthRow {
    run.family_depth_rows
        .iter()
        .find(|row| row.family == family)
        .expect("family snapshot should exist")
}

fn representative_mechanism_sentence(row: &HingeFeatureRow) -> &'static str {
    match (row.m1_to_m2_persistent, row.shared_yield_core) {
        (true, true) => "survives both axes",
        (true, false) => "survives persistence but not overlap dominance",
        (false, true) => "survives overlap dominance but not persistence",
        (false, false) => "misses both",
    }
}

fn scenario_group_rank(group: HingeScenarioGroup) -> usize {
    match group {
        HingeScenarioGroup::DataSurface => 0,
        HingeScenarioGroup::ThresholdVocabulary => 1,
        HingeScenarioGroup::AdversarialCatalog => 2,
    }
}

fn bool_color(value: bool) -> RGBColor {
    if value {
        RGBColor(68, 148, 102)
    } else {
        RGBColor(196, 88, 80)
    }
}

fn drift_color(label: &str) -> RGBColor {
    match label {
        "unchanged" => RGBColor(48, 119, 142),
        "drifted" => RGBColor(218, 143, 53),
        _ => RGBColor(122, 122, 122),
    }
}

fn depth_color(score: f64) -> RGBColor {
    if score >= 2.5 {
        RGBColor(48, 119, 142)
    } else if score >= 1.5 {
        RGBColor(218, 143, 53)
    } else {
        RGBColor(122, 122, 122)
    }
}

fn truncate_label(label: &str, max_chars: usize) -> String {
    if label.chars().count() <= max_chars {
        label.to_string()
    } else {
        let mut truncated = label
            .chars()
            .take(max_chars.saturating_sub(1))
            .collect::<String>();
        truncated.push('…');
        truncated
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn unique_preserve_order(values: Vec<String>) -> Vec<String> {
    let mut seen = BTreeMap::<String, ()>::new();
    let mut unique = Vec::new();
    for value in values {
        if seen.insert(value.clone(), ()).is_none() {
            unique.push(value);
        }
    }
    unique
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_scenario_reproduces_expected_primary_rule_and_ladder() {
        let rows = build_hinge_feature_rows(MAIN_BASES);
        let scenario = default_hinge_robustness_scenarios()
            .into_iter()
            .find(|scenario| scenario.id == "baseline_main")
            .expect("baseline scenario should exist");
        let run = run_hinge_robustness_scenario(
            &rows,
            &scenario,
            MAX_RULE_ATOMS,
            EXPORTED_RULE_FRONTIER,
            BEST_RULES_PER_SEARCH,
        );
        let best = run
            .best_primary_rule
            .as_ref()
            .expect("baseline should keep an exact primary rule");
        assert_eq!(
            best.rule_label,
            "m1 anomaly_mass_pp > 0 AND m2 boundary_prime_delta_count <= 0"
        );

        let overlap = family_snapshot(&run, HingeAtomFamily::OverlapBoundary);
        let carry = family_snapshot(&run, HingeAtomFamily::CarryThrough);
        let threshold = family_snapshot(&run, HingeAtomFamily::ThresholdShape);
        let geometry = family_snapshot(&run, HingeAtomFamily::Geometry);
        let template = family_snapshot(&run, HingeAtomFamily::TemplateChoice);

        assert_eq!(overlap.depth_label, HingeFamilyDepthLabel::Deepest);
        assert_eq!(carry.depth_label, HingeFamilyDepthLabel::Bridge);
        assert_eq!(threshold.depth_label, HingeFamilyDepthLabel::Bridge);
        assert_eq!(geometry.depth_label, HingeFamilyDepthLabel::Diagnostic);
        assert_eq!(template.depth_label, HingeFamilyDepthLabel::Diagnostic);
    }

    #[test]
    fn stability_scenarios_keep_positive_primary_rows() {
        let rows = build_hinge_feature_rows(MAIN_BASES);
        for scenario in default_hinge_robustness_scenarios()
            .into_iter()
            .filter(|scenario| scenario.scenario_kind == HingeScenarioKind::Stability)
        {
            let filtered = primes::validation::hinge_robustness::filter_hinge_rows(
                &rows,
                &scenario.row_filter,
            );
            let analysis = primes::validation::hinge_robustness::analyze_hinge_family_depth(
                &filtered,
                &scenario.atom_catalog_policy,
                MAX_RULE_ATOMS,
                EXPORTED_RULE_FRONTIER,
                BEST_RULES_PER_SEARCH,
            );
            let primary = analysis
                .search_runs
                .get(primes::validation::hinge_atoms::HINGE_SEARCH_PRIMARY)
                .expect("primary run should exist");
            let best = primary
                .best_candidates
                .first()
                .expect("primary best candidate should exist");
            assert!(
                best.true_positive + best.false_negative > 0,
                "scenario {} removed the entire positive class",
                scenario.id
            );
        }
    }

    #[test]
    fn stability_scenarios_do_not_promote_geometry_or_template() {
        let rows = build_hinge_feature_rows(MAIN_BASES);
        for scenario in default_hinge_robustness_scenarios()
            .into_iter()
            .filter(|scenario| scenario.scenario_kind == HingeScenarioKind::Stability)
        {
            let run = run_hinge_robustness_scenario(
                &rows,
                &scenario,
                MAX_RULE_ATOMS,
                EXPORTED_RULE_FRONTIER,
                BEST_RULES_PER_SEARCH,
            );
            assert_eq!(
                family_snapshot(&run, HingeAtomFamily::Geometry).depth_label,
                HingeFamilyDepthLabel::Diagnostic
            );
            assert_eq!(
                family_snapshot(&run, HingeAtomFamily::TemplateChoice).depth_label,
                HingeFamilyDepthLabel::Diagnostic
            );
        }
    }

    #[test]
    fn no_overlap_boundary_is_adversarial_only_and_not_counted_in_stability_tally() {
        let rows = build_hinge_feature_rows(MAIN_BASES);
        let scenarios = default_hinge_robustness_scenarios();
        let baseline = scenarios
            .iter()
            .find(|scenario| scenario.id == "baseline_main")
            .expect("baseline scenario should exist");
        let baseline_run = run_hinge_robustness_scenario(
            &rows,
            baseline,
            MAX_RULE_ATOMS,
            EXPORTED_RULE_FRONTIER,
            BEST_RULES_PER_SEARCH,
        );
        let no_overlap = scenarios
            .iter()
            .find(|scenario| scenario.id == "no_overlap_boundary")
            .expect("no_overlap_boundary scenario should exist");
        let run = run_hinge_robustness_scenario(
            &rows,
            no_overlap,
            MAX_RULE_ATOMS,
            EXPORTED_RULE_FRONTIER,
            BEST_RULES_PER_SEARCH,
        );
        let row = build_scenario_row(
            no_overlap,
            &run,
            &baseline_run.family_ladder_signature,
            baseline_run
                .best_primary_rule
                .as_ref()
                .expect("baseline rule should exist")
                .rule_label
                .as_str(),
        );
        assert_eq!(row.scenario_status, "fail");
        assert!(!row.counted_in_stability_tally);
    }

    #[test]
    fn appendix_base34_remains_non_hinge_like_under_stability_audits() {
        let main_rows = build_hinge_feature_rows(MAIN_BASES);
        let appendix_rows = build_hinge_feature_rows(APPENDIX_BASES);
        for scenario in default_hinge_robustness_scenarios()
            .into_iter()
            .filter(|scenario| scenario.scenario_kind == HingeScenarioKind::Stability)
        {
            let run = run_hinge_robustness_scenario(
                &main_rows,
                &scenario,
                MAX_RULE_ATOMS,
                EXPORTED_RULE_FRONTIER,
                BEST_RULES_PER_SEARCH,
            );
            for row in build_appendix_audit_rows(
                &ScenarioRunBundle {
                    scenario: scenario.clone(),
                    run,
                    scenario_row: RobustnessScenarioRow {
                        scenario_id: String::new(),
                        scenario_label: String::new(),
                        scenario_group: scenario.group,
                        scenario_kind: scenario.scenario_kind,
                        counted_in_stability_tally: true,
                        threshold_policy: String::new(),
                        included_families: String::new(),
                        primary_dataset_rows: 0,
                        primary_positive_rows: 0,
                        any_exact_primary_rule: false,
                        primary_rule_label: String::new(),
                        primary_rule_unchanged: false,
                        family_ladder_stable: false,
                        overlap_boundary_deepest: false,
                        overlap_boundary_closest_to_theorem: false,
                        carry_through_bridge: false,
                        threshold_shape_bridge: false,
                        geometry_diagnostic: false,
                        template_choice_diagnostic: false,
                        scenario_status: String::new(),
                    },
                },
                &appendix_rows,
            ) {
                if row.appendix_base == 34 {
                    assert_eq!(row.false_positive, 0, "scenario {}", row.scenario_id);
                    assert_eq!(row.false_negative, 0, "scenario {}", row.scenario_id);
                }
            }
        }
    }
}
