//! Find large primes using membrane configurations

use primes::is_prime_miller_rabin;
use num_bigint::BigUint;
use std::time::Instant;
fn main() {
    println!("=== Finding Large Primes ===\n");
    
    // Use base-12 for high density
    let base = 12;
    let (l, r) = (11, 11);
    let width = 5; // Larger width for bigger primes
    println!("Configuration: base-{}, width={}, boundary=({},{})", base, width, l, r);
    println!("Searching for primes with ~50 decimal digits...\n");
    let start = Instant::now();
    let mut found = 0;
    let target = 5;
    // Start with larger seeds for bigger primes
    let mut c = 1_000_000_000u64;
    while found < target {
        let value = compute_membrane(base, width, l, r, 0, 0, c);
        
        if is_prime_miller_rabin(&value) {
            found += 1;
            let digits = value.to_string().len();
            println!("Prime #{}: {} digits", found, digits);
            println!("  Seed: C={}", c);
            println!("  Value: {}", value);
            println!("  Wolfram: https://www.wolframalpha.com/input?i=isprime+{}", value);
            println!();
        }
        c += 1;
        // Progress indicator
        if c % 10000 == 0 {
            print!(".");
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        }
    }
    let elapsed = start.elapsed();
    println!("\nFound {} large primes in {:.2}s", found, elapsed.as_secs_f64());
    println!("Average time per prime: {:.2}s", elapsed.as_secs_f64() / found as f64);
}
fn compute_membrane(base: u32, w: u32, l: u32, r: u32, r1: u32, r2: u32, c: u64) -> BigUint {
    let b = BigUint::from(base);
    let l = BigUint::from(l);
    let r = BigUint::from(r);
    let c = BigUint::from(c);
    &l * b.pow(w - 1) +
    &r * b.pow(w - 2 - r1) +
    &c * b.pow(w / 2) +
    &r * b.pow(r2 + 1) +
    &l
}
