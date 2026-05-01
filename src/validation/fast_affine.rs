//! Fast deterministic prime generation for maintained affine membrane lanes.
//!
//! This module is deliberately narrower than the older optimization-sketch
//! binaries. It uses the maintained bounded-`k` digit grammar, precomputes the
//! affine line `N(s) = shift + gradient * s`, skips exact small-prime residue
//! obstructions with a combined seed wheel, and only claims deterministic
//! throughput for candidates that fit in `u64`.

use crate::validation::bounded_k::{
    digit_symbol, format_k, BoundedKConfig, DEFAULT_PREFILTER_PRIMES,
};
use serde::Serialize;
use std::time::Instant;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum FastPrimeError {
    #[error("base must be at least 2, got {0}")]
    InvalidBase(u32),
    #[error("digit {digit} is not valid for base {base}")]
    InvalidDigit { digit: u32, base: u32 },
    #[error("middle length {0} is too large for deterministic u64 scanning")]
    MiddleLengthTooLarge(usize),
    #[error("affine lane does not fit in u64 for the requested finite seed space")]
    LaneExceedsU64,
    #[error("wheel period cap must be at least 1")]
    InvalidWheelPeriodCap,
}

#[derive(Debug, Clone, Serialize)]
pub struct FastLaneConfig {
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub middle_length: usize,
    pub k_outer: u32,
    pub k_inner: u32,
}

impl FastLaneConfig {
    pub fn new(
        base: u32,
        outer: u32,
        inner: u32,
        middle_length: usize,
        (k_outer, k_inner): BoundedKConfig,
    ) -> Self {
        Self {
            base,
            outer,
            inner,
            middle_length,
            k_outer,
            k_inner,
        }
    }

    pub fn k(&self) -> BoundedKConfig {
        (self.k_outer, self.k_inner)
    }

    pub fn k_label(&self) -> String {
        format_k(self.k())
    }

    pub fn pair_label(&self) -> String {
        format!(
            "({},{})",
            digit_symbol(self.outer),
            digit_symbol(self.inner)
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FastAffineLane {
    pub config: FastLaneConfig,
    pub shift: u64,
    pub gradient: u64,
    pub suffix_len: u32,
    pub seed_capacity: u64,
    pub max_candidate: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResidueWheel {
    pub period_cap: u64,
    pub period: u64,
    pub moduli: Vec<u32>,
    pub admissible_residues: Vec<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FastPrimeWitness {
    pub seed: u64,
    pub middle_digits: String,
    pub template_digits: String,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FastPrimeRun {
    pub config: FastLaneConfig,
    pub pair_label: String,
    pub k_label: String,
    pub shift: u64,
    pub gradient: u64,
    pub suffix_len: u32,
    pub requested_seed_count: u64,
    pub seed_capacity: u64,
    pub scanned_seed_count: u64,
    pub capped_to_seed_capacity: bool,
    pub wheel_period_cap: u64,
    pub wheel_period: u64,
    pub wheel_moduli: Vec<u32>,
    pub wheel_admissible_residue_count: usize,
    pub admissible_seed_count: u64,
    pub primality_tests: u64,
    pub primes_found: u64,
    pub elapsed_seconds: f64,
    pub seeds_per_second: f64,
    pub admissible_per_second: f64,
    pub primality_tests_per_second: f64,
    pub primes_per_second: f64,
    pub witnesses: Vec<FastPrimeWitness>,
}

pub fn build_fast_affine_lane(config: FastLaneConfig) -> Result<FastAffineLane, FastPrimeError> {
    validate_config(&config)?;

    let prefix_digits = {
        let mut digits = Vec::with_capacity((2 + config.k_outer + config.k_inner) as usize);
        digits.push(config.outer);
        digits.extend(std::iter::repeat_n(0, config.k_outer as usize));
        digits.push(config.inner);
        digits.extend(std::iter::repeat_n(0, config.k_inner as usize));
        digits
    };
    let suffix_digits = {
        let mut digits = Vec::with_capacity((2 + config.k_outer + config.k_inner) as usize);
        digits.extend(std::iter::repeat_n(0, config.k_inner as usize));
        digits.push(config.inner);
        digits.extend(std::iter::repeat_n(0, config.k_outer as usize));
        digits.push(config.outer);
        digits
    };
    let suffix_len = suffix_digits.len() as u32;
    let seed_capacity = checked_pow_u64(config.base, config.middle_length)
        .ok_or(FastPrimeError::MiddleLengthTooLarge(config.middle_length))?;
    let gradient =
        checked_pow_u64(config.base, suffix_len as usize).ok_or(FastPrimeError::LaneExceedsU64)?;
    let prefix_shift = checked_pow_u128(config.base, config.middle_length + suffix_len as usize)
        .ok_or(FastPrimeError::LaneExceedsU64)?;
    let prefix_value = digits_to_u128(config.base, &prefix_digits)?;
    let suffix_value = digits_to_u128(config.base, &suffix_digits)?;
    let shift = prefix_value
        .checked_mul(prefix_shift)
        .and_then(|value| value.checked_add(suffix_value))
        .ok_or(FastPrimeError::LaneExceedsU64)?;
    let max_seed = seed_capacity.saturating_sub(1) as u128;
    let max_candidate = shift
        .checked_add((gradient as u128).saturating_mul(max_seed))
        .ok_or(FastPrimeError::LaneExceedsU64)?;

    Ok(FastAffineLane {
        config,
        shift: u64::try_from(shift).map_err(|_| FastPrimeError::LaneExceedsU64)?,
        gradient,
        suffix_len,
        seed_capacity,
        max_candidate: u64::try_from(max_candidate).map_err(|_| FastPrimeError::LaneExceedsU64)?,
    })
}

pub fn build_residue_wheel(
    lane: &FastAffineLane,
    period_cap: u64,
) -> Result<ResidueWheel, FastPrimeError> {
    if period_cap == 0 {
        return Err(FastPrimeError::InvalidWheelPeriodCap);
    }

    let mut period = 1u64;
    let mut moduli = Vec::new();
    for &modulus in DEFAULT_PREFILTER_PRIMES {
        if gcd_u32(lane.config.base, modulus) != 1 {
            continue;
        }
        let next_period = period.saturating_mul(modulus as u64);
        if next_period > period_cap {
            continue;
        }
        period = next_period;
        moduli.push(modulus);
    }

    let mut admissible_residues = Vec::new();
    for residue in 0..period {
        if is_seed_admissible(lane, residue, &moduli) {
            admissible_residues.push(residue);
        }
    }

    Ok(ResidueWheel {
        period_cap,
        period,
        moduli,
        admissible_residues,
    })
}

pub fn scan_fast_prime_lane(
    config: FastLaneConfig,
    requested_seed_count: u64,
    max_witnesses: usize,
    wheel_period_cap: u64,
) -> Result<FastPrimeRun, FastPrimeError> {
    let lane = build_fast_affine_lane(config)?;
    let wheel = build_residue_wheel(&lane, wheel_period_cap)?;
    let scanned_seed_count = requested_seed_count.min(lane.seed_capacity);
    let capped_to_seed_capacity = scanned_seed_count < requested_seed_count;
    let start = Instant::now();

    let mut admissible_seed_count = 0u64;
    let mut primes_found = 0u64;
    let mut witnesses = Vec::new();

    let period = wheel.period;
    let mut cycle_start = 0u64;
    while cycle_start < scanned_seed_count {
        for &residue in &wheel.admissible_residues {
            let Some(seed) = cycle_start.checked_add(residue) else {
                continue;
            };
            if seed >= scanned_seed_count {
                break;
            }
            admissible_seed_count += 1;
            let Some(value) = lane.candidate_value(seed) else {
                continue;
            };
            if primal::is_prime(value) {
                primes_found += 1;
                if witnesses.len() < max_witnesses {
                    witnesses.push(FastPrimeWitness {
                        seed,
                        middle_digits: lane.middle_digits(seed),
                        template_digits: lane.template_digits(seed),
                        value,
                    });
                }
            }
        }
        cycle_start = match cycle_start.checked_add(period) {
            Some(next) => next,
            None => break,
        };
    }

    let elapsed_seconds = start.elapsed().as_secs_f64().max(1e-12);
    Ok(FastPrimeRun {
        config: lane.config.clone(),
        pair_label: lane.config.pair_label(),
        k_label: lane.config.k_label(),
        shift: lane.shift,
        gradient: lane.gradient,
        suffix_len: lane.suffix_len,
        requested_seed_count,
        seed_capacity: lane.seed_capacity,
        scanned_seed_count,
        capped_to_seed_capacity,
        wheel_period_cap,
        wheel_period: wheel.period,
        wheel_moduli: wheel.moduli,
        wheel_admissible_residue_count: wheel.admissible_residues.len(),
        admissible_seed_count,
        primality_tests: admissible_seed_count,
        primes_found,
        elapsed_seconds,
        seeds_per_second: scanned_seed_count as f64 / elapsed_seconds,
        admissible_per_second: admissible_seed_count as f64 / elapsed_seconds,
        primality_tests_per_second: admissible_seed_count as f64 / elapsed_seconds,
        primes_per_second: primes_found as f64 / elapsed_seconds,
        witnesses,
    })
}

impl FastAffineLane {
    pub fn candidate_value(&self, seed: u64) -> Option<u64> {
        if seed >= self.seed_capacity {
            return None;
        }
        (self.shift as u128)
            .checked_add((self.gradient as u128).checked_mul(seed as u128)?)?
            .try_into()
            .ok()
    }

    pub fn middle_digits(&self, mut seed: u64) -> String {
        let mut digits = vec!['0'; self.config.middle_length];
        for digit in digits.iter_mut().rev() {
            let value = seed % self.config.base as u64;
            *digit = digit_char(value as u32);
            seed /= self.config.base as u64;
        }
        digits.into_iter().collect()
    }

    pub fn template_digits(&self, seed: u64) -> String {
        let mut digits = String::new();
        digits.push_str(&digit_symbol(self.config.outer));
        digits.extend(std::iter::repeat_n('0', self.config.k_outer as usize));
        digits.push_str(&digit_symbol(self.config.inner));
        digits.extend(std::iter::repeat_n('0', self.config.k_inner as usize));
        digits.push_str(&self.middle_digits(seed));
        digits.extend(std::iter::repeat_n('0', self.config.k_inner as usize));
        digits.push_str(&digit_symbol(self.config.inner));
        digits.extend(std::iter::repeat_n('0', self.config.k_outer as usize));
        digits.push_str(&digit_symbol(self.config.outer));
        digits
    }
}

fn validate_config(config: &FastLaneConfig) -> Result<(), FastPrimeError> {
    if config.base < 2 {
        return Err(FastPrimeError::InvalidBase(config.base));
    }
    for digit in [config.outer, config.inner] {
        if digit >= config.base {
            return Err(FastPrimeError::InvalidDigit {
                digit,
                base: config.base,
            });
        }
    }
    Ok(())
}

fn is_seed_admissible(lane: &FastAffineLane, seed: u64, moduli: &[u32]) -> bool {
    moduli.iter().copied().all(|modulus| {
        !((lane.shift % modulus as u64)
            + (lane.gradient % modulus as u64) * (seed % modulus as u64))
            .is_multiple_of(modulus as u64)
    })
}

fn digits_to_u128(base: u32, digits: &[u32]) -> Result<u128, FastPrimeError> {
    let mut value = 0u128;
    for &digit in digits {
        if digit >= base {
            return Err(FastPrimeError::InvalidDigit { digit, base });
        }
        value = value
            .checked_mul(base as u128)
            .and_then(|value| value.checked_add(digit as u128))
            .ok_or(FastPrimeError::LaneExceedsU64)?;
    }
    Ok(value)
}

fn checked_pow_u64(base: u32, exp: usize) -> Option<u64> {
    let mut value = 1u64;
    for _ in 0..exp {
        value = value.checked_mul(base as u64)?;
    }
    Some(value)
}

fn checked_pow_u128(base: u32, exp: usize) -> Option<u128> {
    let mut value = 1u128;
    for _ in 0..exp {
        value = value.checked_mul(base as u128)?;
    }
    Some(value)
}

fn digit_char(digit: u32) -> char {
    if digit < 10 {
        char::from_digit(digit, 10).expect("decimal digit")
    } else {
        char::from_u32('A' as u32 + digit - 10).expect("uppercase digit")
    }
}

fn gcd_u32(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let tmp = left % right;
        left = right;
        right = tmp;
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affine_values_match_visible_witnesses() {
        for (config, seed, expected) in [
            (
                FastLaneConfig::new(10, 3, 7, 2, (2, 1)),
                20,
                300_702_007_003u64,
            ),
            (
                FastLaneConfig::new(10, 3, 7, 2, (1, 1)),
                5,
                3_070_050_703u64,
            ),
            (
                FastLaneConfig::new(10, 1, 7, 2, (2, 2)),
                4,
                10_070_004_007_001u64,
            ),
            (
                FastLaneConfig::new(10, 3, 1, 3, (2, 2)),
                30,
                300_100_030_001_003u64,
            ),
            (FastLaneConfig::new(6, 1, 5, 1, (0, 0)), 4, 2551u64),
            (
                FastLaneConfig::new(22, 17, 19, 2, (0, 0)),
                10,
                92_067_883u64,
            ),
            (
                FastLaneConfig::new(22, 17, 19, 2, (2, 2)),
                13,
                4_808_275_624_019_584_921u64,
            ),
        ] {
            let lane = build_fast_affine_lane(config).expect("lane should fit u64");
            assert_eq!(lane.candidate_value(seed), Some(expected));
            assert!(primal::is_prime(expected));
        }
    }

    #[test]
    fn residue_wheel_matches_exhaustive_filtering() {
        let lane = build_fast_affine_lane(FastLaneConfig::new(10, 3, 7, 2, (2, 1))).unwrap();
        let wheel = build_residue_wheel(&lane, 1_000_000).unwrap();
        for seed in 0..lane.seed_capacity {
            let wheel_admissible = wheel.admissible_residues.contains(&(seed % wheel.period));
            let exhaustive = is_seed_admissible(&lane, seed, &wheel.moduli);
            assert_eq!(wheel_admissible, exhaustive, "seed {seed}");
        }
    }

    #[test]
    fn fast_scan_finds_known_visible_witnesses() {
        let run = scan_fast_prime_lane(
            FastLaneConfig::new(10, 3, 7, 2, (2, 1)),
            10_000,
            5,
            1_000_000,
        )
        .expect("fast scan should run");
        assert!(run.capped_to_seed_capacity);
        assert!(run.primes_found >= 5);
        assert_eq!(run.witnesses.first().map(|row| row.seed), Some(20));
        assert_eq!(
            run.witnesses.first().map(|row| row.value),
            Some(300_702_007_003u64)
        );
    }
}
