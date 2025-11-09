//! Symmetric membrane construction
//!
//! The classic symmetric pattern: outer + zeros + inner + zeros + middle + zeros + inner + zeros + outer

use crate::{PhysicsError, PhysicsResult};

/// Construct a symmetric membrane number
pub fn construct_symmetric_membrane(
    outer: u32,
    inner: u32,
    middle: &str,
    k_outer: u32,
    k_inner: u32,
) -> PhysicsResult<String> {
    if outer >= 10 || inner >= 10 {
        return Err(PhysicsError::InvalidMembrane(
            "Digits must be 0-9".to_string(),
        ));
    }

    let left_outer = outer.to_string();
    let left_zeros_outer = "0".repeat(k_outer as usize);
    let left_inner = inner.to_string();
    let left_zeros_inner = "0".repeat(k_inner as usize);

    let right_zeros_inner = "0".repeat(k_inner as usize);
    let right_inner = inner.to_string();
    let right_zeros_outer = "0".repeat(k_outer as usize);
    let right_outer = outer.to_string();

    let membrane = format!(
        "{left_outer}{left_zeros_outer}{left_inner}{left_zeros_inner}{middle}{right_zeros_inner}{right_inner}{right_zeros_outer}{right_outer}"
    );

    Ok(membrane)
}

/// Analyze the structure of a symmetric membrane
pub fn analyze_symmetric_structure(membrane: &str) -> PhysicsResult<SymmetricAnalysis> {
    let chars: Vec<char> = membrane.chars().collect();
    let len = chars.len();

    if len < 4 {
        return Err(PhysicsError::InvalidMembrane(
            "Membrane too short for analysis".to_string(),
        ));
    }

    // Parse from left side
    let mut lpos = 0;
    let left_outer = chars[lpos].to_digit(10).unwrap_or(0);
    lpos += 1;

    let mut k_outer_left = 0;
    while lpos < len && chars[lpos] == '0' {
        k_outer_left += 1;
        lpos += 1;
    }

    if lpos >= len {
        return Err(PhysicsError::InvalidMembrane(
            "No inner structure found".to_string(),
        ));
    }

    let left_inner = chars[lpos].to_digit(10).unwrap_or(0);
    lpos += 1;

    let mut k_inner_left = 0;
    while lpos < len && chars[lpos] == '0' {
        k_inner_left += 1;
        lpos += 1;
    }

    // Parse from right side
    let mut rpos = len - 1;
    let right_outer = chars[rpos].to_digit(10).unwrap_or(0);
    rpos = rpos.saturating_sub(1);

    let mut k_outer_right = 0;
    while rpos > 0 && chars[rpos] == '0' {
        k_outer_right += 1;
        rpos = rpos.saturating_sub(1);
    }

    let right_inner = chars[rpos].to_digit(10).unwrap_or(0);
    rpos = rpos.saturating_sub(1);

    let mut k_inner_right = 0;
    while rpos > 0 && chars[rpos] == '0' {
        k_inner_right += 1;
        rpos = rpos.saturating_sub(1);
    }

    // Verify symmetry
    if left_outer != right_outer {
        return Err(PhysicsError::InvalidMembrane(
            "Outer digits don't match".to_string(),
        ));
    }

    if left_inner != right_inner {
        return Err(PhysicsError::InvalidMembrane(
            "Inner digits don't match".to_string(),
        ));
    }

    if k_outer_left != k_outer_right {
        return Err(PhysicsError::InvalidMembrane(
            "Outer zero padding doesn't match".to_string(),
        ));
    }

    if k_inner_left != k_inner_right {
        return Err(PhysicsError::InvalidMembrane(
            "Inner zero padding doesn't match".to_string(),
        ));
    }

    // Extract middle
    let middle: String = chars[lpos..=rpos].iter().collect();

    Ok(SymmetricAnalysis {
        outer_digit: left_outer,
        inner_digit: left_inner,
        k_outer: k_outer_left,
        k_inner: k_inner_left,
        middle: middle.clone(),
        total_length: len,
        structure_percentage: calculate_structure_percentage(len, middle.len()),
    })
}

/// Analysis result for symmetric membranes
#[derive(Debug, Clone)]
pub struct SymmetricAnalysis {
    pub outer_digit: u32,
    pub inner_digit: u32,
    pub k_outer: u32,
    pub k_inner: u32,
    pub middle: String,
    pub total_length: usize,
    pub structure_percentage: f64,
}

impl SymmetricAnalysis {
    /// Check if this follows the 3-7 pattern
    pub fn is_37_pattern(&self) -> bool {
        (self.outer_digit == 3 && self.inner_digit == 7)
            || (self.outer_digit == 7 && self.inner_digit == 3)
    }

    /// Check if middle contains 37 or 73 patterns
    pub fn has_37_middle(&self) -> bool {
        self.middle.contains("37") || self.middle.contains("73")
    }

    /// Calculate resonance score based on discovered patterns
    pub fn resonance_score(&self) -> f64 {
        let mut score = 1.0;

        // 3-7 boundary bonus
        if self.is_37_pattern() {
            score *= 1.8; // The magical 18.55% vs 2-3% for others
        }

        // 37/73 middle bonus
        if self.has_37_middle() {
            score *= 1.3;
        }

        // Twin digit bonus
        if self.outer_digit == self.inner_digit {
            score *= 1.2; // Twin digits showed higher density
        }

        // Optimal k-value bonus (k=2,2 was often good)
        if self.k_outer == 2 && self.k_inner == 2 {
            score *= 1.1;
        }

        // Structure percentage bonus (90%+ showed interesting behavior)
        if self.structure_percentage > 0.9 {
            score *= 1.5;
        }

        score
    }

    /// Get a human-readable description
    pub fn description(&self) -> String {
        format!(
            "({},{}) k=({},{}) middle='{}' [{}% structure, resonance: {:.2}]",
            self.outer_digit,
            self.inner_digit,
            self.k_outer,
            self.k_inner,
            self.middle,
            (self.structure_percentage * 100.0) as u32,
            self.resonance_score()
        )
    }
}

/// Calculate what percentage of the number is structural vs content
fn calculate_structure_percentage(total_length: usize, middle_length: usize) -> f64 {
    let structure_length = total_length - middle_length;
    structure_length as f64 / total_length as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric_construction() {
        let result = construct_symmetric_membrane(3, 7, "5", 2, 1);
        assert!(result.is_ok());
        let membrane = result.unwrap();
        assert_eq!(membrane, "30070507003");
    }

    #[test]
    fn test_symmetric_analysis() {
        let membrane = "30070507003";
        let analysis = analyze_symmetric_structure(membrane);
        assert!(analysis.is_ok());

        let result = analysis.unwrap();
        assert_eq!(result.outer_digit, 3);
        assert_eq!(result.inner_digit, 7);
        assert_eq!(result.k_outer, 2);
        assert_eq!(result.k_inner, 1);
        assert_eq!(result.middle, "5");
        assert!(result.is_37_pattern());
    }

    #[test]
    fn test_resonance_scoring() {
        // Test with a valid symmetric structure
        let membrane = "30070373707003";
        let analysis = analyze_symmetric_structure(membrane).unwrap();
        let score = analysis.resonance_score();

        // Should have high resonance due to 3-7 boundary and 373 middle
        assert!(score > 2.0);

        // Also test the original asymmetric pattern with flexible analyzer
        let asymmetric = "300737373700003";
        let flex_analysis =
            crate::membrane::flexible::FlexibleAnalysis::analyze(asymmetric).unwrap();

        // Should find high resonance even in asymmetric pattern
        assert!(flex_analysis.total_resonance() > 2.0);
        assert!(flex_analysis.breathing_score > 0.0); // Should detect asymmetry
    }
}
