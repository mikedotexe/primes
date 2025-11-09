//! Deterministic prime count smoke tests for release verification
//!
//! These tests verify that our prime generation algorithms produce
//! the expected exact counts for well-known limits. Any deviation
//! indicates a serious bug that must be fixed before release.
//!
//! # Purpose
//!
//! Validates core primality testing functionality up to 10 million.
//!
//! # Expected Output
//!
//! ```text
//! 🔍 Running deterministic prime count smoke tests...
//!
//! Testing π(100) = 25 ... ✅ PASS
//! Testing π(1000) = 168 ... ✅ PASS
//! Testing π(10000) = 1229 ... ✅ PASS
//! Testing π(100000) = 9592 ... ✅ PASS
//! Testing π(1000000) = 78498 ... ✅ PASS
//! Testing π(10000000) = 664579 ... ✅ PASS
//!
//! 🧪 Testing individual large prime verification...
//! Testing 2147483647 ... ✅ PRIME
//! Testing 1073741827 ... ✅ PRIME
//! [...]
//!
//! 🎉 All smoke tests PASSED! ✅
//!    Prime counting algorithms are functioning correctly.
//! ```
//!
//! # Runtime
//!
//! Approximately 30 seconds on modern hardware.
//!
//! # Success Indicator
//!
//! All tests should show ✅ PASS. Any ❌ FAIL indicates a critical bug.

use num_bigint::BigUint;
use prime_physics_engine::{is_prime, BitSieve};

fn main() {
    println!("🔍 Running deterministic prime count smoke tests...\n");

    // Test cases with verified prime counts from OEIS A000720
    let test_cases = vec![
        (100, 25),
        (1_000, 168),
        (10_000, 1_229),
        (100_000, 9_592),
        (1_000_000, 78_498),
        (10_000_000, 664_579),
    ];

    let mut all_passed = true;

    for (limit, expected_count) in test_cases {
        print!("Testing π({}) = {} ... ", limit, expected_count);

        let sieve = BitSieve::new(limit);
        let actual_count = sieve.primes().len();

        if actual_count == expected_count {
            println!("✅ PASS");
        } else {
            println!("❌ FAIL (got {})", actual_count);
            all_passed = false;
        }
    }

    println!();

    // Additional verification: test a few known large primes individually
    let large_primes = vec![
        2147483647, // 2^31 - 1 (Mersenne prime)
        1073741827, // Large prime
        536870923,  // Large prime
        268435459,  // Large prime
        134217757,  // Large prime
    ];

    println!("🧪 Testing individual large prime verification...");
    for prime in large_primes {
        print!("Testing {} ... ", prime);
        if is_prime(&BigUint::from(prime as u64)) {
            println!("✅ PRIME");
        } else {
            println!("❌ NOT PRIME");
            all_passed = false;
        }
    }

    println!();

    if all_passed {
        println!("🎉 All smoke tests PASSED! ✅");
        println!("   Prime counting algorithms are functioning correctly.");
        std::process::exit(0);
    } else {
        println!("💥 Some tests FAILED! ❌");
        println!("   DO NOT RELEASE - investigate prime counting bugs!");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_counts() {
        // Core counts that must never change
        assert_eq!(BitSieve::new(1000).primes().len(), 168);
        assert_eq!(BitSieve::new(10000).primes().len(), 1229);
        assert_eq!(BitSieve::new(100000).primes().len(), 9592);
    }

    #[test]
    fn test_known_primes() {
        // Test some well-known primes
        assert!(is_prime(&BigUint::from(97u64)));
        assert!(is_prime(&BigUint::from(2147483647u64))); // 2^31 - 1 (Mersenne prime)
        assert!(is_prime(&BigUint::from(982451653u64)));
    }

    #[test]
    fn test_known_composites() {
        // Test some well-known composites
        assert!(!is_prime(&BigUint::from(100u64)));
        assert!(!is_prime(&BigUint::from(2147483646u64))); // 2^31 - 2
        assert!(!is_prime(&BigUint::from(982451652u64)));
    }
}
