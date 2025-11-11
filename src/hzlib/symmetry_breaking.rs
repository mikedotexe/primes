//! Symmetry Breaking Point Analysis
//!
//! Analyzes WHERE in seed space symmetric membrane patterns fail to generate primes.
//! Unlike failure_analysis.rs which analyzes configurations, this module tracks
//! seed-level patterns: which seeds work, which fail, and where failures cluster.
//!
//! # Core Concept
//!
//! A membrane configuration (base, outer, inner, k) combined with different seeds
//! produces different primality results. This module asks:
//!
//! - Do failures cluster at specific seed values?
//! - Are there "dark zones" where patterns consistently break?
//! - Can we predict failure points from seed properties?
//! - Do failures follow modular arithmetic patterns?
//!
//! # Example
//!
//! ```text
//! Base 6, (1,5) k=(0,0):
//!   Seed 0: ✗ composite    ← failure
//!   Seed 1: ✓ prime
//!   Seed 2: ✗ composite    ← failure
//!   Seed 3: ✗ composite    ← failure cluster?
//!   Seed 4: ✓ prime
//!   Seed 5: ✓ prime
//!
//! Analysis: Failures at seeds 0,2,3 - is there a pattern?
//! ```

use std::collections::HashMap;

/// Tracks success/failure for individual seeds in a configuration
#[derive(Clone, Debug)]
pub struct SymmetryBreaker {
    pub base: usize,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,

    /// Map from seed value to primality result
    pub seed_results: HashMap<u32, bool>,

    /// Cached statistics
    success_count: usize,
    failure_count: usize,
}

impl SymmetryBreaker {
    /// Create new symmetry breaker for a configuration
    pub fn new(base: usize, outer: u32, inner: u32, k_outer: u32, k_inner: u32) -> Self {
        Self {
            base,
            outer,
            inner,
            k_outer,
            k_inner,
            seed_results: HashMap::new(),
            success_count: 0,
            failure_count: 0,
        }
    }

    /// Record result for a seed
    pub fn record_seed(&mut self, seed: u32, is_prime: bool) {
        self.seed_results.insert(seed, is_prime);

        if is_prime {
            self.success_count += 1;
        } else {
            self.failure_count += 1;
        }
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        let total = self.success_count + self.failure_count;
        if total == 0 {
            0.0
        } else {
            self.success_count as f64 / total as f64
        }
    }

    /// Get total seeds tested
    pub fn total_tested(&self) -> usize {
        self.success_count + self.failure_count
    }

    /// Get success count
    pub fn success_count(&self) -> usize {
        self.success_count
    }

    /// Get failure count
    pub fn failure_count(&self) -> usize {
        self.failure_count
    }

    /// Find failure clusters (consecutive failures)
    ///
    /// Returns Vec<(start_seed, length)> of failure runs
    pub fn find_failure_clusters(&self) -> Vec<(u32, usize)> {
        let mut sorted_seeds: Vec<_> = self.seed_results.keys().copied().collect();
        sorted_seeds.sort_unstable();

        let mut clusters = Vec::new();
        let mut cluster_start: Option<u32> = None;
        let mut cluster_len = 0usize;

        for &seed in &sorted_seeds {
            let is_prime = self.seed_results.get(&seed).copied().unwrap_or(false);

            if !is_prime {
                // Failure
                if cluster_start.is_none() {
                    cluster_start = Some(seed);
                    cluster_len = 1;
                } else {
                    cluster_len += 1;
                }
            } else if let Some(start) = cluster_start {
                // End of cluster
                if cluster_len >= 2 {
                    clusters.push((start, cluster_len));
                }
                cluster_start = None;
                cluster_len = 0;
            }
        }

        // Catch final cluster
        if let Some(start) = cluster_start {
            if cluster_len >= 2 {
                clusters.push((start, cluster_len));
            }
        }

        clusters
    }

    /// Analyze modular patterns in failures
    ///
    /// Tests if failures correlate with seed mod m for various m
    pub fn modular_failure_pattern(&self) -> Vec<(usize, f64)> {
        let moduli = [2, 3, 4, 5, 6, 7, 8, 10];
        let mut patterns = Vec::new();

        for &m in &moduli {
            let mut residue_failures = vec![0usize; m];
            let mut residue_totals = vec![0usize; m];

            for (&seed, &is_prime) in &self.seed_results {
                let residue = (seed as usize) % m;
                residue_totals[residue] += 1;

                if !is_prime {
                    residue_failures[residue] += 1;
                }
            }

            // Calculate variance in failure rates across residues
            let failure_rates: Vec<f64> = residue_totals
                .iter()
                .zip(residue_failures.iter())
                .map(|(&total, &failures)| {
                    if total == 0 {
                        0.0
                    } else {
                        failures as f64 / total as f64
                    }
                })
                .collect();

            // Calculate variance
            let mean_rate: f64 = failure_rates.iter().sum::<f64>() / failure_rates.len() as f64;
            let variance: f64 = failure_rates
                .iter()
                .map(|&rate| (rate - mean_rate).powi(2))
                .sum::<f64>()
                / failure_rates.len() as f64;

            patterns.push((m, variance));
        }

        // Sort by variance (highest first = strongest pattern)
        patterns.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        patterns
    }

    /// Find "dark zones" - seeds that always fail
    ///
    /// Returns list of consistently failing seeds
    pub fn find_dark_zones(&self) -> Vec<u32> {
        self.seed_results
            .iter()
            .filter(|(_, &is_prime)| !is_prime)
            .map(|(&seed, _)| seed)
            .collect()
    }

    /// Find "bright zones" - seeds that always succeed
    ///
    /// Returns list of consistently succeeding seeds
    pub fn find_bright_zones(&self) -> Vec<u32> {
        self.seed_results
            .iter()
            .filter(|(_, &is_prime)| is_prime)
            .map(|(&seed, _)| seed)
            .collect()
    }

    /// Analyze digit properties of failing seeds
    ///
    /// Returns map of digit property to failure rate correlation
    pub fn digit_property_correlation(&self) -> HashMap<String, f64> {
        let mut correlations = HashMap::new();

        // Test various digit properties
        type PropertyTest = Box<dyn Fn(u32) -> bool>;
        let properties: Vec<(&str, PropertyTest)> = vec![
            ("even", Box::new(|seed| seed % 2 == 0)),
            ("divisible_by_3", Box::new(|seed| seed % 3 == 0)),
            ("divisible_by_5", Box::new(|seed| seed % 5 == 0)),
            (
                "digit_sum_even",
                Box::new(|seed| {
                    seed.to_string()
                        .chars()
                        .filter_map(|c| c.to_digit(10))
                        .sum::<u32>()
                        % 2
                        == 0
                }),
            ),
            (
                "palindrome_digit",
                Box::new(|seed| {
                    let s = seed.to_string();
                    s == s.chars().rev().collect::<String>()
                }),
            ),
        ];

        for (prop_name, predicate) in properties {
            let mut with_property_failures = 0usize;
            let mut with_property_total = 0usize;
            let mut without_property_failures = 0usize;
            let mut without_property_total = 0usize;

            for (&seed, &is_prime) in &self.seed_results {
                if predicate(seed) {
                    with_property_total += 1;
                    if !is_prime {
                        with_property_failures += 1;
                    }
                } else {
                    without_property_total += 1;
                    if !is_prime {
                        without_property_failures += 1;
                    }
                }
            }

            let rate_with = if with_property_total > 0 {
                with_property_failures as f64 / with_property_total as f64
            } else {
                0.0
            };

            let rate_without = if without_property_total > 0 {
                without_property_failures as f64 / without_property_total as f64
            } else {
                0.0
            };

            // Correlation: positive means property correlates with failure
            let correlation = rate_with - rate_without;
            correlations.insert(prop_name.to_string(), correlation);
        }

        correlations
    }

    /// Configuration summary string
    pub fn config_summary(&self) -> String {
        format!(
            "Base {} ({},{}) k=({},{})",
            self.base, self.outer, self.inner, self.k_outer, self.k_inner
        )
    }
}

/// Accumulator for analyzing symmetry breaking across multiple configurations
pub struct BreakingAccumulator {
    pub breakers: Vec<SymmetryBreaker>,
}

impl BreakingAccumulator {
    pub fn new() -> Self {
        Self {
            breakers: Vec::new(),
        }
    }

    /// Add a breaker to the accumulator
    pub fn add_breaker(&mut self, breaker: SymmetryBreaker) {
        self.breakers.push(breaker);
    }

    /// Find universal failure seeds (fail in all configs)
    pub fn universal_failures(&self) -> Vec<u32> {
        if self.breakers.is_empty() {
            return Vec::new();
        }

        // Get all seeds tested in first breaker
        let first_seeds: Vec<u32> = self.breakers[0].seed_results.keys().copied().collect();

        // Filter to seeds that fail in ALL breakers
        first_seeds
            .into_iter()
            .filter(|&seed| {
                self.breakers.iter().all(|breaker| {
                    breaker
                        .seed_results
                        .get(&seed)
                        .map(|&is_prime| !is_prime)
                        .unwrap_or(false)
                })
            })
            .collect()
    }

    /// Find universal success seeds (succeed in all configs)
    pub fn universal_successes(&self) -> Vec<u32> {
        if self.breakers.is_empty() {
            return Vec::new();
        }

        let first_seeds: Vec<u32> = self.breakers[0].seed_results.keys().copied().collect();

        first_seeds
            .into_iter()
            .filter(|&seed| {
                self.breakers
                    .iter()
                    .all(|breaker| breaker.seed_results.get(&seed).copied().unwrap_or(false))
            })
            .collect()
    }

    /// Calculate average success rate across all breakers
    pub fn average_success_rate(&self) -> f64 {
        if self.breakers.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.breakers.iter().map(|b| b.success_rate()).sum();
        sum / self.breakers.len() as f64
    }
}

impl Default for BreakingAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetry_breaker_basic() {
        let mut breaker = SymmetryBreaker::new(6, 1, 5, 0, 0);

        breaker.record_seed(1, true);
        breaker.record_seed(2, false);
        breaker.record_seed(3, true);

        assert_eq!(breaker.success_count, 2);
        assert_eq!(breaker.failure_count, 1);
        assert!((breaker.success_rate() - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_find_failure_clusters() {
        let mut breaker = SymmetryBreaker::new(10, 3, 7, 0, 0);

        // Cluster: 2,3,4 all fail
        breaker.record_seed(1, true);
        breaker.record_seed(2, false);
        breaker.record_seed(3, false);
        breaker.record_seed(4, false);
        breaker.record_seed(5, true);

        let clusters = breaker.find_failure_clusters();
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0], (2, 3)); // Start at 2, length 3
    }

    #[test]
    fn test_dark_zones() {
        let mut breaker = SymmetryBreaker::new(7, 2, 5, 0, 0);

        breaker.record_seed(1, true);
        breaker.record_seed(2, false);
        breaker.record_seed(3, false);
        breaker.record_seed(4, true);

        let dark = breaker.find_dark_zones();
        assert_eq!(dark.len(), 2);
        assert!(dark.contains(&2));
        assert!(dark.contains(&3));
    }
}
