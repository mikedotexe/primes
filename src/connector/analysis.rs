//! Arithmetic-first connector analysis utilities.
//!
//! This module provides the maintained vocabulary and exact bounded scans for
//! the connector lane:
//!
//! - [`ConnectorHit`]: a fixed-pair, fixed-width, single-digit connector case,
//! - [`ResidueAdmissible`]: whether a candidate survives the exact small-modulus
//!   filters currently used in the repo,
//! - [`ResonancePosition`]: a width/position with multiple working digits in a
//!   matched scan,
//! - [`DirectionalAsymmetry`]: a summary of forward/reverse differences after
//!   the direction-independent residue layer is accounted for.
//!
//! The repository still permits "Lagrange point" as an informal alias for a
//! productive insertion point, but this module uses arithmetic language first.

use super::{utils, ConcatenationSystem, Direction};
use crate::is_prime;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Canonical width-5 connector cases preserved in the Agda shell.
///
/// These remain part of the maintained source case, but shell-reported cases
/// should be rerun through the current arithmetic checker before being reused
/// as verified prime hits.
pub const CANONICAL_WIDTH5_HITS: &[(u32, u32, u8)] = &[(5, 1, 6), (5, 4, 6)];

/// Canonical forward-direction cases preserved by the maintained Rust demo.
///
/// These remain part of the maintained source case. They should be rerun
/// through the current arithmetic checker before being promoted into a narrow
/// empirical claim.
pub const CANONICAL_DOCUMENTED_FORWARD_HITS: &[(u32, u32, u8)] =
    &[(5, 4, 6), (6, 1, 6), (6, 4, 6), (7, 3, 6)];

/// Canonical source cases preserved across the maintained examples and shells.
///
/// This union contains:
/// - the two width-5 cases from the Agda connector shell, and
/// - the four forward cases currently shown by the Rust connector demo.
pub const CANONICAL_SOURCE_HITS: &[(u32, u32, u8)] =
    &[(5, 1, 6), (5, 4, 6), (6, 1, 6), (6, 4, 6), (7, 3, 6)];

/// Default small-prime sieve used for density-aware connector audits.
pub const DEFAULT_SMALL_PRIMES: &[u32] = &[2, 3, 5, 7, 11, 13, 17, 19];

/// Whether a candidate survives the maintained residue filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResidueAdmissible {
    /// At least one exact modulus filter rules the candidate out.
    No,
    /// The candidate survives all configured exact modulus filters.
    Yes,
}

impl ResidueAdmissible {
    fn from_bool(value: bool) -> Self {
        if value {
            Self::Yes
        } else {
            Self::No
        }
    }

    /// Return `true` when the candidate survives the configured filters.
    pub fn is_yes(self) -> bool {
        matches!(self, Self::Yes)
    }
}

/// Residue profile for one fixed pair modulo one small modulus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PairResidueProfile {
    /// Fixed prime pair under study.
    pub pair: ConcatenationSystem,
    /// Small modulus used for the exact obstruction.
    pub modulus: u32,
    /// `(left + right) mod modulus`.
    pub pair_residue: u32,
    /// Connector residue class that forces divisibility.
    pub blocked_connector_residue: u32,
}

impl PairResidueProfile {
    /// Build the residue profile for one pair/modulus combination.
    pub fn new(pair: ConcatenationSystem, modulus: u32) -> Self {
        let pair_residue = utils::pair_residue_mod(pair.left, pair.right, modulus);
        let blocked_connector_residue =
            utils::blocked_connector_residue(pair.left, pair.right, modulus);
        Self {
            pair,
            modulus,
            pair_residue,
            blocked_connector_residue,
        }
    }

    /// Classify a single-digit connector case against this modulus.
    pub fn classify(&self, hit: &ConnectorHit) -> ResidueAdmissible {
        debug_assert_eq!(
            hit.pair, self.pair,
            "residue profile and hit use different pairs"
        );
        let connector = match hit.connector_value() {
            Some(connector) => connector,
            None => return ResidueAdmissible::No,
        };
        ResidueAdmissible::from_bool(!utils::should_skip_modulo(
            connector,
            (hit.pair.left % self.modulus as u128) as u32,
            (hit.pair.right % self.modulus as u128) as u32,
            self.modulus,
        ))
    }
}

/// Arithmetic-first description of one single-digit connector candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConnectorHit {
    /// Fixed prime pair under study.
    pub pair: ConcatenationSystem,
    /// Buffer width in decimal digits.
    pub width: u32,
    /// Zero-based digit position counted from the left edge of the buffer.
    pub position: u32,
    /// Non-zero digit inserted into the zero buffer.
    pub digit: u8,
    /// Concatenation direction.
    pub direction: Direction,
}

impl ConnectorHit {
    /// Create a checked single-digit connector case.
    pub fn new(
        pair: ConcatenationSystem,
        width: u32,
        position: u32,
        digit: u8,
        direction: Direction,
    ) -> Option<Self> {
        if width == 0 || position >= width || !(1..=9).contains(&digit) || !pair.fits_in_u128(width)
        {
            return None;
        }

        Some(Self {
            pair,
            width,
            position,
            digit,
            direction,
        })
    }

    /// Compute the numeric connector value represented by this hit.
    pub fn connector_value(&self) -> Option<u128> {
        utils::single_digit_connector_value(self.width, self.position, self.digit)
    }

    /// Render the connector as a zero-padded decimal string.
    pub fn connector_string(&self) -> Option<String> {
        self.connector_value()
            .map(|connector| format!("{connector:0width$}", width = self.width as usize))
    }

    /// Build the full concatenated integer.
    pub fn concatenated_value(&self) -> Option<u128> {
        let connector = self.connector_value()?;
        match self.direction {
            Direction::Forward => self.pair.forward(connector, self.width),
            Direction::Reverse => self.pair.reverse(connector, self.width),
        }
    }

    /// Test whether the full concatenation is prime.
    pub fn is_prime(&self) -> bool {
        self.concatenated_value()
            .map(BigUint::from)
            .map(|n| is_prime(&n))
            .unwrap_or(false)
    }

    /// Natural logarithm of the full concatenated value.
    pub fn natural_log(&self) -> Option<f64> {
        self.concatenated_value().and_then(|value| {
            let value = value as f64;
            (value > 1.0).then_some(value.ln())
        })
    }

    /// Classify this hit against several exact residue profiles.
    pub fn residue_admissible(&self, profiles: &[PairResidueProfile]) -> ResidueAdmissible {
        ResidueAdmissible::from_bool(
            profiles
                .iter()
                .all(|profile| profile.classify(self).is_yes()),
        )
    }
}

/// One scanned single-digit candidate with its exact filter outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConnectorCandidate {
    /// Arithmetic description of the candidate.
    pub hit: ConnectorHit,
    /// Whether the candidate survives the exact residue layer.
    pub residue_admissible: ResidueAdmissible,
    /// Whether the full concatenation is prime.
    pub is_prime: bool,
}

impl ConnectorCandidate {
    /// Natural-log baseline probability proxy `1 / ln(n)` for this candidate.
    pub fn naive_prime_probability(&self) -> f64 {
        self.hit
            .natural_log()
            .filter(|ln| *ln > 0.0)
            .map(|ln| 1.0 / ln)
            .unwrap_or(0.0)
    }

    /// Return `true` when the full concatenation is divisible by `prime`.
    pub fn divisible_by_small_prime(&self, prime: u32) -> bool {
        if prime < 2 {
            return false;
        }

        self.hit
            .concatenated_value()
            .map(|value| value != prime as u128 && value % prime as u128 == 0)
            .unwrap_or(true)
    }

    /// Return `true` when the candidate survives every requested small-prime check.
    pub fn survives_small_prime_sieve(&self, small_primes: &[u32]) -> bool {
        small_primes
            .iter()
            .copied()
            .all(|prime| !self.divisible_by_small_prime(prime))
    }
}

/// Width/position location with multiple working digits in one matched scan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResonancePosition {
    /// Fixed prime pair under study.
    pub pair: ConcatenationSystem,
    /// Concatenation direction for this resonance summary.
    pub direction: Direction,
    /// Buffer width in decimal digits.
    pub width: u32,
    /// Zero-based position counted from the left edge of the buffer.
    pub position: u32,
    /// Sorted working digits at this width/position.
    pub digits: Vec<u8>,
}

impl ResonancePosition {
    /// Number of working digits at this width/position.
    pub fn multiplicity(&self) -> usize {
        self.digits.len()
    }
}

/// Per-direction scan summary for one pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionScanStats {
    /// Concatenation direction summarized by this record.
    pub direction: Direction,
    /// All matched candidates in the bounded scan.
    pub candidates: Vec<ConnectorCandidate>,
}

impl DirectionScanStats {
    /// Number of tested width/position/digit cases.
    pub fn raw_candidates(&self) -> usize {
        self.candidates.len()
    }

    /// Number of candidates that survive the exact residue filters.
    pub fn residue_admissible_candidates(&self) -> usize {
        self.candidates
            .iter()
            .filter(|candidate| candidate.residue_admissible.is_yes())
            .count()
    }

    /// Prime hits found in this direction.
    pub fn prime_hits(&self) -> usize {
        self.candidates
            .iter()
            .filter(|candidate| candidate.is_prime)
            .count()
    }

    /// Prime hits as arithmetic connector descriptions.
    pub fn hit_cases(&self) -> Vec<ConnectorHit> {
        self.candidates
            .iter()
            .filter(|candidate| candidate.is_prime)
            .map(|candidate| candidate.hit)
            .collect()
    }

    /// Prime rate after removing the exact residue exclusions.
    pub fn post_filter_prime_rate(&self) -> f64 {
        let admissible = self.residue_admissible_candidates();
        if admissible == 0 {
            0.0
        } else {
            self.prime_hits() as f64 / admissible as f64
        }
    }

    fn working_digits_by_position(&self) -> BTreeMap<(u32, u32), BTreeSet<u8>> {
        let mut map = BTreeMap::new();
        for candidate in self
            .candidates
            .iter()
            .filter(|candidate| candidate.is_prime)
        {
            map.entry((candidate.hit.width, candidate.hit.position))
                .or_insert_with(BTreeSet::new)
                .insert(candidate.hit.digit);
        }
        map
    }

    fn residue_admissible_candidates_iter(&self) -> impl Iterator<Item = &ConnectorCandidate> {
        self.candidates
            .iter()
            .filter(|candidate| candidate.residue_admissible.is_yes())
    }

    /// Naive PNT-style expected hits across residue-admissible candidates.
    pub fn naive_expected_hits(&self) -> f64 {
        self.residue_admissible_candidates_iter()
            .map(ConnectorCandidate::naive_prime_probability)
            .sum()
    }

    /// Per-prime divisibility profile inside the residue-admissible lane.
    pub fn small_prime_profiles(&self, small_primes: &[u32]) -> Vec<SmallPrimeProfile> {
        let admissible: Vec<_> = self.residue_admissible_candidates_iter().collect();
        let admissible_count = admissible.len();

        sanitize_small_primes(small_primes)
            .into_iter()
            .map(|prime| {
                let blocked_candidates = admissible
                    .iter()
                    .filter(|candidate| candidate.divisible_by_small_prime(prime))
                    .count();
                let blocked_share = if admissible_count == 0 {
                    0.0
                } else {
                    blocked_candidates as f64 / admissible_count as f64
                };
                SmallPrimeProfile {
                    prime,
                    blocked_candidates,
                    blocked_share,
                }
            })
            .collect()
    }

    /// Density-aware audit after the exact residue layer.
    pub fn signal_stats(&self, small_primes: &[u32]) -> DirectionSignalStats {
        let small_primes = sanitize_small_primes(small_primes);
        let admissible: Vec<_> = self.residue_admissible_candidates_iter().copied().collect();
        let admissible_count = admissible.len();
        let prime_hits = self.prime_hits();
        let naive_expected_hits: f64 = admissible
            .iter()
            .map(ConnectorCandidate::naive_prime_probability)
            .sum();
        let joint_small_prime_survivors: Vec<_> = admissible
            .iter()
            .copied()
            .filter(|candidate| candidate.survives_small_prime_sieve(&small_primes))
            .collect();
        let joint_small_prime_survivor_count = joint_small_prime_survivors.len();
        let joint_small_prime_survival_share = if admissible_count == 0 {
            0.0
        } else {
            joint_small_prime_survivor_count as f64 / admissible_count as f64
        };
        let random_joint_survival_share = small_primes
            .iter()
            .fold(1.0, |acc, prime| acc * (1.0 - 1.0 / *prime as f64));
        let local_correction_factor = if random_joint_survival_share > 0.0 {
            joint_small_prime_survival_share / random_joint_survival_share
        } else {
            0.0
        };
        let small_prime_corrected_expected_hits = if random_joint_survival_share > 0.0 {
            joint_small_prime_survivors
                .iter()
                .map(ConnectorCandidate::naive_prime_probability)
                .sum::<f64>()
                / random_joint_survival_share
        } else {
            0.0
        };

        DirectionSignalStats {
            direction: self.direction,
            raw_candidates: self.raw_candidates(),
            residue_admissible_candidates: admissible_count,
            prime_hits,
            naive_expected_hits,
            small_primes: small_primes.clone(),
            small_prime_profiles: self.small_prime_profiles(&small_primes),
            joint_small_prime_survivor_count,
            joint_small_prime_survival_share,
            random_joint_survival_share,
            local_correction_factor,
            small_prime_corrected_expected_hits,
            observed_to_naive_ratio: safe_ratio(prime_hits as f64, naive_expected_hits),
            observed_to_corrected_ratio: safe_ratio(
                prime_hits as f64,
                small_prime_corrected_expected_hits,
            ),
            corrected_poisson_residual_z: poisson_residual_z(
                prime_hits as f64,
                small_prime_corrected_expected_hits,
            ),
        }
    }

    /// Width/position buckets for constructive follow-up on residual signal.
    pub fn position_signal_rows(&self, small_primes: &[u32]) -> Vec<PositionSignalRow> {
        let small_primes = sanitize_small_primes(small_primes);
        let mut buckets: BTreeMap<(u32, u32), Vec<ConnectorCandidate>> = BTreeMap::new();

        for candidate in self.residue_admissible_candidates_iter().copied() {
            buckets
                .entry((candidate.hit.width, candidate.hit.position))
                .or_default()
                .push(candidate);
        }

        buckets
            .into_iter()
            .map(|((width, position), candidates)| {
                let prime_hits = candidates
                    .iter()
                    .filter(|candidate| candidate.is_prime)
                    .count();
                let naive_expected_hits: f64 = candidates
                    .iter()
                    .map(ConnectorCandidate::naive_prime_probability)
                    .sum();
                let joint_small_prime_survivors: Vec<_> = candidates
                    .iter()
                    .copied()
                    .filter(|candidate| candidate.survives_small_prime_sieve(&small_primes))
                    .collect();
                let random_joint_survival_share = small_primes
                    .iter()
                    .fold(1.0, |acc, prime| acc * (1.0 - 1.0 / *prime as f64));
                let corrected_expected_hits = if random_joint_survival_share > 0.0 {
                    joint_small_prime_survivors
                        .iter()
                        .map(ConnectorCandidate::naive_prime_probability)
                        .sum::<f64>()
                        / random_joint_survival_share
                } else {
                    0.0
                };

                PositionSignalRow {
                    direction: self.direction,
                    width,
                    position,
                    residue_admissible_candidates: candidates.len(),
                    prime_hits,
                    working_digits: candidates
                        .iter()
                        .filter(|candidate| candidate.is_prime)
                        .map(|candidate| candidate.hit.digit)
                        .collect(),
                    naive_expected_hits,
                    small_prime_corrected_expected_hits: corrected_expected_hits,
                    observed_to_corrected_ratio: safe_ratio(
                        prime_hits as f64,
                        corrected_expected_hits,
                    ),
                }
            })
            .collect()
    }
}

/// Forward/reverse summary after the exact residue layer is accounted for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionalAsymmetry {
    /// Fixed prime pair under study.
    pub pair: ConcatenationSystem,
    /// Forward-direction prime hits.
    pub forward_hit_count: usize,
    /// Reverse-direction prime hits.
    pub reverse_hit_count: usize,
    /// Forward post-filter prime rate.
    pub forward_post_filter_rate: f64,
    /// Reverse post-filter prime rate.
    pub reverse_post_filter_rate: f64,
}

impl DirectionalAsymmetry {
    /// Forward minus reverse prime-hit difference.
    pub fn hit_delta(&self) -> isize {
        self.forward_hit_count as isize - self.reverse_hit_count as isize
    }

    /// Forward minus reverse post-filter prime-rate difference.
    pub fn post_filter_rate_delta(&self) -> f64 {
        self.forward_post_filter_rate - self.reverse_post_filter_rate
    }
}

/// Exact small-prime divisibility profile inside one residue-admissible lane.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmallPrimeProfile {
    /// Small prime used for the exact divisibility check.
    pub prime: u32,
    /// Residue-admissible candidates divisible by this prime.
    pub blocked_candidates: usize,
    /// Blocked share among residue-admissible candidates.
    pub blocked_share: f64,
}

/// Density-aware direction summary after exact residue filtering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionSignalStats {
    /// Direction summarized by this record.
    pub direction: Direction,
    /// Raw width/position/digit candidates in the bounded scan.
    pub raw_candidates: usize,
    /// Candidates that survive the exact residue layer.
    pub residue_admissible_candidates: usize,
    /// Prime hits observed in this direction.
    pub prime_hits: usize,
    /// Baseline expected hits from `sum 1 / ln(n)` over admissible candidates.
    pub naive_expected_hits: f64,
    /// Small primes used for the local correction layer.
    pub small_primes: Vec<u32>,
    /// Per-prime blocked shares within the admissible lane.
    pub small_prime_profiles: Vec<SmallPrimeProfile>,
    /// Admissible candidates surviving every requested small-prime test.
    pub joint_small_prime_survivor_count: usize,
    /// Survivor share inside the admissible lane.
    pub joint_small_prime_survival_share: f64,
    /// Reference survival share for uniformly random integers.
    pub random_joint_survival_share: f64,
    /// Family-specific local correction factor relative to the random baseline.
    pub local_correction_factor: f64,
    /// Expected hits after exact small-prime conditioning.
    pub small_prime_corrected_expected_hits: f64,
    /// Observed/expected ratio against the naive `1 / ln(n)` baseline.
    pub observed_to_naive_ratio: f64,
    /// Observed/expected ratio against the small-prime corrected baseline.
    pub observed_to_corrected_ratio: f64,
    /// Poisson-style residual z-score against the corrected expectation.
    pub corrected_poisson_residual_z: f64,
}

/// Width/position bucket summary for residual-signal follow-up.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionSignalRow {
    /// Direction summarized by this record.
    pub direction: Direction,
    /// Buffer width in decimal digits.
    pub width: u32,
    /// Zero-based digit position counted from the left edge of the buffer.
    pub position: u32,
    /// Residue-admissible candidates in this width/position bucket.
    pub residue_admissible_candidates: usize,
    /// Prime hits found in this width/position bucket.
    pub prime_hits: usize,
    /// Sorted working digits in this bucket.
    pub working_digits: Vec<u8>,
    /// Baseline expected hits from `sum 1 / ln(n)` across the bucket.
    pub naive_expected_hits: f64,
    /// Expected hits after exact small-prime conditioning.
    pub small_prime_corrected_expected_hits: f64,
    /// Observed/expected ratio against the corrected baseline.
    pub observed_to_corrected_ratio: f64,
}

/// Pair-level residual audit used by the maintained connector report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PairSignalAudit {
    /// Fixed prime pair under study.
    pub pair: ConcatenationSystem,
    /// Small primes used for the local correction layer.
    pub small_primes: Vec<u32>,
    /// Forward-direction audit.
    pub forward: DirectionSignalStats,
    /// Reverse-direction audit.
    pub reverse: DirectionSignalStats,
    /// Forward width/position residual buckets.
    pub forward_positions: Vec<PositionSignalRow>,
    /// Reverse width/position residual buckets.
    pub reverse_positions: Vec<PositionSignalRow>,
}

impl PairSignalAudit {
    /// Forward minus reverse residual-ratio gap after the corrected baseline.
    pub fn corrected_residual_ratio_delta(&self) -> f64 {
        self.forward.observed_to_corrected_ratio - self.reverse.observed_to_corrected_ratio
    }

    /// Forward minus reverse corrected expected-hit difference.
    pub fn corrected_expected_hit_delta(&self) -> f64 {
        self.forward.small_prime_corrected_expected_hits
            - self.reverse.small_prime_corrected_expected_hits
    }
}

/// Exact bounded scan summary for one fixed pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PairScanSummary {
    /// Fixed prime pair under study.
    pub pair: ConcatenationSystem,
    /// Buffer widths included in the matched scan.
    pub widths: Vec<u32>,
    /// Digits included in the matched scan.
    pub digits: Vec<u8>,
    /// Exact residue profiles used to prefilter the scan.
    pub residue_profiles: Vec<PairResidueProfile>,
    /// Forward-direction results.
    pub forward: DirectionScanStats,
    /// Reverse-direction results.
    pub reverse: DirectionScanStats,
}

impl PairScanSummary {
    /// Return the per-pair forward/reverse asymmetry summary.
    pub fn directional_asymmetry(&self) -> DirectionalAsymmetry {
        DirectionalAsymmetry {
            pair: self.pair,
            forward_hit_count: self.forward.prime_hits(),
            reverse_hit_count: self.reverse.prime_hits(),
            forward_post_filter_rate: self.forward.post_filter_prime_rate(),
            reverse_post_filter_rate: self.reverse.post_filter_prime_rate(),
        }
    }

    /// Width/position buckets with multiple working digits in one direction.
    pub fn resonance_positions(&self) -> Vec<ResonancePosition> {
        let mut positions = Vec::new();

        for stats in [&self.forward, &self.reverse] {
            for ((width, position), digits) in stats.working_digits_by_position() {
                if digits.len() > 1 {
                    positions.push(ResonancePosition {
                        pair: self.pair,
                        direction: stats.direction,
                        width,
                        position,
                        digits: digits.into_iter().collect(),
                    });
                }
            }
        }

        positions
    }

    /// All prime hits for both directions.
    pub fn all_hits(&self) -> Vec<ConnectorHit> {
        let mut hits = self.forward.hit_cases();
        hits.extend(self.reverse.hit_cases());
        hits
    }

    /// Density-aware residual audit for this fixed pair.
    pub fn signal_audit(&self, small_primes: &[u32]) -> PairSignalAudit {
        PairSignalAudit {
            pair: self.pair,
            small_primes: sanitize_small_primes(small_primes),
            forward: self.forward.signal_stats(small_primes),
            reverse: self.reverse.signal_stats(small_primes),
            forward_positions: self.forward.position_signal_rows(small_primes),
            reverse_positions: self.reverse.position_signal_rows(small_primes),
        }
    }
}

/// Scan matched width/position/digit budgets for one fixed pair.
pub fn scan_single_digit_hits(
    pair: ConcatenationSystem,
    widths: &[u32],
    digits: &[u8],
    residue_moduli: &[u32],
) -> PairScanSummary {
    let residue_profiles: Vec<_> = residue_moduli
        .iter()
        .copied()
        .map(|modulus| PairResidueProfile::new(pair, modulus))
        .collect();

    PairScanSummary {
        pair,
        widths: widths.to_vec(),
        digits: digits.to_vec(),
        forward: scan_direction(pair, widths, digits, &residue_profiles, Direction::Forward),
        reverse: scan_direction(pair, widths, digits, &residue_profiles, Direction::Reverse),
        residue_profiles,
    }
}

/// Canonical forward-direction source hits as checked [`ConnectorHit`] values.
pub fn canonical_source_hits() -> Vec<ConnectorHit> {
    let pair = ConcatenationSystem::new(super::CANONICAL_LEFT, super::CANONICAL_RIGHT);

    CANONICAL_SOURCE_HITS
        .iter()
        .filter_map(|&(width, position, digit)| {
            ConnectorHit::new(pair, width, position, digit, Direction::Forward)
        })
        .collect()
}

/// Return sorted unique prime numbers up to and including `bound`.
pub fn small_primes_up_to(bound: u32) -> Vec<u32> {
    (2..=bound).filter(|&n| is_prime_u32(n)).collect()
}

fn scan_direction(
    pair: ConcatenationSystem,
    widths: &[u32],
    digits: &[u8],
    residue_profiles: &[PairResidueProfile],
    direction: Direction,
) -> DirectionScanStats {
    let mut candidates = Vec::new();

    for &width in widths {
        if width == 0 || !pair.fits_in_u128(width) {
            continue;
        }

        for position in 0..width {
            for &digit in digits {
                let hit = match ConnectorHit::new(pair, width, position, digit, direction) {
                    Some(hit) => hit,
                    None => continue,
                };
                let residue_admissible = hit.residue_admissible(residue_profiles);
                let is_prime = residue_admissible.is_yes() && hit.is_prime();
                candidates.push(ConnectorCandidate {
                    hit,
                    residue_admissible,
                    is_prime,
                });
            }
        }
    }

    DirectionScanStats {
        direction,
        candidates,
    }
}

fn sanitize_small_primes(small_primes: &[u32]) -> Vec<u32> {
    let mut primes: Vec<u32> = small_primes
        .iter()
        .copied()
        .filter(|&prime| is_prime_u32(prime))
        .collect();
    primes.sort_unstable();
    primes.dedup();
    primes
}

fn is_prime_u32(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }

    let mut factor = 3u32;
    while factor.saturating_mul(factor) <= n {
        if n.is_multiple_of(factor) {
            return false;
        }
        factor += 2;
    }

    true
}

fn safe_ratio(numerator: f64, denominator: f64) -> f64 {
    if denominator > 0.0 {
        numerator / denominator
    } else {
        0.0
    }
}

fn poisson_residual_z(observed: f64, expected: f64) -> f64 {
    if expected > 0.0 {
        (observed - expected) / expected.sqrt()
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connector::{CANONICAL_LEFT, CANONICAL_RIGHT};

    #[test]
    fn test_pair_residue_profiles_canonical_pair() {
        let pair = ConcatenationSystem::new(CANONICAL_LEFT, CANONICAL_RIGHT);
        let mod3 = PairResidueProfile::new(pair, 3);
        let mod9 = PairResidueProfile::new(pair, 9);

        assert_eq!(mod3.pair_residue, 1);
        assert_eq!(mod3.blocked_connector_residue, 2);
        assert_eq!(mod9.pair_residue, 1);
        assert_eq!(mod9.blocked_connector_residue, 8);
    }

    #[test]
    fn test_connector_hit_numeric_examples() {
        let pair = ConcatenationSystem::new(CANONICAL_LEFT, CANONICAL_RIGHT);
        let hit = ConnectorHit::new(pair, 5, 4, 6, Direction::Forward).unwrap();
        assert_eq!(hit.connector_value(), Some(6));
        assert_eq!(hit.connector_string().as_deref(), Some("00006"));
        assert_eq!(hit.concatenated_value(), Some(10301000063007003007003u128));

        let left_hit = ConnectorHit::new(pair, 5, 1, 6, Direction::Forward).unwrap();
        assert_eq!(left_hit.connector_value(), Some(6000));
        assert_eq!(left_hit.connector_string().as_deref(), Some("06000"));
        assert_eq!(
            left_hit.concatenated_value(),
            Some(10301060003007003007003u128)
        );
    }

    #[test]
    fn test_canonical_width5_cases_have_expected_values() {
        let pair = ConcatenationSystem::new(CANONICAL_LEFT, CANONICAL_RIGHT);
        let left_case = ConnectorHit::new(pair, 5, 1, 6, Direction::Forward).unwrap();
        let right_case = ConnectorHit::new(pair, 5, 4, 6, Direction::Forward).unwrap();

        assert_eq!(left_case.connector_string().as_deref(), Some("06000"));
        assert_eq!(right_case.connector_string().as_deref(), Some("00006"));
        assert_eq!(
            left_case.concatenated_value(),
            Some(10301060003007003007003u128)
        );
        assert_eq!(
            right_case.concatenated_value(),
            Some(10301000063007003007003u128)
        );
    }

    #[test]
    fn test_documented_forward_cases_have_expected_values() {
        let pair = ConcatenationSystem::new(CANONICAL_LEFT, CANONICAL_RIGHT);
        let width5 = ConnectorHit::new(pair, 5, 4, 6, Direction::Forward).unwrap();
        let width6_left = ConnectorHit::new(pair, 6, 1, 6, Direction::Forward).unwrap();
        let width6_right = ConnectorHit::new(pair, 6, 4, 6, Direction::Forward).unwrap();
        let width7_center = ConnectorHit::new(pair, 7, 3, 6, Direction::Forward).unwrap();

        assert_eq!(width5.connector_string().as_deref(), Some("00006"));
        assert_eq!(width6_left.connector_string().as_deref(), Some("060000"));
        assert_eq!(width6_right.connector_string().as_deref(), Some("000060"));
        assert_eq!(width7_center.connector_string().as_deref(), Some("0006000"));
        assert_eq!(
            width6_left.concatenated_value(),
            Some(103010600003007003007003u128)
        );
        assert_eq!(
            width6_right.concatenated_value(),
            Some(103010000603007003007003u128)
        );
        assert_eq!(
            width7_center.concatenated_value(),
            Some(1030100060003007003007003u128)
        );
    }

    #[test]
    fn test_small_primes_up_to_works() {
        assert_eq!(small_primes_up_to(1), Vec::<u32>::new());
        assert_eq!(small_primes_up_to(13), vec![2, 3, 5, 7, 11, 13]);
    }

    #[test]
    fn test_signal_audit_is_finite_for_canonical_pair() {
        let pair = ConcatenationSystem::new(CANONICAL_LEFT, CANONICAL_RIGHT);
        let summary =
            scan_single_digit_hits(pair, &[5, 6, 7], &[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 9]);
        let audit = summary.signal_audit(DEFAULT_SMALL_PRIMES);

        assert!(audit.forward.naive_expected_hits.is_finite());
        assert!(audit
            .forward
            .small_prime_corrected_expected_hits
            .is_finite());
        assert!(audit.reverse.naive_expected_hits.is_finite());
        assert!(audit
            .reverse
            .small_prime_corrected_expected_hits
            .is_finite());
        assert_eq!(audit.forward_positions.len(), 18);
        assert_eq!(audit.reverse_positions.len(), 18);
    }
}
