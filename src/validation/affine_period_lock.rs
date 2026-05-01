//! Exact period-lock vocabulary for affine gradient agreement.
//!
//! The affine core is `N(s) = A + G*s`: after the base, boundary digits,
//! middle length, and `k` lane are fixed, the seed `s` is the only moving part.
//! Modulo a coprime prime `p`, this becomes `N(s) = A_p + G_p*s`, so each lane
//! has exact local shift / gradient / zero-seed data before any primality
//! testing happens.
//!
//! For bounded-`k` lanes the local affine gradient is a power of the base:
//! `base ^ middle_position (mod p)`, where
//! `middle_position = k_outer + k_inner + 2`. Gradient equality between two
//! lanes is therefore expected exactly when the difference in those positions
//! is `0` modulo the multiplicative order of the base in `(Z/pZ)^×`.
//!
//! This module packages that comparison as a reusable exact validation surface
//! so exploratory reports can ask whether observed gradient agreement is a
//! genuine period-lock phenomenon or just an empirical coincidence. It does not
//! claim that period lock is enough for primality or for a density theorem.

use crate::{
    hzlib::num_theory::multiplicative_order,
    validation::bounded_k::{
        digit_symbol, format_k, scan_k_config_affine_lane_comparison, BoundedKConfig,
        KConfigAffineLaneComparison, KConfigAffineLaneComparisonModulusRow,
    },
};
use serde::Serialize;

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AffinePeriodLockModulusRow {
    pub modulus: u32,
    pub base_modulus: u32,
    pub gradient_position_from: u32,
    pub gradient_position_to: u32,
    pub gradient_position_delta: i32,
    pub multiplicative_order: u32,
    pub delta_mod_order: u32,
    pub period_lock_expected: bool,
    pub observed_gradient_equal: bool,
    pub expected_matches_observation: bool,
    pub shift_equal: bool,
    pub zero_seed_equal: bool,
    pub local_relation_label: String,
    pub shift_modulus_from: u32,
    pub shift_modulus_to: u32,
    pub gradient_modulus_from: u32,
    pub gradient_modulus_to: u32,
    pub zero_seed_class_from: u32,
    pub zero_seed_class_to: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffinePeriodLockComparison {
    pub base: u32,
    pub middle_length: usize,
    pub outer: u32,
    pub inner: u32,
    pub pair_label: String,
    pub from_k: String,
    pub to_k: String,
    pub compared_moduli_count: usize,
    pub gradient_position_from: u32,
    pub gradient_position_to: u32,
    pub gradient_position_delta: i32,
    pub period_lock_expected_count: usize,
    pub observed_gradient_equal_count: usize,
    pub period_lock_match_count: usize,
    pub period_lock_mismatch_count: usize,
    pub period_lock_expected_share: f64,
    pub observed_gradient_equal_share: f64,
    pub period_lock_match_share: f64,
    pub period_lock_perfect: bool,
    pub same_shift_count: usize,
    pub same_zero_seed_count: usize,
    pub identity_count: usize,
    pub shift_only_count: usize,
    pub gradient_only_count: usize,
    pub shift_and_gradient_count: usize,
    pub period_locked_identity_count: usize,
    pub period_locked_gradient_only_count: usize,
    pub affine_comparison: KConfigAffineLaneComparison,
    pub modulus_rows: Vec<AffinePeriodLockModulusRow>,
}

pub fn bounded_k_gradient_position((k_outer, k_inner): BoundedKConfig) -> u32 {
    k_outer + k_inner + 2
}

pub fn bounded_k_gradient_position_delta(from_k: BoundedKConfig, to_k: BoundedKConfig) -> i32 {
    bounded_k_gradient_position(to_k) as i32 - bounded_k_gradient_position(from_k) as i32
}

fn delta_mod_order(delta: i32, order: u32) -> u32 {
    debug_assert!(order > 0);
    delta.rem_euclid(order as i32) as u32
}

fn build_period_lock_row(
    base: u32,
    gradient_position_from: u32,
    gradient_position_to: u32,
    gradient_position_delta: i32,
    row: &KConfigAffineLaneComparisonModulusRow,
) -> AffinePeriodLockModulusRow {
    let multiplicative_order = multiplicative_order(base as u64, row.modulus as u64) as u32;
    let delta_mod_order = if multiplicative_order == 0 {
        0
    } else {
        delta_mod_order(gradient_position_delta, multiplicative_order)
    };
    let period_lock_expected = multiplicative_order > 0 && delta_mod_order == 0;
    let observed_gradient_equal = row.gradient_equal;

    AffinePeriodLockModulusRow {
        modulus: row.modulus,
        base_modulus: base % row.modulus,
        gradient_position_from,
        gradient_position_to,
        gradient_position_delta,
        multiplicative_order,
        delta_mod_order,
        period_lock_expected,
        observed_gradient_equal,
        expected_matches_observation: period_lock_expected == observed_gradient_equal,
        shift_equal: row.shift_equal,
        zero_seed_equal: row.zero_seed_equal,
        local_relation_label: row.local_relation_label.clone(),
        shift_modulus_from: row.shift_modulus_from,
        shift_modulus_to: row.shift_modulus_to,
        gradient_modulus_from: row.gradient_modulus_from,
        gradient_modulus_to: row.gradient_modulus_to,
        zero_seed_class_from: row.zero_seed_class_from,
        zero_seed_class_to: row.zero_seed_class_to,
    }
}

pub fn scan_k_config_affine_period_lock_comparison(
    base: u32,
    middle_length: usize,
    outer: u32,
    inner: u32,
    from_k: BoundedKConfig,
    to_k: BoundedKConfig,
) -> AffinePeriodLockComparison {
    let affine_comparison =
        scan_k_config_affine_lane_comparison(base, middle_length, outer, inner, from_k, to_k);
    let gradient_position_from = bounded_k_gradient_position(from_k);
    let gradient_position_to = bounded_k_gradient_position(to_k);
    let gradient_position_delta = bounded_k_gradient_position_delta(from_k, to_k);
    let modulus_rows = affine_comparison
        .modulus_rows
        .iter()
        .map(|row| {
            build_period_lock_row(
                base,
                gradient_position_from,
                gradient_position_to,
                gradient_position_delta,
                row,
            )
        })
        .collect::<Vec<_>>();

    let compared_moduli_count = modulus_rows.len();
    let period_lock_expected_count = modulus_rows
        .iter()
        .filter(|row| row.period_lock_expected)
        .count();
    let observed_gradient_equal_count = modulus_rows
        .iter()
        .filter(|row| row.observed_gradient_equal)
        .count();
    let period_lock_match_count = modulus_rows
        .iter()
        .filter(|row| row.expected_matches_observation)
        .count();
    let period_lock_mismatch_count = compared_moduli_count - period_lock_match_count;
    let period_locked_identity_count = modulus_rows
        .iter()
        .filter(|row| row.period_lock_expected && row.shift_equal)
        .count();
    let period_locked_gradient_only_count = modulus_rows
        .iter()
        .filter(|row| row.period_lock_expected && !row.shift_equal)
        .count();

    AffinePeriodLockComparison {
        base,
        middle_length,
        outer,
        inner,
        pair_label: format!("({},{})", digit_symbol(outer), digit_symbol(inner)),
        from_k: format_k(from_k),
        to_k: format_k(to_k),
        compared_moduli_count,
        gradient_position_from,
        gradient_position_to,
        gradient_position_delta,
        period_lock_expected_count,
        observed_gradient_equal_count,
        period_lock_match_count,
        period_lock_mismatch_count,
        period_lock_expected_share: ratio(period_lock_expected_count, compared_moduli_count),
        observed_gradient_equal_share: ratio(observed_gradient_equal_count, compared_moduli_count),
        period_lock_match_share: ratio(period_lock_match_count, compared_moduli_count),
        period_lock_perfect: period_lock_mismatch_count == 0,
        same_shift_count: affine_comparison.same_shift_count,
        same_zero_seed_count: affine_comparison.same_zero_seed_count,
        identity_count: affine_comparison.identity_count,
        shift_only_count: affine_comparison.shift_only_count,
        gradient_only_count: affine_comparison.gradient_only_count,
        shift_and_gradient_count: affine_comparison.shift_and_gradient_count,
        period_locked_identity_count,
        period_locked_gradient_only_count,
        affine_comparison,
        modulus_rows,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{is_prime, validation::bounded_k::scan_k_config_examples};
    use num_bigint::BigUint;
    use std::str::FromStr;

    #[test]
    fn gradient_positions_match_lane_grid() {
        assert_eq!(bounded_k_gradient_position((0, 0)), 2);
        assert_eq!(bounded_k_gradient_position((0, 1)), 3);
        assert_eq!(bounded_k_gradient_position((1, 0)), 3);
        assert_eq!(bounded_k_gradient_position((1, 1)), 4);
        assert_eq!(bounded_k_gradient_position((2, 2)), 6);
        assert_eq!(bounded_k_gradient_position_delta((0, 0), (2, 2)), 4);
    }

    #[test]
    fn period_lock_explains_base22_active_mod5_pocket() {
        let comparison = scan_k_config_affine_period_lock_comparison(22, 2, 17, 19, (0, 0), (2, 2));
        let mod5 = comparison
            .modulus_rows
            .iter()
            .find(|row| row.modulus == 5)
            .expect("mod-5 row should exist");
        assert_eq!(mod5.multiplicative_order, 4);
        assert_eq!(mod5.gradient_position_delta, 4);
        assert_eq!(mod5.delta_mod_order, 0);
        assert!(mod5.period_lock_expected);
        assert!(mod5.observed_gradient_equal);
        assert!(!mod5.shift_equal);
        assert!(!mod5.zero_seed_equal);
        assert_eq!(mod5.shift_modulus_from, 3);
        assert_eq!(mod5.shift_modulus_to, 4);
        assert_eq!(mod5.gradient_modulus_from, 4);
        assert_eq!(mod5.gradient_modulus_to, 4);
        assert_eq!(mod5.zero_seed_class_from, 3);
        assert_eq!(mod5.zero_seed_class_to, 4);
        assert_eq!(mod5.local_relation_label, "gradient_only");
        assert!(mod5.expected_matches_observation);
    }

    #[test]
    fn base22_canonical_report_witnesses_are_prime() {
        let (_, compact_examples) = scan_k_config_examples(22, 2, 17, 19, (0, 0), 1);
        let compact = compact_examples
            .first()
            .expect("compact base-22 witness should exist");
        assert_eq!(compact.middle_index, 10);
        assert_eq!(compact.middle_digits, "0A");
        assert_eq!(compact.decimal_value, "92067883");
        assert!(is_prime(
            &BigUint::from_str(&compact.decimal_value).unwrap()
        ));

        let (_, side_examples) = scan_k_config_examples(22, 2, 17, 19, (2, 2), 1);
        let side = side_examples
            .first()
            .expect("side-pocket base-22 witness should exist");
        assert_eq!(side.middle_index, 13);
        assert_eq!(side.middle_digits, "0D");
        assert_eq!(side.decimal_value, "4808275624019584921");
        assert!(is_prime(&BigUint::from_str(&side.decimal_value).unwrap()));

        for (base, middle_length, outer, inner, k, seed, digits, decimal) in [
            (10, 2, 3, 7, (2, 1), 20, "20", "300702007003"),
            (10, 2, 3, 7, (1, 1), 5, "05", "3070050703"),
            (10, 2, 1, 7, (2, 2), 4, "04", "10070004007001"),
            (10, 3, 3, 1, (2, 2), 30, "030", "300100030001003"),
            (10, 2, 3, 3, (1, 0), 1, "01", "30301303"),
            (14, 2, 13, 11, (0, 1), 22, "18", "1453260983"),
            (6, 1, 1, 5, (0, 0), 4, "4", "2551"),
        ] {
            let (_, examples) = scan_k_config_examples(base, middle_length, outer, inner, k, 1);
            let witness = examples.first().expect("gallery witness should exist");
            assert_eq!(witness.middle_index, seed);
            assert_eq!(witness.middle_digits, digits);
            assert_eq!(witness.decimal_value, decimal);
            assert!(is_prime(
                &BigUint::from_str(&witness.decimal_value).unwrap()
            ));
        }
    }

    #[test]
    fn period_lock_explains_base22_collapsed_mod5_column() {
        let comparison = scan_k_config_affine_period_lock_comparison(22, 2, 17, 15, (0, 0), (2, 2));
        let mod5 = comparison
            .modulus_rows
            .iter()
            .find(|row| row.modulus == 5)
            .expect("mod-5 row should exist");
        assert_eq!(mod5.multiplicative_order, 4);
        assert_eq!(mod5.gradient_position_delta, 4);
        assert!(mod5.period_lock_expected);
        assert!(mod5.observed_gradient_equal);
        assert!(mod5.shift_equal);
        assert_eq!(mod5.local_relation_label, "identity");
        assert!(mod5.expected_matches_observation);
    }
}
