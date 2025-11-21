//! Multi-Layer Composite Predictor
//!
//! ## Architecture
//!
//! Combines ALL discovered constraints into unified primality prediction:
//!
//! **Layer 1 (Algebraic)**: Perfect square lock + Legendre symbols + discriminant quality
//! **Layer 2 (Modular)**: Coprimality + residue classes
//! **Layer 3 (Geometric)**: Mirror obstruction + symmetry index
//! **Layer 4 (Analytic)**: Length penalty (PNT baseline)
//!
//! ## Prediction Model
//!
//! ```
//! if perfect_square_lock OR mirror_obstruction:
//!     P(prime) = 0.0  (deterministic composite)
//! else:
//!     P(prime) = PNT_baseline × discriminant_quality × (1 - symmetry_penalty) × coprimality_bonus
//! ```
//!
//! ## Validation Strategy
//!
//! Test predictor accuracy across multiple bases, compare to actual primality.

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// ============================================================================
// Discriminant and Legendre Symbol Computation
// ============================================================================

fn compute_discriminant(outer: u32, seed: u64) -> i128 {
    let s = seed as i128;
    let a = outer as i128;
    s * s - 4 * a * a
}

fn is_perfect_square(n: i128) -> bool {
    if n < 0 {
        return false;
    }
    let sqrt = (n as f64).sqrt() as i128;
    sqrt * sqrt == n || (sqrt + 1) * (sqrt + 1) == n
}

fn legendre_symbol(a: i128, p: i128) -> i8 {
    if a % p == 0 {
        return 0;
    }

    // Simplified Legendre computation (for small primes)
    let mut result = 1i128;
    let mut a_mod = ((a % p) + p) % p;
    let mut p_val = p;

    while a_mod != 0 {
        while a_mod % 2 == 0 {
            a_mod /= 2;
            if p_val % 8 == 3 || p_val % 8 == 5 {
                result = -result;
            }
        }
        std::mem::swap(&mut a_mod, &mut p_val);
        if a_mod % 4 == 3 && p_val % 4 == 3 {
            result = -result;
        }
        a_mod %= p_val;
    }

    if p_val == 1 {
        result as i8
    } else {
        0
    }
}

fn discriminant_quality_score(discriminant: i128) -> f64 {
    // Perfect square → automatic fail (return 0.0)
    if is_perfect_square(discriminant) {
        return 0.0;
    }

    let primes = vec![3, 5, 7, 11, 13];
    let mut admissible = 0;
    let mut obstructed = 0;
    let mut divisible = 0;

    for &p in &primes {
        let leg = legendre_symbol(discriminant, p);
        match leg {
            -1 => admissible += 1,  // Non-residue (good)
            1 => obstructed += 1,   // Quadratic residue (obstruction)
            0 => divisible += 1,    // Divisible (bad)
            _ => {}
        }
    }

    // Quality score: favor admissible, penalize obstructed/divisible
    let raw_score = admissible as f64 - obstructed as f64 - 5.0 * divisible as f64;

    // Normalize to [0, 1] range (roughly)
    // Max possible: 5 admissible = +5
    // Min possible: 5 divisible = -25
    // Range: 30, center at 0
    ((raw_score + 25.0) / 30.0).max(0.0).min(1.0)
}

// ============================================================================
// Membrane Construction and Analysis
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

    add_digit(outer);
    for _ in 0..k {
        add_digit(0);
    }
    add_digit(inner);
    for _ in 0..k {
        add_digit(0);
    }

    let mut seed_val = seed;
    for _ in 0..m {
        add_digit((seed_val % base as u64) as u32);
        seed_val /= base as u64;
    }

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

fn is_repeated_digit_seed(seed: u64, base: u32, m: usize) -> bool {
    let mut digits = Vec::new();
    let mut s = seed;
    for _ in 0..m {
        digits.push(s % base as u64);
        s /= base as u64;
    }

    if digits.is_empty() {
        return false;
    }

    digits.iter().all(|&d| d == digits[0])
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
// Multi-Layer Prediction Model
// ============================================================================

#[derive(Debug, Clone)]
struct PrimalityPrediction {
    // Input parameters
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
    seed: u64,

    // Layer 1: Algebraic (discriminant)
    discriminant: i128,
    perfect_square_lock: bool,       // true = LOCKED (100% composite)
    discriminant_quality: f64,       // 0.0-1.0

    // Layer 2: Modular
    coprime_to_base: bool,           // gcd(outer, base) = 1

    // Layer 3: Geometric
    mirror_obstruction: bool,        // Repeated digit → perfect palindrome
    symmetry_index: f64,             // 0.0-1.0 (1.0 = perfect mirror)

    // Layer 4: Analytic
    digit_count: usize,
    length_adjustment: f64,          // PNT-based baseline

    // Combined prediction
    predicted_probability: f64,
    prediction_tier: String,         // "LOCKED", "OBSTRUCTED", "FAVORABLE", "UNFAVORABLE"

    // Actual result
    is_prime: bool,
}

impl PrimalityPrediction {
    fn new(base: u32, outer: u32, inner: u32, m: usize, k: u32, seed: u64) -> Self {
        // Construct membrane
        let membrane = construct_membrane(base, outer, inner, m, k, seed);
        let is_prime_result = is_prime(&membrane);

        // Layer 1: Algebraic
        let discriminant = compute_discriminant(outer, seed);
        let perfect_square_lock = is_perfect_square(discriminant);
        let discriminant_quality = discriminant_quality_score(discriminant);

        // Layer 2: Modular
        let coprime_to_base = gcd(outer, base) == 1 && gcd(inner, base) == 1;

        // Layer 3: Geometric
        let mirror_obstruction = k == 0 && is_repeated_digit_seed(seed, base, m);
        let digits = extract_digits(&membrane, base);
        let symmetry_index = mirror_symmetry_index(&digits);

        // Layer 4: Analytic (PNT baseline)
        let digit_count = count_digits(&membrane, base);
        // Baseline prime density ≈ 1/(digit_count * ln(base))
        // For comparison, use a reference digit count (e.g., 6)
        let length_adjustment = 6.0 / digit_count as f64;

        // Compute combined prediction
        let (predicted_probability, prediction_tier) = if perfect_square_lock {
            (0.0, "LOCKED".to_string())
        } else if mirror_obstruction {
            (0.0, "OBSTRUCTED".to_string())
        } else {
            // Multiplicative model: baseline × quality factors
            let base_prob = 0.15; // Empirical baseline (15% from our tests)

            let disc_factor = 0.5 + 0.5 * discriminant_quality; // 0.5-1.0 range
            let coprime_factor = if coprime_to_base { 1.2 } else { 0.8 };
            let symmetry_penalty = 1.0 - 0.5 * symmetry_index; // High symmetry → penalty

            let prob = base_prob
                * length_adjustment
                * disc_factor
                * coprime_factor
                * symmetry_penalty;

            let tier = if prob > 0.20 {
                "FAVORABLE"
            } else {
                "UNFAVORABLE"
            };

            (prob.min(1.0), tier.to_string())
        };

        PrimalityPrediction {
            base,
            outer,
            inner,
            m,
            k,
            seed,
            discriminant,
            perfect_square_lock,
            discriminant_quality,
            coprime_to_base,
            mirror_obstruction,
            symmetry_index,
            digit_count,
            length_adjustment,
            predicted_probability,
            prediction_tier,
            is_prime: is_prime_result,
        }
    }
}

// ============================================================================
// Validation and Analysis
// ============================================================================

fn validate_predictor(base: u32, outer: u32, inner: u32, m: usize, k: u32) {
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Config: Base {} ({},{}) M={} k={}", base, outer, inner, m, k);
    println!("═══════════════════════════════════════════════════════════════\n");

    let seed_min = if m > 1 {
        (base as u64).pow((m - 1) as u32)
    } else {
        1
    };
    let seed_max = (base as u64).pow(m as u32).min(seed_min + 1000); // Cap for performance

    let mut predictions = Vec::new();

    for seed in seed_min..seed_max {
        predictions.push(PrimalityPrediction::new(base, outer, inner, m, k, seed));
    }

    // Compute accuracy by tier
    let mut tier_stats: HashMap<String, (usize, usize, usize)> = HashMap::new();

    for pred in &predictions {
        let entry = tier_stats.entry(pred.prediction_tier.clone()).or_insert((0, 0, 0));
        entry.0 += 1; // total count
        if pred.is_prime {
            entry.1 += 1; // actual primes
        }
        if pred.predicted_probability > 0.15 {
            entry.2 += 1; // predicted primes
        }
    }

    println!("Prediction tier breakdown:");
    println!("{:<15} {:>8} {:>12} {:>12} {:>10}", "Tier", "Count", "Actual Prime", "Pred Prime", "Accuracy");
    println!("{:-<60}", "");

    for tier in ["LOCKED", "OBSTRUCTED", "UNFAVORABLE", "FAVORABLE"] {
        if let Some((count, actual, predicted)) = tier_stats.get(tier) {
            let accuracy = if *count > 0 {
                if tier == "LOCKED" || tier == "OBSTRUCTED" {
                    // For deterministic tiers, accuracy = how many were correctly composite
                    (count - actual) as f64 / *count as f64 * 100.0
                } else {
                    // For probabilistic tiers, show actual prime rate
                    *actual as f64 / *count as f64 * 100.0
                }
            } else {
                0.0
            };

            println!("{:<15} {:>8} {:>12} {:>12} {:>9.1}%",
                     tier, count, actual, predicted, accuracy);
        }
    }

    // Overall statistics
    let total_count = predictions.len();
    let total_primes = predictions.iter().filter(|p| p.is_prime).count();
    let locked_count = predictions.iter().filter(|p| p.perfect_square_lock).count();
    let obstructed_count = predictions.iter().filter(|p| p.mirror_obstruction).count();

    println!("\nOverall statistics:");
    println!("  Total seeds tested: {}", total_count);
    println!("  Actual primes: {} ({:.1}%)", total_primes, total_primes as f64 / total_count as f64 * 100.0);
    println!("  Perfect square locked: {} ({:.1}%)", locked_count, locked_count as f64 / total_count as f64 * 100.0);
    println!("  Mirror obstructed: {} ({:.1}%)", obstructed_count, obstructed_count as f64 / total_count as f64 * 100.0);

    // Validate deterministic constraints
    let locked_primes = predictions.iter()
        .filter(|p| p.perfect_square_lock && p.is_prime)
        .count();
    let obstructed_primes = predictions.iter()
        .filter(|p| p.mirror_obstruction && p.is_prime)
        .count();

    println!("\nDeterministic constraint validation:");
    println!("  Perfect square lock violations: {} (should be 0)", locked_primes);
    println!("  Mirror obstruction violations: {} (should be 0)", obstructed_primes);

    if locked_primes == 0 && obstructed_primes == 0 {
        println!("  ✅ All deterministic constraints hold!");
    } else {
        println!("  ⚠️  Violations detected!");
    }
}

fn run_comprehensive_validation() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║         MULTI-LAYER COMPOSITE PREDICTOR                      ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("Validating 4-layer prediction model across multiple configs...\n");

    // Test multiple configurations
    let configs = vec![
        (6, 1, 5, 2, 0, "Base 6 (1,5) M=2 k=0 - champion"),
        (10, 3, 7, 2, 0, "Base 10 (3,7) M=2 k=0 - standard"),
        (10, 3, 7, 2, 1, "Base 10 (3,7) M=2 k=1 - comparison"),
        (12, 1, 5, 2, 0, "Base 12 (1,5) M=2 k=0 - universal"),
        (30, 11, 7, 2, 0, "Base 30 (11,7) M=2 k=0 - high performer"),
    ];

    for (base, outer, inner, m, k, desc) in configs {
        println!("\n{}", desc);
        validate_predictor(base, outer, inner, m, k);
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Validation complete!");
    println!("═══════════════════════════════════════════════════════════════");
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("🔬 Multi-Layer Composite Predictor");
    println!("Combining all discovered constraints into unified model...\n");

    run_comprehensive_validation();

    println!("\n✅ Predictor validation complete!");
}
