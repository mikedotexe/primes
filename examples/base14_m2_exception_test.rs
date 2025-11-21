// Base 14 M=2 Exception Test
// ============================
//
// Critical experiment to test the "2×p pattern" hypothesis:
//
// HYPOTHESIS: Bases of form 2×p (p prime) show M=2 k*=1 exception
//
// EVIDENCE SO FAR:
// - Base 10 (2×5), M=2: k=1 outperforms k=0 by 5.9pp (p=0.01)
//
// THIS TEST:
// - Base 14 (2×7), M=2: Does k=1 also outperform k=0?
//
// OUTCOMES:
// - If YES → 2×p resonance pattern confirmed
// - If NO  → Base 10 is isolated exception
//
// This single experiment determines theoretical direction for next 6 months.

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

    // Helper to add digit at position
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

/// Test result for a single (k_outer, k_inner) configuration
#[derive(Debug)]
struct TestResult {
    k_outer: u32,
    k_inner: u32,
    samples: usize,
    primes_found: usize,
    density: f64,
}

/// Measure prime density for a given padding configuration
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

    // Random seed range for M-digit numbers in base
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

/// Calculate two-proportion z-test
fn proportion_z_test(x1: usize, n1: usize, x2: usize, n2: usize) -> (f64, f64) {
    let p1 = x1 as f64 / n1 as f64;
    let p2 = x2 as f64 / n2 as f64;
    let p_pooled = (x1 + x2) as f64 / (n1 + n2) as f64;

    let se = (p_pooled * (1.0 - p_pooled) * (1.0 / n1 as f64 + 1.0 / n2 as f64)).sqrt();
    let z = (p1 - p2).abs() / se;

    // Two-tailed p-value (approximate)
    let p_value = if z > 2.576 {
        0.01
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
    println!("║      BASE 14 M=2 EXCEPTION TEST: 2×p PATTERN VERIFICATION    ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Base 14 = 2×7 (testing 2×p hypothesis)                      ║");
    println!("║  M=2 (two-digit middles)                                     ║");
    println!("║  Boundaries: (1,3) coprime to 14                             ║");
    println!("║  Sample size: 1000 per k value                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Configuration
    let base = 14u32;
    let outer = 1u32;
    let inner = 3u32;
    let m = 2usize;
    let samples = 1000usize;

    println!("🔬 TESTING BASE 14, M=2, (1,3) WITH k∈{{0,1,2}}");
    println!("======================================================================");
    println!();

    let start_time = Instant::now();

    // Test k=0, k=1, k=2 (symmetric padding)
    let mut results = Vec::new();

    for k in 0..=2 {
        println!("Testing k={} (n={} samples)...", k, samples);
        let result = measure_density(base, outer, inner, m, k, k, samples);
        println!(
            "  → Primes found: {}/{} ({:.1}%)",
            result.primes_found,
            result.samples,
            result.density * 100.0
        );
        results.push(result);
        println!();
    }

    let elapsed = start_time.elapsed();

    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                         RESULTS                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Display results table
    println!("Base 14, M=2, (1,3) - Density by k:");
    println!("┌───────┬─────────┬──────────┬──────────┐");
    println!("│   k   │ Samples │  Primes  │ Density  │");
    println!("├───────┼─────────┼──────────┼──────────┤");
    for result in &results {
        println!(
            "│  {:2}   │  {:4}   │   {:3}    │  {:.1}%  │",
            result.k_outer,
            result.samples,
            result.primes_found,
            result.density * 100.0
        );
    }
    println!("└───────┴─────────┴──────────┴──────────┘");
    println!();

    // Find optimal k
    let optimal_idx = results
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.density.partial_cmp(&b.density).unwrap())
        .map(|(idx, _)| idx)
        .unwrap();

    let k_star = results[optimal_idx].k_outer;

    println!("🎯 OPTIMAL PADDING: k* = {}", k_star);
    println!();

    // Statistical comparison: k=0 vs k=1
    println!("📊 STATISTICAL ANALYSIS: k=0 vs k=1");
    println!("======================================================================");
    println!();

    let k0 = &results[0];
    let k1 = &results[1];

    let delta = (k1.density - k0.density) * 100.0;
    let (z_score, p_value) = proportion_z_test(
        k1.primes_found,
        k1.samples,
        k0.primes_found,
        k0.samples,
    );

    println!("k=0: {}/{} = {:.1}%", k0.primes_found, k0.samples, k0.density * 100.0);
    println!("k=1: {}/{} = {:.1}%", k1.primes_found, k1.samples, k1.density * 100.0);
    println!();
    println!("Difference (k=1 - k=0): {:.2} percentage points", delta);
    println!("Z-score: {:.3}", z_score);
    println!("p-value: ~{:.2}", p_value);
    println!();

    // Determine significance
    let significant = p_value <= 0.05;
    if significant {
        if delta > 0.0 {
            println!("✅ SIGNIFICANT: k=1 outperforms k=0 (p ≤ 0.05)");
        } else {
            println!("✅ SIGNIFICANT: k=0 outperforms k=1 (p ≤ 0.05)");
        }
    } else {
        println!("❌ NOT SIGNIFICANT: No clear winner (p > 0.05)");
    }
    println!();

    // Compare to Base 10 M=2 results
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║              COMPARISON TO BASE 10 M=2 PATTERN                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    println!("Base 10 M=2 (known exception):");
    println!("  k=0: 17.1%  k=1: 23.0%  Δ=+5.9pp  p=0.01  k*=1 ✓");
    println!();

    println!("Base 14 M=2 (THIS TEST):");
    println!(
        "  k=0: {:.1}%  k=1: {:.1}%  Δ={:+.1}pp  p~{:.2}  k*={}",
        k0.density * 100.0,
        k1.density * 100.0,
        delta,
        p_value,
        k_star
    );
    println!();

    // Hypothesis evaluation
    println!("🧮 HYPOTHESIS EVALUATION: 2×p RESONANCE PATTERN");
    println!("======================================================================");
    println!();

    if k_star == 1 && significant && delta > 3.0 {
        println!("✅✅✅ HYPOTHESIS CONFIRMED! ✅✅✅");
        println!();
        println!("Both Base 10 (2×5) and Base 14 (2×7) show k*=1 for M=2.");
        println!("This suggests a SYSTEMATIC 2×p RESONANCE PATTERN.");
        println!();
        println!("🎯 THEORETICAL IMPLICATIONS:");
        println!("  → Bases of form 2×p (p prime) exhibit special M=2 behavior");
        println!("  → Hardy-Littlewood singular series analysis warranted");
        println!("  → Test additional 2×p bases: 22 (2×11), 26 (2×13), 34 (2×17)");
    } else if k_star == 0 {
        println!("❌ HYPOTHESIS REFUTED");
        println!();
        println!("Base 14 (2×7) shows k*=0, unlike Base 10 (2×5) with k*=1.");
        println!("Base 10 M=2 is an ISOLATED EXCEPTION, not a 2×p pattern.");
        println!();
        println!("🎯 THEORETICAL IMPLICATIONS:");
        println!("  → Base 10 has unique decimal properties causing M=2 k*=1");
        println!("  → Not generalizable to other 2×p bases");
        println!("  → Focus on Base-10-specific mechanism (mod-10 residue classes)");
    } else {
        println!("⚠️  INCONCLUSIVE");
        println!();
        println!("Base 14 shows k*={} (not k=0 or k=1 decisively).", k_star);
        println!("May require higher sample size for definitive answer.");
    }
    println!();

    // CSV output
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                        CSV OUTPUT                             ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("base,M,outer,inner,k,samples,primes,density");
    for result in &results {
        println!(
            "{},{},{},{},{},{},{},{:.6}",
            base,
            m,
            outer,
            inner,
            result.k_outer,
            result.samples,
            result.primes_found,
            result.density
        );
    }
    println!();

    println!("⏱️  Total runtime: {:.2?}", elapsed);
    println!();
    println!("🏁 Base 14 M=2 Exception Test Complete!");
}
