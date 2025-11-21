// Base 12 Midpoint Threshold Test
// ================================
//
// CRITICAL EXPERIMENT: Testing the Midpoint-7 Chaos Threshold Hypothesis
//
// HYPOTHESIS: Bases with midpoint m ≥ 7 show universal k*=0
//
// BASE 12 TEST:
// - Base: 12 = 2²×3
// - Midpoint: 6 (BELOW threshold 7)
// - Largest prime factor: 3
// - Prediction: May show k*>0 exception like Base 10 (m=5)
//
// COMPARISON:
// - Base 10 (m=5): Shows k*=1 for M=2 ✓ (exception, below threshold)
// - Base 12 (m=6): ? (TESTING NOW, just below threshold)
// - Base 14 (m=7): Shows k*=0 for M=2 ✓ (at/above threshold)
//
// If Base 12 shows k*>0 for M=2 → midpoint<7 allows exceptions
// If Base 12 shows k*=0 for M=2 → Base 10 is uniquely special

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use rand::Rng;
use std::time::Instant;

/// Construct a symmetric membrane prime candidate
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

    // Structure: outer [k_outer×0] inner [k_inner×0] SEED [k_inner×0] inner [k_outer×0] outer

    // Left side
    add_digit(outer);
    for _ in 0..k_outer {
        add_digit(0);
    }
    add_digit(inner);
    for _ in 0..k_inner {
        add_digit(0);
    }

    // Middle (seed in base representation)
    let mut seed_val = seed;
    for _ in 0..m {
        add_digit((seed_val % base as u64) as u32);
        seed_val /= base as u64;
    }

    // Right side (mirror)
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
    println!("║     BASE 12 MIDPOINT THRESHOLD TEST: BELOW CHAOS BOUNDARY    ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Base 12 = 2²×3 (midpoint m=6)                               ║");
    println!("║  Testing: Does m<7 allow k*>0 exceptions like Base 10?      ║");
    println!("║  Critical M=2 regime                                         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let base = 12u32;
    let m = 2usize;  // Critical test regime
    let samples = 1000usize;

    // Coprime boundary pairs for base 12
    // Valid digits: 1, 5, 7, 11 (coprime to 12)
    let boundary_pairs = vec![(1, 5), (5, 7), (7, 11)];

    println!("🎯 MIDPOINT-7 CHAOS THRESHOLD HYPOTHESIS");
    println!("======================================================================");
    println!();
    println!("Known data:");
    println!("  Base 10 (m=5):  M=2 shows k*=1 (+5.9pp advantage)");
    println!("  Base 14 (m=7):  M=2 shows k*=0 (-6.2pp for k=1)");
    println!();
    println!("Base 12 prediction:");
    println!("  If m<7 allows exceptions → Base 12 may show k*>0");
    println!("  If Base 10 is unique    → Base 12 shows k*=0");
    println!();
    println!("Testing with THREE coprime boundary pairs for robustness...");
    println!();

    let start_time = Instant::now();

    for (pair_idx, (outer, inner)) in boundary_pairs.iter().enumerate() {
        println!("════════════════════════════════════════════════════════════════════");
        println!("PAIR {}: Base 12, M=2, ({},{}) - Testing k∈{{0,1,2}}",
                 pair_idx + 1, outer, inner);
        println!("════════════════════════════════════════════════════════════════════");
        println!();

        let mut results = Vec::new();

        for k in 0..=2 {
            println!("Testing k={} (n={} samples)...", k, samples);
            let result = measure_density(base, *outer, *inner, m, k, k, samples);
            println!(
                "  → Primes found: {}/{} ({:.1}%)",
                result.primes_found,
                result.samples,
                result.density * 100.0
            );
            results.push(result);
            println!();
        }

        // Find optimal k for this pair
        let optimal_idx = results
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.density.partial_cmp(&b.density).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        let k_star = results[optimal_idx].k_outer;

        println!("📊 RESULTS FOR PAIR ({},{}):", outer, inner);
        println!("┌───────┬─────────┬──────────┬──────────┐");
        println!("│   k   │ Samples │  Primes  │ Density  │");
        println!("├───────┼─────────┼──────────┼──────────┤");
        for result in &results {
            let marker = if result.k_outer == k_star { "★" } else { " " };
            println!(
                "│  {:2}   │  {:4}   │   {:3}    │  {:.1}% {}│",
                result.k_outer,
                result.samples,
                result.primes_found,
                result.density * 100.0,
                marker
            );
        }
        println!("└───────┴─────────┴──────────┴──────────┘");
        println!();
        println!("🎯 Optimal padding for ({},{}): k* = {}", outer, inner, k_star);
        println!();

        // Statistical test: k=0 vs k=1
        let k0 = &results[0];
        let k1 = &results[1];
        let delta = (k1.density - k0.density) * 100.0;
        let (z_score, p_value) = proportion_z_test(
            k1.primes_found,
            k1.samples,
            k0.primes_found,
            k0.samples,
        );

        println!("📈 Statistical Analysis (k=0 vs k=1):");
        println!("  Δ = {:+.2} percentage points", delta);
        println!("  Z-score = {:.3}", z_score);
        println!("  p-value ≈ {:.3}", p_value);

        if p_value <= 0.05 {
            if delta > 0.0 {
                println!("  ✅ SIGNIFICANT: k=1 outperforms k=0");
            } else {
                println!("  ✅ SIGNIFICANT: k=0 outperforms k=1");
            }
        } else {
            println!("  ❌ NOT SIGNIFICANT: No clear winner");
        }
        println!();
    }

    let elapsed = start_time.elapsed();

    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║            MIDPOINT THRESHOLD HYPOTHESIS EVALUATION           ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    println!("Base 12 (m=6) M=2 results across 3 boundary pairs:");
    println!("  (Check if majority show k*>0 or k*=0)");
    println!();

    println!("🔬 INTERPRETATION GUIDE:");
    println!();
    println!("If k*>0 for majority of pairs:");
    println!("  → Midpoint m<7 allows exceptions (hypothesis SUPPORTED)");
    println!("  → Base 10 (m=5) and Base 12 (m=6) both show k*>0");
    println!("  → Threshold at m=7 is REAL");
    println!();
    println!("If k*=0 for majority of pairs:");
    println!("  → Base 10 is uniquely special (hypothesis REFUTED)");
    println!("  → Midpoint is NOT the determining factor");
    println!("  → Focus on Base-10-specific mechanisms");
    println!();

    println!("⏱️  Total runtime: {:.2?}", elapsed);
    println!();

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                        CSV OUTPUT                             ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("base,M,outer,inner,k,samples,primes,density,midpoint");

    // Re-run to get CSV (simple approach)
    for (outer, inner) in &boundary_pairs {
        for k in 0..=2 {
            let result = measure_density(base, *outer, *inner, m, k, k, samples);
            println!(
                "{},{},{},{},{},{},{},{:.6},{}",
                base, m, outer, inner, k, result.samples, result.primes_found, result.density, 6
            );
        }
    }

    println!();
    println!("🏁 Base 12 Midpoint Threshold Test Complete!");
}
