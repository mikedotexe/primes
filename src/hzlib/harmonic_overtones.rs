//! Harmonic Overtones Analysis
//!
//! Analyzes prime generation patterns at harmonic multiples of successful bases.
//! Inspired by acoustic physics: if base 6 is a "fundamental frequency" with 33%
//! success rate, do bases 12, 18, 24 (overtones) show harmonic resonance?
//!
//! # Core Concept
//!
//! In acoustics, a fundamental frequency f₀ produces overtones at 2f₀, 3f₀, 4f₀...
//! These overtones share mathematical relationships with the fundamental.
//!
//! Similarly, if a base B shows strong prime generation:
//! - Fundamental: B (e.g., 6)
//! - First overtone: 2B (e.g., 12)
//! - Second overtone: 3B (e.g., 18)
//! - Third overtone: 4B (e.g., 24)
//!
//! ## Research Questions
//!
//! 1. Do overtone bases inherit success patterns from fundamentals?
//! 2. Is there amplitude decay (success rate decreases with higher overtones)?
//! 3. Do optimal configurations at fundamental map to overtones?
//! 4. Are there "harmonic series" of bases with related properties?
//!
//! # Example
//!
//! ```text
//! Fundamental: Base 6, (1,5) k=(0,0) → 33% success
//! 1st Overtone: Base 12, (1,5) k=(0,0) → ?% success
//! 2nd Overtone: Base 18, (1,5) k=(0,0) → ?% success
//!
//! Question: Does the (1,5) configuration remain optimal?
//! ```

use std::collections::HashMap;

/// Represents a fundamental base and its harmonic series
#[derive(Clone, Debug)]
pub struct HarmonicSeries {
    pub fundamental: usize,
    pub overtones: Vec<usize>,
    pub fundamental_success_rate: f64,
    pub overtone_success_rates: HashMap<usize, f64>,
}

impl HarmonicSeries {
    /// Create new harmonic series from fundamental base
    ///
    /// # Arguments
    /// * `fundamental` - The fundamental base
    /// * `max_overtone_order` - Maximum overtone to generate (e.g., 4 for up to 4×fundamental)
    pub fn new(fundamental: usize, max_overtone_order: usize) -> Self {
        let overtones: Vec<usize> = (2..=max_overtone_order).map(|n| n * fundamental).collect();

        Self {
            fundamental,
            overtones,
            fundamental_success_rate: 0.0,
            overtone_success_rates: HashMap::new(),
        }
    }

    /// Set fundamental success rate
    pub fn set_fundamental_rate(&mut self, rate: f64) {
        self.fundamental_success_rate = rate;
    }

    /// Record overtone success rate
    pub fn record_overtone(&mut self, overtone: usize, rate: f64) {
        self.overtone_success_rates.insert(overtone, rate);
    }

    /// Get overtone order (2 for first overtone, 3 for second, etc.)
    pub fn overtone_order(&self, base: usize) -> Option<usize> {
        if base.is_multiple_of(self.fundamental) && base > self.fundamental {
            Some(base / self.fundamental)
        } else {
            None
        }
    }

    /// Calculate amplitude decay factor
    ///
    /// Returns (slope, r²) of success rate vs overtone order
    pub fn amplitude_decay(&self) -> (f64, f64) {
        if self.overtone_success_rates.is_empty() {
            return (0.0, 0.0);
        }

        let mut xs = Vec::new();
        let mut ys = Vec::new();

        // Include fundamental as order 1
        xs.push(1.0);
        ys.push(self.fundamental_success_rate);

        // Add overtones
        for (&overtone, &rate) in &self.overtone_success_rates {
            if let Some(order) = self.overtone_order(overtone) {
                xs.push(order as f64);
                ys.push(rate);
            }
        }

        // Simple linear regression
        let n = xs.len() as f64;
        if n < 2.0 {
            return (0.0, 0.0);
        }

        let sum_x: f64 = xs.iter().sum();
        let sum_y: f64 = ys.iter().sum();
        let sum_xx: f64 = xs.iter().map(|x| x * x).sum();
        let sum_xy: f64 = xs.iter().zip(ys.iter()).map(|(x, y)| x * y).sum();

        let denom = n * sum_xx - sum_x * sum_x;
        if denom.abs() < 1e-10 {
            return (0.0, 0.0);
        }

        let slope = (n * sum_xy - sum_x * sum_y) / denom;

        // Calculate R²
        let mean_y = sum_y / n;
        let ss_tot: f64 = ys.iter().map(|y| (y - mean_y).powi(2)).sum();
        let ss_res: f64 = xs
            .iter()
            .zip(ys.iter())
            .map(|(x, y)| {
                let pred = slope * x + (sum_y - slope * sum_x) / n;
                (y - pred).powi(2)
            })
            .sum();

        let r2 = if ss_tot > 0.0 {
            1.0 - ss_res / ss_tot
        } else {
            0.0
        };

        (slope, r2)
    }

    /// Check if overtones show coherent resonance
    ///
    /// Returns true if overtones maintain > 50% of fundamental's success rate
    pub fn has_coherent_resonance(&self) -> bool {
        if self.overtone_success_rates.is_empty() {
            return false;
        }

        let threshold = self.fundamental_success_rate * 0.5;

        self.overtone_success_rates
            .values()
            .all(|&rate| rate >= threshold)
    }

    /// Calculate harmonic mean of all rates (fundamental + overtones)
    pub fn harmonic_mean_rate(&self) -> f64 {
        let mut reciprocal_sum = 1.0 / self.fundamental_success_rate;
        let mut count = 1.0;

        for &rate in self.overtone_success_rates.values() {
            if rate > 0.0 {
                reciprocal_sum += 1.0 / rate;
                count += 1.0;
            }
        }

        count / reciprocal_sum
    }

    /// Find strongest overtone (highest success rate)
    pub fn strongest_overtone(&self) -> Option<(usize, f64)> {
        self.overtone_success_rates
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(&base, &rate)| (base, rate))
    }

    /// Check for overtone enhancement (overtone stronger than fundamental)
    pub fn has_overtone_enhancement(&self) -> bool {
        self.overtone_success_rates
            .values()
            .any(|&rate| rate > self.fundamental_success_rate)
    }
}

/// Accumulator for multiple harmonic series analysis
pub struct HarmonicAccumulator {
    pub series: Vec<HarmonicSeries>,
}

impl HarmonicAccumulator {
    pub fn new() -> Self {
        Self { series: Vec::new() }
    }

    /// Add a harmonic series
    pub fn add_series(&mut self, series: HarmonicSeries) {
        self.series.push(series);
    }

    /// Find all fundamentals that show coherent resonance
    pub fn coherent_fundamentals(&self) -> Vec<usize> {
        self.series
            .iter()
            .filter(|s| s.has_coherent_resonance())
            .map(|s| s.fundamental)
            .collect()
    }

    /// Find all series with overtone enhancement
    pub fn enhanced_series(&self) -> Vec<usize> {
        self.series
            .iter()
            .filter(|s| s.has_overtone_enhancement())
            .map(|s| s.fundamental)
            .collect()
    }

    /// Calculate average amplitude decay across all series
    pub fn average_decay_slope(&self) -> f64 {
        if self.series.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.series.iter().map(|s| s.amplitude_decay().0).sum();
        sum / self.series.len() as f64
    }

    /// Find series with strongest coherence (highest R² in decay)
    pub fn strongest_coherence(&self) -> Option<(usize, f64)> {
        self.series
            .iter()
            .map(|s| (s.fundamental, s.amplitude_decay().1))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl Default for HarmonicAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

/// Analyze configuration compatibility across harmonic series
///
/// Tests if a configuration (outer, inner, k) that works at fundamental
/// also works at overtones (with appropriate scaling)
#[derive(Clone, Debug)]
pub struct ConfigurationHarmonic {
    pub fundamental: usize,
    pub outer: u32,
    pub inner: u32,
    pub k_outer: u32,
    pub k_inner: u32,

    /// Success rates at each base in harmonic series
    pub rates: HashMap<usize, f64>,
}

impl ConfigurationHarmonic {
    pub fn new(fundamental: usize, outer: u32, inner: u32, k_outer: u32, k_inner: u32) -> Self {
        Self {
            fundamental,
            outer,
            inner,
            k_outer,
            k_inner,
            rates: HashMap::new(),
        }
    }

    /// Record success rate at a base
    pub fn record_rate(&mut self, base: usize, rate: f64) {
        self.rates.insert(base, rate);
    }

    /// Check if configuration remains optimal across overtones
    ///
    /// Returns true if this config is in top 3 for all tested bases
    pub fn is_universally_optimal(&self, top_configs: &HashMap<usize, Vec<String>>) -> bool {
        let config_str = format!(
            "({},{}) k=({},{})",
            self.outer, self.inner, self.k_outer, self.k_inner
        );

        self.rates.keys().all(|&base| {
            top_configs
                .get(&base)
                .map(|configs| configs.iter().any(|c| c.contains(&config_str)))
                .unwrap_or(false)
        })
    }

    /// Calculate configuration persistence score
    ///
    /// Returns fraction of overtones where config maintains > 80% of fundamental rate
    pub fn persistence_score(&self) -> f64 {
        let fundamental_rate = self.rates.get(&self.fundamental).copied().unwrap_or(0.0);

        if fundamental_rate == 0.0 {
            return 0.0;
        }

        let threshold = fundamental_rate * 0.8;
        let persistent_count = self
            .rates
            .iter()
            .filter(|(&base, &rate)| base != self.fundamental && rate >= threshold)
            .count();

        let overtone_count = self.rates.len().saturating_sub(1);
        if overtone_count == 0 {
            return 0.0;
        }

        persistent_count as f64 / overtone_count as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmonic_series_basic() {
        let mut series = HarmonicSeries::new(6, 4);

        assert_eq!(series.fundamental, 6);
        assert_eq!(series.overtones, vec![12, 18, 24]);
    }

    #[test]
    fn test_overtone_order() {
        let series = HarmonicSeries::new(6, 4);

        assert_eq!(series.overtone_order(12), Some(2));
        assert_eq!(series.overtone_order(18), Some(3));
        assert_eq!(series.overtone_order(24), Some(4));
        assert_eq!(series.overtone_order(7), None);
    }

    #[test]
    fn test_amplitude_decay() {
        let mut series = HarmonicSeries::new(6, 3);

        series.set_fundamental_rate(0.30);
        series.record_overtone(12, 0.20);
        series.record_overtone(18, 0.15);

        let (slope, r2) = series.amplitude_decay();

        // Slope should be negative (decay)
        assert!(slope < 0.0);

        // R² should be reasonably high for linear decay
        assert!(r2 > 0.8);
    }

    #[test]
    fn test_coherent_resonance() {
        let mut series = HarmonicSeries::new(6, 3);

        series.set_fundamental_rate(0.30);
        series.record_overtone(12, 0.20); // > 50% of fundamental
        series.record_overtone(18, 0.16); // > 50% of fundamental

        assert!(series.has_coherent_resonance());
    }

    #[test]
    fn test_overtone_enhancement() {
        let mut series = HarmonicSeries::new(6, 2);

        series.set_fundamental_rate(0.20);
        series.record_overtone(12, 0.25); // Stronger than fundamental!

        assert!(series.has_overtone_enhancement());
    }

    #[test]
    fn test_configuration_persistence() {
        let mut config = ConfigurationHarmonic::new(6, 1, 5, 0, 0);

        config.record_rate(6, 0.30);
        config.record_rate(12, 0.25); // 83% of fundamental
        config.record_rate(18, 0.24); // 80% of fundamental

        let score = config.persistence_score();
        assert!(score > 0.9); // Both overtones maintain >80%
    }
}
