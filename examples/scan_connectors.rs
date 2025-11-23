//! Prime Connector Scanner - Mathematical Structure Analysis
//!
//! Fast CLI tool to discover and analyze prime connectors between two primes.
//! Uses Miller-Rabin primality testing with mod-3 filtering for efficiency.
//!
//! A "connector" C is a zero-padded string such that concat(p1, C, p2) is prime.
//!
//! Example:
//!   p1 = 10301, p2 = 3007003007003
//!   C = "00006" → 10301000063007003007003 is prime!
//!
//! Usage:
//!   cargo run --example scan_connectors -- 10301 3007003007003 1 7
//!   cargo run --example scan_connectors -- <prime1> <prime2> <min_len> <max_len>

use num_bigint::BigUint;
use num_traits::{One, Zero, ToPrimitive};
use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::time::Instant;

// ========================================
// MILLER-RABIN PRIMALITY TEST
// ========================================

/// Miller-Rabin primality test for BigUint.
/// Uses deterministic bases for numbers in reasonable range.
/// `rounds` controls how many witness bases to test.
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

    // Write n - 1 = d * 2^s with d odd
    let mut d = n_minus_one.clone();
    let mut s: u32 = 0;
    while &d % 2u32 == BigUint::zero() {
        d /= 2u32;
        s += 1;
    }

    // Use deterministic bases (sufficient for most practical ranges)
    let bases: [u32; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    let mut used_rounds = 0u32;
    for &a_u32 in bases.iter() {
        if used_rounds >= rounds {
            break;
        }
        used_rounds += 1;

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
// CONNECTOR ANALYSIS
// ========================================

#[derive(Debug, Clone)]
struct ConnectorInfo {
    length: usize,
    pattern: String,
    full_number: String,
    digit_sum: u32,
    last_digit: u8,
    mod3: u32,
    mod7: u32,
    mod11: u32,
}

impl ConnectorInfo {
    fn new(connector: &str, p1: &str, p2: &str, full_num: &BigUint) -> Self {
        let full_str = format!("{}{}{}", p1, connector, p2);
        let digit_sum: u32 = full_str.chars().filter_map(|c| c.to_digit(10)).sum();
        let last_digit = full_str.chars().last().unwrap().to_digit(10).unwrap() as u8;

        let mod3 = (full_num % 3u32).to_u32().unwrap_or(0);
        let mod7 = (full_num % 7u32).to_u32().unwrap_or(0);
        let mod11 = (full_num % 11u32).to_u32().unwrap_or(0);

        ConnectorInfo {
            length: connector.len(),
            pattern: connector.to_string(),
            full_number: full_str,
            digit_sum,
            last_digit,
            mod3,
            mod7,
            mod11,
        }
    }
}

struct ScanResults {
    connectors: Vec<ConnectorInfo>,
    total_candidates: u64,
    total_tested: u64,
    total_skipped_mod3: u64,
    scan_time_ms: u128,
}

// ========================================
// SCANNER ENGINE
// ========================================

fn scan_connectors(
    p1_str: &str,
    p2_str: &str,
    min_len: usize,
    max_len: usize,
) -> ScanResults {
    let start_time = Instant::now();

    let p1 = BigUint::from_str(p1_str).expect("invalid prime1");
    let p2 = BigUint::from_str(p2_str).expect("invalid prime2");

    // Precompute p1 and p2 mod 3 for fast filtering
    let p1_mod3 = (&p1 % 3u32).to_u32().unwrap_or(0);
    let p2_mod3 = (&p2 % 3u32).to_u32().unwrap_or(0);

    let mut connectors = Vec::new();
    let mut total_candidates: u64 = 0;
    let mut total_tested: u64 = 0;
    let mut total_skipped_mod3: u64 = 0;

    for len in min_len..=max_len {
        let upper = 10u64.pow(len as u32);

        eprintln!("Scanning length {} (0..{})...", len, upper - 1);

        for value in 0..upper {
            total_candidates += 1;

            // Zero-padded connector
            let connector = format!("{:0len$}", value, len = len);

            // Fast mod-3 filter: N(C) ≡ p1 + C + p2 (mod 3)
            // For primality (except 3), we need N(C) ≢ 0 (mod 3)
            let sum_digits: u32 = connector.bytes().map(|b| (b - b'0') as u32).sum();
            let n_mod3 = (p1_mod3 + p2_mod3 + (sum_digits % 3)) % 3;

            if n_mod3 == 0 {
                // Divisible by 3, skip
                total_skipped_mod3 += 1;
                continue;
            }

            total_tested += 1;

            // Build full concatenation
            let full_str = format!("{}{}{}", p1_str, connector, p2_str);
            let full_n = BigUint::from_str(&full_str).expect("failed to parse");

            // Miller-Rabin test
            if is_probably_prime(&full_n, 10) {
                let info = ConnectorInfo::new(&connector, p1_str, p2_str, &full_n);
                connectors.push(info);
            }
        }
    }

    let scan_time_ms = start_time.elapsed().as_millis();

    ScanResults {
        connectors,
        total_candidates,
        total_tested,
        total_skipped_mod3,
        scan_time_ms,
    }
}

// ========================================
// ANALYSIS FUNCTIONS
// ========================================

fn analyze_modular_properties(connectors: &[ConnectorInfo]) {
    println!("\n=== MODULAR ARITHMETIC ANALYSIS ===\n");

    if connectors.is_empty() {
        println!("No connectors to analyze.\n");
        return;
    }

    // Analyze mod 3, 7, 11
    let mut mod3_dist: HashMap<u32, usize> = HashMap::new();
    let mut mod7_dist: HashMap<u32, usize> = HashMap::new();
    let mut mod11_dist: HashMap<u32, usize> = HashMap::new();

    for conn in connectors {
        *mod3_dist.entry(conn.mod3).or_insert(0) += 1;
        *mod7_dist.entry(conn.mod7).or_insert(0) += 1;
        *mod11_dist.entry(conn.mod11).or_insert(0) += 1;
    }

    println!("Modulo 3 distribution:");
    let mut sorted: Vec<_> = mod3_dist.iter().collect();
    sorted.sort_by_key(|(k, _)| **k);
    for (residue, count) in sorted {
        println!(
            "  {} mod 3: {} connectors ({:.1}%)",
            residue,
            count,
            (*count as f64 / connectors.len() as f64) * 100.0
        );
    }

    println!("\nModulo 7 distribution:");
    let mut sorted: Vec<_> = mod7_dist.iter().collect();
    sorted.sort_by_key(|(k, _)| **k);
    for (residue, count) in sorted {
        println!(
            "  {} mod 7: {} connectors ({:.1}%)",
            residue,
            count,
            (*count as f64 / connectors.len() as f64) * 100.0
        );
    }

    println!("\nModulo 11 distribution:");
    let mut sorted: Vec<_> = mod11_dist.iter().collect();
    sorted.sort_by_key(|(k, _)| **k);
    for (residue, count) in sorted {
        println!(
            "  {} mod 11: {} connectors ({:.1}%)",
            residue,
            count,
            (*count as f64 / connectors.len() as f64) * 100.0
        );
    }
    println!();
}

fn analyze_digit_patterns(connectors: &[ConnectorInfo]) {
    println!("\n=== DIGIT PATTERN ANALYSIS ===\n");

    if connectors.is_empty() {
        return;
    }

    let mut digit_frequency: HashMap<char, usize> = HashMap::new();
    let mut total_digits = 0;

    for conn in connectors {
        for ch in conn.pattern.chars() {
            *digit_frequency.entry(ch).or_insert(0) += 1;
            total_digits += 1;
        }
    }

    println!("Digit frequency across all connectors:");
    let mut sorted: Vec<_> = digit_frequency.iter().collect();
    sorted.sort_by_key(|(k, _)| **k);
    for (digit, count) in sorted {
        println!(
            "  '{}': {} occurrences ({:.1}%)",
            digit,
            count,
            (*count as f64 / total_digits as f64) * 100.0
        );
    }

    // Analyze zero/six dominance
    let zero_six_only: Vec<_> = connectors
        .iter()
        .filter(|c| c.pattern.chars().all(|ch| ch == '0' || ch == '6'))
        .collect();

    let zero_three_only: Vec<_> = connectors
        .iter()
        .filter(|c| c.pattern.chars().all(|ch| ch == '0' || ch == '3'))
        .collect();

    println!("\nConnectors using only 0 and 6: {}", zero_six_only.len());
    for c in zero_six_only.iter().take(10) {
        println!("  {}", c.pattern);
    }
    if zero_six_only.len() > 10 {
        println!("  ... and {} more", zero_six_only.len() - 10);
    }

    println!("\nConnectors using only 0 and 3: {}", zero_three_only.len());
    for c in zero_three_only.iter().take(10) {
        println!("  {}", c.pattern);
    }
    if zero_three_only.len() > 10 {
        println!("  ... and {} more", zero_three_only.len() - 10);
    }

    // Zero ratio analysis
    let high_zero: Vec<_> = connectors
        .iter()
        .map(|c| {
            let zeros = c.pattern.chars().filter(|&ch| ch == '0').count();
            let ratio = (zeros * 100) / c.pattern.len();
            (c, ratio)
        })
        .filter(|(_, ratio)| *ratio > 70)
        .collect();

    println!("\nHigh zero ratio (>70%): {} connectors", high_zero.len());
    println!();
}

fn analyze_symmetry(connectors: &[ConnectorInfo]) {
    println!("\n=== SYMMETRY ANALYSIS ===\n");

    if connectors.is_empty() {
        return;
    }

    let palindromes: Vec<_> = connectors
        .iter()
        .filter(|c| {
            let chars: Vec<char> = c.pattern.chars().collect();
            chars.iter().eq(chars.iter().rev())
        })
        .collect();

    println!("Palindromic connectors: {}", palindromes.len());
    for p in palindromes.iter().take(10) {
        println!("  {}", p.pattern);
    }
    if palindromes.len() > 10 {
        println!("  ... and {} more", palindromes.len() - 10);
    }
    println!();
}

fn analyze_by_length(connectors: &[ConnectorInfo]) {
    println!("\n=== LENGTH DISTRIBUTION ===\n");

    if connectors.is_empty() {
        return;
    }

    let mut by_length: HashMap<usize, Vec<&ConnectorInfo>> = HashMap::new();

    for conn in connectors {
        by_length.entry(conn.length).or_insert_with(Vec::new).push(conn);
    }

    let mut sorted_lengths: Vec<_> = by_length.keys().collect();
    sorted_lengths.sort();

    for len in sorted_lengths {
        let conns = &by_length[len];
        println!("Length {}: {} connectors", len, conns.len());

        // Show up to 5 examples
        for c in conns.iter().take(5) {
            println!("  {}", c.pattern);
        }
        if conns.len() > 5 {
            println!("  ... and {} more", conns.len() - 5);
        }
        println!();
    }
}

// ========================================
// MAIN DRIVER
// ========================================

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Prime Connector Scanner");
        eprintln!("=======================\n");
        eprintln!("Usage: {} <prime1> <prime2> <min_len> <max_len>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} 10301 3007003007003 1 7", args[0]);
        eprintln!("\nThis will scan for all connectors C of length 1-7 such that");
        eprintln!("concat(prime1, C, prime2) is prime.\n");
        std::process::exit(1);
    }

    let p1_str = &args[1];
    let p2_str = &args[2];
    let min_len: usize = args[3].parse().expect("min_len must be a positive integer");
    let max_len: usize = args[4].parse().expect("max_len must be a positive integer");

    if min_len == 0 || max_len < min_len {
        eprintln!("Error: Require 1 <= min_len <= max_len");
        std::process::exit(1);
    }

    if max_len > 9 {
        eprintln!(
            "Warning: max_len > 9 may take very long (10^{} candidates per length)",
            max_len
        );
    }

    println!("🔬 Prime Connector Scanner");
    println!("===========================\n");
    println!("Prime 1: {}", p1_str);
    println!("Prime 2: {}", p2_str);
    println!("Scanning connector lengths: {} to {}\n", min_len, max_len);

    // Run the scan
    let results = scan_connectors(p1_str, p2_str, min_len, max_len);

    // Print summary
    println!("\n=== SCAN RESULTS ===\n");
    println!("Total candidates examined: {}", results.total_candidates);
    println!("Skipped (divisible by 3): {}", results.total_skipped_mod3);
    println!("Actually tested (Miller-Rabin): {}", results.total_tested);
    println!("Prime connectors found: {}", results.connectors.len());
    println!("Scan time: {:.2}s", results.scan_time_ms as f64 / 1000.0);

    if results.connectors.is_empty() {
        println!("\nNo prime connectors found in this range.");
        return;
    }

    println!(
        "\nDensity: {:.4}% of tested candidates are prime",
        (results.connectors.len() as f64 / results.total_tested as f64) * 100.0
    );

    // Detailed analysis
    analyze_by_length(&results.connectors);
    analyze_modular_properties(&results.connectors);
    analyze_digit_patterns(&results.connectors);
    analyze_symmetry(&results.connectors);

    // Print all connectors
    println!("\n=== ALL PRIME CONNECTORS ===\n");
    for (i, conn) in results.connectors.iter().enumerate() {
        println!(
            "{:3}. len={} connector={} (full: {} digits)",
            i + 1,
            conn.length,
            conn.pattern,
            conn.full_number.len()
        );
    }

    println!("\n✨ Mathematical Insights:");
    println!("=========================");
    println!("• Mod-3 filter eliminated {:.1}% of candidates before primality testing",
        (results.total_skipped_mod3 as f64 / results.total_candidates as f64) * 100.0);
    println!("• Average connector density: {:.2}%",
        (results.connectors.len() as f64 / results.total_tested as f64) * 100.0);

    if results.connectors.len() > 100 {
        println!("• Exceptional density detected! This prime pair shows remarkable connector abundance.");
    }

    println!("\n💡 Next Steps:");
    println!("• Use these connectors as 'known_connectors' in the TUI presets");
    println!("• Compare density across different prime pairs");
    println!("• Export to CSV for further statistical analysis");
}
