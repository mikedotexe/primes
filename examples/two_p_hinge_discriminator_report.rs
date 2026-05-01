//! Exact hinge discriminator pass for the `B = 2p` lane.
//!
//! This report does two things:
//! - searches for the smallest non-tautological exact rule behind the current
//!   `persistent_core` hinge behavior
//! - explains the fixed representative near-miss species with a light atlas
//!
//! The hinge is treated as an arithmetic problem with two axes:
//! - `M=1 -> M=2` persistence
//! - `M=2` shared-overlap dominance
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example two_p_hinge_discriminator_report
//! cargo run --release --example two_p_hinge_discriminator_report -- --out-dir /tmp/primes_two_p_hinge_discriminator_alt
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{
        analyze_hinge_feature_row, digit_symbol, ordered_unit_pairs, HingeFeatureRow,
        HINGE_CATEGORY_ACTIVE_NEITHER, HINGE_CATEGORY_CORE_ONLY, HINGE_CATEGORY_PERSISTENCE_ONLY,
        HINGE_CATEGORY_PERSISTENT_CORE,
    },
    hinge_atoms::{
        build_hinge_atom_specs, build_hinge_search_problems, run_hinge_rule_search,
        HingeRuleCandidate, HINGE_SEARCH_CORE as SEARCH_CORE,
        HINGE_SEARCH_PERSISTENT as SEARCH_PERSISTENT, HINGE_SEARCH_PRIMARY as SEARCH_PRIMARY,
    },
    reporting::{
        ensure_dir, export_timestamp_utc, write_artifact_manifest, write_csv_rows,
        write_json_pretty, write_text_file, ArtifactManifest,
    },
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
};

const MAIN_BASES: &[u32] = &[10, 14, 22, 26];
const APPENDIX_BASES: &[u32] = &[34, 6];
const DEFAULT_OUT_DIR: &str = "/tmp/primes_two_p_hinge_discriminator";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "two_p_hinge_discriminator_report";
const MAX_RULE_ATOMS: usize = 3;
const EXPORTED_RULE_FRONTIER: usize = 60;
const BEST_RULES_PER_SEARCH: usize = 5;

const REPRESENTATIVES: &[RepresentativeSpec] = &[
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

#[derive(Debug)]
struct Options {
    out_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSettings {
    out_dir: String,
    main_bases: Vec<u32>,
    appendix_bases: Vec<u32>,
    middle_lengths: Vec<usize>,
    pair_catalog_mode: String,
    max_rule_atoms: usize,
}

#[derive(Debug, Clone, Serialize)]
struct RuleCandidateRow {
    search_id: String,
    search_label: String,
    selection_mode: String,
    rank: usize,
    dataset_rows: usize,
    positive_rows: usize,
    atom_count: usize,
    rule_label: String,
    exact_match: bool,
    total_errors: usize,
    true_positive: usize,
    false_positive: usize,
    false_negative: usize,
    true_negative: usize,
    precision: f64,
    recall: f64,
    f1: f64,
    jaccard: f64,
    positive_support: usize,
    complexity_score: usize,
}

#[derive(Debug, Clone, Serialize)]
struct RepresentativeRow {
    role: String,
    base: u32,
    pair_label: String,
    hinge_category: String,
    best_k_m1: String,
    best_k_m2: String,
    m1_anomaly_mass_pp: f64,
    m2_anomaly_mass_pp: f64,
    m1_stable_zero_signal_margin_count: isize,
    m2_stable_zero_signal_margin_count: isize,
    m2_stable_zero_prime_delta_count: isize,
    m2_boundary_prime_delta_count: isize,
    m2_stable_zero_count: usize,
    m2_gain_zero_count: usize,
    m2_loss_zero_count: usize,
    m2_stable_nonzero_count: usize,
    m2_nonzero_churn_count: usize,
    exact_outcome_label: String,
}

#[derive(Debug, Clone, Serialize)]
struct AppendixBaseRow {
    base: u32,
    ordered_pair_count: usize,
    active_pair_count: usize,
    persistent_pair_count: usize,
    shared_yield_core_pairs: usize,
    mean_m2_stable_zero_prime_delta_pp_given_active: Option<f64>,
    mean_m2_boundary_prime_delta_pp_given_active: Option<f64>,
    base_note: String,
}

#[derive(Debug, Clone, Serialize)]
struct SearchSummaryRow {
    search_id: String,
    search_label: String,
    dataset_rows: usize,
    positive_rows: usize,
    atom_pool_size: usize,
    searched_rule_count: usize,
    any_exact_rule: bool,
    best_rule_label: String,
    best_rule_status: String,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    main_pair_rows: usize,
    main_active_pair_rows: usize,
    primary_search_status: String,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    hinge_feature_rows: Vec<HingeFeatureRow>,
    rule_candidate_rows: Vec<RuleCandidateRow>,
    best_rules: Vec<RuleCandidateRow>,
    representative_rows: Vec<RepresentativeRow>,
    appendix_base_rows: Vec<AppendixBaseRow>,
    search_summary_rows: Vec<SearchSummaryRow>,
    image_artifact_rows: Vec<ImageArtifactRow>,
    report_summary: ReportSummary,
    observations: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
struct RepresentativeSpec {
    role: &'static str,
    base: u32,
    outer: u32,
    inner: u32,
}

#[derive(Debug, Clone)]
struct SearchOutcome {
    summary_row: SearchSummaryRow,
    candidate_rows: Vec<RuleCandidateRow>,
    best_rows: Vec<RuleCandidateRow>,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        main_bases: MAIN_BASES.to_vec(),
        appendix_bases: APPENDIX_BASES.to_vec(),
        middle_lengths: vec![1, 2],
        pair_catalog_mode: "full".to_string(),
        max_rule_atoms: MAX_RULE_ATOMS,
    };

    let mut hinge_feature_rows = build_hinge_feature_rows(MAIN_BASES);
    hinge_feature_rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
    });

    let appendix_rows = build_hinge_feature_rows(APPENDIX_BASES);
    let search_outcomes = build_search_outcomes(&hinge_feature_rows);
    let mut rule_candidate_rows = search_outcomes
        .iter()
        .flat_map(|outcome| outcome.candidate_rows.clone())
        .collect::<Vec<_>>();
    let mut best_rules = search_outcomes
        .iter()
        .flat_map(|outcome| outcome.best_rows.clone())
        .collect::<Vec<_>>();
    let search_summary_rows = search_outcomes
        .iter()
        .map(|outcome| outcome.summary_row.clone())
        .collect::<Vec<_>>();
    let representative_rows = build_representative_rows(&hinge_feature_rows);
    let appendix_base_rows = build_appendix_base_rows(&appendix_rows);

    rule_candidate_rows.sort_by(rule_row_sort_key);
    best_rules.sort_by(rule_row_sort_key);

    let hinge_plane_path = options.out_dir.join("hinge_discriminator_plane.png");
    render_hinge_discriminator_plane(&hinge_feature_rows, &hinge_plane_path);
    let rule_frontier_grid_path = options.out_dir.join("rule_frontier_grid.png");
    render_rule_frontier_grid(&search_summary_rows, &best_rules, &rule_frontier_grid_path);
    let representative_strip_path = options.out_dir.join("representative_hinge_strip.png");
    render_representative_hinge_strip(&representative_rows, &representative_strip_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "hinge_discriminator_plane".to_string(),
            label: "Hinge discriminator plane".to_string(),
            path: hinge_plane_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "rule_frontier_grid".to_string(),
            label: "Rule frontier grid".to_string(),
            path: rule_frontier_grid_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "representative_hinge_strip".to_string(),
            label: "Representative hinge strip".to_string(),
            path: representative_strip_path.display().to_string(),
        },
    ];

    let report_summary =
        build_report_summary(&hinge_feature_rows, &search_summary_rows, &best_rules);
    let observations = derive_observations(
        &hinge_feature_rows,
        &search_summary_rows,
        &best_rules,
        &representative_rows,
        &appendix_base_rows,
    );

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        hinge_feature_rows: hinge_feature_rows.clone(),
        rule_candidate_rows: rule_candidate_rows.clone(),
        best_rules: best_rules.clone(),
        representative_rows: representative_rows.clone(),
        appendix_base_rows: appendix_base_rows.clone(),
        search_summary_rows: search_summary_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("hinge_feature_rows.csv"),
        &hinge_feature_rows,
    )
    .expect("failed to write hinge_feature_rows.csv");
    write_csv_rows(
        options.out_dir.join("rule_candidate_rows.csv"),
        &rule_candidate_rows,
    )
    .expect("failed to write rule_candidate_rows.csv");
    write_csv_rows(options.out_dir.join("best_rules.csv"), &best_rules)
        .expect("failed to write best_rules.csv");
    write_csv_rows(
        options.out_dir.join("representative_rows.csv"),
        &representative_rows,
    )
    .expect("failed to write representative_rows.csv");
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
                "two_p_hinge_discriminator_report".to_string(),
                "--".to_string(),
                "--out-dir".to_string(),
                options.out_dir.display().to_string(),
            ],
            upstream_inputs: vec![],
            expected_outputs: vec![
                "hinge_feature_rows.csv".to_string(),
                "rule_candidate_rows.csv".to_string(),
                "best_rules.csv".to_string(),
                "representative_rows.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
                "artifact_manifest.json".to_string(),
                "hinge_discriminator_plane.png".to_string(),
                "rule_frontier_grid.png".to_string(),
                "representative_hinge_strip.png".to_string(),
            ],
        },
    )
    .expect("failed to write artifact manifest");

    println!("2p hinge discriminator report");
    println!("  output dir: {}", options.out_dir.display());
    for row in &search_summary_rows {
        println!(
            "  {} | rows {:>2} | positives {:>2} | atoms {:>3} | searched {:>5} | status {} | best {}",
            row.search_id,
            row.dataset_rows,
            row.positive_rows,
            row.atom_pool_size,
            row.searched_rule_count,
            row.best_rule_status,
            row.best_rule_label,
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
    println!("  cargo run --release --example two_p_hinge_discriminator_report -- [options]");
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

fn build_search_outcomes(rows: &[HingeFeatureRow]) -> Vec<SearchOutcome> {
    build_hinge_search_problems(rows)
        .into_iter()
        .map(|problem| {
            let atoms = build_hinge_atom_specs(&problem);
            let outcome = run_hinge_rule_search(
                &problem,
                &atoms,
                MAX_RULE_ATOMS,
                EXPORTED_RULE_FRONTIER,
                BEST_RULES_PER_SEARCH,
            );
            let any_exact_rule = outcome.summary.any_exact_rule;
            let candidate_rows = outcome
                .candidate_rows
                .iter()
                .enumerate()
                .map(|(index, row)| {
                    shared_rule_candidate_to_row(
                        row,
                        "candidate_frontier",
                        index + 1,
                        problem.rows.len(),
                        problem.target.iter().filter(|&&value| value).count(),
                    )
                })
                .collect::<Vec<_>>();
            let best_rows = outcome
                .best_rows
                .iter()
                .enumerate()
                .map(|(index, row)| {
                    shared_rule_candidate_to_row(
                        row,
                        if any_exact_rule {
                            "best_exact"
                        } else {
                            "no_exact_rule_frontier"
                        },
                        index + 1,
                        problem.rows.len(),
                        problem.target.iter().filter(|&&value| value).count(),
                    )
                })
                .collect::<Vec<_>>();

            SearchOutcome {
                summary_row: SearchSummaryRow {
                    search_id: outcome.summary.search_id,
                    search_label: outcome.summary.search_label,
                    dataset_rows: outcome.summary.dataset_rows,
                    positive_rows: outcome.summary.positive_rows,
                    atom_pool_size: outcome.summary.atom_pool_size,
                    searched_rule_count: outcome.summary.searched_rule_count,
                    any_exact_rule: outcome.summary.any_exact_rule,
                    best_rule_label: outcome.summary.best_rule_label,
                    best_rule_status: outcome.summary.best_rule_status,
                },
                candidate_rows,
                best_rows,
            }
        })
        .collect::<Vec<_>>()
}

fn shared_rule_candidate_to_row(
    candidate: &HingeRuleCandidate,
    selection_mode: &str,
    rank: usize,
    dataset_rows: usize,
    positive_rows: usize,
) -> RuleCandidateRow {
    RuleCandidateRow {
        search_id: candidate.search_id.clone(),
        search_label: candidate.search_label.clone(),
        selection_mode: selection_mode.to_string(),
        rank,
        dataset_rows,
        positive_rows,
        atom_count: candidate.atom_count,
        rule_label: candidate.rule_label.clone(),
        exact_match: candidate.exact_match,
        total_errors: candidate.total_errors,
        true_positive: candidate.true_positive,
        false_positive: candidate.false_positive,
        false_negative: candidate.false_negative,
        true_negative: candidate.true_negative,
        precision: candidate.precision,
        recall: candidate.recall,
        f1: candidate.f1,
        jaccard: candidate.jaccard,
        positive_support: candidate.positive_support,
        complexity_score: candidate.complexity_score,
    }
}

fn build_representative_rows(rows: &[HingeFeatureRow]) -> Vec<RepresentativeRow> {
    REPRESENTATIVES
        .iter()
        .map(|spec| {
            let row = if MAIN_BASES.contains(&spec.base) {
                rows.iter()
                    .find(|row| {
                        row.base == spec.base && row.outer == spec.outer && row.inner == spec.inner
                    })
                    .expect("main representative should exist")
                    .clone()
            } else {
                analyze_hinge_feature_row(spec.base, spec.outer, spec.inner)
            };
            RepresentativeRow {
                role: spec.role.to_string(),
                base: spec.base,
                pair_label: format!(
                    "({},{})",
                    digit_symbol(spec.outer),
                    digit_symbol(spec.inner)
                ),
                hinge_category: row.hinge_category.clone(),
                best_k_m1: row.m1_best_k.clone(),
                best_k_m2: row.m2_best_k.clone(),
                m1_anomaly_mass_pp: row.m1_anomaly_mass_pp,
                m2_anomaly_mass_pp: row.m2_anomaly_mass_pp,
                m1_stable_zero_signal_margin_count: row.m1_stable_zero_signal_margin_count,
                m2_stable_zero_signal_margin_count: row.m2_stable_zero_signal_margin_count,
                m2_stable_zero_prime_delta_count: row.m2_stable_zero_prime_delta_count,
                m2_boundary_prime_delta_count: row.m2_boundary_prime_delta_count,
                m2_stable_zero_count: row.m2_stable_zero_count,
                m2_gain_zero_count: row.m2_gain_zero_count,
                m2_loss_zero_count: row.m2_loss_zero_count,
                m2_stable_nonzero_count: row.m2_stable_nonzero_count,
                m2_nonzero_churn_count: row.m2_nonzero_churn_count,
                exact_outcome_label: representative_outcome_label(&row).to_string(),
            }
        })
        .collect()
}

fn build_appendix_base_rows(rows: &[HingeFeatureRow]) -> Vec<AppendixBaseRow> {
    let mut by_base = BTreeMap::<u32, Vec<&HingeFeatureRow>>::new();
    for row in rows {
        by_base.entry(row.base).or_default().push(row);
    }

    APPENDIX_BASES
        .iter()
        .copied()
        .filter_map(|base| {
            let group = by_base.get(&base)?;
            let active_rows = group
                .iter()
                .copied()
                .filter(|row| row.m2_active)
                .collect::<Vec<_>>();
            Some(AppendixBaseRow {
                base,
                ordered_pair_count: group.len(),
                active_pair_count: active_rows.len(),
                persistent_pair_count: group.iter().filter(|row| row.m1_to_m2_persistent).count(),
                shared_yield_core_pairs: group.iter().filter(|row| row.shared_yield_core).count(),
                mean_m2_stable_zero_prime_delta_pp_given_active: mean_option(
                    active_rows
                        .iter()
                        .map(|row| row.m2_stable_zero_prime_delta_pp),
                ),
                mean_m2_boundary_prime_delta_pp_given_active: mean_option(
                    active_rows.iter().map(|row| row.m2_boundary_prime_delta_pp),
                ),
                base_note: appendix_note(base, group.iter().copied()),
            })
        })
        .collect()
}

fn build_report_summary(
    rows: &[HingeFeatureRow],
    search_summary_rows: &[SearchSummaryRow],
    best_rules: &[RuleCandidateRow],
) -> ReportSummary {
    let primary_summary = search_summary_rows
        .iter()
        .find(|row| row.search_id == SEARCH_PRIMARY)
        .expect("primary search summary should exist");
    let primary_best = best_rules
        .iter()
        .find(|row| row.search_id == SEARCH_PRIMARY)
        .expect("primary best rule should exist");

    ReportSummary {
        main_pair_rows: rows.len(),
        main_active_pair_rows: rows.iter().filter(|row| row.m2_active).count(),
        primary_search_status: primary_summary.best_rule_status.clone(),
        main_takeaway: if primary_summary.any_exact_rule {
            format!(
                "The primary hinge search found an exact small-rule separator: `{}`.",
                primary_best.rule_label
            )
        } else {
            format!(
                "The primary hinge search found `no_exact_rule`; the best frontier rule is `{}` with `tp/fp/fn = {}/{}/{}`.",
                primary_best.rule_label,
                primary_best.true_positive,
                primary_best.false_positive,
                primary_best.false_negative
            )
        },
    }
}

fn derive_observations(
    rows: &[HingeFeatureRow],
    search_summary_rows: &[SearchSummaryRow],
    best_rules: &[RuleCandidateRow],
    representative_rows: &[RepresentativeRow],
    appendix_base_rows: &[AppendixBaseRow],
) -> Vec<String> {
    let persistent_core_rows = rows
        .iter()
        .filter(|row| row.hinge_category == HINGE_CATEGORY_PERSISTENT_CORE)
        .collect::<Vec<_>>();
    let persistence_only_rows = rows
        .iter()
        .filter(|row| row.hinge_category == HINGE_CATEGORY_PERSISTENCE_ONLY)
        .collect::<Vec<_>>();
    let core_only_rows = rows
        .iter()
        .filter(|row| row.hinge_category == HINGE_CATEGORY_CORE_ONLY)
        .collect::<Vec<_>>();
    let primary_summary = search_summary_rows
        .iter()
        .find(|row| row.search_id == SEARCH_PRIMARY)
        .expect("primary search summary should exist");
    let primary_best = best_rules
        .iter()
        .find(|row| row.search_id == SEARCH_PRIMARY)
        .expect("primary best rule should exist");
    let persistent_best = best_rules
        .iter()
        .find(|row| row.search_id == SEARCH_PERSISTENT)
        .expect("persistent split best rule should exist");
    let core_best = best_rules
        .iter()
        .find(|row| row.search_id == SEARCH_CORE)
        .expect("core split best rule should exist");
    let base34 = appendix_base_rows
        .iter()
        .find(|row| row.base == 34)
        .expect("base 34 appendix row should exist");

    vec![
        if primary_summary.any_exact_rule {
            format!(
                "The primary search found an exact non-tautological separator for `persistent_core`: `{}`.",
                primary_best.rule_label
            )
        } else {
            format!(
                "The primary search found `no_exact_rule`. The best frontier rule is `{}` with `tp/fp/fn = {}/{}/{}`; the hinge still resists collapse to one tiny exact predicate on the current feature surface.",
                primary_best.rule_label,
                primary_best.true_positive,
                primary_best.false_positive,
                primary_best.false_negative
            )
        },
        format!(
            "The persistent split is cleaner than the global hinge split: best persistent-only rule `{}` has status `{}`.",
            persistent_best.rule_label,
            persistent_best.selection_mode
        ),
        format!(
            "The overlap/core-like split stays sharp around `M=1` carry-through: best rule `{}` has status `{}`.",
            core_best.rule_label,
            core_best.selection_mode
        ),
        format!(
            "The hinge witnesses remain narrow and exact in species terms: persistent-core rows have mean `M=2` stable-zero margin `{:.2}`, persistence-only rows `{:.2}`, and core-only rows `{:.2}`.",
            mean(persistent_core_rows.iter().map(|row| row.m2_stable_zero_signal_margin_count as f64)),
            mean(persistence_only_rows.iter().map(|row| row.m2_stable_zero_signal_margin_count as f64)),
            mean(core_only_rows.iter().map(|row| row.m2_stable_zero_signal_margin_count as f64)),
        ),
        format!(
            "The appendix outgroup remains a second species instead of a hinge extension: base `34` has `{}` active pairs, `{}` persistent pairs, and mean `M=2` stable-zero vs boundary deltas `{}` / `{}`.",
            base34.active_pair_count,
            base34.persistent_pair_count,
            format_option_pp(base34.mean_m2_stable_zero_prime_delta_pp_given_active),
            format_option_pp(base34.mean_m2_boundary_prime_delta_pp_given_active),
        ),
        format!(
            "The fixed representative strip still tells the clean qualitative story: `{}`.",
            representative_rows
                .iter()
                .map(|row| format!("{} {} {}", row.base, row.pair_label, row.exact_outcome_label))
                .collect::<Vec<_>>()
                .join("; ")
        ),
    ]
}

fn render_hinge_discriminator_plane(rows: &[HingeFeatureRow], path: &Path) {
    let active_rows = rows.iter().filter(|row| row.m2_active).collect::<Vec<_>>();
    let max_x = active_rows
        .iter()
        .map(|row| row.m1_anomaly_mass_pp)
        .fold(0.0_f64, f64::max)
        .max(0.5);
    let min_y = active_rows
        .iter()
        .map(|row| row.m2_stable_zero_signal_margin_count as f64)
        .fold(0.0_f64, f64::min)
        .min(-1.0);
    let max_y = active_rows
        .iter()
        .map(|row| row.m2_stable_zero_signal_margin_count as f64)
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let root = BitMapBackend::new(path, (1180, 760)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill hinge discriminator plane canvas");
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Exact Hinge Discriminator Plane  (x = M1 anomaly, y = M2 stable-zero margin count)",
            ("sans-serif", 26),
        )
        .margin(28)
        .x_label_area_size(70)
        .y_label_area_size(90)
        .build_cartesian_2d(-0.2f64..(max_x + 0.4), (min_y - 1.0)..(max_y + 1.0))
        .expect("failed to build hinge discriminator plane");

    chart
        .configure_mesh()
        .x_desc("M1 anomaly mass (pp)")
        .y_desc("M2 stable-zero signal margin (count)")
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .light_line_style(RGBColor(222, 216, 207))
        .draw()
        .expect("failed to draw hinge discriminator mesh");

    chart
        .draw_series(LineSeries::new(
            vec![(0.0, min_y - 1.0), (0.0, max_y + 1.0)],
            ShapeStyle::from(&RGBColor(130, 130, 130)).stroke_width(2),
        ))
        .expect("failed to draw x=0 reference");
    chart
        .draw_series(LineSeries::new(
            vec![(-0.2, 0.0), (max_x + 0.4, 0.0)],
            ShapeStyle::from(&RGBColor(130, 130, 130)).stroke_width(2),
        ))
        .expect("failed to draw y=0 reference");

    for row in active_rows {
        let radius = (5.0 + row.m2_anomaly_mass_pp * 1.4).round() as i32;
        chart
            .draw_series(std::iter::once(Circle::new(
                (
                    row.m1_anomaly_mass_pp,
                    row.m2_stable_zero_signal_margin_count as f64,
                ),
                radius.max(5),
                ShapeStyle::from(&hinge_color(&row.hinge_category)).filled(),
            )))
            .expect("failed to draw hinge point");

        if is_representative(row.base, row.outer, row.inner) {
            chart
                .draw_series(std::iter::once(Text::new(
                    format!("{} {}", row.base, row.pair_label),
                    (
                        row.m1_anomaly_mass_pp + 0.08,
                        row.m2_stable_zero_signal_margin_count as f64 + 0.22,
                    ),
                    ("sans-serif", 15).into_font().color(&BLACK),
                )))
                .expect("failed to draw representative label");
        }
    }

    root.present()
        .expect("failed to present hinge discriminator plane");
}

fn render_rule_frontier_grid(
    search_summary_rows: &[SearchSummaryRow],
    best_rules: &[RuleCandidateRow],
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1320, 980)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill rule frontier grid canvas");
    let areas = root.split_evenly((3, 1));

    for (area, summary_row) in areas.into_iter().zip(search_summary_rows) {
        let mut rows = best_rules
            .iter()
            .filter(|row| row.search_id == summary_row.search_id)
            .cloned()
            .collect::<Vec<_>>();
        if rows.is_empty() {
            continue;
        }
        rows.sort_by(rule_row_sort_key);
        let labels = rows
            .iter()
            .map(|row| truncate_label(&row.rule_label, 64))
            .collect::<Vec<_>>();
        let mut chart = ChartBuilder::on(&area)
            .caption(
                format!(
                    "{}  [{} | {}]",
                    summary_row.search_label,
                    summary_row.best_rule_status,
                    summary_row.best_rule_label
                ),
                ("sans-serif", 18),
            )
            .margin(22)
            .x_label_area_size(38)
            .y_label_area_size(300)
            .build_cartesian_2d(0.0f64..1.02f64, 0usize..rows.len())
            .expect("failed to build rule frontier panel");

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc("F1 score")
            .y_labels(rows.len())
            .y_label_formatter(&move |value| {
                let index = *value;
                if index < labels.len() {
                    labels[index].clone()
                } else {
                    String::new()
                }
            })
            .label_style(("sans-serif", 13))
            .axis_style(RGBColor(92, 86, 78))
            .draw()
            .expect("failed to draw rule frontier panel mesh");

        for (index, row) in rows.iter().enumerate() {
            let color = if row.exact_match {
                RGBColor(48, 119, 142)
            } else {
                RGBColor(218, 143, 53)
            };
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(0.0, index), (row.f1.max(0.01), index + 1)],
                    ShapeStyle::from(&color).filled(),
                )))
                .expect("failed to draw rule frontier bar");
            chart
                .draw_series(std::iter::once(Text::new(
                    format!(
                        "tp/fp/fn = {}/{}/{}",
                        row.true_positive, row.false_positive, row.false_negative
                    ),
                    (row.f1.min(0.92) + 0.02, index),
                    ("sans-serif", 12).into_font().color(&BLACK),
                )))
                .expect("failed to draw rule frontier annotation");
        }
    }

    root.present()
        .expect("failed to present rule frontier grid");
}

fn render_representative_hinge_strip(rows: &[RepresentativeRow], path: &Path) {
    let root = BitMapBackend::new(path, (1280, 840)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill representative strip canvas");
    let areas = root.split_evenly((2, 1));

    let labels = rows
        .iter()
        .map(|row| format!("{} {}", row.base, row.pair_label))
        .collect::<Vec<_>>();

    let max_anomaly = rows
        .iter()
        .flat_map(|row| [row.m1_anomaly_mass_pp, row.m2_anomaly_mass_pp])
        .fold(0.0_f64, f64::max)
        .max(0.5);
    let min_margin = rows
        .iter()
        .flat_map(|row| {
            [
                row.m1_stable_zero_signal_margin_count as f64,
                row.m2_stable_zero_signal_margin_count as f64,
            ]
        })
        .fold(0.0_f64, f64::min)
        .min(-1.0);
    let max_margin = rows
        .iter()
        .flat_map(|row| {
            [
                row.m1_stable_zero_signal_margin_count as f64,
                row.m2_stable_zero_signal_margin_count as f64,
            ]
        })
        .fold(0.0_f64, f64::max)
        .max(1.0);

    {
        let mut chart = ChartBuilder::on(&areas[0])
            .caption("Representative anomaly strip", ("sans-serif", 22))
            .margin(22)
            .x_label_area_size(56)
            .y_label_area_size(80)
            .build_cartesian_2d(0.0f64..rows.len() as f64, 0.0f64..(max_anomaly + 1.0))
            .expect("failed to build anomaly strip");

        chart
            .configure_mesh()
            .x_desc("representatives")
            .y_desc("anomaly mass (pp)")
            .x_labels(labels.len())
            .x_label_formatter(&move |value| {
                let index = value.floor() as usize;
                if index < labels.len() {
                    labels[index].clone()
                } else {
                    String::new()
                }
            })
            .label_style(("sans-serif", 14))
            .axis_style(RGBColor(92, 86, 78))
            .light_line_style(RGBColor(222, 216, 207))
            .draw()
            .expect("failed to draw anomaly strip mesh");

        for (index, row) in rows.iter().enumerate() {
            let x = index as f64 + 0.5;
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(x - 0.28, 0.0), (x - 0.06, row.m1_anomaly_mass_pp)],
                    ShapeStyle::from(&RGBColor(72, 118, 174)).filled(),
                )))
                .expect("failed to draw M1 anomaly bar");
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(x + 0.06, 0.0), (x + 0.28, row.m2_anomaly_mass_pp)],
                    ShapeStyle::from(&RGBColor(218, 143, 53)).filled(),
                )))
                .expect("failed to draw M2 anomaly bar");
        }
    }

    {
        let labels = rows
            .iter()
            .map(|row| format!("{} {}", row.base, row.pair_label))
            .collect::<Vec<_>>();
        let mut chart = ChartBuilder::on(&areas[1])
            .caption(
                "Representative stable-zero margin strip",
                ("sans-serif", 22),
            )
            .margin(22)
            .x_label_area_size(56)
            .y_label_area_size(80)
            .build_cartesian_2d(
                0.0f64..rows.len() as f64,
                (min_margin - 1.0)..(max_margin + 1.0),
            )
            .expect("failed to build margin strip");

        chart
            .configure_mesh()
            .x_desc("representatives")
            .y_desc("stable-zero signal margin (count)")
            .x_labels(labels.len())
            .x_label_formatter(&move |value| {
                let index = value.floor() as usize;
                if index < labels.len() {
                    labels[index].clone()
                } else {
                    String::new()
                }
            })
            .label_style(("sans-serif", 14))
            .axis_style(RGBColor(92, 86, 78))
            .light_line_style(RGBColor(222, 216, 207))
            .draw()
            .expect("failed to draw margin strip mesh");

        chart
            .draw_series(LineSeries::new(
                vec![(0.0, 0.0), (rows.len() as f64, 0.0)],
                ShapeStyle::from(&RGBColor(120, 120, 120)).stroke_width(2),
            ))
            .expect("failed to draw margin zero line");

        for (index, row) in rows.iter().enumerate() {
            let x = index as f64 + 0.5;
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [
                        (x - 0.28, 0.0),
                        (x - 0.06, row.m1_stable_zero_signal_margin_count as f64),
                    ],
                    ShapeStyle::from(&RGBColor(72, 118, 174)).filled(),
                )))
                .expect("failed to draw M1 margin bar");
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [
                        (x + 0.06, 0.0),
                        (x + 0.28, row.m2_stable_zero_signal_margin_count as f64),
                    ],
                    ShapeStyle::from(&RGBColor(218, 143, 53)).filled(),
                )))
                .expect("failed to draw M2 margin bar");
        }
    }

    root.present()
        .expect("failed to present representative hinge strip");
}

fn render_markdown(bundle: &ReportBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Exact Hinge Discriminator Pass\n\n");
    markdown.push_str("_Generated from `examples/two_p_hinge_discriminator_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Output directory: `{}`\n- Main bases: `{}`\n- Appendix bases: `{}`\n- Pair catalog mode: `{}`\n- Max rule atoms: `{}`\n\n",
        bundle.settings.out_dir,
        bundle.settings.main_bases.iter().map(u32::to_string).collect::<Vec<_>>().join(", "),
        bundle.settings.appendix_bases.iter().map(u32::to_string).collect::<Vec<_>>().join(", "),
        bundle.settings.pair_catalog_mode,
        bundle.settings.max_rule_atoms
    ));

    markdown.push_str("## Search Summary\n\n");
    markdown
        .push_str("| Search | Rows | Positives | Atoms | Rules searched | Status | Best rule |\n");
    markdown.push_str("|---|---:|---:|---:|---:|---|---|\n");
    for row in &bundle.search_summary_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            row.search_label,
            row.dataset_rows,
            row.positive_rows,
            row.atom_pool_size,
            row.searched_rule_count,
            row.best_rule_status,
            row.best_rule_label
        ));
    }
    markdown.push('\n');

    let primary_status = bundle
        .search_summary_rows
        .iter()
        .find(|row| row.search_id == SEARCH_PRIMARY)
        .expect("primary summary should exist");
    if primary_status.any_exact_rule {
        markdown.push_str("Primary search verdict: `exact_rule`.\n\n");
    } else {
        markdown.push_str("Primary search verdict: `no_exact_rule`.\n\n");
    }

    markdown.push_str("## Best Rules\n\n");
    markdown.push_str("| Search | Mode | Rank | Exact | Rule | tp/fp/fn | F1 |\n");
    markdown.push_str("|---|---|---:|---|---|---|---:|\n");
    for row in &bundle.best_rules {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {}/{}/{} | {:.3} |\n",
            row.search_id,
            row.selection_mode,
            row.rank,
            if row.exact_match { "yes" } else { "no" },
            row.rule_label,
            row.true_positive,
            row.false_positive,
            row.false_negative,
            row.f1,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Representative Atlas\n\n");
    markdown.push_str("| Role | Base | Pair | Category | M1 anomaly | M2 anomaly | M1 margin | M2 margin | M2 stable-zero | M2 boundary | Buckets (S/G/L/SN/C) | Exact outcome |\n");
    markdown.push_str("|---|---:|---|---|---:|---:|---:|---:|---:|---:|---|---|\n");
    for row in &bundle.representative_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {:.2}pp | {:.2}pp | {} | {} | {} | {} | {}/{}/{}/{}/{} | {} |\n",
            row.role,
            row.base,
            row.pair_label,
            row.hinge_category,
            row.m1_anomaly_mass_pp,
            row.m2_anomaly_mass_pp,
            row.m1_stable_zero_signal_margin_count,
            row.m2_stable_zero_signal_margin_count,
            row.m2_stable_zero_prime_delta_count,
            row.m2_boundary_prime_delta_count,
            row.m2_stable_zero_count,
            row.m2_gain_zero_count,
            row.m2_loss_zero_count,
            row.m2_stable_nonzero_count,
            row.m2_nonzero_churn_count,
            row.exact_outcome_label,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Appendix Controls\n\n");
    markdown.push_str("| Base | Active pairs | Persistent pairs | Shared-yield-core pairs | Mean M2 stable-zero | Mean M2 boundary | Note |\n");
    markdown.push_str("|---:|---:|---:|---:|---:|---:|---|\n");
    for row in &bundle.appendix_base_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            row.base,
            row.active_pair_count,
            row.persistent_pair_count,
            row.shared_yield_core_pairs,
            format_option_pp(row.mean_m2_stable_zero_prime_delta_pp_given_active),
            format_option_pp(row.mean_m2_boundary_prime_delta_pp_given_active),
            row.base_note,
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

fn representative_outcome_label(row: &HingeFeatureRow) -> &'static str {
    match (row.m1_to_m2_persistent, row.shared_yield_core) {
        (true, true) => "survives both axes",
        (true, false) => "survives persistence but not overlap dominance",
        (false, true) => "survives overlap dominance but not persistence",
        (false, false) => "misses both",
    }
}

fn appendix_note<'a>(base: u32, rows: impl Iterator<Item = &'a HingeFeatureRow>) -> String {
    let rows = rows.collect::<Vec<_>>();
    let active_rows = rows
        .iter()
        .copied()
        .filter(|row| row.m2_active)
        .collect::<Vec<_>>();
    if active_rows.is_empty() {
        return "inactive control".to_string();
    }
    let signal_source = most_common_label(
        active_rows
            .iter()
            .map(|row| row.m2_signal_source_label.clone()),
    );
    let category_mix = rows
        .iter()
        .filter(|row| row.m2_active)
        .map(|row| row.hinge_category.clone())
        .collect::<Vec<_>>()
        .join(", ");
    format!("base {base}: source {signal_source}; active mix {category_mix}")
}

fn rule_row_sort_key(left: &RuleCandidateRow, right: &RuleCandidateRow) -> Ordering {
    left.search_id
        .cmp(&right.search_id)
        .then_with(|| left.rank.cmp(&right.rank))
        .then_with(|| left.rule_label.cmp(&right.rule_label))
}

fn mean(values: impl Iterator<Item = f64>) -> f64 {
    let values = values.collect::<Vec<_>>();
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn mean_option(values: impl Iterator<Item = f64>) -> Option<f64> {
    let values = values.collect::<Vec<_>>();
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

fn format_option_pp(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.2}pp"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn most_common_label(labels: impl Iterator<Item = String>) -> String {
    let mut counts = BTreeMap::<String, usize>::new();
    for label in labels {
        *counts.entry(label).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by(|left, right| left.1.cmp(&right.1).then_with(|| right.0.cmp(&left.0)))
        .map(|(label, _)| label)
        .unwrap_or_else(|| "n/a".to_string())
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

fn hinge_color(category: &str) -> RGBColor {
    match category {
        HINGE_CATEGORY_PERSISTENT_CORE => RGBColor(48, 119, 142),
        HINGE_CATEGORY_PERSISTENCE_ONLY => RGBColor(218, 143, 53),
        HINGE_CATEGORY_CORE_ONLY => RGBColor(181, 76, 64),
        HINGE_CATEGORY_ACTIVE_NEITHER => RGBColor(122, 122, 122),
        _ => RGBColor(122, 122, 122),
    }
}

fn is_representative(base: u32, outer: u32, inner: u32) -> bool {
    REPRESENTATIVES
        .iter()
        .any(|spec| spec.base == base && spec.outer == outer && spec.inner == inner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_search_excludes_banned_shortcuts() {
        let rows = build_hinge_feature_rows(MAIN_BASES);
        let problems = build_hinge_search_problems(&rows);
        let primary = problems
            .iter()
            .find(|problem| problem.id == SEARCH_PRIMARY)
            .expect("primary search should exist");
        let atoms = build_hinge_atom_specs(primary);
        assert!(atoms
            .iter()
            .all(|atom| !atom.label.contains("hinge_category")));
        assert!(atoms
            .iter()
            .all(|atom| !atom.label.contains("shared_yield_core")));
        assert!(atoms
            .iter()
            .all(|atom| !atom.label.contains("m1_to_m2_persistent")));
    }

    #[test]
    fn search_summary_reproduces_current_main_species() {
        let rows = build_hinge_feature_rows(MAIN_BASES);
        let mut by_base = BTreeMap::<u32, Vec<&HingeFeatureRow>>::new();
        for row in rows.iter().filter(|row| row.m2_active) {
            by_base.entry(row.base).or_default().push(row);
        }

        let base14 = by_base.get(&14).expect("base 14 should have active rows");
        assert!(base14
            .iter()
            .any(|row| row.hinge_category == HINGE_CATEGORY_PERSISTENT_CORE));

        let base10 = by_base.get(&10).expect("base 10 should have active rows");
        assert!(base10
            .iter()
            .any(|row| row.hinge_category == HINGE_CATEGORY_PERSISTENCE_ONLY));
        assert!(base10
            .iter()
            .all(|row| row.hinge_category != HINGE_CATEGORY_PERSISTENT_CORE));

        let base26 = by_base.get(&26).expect("base 26 should have active rows");
        assert!(base26
            .iter()
            .any(|row| row.hinge_category == HINGE_CATEGORY_CORE_ONLY));
        assert!(base26
            .iter()
            .all(|row| row.hinge_category != HINGE_CATEGORY_PERSISTENT_CORE));

        let base22 = by_base.get(&22).expect("base 22 should have active rows");
        assert!(base22
            .iter()
            .all(|row| row.hinge_category == HINGE_CATEGORY_ACTIVE_NEITHER));
    }

    #[test]
    fn primary_search_still_finds_current_exact_separator() {
        let rows = build_hinge_feature_rows(MAIN_BASES);
        let problems = build_hinge_search_problems(&rows);
        let primary = problems
            .iter()
            .find(|problem| problem.id == SEARCH_PRIMARY)
            .expect("primary search should exist");
        let atoms = build_hinge_atom_specs(primary);
        let outcome = run_hinge_rule_search(
            primary,
            &atoms,
            MAX_RULE_ATOMS,
            EXPORTED_RULE_FRONTIER,
            BEST_RULES_PER_SEARCH,
        );
        let best = outcome
            .best_rows
            .first()
            .expect("primary search should return a best rule");

        assert!(outcome.summary.any_exact_rule);
        assert_eq!(
            best.rule_label,
            "m1 anomaly_mass_pp > 0 AND m2 boundary_prime_delta_count <= 0"
        );
    }

    #[test]
    fn representative_rows_keep_expected_outcomes() {
        let rows = build_hinge_feature_rows(MAIN_BASES);
        let representatives = build_representative_rows(&rows);
        let db = representatives
            .iter()
            .find(|row| row.base == 14 && row.pair_label == "(D,B)")
            .expect("base 14 (D,B) representative should exist");
        let p33 = representatives
            .iter()
            .find(|row| row.base == 10 && row.pair_label == "(3,3)")
            .expect("base 10 (3,3) representative should exist");
        let nn = representatives
            .iter()
            .find(|row| row.base == 26 && row.pair_label == "(N,N)")
            .expect("base 26 (N,N) representative should exist");
        let hj = representatives
            .iter()
            .find(|row| row.base == 22 && row.pair_label == "(H,J)")
            .expect("base 22 (H,J) representative should exist");

        assert_eq!(db.exact_outcome_label, "survives both axes");
        assert_eq!(
            p33.exact_outcome_label,
            "survives persistence but not overlap dominance"
        );
        assert_eq!(
            nn.exact_outcome_label,
            "survives overlap dominance but not persistence"
        );
        assert_eq!(hj.exact_outcome_label, "misses both");
    }
}
