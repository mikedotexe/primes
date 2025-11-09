//! Base 6 Investigation
//!
//! Why aren't we finding base 6 primes with the expected seeds?

use num_bigint::BigUint;
use primes::{is_prime, MembraneConfig};

fn main() {
    println!("🔍 Base 6 Investigation");
    println!("{}", "=".repeat(60));
    println!();

    // The claimed champion: Base 6 (1,5) k=(0,0)
    let config = MembraneConfig::new(6, 1, 5, 0, 0);

    println!(
        "Configuration: Base {} ({},{}) k=({},{})",
        config.base, config.outer, config.inner, config.k_outer, config.k_inner
    );
    println!();

    // Test single-digit seeds
    println!("Testing single-digit seeds (0-5 in base 6):");
    for seed in 0..6 {
        let base6_str = format!("1{}5{}1", seed, seed);
        println!("\nSeed {}: Base-6 string = {}", seed, base6_str);

        // Convert from base 6 to decimal
        let decimal = convert_from_base(&base6_str, 6);
        println!("  Decimal: {}", decimal);
        println!("  Is prime: {}", if is_prime(&decimal) { "✓" } else { "✗" });

        // Also try the membrane construction
        if let Ok(num) = config.construct_number(seed) {
            println!(
                "  Via construct_number: {} {}",
                num,
                if is_prime(&num) { "✓" } else { "✗" }
            );
        }
    }

    // Let's also check what the correct format might be
    println!("\n\nChecking different interpretations:");

    // Maybe it's 15[seed]51?
    println!("\nFormat: 15[seed]51");
    for seed in 0..6 {
        let base6_str = format!("15{}51", seed);
        let decimal = convert_from_base(&base6_str, 6);
        println!(
            "Seed {}: {} (base 6) = {} (decimal) → {}",
            seed,
            base6_str,
            decimal,
            if is_prime(&decimal) {
                "✓ PRIME"
            } else {
                "✗"
            }
        );
    }

    // Or maybe with zero padding?
    println!("\nFormat: 1005[seed]5001");
    for seed in 0..6 {
        let base6_str = format!("1005{}5001", seed);
        let decimal = convert_from_base(&base6_str, 6);
        println!(
            "Seed {}: {} (base 6) = {} (decimal) → {}",
            seed,
            base6_str,
            decimal,
            if is_prime(&decimal) {
                "✓ PRIME"
            } else {
                "✗"
            }
        );
    }

    // Check the verified example from EVIDENCE.md
    println!("\n\nChecking verified example:");
    let verified = "15651"; // From evidence: Base 6, seed 6
    let decimal = convert_from_base(verified, 6);
    println!("Verified: {} (base 6) = {} (decimal)", verified, decimal);
    println!("Is prime: {}", if is_prime(&decimal) { "✓" } else { "✗" });

    // Aha! Seed 6 is not a single digit in base 6!
    // In base 6, digits are 0-5, so "seed 6" means "10" in base 6
    println!("\n\nREALIZATION: In base 6, valid digits are 0-5!");
    println!("'Seed 6' in the evidence likely means the 6th seed tested (index 6),");
    println!("not the digit 6 (which doesn't exist in base 6)!");

    // Test with proper base-6 thinking
    println!("\n\nProper base-6 membrane construction:");
    for i in 0..10 {
        // Convert seed index to base-6 representation
        let seed_base6 = format!("{}", to_base_6(i));
        let membrane = format!("15{}51", seed_base6);

        if let Some(decimal) = BigUint::parse_bytes(membrane.as_bytes(), 6) {
            println!(
                "Index {}: {} (base 6) = {} → {}",
                i,
                membrane,
                decimal,
                if is_prime(&decimal) {
                    "✓ PRIME"
                } else {
                    "✗"
                }
            );
        }
    }
}

fn convert_from_base(s: &str, base: u32) -> BigUint {
    BigUint::parse_bytes(s.as_bytes(), base).unwrap_or(BigUint::from(0u32))
}

fn to_base_6(mut n: usize) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    while n > 0 {
        result.insert(0, char::from_digit((n % 6) as u32, 10).unwrap());
        n /= 6;
    }
    result
}
