//! # Failure Analysis System
//! 
//! This module analyzes configurations that DON'T work, providing crucial insights
//! into why certain patterns fail to generate primes. Understanding failures is
//! as important as understanding successes.
//! 
//! ## Key Discoveries from Failures
//! 
//! - Digit 5 as boundary: 0% success (it's the wave node, not antinode)
//! - Base 12 native configs: 0% success (powers of 2 in boundaries)
//! - Random boundaries: <1% success (no resonance structure)
//! - Asymmetric structures: Usually fail (break standing wave)

use num_bigint::BigUint;
use std::collections::HashMap;
use crate::membrane::{MembraneConfig, ConstructionType};

/// Categories of failure modes
#[derive(Debug, Clone, PartialEq)]
pub enum FailureMode {
    /// Complete failure - 0% success rate
    CompleteFailure {
        reason: String,
    },
    
    /// Very low density - below random expectation
    BelowRandom {
        actual_rate: f64,
        expected_random: f64,
    },
    
    /// Specific digit exclusion - only certain digits work
    DigitExclusion {
        working_digits: Vec<u32>,
        failed_digits: Vec<u32>,
    },
    
    /// Base incompatibility - fails in specific bases
    BaseIncompatible {
        failed_bases: Vec<u32>,
        working_bases: Vec<u32>,
    },
    
    /// Structural defect - specific pattern issue
    StructuralDefect {
        defect_type: String,
        description: String,
    },
}

/// Analysis results for a failed configuration
#[derive(Debug, Clone)]
pub struct FailureAnalysis {
    /// The configuration that failed
    pub config: MembraneConfig,
    
    /// Primary failure mode
    pub failure_mode: FailureMode,
    
    /// Success rate achieved
    pub success_rate: f64,
    
    /// Number of primes found (if any)
    pub primes_found: usize,
    
    /// Total candidates tested
    pub candidates_tested: usize,
    
    /// Detailed breakdown by parameter
    pub parameter_analysis: HashMap<String, f64>,
    
    /// Lessons learned from this failure
    pub lessons: Vec<String>,
    
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}

/// Failure analyzer
pub struct FailureAnalyzer {
    /// Track all analyzed failures
    pub failure_database: Vec<FailureAnalysis>,
    
    /// Patterns found across failures
    pub failure_patterns: HashMap<String, usize>,
}

impl Default for FailureAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl FailureAnalyzer {
    pub fn new() -> Self {
        Self {
            failure_database: Vec::new(),
            failure_patterns: HashMap::new(),
        }
    }
    
    /// Analyze a configuration that performed poorly
    pub fn analyze_failure(
        &mut self,
        config: &MembraneConfig,
        results: &[(BigUint, bool)],
    ) -> FailureAnalysis {
        let primes_found = results.iter().filter(|(_, is_prime)| *is_prime).count();
        let success_rate = primes_found as f64 / results.len() as f64;
        
        // Determine failure mode
        let failure_mode = self.categorize_failure(config, success_rate, results);
        
        // Analyze parameters
        let parameter_analysis = self.analyze_parameters(config, results);
        
        // Extract lessons
        let lessons = self.extract_lessons(&failure_mode, config, &parameter_analysis);
        
        // Generate suggestions
        let suggestions = self.generate_suggestions(&failure_mode, config);
        
        let analysis = FailureAnalysis {
            config: config.clone(),
            failure_mode,
            success_rate,
            primes_found,
            candidates_tested: results.len(),
            parameter_analysis,
            lessons,
            suggestions,
        };
        
        // Update pattern database
        self.update_patterns(&analysis);
        self.failure_database.push(analysis.clone());
        
        analysis
    }
    
    /// Categorize the type of failure
    fn categorize_failure(
        &self,
        config: &MembraneConfig,
        success_rate: f64,
        results: &[(BigUint, bool)],
    ) -> FailureMode {
        // Complete failure
        if success_rate == 0.0 {
            let reason = self.diagnose_complete_failure(config);
            return FailureMode::CompleteFailure { reason };
        }
        
        // Check for digit exclusion
        if let Some(exclusion) = self.check_digit_exclusion(results) {
            return exclusion;
        }
        
        // Below random expectation
        let expected_random = self.calculate_expected_random(config);
        if success_rate < expected_random {
            return FailureMode::BelowRandom {
                actual_rate: success_rate,
                expected_random,
            };
        }
        
        // Check for structural defects
        if let Some(defect) = self.check_structural_defects(config) {
            return defect;
        }
        
        // Default: generic low performance
        FailureMode::StructuralDefect {
            defect_type: "Unknown".to_string(),
            description: "Configuration underperforms without clear reason".to_string(),
        }
    }
    
    /// Diagnose why a configuration completely failed
    fn diagnose_complete_failure(&self, config: &MembraneConfig) -> String {
        // Check for known fatal patterns
        
        // 5 as boundary in base 10
        if config.base == 10 && config.outer == 5 {
            return "Digit 5 as boundary in base 10 creates wave node (center point), not antinode".to_string();
        }
        
        // Powers of 2 in even bases
        if config.base % 2 == 0 {
            let powers_of_2 = [2, 4, 8, 16];
            if powers_of_2.contains(&config.outer) || powers_of_2.contains(&config.inner) {
                return format!("Power of 2 as boundary in even base {} creates systematic divisibility", config.base);
            }
        }
        
        // Zero padding too large
        if config.k_outer > 10 || config.k_inner > 10 {
            return "Excessive zero padding (k > 10) dilutes prime density below detection".to_string();
        }
        
        "Unknown complete failure - requires investigation".to_string()
    }
    
    /// Check if only specific digits produce primes
    fn check_digit_exclusion(&self, results: &[(BigUint, bool)]) -> Option<FailureMode> {
        let mut digit_success: HashMap<u32, (usize, usize)> = HashMap::new();
        
        // Analyze last digit of middle section
        for (num, is_prime) in results {
            let num_str = num.to_string();
            if let Some(last_char) = num_str.chars().last() {
                if let Some(digit) = last_char.to_digit(10) {
                    let entry = digit_success.entry(digit % 10).or_insert((0, 0));
                    entry.1 += 1; // Total count
                    if *is_prime {
                        entry.0 += 1; // Success count
                    }
                }
            }
        }
        
        let working_digits: Vec<u32> = digit_success.iter()
            .filter(|(_, (success, total))| *success > 0 && *total > 0)
            .map(|(digit, _)| *digit)
            .collect();
            
        let failed_digits: Vec<u32> = digit_success.iter()
            .filter(|(_, (success, total))| *success == 0 && *total > 0)
            .map(|(digit, _)| *digit)
            .collect();
        
        if !failed_digits.is_empty() && working_digits.len() < 5 {
            return Some(FailureMode::DigitExclusion {
                working_digits,
                failed_digits,
            });
        }
        
        None
    }
    
    /// Check for structural defects in configuration
    fn check_structural_defects(&self, config: &MembraneConfig) -> Option<FailureMode> {
        // Check for asymmetry issues
        if let ConstructionType::Breathing { left_k_outer, left_k_inner, right_k_outer, right_k_inner } = config.construction_type {
            let asymmetry = ((left_k_outer as i32 - right_k_outer as i32).abs() +
                           (left_k_inner as i32 - right_k_inner as i32).abs()) as f64;
            
            if asymmetry > 5.0 {
                return Some(FailureMode::StructuralDefect {
                    defect_type: "Excessive Asymmetry".to_string(),
                    description: format!("Left/right imbalance of {asymmetry} breaks wave symmetry"),
                });
            }
        }
        
        // Check for incompatible digit pairs
        if self.is_incompatible_pair(config.outer, config.inner, config.base) {
            return Some(FailureMode::StructuralDefect {
                defect_type: "Incompatible Boundaries".to_string(),
                description: format!("Digits {} and {} create destructive interference in base {}", 
                    config.outer, config.inner, config.base),
            });
        }
        
        None
    }
    
    /// Check if digit pair is incompatible
    fn is_incompatible_pair(&self, digit1: u32, digit2: u32, base: u32) -> bool {
        // Both even in even base
        if base % 2 == 0 && digit1 % 2 == 0 && digit2 % 2 == 0 {
            return true;
        }
        
        // Both multiples of 5 in base 10
        if base == 10 && digit1 % 5 == 0 && digit2 % 5 == 0 {
            return true;
        }
        
        false
    }
    
    /// Calculate expected random success rate
    fn calculate_expected_random(&self, config: &MembraneConfig) -> f64 {
        let total_digits = config.total_digits();
        // Prime number theorem approximation
        1.0 / (total_digits as f64 * 2.303) // ln(10) ≈ 2.303
    }
    
    /// Analyze parameters in detail
    fn analyze_parameters(
        &self,
        config: &MembraneConfig,
        results: &[(BigUint, bool)],
    ) -> HashMap<String, f64> {
        let mut analysis = HashMap::new();
        
        // Basic metrics
        let success_rate = results.iter().filter(|(_, p)| *p).count() as f64 / results.len() as f64;
        analysis.insert("success_rate".to_string(), success_rate);
        analysis.insert("total_digits".to_string(), config.total_digits() as f64);
        
        // Structure percentage
        let structure_pct = self.calculate_structure_percentage(config);
        analysis.insert("structure_percentage".to_string(), structure_pct);
        
        // Boundary analysis
        let boundary_sum = config.outer + config.inner;
        analysis.insert("boundary_digit_sum".to_string(), boundary_sum as f64);
        
        // K-value metrics
        let k_values = match &config.construction_type {
            ConstructionType::Breathing { left_k_outer, left_k_inner, right_k_outer, right_k_inner } => 
                vec![*left_k_outer, *left_k_inner, *right_k_outer, *right_k_inner],
            _ => vec![config.k_outer, config.k_inner],
        };
        let k_sum: u32 = k_values.iter().sum();
        let k_variance = self.calculate_variance(&k_values);
        analysis.insert("k_sum".to_string(), k_sum as f64);
        analysis.insert("k_variance".to_string(), k_variance);
        
        analysis
    }
    
    /// Calculate structure percentage
    fn calculate_structure_percentage(&self, config: &MembraneConfig) -> f64 {
        let boundary_digits = 4; // Two outer, two inner
        let zeros = match &config.construction_type {
            ConstructionType::Breathing { left_k_outer, left_k_inner, right_k_outer, right_k_inner } => 
                left_k_outer + left_k_inner + right_k_outer + right_k_inner,
            _ => (config.k_outer + config.k_inner) * 2,
        };
        let total = config.total_digits() as u32;
        
        (boundary_digits + zeros) as f64 / total as f64 * 100.0
    }
    
    /// Calculate variance of k-values
    fn calculate_variance(&self, values: &[u32]) -> f64 {
        let mean = values.iter().sum::<u32>() as f64 / values.len() as f64;
        let variance = values.iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        variance.sqrt()
    }
    
    /// Extract lessons from failure
    fn extract_lessons(
        &self,
        failure_mode: &FailureMode,
        config: &MembraneConfig,
        params: &HashMap<String, f64>,
    ) -> Vec<String> {
        let mut lessons = Vec::new();
        
        match failure_mode {
            FailureMode::CompleteFailure { reason } => {
                lessons.push(format!("Complete failure teaches: {reason}"));
                lessons.push("This configuration creates systematic composite generation".to_string());
            }
            
            FailureMode::DigitExclusion { working_digits, failed_digits } => {
                lessons.push(format!("Only digits {working_digits:?} work in this configuration"));
                lessons.push(format!("Digits {failed_digits:?} are systematically excluded"));
                lessons.push("Configuration creates selective resonance affecting specific digits".to_string());
            }
            
            FailureMode::BelowRandom { actual_rate, expected_random } => {
                lessons.push(format!("Performance {:.2}% vs random {:.4}% suggests active suppression", 
                    actual_rate * 100.0, expected_random * 100.0));
                lessons.push("Structure may create destructive interference".to_string());
            }
            
            FailureMode::StructuralDefect { defect_type, description } => {
                lessons.push(format!("{defect_type}: {description}"));
                if let Some(&structure_pct) = params.get("structure_percentage") {
                    if structure_pct > 90.0 {
                        lessons.push("Excessive structure (>90%) enters volatile resonance regime".to_string());
                    }
                }
            }
            
            _ => {}
        }
        
        // General lessons
        if config.base % 2 == 0 {
            lessons.push("Even bases generally suppress prime generation".to_string());
        }
        
        lessons
    }
    
    /// Generate improvement suggestions
    fn generate_suggestions(
        &self,
        failure_mode: &FailureMode,
        config: &MembraneConfig,
    ) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        match failure_mode {
            FailureMode::CompleteFailure { reason } => {
                if reason.contains("digit 5") {
                    suggestions.push("Move 5 to middle position where it excels".to_string());
                    suggestions.push("Use edge pairs like (3,7) for boundaries".to_string());
                }
                if reason.contains("power of 2") {
                    suggestions.push("Avoid powers of 2 in boundaries".to_string());
                    suggestions.push("Use odd primes for boundary digits".to_string());
                }
            }
            
            FailureMode::DigitExclusion { working_digits, .. } => {
                suggestions.push(format!("Focus on configurations that emphasize digits {working_digits:?}"));
                suggestions.push("Study why these specific digits resonate".to_string());
            }
            
            FailureMode::StructuralDefect { defect_type, .. } => {
                if defect_type.contains("Asymmetry") {
                    suggestions.push("Return to symmetric configuration".to_string());
                    suggestions.push("Or explore controlled asymmetry with k-difference ≤ 2".to_string());
                }
            }
            
            _ => {}
        }
        
        // Universal suggestions
        if config.base != 10 {
            suggestions.push("Test this configuration in base 10 for comparison".to_string());
        }
        suggestions.push("Try classic (3,7) boundaries as control".to_string());
        
        suggestions
    }
    
    /// Update pattern database
    fn update_patterns(&mut self, analysis: &FailureAnalysis) {
        // Track failure mode frequency
        let mode_key = format!("{:?}", analysis.failure_mode);
        *self.failure_patterns.entry(mode_key).or_insert(0) += 1;
        
        // Track specific failure combinations
        let key = format!("outer_{}_base_{}", analysis.config.outer, analysis.config.base);
        *self.failure_patterns.entry(key).or_insert(0) += 1;
        
        let key = format!("inner_{}_base_{}", analysis.config.inner, analysis.config.base);
        *self.failure_patterns.entry(key).or_insert(0) += 1;
    }
    
    /// Generate comprehensive failure report
    pub fn generate_failure_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("\n{}\n", "=".repeat(60)));
        report.push_str("FAILURE ANALYSIS REPORT\n");
        report.push_str(&format!("{}\n\n", "=".repeat(60)));
        
        report.push_str(&format!("Total Failures Analyzed: {}\n\n", self.failure_database.len()));
        
        // Failure mode breakdown
        report.push_str("FAILURE MODES:\n");
        let mut mode_counts: HashMap<String, usize> = HashMap::new();
        for failure in &self.failure_database {
            let mode = match &failure.failure_mode {
                FailureMode::CompleteFailure { .. } => "Complete Failure",
                FailureMode::BelowRandom { .. } => "Below Random",
                FailureMode::DigitExclusion { .. } => "Digit Exclusion",
                FailureMode::BaseIncompatible { .. } => "Base Incompatible",
                FailureMode::StructuralDefect { .. } => "Structural Defect",
            };
            *mode_counts.entry(mode.to_string()).or_insert(0) += 1;
        }
        
        for (mode, count) in mode_counts {
            report.push_str(&format!("  {}: {} ({:.1}%)\n", 
                mode, count, count as f64 / self.failure_database.len() as f64 * 100.0));
        }
        
        // Most common patterns
        report.push_str("\nMOST COMMON FAILURE PATTERNS:\n");
        let mut patterns: Vec<(&String, &usize)> = self.failure_patterns.iter().collect();
        patterns.sort_by(|a, b| b.1.cmp(a.1));
        
        for (pattern, count) in patterns.iter().take(10) {
            report.push_str(&format!("  {pattern}: {count}\n"));
        }
        
        // Key lessons
        report.push_str("\nKEY LESSONS FROM FAILURES:\n");
        let all_lessons: Vec<String> = self.failure_database.iter()
            .flat_map(|f| f.lessons.clone())
            .collect();
        
        let mut lesson_counts: HashMap<String, usize> = HashMap::new();
        for lesson in all_lessons {
            *lesson_counts.entry(lesson).or_insert(0) += 1;
        }
        
        let mut top_lessons: Vec<(String, usize)> = lesson_counts.into_iter().collect();
        top_lessons.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (lesson, count) in top_lessons.iter().take(5) {
            report.push_str(&format!("  {lesson}: {count} occurrences\n"));
        }
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_failure_categorization() {
        let mut analyzer = FailureAnalyzer::new();
        
        // Test complete failure
        let config = MembraneConfig::new(10, 5, 5, 2, 2);
        
        let results = vec![(BigUint::from(123u32), false); 10];
        let analysis = analyzer.analyze_failure(&config, &results);
        
        match analysis.failure_mode {
            FailureMode::CompleteFailure { reason } => {
                assert!(reason.contains("wave node"));
            }
            other => panic!("Expected complete failure, got: {:?}", other),
        }
    }
}