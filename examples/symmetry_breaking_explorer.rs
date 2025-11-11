//! Symmetry Breaking Point Explorer
//!
//! Analyzes WHERE in seed space symmetric membrane patterns fail to generate primes.
//! Identifies "dark zones" (failure clusters) and patterns in symmetry breaking.
//!
//! ## Key Questions
//!
//! 1. Do failures cluster at specific seed values?
//! 2. Are there modular arithmetic patterns in failures?
//! 3. Can we predict which seeds will fail?
//! 4. Do certain digit properties correlate with failure?
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example symmetry_breaking_explorer
//! ```
//!
//! ## Output
//!
//! - Seed-by-seed success/failure visualization
//! - Failure cluster analysis
//! - Modular pattern detection
//! - Digit property correlations

use num_bigint::BigUint;
use primes::hzlib::*;
use primes::is_prime;

#[derive(Debug)]
struct TestConfig {
    base: usize,
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    description: &'static str,
}

/// Build membrane number from config and seed
fn build_membrane(config: &TestConfig, seed: u32) -> Option<BigUint> {
    let outer_str = config.outer.to_string();
    let inner_str = config.inner.to_string();
    let seed_str = seed.to_string();

    let zeros_outer = "0".repeat(config.k_outer as usize);
    let zeros_inner = "0".repeat(config.k_inner as usize);

    let membrane_str = format!(
        "{}{}{}{}{}{}{}{}{}",
        outer_str,
        zeros_outer,
        inner_str,
        zeros_inner,
        seed_str,
        zeros_inner,
        inner_str,
        zeros_outer,
        outer_str
    );

    membrane_str.parse::<BigUint>().ok()
}

/// Test a configuration across seed range
fn test_configuration(config: &TestConfig, seed_range: std::ops::Range<u32>) -> SymmetryBreaker {
    let mut breaker = SymmetryBreaker::new(
        config.base,
        config.outer,
        config.inner,
        config.k_outer,
        config.k_inner,
    );

    for seed in seed_range {
        if let Some(num) = build_membrane(config, seed) {
            let prime = is_prime(&num);
            breaker.record_seed(seed, prime);
        }
    }

    breaker
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         SYMMETRY BREAKING POINT EXPLORER                  ║");
    println!("║         Where Do Membrane Patterns Break Down?            ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Research Question:");
    println!("  Do membrane failures cluster at specific seed values?");
    println!("  Can we predict which seeds will break symmetry?");
    println!();

    // Test configurations: mix of high and low performers
    let configs = vec![
        TestConfig {
            base: 6,
            outer: 1,
            inner: 5,
            k_outer: 0,
            k_inner: 0,
            description: "Base 6 Champion (31% success)",
        },
        TestConfig {
            base: 10,
            outer: 3,
            inner: 7,
            k_outer: 0,
            k_inner: 0,
            description: "Base 10 Classic (20% success)",
        },
        TestConfig {
            base: 14,
            outer: 1,
            inner: 9,
            k_outer: 0,
            k_inner: 0,
            description: "Base 14 Strong (27% success)",
        },
        TestConfig {
            base: 7,
            outer: 1,
            inner: 6,
            k_outer: 0,
            k_inner: 0,
            description: "Base 7 Moderate (12% success)",
        },
    ];

    let seed_range = 0..20; // Test seeds 0-19

    println!("═══════════════════════════════════════════════════════════════");
    println!("SEED-LEVEL ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let mut accum = BreakingAccumulator::new();

    for config in &configs {
        println!("─────────────────────────────────────────────────────────────");
        println!("{}", config.description);
        println!(
            "Config: {}",
            format!(
                "Base {} ({},{}) k=({},{})",
                config.base, config.outer, config.inner, config.k_outer, config.k_inner
            )
        );
        println!("─────────────────────────────────────────────────────────────");
        println!();

        let breaker = test_configuration(config, seed_range.clone());

        // Visualization: seed-by-seed results
        println!("Seed Results:");
        println!("  (✓ = prime, ✗ = composite, ░ = dark zone)");
        println!();

        let mut seeds: Vec<_> = breaker.seed_results.keys().copied().collect();
        seeds.sort_unstable();

        print!("  Seeds: ");
        for (i, &seed) in seeds.iter().enumerate() {
            if i > 0 && i % 10 == 0 {
                println!();
                print!("         ");
            }
            print!("{:2} ", seed);
        }
        println!();

        print!("  Prime: ");
        for (i, &seed) in seeds.iter().enumerate() {
            if i > 0 && i % 10 == 0 {
                println!();
                print!("         ");
            }

            let is_prime = breaker.seed_results.get(&seed).copied().unwrap_or(false);
            let symbol = if is_prime { "✓ " } else { "✗ " };
            print!("{} ", symbol);
        }
        println!();
        println!();

        // Success rate
        println!(
            "  Success Rate: {:.1}% ({}/{})",
            breaker.success_rate() * 100.0,
            breaker.success_count(),
            breaker.total_tested()
        );
        println!();

        // Failure clusters
        let clusters = breaker.find_failure_clusters();
        if !clusters.is_empty() {
            println!("  Failure Clusters:");
            for (start, len) in &clusters {
                println!(
                    "    Seeds {}-{}: {} consecutive failures",
                    start,
                    start + *len as u32 - 1,
                    len
                );
            }
            println!();
        } else {
            println!("  No failure clusters detected (no consecutive failures)");
            println!();
        }

        // Dark zones
        let dark = breaker.find_dark_zones();
        if !dark.is_empty() {
            println!("  Dark Zones (always fail):");
            print!("    Seeds: ");
            for (i, &seed) in dark.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", seed);
            }
            println!();
            println!("    Count: {} seeds", dark.len());
            println!();
        }

        // Bright zones
        let bright = breaker.find_bright_zones();
        if !bright.is_empty() {
            println!("  Bright Zones (always succeed):");
            print!("    Seeds: ");
            let display_count = bright.len().min(10);
            for (i, &seed) in bright.iter().take(display_count).enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", seed);
            }
            if bright.len() > display_count {
                print!(" ... ({} total)", bright.len());
            }
            println!();
            println!();
        }

        // Modular patterns
        println!("  Modular Failure Patterns:");
        println!("    (Testing if failures correlate with seed mod m)");
        println!();

        let patterns = breaker.modular_failure_pattern();
        println!("    Top patterns by variance:");
        for (i, (m, variance)) in patterns.iter().take(3).enumerate() {
            let strength = if *variance > 0.05 {
                "Strong"
            } else if *variance > 0.02 {
                "Moderate"
            } else {
                "Weak"
            };

            println!(
                "      {}. mod {}: variance = {:.4} [{}]",
                i + 1,
                m,
                variance,
                strength
            );
        }
        println!();

        // Digit property correlations
        println!("  Digit Property Correlations:");
        println!("    (Positive = property correlates with failure)");
        println!();

        let correlations = breaker.digit_property_correlation();
        let mut corr_vec: Vec<_> = correlations.iter().collect();
        corr_vec.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap());

        for (prop, corr) in corr_vec.iter().take(3) {
            let direction = if **corr > 0.0 {
                "correlates with failure"
            } else {
                "correlates with success"
            };

            let strength = if corr.abs() > 0.2 {
                "Strong"
            } else if corr.abs() > 0.1 {
                "Moderate"
            } else {
                "Weak"
            };

            println!(
                "      {:<20}: {:+.3} [{}] {}",
                prop, corr, strength, direction
            );
        }
        println!();

        accum.add_breaker(breaker);
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("CROSS-CONFIGURATION ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Universal failures
    let universal_fail = accum.universal_failures();
    if !universal_fail.is_empty() {
        println!("Universal Failure Seeds (fail in ALL configs):");
        print!("  Seeds: ");
        for (i, &seed) in universal_fail.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", seed);
        }
        println!();
        println!("  Count: {} seeds", universal_fail.len());
        println!();
        println!("  💡 These seeds may have fundamental compositeness properties!");
        println!();
    } else {
        println!("No universal failure seeds found.");
        println!("  (No seeds fail in all tested configurations)");
        println!();
    }

    // Universal successes
    let universal_success = accum.universal_successes();
    if !universal_success.is_empty() {
        println!("Universal Success Seeds (succeed in ALL configs):");
        print!("  Seeds: ");
        for (i, &seed) in universal_success.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", seed);
        }
        println!();
        println!("  Count: {} seeds", universal_success.len());
        println!();
        println!("  💡 These seeds may have fundamental primality-enhancing properties!");
        println!();
    } else {
        println!("No universal success seeds found.");
        println!("  (Success is configuration-dependent)");
        println!();
    }

    // Average success rate
    println!(
        "Average Success Rate: {:.1}%",
        accum.average_success_rate() * 100.0
    );
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("KEY INSIGHTS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("1. FAILURE CLUSTERING:");
    println!("   Consecutive failures suggest systematic breakdown regions");
    println!("   These \"dark zones\" may indicate divisibility patterns");
    println!();

    println!("2. MODULAR PATTERNS:");
    println!("   High variance in failure rates by residue class suggests");
    println!("   modular arithmetic plays a role in symmetry breaking");
    println!();

    println!("3. DIGIT PROPERTIES:");
    println!("   Correlations between seed properties and failure rates");
    println!("   may reveal structural requirements for primality");
    println!();

    println!("4. UNIVERSAL PATTERNS:");
    println!("   Seeds that universally fail/succeed across configurations");
    println!("   represent fundamental mathematical properties");
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("RECOMMENDATIONS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("1. AVOID DARK ZONES:");
    println!("   Skip seeds in identified failure clusters");
    println!("   Focus computational effort on bright zones");
    println!();

    println!("2. TEST MODULAR HYPOTHESES:");
    println!("   Investigate high-variance modular patterns");
    println!("   May reveal base-specific divisibility rules");
    println!();

    println!("3. EXPLOIT DIGIT PROPERTIES:");
    println!("   Preferentially test seeds with success-correlated properties");
    println!("   Avoid seeds with failure-correlated properties");
    println!();

    println!("4. EXPAND SEED RANGE:");
    println!("   Test larger seed ranges to confirm patterns");
    println!("   Look for periodicities in failure/success regions");
    println!();
}
