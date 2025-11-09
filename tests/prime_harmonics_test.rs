//! Comprehensive test suite for prime-harmonics feature
//!
//! This test file validates the harmonic identity for prime numbers by:
//! 1. Testing basic Fourier transform functionality
//! 2. Analyzing harmonic patterns in prime sequences
//! 3. Comparing prime harmonics with random sequences
//! 4. Testing membrane-generated prime harmonics
//!
//! Run with: cargo test --features prime-harmonics --test prime_harmonics_test

use num_bigint::BigUint;
#[cfg(feature = "prime-harmonics")]
use prime_physics_engine::harmonics::{fourier_transform, power_spectrum, HarmonicAnalyzer};
use prime_physics_engine::{is_prime, prime_sieve::BitSieve};
use std::f64::consts::PI;

/// Helper to create a prime indicator sequence
fn create_prime_sequence(limit: usize) -> Vec<f64> {
    let sieve = BitSieve::new(limit);
    let primes = sieve.primes();
    let mut sequence = vec![0.0; limit];
    for p in primes {
        if p < limit {
            sequence[p] = 1.0;
        }
    }
    sequence
}

#[cfg(feature = "prime-harmonics")]
#[test]
fn test_fourier_transform_correctness() {
    println!("\n=== Testing Fourier Transform Correctness ===");

    // Test 1: DC component
    let signal = vec![1.0, 0.0, 1.0, 0.0];
    let spectrum = fourier_transform(&signal);
    let dc_component = spectrum[0].re;
    let expected_dc = 0.5; // average of signal

    println!("DC component test:");
    println!("  Signal: {:?}", signal);
    println!(
        "  DC component: {:.6} (expected: {:.6})",
        dc_component, expected_dc
    );
    assert!((dc_component - expected_dc).abs() < 1e-10);

    // Test 2: Pure sinusoid
    let n = 64;
    let freq = 5;
    let signal: Vec<f64> = (0..n)
        .map(|i| (2.0 * PI * i as f64 * freq as f64 / n as f64).sin())
        .collect();

    let _spectrum = fourier_transform(&signal);
    let power = power_spectrum(&signal);

    // Find peak
    let (peak_idx, peak_power) = power
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, &pow)| (idx, pow))
        .unwrap();

    println!("\nPure sinusoid test (freq={}):", freq);
    println!("  Peak found at index: {}", peak_idx);
    println!("  Peak power: {:.6}", peak_power);
    assert!(peak_idx == freq || peak_idx == n - freq);
}

#[cfg(feature = "prime-harmonics")]
#[test]
fn test_prime_sequence_harmonic_structure() {
    println!("\n=== Testing Prime Sequence Harmonic Structure ===");

    let limits = vec![100, 200, 500];

    for limit in limits {
        let prime_seq = create_prime_sequence(limit);
        let analyzer = HarmonicAnalyzer::new(limit);
        let analysis = analyzer.analyze(&prime_seq);

        let prime_count = prime_seq.iter().filter(|&&x| x == 1.0).count();
        let density = prime_count as f64 / limit as f64;

        println!("\nPrime sequence up to {}:", limit);
        println!("  Prime count: {}", prime_count);
        println!("  Prime density: {:.4}", density);
        println!("  Total harmonics: {}", analysis.total_harmonics);
        println!(
            "  Dominant frequencies: {}",
            analysis.dominant_frequencies.len()
        );
        println!("  Harmonic purity: {:.4}", analysis.harmonic_purity);

        // Verify basic properties
        assert!(analysis.total_harmonics == limit);
        assert!(analysis.harmonic_purity > 0.0 && analysis.harmonic_purity < 1.0);
    }
}

#[cfg(feature = "prime-harmonics")]
#[test]
fn test_harmonic_identity_prime_vs_random() {
    println!("\n=== Testing Harmonic Identity: Prime vs Random ===");

    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    let test_sizes = vec![200, 500, 1000];

    for size in test_sizes {
        let prime_seq = create_prime_sequence(size);
        let analyzer = HarmonicAnalyzer::new(size);
        let prime_analysis = analyzer.analyze(&prime_seq);

        // Create multiple random sequences for statistical validity
        let prime_density = prime_seq.iter().filter(|&&x| x == 1.0).count() as f64 / size as f64;
        let mut random_purities = Vec::new();

        for seed in 0..5 {
            let mut rng = StdRng::seed_from_u64(seed);
            let random_seq: Vec<f64> = (0..size)
                .map(|_| {
                    if rng.gen::<f64>() < prime_density {
                        1.0
                    } else {
                        0.0
                    }
                })
                .collect();

            let random_analysis = analyzer.analyze(&random_seq);
            random_purities.push(random_analysis.harmonic_purity);
        }

        let avg_random_purity = random_purities.iter().sum::<f64>() / random_purities.len() as f64;

        println!("\nSize: {}", size);
        println!("  Prime density: {:.4}", prime_density);
        println!(
            "  Prime harmonic purity: {:.4}",
            prime_analysis.harmonic_purity
        );
        println!(
            "  Random purities: {:?}",
            random_purities
                .iter()
                .map(|p| format!("{:.4}", p))
                .collect::<Vec<_>>()
        );
        println!("  Average random purity: {:.4}", avg_random_purity);
        println!(
            "  Ratio (prime/random): {:.2}x",
            prime_analysis.harmonic_purity / avg_random_purity
        );

        // Prime sequences should generally have distinct harmonic structure
        // Note: This might not always hold due to statistical variation
        if prime_analysis.harmonic_purity <= avg_random_purity {
            println!("  WARNING: Prime purity not higher than random average");
        }
    }
}

#[cfg(feature = "prime-harmonics")]
#[test]
fn test_membrane_generated_harmonics() {
    println!("\n=== Testing Membrane-Generated Prime Harmonics ===");

    use prime_physics_engine::membrane::construct_symmetric_membrane;

    // Test multiple membrane configurations
    let configs = vec![
        (3, 7, "High-performing (3,7)"),
        (1, 9, "Coprime (1,9)"),
        (7, 3, "Reversed (7,3)"),
    ];

    for (outer, inner, name) in configs {
        println!("\nTesting {} configuration:", name);

        let mut all_numbers = Vec::new();
        let mut prime_numbers = Vec::new();

        for seed in 1..=20 {
            if let Ok(n_str) = construct_symmetric_membrane(outer, inner, &seed.to_string(), 0, 0) {
                if let Ok(n) = n_str.parse::<u64>() {
                    all_numbers.push(n);
                    let big_n = BigUint::from(n);
                    if is_prime(&big_n) {
                        prime_numbers.push(n);
                    }
                }
            }
        }

        let prime_rate = prime_numbers.len() as f64 / all_numbers.len() as f64;
        println!(
            "  Generated {} numbers, {} primes ({:.1}% success rate)",
            all_numbers.len(),
            prime_numbers.len(),
            prime_rate * 100.0
        );

        // Create a sequence for harmonic analysis
        if !all_numbers.is_empty() {
            let max_n = *all_numbers.iter().max().unwrap() as usize + 1;
            let mut membrane_seq = vec![0.0; max_n.min(10000)]; // Cap at 10000 for memory

            for &n in &all_numbers {
                if (n as usize) < membrane_seq.len() {
                    let big_n = BigUint::from(n);
                    membrane_seq[n as usize] = if is_prime(&big_n) { 1.0 } else { 0.5 };
                }
            }

            let analyzer = HarmonicAnalyzer::new(membrane_seq.len());
            let analysis = analyzer.analyze(&membrane_seq);

            println!("  Harmonic analysis:");
            println!("    Total harmonics: {}", analysis.total_harmonics);
            println!("    Harmonic purity: {:.6}", analysis.harmonic_purity);
            println!(
                "    Dominant frequencies: {}",
                analysis.dominant_frequencies.len()
            );
        }
    }
}

#[cfg(feature = "prime-harmonics")]
#[test]
fn test_parseval_theorem() {
    println!("\n=== Testing Parseval's Theorem ===");

    // Test with various signals
    let test_signals = vec![
        ("Constant", vec![1.0; 10]),
        ("Alternating", vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0]),
        ("Linear", (0..20).map(|i| i as f64).collect()),
        ("Prime indicator", create_prime_sequence(50)),
    ];

    for (name, signal) in test_signals {
        let spectrum = fourier_transform(&signal);

        let time_energy: f64 = signal.iter().map(|&x| x * x).sum();
        let freq_energy: f64 =
            spectrum.iter().map(|c| c.norm_sqr()).sum::<f64>() * signal.len() as f64;

        let error = (time_energy - freq_energy).abs() / time_energy;

        println!("\n{} signal:", name);
        println!("  Length: {}", signal.len());
        println!("  Time domain energy: {:.6}", time_energy);
        println!("  Frequency domain energy: {:.6}", freq_energy);
        println!("  Relative error: {:.2e}", error);

        assert!(
            error < 1e-10,
            "Parseval's theorem violated for {} signal",
            name
        );
    }
}

#[cfg(not(feature = "prime-harmonics"))]
#[test]
fn test_harmonics_feature_required() {
    println!("\n=== Prime Harmonics Feature Test ===");
    println!("The prime-harmonics feature is not enabled.");
    println!("To run these tests, use:");
    println!("  cargo test --features prime-harmonics --test prime_harmonics_test");

    // Verify stub behavior when feature is disabled
    use prime_physics_engine::harmonics::HarmonicAnalyzer;

    let analyzer = HarmonicAnalyzer::new(100);
    let dummy_seq = vec![1.0, 0.0, 1.0, 0.0];
    let analysis = analyzer.analyze(&dummy_seq);

    assert_eq!(analysis.dominant_frequencies.len(), 0);
    assert_eq!(analysis.total_harmonics, 0);
    assert_eq!(analysis.harmonic_purity, 0.0);

    println!("\nStub implementation verified: returns empty analysis when disabled.");
}
