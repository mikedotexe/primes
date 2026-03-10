//! Belphegor-Style Palindromic Prime Scanner
//!
//! Explores the parametric family: B(n, d) = (10^(n+k) + d) × 10^n + 1
//! where n = padding length, d = central seed, k = digits in d
//!
//! **Belphegor's Prime**: B(13, 666) = 1000000000000066600000000000001
//!
//! This tool investigates three profound questions:
//! 1. Is the seed 666 special, or do other seeds work?
//! 2. Is n=13 optimal for d=666, or do other lengths work?
//! 3. How does this relate to prime connector theory?
//!
//! Mathematical Framework:
//! - Bilateral symmetry: outer-0ₙ-seed-0ₙ-outer
//! - Zero-heavy structure (matches connector research: 52.9% avg)
//! - Modular arithmetic optimization
//! - Spectral resonances in (n,d) parameter space
//!
//! Usage:
//!   cargo run --example belphegor_scanner -- --seed-min 100 --seed-max 999 --pad-min 1 --pad-max 15
//!   cargo run --example belphegor_scanner -- --verify-belphegor
//!   cargo run --example belphegor_scanner -- --scan-666  # Test different n for seed=666

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use std::collections::HashMap;
use std::env;
use std::time::Instant;

// ========================================
// MILLER-RABIN (reused from scan_connectors.rs)
// ========================================

fn is_probably_prime(n: &BigUint, rounds: u32) -> bool {
    let two = BigUint::from(2u32);
    let three = BigUint::from(3u32);

    if *n < two {
        return false;
    }
    if *n == two || *n == three {
        return true;
    }
    if n % 2u32 == BigUint::zero() {
        return false;
    }

    let one = BigUint::one();
    let n_minus_one = n - &one;

    let mut d = n_minus_one.clone();
    let mut s: u32 = 0;
    while &d % 2u32 == BigUint::zero() {
        d /= 2u32;
        s += 1;
    }

    let bases: [u32; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    for (used_rounds, &a_u32) in bases.iter().enumerate() {
        if used_rounds as u32 >= rounds {
            break;
        }

        if BigUint::from(a_u32) >= n_minus_one {
            continue;
        }

        let mut x = BigUint::from(a_u32).modpow(&d, n);
        if x == one || x == n_minus_one {
            continue;
        }

        let mut composite = true;
        for _ in 1..s {
            x = x.modpow(&two, n);
            if x == n_minus_one {
                composite = false;
                break;
            }
        }

        if composite {
            return false;
        }
    }

    true
}

// ========================================
// BELPHEGOR CONSTRUCTION
// ========================================

#[derive(Debug, Clone)]
struct BelphegorPrime {
    outer: u8,
    seed: u32,
    padding: usize,
    _full_number: String,
    digit_count: usize,
    digit_sum: u32,
    zero_fraction: f64,
    is_palindrome: bool,
    mod3: u32,
    mod7: u32,
    mod11: u32,
}

impl BelphegorPrime {
    fn new(outer: u8, seed: u32, padding: usize, full_num: &BigUint) -> Self {
        let full_str = construct_belphegor_string(outer, seed, padding);
        let digit_sum: u32 = full_str.chars().filter_map(|c| c.to_digit(10)).sum();
        let zero_count = full_str.chars().filter(|&c| c == '0').count();
        let zero_fraction = zero_count as f64 / full_str.len() as f64;

        // Check palindrome
        let chars: Vec<char> = full_str.chars().collect();
        let is_palindrome = chars.iter().eq(chars.iter().rev());

        let mod3 = (full_num % 3u32).to_u32().unwrap_or(0);
        let mod7 = (full_num % 7u32).to_u32().unwrap_or(0);
        let mod11 = (full_num % 11u32).to_u32().unwrap_or(0);

        BelphegorPrime {
            outer,
            seed,
            padding,
            _full_number: full_str.clone(),
            digit_count: full_str.len(),
            digit_sum,
            zero_fraction,
            is_palindrome,
            mod3,
            mod7,
            mod11,
        }
    }
}

/// Construct Belphegor number as string: outer-0ₙ-seed-0ₙ-outer
fn construct_belphegor_string(outer: u8, seed: u32, padding: usize) -> String {
    format!(
        "{}{}{}{}{}",
        outer,
        "0".repeat(padding),
        seed,
        "0".repeat(padding),
        outer
    )
}

/// Construct Belphegor number as BigUint using formula:
/// Structure: outer-0ₙ-seed-0ₙ-outer
/// Example: 1-0₁₃-666-0₁₃-1 (31 digits total)
/// Positions: [30][29...17][16-15-14][13...1][0]
///
/// Numerical breakdown:
///   1 × 10^30              (left outer)
/// + 666 × 10^14            (seed with rightmost digit at pos 14 = padding + 1)
/// + 1                      (right outer at pos 0)
fn construct_belphegor_bigint(outer: u8, seed: u32, padding: usize) -> BigUint {
    let seed_digits = seed.to_string().len();
    let base = BigUint::from(10u32);

    // Seed starts at position (padding + 1) because of right_padding + right_outer
    let seed_position = padding + 1;
    // Left outer at position (seed_position + seed_digits + padding)
    let left_position = seed_position + seed_digits + padding;

    let left_outer = BigUint::from(outer) * base.pow(left_position as u32);
    let middle_seed = BigUint::from(seed) * base.pow(seed_position as u32);
    let right_outer = BigUint::from(outer);

    left_outer + middle_seed + right_outer
}

// ========================================
// SCANNING FUNCTIONS
// ========================================

struct ScanResults {
    primes: Vec<BelphegorPrime>,
    total_tested: u64,
    scan_time_ms: u128,
}

fn scan_belphegor_family(
    outer_digits: &[u8],
    seed_min: u32,
    seed_max: u32,
    pad_min: usize,
    pad_max: usize,
) -> ScanResults {
    let start_time = Instant::now();
    let mut primes = Vec::new();
    let mut total_tested: u64 = 0;

    eprintln!("🔱 Belphegor-Style Prime Scanner");
    eprintln!("================================\n");
    eprintln!("Configuration:");
    eprintln!("  Outer digits: {:?}", outer_digits);
    eprintln!("  Seed range: {}-{}", seed_min, seed_max);
    eprintln!("  Padding range: {}-{}", pad_min, pad_max);

    let total_candidates = outer_digits.len() as u64
        * (seed_max - seed_min + 1) as u64
        * (pad_max - pad_min + 1) as u64;
    eprintln!("  Total candidates: {}\n", total_candidates);

    eprintln!("Scanning...");

    for &outer in outer_digits {
        for seed in seed_min..=seed_max {
            for padding in pad_min..=pad_max {
                total_tested += 1;

                if total_tested.is_multiple_of(1000) {
                    eprint!(
                        "\r  Progress: {}/{} ({:.1}%) | Found: {}",
                        total_tested,
                        total_candidates,
                        (total_tested as f64 / total_candidates as f64) * 100.0,
                        primes.len()
                    );
                }

                let num = construct_belphegor_bigint(outer, seed, padding);

                if is_probably_prime(&num, 10) {
                    let prime_info = BelphegorPrime::new(outer, seed, padding, &num);
                    primes.push(prime_info);
                }
            }
        }
    }

    eprintln!(
        "\r  Progress: {}/{} (100.0%) | Found: {}    ",
        total_tested,
        total_candidates,
        primes.len()
    );

    let scan_time_ms = start_time.elapsed().as_millis();

    ScanResults {
        primes,
        total_tested,
        scan_time_ms,
    }
}

/// Specialized scan: test different padding lengths for seed=666
fn scan_666_across_paddings(pad_min: usize, pad_max: usize) -> Vec<BelphegorPrime> {
    let mut results = Vec::new();

    eprintln!(
        "\n🔥 Special scan: seed=666 across n={} to n={}",
        pad_min, pad_max
    );
    eprintln!("Testing if n=13 is unique or if other paddings also work...\n");

    for n in pad_min..=pad_max {
        let num = construct_belphegor_bigint(1, 666, n);

        eprint!(
            "\r  Testing n={:2} ({}  digits)...",
            n,
            num.to_string().len()
        );

        if is_probably_prime(&num, 15) {
            let prime_info = BelphegorPrime::new(1, 666, n, &num);
            eprintln!("\r  ✓ n={:2} is PRIME! (31 digits)", n);
            results.push(prime_info);
        }
    }

    eprintln!();
    results
}

// ========================================
// ANALYSIS FUNCTIONS
// ========================================

fn analyze_modular_distributions(primes: &[BelphegorPrime]) {
    println!("\n=== MODULAR ARITHMETIC ANALYSIS ===\n");

    if primes.is_empty() {
        println!("No primes to analyze.\n");
        return;
    }

    let mut mod3_dist: HashMap<u32, usize> = HashMap::new();
    let mut mod7_dist: HashMap<u32, usize> = HashMap::new();
    let mut mod11_dist: HashMap<u32, usize> = HashMap::new();

    for p in primes {
        *mod3_dist.entry(p.mod3).or_insert(0) += 1;
        *mod7_dist.entry(p.mod7).or_insert(0) += 1;
        *mod11_dist.entry(p.mod11).or_insert(0) += 1;
    }

    println!("Modulo 3 distribution:");
    let mut sorted: Vec<_> = mod3_dist.iter().collect();
    sorted.sort_by_key(|(k, _)| **k);
    for (residue, count) in sorted {
        println!(
            "  {} mod 3: {} primes ({:.1}%)",
            residue,
            count,
            (*count as f64 / primes.len() as f64) * 100.0
        );
    }

    println!("\nModulo 7 distribution:");
    let mut sorted: Vec<_> = mod7_dist.iter().collect();
    sorted.sort_by_key(|(k, _)| **k);
    for (residue, count) in sorted {
        println!(
            "  {} mod 7: {} primes ({:.1}%)",
            residue,
            count,
            (*count as f64 / primes.len() as f64) * 100.0
        );
    }

    println!("\nModulo 11 distribution:");
    let mut sorted: Vec<_> = mod11_dist.iter().collect();
    sorted.sort_by_key(|(k, _)| **k);
    for (residue, count) in sorted {
        println!(
            "  {} mod 11: {} primes ({:.1}%)",
            residue,
            count,
            (*count as f64 / primes.len() as f64) * 100.0
        );
    }
    println!();
}

fn analyze_seed_patterns(primes: &[BelphegorPrime]) {
    println!("\n=== SEED PATTERN ANALYSIS ===\n");

    if primes.is_empty() {
        return;
    }

    // Group by seed
    let mut by_seed: HashMap<u32, Vec<&BelphegorPrime>> = HashMap::new();
    for p in primes {
        by_seed.entry(p.seed).or_default().push(p);
    }

    println!("Seeds that produce primes (top 20):");
    let mut sorted: Vec<_> = by_seed.iter().collect();
    sorted.sort_by_key(|(_, v)| std::cmp::Reverse(v.len()));

    for (seed, instances) in sorted.iter().take(20) {
        let paddings: Vec<usize> = instances.iter().map(|p| p.padding).collect();
        println!(
            "  seed={:3}: {} primes at n={:?}",
            seed,
            instances.len(),
            paddings
        );
    }

    // Analyze seed properties
    let palindromic: Vec<_> = primes
        .iter()
        .filter(|p| {
            let s = p.seed.to_string();
            let chars: Vec<char> = s.chars().collect();
            chars.iter().eq(chars.iter().rev())
        })
        .collect();

    let repeating: Vec<_> = primes
        .iter()
        .filter(|p| {
            let s = p.seed.to_string();
            let first = s.chars().next().unwrap();
            s.chars().all(|c| c == first)
        })
        .collect();

    let mod3_zero: Vec<_> = primes.iter().filter(|p| p.seed % 3 == 0).collect();

    println!("\nSeed characteristics:");
    println!(
        "  Palindromic seeds: {}/{} ({:.1}%)",
        palindromic.len(),
        primes.len(),
        (palindromic.len() as f64 / primes.len() as f64) * 100.0
    );
    println!(
        "  Repeating digit seeds: {}/{} ({:.1}%)",
        repeating.len(),
        primes.len(),
        (repeating.len() as f64 / primes.len() as f64) * 100.0
    );
    println!(
        "  Seeds ≡ 0 (mod 3): {}/{} ({:.1}%)",
        mod3_zero.len(),
        primes.len(),
        (mod3_zero.len() as f64 / primes.len() as f64) * 100.0
    );
    println!();
}

fn analyze_padding_resonances(primes: &[BelphegorPrime]) {
    println!("\n=== PADDING RESONANCE ANALYSIS ===\n");

    if primes.is_empty() {
        return;
    }

    let mut by_padding: HashMap<usize, Vec<&BelphegorPrime>> = HashMap::new();
    for p in primes {
        by_padding.entry(p.padding).or_default().push(p);
    }

    println!("Resonance at different padding lengths:");
    let mut sorted: Vec<_> = by_padding.iter().collect();
    sorted.sort_by_key(|(k, _)| **k);

    for (padding, instances) in sorted {
        println!(
            "  n={:2}: {} primes ({:.1}% of total)",
            padding,
            instances.len(),
            (instances.len() as f64 / primes.len() as f64) * 100.0
        );
    }
    println!();
}

fn analyze_zero_fraction(primes: &[BelphegorPrime]) {
    println!("\n=== ZERO-HEAVY STRUCTURE ANALYSIS ===\n");

    if primes.is_empty() {
        return;
    }

    let avg_zero_fraction: f64 =
        primes.iter().map(|p| p.zero_fraction).sum::<f64>() / primes.len() as f64;

    println!("Average zero fraction: {:.1}%", avg_zero_fraction * 100.0);
    println!("(Compare to connector research: 52.9% average)\n");

    // Histogram of zero fractions
    let mut bins: HashMap<u32, usize> = HashMap::new();
    for p in primes {
        let bin = ((p.zero_fraction * 10.0) as u32).min(10);
        *bins.entry(bin).or_insert(0) += 1;
    }

    println!("Zero fraction distribution:");
    for bin in 0..=10 {
        let count = bins.get(&bin).unwrap_or(&0);
        let bar = "█".repeat(*count);
        println!("  {:2}-{:2}%: {} {}", bin * 10, (bin + 1) * 10, bar, count);
    }
    println!();
}

// ========================================
// MAIN DRIVER
// ========================================

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_usage(&args[0]);
        return;
    }

    // Parse command
    let command = &args[1];

    match command.as_str() {
        "--verify-belphegor" => {
            println!("\n🔍 Verifying Belphegor's Prime...\n");
            let num = construct_belphegor_bigint(1, 666, 13);
            let expected = "1000000000000066600000000000001";
            let actual = num.to_string();

            println!("Expected: {}", expected);
            println!("Actual:   {}", actual);
            println!("Match: {}", expected == actual);

            println!("\nTesting primality...");
            let is_prime = is_probably_prime(&num, 20);
            println!("Is prime: {}", if is_prime { "✓ YES" } else { "✗ NO" });

            if is_prime {
                let info = BelphegorPrime::new(1, 666, 13, &num);
                println!("\nProperties:");
                println!("  Digits: {}", info.digit_count);
                println!("  Digit sum: {}", info.digit_sum);
                println!("  Zero fraction: {:.1}%", info.zero_fraction * 100.0);
                println!("  Is palindrome: {}", info.is_palindrome);
                println!("  Mod 3: {}", info.mod3);
                println!("  Mod 7: {}", info.mod7);
                println!("  Mod 11: {}", info.mod11);
            }
        }

        "--scan-666" => {
            let results = scan_666_across_paddings(1, 20);
            println!("\n=== RESULTS ===\n");
            println!("Primes found for seed=666: {}", results.len());

            if results.is_empty() {
                println!("\nNo primes found! This would be surprising if n=13 wasn't tested.");
            } else {
                println!("\nAll padding lengths that produce primes:");
                for (i, p) in results.iter().enumerate() {
                    println!(
                        "  {}. n={} → {} digits (zero fraction: {:.1}%)",
                        i + 1,
                        p.padding,
                        p.digit_count,
                        p.zero_fraction * 100.0
                    );
                }

                if results.len() == 1 && results[0].padding == 13 {
                    println!("\n✨ Conclusion: n=13 is UNIQUE for seed=666!");
                    println!("   Belphegor's prime is truly special.");
                } else {
                    println!("\n✨ Conclusion: n=13 is NOT unique!");
                    println!("   Multiple resonances exist for seed=666.");
                }
            }
        }

        "--scan" => {
            // Full parametric scan
            let seed_min = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(100);
            let seed_max = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(999);
            let pad_min = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(1);
            let pad_max = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(15);

            let results = scan_belphegor_family(&[1], seed_min, seed_max, pad_min, pad_max);

            println!("\n=== SCAN RESULTS ===\n");
            println!("Total tested: {}", results.total_tested);
            println!("Primes found: {}", results.primes.len());
            println!("Scan time: {:.2}s", results.scan_time_ms as f64 / 1000.0);

            if results.primes.is_empty() {
                println!("\nNo Belphegor-style primes found in this range.");
                return;
            }

            println!(
                "\nSuccess rate: {:.4}%",
                (results.primes.len() as f64 / results.total_tested as f64) * 100.0
            );

            // Detailed analysis
            analyze_seed_patterns(&results.primes);
            analyze_padding_resonances(&results.primes);
            analyze_modular_distributions(&results.primes);
            analyze_zero_fraction(&results.primes);

            // Print all discoveries
            println!("\n=== ALL BELPHEGOR-STYLE PRIMES ===\n");
            for (i, p) in results.primes.iter().enumerate() {
                println!(
                    "{:3}. outer={}, seed={:3}, n={:2} → {} digits (zero: {:.1}%)",
                    i + 1,
                    p.outer,
                    p.seed,
                    p.padding,
                    p.digit_count,
                    p.zero_fraction * 100.0
                );
            }
        }

        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage(&args[0]);
        }
    }
}

fn print_usage(prog: &str) {
    eprintln!("Belphegor-Style Prime Scanner");
    eprintln!("=============================\n");
    eprintln!("Usage:");
    eprintln!("  {} --verify-belphegor", prog);
    eprintln!("      Verify the original Belphegor's prime\n");
    eprintln!("  {} --scan-666", prog);
    eprintln!("      Test different padding lengths for seed=666\n");
    eprintln!(
        "  {} --scan <seed_min> <seed_max> <pad_min> <pad_max>",
        prog
    );
    eprintln!("      Full parametric scan of Belphegor family\n");
    eprintln!("Examples:");
    eprintln!("  {} --verify-belphegor", prog);
    eprintln!("  {} --scan-666", prog);
    eprintln!("  {} --scan 100 999 10 15", prog);
}
