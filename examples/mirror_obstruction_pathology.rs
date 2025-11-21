//! Mirror Obstruction Pathology Hunter
//!
//! Investigation of the 9 k=0 highly symmetric cases that ALL failed (0/9 prime).
//!
//! ## Key Question
//!
//! Why does k=0 with high symmetry (>0.7) show 0% density while k=1 maintains 10%?
//!
//! ## Hypothesis
//!
//! The 9 pathological k=0 cases might:
//! 1. Have perfect mirror structure triggering algebraic factorization
//! 2. Create systematic modular obstructions not present in k=1
//! 3. Explain the "residual" 3.5× gap beyond length penalty
//!
//! ## Approach
//!
//! 1. Export all high-symmetry cases with full structure
//! 2. Compare k=0 vs k=1 apples-to-apples (high-sym vs high-sym)
//! 3. Calculate adjusted densities excluding pathological cases
//! 4. Analyze membrane structure patterns

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use std::fs::File;
use std::io::Write;

// ============================================================================
// Membrane Construction
// ============================================================================

fn construct_membrane(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
    seed: u64,
) -> BigUint {
    let base_big = BigUint::from(base);
    let mut result = BigUint::zero();
    let mut position = 0;

    let mut add_digit = |digit: u32| {
        result += BigUint::from(digit) * base_big.pow(position);
        position += 1;
    };

    // Structure: outer [k×0] inner [k×0] SEED [k×0] inner [k×0] outer
    add_digit(outer);
    for _ in 0..k {
        add_digit(0);
    }
    add_digit(inner);
    for _ in 0..k {
        add_digit(0);
    }

    // Middle (seed in base representation)
    let mut seed_val = seed;
    for _ in 0..m {
        add_digit((seed_val % base as u64) as u32);
        seed_val /= base as u64;
    }

    // Mirror
    for _ in 0..k {
        add_digit(0);
    }
    add_digit(inner);
    for _ in 0..k {
        add_digit(0);
    }
    add_digit(outer);

    result
}

fn extract_digits(n: &BigUint, base: u32) -> Vec<u32> {
    if n.is_zero() {
        return vec![0];
    }

    let base_big = BigUint::from(base);
    let mut temp = n.clone();
    let mut digits = Vec::new();

    while temp > BigUint::zero() {
        let digit = (&temp % &base_big).to_u32_digits();
        digits.push(if digit.is_empty() { 0 } else { digit[0] });
        temp /= &base_big;
    }

    digits
}

fn mirror_symmetry_index(digits: &[u32]) -> f64 {
    if digits.is_empty() {
        return 1.0;
    }

    let n = digits.len();
    let mut matches = 0;
    let pairs = n / 2;

    for i in 0..pairs {
        if digits[i] == digits[n - 1 - i] {
            matches += 1;
        }
    }

    let total_pairs = if n % 2 == 0 { pairs } else { pairs + 1 };
    matches as f64 / total_pairs as f64
}

// ============================================================================
// Analysis Structure
// ============================================================================

#[derive(Debug, Clone)]
struct PathologyCase {
    seed: u64,
    k: u32,
    membrane: BigUint,
    digits: Vec<u32>,
    is_prime: bool,
    symmetry_index: f64,

    // Pattern analysis
    perfect_palindrome: bool,
    zero_runs: Vec<usize>,
    seed_digits: Vec<u32>,
}

impl PathologyCase {
    fn new(base: u32, outer: u32, inner: u32, m: usize, k: u32, seed: u64) -> Self {
        let membrane = construct_membrane(base, outer, inner, m, k, seed);
        let digits = extract_digits(&membrane, base);
        let symmetry_index = mirror_symmetry_index(&digits);
        let is_prime_result = is_prime(&membrane);

        // Check perfect palindrome
        let n = digits.len();
        let perfect_palindrome = (0..n/2).all(|i| digits[i] == digits[n-1-i])
            && (n % 2 == 0 || true); // middle digit can be anything

        // Find zero runs
        let mut zero_runs = Vec::new();
        let mut current_run = 0;
        for &d in &digits {
            if d == 0 {
                current_run += 1;
            } else {
                if current_run > 0 {
                    zero_runs.push(current_run);
                    current_run = 0;
                }
            }
        }
        if current_run > 0 {
            zero_runs.push(current_run);
        }

        // Extract seed digits (middle M digits)
        let start = 2 + 2*k as usize; // outer + k zeros + inner + k zeros
        let seed_digits = if start + m <= digits.len() {
            digits[start..start+m].to_vec()
        } else {
            Vec::new()
        };

        PathologyCase {
            seed,
            k,
            membrane,
            digits,
            is_prime: is_prime_result,
            symmetry_index,
            perfect_palindrome,
            zero_runs,
            seed_digits,
        }
    }

    fn digit_string(&self) -> String {
        self.digits.iter().rev()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

// ============================================================================
// Pathology Analysis
// ============================================================================

fn analyze_pathology(base: u32, outer: u32, inner: u32, m: usize) {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║          MIRROR OBSTRUCTION PATHOLOGY HUNTER                  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("Configuration: Base {}, ({},{}), M={}", base, outer, inner, m);
    println!("Hunting for pathological high-symmetry k=0 cases...\n");

    let seed_min = if m > 1 {
        (base as u64).pow((m - 1) as u32)
    } else {
        1
    };
    let seed_max = (base as u64).pow(m as u32);

    // Collect ALL cases
    let mut all_k0 = Vec::new();
    let mut all_k1 = Vec::new();

    for seed in seed_min..seed_max {
        all_k0.push(PathologyCase::new(base, outer, inner, m, 0, seed));
        all_k1.push(PathologyCase::new(base, outer, inner, m, 1, seed));
    }

    // Filter high-symmetry cases (>0.7)
    let high_sym_k0: Vec<_> = all_k0.iter()
        .filter(|c| c.symmetry_index > 0.7)
        .collect();

    let high_sym_k1: Vec<_> = all_k1.iter()
        .filter(|c| c.symmetry_index > 0.7)
        .collect();

    // ========================================================================
    // Overall Statistics
    // ========================================================================

    println!("═══════════════════════════════════════════════════════════════");
    println!("OVERALL STATISTICS");
    println!("═══════════════════════════════════════════════════════════════\n");

    let total = (seed_max - seed_min) as usize;

    let density_k0 = all_k0.iter().filter(|c| c.is_prime).count() as f64 / total as f64;
    let density_k1 = all_k1.iter().filter(|c| c.is_prime).count() as f64 / total as f64;

    println!("Overall densities:");
    println!("  k=0: {:.1}% ({}/{})", density_k0 * 100.0,
             all_k0.iter().filter(|c| c.is_prime).count(), total);
    println!("  k=1: {:.1}% ({}/{})", density_k1 * 100.0,
             all_k1.iter().filter(|c| c.is_prime).count(), total);
    println!("  Ratio: {:.2}×\n", density_k0 / density_k1);

    println!("High-symmetry counts (>0.7):");
    println!("  k=0: {} cases ({:.1}% of total)", high_sym_k0.len(),
             high_sym_k0.len() as f64 / total as f64 * 100.0);
    println!("  k=1: {} cases ({:.1}% of total)", high_sym_k1.len(),
             high_sym_k1.len() as f64 / total as f64 * 100.0);

    // ========================================================================
    // High-Symmetry Comparison
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("HIGH-SYMMETRY PATHOLOGY (>0.7 symmetry index)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let hs_density_k0 = if !high_sym_k0.is_empty() {
        high_sym_k0.iter().filter(|c| c.is_prime).count() as f64 / high_sym_k0.len() as f64
    } else {
        0.0
    };

    let hs_density_k1 = if !high_sym_k1.is_empty() {
        high_sym_k1.iter().filter(|c| c.is_prime).count() as f64 / high_sym_k1.len() as f64
    } else {
        0.0
    };

    println!("Prime density for high-symmetry cases:");
    println!("  k=0: {:.1}% ({}/{})", hs_density_k0 * 100.0,
             high_sym_k0.iter().filter(|c| c.is_prime).count(), high_sym_k0.len());
    println!("  k=1: {:.1}% ({}/{})", hs_density_k1 * 100.0,
             high_sym_k1.iter().filter(|c| c.is_prime).count(), high_sym_k1.len());

    if hs_density_k0 == 0.0 && high_sym_k0.len() > 0 {
        println!("\n🚨 PATHOLOGY CONFIRMED:");
        println!("   ALL {} high-symmetry k=0 cases FAILED!", high_sym_k0.len());
    }

    // ========================================================================
    // Adjusted Densities (Excluding Pathological Cases)
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("ADJUSTED DENSITIES (excluding pathological high-sym k=0)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let normal_k0: Vec<_> = all_k0.iter()
        .filter(|c| c.symmetry_index <= 0.7)
        .collect();

    let adj_density_k0 = if !normal_k0.is_empty() {
        normal_k0.iter().filter(|c| c.is_prime).count() as f64 / normal_k0.len() as f64
    } else {
        0.0
    };

    println!("k=0 normal-symmetry (≤0.7): {:.1}% ({}/{})", adj_density_k0 * 100.0,
             normal_k0.iter().filter(|c| c.is_prime).count(), normal_k0.len());
    println!("k=1 overall: {:.1}% ({}/{})", density_k1 * 100.0,
             all_k1.iter().filter(|c| c.is_prime).count(), total);

    let adj_ratio = adj_density_k0 / density_k1;
    println!("\nAdjusted ratio (normal k=0 / all k=1): {:.2}×", adj_ratio);
    println!("Original ratio (all k=0 / all k=1): {:.2}×", density_k0 / density_k1);
    println!("Pathology impact: {:.1}pp ({:.1}% relative drop)",
             (adj_density_k0 - density_k0) * 100.0,
             ((adj_density_k0 - density_k0) / density_k0) * 100.0);

    // ========================================================================
    // Export Pathological Cases
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("EXPORTING PATHOLOGICAL CASES");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Export k=0 high-symmetry failures
    let mut csv_file = File::create("mirror_pathology_k0_high_sym.csv")
        .expect("Unable to create CSV");

    writeln!(csv_file, "seed,symmetry_index,membrane_decimal,membrane_digits,perfect_palindrome,zero_run_count,max_zero_run,seed_pattern")
        .unwrap();

    for case in &high_sym_k0 {
        let max_zero_run = case.zero_runs.iter().max().copied().unwrap_or(0);
        let seed_pattern = case.seed_digits.iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(",");

        writeln!(csv_file, "{},{:.3},{},{},{},{},{},\"{}\"",
                 case.seed,
                 case.symmetry_index,
                 case.membrane,
                 case.digit_string(),
                 case.perfect_palindrome,
                 case.zero_runs.len(),
                 max_zero_run,
                 seed_pattern)
            .unwrap();
    }

    println!("✅ Exported {} k=0 high-symmetry cases to: mirror_pathology_k0_high_sym.csv",
             high_sym_k0.len());

    // Export k=1 high-symmetry (for comparison)
    let mut csv_file = File::create("mirror_pathology_k1_high_sym.csv")
        .expect("Unable to create CSV");

    writeln!(csv_file, "seed,symmetry_index,is_prime,membrane_decimal,membrane_digits,perfect_palindrome,zero_run_count,max_zero_run")
        .unwrap();

    for case in &high_sym_k1 {
        let max_zero_run = case.zero_runs.iter().max().copied().unwrap_or(0);

        writeln!(csv_file, "{},{:.3},{},{},{},{},{},{}",
                 case.seed,
                 case.symmetry_index,
                 case.is_prime as u8,
                 case.membrane,
                 case.digit_string(),
                 case.perfect_palindrome,
                 case.zero_runs.len(),
                 max_zero_run)
            .unwrap();
    }

    println!("✅ Exported {} k=1 high-symmetry cases to: mirror_pathology_k1_high_sym.csv",
             high_sym_k1.len());

    // ========================================================================
    // Pattern Analysis
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("PATHOLOGY PATTERN ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════\n");

    if !high_sym_k0.is_empty() {
        println!("k=0 High-Symmetry Pathological Cases:");
        println!("Seeds: {:?}", high_sym_k0.iter().map(|c| c.seed).collect::<Vec<_>>());

        // Show first 3 examples
        println!("\nExample structures (first 3):");
        for (i, case) in high_sym_k0.iter().take(3).enumerate() {
            println!("\n  {}. Seed {}: symmetry={:.3}", i+1, case.seed, case.symmetry_index);
            println!("     Digits: {}", case.digit_string());
            println!("     Perfect palindrome: {}", case.perfect_palindrome);
            println!("     Zero runs: {:?}", case.zero_runs);
        }
    }

    if high_sym_k1.iter().any(|c| c.is_prime) {
        println!("\n\nk=1 High-Symmetry SUCCESSFUL Cases:");
        let successes: Vec<_> = high_sym_k1.iter().filter(|c| c.is_prime).collect();
        println!("Seeds: {:?}", successes.iter().map(|c| c.seed).collect::<Vec<_>>());

        println!("\nExample structures (first 3):");
        for (i, case) in successes.iter().take(3).enumerate() {
            println!("\n  {}. Seed {}: symmetry={:.3} ✨ PRIME!", i+1, case.seed, case.symmetry_index);
            println!("     Digits: {}", case.digit_string());
            println!("     Perfect palindrome: {}", case.perfect_palindrome);
            println!("     Zero runs: {:?}", case.zero_runs);
        }
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("🔬 Mirror Obstruction Pathology Hunter");
    println!("Investigating the 9 k=0 high-symmetry failures...\n");

    // Test Base 10 M=2 (3,7) - the pathological configuration
    analyze_pathology(10, 3, 7, 2);

    println!("\n✅ Pathology hunt complete!");
}
