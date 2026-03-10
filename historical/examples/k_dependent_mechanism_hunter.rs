//! K-Dependent Mechanism Hunter
//!
//! Now that we know discriminants are k-independent, what DOES explain
//! the 2× density difference between k=0 and k=1?
//!
//! ## Candidates
//!
//! 1. **Length Penalty (PNT)**:
//!    - k=0: 6 digits, k=1: 10 digits
//!    - Expected ratio: ~6/10 = 1.67×
//!    - Observed ratio: 21.1%/10.0% = 2.1×
//!    - Hypothesis: Length explains most, but not all
//!
//! 2. **Mirror Symmetry Index**:
//!    - k=0: Less symmetric (seed dominates structure)
//!    - k=1: More symmetric (zeros create regularity)
//!    - Hypothesis: Higher symmetry → more mirror obstruction
//!
//! 3. **Higher-Order Modular Obstructions**:
//!    - Beyond mod 2, mod 5 (which are identical for both k)
//!    - Test mod 3, mod 7, mod 11, mod 13
//!    - Hypothesis: k=1 creates systematic obstructions

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;

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

fn count_digits(n: &BigUint, base: u32) -> usize {
    if n.is_zero() {
        return 1;
    }
    let base_big = BigUint::from(base);
    let mut temp = n.clone();
    let mut count = 0;
    while temp > BigUint::zero() {
        temp /= &base_big;
        count += 1;
    }
    count
}

// ============================================================================
// Symmetry Analysis
// ============================================================================

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

    // Handle middle digit if odd length
    let total_pairs = if n % 2 == 0 { pairs } else { pairs + 1 };

    matches as f64 / total_pairs as f64
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

fn zero_count(digits: &[u32]) -> usize {
    digits.iter().filter(|&&d| d == 0).count()
}

// ============================================================================
// Analysis Structure
// ============================================================================

#[derive(Debug, Clone)]
struct MechanismAnalysis {
    seed: u64,
    k: u32,

    // Membrane properties
    membrane: BigUint,
    is_prime: bool,
    digit_count: usize,

    // Symmetry metrics
    symmetry_index: f64,
    zero_percentage: f64,

    // Higher-order residues
    mod_3: u64,
    mod_7: u64,
    mod_11: u64,
    mod_13: u64,
}

fn analyze_seed(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
    seed: u64,
) -> MechanismAnalysis {
    let membrane = construct_membrane(base, outer, inner, m, k, seed);
    let is_prime_result = is_prime(&membrane);
    let digit_count = count_digits(&membrane, base);

    // Extract digits for symmetry analysis
    let digits = extract_digits(&membrane, base);
    let symmetry_index = mirror_symmetry_index(&digits);
    let zeros = zero_count(&digits);
    let zero_percentage = zeros as f64 / digits.len() as f64;

    // Higher-order residues
    let mod_3 = (&membrane % 3u32).to_u64_digits().get(0).copied().unwrap_or(0);
    let mod_7 = (&membrane % 7u32).to_u64_digits().get(0).copied().unwrap_or(0);
    let mod_11 = (&membrane % 11u32).to_u64_digits().get(0).copied().unwrap_or(0);
    let mod_13 = (&membrane % 13u32).to_u64_digits().get(0).copied().unwrap_or(0);

    MechanismAnalysis {
        seed,
        k,
        membrane,
        is_prime: is_prime_result,
        digit_count,
        symmetry_index,
        zero_percentage,
        mod_3,
        mod_7,
        mod_11,
        mod_13,
    }
}

// ============================================================================
// Comparative Analysis
// ============================================================================

fn hunt_mechanism(base: u32, outer: u32, inner: u32, m: usize) {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║          K-DEPENDENT MECHANISM HUNTER                         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("Configuration: Base {}, ({},{}), M={}", base, outer, inner, m);
    println!("Hunting for the mechanism that makes k=0 outperform k=1...\n");

    let seed_min = if m > 1 {
        (base as u64).pow((m - 1) as u32)
    } else {
        1
    };
    let seed_max = (base as u64).pow(m as u32);

    let mut results_k0 = Vec::new();
    let mut results_k1 = Vec::new();

    println!("Analyzing {} seeds for each k value...\n", seed_max - seed_min);

    for seed in seed_min..seed_max {
        results_k0.push(analyze_seed(base, outer, inner, m, 0, seed));
        results_k1.push(analyze_seed(base, outer, inner, m, 1, seed));
    }

    // ========================================================================
    // Test 1: Length Penalty (PNT Hypothesis)
    // ========================================================================

    println!("═══════════════════════════════════════════════════════════════");
    println!("HYPOTHESIS 1: Length Penalty (Prime Number Theorem)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let avg_length_k0 = results_k0.iter().map(|r| r.digit_count).sum::<usize>() as f64 / results_k0.len() as f64;
    let avg_length_k1 = results_k1.iter().map(|r| r.digit_count).sum::<usize>() as f64 / results_k1.len() as f64;

    let density_k0 = results_k0.iter().filter(|r| r.is_prime).count() as f64 / results_k0.len() as f64;
    let density_k1 = results_k1.iter().filter(|r| r.is_prime).count() as f64 / results_k1.len() as f64;

    println!("Average digit count:");
    println!("  k=0: {:.1} digits", avg_length_k0);
    println!("  k=1: {:.1} digits", avg_length_k1);

    let length_ratio = avg_length_k0 / avg_length_k1;
    let density_ratio = density_k0 / density_k1;

    println!("\nPNT predicts density ratio ≈ length ratio:");
    println!("  Length ratio: {:.3}×", length_ratio);
    println!("  Observed density ratio: {:.3}×", density_ratio);
    println!("  Match: {:.1}%\n", (length_ratio / density_ratio) * 100.0);

    if (length_ratio / density_ratio - 1.0).abs() < 0.2 {
        println!("✅ LENGTH PENALTY explains the difference (within 20%)");
    } else if (length_ratio / density_ratio - 1.0).abs() < 0.5 {
        println!("⚠️  LENGTH PENALTY explains MOST of the difference");
        println!("   Residual: {:.1}× unexplained", density_ratio / length_ratio);
    } else {
        println!("❌ LENGTH PENALTY does NOT fully explain the difference");
        println!("   Residual: {:.1}× unexplained", density_ratio / length_ratio);
    }

    // ========================================================================
    // Test 2: Mirror Symmetry Index
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("HYPOTHESIS 2: Mirror Symmetry Obstruction");
    println!("═══════════════════════════════════════════════════════════════\n");

    let avg_symmetry_k0 = results_k0.iter().map(|r| r.symmetry_index).sum::<f64>() / results_k0.len() as f64;
    let avg_symmetry_k1 = results_k1.iter().map(|r| r.symmetry_index).sum::<f64>() / results_k1.len() as f64;

    let avg_zeros_k0 = results_k0.iter().map(|r| r.zero_percentage).sum::<f64>() / results_k0.len() as f64;
    let avg_zeros_k1 = results_k1.iter().map(|r| r.zero_percentage).sum::<f64>() / results_k1.len() as f64;

    println!("Mirror symmetry index (1.0 = perfect palindrome):");
    println!("  k=0: {:.3}", avg_symmetry_k0);
    println!("  k=1: {:.3}", avg_symmetry_k1);
    println!("  Δ: {:+.3} ({:+.1}%)\n", avg_symmetry_k1 - avg_symmetry_k0,
             ((avg_symmetry_k1 - avg_symmetry_k0) / avg_symmetry_k0) * 100.0);

    println!("Zero percentage:");
    println!("  k=0: {:.1}%", avg_zeros_k0 * 100.0);
    println!("  k=1: {:.1}%", avg_zeros_k1 * 100.0);
    println!("  Δ: {:+.1}pp\n", (avg_zeros_k1 - avg_zeros_k0) * 100.0);

    // Correlation: symmetry vs primality
    let high_sym_k0 = results_k0.iter().filter(|r| r.symmetry_index > 0.7).collect::<Vec<_>>();
    let high_sym_k1 = results_k1.iter().filter(|r| r.symmetry_index > 0.7).collect::<Vec<_>>();

    let high_sym_density_k0 = if !high_sym_k0.is_empty() {
        high_sym_k0.iter().filter(|r| r.is_prime).count() as f64 / high_sym_k0.len() as f64
    } else {
        0.0
    };

    let high_sym_density_k1 = if !high_sym_k1.is_empty() {
        high_sym_k1.iter().filter(|r| r.is_prime).count() as f64 / high_sym_k1.len() as f64
    } else {
        0.0
    };

    println!("Prime density for highly symmetric (>0.7) membranes:");
    println!("  k=0: {:.1}% ({}/{})", high_sym_density_k0 * 100.0,
             high_sym_k0.iter().filter(|r| r.is_prime).count(), high_sym_k0.len());
    println!("  k=1: {:.1}% ({}/{})", high_sym_density_k1 * 100.0,
             high_sym_k1.iter().filter(|r| r.is_prime).count(), high_sym_k1.len());

    if avg_symmetry_k1 > avg_symmetry_k0 && high_sym_density_k1 < density_k1 {
        println!("\n✅ SYMMETRY OBSTRUCTION detected:");
        println!("   k=1 has higher symmetry AND lower density for symmetric cases");
    } else {
        println!("\n❌ No clear symmetry obstruction pattern");
    }

    // ========================================================================
    // Test 3: Higher-Order Modular Obstructions
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("HYPOTHESIS 3: Higher-Order Modular Obstructions");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Check if k=0 and k=1 have different mod distributions
    for modulus in [3, 7, 11, 13] {
        println!("Mod {} distribution:", modulus);

        let get_mod = |r: &MechanismAnalysis| match modulus {
            3 => r.mod_3,
            7 => r.mod_7,
            11 => r.mod_11,
            13 => r.mod_13,
            _ => 0,
        };

        // Count distribution for each k
        let mut dist_k0 = vec![0; modulus as usize];
        let mut dist_k1 = vec![0; modulus as usize];

        for r in &results_k0 {
            dist_k0[get_mod(r) as usize] += 1;
        }
        for r in &results_k1 {
            dist_k1[get_mod(r) as usize] += 1;
        }

        // Find max difference
        let mut max_diff = 0;
        let mut max_diff_residue = 0;

        for residue in 0..modulus {
            let diff = (dist_k0[residue as usize] as i32 - dist_k1[residue as usize] as i32).abs();
            if diff > max_diff {
                max_diff = diff;
                max_diff_residue = residue;
            }
        }

        println!("  Max difference: ±{} at residue {}", max_diff, max_diff_residue);

        if max_diff > 10 {
            println!("  ⚠️  SIGNIFICANT difference detected!");
            // Show the distribution
            for residue in 0..modulus {
                if (dist_k0[residue as usize] as i32 - dist_k1[residue as usize] as i32).abs() > 5 {
                    println!("    ≡{} (mod {}): k=0={}, k=1={}",
                             residue, modulus, dist_k0[residue as usize], dist_k1[residue as usize]);
                }
            }
        } else {
            println!("  ✓ Similar distributions");
        }
    }

    // ========================================================================
    // Synthesis
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("MECHANISM IDENTIFICATION SUMMARY");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Observed density ratio: {:.2}× (k=0 / k=1)", density_ratio);
    println!("Explained by length penalty: {:.2}×", length_ratio);
    println!("Residual: {:.2}×\n", density_ratio / length_ratio);

    if (density_ratio / length_ratio - 1.0).abs() < 0.15 {
        println!("🎯 PRIMARY MECHANISM: Length Penalty (PNT)");
        println!("   → k=0 has fewer digits → higher expected density");
        println!("   → Accounts for ~100% of observed difference");
    } else if (density_ratio / length_ratio - 1.0).abs() < 0.3 {
        println!("🎯 PRIMARY MECHANISM: Length Penalty (PNT)");
        println!("   → Accounts for ~{:.0}% of difference",
                 (length_ratio / density_ratio) * 100.0);
        println!("🔍 SECONDARY MECHANISMS: {:.0}% unexplained",
                 (1.0 - length_ratio / density_ratio) * 100.0);
        println!("   → Check symmetry/modular obstruction results above");
    } else {
        println!("❓ COMPLEX INTERACTION:");
        println!("   Length penalty alone insufficient");
        println!("   Multiple mechanisms likely interacting");
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("🔬 K-Dependent Mechanism Hunter");
    println!("Finding what causes k=0 to outperform k=1...\n");

    // Test Base 10 M=2 (3,7) - the k-dependent configuration
    hunt_mechanism(10, 3, 7, 2);

    println!("\n✅ Hunt complete!");
}
