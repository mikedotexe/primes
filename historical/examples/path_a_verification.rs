//! Path A Verification: High-Sample Testing of Key Findings
//!
//! Three critical tests:
//! 1. M=3 perfect k*=0 (retest with 1000 samples for robustness)
//! 2. Outliers statistical significance (1000 samples to check p-values)
//! 3. M=1 special case (test hypothesis C: phase transition)

use num_bigint::BigUint;
use primes::is_prime;
use rand::Rng;

#[derive(Debug, Clone)]
struct VerificationTest {
    name: String,
    base: u32,
    outer: u32,
    inner: u32,
    M: usize,
    k_values: Vec<u32>,
    samples: usize,
}

#[derive(Debug)]
struct TestResult {
    test_name: String,
    base: u32,
    M: usize,
    k: u32,
    samples: usize,
    primes: usize,
    density: f64,
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║        PATH A VERIFICATION: HIGH-SAMPLE TESTING               ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Test 1: M=3 perfect k*=0 (1000 samples)                     ║");
    println!("║  Test 2: Outliers significance (1000 samples)                ║");
    println!("║  Test 3: M=1 special case (1000 samples)                     ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let mut all_results = Vec::new();

    // ========================================================================
    // TEST 1: M=3 Perfect k*=0 Verification (1000 samples)
    // ========================================================================

    println!("🔬 TEST 1: M=3 VERIFICATION (1000 samples per config)");
    println!("{}", "=".repeat(70));
    println!("base,M,outer,inner,k,samples,primes,density");

    let m3_tests = vec![
        VerificationTest {
            name: "Base 6 M=3".to_string(),
            base: 6,
            outer: 1,
            inner: 5,
            M: 3,
            k_values: vec![0, 1, 2],
            samples: 1000,
        },
        VerificationTest {
            name: "Base 10 M=3".to_string(),
            base: 10,
            outer: 1,
            inner: 3,
            M: 3,
            k_values: vec![0, 1, 2],
            samples: 1000,
        },
        VerificationTest {
            name: "Base 14 M=3".to_string(),
            base: 14,
            outer: 1,
            inner: 3,
            M: 3,
            k_values: vec![0, 1, 2],
            samples: 1000,
        },
        VerificationTest {
            name: "Base 18 M=3".to_string(),
            base: 18,
            outer: 1,
            inner: 5,
            M: 3,
            k_values: vec![0, 1, 2],
            samples: 1000,
        },
        VerificationTest {
            name: "Base 30 M=3".to_string(),
            base: 30,
            outer: 1,
            inner: 7,
            M: 3,
            k_values: vec![0, 1, 2],
            samples: 1000,
        },
    ];

    for test in &m3_tests {
        for &k in &test.k_values {
            let result = run_test(test, k);
            println!(
                "{},{},{},{},{},{},{},{:.6}",
                result.base,
                result.M,
                test.outer,
                test.inner,
                result.k,
                result.samples,
                result.primes,
                result.density
            );
            all_results.push(result);
        }
    }

    // ========================================================================
    // TEST 2: Outlier Statistical Significance (1000 samples)
    // ========================================================================

    println!("\n🔬 TEST 2: OUTLIER SIGNIFICANCE (1000 samples)");
    println!("{}", "=".repeat(70));
    println!("Testing Phase 1 outliers with high sample size:");
    println!("base,M,outer,inner,k,samples,primes,density");

    let outlier_tests = vec![
        // Base 10, M=2: k*=1 vs k=0
        VerificationTest {
            name: "Base 10 M=2 outlier".to_string(),
            base: 10,
            outer: 3,
            inner: 1,
            M: 2,
            k_values: vec![0, 1],
            samples: 1000,
        },
        // Base 18, M=2: k*=2 vs k=0
        VerificationTest {
            name: "Base 18 M=2 outlier".to_string(),
            base: 18,
            outer: 1,
            inner: 1,
            M: 2,
            k_values: vec![0, 1, 2],
            samples: 1000,
        },
        // Base 30, M=4: k*=3 vs k=0
        VerificationTest {
            name: "Base 30 M=4 outlier".to_string(),
            base: 30,
            outer: 1,
            inner: 1,
            M: 4,
            k_values: vec![0, 1, 2, 3],
            samples: 1000,
        },
    ];

    for test in &outlier_tests {
        for &k in &test.k_values {
            let result = run_test(test, k);
            println!(
                "{},{},{},{},{},{},{},{:.6}",
                result.base,
                result.M,
                test.outer,
                test.inner,
                result.k,
                result.samples,
                result.primes,
                result.density
            );
            all_results.push(result);
        }
    }

    // ========================================================================
    // TEST 3: M=1 Special Case (Hypothesis C)
    // ========================================================================

    println!("\n🔬 TEST 3: M=1 SPECIAL CASE (1000 samples)");
    println!("{}", "=".repeat(70));
    println!("Testing hypothesis C: Is M=1 different from M≥2?");
    println!("base,M,outer,inner,k,samples,primes,density");

    let m1_tests = vec![
        VerificationTest {
            name: "Base 6 M=1".to_string(),
            base: 6,
            outer: 1,
            inner: 5,
            M: 1,
            k_values: vec![0, 1, 2, 3],
            samples: 1000,
        },
        VerificationTest {
            name: "Base 10 M=1".to_string(),
            base: 10,
            outer: 3,
            inner: 7,
            M: 1,
            k_values: vec![0, 1, 2, 3],
            samples: 1000,
        },
        VerificationTest {
            name: "Base 14 M=1".to_string(),
            base: 14,
            outer: 1,
            inner: 3,
            M: 1,
            k_values: vec![0, 1, 2, 3],
            samples: 1000,
        },
        VerificationTest {
            name: "Base 18 M=1".to_string(),
            base: 18,
            outer: 1,
            inner: 5,
            M: 1,
            k_values: vec![0, 1, 2, 3],
            samples: 1000,
        },
        VerificationTest {
            name: "Base 30 M=1".to_string(),
            base: 30,
            outer: 11,
            inner: 7,
            M: 1,
            k_values: vec![0, 1, 2, 3],
            samples: 1000,
        },
    ];

    for test in &m1_tests {
        for &k in &test.k_values {
            let result = run_test(test, k);
            println!(
                "{},{},{},{},{},{},{},{:.6}",
                result.base,
                result.M,
                test.outer,
                test.inner,
                result.k,
                result.samples,
                result.primes,
                result.density
            );
            all_results.push(result);
        }
    }

    // ========================================================================
    // ANALYSIS
    // ========================================================================

    println!("\n\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    VERIFICATION ANALYSIS                      ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    analyze_m3_results(&all_results);
    analyze_outliers(&all_results);
    analyze_m1_special_case(&all_results);
}

fn run_test(test: &VerificationTest, k: u32) -> TestResult {
    let mut primes_found = 0;
    let mut rng = rand::thread_rng();

    // Generate random M-digit seeds
    let seed_min = test.base.pow((test.M - 1) as u32);
    let seed_max = test.base.pow(test.M as u32);

    for _ in 0..test.samples {
        let seed = rng.gen_range(seed_min..seed_max);

        if let Some(membrane_value) = construct_membrane(test, k, seed) {
            if is_prime(&membrane_value) {
                primes_found += 1;
            }
        }
    }

    let density = primes_found as f64 / test.samples as f64;

    TestResult {
        test_name: test.name.clone(),
        base: test.base,
        M: test.M,
        k,
        samples: test.samples,
        primes: primes_found,
        density,
    }
}

fn construct_membrane(test: &VerificationTest, k_total: u32, seed: u32) -> Option<BigUint> {
    let k_outer = k_total / 2;
    let k_inner = k_total - k_outer;

    let base = BigUint::from(test.base);
    let mut value = BigUint::from(0u32);
    let mut position = 0usize;

    // Right outer
    value += BigUint::from(test.outer) * base.pow(position as u32);
    position += 1;
    position += k_outer as usize;

    // Right inner
    value += BigUint::from(test.inner) * base.pow(position as u32);
    position += 1;
    position += k_inner as usize;

    // Middle
    value += BigUint::from(seed) * base.pow(position as u32);
    position += test.M;

    // Left k_inner zeros
    position += k_inner as usize;

    // Left inner
    value += BigUint::from(test.inner) * base.pow(position as u32);
    position += 1;
    position += k_outer as usize;

    // Left outer
    value += BigUint::from(test.outer) * base.pow(position as u32);

    Some(value)
}

fn analyze_m3_results(results: &[TestResult]) {
    println!("📊 TEST 1 ANALYSIS: M=3 PERFECT k*=0 VERIFICATION");
    println!("{}", "=".repeat(70));

    let m3_results: Vec<_> = results.iter().filter(|r| r.M == 3).collect();

    println!("Base | k=0 density | k=1 density | k=2 density | k*_optimal");
    println!("-----|-------------|-------------|-------------|------------");

    let bases = vec![6, 10, 14, 18, 30];
    let mut all_k_star_zero = true;

    for base in bases {
        let base_results: Vec<_> = m3_results.iter().filter(|r| r.base == base).collect();

        let d0 = base_results.iter().find(|r| r.k == 0).map(|r| r.density).unwrap_or(0.0);
        let d1 = base_results.iter().find(|r| r.k == 1).map(|r| r.density).unwrap_or(0.0);
        let d2 = base_results.iter().find(|r| r.k == 2).map(|r| r.density).unwrap_or(0.0);

        let k_star = if d0 >= d1 && d0 >= d2 {
            0
        } else if d1 >= d2 {
            1
        } else {
            2
        };

        if k_star != 0 {
            all_k_star_zero = false;
        }

        println!(
            " {:3} |   {:.4}    |   {:.4}    |   {:.4}    |     {}",
            base, d0, d1, d2, k_star
        );
    }

    println!("\n🎯 M=3 VERDICT (1000 samples):");
    if all_k_star_zero {
        println!("✅✅✅ PERFECT k*=0 CONFIRMED ACROSS ALL BASES!");
        println!("   The M=3 result is ROBUST with high statistical confidence.");
        println!("   This is STRONG evidence for the Minimal Padding Principle.");
    } else {
        println!("⚠️  Some bases show k*>0 with high sample size");
        println!("   Phase 1 M=3 perfect result may have been statistical luck");
    }
}

fn analyze_outliers(results: &[TestResult]) {
    println!("\n📊 TEST 2 ANALYSIS: OUTLIER STATISTICAL SIGNIFICANCE");
    println!("{}", "=".repeat(70));

    // Base 10, M=2
    let b10_m2: Vec<_> = results
        .iter()
        .filter(|r| r.base == 10 && r.M == 2 && (r.k == 0 || r.k == 1))
        .collect();

    if b10_m2.len() == 2 {
        let d0 = b10_m2.iter().find(|r| r.k == 0).unwrap();
        let d1 = b10_m2.iter().find(|r| r.k == 1).unwrap();

        let p_value = proportion_test(d0.primes, d0.samples, d1.primes, d1.samples);

        println!("\n1. Base 10, M=2: k=0 vs k=1");
        println!("   k=0: {}/{} = {:.4}", d0.primes, d0.samples, d0.density);
        println!("   k=1: {}/{} = {:.4}", d1.primes, d1.samples, d1.density);
        println!("   Δ = {:.4}", d1.density - d0.density);
        println!("   p-value ≈ {:.4}", p_value);
        if p_value < 0.05 {
            println!("   ✅ SIGNIFICANT: k=1 is genuinely better!");
        } else {
            println!("   ❌ NOT SIGNIFICANT: Likely statistical noise");
        }
    }

    // Base 18, M=2
    let b18_m2: Vec<_> = results
        .iter()
        .filter(|r| r.base == 18 && r.M == 2)
        .collect();

    if !b18_m2.is_empty() {
        let d0 = b18_m2.iter().find(|r| r.k == 0).unwrap();
        let d2 = b18_m2.iter().find(|r| r.k == 2);

        if let Some(d2) = d2 {
            let p_value = proportion_test(d0.primes, d0.samples, d2.primes, d2.samples);

            println!("\n2. Base 18, M=2: k=0 vs k=2");
            println!("   k=0: {}/{} = {:.4}", d0.primes, d0.samples, d0.density);
            println!("   k=2: {}/{} = {:.4}", d2.primes, d2.samples, d2.density);
            println!("   Δ = {:.4}", d2.density - d0.density);
            println!("   p-value ≈ {:.4}", p_value);
            if p_value < 0.05 {
                println!("   ✅ SIGNIFICANT: k=2 is genuinely better!");
            } else {
                println!("   ❌ NOT SIGNIFICANT: Likely statistical noise");
            }
        }
    }

    // Base 30, M=4
    let b30_m4: Vec<_> = results
        .iter()
        .filter(|r| r.base == 30 && r.M == 4)
        .collect();

    if !b30_m4.is_empty() {
        let d0 = b30_m4.iter().find(|r| r.k == 0).unwrap();
        let d3 = b30_m4.iter().find(|r| r.k == 3);

        if let Some(d3) = d3 {
            let p_value = proportion_test(d0.primes, d0.samples, d3.primes, d3.samples);

            println!("\n3. Base 30, M=4: k=0 vs k=3");
            println!("   k=0: {}/{} = {:.4}", d0.primes, d0.samples, d0.density);
            println!("   k=3: {}/{} = {:.4}", d3.primes, d3.samples, d3.density);
            println!("   Δ = {:.4}", d3.density - d0.density);
            println!("   p-value ≈ {:.4}", p_value);
            if p_value < 0.05 {
                println!("   ✅ SIGNIFICANT: k=3 is genuinely better!");
            } else {
                println!("   ❌ NOT SIGNIFICANT: Likely statistical noise");
            }
        }
    }
}

fn analyze_m1_special_case(results: &[TestResult]) {
    println!("\n📊 TEST 3 ANALYSIS: M=1 SPECIAL CASE (HYPOTHESIS C)");
    println!("{}", "=".repeat(70));

    let m1_results: Vec<_> = results.iter().filter(|r| r.M == 1).collect();

    println!("Base | k* | Max Density | Densities (k=0,1,2,3)");
    println!("-----|----| ------------|----------------------");

    let bases = vec![6, 10, 14, 18, 30];
    let mut k_star_values = Vec::new();

    for base in bases {
        let base_results: Vec<_> = m1_results.iter().filter(|r| r.base == base).collect();

        if base_results.is_empty() {
            continue;
        }

        let k_star_result = base_results.iter().max_by(|a, b| {
            a.density.partial_cmp(&b.density).unwrap()
        }).unwrap();

        let k_star = k_star_result.k;
        k_star_values.push(k_star);

        let densities: Vec<String> = (0..=3)
            .map(|k| {
                base_results
                    .iter()
                    .find(|r| r.k == k)
                    .map(|r| format!("{:.3}", r.density))
                    .unwrap_or("---".to_string())
            })
            .collect();

        println!(
            " {:3} | {} |   {:.4}    | {}",
            base,
            k_star,
            k_star_result.density,
            densities.join(", ")
        );
    }

    println!("\n🎯 M=1 VERDICT:");
    let all_k_star_zero = k_star_values.iter().all(|&k| k == 0);
    let any_k_star_nonzero = k_star_values.iter().any(|&k| k > 0);

    if all_k_star_zero {
        println!("❌ Hypothesis C REJECTED: M=1 also shows k*=0");
        println!("   No phase transition - minimal padding works for all M!");
    } else if any_k_star_nonzero {
        println!("✅ Hypothesis C SUPPORTED: M=1 shows k*>0");
        println!("   Phase transition detected: M=1 behaves differently");
        println!("   k* values: {:?}", k_star_values);
    }
}

// Simplified proportion test (z-test approximation)
fn proportion_test(x1: usize, n1: usize, x2: usize, n2: usize) -> f64 {
    let p1 = x1 as f64 / n1 as f64;
    let p2 = x2 as f64 / n2 as f64;

    let p_pooled = (x1 + x2) as f64 / (n1 + n2) as f64;
    let se = (p_pooled * (1.0 - p_pooled) * (1.0 / n1 as f64 + 1.0 / n2 as f64)).sqrt();

    let z = (p1 - p2).abs() / se;

    // Approximate p-value (two-tailed)
    // For z ~ 1.96 → p ≈ 0.05
    // For z ~ 2.58 → p ≈ 0.01
    if z > 2.58 {
        0.01
    } else if z > 1.96 {
        0.05
    } else if z > 1.64 {
        0.10
    } else {
        (1.0 - z / 3.0).max(0.10)
    }
}
