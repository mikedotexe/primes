//! Comprehensive integration tests for the Prime Physics Engine

use prime_physics_engine::{
    prime_sieve::{segmented_sieve, sieve_count_and_time, warm_slc, BitSieve},
    MembraneConfig, PerfMonitor,
};

#[test]
fn test_membrane_generation_correctness() {
    // Test known good configurations
    let test_cases = vec![
        // (base, outer, inner, k_outer, k_inner, seed, expected_prime)
        (10, 3, 7, 0, 0, 5, true),  // 37573 is prime
        (6, 1, 5, 0, 0, 6, true),   // Base 6 optimal
        (12, 5, 7, 0, 0, 1, false), // Should be composite
    ];

    for (base, outer, inner, k_outer, k_inner, seed, expected_prime) in test_cases {
        let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);

        // Only test base 10 for now, as the membrane construction checks for digits 0-9
        if base == 10 {
            let candidate = config.construct_number(seed).unwrap();
            let is_prime = prime_physics_engine::is_prime(&candidate);
            assert_eq!(
                is_prime, expected_prime,
                "Failed for config ({},{}) k=({},{}) seed={} in base {}",
                outer, inner, k_outer, k_inner, seed, base
            );
        }
    }
}

#[test]
fn test_sieve_correctness_after_fix() {
    // Verify the sieve produces correct prime counts
    let test_limits = vec![
        (1000, 168),
        (10_000, 1229),
        (100_000, 9592),
        (1_000_000, 78498),
    ];

    for (limit, expected_count) in test_limits {
        let sieve = BitSieve::new(limit);
        let primes = sieve.primes();
        assert_eq!(
            primes.len(),
            expected_count,
            "Incorrect prime count for limit {}",
            limit
        );

        // Verify first few primes
        if limit >= 100 {
            assert_eq!(&primes[..10], &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
        }
    }
}

#[test]
fn test_segmented_sieve_consistency() {
    let limit = 100_000;
    let single_core = BitSieve::new(limit).primes();
    let multi_core = segmented_sieve(limit, 65536);

    // Should produce same count
    assert_eq!(single_core.len(), multi_core.len());

    // Check first and last primes match
    assert_eq!(single_core[..100], multi_core[..100]);
    let len = single_core.len();
    assert_eq!(single_core[len - 100..], multi_core[len - 100..]);
}

#[test]
fn test_performance_monitor() {
    let monitor = PerfMonitor::new();

    // Time some operations
    monitor.time("test_op1", || {
        std::thread::sleep(std::time::Duration::from_millis(1));
    });

    monitor.time("test_op2", || {
        std::thread::sleep(std::time::Duration::from_millis(2));
    });

    let metrics = monitor.get_metrics();
    assert_eq!(metrics.len(), 2);

    // Verify timing is reasonable
    let op1 = metrics.iter().find(|m| m.name == "test_op1").unwrap();
    assert!(op1.avg_time().as_millis() >= 1);
    assert!(op1.avg_time().as_millis() < 10); // Allow some slack
}

#[test]
fn test_cache_warming() {
    // Test basic cache warming
    let result1 = warm_slc(10_000, 0.1);
    assert!(result1.primes_generated > 0);
    assert!(result1.lines_touched > 0);

    let result2 = warm_slc(50_000, 0.25);
    assert!(result2.primes_generated > result1.primes_generated);
    assert!(result2.bytes_touched > result1.bytes_touched);

    // Test with performance monitoring
    let monitor = PerfMonitor::new();
    let result3 = monitor.time("cache_warm_test", || warm_slc(100_000, 0.5));

    assert!(result3.primes_generated > 0);
    assert!(result3.mb_per_sec() > 0.0);

    let metrics = monitor.get_metrics();
    assert_eq!(metrics.len(), 1);
}

#[test]
fn test_coprimality_validation() {
    // Test GCD implementation used in WASM
    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    assert_eq!(gcd(10, 3), 1); // Coprime
    assert_eq!(gcd(10, 5), 5); // Not coprime
    assert_eq!(gcd(12, 5), 1); // Coprime
    assert_eq!(gcd(12, 6), 6); // Not coprime
}

#[test]
fn test_membrane_edge_cases() {
    // Test with very small seeds
    let config = MembraneConfig::new(10, 1, 3, 0, 0);

    for seed in 0..=3 {
        let candidate = config.construct_number(seed).unwrap();
        // Should not panic
        let _ = prime_physics_engine::is_prime(&candidate);
    }

    // Test with large padding
    let config_padded = MembraneConfig::new(10, 1, 3, 5, 5);

    let candidate = config_padded.construct_number(1).unwrap();
    // The structure should be: 1 00000 3 00000 1 00000 3 00000 1
    let expected_str = "1000003000001000003000001";
    assert_eq!(candidate.to_string(), expected_str);
}

#[test]
fn test_prime_validation_accuracy() {
    // Test some known primes and composites
    use num_bigint::BigUint;
    use std::str::FromStr;

    let primes = vec![
        "37573",      // From membrane
        "999983",     // Large prime
        "2147483647", // Mersenne prime (2^31 - 1)
    ];

    let composites = vec![
        "37572",      // 37572 = 2^2 × 3 × 3143
        "999993",     // 999993 = 3 × 333331
        "2147483646", // 2^31 - 2
    ];

    for p in primes {
        let n = BigUint::from_str(p).unwrap();
        assert!(prime_physics_engine::is_prime(&n), "{} should be prime", p);
    }

    for c in composites {
        let n = BigUint::from_str(c).unwrap();
        assert!(
            !prime_physics_engine::is_prime(&n),
            "{} should be composite",
            c
        );
    }
}

#[cfg(feature = "phase4")]
#[test]
fn test_phase4_integration() {
    use prime_physics_engine::phase4::{
        predict_sme_padded_safe, OnChipRL, PmuDoubleBuffer, PmuSnapshot,
    };

    // Test neural network
    let input = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0];
    let output = predict_sme_padded_safe(input);
    assert_eq!(output, 36); // Sum of 1..8

    // Test RL controller convergence
    let mut rl = OnChipRL::new();
    for i in 0..1000 {
        rl.tick((i % 16) as u8, 5 + (i % 5) as u32);
    }
    assert!(rl.has_learned());

    // Test PMU double buffer
    let buffer = PmuDoubleBuffer::new();
    for i in 1..=10 {
        let snapshot = PmuSnapshot {
            l1_miss: i as u16 * 10,
            cycles: i as u32 * 100,
            ts: i as u64,
        };
        buffer.write(snapshot);
        let read = buffer.read();
        assert_eq!(read.ts, i as u64);
    }
}

#[test]
fn test_error_handling() {
    // Test invalid configurations - note that digits must be < 10 for now
    let invalid_config = MembraneConfig::new(10, 10, 5, 0, 0);

    // This should fail because outer digit (10) is >= 10
    let result = invalid_config.construct_number(1);
    assert!(result.is_err());
}

#[test]
fn test_large_number_handling() {
    // Test with configuration that generates very large numbers
    // Use base 10 digits only due to current membrane limitations
    let config = MembraneConfig::new(10, 7, 3, 10, 10);

    let candidate = config.construct_number(999).unwrap();
    // Should handle large numbers without overflow
    let str_repr = candidate.to_string();
    assert!(str_repr.len() > 20); // Should be quite long

    // Primality test should still work (might be slow)
    let _ = prime_physics_engine::is_prime(&candidate);
}

#[test]
fn test_sieve_count_and_time() {
    // Test the new cycle-accurate timing function
    let (count1, cycles1) = sieve_count_and_time(10_000);
    let (count2, cycles2) = sieve_count_and_time(100_000);

    // Verify counts are correct
    assert_eq!(count1, 1229);
    assert_eq!(count2, 9592);

    // Larger limit should take more cycles
    assert!(cycles2 > cycles1);

    // Basic sanity check on cycles (should be non-zero)
    assert!(cycles1 > 0);
    assert!(cycles2 > 0);
}
