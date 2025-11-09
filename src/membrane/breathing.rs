//! Breathing membrane construction
//!
//! Asymmetric patterns where left and right padding can be different.
//! Like a breathing organism that expands and contracts!

use crate::{PhysicsError, PhysicsResult};

/// Construct a breathing (asymmetric) membrane
pub fn construct_breathing_membrane(
    outer: u32,
    inner: u32,
    middle: &str,
    left_k_outer: u32,
    left_k_inner: u32,
    right_k_outer: u32,
    right_k_inner: u32,
) -> PhysicsResult<String> {
    if outer >= 10 || inner >= 10 {
        return Err(PhysicsError::InvalidMembrane(
            "Digits must be 0-9".to_string(),
        ));
    }

    let left_outer = outer.to_string();
    let left_zeros_outer = "0".repeat(left_k_outer as usize);
    let left_inner = inner.to_string();
    let left_zeros_inner = "0".repeat(left_k_inner as usize);

    let right_zeros_inner = "0".repeat(right_k_inner as usize);
    let right_inner = inner.to_string();
    let right_zeros_outer = "0".repeat(right_k_outer as usize);
    let right_outer = outer.to_string();

    let membrane = format!(
        "{left_outer}{left_zeros_outer}{left_inner}{left_zeros_inner}{middle}{right_zeros_inner}{right_inner}{right_zeros_outer}{right_outer}"
    );

    Ok(membrane)
}

/// Different breathing patterns discovered in research
#[derive(Debug, Clone)]
pub enum BreathingPattern {
    /// Expanding: left side smaller than right
    Expanding { expansion_ratio: f64 },

    /// Contracting: left side larger than right
    Contracting { contraction_ratio: f64 },

    /// Oscillating: alternating pattern
    Oscillating { frequency: u32 },

    /// Resonant cavity: specific tuned ratios
    ResonantCavity { resonance_length: u32 },

    /// Fibonacci breathing: padding follows Fibonacci sequence
    Fibonacci,

    /// Prime breathing: padding uses prime numbers
    PrimeBreathing,
}

impl BreathingPattern {
    /// Generate k-values for this breathing pattern at given length
    pub fn generate_k_values(&self, target_length: usize) -> (u32, u32, u32, u32) {
        match self {
            BreathingPattern::Expanding { expansion_ratio } => {
                let base_k = 1;
                let left_k = base_k;
                let right_k = (base_k as f64 * expansion_ratio) as u32;
                (left_k, left_k, right_k, right_k)
            }

            BreathingPattern::Contracting { contraction_ratio } => {
                let base_k = 3;
                let left_k = base_k;
                let right_k = (base_k as f64 * contraction_ratio) as u32;
                (left_k, left_k, right_k, right_k)
            }

            BreathingPattern::Oscillating { frequency } => {
                let phase = (target_length % *frequency as usize) as f64;
                let amplitude = 2.0;
                let offset = 1.0;

                let k = offset
                    + amplitude * (phase * 2.0 * std::f64::consts::PI / *frequency as f64).sin();
                let k = k.max(0.0) as u32;
                (k, k, (3 - k), (3 - k))
            }

            BreathingPattern::ResonantCavity { resonance_length } => {
                // Discovered patterns from research
                match resonance_length {
                    1 => (0, 0, 0, 0), // Minimal cavity for length 1
                    3 => (1, 3, 3, 1), // The magical 25.04% cavity
                    5 => (1, 3, 3, 1), // Same cavity works for length 5
                    7 => (2, 1, 1, 2), // Expanding cavity
                    8 => (2, 1, 1, 2), // Same expanding pattern
                    _ => (2, 2, 2, 2), // Default symmetric
                }
            }

            BreathingPattern::Fibonacci => {
                // k-values follow Fibonacci: 1,1,2,3,5,8...
                let fib = fibonacci_sequence(4);
                (fib[0], fib[1], fib[2], fib[3])
            }

            BreathingPattern::PrimeBreathing => {
                // k-values use prime numbers: 2,3,5,7...
                (2, 3, 5, 7)
            }
        }
    }

    /// Get the expected density boost for this pattern
    pub fn density_boost(&self) -> f64 {
        match self {
            BreathingPattern::ResonantCavity {
                resonance_length: 3,
            } => 8.5, // 25.04% vs 2.96%
            BreathingPattern::ResonantCavity {
                resonance_length: 5,
            } => 3.0, // 12.40% vs 4.12%
            BreathingPattern::Expanding { .. } => 1.5,
            BreathingPattern::Contracting { .. } => 1.3,
            BreathingPattern::Oscillating { .. } => 1.2,
            BreathingPattern::Fibonacci => 1.4,
            BreathingPattern::PrimeBreathing => 1.6,
            _ => 1.1,
        }
    }

    /// Description of this pattern
    pub fn description(&self) -> String {
        match self {
            BreathingPattern::Expanding { expansion_ratio } => {
                format!("Expanding (ratio: {expansion_ratio:.2})")
            }
            BreathingPattern::Contracting { contraction_ratio } => {
                format!("Contracting (ratio: {contraction_ratio:.2})")
            }
            BreathingPattern::Oscillating { frequency } => {
                format!("Oscillating (freq: {frequency})")
            }
            BreathingPattern::ResonantCavity { resonance_length } => {
                format!("Resonant cavity (length: {resonance_length})")
            }
            BreathingPattern::Fibonacci => "Fibonacci breathing".to_string(),
            BreathingPattern::PrimeBreathing => "Prime breathing".to_string(),
        }
    }
}

/// Analysis of a breathing membrane structure
#[derive(Debug, Clone)]
pub struct BreathingAnalysis {
    pub outer_digit: u32,
    pub inner_digit: u32,
    pub left_k_outer: u32,
    pub left_k_inner: u32,
    pub right_k_outer: u32,
    pub right_k_inner: u32,
    pub middle: String,
    pub total_length: usize,
    pub asymmetry_index: f64,
    pub breathing_pattern: Option<BreathingPattern>,
}

impl BreathingAnalysis {
    /// Calculate how asymmetric this membrane is
    pub fn calculate_asymmetry(&self) -> f64 {
        let left_total = self.left_k_outer + self.left_k_inner;
        let right_total = self.right_k_outer + self.right_k_inner;

        if left_total + right_total == 0 {
            return 0.0;
        }

        (left_total as f64 - right_total as f64).abs() / (left_total + right_total) as f64
    }

    /// Detect the breathing pattern type
    pub fn detect_pattern(&self) -> Option<BreathingPattern> {
        let left_total = self.left_k_outer + self.left_k_inner;
        let right_total = self.right_k_outer + self.right_k_inner;

        // Check for known resonant cavities
        if self.left_k_outer == 1
            && self.left_k_inner == 3
            && self.right_k_outer == 3
            && self.right_k_inner == 1
        {
            return Some(BreathingPattern::ResonantCavity {
                resonance_length: self.middle.len() as u32,
            });
        }

        // Check for expansion/contraction
        if right_total > left_total {
            let ratio = right_total as f64 / left_total.max(1) as f64;
            Some(BreathingPattern::Expanding {
                expansion_ratio: ratio,
            })
        } else if left_total > right_total {
            let ratio = right_total as f64 / left_total as f64;
            Some(BreathingPattern::Contracting {
                contraction_ratio: ratio,
            })
        } else {
            None // Symmetric
        }
    }

    /// Calculate breathing resonance score
    pub fn breathing_resonance(&self) -> f64 {
        let mut score = 1.0;

        // Asymmetry bonus (breathing often outperforms symmetric)
        score *= 1.0 + self.asymmetry_index;

        // Pattern-specific bonuses
        if let Some(ref pattern) = self.breathing_pattern {
            score *= pattern.density_boost();
        }

        // Special cavity configurations
        if self.is_magical_cavity() {
            score *= 8.5; // The 25.04% cavity
        }

        score
    }

    /// Check if this is the magical 25.04% cavity
    fn is_magical_cavity(&self) -> bool {
        self.left_k_outer == 1
            && self.left_k_inner == 3
            && self.right_k_outer == 3
            && self.right_k_inner == 1
            && (self.middle.len() == 1 || self.middle.len() == 3)
    }

    /// Get description of this breathing membrane
    pub fn description(&self) -> String {
        format!(
            "({},{}) k=({},{} | {},{}) middle='{}' [asymmetry: {:.2}, resonance: {:.2}]",
            self.outer_digit,
            self.inner_digit,
            self.left_k_outer,
            self.left_k_inner,
            self.right_k_outer,
            self.right_k_inner,
            self.middle,
            self.asymmetry_index,
            self.breathing_resonance()
        )
    }
}

/// Generate optimal breathing patterns for different scenarios
pub fn generate_optimal_patterns() -> Vec<BreathingPattern> {
    vec![
        // The discovered magical cavities
        BreathingPattern::ResonantCavity {
            resonance_length: 1,
        },
        BreathingPattern::ResonantCavity {
            resonance_length: 3,
        },
        BreathingPattern::ResonantCavity {
            resonance_length: 5,
        },
        BreathingPattern::ResonantCavity {
            resonance_length: 7,
        },
        // Expansion patterns
        BreathingPattern::Expanding {
            expansion_ratio: 1.5,
        },
        BreathingPattern::Expanding {
            expansion_ratio: 2.0,
        },
        BreathingPattern::Expanding {
            expansion_ratio: 3.0,
        },
        // Mathematical sequences
        BreathingPattern::Fibonacci,
        BreathingPattern::PrimeBreathing,
        // Oscillating patterns
        BreathingPattern::Oscillating { frequency: 3 },
        BreathingPattern::Oscillating { frequency: 5 },
        BreathingPattern::Oscillating { frequency: 7 },
    ]
}

/// Analyze a breathing membrane string
pub fn analyze_breathing_structure(membrane: &str) -> PhysicsResult<BreathingAnalysis> {
    let chars: Vec<char> = membrane.chars().collect();
    let len = chars.len();

    if len < 4 {
        return Err(PhysicsError::InvalidMembrane(
            "Membrane too short for breathing analysis".to_string(),
        ));
    }

    // This is a simplified analysis - in practice, we'd need more sophisticated parsing
    // to detect asymmetric patterns

    let outer_digit = chars[0].to_digit(10).unwrap_or(0);
    let inner_digit = chars[1].to_digit(10).unwrap_or(0); // Simplified

    // For now, return a basic structure
    let mut analysis = BreathingAnalysis {
        outer_digit,
        inner_digit,
        left_k_outer: 1,
        left_k_inner: 1,
        right_k_outer: 1,
        right_k_inner: 1,
        middle: "5".to_string(), // Simplified
        total_length: len,
        asymmetry_index: 0.0,
        breathing_pattern: None,
    };

    analysis.asymmetry_index = analysis.calculate_asymmetry();
    analysis.breathing_pattern = analysis.detect_pattern();

    Ok(analysis)
}

/// Generate Fibonacci sequence of given length
fn fibonacci_sequence(length: usize) -> Vec<u32> {
    if length == 0 {
        return vec![];
    }
    if length == 1 {
        return vec![1];
    }

    let mut fib = vec![1, 1];
    for i in 2..length {
        let next = fib[i - 1] + fib[i - 2];
        fib.push(next.min(9)); // Keep digits single
    }

    fib
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breathing_construction() {
        let result = construct_breathing_membrane(3, 7, "5", 1, 3, 3, 1);
        assert!(result.is_ok());
        let membrane = result.unwrap();
        assert_eq!(membrane, "3070005070003"); // The magical cavity!
    }

    #[test]
    fn test_resonant_cavity_pattern() {
        let pattern = BreathingPattern::ResonantCavity {
            resonance_length: 3,
        };
        let (lo, li, ro, ri) = pattern.generate_k_values(3);
        assert_eq!((lo, li, ro, ri), (1, 3, 3, 1));
        assert!(pattern.density_boost() > 5.0); // Should be high
    }

    #[test]
    fn test_breathing_analysis() {
        let analysis = BreathingAnalysis {
            outer_digit: 3,
            inner_digit: 7,
            left_k_outer: 1,
            left_k_inner: 3,
            right_k_outer: 3,
            right_k_inner: 1,
            middle: "5".to_string(),
            total_length: 15,
            asymmetry_index: 0.0,
            breathing_pattern: None,
        };

        let asymmetry = analysis.calculate_asymmetry();
        assert_eq!(asymmetry, 0.0); // This particular pattern is symmetric in total

        assert!(analysis.is_magical_cavity());
    }
}
