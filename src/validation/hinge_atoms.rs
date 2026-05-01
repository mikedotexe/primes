//! Shared deterministic hinge-atom search vocabulary.
//!
//! This module lifts the bounded conjunction search used by the maintained
//! hinge reports into the validation layer so downstream reports can reuse the
//! exact same rule semantics.

use crate::validation::bounded_k::{HingeFeatureRow, HINGE_CATEGORY_PERSISTENT_CORE};
use serde::Serialize;
use std::{cmp::Ordering, collections::BTreeMap};

pub const HINGE_SEARCH_PRIMARY: &str = "primary_persistent_core";
pub const HINGE_SEARCH_PERSISTENT: &str = "persistent_overlap_split";
pub const HINGE_SEARCH_CORE: &str = "core_persistence_split";

pub const HINGE_SEARCHES: &[(&str, &str)] = &[
    (
        HINGE_SEARCH_PRIMARY,
        "Primary: persistent_core vs other main M=2 actives",
    ),
    (
        HINGE_SEARCH_PERSISTENT,
        "Secondary: overlap-dominant vs boundary-dominant persistent survivors",
    ),
    (
        HINGE_SEARCH_CORE,
        "Secondary: persistent vs non-persistent overlap/core-like pairs",
    ),
];

pub const HINGE_FAMILIES: &[HingeAtomFamily] = &[
    HingeAtomFamily::OverlapBoundary,
    HingeAtomFamily::CarryThrough,
    HingeAtomFamily::ThresholdShape,
    HingeAtomFamily::Geometry,
    HingeAtomFamily::TemplateChoice,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HingeAtomFamily {
    CarryThrough,
    OverlapBoundary,
    ThresholdShape,
    Geometry,
    TemplateChoice,
}

impl HingeAtomFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CarryThrough => "carry_through",
            Self::OverlapBoundary => "overlap_boundary",
            Self::ThresholdShape => "threshold_shape",
            Self::Geometry => "geometry",
            Self::TemplateChoice => "template_choice",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HingeThresholdPolicy {
    Observed,
    ObservedMinSide2,
    Quantized3Dp,
    Quantized2Dp,
}

impl HingeThresholdPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Observed => "observed",
            Self::ObservedMinSide2 => "observed_min_side_2",
            Self::Quantized3Dp => "quantized_3dp",
            Self::Quantized2Dp => "quantized_2dp",
        }
    }

    fn min_side(self) -> usize {
        match self {
            Self::ObservedMinSide2 => 2,
            _ => 1,
        }
    }

    fn quantize(self, value: f64) -> f64 {
        match self {
            Self::Quantized3Dp => (value * 1000.0).round() / 1000.0,
            Self::Quantized2Dp => (value * 100.0).round() / 100.0,
            _ => value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HingeAtomCatalogPolicy {
    pub included_families: Vec<HingeAtomFamily>,
    pub threshold_policy: HingeThresholdPolicy,
}

impl HingeAtomCatalogPolicy {
    pub fn includes_family(&self, family: HingeAtomFamily) -> bool {
        self.included_families.contains(&family)
    }
}

pub fn default_hinge_atom_catalog_policy() -> HingeAtomCatalogPolicy {
    HingeAtomCatalogPolicy {
        included_families: HINGE_FAMILIES.to_vec(),
        threshold_policy: HingeThresholdPolicy::Observed,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HingeAtomTheoremClass {
    ExactTransferSubstrate,
    CrossMExactButEmpirical,
    DerivedThreshold,
    Diagnostic,
}

impl HingeAtomTheoremClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactTransferSubstrate => "exact_transfer_substrate",
            Self::CrossMExactButEmpirical => "cross_m_exact_but_empirical",
            Self::DerivedThreshold => "derived_threshold",
            Self::Diagnostic => "diagnostic",
        }
    }

    pub fn depth_rank(self) -> usize {
        match self {
            Self::ExactTransferSubstrate => 0,
            Self::CrossMExactButEmpirical => 1,
            Self::DerivedThreshold => 2,
            Self::Diagnostic => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HingeThresholdField {
    M2AdmissibleOverlapJaccard,
    M2MaskStabilityShare,
    M2NonzeroChurnShare,
    M2StableZeroSupportRatio,
    M1AnomalyMassPp,
    M2AnomalyMassPp,
    M1StableZeroSignalMarginCount,
    M2StableZeroSignalMarginCount,
}

impl HingeThresholdField {
    fn as_str(self) -> &'static str {
        match self {
            Self::M2AdmissibleOverlapJaccard => "m2 admissible_overlap_jaccard",
            Self::M2MaskStabilityShare => "m2 mask_stability_share",
            Self::M2NonzeroChurnShare => "m2 nonzero_churn_share",
            Self::M2StableZeroSupportRatio => "m2 stable_zero_support_ratio",
            Self::M1AnomalyMassPp => "m1 anomaly_mass_pp",
            Self::M2AnomalyMassPp => "m2 anomaly_mass_pp",
            Self::M1StableZeroSignalMarginCount => "m1 stable_zero_signal_margin_count",
            Self::M2StableZeroSignalMarginCount => "m2 stable_zero_signal_margin_count",
        }
    }

    fn value(self, row: &HingeFeatureRow) -> f64 {
        match self {
            Self::M2AdmissibleOverlapJaccard => row.m2_admissible_overlap_jaccard,
            Self::M2MaskStabilityShare => row.m2_mask_stability_share,
            Self::M2NonzeroChurnShare => row.m2_nonzero_churn_share,
            Self::M2StableZeroSupportRatio => row.m2_stable_zero_support_ratio,
            Self::M1AnomalyMassPp => row.m1_anomaly_mass_pp,
            Self::M2AnomalyMassPp => row.m2_anomaly_mass_pp,
            Self::M1StableZeroSignalMarginCount => row.m1_stable_zero_signal_margin_count as f64,
            Self::M2StableZeroSignalMarginCount => row.m2_stable_zero_signal_margin_count as f64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HingeThresholdOp {
    Ge,
    Le,
}

impl HingeThresholdOp {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ge => ">=",
            Self::Le => "<=",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum HingeAtomPredicate {
    M2StableZeroPrimeDeltaCountPositive,
    M2BoundaryPrimeDeltaCountNonpositive,
    M2StableZeroSignalMarginCountPositive,
    M2SharedPrimeRateDeltaPositive,
    M1AnomalyMassPositive,
    M1BestKNonzero,
    M2BestKNonzero,
    GapBucketSameOrAdjacent,
    SameDigit,
    Threshold {
        field: HingeThresholdField,
        op: HingeThresholdOp,
        value: f64,
    },
}

impl HingeAtomPredicate {
    pub fn label(&self) -> String {
        match self {
            Self::M2StableZeroPrimeDeltaCountPositive => {
                "m2 stable_zero_prime_delta_count > 0".to_string()
            }
            Self::M2BoundaryPrimeDeltaCountNonpositive => {
                "m2 boundary_prime_delta_count <= 0".to_string()
            }
            Self::M2StableZeroSignalMarginCountPositive => {
                "m2 stable_zero_signal_margin_count > 0".to_string()
            }
            Self::M2SharedPrimeRateDeltaPositive => "m2 shared_prime_rate_delta_pp > 0".to_string(),
            Self::M1AnomalyMassPositive => "m1 anomaly_mass_pp > 0".to_string(),
            Self::M1BestKNonzero => "m1 best_k != k=(0,0)".to_string(),
            Self::M2BestKNonzero => "m2 best_k != k=(0,0)".to_string(),
            Self::GapBucketSameOrAdjacent => "gap_bucket in {same,adjacent}".to_string(),
            Self::SameDigit => "same_digit".to_string(),
            Self::Threshold { field, op, value } => {
                format!(
                    "{} {} {}",
                    field.as_str(),
                    op.as_str(),
                    format_threshold_value(*value)
                )
            }
        }
    }

    pub fn evaluate(&self, row: &HingeFeatureRow) -> bool {
        match self {
            Self::M2StableZeroPrimeDeltaCountPositive => row.m2_stable_zero_prime_delta_count > 0,
            Self::M2BoundaryPrimeDeltaCountNonpositive => row.m2_boundary_prime_delta_count <= 0,
            Self::M2StableZeroSignalMarginCountPositive => {
                row.m2_stable_zero_signal_margin_count > 0
            }
            Self::M2SharedPrimeRateDeltaPositive => row.m2_shared_prime_rate_delta_pp > 0.0,
            Self::M1AnomalyMassPositive => row.m1_anomaly_mass_pp > 0.0,
            Self::M1BestKNonzero => row.m1_best_k != "k=(0,0)",
            Self::M2BestKNonzero => row.m2_best_k != "k=(0,0)",
            Self::GapBucketSameOrAdjacent => row.gap_bucket != "wide",
            Self::SameDigit => row.same_digit,
            Self::Threshold { field, op, value } => match op {
                HingeThresholdOp::Ge => field.value(row) + 1e-12 >= *value,
                HingeThresholdOp::Le => field.value(row) <= *value + 1e-12,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct HingeAtomSpec {
    pub label: String,
    pub family: HingeAtomFamily,
    pub theorem_class: HingeAtomTheoremClass,
    pub threshold_based: bool,
    pub complexity_score: usize,
    pub predicate: HingeAtomPredicate,
    pub mask: Vec<bool>,
}

#[derive(Debug, Clone)]
struct AtomRegistration<'a> {
    family: HingeAtomFamily,
    theorem_class: HingeAtomTheoremClass,
    threshold_based: bool,
    complexity_score: usize,
    predicate: HingeAtomPredicate,
    label: &'a str,
}

#[derive(Debug, Clone)]
pub struct HingeSearchProblem<'a> {
    pub id: &'static str,
    pub label: &'static str,
    pub rows: Vec<&'a HingeFeatureRow>,
    pub target: Vec<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HingeRuleCandidate {
    pub search_id: String,
    pub search_label: String,
    pub atom_count: usize,
    pub rule_label: String,
    pub exact_match: bool,
    pub total_errors: usize,
    pub true_positive: usize,
    pub false_positive: usize,
    pub false_negative: usize,
    pub true_negative: usize,
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
    pub jaccard: f64,
    pub positive_support: usize,
    pub complexity_score: usize,
    pub threshold_free: bool,
    pub rule_theorem_class: HingeAtomTheoremClass,
    pub atom_labels: Vec<String>,
    pub atom_predicates: Vec<HingeAtomPredicate>,
    pub atom_families: Vec<HingeAtomFamily>,
    pub atom_theorem_classes: Vec<HingeAtomTheoremClass>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HingeSearchSummary {
    pub search_id: String,
    pub search_label: String,
    pub dataset_rows: usize,
    pub positive_rows: usize,
    pub atom_pool_size: usize,
    pub searched_rule_count: usize,
    pub any_exact_rule: bool,
    pub best_rule_label: String,
    pub best_rule_status: String,
}

#[derive(Debug, Clone)]
pub struct HingeSearchOutcome {
    pub summary: HingeSearchSummary,
    pub candidate_rows: Vec<HingeRuleCandidate>,
    pub best_rows: Vec<HingeRuleCandidate>,
}

pub fn search_label(search_id: &str) -> &'static str {
    HINGE_SEARCHES
        .iter()
        .find(|(id, _)| *id == search_id)
        .map(|(_, label)| *label)
        .unwrap_or("unknown search")
}

pub fn build_hinge_search_problems<'a>(rows: &'a [HingeFeatureRow]) -> Vec<HingeSearchProblem<'a>> {
    let main_active_rows = rows.iter().filter(|row| row.m2_active).collect::<Vec<_>>();
    let persistent_rows = rows
        .iter()
        .filter(|row| row.m2_active && row.m1_to_m2_persistent)
        .collect::<Vec<_>>();
    let core_like_rows = rows
        .iter()
        .filter(|row| row.m2_active && row.shared_yield_core)
        .collect::<Vec<_>>();

    vec![
        HingeSearchProblem {
            id: HINGE_SEARCH_PRIMARY,
            label: search_label(HINGE_SEARCH_PRIMARY),
            target: main_active_rows
                .iter()
                .map(|row| row.hinge_category == HINGE_CATEGORY_PERSISTENT_CORE)
                .collect(),
            rows: main_active_rows,
        },
        HingeSearchProblem {
            id: HINGE_SEARCH_PERSISTENT,
            label: search_label(HINGE_SEARCH_PERSISTENT),
            target: persistent_rows
                .iter()
                .map(|row| row.shared_yield_core)
                .collect(),
            rows: persistent_rows,
        },
        HingeSearchProblem {
            id: HINGE_SEARCH_CORE,
            label: search_label(HINGE_SEARCH_CORE),
            target: core_like_rows
                .iter()
                .map(|row| row.m1_to_m2_persistent)
                .collect(),
            rows: core_like_rows,
        },
    ]
}

pub fn build_hinge_atom_specs(problem: &HingeSearchProblem<'_>) -> Vec<HingeAtomSpec> {
    build_hinge_atom_specs_with_policy(problem, &default_hinge_atom_catalog_policy())
}

pub fn build_hinge_atom_specs_with_policy(
    problem: &HingeSearchProblem<'_>,
    policy: &HingeAtomCatalogPolicy,
) -> Vec<HingeAtomSpec> {
    let mut dedup = BTreeMap::<String, HingeAtomSpec>::new();

    register_fixed_atoms(&mut dedup, &problem.rows, policy);
    add_threshold_atoms_with_policy(
        &mut dedup,
        &problem.rows,
        policy,
        HingeThresholdField::M2AdmissibleOverlapJaccard,
    );
    add_threshold_atoms_with_policy(
        &mut dedup,
        &problem.rows,
        policy,
        HingeThresholdField::M2MaskStabilityShare,
    );
    add_threshold_atoms_with_policy(
        &mut dedup,
        &problem.rows,
        policy,
        HingeThresholdField::M2NonzeroChurnShare,
    );
    add_threshold_atoms_with_policy(
        &mut dedup,
        &problem.rows,
        policy,
        HingeThresholdField::M2StableZeroSupportRatio,
    );
    add_threshold_atoms_with_policy(
        &mut dedup,
        &problem.rows,
        policy,
        HingeThresholdField::M1AnomalyMassPp,
    );
    add_threshold_atoms_with_policy(
        &mut dedup,
        &problem.rows,
        policy,
        HingeThresholdField::M2AnomalyMassPp,
    );
    add_threshold_atoms_with_policy(
        &mut dedup,
        &problem.rows,
        policy,
        HingeThresholdField::M1StableZeroSignalMarginCount,
    );
    add_threshold_atoms_with_policy(
        &mut dedup,
        &problem.rows,
        policy,
        HingeThresholdField::M2StableZeroSignalMarginCount,
    );

    let mut atoms = dedup.into_values().collect::<Vec<_>>();
    atoms.sort_by(|left, right| {
        left.complexity_score
            .cmp(&right.complexity_score)
            .then_with(|| left.label.cmp(&right.label))
    });
    atoms
}

fn register_fixed_atoms(
    dedup: &mut BTreeMap<String, HingeAtomSpec>,
    rows: &[&HingeFeatureRow],
    policy: &HingeAtomCatalogPolicy,
) {
    let registrations = [
        AtomRegistration {
            family: HingeAtomFamily::OverlapBoundary,
            theorem_class: HingeAtomTheoremClass::ExactTransferSubstrate,
            threshold_based: false,
            complexity_score: 0,
            predicate: HingeAtomPredicate::M2StableZeroPrimeDeltaCountPositive,
            label: "m2 stable_zero_prime_delta_count > 0",
        },
        AtomRegistration {
            family: HingeAtomFamily::OverlapBoundary,
            theorem_class: HingeAtomTheoremClass::ExactTransferSubstrate,
            threshold_based: false,
            complexity_score: 0,
            predicate: HingeAtomPredicate::M2BoundaryPrimeDeltaCountNonpositive,
            label: "m2 boundary_prime_delta_count <= 0",
        },
        AtomRegistration {
            family: HingeAtomFamily::OverlapBoundary,
            theorem_class: HingeAtomTheoremClass::ExactTransferSubstrate,
            threshold_based: false,
            complexity_score: 0,
            predicate: HingeAtomPredicate::M2StableZeroSignalMarginCountPositive,
            label: "m2 stable_zero_signal_margin_count > 0",
        },
        AtomRegistration {
            family: HingeAtomFamily::OverlapBoundary,
            theorem_class: HingeAtomTheoremClass::ExactTransferSubstrate,
            threshold_based: false,
            complexity_score: 0,
            predicate: HingeAtomPredicate::M2SharedPrimeRateDeltaPositive,
            label: "m2 shared_prime_rate_delta_pp > 0",
        },
        AtomRegistration {
            family: HingeAtomFamily::CarryThrough,
            theorem_class: HingeAtomTheoremClass::CrossMExactButEmpirical,
            threshold_based: false,
            complexity_score: 0,
            predicate: HingeAtomPredicate::M1AnomalyMassPositive,
            label: "m1 anomaly_mass_pp > 0",
        },
        AtomRegistration {
            family: HingeAtomFamily::TemplateChoice,
            theorem_class: HingeAtomTheoremClass::Diagnostic,
            threshold_based: false,
            complexity_score: 0,
            predicate: HingeAtomPredicate::M1BestKNonzero,
            label: "m1 best_k != k=(0,0)",
        },
        AtomRegistration {
            family: HingeAtomFamily::TemplateChoice,
            theorem_class: HingeAtomTheoremClass::Diagnostic,
            threshold_based: false,
            complexity_score: 0,
            predicate: HingeAtomPredicate::M2BestKNonzero,
            label: "m2 best_k != k=(0,0)",
        },
        AtomRegistration {
            family: HingeAtomFamily::Geometry,
            theorem_class: HingeAtomTheoremClass::Diagnostic,
            threshold_based: false,
            complexity_score: 0,
            predicate: HingeAtomPredicate::GapBucketSameOrAdjacent,
            label: "gap_bucket in {same,adjacent}",
        },
        AtomRegistration {
            family: HingeAtomFamily::Geometry,
            theorem_class: HingeAtomTheoremClass::Diagnostic,
            threshold_based: false,
            complexity_score: 0,
            predicate: HingeAtomPredicate::SameDigit,
            label: "same_digit",
        },
    ];

    for registration in registrations {
        if !policy.includes_family(registration.family) {
            continue;
        }
        register_atom(dedup, rows, registration);
    }
}

pub fn enumerate_hinge_rule_candidates(
    problem: &HingeSearchProblem<'_>,
    atoms: &[HingeAtomSpec],
    max_rule_atoms: usize,
) -> Vec<HingeRuleCandidate> {
    let positive_rows = problem.target.iter().filter(|&&value| value).count();
    let dataset_rows = problem.rows.len();
    let mut candidates = Vec::new();

    for first in 0..atoms.len() {
        candidates.push(build_rule_candidate(
            problem,
            &[first],
            atoms,
            dataset_rows,
            positive_rows,
        ));
        if max_rule_atoms < 2 {
            continue;
        }
        for second in (first + 1)..atoms.len() {
            candidates.push(build_rule_candidate(
                problem,
                &[first, second],
                atoms,
                dataset_rows,
                positive_rows,
            ));
            if max_rule_atoms < 3 {
                continue;
            }
            for third in (second + 1)..atoms.len() {
                candidates.push(build_rule_candidate(
                    problem,
                    &[first, second, third],
                    atoms,
                    dataset_rows,
                    positive_rows,
                ));
            }
        }
    }
    candidates
}

pub fn run_hinge_rule_search(
    problem: &HingeSearchProblem<'_>,
    atoms: &[HingeAtomSpec],
    max_rule_atoms: usize,
    exported_rule_frontier: usize,
    best_rules_per_search: usize,
) -> HingeSearchOutcome {
    let mut all_candidates = enumerate_hinge_rule_candidates(problem, atoms, max_rule_atoms);
    all_candidates.sort_by(rule_candidate_quality_sort);

    let any_exact_rule = all_candidates.iter().any(|row| row.exact_match);
    let best_rule_status = if any_exact_rule {
        "exact_rule"
    } else {
        "no_exact_rule"
    }
    .to_string();

    let mut best_rows = if any_exact_rule {
        let mut exact_rows = all_candidates
            .iter()
            .filter(|row| row.exact_match)
            .cloned()
            .collect::<Vec<_>>();
        exact_rows.sort_by(rule_candidate_exact_sort);
        exact_rows
    } else {
        let mut frontier_rows = all_candidates.clone();
        frontier_rows.sort_by(rule_candidate_near_sort);
        frontier_rows
    };
    best_rows.truncate(best_rules_per_search);

    let mut candidate_rows = if any_exact_rule {
        let mut rows = all_candidates.clone();
        rows.sort_by(rule_candidate_quality_sort);
        rows
    } else {
        let mut rows = all_candidates.clone();
        rows.sort_by(rule_candidate_near_sort);
        rows
    };
    candidate_rows.truncate(exported_rule_frontier);

    HingeSearchOutcome {
        summary: HingeSearchSummary {
            search_id: problem.id.to_string(),
            search_label: problem.label.to_string(),
            dataset_rows: problem.rows.len(),
            positive_rows: problem.target.iter().filter(|&&value| value).count(),
            atom_pool_size: atoms.len(),
            searched_rule_count: all_candidates.len(),
            any_exact_rule,
            best_rule_label: best_rows
                .first()
                .map(|row| row.rule_label.clone())
                .unwrap_or_else(|| "none".to_string()),
            best_rule_status,
        },
        candidate_rows,
        best_rows,
    }
}

pub fn reevaluate_hinge_rule_candidate(
    problem: &HingeSearchProblem<'_>,
    candidate: &HingeRuleCandidate,
) -> HingeRuleCandidate {
    let predicted = problem
        .rows
        .iter()
        .map(|row| {
            candidate
                .atom_predicates
                .iter()
                .all(|predicate| predicate.evaluate(row))
        })
        .collect::<Vec<_>>();

    build_rule_candidate_from_prediction(
        problem,
        &candidate.atom_labels,
        &candidate.atom_predicates,
        &candidate.atom_families,
        &candidate.atom_theorem_classes,
        &predicted,
        candidate.complexity_score,
    )
}

pub fn rule_candidate_quality_sort(
    left: &HingeRuleCandidate,
    right: &HingeRuleCandidate,
) -> Ordering {
    right
        .exact_match
        .cmp(&left.exact_match)
        .then_with(|| left.total_errors.cmp(&right.total_errors))
        .then_with(|| left.atom_count.cmp(&right.atom_count))
        .then_with(|| right.true_positive.cmp(&left.true_positive))
        .then_with(|| left.complexity_score.cmp(&right.complexity_score))
        .then_with(|| {
            left.rule_theorem_class
                .depth_rank()
                .cmp(&right.rule_theorem_class.depth_rank())
        })
        .then_with(|| right.f1.total_cmp(&left.f1))
        .then_with(|| left.rule_label.cmp(&right.rule_label))
}

pub fn rule_candidate_exact_sort(
    left: &HingeRuleCandidate,
    right: &HingeRuleCandidate,
) -> Ordering {
    left.atom_count
        .cmp(&right.atom_count)
        .then_with(|| right.true_positive.cmp(&left.true_positive))
        .then_with(|| left.complexity_score.cmp(&right.complexity_score))
        .then_with(|| {
            left.rule_theorem_class
                .depth_rank()
                .cmp(&right.rule_theorem_class.depth_rank())
        })
        .then_with(|| left.rule_label.cmp(&right.rule_label))
}

pub fn rule_candidate_near_sort(left: &HingeRuleCandidate, right: &HingeRuleCandidate) -> Ordering {
    left.total_errors
        .cmp(&right.total_errors)
        .then_with(|| left.atom_count.cmp(&right.atom_count))
        .then_with(|| right.true_positive.cmp(&left.true_positive))
        .then_with(|| left.complexity_score.cmp(&right.complexity_score))
        .then_with(|| {
            left.rule_theorem_class
                .depth_rank()
                .cmp(&right.rule_theorem_class.depth_rank())
        })
        .then_with(|| right.f1.total_cmp(&left.f1))
        .then_with(|| left.rule_label.cmp(&right.rule_label))
}

fn register_atom(
    dedup: &mut BTreeMap<String, HingeAtomSpec>,
    rows: &[&HingeFeatureRow],
    registration: AtomRegistration<'_>,
) {
    let mask = rows
        .iter()
        .map(|row| registration.predicate.evaluate(row))
        .collect::<Vec<_>>();
    register_atom_mask(
        dedup,
        HingeAtomSpec {
            label: registration.label.to_string(),
            family: registration.family,
            theorem_class: registration.theorem_class,
            threshold_based: registration.threshold_based,
            complexity_score: registration.complexity_score,
            predicate: registration.predicate.clone(),
            mask,
        },
    );
}

fn add_threshold_atoms_with_policy(
    dedup: &mut BTreeMap<String, HingeAtomSpec>,
    rows: &[&HingeFeatureRow],
    policy: &HingeAtomCatalogPolicy,
    field: HingeThresholdField,
) {
    if !policy.includes_family(HingeAtomFamily::ThresholdShape) {
        return;
    }

    let thresholds = unique_thresholds(
        rows.iter()
            .map(|row| policy.threshold_policy.quantize(field.value(row))),
    );
    for threshold in thresholds {
        for op in [HingeThresholdOp::Ge, HingeThresholdOp::Le] {
            let predicate = HingeAtomPredicate::Threshold {
                field,
                op,
                value: threshold,
            };
            let mask = rows
                .iter()
                .map(|row| predicate.evaluate(row))
                .collect::<Vec<_>>();
            let true_count = mask.iter().filter(|&&value| value).count();
            let false_count = mask.len().saturating_sub(true_count);
            if true_count < policy.threshold_policy.min_side()
                || false_count < policy.threshold_policy.min_side()
            {
                continue;
            }
            register_atom_mask(
                dedup,
                HingeAtomSpec {
                    label: predicate.label(),
                    family: HingeAtomFamily::ThresholdShape,
                    theorem_class: HingeAtomTheoremClass::DerivedThreshold,
                    threshold_based: true,
                    complexity_score: 1,
                    predicate,
                    mask,
                },
            );
        }
    }
}

fn register_atom_mask(dedup: &mut BTreeMap<String, HingeAtomSpec>, atom: HingeAtomSpec) {
    if atom.mask.is_empty()
        || atom.mask.iter().all(|&value| value)
        || atom.mask.iter().all(|&value| !value)
    {
        return;
    }

    let key = atom_mask_key(&atom.mask);
    match dedup.get_mut(&key) {
        Some(existing) => {
            let ordering = atom.complexity_score.cmp(&existing.complexity_score);
            if ordering == Ordering::Less
                || (ordering == Ordering::Equal && atom.label < existing.label)
            {
                *existing = atom;
            }
        }
        None => {
            dedup.insert(key, atom);
        }
    }
}

fn build_rule_candidate(
    problem: &HingeSearchProblem<'_>,
    atom_indices: &[usize],
    atoms: &[HingeAtomSpec],
    dataset_rows: usize,
    _positive_rows: usize,
) -> HingeRuleCandidate {
    let mut predicted = vec![true; dataset_rows];
    let mut labels = Vec::with_capacity(atom_indices.len());
    let mut predicates = Vec::with_capacity(atom_indices.len());
    let mut families = Vec::with_capacity(atom_indices.len());
    let mut theorem_classes = Vec::with_capacity(atom_indices.len());
    let mut complexity_score = 0usize;
    let mut rule_theorem_class = HingeAtomTheoremClass::ExactTransferSubstrate;

    for &atom_index in atom_indices {
        let atom = &atoms[atom_index];
        labels.push(atom.label.clone());
        predicates.push(atom.predicate.clone());
        families.push(atom.family);
        theorem_classes.push(atom.theorem_class);
        complexity_score += atom.complexity_score;
        if atom.theorem_class.depth_rank() > rule_theorem_class.depth_rank() {
            rule_theorem_class = atom.theorem_class;
        }
        for (predicted_value, atom_value) in predicted.iter_mut().zip(&atom.mask) {
            *predicted_value &= *atom_value;
        }
    }

    build_rule_candidate_from_prediction(
        problem,
        &labels,
        &predicates,
        &families,
        &theorem_classes,
        &predicted,
        complexity_score,
    )
}

fn build_rule_candidate_from_prediction(
    problem: &HingeSearchProblem<'_>,
    labels: &[String],
    predicates: &[HingeAtomPredicate],
    families: &[HingeAtomFamily],
    theorem_classes: &[HingeAtomTheoremClass],
    predicted: &[bool],
    complexity_score: usize,
) -> HingeRuleCandidate {
    let mut true_positive = 0usize;
    let mut false_positive = 0usize;
    let mut false_negative = 0usize;
    let mut true_negative = 0usize;
    for (&predicted_value, &target_value) in predicted.iter().zip(&problem.target) {
        match (predicted_value, target_value) {
            (true, true) => true_positive += 1,
            (true, false) => false_positive += 1,
            (false, true) => false_negative += 1,
            (false, false) => true_negative += 1,
        }
    }

    let precision = ratio(true_positive, true_positive + false_positive);
    let recall = ratio(true_positive, true_positive + false_negative);
    let f1 = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    };
    let jaccard = ratio(
        true_positive,
        true_positive + false_positive + false_negative,
    );
    let exact_match = false_positive == 0 && false_negative == 0;
    let threshold_free = predicates
        .iter()
        .all(|predicate| !matches!(predicate, HingeAtomPredicate::Threshold { .. }));
    let mut rule_theorem_class = HingeAtomTheoremClass::ExactTransferSubstrate;
    for theorem_class in theorem_classes {
        if theorem_class.depth_rank() > rule_theorem_class.depth_rank() {
            rule_theorem_class = *theorem_class;
        }
    }

    HingeRuleCandidate {
        search_id: problem.id.to_string(),
        search_label: problem.label.to_string(),
        atom_count: labels.len(),
        rule_label: labels.join(" AND "),
        exact_match,
        total_errors: false_positive + false_negative,
        true_positive,
        false_positive,
        false_negative,
        true_negative,
        precision,
        recall,
        f1,
        jaccard,
        positive_support: true_positive,
        complexity_score,
        threshold_free,
        rule_theorem_class,
        atom_labels: labels.to_vec(),
        atom_predicates: predicates.to_vec(),
        atom_families: families.to_vec(),
        atom_theorem_classes: theorem_classes.to_vec(),
    }
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn unique_thresholds(values: impl Iterator<Item = f64>) -> Vec<f64> {
    let mut values = values.collect::<Vec<_>>();
    values.sort_by(|left, right| left.total_cmp(right));
    values.dedup_by(|left, right| (*left - *right).abs() < 1e-12);
    values
}

fn format_threshold_value(value: f64) -> String {
    if (value - value.round()).abs() < 1e-9 {
        format!("{:.0}", value)
    } else {
        format!("{value:.6}")
    }
}

fn atom_mask_key(mask: &[bool]) -> String {
    mask.iter()
        .map(|&value| if value { '1' } else { '0' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::bounded_k::analyze_hinge_feature_row;

    #[test]
    fn primary_atom_catalog_excludes_tautological_shortcuts() {
        let rows = vec![
            analyze_hinge_feature_row(10, 3, 3),
            analyze_hinge_feature_row(14, 13, 11),
            analyze_hinge_feature_row(14, 3, 1),
            analyze_hinge_feature_row(22, 17, 19),
            analyze_hinge_feature_row(26, 23, 23),
        ];
        let problems = build_hinge_search_problems(&rows);
        let primary = problems
            .iter()
            .find(|problem| problem.id == HINGE_SEARCH_PRIMARY)
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
}
