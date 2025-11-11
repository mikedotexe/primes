//! Harmonic Lagrange Point Analysis
//!
//! Explores whether Lagrange point positions follow harmonic relationships
//! when concatenating primes from harmonically-related bases.
//!
//! # Core Hypothesis
//!
//! If we concatenate:
//! - Prime from base 6 (fundamental): `10301`
//! - Prime from base 12 (first overtone = 2×6): `3007003007003`
//!
//! Do the Lagrange positions in the zero buffer show harmonic positioning?
//! E.g., at positions that are simple fractions of buffer length (1/2, 1/3, 2/3, φ)?
//!
//! # Harmonic Ratios
//!
//! Musical overtone series produces these ratios:
//! - Octave: 1/2
//! - Perfect fifth: 2/3
//! - Perfect fourth: 3/4
//! - Major third: 4/5
//! - Golden ratio: φ ≈ 0.618
//!
//! If Lagrange points cluster at these positions, it suggests deep
//! mathematical harmony between spatial equilibrium and base relationships.

use std::collections::HashMap;

/// Golden ratio constant
pub const GOLDEN_RATIO: f64 = 0.6180339887498948;

/// Common harmonic ratios to test
pub const HARMONIC_RATIOS: &[(f64, &str)] = &[
    (0.5, "octave (1/2)"),
    (0.333, "major third (1/3)"),
    (0.667, "perfect fifth (2/3)"),
    (0.25, "double octave (1/4)"),
    (0.75, "major sixth (3/4)"),
    (0.2, "1/5"),
    (0.4, "2/5"),
    (0.6, "3/5"),
    (0.8, "4/5"),
    (GOLDEN_RATIO, "golden ratio (φ)"),
];

/// A single Lagrange point with position and creating digit
#[derive(Clone, Debug)]
pub struct LagrangePoint {
    pub position: usize,
    pub digit: u8,
    pub buffer_size: usize,
}

impl LagrangePoint {
    /// Get fractional position in buffer (0.0 to 1.0)
    pub fn fractional_position(&self) -> f64 {
        if self.buffer_size == 0 {
            0.0
        } else {
            self.position as f64 / self.buffer_size as f64
        }
    }

    /// Find nearest harmonic ratio
    pub fn nearest_harmonic(&self) -> Option<(f64, &'static str, f64)> {
        let frac = self.fractional_position();

        let mut best_ratio: Option<(f64, &'static str, f64)> = None;
        let mut best_distance = f64::INFINITY;

        for &(ratio, name) in HARMONIC_RATIOS {
            let distance = (frac - ratio).abs();
            if distance < best_distance {
                best_distance = distance;
                best_ratio = Some((ratio, name, distance));
            }
        }

        best_ratio
    }

    /// Check if within tolerance of a harmonic ratio
    pub fn is_near_harmonic(&self, tolerance: f64) -> bool {
        if let Some((_, _, distance)) = self.nearest_harmonic() {
            distance <= tolerance
        } else {
            false
        }
    }
}

/// Pair of primes from harmonically-related bases
#[derive(Clone, Debug)]
pub struct HarmonicLagrangePair {
    pub prime1_base: usize,
    pub prime2_base: usize,
    pub prime1: String,
    pub prime2: String,
    pub harmonic_order: Option<usize>, // None if not harmonic
}

impl HarmonicLagrangePair {
    /// Create new pair, detecting harmonic relationship
    pub fn new(prime1: String, prime1_base: usize, prime2: String, prime2_base: usize) -> Self {
        let harmonic_order = if prime2_base.is_multiple_of(prime1_base) {
            Some(prime2_base / prime1_base)
        } else if prime1_base.is_multiple_of(prime2_base) {
            Some(prime1_base / prime2_base)
        } else {
            None
        };

        Self {
            prime1_base,
            prime2_base,
            prime1,
            prime2,
            harmonic_order,
        }
    }

    /// Is this a harmonic pair?
    pub fn is_harmonic(&self) -> bool {
        self.harmonic_order.is_some()
    }

    /// Get harmonic relationship description
    pub fn harmonic_description(&self) -> String {
        if let Some(order) = self.harmonic_order {
            let (fundamental, overtone) = if self.prime2_base > self.prime1_base {
                (self.prime1_base, self.prime2_base)
            } else {
                (self.prime2_base, self.prime1_base)
            };

            format!(
                "Harmonic: base {} is {}× fundamental (base {})",
                overtone, order, fundamental
            )
        } else {
            format!(
                "Non-harmonic: bases {} and {} are not multiples",
                self.prime1_base, self.prime2_base
            )
        }
    }
}

/// Analysis of Lagrange point positions in a concatenated pair
#[derive(Clone, Debug)]
pub struct PositionalAnalysis {
    pub pair: HarmonicLagrangePair,
    pub buffer_size: usize,
    pub lagrange_points: Vec<LagrangePoint>,
    pub total_tested: usize,
}

impl PositionalAnalysis {
    /// Create new positional analysis
    pub fn new(pair: HarmonicLagrangePair, buffer_size: usize) -> Self {
        Self {
            pair,
            buffer_size,
            lagrange_points: Vec::new(),
            total_tested: 0,
        }
    }

    /// Record a Lagrange point
    pub fn add_lagrange_point(&mut self, position: usize, digit: u8) {
        self.lagrange_points.push(LagrangePoint {
            position,
            digit,
            buffer_size: self.buffer_size,
        });
    }

    /// Get success rate (lagrange points / total tests)
    pub fn success_rate(&self) -> f64 {
        if self.total_tested == 0 {
            0.0
        } else {
            self.lagrange_points.len() as f64 / self.total_tested as f64
        }
    }

    /// Count points near each harmonic ratio
    pub fn harmonic_distribution(&self, tolerance: f64) -> HashMap<&'static str, usize> {
        let mut distribution: HashMap<&'static str, usize> = HashMap::new();

        for lp in &self.lagrange_points {
            if let Some((_, name, distance)) = lp.nearest_harmonic() {
                if distance <= tolerance {
                    *distribution.entry(name).or_insert(0) += 1;
                }
            }
        }

        distribution
    }

    /// Test if positions cluster at harmonics (chi-squared-like test)
    ///
    /// Returns (clustered_count, expected_random, enrichment_factor)
    pub fn harmonic_clustering_test(&self, tolerance: f64) -> (usize, f64, f64) {
        let clustered = self
            .lagrange_points
            .iter()
            .filter(|lp| lp.is_near_harmonic(tolerance))
            .count();

        // Expected by random chance: tolerance × 2 (both sides) × number of ratios
        let expected_random =
            tolerance * 2.0 * HARMONIC_RATIOS.len() as f64 * self.lagrange_points.len() as f64;

        let enrichment = if expected_random > 0.0 {
            clustered as f64 / expected_random
        } else {
            0.0
        };

        (clustered, expected_random, enrichment)
    }

    /// Find most populated harmonic position
    pub fn dominant_harmonic(&self, tolerance: f64) -> Option<(&'static str, usize, f64)> {
        let distribution = self.harmonic_distribution(tolerance);

        distribution
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&name, &count)| {
                // Find actual ratio for this name
                let ratio = HARMONIC_RATIOS
                    .iter()
                    .find(|(_, n)| *n == name)
                    .map(|(r, _)| *r)
                    .unwrap_or(0.0);

                (name, count, ratio)
            })
    }

    /// Calculate position variance (how spread out are positions?)
    pub fn position_variance(&self) -> f64 {
        if self.lagrange_points.is_empty() {
            return 0.0;
        }

        let positions: Vec<f64> = self
            .lagrange_points
            .iter()
            .map(|lp| lp.fractional_position())
            .collect();

        let mean = positions.iter().sum::<f64>() / positions.len() as f64;
        let variance =
            positions.iter().map(|&p| (p - mean).powi(2)).sum::<f64>() / positions.len() as f64;

        variance
    }
}

/// Accumulator for comparing harmonic vs non-harmonic pairs
#[derive(Default)]
pub struct HarmonicComparator {
    pub harmonic_analyses: Vec<PositionalAnalysis>,
    pub non_harmonic_analyses: Vec<PositionalAnalysis>,
}

impl HarmonicComparator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an analysis
    pub fn add_analysis(&mut self, analysis: PositionalAnalysis) {
        if analysis.pair.is_harmonic() {
            self.harmonic_analyses.push(analysis);
        } else {
            self.non_harmonic_analyses.push(analysis);
        }
    }

    /// Compare harmonic vs non-harmonic clustering
    ///
    /// Returns (harmonic_enrichment, non_harmonic_enrichment, ratio)
    pub fn compare_clustering(&self, tolerance: f64) -> (f64, f64, f64) {
        let harmonic_enrichment: f64 = if !self.harmonic_analyses.is_empty() {
            self.harmonic_analyses
                .iter()
                .map(|a| a.harmonic_clustering_test(tolerance).2)
                .sum::<f64>()
                / self.harmonic_analyses.len() as f64
        } else {
            0.0
        };

        let non_harmonic_enrichment: f64 = if !self.non_harmonic_analyses.is_empty() {
            self.non_harmonic_analyses
                .iter()
                .map(|a| a.harmonic_clustering_test(tolerance).2)
                .sum::<f64>()
                / self.non_harmonic_analyses.len() as f64
        } else {
            0.0
        };

        let ratio = if non_harmonic_enrichment > 0.0 {
            harmonic_enrichment / non_harmonic_enrichment
        } else {
            0.0
        };

        (harmonic_enrichment, non_harmonic_enrichment, ratio)
    }

    /// Average position variance for each group
    pub fn compare_variance(&self) -> (f64, f64) {
        let harmonic_var = if !self.harmonic_analyses.is_empty() {
            self.harmonic_analyses
                .iter()
                .map(|a| a.position_variance())
                .sum::<f64>()
                / self.harmonic_analyses.len() as f64
        } else {
            0.0
        };

        let non_harmonic_var = if !self.non_harmonic_analyses.is_empty() {
            self.non_harmonic_analyses
                .iter()
                .map(|a| a.position_variance())
                .sum::<f64>()
                / self.non_harmonic_analyses.len() as f64
        } else {
            0.0
        };

        (harmonic_var, non_harmonic_var)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lagrange_point_fractional_position() {
        let lp = LagrangePoint {
            position: 5,
            digit: 3,
            buffer_size: 10,
        };

        assert!((lp.fractional_position() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_nearest_harmonic() {
        let lp = LagrangePoint {
            position: 5,
            digit: 3,
            buffer_size: 10,
        };

        let (ratio, name, _distance) = lp.nearest_harmonic().unwrap();
        assert_eq!(ratio, 0.5);
        assert!(name.contains("octave"));
    }

    #[test]
    fn test_harmonic_pair_detection() {
        let pair = HarmonicLagrangePair::new("101".to_string(), 6, "121".to_string(), 12);

        assert!(pair.is_harmonic());
        assert_eq!(pair.harmonic_order, Some(2));
    }

    #[test]
    fn test_non_harmonic_pair() {
        let pair = HarmonicLagrangePair::new("101".to_string(), 6, "121".to_string(), 7);

        assert!(!pair.is_harmonic());
        assert_eq!(pair.harmonic_order, None);
    }
}
