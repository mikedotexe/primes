//! Fix the base-6 example error
//!
//! The documentation claims 15651 (base 6) = 2551 (decimal)
//! But 15651 isn't valid in base 6!

use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};
use primes::is_prime;

fn main() {
    println!("Investigating the base-6 example error...\n");

    // The claim: 15651 (base 6) = 2551 (decimal)

    // First problem: 15651 contains digit '6' which isn't valid in base 6!
    println!("PROBLEM 1: '15651' contains digit '6', which is invalid in base 6!");
    println!("Valid digits in base 6: 0, 1, 2, 3, 4, 5\n");

    // Let's check what 15651 in decimal converts to in base 6
    println!(
        "15651 in decimal = {} in base 6",
        to_base_n(&15651u32.into(), 6)
    );

    // And check what 2551 in decimal is in base 6
    println!(
        "2551 in decimal = {} in base 6",
        to_base_n(&2551u32.into(), 6)
    );

    // Now let's manually construct membrane numbers in base 6
    println!("\nManually constructing (1,5) k=(0,0) membrane in base 6:");
    println!("Pattern: outer-inner-seed-inner-outer = 1-5-seed-5-1\n");

    for seed in 0..6 {
        // In base 6: 15X51 where X is the seed
        let base6_str = format!("15{}51", seed);

        // Convert to decimal
        let decimal_value = from_base_n(&base6_str, 6);
        let is_prime_result = is_prime(&decimal_value);

        println!(
            "Seed {}: {} (base 6) = {} (decimal) -> {}",
            seed,
            base6_str,
            decimal_value,
            if is_prime_result {
                "PRIME ✓"
            } else {
                "composite"
            }
        );
    }

    // What was probably meant
    println!("\nWhat was probably meant:");
    println!("The example should probably be: Seed 4 → 15451 (base 6) = 2551 (decimal)");
    println!("Is 2551 prime? {}", is_prime(&2551u32.into()));
}

fn from_base_n(s: &str, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    for ch in s.chars() {
        let digit = ch.to_digit(base).unwrap_or(0);
        result = result * &base_big + digit;
    }

    result
}

fn to_base_n(num: &BigUint, base: u32) -> String {
    if num.is_zero() {
        return "0".to_string();
    }

    let mut result = Vec::new();
    let mut n = num.clone();
    let base_big = BigUint::from(base);

    while !n.is_zero() {
        let digit = (&n % &base_big).to_u32().unwrap();
        result.push(std::char::from_digit(digit, base).unwrap());
        n /= &base_big;
    }

    result.reverse();
    result.into_iter().collect()
}
