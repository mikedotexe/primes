//! Comprehensive Base Analysis
//!
//! Systematically tests membrane configurations across different bases
//! to find the true optimal patterns using proper statistics.

use num_bigint::BigUint;
use prime_physics_engine::{is_prime, MembraneConfig};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct BaseAnalysis {
    #[allow(dead_code)]
    base: u32,
    configs_tested: usize,
    total_candidates: usize,
    total_primes: usize,
    best_config: Option<ConfigStats>,
    top_configs: Vec<ConfigStats>,
}

#[derive(Debug, Clone)]
struct ConfigStats {
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    primes_found: usize,
    seeds_tested: usize,
    success_rate: f64,
    successful_seeds: Vec<u32>,
}

fn main() {
    println!("🔬 Comprehensive Base Analysis");
    println!("{}", "=".repeat(80));
    println!();

    let bases_to_test = vec![6, 8, 10, 12, 14, 16, 18, 20, 30];
    let mut results: HashMap<u32, BaseAnalysis> = HashMap::new();

    for base in bases_to_test {
        println!("Analyzing base {}...", base);
        let analysis = analyze_base(base);

        if let Some(ref best) = analysis.best_config {
            println!(
                "  Best: ({},{}) k=({},{}) - {:.1}% success",
                best.outer,
                best.inner,
                best.k_outer,
                best.k_inner,
                best.success_rate * 100.0
            );
        }

        results.insert(base, analysis);
    }

    // Print comprehensive results
    print_analysis_results(&results);

    // Test specific known good configurations
    test_known_configurations();
}

fn analyze_base(base: u32) -> BaseAnalysis {
    let mut analysis = BaseAnalysis {
        base,
        configs_tested: 0,
        total_candidates: 0,
        total_primes: 0,
        best_config: None,
        top_configs: Vec::new(),
    };

    let mut all_configs = Vec::new();

    // Test various configurations
    for outer in 1..base {
        if gcd(outer, base) != 1 {
            continue;
        } // Skip non-coprime

        for inner in 1..base {
            if gcd(inner, base) != 1 {
                continue;
            } // Skip non-coprime

            for k_outer in 0..=2 {
                for k_inner in 0..=2 {
                    let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);
                    let stats = test_configuration(&config, base);

                    analysis.configs_tested += 1;
                    analysis.total_candidates += stats.seeds_tested;
                    analysis.total_primes += stats.primes_found;

                    if stats.primes_found > 0 {
                        all_configs.push(stats);
                    }
                }
            }
        }
    }

    // Sort by success rate
    all_configs.sort_by(|a, b| b.success_rate.partial_cmp(&a.success_rate).unwrap());

    // Keep top 5
    analysis.top_configs = all_configs.into_iter().take(5).collect();
    analysis.best_config = analysis.top_configs.first().cloned();

    analysis
}

fn test_configuration(config: &MembraneConfig, base: u32) -> ConfigStats {
    let mut stats = ConfigStats {
        outer: config.outer,
        inner: config.inner,
        k_outer: config.k_outer,
        k_inner: config.k_inner,
        primes_found: 0,
        seeds_tested: 0,
        success_rate: 0.0,
        successful_seeds: Vec::new(),
    };

    // Test appropriate number of seeds based on base
    let max_seed = if base <= 10 { base } else { 20.min(base) };

    for seed in 0..max_seed {
        stats.seeds_tested += 1;

        // Build membrane string
        let membrane = construct_membrane_string(config, seed);

        // Convert from base to decimal
        if let Some(decimal) = convert_from_base(&membrane, base) {
            if is_prime(&decimal) {
                stats.primes_found += 1;
                stats.successful_seeds.push(seed);
            }
        }
    }

    stats.success_rate = stats.primes_found as f64 / stats.seeds_tested as f64;
    stats
}

fn construct_membrane_string(config: &MembraneConfig, seed: u32) -> String {
    // Convert numbers to proper base representation
    let outer_str = to_base_string(config.outer, config.base);
    let inner_str = to_base_string(config.inner, config.base);
    let seed_str = to_base_string(seed, config.base);

    format!(
        "{}{}{}{}{}{}{}{}{}",
        outer_str,
        "0".repeat(config.k_outer as usize),
        inner_str,
        "0".repeat(config.k_inner as usize),
        seed_str,
        "0".repeat(config.k_inner as usize),
        inner_str,
        "0".repeat(config.k_outer as usize),
        outer_str
    )
}

fn to_base_string(mut n: u32, base: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    while n > 0 {
        let digit = n % base;
        let ch = if digit < 10 {
            char::from_digit(digit, 10).unwrap()
        } else {
            // For bases > 10, use A, B, C, etc.
            char::from_u32('A' as u32 + digit - 10).unwrap()
        };
        result.insert(0, ch);
        n /= base;
    }
    result
}

fn convert_from_base(s: &str, base: u32) -> Option<BigUint> {
    BigUint::parse_bytes(s.as_bytes(), base)
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn print_analysis_results(results: &HashMap<u32, BaseAnalysis>) {
    println!("\n\n📊 COMPREHENSIVE RESULTS");
    println!("{}", "=".repeat(80));
    println!("Base | Best Config        | Success | Primes/Total | Top Performer");
    println!("-----|-------------------|---------|--------------|---------------");

    let mut bases: Vec<_> = results.keys().copied().collect();
    bases.sort();

    for base in bases {
        if let Some(analysis) = results.get(&base) {
            if let Some(ref best) = analysis.best_config {
                println!(
                    "{:4} | ({},{}) k=({},{})    | {:5.1}%  | {:4}/{:5}    | Seeds: {:?}",
                    base,
                    best.outer,
                    best.inner,
                    best.k_outer,
                    best.k_inner,
                    best.success_rate * 100.0,
                    analysis.total_primes,
                    analysis.total_candidates,
                    best.successful_seeds.iter().take(3).collect::<Vec<_>>()
                );
            }
        }
    }

    // Find universal patterns
    println!("\n\n🌟 UNIVERSAL PATTERNS");
    println!("{}", "=".repeat(80));

    // Check if (1,base-1) pattern works across bases
    println!("Testing (1,n-1) pattern:");
    for base in &[6u32, 10, 12, 14, 16, 18, 20, 30] {
        let config = MembraneConfig::new(*base, 1, base - 1, 0, 0);
        let stats = test_configuration(&config, *base);
        if stats.success_rate > 0.0 {
            println!(
                "  Base {:2}: ({},{}) → {:.1}% success",
                base,
                1,
                base - 1,
                stats.success_rate * 100.0
            );
        }
    }
}

fn test_known_configurations() {
    println!("\n\n✓ VERIFYING KNOWN CONFIGURATIONS");
    println!("{}", "=".repeat(80));

    let known_configs = vec![
        (6, 1, 5, 0, 0, vec![4], "Base 6 champion"),
        (10, 3, 3, 0, 1, vec![4, 5, 7], "Breathing pattern"),
        (10, 3, 7, 1, 1, vec![5], "Exclusive config"),
        (30, 11, 7, 0, 0, vec![], "Base 30 optimal"),
    ];

    for (base, outer, inner, k_outer, k_inner, expected_seeds, name) in known_configs {
        println!(
            "\n{} - Base {} ({},{}) k=({},{})",
            name, base, outer, inner, k_outer, k_inner
        );

        let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);
        let stats = test_configuration(&config, base);

        println!(
            "  Found: {} primes from {} seeds ({:.1}%)",
            stats.primes_found,
            stats.seeds_tested,
            stats.success_rate * 100.0
        );
        println!("  Successful seeds: {:?}", stats.successful_seeds);

        if !expected_seeds.is_empty() && stats.successful_seeds != expected_seeds {
            println!(
                "  ⚠️  WARNING: Seeds don't match expected {:?}",
                expected_seeds
            );
        }
    }
}
