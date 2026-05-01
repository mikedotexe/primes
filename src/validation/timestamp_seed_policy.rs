//! Bounded empirical policy for timestamp-origin seed-to-witness searches.
//!
//! This module turns the casual claim "a timestamp seed usually finds a nearby
//! witness" into a bounded, reproducible measurement surface.

use crate::validation::seed_to_witness::{
    find_seed_to_witness, SeedToWitnessConfig, SeedToWitnessError,
};
use serde::Serialize;

pub const DEFAULT_TIMESTAMP_ANCHOR_SEED: u64 = 1_777_651_200_000_000_000;
pub const DEFAULT_TIMESTAMP_STRIDE: u64 = 1_000_003;

#[derive(Debug, Clone, Serialize)]
pub struct TimestampPolicySettings {
    pub policy_label: String,
    pub anchor_seed: u64,
    pub sample_count: usize,
    pub stride: u64,
    pub visible_digits: usize,
    pub max_steps: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimestampPolicySummaryRow {
    pub policy_label: String,
    pub visible_digits: usize,
    pub max_steps: u64,
    pub sample_count: usize,
    pub successes: usize,
    pub failures: usize,
    pub success_rate: f64,
    pub min_steps: u64,
    pub median_steps: u64,
    pub p90_steps: u64,
    pub p95_steps: u64,
    pub p99_steps: u64,
    pub max_steps_observed: u64,
    pub mean_steps_to_witness: f64,
    pub mean_probable_prime_tests: f64,
    pub mean_residue_survivors: f64,
    pub mean_elapsed_ms: f64,
    pub bounded_statement: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimestampPolicyTrialRow {
    pub policy_label: String,
    pub trial_index: usize,
    pub input_seed: u64,
    pub status: String,
    pub witness_seed: Option<u64>,
    pub steps_to_witness: Option<u64>,
    pub scanned_seed_count: u64,
    pub residue_survivor_count: u64,
    pub probable_prime_tests: u64,
    pub elapsed_seconds: f64,
    pub confirmation: String,
    pub is_mersenne: bool,
    pub mersenne_exponent: Option<u64>,
    pub mersenne_class: String,
    pub decimal_digits: Option<usize>,
    pub decimal_value: String,
    pub failure_reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimestampSeedPolicyReportData {
    pub settings: Vec<TimestampPolicySettings>,
    pub policy_rows: Vec<TimestampPolicySummaryRow>,
    pub trial_rows: Vec<TimestampPolicyTrialRow>,
}

pub fn default_timestamp_policy_settings(profile: &str) -> Vec<TimestampPolicySettings> {
    match profile {
        "smoke" => vec![
            TimestampPolicySettings {
                policy_label: "timestamp_full_middle_29d_512_steps".to_string(),
                anchor_seed: DEFAULT_TIMESTAMP_ANCHOR_SEED,
                sample_count: 32,
                stride: DEFAULT_TIMESTAMP_STRIDE,
                visible_digits: 29,
                max_steps: 512,
            },
            TimestampPolicySettings {
                policy_label: "timestamp_large_128d_20000_steps".to_string(),
                anchor_seed: DEFAULT_TIMESTAMP_ANCHOR_SEED,
                sample_count: 8,
                stride: DEFAULT_TIMESTAMP_STRIDE,
                visible_digits: 128,
                max_steps: 20_000,
            },
        ],
        _ => vec![
            TimestampPolicySettings {
                policy_label: "timestamp_full_middle_29d_512_steps".to_string(),
                anchor_seed: DEFAULT_TIMESTAMP_ANCHOR_SEED,
                sample_count: 256,
                stride: DEFAULT_TIMESTAMP_STRIDE,
                visible_digits: 29,
                max_steps: 512,
            },
            TimestampPolicySettings {
                policy_label: "timestamp_large_128d_20000_steps".to_string(),
                anchor_seed: DEFAULT_TIMESTAMP_ANCHOR_SEED,
                sample_count: 64,
                stride: DEFAULT_TIMESTAMP_STRIDE,
                visible_digits: 128,
                max_steps: 20_000,
            },
        ],
    }
}

pub fn build_timestamp_seed_policy_report(
    settings: Vec<TimestampPolicySettings>,
) -> TimestampSeedPolicyReportData {
    let mut policy_rows = Vec::new();
    let mut trial_rows = Vec::new();
    for policy in &settings {
        let rows = measure_policy(policy);
        policy_rows.push(summarize_policy(policy, &rows));
        trial_rows.extend(rows);
    }
    TimestampSeedPolicyReportData {
        settings,
        policy_rows,
        trial_rows,
    }
}

fn measure_policy(policy: &TimestampPolicySettings) -> Vec<TimestampPolicyTrialRow> {
    (0..policy.sample_count)
        .map(|idx| {
            let input_seed = policy.anchor_seed.checked_add(idx as u64 * policy.stride);
            match input_seed {
                Some(seed) => run_trial(policy, idx, seed),
                None => TimestampPolicyTrialRow {
                    policy_label: policy.policy_label.clone(),
                    trial_index: idx,
                    input_seed: policy.anchor_seed,
                    status: "failure".to_string(),
                    witness_seed: None,
                    steps_to_witness: None,
                    scanned_seed_count: 0,
                    residue_survivor_count: 0,
                    probable_prime_tests: 0,
                    elapsed_seconds: 0.0,
                    confirmation: String::new(),
                    is_mersenne: false,
                    mersenne_exponent: None,
                    mersenne_class: String::new(),
                    decimal_digits: None,
                    decimal_value: String::new(),
                    failure_reason: "seed schedule overflowed u64".to_string(),
                },
            }
        })
        .collect()
}

fn run_trial(
    policy: &TimestampPolicySettings,
    trial_index: usize,
    input_seed: u64,
) -> TimestampPolicyTrialRow {
    let config = SeedToWitnessConfig::default_for_seed(input_seed)
        .with_visible_digits(policy.visible_digits)
        .with_max_steps(policy.max_steps);
    match find_seed_to_witness(config) {
        Ok(result) => TimestampPolicyTrialRow {
            policy_label: policy.policy_label.clone(),
            trial_index,
            input_seed,
            status: "success".to_string(),
            witness_seed: Some(result.witness_seed),
            steps_to_witness: Some(result.steps_to_witness),
            scanned_seed_count: result.scanned_seed_count,
            residue_survivor_count: result.residue_survivor_count,
            probable_prime_tests: result.probable_prime_tests,
            elapsed_seconds: result.elapsed_seconds,
            confirmation: result.confirmation,
            is_mersenne: result.is_mersenne,
            mersenne_exponent: result.mersenne_exponent,
            mersenne_class: result.mersenne_class,
            decimal_digits: Some(result.decimal_digits),
            decimal_value: result.decimal_value,
            failure_reason: String::new(),
        },
        Err(err) => failure_row(policy, trial_index, input_seed, err),
    }
}

fn failure_row(
    policy: &TimestampPolicySettings,
    trial_index: usize,
    input_seed: u64,
    err: SeedToWitnessError,
) -> TimestampPolicyTrialRow {
    let (scanned_seed_count, residue_survivor_count) = match &err {
        SeedToWitnessError::NoWitnessFound {
            scanned_seed_count,
            residue_survivor_count,
            ..
        } => (*scanned_seed_count, *residue_survivor_count),
        _ => (0, 0),
    };
    TimestampPolicyTrialRow {
        policy_label: policy.policy_label.clone(),
        trial_index,
        input_seed,
        status: "failure".to_string(),
        witness_seed: None,
        steps_to_witness: None,
        scanned_seed_count,
        residue_survivor_count,
        probable_prime_tests: residue_survivor_count,
        elapsed_seconds: 0.0,
        confirmation: String::new(),
        is_mersenne: false,
        mersenne_exponent: None,
        mersenne_class: String::new(),
        decimal_digits: None,
        decimal_value: String::new(),
        failure_reason: err.to_string(),
    }
}

fn summarize_policy(
    policy: &TimestampPolicySettings,
    rows: &[TimestampPolicyTrialRow],
) -> TimestampPolicySummaryRow {
    let successes = rows
        .iter()
        .filter(|row| row.status == "success")
        .collect::<Vec<_>>();
    let failures = rows.len().saturating_sub(successes.len());
    let mut steps = successes
        .iter()
        .filter_map(|row| row.steps_to_witness)
        .collect::<Vec<_>>();
    steps.sort_unstable();
    let success_rate = ratio(successes.len() as u64, rows.len() as u64);
    let mean_steps_to_witness = mean_u64(&steps);
    let mean_probable_prime_tests = mean_u64(
        &successes
            .iter()
            .map(|row| row.probable_prime_tests)
            .collect::<Vec<_>>(),
    );
    let mean_residue_survivors = mean_u64(
        &successes
            .iter()
            .map(|row| row.residue_survivor_count)
            .collect::<Vec<_>>(),
    );
    let mean_elapsed_ms = if successes.is_empty() {
        0.0
    } else {
        successes
            .iter()
            .map(|row| row.elapsed_seconds * 1000.0)
            .sum::<f64>()
            / successes.len() as f64
    };
    let bounded_statement = if failures == 0 {
        format!(
            "{}/{} timestamp-like seed origins found a witness within {} steps",
            successes.len(),
            rows.len(),
            policy.max_steps
        )
    } else {
        format!(
            "{}/{} timestamp-like seed origins found a witness within {} steps; {} missed the bound",
            successes.len(),
            rows.len(),
            policy.max_steps,
            failures
        )
    };
    TimestampPolicySummaryRow {
        policy_label: policy.policy_label.clone(),
        visible_digits: policy.visible_digits,
        max_steps: policy.max_steps,
        sample_count: rows.len(),
        successes: successes.len(),
        failures,
        success_rate,
        min_steps: steps.first().copied().unwrap_or_default(),
        median_steps: quantile_nearest_rank(&steps, 0.50),
        p90_steps: quantile_nearest_rank(&steps, 0.90),
        p95_steps: quantile_nearest_rank(&steps, 0.95),
        p99_steps: quantile_nearest_rank(&steps, 0.99),
        max_steps_observed: steps.last().copied().unwrap_or_default(),
        mean_steps_to_witness,
        mean_probable_prime_tests,
        mean_residue_survivors,
        mean_elapsed_ms,
        bounded_statement,
    }
}

fn quantile_nearest_rank(sorted: &[u64], q: f64) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    let clamped = q.clamp(0.0, 1.0);
    let rank = (clamped * sorted.len() as f64).ceil() as usize;
    sorted[rank.saturating_sub(1).min(sorted.len() - 1)]
}

fn mean_u64(values: &[u64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<u64>() as f64 / values.len() as f64
    }
}

fn ratio(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_policy_rows_are_bounded_and_successful() {
        let settings = vec![TimestampPolicySettings {
            policy_label: "test_timestamp_29d".to_string(),
            anchor_seed: DEFAULT_TIMESTAMP_ANCHOR_SEED,
            sample_count: 4,
            stride: DEFAULT_TIMESTAMP_STRIDE,
            visible_digits: 29,
            max_steps: 512,
        }];
        let report = build_timestamp_seed_policy_report(settings);
        assert_eq!(report.trial_rows.len(), 4);
        assert_eq!(report.policy_rows[0].successes, 4);
        assert_eq!(report.policy_rows[0].failures, 0);
        assert!(report.policy_rows[0].max_steps_observed <= 512);
        assert!(report
            .trial_rows
            .iter()
            .all(|row| row.decimal_digits == Some(29)));
        assert!(report.trial_rows.iter().all(|row| !row.is_mersenne));
        assert!(report
            .trial_rows
            .iter()
            .all(|row| row.mersenne_class == "not_mersenne"));
    }

    #[test]
    fn quantile_rows_are_monotone() {
        let settings = vec![TimestampPolicySettings {
            policy_label: "test_timestamp_128d".to_string(),
            anchor_seed: DEFAULT_TIMESTAMP_ANCHOR_SEED,
            sample_count: 3,
            stride: DEFAULT_TIMESTAMP_STRIDE,
            visible_digits: 128,
            max_steps: 20_000,
        }];
        let report = build_timestamp_seed_policy_report(settings);
        let row = &report.policy_rows[0];
        assert_eq!(row.successes, 3);
        assert!(row.median_steps <= row.p90_steps);
        assert!(row.p90_steps <= row.p95_steps);
        assert!(row.p95_steps <= row.p99_steps);
    }
}
