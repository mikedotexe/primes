//! Adaptive membrane construction
//!
//! Base-specific optimized configurations that adapt to different number systems

use super::{construct_symmetric_membrane, MembraneConfig, OptimizationTarget};
use crate::PhysicsResult;

/// Construct an adaptive membrane optimized for the specific base and target
pub fn construct_adaptive_membrane(
    config: &MembraneConfig,
    middle: &str,
    target: &OptimizationTarget,
) -> PhysicsResult<String> {
    // Get optimized parameters for this base and target
    let (outer, inner, k_outer, k_inner) = get_adaptive_parameters(config.base, target);

    // Use symmetric construction with optimized parameters
    construct_symmetric_membrane(outer, inner, middle, k_outer, k_inner)
}

/// Get optimized parameters for a specific base and optimization target
fn get_adaptive_parameters(base: u32, target: &OptimizationTarget) -> (u32, u32, u32, u32) {
    match (base, target) {
        // Base 10 optimizations
        (10, OptimizationTarget::MaxDensity) => (3, 7, 2, 2), // The magical 18.55%
        (10, OptimizationTarget::PatternMatching(patterns)) => {
            if patterns
                .iter()
                .any(|p| p.contains("37") || p.contains("73"))
            {
                (3, 7, 1, 1) // Emphasize 37/73 patterns
            } else if patterns.iter().any(|p| p.contains("palindrome")) {
                (3, 3, 2, 2) // Twin boundaries for palindromes
            } else {
                (3, 7, 2, 2) // Default best
            }
        }
        (10, OptimizationTarget::MinLength) => (3, 7, 0, 0), // Minimal structure
        (10, OptimizationTarget::Balanced) => (3, 7, 1, 1),  // Good compromise

        // Base 11 optimizations
        (11, OptimizationTarget::MaxDensity) => (3, 8, 2, 2),
        (11, _) => (3, 8, 1, 1),

        // Base 12 optimizations (bridge configurations)
        (12, OptimizationTarget::MaxDensity) => (5, 7, 2, 2), // Bridge config
        (12, _) => (5, 6, 1, 1),                              // Mixed center

        // Odd bases (discovered to work better)
        (base, _) if base % 2 == 1 => {
            let mid = base / 2;
            (mid.saturating_sub(1), mid + 1, 1, 1)
        }

        // Even bases (generally problematic)
        (base, _) => {
            // Use edge pair strategy
            let quarter = base / 4;
            (quarter, base - quarter - 1, 1, 1)
        }
    }
}

/// Adaptive configuration analyzer
#[derive(Debug, Clone)]
pub struct AdaptiveAnalysis {
    pub base: u32,
    pub configuration: (u32, u32, u32, u32),
    pub optimization_target: OptimizationTarget,
    pub predicted_density: f64,
    pub base_compatibility: f64,
    pub edge_pair_score: f64,
}

impl AdaptiveAnalysis {
    /// Create analysis for a given configuration
    pub fn new(base: u32, config: (u32, u32, u32, u32), target: OptimizationTarget) -> Self {
        let predicted_density = predict_density(base, config);
        let base_compatibility = calculate_base_compatibility(base, config);
        let edge_pair_score = calculate_edge_pair_score(base, config);

        Self {
            base,
            configuration: config,
            optimization_target: target,
            predicted_density,
            base_compatibility,
            edge_pair_score,
        }
    }

    /// Overall adaptation score
    pub fn adaptation_score(&self) -> f64 {
        self.predicted_density * self.base_compatibility * (1.0 + self.edge_pair_score)
    }

    /// Check if this configuration uses edge pairs
    pub fn uses_edge_pairs(&self) -> bool {
        let (outer, inner, _, _) = self.configuration;

        // Check if outer and inner are equidistant from base boundaries
        let outer_dist = outer.min(self.base.saturating_sub(outer));
        let inner_dist = inner.min(self.base.saturating_sub(inner));

        outer_dist == inner_dist || (outer + inner + 1) == self.base
    }

    /// Get configuration description
    pub fn description(&self) -> String {
        let (outer, inner, k_outer, k_inner) = self.configuration;
        format!(
            "Base {} adaptive: ({},{}) k=({},{}) [score: {:.3}, edge_pairs: {}]",
            self.base,
            outer,
            inner,
            k_outer,
            k_inner,
            self.adaptation_score(),
            self.uses_edge_pairs()
        )
    }
}

/// Predict prime density for a configuration in a specific base
fn predict_density(base: u32, config: (u32, u32, u32, u32)) -> f64 {
    let (outer, inner, k_outer, k_inner) = config;

    // Base density factors from research
    let base_factor = match base {
        10 => 1.0,                 // Reference base
        11 => 0.6,                 // Prime base, moderate performance
        12 => 0.3,                 // Even base, poor performance
        9 => 0.8,                  // Odd composite, good performance
        15 => 0.7,                 // Odd composite
        _ if base % 2 == 1 => 0.5, // Other odd bases
        _ => 0.2,                  // Other even bases
    };

    // Boundary digit factors
    let boundary_factor = if base == 10 {
        match (outer, inner) {
            (3, 7) | (7, 3) => 1.8, // The magical combination
            (3, _) => 1.4,          // 3 is special
            (_, 7) => 1.2,          // 7 is good too
            _ => 1.0,
        }
    } else {
        // Check for edge pairs in other bases
        let is_edge_pair = (outer + inner + 1) == base;
        if is_edge_pair {
            1.5
        } else {
            1.0
        }
    };

    // K-value factors (some patterns work better)
    let k_factor = match (k_outer, k_inner) {
        (2, 2) => 1.1, // Often optimal
        (1, 1) => 1.0, // Good balance
        (0, 0) => 0.8, // Minimal padding
        _ => 0.9,
    };

    // Baseline prime density (roughly 1/ln(n))
    let baseline = 0.05;

    baseline * base_factor * boundary_factor * k_factor
}

/// Calculate how compatible this configuration is with the base
fn calculate_base_compatibility(base: u32, config: (u32, u32, u32, u32)) -> f64 {
    let (outer, inner, _, _) = config;

    // Check if digits are valid for this base
    if outer >= base || inner >= base {
        return 0.0;
    }

    // Bonus for using significant positions in the base
    let outer_significance = if outer == 0 || outer == base - 1 {
        0.5 // Boundary positions
    } else if outer == base / 2 {
        0.3 // Center position
    } else {
        1.0 // Other positions
    };

    let inner_significance = if inner == 0 || inner == base - 1 {
        0.5
    } else if inner == base / 2 {
        0.3
    } else {
        1.0
    };

    (outer_significance + inner_significance) / 2.0
}

/// Calculate edge pair score
fn calculate_edge_pair_score(base: u32, config: (u32, u32, u32, u32)) -> f64 {
    let (outer, inner, _, _) = config;

    // Perfect edge pair: equidistant from boundaries
    let outer_dist_from_start = outer;
    let outer_dist_from_end = base.saturating_sub(outer + 1);
    let inner_dist_from_start = inner;
    let inner_dist_from_end = base.saturating_sub(inner + 1);

    let outer_edge_score = if outer_dist_from_start == outer_dist_from_end {
        1.0 // Perfect edge position
    } else {
        let min_dist = outer_dist_from_start.min(outer_dist_from_end);
        let max_dist = outer_dist_from_start.max(outer_dist_from_end);
        1.0 - (max_dist - min_dist) as f64 / base as f64
    };

    let inner_edge_score = if inner_dist_from_start == inner_dist_from_end {
        1.0
    } else {
        let min_dist = inner_dist_from_start.min(inner_dist_from_end);
        let max_dist = inner_dist_from_start.max(inner_dist_from_end);
        1.0 - (max_dist - min_dist) as f64 / base as f64
    };

    (outer_edge_score + inner_edge_score) / 2.0
}

/// Generate all reasonable adaptive configurations for a base
pub fn generate_adaptive_configs(base: u32) -> Vec<AdaptiveAnalysis> {
    let mut configs = Vec::new();

    let targets = vec![
        OptimizationTarget::MaxDensity,
        OptimizationTarget::Balanced,
        OptimizationTarget::MinLength,
        OptimizationTarget::PatternMatching(vec!["37".to_string(), "73".to_string()]),
    ];

    for target in targets {
        let config = get_adaptive_parameters(base, &target);
        configs.push(AdaptiveAnalysis::new(base, config, target));
    }

    // Sort by adaptation score
    configs.sort_by(|a, b| {
        b.adaptation_score()
            .partial_cmp(&a.adaptation_score())
            .unwrap()
    });

    configs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_base_10() {
        let config = get_adaptive_parameters(10, &OptimizationTarget::MaxDensity);
        assert_eq!(config, (3, 7, 2, 2)); // Should be the magical combination
    }

    #[test]
    fn test_edge_pair_detection() {
        let analysis = AdaptiveAnalysis::new(10, (3, 7, 2, 2), OptimizationTarget::MaxDensity);

        // 3 and 7 are edge pairs in base 10 (3 from 0, 3 from 10)
        assert!(analysis.uses_edge_pairs());
        assert!(analysis.adaptation_score() > 0.1);
    }

    #[test]
    fn test_density_prediction() {
        let density = predict_density(10, (3, 7, 2, 2));
        assert!(density > 0.05); // Should be higher than baseline

        let bad_density = predict_density(12, (4, 8, 2, 2));
        assert!(bad_density < density); // Even base should be worse
    }
}
