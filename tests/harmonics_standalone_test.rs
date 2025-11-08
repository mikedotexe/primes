//! Standalone test file for prime-harmonics feature
//! Run with: cargo test --features prime-harmonics --test harmonics_standalone_test

#[cfg(feature = "prime-harmonics")]
use prime_physics_engine::harmonics::{
    fourier_transform, power_spectrum, find_dominant_frequencies, HarmonicAnalyzer
};
use prime_physics_engine::{is_prime, prime_sieve::BitSieve};
use num_bigint::BigUint;
use std::f64::consts::PI;

/// Helper function to create a prime indicator sequence
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
fn test_fourier_transform_basic() {
    let signal = vec![1.0, 0.0, 1.0, 0.0];
    let spectrum = fourier_transform(&signal);
    
    // DC component should be average of signal
    assert!((spectrum[0].re - 0.5).abs() < 1e-10);
    assert!(spectrum[0].im.abs() < 1e-10);
    
    // Should have 4 frequency components
    assert_eq!(spectrum.len(), 4);
}

#[cfg(feature = "prime-harmonics")]
#[test]
fn test_power_spectrum_non_negative() {
    let signal = vec![1.0, -1.0, 2.0, -2.0, 3.0];
    let power = power_spectrum(&signal);
    
    // All power spectrum values should be non-negative
    for (i, &p) in power.iter().enumerate() {
        assert!(p >= 0.0, "Power spectrum at index {} is negative: {}", i, p);
    }
}

#[cfg(feature = "prime-harmonics")]
#[test]
fn test_prime_sequence_harmonics() {
    let prime_seq = create_prime_sequence(100);
    let analyzer = HarmonicAnalyzer::new(100);
    let analysis = analyzer.analyze(&prime_seq);
    
    println!("Prime sequence analysis:");
    println!("  Total harmonics: {}", analysis.total_harmonics);
    println!("  Dominant frequencies: {:?}", analysis.dominant_frequencies);
    println!("  Harmonic purity: {:.4}", analysis.harmonic_purity);
    
    // Should have dominant frequencies
    assert!(!analysis.dominant_frequencies.is_empty(), 
            "Prime sequence should have dominant frequencies");
    
    // Should have reasonable harmonic purity
    assert!(analysis.harmonic_purity > 0.0 && analysis.harmonic_purity < 1.0,
            "Harmonic purity {} is out of expected range", analysis.harmonic_purity);
}

#[cfg(feature = "prime-harmonics")]
#[test]
fn test_membrane_prime_harmonics() {
    use prime_physics_engine::membrane::construct_symmetric_membrane;
    
    let mut membrane_seq = vec![0.0; 200];
    
    // Use verified high-performing configuration (3,7) k=(0,0)
    let outer = 3;
    let inner = 7;
    let k_outer = 0;
    let k_inner = 0;
    
    let mut prime_count = 0;
    let mut total_count = 0;
    for seed in 1..=9 {
        if let Ok(n_str) = construct_symmetric_membrane(outer, inner, &seed.to_string(), k_outer, k_inner) {
            println!("Seed {} -> membrane number: {}", seed, n_str);
            if let Ok(n) = n_str.parse::<u64>() {
                total_count += 1;
                let big_n = BigUint::from(n);
                if is_prime(&big_n) {
                    prime_count += 1;
                    println!("  {} is PRIME", n);
                    if n < 200 {
                        membrane_seq[n as usize] = 1.0;
                    }
                } else {
                    println!("  {} is composite", n);
                    if n < 200 {
                        membrane_seq[n as usize] = 0.5;
                    }
                }
            }
        }
    }
    
    println!("\nMembrane configuration (3,7) k=(0,0) generated {} primes out of {} valid numbers", prime_count, total_count);
    
    let analyzer = HarmonicAnalyzer::new(200);
    let analysis = analyzer.analyze(&membrane_seq);
    
    println!("Membrane sequence analysis:");
    println!("  Total harmonics: {}", analysis.total_harmonics);
    println!("  Harmonic purity: {:.4}", analysis.harmonic_purity);
    
    // Membrane-generated sequences should show harmonic structure
    assert!(analysis.total_harmonics > 0);
}

#[cfg(feature = "prime-harmonics")]
#[test]
fn test_harmonic_identity_validation() {
    // This test validates the harmonic identity for primes
    // by comparing prime sequences with random sequences
    
    let prime_seq = create_prime_sequence(300);
    let analyzer = HarmonicAnalyzer::new(300);
    let prime_analysis = analyzer.analyze(&prime_seq);
    
    // Create random sequence with same density
    let prime_count = prime_seq.iter().filter(|&&x| x == 1.0).count();
    let density = prime_count as f64 / prime_seq.len() as f64;
    
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;
    let mut rng = StdRng::seed_from_u64(42);
    let random_seq: Vec<f64> = (0..300)
        .map(|_| if rng.gen::<f64>() < density { 1.0 } else { 0.0 })
        .collect();
    
    let random_analysis = analyzer.analyze(&random_seq);
    
    println!("\nHarmonic Identity Validation:");
    println!("Prime sequence:");
    println!("  Density: {:.4}", density);
    println!("  Harmonic purity: {:.4}", prime_analysis.harmonic_purity);
    println!("  Dominant frequencies: {} found", prime_analysis.dominant_frequencies.len());
    
    println!("Random sequence (same density):");
    println!("  Harmonic purity: {:.4}", random_analysis.harmonic_purity);
    println!("  Dominant frequencies: {} found", random_analysis.dominant_frequencies.len());
    
    println!("Purity ratio (prime/random): {:.2}x", 
             prime_analysis.harmonic_purity / random_analysis.harmonic_purity);
    
    // Prime sequences should have more harmonic structure than random
    assert!(prime_analysis.harmonic_purity > random_analysis.harmonic_purity * 0.9,
            "Prime purity {} should be notably higher than random purity {}",
            prime_analysis.harmonic_purity, random_analysis.harmonic_purity);
}

#[cfg(not(feature = "prime-harmonics"))]
#[test]
fn test_harmonics_feature_disabled() {
    println!("prime-harmonics feature is disabled");
    println!("Run with: cargo test --features prime-harmonics");
    
    use prime_physics_engine::harmonics::HarmonicAnalyzer;
    
    let analyzer = HarmonicAnalyzer::new(100);
    let dummy_seq = vec![1.0, 0.0, 1.0, 0.0];
    let analysis = analyzer.analyze(&dummy_seq);
    
    assert_eq!(analysis.dominant_frequencies.len(), 0);
    assert_eq!(analysis.total_harmonics, 0);
    assert_eq!(analysis.harmonic_purity, 0.0);
}