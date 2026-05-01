//! Hinge atom-family depth pass.
//!
//! This report classifies the deterministic atom families behind the current
//! hinge surface by:
//! - family-only rule leverage
//! - leave-one-family-out ablation leverage
//! - mixed-rule participation
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --example two_p_hinge_atom_family_report
//! cargo run --release --example two_p_hinge_atom_family_report -- --out-dir /tmp/primes_two_p_hinge_atom_family_alt
//! ```

use plotters::prelude::*;
use primes::validation::{
    bounded_k::{analyze_hinge_feature_row, digit_symbol, ordered_unit_pairs, HingeFeatureRow},
    hinge_atoms::{
        build_hinge_atom_specs, build_hinge_search_problems, enumerate_hinge_rule_candidates,
        rule_candidate_exact_sort, rule_candidate_quality_sort, run_hinge_rule_search,
        HingeAtomFamily, HingeAtomSpec, HingeAtomTheoremClass, HingeRuleCandidate,
        HingeSearchProblem, HINGE_SEARCH_CORE, HINGE_SEARCH_PERSISTENT, HINGE_SEARCH_PRIMARY,
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
const DEFAULT_OUT_DIR: &str = "/tmp/primes_two_p_hinge_atom_family";
const REPORT_EXPORT_VERSION: u32 = 1;
const ARTIFACT_ID: &str = "two_p_hinge_atom_family_report";
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

const FAMILIES: &[HingeAtomFamily] = &[
    HingeAtomFamily::OverlapBoundary,
    HingeAtomFamily::CarryThrough,
    HingeAtomFamily::ThresholdShape,
    HingeAtomFamily::Geometry,
    HingeAtomFamily::TemplateChoice,
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
    pair_catalog_mode: String,
    max_rule_atoms: usize,
}

#[derive(Debug, Clone, Serialize)]
struct AtomCatalogRow {
    search_id: String,
    search_label: String,
    atom_label: String,
    family: HingeAtomFamily,
    theorem_class: HingeAtomTheoremClass,
    threshold_based: bool,
    complexity_score: usize,
    true_count: usize,
    false_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct FamilyOnlySearchRow {
    search_id: String,
    search_label: String,
    family: HingeAtomFamily,
    atom_pool_size: usize,
    searched_rule_count: usize,
    any_exact_rule: bool,
    best_rule_label: String,
    best_error_count: usize,
    best_f1: f64,
    best_true_positive: usize,
    best_false_positive: usize,
    best_false_negative: usize,
    best_threshold_free: bool,
    best_rule_theorem_class: Option<HingeAtomTheoremClass>,
}

#[derive(Debug, Clone, Serialize)]
struct FamilyAblationRow {
    search_id: String,
    search_label: String,
    removed_family: HingeAtomFamily,
    baseline_any_exact_rule: bool,
    ablated_any_exact_rule: bool,
    baseline_best_rule_label: String,
    ablated_best_rule_label: String,
    baseline_best_rule_theorem_class: Option<HingeAtomTheoremClass>,
    ablated_best_rule_theorem_class: Option<HingeAtomTheoremClass>,
    destroys_exact_separator: bool,
    best_error_delta: isize,
    atom_count_delta: isize,
    theorem_class_shift_downward: bool,
}

#[derive(Debug, Clone, Serialize)]
struct FamilyMixedRuleRow {
    search_id: String,
    search_label: String,
    family: HingeAtomFamily,
    exact_mixed_rule_count: usize,
    exact_rule_rank_presence: Option<usize>,
    smallest_exact_mixed_rule_atom_count: Option<usize>,
    top_frontier_presence: bool,
}

#[derive(Debug, Clone, Serialize)]
struct FamilyDepthRow {
    family: HingeAtomFamily,
    theorem_class: HingeAtomTheoremClass,
    primary_family_only_exact: bool,
    persistent_family_only_exact: bool,
    core_family_only_exact: bool,
    primary_ablation_breaks_exact: bool,
    primary_exact_mixed_rule_count: usize,
    primary_top_frontier_presence: bool,
    depth_label: String,
    theorem_language_label: String,
    rationale: String,
}

#[derive(Debug, Clone, Serialize)]
struct RepresentativeFamilyRow {
    role: String,
    base: u32,
    pair_label: String,
    hinge_category: String,
    gap_bucket: String,
    same_digit: bool,
    m1_anomaly_mass_pp: f64,
    m2_anomaly_mass_pp: f64,
    m1_stable_zero_signal_margin_count: isize,
    m2_stable_zero_signal_margin_count: isize,
    m2_stable_zero_prime_delta_count: isize,
    m2_boundary_prime_delta_count: isize,
    m1_signal_source_label: String,
    m2_signal_source_label: String,
    mechanism_sentence: String,
}

#[derive(Debug, Clone, Serialize)]
struct ImageArtifactRow {
    kind: String,
    label: String,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportSummary {
    primary_best_rule: String,
    family_depth_ladder: Vec<String>,
    main_takeaway: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBundle {
    export_version: u32,
    generated_at_utc: String,
    settings: ReportSettings,
    atom_catalog_rows: Vec<AtomCatalogRow>,
    family_only_search_rows: Vec<FamilyOnlySearchRow>,
    family_ablation_rows: Vec<FamilyAblationRow>,
    family_mixed_rule_rows: Vec<FamilyMixedRuleRow>,
    family_depth_rows: Vec<FamilyDepthRow>,
    representative_family_rows: Vec<RepresentativeFamilyRow>,
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
struct SearchRunDetail {
    all_candidates: Vec<HingeRuleCandidate>,
    best_candidates: Vec<HingeRuleCandidate>,
    any_exact_rule: bool,
}

fn main() {
    let options = parse_args();
    ensure_dir(&options.out_dir).expect("failed to create output directory");

    let settings = ReportSettings {
        out_dir: options.out_dir.display().to_string(),
        main_bases: MAIN_BASES.to_vec(),
        appendix_bases: APPENDIX_BASES.to_vec(),
        pair_catalog_mode: "full".to_string(),
        max_rule_atoms: MAX_RULE_ATOMS,
    };

    let mut main_rows = build_hinge_feature_rows(MAIN_BASES);
    main_rows.sort_by(|left, right| {
        left.base
            .cmp(&right.base)
            .then_with(|| left.outer.cmp(&right.outer))
            .then_with(|| left.inner.cmp(&right.inner))
    });
    let appendix_rows = build_hinge_feature_rows(APPENDIX_BASES);

    let problems = build_hinge_search_problems(&main_rows);
    let mut atom_catalog_rows = Vec::new();
    let mut family_only_search_rows = Vec::new();
    let mut family_ablation_rows = Vec::new();
    let mut family_mixed_rule_rows = Vec::new();
    let mut baseline_runs = BTreeMap::<String, SearchRunDetail>::new();

    for problem in &problems {
        let atoms = build_hinge_atom_specs(problem);
        atom_catalog_rows.extend(build_atom_catalog_rows(problem, &atoms));
        let baseline_run = run_search_detail(problem, &atoms);
        baseline_runs.insert(problem.id.to_string(), baseline_run.clone());

        for &family in FAMILIES {
            let family_atoms = atoms
                .iter()
                .filter(|atom| atom.family == family)
                .cloned()
                .collect::<Vec<_>>();
            family_only_search_rows.push(build_family_only_row(problem, family, &family_atoms));

            let ablated_atoms = atoms
                .iter()
                .filter(|atom| atom.family != family)
                .cloned()
                .collect::<Vec<_>>();
            family_ablation_rows.push(build_family_ablation_row(
                problem,
                family,
                &baseline_run,
                &ablated_atoms,
            ));
            family_mixed_rule_rows.push(build_family_mixed_rule_row(
                problem,
                family,
                &baseline_run,
            ));
        }
    }

    let family_depth_rows = build_family_depth_rows(
        &family_only_search_rows,
        &family_ablation_rows,
        &family_mixed_rule_rows,
    );
    let representative_family_rows = build_representative_rows(&main_rows, &appendix_rows);

    let family_leverage_heatmap_path = options.out_dir.join("family_leverage_heatmap.png");
    render_family_leverage_heatmap(
        &family_only_search_rows,
        &family_ablation_rows,
        &family_mixed_rule_rows,
        &family_leverage_heatmap_path,
    );
    let family_depth_ladder_path = options.out_dir.join("family_depth_ladder.png");
    render_family_depth_ladder(&family_depth_rows, &family_depth_ladder_path);
    let family_rule_examples_path = options.out_dir.join("family_rule_examples.png");
    render_family_rule_examples(&family_only_search_rows, &family_rule_examples_path);

    let image_artifact_rows = vec![
        ImageArtifactRow {
            kind: "family_leverage_heatmap".to_string(),
            label: "Family leverage heatmap".to_string(),
            path: family_leverage_heatmap_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "family_depth_ladder".to_string(),
            label: "Family depth ladder".to_string(),
            path: family_depth_ladder_path.display().to_string(),
        },
        ImageArtifactRow {
            kind: "family_rule_examples".to_string(),
            label: "Family rule examples".to_string(),
            path: family_rule_examples_path.display().to_string(),
        },
    ];

    atom_catalog_rows.sort_by(|left, right| {
        left.search_id
            .cmp(&right.search_id)
            .then_with(|| left.family.cmp(&right.family))
            .then_with(|| left.atom_label.cmp(&right.atom_label))
    });
    family_only_search_rows.sort_by(|left, right| {
        left.search_id
            .cmp(&right.search_id)
            .then_with(|| left.family.cmp(&right.family))
    });
    family_ablation_rows.sort_by(|left, right| {
        left.search_id
            .cmp(&right.search_id)
            .then_with(|| left.removed_family.cmp(&right.removed_family))
    });
    family_mixed_rule_rows.sort_by(|left, right| {
        left.search_id
            .cmp(&right.search_id)
            .then_with(|| left.family.cmp(&right.family))
    });

    let report_summary = build_report_summary(&baseline_runs, &family_depth_rows);
    let observations = derive_observations(
        &family_only_search_rows,
        &family_ablation_rows,
        &family_mixed_rule_rows,
        &family_depth_rows,
        &representative_family_rows,
    );

    let bundle = ReportBundle {
        export_version: REPORT_EXPORT_VERSION,
        generated_at_utc: export_timestamp_utc(),
        settings,
        atom_catalog_rows: atom_catalog_rows.clone(),
        family_only_search_rows: family_only_search_rows.clone(),
        family_ablation_rows: family_ablation_rows.clone(),
        family_mixed_rule_rows: family_mixed_rule_rows.clone(),
        family_depth_rows: family_depth_rows.clone(),
        representative_family_rows: representative_family_rows.clone(),
        image_artifact_rows: image_artifact_rows.clone(),
        report_summary,
        observations,
    };

    write_csv_rows(
        options.out_dir.join("atom_catalog_rows.csv"),
        &atom_catalog_rows,
    )
    .expect("failed to write atom_catalog_rows.csv");
    write_csv_rows(
        options.out_dir.join("family_only_search_rows.csv"),
        &family_only_search_rows,
    )
    .expect("failed to write family_only_search_rows.csv");
    write_csv_rows(
        options.out_dir.join("family_ablation_rows.csv"),
        &family_ablation_rows,
    )
    .expect("failed to write family_ablation_rows.csv");
    write_csv_rows(
        options.out_dir.join("family_mixed_rule_rows.csv"),
        &family_mixed_rule_rows,
    )
    .expect("failed to write family_mixed_rule_rows.csv");
    write_csv_rows(
        options.out_dir.join("family_depth_rows.csv"),
        &family_depth_rows,
    )
    .expect("failed to write family_depth_rows.csv");
    write_csv_rows(
        options.out_dir.join("representative_family_rows.csv"),
        &representative_family_rows,
    )
    .expect("failed to write representative_family_rows.csv");
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
                "two_p_hinge_atom_family_report".to_string(),
                "--".to_string(),
                "--out-dir".to_string(),
                options.out_dir.display().to_string(),
            ],
            upstream_inputs: vec![],
            expected_outputs: vec![
                "atom_catalog_rows.csv".to_string(),
                "family_only_search_rows.csv".to_string(),
                "family_ablation_rows.csv".to_string(),
                "family_mixed_rule_rows.csv".to_string(),
                "family_depth_rows.csv".to_string(),
                "representative_family_rows.csv".to_string(),
                "summary.json".to_string(),
                "report.md".to_string(),
                "artifact_manifest.json".to_string(),
                "family_leverage_heatmap.png".to_string(),
                "family_depth_ladder.png".to_string(),
                "family_rule_examples.png".to_string(),
            ],
        },
    )
    .expect("failed to write artifact manifest");

    println!("2p hinge atom-family report");
    println!("  output dir: {}", options.out_dir.display());
    for row in &family_depth_rows {
        println!(
            "  {:<18} | {:<10} | {:<24} | {}",
            row.family.as_str(),
            row.depth_label,
            row.theorem_language_label,
            row.rationale
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
    println!("  cargo run --release --example two_p_hinge_atom_family_report -- [options]");
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

fn build_atom_catalog_rows(
    problem: &HingeSearchProblem<'_>,
    atoms: &[HingeAtomSpec],
) -> Vec<AtomCatalogRow> {
    atoms
        .iter()
        .map(|atom| AtomCatalogRow {
            search_id: problem.id.to_string(),
            search_label: problem.label.to_string(),
            atom_label: atom.label.clone(),
            family: atom.family,
            theorem_class: atom.theorem_class,
            threshold_based: atom.threshold_based,
            complexity_score: atom.complexity_score,
            true_count: atom.mask.iter().filter(|&&value| value).count(),
            false_count: atom.mask.iter().filter(|&&value| !value).count(),
        })
        .collect()
}

fn run_search_detail(problem: &HingeSearchProblem<'_>, atoms: &[HingeAtomSpec]) -> SearchRunDetail {
    let mut all_candidates = enumerate_hinge_rule_candidates(problem, atoms, MAX_RULE_ATOMS);
    all_candidates.sort_by(rule_candidate_quality_sort);
    let outcome = run_hinge_rule_search(
        problem,
        atoms,
        MAX_RULE_ATOMS,
        EXPORTED_RULE_FRONTIER,
        BEST_RULES_PER_SEARCH,
    );
    SearchRunDetail {
        all_candidates,
        best_candidates: outcome.best_rows,
        any_exact_rule: outcome.summary.any_exact_rule,
    }
}

fn build_family_only_row(
    problem: &HingeSearchProblem<'_>,
    family: HingeAtomFamily,
    atoms: &[HingeAtomSpec],
) -> FamilyOnlySearchRow {
    let run = run_search_detail(problem, atoms);
    let best = run.best_candidates.first();
    FamilyOnlySearchRow {
        search_id: problem.id.to_string(),
        search_label: problem.label.to_string(),
        family,
        atom_pool_size: atoms.len(),
        searched_rule_count: run.all_candidates.len(),
        any_exact_rule: run.any_exact_rule,
        best_rule_label: best
            .map(|row| row.rule_label.clone())
            .unwrap_or_else(|| "no_atoms".to_string()),
        best_error_count: best
            .map(|row| row.total_errors)
            .unwrap_or(problem.rows.len()),
        best_f1: best.map(|row| row.f1).unwrap_or(0.0),
        best_true_positive: best.map(|row| row.true_positive).unwrap_or(0),
        best_false_positive: best.map(|row| row.false_positive).unwrap_or(0),
        best_false_negative: best.map(|row| row.false_negative).unwrap_or(0),
        best_threshold_free: best.map(|row| row.threshold_free).unwrap_or(false),
        best_rule_theorem_class: best.map(|row| row.rule_theorem_class),
    }
}

fn build_family_ablation_row(
    problem: &HingeSearchProblem<'_>,
    removed_family: HingeAtomFamily,
    baseline: &SearchRunDetail,
    ablated_atoms: &[HingeAtomSpec],
) -> FamilyAblationRow {
    let ablated = run_search_detail(problem, ablated_atoms);
    let baseline_best = baseline.best_candidates.first();
    let ablated_best = ablated.best_candidates.first();
    let baseline_class = baseline_best.map(|row| row.rule_theorem_class);
    let ablated_class = ablated_best.map(|row| row.rule_theorem_class);

    FamilyAblationRow {
        search_id: problem.id.to_string(),
        search_label: problem.label.to_string(),
        removed_family,
        baseline_any_exact_rule: baseline.any_exact_rule,
        ablated_any_exact_rule: ablated.any_exact_rule,
        baseline_best_rule_label: baseline_best
            .map(|row| row.rule_label.clone())
            .unwrap_or_else(|| "none".to_string()),
        ablated_best_rule_label: ablated_best
            .map(|row| row.rule_label.clone())
            .unwrap_or_else(|| "none".to_string()),
        baseline_best_rule_theorem_class: baseline_class,
        ablated_best_rule_theorem_class: ablated_class,
        destroys_exact_separator: baseline.any_exact_rule && !ablated.any_exact_rule,
        best_error_delta: ablated_best
            .map(|row| row.total_errors as isize)
            .unwrap_or(problem.rows.len() as isize)
            - baseline_best
                .map(|row| row.total_errors as isize)
                .unwrap_or(problem.rows.len() as isize),
        atom_count_delta: ablated_best.map(|row| row.atom_count as isize).unwrap_or(0)
            - baseline_best
                .map(|row| row.atom_count as isize)
                .unwrap_or(0),
        theorem_class_shift_downward: match (baseline_class, ablated_class) {
            (Some(left), Some(right)) => right.depth_rank() > left.depth_rank(),
            _ => false,
        },
    }
}

fn build_family_mixed_rule_row(
    problem: &HingeSearchProblem<'_>,
    family: HingeAtomFamily,
    baseline: &SearchRunDetail,
) -> FamilyMixedRuleRow {
    let exact_rules = baseline
        .all_candidates
        .iter()
        .filter(|row| row.exact_match)
        .collect::<Vec<_>>();
    let mut sorted_exact_rules = exact_rules.to_vec();
    sorted_exact_rules.sort_by(|left, right| rule_candidate_exact_sort(left, right));

    FamilyMixedRuleRow {
        search_id: problem.id.to_string(),
        search_label: problem.label.to_string(),
        family,
        exact_mixed_rule_count: exact_rules
            .iter()
            .filter(|row| row.atom_count > 1 && row.atom_families.contains(&family))
            .count(),
        exact_rule_rank_presence: sorted_exact_rules
            .iter()
            .position(|row| row.atom_families.contains(&family))
            .map(|index| index + 1),
        smallest_exact_mixed_rule_atom_count: exact_rules
            .iter()
            .filter(|row| row.atom_count > 1 && row.atom_families.contains(&family))
            .map(|row| row.atom_count)
            .min(),
        top_frontier_presence: baseline
            .best_candidates
            .iter()
            .any(|row| row.atom_families.contains(&family)),
    }
}

fn build_family_depth_rows(
    family_only_rows: &[FamilyOnlySearchRow],
    family_ablation_rows: &[FamilyAblationRow],
    family_mixed_rows: &[FamilyMixedRuleRow],
) -> Vec<FamilyDepthRow> {
    FAMILIES
        .iter()
        .copied()
        .map(|family| {
            let theorem_class = family_default_theorem_class(family);
            let primary_family_only = lookup_family_only_row(
                family_only_rows,
                HINGE_SEARCH_PRIMARY,
                family,
            );
            let persistent_family_only = lookup_family_only_row(
                family_only_rows,
                HINGE_SEARCH_PERSISTENT,
                family,
            );
            let core_family_only =
                lookup_family_only_row(family_only_rows, HINGE_SEARCH_CORE, family);
            let primary_ablation = lookup_family_ablation_row(
                family_ablation_rows,
                HINGE_SEARCH_PRIMARY,
                family,
            );
            let primary_mixed =
                lookup_family_mixed_row(family_mixed_rows, HINGE_SEARCH_PRIMARY, family);

            let depth_label = if theorem_class == HingeAtomTheoremClass::ExactTransferSubstrate
                && (primary_family_only.any_exact_rule
                    || persistent_family_only.any_exact_rule
                    || primary_ablation.destroys_exact_separator)
            {
                "deepest"
            } else if theorem_class == HingeAtomTheoremClass::Diagnostic {
                "diagnostic"
            } else if persistent_family_only.any_exact_rule
                || core_family_only.any_exact_rule
                || primary_mixed.exact_mixed_rule_count > 0
            {
                "bridge"
            } else {
                "diagnostic"
            };

            let theorem_language_label = if depth_label == "deepest"
                && theorem_class == HingeAtomTheoremClass::ExactTransferSubstrate
                && ((primary_family_only.any_exact_rule && primary_family_only.best_threshold_free)
                    || (persistent_family_only.any_exact_rule
                        && persistent_family_only.best_threshold_free))
            {
                "closest_to_theorem"
            } else if depth_label != "diagnostic" {
                "supporting_bridge"
            } else {
                "not_yet_theorem_language"
            };

            let rationale = match family {
                HingeAtomFamily::OverlapBoundary => {
                    "exact transfer-sign family; survives ablation and owns the clean persistent split"
                }
                HingeAtomFamily::CarryThrough => {
                    "M1 carry-through family; exact on the core/persistence split but still leans on the empirical cross-M boundary"
                }
                HingeAtomFamily::ThresholdShape => {
                    "derived threshold family; can win exact finite cutpoint rules, but those rules are not substrate-exact"
                }
                HingeAtomFamily::Geometry => {
                    "residue geometry family; classifies pockets but does not currently explain the hinge on its own"
                }
                HingeAtomFamily::TemplateChoice => {
                    "best-k family; helpful as a descriptor, not yet explanatory at hinge depth"
                }
            }
            .to_string();

            FamilyDepthRow {
                family,
                theorem_class,
                primary_family_only_exact: primary_family_only.any_exact_rule,
                persistent_family_only_exact: persistent_family_only.any_exact_rule,
                core_family_only_exact: core_family_only.any_exact_rule,
                primary_ablation_breaks_exact: primary_ablation.destroys_exact_separator,
                primary_exact_mixed_rule_count: primary_mixed.exact_mixed_rule_count,
                primary_top_frontier_presence: primary_mixed.top_frontier_presence,
                depth_label: depth_label.to_string(),
                theorem_language_label: theorem_language_label.to_string(),
                rationale,
            }
        })
        .collect()
}

fn build_representative_rows(
    main_rows: &[HingeFeatureRow],
    appendix_rows: &[HingeFeatureRow],
) -> Vec<RepresentativeFamilyRow> {
    REPRESENTATIVES
        .iter()
        .map(|spec| {
            let row = if MAIN_BASES.contains(&spec.base) {
                main_rows
                    .iter()
                    .find(|row| {
                        row.base == spec.base && row.outer == spec.outer && row.inner == spec.inner
                    })
                    .expect("main representative should exist")
                    .clone()
            } else {
                appendix_rows
                    .iter()
                    .find(|row| {
                        row.base == spec.base && row.outer == spec.outer && row.inner == spec.inner
                    })
                    .expect("appendix representative should exist")
                    .clone()
            };

            RepresentativeFamilyRow {
                role: spec.role.to_string(),
                base: spec.base,
                pair_label: format!(
                    "({},{})",
                    digit_symbol(spec.outer),
                    digit_symbol(spec.inner)
                ),
                hinge_category: row.hinge_category.clone(),
                gap_bucket: row.gap_bucket.clone(),
                same_digit: row.same_digit,
                m1_anomaly_mass_pp: row.m1_anomaly_mass_pp,
                m2_anomaly_mass_pp: row.m2_anomaly_mass_pp,
                m1_stable_zero_signal_margin_count: row.m1_stable_zero_signal_margin_count,
                m2_stable_zero_signal_margin_count: row.m2_stable_zero_signal_margin_count,
                m2_stable_zero_prime_delta_count: row.m2_stable_zero_prime_delta_count,
                m2_boundary_prime_delta_count: row.m2_boundary_prime_delta_count,
                m1_signal_source_label: row.m1_signal_source_label.clone(),
                m2_signal_source_label: row.m2_signal_source_label.clone(),
                mechanism_sentence: representative_mechanism_sentence(&row).to_string(),
            }
        })
        .collect()
}

fn build_report_summary(
    baseline_runs: &BTreeMap<String, SearchRunDetail>,
    family_depth_rows: &[FamilyDepthRow],
) -> ReportSummary {
    let primary_best_rule = baseline_runs
        .get(HINGE_SEARCH_PRIMARY)
        .and_then(|run| run.best_candidates.first())
        .map(|row| row.rule_label.clone())
        .unwrap_or_else(|| "none".to_string());
    let mut ladder = family_depth_rows.to_vec();
    ladder.sort_by(|left, right| {
        family_depth_rank(&left.depth_label)
            .cmp(&family_depth_rank(&right.depth_label))
            .then_with(|| {
                theorem_language_rank(&left.theorem_language_label)
                    .cmp(&theorem_language_rank(&right.theorem_language_label))
            })
            .then_with(|| left.family.cmp(&right.family))
    });

    ReportSummary {
        primary_best_rule,
        family_depth_ladder: ladder
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}",
                    row.family.as_str(),
                    row.depth_label,
                    row.theorem_language_label
                )
            })
            .collect(),
        main_takeaway: ladder
            .first()
            .map(|row| {
                format!(
                    "The current deepest family is `{}` with theorem-language label `{}`.",
                    row.family.as_str(),
                    row.theorem_language_label
                )
            })
            .unwrap_or_else(|| "No family depth rows were produced.".to_string()),
    }
}

fn derive_observations(
    family_only_rows: &[FamilyOnlySearchRow],
    family_ablation_rows: &[FamilyAblationRow],
    family_mixed_rows: &[FamilyMixedRuleRow],
    family_depth_rows: &[FamilyDepthRow],
    representatives: &[RepresentativeFamilyRow],
) -> Vec<String> {
    let overlap_primary = lookup_family_only_row(
        family_only_rows,
        HINGE_SEARCH_PRIMARY,
        HingeAtomFamily::OverlapBoundary,
    );
    let overlap_persistent = lookup_family_only_row(
        family_only_rows,
        HINGE_SEARCH_PERSISTENT,
        HingeAtomFamily::OverlapBoundary,
    );
    let carry_core = lookup_family_only_row(
        family_only_rows,
        HINGE_SEARCH_CORE,
        HingeAtomFamily::CarryThrough,
    );
    let threshold_primary = lookup_family_only_row(
        family_only_rows,
        HINGE_SEARCH_PRIMARY,
        HingeAtomFamily::ThresholdShape,
    );
    let overlap_ablation = lookup_family_ablation_row(
        family_ablation_rows,
        HINGE_SEARCH_PRIMARY,
        HingeAtomFamily::OverlapBoundary,
    );
    let geometry_mixed = lookup_family_mixed_row(
        family_mixed_rows,
        HINGE_SEARCH_PRIMARY,
        HingeAtomFamily::Geometry,
    );
    let ladder = family_depth_rows
        .iter()
        .map(|row| format!("{}→{}", row.family.as_str(), row.depth_label))
        .collect::<Vec<_>>()
        .join(", ");

    vec![
        format!(
            "The family-depth ladder currently reads `{}`.",
            ladder
        ),
        format!(
            "The overlap/boundary family is the deepest one on the maintained surface because its persistent-split family-only search is `{}` and removing it from the primary search `destroys_exact_separator = {}`.",
            if overlap_persistent.any_exact_rule {
                "exact_rule"
            } else {
                "no_exact_rule"
            },
            overlap_ablation.destroys_exact_separator
        ),
        format!(
            "Carry-through atoms behave like a bridge family: the core/persistence split is `{}` with best rule `{}`.",
            if carry_core.any_exact_rule {
                "exact_rule"
            } else {
                "no_exact_rule"
            },
            carry_core.best_rule_label
        ),
        format!(
            "Threshold-shape atoms remain useful but derived: the primary family-only run is `{}` with best rule `{}`.",
            if threshold_primary.any_exact_rule {
                "exact_rule"
            } else {
                "no_exact_rule"
            },
            threshold_primary.best_rule_label
        ),
        format!(
            "Geometry stays diagnostic on the current hinge surface: primary exact mixed-rule count `{}` and top-frontier presence `{}`.",
            geometry_mixed.exact_mixed_rule_count,
            geometry_mixed.top_frontier_presence
        ),
        format!(
            "The fixed representatives still narrate the species cleanly: `{}`.",
            representatives
                .iter()
                .map(|row| format!("{} {} {}", row.base, row.pair_label, row.mechanism_sentence))
                .collect::<Vec<_>>()
                .join("; ")
        ),
        format!(
            "The primary overlap/boundary family-only run remains `{}`; the hinge still prefers a mixed carry-through plus overlap explanation rather than a one-family primary law.",
            if overlap_primary.any_exact_rule {
                "exact_rule"
            } else {
                "no_exact_rule"
            }
        ),
    ]
}

fn render_family_leverage_heatmap(
    family_only_rows: &[FamilyOnlySearchRow],
    family_ablation_rows: &[FamilyAblationRow],
    family_mixed_rows: &[FamilyMixedRuleRow],
    path: &Path,
) {
    let root = BitMapBackend::new(path, (1180, 820)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill family leverage heatmap");
    let mut chart = ChartBuilder::on(&root)
        .caption("Hinge Atom-Family Leverage Heatmap", ("sans-serif", 26))
        .margin(26)
        .x_label_area_size(80)
        .y_label_area_size(170)
        .build_cartesian_2d(0..3, 0..FAMILIES.len())
        .expect("failed to build family leverage heatmap");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(3)
        .x_label_formatter(&|value| match *value {
            0 => "primary".to_string(),
            1 => "persistent".to_string(),
            2 => "core".to_string(),
            _ => String::new(),
        })
        .y_labels(FAMILIES.len())
        .y_label_formatter(&|value| {
            FAMILIES
                .get(*value)
                .map(|family| family.as_str().to_string())
                .unwrap_or_default()
        })
        .label_style(("sans-serif", 16))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw family leverage mesh");

    for (row_index, family) in FAMILIES.iter().copied().enumerate() {
        for (column_index, search_id) in [
            HINGE_SEARCH_PRIMARY,
            HINGE_SEARCH_PERSISTENT,
            HINGE_SEARCH_CORE,
        ]
        .iter()
        .enumerate()
        {
            let family_only = lookup_family_only_row(family_only_rows, search_id, family);
            let ablation = lookup_family_ablation_row(family_ablation_rows, search_id, family);
            let mixed = lookup_family_mixed_row(family_mixed_rows, search_id, family);
            let score = leverage_score(family_only, ablation, mixed);
            let color = heatmap_color(score / 4.0);
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [
                        (column_index as i32, row_index),
                        (column_index as i32 + 1, row_index + 1),
                    ],
                    ShapeStyle::from(&color).filled(),
                )))
                .expect("failed to draw leverage heatmap cell");
            chart
                .draw_series(std::iter::once(Text::new(
                    format!("{score:.1}"),
                    (column_index as i32, row_index),
                    ("sans-serif", 16).into_font().color(&BLACK),
                )))
                .expect("failed to draw leverage heatmap label");
        }
    }

    root.present()
        .expect("failed to present family leverage heatmap");
}

fn render_family_depth_ladder(rows: &[FamilyDepthRow], path: &Path) {
    let mut ordered_rows = rows.to_vec();
    ordered_rows.sort_by(|left, right| {
        family_depth_rank(&left.depth_label)
            .cmp(&family_depth_rank(&right.depth_label))
            .then_with(|| left.family.cmp(&right.family))
    });

    let root = BitMapBackend::new(path, (1220, 720)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill family depth ladder");
    let mut chart = ChartBuilder::on(&root)
        .caption("Hinge Atom-Family Depth Ladder", ("sans-serif", 28))
        .margin(28)
        .x_label_area_size(60)
        .y_label_area_size(220)
        .build_cartesian_2d(0.0f64..3.1f64, 0usize..ordered_rows.len())
        .expect("failed to build family depth ladder");

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("Depth score")
        .y_labels(ordered_rows.len())
        .y_label_formatter(&{
            let labels = ordered_rows
                .iter()
                .map(|row| row.family.as_str().to_string())
                .collect::<Vec<_>>();
            move |value| labels.get(*value).cloned().unwrap_or_default()
        })
        .label_style(("sans-serif", 15))
        .axis_style(RGBColor(92, 86, 78))
        .draw()
        .expect("failed to draw family depth ladder mesh");

    for (index, row) in ordered_rows.iter().enumerate() {
        let score = match row.depth_label.as_str() {
            "deepest" => 3.0,
            "bridge" => 2.0,
            _ => 1.0,
        };
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(0.0, index), (score, index + 1)],
                ShapeStyle::from(&family_color(row.family)).filled(),
            )))
            .expect("failed to draw family depth ladder bar");
        chart
            .draw_series(std::iter::once(Text::new(
                format!("{} | {}", row.depth_label, row.theorem_language_label),
                (score.min(2.8) + 0.08, index),
                ("sans-serif", 14).into_font().color(&BLACK),
            )))
            .expect("failed to draw family depth ladder label");
    }

    root.present()
        .expect("failed to present family depth ladder");
}

fn render_family_rule_examples(rows: &[FamilyOnlySearchRow], path: &Path) {
    let root = BitMapBackend::new(path, (1320, 980)).into_drawing_area();
    root.fill(&RGBColor(249, 246, 240))
        .expect("failed to fill family rule examples");
    let areas = root.split_evenly((3, 1));

    for (area, search_id) in areas.into_iter().zip([
        HINGE_SEARCH_PRIMARY,
        HINGE_SEARCH_PERSISTENT,
        HINGE_SEARCH_CORE,
    ]) {
        let mut search_rows = rows
            .iter()
            .filter(|row| row.search_id == search_id)
            .cloned()
            .collect::<Vec<_>>();
        search_rows.sort_by(|left, right| {
            left.best_error_count
                .cmp(&right.best_error_count)
                .then_with(|| right.any_exact_rule.cmp(&left.any_exact_rule))
                .then_with(|| left.family.cmp(&right.family))
        });
        let labels = search_rows
            .iter()
            .map(|row| row.family.as_str().to_string())
            .collect::<Vec<_>>();
        let mut chart = ChartBuilder::on(&area)
            .caption(
                format!("Family-only best rules  [{}]", search_id),
                ("sans-serif", 18),
            )
            .margin(22)
            .x_label_area_size(42)
            .y_label_area_size(200)
            .build_cartesian_2d(0.0f64..1.02f64, 0usize..search_rows.len())
            .expect("failed to build family rule example panel");

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc("F1 score")
            .y_labels(search_rows.len())
            .y_label_formatter(&move |value| labels.get(*value).cloned().unwrap_or_default())
            .label_style(("sans-serif", 13))
            .axis_style(RGBColor(92, 86, 78))
            .draw()
            .expect("failed to draw family rule example mesh");

        for (index, row) in search_rows.iter().enumerate() {
            let fill = if row.any_exact_rule {
                family_color(row.family)
            } else {
                RGBColor(188, 188, 188)
            };
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(0.0, index), (row.best_f1.max(0.01), index + 1)],
                    ShapeStyle::from(&fill).filled(),
                )))
                .expect("failed to draw family rule example bar");
            chart
                .draw_series(std::iter::once(Text::new(
                    truncate_label(&row.best_rule_label, 68),
                    (row.best_f1.min(0.82) + 0.02, index),
                    ("sans-serif", 12).into_font().color(&BLACK),
                )))
                .expect("failed to draw family rule example label");
        }
    }

    root.present()
        .expect("failed to present family rule examples");
}

fn render_markdown(bundle: &ReportBundle) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Hinge Atom-Family Depth Pass\n\n");
    markdown.push_str("_Generated from `examples/two_p_hinge_atom_family_report.rs`._\n\n");
    markdown.push_str(&format!(
        "- Output directory: `{}`\n- Main bases: `{}`\n- Appendix bases: `{}`\n- Pair catalog mode: `{}`\n- Max rule atoms: `{}`\n\n",
        bundle.settings.out_dir,
        bundle
            .settings
            .main_bases
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        bundle
            .settings
            .appendix_bases
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        bundle.settings.pair_catalog_mode,
        bundle.settings.max_rule_atoms
    ));

    markdown.push_str("## Family Depth Ladder\n\n");
    markdown.push_str("| Family | Theorem class | Primary exact | Persistent exact | Core exact | Primary ablation breaks exact | Primary mixed exact count | Depth | Theorem language |\n");
    markdown.push_str("|---|---|---|---|---|---|---:|---|---|\n");
    for row in &bundle.family_depth_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.family.as_str(),
            row.theorem_class.as_str(),
            yes_no(row.primary_family_only_exact),
            yes_no(row.persistent_family_only_exact),
            yes_no(row.core_family_only_exact),
            yes_no(row.primary_ablation_breaks_exact),
            row.primary_exact_mixed_rule_count,
            row.depth_label,
            row.theorem_language_label,
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Family-Only Search\n\n");
    markdown.push_str("| Search | Family | Exact | Best rule | Errors | F1 | Threshold-free |\n");
    markdown.push_str("|---|---|---|---|---:|---:|---|\n");
    for row in &bundle.family_only_search_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {:.3} | {} |\n",
            row.search_id,
            row.family.as_str(),
            yes_no(row.any_exact_rule),
            row.best_rule_label,
            row.best_error_count,
            row.best_f1,
            yes_no(row.best_threshold_free),
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Leave-One-Family-Out Ablation\n\n");
    markdown.push_str("| Search | Removed family | Destroy exact separator | Error delta | Atom delta | Theorem class shift downward |\n");
    markdown.push_str("|---|---|---|---:|---:|---|\n");
    for row in &bundle.family_ablation_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            row.search_id,
            row.removed_family.as_str(),
            yes_no(row.destroys_exact_separator),
            row.best_error_delta,
            row.atom_count_delta,
            yes_no(row.theorem_class_shift_downward),
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Mixed-Rule Participation\n\n");
    markdown.push_str("| Search | Family | Exact mixed count | Exact rank presence | Smallest exact mixed atom count | Top frontier presence |\n");
    markdown.push_str("|---|---|---:|---:|---:|---|\n");
    for row in &bundle.family_mixed_rule_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            row.search_id,
            row.family.as_str(),
            row.exact_mixed_rule_count,
            format_option_usize(row.exact_rule_rank_presence),
            format_option_usize(row.smallest_exact_mixed_rule_atom_count),
            yes_no(row.top_frontier_presence),
        ));
    }
    markdown.push('\n');

    markdown.push_str("## Representative Atlas\n\n");
    markdown.push_str("| Role | Base | Pair | Category | Gap | Same digit | M1 anomaly | M2 anomaly | M1 margin | M2 margin | M2 stable-zero | M2 boundary | Sentence |\n");
    markdown.push_str("|---|---:|---|---|---|---|---:|---:|---:|---:|---:|---:|---|\n");
    for row in &bundle.representative_family_rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {:.2}pp | {:.2}pp | {} | {} | {} | {} | {} |\n",
            row.role,
            row.base,
            row.pair_label,
            row.hinge_category,
            row.gap_bucket,
            yes_no(row.same_digit),
            row.m1_anomaly_mass_pp,
            row.m2_anomaly_mass_pp,
            row.m1_stable_zero_signal_margin_count,
            row.m2_stable_zero_signal_margin_count,
            row.m2_stable_zero_prime_delta_count,
            row.m2_boundary_prime_delta_count,
            row.mechanism_sentence,
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

fn lookup_family_only_row<'a>(
    rows: &'a [FamilyOnlySearchRow],
    search_id: &str,
    family: HingeAtomFamily,
) -> &'a FamilyOnlySearchRow {
    rows.iter()
        .find(|row| row.search_id == search_id && row.family == family)
        .expect("family-only row should exist")
}

fn lookup_family_ablation_row<'a>(
    rows: &'a [FamilyAblationRow],
    search_id: &str,
    family: HingeAtomFamily,
) -> &'a FamilyAblationRow {
    rows.iter()
        .find(|row| row.search_id == search_id && row.removed_family == family)
        .expect("family-ablation row should exist")
}

fn lookup_family_mixed_row<'a>(
    rows: &'a [FamilyMixedRuleRow],
    search_id: &str,
    family: HingeAtomFamily,
) -> &'a FamilyMixedRuleRow {
    rows.iter()
        .find(|row| row.search_id == search_id && row.family == family)
        .expect("family-mixed row should exist")
}

fn family_default_theorem_class(family: HingeAtomFamily) -> HingeAtomTheoremClass {
    match family {
        HingeAtomFamily::OverlapBoundary => HingeAtomTheoremClass::ExactTransferSubstrate,
        HingeAtomFamily::CarryThrough => HingeAtomTheoremClass::CrossMExactButEmpirical,
        HingeAtomFamily::ThresholdShape => HingeAtomTheoremClass::DerivedThreshold,
        HingeAtomFamily::Geometry | HingeAtomFamily::TemplateChoice => {
            HingeAtomTheoremClass::Diagnostic
        }
    }
}

fn family_depth_rank(label: &str) -> usize {
    match label {
        "deepest" => 0,
        "bridge" => 1,
        _ => 2,
    }
}

fn theorem_language_rank(label: &str) -> usize {
    match label {
        "closest_to_theorem" => 0,
        "supporting_bridge" => 1,
        _ => 2,
    }
}

fn representative_mechanism_sentence(row: &HingeFeatureRow) -> &'static str {
    match (row.m1_to_m2_persistent, row.shared_yield_core) {
        (true, true) => "survives both axes",
        (true, false) => "survives persistence but not overlap dominance",
        (false, true) => "survives overlap dominance but not persistence",
        (false, false) => "misses both",
    }
}

fn leverage_score(
    family_only: &FamilyOnlySearchRow,
    ablation: &FamilyAblationRow,
    mixed: &FamilyMixedRuleRow,
) -> f64 {
    let mut score = 0.0;
    if family_only.any_exact_rule {
        score += 2.0;
        if family_only.best_threshold_free {
            score += 0.5;
        }
    } else {
        score += (1.0 - family_only.best_error_count as f64 / 6.0).clamp(0.0, 1.0);
    }
    if ablation.destroys_exact_separator {
        score += 1.5;
    } else if ablation.best_error_delta > 0 {
        score += 0.5;
    }
    if mixed.exact_mixed_rule_count > 0 {
        score += 0.5;
    }
    score
}

fn family_color(family: HingeAtomFamily) -> RGBColor {
    match family {
        HingeAtomFamily::OverlapBoundary => RGBColor(48, 119, 142),
        HingeAtomFamily::CarryThrough => RGBColor(86, 147, 101),
        HingeAtomFamily::ThresholdShape => RGBColor(218, 143, 53),
        HingeAtomFamily::Geometry => RGBColor(181, 76, 64),
        HingeAtomFamily::TemplateChoice => RGBColor(122, 122, 122),
    }
}

fn heatmap_color(intensity: f64) -> RGBColor {
    let clamped = intensity.clamp(0.0, 1.0);
    let r = (233.0 - 165.0 * clamped).round() as u8;
    let g = (233.0 - 98.0 * clamped).round() as u8;
    let b = (228.0 - 136.0 * clamped).round() as u8;
    RGBColor(r, g, b)
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

fn format_option_usize(value: Option<usize>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "n/a".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_boundary_family_remains_top_classification() {
        let rows = build_hinge_feature_rows(MAIN_BASES);
        let problems = build_hinge_search_problems(&rows);
        let mut family_only_rows = Vec::new();
        let mut family_ablation_rows = Vec::new();
        let mut family_mixed_rows = Vec::new();

        for problem in &problems {
            let atoms = build_hinge_atom_specs(problem);
            let baseline = run_search_detail(problem, &atoms);
            for &family in FAMILIES {
                let family_atoms = atoms
                    .iter()
                    .filter(|atom| atom.family == family)
                    .cloned()
                    .collect::<Vec<_>>();
                family_only_rows.push(build_family_only_row(problem, family, &family_atoms));
                let ablated_atoms = atoms
                    .iter()
                    .filter(|atom| atom.family != family)
                    .cloned()
                    .collect::<Vec<_>>();
                family_ablation_rows.push(build_family_ablation_row(
                    problem,
                    family,
                    &baseline,
                    &ablated_atoms,
                ));
                family_mixed_rows.push(build_family_mixed_rule_row(problem, family, &baseline));
            }
        }

        let depth_rows =
            build_family_depth_rows(&family_only_rows, &family_ablation_rows, &family_mixed_rows);
        let overlap = depth_rows
            .iter()
            .find(|row| row.family == HingeAtomFamily::OverlapBoundary)
            .expect("overlap family should exist");
        let geometry = depth_rows
            .iter()
            .find(|row| row.family == HingeAtomFamily::Geometry)
            .expect("geometry family should exist");
        let template = depth_rows
            .iter()
            .find(|row| row.family == HingeAtomFamily::TemplateChoice)
            .expect("template family should exist");

        assert_eq!(overlap.depth_label, "deepest");
        assert_eq!(overlap.theorem_language_label, "closest_to_theorem");
        assert_eq!(geometry.depth_label, "diagnostic");
        assert_eq!(template.depth_label, "diagnostic");
    }
}
