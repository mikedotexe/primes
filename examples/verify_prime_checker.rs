//! Verify our prime checker is working correctly
//!
//! Tests known primes and composites to ensure accuracy

use num_bigint::BigUint;
use primes::is_prime;
use std::str::FromStr;

fn main() {
    println!("Testing prime checker accuracy...\n");

    // Test the problematic number
    let problem_num = BigUint::from_str("300700300703").unwrap();
    println!("Testing 300700300703:");
    println!("  is_prime: {}", is_prime(&problem_num));
    println!("  300700300703 % 11 = {}", &problem_num % 11u32);
    println!("  300700300703 / 11 = {}\n", &problem_num / 11u32);

    // Test some known primes
    let known_primes = vec![
        "2",
        "3",
        "5",
        "7",
        "11",
        "13",
        "17",
        "19",
        "23",
        "29",
        "97",
        "151",
        "10301",
        "30703",
        "1003001",
        "999999999989", // Large known prime
    ];

    println!("Testing known primes:");
    for p in &known_primes {
        let num = BigUint::from_str(p).unwrap();
        let result = is_prime(&num);
        println!("  {} -> {}", p, result);
        if !result {
            println!("    ERROR: {} should be prime!", p);
        }
    }

    // Test some known composites
    let known_composites = vec![
        ("4", vec![2u32]),
        ("6", vec![2, 3]),
        ("9", vec![3]),
        ("15", vec![3, 5]),
        ("21", vec![3, 7]),
        ("300700300703", vec![11]),
        ("1000000000000", vec![2, 5]),
    ];

    println!("\nTesting known composites:");
    for (c, factors) in &known_composites {
        let num = BigUint::from_str(c).unwrap();
        let result = is_prime(&num);
        println!("  {} -> {}", c, result);
        if result {
            println!("    ERROR: {} should be composite!", c);
        }
        // Show factors
        for f in factors {
            println!("    {} % {} = {}", c, f, &num % *f);
        }
    }

    // Test membrane patterns that might be problematic
    println!("\nTesting membrane patterns:");
    let patterns = vec![
        ("3-0-7-0-3", "30703"),               // This is prime
        ("3-00-7-00-3", "300700303"),         // Check this
        ("3-007-003-007-03", "300700300703"), // The problematic one
    ];

    for (pattern, num_str) in patterns {
        let num = BigUint::from_str(num_str).unwrap();
        let result = is_prime(&num);
        println!("  {} = {} -> {}", pattern, num_str, result);

        // Quick factor check for small primes
        for p in &[2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29] {
            if &num % p == 0u32.into() && &num != &(*p).into() {
                println!("    Divisible by {}", p);
            }
        }
    }
}
