//! Modular Profile Computation
//!
//! Computes the distribution of residues across multiple moduli for a collection of numbers.
//! This forms the core spectral fingerprint used to classify prime construction methods.

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::collections::HashMap;

/// Modular profile: distributions of residues for multiple moduli
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModularProfile {
    /// Distribution for mod 3: [p(r=0), p(r=1), p(r=2)]
    pub mod3: Vec<f64>,

    /// Distribution for mod 7: [p(r=0), ..., p(r=6)]
    pub mod7: Vec<f64>,

    /// Distribution for mod 11: [p(r=0), ..., p(r=10)]
    pub mod11: Vec<f64>,

    /// Distribution for mod 13: [p(r=0), ..., p(r=12)]
    pub mod13: Vec<f64>,

    /// Distribution for mod 17: [p(r=0), ..., p(r=16)]
    pub mod17: Vec<f64>,

    /// Distribution for mod 19: [p(r=0), ..., p(r=18)]
    pub mod19: Vec<f64>,
}

impl ModularProfile {
    /// Create empty profile with zeros
    pub fn empty() -> Self {
        ModularProfile {
            mod3: vec![0.0; 3],
            mod7: vec![0.0; 7],
            mod11: vec![0.0; 11],
            mod13: vec![0.0; 13],
            mod17: vec![0.0; 17],
            mod19: vec![0.0; 19],
        }
    }

    /// Convert to flat feature vector for ML
    pub fn to_feature_vector(&self) -> Vec<f64> {
        let mut features = Vec::new();
        features.extend(&self.mod3);
        features.extend(&self.mod7);
        features.extend(&self.mod11);
        features.extend(&self.mod13);
        features.extend(&self.mod17);
        features.extend(&self.mod19);
        features
    }

    /// Compute chi-squared distance from another profile
    pub fn chi_squared_distance(&self, other: &ModularProfile) -> f64 {
        let mut total = 0.0;

        // Mod 3
        for (obs, exp) in self.mod3.iter().zip(&other.mod3) {
            if *exp > 0.0 {
                total += (obs - exp).powi(2) / exp;
            }
        }

        // Mod 7
        for (obs, exp) in self.mod7.iter().zip(&other.mod7) {
            if *exp > 0.0 {
                total += (obs - exp).powi(2) / exp;
            }
        }

        // Mod 11
        for (obs, exp) in self.mod11.iter().zip(&other.mod11) {
            if *exp > 0.0 {
                total += (obs - exp).powi(2) / exp;
            }
        }

        // Mod 13
        for (obs, exp) in self.mod13.iter().zip(&other.mod13) {
            if *exp > 0.0 {
                total += (obs - exp).powi(2) / exp;
            }
        }

        // Mod 17
        for (obs, exp) in self.mod17.iter().zip(&other.mod17) {
            if *exp > 0.0 {
                total += (obs - exp).powi(2) / exp;
            }
        }

        // Mod 19
        for (obs, exp) in self.mod19.iter().zip(&other.mod19) {
            if *exp > 0.0 {
                total += (obs - exp).powi(2) / exp;
            }
        }

        total
    }
}

/// Compute modular profile for a collection of numbers
pub fn compute_modular_profile(numbers: &[BigUint]) -> ModularProfile {
    if numbers.is_empty() {
        return ModularProfile::empty();
    }

    let moduli = [3u32, 7, 11, 13, 17, 19];
    let mut profiles: HashMap<u32, Vec<usize>> = HashMap::new();

    // Initialize counters
    for &m in &moduli {
        profiles.insert(m, vec![0; m as usize]);
    }

    // Count residues
    for number in numbers {
        for &modulus in &moduli {
            let residue = (number % modulus).to_u32().unwrap_or(0) as usize;
            profiles.get_mut(&modulus).unwrap()[residue] += 1;
        }
    }

    // Normalize to probabilities
    let n = numbers.len() as f64;

    let mod3_dist: Vec<f64> = profiles[&3].iter().map(|&count| count as f64 / n).collect();
    let mod7_dist: Vec<f64> = profiles[&7].iter().map(|&count| count as f64 / n).collect();
    let mod11_dist: Vec<f64> = profiles[&11]
        .iter()
        .map(|&count| count as f64 / n)
        .collect();
    let mod13_dist: Vec<f64> = profiles[&13]
        .iter()
        .map(|&count| count as f64 / n)
        .collect();
    let mod17_dist: Vec<f64> = profiles[&17]
        .iter()
        .map(|&count| count as f64 / n)
        .collect();
    let mod19_dist: Vec<f64> = profiles[&19]
        .iter()
        .map(|&count| count as f64 / n)
        .collect();

    ModularProfile {
        mod3: mod3_dist,
        mod7: mod7_dist,
        mod11: mod11_dist,
        mod13: mod13_dist,
        mod17: mod17_dist,
        mod19: mod19_dist,
    }
}

/// Compute gap statistics for a single modulus
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GapStatistics {
    pub mean_gap: f64,
    pub var_gap: f64,
    pub small_gap_excess: f64, // Fraction < 0.5 * mean
    pub large_gap_excess: f64, // Fraction > 2.0 * mean
}

/// Compute gap statistics for residues under a given modulus
pub fn compute_gap_statistics(numbers: &[BigUint], modulus: u32) -> GapStatistics {
    // Extract and sort residues
    let mut residues: Vec<u32> = numbers
        .iter()
        .map(|n| (n % modulus).to_u32().unwrap_or(0))
        .collect();

    residues.sort_unstable();
    residues.dedup(); // Remove duplicates

    if residues.len() < 2 {
        return GapStatistics {
            mean_gap: 0.0,
            var_gap: 0.0,
            small_gap_excess: 0.0,
            large_gap_excess: 0.0,
        };
    }

    // Compute gaps
    let gaps: Vec<u32> = residues.windows(2).map(|w| w[1] - w[0]).collect();

    // Statistics
    let mean_gap = gaps.iter().sum::<u32>() as f64 / gaps.len() as f64;
    let var_gap = gaps
        .iter()
        .map(|&g| {
            let diff = g as f64 - mean_gap;
            diff * diff
        })
        .sum::<f64>()
        / gaps.len() as f64;

    let small_threshold = 0.5 * mean_gap;
    let large_threshold = 2.0 * mean_gap;

    let small_count = gaps
        .iter()
        .filter(|&&g| (g as f64) < small_threshold)
        .count();
    let large_count = gaps
        .iter()
        .filter(|&&g| (g as f64) > large_threshold)
        .count();

    GapStatistics {
        mean_gap,
        var_gap,
        small_gap_excess: small_count as f64 / gaps.len() as f64,
        large_gap_excess: large_count as f64 / gaps.len() as f64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_profile() {
        let profile = ModularProfile::empty();
        assert_eq!(profile.mod3.len(), 3);
        assert_eq!(profile.mod7.len(), 7);
        assert!(profile.mod3.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_modular_profile_uniform() {
        // Create numbers that should distribute uniformly mod 7
        let numbers: Vec<BigUint> = (0..70).map(|i| BigUint::from(i as u32)).collect();
        let profile = compute_modular_profile(&numbers);

        // Each residue class should have ~1/7 ≈ 0.143
        for &prob in &profile.mod7 {
            assert!(
                (prob - 0.1428).abs() < 0.01,
                "Expected ~0.143, got {}",
                prob
            );
        }
    }

    #[test]
    fn test_feature_vector() {
        let profile = ModularProfile::empty();
        let features = profile.to_feature_vector();
        assert_eq!(features.len(), 3 + 7 + 11 + 13 + 17 + 19);
    }
}
