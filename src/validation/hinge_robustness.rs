//! Shared hinge robustness and family-depth analysis.

use crate::validation::{
    bounded_k::HingeFeatureRow,
    hinge_atoms::{
        build_hinge_atom_specs_with_policy, build_hinge_search_problems,
        default_hinge_atom_catalog_policy, enumerate_hinge_rule_candidates,
        rule_candidate_exact_sort, rule_candidate_quality_sort, run_hinge_rule_search,
        HingeAtomCatalogPolicy, HingeAtomFamily, HingeAtomSpec, HingeAtomTheoremClass,
        HingeRuleCandidate, HingeSearchProblem, HingeThresholdPolicy, HINGE_FAMILIES,
        HINGE_SEARCH_CORE, HINGE_SEARCH_PERSISTENT, HINGE_SEARCH_PRIMARY,
    },
};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HingeScenarioGroup {
    DataSurface,
    ThresholdVocabulary,
    AdversarialCatalog,
}

impl HingeScenarioGroup {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DataSurface => "data_surface",
            Self::ThresholdVocabulary => "threshold_vocabulary",
            Self::AdversarialCatalog => "adversarial_catalog",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HingeScenarioKind {
    Stability,
    Adversarial,
}

impl HingeScenarioKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stability => "stability",
            Self::Adversarial => "adversarial",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum HingeRowFilter {
    All,
    DropRepresentative { base: u32, outer: u32, inner: u32 },
}

#[derive(Debug, Clone, Serialize)]
pub struct HingeRobustnessScenario {
    pub id: &'static str,
    pub label: &'static str,
    pub group: HingeScenarioGroup,
    pub row_filter: HingeRowFilter,
    pub atom_catalog_policy: HingeAtomCatalogPolicy,
    pub scenario_kind: HingeScenarioKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HingeFamilyDepthLabel {
    Deepest,
    Bridge,
    Diagnostic,
}

impl HingeFamilyDepthLabel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deepest => "deepest",
            Self::Bridge => "bridge",
            Self::Diagnostic => "diagnostic",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HingeTheoremLanguageLabel {
    ClosestToTheorem,
    SupportingBridge,
    NotYetTheoremLanguage,
}

impl HingeTheoremLanguageLabel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ClosestToTheorem => "closest_to_theorem",
            Self::SupportingBridge => "supporting_bridge",
            Self::NotYetTheoremLanguage => "not_yet_theorem_language",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HingeAtomCatalogRow {
    pub search_id: String,
    pub search_label: String,
    pub atom_label: String,
    pub family: HingeAtomFamily,
    pub theorem_class: HingeAtomTheoremClass,
    pub threshold_based: bool,
    pub complexity_score: usize,
    pub true_count: usize,
    pub false_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct HingeFamilyOnlySearchRow {
    pub search_id: String,
    pub search_label: String,
    pub family: HingeAtomFamily,
    pub atom_pool_size: usize,
    pub searched_rule_count: usize,
    pub any_exact_rule: bool,
    pub best_rule_label: String,
    pub best_error_count: usize,
    pub best_f1: f64,
    pub best_true_positive: usize,
    pub best_false_positive: usize,
    pub best_false_negative: usize,
    pub best_threshold_free: bool,
    pub best_rule_theorem_class: Option<HingeAtomTheoremClass>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HingeFamilyAblationRow {
    pub search_id: String,
    pub search_label: String,
    pub removed_family: HingeAtomFamily,
    pub baseline_any_exact_rule: bool,
    pub ablated_any_exact_rule: bool,
    pub baseline_best_rule_label: String,
    pub ablated_best_rule_label: String,
    pub baseline_best_rule_theorem_class: Option<HingeAtomTheoremClass>,
    pub ablated_best_rule_theorem_class: Option<HingeAtomTheoremClass>,
    pub destroys_exact_separator: bool,
    pub best_error_delta: isize,
    pub atom_count_delta: isize,
    pub theorem_class_shift_downward: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct HingeFamilyMixedRuleRow {
    pub search_id: String,
    pub search_label: String,
    pub family: HingeAtomFamily,
    pub exact_mixed_rule_count: usize,
    pub exact_rule_rank_presence: Option<usize>,
    pub smallest_exact_mixed_rule_atom_count: Option<usize>,
    pub top_frontier_presence: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct HingeFamilyDepthRow {
    pub family: HingeAtomFamily,
    pub theorem_class: HingeAtomTheoremClass,
    pub primary_family_only_exact: bool,
    pub persistent_family_only_exact: bool,
    pub core_family_only_exact: bool,
    pub primary_ablation_breaks_exact: bool,
    pub primary_exact_mixed_rule_count: usize,
    pub primary_top_frontier_presence: bool,
    pub depth_label: HingeFamilyDepthLabel,
    pub theorem_language_label: HingeTheoremLanguageLabel,
    pub rationale: String,
}

#[derive(Debug, Clone)]
pub struct HingeSearchRunDetail {
    pub all_candidates: Vec<HingeRuleCandidate>,
    pub best_candidates: Vec<HingeRuleCandidate>,
    pub any_exact_rule: bool,
}

#[derive(Debug, Clone)]
pub struct HingeFamilyDepthAnalysis {
    pub atom_catalog_rows: Vec<HingeAtomCatalogRow>,
    pub family_only_rows: Vec<HingeFamilyOnlySearchRow>,
    pub family_ablation_rows: Vec<HingeFamilyAblationRow>,
    pub family_mixed_rows: Vec<HingeFamilyMixedRuleRow>,
    pub family_depth_rows: Vec<HingeFamilyDepthRow>,
    pub search_runs: BTreeMap<String, HingeSearchRunDetail>,
}

#[derive(Debug, Clone)]
pub struct HingeRobustnessRun {
    pub scenario_id: String,
    pub search_id: String,
    pub dataset_rows: usize,
    pub positive_rows: usize,
    pub best_primary_rule: Option<HingeRuleCandidate>,
    pub any_exact_primary_rule: bool,
    pub family_depth_rows: Vec<HingeFamilyDepthRow>,
    pub family_ladder_signature: Vec<String>,
    pub filtered_rows: Vec<HingeFeatureRow>,
    pub analysis: HingeFamilyDepthAnalysis,
}

pub fn filter_hinge_rows(
    rows: &[HingeFeatureRow],
    filter: &HingeRowFilter,
) -> Vec<HingeFeatureRow> {
    match filter {
        HingeRowFilter::All => rows.to_vec(),
        HingeRowFilter::DropRepresentative { base, outer, inner } => rows
            .iter()
            .filter(|row| !(row.base == *base && row.outer == *outer && row.inner == *inner))
            .cloned()
            .collect(),
    }
}

pub fn default_hinge_robustness_scenarios() -> Vec<HingeRobustnessScenario> {
    let all_families = HINGE_FAMILIES.to_vec();
    vec![
        HingeRobustnessScenario {
            id: "baseline_main",
            label: "Baseline main surface",
            group: HingeScenarioGroup::DataSurface,
            row_filter: HingeRowFilter::All,
            atom_catalog_policy: HingeAtomCatalogPolicy {
                included_families: all_families.clone(),
                threshold_policy: HingeThresholdPolicy::Observed,
            },
            scenario_kind: HingeScenarioKind::Stability,
        },
        HingeRobustnessScenario {
            id: "drop_rep_db",
            label: "Drop base 14 (D,B)",
            group: HingeScenarioGroup::DataSurface,
            row_filter: HingeRowFilter::DropRepresentative {
                base: 14,
                outer: 13,
                inner: 11,
            },
            atom_catalog_policy: default_hinge_atom_catalog_policy(),
            scenario_kind: HingeScenarioKind::Stability,
        },
        HingeRobustnessScenario {
            id: "drop_rep_31",
            label: "Drop base 14 (3,1)",
            group: HingeScenarioGroup::DataSurface,
            row_filter: HingeRowFilter::DropRepresentative {
                base: 14,
                outer: 3,
                inner: 1,
            },
            atom_catalog_policy: default_hinge_atom_catalog_policy(),
            scenario_kind: HingeScenarioKind::Stability,
        },
        HingeRobustnessScenario {
            id: "drop_rep_33",
            label: "Drop base 10 (3,3)",
            group: HingeScenarioGroup::DataSurface,
            row_filter: HingeRowFilter::DropRepresentative {
                base: 10,
                outer: 3,
                inner: 3,
            },
            atom_catalog_policy: default_hinge_atom_catalog_policy(),
            scenario_kind: HingeScenarioKind::Stability,
        },
        HingeRobustnessScenario {
            id: "drop_rep_nn",
            label: "Drop base 26 (N,N)",
            group: HingeScenarioGroup::DataSurface,
            row_filter: HingeRowFilter::DropRepresentative {
                base: 26,
                outer: 23,
                inner: 23,
            },
            atom_catalog_policy: default_hinge_atom_catalog_policy(),
            scenario_kind: HingeScenarioKind::Stability,
        },
        HingeRobustnessScenario {
            id: "drop_rep_hj",
            label: "Drop base 22 (H,J)",
            group: HingeScenarioGroup::DataSurface,
            row_filter: HingeRowFilter::DropRepresentative {
                base: 22,
                outer: 17,
                inner: 19,
            },
            atom_catalog_policy: default_hinge_atom_catalog_policy(),
            scenario_kind: HingeScenarioKind::Stability,
        },
        HingeRobustnessScenario {
            id: "threshold_observed",
            label: "Threshold observed",
            group: HingeScenarioGroup::ThresholdVocabulary,
            row_filter: HingeRowFilter::All,
            atom_catalog_policy: HingeAtomCatalogPolicy {
                included_families: all_families.clone(),
                threshold_policy: HingeThresholdPolicy::Observed,
            },
            scenario_kind: HingeScenarioKind::Stability,
        },
        HingeRobustnessScenario {
            id: "threshold_min_side_2",
            label: "Threshold min side 2",
            group: HingeScenarioGroup::ThresholdVocabulary,
            row_filter: HingeRowFilter::All,
            atom_catalog_policy: HingeAtomCatalogPolicy {
                included_families: all_families.clone(),
                threshold_policy: HingeThresholdPolicy::ObservedMinSide2,
            },
            scenario_kind: HingeScenarioKind::Stability,
        },
        HingeRobustnessScenario {
            id: "threshold_quantized_3dp",
            label: "Threshold quantized 3dp",
            group: HingeScenarioGroup::ThresholdVocabulary,
            row_filter: HingeRowFilter::All,
            atom_catalog_policy: HingeAtomCatalogPolicy {
                included_families: all_families.clone(),
                threshold_policy: HingeThresholdPolicy::Quantized3Dp,
            },
            scenario_kind: HingeScenarioKind::Stability,
        },
        HingeRobustnessScenario {
            id: "threshold_quantized_2dp",
            label: "Threshold quantized 2dp",
            group: HingeScenarioGroup::ThresholdVocabulary,
            row_filter: HingeRowFilter::All,
            atom_catalog_policy: HingeAtomCatalogPolicy {
                included_families: all_families.clone(),
                threshold_policy: HingeThresholdPolicy::Quantized2Dp,
            },
            scenario_kind: HingeScenarioKind::Stability,
        },
        HingeRobustnessScenario {
            id: "no_overlap_boundary",
            label: "No overlap-boundary atoms",
            group: HingeScenarioGroup::AdversarialCatalog,
            row_filter: HingeRowFilter::All,
            atom_catalog_policy: HingeAtomCatalogPolicy {
                included_families: all_families
                    .iter()
                    .copied()
                    .filter(|family| *family != HingeAtomFamily::OverlapBoundary)
                    .collect(),
                threshold_policy: HingeThresholdPolicy::Observed,
            },
            scenario_kind: HingeScenarioKind::Adversarial,
        },
        HingeRobustnessScenario {
            id: "no_carry_through",
            label: "No carry-through atoms",
            group: HingeScenarioGroup::AdversarialCatalog,
            row_filter: HingeRowFilter::All,
            atom_catalog_policy: HingeAtomCatalogPolicy {
                included_families: all_families
                    .iter()
                    .copied()
                    .filter(|family| *family != HingeAtomFamily::CarryThrough)
                    .collect(),
                threshold_policy: HingeThresholdPolicy::Observed,
            },
            scenario_kind: HingeScenarioKind::Adversarial,
        },
        HingeRobustnessScenario {
            id: "no_threshold_shape",
            label: "No threshold-shape atoms",
            group: HingeScenarioGroup::AdversarialCatalog,
            row_filter: HingeRowFilter::All,
            atom_catalog_policy: HingeAtomCatalogPolicy {
                included_families: all_families
                    .iter()
                    .copied()
                    .filter(|family| *family != HingeAtomFamily::ThresholdShape)
                    .collect(),
                threshold_policy: HingeThresholdPolicy::Observed,
            },
            scenario_kind: HingeScenarioKind::Adversarial,
        },
    ]
}

pub fn analyze_hinge_family_depth(
    rows: &[HingeFeatureRow],
    atom_policy: &HingeAtomCatalogPolicy,
    max_rule_atoms: usize,
    exported_rule_frontier: usize,
    best_rules_per_search: usize,
) -> HingeFamilyDepthAnalysis {
    let problems = build_hinge_search_problems(rows);
    let mut atom_catalog_rows = Vec::new();
    let mut family_only_rows = Vec::new();
    let mut family_ablation_rows = Vec::new();
    let mut family_mixed_rows = Vec::new();
    let mut search_runs = BTreeMap::<String, HingeSearchRunDetail>::new();

    for problem in &problems {
        let atoms = build_hinge_atom_specs_with_policy(problem, atom_policy);
        atom_catalog_rows.extend(build_atom_catalog_rows(problem, &atoms));
        let baseline_run = run_search_detail(
            problem,
            &atoms,
            max_rule_atoms,
            exported_rule_frontier,
            best_rules_per_search,
        );
        search_runs.insert(problem.id.to_string(), baseline_run.clone());

        for &family in HINGE_FAMILIES {
            let family_atoms = atoms
                .iter()
                .filter(|atom| atom.family == family)
                .cloned()
                .collect::<Vec<_>>();
            family_only_rows.push(build_family_only_row(
                problem,
                family,
                &family_atoms,
                max_rule_atoms,
                exported_rule_frontier,
                best_rules_per_search,
            ));

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
                max_rule_atoms,
                exported_rule_frontier,
                best_rules_per_search,
            ));
            family_mixed_rows.push(build_family_mixed_rule_row(problem, family, &baseline_run));
        }
    }

    let family_depth_rows =
        build_family_depth_rows(&family_only_rows, &family_ablation_rows, &family_mixed_rows);

    HingeFamilyDepthAnalysis {
        atom_catalog_rows,
        family_only_rows,
        family_ablation_rows,
        family_mixed_rows,
        family_depth_rows,
        search_runs,
    }
}

pub fn run_hinge_robustness_scenario(
    rows: &[HingeFeatureRow],
    scenario: &HingeRobustnessScenario,
    max_rule_atoms: usize,
    exported_rule_frontier: usize,
    best_rules_per_search: usize,
) -> HingeRobustnessRun {
    let filtered_rows = filter_hinge_rows(rows, &scenario.row_filter);
    let analysis = analyze_hinge_family_depth(
        &filtered_rows,
        &scenario.atom_catalog_policy,
        max_rule_atoms,
        exported_rule_frontier,
        best_rules_per_search,
    );
    let primary = analysis
        .search_runs
        .get(HINGE_SEARCH_PRIMARY)
        .expect("primary search run should exist");

    HingeRobustnessRun {
        scenario_id: scenario.id.to_string(),
        search_id: HINGE_SEARCH_PRIMARY.to_string(),
        dataset_rows: primary
            .best_candidates
            .first()
            .map(|row| {
                row.true_positive + row.false_positive + row.false_negative + row.true_negative
            })
            .unwrap_or(0),
        positive_rows: primary
            .best_candidates
            .first()
            .map(|row| row.true_positive + row.false_negative)
            .unwrap_or(0),
        best_primary_rule: primary.best_candidates.first().cloned(),
        any_exact_primary_rule: primary.any_exact_rule,
        family_depth_rows: analysis.family_depth_rows.clone(),
        family_ladder_signature: family_ladder_signature(&analysis.family_depth_rows),
        filtered_rows,
        analysis,
    }
}

pub fn family_ladder_signature(rows: &[HingeFamilyDepthRow]) -> Vec<String> {
    let mut ordered = rows.to_vec();
    ordered.sort_by(|left, right| {
        left.depth_label
            .cmp(&right.depth_label)
            .then_with(|| {
                left.theorem_language_label
                    .cmp(&right.theorem_language_label)
            })
            .then_with(|| left.family.cmp(&right.family))
    });
    ordered
        .iter()
        .map(|row| {
            format!(
                "{}:{}:{}",
                row.family.as_str(),
                row.depth_label.as_str(),
                row.theorem_language_label.as_str()
            )
        })
        .collect()
}

pub fn lookup_family_only_row<'a>(
    rows: &'a [HingeFamilyOnlySearchRow],
    search_id: &str,
    family: HingeAtomFamily,
) -> &'a HingeFamilyOnlySearchRow {
    rows.iter()
        .find(|row| row.search_id == search_id && row.family == family)
        .expect("family-only row should exist")
}

pub fn lookup_family_ablation_row<'a>(
    rows: &'a [HingeFamilyAblationRow],
    search_id: &str,
    family: HingeAtomFamily,
) -> &'a HingeFamilyAblationRow {
    rows.iter()
        .find(|row| row.search_id == search_id && row.removed_family == family)
        .expect("family-ablation row should exist")
}

pub fn lookup_family_mixed_row<'a>(
    rows: &'a [HingeFamilyMixedRuleRow],
    search_id: &str,
    family: HingeAtomFamily,
) -> &'a HingeFamilyMixedRuleRow {
    rows.iter()
        .find(|row| row.search_id == search_id && row.family == family)
        .expect("family-mixed row should exist")
}

fn build_atom_catalog_rows(
    problem: &HingeSearchProblem<'_>,
    atoms: &[HingeAtomSpec],
) -> Vec<HingeAtomCatalogRow> {
    atoms
        .iter()
        .map(|atom| HingeAtomCatalogRow {
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

fn run_search_detail(
    problem: &HingeSearchProblem<'_>,
    atoms: &[HingeAtomSpec],
    max_rule_atoms: usize,
    exported_rule_frontier: usize,
    best_rules_per_search: usize,
) -> HingeSearchRunDetail {
    let mut all_candidates = enumerate_hinge_rule_candidates(problem, atoms, max_rule_atoms);
    all_candidates.sort_by(rule_candidate_quality_sort);
    let outcome = run_hinge_rule_search(
        problem,
        atoms,
        max_rule_atoms,
        exported_rule_frontier,
        best_rules_per_search,
    );
    HingeSearchRunDetail {
        all_candidates,
        best_candidates: outcome.best_rows,
        any_exact_rule: outcome.summary.any_exact_rule,
    }
}

fn build_family_only_row(
    problem: &HingeSearchProblem<'_>,
    family: HingeAtomFamily,
    atoms: &[HingeAtomSpec],
    max_rule_atoms: usize,
    exported_rule_frontier: usize,
    best_rules_per_search: usize,
) -> HingeFamilyOnlySearchRow {
    let run = run_search_detail(
        problem,
        atoms,
        max_rule_atoms,
        exported_rule_frontier,
        best_rules_per_search,
    );
    let best = run.best_candidates.first();
    HingeFamilyOnlySearchRow {
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
    baseline: &HingeSearchRunDetail,
    ablated_atoms: &[HingeAtomSpec],
    max_rule_atoms: usize,
    exported_rule_frontier: usize,
    best_rules_per_search: usize,
) -> HingeFamilyAblationRow {
    let ablated = run_search_detail(
        problem,
        ablated_atoms,
        max_rule_atoms,
        exported_rule_frontier,
        best_rules_per_search,
    );
    let baseline_best = baseline.best_candidates.first();
    let ablated_best = ablated.best_candidates.first();
    let baseline_class = baseline_best.map(|row| row.rule_theorem_class);
    let ablated_class = ablated_best.map(|row| row.rule_theorem_class);

    HingeFamilyAblationRow {
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
    baseline: &HingeSearchRunDetail,
) -> HingeFamilyMixedRuleRow {
    let exact_rules = baseline
        .all_candidates
        .iter()
        .filter(|row| row.exact_match)
        .collect::<Vec<_>>();
    let mut sorted_exact_rules = exact_rules.to_vec();
    sorted_exact_rules.sort_by(|left, right| rule_candidate_exact_sort(left, right));

    HingeFamilyMixedRuleRow {
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
    family_only_rows: &[HingeFamilyOnlySearchRow],
    family_ablation_rows: &[HingeFamilyAblationRow],
    family_mixed_rows: &[HingeFamilyMixedRuleRow],
) -> Vec<HingeFamilyDepthRow> {
    HINGE_FAMILIES
        .iter()
        .copied()
        .map(|family| {
            let theorem_class = family_default_theorem_class(family);
            let primary_family_only =
                lookup_family_only_row(family_only_rows, HINGE_SEARCH_PRIMARY, family);
            let persistent_family_only =
                lookup_family_only_row(family_only_rows, HINGE_SEARCH_PERSISTENT, family);
            let core_family_only =
                lookup_family_only_row(family_only_rows, HINGE_SEARCH_CORE, family);
            let primary_ablation =
                lookup_family_ablation_row(family_ablation_rows, HINGE_SEARCH_PRIMARY, family);
            let primary_mixed =
                lookup_family_mixed_row(family_mixed_rows, HINGE_SEARCH_PRIMARY, family);

            let depth_label = if theorem_class == HingeAtomTheoremClass::ExactTransferSubstrate
                && (primary_family_only.any_exact_rule
                    || persistent_family_only.any_exact_rule
                    || primary_ablation.destroys_exact_separator)
            {
                HingeFamilyDepthLabel::Deepest
            } else if theorem_class == HingeAtomTheoremClass::Diagnostic {
                HingeFamilyDepthLabel::Diagnostic
            } else if persistent_family_only.any_exact_rule
                || core_family_only.any_exact_rule
                || primary_mixed.exact_mixed_rule_count > 0
            {
                HingeFamilyDepthLabel::Bridge
            } else {
                HingeFamilyDepthLabel::Diagnostic
            };

            let theorem_language_label = if depth_label == HingeFamilyDepthLabel::Deepest
                && theorem_class == HingeAtomTheoremClass::ExactTransferSubstrate
                && ((primary_family_only.any_exact_rule && primary_family_only.best_threshold_free)
                    || (persistent_family_only.any_exact_rule
                        && persistent_family_only.best_threshold_free))
            {
                HingeTheoremLanguageLabel::ClosestToTheorem
            } else if depth_label != HingeFamilyDepthLabel::Diagnostic {
                HingeTheoremLanguageLabel::SupportingBridge
            } else {
                HingeTheoremLanguageLabel::NotYetTheoremLanguage
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

            HingeFamilyDepthRow {
                family,
                theorem_class,
                primary_family_only_exact: primary_family_only.any_exact_rule,
                persistent_family_only_exact: persistent_family_only.any_exact_rule,
                core_family_only_exact: core_family_only.any_exact_rule,
                primary_ablation_breaks_exact: primary_ablation.destroys_exact_separator,
                primary_exact_mixed_rule_count: primary_mixed.exact_mixed_rule_count,
                primary_top_frontier_presence: primary_mixed.top_frontier_presence,
                depth_label,
                theorem_language_label,
                rationale,
            }
        })
        .collect()
}

pub fn family_default_theorem_class(family: HingeAtomFamily) -> HingeAtomTheoremClass {
    match family {
        HingeAtomFamily::OverlapBoundary => HingeAtomTheoremClass::ExactTransferSubstrate,
        HingeAtomFamily::CarryThrough => HingeAtomTheoremClass::CrossMExactButEmpirical,
        HingeAtomFamily::ThresholdShape => HingeAtomTheoremClass::DerivedThreshold,
        HingeAtomFamily::Geometry | HingeAtomFamily::TemplateChoice => {
            HingeAtomTheoremClass::Diagnostic
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::bounded_k::{analyze_hinge_feature_row, HINGE_CATEGORY_PERSISTENT_CORE};

    fn main_rows() -> Vec<HingeFeatureRow> {
        [10u32, 14, 22, 26]
            .into_iter()
            .flat_map(|base| {
                crate::validation::bounded_k::ordered_unit_pairs(base)
                    .into_iter()
                    .map(move |(outer, inner)| analyze_hinge_feature_row(base, outer, inner))
            })
            .collect()
    }

    #[test]
    fn baseline_analysis_reproduces_expected_primary_rule_and_ladder() {
        let rows = main_rows();
        let analysis =
            analyze_hinge_family_depth(&rows, &default_hinge_atom_catalog_policy(), 3, 60, 5);
        let primary = analysis
            .search_runs
            .get(HINGE_SEARCH_PRIMARY)
            .expect("primary run should exist");
        let best = primary
            .best_candidates
            .first()
            .expect("primary should have best rule");
        assert_eq!(
            best.rule_label,
            "m1 anomaly_mass_pp > 0 AND m2 boundary_prime_delta_count <= 0"
        );

        let overlap = analysis
            .family_depth_rows
            .iter()
            .find(|row| row.family == HingeAtomFamily::OverlapBoundary)
            .expect("overlap row should exist");
        let carry = analysis
            .family_depth_rows
            .iter()
            .find(|row| row.family == HingeAtomFamily::CarryThrough)
            .expect("carry row should exist");
        let threshold = analysis
            .family_depth_rows
            .iter()
            .find(|row| row.family == HingeAtomFamily::ThresholdShape)
            .expect("threshold row should exist");
        let geometry = analysis
            .family_depth_rows
            .iter()
            .find(|row| row.family == HingeAtomFamily::Geometry)
            .expect("geometry row should exist");
        let template = analysis
            .family_depth_rows
            .iter()
            .find(|row| row.family == HingeAtomFamily::TemplateChoice)
            .expect("template row should exist");

        assert_eq!(overlap.depth_label, HingeFamilyDepthLabel::Deepest);
        assert_eq!(
            overlap.theorem_language_label,
            HingeTheoremLanguageLabel::ClosestToTheorem
        );
        assert_eq!(carry.depth_label, HingeFamilyDepthLabel::Bridge);
        assert_eq!(threshold.depth_label, HingeFamilyDepthLabel::Bridge);
        assert_eq!(geometry.depth_label, HingeFamilyDepthLabel::Diagnostic);
        assert_eq!(template.depth_label, HingeFamilyDepthLabel::Diagnostic);
    }

    #[test]
    fn drop_rep_db_preserves_at_least_one_positive_primary_row() {
        let rows = main_rows();
        let filtered = filter_hinge_rows(
            &rows,
            &HingeRowFilter::DropRepresentative {
                base: 14,
                outer: 13,
                inner: 11,
            },
        );
        let problems = build_hinge_search_problems(&filtered);
        let primary = problems
            .iter()
            .find(|problem| problem.id == HINGE_SEARCH_PRIMARY)
            .expect("primary search should exist");
        assert!(primary
            .rows
            .iter()
            .any(|row| row.hinge_category == HINGE_CATEGORY_PERSISTENT_CORE));
    }
}
