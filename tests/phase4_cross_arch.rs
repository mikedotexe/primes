//! Cross-architecture tests for Phase 4
//! Tests that work on non-ARM architectures (x86, etc)

#![cfg(feature = "phase4")]

use primes::phase4::{OnChipRL, PmuDoubleBuffer, PmuSnapshot, SlcResident};
use primes::prime_sieve::{segmented_sieve, warm_slc};

#[test]
fn rl_controller_works_cross_arch() {
    let mut rl = OnChipRL::new();

    // Feed training samples
    for i in 0..1000 {
        let pmu_sample = (i % 16) as u8;
        let latency = 5 + (i % 10) as u32;
        rl.tick(pmu_sample, latency);
    }

    // Should have learned something
    assert!(
        rl.has_learned(),
        "RL should have non-zero Q-values after training"
    );
}

#[test]
fn pmu_buffer_works_cross_arch() {
    let buffer = PmuDoubleBuffer::new();

    // Write and read snapshots
    for i in 1..=10 {
        let snapshot = PmuSnapshot {
            l1_miss: (i * 10) as u16,
            cycles: (i * 100) as u32,
            ts: i as u64,
        };
        buffer.write(snapshot);

        let read = buffer.read();
        assert_eq!(read.ts, i as u64, "Should read back what was written");
    }
}

#[test]
fn prime_sieve_works_cross_arch() {
    // Test basic sieve functionality
    let primes = segmented_sieve(1000, 65536);
    assert_eq!(primes.len(), 168, "Should find 168 primes below 1000");

    // Test cache warming
    warm_slc(10_000, 0.1);
}

#[test]
fn slc_controller_basic() {
    let mut slc = SlcResident::new(0.8);
    let dummy_data = vec![0u8; 4096];

    // Low warmth should trigger maintenance
    // SAFETY: dummy_data is valid for its entire length and lives long enough
    unsafe {
        slc.maintain_residency(0.5, dummy_data.as_ptr(), dummy_data.len());
    }

    // High warmth should not
    // SAFETY: dummy_data is valid for its entire length and lives long enough
    unsafe {
        slc.maintain_residency(0.9, dummy_data.as_ptr(), dummy_data.len());
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[test]
fn sme_fallback_works() {
    // On non-ARM, the SME prediction should still work via fallback
    let x = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0];
    let result = primes::phase4::predict_sme_padded_safe(x);
    assert_eq!(result, 36, "Should sum first 8 elements: 1+2+3+4+5+6+7+8");
}
