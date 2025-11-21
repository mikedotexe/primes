// Phase-Lock Harmonic Hypothesis Testing Framework
//
// Tests the hypothesis that bases with "phase-locked prime pairs" (where p₁ + p₂ = base
// and both are prime) exhibit harmonic resonance enabling padding optimization (k*>0)
// when midpoint m < 7.
//
// Critical prediction: Base 12 with phase-lock (5,7) and harmonic power 35 should show
// k*>0 for M=2, stronger than Base 10's (3,7) with power 21.

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use std::collections::HashMap;

// ============================================================================
// Phase-Lock Detection and Analysis
// ============================================================================

/// Find all phase-locked prime pairs (a, b) where a + b = base and both are prime
fn find_phase_locked_prime_pairs(base: u32) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();

    for a in 2..base {
        let b = base - a;
        if b > a {
            let a_big = BigUint::from(a);
            let b_big = BigUint::from(b);
            if is_prime(&a_big) && is_prime(&b_big) {
                pairs.push((a, b));
            }
        }
    }

    pairs
}

/// Calculate maximum harmonic power across all phase-locked pairs
/// Harmonic power = p₁ × p₂ for each pair
fn harmonic_power(pairs: &[(u32, u32)]) -> u32 {
    pairs.iter()
        .map(|(p1, p2)| p1 * p2)
        .max()
        .unwrap_or(0)
}

/// Predict k* behavior based on phase-lock hypothesis decision tree
fn predict_k_star_exception(base: u32, m: usize) -> (String, String) {
    // M≥3: Universal k*=0 (asymptotic regime)
    if m >= 3 {
        return ("0".to_string(), "Asymptotic regime: coprimality dominates".to_string());
    }

    // M=1: Mixed regime, insufficient theory
    if m == 1 {
        return ("?".to_string(), "Mixed regime: test required".to_string());
    }

    // M=2: Test phase-lock conditions
    let midpoint = base / 2;
    let phase_locked_primes = find_phase_locked_prime_pairs(base);

    // Condition 1: Midpoint < 7 (computational tractability)
    if midpoint >= 7 {
        return ("0".to_string(), "Midpoint ≥7: chaos threshold exceeded".to_string());
    }

    // Condition 2: Valid phase-locked prime pair exists
    if phase_locked_primes.is_empty() {
        return ("0".to_string(), "No phase-locked prime pairs".to_string());
    }

    // Condition 3: Sufficient harmonic power (empirical threshold ~15)
    let max_harmonic_power = harmonic_power(&phase_locked_primes);
    if max_harmonic_power < 15 {
        return ("0".to_string(), format!("Insufficient harmonic power ({})", max_harmonic_power));
    }

    // Condition 4: Sufficient candidate space
    let candidate_space = base.pow(m as u32);
    if candidate_space < 50 {
        return ("0".to_string(), "Insufficient candidate space".to_string());
    }

    // All conditions satisfied
    (">0".to_string(), "All conditions met: harmonic optimization possible".to_string())
}

// ============================================================================
// Membrane Testing Infrastructure
// ============================================================================

#[allow(dead_code)]
struct TestResult {
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k: u32,
    samples: usize,
    primes: usize,
    density: f64,
}

/// Generate membrane number with given parameters using BigUint
fn generate_membrane(base: u32, outer: u32, inner: u32, m: usize, k: u32, seed: u64) -> BigUint {
    let base_big = BigUint::from(base);
    let mut result = BigUint::zero();
    let mut position = 0;

    let mut add_digit = |digit: u32| {
        result += BigUint::from(digit) * base_big.pow(position);
        position += 1;
    };

    // Structure: outer [k×0] inner [k×0] SEED [k×0] inner [k×0] outer

    // Left side
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

    // Right side (mirror)
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

/// Test membrane configuration with given parameters
fn test_membrane_config(base: u32, outer: u32, inner: u32, m: usize, k: u32, samples: usize) -> TestResult {
    let mut prime_count = 0;

    // Calculate seed range
    let seed_min = base.pow((m.saturating_sub(1)) as u32) as u64;
    let seed_max = base.pow(m as u32) as u64;

    // Generate and test samples
    let mut seed = seed_min;
    for _ in 0..samples {
        let membrane = generate_membrane(base, outer, inner, m, k, seed);
        if is_prime(&membrane) {
            prime_count += 1;
        }

        // Advance seed
        seed += 1;
        if seed >= seed_max {
            seed = seed_min;
        }
    }

    let density = prime_count as f64 / samples as f64;

    TestResult {
        base,
        outer,
        inner,
        m,
        k,
        samples,
        primes: prime_count,
        density,
    }
}

/// Find optimal k for given configuration across k∈{0,1,2}
fn find_optimal_k(base: u32, outer: u32, inner: u32, m: usize, samples: usize) -> (u32, Vec<TestResult>) {
    let mut results = Vec::new();
    let mut best_k = 0;
    let mut best_density = 0.0;

    for k in 0..=2 {
        let result = test_membrane_config(base, outer, inner, m, k, samples);

        if result.density > best_density {
            best_density = result.density;
            best_k = k;
        }

        results.push(result);
    }

    (best_k, results)
}

// ============================================================================
// Main Testing Framework
// ============================================================================

fn main() {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║  PHASE-LOCK HARMONIC HYPOTHESIS TESTING               ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Hypothesis: Bases with phase-locked prime pairs (p₁ + p₂ = base)");
    println!("create harmonic resonance enabling padding optimization (k*>0)");
    println!("when midpoint m < 7.\n");

    // Test cases
    let test_bases = vec![
        (12, "Strongest phase-lock below chaos threshold"),
        (22, "Tests midpoint dominance over phase-lock"),
        (15, "Tests threshold boundary behavior"),
    ];

    let mut all_results = HashMap::new();

    // Analyze each base
    for (base, rationale) in &test_bases {
        println!("════════════════════════════════════════════════════════");
        println!("BASE {}: {}", base, rationale);
        println!("════════════════════════════════════════════════════════\n");

        // Find phase-locked pairs
        let pairs = find_phase_locked_prime_pairs(*base);
        let power = harmonic_power(&pairs);
        let midpoint = base / 2;

        println!("  Base factorization: {}", factorize(*base));
        println!("  Midpoint: {}", midpoint);
        println!("  Phase-locked prime pairs: {:?}", pairs);
        println!("  Harmonic power: {}", power);

        // Get prediction
        let (predicted_k, reasoning) = predict_k_star_exception(*base, 2);
        println!("  \n  PREDICTION: k* {} ({})", predicted_k, reasoning);

        // Test with standard boundary pairs
        let boundary_pairs = get_boundary_pairs(*base);

        println!("\n  TESTING M=2 with {} boundary pairs:\n", boundary_pairs.len());

        let mut base_results = Vec::new();

        for (outer, inner) in &boundary_pairs {
            println!("  Pair ({},{})", outer, inner);

            let (optimal_k, results) = find_optimal_k(*base, *outer, *inner, 2, 1000);

            for result in &results {
                println!("    k={}: {}/1000 = {:.1}%",
                    result.k, result.primes, result.density * 100.0);
            }

            println!("    → k* = {}\n", optimal_k);

            base_results.push(((*outer, *inner), optimal_k, results));
        }

        // Summary for this base
        let k_star_counts: HashMap<u32, usize> = base_results.iter()
            .map(|(_, k, _)| *k)
            .fold(HashMap::new(), |mut acc, k| {
                *acc.entry(k).or_insert(0) += 1;
                acc
            });

        let dominant_k = k_star_counts.iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&k, _)| k)
            .unwrap_or(0);

        println!("  RESULT: Dominant k* = {}", dominant_k);
        println!("  Distribution: {:?}", k_star_counts);

        // Compare with prediction
        let matches = if predicted_k == "?" {
            "Uncertain"
        } else if (predicted_k == ">0" && dominant_k > 0) || (predicted_k == "0" && dominant_k == 0) {
            "✓ MATCH"
        } else {
            "✗ HYPOTHESIS REFUTED"
        };

        println!("  Match prediction: {}\n", matches);

        all_results.insert(*base, (dominant_k, predicted_k.clone(), matches.to_string()));
    }

    // Final summary
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                  HYPOTHESIS EVALUATION                 ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Summary of Results:\n");
    println!("  Base | Predicted | Actual | Match | Interpretation");
    println!("  -----|-----------|--------|-------|----------------");

    for base in [12, 22, 15] {
        if let Some((actual_k, predicted, matches)) = all_results.get(&base) {
            let interp = match base {
                12 => "Critical test - refutes if k*=0",
                22 => "Chaos threshold validation",
                15 => "Boundary case control",
                _ => "Unknown",
            };
            println!("  {:4} | {:9} | {:6} | {:5} | {}",
                base, predicted, actual_k,
                if matches.contains("MATCH") { "✓" } else { "✗" },
                interp);
        }
    }

    println!("\nCONCLUSION:");

    if let Some((actual_12, _, _match_12)) = all_results.get(&12) {
        if *actual_12 == 0 {
            println!("  Base 12 shows k*=0 despite harmonic power 35 > Base 10's 21");
            println!("  PHASE-LOCK HYPOTHESIS DECISIVELY REFUTED");
            println!("\n  Implications:");
            println!("  • Harmonic power does NOT predict padding optimization");
            println!("  • Self-referential arithmetic resonance is not causal");
            println!("  • Base 10 exception requires base-specific mechanism");
            println!("  • Recommend: Hardy-Littlewood singular series analysis");
        } else {
            println!("  Base 12 shows k*>0 matching phase-lock prediction");
            println!("  PHASE-LOCK HYPOTHESIS SUPPORTED");
            println!("\n  Implications:");
            println!("  • Harmonic resonance is real mathematical phenomenon");
            println!("  • Phase-locked pairs enable padding optimization");
            println!("  • Major discovery in arithmetic number theory");
            println!("  • Potential Nature-level publication");
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn factorize(n: u32) -> String {
    let mut factors = Vec::new();
    let mut remaining = n;
    let mut d = 2;

    while d * d <= remaining {
        let mut count = 0;
        while remaining % d == 0 {
            remaining /= d;
            count += 1;
        }
        if count > 0 {
            if count == 1 {
                factors.push(format!("{}", d));
            } else {
                factors.push(format!("{}^{}", d, count));
            }
        }
        d += 1;
    }

    if remaining > 1 {
        factors.push(format!("{}", remaining));
    }

    if factors.is_empty() {
        "1".to_string()
    } else {
        factors.join(" × ")
    }
}

fn get_boundary_pairs(base: u32) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();

    // Find coprime digit pairs
    for outer in 1..base {
        if gcd(outer, base) != 1 {
            continue;
        }

        for inner in 1..base {
            if gcd(inner, base) != 1 {
                continue;
            }

            if outer != inner {
                pairs.push((outer, inner));
            }
        }
    }

    // Take first 3 pairs for testing
    pairs.truncate(3);
    pairs
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
