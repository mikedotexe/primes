//! Variance Tracking for Prime Generation
//! ======================================
//!
//! Tracks how prime generation varies with different seeds and initial conditions

use crate::membrane::MembraneConfig;
use crate::PhysicsResult;
use num_bigint::BigUint;
use std::collections::HashMap;

/// Tracks variance in prime generation across multiple runs
#[derive(Debug)]
pub struct VarianceTracker {
    /// Configuration being tested
    pub config: MembraneConfig,

    /// Number of runs per seed
    pub runs_per_seed: usize,

    /// Seeds to test
    pub seed_range: std::ops::Range<u8>,

    /// Results: seed -> (successes, generated_primes)
    pub results: HashMap<u8, (usize, Vec<BigUint>)>,

    /// Unique primes found
    pub unique_primes: HashMap<String, usize>, // prime_string -> count
}

impl VarianceTracker {
    pub fn new(
        config: MembraneConfig,
        runs_per_seed: usize,
        seed_range: std::ops::Range<u8>,
    ) -> Self {
        Self {
            config,
            runs_per_seed,
            seed_range,
            results: HashMap::new(),
            unique_primes: HashMap::new(),
        }
    }

    /// Run variance analysis
    pub fn analyze(&mut self) -> PhysicsResult<VarianceReport> {
        // Test each seed
        for seed in self.seed_range.clone() {
            let mut successes = 0;
            let mut primes = Vec::new();

            for _ in 0..self.runs_per_seed {
                let builder =
                    crate::membrane::MembraneBuilder::new(self.config.clone()).with_seed(seed);

                if let Ok(particle) = builder.build() {
                    successes += 1;
                    let prime_str = particle.value.to_string();

                    // Track unique primes
                    *self.unique_primes.entry(prime_str.clone()).or_insert(0) += 1;
                    primes.push(particle.value);
                }
            }

            self.results.insert(seed, (successes, primes));
        }

        // Calculate statistics
        let total_runs = self.results.len() * self.runs_per_seed;
        let total_successes: usize = self.results.values().map(|(s, _)| *s).sum();

        let success_rate = total_successes as f64 / total_runs as f64;

        // Calculate per-seed success rates
        let seed_rates: Vec<f64> = self
            .results
            .values()
            .map(|(s, _)| *s as f64 / self.runs_per_seed as f64)
            .collect();

        // Calculate variance
        let mean_rate = seed_rates.iter().sum::<f64>() / seed_rates.len() as f64;
        let variance = seed_rates
            .iter()
            .map(|r| (r - mean_rate).powi(2))
            .sum::<f64>()
            / seed_rates.len() as f64;

        let std_dev = variance.sqrt();

        // Find best and worst seeds
        let mut seed_performance: Vec<(u8, f64)> = self
            .results
            .iter()
            .map(|(seed, (successes, _))| (*seed, *successes as f64 / self.runs_per_seed as f64))
            .collect();
        seed_performance.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let best_seeds: Vec<(u8, f64)> = seed_performance.iter().take(3).cloned().collect();

        let worst_seeds: Vec<(u8, f64)> = seed_performance.iter().rev().take(3).cloned().collect();

        Ok(VarianceReport {
            config_summary: self.config.summary(),
            total_runs,
            total_successes,
            success_rate,
            unique_primes_count: self.unique_primes.len(),
            mean_success_rate: mean_rate,
            std_deviation: std_dev,
            best_seeds,
            worst_seeds,
            prime_diversity: self.calculate_diversity(),
        })
    }

    /// Calculate diversity metric (0 = all same, 1 = all different)
    fn calculate_diversity(&self) -> f64 {
        if self.unique_primes.is_empty() {
            return 0.0;
        }

        let total_primes: usize = self.unique_primes.values().sum();
        let _unique_count = self.unique_primes.len();

        if total_primes <= 1 {
            return 0.0;
        }

        // Shannon diversity index
        let mut entropy = 0.0;
        for count in self.unique_primes.values() {
            let p = *count as f64 / total_primes as f64;
            if p > 0.0 {
                entropy -= p * p.ln();
            }
        }

        // Normalize to [0, 1]
        let max_entropy = (total_primes as f64).ln();
        if max_entropy > 0.0 {
            entropy / max_entropy
        } else {
            0.0
        }
    }
}

/// Report from variance analysis
#[derive(Debug)]
pub struct VarianceReport {
    pub config_summary: String,
    pub total_runs: usize,
    pub total_successes: usize,
    pub success_rate: f64,
    pub unique_primes_count: usize,
    pub mean_success_rate: f64,
    pub std_deviation: f64,
    pub best_seeds: Vec<(u8, f64)>,
    pub worst_seeds: Vec<(u8, f64)>,
    pub prime_diversity: f64,
}

impl VarianceReport {
    /// Generate a detailed report string
    pub fn format_report(&self) -> String {
        let mut report = String::new();

        report.push_str("Variance Analysis Report\n");
        report.push_str("========================\n");
        report.push_str(&format!("Configuration: {}\n", self.config_summary));
        report.push('\n');
        report.push_str("Overall Statistics:\n");
        report.push_str(&format!("  Total runs: {}\n", self.total_runs));
        report.push_str(&format!(
            "  Successful generations: {} ({:.1}%)\n",
            self.total_successes,
            self.success_rate * 100.0
        ));
        report.push_str(&format!(
            "  Unique primes found: {}\n",
            self.unique_primes_count
        ));
        report.push_str(&format!(
            "  Prime diversity index: {:.3}\n",
            self.prime_diversity
        ));
        report.push('\n');
        report.push_str("Seed Performance:\n");
        report.push_str(&format!(
            "  Mean success rate: {:.1}%\n",
            self.mean_success_rate * 100.0
        ));
        report.push_str(&format!(
            "  Standard deviation: {:.1}%\n",
            self.std_deviation * 100.0
        ));
        report.push('\n');
        report.push_str("  Best seeds:\n");
        for (seed, rate) in &self.best_seeds {
            report.push_str(&format!(
                "    Seed {}: {:.1}% success\n",
                seed,
                rate * 100.0
            ));
        }
        report.push('\n');
        report.push_str("  Worst seeds:\n");
        for (seed, rate) in &self.worst_seeds {
            report.push_str(&format!(
                "    Seed {}: {:.1}% success\n",
                seed,
                rate * 100.0
            ));
        }

        report
    }
}
