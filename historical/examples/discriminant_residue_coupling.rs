//! Discriminant-Residue Coupling Analysis
//!
//! Tests the hypothesis that residue constraints (e.g., k=1 in Base 10)
//! systematically SELECT seeds with better discriminant properties.
//!
//! ## Hypothesis (from collaborator)
//!
//! "Seeds that would produce discriminants with many small prime factors
//!  or that are perfect squares get filtered out by the residue requirements,
//!  leaving a pool with algebraically favorable properties."
//!
//! ## Test Strategy
//!
//! For Base 10, M=2, compare k=0 vs k=1:
//! 1. Compute discriminant Δ = S² - 4 for each seed
//! 2. Analyze discriminant quality (Legendre symbols, factorization)
//! 3. Track membrane residues mod 2, mod 5
//! 4. Test if k=1 seeds have BOTH better discriminants AND favorable residues
//!
//! ## Expected Result
//!
//! If coupling exists:
//! - k=1 seeds: higher quality scores, favorable mod patterns
//! - k=0 seeds: lower quality scores, mixed mod patterns
//! - Seeds with quality>median AND favorable residues: >30% density

use num_bigint::BigUint;
use num_traits::{One, Zero};
use primes::is_prime;
use std::collections::HashMap;

// ============================================================================
// Discriminant Mathematics (from Agda Core/Discriminant.agda)
// ============================================================================

/// Compute discriminant Δ = S² - 4A² for membrane polynomial N(X) = A·X² + S·X + A
fn discriminant(outer: u64, seed: u64) -> i128 {
    let s = seed as i128;
    let a = outer as i128;
    s * s - 4 * a * a
}

/// Check if discriminant is a perfect square
fn is_perfect_square(disc: i128) -> (bool, i128) {
    if disc < 0 {
        return (false, -1);
    }

    let disc_abs = disc.abs() as u128;
    let r = (disc_abs as f64).sqrt() as i128;

    for candidate in [r - 1, r, r + 1] {
        if candidate < 0 {
            continue;
        }
        if (candidate * candidate) == disc {
            return (true, candidate);
        }
    }

    (false, -1)
}

/// Compute Legendre symbol (a/p) using Euler's criterion
/// Returns: +1 (quadratic residue), -1 (non-residue), 0 (divisible)
fn legendre_symbol(a: i128, p: u32) -> i8 {
    let p_i128 = p as i128;
    let a_mod = ((a % p_i128) + p_i128) % p_i128;

    if a_mod == 0 {
        return 0;
    }

    // Euler's criterion: (a/p) ≡ a^((p-1)/2) (mod p)
    let exp = (p - 1) / 2;
    let result = mod_exp(a_mod as u128, exp as u64, p as u64);

    if result == 1 {
        1
    } else if result == (p - 1) as u64 {
        -1
    } else {
        0
    }
}

/// Modular exponentiation
fn mod_exp(mut base: u128, mut exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let m_u128 = m as u128;
    let mut result = 1u128;
    base %= m_u128;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % m_u128;
        }
        exp >>= 1;
        base = (base * base) % m_u128;
    }

    result as u64
}

/// Discriminant quality score based on Legendre symbols
fn quality_score(disc: i128) -> f64 {
    let (is_square, _) = is_perfect_square(disc);
    if is_square {
        return -100.0; // Algebraic lock - worst possible
    }

    let leg_3 = legendre_symbol(disc, 3);
    let leg_5 = legendre_symbol(disc, 5);
    let leg_7 = legendre_symbol(disc, 7);
    let leg_11 = legendre_symbol(disc, 11);
    let leg_13 = legendre_symbol(disc, 13);

    let symbols = [leg_3, leg_5, leg_7, leg_11, leg_13];
    let admissible = symbols.iter().filter(|&&s| s == -1).count() as f64;
    let obstructed = symbols.iter().filter(|&&s| s == 1).count() as f64;
    let divisible = symbols.iter().filter(|&&s| s == 0).count() as f64;

    // Score: +1 for admissible, -1 for obstructed, -5 for divisible
    admissible - obstructed - 5.0 * divisible
}

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

// ============================================================================
// Residue Analysis
// ============================================================================

#[derive(Debug, Clone)]
struct SeedAnalysis {
    seed: u64,
    k: u32,

    // Discriminant properties
    discriminant: i128,
    is_perfect_square: bool,
    quality_score: f64,

    // Legendre symbols
    leg_3: i8,
    leg_5: i8,
    leg_7: i8,

    // Membrane properties
    membrane_value: BigUint,
    is_prime: bool,

    // Residue signature
    mod_2: u64,
    mod_5: u64,

    // Derived: does it have "favorable" residues?
    has_favorable_residues: bool,
}

fn analyze_seed(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
    seed: u64,
) -> SeedAnalysis {
    // Discriminant analysis
    let disc = discriminant(outer as u64, seed);
    let (is_square, _) = is_perfect_square(disc);
    let quality = quality_score(disc);

    let leg_3 = legendre_symbol(disc, 3);
    let leg_5 = legendre_symbol(disc, 5);
    let leg_7 = legendre_symbol(disc, 7);

    // Membrane construction and primality
    let membrane = construct_membrane(base, outer, inner, m, k, seed);
    let is_prime_result = is_prime(&membrane);

    // Residue analysis
    let mod_2 = (&membrane % 2u32).to_u64_digits()[0];
    let mod_5 = (&membrane % 5u32).to_u64_digits()[0];

    // Favorable residues: odd (mod 2 = 1) and not divisible by 5 (mod 5 ∈ {1,2,3,4})
    let has_favorable_residues = (mod_2 == 1) && (mod_5 != 0);

    SeedAnalysis {
        seed,
        k,
        discriminant: disc,
        is_perfect_square: is_square,
        quality_score: quality,
        leg_3,
        leg_5,
        leg_7,
        membrane_value: membrane,
        is_prime: is_prime_result,
        mod_2,
        mod_5,
        has_favorable_residues,
    }
}

// ============================================================================
// Coupling Analysis
// ============================================================================

fn analyze_coupling(base: u32, outer: u32, inner: u32, m: usize) {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║     DISCRIMINANT-RESIDUE COUPLING ANALYSIS                    ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("Configuration: Base {}, ({},{}), M={}", base, outer, inner, m);
    println!("Comparing k=0 vs k=1 seed selections\n");

    let seed_min = if m > 1 {
        (base as u64).pow((m - 1) as u32)
    } else {
        1
    };
    let seed_max = (base as u64).pow(m as u32);

    let mut results_k0 = Vec::new();
    let mut results_k1 = Vec::new();

    println!("Analyzing {} seeds for each k value...", seed_max - seed_min);

    for seed in seed_min..seed_max {
        results_k0.push(analyze_seed(base, outer, inner, m, 0, seed));
        results_k1.push(analyze_seed(base, outer, inner, m, 1, seed));
    }

    // ========================================================================
    // Test 1: Discriminant Quality Comparison
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("TEST 1: Discriminant Quality Distribution");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mean_quality_k0: f64 = results_k0.iter().map(|r| r.quality_score).sum::<f64>() / results_k0.len() as f64;
    let mean_quality_k1: f64 = results_k1.iter().map(|r| r.quality_score).sum::<f64>() / results_k1.len() as f64;

    let perfect_squares_k0 = results_k0.iter().filter(|r| r.is_perfect_square).count();
    let perfect_squares_k1 = results_k1.iter().filter(|r| r.is_perfect_square).count();

    println!("k=0: Mean quality score = {:.3}", mean_quality_k0);
    println!("k=1: Mean quality score = {:.3}", mean_quality_k1);
    println!("Δ Quality = {:.3} ({:+.1}%)\n",
             mean_quality_k1 - mean_quality_k0,
             (mean_quality_k1 - mean_quality_k0) / mean_quality_k0.abs() * 100.0);

    println!("Perfect square discriminants:");
    println!("  k=0: {}/{}", perfect_squares_k0, results_k0.len());
    println!("  k=1: {}/{}", perfect_squares_k1, results_k1.len());

    if mean_quality_k1 > mean_quality_k0 {
        println!("\n✅ k=1 has BETTER discriminant quality");
    } else {
        println!("\n❌ k=1 does NOT have better discriminant quality");
    }

    // ========================================================================
    // Test 2: Residue Pattern Comparison
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("TEST 2: Residue Pattern Distribution");
    println!("═══════════════════════════════════════════════════════════════\n");

    let favorable_k0 = results_k0.iter().filter(|r| r.has_favorable_residues).count();
    let favorable_k1 = results_k1.iter().filter(|r| r.has_favorable_residues).count();

    println!("Favorable residues (odd & not divisible by 5):");
    println!("  k=0: {}/{} ({:.1}%)", favorable_k0, results_k0.len(),
             favorable_k0 as f64 / results_k0.len() as f64 * 100.0);
    println!("  k=1: {}/{} ({:.1}%)", favorable_k1, results_k1.len(),
             favorable_k1 as f64 / results_k1.len() as f64 * 100.0);

    // Mod 5 distribution
    println!("\nMod 5 distribution:");
    for mod_val in 0..5 {
        let count_k0 = results_k0.iter().filter(|r| r.mod_5 == mod_val).count();
        let count_k1 = results_k1.iter().filter(|r| r.mod_5 == mod_val).count();
        println!("  ≡{} (mod 5): k=0={}/{}, k=1={}/{}",
                 mod_val, count_k0, results_k0.len(), count_k1, results_k1.len());
    }

    // ========================================================================
    // Test 3: Coupling Test (Key Hypothesis)
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("TEST 3: Discriminant-Residue Coupling");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Compute median quality score across both k values
    let mut all_qualities: Vec<f64> = results_k0.iter().chain(results_k1.iter())
        .map(|r| r.quality_score)
        .collect();
    all_qualities.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_quality = all_qualities[all_qualities.len() / 2];

    println!("Median quality score (across all): {:.3}\n", median_quality);

    // Stratify by quality AND residues
    let high_quality_favorable_k0 = results_k0.iter()
        .filter(|r| r.quality_score > median_quality && r.has_favorable_residues)
        .collect::<Vec<_>>();
    let high_quality_favorable_k1 = results_k1.iter()
        .filter(|r| r.quality_score > median_quality && r.has_favorable_residues)
        .collect::<Vec<_>>();

    let density_hq_fav_k0 = if !high_quality_favorable_k0.is_empty() {
        high_quality_favorable_k0.iter().filter(|r| r.is_prime).count() as f64 / high_quality_favorable_k0.len() as f64
    } else {
        0.0
    };

    let density_hq_fav_k1 = if !high_quality_favorable_k1.is_empty() {
        high_quality_favorable_k1.iter().filter(|r| r.is_prime).count() as f64 / high_quality_favorable_k1.len() as f64
    } else {
        0.0
    };

    println!("Seeds with HIGH quality (>{:.1}) AND favorable residues:", median_quality);
    println!("  k=0: {}/{} = {:.1}% prime",
             high_quality_favorable_k0.iter().filter(|r| r.is_prime).count(),
             high_quality_favorable_k0.len(),
             density_hq_fav_k0 * 100.0);
    println!("  k=1: {}/{} = {:.1}% prime",
             high_quality_favorable_k1.iter().filter(|r| r.is_prime).count(),
             high_quality_favorable_k1.len(),
             density_hq_fav_k1 * 100.0);

    // Overall densities for comparison
    let density_k0 = results_k0.iter().filter(|r| r.is_prime).count() as f64 / results_k0.len() as f64;
    let density_k1 = results_k1.iter().filter(|r| r.is_prime).count() as f64 / results_k1.len() as f64;

    println!("\nOverall densities (for comparison):");
    println!("  k=0: {:.1}%", density_k0 * 100.0);
    println!("  k=1: {:.1}%", density_k1 * 100.0);

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("COUPLING HYPOTHESIS TEST");
    println!("═══════════════════════════════════════════════════════════════\n");

    if density_hq_fav_k1 > density_k1 && density_hq_fav_k1 > 0.30 {
        println!("✅ COUPLING DETECTED:");
        println!("   Seeds with high quality + favorable residues show enhanced density");
        println!("   ({:.1}% vs {:.1}% baseline)", density_hq_fav_k1 * 100.0, density_k1 * 100.0);
    } else if density_hq_fav_k1 > density_k1 {
        println!("⚠️  WEAK COUPLING:");
        println!("   Some enhancement ({:.1}% vs {:.1}%) but below threshold",
                 density_hq_fav_k1 * 100.0, density_k1 * 100.0);
    } else {
        println!("❌ NO COUPLING DETECTED:");
        println!("   High quality + favorable residues does not enhance density");
    }

    // ========================================================================
    // Export CSV for further analysis
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Exporting data to CSV...");
    println!("═══════════════════════════════════════════════════════════════\n");

    let filename = format!("discriminant_residue_base{}_m{}.csv", base, m);
    export_csv(&filename, &results_k0, &results_k1);

    println!("✅ Data exported to: {}", filename);
    println!("   {} rows (k=0 and k=1 combined)", results_k0.len() + results_k1.len());
}

fn export_csv(filename: &str, results_k0: &[SeedAnalysis], results_k1: &[SeedAnalysis]) {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename).expect("Failed to create file");

    // Header
    writeln!(file, "seed,k,discriminant,is_perfect_square,quality_score,leg_3,leg_5,leg_7,mod_2,mod_5,has_favorable_residues,is_prime").unwrap();

    // k=0 data
    for r in results_k0 {
        writeln!(file, "{},{},{},{},{},{},{},{},{},{},{},{}",
                 r.seed, r.k, r.discriminant, r.is_perfect_square as u8,
                 r.quality_score, r.leg_3, r.leg_5, r.leg_7,
                 r.mod_2, r.mod_5, r.has_favorable_residues as u8, r.is_prime as u8).unwrap();
    }

    // k=1 data
    for r in results_k1 {
        writeln!(file, "{},{},{},{},{},{},{},{},{},{},{},{}",
                 r.seed, r.k, r.discriminant, r.is_perfect_square as u8,
                 r.quality_score, r.leg_3, r.leg_5, r.leg_7,
                 r.mod_2, r.mod_5, r.has_favorable_residues as u8, r.is_prime as u8).unwrap();
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("🔬 Discriminant-Residue Coupling Analyzer");
    println!("Testing: Do residue constraints select seeds with better discriminants?\n");

    // Test Base 10 M=2 (3,7) - the configuration that shows k=1 advantage
    analyze_coupling(10, 3, 7, 2);

    println!("\n✅ Analysis complete!");
}
