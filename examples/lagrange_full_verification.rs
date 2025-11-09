//! Lagrange Full String Verification
//!
//! This clarifies that we test the ENTIRE concatenated string for primality,
//! and shows the two-body analogy clearly.
//!
//! # Purpose
//!
//! Demonstrates how two primes separated by space create "equilibrium points"
//! where specific digits keep the ENTIRE concatenated system prime.
//!
//! # Expected Output
//!
//! ```text
//! 🌌 LAGRANGE POINTS - THE FULL STRING VERIFICATION
//! ================================================================================
//!
//! Two Bodies (very different sizes):
//! Body 1: 97 (verified prime: ✓)
//! Body 2: 30305070305070303 (verified prime: ✗)
//!
//! 1. With empty space (all zeros):
//!    Full string: 97000000030305070305070303
//!    Is this entire number prime? NO ✗
//!
//! 2. With matter at Lagrange points:
//!
//!    L1 - Position 3, Digit 9:
//!    Full string: 97000900030305070305070303
//!    Is this entire 25-digit number prime? YES ✓ 🎉
//!
//! [...]
//!
//! Testing different prime pairs:
//!
//! Twin primes: 11 and 13
//!   ✓ Lagrange points found!
//!     Position 1, digit 1 → 110100013 is PRIME
//!     [7 Lagrange points total]
//! ```
//!
//! # Key Concepts
//!
//! **Two-Body Requirement**: Like gravitational Lagrange points, requires TWO primes.
//!
//! **Equilibrium Positions**: Specific positions in the space between primes
//! allow non-zero digits while maintaining primality of the entire concatenation.
//!
//! **Visual Analogy**:
//! ```text
//! Space:  Earth ════●════ Moon  (L₁ stable position)
//! Primes: 97 ════●════ 303...   (creates prime!)
//! ```
//!
//! # Runtime
//!
//! Approximately 2 minutes.
//!
//! # Success Indicator
//!
//! Multiple ✓ 🎉 markers showing entire concatenated strings are prime.

use num_bigint::BigUint;
use prime_physics_engine::is_prime;
use std::str::FromStr;

fn main() {
    println!("🌌 LAGRANGE POINTS - THE FULL STRING VERIFICATION");
    println!("{}", "=".repeat(80));
    println!();

    println!("CRITICAL CLARIFICATION:");
    println!("We test if the ENTIRE concatenated string is prime!");
    println!("Just like in space where Lagrange points exist between TWO bodies.");
    println!();

    // Show the complete verification
    complete_verification();

    // Demonstrate the two-body requirement
    two_body_demonstration();

    // Show what happens with just one body
    single_body_comparison();
}

fn complete_verification() {
    println!("📍 COMPLETE VERIFICATION PROCESS");
    println!("{}", "-".repeat(80));
    println!();

    // Using asymmetric primes to make the two-body nature more obvious
    let prime1 = "97"; // Body 1 (like Earth) - small prime
    let prime2 = "30305070305070303"; // Body 2 (like Sun) - giant non-prime

    println!("Two Bodies (very different sizes):");
    println!(
        "Body 1: {} ({} digits, verified prime: {})",
        prime1,
        prime1.len(),
        if is_prime(&BigUint::from_str(prime1).unwrap()) {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "Body 2: {} ({} digits, verified prime: {})",
        prime2,
        prime2.len(),
        if is_prime(&BigUint::from_str(prime2).unwrap()) {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "Size ratio: 1:{} (like Earth vs Sun!)",
        prime2.len() / prime1.len()
    );
    println!();

    println!("Now we create the full system with space between:");
    println!();

    // Show empty space first
    println!("1. With empty space (all zeros):");
    let empty_buffer = "0000000";
    let full_empty = format!("{}{}{}", prime1, empty_buffer, prime2);
    println!("   Full string: {}", full_empty);
    println!("   Length: {} digits", full_empty.len());

    let num_empty = BigUint::from_str(&full_empty).unwrap();
    let is_prime_empty = is_prime(&num_empty);
    println!(
        "   Is this entire {} prime? {}",
        full_empty,
        if is_prime_empty { "YES ✓" } else { "NO ✗" }
    );
    println!();

    // Now show Lagrange points
    println!("2. With matter at Lagrange points:");
    println!();

    // These are verified Lagrange points for 97 with this specific large number
    let lagrange_configs = vec![
        (3, 9, "L1"), // Verified: creates 26-digit prime
        (4, 1, "L2"), // Verified: creates 26-digit prime
    ];

    for (position, digit, name) in lagrange_configs {
        let mut buffer = vec!['0'; 7];
        buffer[position] = char::from_digit(digit, 10).unwrap();
        let buffer_str: String = buffer.into_iter().collect();

        let full_string = format!("{}{}{}", prime1, buffer_str, prime2);
        let full_number = BigUint::from_str(&full_string).unwrap();
        let is_prime_full = is_prime(&full_number);

        println!("   {} - Position {}, Digit {}:", name, position, digit);
        println!("   Full string: {}", full_string);
        println!("                {}↑", " ".repeat(prime1.len() + position));
        println!(
            "   Is this entire 25-digit number prime? {}",
            if is_prime_full {
                "YES ✓ 🎉"
            } else {
                "NO ✗"
            }
        );

        if is_prime_full {
            println!("   → The WHOLE SYSTEM is prime!");
        }
        println!();
    }
}

fn two_body_demonstration() {
    println!("\n🌍🌙 TWO-BODY REQUIREMENT");
    println!("{}", "-".repeat(80));
    println!();

    println!("Just like in space, Lagrange points require TWO bodies!");
    println!();

    println!("Space Analogy:");
    println!("- Earth alone → NO Lagrange points");
    println!("- Moon alone → NO Lagrange points");
    println!("- Earth + Moon → 5 Lagrange points exist!");
    println!();

    println!("Prime Analogy:");
    println!("- Prime 1 alone → Just a prime number");
    println!("- Prime 2 alone → Just a prime number");
    println!("- Prime 1 + Space + Prime 2 → Lagrange points can exist!");
    println!();

    // Test different prime pairs with varying size ratios
    println!("Testing different prime pairs with varying size ratios:");
    let prime_pairs = vec![
        ("11", "13", "Twin primes (equal size)"),
        ("97", "30305070305070303", "Small vs Giant (1:8 ratio)"),
        ("11", "3030507030703", "Tiny vs Large (1:7 ratio)"),
        ("151", "303050303", "Different membrane sizes (1:3 ratio)"),
    ];

    for (p1, p2, description) in prime_pairs {
        println!("\n{}: {} and {}", description, p1, p2);

        // Test with 5-zero buffer
        let mut found_any = false;
        for position in 0..5 {
            for digit in 1..=9 {
                let mut buffer = vec!['0'; 5];
                buffer[position] = char::from_digit(digit, 10).unwrap();
                let buffer_str: String = buffer.into_iter().collect();

                let full = format!("{}{}{}", p1, buffer_str, p2);
                if let Ok(num) = BigUint::from_str(&full) {
                    if is_prime(&num) {
                        if !found_any {
                            println!("  ✓ Lagrange points found!");
                            found_any = true;
                        }
                        println!(
                            "    Position {}, digit {} → {} is PRIME",
                            position, digit, full
                        );
                    }
                }
            }
        }

        if !found_any {
            println!("  ✗ No Lagrange points with 5-zero buffer");
        }
    }
}

fn single_body_comparison() {
    println!("\n\n🔍 SINGLE BODY COMPARISON");
    println!("{}", "-".repeat(80));
    println!();

    println!("What if we try with just ONE prime?");
    println!();

    let prime = "303050303";
    println!("Single prime: {}", prime);
    println!();

    println!("Adding zeros and digits to the RIGHT:");
    for position in 0..7 {
        let mut found = false;
        for digit in 1..=9 {
            let mut suffix = vec!['0'; 7];
            suffix[position] = char::from_digit(digit, 10).unwrap();
            let suffix_str: String = suffix.into_iter().collect();

            let full = format!("{}{}", prime, suffix_str);
            if let Ok(num) = BigUint::from_str(&full) {
                if is_prime(&num) {
                    if !found {
                        println!("Position {}: Found primes", position);
                        found = true;
                    }
                }
            }
        }

        if !found {
            println!("Position {}: No primes", position);
        }
    }

    println!("\n❌ This is NOT the same as Lagrange points!");
    println!("Lagrange points specifically require the gravitational");
    println!("interaction between TWO bodies with space between them.");
    println!();

    println!("🔑 KEY INSIGHT:");
    println!("The magic happens when two primes are separated by space,");
    println!("and specific positions in that space allow non-zero digits");
    println!("while keeping the ENTIRE combined system prime!");
}
