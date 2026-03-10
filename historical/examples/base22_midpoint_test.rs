// Base 22 Midpoint Threshold Test
// ================================
//
// CRITICAL EXPERIMENT: Testing Deep-Chaos Regime (m >> 7)
//
// HYPOTHESIS: Bases with midpoint m ≥ 7 show universal k*=0
//
// BASE 22 TEST:
// - Base: 22 = 2×11
// - Midpoint: 11 (WELL ABOVE threshold 7)
// - Largest prime factor: 11
// - Prediction: Universal k*=0 due to computational complexity
//
// COMPARISON:
// - Base 10 (m=5):  M=2 shows k*=1 (exception, below threshold)
// - Base 14 (m=7):  M=2 shows k*=0 (at threshold)
// - Base 22 (m=11): ? (TESTING NOW, deep chaos regime)
//
// If Base 22 shows k*=0 → midpoint≥7 forces universal minimal padding
// If Base 22 shows k*>0 → hypothesis REFUTED, pattern more complex

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
    println!("║  BASE 22 MIDPOINT THRESHOLD TEST: DEEP CHAOS REGIME (m=11)   ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Base 22 = 2×11 (midpoint m=11 >> 7)                         ║");
    println!("║  Testing: Does high midpoint force universal k*=0?           ║");
    println!("║  M∈{{1,2,3}} comprehensive test                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let base = 22u32;
    let samples = 1000usize;

    // Coprime boundary pair for base 22
    // Valid digits coprime to 22: 1, 3, 5, 7, 9, 13, 15, 17, 19, 21
    let outer = 1u32;
    let inner = 3u32;

    println!("🎯 MIDPOINT-7 CHAOS THRESHOLD HYPOTHESIS (DEEP CHAOS TEST)");
    println!("======================================================================");
    println!();
    println!("Known data:");
    println!("  Base 10 (m=5):  M=2 shows k*=1 (below threshold)");
    println!("  Base 14 (m=7):  M=2 shows k*=0 (at threshold)");
    println!();
    println!("Base 22 prediction:");
    println!("  If m≥7 → universal k*=0 → Base 22 (m=11) shows k*=0 for ALL M");
    println!("  If not → hypothesis REFUTED");
    println!();
    println!("Testing M∈{{1,2,3}} to verify across asymptotic regimes...");
    println!();

    let start_time = Instant::now();

    let m_values = vec![1, 2, 3];

    for m in &m_values {
        println!("════════════════════════════════════════════════════════════════════");
        println!("M={}: Base 22 ({},{}) - Testing k∈{{0,1,2}}", m, outer, inner);
        println!("════════════════════════════════════════════════════════════════════");
        println!();

        let mut results = Vec::new();

        for k in 0..=2 {
            println!("Testing M={}, k={} (n={} samples)...", m, k, samples);
            let result = measure_density(base, outer, inner, *m, k, k, samples);
            println!(
                "  → Primes found: {}/{} ({:.1}%)",
                result.primes_found,
                result.samples,
                result.density * 100.0
            );
            results.push(result);
            println!();
        }

        let optimal_idx = results
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.density.partial_cmp(&b.density).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        let k_star = results[optimal_idx].k_outer;

        println!("📊 RESULTS FOR M={}:", m);
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
        println!("🎯 Optimal padding for M={}: k* = {}", m, k_star);
        println!();

        // Statistical test
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

    println!("Base 22 (m=11) results across M∈{{1,2,3}}:");
    println!("  M=1: k* = ?");
    println!("  M=2: k* = ? (critical test regime)");
    println!("  M=3: k* = ? (asymptotic regime)");
    println!();

    println!("🔬 INTERPRETATION GUIDE:");
    println!();
    println!("If k*=0 for ALL M (especially M=2):");
    println!("  → Deep chaos regime (m=11 >> 7) forces k*=0");
    println!("  → Midpoint threshold hypothesis SUPPORTED");
    println!("  → Computational complexity exceeds optimization tractability");
    println!();
    println!("If k*>0 for any M:");
    println!("  → High midpoint does NOT guarantee k*=0");
    println!("  → Hypothesis REFUTED");
    println!("  → Other factors (factorization, M value) dominate");
    println!();

    println!("⏱️  Total runtime: {:.2?}", elapsed);
    println!();

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                        CSV OUTPUT                             ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("base,M,outer,inner,k,samples,primes,density,midpoint,p_max");

    for m in &m_values {
        for k in 0..=2 {
            let result = measure_density(base, outer, inner, *m, k, k, samples);
            println!(
                "{},{},{},{},{},{},{},{:.6},{},{}",
                base, m, outer, inner, k, result.samples, result.primes_found, result.density, 11, 11
            );
        }
    }

    println!();
    println!("🏁 Base 22 Deep Chaos Regime Test Complete!");
}
