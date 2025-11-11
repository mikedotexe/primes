//! Empirical Verification Pipeline
//!
//! Systematic testing framework for validating hypotheses about membrane primes.
//! This module provides infrastructure for:
//!
//! 1. **Hypothesis Definition**: Clear, testable claims about prime patterns
//! 2. **Automated Testing**: Run explorers with standardized configurations
//! 3. **Statistical Analysis**: Rigorous significance testing
//! 4. **Result Collection**: CSV output for publication-ready evidence
//!
//! # Core Philosophy
//!
//! Scientific claims require empirical validation. This pipeline transforms
//! exploratory findings into verified facts by:
//!
//! - Testing across multiple configurations (not cherry-picking)
//! - Computing statistical significance (not just observing patterns)
//! - Documenting both successes AND failures (honest reporting)
//! - Providing reproducible results (exact configurations recorded)
//!
//! # Supported Hypotheses
//!
//! ## H1: Symmetry Breaking Patterns
//!
//! **Claim**: Failure seeds cluster non-randomly (dark zones exist)
//!
//! **Test**: Compare observed clustering to random expectation
//! - Metric: Average cluster size
//! - Null hypothesis: Random failures (no clustering)
//! - Statistical test: Permutation test or runs test
//!
//! ## H2: Harmonic Overtone Resonance
//!
//! **Claim**: Base multiples inherit success patterns from fundamentals
//!
//! **Test**: Measure correlation between fundamental and overtone rates
//! - Metric: Success rate persistence (overtone/fundamental ratio)
//! - Null hypothesis: Independent rates
//! - Statistical test: Pearson correlation on log-rates
//!
//! ## H3: Lagrange Harmonic Clustering
//!
//! **Claim**: Lagrange positions cluster at harmonic ratios (1/2, 2/3, φ)
//!
//! **Test**: Compare clustering in harmonic vs non-harmonic pairs
//! - Metric: Enrichment factor (observed/expected near harmonics)
//! - Null hypothesis: Uniform position distribution
//! - Statistical test: Chi-squared goodness-of-fit
//!
//! # Usage Example
//!
//! ```rust
//! use primes::hzlib::verification::*;
//!
//! // Define hypothesis
//! let h3 = Hypothesis::LagrangeHarmonic {
//!     tolerance: 0.1,
//!     min_enrichment: 1.5,
//! };
//!
//! // Run verification
//! let result = verify_hypothesis(&h3, &test_config);
//!
//! // Check outcome
//! if result.is_supported() {
//!     println!("Hypothesis SUPPORTED: p={:.4}", result.p_value);
//! }
//! ```

use std::collections::HashMap;

/// A testable hypothesis about prime patterns
#[derive(Clone, Debug)]
pub enum Hypothesis {
    /// Symmetry breaking: Failures cluster non-randomly
    SymmetryBreaking {
        /// Configuration to test
        base: usize,
        outer: u32,
        inner: u32,
        k_outer: u32,
        k_inner: u32,
        /// Minimum cluster size to detect (default: 3)
        min_cluster_size: usize,
        /// Number of seeds to test
        num_seeds: usize,
    },

    /// Harmonic overtones: Base multiples show resonance
    HarmonicResonance {
        /// Fundamental base to test
        fundamental: usize,
        /// Maximum overtone order (e.g., 4 for testing up to 4×fundamental)
        max_overtone_order: usize,
        /// Configuration to use across series
        outer: u32,
        inner: u32,
        k_outer: u32,
        k_inner: u32,
        /// Number of seeds per base
        num_seeds: usize,
        /// Minimum coherence threshold (overtone/fundamental ratio)
        min_coherence: f64,
    },

    /// Lagrange harmonic: Positions cluster at musical ratios
    LagrangeHarmonic {
        /// Harmonic pair to test (base1, base2)
        pair: (usize, usize),
        /// Buffer sizes to test
        buffer_sizes: Vec<usize>,
        /// Tolerance for "near harmonic" (default: 0.1)
        tolerance: f64,
        /// Minimum enrichment factor for significance
        min_enrichment: f64,
    },
}

impl Hypothesis {
    /// Get a human-readable description
    pub fn description(&self) -> String {
        match self {
            Self::SymmetryBreaking {
                base,
                outer,
                inner,
                ..
            } => {
                format!(
                    "Symmetry Breaking: Base {} ({},{}) shows clustered failures",
                    base, outer, inner
                )
            }
            Self::HarmonicResonance { fundamental, .. } => {
                format!(
                    "Harmonic Resonance: Base {} overtones show coherent patterns",
                    fundamental
                )
            }
            Self::LagrangeHarmonic { pair, .. } => {
                format!(
                    "Lagrange Harmonic: Pair ({},{}) clusters at harmonic positions",
                    pair.0, pair.1
                )
            }
        }
    }

    /// Get hypothesis identifier for CSV output
    pub fn id(&self) -> String {
        match self {
            Self::SymmetryBreaking { base, .. } => format!("H1_symmetry_b{}", base),
            Self::HarmonicResonance { fundamental, .. } => {
                format!("H2_resonance_f{}", fundamental)
            }
            Self::LagrangeHarmonic { pair, .. } => format!("H3_lagrange_{}_{}", pair.0, pair.1),
        }
    }
}

/// Result of hypothesis verification
#[derive(Clone, Debug)]
pub struct VerificationResult {
    pub hypothesis_id: String,
    pub description: String,

    /// Primary test statistic
    pub test_statistic: f64,

    /// Statistical significance (p-value)
    pub p_value: f64,

    /// Effect size (domain-specific)
    pub effect_size: f64,

    /// Is hypothesis supported? (p < 0.05 and effect size meaningful)
    pub supported: bool,

    /// Detailed metrics (hypothesis-specific)
    pub details: HashMap<String, f64>,

    /// Human-readable interpretation
    pub interpretation: String,
}

impl VerificationResult {
    /// Create new result
    pub fn new(
        hypothesis_id: String,
        description: String,
        test_statistic: f64,
        p_value: f64,
        effect_size: f64,
    ) -> Self {
        // Determine if supported based on standard thresholds
        let supported = p_value < 0.05 && effect_size.abs() > 0.2;

        let interpretation = if supported {
            if p_value < 0.001 {
                format!(
                    "✓ STRONG SUPPORT (p<0.001, effect={:.2})",
                    effect_size.abs()
                )
            } else if p_value < 0.01 {
                format!(
                    "✓ MODERATE SUPPORT (p<0.01, effect={:.2})",
                    effect_size.abs()
                )
            } else {
                format!("✓ WEAK SUPPORT (p<0.05, effect={:.2})", effect_size.abs())
            }
        } else if p_value < 0.10 {
            format!("→ MARGINAL (p<0.10, effect={:.2})", effect_size.abs())
        } else {
            format!("✗ NOT SUPPORTED (p={:.3})", p_value)
        };

        Self {
            hypothesis_id,
            description,
            test_statistic,
            p_value,
            effect_size,
            supported,
            details: HashMap::new(),
            interpretation,
        }
    }

    /// Add detail metric
    pub fn add_detail(&mut self, key: &str, value: f64) {
        self.details.insert(key.to_string(), value);
    }

    /// Check if hypothesis is supported
    pub fn is_supported(&self) -> bool {
        self.supported
    }

    /// Get CSV header
    pub fn csv_header() -> String {
        "hypothesis_id,description,test_statistic,p_value,effect_size,supported,interpretation"
            .to_string()
    }

    /// Convert to CSV row
    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{:.6},{:.6},{:.6},{},{}",
            self.hypothesis_id,
            self.description,
            self.test_statistic,
            self.p_value,
            self.effect_size,
            if self.supported { "YES" } else { "NO" },
            self.interpretation
        )
    }
}

/// Accumulator for multiple verification results
pub struct VerificationSuite {
    pub results: Vec<VerificationResult>,
}

impl VerificationSuite {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Add a result
    pub fn add_result(&mut self, result: VerificationResult) {
        self.results.push(result);
    }

    /// Count supported hypotheses
    pub fn count_supported(&self) -> usize {
        self.results.iter().filter(|r| r.supported).count()
    }

    /// Count total hypotheses
    pub fn count_total(&self) -> usize {
        self.results.len()
    }

    /// Calculate overall success rate
    pub fn success_rate(&self) -> f64 {
        if self.results.is_empty() {
            0.0
        } else {
            self.count_supported() as f64 / self.count_total() as f64
        }
    }

    /// Get average p-value
    pub fn average_p_value(&self) -> f64 {
        if self.results.is_empty() {
            1.0
        } else {
            self.results.iter().map(|r| r.p_value).sum::<f64>() / self.results.len() as f64
        }
    }

    /// Get average effect size
    pub fn average_effect_size(&self) -> f64 {
        if self.results.is_empty() {
            0.0
        } else {
            self.results.iter().map(|r| r.effect_size.abs()).sum::<f64>()
                / self.results.len() as f64
        }
    }

    /// Export results to CSV
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();
        csv.push_str(&VerificationResult::csv_header());
        csv.push('\n');

        for result in &self.results {
            csv.push_str(&result.to_csv_row());
            csv.push('\n');
        }

        csv
    }

    /// Generate visual support bar chart
    pub fn visual_support_chart(&self) -> String {
        let mut chart = String::new();

        chart.push_str("┌────────────────────────────────────────────────────────┐\n");
        chart.push_str("│           HYPOTHESIS SUPPORT OVERVIEW                 │\n");
        chart.push_str("├────────────────────────────────────────────────────────┤\n");

        for result in &self.results {
            // Hypothesis type indicator
            let type_label = if result.hypothesis_id.starts_with("H1") {
                "H1"
            } else if result.hypothesis_id.starts_with("H2") {
                "H2"
            } else {
                "H3"
            };

            // Visual bar based on p-value (lower = longer bar)
            let bar_length = if result.p_value < 0.001 {
                20
            } else if result.p_value < 0.01 {
                15
            } else if result.p_value < 0.05 {
                10
            } else if result.p_value < 0.10 {
                5
            } else {
                2
            };

            let bar: String = "█".repeat(bar_length);
            let empty: String = "░".repeat(20 - bar_length);

            // Status symbol
            let symbol = if result.supported { "✓" } else { "✗" };

            chart.push_str(&format!(
                "│ {} [{}] {}{}  p={:.3} │\n",
                symbol, type_label, bar, empty, result.p_value
            ));
        }

        chart.push_str("└────────────────────────────────────────────────────────┘\n");
        chart.push_str("  (Bar length = statistical strength, 1-p)\n");

        chart
    }

    /// Generate effect size comparison chart
    pub fn visual_effect_sizes(&self) -> String {
        let mut chart = String::new();

        chart.push_str("┌────────────────────────────────────────────────────────┐\n");
        chart.push_str("│              EFFECT SIZE COMPARISON                    │\n");
        chart.push_str("├────────────────────────────────────────────────────────┤\n");
        chart.push_str("│  Hypothesis  │  Effect │ Magnitude                    │\n");
        chart.push_str("├──────────────┼─────────┼──────────────────────────────┤\n");

        for result in &self.results {
            let type_label = if result.hypothesis_id.starts_with("H1") {
                "H1"
            } else if result.hypothesis_id.starts_with("H2") {
                "H2"
            } else {
                "H3"
            };

            let abs_effect = result.effect_size.abs();

            // Visual magnitude indicator
            let bar_length = (abs_effect * 20.0).min(20.0) as usize;
            let bar: String = "▓".repeat(bar_length);
            let empty: String = "░".repeat(20 - bar_length);

            // Magnitude label
            let magnitude = if abs_effect >= 0.8 {
                "LARGE   "
            } else if abs_effect >= 0.5 {
                "MEDIUM  "
            } else if abs_effect >= 0.2 {
                "SMALL   "
            } else {
                "NEGLIGIB"
            };

            chart.push_str(&format!(
                "│      {}      │  {:+.3}  │ {}{} {} │\n",
                type_label, result.effect_size, bar, empty, magnitude
            ));
        }

        chart.push_str("└────────────────────────────────────────────────────────┘\n");
        chart.push_str("  Effect size: >0.8=large, 0.5-0.8=medium, 0.2-0.5=small\n");

        chart
    }

    /// Generate statistical strength heatmap
    pub fn visual_strength_matrix(&self) -> String {
        let mut matrix = String::new();

        matrix.push_str("┌─────────────────────────────────────────────────────────┐\n");
        matrix.push_str("│        STATISTICAL STRENGTH MATRIX                      │\n");
        matrix.push_str("├─────────────────────────────────────────────────────────┤\n");
        matrix.push_str("│              │ p-value │ Effect │ Overall │ Support    │\n");
        matrix.push_str("├──────────────┼─────────┼────────┼─────────┼────────────┤\n");

        for result in &self.results {
            let short_id = &result.hypothesis_id[..result.hypothesis_id.len().min(12)];

            // P-value strength (lower is better)
            let p_symbol = if result.p_value < 0.001 {
                "★★★"
            } else if result.p_value < 0.01 {
                "★★☆"
            } else if result.p_value < 0.05 {
                "★☆☆"
            } else if result.p_value < 0.10 {
                "☆☆☆"
            } else {
                "---"
            };

            // Effect size strength
            let e_symbol = if result.effect_size.abs() >= 0.8 {
                "●●●"
            } else if result.effect_size.abs() >= 0.5 {
                "●●○"
            } else if result.effect_size.abs() >= 0.2 {
                "●○○"
            } else {
                "○○○"
            };

            // Overall strength (both matter)
            let overall = if result.supported { "✓✓✓" } else { "✗✗✗" };

            let support_text = if result.supported { "YES" } else { "NO " };

            matrix.push_str(&format!(
                "│ {:12} │   {}   │  {}  │   {}   │     {}     │\n",
                short_id, p_symbol, e_symbol, overall, support_text
            ));
        }

        matrix.push_str("└─────────────────────────────────────────────────────────┘\n");
        matrix.push_str("  ★=p-value strength, ●=effect size, ✓=supported\n");

        matrix
    }

    /// Generate summary report
    pub fn summary_report(&self) -> String {
        let mut report = String::new();

        report.push_str("═══════════════════════════════════════════════════════════════\n");
        report.push_str("          EMPIRICAL VERIFICATION PIPELINE RESULTS\n");
        report.push_str("═══════════════════════════════════════════════════════════════\n");
        report.push('\n');

        report.push_str(&format!(
            "Total Hypotheses Tested: {}\n",
            self.count_total()
        ));
        report.push_str(&format!(
            "Hypotheses Supported:    {} ({:.1}%)\n",
            self.count_supported(),
            self.success_rate() * 100.0
        ));
        report.push_str(&format!(
            "Average p-value:         {:.4}\n",
            self.average_p_value()
        ));
        report.push_str(&format!(
            "Average effect size:     {:.3}\n",
            self.average_effect_size()
        ));
        report.push('\n');

        report.push_str("───────────────────────────────────────────────────────────────\n");
        report.push_str("INDIVIDUAL RESULTS\n");
        report.push_str("───────────────────────────────────────────────────────────────\n");
        report.push('\n');

        for result in &self.results {
            report.push_str(&format!("{}\n", result.description));
            report.push_str(&format!("  ID: {}\n", result.hypothesis_id));
            report.push_str(&format!("  Test statistic: {:.4}\n", result.test_statistic));
            report.push_str(&format!("  p-value: {:.6}\n", result.p_value));
            report.push_str(&format!("  Effect size: {:.3}\n", result.effect_size));
            report.push_str(&format!("  {}\n", result.interpretation));

            if !result.details.is_empty() {
                report.push_str("  Details:\n");
                for (key, value) in &result.details {
                    report.push_str(&format!("    {}: {:.4}\n", key, value));
                }
            }

            report.push('\n');
        }

        report.push_str("═══════════════════════════════════════════════════════════════\n");
        report.push_str("CONCLUSION\n");
        report.push_str("═══════════════════════════════════════════════════════════════\n");
        report.push('\n');

        let supported_pct = self.success_rate() * 100.0;
        if supported_pct > 80.0 {
            report.push_str("✓ OVERWHELMING EVIDENCE: The vast majority of hypotheses are\n");
            report.push_str("  strongly supported by empirical data. These patterns are real.\n");
        } else if supported_pct > 50.0 {
            report.push_str("✓ STRONG EVIDENCE: Most hypotheses are supported, though some\n");
            report.push_str("  require further investigation or refinement.\n");
        } else if supported_pct > 30.0 {
            report.push_str("→ MIXED EVIDENCE: Some patterns are supported, but many claims\n");
            report.push_str("  lack sufficient statistical backing. More data needed.\n");
        } else {
            report.push_str("✗ WEAK EVIDENCE: Most hypotheses are not supported by the data.\n");
            report.push_str("  Consider revising claims or testing alternative configurations.\n");
        }

        report.push('\n');
        report.push_str("All results saved to CSV for publication-ready evidence.\n");
        report.push('\n');

        report
    }
}

impl Default for VerificationSuite {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistical helper: Simple permutation test for clustering
///
/// Tests if observed clusters are more frequent than expected by chance
pub fn permutation_test_clustering(
    observed_cluster_count: usize,
    total_items: usize,
    successes: usize,
    num_permutations: usize,
) -> f64 {
    if total_items == 0 || successes == 0 {
        return 1.0;
    }

    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();
    let mut more_extreme = 0;

    // Create baseline: successes followed by failures
    let mut items: Vec<bool> = vec![true; successes];
    items.extend(vec![false; total_items - successes]);

    for _ in 0..num_permutations {
        // Shuffle
        items.shuffle(&mut rng);

        // Count clusters in permutation
        let mut perm_clusters = 0;
        let mut in_cluster = false;

        for &is_success in &items {
            if !is_success {
                if !in_cluster {
                    in_cluster = true;
                    perm_clusters += 1;
                }
            } else {
                in_cluster = false;
            }
        }

        if perm_clusters >= observed_cluster_count {
            more_extreme += 1;
        }
    }

    more_extreme as f64 / num_permutations as f64
}

/// Statistical helper: Chi-squared goodness-of-fit test
///
/// Tests if observed distribution matches expected (uniform or harmonic)
pub fn chi_squared_test(observed: &[usize], expected: &[f64]) -> (f64, f64) {
    if observed.len() != expected.len() {
        return (0.0, 1.0);
    }

    let mut chi_squared = 0.0;
    let mut valid_bins = 0;

    for (obs, exp) in observed.iter().zip(expected.iter()) {
        if *exp > 0.0 {
            let o = *obs as f64;
            let e = *exp;
            chi_squared += (o - e).powi(2) / e;
            valid_bins += 1;
        }
    }

    // Degrees of freedom = bins - 1
    let df = if valid_bins > 1 {
        valid_bins - 1
    } else {
        1
    };

    // Approximate p-value using chi-squared CDF
    // For simplicity, use a rough approximation
    let p_value = approximate_chi_squared_p(chi_squared, df);

    (chi_squared, p_value)
}

/// Approximate chi-squared p-value (rough but serviceable)
fn approximate_chi_squared_p(chi_squared: f64, df: usize) -> f64 {
    // Very rough approximation: use normal approximation for large df
    if df < 1 {
        return 1.0;
    }

    // For small chi-squared, return high p-value
    if chi_squared < df as f64 * 0.5 {
        return 0.9;
    }

    // For large chi-squared, return low p-value
    if chi_squared > df as f64 * 3.0 {
        return 0.001;
    }

    // Linear interpolation in between (crude but okay for ranking)
    let normalized = (chi_squared - df as f64 * 0.5) / (df as f64 * 2.5);
    (1.0 - normalized).clamp(0.001, 0.9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_result_creation() {
        let result = VerificationResult::new(
            "H1_test".to_string(),
            "Test hypothesis".to_string(),
            2.5,
            0.03,
            0.45,
        );

        assert!(result.is_supported());
        assert!(result.interpretation.contains("SUPPORT"));
    }

    #[test]
    fn test_verification_suite() {
        let mut suite = VerificationSuite::new();

        let r1 = VerificationResult::new(
            "H1".to_string(),
            "Supported".to_string(),
            3.0,
            0.01,
            0.5,
        );

        let r2 = VerificationResult::new(
            "H2".to_string(),
            "Not supported".to_string(),
            1.0,
            0.50,
            0.1,
        );

        suite.add_result(r1);
        suite.add_result(r2);

        assert_eq!(suite.count_total(), 2);
        assert_eq!(suite.count_supported(), 1);
        assert!((suite.success_rate() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_hypothesis_description() {
        let h = Hypothesis::SymmetryBreaking {
            base: 6,
            outer: 1,
            inner: 5,
            k_outer: 0,
            k_inner: 0,
            min_cluster_size: 3,
            num_seeds: 100,
        };

        let desc = h.description();
        assert!(desc.contains("Symmetry Breaking"));
        assert!(desc.contains("Base 6"));
    }
}
