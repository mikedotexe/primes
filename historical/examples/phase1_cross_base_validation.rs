//! Phase 1: Cross-Base Validation of k* for M∈{2,3,4}
//!
//! Tests the hypothesis that k*≈0 is universal across bases,
//! not just a Base-6-specific phenomenon.
//!
//! Competing hypotheses:
//! A) k*≈0 universally (minimal padding principle)
//! B) k* scales with M but below detection threshold
//! C) Phase transition at M=1 (k*>0 only for M=1)

use num_bigint::BigUint;
use primes::{is_prime, MembraneConfig};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct PhaseOneConfig {
    base: u32,
    outer: u32,
    inner: u32,
    M: usize,          // Middle length
    k_total: u32,      // Total padding (k_outer + k_inner, symmetric assumed)
}

#[derive(Debug, Clone)]
struct MeasurementResult {
    config: PhaseOneConfig,
    samples_tested: usize,
    primes_found: usize,
    density: f64,
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║     PHASE 1: CROSS-BASE VALIDATION OF k*(M) HYPOTHESIS       ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Testing: Is k*≈0 universal or base-6-specific?              ║");
    println!("║  Range: M∈{{2,3,4}}, k∈{{0,1,2,3,4,5}}, bases={{6,10,14,18,30}}  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Configuration
    let bases = vec![6, 10, 14, 18, 30];
    let M_values = vec![2, 3, 4];
    let k_max = 5;
    let samples_per_config = 100; // Statistical validity

    // Storage for all measurements
    let mut all_results: Vec<MeasurementResult> = Vec::new();

    // CSV output
    println!("base,M,outer,inner,k_total,samples,primes,density");

    for base in &bases {
        println!("\n🔬 Testing base {}...", base);

        // Get coprime boundary pairs for this base
        let boundary_pairs = get_coprime_boundary_pairs(*base);

        for (outer, inner) in &boundary_pairs {
            for M in &M_values {
                for k_total in 0..=k_max {
                    // Symmetric padding: k_outer = k_inner = k_total/2
                    let k_outer = k_total / 2;
                    let k_inner = k_total - k_outer;

                    let config = PhaseOneConfig {
                        base: *base,
                        outer: *outer,
                        inner: *inner,
                        M: *M,
                        k_total,
                    };

                    let result = measure_density(&config, k_outer, k_inner, samples_per_config);

                    // Output CSV line
                    println!(
                        "{},{},{},{},{},{},{},{:.6}",
                        base, M, outer, inner, k_total,
                        result.samples_tested,
                        result.primes_found,
                        result.density
                    );

                    all_results.push(result);
                }
            }
        }
    }

    // Statistical Analysis
    println!("\n\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                  STATISTICAL ANALYSIS                         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    analyze_k_star_distribution(&all_results);
}

fn get_coprime_boundary_pairs(base: u32) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();

    // For Phase 1, test top 3 coprime pairs to balance thoroughness and runtime
    let mut candidates = Vec::new();
    for outer in 1..base {
        if gcd(outer, base) == 1 {
            for inner in 1..base {
                if gcd(inner, base) == 1 {
                    candidates.push((outer, inner));
                }
            }
        }
    }

    // Prioritize known good patterns: (1, base-1), small values, etc.
    candidates.sort_by_key(|(o, i)| o + i); // Prefer small sum (like 1,5 in base 6)

    // Take top 3 pairs for efficiency
    pairs.extend(candidates.iter().take(3));

    if pairs.is_empty() {
        eprintln!("⚠️  Warning: No coprime pairs found for base {}", base);
    }

    pairs
}

fn measure_density(
    config: &PhaseOneConfig,
    k_outer: u32,
    k_inner: u32,
    samples: usize,
) -> MeasurementResult {
    let mut primes_found = 0;
    let mut rng = rand::thread_rng();

    // Generate random M-digit seeds
    let seed_min = config.base.pow((config.M - 1) as u32); // Minimum M-digit number
    let seed_max = config.base.pow(config.M as u32);       // Maximum M-digit number

    for _ in 0..samples {
        // Random seed in range [seed_min, seed_max)
        let seed = rng.gen_range(seed_min..seed_max);

        // Build membrane
        if let Some(membrane_value) = construct_membrane(config, k_outer, k_inner, seed) {
            if is_prime(&membrane_value) {
                primes_found += 1;
            }
        }
    }

    let density = primes_found as f64 / samples as f64;

    MeasurementResult {
        config: config.clone(),
        samples_tested: samples,
        primes_found,
        density,
    }
}

fn construct_membrane(
    config: &PhaseOneConfig,
    k_outer: u32,
    k_inner: u32,
    seed: u32,
) -> Option<BigUint> {
    let base = BigUint::from(config.base);
    let mut value = BigUint::from(0u32);
    let mut position = 0usize;

    // Membrane structure: outer [k_outer×0] inner [k_inner×0] SEED [k_inner×0] inner [k_outer×0] outer

    // Right outer
    value += BigUint::from(config.outer) * base.pow(position as u32);
    position += 1;

    // Right k_outer zeros
    position += k_outer as usize;

    // Right inner
    value += BigUint::from(config.inner) * base.pow(position as u32);
    position += 1;

    // Right k_inner zeros
    position += k_inner as usize;

    // Middle (M-digit seed)
    value += BigUint::from(seed) * base.pow(position as u32);
    position += config.M;

    // Left k_inner zeros
    position += k_inner as usize;

    // Left inner
    value += BigUint::from(config.inner) * base.pow(position as u32);
    position += 1;

    // Left k_outer zeros
    position += k_outer as usize;

    // Left outer
    value += BigUint::from(config.outer) * base.pow(position as u32);

    Some(value)
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn analyze_k_star_distribution(results: &[MeasurementResult]) {
    // Group by (base, M) and find k* (optimal k_total)
    let mut k_star_map: HashMap<(u32, usize), Vec<(u32, f64)>> = HashMap::new();

    for result in results {
        let key = (result.config.base, result.config.M);
        k_star_map
            .entry(key)
            .or_insert_with(Vec::new)
            .push((result.config.k_total, result.density));
    }

    println!("📊 OPTIMAL k* BY (BASE, M):");
    println!("{}", "=".repeat(60));
    println!("Base | M | k*_optimal | max_density | All k densities");
    println!("-----|---|------------|-------------|------------------");

    let mut k_star_values = Vec::new();
    let mut k_star_summary: HashMap<usize, Vec<u32>> = HashMap::new();

    for ((base, M), densities) in k_star_map.iter() {
        // Find k with maximum density
        let (k_star, max_density) = densities
            .iter()
            .max_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
            .unwrap();

        k_star_values.push(*k_star);
        k_star_summary.entry(*M).or_insert_with(Vec::new).push(*k_star);

        // Format density distribution
        let density_str = densities
            .iter()
            .map(|(k, d)| format!("{}:{:.3}", k, d))
            .collect::<Vec<_>>()
            .join(" ");

        println!(
            " {:3} | {} |     {:2}     |   {:.4}    | {}",
            base, M, k_star, max_density, density_str
        );
    }

    // Summary statistics
    println!("\n\n📈 K* SUMMARY STATISTICS BY M:");
    println!("{}", "=".repeat(60));

    for M in [2, 3, 4] {
        if let Some(k_stars) = k_star_summary.get(&M) {
            let mean = k_stars.iter().sum::<u32>() as f64 / k_stars.len() as f64;
            let median = {
                let mut sorted = k_stars.clone();
                sorted.sort();
                sorted[sorted.len() / 2]
            };
            let mode = mode(k_stars);
            let all_zero = k_stars.iter().all(|&k| k == 0);

            println!("\nM = {}:", M);
            println!("  Mean k*:   {:.2}", mean);
            println!("  Median k*: {}", median);
            println!("  Mode k*:   {}", mode);
            println!("  All k*=0?: {}", if all_zero { "✅ YES" } else { "❌ NO" });
            println!("  k* values: {:?}", k_stars);
        }
    }

    // Hypothesis testing
    println!("\n\n🎯 HYPOTHESIS TEST RESULTS:");
    println!("{}", "=".repeat(60));

    let all_k_star_zero = k_star_values.iter().all(|&k| k == 0);
    let mostly_k_star_zero = k_star_values.iter().filter(|&&k| k == 0).count() as f64
        / k_star_values.len() as f64;

    println!("Total (base,M) configurations tested: {}", k_star_values.len());
    println!("Configurations with k*=0: {}", k_star_values.iter().filter(|&&k| k == 0).count());
    println!("Percentage k*=0: {:.1}%\n", mostly_k_star_zero * 100.0);

    if all_k_star_zero {
        println!("✅ HYPOTHESIS A STRONGLY SUPPORTED:");
        println!("   k*≈0 universally across all tested (base,M) pairs");
        println!("   Evidence for Minimal Padding Principle");
    } else if mostly_k_star_zero > 0.8 {
        println!("✅ HYPOTHESIS A LIKELY:");
        println!("   k*≈0 in {:.0}% of cases", mostly_k_star_zero * 100.0);
        println!("   Outliers may be due to statistical noise or special cases");
    } else {
        println!("⚠️  HYPOTHESIS A WEAK:");
        println!("   Only {:.0}% show k*=0", mostly_k_star_zero * 100.0);
        println!("   May need to test Hypothesis B (scaling below threshold)");
    }

    println!("\n💡 NEXT STEPS:");
    if all_k_star_zero || mostly_k_star_zero > 0.8 {
        println!("   → Proceed to Phase 2: Test larger M∈{{5..10}} to confirm k* remains 0");
        println!("   → Update COLLABORATION.md with Phase 1 confirmation");
        println!("   → Consider theoretical proof of minimal padding principle");
    } else {
        println!("   → Investigate non-zero k* cases for patterns");
        println!("   → Expand M range immediately to M∈{{5,6,7,8}} to detect scaling");
        println!("   → Consider continuous k optimization");
    }
}

fn mode(values: &[u32]) -> u32 {
    let mut counts = HashMap::new();
    for &v in values {
        *counts.entry(v).or_insert(0) += 1;
    }
    *counts.iter().max_by_key(|(_, &count)| count).unwrap().0
}
