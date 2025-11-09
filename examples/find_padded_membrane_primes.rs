//! Find small membrane primes with actual zero-padding

use num_bigint::BigUint;
use primes::is_prime;
use std::str::FromStr;

fn main() {
    println!("🔍 Finding small membrane primes with symmetric zero-padding");
    println!("{}", "=".repeat(60));
    println!();

    // Look for patterns like X0Y0X where X and Y are single digits
    println!("Pattern: X0Y0X (symmetric with one zero on each side)");
    for x in 1..=9 {
        for y in 1..=9 {
            let membrane = format!("{}0{}0{}", x, y, x);
            if let Ok(num) = BigUint::from_str(&membrane) {
                if is_prime(&num) {
                    println!("✓ {} is PRIME! Structure: {}-0-{}-0-{}", membrane, x, y, x);
                }
            }
        }
    }

    println!("\nPattern: X00Y00X (symmetric with two zeros on each side)");
    for x in 1..=9 {
        for y in 1..=9 {
            let membrane = format!("{}00{}00{}", x, y, x);
            if let Ok(num) = BigUint::from_str(&membrane) {
                if is_prime(&num) {
                    println!(
                        "✓ {} is PRIME! Structure: {}-00-{}-00-{}",
                        membrane, x, y, x
                    );
                }
            }
        }
    }

    println!("\nPattern: XY0Z0YX (double-digit boundaries with zeros)");
    for x in 1..=3 {
        for y in 1..=9 {
            for z in 1..=9 {
                let membrane = format!("{}{}0{}0{}{}", x, y, z, y, x);
                if let Ok(num) = BigUint::from_str(&membrane) {
                    if is_prime(&num) {
                        println!(
                            "✓ {} is PRIME! Structure: {}{}-0-{}-0-{}{}",
                            membrane, x, y, z, y, x
                        );
                    }
                }
            }
        }
    }

    println!("\nPattern: X0Y0Y0X (symmetric with repeated middle)");
    for x in 1..=9 {
        for y in 1..=9 {
            let membrane = format!("{}0{}0{}0{}", x, y, y, x);
            if let Ok(num) = BigUint::from_str(&membrane) {
                if is_prime(&num) {
                    println!(
                        "✓ {} is PRIME! Structure: {}-0-{}-0-{}-0-{}",
                        membrane, x, y, y, x
                    );
                }
            }
        }
    }

    // Look for slightly larger ones with more zeros
    println!("\nPattern: X000Y000X (three zeros on each side)");
    for x in 1..=9 {
        for y in 1..=9 {
            let membrane = format!("{}000{}000{}", x, y, x);
            if let Ok(num) = BigUint::from_str(&membrane) {
                if is_prime(&num) {
                    println!(
                        "✓ {} is PRIME! Structure: {}-000-{}-000-{}",
                        membrane, x, y, x
                    );
                }
            }
        }
    }
}
