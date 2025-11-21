// Base 15 Midpoint Threshold Test
// ================================
//
// CRITICAL EXPERIMENT: Testing Boundary Case + Non-2×p Control
//
// HYPOTHESIS: Bases with midpoint m ≥ 7 show universal k*=0
//
// BASE 15 TEST:
// - Base: 15 = 3×5 (NOT form 2×p - control for factorization pattern)
// - Midpoint: 7.5 (AT chaos threshold, but not integer)
// - Largest prime factor: 5 (< 7)
// - Prediction: BOUNDARY CASE - competing effects
//
// SIGNIFICANCE:
// 1. Tests if m≈7 is sufficient (despite non-integer midpoint)
// 2. Tests if p_max<7 allows exceptions (factor 5 vs threshold 7)
// 3. Controls for 2×p pattern (Base 15 = 3×5, not 2×p)
//
// COMPARISON:
// - Base 10 (2×5, m=5, p_max=5):   M=2 shows k*=1 (exception)
// - Base 14 (2×7, m=7, p_max=7):   M=2 shows k*=0 (at threshold)
// - Base 15 (3×5, m=7.5, p_max=5): ? (TESTING NOW)
//
// PREDICTIONS:
// - If m≈7 drives behavior: Base 15 shows k*=0 (like Base 14)
// - If p_max<7 allows exceptions: Base 15 shows k*>0 (like Base 10)
// - If 2×p is necessary: Base 15 (3×5) provides control

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use rand::Rng;
use std::time::Instant;

fn construct_membrane(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k_outer: u32,
    k_inner: u32,
    seed: u64,
) -> Option<BigUint> {
    let base_big = BigUint::from(base);
    let mut result = BigUint::zero();
    let mut position = 0;

    let mut add_digit = |digit: u32| {
        result += BigUint::from(digit) * base_big.pow(position);
        position += 1;
    };

    add_digit(outer);
    for _ in 0..k_outer {
        add_digit(0);
    }
    add_digit(inner);
    for _ in 0..k_inner {
        add_digit(0);
    }

    let mut seed_val = seed;
    for _ in 0..m {
        add_digit((seed_val % base as u64) as u32);
        seed_val /= base as u64;
    }

    for _ in 0..k_inner {
        add_digit(0);
    }
    add_digit(inner);
    for _ in 0..k_outer {
        add_digit(0);
    }
    add_digit(outer);

    Some(result)
}

#[derive(Debug)]
struct TestResult {
    m: usize,
    k_outer: u32,
    k_inner: u32,
    samples: usize,
    primes_found: usize,
    density: f64,
}

fn measure_density(
    base: u32,
    outer: u32,
    inner: u32,
    m: usize,
    k_outer: u32,
    k_inner: u32,
    samples: usize,
) -> TestResult {
    let mut rng = rand::thread_rng();
    let mut primes_found = 0;

    let seed_min = base.pow((m - 1) as u32) as u64;
    let seed_max = base.pow(m as u32) as u64;

    for _ in 0..samples {
        let seed = rng.gen_range(seed_min..seed_max);

        if let Some(membrane) = construct_membrane(base, outer, inner, m, k_outer, k_inner, seed) {
            if is_prime(&membrane) {
                primes_found += 1;
            }
        }
    }

    let density = primes_found as f64 / samples as f64;

    TestResult {
        m,
        k_outer,
        k_inner,
        samples,
        primes_found,
        density,
    }
}

fn proportion_z_test(x1: usize, n1: usize, x2: usize, n2: usize) -> (f64, f64) {
    let p1 = x1 as f64 / n1 as f64;
    let p2 = x2 as f64 / n2 as f64;
    let p_pooled = (x1 + x2) as f64 / (n1 + n2) as f64;

    let se = (p_pooled * (1.0 - p_pooled) * (1.0 / n1 as f64 + 1.0 / n2 as f64)).sqrt();
    let z = (p1 - p2).abs() / se;

    let p_value = if z > 2.576 {
        0.001
    } else if z > 1.96 {
        0.05
    } else if z > 1.645 {
        0.10
    } else {
        0.20
    };

    (z, p_value)
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║   BASE 15 MIDPOINT THRESHOLD TEST: AT BOUNDARY + NON-2×p     ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Base 15 = 3×5 (midpoint m=7.5, NOT form 2×p)                ║");
    println!("║  Testing: Boundary case with p_max=5 < 7                     ║");
    println!("║  Control: Non-2×p factorization pattern                      ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let base = 15u32;
    let samples = 1000usize;

    // Coprime boundary pairs for base 15
    // Valid digits coprime to 15: 1, 2, 4, 7, 8, 11, 13, 14
    let boundary_pairs = vec![(1, 2), (2, 7), (7, 11)];

    println!("🎯 MIDPOINT-7 CHAOS THRESHOLD HYPOTHESIS (BOUNDARY TEST)");
    println!("======================================================================");
    println!();
    println!("Known data:");
    println!("  Base 10 (2×5, m=5, p_max=5):  M=2 shows k*=1 (below threshold)");
    println!("  Base 14 (2×7, m=7, p_max=7):  M=2 shows k*=0 (at threshold)");
    println!();
    println!("Base 15 characteristics:");
    println!("  Factorization: 3×5 (NOT 2×p pattern)");
    println!("  Midpoint: 7.5 (≈7, AT chaos boundary)");
    println!("  Largest prime: 5 (< 7, BELOW threshold)");
    println!();
    println!("Competing predictions:");
    println!("  If m≈7 dominates    → k*=0 (like Base 14)");
    println!("  If p_max<7 matters  → k*>0 possible (like Base 10)");
    println!("  If 2×p required     → No exception (3×5 ≠ 2×p)");
    println!();
    println!("Testing M∈{{1,2,3}} with THREE boundary pairs...");
    println!();

    let start_time = Instant::now();

    let m_values = vec![1, 2, 3];

    for m in &m_values {
        println!("════════════════════════════════════════════════════════════════════");
        println!("M={}: Testing across THREE boundary pairs", m);
        println!("════════════════════════════════════════════════════════════════════");
        println!();

        for (pair_idx, (outer, inner)) in boundary_pairs.iter().enumerate() {
            println!("──────────────────────────────────────────────────────────────");
            println!("PAIR {}: Base 15, M={}, ({},{}) - k∈{{0,1,2}}",
                     pair_idx + 1, m, outer, inner);
            println!("──────────────────────────────────────────────────────────────");
            println!();

            let mut results = Vec::new();

            for k in 0..=2 {
                let result = measure_density(base, *outer, *inner, *m, k, k, samples);
                println!(
                    "k={}: {}/{} = {:.1}%",
                    k,
                    result.primes_found,
                    result.samples,
                    result.density * 100.0
                );
                results.push(result);
            }
            println!();

            let optimal_idx = results
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.density.partial_cmp(&b.density).unwrap())
                .map(|(idx, _)| idx)
                .unwrap();

            let k_star = results[optimal_idx].k_outer;
            println!("→ k* = {} for M={}, ({},{})", k_star, m, outer, inner);

            let k0 = &results[0];
            let k1 = &results[1];
            let delta = (k1.density - k0.density) * 100.0;
            let (z_score, p_value) = proportion_z_test(
                k1.primes_found,
                k1.samples,
                k0.primes_found,
                k0.samples,
            );

            println!("  Δ(k=1 - k=0) = {:+.2}pp, z={:.2}, p≈{:.3}",
                     delta, z_score, p_value);
            println!();
        }
    }

    let elapsed = start_time.elapsed();

    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║            MIDPOINT THRESHOLD HYPOTHESIS EVALUATION           ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    println!("Base 15 (3×5, m=7.5, p_max=5) summary:");
    println!("  M=1, M=2, M=3 tested across 3 boundary pairs");
    println!("  Check k* distribution for M=2 (critical regime)");
    println!();

    println!("🔬 INTERPRETATION GUIDE:");
    println!();
    println!("If M=2 shows k*=0 for majority of pairs:");
    println!("  → Midpoint m≈7 is sufficient (hypothesis SUPPORTED)");
    println!("  → Chaos threshold at m=7 regardless of p_max");
    println!();
    println!("If M=2 shows k*>0 for majority of pairs:");
    println!("  → p_max<7 allows exceptions (multi-factor model)");
    println!("  → Both midpoint AND p_max matter");
    println!();
    println!("If M=2 shows k*=0 but Base 10 showed k*=1:");
    println!("  → 2×p factorization may be necessary for exception");
    println!("  → Base 15 (3×5) vs Base 10 (2×5) factorization matters");
    println!();

    println!("⏱️  Total runtime: {:.2?}", elapsed);
    println!();

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                        CSV OUTPUT                             ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("base,M,outer,inner,k,samples,primes,density,midpoint,p_max,factorization");

    for m in &m_values {
        for (outer, inner) in &boundary_pairs {
            for k in 0..=2 {
                let result = measure_density(base, *outer, *inner, *m, k, k, samples);
                println!(
                    "{},{},{},{},{},{},{},{:.6},{},{},3x5",
                    base, m, outer, inner, k, result.samples, result.primes_found, result.density, 7.5, 5
                );
            }
        }
    }

    println!();
    println!("🏁 Base 15 Boundary Case Test Complete!");
}
