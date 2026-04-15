//! Derived species/order analysis built on top of the exact affine period-lock
//! surface.
//!
//! This module keeps the exact local law in `affine_period_lock` untouched and
//! instead asks a downstream atlas question:
//!
//! - which direct lane comparisons are period-locked at low order vs higher order
//! - how the locked rows split into `identity` vs `gradient_only`
//! - how the unlocked rows split into `shift_only` vs `shift_and_gradient`
//! - whether the hinge species separate on that order/residual surface

use crate::validation::{
    affine_period_lock::scan_k_config_affine_period_lock_comparison,
    bounded_k::{
        analyze_hinge_feature_row, format_k, parse_k_label, BoundedKConfig, HingeFeatureRow,
        DEFAULT_BOUNDED_K_GRID, HINGE_CATEGORY_PERSISTENT_CORE,
    },
    hinge_atoms::{
        build_hinge_atom_specs, HingeAtomPredicate, HingeSearchProblem, HINGE_SEARCH_PERSISTENT,
        HINGE_SEARCH_PRIMARY,
    },
};
use serde::Serialize;
use std::{cmp::Ordering, collections::BTreeMap};

const MAIN_BASES: &[u32] = &[10, 14, 22, 26];
const CONTROL_BASE: u32 = 30;
const K00: BoundedKConfig = (0, 0);

pub const PERIOD_LOCK_SEARCH_BASE22_POCKET: &str = "base22_gradient_pocket_split";
pub const PERIOD_LOCK_SEARCH_BASE30_CONTROL: &str = "base30_control_split";

pub const PERIOD_LOCK_SEARCHES: &[(&str, &str)] = &[
    (
        HINGE_SEARCH_PRIMARY,
        "Primary: persistent_core vs other main M=2 actives",
    ),
    (
        HINGE_SEARCH_PERSISTENT,
        "Secondary: overlap-dominant vs boundary-dominant persistent survivors",
    ),
    (
        PERIOD_LOCK_SEARCH_BASE22_POCKET,
        "Secondary: active base-22 pocket vs hinge-like rows",
    ),
    (
        PERIOD_LOCK_SEARCH_BASE30_CONTROL,
        "Secondary: main active rows vs base-30 control rows",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderBucket {
    Ord1,
    Ord2,
    OrdGe3,
}

impl OrderBucket {
    pub fn from_order(order: u32) -> Self {
        match order {
            1 => Self::Ord1,
            2 => Self::Ord2,
            _ => Self::OrdGe3,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ord1 => "ord_1",
            Self::Ord2 => "ord_2",
            Self::OrdGe3 => "ord_ge_3",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PeriodLockOrderCellRow {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub pair_label: String,
    pub from_k: String,
    pub to_k: String,
    pub modulus: u32,
    pub multiplicative_order: u32,
    pub order_bucket: String,
    pub locked: bool,
    pub shift_equal: bool,
    pub zero_seed_equal: bool,
    pub gradient_position_delta: i32,
    pub local_relation_label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LockedShiftResidualRow {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub pair_label: String,
    pub from_k: String,
    pub to_k: String,
    pub modulus: u32,
    pub multiplicative_order: u32,
    pub order_bucket: String,
    pub locked_relation_label: String,
    pub shift_equal: bool,
    pub zero_seed_equal: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PeriodLockLaneSummary {
    pub middle_length: usize,
    pub from_k: String,
    pub to_k: String,
    pub compared_moduli_count: usize,
    pub locked_count: usize,
    pub unlocked_count: usize,
    pub locked_share: f64,
    pub locked_identity_count: usize,
    pub locked_gradient_only_count: usize,
    pub locked_identity_share: f64,
    pub locked_gradient_only_share: f64,
    pub unlocked_shift_only_count: usize,
    pub unlocked_shift_and_gradient_count: usize,
    pub unlocked_shift_only_share: f64,
    pub unlocked_shift_and_gradient_share: f64,
    pub order_1_locked_count: usize,
    pub order_2_locked_count: usize,
    pub order_ge_3_locked_count: usize,
    pub max_lock_order: u32,
    pub has_higher_order_lock: bool,
    pub rare_lock_share: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PeriodLockSpeciesFeatureRow {
    #[serde(flatten)]
    pub hinge_row: HingeFeatureRow,
    pub m1_compared_moduli_count: usize,
    pub m1_locked_count: usize,
    pub m1_unlocked_count: usize,
    pub m1_locked_share: f64,
    pub m1_locked_identity_count: usize,
    pub m1_locked_gradient_only_count: usize,
    pub m1_locked_identity_share: f64,
    pub m1_locked_gradient_only_share: f64,
    pub m1_unlocked_shift_only_count: usize,
    pub m1_unlocked_shift_and_gradient_count: usize,
    pub m1_unlocked_shift_only_share: f64,
    pub m1_unlocked_shift_and_gradient_share: f64,
    pub m1_order_1_locked_count: usize,
    pub m1_order_2_locked_count: usize,
    pub m1_order_ge_3_locked_count: usize,
    pub m1_max_lock_order: u32,
    pub m1_has_higher_order_lock: bool,
    pub m1_rare_lock_share: f64,
    pub m1_direct_lane_summaries: Vec<PeriodLockLaneSummary>,
    pub m1_winner_projection: Option<PeriodLockLaneSummary>,
    pub m2_compared_moduli_count: usize,
    pub m2_locked_count: usize,
    pub m2_unlocked_count: usize,
    pub m2_locked_share: f64,
    pub m2_locked_identity_count: usize,
    pub m2_locked_gradient_only_count: usize,
    pub m2_locked_identity_share: f64,
    pub m2_locked_gradient_only_share: f64,
    pub m2_unlocked_shift_only_count: usize,
    pub m2_unlocked_shift_and_gradient_count: usize,
    pub m2_unlocked_shift_only_share: f64,
    pub m2_unlocked_shift_and_gradient_share: f64,
    pub m2_order_1_locked_count: usize,
    pub m2_order_2_locked_count: usize,
    pub m2_order_ge_3_locked_count: usize,
    pub m2_max_lock_order: u32,
    pub m2_has_higher_order_lock: bool,
    pub m2_rare_lock_share: f64,
    pub m2_direct_lane_summaries: Vec<PeriodLockLaneSummary>,
    pub m2_winner_projection: Option<PeriodLockLaneSummary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PeriodLockSearchMode {
    PeriodLockOnly,
    PeriodLockMixed,
}

impl PeriodLockSearchMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PeriodLockOnly => "period_lock_only",
            Self::PeriodLockMixed => "period_lock_mixed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PeriodLockThresholdField {
    M2LockedShare,
    M2RareLockShare,
    M2LockedGradientOnlyShare,
    M2LockedIdentityShare,
    M1LockedShare,
    M1RareLockShare,
}

impl PeriodLockThresholdField {
    fn as_str(self) -> &'static str {
        match self {
            Self::M2LockedShare => "m2 locked_share",
            Self::M2RareLockShare => "m2 rare_lock_share",
            Self::M2LockedGradientOnlyShare => "m2 locked_gradient_only_share",
            Self::M2LockedIdentityShare => "m2 locked_identity_share",
            Self::M1LockedShare => "m1 locked_share",
            Self::M1RareLockShare => "m1 rare_lock_share",
        }
    }

    fn value(self, row: &PeriodLockSpeciesFeatureRow) -> f64 {
        match self {
            Self::M2LockedShare => row.m2_locked_share,
            Self::M2RareLockShare => row.m2_rare_lock_share,
            Self::M2LockedGradientOnlyShare => row.m2_locked_gradient_only_share,
            Self::M2LockedIdentityShare => row.m2_locked_identity_share,
            Self::M1LockedShare => row.m1_locked_share,
            Self::M1RareLockShare => row.m1_rare_lock_share,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PeriodLockThresholdOp {
    Ge,
    Le,
}

impl PeriodLockThresholdOp {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ge => ">=",
            Self::Le => "<=",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PeriodLockAtomPredicate {
    M2HasHigherOrderLock,
    M2MaxLockOrderGe3,
    M2LockedGradientOnlyCountPositive,
    M2LockedIdentityCountAll,
    M2UnlockedShiftOnlyCountZero,
    M1LockedCountPositive,
    M1HasHigherOrderLock,
    Threshold {
        field: PeriodLockThresholdField,
        op: PeriodLockThresholdOp,
        value: f64,
    },
}

impl PeriodLockAtomPredicate {
    pub fn label(&self) -> String {
        match self {
            Self::M2HasHigherOrderLock => "m2 has_higher_order_lock".to_string(),
            Self::M2MaxLockOrderGe3 => "m2 max_lock_order >= 3".to_string(),
            Self::M2LockedGradientOnlyCountPositive => {
                "m2 locked_gradient_only_count > 0".to_string()
            }
            Self::M2LockedIdentityCountAll => {
                "m2 locked_identity_count = compared_locked_count".to_string()
            }
            Self::M2UnlockedShiftOnlyCountZero => "m2 unlocked_shift_only_count = 0".to_string(),
            Self::M1LockedCountPositive => "m1 locked_count > 0".to_string(),
            Self::M1HasHigherOrderLock => "m1 has_higher_order_lock".to_string(),
            Self::Threshold { field, op, value } => format!(
                "{} {} {}",
                field.as_str(),
                op.as_str(),
                format_threshold_value(*value)
            ),
        }
    }

    pub fn evaluate(&self, row: &PeriodLockSpeciesFeatureRow) -> bool {
        match self {
            Self::M2HasHigherOrderLock => row.m2_has_higher_order_lock,
            Self::M2MaxLockOrderGe3 => row.m2_max_lock_order >= 3,
            Self::M2LockedGradientOnlyCountPositive => row.m2_locked_gradient_only_count > 0,
            Self::M2LockedIdentityCountAll => row.m2_locked_identity_count == row.m2_locked_count,
            Self::M2UnlockedShiftOnlyCountZero => row.m2_unlocked_shift_only_count == 0,
            Self::M1LockedCountPositive => row.m1_locked_count > 0,
            Self::M1HasHigherOrderLock => row.m1_has_higher_order_lock,
            Self::Threshold { field, op, value } => match op {
                PeriodLockThresholdOp::Ge => field.value(row) + 1e-12 >= *value,
                PeriodLockThresholdOp::Le => field.value(row) <= *value + 1e-12,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum PeriodLockClassifierPredicate {
    PeriodLock(PeriodLockAtomPredicate),
    Existing(HingeAtomPredicate),
}

impl PeriodLockClassifierPredicate {
    pub fn label(&self) -> String {
        match self {
            Self::PeriodLock(predicate) => predicate.label(),
            Self::Existing(predicate) => predicate.label(),
        }
    }

    pub fn evaluate(&self, row: &PeriodLockSpeciesFeatureRow) -> bool {
        match self {
            Self::PeriodLock(predicate) => predicate.evaluate(row),
            Self::Existing(predicate) => predicate.evaluate(&row.hinge_row),
        }
    }

    fn interpretability_rank(&self, threshold_based: bool) -> usize {
        match (self, threshold_based) {
            (Self::PeriodLock(_), false) => 0,
            (Self::PeriodLock(_), true) => 1,
            (Self::Existing(_), false) => 2,
            (Self::Existing(_), true) => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PeriodLockAtomSpec {
    pub label: String,
    pub predicate: PeriodLockClassifierPredicate,
    pub threshold_based: bool,
    pub complexity_score: usize,
    pub interpretability_rank: usize,
    pub mask: Vec<bool>,
}

#[derive(Debug, Clone)]
pub struct PeriodLockSearchProblem<'a> {
    pub id: &'static str,
    pub label: &'static str,
    pub rows: Vec<&'a PeriodLockSpeciesFeatureRow>,
    pub target: Vec<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PeriodLockRuleCandidate {
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
pub struct PeriodLockSearchSummary {
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
pub struct PeriodLockSearchOutcome {
    pub summary: PeriodLockSearchSummary,
    pub candidate_rows: Vec<PeriodLockRuleCandidate>,
    pub best_rows: Vec<PeriodLockRuleCandidate>,
}

#[derive(Debug, Clone)]
struct PhaseAggregate {
    compared_moduli_count: usize,
    locked_count: usize,
    unlocked_count: usize,
    locked_share: f64,
    locked_identity_count: usize,
    locked_gradient_only_count: usize,
    locked_identity_share: f64,
    locked_gradient_only_share: f64,
    unlocked_shift_only_count: usize,
    unlocked_shift_and_gradient_count: usize,
    unlocked_shift_only_share: f64,
    unlocked_shift_and_gradient_share: f64,
    order_1_locked_count: usize,
    order_2_locked_count: usize,
    order_ge_3_locked_count: usize,
    max_lock_order: u32,
    has_higher_order_lock: bool,
    rare_lock_share: f64,
    direct_lane_summaries: Vec<PeriodLockLaneSummary>,
    winner_projection: Option<PeriodLockLaneSummary>,
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn noncompact_lanes() -> impl Iterator<Item = BoundedKConfig> {
    DEFAULT_BOUNDED_K_GRID
        .iter()
        .copied()
        .filter(|&config| config != K00)
}

pub fn order_bucket_label(order: u32) -> &'static str {
    OrderBucket::from_order(order).as_str()
}

fn build_lane_summary(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    to_k: BoundedKConfig,
) -> PeriodLockLaneSummary {
    let comparison =
        scan_k_config_affine_period_lock_comparison(base, middle_length, outer, inner, K00, to_k);
    let locked_rows = comparison
        .modulus_rows
        .iter()
        .filter(|row| row.period_lock_expected)
        .collect::<Vec<_>>();
    let unlocked_rows = comparison
        .modulus_rows
        .iter()
        .filter(|row| !row.period_lock_expected)
        .collect::<Vec<_>>();
    let order_1_locked_count = locked_rows
        .iter()
        .filter(|row| row.multiplicative_order == 1)
        .count();
    let order_2_locked_count = locked_rows
        .iter()
        .filter(|row| row.multiplicative_order == 2)
        .count();
    let order_ge_3_locked_count = locked_rows
        .iter()
        .filter(|row| row.multiplicative_order >= 3)
        .count();
    let max_lock_order = locked_rows
        .iter()
        .map(|row| row.multiplicative_order)
        .max()
        .unwrap_or(0);
    let locked_count = locked_rows.len();
    let unlocked_count = unlocked_rows.len();
    let locked_identity_count = comparison.period_locked_identity_count;
    let locked_gradient_only_count = comparison.period_locked_gradient_only_count;
    let unlocked_shift_only_count = comparison.shift_only_count;
    let unlocked_shift_and_gradient_count = comparison.shift_and_gradient_count;

    PeriodLockLaneSummary {
        middle_length,
        from_k: format_k(K00),
        to_k: format_k(to_k),
        compared_moduli_count: comparison.compared_moduli_count,
        locked_count,
        unlocked_count,
        locked_share: ratio(locked_count, comparison.compared_moduli_count),
        locked_identity_count,
        locked_gradient_only_count,
        locked_identity_share: ratio(locked_identity_count, locked_count),
        locked_gradient_only_share: ratio(locked_gradient_only_count, locked_count),
        unlocked_shift_only_count,
        unlocked_shift_and_gradient_count,
        unlocked_shift_only_share: ratio(unlocked_shift_only_count, unlocked_count),
        unlocked_shift_and_gradient_share: ratio(unlocked_shift_and_gradient_count, unlocked_count),
        order_1_locked_count,
        order_2_locked_count,
        order_ge_3_locked_count,
        max_lock_order,
        has_higher_order_lock: order_ge_3_locked_count > 0,
        rare_lock_share: ratio(order_ge_3_locked_count, locked_count),
    }
}

fn aggregate_phase(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    winner_k: BoundedKConfig,
) -> PhaseAggregate {
    let direct_lane_summaries = noncompact_lanes()
        .map(|to_k| build_lane_summary(base, middle_length, outer, inner, to_k))
        .collect::<Vec<_>>();
    let winner_projection = if winner_k == K00 {
        None
    } else {
        direct_lane_summaries
            .iter()
            .find(|summary| summary.to_k == format_k(winner_k))
            .cloned()
    };

    let compared_moduli_count = direct_lane_summaries
        .iter()
        .map(|summary| summary.compared_moduli_count)
        .sum();
    let locked_count = direct_lane_summaries
        .iter()
        .map(|summary| summary.locked_count)
        .sum();
    let unlocked_count = direct_lane_summaries
        .iter()
        .map(|summary| summary.unlocked_count)
        .sum();
    let locked_identity_count = direct_lane_summaries
        .iter()
        .map(|summary| summary.locked_identity_count)
        .sum();
    let locked_gradient_only_count = direct_lane_summaries
        .iter()
        .map(|summary| summary.locked_gradient_only_count)
        .sum();
    let unlocked_shift_only_count = direct_lane_summaries
        .iter()
        .map(|summary| summary.unlocked_shift_only_count)
        .sum();
    let unlocked_shift_and_gradient_count = direct_lane_summaries
        .iter()
        .map(|summary| summary.unlocked_shift_and_gradient_count)
        .sum();
    let order_1_locked_count = direct_lane_summaries
        .iter()
        .map(|summary| summary.order_1_locked_count)
        .sum();
    let order_2_locked_count = direct_lane_summaries
        .iter()
        .map(|summary| summary.order_2_locked_count)
        .sum();
    let order_ge_3_locked_count = direct_lane_summaries
        .iter()
        .map(|summary| summary.order_ge_3_locked_count)
        .sum();
    let max_lock_order = direct_lane_summaries
        .iter()
        .map(|summary| summary.max_lock_order)
        .max()
        .unwrap_or(0);

    PhaseAggregate {
        compared_moduli_count,
        locked_count,
        unlocked_count,
        locked_share: ratio(locked_count, compared_moduli_count),
        locked_identity_count,
        locked_gradient_only_count,
        locked_identity_share: ratio(locked_identity_count, locked_count),
        locked_gradient_only_share: ratio(locked_gradient_only_count, locked_count),
        unlocked_shift_only_count,
        unlocked_shift_and_gradient_count,
        unlocked_shift_only_share: ratio(unlocked_shift_only_count, unlocked_count),
        unlocked_shift_and_gradient_share: ratio(unlocked_shift_and_gradient_count, unlocked_count),
        order_1_locked_count,
        order_2_locked_count,
        order_ge_3_locked_count,
        max_lock_order,
        has_higher_order_lock: order_ge_3_locked_count > 0,
        rare_lock_share: ratio(order_ge_3_locked_count, locked_count),
        direct_lane_summaries,
        winner_projection,
    }
}

pub fn scan_period_lock_order_cells(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    from_k: BoundedKConfig,
    to_k: BoundedKConfig,
) -> Vec<PeriodLockOrderCellRow> {
    let comparison = scan_k_config_affine_period_lock_comparison(
        base,
        middle_length,
        outer,
        inner,
        from_k,
        to_k,
    );
    comparison
        .modulus_rows
        .iter()
        .map(|row| PeriodLockOrderCellRow {
            base,
            middle_length,
            outer,
            inner,
            pair_label: comparison.pair_label.clone(),
            from_k: comparison.from_k.clone(),
            to_k: comparison.to_k.clone(),
            modulus: row.modulus,
            multiplicative_order: row.multiplicative_order,
            order_bucket: order_bucket_label(row.multiplicative_order).to_string(),
            locked: row.period_lock_expected,
            shift_equal: row.shift_equal,
            zero_seed_equal: row.zero_seed_equal,
            gradient_position_delta: row.gradient_position_delta,
            local_relation_label: row.local_relation_label.clone(),
        })
        .collect()
}

pub fn scan_locked_shift_residuals(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    from_k: BoundedKConfig,
    to_k: BoundedKConfig,
) -> Vec<LockedShiftResidualRow> {
    let comparison = scan_k_config_affine_period_lock_comparison(
        base,
        middle_length,
        outer,
        inner,
        from_k,
        to_k,
    );
    comparison
        .modulus_rows
        .iter()
        .filter(|row| row.period_lock_expected)
        .map(|row| LockedShiftResidualRow {
            base,
            middle_length,
            outer,
            inner,
            pair_label: comparison.pair_label.clone(),
            from_k: comparison.from_k.clone(),
            to_k: comparison.to_k.clone(),
            modulus: row.modulus,
            multiplicative_order: row.multiplicative_order,
            order_bucket: order_bucket_label(row.multiplicative_order).to_string(),
            locked_relation_label: row.local_relation_label.clone(),
            shift_equal: row.shift_equal,
            zero_seed_equal: row.zero_seed_equal,
        })
        .collect()
}

pub fn analyze_period_lock_species_feature_row(
    base: u32,
    outer: u32,
    inner: u32,
) -> PeriodLockSpeciesFeatureRow {
    let hinge_row = analyze_hinge_feature_row(base, outer, inner);
    let m1 = aggregate_phase(base, 1, outer, inner, parse_k_label(&hinge_row.m1_best_k));
    let m2 = aggregate_phase(base, 2, outer, inner, parse_k_label(&hinge_row.m2_best_k));

    PeriodLockSpeciesFeatureRow {
        hinge_row,
        m1_compared_moduli_count: m1.compared_moduli_count,
        m1_locked_count: m1.locked_count,
        m1_unlocked_count: m1.unlocked_count,
        m1_locked_share: m1.locked_share,
        m1_locked_identity_count: m1.locked_identity_count,
        m1_locked_gradient_only_count: m1.locked_gradient_only_count,
        m1_locked_identity_share: m1.locked_identity_share,
        m1_locked_gradient_only_share: m1.locked_gradient_only_share,
        m1_unlocked_shift_only_count: m1.unlocked_shift_only_count,
        m1_unlocked_shift_and_gradient_count: m1.unlocked_shift_and_gradient_count,
        m1_unlocked_shift_only_share: m1.unlocked_shift_only_share,
        m1_unlocked_shift_and_gradient_share: m1.unlocked_shift_and_gradient_share,
        m1_order_1_locked_count: m1.order_1_locked_count,
        m1_order_2_locked_count: m1.order_2_locked_count,
        m1_order_ge_3_locked_count: m1.order_ge_3_locked_count,
        m1_max_lock_order: m1.max_lock_order,
        m1_has_higher_order_lock: m1.has_higher_order_lock,
        m1_rare_lock_share: m1.rare_lock_share,
        m1_direct_lane_summaries: m1.direct_lane_summaries,
        m1_winner_projection: m1.winner_projection,
        m2_compared_moduli_count: m2.compared_moduli_count,
        m2_locked_count: m2.locked_count,
        m2_unlocked_count: m2.unlocked_count,
        m2_locked_share: m2.locked_share,
        m2_locked_identity_count: m2.locked_identity_count,
        m2_locked_gradient_only_count: m2.locked_gradient_only_count,
        m2_locked_identity_share: m2.locked_identity_share,
        m2_locked_gradient_only_share: m2.locked_gradient_only_share,
        m2_unlocked_shift_only_count: m2.unlocked_shift_only_count,
        m2_unlocked_shift_and_gradient_count: m2.unlocked_shift_and_gradient_count,
        m2_unlocked_shift_only_share: m2.unlocked_shift_only_share,
        m2_unlocked_shift_and_gradient_share: m2.unlocked_shift_and_gradient_share,
        m2_order_1_locked_count: m2.order_1_locked_count,
        m2_order_2_locked_count: m2.order_2_locked_count,
        m2_order_ge_3_locked_count: m2.order_ge_3_locked_count,
        m2_max_lock_order: m2.max_lock_order,
        m2_has_higher_order_lock: m2.has_higher_order_lock,
        m2_rare_lock_share: m2.rare_lock_share,
        m2_direct_lane_summaries: m2.direct_lane_summaries,
        m2_winner_projection: m2.winner_projection,
    }
}

pub fn search_label(search_id: &str) -> &'static str {
    PERIOD_LOCK_SEARCHES
        .iter()
        .find(|(id, _)| *id == search_id)
        .map(|(_, label)| *label)
        .unwrap_or("unknown search")
}

pub fn build_period_lock_search_problems<'a>(
    rows: &'a [PeriodLockSpeciesFeatureRow],
) -> Vec<PeriodLockSearchProblem<'a>> {
    let primary_rows = rows
        .iter()
        .filter(|row| MAIN_BASES.contains(&row.hinge_row.base) && row.hinge_row.m2_active)
        .collect::<Vec<_>>();
    let persistent_rows = rows
        .iter()
        .filter(|row| {
            MAIN_BASES.contains(&row.hinge_row.base)
                && row.hinge_row.m2_active
                && row.hinge_row.m1_to_m2_persistent
        })
        .collect::<Vec<_>>();
    let pocket_rows = rows
        .iter()
        .filter(|row| {
            (row.hinge_row.base == 22
                && row.hinge_row.m2_active
                && row.hinge_row.m2_best_k == format_k((2, 2))
                && row.m2_locked_gradient_only_count > 0)
                || row.hinge_row.hinge_category == HINGE_CATEGORY_PERSISTENT_CORE
        })
        .collect::<Vec<_>>();
    let control_rows = rows
        .iter()
        .filter(|row| {
            (MAIN_BASES.contains(&row.hinge_row.base) && row.hinge_row.m2_active)
                || row.hinge_row.base == CONTROL_BASE
        })
        .collect::<Vec<_>>();

    vec![
        PeriodLockSearchProblem {
            id: HINGE_SEARCH_PRIMARY,
            label: search_label(HINGE_SEARCH_PRIMARY),
            target: primary_rows
                .iter()
                .map(|row| row.hinge_row.hinge_category == HINGE_CATEGORY_PERSISTENT_CORE)
                .collect(),
            rows: primary_rows,
        },
        PeriodLockSearchProblem {
            id: HINGE_SEARCH_PERSISTENT,
            label: search_label(HINGE_SEARCH_PERSISTENT),
            target: persistent_rows
                .iter()
                .map(|row| row.hinge_row.shared_yield_core)
                .collect(),
            rows: persistent_rows,
        },
        PeriodLockSearchProblem {
            id: PERIOD_LOCK_SEARCH_BASE22_POCKET,
            label: search_label(PERIOD_LOCK_SEARCH_BASE22_POCKET),
            target: pocket_rows
                .iter()
                .map(|row| row.hinge_row.base == 22)
                .collect(),
            rows: pocket_rows,
        },
        PeriodLockSearchProblem {
            id: PERIOD_LOCK_SEARCH_BASE30_CONTROL,
            label: search_label(PERIOD_LOCK_SEARCH_BASE30_CONTROL),
            target: control_rows
                .iter()
                .map(|row| row.hinge_row.base != CONTROL_BASE)
                .collect(),
            rows: control_rows,
        },
    ]
}

pub fn build_period_lock_atom_specs(
    problem: &PeriodLockSearchProblem<'_>,
    mode: PeriodLockSearchMode,
) -> Vec<PeriodLockAtomSpec> {
    let mut dedup = BTreeMap::<String, PeriodLockAtomSpec>::new();
    register_period_lock_atom(
        &mut dedup,
        PeriodLockClassifierPredicate::PeriodLock(PeriodLockAtomPredicate::M2HasHigherOrderLock),
        false,
        1,
        &problem.rows,
    );
    register_period_lock_atom(
        &mut dedup,
        PeriodLockClassifierPredicate::PeriodLock(PeriodLockAtomPredicate::M2MaxLockOrderGe3),
        false,
        1,
        &problem.rows,
    );
    register_period_lock_atom(
        &mut dedup,
        PeriodLockClassifierPredicate::PeriodLock(
            PeriodLockAtomPredicate::M2LockedGradientOnlyCountPositive,
        ),
        false,
        1,
        &problem.rows,
    );
    register_period_lock_atom(
        &mut dedup,
        PeriodLockClassifierPredicate::PeriodLock(
            PeriodLockAtomPredicate::M2LockedIdentityCountAll,
        ),
        false,
        1,
        &problem.rows,
    );
    register_period_lock_atom(
        &mut dedup,
        PeriodLockClassifierPredicate::PeriodLock(
            PeriodLockAtomPredicate::M2UnlockedShiftOnlyCountZero,
        ),
        false,
        1,
        &problem.rows,
    );
    register_period_lock_atom(
        &mut dedup,
        PeriodLockClassifierPredicate::PeriodLock(PeriodLockAtomPredicate::M1LockedCountPositive),
        false,
        1,
        &problem.rows,
    );
    register_period_lock_atom(
        &mut dedup,
        PeriodLockClassifierPredicate::PeriodLock(PeriodLockAtomPredicate::M1HasHigherOrderLock),
        false,
        1,
        &problem.rows,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        PeriodLockThresholdField::M2LockedShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        PeriodLockThresholdField::M2RareLockShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        PeriodLockThresholdField::M2LockedGradientOnlyShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        PeriodLockThresholdField::M2LockedIdentityShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        PeriodLockThresholdField::M1LockedShare,
    );
    add_threshold_atoms(
        &mut dedup,
        &problem.rows,
        PeriodLockThresholdField::M1RareLockShare,
    );

    if mode == PeriodLockSearchMode::PeriodLockMixed {
        let hinge_problem = HingeSearchProblem {
            id: problem.id,
            label: problem.label,
            rows: problem.rows.iter().map(|row| &row.hinge_row).collect(),
            target: problem.target.clone(),
        };
        for existing in build_hinge_atom_specs(&hinge_problem) {
            register_period_lock_atom(
                &mut dedup,
                PeriodLockClassifierPredicate::Existing(existing.predicate),
                existing.threshold_based,
                existing.complexity_score,
                &problem.rows,
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

pub fn run_period_lock_rule_search(
    problem: &PeriodLockSearchProblem<'_>,
    search_mode: PeriodLockSearchMode,
    atoms: &[PeriodLockAtomSpec],
    max_rule_atoms: usize,
    exported_rule_frontier: usize,
    best_rules_per_search: usize,
) -> PeriodLockSearchOutcome {
    let positive_rows = problem.target.iter().filter(|&&value| value).count();
    let dataset_rows = problem.rows.len();
    if dataset_rows == 0 || positive_rows == 0 || positive_rows == dataset_rows {
        return PeriodLockSearchOutcome {
            summary: PeriodLockSearchSummary {
                search_id: problem.id.to_string(),
                search_label: problem.label.to_string(),
                search_mode: search_mode.as_str().to_string(),
                dataset_rows,
                positive_rows,
                atom_pool_size: atoms.len(),
                searched_rule_count: 0,
                any_exact_rule: false,
                best_rule_label: "none".to_string(),
                best_rule_status: "degenerate_target".to_string(),
            },
            candidate_rows: Vec::new(),
            best_rows: Vec::new(),
        };
    }

    let mut all_candidates =
        enumerate_period_lock_rule_candidates(problem, search_mode, atoms, max_rule_atoms);
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

    PeriodLockSearchOutcome {
        summary: PeriodLockSearchSummary {
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
    left: &PeriodLockRuleCandidate,
    right: &PeriodLockRuleCandidate,
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
    left: &PeriodLockRuleCandidate,
    right: &PeriodLockRuleCandidate,
) -> Ordering {
    left.atom_count
        .cmp(&right.atom_count)
        .then_with(|| right.true_positive.cmp(&left.true_positive))
        .then_with(|| left.complexity_score.cmp(&right.complexity_score))
        .then_with(|| left.interpretability_rank.cmp(&right.interpretability_rank))
        .then_with(|| left.rule_label.cmp(&right.rule_label))
}

pub fn rule_candidate_near_sort(
    left: &PeriodLockRuleCandidate,
    right: &PeriodLockRuleCandidate,
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

fn register_period_lock_atom(
    dedup: &mut BTreeMap<String, PeriodLockAtomSpec>,
    predicate: PeriodLockClassifierPredicate,
    threshold_based: bool,
    complexity_score: usize,
    rows: &[&PeriodLockSpeciesFeatureRow],
) {
    let label = predicate.label();
    let interpretability_rank = predicate.interpretability_rank(threshold_based);
    let mask = rows
        .iter()
        .map(|row| predicate.evaluate(row))
        .collect::<Vec<_>>();
    dedup.entry(label.clone()).or_insert(PeriodLockAtomSpec {
        label,
        predicate,
        threshold_based,
        complexity_score,
        interpretability_rank,
        mask,
    });
}

fn add_threshold_atoms(
    dedup: &mut BTreeMap<String, PeriodLockAtomSpec>,
    rows: &[&PeriodLockSpeciesFeatureRow],
    field: PeriodLockThresholdField,
) {
    let mut values = rows
        .iter()
        .map(|row| field.value(row))
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    values.sort_by(f64::total_cmp);
    values.dedup_by(|left, right| left.total_cmp(right) == Ordering::Equal);
    for value in values {
        register_period_lock_atom(
            dedup,
            PeriodLockClassifierPredicate::PeriodLock(PeriodLockAtomPredicate::Threshold {
                field,
                op: PeriodLockThresholdOp::Ge,
                value,
            }),
            true,
            2,
            rows,
        );
        register_period_lock_atom(
            dedup,
            PeriodLockClassifierPredicate::PeriodLock(PeriodLockAtomPredicate::Threshold {
                field,
                op: PeriodLockThresholdOp::Le,
                value,
            }),
            true,
            2,
            rows,
        );
    }
}

fn enumerate_period_lock_rule_candidates(
    problem: &PeriodLockSearchProblem<'_>,
    search_mode: PeriodLockSearchMode,
    atoms: &[PeriodLockAtomSpec],
    max_rule_atoms: usize,
) -> Vec<PeriodLockRuleCandidate> {
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
    problem: &PeriodLockSearchProblem<'_>,
    search_mode: PeriodLockSearchMode,
    atom_indices: &[usize],
    atoms: &[PeriodLockAtomSpec],
    dataset_rows: usize,
    positive_rows: usize,
) -> PeriodLockRuleCandidate {
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
    for (actual, predicted) in problem.target.iter().zip(predicted.iter()) {
        match (*actual, *predicted) {
            (true, true) => true_positive += 1,
            (false, true) => false_positive += 1,
            (true, false) => false_negative += 1,
            (false, false) => true_negative += 1,
        }
    }
    let total_errors = false_positive + false_negative;
    let precision = if true_positive + false_positive == 0 {
        0.0
    } else {
        true_positive as f64 / (true_positive + false_positive) as f64
    };
    let recall = if positive_rows == 0 {
        0.0
    } else {
        true_positive as f64 / positive_rows as f64
    };
    let f1 = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    };
    let complexity_score = atom_indices
        .iter()
        .map(|&index| atoms[index].complexity_score)
        .sum();
    let threshold_free = atom_indices
        .iter()
        .all(|&index| !atoms[index].threshold_based);
    let interpretability_rank = atom_indices
        .iter()
        .map(|&index| atoms[index].interpretability_rank)
        .max()
        .unwrap_or(0);

    PeriodLockRuleCandidate {
        search_id: problem.id.to_string(),
        search_label: problem.label.to_string(),
        search_mode: search_mode.as_str().to_string(),
        atom_count: atom_indices.len(),
        rule_label: atom_labels.join(" AND "),
        exact_match: total_errors == 0,
        total_errors,
        true_positive,
        false_positive,
        false_negative,
        true_negative,
        precision,
        recall,
        f1,
        positive_support: true_positive,
        complexity_score,
        threshold_free,
        interpretability_rank,
        atom_labels,
    }
}

fn format_threshold_value(value: f64) -> String {
    let scaled = (value * 1_000_000.0).round() / 1_000_000.0;
    let text = format!("{scaled:.6}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::affine_period_lock::scan_k_config_affine_period_lock_comparison;
    use crate::validation::bounded_k::ordered_unit_pairs;

    #[test]
    fn widened_main_plus_base30_surface_stays_period_lock_exact() {
        for &base in &[10, 14, 22, 26, 30] {
            for middle_length in [1usize, 2usize] {
                for (outer, inner) in ordered_unit_pairs(base) {
                    for to_k in noncompact_lanes() {
                        let comparison = scan_k_config_affine_period_lock_comparison(
                            base,
                            middle_length,
                            outer,
                            inner,
                            K00,
                            to_k,
                        );
                        assert!(
                            comparison.period_lock_perfect,
                            "period-lock mismatch at base={base}, M={middle_length}, pair=({outer},{inner}), to_k={to_k:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn phase_partitions_hold_for_period_lock_species_rows() {
        let row = analyze_period_lock_species_feature_row(22, 17, 19);
        assert_eq!(
            row.m2_locked_count + row.m2_unlocked_count,
            row.m2_compared_moduli_count
        );
        assert_eq!(
            row.m2_locked_identity_count + row.m2_locked_gradient_only_count,
            row.m2_locked_count
        );
        assert_eq!(
            row.m2_unlocked_shift_only_count + row.m2_unlocked_shift_and_gradient_count,
            row.m2_unlocked_count
        );
    }

    #[test]
    fn base22_k22_mod5_remains_canonical_higher_order_locked_gradient_only_pocket() {
        let rows = scan_locked_shift_residuals(22, 2, 17, 19, K00, (2, 2));
        let mod5 = rows.iter().find(|row| row.modulus == 5).expect("mod-5 row");
        assert_eq!(mod5.multiplicative_order, 4);
        assert_eq!(mod5.order_bucket, "ord_ge_3");
        assert_eq!(mod5.locked_relation_label, "gradient_only");
    }

    #[test]
    fn base22_k11_mod5_remains_unlocked() {
        let rows = scan_period_lock_order_cells(22, 2, 17, 19, K00, (1, 1));
        let mod5 = rows.iter().find(|row| row.modulus == 5).expect("mod-5 row");
        assert!(!mod5.locked);
    }
}
