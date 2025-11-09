//! Simple command-line prime checker
//!
//! Usage: echo "12345" | cargo run --example check_prime

use num_bigint::BigUint;
use prime_physics_engine::is_prime;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    println!("Prime Checker - Enter numbers to check (Ctrl+D to exit):");
    println!("You can also pipe: echo '151' | cargo run --example check_prime\n");

    for line in stdin.lock().lines() {
        if let Ok(num_str) = line {
            let num_str = num_str.trim();
            if num_str.is_empty() {
                continue;
            }

            match BigUint::parse_bytes(num_str.as_bytes(), 10) {
                Some(num) => {
                    let result = is_prime(&num);

                    if result {
                        println!("{} ✓ PRIME", num_str);
                    } else {
                        // Find small factors
                        let mut factors = Vec::new();
                        for p in &[2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47] {
                            if &num % p == 0u32.into() && num != (*p).into() {
                                factors.push(*p);
                            }
                        }

                        if factors.is_empty() {
                            println!("{} ✗ COMPOSITE", num_str);
                        } else {
                            println!("{} ✗ COMPOSITE (divisible by {:?})", num_str, factors);
                        }
                    }
                }
                None => {
                    println!("Error: '{}' is not a valid number", num_str);
                }
            }
        }
    }
}
