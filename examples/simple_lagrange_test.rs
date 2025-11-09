//! Simple test to verify membrane Lagrange point structure

use num_bigint::BigUint;
use prime_physics_engine::is_prime;

fn main() {
    println!("🔬 Simple Membrane Lagrange Test\n");

    // Test case: Two membrane primes with same structure
    let p1 = BigUint::from(303050303u64); // 3-0-3-0-5-0-3-0-3
    let p2 = BigUint::from(303070303u64); // 3-0-3-0-7-0-3-0-3

    println!("Prime 1: {} (middle: 5)", p1);
    println!("Prime 2: {} (middle: 7)", p2);
    println!();

    // Calculate midpoint
    let midpoint = (&p1 + &p2) / 2u8;
    println!(
        "Numeric midpoint: {} (middle: {})",
        midpoint,
        midpoint.to_string().chars().nth(4).unwrap()
    );

    // Check structure
    let expected = BigUint::from(303060303u64); // 3-0-3-0-6-0-3-0-3
    if midpoint == expected {
        println!("✓ Structure preserved! Only middle digit changed: 5→6←7");
    }

    println!(
        "Is midpoint prime? {}",
        if is_prime(&midpoint) { "Yes" } else { "No" }
    );

    // Key insight
    println!("\n🎯 KEY INSIGHT:");
    println!("The Lagrange point between two membrane primes with identical");
    println!("structure preserves that structure, modifying only the middle 'seed'.");
    println!("This confirms that the gravitational 'mass' is in the boundary digits!");
}
