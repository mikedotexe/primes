//! Large affine witness ladder validation helpers.
//!
//! This module supports reports that measure large, human-readable affine
//! membrane prime witnesses. It keeps the arithmetic deterministic so report
//! rows are reproducible across runs.

use crate::validation::bounded_k::{digit_symbol, DEFAULT_PREFILTER_PRIMES};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::Serialize;
use std::time::Instant;

pub const PRIMARY_BASE: u32 = 10;
pub const PRIMARY_OUTER: u32 = 3;
pub const PRIMARY_INNER: u32 = 7;
pub const PRIMARY_K: (u32, u32) = (2, 1);
pub const PROBABLE_PRIME_BASES: &[u64] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
];
pub const U128_PROBABLE_PRIME_BASES: &[u128] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MersenneClassification {
    pub is_mersenne: bool,
    pub mersenne_exponent: Option<u64>,
    pub mersenne_class: String,
}

impl MersenneClassification {
    pub fn not_mersenne() -> Self {
        Self {
            is_mersenne: false,
            mersenne_exponent: None,
            mersenne_class: "not_mersenne".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LargeWitnessSettings {
    pub profile: String,
    pub seed_count: u64,
    pub control_sample_count: u64,
    pub max_witnesses: usize,
    pub middle_lengths: Vec<usize>,
    pub probable_prime_bases: Vec<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LargeWitnessReportData {
    pub settings: LargeWitnessSettings,
    pub affine_witness_rows: Vec<AffineWitnessRow>,
    pub backend_rows: Vec<BackendRow>,
    pub control_rows: Vec<ControlRow>,
    pub rarity_rows: Vec<RarityRow>,
    pub witness_gallery_rows: Vec<WitnessGalleryRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineWitnessRow {
    pub role: String,
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub k_label: String,
    pub middle_length: usize,
    pub visible_digits: usize,
    pub decimal_digits: usize,
    pub seed_count: u64,
    pub residue_moduli_label: String,
    pub residue_survivor_count: u64,
    pub residue_survivor_share: f64,
    pub probable_prime_tests: u64,
    pub witnesses_found: u64,
    pub raw_hit_rate: f64,
    pub survivor_hit_rate: f64,
    pub seeds_per_witness: f64,
    pub elapsed_seconds: f64,
    pub time_to_first_witness_seconds: f64,
    pub witnesses_per_second: f64,
    pub first_witness_seed: Option<u64>,
    pub first_witness_value: String,
    pub first_witness_template: String,
    pub compact_description: String,
    pub pnt_expected_density: f64,
    pub confirmation: String,
    pub first_witness_is_mersenne: bool,
    pub first_witness_mersenne_exponent: Option<u64>,
    pub first_witness_mersenne_class: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BackendRow {
    pub role: String,
    pub backend: String,
    pub status: String,
    pub confirmation: String,
    pub visible_digits: usize,
    pub seed_count: u64,
    pub residue_survivor_count: u64,
    pub probable_prime_tests: u64,
    pub witnesses_found: u64,
    pub raw_hit_rate: f64,
    pub survivor_hit_rate: f64,
    pub seeds_per_witness: f64,
    pub elapsed_seconds: f64,
    pub witnesses_per_second: f64,
    pub first_witness_value: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ControlRow {
    pub role: String,
    pub control_type: String,
    pub status: String,
    pub visible_digits: usize,
    pub sample_count: u64,
    pub candidates_tested: u64,
    pub witnesses_found: u64,
    pub raw_hit_rate: f64,
    pub elapsed_seconds: f64,
    pub witnesses_per_second: f64,
    pub first_witness_value: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RarityRow {
    pub role: String,
    pub visible_digits: usize,
    pub middle_length: usize,
    pub fixed_template_digits: usize,
    pub seed_digit_slots: usize,
    pub template_space_log10: f64,
    pub same_digit_space_log10: f64,
    pub template_share_log10: f64,
    pub compact_description_example: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WitnessGalleryRow {
    pub role: String,
    pub rank: usize,
    pub seed: u64,
    pub middle_digits: String,
    pub template_digits: String,
    pub decimal_value: String,
    pub compact_description: String,
    pub confirmation: String,
    pub is_mersenne: bool,
    pub mersenne_exponent: Option<u64>,
    pub mersenne_class: String,
}

#[derive(Debug, Clone)]
pub struct BigAffineLane {
    pub base: u32,
    pub outer: u32,
    pub inner: u32,
    pub middle_length: usize,
    pub k_outer: u32,
    pub k_inner: u32,
    pub shift: BigUint,
    pub gradient: BigUint,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub row: AffineWitnessRow,
    pub witnesses: Vec<WitnessGalleryRow>,
}

#[derive(Debug, Clone)]
struct WitnessHit {
    seed: u64,
    middle_digits: String,
    template_digits: String,
    value: BigUint,
    elapsed_seconds: f64,
}

pub fn build_large_witness_report(settings: LargeWitnessSettings) -> LargeWitnessReportData {
    let mut affine_witness_rows = Vec::new();
    let mut backend_rows = Vec::new();
    let mut control_rows = Vec::new();
    let mut rarity_rows = Vec::new();
    let mut witness_gallery_rows = Vec::new();

    for &middle_length in &settings.middle_lengths {
        let lane = build_primary_lane(middle_length);
        let scan = scan_biguint_affine_lane(&lane, &settings);
        backend_rows.push(backend_row_from_affine_scan(&scan.row));
        backend_rows.push(scan_u128_backend(&lane, &settings));
        backend_rows.push(scan_u64_backend(&lane, &settings));
        control_rows.extend(build_control_rows(&lane, &settings));
        rarity_rows.push(build_rarity_row(&lane, scan.row.first_witness_seed));
        witness_gallery_rows.extend(scan.witnesses.clone());
        affine_witness_rows.push(scan.row);
    }

    LargeWitnessReportData {
        settings,
        affine_witness_rows,
        backend_rows,
        control_rows,
        rarity_rows,
        witness_gallery_rows,
    }
}

pub fn build_primary_lane(middle_length: usize) -> BigAffineLane {
    build_big_affine_lane(
        PRIMARY_BASE,
        PRIMARY_OUTER,
        PRIMARY_INNER,
        middle_length,
        PRIMARY_K,
    )
}

pub fn middle_length_for_visible_digits(visible_digits: usize) -> Option<usize> {
    let fixed_digits = fixed_template_digits(PRIMARY_K);
    visible_digits.checked_sub(fixed_digits)
}

pub fn visible_digits_for_middle_length(middle_length: usize) -> usize {
    middle_length + fixed_template_digits(PRIMARY_K)
}

pub fn role_for_digits(visible_digits: usize) -> String {
    format!("decimal_{visible_digits}_digit_visible_lane")
}

pub fn build_big_affine_lane(
    base: u32,
    outer: u32,
    inner: u32,
    middle_length: usize,
    (k_outer, k_inner): (u32, u32),
) -> BigAffineLane {
    let prefix_digits = {
        let mut digits = Vec::with_capacity((2 + k_outer + k_inner) as usize);
        digits.push(outer);
        digits.extend(std::iter::repeat_n(0, k_outer as usize));
        digits.push(inner);
        digits.extend(std::iter::repeat_n(0, k_inner as usize));
        digits
    };
    let suffix_digits = {
        let mut digits = Vec::with_capacity((2 + k_outer + k_inner) as usize);
        digits.extend(std::iter::repeat_n(0, k_inner as usize));
        digits.push(inner);
        digits.extend(std::iter::repeat_n(0, k_outer as usize));
        digits.push(outer);
        digits
    };
    let base_big = BigUint::from(base);
    let suffix_len = suffix_digits.len() as u32;
    let gradient = base_big.pow(suffix_len);
    let prefix_shift = base_big.pow(middle_length as u32 + suffix_len);
    let prefix_value = digits_to_biguint(base, &prefix_digits);
    let suffix_value = digits_to_biguint(base, &suffix_digits);
    let shift = prefix_value * prefix_shift + suffix_value;
    BigAffineLane {
        base,
        outer,
        inner,
        middle_length,
        k_outer,
        k_inner,
        shift,
        gradient,
    }
}

pub fn candidate_value(lane: &BigAffineLane, seed: u64) -> BigUint {
    &lane.shift + &lane.gradient * BigUint::from(seed)
}

pub fn classify_mersenne(n: &BigUint) -> MersenneClassification {
    if n.is_zero() {
        return MersenneClassification::not_mersenne();
    }
    let candidate_power = n + BigUint::one();
    let bits = candidate_power.bits();
    if bits == 0 {
        return MersenneClassification::not_mersenne();
    }
    let exponent = bits - 1;
    let Ok(shift) = usize::try_from(exponent) else {
        return MersenneClassification::not_mersenne();
    };
    if (BigUint::one() << shift) == candidate_power {
        MersenneClassification {
            is_mersenne: true,
            mersenne_exponent: Some(exponent),
            mersenne_class: format!("mersenne_2^{exponent}-1"),
        }
    } else {
        MersenneClassification::not_mersenne()
    }
}

pub fn template_digits(lane: &BigAffineLane, seed: u64) -> String {
    let mut digits = String::new();
    digits.push_str(&digit_symbol(lane.outer));
    digits.extend(std::iter::repeat_n('0', lane.k_outer as usize));
    digits.push_str(&digit_symbol(lane.inner));
    digits.extend(std::iter::repeat_n('0', lane.k_inner as usize));
    digits.push_str(&middle_digits(lane.base, lane.middle_length, seed));
    digits.extend(std::iter::repeat_n('0', lane.k_inner as usize));
    digits.push_str(&digit_symbol(lane.inner));
    digits.extend(std::iter::repeat_n('0', lane.k_outer as usize));
    digits.push_str(&digit_symbol(lane.outer));
    digits
}

pub fn middle_digits(base: u32, middle_length: usize, mut seed: u64) -> String {
    let mut digits = vec!['0'; middle_length];
    for digit in digits.iter_mut().rev() {
        let value = seed % base as u64;
        *digit = digit_char(value as u32);
        seed /= base as u64;
    }
    digits.into_iter().collect()
}

pub fn residue_moduli(base: u32) -> Vec<u32> {
    DEFAULT_PREFILTER_PRIMES
        .iter()
        .copied()
        .filter(|&modulus| gcd_u32(base, modulus) == 1)
        .collect()
}

pub fn residue_allows_seed(lane: &BigAffineLane, seed: u64, moduli: &[u32]) -> bool {
    moduli.iter().copied().all(|modulus| {
        let p = BigUint::from(modulus);
        let seed_big = BigUint::from(seed);
        let value_mod = (&lane.shift + &lane.gradient * seed_big) % &p;
        !value_mod.is_zero()
    })
}

pub fn is_probable_prime_fixed_bases(n: &BigUint, bases: &[u64]) -> bool {
    if n < &BigUint::from(2u32) {
        return false;
    }
    for &prime in bases {
        let prime_big = BigUint::from(prime);
        if n == &prime_big {
            return true;
        }
        if n % &prime_big == BigUint::zero() {
            return false;
        }
    }

    let one = BigUint::one();
    let two = BigUint::from(2u32);
    let n_minus_one = n - &one;
    let mut d = n_minus_one.clone();
    let mut s = 0usize;
    while &d % &two == BigUint::zero() {
        d /= &two;
        s += 1;
    }

    'bases: for &base in bases {
        let base_big = BigUint::from(base);
        if base_big >= n_minus_one {
            continue;
        }
        let mut x = base_big.modpow(&d, n);
        if x == one || x == n_minus_one {
            continue;
        }
        for _ in 1..s {
            x = (&x * &x) % n;
            if x == n_minus_one {
                continue 'bases;
            }
        }
        return false;
    }
    true
}

pub fn is_probable_prime_u128(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    for &prime in U128_PROBABLE_PRIME_BASES {
        if n == prime {
            return true;
        }
        if n.is_multiple_of(prime) {
            return false;
        }
    }

    let mut d = n - 1;
    let mut s = 0u32;
    while d.is_multiple_of(2) {
        d /= 2;
        s += 1;
    }

    'bases: for &base in U128_PROBABLE_PRIME_BASES {
        if base >= n - 1 {
            continue;
        }
        let mut x = mod_pow_u128(base, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mod_mul_u128(x, x, n);
            if x == n - 1 {
                continue 'bases;
            }
        }
        return false;
    }
    true
}

pub fn random_odd_same_digits(digits: usize, rng: &mut XorShift64) -> BigUint {
    let mut text = String::with_capacity(digits);
    text.push(digit_char(1 + (rng.next_u64() % 9) as u32));
    for _ in 1..digits.saturating_sub(1) {
        text.push(digit_char((rng.next_u64() % 10) as u32));
    }
    let last = [1u32, 3, 5, 7, 9][(rng.next_u64() % 5) as usize];
    text.push(digit_char(last));
    BigUint::parse_bytes(text.as_bytes(), 10).expect("decimal candidate")
}

pub fn random_coprime_same_digits(digits: usize, rng: &mut XorShift64) -> BigUint {
    let mut text = String::with_capacity(digits);
    text.push(digit_char(1 + (rng.next_u64() % 9) as u32));
    for _ in 1..digits.saturating_sub(1) {
        text.push(digit_char((rng.next_u64() % 10) as u32));
    }
    let last = [1u32, 3, 7, 9][(rng.next_u64() % 4) as usize];
    text.push(digit_char(last));
    BigUint::parse_bytes(text.as_bytes(), 10).expect("decimal candidate")
}

pub fn same_slot_random_candidate(lane: &BigAffineLane, rng: &mut XorShift64) -> BigUint {
    let units = [1u32, 3, 7, 9];
    let outer = units[(rng.next_u64() % units.len() as u64) as usize];
    let inner = units[(rng.next_u64() % units.len() as u64) as usize];
    let seed = rng.next_u64() % pow10_u64_capped(lane.middle_length);
    let random_lane = build_big_affine_lane(
        lane.base,
        outer,
        inner,
        lane.middle_length,
        (lane.k_outer, lane.k_inner),
    );
    candidate_value(&random_lane, seed)
}

pub fn compact_description(lane: &BigAffineLane, seed: u64) -> String {
    format!(
        "base={}, pair=({},{}), k=({},{}), M={}, seed={}",
        lane.base, lane.outer, lane.inner, lane.k_outer, lane.k_inner, lane.middle_length, seed
    )
}

pub fn same_digit_space_log10(digits: usize) -> f64 {
    (digits.saturating_sub(1)) as f64 + 9f64.log10()
}

pub fn template_share_log10(lane: &BigAffineLane) -> f64 {
    lane.middle_length as f64 - same_digit_space_log10(visible_digits(lane))
}

pub fn visible_digits(lane: &BigAffineLane) -> usize {
    lane.middle_length + fixed_template_digits((lane.k_outer, lane.k_inner))
}

fn scan_biguint_affine_lane(lane: &BigAffineLane, settings: &LargeWitnessSettings) -> ScanResult {
    let role = role_for_digits(visible_digits(lane));
    let moduli = residue_moduli(lane.base);
    let started = Instant::now();
    let mut survivor_count = 0u64;
    let mut tests = 0u64;
    let mut witnesses = Vec::new();
    let mut first_elapsed = 0.0;

    for seed in 0..settings.seed_count {
        if !residue_allows_seed(lane, seed, &moduli) {
            continue;
        }
        survivor_count += 1;
        tests += 1;
        let value = candidate_value(lane, seed);
        if is_probable_prime_fixed_bases(&value, &settings.probable_prime_bases) {
            let elapsed = started.elapsed().as_secs_f64();
            if witnesses.is_empty() {
                first_elapsed = elapsed;
            }
            witnesses.push(WitnessHit {
                seed,
                middle_digits: middle_digits(lane.base, lane.middle_length, seed),
                template_digits: template_digits(lane, seed),
                value,
                elapsed_seconds: elapsed,
            });
        }
    }

    let elapsed = started.elapsed().as_secs_f64().max(1e-12);
    let witness_count = witnesses.len() as u64;
    let gallery = witnesses
        .iter()
        .take(settings.max_witnesses)
        .enumerate()
        .map(|(idx, witness)| {
            let mersenne = classify_mersenne(&witness.value);
            WitnessGalleryRow {
                role: role.clone(),
                rank: idx + 1,
                seed: witness.seed,
                middle_digits: witness.middle_digits.clone(),
                template_digits: witness.template_digits.clone(),
                decimal_value: witness.value.to_str_radix(10),
                compact_description: compact_description(lane, witness.seed),
                confirmation: format!(
                    "probable_prime_fixed_{}_bases",
                    settings.probable_prime_bases.len()
                ),
                is_mersenne: mersenne.is_mersenne,
                mersenne_exponent: mersenne.mersenne_exponent,
                mersenne_class: mersenne.mersenne_class,
            }
        })
        .collect::<Vec<_>>();
    let first = witnesses.first();
    let first_mersenne = first
        .map(|hit| classify_mersenne(&hit.value))
        .unwrap_or_else(MersenneClassification::not_mersenne);
    let decimal_digits = lane.shift.to_str_radix(10).len();
    let row = AffineWitnessRow {
        role,
        base: lane.base,
        outer: lane.outer,
        inner: lane.inner,
        k_label: format!("k=({},{})", lane.k_outer, lane.k_inner),
        middle_length: lane.middle_length,
        visible_digits: visible_digits(lane),
        decimal_digits,
        seed_count: settings.seed_count,
        residue_moduli_label: join_moduli(&moduli),
        residue_survivor_count: survivor_count,
        residue_survivor_share: ratio_count(survivor_count, settings.seed_count),
        probable_prime_tests: tests,
        witnesses_found: witness_count,
        raw_hit_rate: ratio_count(witness_count, settings.seed_count),
        survivor_hit_rate: ratio_count(witness_count, survivor_count),
        seeds_per_witness: ratio_count(settings.seed_count, witness_count),
        elapsed_seconds: elapsed,
        time_to_first_witness_seconds: first
            .map(|hit| hit.elapsed_seconds)
            .unwrap_or(first_elapsed),
        witnesses_per_second: ratio_time(witness_count, elapsed),
        first_witness_seed: first.map(|hit| hit.seed),
        first_witness_value: first
            .map(|hit| hit.value.to_str_radix(10))
            .unwrap_or_default(),
        first_witness_template: first
            .map(|hit| hit.template_digits.clone())
            .unwrap_or_default(),
        compact_description: first
            .map(|hit| compact_description(lane, hit.seed))
            .unwrap_or_else(|| compact_description(lane, 0)),
        pnt_expected_density: pnt_density_estimate(&lane.shift),
        confirmation: format!(
            "probable_prime_fixed_{}_bases",
            settings.probable_prime_bases.len()
        ),
        first_witness_is_mersenne: first_mersenne.is_mersenne,
        first_witness_mersenne_exponent: first_mersenne.mersenne_exponent,
        first_witness_mersenne_class: first_mersenne.mersenne_class,
    };
    ScanResult {
        row,
        witnesses: gallery,
    }
}

fn backend_row_from_affine_scan(row: &AffineWitnessRow) -> BackendRow {
    BackendRow {
        role: row.role.clone(),
        backend: "biguint_fixed_bases".to_string(),
        status: "ok".to_string(),
        confirmation: row.confirmation.clone(),
        visible_digits: row.visible_digits,
        seed_count: row.seed_count,
        residue_survivor_count: row.residue_survivor_count,
        probable_prime_tests: row.probable_prime_tests,
        witnesses_found: row.witnesses_found,
        raw_hit_rate: row.raw_hit_rate,
        survivor_hit_rate: row.survivor_hit_rate,
        seeds_per_witness: row.seeds_per_witness,
        elapsed_seconds: row.elapsed_seconds,
        witnesses_per_second: row.witnesses_per_second,
        first_witness_value: row.first_witness_value.clone(),
        note: "BigUint probable-prime confirmation with fixed Miller-Rabin bases".to_string(),
    }
}

fn scan_u128_backend(lane: &BigAffineLane, settings: &LargeWitnessSettings) -> BackendRow {
    let role = role_for_digits(visible_digits(lane));
    let Some((shift, gradient)) = lane_u128_parts(lane) else {
        return unavailable_backend(
            &role,
            "u128_fixed_bases",
            visible_digits(lane),
            "candidate lane does not fit in u128",
        );
    };
    if shift
        .checked_add(gradient.saturating_mul(settings.seed_count.saturating_sub(1) as u128))
        .is_none()
    {
        return unavailable_backend(
            &role,
            "u128_fixed_bases",
            visible_digits(lane),
            "requested seed prefix exceeds u128",
        );
    }

    let moduli = residue_moduli(lane.base);
    let started = Instant::now();
    let mut survivors = 0u64;
    let mut witnesses = 0u64;
    let mut first = String::new();
    for seed in 0..settings.seed_count {
        if !residue_allows_seed_u128(shift, gradient, seed, &moduli) {
            continue;
        }
        survivors += 1;
        let Some(product) = gradient.checked_mul(seed as u128) else {
            continue;
        };
        let Some(value) = shift.checked_add(product) else {
            continue;
        };
        if is_probable_prime_u128(value) {
            witnesses += 1;
            if first.is_empty() {
                first = value.to_string();
            }
        }
    }
    let elapsed = started.elapsed().as_secs_f64().max(1e-12);
    BackendRow {
        role,
        backend: "u128_fixed_bases".to_string(),
        status: "ok".to_string(),
        confirmation: format!(
            "probable_prime_fixed_{}_u128_bases",
            U128_PROBABLE_PRIME_BASES.len()
        ),
        visible_digits: visible_digits(lane),
        seed_count: settings.seed_count,
        residue_survivor_count: survivors,
        probable_prime_tests: survivors,
        witnesses_found: witnesses,
        raw_hit_rate: ratio_count(witnesses, settings.seed_count),
        survivor_hit_rate: ratio_count(witnesses, survivors),
        seeds_per_witness: ratio_count(settings.seed_count, witnesses),
        elapsed_seconds: elapsed,
        witnesses_per_second: ratio_time(witnesses, elapsed),
        first_witness_value: first,
        note: "Fixed-width u128 probable-prime confirmation".to_string(),
    }
}

fn scan_u64_backend(lane: &BigAffineLane, settings: &LargeWitnessSettings) -> BackendRow {
    let role = role_for_digits(visible_digits(lane));
    let Some((shift, gradient)) = lane_u64_parts(lane) else {
        return unavailable_backend(
            &role,
            "u64_deterministic",
            visible_digits(lane),
            "candidate lane does not fit in u64",
        );
    };
    if shift
        .checked_add(gradient.saturating_mul(settings.seed_count.saturating_sub(1)))
        .is_none()
    {
        return unavailable_backend(
            &role,
            "u64_deterministic",
            visible_digits(lane),
            "requested seed prefix exceeds u64",
        );
    }

    let moduli = residue_moduli(lane.base);
    let started = Instant::now();
    let mut survivors = 0u64;
    let mut witnesses = 0u64;
    let mut first = String::new();
    for seed in 0..settings.seed_count {
        if !residue_allows_seed_u64(shift, gradient, seed, &moduli) {
            continue;
        }
        survivors += 1;
        let Some(value) = shift.checked_add(gradient.saturating_mul(seed)) else {
            continue;
        };
        if primal::is_prime(value) {
            witnesses += 1;
            if first.is_empty() {
                first = value.to_string();
            }
        }
    }
    let elapsed = started.elapsed().as_secs_f64().max(1e-12);
    BackendRow {
        role,
        backend: "u64_deterministic".to_string(),
        status: "ok".to_string(),
        confirmation: "deterministic_primal_u64".to_string(),
        visible_digits: visible_digits(lane),
        seed_count: settings.seed_count,
        residue_survivor_count: survivors,
        probable_prime_tests: survivors,
        witnesses_found: witnesses,
        raw_hit_rate: ratio_count(witnesses, settings.seed_count),
        survivor_hit_rate: ratio_count(witnesses, survivors),
        seeds_per_witness: ratio_count(settings.seed_count, witnesses),
        elapsed_seconds: elapsed,
        witnesses_per_second: ratio_time(witnesses, elapsed),
        first_witness_value: first,
        note: "Deterministic u64 primality confirmation".to_string(),
    }
}

pub fn build_control_rows(
    lane: &BigAffineLane,
    settings: &LargeWitnessSettings,
) -> Vec<ControlRow> {
    let mut rng = XorShift64::new(control_seed(lane));
    vec![
        scan_control(
            lane,
            settings,
            "random_odd_same_digits",
            |lane, rng| random_odd_same_digits(visible_digits(lane), rng),
            &mut rng,
        ),
        scan_control(
            lane,
            settings,
            "random_coprime_same_digits",
            |lane, rng| random_coprime_same_digits(visible_digits(lane), rng),
            &mut rng,
        ),
        scan_control(
            lane,
            settings,
            "same_slot_random_membrane",
            same_slot_random_candidate,
            &mut rng,
        ),
    ]
}

fn scan_control<F>(
    lane: &BigAffineLane,
    settings: &LargeWitnessSettings,
    control_type: &str,
    mut candidate: F,
    rng: &mut XorShift64,
) -> ControlRow
where
    F: FnMut(&BigAffineLane, &mut XorShift64) -> BigUint,
{
    let started = Instant::now();
    let mut witnesses = 0u64;
    let mut first = String::new();
    for _ in 0..settings.control_sample_count {
        let value = candidate(lane, rng);
        if is_probable_prime_fixed_bases(&value, &settings.probable_prime_bases) {
            witnesses += 1;
            if first.is_empty() {
                first = value.to_str_radix(10);
            }
        }
    }
    let elapsed = started.elapsed().as_secs_f64().max(1e-12);
    ControlRow {
        role: role_for_digits(visible_digits(lane)),
        control_type: control_type.to_string(),
        status: "ok".to_string(),
        visible_digits: visible_digits(lane),
        sample_count: settings.control_sample_count,
        candidates_tested: settings.control_sample_count,
        witnesses_found: witnesses,
        raw_hit_rate: ratio_count(witnesses, settings.control_sample_count),
        elapsed_seconds: elapsed,
        witnesses_per_second: ratio_time(witnesses, elapsed),
        first_witness_value: first,
        note: "Deterministic local control using fixed Miller-Rabin bases".to_string(),
    }
}

fn build_rarity_row(lane: &BigAffineLane, first_seed: Option<u64>) -> RarityRow {
    let role = role_for_digits(visible_digits(lane));
    let fixed_digits = fixed_template_digits((lane.k_outer, lane.k_inner));
    RarityRow {
        role,
        visible_digits: visible_digits(lane),
        middle_length: lane.middle_length,
        fixed_template_digits: fixed_digits,
        seed_digit_slots: lane.middle_length,
        template_space_log10: lane.middle_length as f64,
        same_digit_space_log10: same_digit_space_log10(visible_digits(lane)),
        template_share_log10: template_share_log10(lane),
        compact_description_example: compact_description(lane, first_seed.unwrap_or(0)),
        note: "This is the share of same-digit decimal strings targeted by the fixed visible template family.".to_string(),
    }
}

fn unavailable_backend(role: &str, backend: &str, visible_digits: usize, note: &str) -> BackendRow {
    BackendRow {
        role: role.to_string(),
        backend: backend.to_string(),
        status: "out_of_scope".to_string(),
        confirmation: String::new(),
        visible_digits,
        seed_count: 0,
        residue_survivor_count: 0,
        probable_prime_tests: 0,
        witnesses_found: 0,
        raw_hit_rate: 0.0,
        survivor_hit_rate: 0.0,
        seeds_per_witness: 0.0,
        elapsed_seconds: 0.0,
        witnesses_per_second: 0.0,
        first_witness_value: String::new(),
        note: note.to_string(),
    }
}

fn lane_u128_parts(lane: &BigAffineLane) -> Option<(u128, u128)> {
    Some((parse_u128(&lane.shift)?, parse_u128(&lane.gradient)?))
}

fn lane_u64_parts(lane: &BigAffineLane) -> Option<(u64, u64)> {
    Some((parse_u64(&lane.shift)?, parse_u64(&lane.gradient)?))
}

fn parse_u128(value: &BigUint) -> Option<u128> {
    value.to_str_radix(10).parse().ok()
}

fn parse_u64(value: &BigUint) -> Option<u64> {
    value.to_str_radix(10).parse().ok()
}

fn residue_allows_seed_u128(shift: u128, gradient: u128, seed: u64, moduli: &[u32]) -> bool {
    moduli.iter().copied().all(|modulus| {
        let p = modulus as u128;
        !(shift % p + (gradient % p) * (seed as u128 % p)).is_multiple_of(p)
    })
}

fn residue_allows_seed_u64(shift: u64, gradient: u64, seed: u64, moduli: &[u32]) -> bool {
    moduli.iter().copied().all(|modulus| {
        let p = modulus as u64;
        !(shift % p + (gradient % p) * (seed % p)).is_multiple_of(p)
    })
}

fn pnt_density_estimate(value: &BigUint) -> f64 {
    let digits = value.to_str_radix(10).len() as f64;
    1.0 / (digits * std::f64::consts::LN_10)
}

fn fixed_template_digits((k_outer, k_inner): (u32, u32)) -> usize {
    (4 + 2 * (k_outer + k_inner)) as usize
}

fn digits_to_biguint(base: u32, digits: &[u32]) -> BigUint {
    let base_big = BigUint::from(base);
    let mut value = BigUint::from(0u32);
    for &digit in digits {
        value *= &base_big;
        value += digit;
    }
    value
}

fn pow10_u64_capped(exp: usize) -> u64 {
    let mut value = 1u64;
    for _ in 0..exp {
        value = value.saturating_mul(10);
    }
    value
}

fn control_seed(lane: &BigAffineLane) -> u64 {
    0x9E37_79B9_7F4A_7C15u64
        ^ ((lane.middle_length as u64) << 32)
        ^ ((lane.outer as u64) << 16)
        ^ lane.inner as u64
}

fn join_moduli(moduli: &[u32]) -> String {
    moduli
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn ratio_count(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn ratio_time(count: u64, seconds: f64) -> f64 {
    if seconds <= 0.0 {
        0.0
    } else {
        count as f64 / seconds
    }
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

fn mod_pow_u128(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut acc = 1u128;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = mod_mul_u128(acc, base, modulus);
        }
        exp >>= 1;
        if exp > 0 {
            base = mod_mul_u128(base, base, modulus);
        }
    }
    acc
}

fn mod_mul_u128(mut left: u128, mut right: u128, modulus: u128) -> u128 {
    if let Some(product) = left.checked_mul(right) {
        return product % modulus;
    }

    left %= modulus;
    let mut acc = 0u128;
    while right > 0 {
        if right & 1 == 1 {
            acc = add_mod_u128(acc, left, modulus);
        }
        right >>= 1;
        if right > 0 {
            left = add_mod_u128(left, left, modulus);
        }
    }
    acc
}

fn add_mod_u128(left: u128, right: u128, modulus: u128) -> u128 {
    if left >= modulus - right {
        left - (modulus - right)
    } else {
        left + right
    }
}

#[derive(Debug, Clone)]
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0xA076_1D64_78BD_642F
            } else {
                seed
            },
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn smoke_settings(seed_count: u64) -> LargeWitnessSettings {
        LargeWitnessSettings {
            profile: "test".to_string(),
            seed_count,
            control_sample_count: 64,
            max_witnesses: 5,
            middle_lengths: vec![12],
            probable_prime_bases: PROBABLE_PRIME_BASES.to_vec(),
        }
    }

    #[test]
    fn primary_lane_visible_digit_mapping_is_stable() {
        assert_eq!(middle_length_for_visible_digits(22), Some(12));
        assert_eq!(middle_length_for_visible_digits(38), Some(28));
        assert_eq!(visible_digits_for_middle_length(12), 22);
        let lane = build_primary_lane(12);
        assert_eq!(template_digits(&lane, 62), "3007000000000006207003");
        assert_eq!(
            candidate_value(&lane, 62).to_str_radix(10),
            "3007000000000006207003"
        );
    }

    #[test]
    fn residue_filter_matches_direct_modulo_for_seed_prefix() {
        let lane = build_primary_lane(12);
        let moduli = residue_moduli(lane.base);
        for seed in 0..200 {
            let direct = moduli.iter().all(|&modulus| {
                let p = BigUint::from(modulus);
                candidate_value(&lane, seed) % p != BigUint::zero()
            });
            assert_eq!(residue_allows_seed(&lane, seed, &moduli), direct);
        }
    }

    #[test]
    fn smoke_witness_rows_are_reproducible_and_metric_consistent() {
        let lane = build_primary_lane(12);
        let settings = smoke_settings(100);
        let first = scan_biguint_affine_lane(&lane, &settings);
        let second = scan_biguint_affine_lane(&lane, &settings);
        assert_eq!(first.row.first_witness_seed, Some(62));
        assert_eq!(
            first.row.first_witness_value,
            second.row.first_witness_value
        );
        assert!(is_probable_prime_fixed_bases(
            &BigUint::parse_bytes(first.row.first_witness_value.as_bytes(), 10).unwrap(),
            &settings.probable_prime_bases
        ));
        assert_eq!(
            first.row.raw_hit_rate,
            first.row.witnesses_found as f64 / first.row.seed_count as f64
        );
        assert_eq!(
            first.row.seeds_per_witness,
            first.row.seed_count as f64 / first.row.witnesses_found as f64
        );
    }

    #[test]
    fn controls_preserve_requested_constraints() {
        let lane = build_primary_lane(12);
        let mut rng = XorShift64::new(123);
        let odd = random_odd_same_digits(22, &mut rng).to_str_radix(10);
        assert_eq!(odd.len(), 22);
        assert!(matches!(
            odd.as_bytes().last(),
            Some(b'1' | b'3' | b'5' | b'7' | b'9')
        ));

        let coprime = random_coprime_same_digits(22, &mut rng).to_str_radix(10);
        assert_eq!(coprime.len(), 22);
        assert!(matches!(
            coprime.as_bytes().last(),
            Some(b'1' | b'3' | b'7' | b'9')
        ));

        let same_slot = same_slot_random_candidate(&lane, &mut rng).to_str_radix(10);
        assert_eq!(same_slot.len(), 22);
        assert_eq!(&same_slot[1..3], "00");
        assert_eq!(&same_slot[4..5], "0");
        assert_eq!(&same_slot[17..18], "0");
        assert_eq!(&same_slot[19..21], "00");
    }

    #[test]
    fn backend_scope_labels_u64_and_u128_boundaries() {
        let settings = smoke_settings(100);
        let lane22 = build_primary_lane(12);
        let u64_row = scan_u64_backend(&lane22, &settings);
        assert_eq!(u64_row.status, "out_of_scope");
        let u128_row = scan_u128_backend(&lane22, &settings);
        assert_eq!(u128_row.status, "ok");

        let lane50 = build_primary_lane(40);
        let u128_row = scan_u128_backend(&lane50, &settings);
        assert_eq!(u128_row.status, "out_of_scope");
    }

    #[test]
    fn mersenne_classification_distinguishes_special_form() {
        let mersenne = classify_mersenne(&BigUint::from(31u32));
        assert!(mersenne.is_mersenne);
        assert_eq!(mersenne.mersenne_exponent, Some(5));
        assert_eq!(mersenne.mersenne_class, "mersenne_2^5-1");

        let non_mersenne = classify_mersenne(&BigUint::from(37u32));
        assert!(!non_mersenne.is_mersenne);
        assert_eq!(non_mersenne.mersenne_exponent, None);
        assert_eq!(non_mersenne.mersenne_class, "not_mersenne");
    }
}
