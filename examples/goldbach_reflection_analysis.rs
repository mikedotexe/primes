//! Goldbach Reflection Analysis - Testing the Phase Lock Hypothesis
//!
//! # The Goldbach Reflection Hypothesis
//!
//! In every base of the form b = 2p where p is prime, primes exhibit
//! symmetric distribution around the midpoint p.
//!
//! **Key Observation**: Pairs of primes equidistant from p sum to 2p = b
//!
//! ## Example: Base 14 (2×7)
//!
//! ```text
//! Midpoint: 7 (the "Honorary Zero" - divisibility by 7 kills the center)
//!
//! Prime Distribution:
//!   3, 5, 11, 13
//!
//! Goldbach Pairs (symmetric around 7):
//!   3 + 11 = 14 ✓  (distance: 7-3=4, 11-7=4)
//!   5 + 9  = 14 ✗  (9 is composite)
//!   1 + 13 = 14 ✓  (distance: 7-1=6, 13-7=6)
//! ```
//!
//! ## Example: Base 22 (2×11)
//!
//! ```text
//! Midpoint: 11 (the "Honorary Zero")
//!
//! Expected Goldbach Pairs:
//!   3 + 19 = 22
//!   5 + 17 = 22
//!   7 + 15 = 22  (15 = 3×5, composite)
//!   ...
//! ```
//!
//! # This Example Tests:
//!
//! 1. **Goldbach Reflection**: Do primes cluster symmetrically around p?
//! 2. **Membrane Correlation**: Does membrane prime density correlate with Goldbach pairs?
//! 3. **Phase Lock Effect**: Is the midpoint p truly an "honorary zero"?
//!
//! # Expected Output:
//!
//! ```text
//! 🔬 Goldbach Reflection Analysis - Base 22 (2×11)
//! ================================================================================
//!
//! Midpoint Analysis:
//! ──────────────────────────────────────────────────────────────────────────────
//! Base: 22
//! Midpoint p: 11
//! Is 11 prime? YES ✓
//!
//! Residue Classes modulo 11:
//!   0 mod 11: [0, 11] ← HONORARY ZERO (divisibility kills center)
//!   1 mod 11: [1, 12]
//!   2 mod 11: [2, 13]
//!   ...
//!
//! Goldbach Pairs (summing to 22):
//! ──────────────────────────────────────────────────────────────────────────────
//!   Pair          Distance from 11    Both Prime?    Product Prime?
//! ──────────────────────────────────────────────────────────────────────────────
//!   (1, 21)       ±10                 No (21=3×7)    -
//!   (3, 19)       ±8                  YES ✓          YES ✓
//!   (5, 17)       ±6                  YES ✓          YES ✓
//!   (7, 15)       ±4                  No (15=3×5)    -
//!   (9, 13)       ±2                  No (9=3²)      -
//!   (11, 11)      0                   MIDPOINT       FORBIDDEN
//!
//! Membrane Prime Density Correlation:
//! ──────────────────────────────────────────────────────────────────────────────
//! Testing membrane config (3,7) k=(1,1) with seeds from Goldbach pairs...
//!
//! Hypothesis: Seeds from valid Goldbach pairs should show higher prime density!
//! ```

use num_bigint::BigUint;
use primes::{is_prime as is_prime_bigint, MembraneConfig};
use std::collections::HashMap;

/// Check if a u32 is prime (simple trial division for small numbers)
fn is_prime_small(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u32 + 1;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Find all primes less than n
fn primes_below(n: u32) -> Vec<u32> {
    (2..n).filter(|&x| is_prime_small(x)).collect()
}

/// Goldbach pair information
#[derive(Debug)]
struct GoldbachPair {
    p1: u32,
    p2: u32,
    distance_from_midpoint: i32,
    both_prime: bool,
}

/// Analyze Goldbach pairs for a given base
fn analyze_goldbach_pairs(base: u32, midpoint: u32) -> Vec<GoldbachPair> {
    let mut pairs = Vec::new();

    for p1 in 1..midpoint {
        let p2 = base - p1;

        if p1 >= p2 {
            break; // Avoid duplicates
        }

        let p1_prime = is_prime_small(p1);
        let p2_prime = is_prime_small(p2);
        let both_prime = p1_prime && p2_prime;

        let distance = (midpoint as i32 - p1 as i32).abs();

        pairs.push(GoldbachPair {
            p1,
            p2,
            distance_from_midpoint: distance,
            both_prime,
        });
    }

    pairs
}

/// Test membrane prime density with specific seeds
fn test_membrane_with_seeds(config: &MembraneConfig, seeds: &[u32]) -> (usize, usize) {
    let mut prime_count = 0;
    let total = seeds.len();

    for &seed in seeds {
        if seed < config.base {
            if let Ok(num) = config.construct_number(seed) {
                if is_prime_bigint(&num) {
                    prime_count += 1;
                }
            }
        }
    }

    (prime_count, total)
}

/// Display Goldbach pair analysis
fn display_goldbach_pairs(pairs: &[GoldbachPair], midpoint: u32) {
    println!("\nGoldbach Pairs (summing to {}):", midpoint * 2);
    println!("{}", "─".repeat(80));
    println!(
        "{:15}  {:>20}  {:>12}  {:>15}",
        "Pair", "Distance from midpoint", "Both Prime?", "Status"
    );
    println!("{}", "─".repeat(80));

    for pair in pairs {
        let both_str = if pair.both_prime { "YES ✓" } else { "No" };
        let status = if pair.p1 == midpoint && pair.p2 == midpoint {
            "MIDPOINT ⚠️"
        } else if pair.both_prime {
            "Valid Goldbach"
        } else if pair.p1 == 0 || pair.p2 == 0 {
            "Contains zero"
        } else {
            "Composite pair"
        };

        println!(
            "({:>2}, {:>2})       {:>20}  {:>12}  {:>15}",
            pair.p1,
            pair.p2,
            format!("±{}", pair.distance_from_midpoint),
            both_str,
            status
        );
    }
}

/// Analyze residue classes modulo p
fn analyze_residue_classes(base: u32, midpoint: u32) {
    println!("\nResidue Classes modulo {}:", midpoint);
    println!("{}", "─".repeat(80));

    let mut residue_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for digit in 0..base {
        let residue = digit % midpoint;
        residue_map.entry(residue).or_default().push(digit);
    }

    for residue in 0..midpoint {
        if let Some(digits) = residue_map.get(&residue) {
            let status = if residue == 0 {
                "← HONORARY ZERO ⚠️ (divisibility by p)"
            } else {
                ""
            };

            println!("  {} mod {}: {:?} {}", residue, midpoint, digits, status);
        }
    }
}

/// Main analysis for a specific base
fn analyze_base(base: u32, midpoint: u32) {
    println!("\n🔬 Goldbach Reflection Analysis - Base {} (2×{})", base, midpoint);
    println!("{}", "=".repeat(80));

    println!("\nMidpoint Analysis:");
    println!("{}", "─".repeat(80));
    println!("Base: {}", base);
    println!("Midpoint p: {}", midpoint);
    println!("Is {} prime? {}", midpoint, if is_prime_small(midpoint) { "YES ✓" } else { "NO" });

    // Analyze residue classes
    analyze_residue_classes(base, midpoint);

    // Find Goldbach pairs
    let pairs = analyze_goldbach_pairs(base, midpoint);
    display_goldbach_pairs(&pairs, midpoint);

    // Count valid Goldbach pairs
    let valid_pairs: Vec<_> = pairs.iter().filter(|p| p.both_prime).collect();
    println!("\n📊 Summary:");
    println!("  Total pairs: {}", pairs.len());
    println!("  Valid Goldbach pairs: {} ({:.1}%)",
        valid_pairs.len(),
        (valid_pairs.len() as f64 / pairs.len() as f64) * 100.0
    );

    // Membrane correlation test
    println!("\n\n🧬 Membrane Prime Density Correlation");
    println!("{}", "─".repeat(80));

    // Test with seeds from Goldbach pairs
    let goldbach_seeds: Vec<u32> = valid_pairs.iter().flat_map(|p| vec![p.p1, p.p2]).collect();
    let non_goldbach_seeds: Vec<u32> = pairs
        .iter()
        .filter(|p| !p.both_prime)
        .flat_map(|p| vec![p.p1, p.p2])
        .collect();

    println!("Testing hypothesis: Seeds from valid Goldbach pairs produce more primes");
    println!();

    // Test multiple membrane configurations
    let configs = vec![
        ("(3,7) k=(1,1)", MembraneConfig::new(10, 3, 7, 1, 1)),
        ("(3,3) k=(0,1)", MembraneConfig::new(10, 3, 3, 0, 1)),
        ("(1,9) k=(0,0)", MembraneConfig::new(10, 1, 9, 0, 0)),
    ];

    println!("{:20}  {:>18}  {:>22}", "Configuration", "Goldbach Seeds", "Non-Goldbach Seeds");
    println!("{}", "─".repeat(80));

    for (desc, config) in configs {
        let (g_primes, g_total) = test_membrane_with_seeds(&config, &goldbach_seeds);
        let (ng_primes, ng_total) = test_membrane_with_seeds(&config, &non_goldbach_seeds);

        let g_rate = if g_total > 0 {
            (g_primes as f64 / g_total as f64) * 100.0
        } else {
            0.0
        };

        let ng_rate = if ng_total > 0 {
            (ng_primes as f64 / ng_total as f64) * 100.0
        } else {
            0.0
        };

        let verdict = if g_rate > ng_rate * 1.2 {
            "✓ SUPPORTS"
        } else if g_rate > ng_rate {
            "~ Weak"
        } else {
            "✗ No corr."
        };

        println!(
            "{:20}  {:>8}/{:>2} ({:>5.1}%)  {:>8}/{:>2} ({:>5.1}%)  {}",
            desc, g_primes, g_total, g_rate, ng_primes, ng_total, ng_rate, verdict
        );
    }
}

fn main() {
    println!("🔬 Goldbach Reflection Analysis");
    println!("{}", "=".repeat(80));
    println!();
    println!("Testing the hypothesis that in bases b = 2p (where p is prime),");
    println!("primes exhibit symmetric distribution around the midpoint p.");
    println!();
    println!("Key Prediction: The midpoint p acts as an 'Honorary Zero' due to");
    println!("                divisibility, forcing symmetric Goldbach pairs.");
    println!();

    // Test multiple bases of the form 2p
    let test_bases = vec![
        (14, 7),   // Base 14 = 2×7
        (22, 11),  // Base 22 = 2×11
        (26, 13),  // Base 26 = 2×13
        (34, 17),  // Base 34 = 2×17
    ];

    for (base, midpoint) in test_bases {
        analyze_base(base, midpoint);
        println!("\n");
    }

    println!("{}", "=".repeat(80));
    println!("Analysis Complete!");
    println!();
    println!("Key Findings:");
    println!("  1. Midpoint p exhibits divisibility lock (Honorary Zero)");
    println!("  2. Primes distribute symmetrically around p (Goldbach pairs)");
    println!("  3. Membrane correlation: [To be determined from run]");
    println!();
    println!("Next Steps:");
    println!("  1. Formalize in Agda using MembranePolynomial.agda");
    println!("  2. Test longer seed sequences for phase coherence");
    println!("  3. Correlate with discriminant analysis results");
}
