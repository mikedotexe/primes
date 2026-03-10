//! Residue Purity Optimization
//!
//! Earlier finding: In base 6, residue 5 mod 6 has higher purity (24.1%)
//! than residue 1 mod 6 (18.1%).
//!
//! Questions:
//! 1. Does this asymmetry exist in other bases?
//! 2. Can we design membranes that preferentially sample high-purity residues?
//! 3. Is there a connection to quadratic residues?

use primes::hzlib::num_theory::{factor, Material};
use std::collections::HashMap;

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3;
    while d * d <= n { if n % d == 0 { return false; } d += 2; }
    true
}

/// Analyze purity by residue class for a given base
fn analyze_residue_purity(base: u64, limit: u64) -> HashMap<u64, (f64, f64)> {
    let base_factors = factor(base);
    let primes: Vec<u64> = base_factors.iter().map(|(p, _)| *p).collect();

    // Find coprime residue classes
    let coprime_residues: Vec<u64> = (1..base)
        .filter(|&r| primes.iter().all(|&p| r % p != 0))
        .collect();

    let mut results = HashMap::new();

    for &residue in &coprime_residues {
        let mut purities = Vec::new();

        for n in (residue..=limit).step_by(base as usize) {
            if n == 0 { continue; }
            let m = Material::for_base(n, base);
            if m.core > 1 {
                purities.push(m.purity);
            }
        }

        if !purities.is_empty() {
            let mean = purities.iter().sum::<f64>() / purities.len() as f64;
            let full_count = purities.iter().filter(|&&p| (p - 1.0).abs() < 1e-10).count();
            let full_frac = full_count as f64 / purities.len() as f64;
            results.insert(residue, (mean, full_frac));
        }
    }

    results
}

/// Legendre symbol (a|p) for quadratic residue analysis
fn legendre(a: i64, p: u64) -> i64 {
    if a % p as i64 == 0 { return 0; }
    let exp = (p - 1) / 2;
    let result = mod_pow(a.rem_euclid(p as i64) as u64, exp, p);
    if result == 1 { 1 } else { -1 }
}

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    if m == 1 { return 0; }
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp % 2 == 1 { result = result * base % m; }
        exp /= 2;
        base = base * base % m;
    }
    result
}

fn main() {
    println!("=== RESIDUE PURITY OPTIMIZATION ===\n");

    let limit = 5000u64;

    // Analyze multiple bases
    let bases = vec![6, 10, 12, 30, 42, 210];

    for base in &bases {
        println!("\n--- BASE {} ---\n", base);

        let purity_data = analyze_residue_purity(*base, limit);

        // Sort by full purity fraction
        let mut sorted: Vec<_> = purity_data.iter().collect();
        sorted.sort_by(|a, b| b.1.1.partial_cmp(&a.1.1).unwrap());

        println!("{:>6} {:>12} {:>12} {:>12}", "Residue", "MeanPurity", "FullPur%", "Type");
        println!("{}", "-".repeat(48));

        for (residue, (mean, full_frac)) in &sorted {
            // Classify the residue
            let res_type = if is_prime(**residue) {
                "prime"
            } else if **residue == 1 {
                "unit"
            } else {
                "composite"
            };

            println!("{:>6} {:>12.4} {:>12.1} {:>12}",
                     residue, mean, full_frac * 100.0, res_type);
        }

        // Summary statistics
        let purities: Vec<f64> = sorted.iter().map(|(_, (_, fp))| *fp).collect();
        let mean_fp = purities.iter().sum::<f64>() / purities.len() as f64;
        let max_fp = purities.iter().cloned().fold(0.0f64, f64::max);
        let min_fp = purities.iter().cloned().fold(1.0f64, f64::min);

        println!("\nSummary: mean_full_purity={:.1}%, range=[{:.1}%, {:.1}%]",
                 mean_fp * 100.0, min_fp * 100.0, max_fp * 100.0);

        // Identify best and worst residues
        if let (Some((best_res, _)), Some((worst_res, _))) = (sorted.first(), sorted.last()) {
            println!("Best: {} ({:.1}%), Worst: {} ({:.1}%)",
                     best_res, sorted.first().unwrap().1.1 * 100.0,
                     worst_res, sorted.last().unwrap().1.1 * 100.0);
        }
    }

    // Quadratic residue analysis for base 6
    println!("\n\n=== QUADRATIC RESIDUE ANALYSIS ===\n");
    println!("Is high purity correlated with being a quadratic residue?\n");

    for &base in &[6u64, 30, 210] {
        println!("\nBase {}:", base);

        let purity_data = analyze_residue_purity(base, limit);

        // For each residue, check if it's a QR mod various primes
        let test_primes = vec![5u64, 7, 11, 13];

        println!("{:>6} {:>8}", "Residue", "FullPur%");

        for (residue, (_, full_frac)) in &purity_data {
            let qr_score: i64 = test_primes.iter()
                .filter(|&&p| p != base && base % p != 0)
                .map(|&p| legendre(*residue as i64, p))
                .sum();

            let qr_str = if qr_score > 0 { "QR+" } else if qr_score < 0 { "NQR" } else { "~" };

            println!("{:>6} {:>8.1}% {}", residue, full_frac * 100.0, qr_str);
        }
    }

    // Practical application: Can we construct membranes that land in high-purity residue classes?
    println!("\n\n=== PRACTICAL APPLICATION ===\n");
    println!("For base 6, residue 5 has higher purity than residue 1.");
    println!("Membrane L|S|R ≡ L*6² + S*6 + R (mod 6)");
    println!("If L=1, R=5, S=k:");
    println!("  Value ≡ 36 + 6k + 5 ≡ 5 (mod 6) for any k coprime to 6");
    println!();
    println!("So (1,5) membranes always land in the HIGH-PURITY residue class 5!");
    println!("This may explain part of (1,5)'s success.");
    println!();

    // Check other configs
    println!("Checking residue class for different configs in base 6:\n");
    println!("{:>4} {:>4} {:>15}", "L", "R", "Residue class");
    println!("{}", "-".repeat(25));

    for left in [1, 5] {
        for right in [1, 5] {
            // L*36 + S*6 + R mod 6 = (L*36 mod 6) + (R mod 6)
            // = 0 + R = R for any S coprime to 6
            let residue = right % 6;
            println!("{:>4} {:>4} {:>15}", left, right, residue);
        }
    }

    println!("\nConclusion: The RIGHT boundary determines the residue class!");
    println!("Use R=5 (high purity) rather than R=1 (lower purity).");

    // Extension to base 210
    println!("\n\n=== BASE 210 RESIDUE OPTIMIZATION ===\n");

    let purity_210 = analyze_residue_purity(210, 3000);
    let mut sorted_210: Vec<_> = purity_210.iter().collect();
    sorted_210.sort_by(|a, b| b.1.1.partial_cmp(&a.1.1).unwrap());

    println!("Top 10 high-purity residue classes in base 210:\n");
    println!("{:>6} {:>12} {:>12}", "Residue", "FullPur%", "IsPrime");
    println!("{}", "-".repeat(35));

    for (residue, (_, full_frac)) in sorted_210.iter().take(10) {
        let prime_str = if is_prime(**residue) { "yes" } else { "no" };
        println!("{:>6} {:>12.1} {:>12}", residue, full_frac * 100.0, prime_str);
    }

    println!("\nRecommendation: Use these as RIGHT boundary digits in base 210:");
    let top_residues: Vec<u64> = sorted_210.iter().take(5).map(|(r, _)| **r).collect();
    println!("{:?}", top_residues);
}
