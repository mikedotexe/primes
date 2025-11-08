//! Unit tests for prime number utilities

use prime_physics_engine::{is_prime, is_prime_miller_rabin};
use num_bigint::BigUint;

#[test]
fn test_small_primes() {
    let known_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    
    for p in known_primes {
        let big_p = BigUint::from(p as u32);
        assert!(is_prime(&big_p), "{} should be prime", p);
        assert!(is_prime_miller_rabin(&big_p), "{} should pass Miller-Rabin", p);
    }
}

#[test]
fn test_small_composites() {
    let known_composites = vec![4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21];
    
    for c in known_composites {
        let big_c = BigUint::from(c as u32);
        assert!(!is_prime(&big_c), "{} should not be prime", c);
        assert!(!is_prime_miller_rabin(&big_c), "{} should fail Miller-Rabin", c);
    }
}

#[test]
fn test_edge_cases() {
    // Test 0 and 1
    assert!(!is_prime(&BigUint::from(0u32)), "0 is not prime");
    assert!(!is_prime(&BigUint::from(1u32)), "1 is not prime");
    
    // Test 2 (only even prime)
    assert!(is_prime(&BigUint::from(2u32)), "2 is prime");
}

#[test]
fn test_large_known_primes() {
    // Some larger known primes
    let large_primes = vec![
        "7919",      // 1000th prime
        "104729",    // 10000th prime
        "1299709",   // 100000th prime
    ];
    
    for p_str in large_primes {
        let p = p_str.parse::<BigUint>().expect("Valid number string");
        assert!(is_prime(&p), "{} should be prime", p_str);
    }
}

#[test]
fn test_carmichael_numbers() {
    // Carmichael numbers are composites that pass certain primality tests
    let carmichael = vec!["561", "1105", "1729"];
    
    for c_str in carmichael {
        let c = c_str.parse::<BigUint>().expect("Valid number string");
        // Miller-Rabin with sufficient rounds should correctly identify these as composite
        assert!(!is_prime_miller_rabin(&c), "{} is a Carmichael number (composite)", c_str);
    }
}

#[test]
fn test_membrane_generated_primes() {
    // Test some primes we've discovered through membrane generation
    let membrane_primes = vec![
        "37573",       // Basic (3,7) membrane with seed 5
        "144000248687", // From find_large_primes example
    ];
    
    for p_str in membrane_primes {
        let p = p_str.parse::<BigUint>().expect("Valid number string");
        assert!(is_prime(&p), "{} should be prime (membrane generated)", p_str);
    }
}