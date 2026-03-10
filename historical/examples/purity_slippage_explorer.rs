//! Purity-Slippage Explorer
//!
//! The material landscape sweep revealed:
//! - Slippage (λ/φ) correlates r=0.55 with membrane success
//! - Full purity fraction correlates r=0.58 with membrane success
//!
//! This script explores WHY these correlations exist by looking at
//! the joint distribution of purity and slippage across bases.

use primes::hzlib::num_theory::{factor, phi_from_factor, carmichael_lambda_from_factor, Material};

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 { return false; }
        d += 2;
    }
    true
}

/// What determines slippage? λ/φ depends on how the multiplicative group
/// decomposes under CRT.
///
/// For prime p: λ(p) = φ(p) = p-1, so slippage = 1.0 (always)
/// For prime power p^k: λ(p^k) = φ(p^k) for odd p, so slippage = 1.0
/// For composite n = p1^a1 * ... * pk^ak:
///   φ(n) = prod φ(pi^ai)
///   λ(n) = lcm(λ(pi^ai))
///   slippage = lcm/prod ≤ 1
///
/// Low slippage means high "redundancy" in the group structure.
fn analyze_slippage_structure() {
    println!("=== SLIPPAGE STRUCTURE ANALYSIS ===\n");

    // Compare slippage for different structural types
    let types = vec![
        ("Primes", vec![7, 11, 13, 17, 19, 23, 29, 31]),
        ("Prime squares", vec![9, 25, 49, 121, 169]),
        ("2 * prime", vec![6, 10, 14, 22, 26, 34]),
        ("prime * prime", vec![15, 21, 33, 35, 55, 77]),
        ("2^k", vec![4, 8, 16, 32, 64]),
        ("2^k * prime", vec![12, 20, 24, 28, 40, 44]),
        ("3 distinct primes", vec![30, 42, 66, 70, 78]),
    ];

    for (name, examples) in types {
        let slippages: Vec<f64> = examples.iter().map(|&n| {
            let f = factor(n);
            let phi = phi_from_factor(&f);
            let lam = carmichael_lambda_from_factor(&f);
            lam as f64 / phi as f64
        }).collect();

        let mean = slippages.iter().sum::<f64>() / slippages.len() as f64;
        println!("{:20} mean_slippage={:.4} examples={:?}", name, mean, slippages);
    }
}

/// Key insight: For base b analysis, we strip b's prime factors.
/// Different bases lead to different "effective cores" for the same number.
///
/// Example: 30 in base 6 vs base 10
/// - Base 6: factors of 6 are {2,3}, so 30 → core=5 (30/6=5)
/// - Base 10: factors of 10 are {2,5}, so 30 → core=3 (30/10=3)
fn analyze_core_differences() {
    println!("\n=== CORE DIFFERENCES BY BASE ===\n");

    let test_numbers = vec![30, 60, 90, 120, 150, 180, 210, 240];
    let bases = vec![6, 10, 12, 30];

    println!("{:>6} {:>8} {:>8} {:>8} {:>8}", "n", "B6_core", "B10_core", "B12_core", "B30_core");
    println!("{}", "-".repeat(50));

    for n in test_numbers {
        let cores: Vec<String> = bases.iter().map(|&b| {
            let m = Material::for_base(n, b);
            format!("{}", m.core)
        }).collect();
        println!("{:>6} {:>8} {:>8} {:>8} {:>8}", n, cores[0], cores[1], cores[2], cores[3]);
    }
}

/// What makes base 6 special for membranes?
///
/// Hypothesis: 6 = 2*3 strips the MOST common small factors.
/// Numbers coprime to 6 have cores that avoid 2 and 3.
/// These residue classes (1, 5 mod 6) are exactly the membrane boundaries!
fn analyze_base6_magic() {
    println!("\n=== BASE 6 MAGIC ===\n");

    // In base 6, what fraction of numbers 1..N have "interesting" cores?
    let limit = 1000u64;

    let mut core_counts: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();

    for n in 1..=limit {
        let m = Material::for_base(n, 6);
        if m.core > 1 {
            *core_counts.entry(m.core).or_insert(0) += 1;
        }
    }

    let mut cores: Vec<(u64, usize)> = core_counts.into_iter().collect();
    cores.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    println!("Most common cores in base 6 (for n ≤ {}):\n", limit);
    println!("{:>8} {:>8} {:>8} {:>10}", "Core", "Count", "Prime?", "Purity");
    println!("{}", "-".repeat(40));

    for (core, count) in cores.iter().take(20) {
        let m = Material::for_base(*core, 6);
        let prime_str = if is_prime(*core) { "yes" } else { "no" };
        println!("{:>8} {:>8} {:>8} {:>10.4}", core, count, prime_str, m.purity);
    }

    // Now check: what's the purity distribution for membrane boundaries?
    // Membrane uses boundary digits (1, 5) in base 6 → residues ≡ 1, 5 (mod 6)
    println!("\n\nPurity by residue class mod 6:\n");

    for r in [1, 5] {
        let mut purities = Vec::new();
        for n in (r..=limit).step_by(6) {
            if n == 0 { continue; }
            let m = Material::for_base(n, 6);
            if m.core > 1 {
                purities.push(m.purity);
            }
        }

        let mean = purities.iter().sum::<f64>() / purities.len() as f64;
        let full_count = purities.iter().filter(|&&p| (p - 1.0).abs() < 1e-10).count();

        println!("n ≡ {} (mod 6): mean_purity={:.4}, full_purity={}/{} ({:.1}%)",
                 r, mean, full_count, purities.len(), 100.0 * full_count as f64 / purities.len() as f64);
    }

    // Compare with all residues
    let mut all_purities = Vec::new();
    for n in 1..=limit {
        let m = Material::for_base(n, 6);
        if m.core > 1 {
            all_purities.push(m.purity);
        }
    }
    let all_mean = all_purities.iter().sum::<f64>() / all_purities.len() as f64;
    let all_full = all_purities.iter().filter(|&&p| (p - 1.0).abs() < 1e-10).count();
    println!("all n: mean_purity={:.4}, full_purity={}/{} ({:.1}%)",
             all_mean, all_full, all_purities.len(), 100.0 * all_full as f64 / all_purities.len() as f64);
}

/// The slippage paradox: why does HIGHER slippage correlate with membrane success?
///
/// Hypothesis: High mean slippage for a base indicates that composite numbers
/// in that base tend to have λ close to φ (less redundancy).
/// This means the decimal-like expansions are "richer" with less structure collapse.
fn slippage_paradox() {
    println!("\n=== SLIPPAGE PARADOX ===\n");

    println!("Membrane success correlates positively with mean slippage.");
    println!("But slippage = λ/φ, and λ ≤ φ always, so slippage ≤ 1.\n");

    println!("Key insight: The correlation is BASE-DEPENDENT.\n");

    // Different bases strip different factors, leaving different cores.
    // Bases that strip more factors tend to leave cores with higher slippage.

    let bases = vec![
        (6, "2*3", 0.33),
        (10, "2*5", 0.185),
        (12, "2^2*3", 0.30),
        (30, "2*3*5", 0.30),
    ];

    println!("{:>4} {:>8} {:>10} {:>12} {:>12}", "Base", "Factors", "Membrane%", "Primes/All", "Interpretation");
    println!("{}", "-".repeat(60));

    for (base, factors, membrane) in bases {
        let base_primes: Vec<u64> = factor(base).iter().map(|(p, _)| *p).collect();

        let limit = 1000u64;
        let mut prime_cores = 0;
        let mut composite_cores = 0;

        for n in 1..=limit {
            if base_primes.iter().any(|&p| n % p == 0) {
                continue; // skip multiples of base factors
            }

            let m = Material::for_base(n, base);
            if m.core > 1 {
                if is_prime(m.core) {
                    prime_cores += 1;
                } else {
                    composite_cores += 1;
                }
            }
        }

        let prime_frac = prime_cores as f64 / (prime_cores + composite_cores) as f64;

        let interp = if prime_frac > 0.5 { "prime-heavy" } else { "composite-heavy" };

        println!("{:>4} {:>8} {:>10.1}% {:>12.1}% {:>12}",
                 base, factors, membrane * 100.0, prime_frac * 100.0, interp);
    }

    println!("\n");
    println!("Primes always have slippage = 1.0 (since λ = φ = p-1).");
    println!("Composites have slippage < 1.0.");
    println!("So bases with more prime cores have higher mean slippage.");
    println!("\nThis explains the correlation: membrane-friendly bases");
    println!("have factorizations that leave more PRIMES as cores!");
}

/// Final synthesis: what predicts membrane success?
fn synthesis() {
    println!("\n=== SYNTHESIS ===\n");

    println!("Three interrelated factors predict membrane success:\n");

    println!("1. PRIMITIVE ROOT DENSITY (weak, r=0.25)");
    println!("   → More primes where base is primitive root");
    println!("   → More numbers with purity = 1.0\n");

    println!("2. FULL PURITY FRACTION (moderate, r=0.58)");
    println!("   → Direct measure of how many numbers have ord = φ");
    println!("   → Higher means more 'clean' cycles\n");

    println!("3. MEAN SLIPPAGE (moderate, r=0.55)");
    println!("   → Proxy for 'how many cores are prime'");
    println!("   → Primes have slippage = 1, composites < 1");
    println!("   → High slippage → more prime cores → better membranes\n");

    println!("UNIFIED HYPOTHESIS:");
    println!("─────────────────────────────────────────────────────────");
    println!("Membrane success depends on having many PRIME CORES.");
    println!("This happens when the base's factorization eliminates");
    println!("common small primes (2, 3, 5) leaving behind larger primes.");
    println!("");
    println!("Base 6 = 2*3 strips {{2,3}} → cores avoid smallest primes");
    println!("Base 30 = 2*3*5 strips {{2,3,5}} → even more filtering");
    println!("Base 10 = 2*5 misses 3 → more composite cores survive");
    println!("─────────────────────────────────────────────────────────");
}

fn main() {
    analyze_slippage_structure();
    analyze_core_differences();
    analyze_base6_magic();
    slippage_paradox();
    synthesis();
}
