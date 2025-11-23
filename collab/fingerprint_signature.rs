//! Prime Constructor Signature
//!
//! Complete fingerprint of a prime construction method, including:
//! - Modular residue profile
//! - Digit distribution
//! - Structural features (zero fraction, palindrome rate, etc.)
//! - Hardy-Littlewood normalized features (theoretical alignment)

use super::profile::{ModularProfile, GapStatistics, compute_gap_statistics};
use num_bigint::BigUint;
use std::collections::HashMap;

/// Complete signature of a prime construction method
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrimeConstructorSignature {
    /// Human-readable label (e.g., "membrane_base6_1_5")
    pub label: String,

    /// Number of samples used to generate this signature
    pub sample_size: usize,

    /// Modular residue profile
    pub modular_profile: ModularProfile,

    /// Additional structural features
    pub features: SignatureFeatures,

    /// Gap statistics for key moduli
    pub gap_stats: HashMap<u32, GapStatistics>,
}

/// Structural and statistical features
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SignatureFeatures {
    /// Digit distribution [p(0), p(1), ..., p(9)]
    pub digit_distribution: Vec<f64>,

    /// Fraction of digits that are zero
    pub zero_fraction: f64,

    /// Entropy of digit distribution (Shannon entropy)
    pub digit_entropy: f64,

    /// Fraction of palindromic numbers
    pub palindrome_rate: f64,

    /// Average number of digits
    pub mean_digit_count: f64,

    /// Variance of digit count
    pub var_digit_count: f64,

    /// Fraction using only digits {0, 3}
    pub zero_three_only_rate: f64,

    /// Fraction using only digits {0, 6}
    pub zero_six_only_rate: f64,

    /// Hardy-Littlewood: Mean modular chi-squared distance from expected uniform
    /// Measures systematic deviation from HL-predicted residue distributions
    pub hl_modular_divergence: f64,

    /// Hardy-Littlewood: Coverage deviation (observed - expected prime density)
    /// Positive = constructor beats HL prediction, negative = underperforms
    pub hl_coverage_deviation: f64,
}

impl PrimeConstructorSignature {
    /// Compute signature from a collection of numbers
    pub fn from_numbers(label: String, numbers: &[BigUint]) -> Self {
        let modular_profile = super::profile::compute_modular_profile(numbers);
        let features = compute_features(numbers);
        let gap_stats = compute_all_gap_stats(numbers);

        PrimeConstructorSignature {
            label,
            sample_size: numbers.len(),
            modular_profile,
            features,
            gap_stats,
        }
    }

    /// Convert to flat feature vector for ML (modular + structural features)
    pub fn to_feature_vector(&self) -> Vec<f64> {
        let mut vec = self.modular_profile.to_feature_vector();
        vec.extend(&self.features.digit_distribution);
        vec.push(self.features.zero_fraction);
        vec.push(self.features.digit_entropy);
        vec.push(self.features.palindrome_rate);
        vec.push(self.features.mean_digit_count);
        vec.push(self.features.var_digit_count);
        vec.push(self.features.zero_three_only_rate);
        vec.push(self.features.zero_six_only_rate);

        // Hardy-Littlewood normalized features
        vec.push(self.features.hl_modular_divergence);
        vec.push(self.features.hl_coverage_deviation);

        // Add gap statistics features (use 0.0 if missing)
        for modulus in [3u32, 7, 11, 13, 17, 19] {
            if let Some(stats) = self.gap_stats.get(&modulus) {
                vec.push(stats.mean_gap);
                vec.push(stats.var_gap);
                vec.push(stats.small_gap_excess);
                vec.push(stats.large_gap_excess);
            } else {
                // Missing gap stats - use zeros
                vec.push(0.0);
                vec.push(0.0);
                vec.push(0.0);
                vec.push(0.0);
            }
        }

        vec
    }

    /// Compute spectral weirdness score relative to baseline
    pub fn weirdness_score(&self, baseline: &PrimeConstructorSignature) -> f64 {
        // Chi-squared distance in modular space
        let modular_dist = self.modular_profile.chi_squared_distance(&baseline.modular_profile);

        // L2 distance in digit space
        let digit_dist: f64 = self.features.digit_distribution
            .iter()
            .zip(&baseline.features.digit_distribution)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();

        // Combined score (weighted)
        modular_dist + 10.0 * digit_dist
    }
}

/// Compute structural features
fn compute_features(numbers: &[BigUint]) -> SignatureFeatures {
    if numbers.is_empty() {
        return SignatureFeatures {
            digit_distribution: vec![0.0; 10],
            zero_fraction: 0.0,
            digit_entropy: 0.0,
            palindrome_rate: 0.0,
            mean_digit_count: 0.0,
            var_digit_count: 0.0,
            zero_three_only_rate: 0.0,
            zero_six_only_rate: 0.0,
            hl_modular_divergence: 0.0,
            hl_coverage_deviation: 0.0,
        };
    }

    // Convert to strings for analysis
    let strings: Vec<String> = numbers.iter().map(|n| n.to_string()).collect();

    // Digit distribution
    let mut digit_counts = vec![0usize; 10];
    let mut total_digits = 0;

    for s in &strings {
        for ch in s.chars() {
            if let Some(d) = ch.to_digit(10) {
                digit_counts[d as usize] += 1;
                total_digits += 1;
            }
        }
    }

    let digit_distribution: Vec<f64> = digit_counts
        .iter()
        .map(|&count| count as f64 / total_digits as f64)
        .collect();

    let zero_fraction = digit_distribution[0];

    // Shannon entropy
    let digit_entropy: f64 = digit_distribution
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.log2())
        .sum();

    // Palindrome rate
    let palindrome_count = strings
        .iter()
        .filter(|s| {
            let chars: Vec<char> = s.chars().collect();
            chars.iter().eq(chars.iter().rev())
        })
        .count();
    let palindrome_rate = palindrome_count as f64 / numbers.len() as f64;

    // Digit count statistics
    let digit_counts_per_num: Vec<f64> = strings.iter().map(|s| s.len() as f64).collect();
    let mean_digit_count = digit_counts_per_num.iter().sum::<f64>() / numbers.len() as f64;
    let var_digit_count = digit_counts_per_num
        .iter()
        .map(|&c| {
            let diff = c - mean_digit_count;
            diff * diff
        })
        .sum::<f64>() / numbers.len() as f64;

    // Special pattern rates
    let zero_three_only = strings
        .iter()
        .filter(|s| s.chars().all(|ch| ch == '0' || ch == '3'))
        .count();
    let zero_six_only = strings
        .iter()
        .filter(|s| s.chars().all(|ch| ch == '0' || ch == '6'))
        .count();

    // Hardy-Littlewood normalized features
    let (hl_modular_divergence, hl_coverage_deviation) = compute_hl_features(numbers);

    SignatureFeatures {
        digit_distribution,
        zero_fraction,
        digit_entropy,
        palindrome_rate,
        mean_digit_count,
        var_digit_count,
        zero_three_only_rate: zero_three_only as f64 / numbers.len() as f64,
        zero_six_only_rate: zero_six_only as f64 / numbers.len() as f64,
        hl_modular_divergence,
        hl_coverage_deviation,
    }
}

/// Compute Hardy-Littlewood normalized features
///
/// Returns (modular_divergence, coverage_deviation):
/// - modular_divergence: Mean chi-squared distance from uniform distribution
/// - coverage_deviation: (observed_density / HL_expected_density) - 1.0
fn compute_hl_features(numbers: &[BigUint]) -> (f64, f64) {
    if numbers.is_empty() {
        return (0.0, 0.0);
    }

    // Compute modular divergence: chi-squared distance from uniform
    let moduli = [3u32, 7, 11, 13, 17, 19];
    let mut total_chi_squared = 0.0;

    for &modulus in &moduli {
        // Count residues
        let mut residue_counts = vec![0usize; modulus as usize];
        for num in numbers {
            let residue = (num % modulus).to_u32_digits();
            if let Some(&r) = residue.first() {
                residue_counts[r as usize % modulus as usize] += 1;
            }
        }

        // Expected count for uniform distribution
        let expected = numbers.len() as f64 / modulus as f64;

        // Chi-squared: Σ (observed - expected)² / expected
        let chi_sq: f64 = residue_counts.iter()
            .map(|&count| {
                let diff = count as f64 - expected;
                (diff * diff) / expected
            })
            .sum();

        total_chi_squared += chi_sq;
    }

    let hl_modular_divergence = total_chi_squared / moduli.len() as f64;

    // Compute coverage deviation: compare actual vs HL-predicted prime density
    // For simplicity, we use average digit length and assume even distribution
    let avg_digits = numbers.iter()
        .map(|n| n.to_string().len() as f64)
        .sum::<f64>() / numbers.len() as f64;

    // Rough HL prediction: 1/ln(10^d) for d-digit numbers
    let avg_magnitude = 10_f64.powf(avg_digits);
    let expected_density = 1.0 / avg_magnitude.ln();

    // Observed density: we generated N primes from some candidate pool
    // Since we only have primes, observed_density is artificially 1.0
    // To make this meaningful, we'd need to track candidate count
    // For now, use a placeholder that measures relative enrichment
    let observed_density = 1.0; // All inputs are primes

    // Deviation: (observed / expected) - 1.0
    // Positive = beats HL prediction, negative = underperforms
    let hl_coverage_deviation = (observed_density / expected_density) - 1.0;

    (hl_modular_divergence, hl_coverage_deviation)
}

/// Compute gap statistics for all key moduli
fn compute_all_gap_stats(numbers: &[BigUint]) -> HashMap<u32, GapStatistics> {
    let moduli = [3u32, 7, 11, 13, 17, 19];
    let mut stats = HashMap::new();

    for &modulus in &moduli {
        let gap_stat = compute_gap_statistics(numbers, modulus);
        stats.insert(modulus, gap_stat);
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_from_numbers() {
        let numbers: Vec<BigUint> = vec![
            BigUint::from(101u32),
            BigUint::from(103u32),
            BigUint::from(107u32),
            BigUint::from(109u32),
        ];

        let sig = PrimeConstructorSignature::from_numbers("test".to_string(), &numbers);
        assert_eq!(sig.label, "test");
        assert_eq!(sig.sample_size, 4);
        assert!(sig.features.mean_digit_count > 0.0);
    }

    #[test]
    fn test_feature_vector() {
        let numbers: Vec<BigUint> = (0..100).map(|i| BigUint::from(i as u32)).collect();
        let sig = PrimeConstructorSignature::from_numbers("test".to_string(), &numbers);
        let vec = sig.to_feature_vector();

        // Should have modular (3+7+11+13+17+19=70) + digits (10) + scalar features (7) + HL features (2) + gap stats (4*6=24)
        assert_eq!(vec.len(), 70 + 10 + 7 + 2 + 24);
    }
}
