//! Shared exploratory search vocabulary for affine hinge classifiers.
//!
//! This module keeps the affine classifier atlas separate from the existing
//! hinge-atom lane so we can ask two distinct questions:
//! - what the affine local-comparison surface can explain on its own
//! - what becomes visible once we combine that surface with the maintained
//!   non-tautological hinge atoms

use crate::validation::{
    bounded_k::{AffineHingeFeatureRow, HINGE_CATEGORY_PERSISTENT_CORE},
    hinge_atoms::{
        build_hinge_atom_specs, HingeAtomPredicate, HingeSearchProblem, HINGE_SEARCH_CORE,
        HINGE_SEARCH_PERSISTENT, HINGE_SEARCH_PRIMARY,
    },
};
use serde::Serialize;
use std::{cmp::Ordering, collections::BTreeMap};

pub const AFFINE_SEARCHES: &[(&str, &str)] = &[
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AffineSearchMode {
    AffineOnly,
    MixedExisting,
}

impl AffineSearchMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AffineOnly => "affine_only",
            Self::MixedExisting => "mixed_existing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AffineThresholdField {
    M2SameGradientShare,
    M2SameShiftShare,
    M2SameZeroSeedShare,
    M2IdentityShare,
    M2ShiftOnlyShare,
    M2GradientOnlyShare,
    M2ShiftAndGradientShare,
    M1SameZeroSeedShare,
    M1SameGradientShare,
}

impl AffineThresholdField {
    fn as_str(self) -> &'static str {
        match self {
            Self::M2SameGradientShare => "m2 same_gradient_share",
            Self::M2SameShiftShare => "m2 same_shift_share",
            Self::M2SameZeroSeedShare => "m2 same_zero_seed_share",
            Self::M2IdentityShare => "m2 identity_share",
            Self::M2ShiftOnlyShare => "m2 shift_only_share",
            Self::M2GradientOnlyShare => "m2 gradient_only_share",
            Self::M2ShiftAndGradientShare => "m2 shift_and_gradient_share",
            Self::M1SameZeroSeedShare => "m1 same_zero_seed_share",
            Self::M1SameGradientShare => "m1 same_gradient_share",
        }
    }

    fn value(self, row: &AffineHingeFeatureRow) -> f64 {
        match self {
            Self::M2SameGradientShare => row.m2_affine_same_gradient_share,
            Self::M2SameShiftShare => row.m2_affine_same_shift_share,
            Self::M2SameZeroSeedShare => row.m2_affine_same_zero_seed_share,
            Self::M2IdentityShare => row.m2_affine_identity_share,
            Self::M2ShiftOnlyShare => row.m2_affine_shift_only_share,
            Self::M2GradientOnlyShare => row.m2_affine_gradient_only_share,
            Self::M2ShiftAndGradientShare => row.m2_affine_shift_and_gradient_share,
            Self::M1SameZeroSeedShare => row.m1_affine_same_zero_seed_share,
            Self::M1SameGradientShare => row.m1_affine_same_gradient_share,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AffineThresholdOp {
    Ge,
    Le,
}

impl AffineThresholdOp {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ge => ">=",
            Self::Le => "<=",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AffineAtomPredicate {
    M2SameGradientCountAll,
    M2SameShiftCountAll,
    M2SameZeroSeedCountAll,
    M2ShiftOnlyCountPositive,
    M2GradientOnlyCountZero,
    M2ShiftAndGradientCountZero,
    M1SameGradientCountAll,
    M1SameShiftCountAll,
    M1SameZeroSeedCountAll,
    M1ShiftOnlyCountPositive,
    M1GradientOnlyCountZero,
    M1ShiftAndGradientCountZero,
    Threshold {
        field: AffineThresholdField,
        op: AffineThresholdOp,
        value: f64,
    },
}

impl AffineAtomPredicate {
    pub fn label(&self) -> String {
        match self {
            Self::M2SameGradientCountAll => {
                "m2 same_gradient_count = compared_moduli_count".to_string()
            }
            Self::M2SameShiftCountAll => "m2 same_shift_count = compared_moduli_count".to_string(),
            Self::M2SameZeroSeedCountAll => {
                "m2 same_zero_seed_count = compared_moduli_count".to_string()
            }
            Self::M2ShiftOnlyCountPositive => "m2 shift_only_count > 0".to_string(),
            Self::M2GradientOnlyCountZero => "m2 gradient_only_count = 0".to_string(),
            Self::M2ShiftAndGradientCountZero => "m2 shift_and_gradient_count = 0".to_string(),
            Self::M1SameGradientCountAll => {
                "m1 same_gradient_count = compared_moduli_count".to_string()
            }
            Self::M1SameShiftCountAll => "m1 same_shift_count = compared_moduli_count".to_string(),
            Self::M1SameZeroSeedCountAll => {
                "m1 same_zero_seed_count = compared_moduli_count".to_string()
            }
            Self::M1ShiftOnlyCountPositive => "m1 shift_only_count > 0".to_string(),
            Self::M1GradientOnlyCountZero => "m1 gradient_only_count = 0".to_string(),
            Self::M1ShiftAndGradientCountZero => "m1 shift_and_gradient_count = 0".to_string(),
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

    pub fn evaluate(&self, row: &AffineHingeFeatureRow) -> bool {
        match self {
            Self::M2SameGradientCountAll => {
                row.m2_affine_same_gradient_count == row.m2_affine_compared_moduli_count
            }
            Self::M2SameShiftCountAll => {
                row.m2_affine_same_shift_count == row.m2_affine_compared_moduli_count
            }
            Self::M2SameZeroSeedCountAll => {
                row.m2_affine_same_zero_seed_count == row.m2_affine_compared_moduli_count
            }
            Self::M2ShiftOnlyCountPositive => row.m2_affine_shift_only_count > 0,
            Self::M2GradientOnlyCountZero => row.m2_affine_gradient_only_count == 0,
            Self::M2ShiftAndGradientCountZero => row.m2_affine_shift_and_gradient_count == 0,
            Self::M1SameGradientCountAll => {
                row.m1_affine_same_gradient_count == row.m1_affine_compared_moduli_count
            }
            Self::M1SameShiftCountAll => {
                row.m1_affine_same_shift_count == row.m1_affine_compared_moduli_count
            }
            Self::M1SameZeroSeedCountAll => {
                row.m1_affine_same_zero_seed_count == row.m1_affine_compared_moduli_count
            }
            Self::M1ShiftOnlyCountPositive => row.m1_affine_shift_only_count > 0,
            Self::M1GradientOnlyCountZero => row.m1_affine_gradient_only_count == 0,
            Self::M1ShiftAndGradientCountZero => row.m1_affine_shift_and_gradient_count == 0,
            Self::Threshold { field, op, value } => match op {
                AffineThresholdOp::Ge => field.value(row) + 1e-12 >= *value,
                AffineThresholdOp::Le => field.value(row) <= *value + 1e-12,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum AffineClassifierPredicate {
    Affine(AffineAtomPredicate),
    Existing(HingeAtomPredicate),
}

impl AffineClassifierPredicate {
    pub fn label(&self) -> String {
        match self {
            Self::Affine(predicate) => predicate.label(),
            Self::Existing(predicate) => predicate.label(),
        }
    }

    pub fn evaluate(&self, row: &AffineHingeFeatureRow) -> bool {
        match self {
            Self::Affine(predicate) => predicate.evaluate(row),
            Self::Existing(predicate) => predicate.evaluate(&row.hinge_row),
        }
    }

    fn interpretability_rank(&self, threshold_based: bool) -> usize {
        match (self, threshold_based) {
            (Self::Affine(_), false) => 0,
            (Self::Affine(_), true) => 1,
            (Self::Existing(_), false) => 2,
            (Self::Existing(_), true) => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AffineAtomSpec {
    pub label: String,
    pub predicate: AffineClassifierPredicate,
    pub threshold_based: bool,
    pub complexity_score: usize,
    pub interpretability_rank: usize,
    pub mask: Vec<bool>,
}

#[derive(Debug, Clone)]
struct AtomRegistration {
    predicate: AffineClassifierPredicate,
    threshold_based: bool,
    complexity_score: usize,
}

#[derive(Debug, Clone)]
pub struct AffineHingeSearchProblem<'a> {
    pub id: &'static str,
    pub label: &'static str,
    pub rows: Vec<&'a AffineHingeFeatureRow>,
    pub target: Vec<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineRuleCandidate {
    pub search_id: String,
    pub search_label: String,
    pub search_mode: String,
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
    pub positive_support: usize,
    pub complexity_score: usize,
    pub threshold_free: bool,
    pub interpretability_rank: usize,
    pub atom_labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineSearchSummary {
    pub search_id: String,
    pub search_label: String,
    pub search_mode: String,
    pub dataset_rows: usize,
    pub positive_rows: usize,
    pub atom_pool_size: usize,
    pub searched_rule_count: usize,
    pub any_exact_rule: bool,
    pub best_rule_label: String,
    pub best_rule_status: String,
}

#[derive(Debug, Clone)]
pub struct AffineSearchOutcome {
    pub summary: AffineSearchSummary,
    pub candidate_rows: Vec<AffineRuleCandidate>,
    pub best_rows: Vec<AffineRuleCandidate>,
}

pub fn search_label(search_id: &str) -> &'static str {
    AFFINE_SEARCHES
        .iter()
        .find(|(id, _)| *id == search_id)
        .map(|(_, label)| *label)
        .unwrap_or("unknown search")
}

pub fn build_affine_hinge_search_problems<'a>(
    rows: &'a [AffineHingeFeatureRow],
) -> Vec<AffineHingeSearchProblem<'a>> {
    let main_active_rows = rows
        .iter()
        .filter(|row| row.hinge_row.m2_active)
        .collect::<Vec<_>>();
    let persistent_rows = rows
        .iter()
        .filter(|row| row.hinge_row.m2_active && row.hinge_row.m1_to_m2_persistent)
        .collect::<Vec<_>>();
    let core_like_rows = rows
        .iter()
        .filter(|row| row.hinge_row.m2_active && row.hinge_row.shared_yield_core)
        .collect::<Vec<_>>();

    vec![
        AffineHingeSearchProblem {
            id: HINGE_SEARCH_PRIMARY,
            label: search_label(HINGE_SEARCH_PRIMARY),
            target: main_active_rows
                .iter()
                .map(|row| row.hinge_row.hinge_category == HINGE_CATEGORY_PERSISTENT_CORE)
                .collect(),
            rows: main_active_rows,
        },
        AffineHingeSearchProblem {
            id: HINGE_SEARCH_PERSISTENT,
            label: search_label(HINGE_SEARCH_PERSISTENT),
            target: persistent_rows
                .iter()
                .map(|row| row.hinge_row.shared_yield_core)
                .collect(),
            rows: persistent_rows,
        },
        AffineHingeSearchProblem {
            id: HINGE_SEARCH_CORE,
            label: search_label(HINGE_SEARCH_CORE),
            target: core_like_rows
                .iter()
                .map(|row| row.hinge_row.m1_to_m2_persistent)
                .collect(),
            rows: core_like_rows,
        },
    ]
}

pub fn build_affine_atom_specs(
    problem: &AffineHingeSearchProblem<'_>,
    mode: AffineSearchMode,
) -> Vec<AffineAtomSpec> {
    let mut dedup = BTreeMap::<String, AffineAtomSpec>::new();
    register_fixed_affine_atoms(&mut dedup, &problem.rows);
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        AffineThresholdField::M2SameGradientShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        AffineThresholdField::M2SameShiftShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        AffineThresholdField::M2SameZeroSeedShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        AffineThresholdField::M2IdentityShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        AffineThresholdField::M2ShiftOnlyShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        AffineThresholdField::M2GradientOnlyShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        AffineThresholdField::M2ShiftAndGradientShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        AffineThresholdField::M1SameZeroSeedShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        AffineThresholdField::M1SameGradientShare,
    );

    if mode == AffineSearchMode::MixedExisting {
        let hinge_problem = HingeSearchProblem {
            id: problem.id,
            label: problem.label,
            rows: problem.rows.iter().map(|row| &row.hinge_row).collect(),
            target: problem.target.clone(),
        };
        for existing in build_hinge_atom_specs(&hinge_problem) {
            register_atom_mask(
                &mut dedup,
                AffineAtomSpec {
                    label: existing.label,
                    interpretability_rank: AffineClassifierPredicate::Existing(
                        existing.predicate.clone(),
                    )
                    .interpretability_rank(existing.threshold_based),
                    predicate: AffineClassifierPredicate::Existing(existing.predicate),
                    threshold_based: existing.threshold_based,
                    complexity_score: existing.complexity_score,
                    mask: existing.mask,
                },
            );
        }
    }

    let mut atoms = dedup.into_values().collect::<Vec<_>>();
    atoms.sort_by(|left, right| {
        left.complexity_score
            .cmp(&right.complexity_score)
            .then_with(|| left.interpretability_rank.cmp(&right.interpretability_rank))
            .then_with(|| left.label.cmp(&right.label))
    });
    atoms
}

pub fn run_affine_rule_search(
    problem: &AffineHingeSearchProblem<'_>,
    search_mode: AffineSearchMode,
    atoms: &[AffineAtomSpec],
    max_rule_atoms: usize,
    exported_rule_frontier: usize,
    best_rules_per_search: usize,
) -> AffineSearchOutcome {
    let mut all_candidates =
        enumerate_affine_rule_candidates(problem, search_mode, atoms, max_rule_atoms);
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

    AffineSearchOutcome {
        summary: AffineSearchSummary {
            search_id: problem.id.to_string(),
            search_label: problem.label.to_string(),
            search_mode: search_mode.as_str().to_string(),
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

pub fn rule_candidate_quality_sort(
    left: &AffineRuleCandidate,
    right: &AffineRuleCandidate,
) -> Ordering {
    right
        .exact_match
        .cmp(&left.exact_match)
        .then_with(|| left.total_errors.cmp(&right.total_errors))
        .then_with(|| left.atom_count.cmp(&right.atom_count))
        .then_with(|| right.true_positive.cmp(&left.true_positive))
        .then_with(|| left.complexity_score.cmp(&right.complexity_score))
        .then_with(|| left.interpretability_rank.cmp(&right.interpretability_rank))
        .then_with(|| right.f1.total_cmp(&left.f1))
        .then_with(|| left.rule_label.cmp(&right.rule_label))
}

pub fn rule_candidate_exact_sort(
    left: &AffineRuleCandidate,
    right: &AffineRuleCandidate,
) -> Ordering {
    left.atom_count
        .cmp(&right.atom_count)
        .then_with(|| right.true_positive.cmp(&left.true_positive))
        .then_with(|| left.complexity_score.cmp(&right.complexity_score))
        .then_with(|| left.interpretability_rank.cmp(&right.interpretability_rank))
        .then_with(|| left.rule_label.cmp(&right.rule_label))
}

pub fn rule_candidate_near_sort(
    left: &AffineRuleCandidate,
    right: &AffineRuleCandidate,
) -> Ordering {
    left.total_errors
        .cmp(&right.total_errors)
        .then_with(|| left.atom_count.cmp(&right.atom_count))
        .then_with(|| right.true_positive.cmp(&left.true_positive))
        .then_with(|| left.complexity_score.cmp(&right.complexity_score))
        .then_with(|| left.interpretability_rank.cmp(&right.interpretability_rank))
        .then_with(|| right.f1.total_cmp(&left.f1))
        .then_with(|| left.rule_label.cmp(&right.rule_label))
}

fn enumerate_affine_rule_candidates(
    problem: &AffineHingeSearchProblem<'_>,
    search_mode: AffineSearchMode,
    atoms: &[AffineAtomSpec],
    max_rule_atoms: usize,
) -> Vec<AffineRuleCandidate> {
    let positive_rows = problem.target.iter().filter(|&&value| value).count();
    let dataset_rows = problem.rows.len();
    let mut candidates = Vec::new();

    for first in 0..atoms.len() {
        candidates.push(build_rule_candidate(
            problem,
            search_mode,
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
                search_mode,
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
                    search_mode,
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

fn build_rule_candidate(
    problem: &AffineHingeSearchProblem<'_>,
    search_mode: AffineSearchMode,
    atom_indices: &[usize],
    atoms: &[AffineAtomSpec],
    dataset_rows: usize,
    positive_rows: usize,
) -> AffineRuleCandidate {
    let atom_labels = atom_indices
        .iter()
        .map(|&index| atoms[index].label.clone())
        .collect::<Vec<_>>();
    let atom_masks = atom_indices
        .iter()
        .map(|&index| atoms[index].mask.as_slice())
        .collect::<Vec<_>>();
    let predicted = (0..dataset_rows)
        .map(|row_index| atom_masks.iter().all(|mask| mask[row_index]))
        .collect::<Vec<_>>();

    let mut true_positive = 0usize;
    let mut false_positive = 0usize;
    let mut false_negative = 0usize;
    let mut true_negative = 0usize;
    for (predicted_positive, actual_positive) in predicted.iter().zip(&problem.target) {
        match (*predicted_positive, *actual_positive) {
            (true, true) => true_positive += 1,
            (true, false) => false_positive += 1,
            (false, true) => false_negative += 1,
            (false, false) => true_negative += 1,
        }
    }

    let threshold_free = atom_indices
        .iter()
        .all(|&index| !atoms[index].threshold_based);
    let complexity_score = atom_indices
        .iter()
        .map(|&index| atoms[index].complexity_score)
        .sum::<usize>();
    let interpretability_rank = atom_indices
        .iter()
        .map(|&index| atoms[index].interpretability_rank)
        .max()
        .unwrap_or(0);
    let positive_support = predicted.iter().filter(|&&value| value).count();
    let precision = ratio(true_positive, true_positive + false_positive);
    let recall = ratio(true_positive, positive_rows);
    let f1 = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    };

    AffineRuleCandidate {
        search_id: problem.id.to_string(),
        search_label: problem.label.to_string(),
        search_mode: search_mode.as_str().to_string(),
        atom_count: atom_indices.len(),
        rule_label: atom_labels.join(" AND "),
        exact_match: false_positive == 0 && false_negative == 0,
        total_errors: false_positive + false_negative,
        true_positive,
        false_positive,
        false_negative,
        true_negative,
        precision,
        recall,
        f1,
        positive_support,
        complexity_score,
        threshold_free,
        interpretability_rank,
        atom_labels,
    }
}

fn register_fixed_affine_atoms(
    dedup: &mut BTreeMap<String, AffineAtomSpec>,
    rows: &[&AffineHingeFeatureRow],
) {
    let registrations = [
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(
                AffineAtomPredicate::M2SameGradientCountAll,
            ),
            threshold_based: false,
            complexity_score: 0,
        },
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(AffineAtomPredicate::M2SameShiftCountAll),
            threshold_based: false,
            complexity_score: 0,
        },
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(
                AffineAtomPredicate::M2SameZeroSeedCountAll,
            ),
            threshold_based: false,
            complexity_score: 0,
        },
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(
                AffineAtomPredicate::M2ShiftOnlyCountPositive,
            ),
            threshold_based: false,
            complexity_score: 0,
        },
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(
                AffineAtomPredicate::M2GradientOnlyCountZero,
            ),
            threshold_based: false,
            complexity_score: 0,
        },
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(
                AffineAtomPredicate::M2ShiftAndGradientCountZero,
            ),
            threshold_based: false,
            complexity_score: 0,
        },
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(
                AffineAtomPredicate::M1SameGradientCountAll,
            ),
            threshold_based: false,
            complexity_score: 0,
        },
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(AffineAtomPredicate::M1SameShiftCountAll),
            threshold_based: false,
            complexity_score: 0,
        },
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(
                AffineAtomPredicate::M1SameZeroSeedCountAll,
            ),
            threshold_based: false,
            complexity_score: 0,
        },
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(
                AffineAtomPredicate::M1ShiftOnlyCountPositive,
            ),
            threshold_based: false,
            complexity_score: 0,
        },
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(
                AffineAtomPredicate::M1GradientOnlyCountZero,
            ),
            threshold_based: false,
            complexity_score: 0,
        },
        AtomRegistration {
            predicate: AffineClassifierPredicate::Affine(
                AffineAtomPredicate::M1ShiftAndGradientCountZero,
            ),
            threshold_based: false,
            complexity_score: 0,
        },
    ];

    for registration in registrations {
        register_atom(dedup, rows, registration);
    }
}

fn add_threshold_atoms(
    dedup: &mut BTreeMap<String, AffineAtomSpec>,
    rows: &[&AffineHingeFeatureRow],
    field: AffineThresholdField,
) {
    let mut values = rows.iter().map(|row| field.value(row)).collect::<Vec<_>>();
    values.sort_by(|left, right| left.total_cmp(right));
    values.dedup_by(|left, right| (*left - *right).abs() < 1e-12);

    for value in values {
        register_atom(
            dedup,
            rows,
            AtomRegistration {
                predicate: AffineClassifierPredicate::Affine(AffineAtomPredicate::Threshold {
                    field,
                    op: AffineThresholdOp::Ge,
                    value,
                }),
                threshold_based: true,
                complexity_score: 1,
            },
        );
        register_atom(
            dedup,
            rows,
            AtomRegistration {
                predicate: AffineClassifierPredicate::Affine(AffineAtomPredicate::Threshold {
                    field,
                    op: AffineThresholdOp::Le,
                    value,
                }),
                threshold_based: true,
                complexity_score: 1,
            },
        );
    }
}

fn register_atom(
    dedup: &mut BTreeMap<String, AffineAtomSpec>,
    rows: &[&AffineHingeFeatureRow],
    registration: AtomRegistration,
) {
    let label = registration.predicate.label();
    let mask = rows
        .iter()
        .map(|row| registration.predicate.evaluate(row))
        .collect::<Vec<_>>();
    register_atom_mask(
        dedup,
        AffineAtomSpec {
            label,
            interpretability_rank: registration
                .predicate
                .interpretability_rank(registration.threshold_based),
            predicate: registration.predicate,
            threshold_based: registration.threshold_based,
            complexity_score: registration.complexity_score,
            mask,
        },
    );
}

fn register_atom_mask(dedup: &mut BTreeMap<String, AffineAtomSpec>, spec: AffineAtomSpec) {
    dedup.entry(spec.label.clone()).or_insert(spec);
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn format_threshold_value(value: f64) -> String {
    if (value.round() - value).abs() < 1e-12 {
        format!("{}", value.round() as i64)
    } else {
        format!("{value:.6}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::bounded_k::analyze_affine_hinge_feature_row;

    fn main_rows() -> Vec<AffineHingeFeatureRow> {
        [10, 14, 22, 26]
            .into_iter()
            .flat_map(|base| {
                crate::validation::bounded_k::ordered_unit_pairs(base)
                    .into_iter()
                    .map(move |(outer, inner)| analyze_affine_hinge_feature_row(base, outer, inner))
            })
            .collect()
    }

    #[test]
    fn affine_searches_are_deterministic_on_main_surface() {
        let rows = main_rows();
        let problems = build_affine_hinge_search_problems(&rows);
        let primary = problems
            .iter()
            .find(|problem| problem.id == HINGE_SEARCH_PRIMARY)
            .expect("primary affine search should exist");
        let atoms = build_affine_atom_specs(primary, AffineSearchMode::AffineOnly);
        let outcome_a =
            run_affine_rule_search(primary, AffineSearchMode::AffineOnly, &atoms, 3, 10, 5);
        let outcome_b =
            run_affine_rule_search(primary, AffineSearchMode::AffineOnly, &atoms, 3, 10, 5);
        assert_eq!(
            outcome_a.summary.best_rule_label,
            outcome_b.summary.best_rule_label
        );
        assert_eq!(
            outcome_a.summary.best_rule_status,
            outcome_b.summary.best_rule_status
        );
    }
}
