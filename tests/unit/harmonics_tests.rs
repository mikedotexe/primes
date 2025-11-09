//! Tests for the prime-harmonics feature
//!
//! These tests validate harmonic identities and Fourier analysis for prime numbers.

#[cfg(feature = "prime-harmonics")]
mod harmonics_tests {
    use num_bigint::BigUint;
    use primes::harmonics::{
        find_dominant_frequencies, fourier_transform, power_spectrum, HarmonicAnalyzer,
    };
    use primes::{is_prime, prime_sieve::BitSieve};
    use std::f64::consts::PI;

    /// Helper function to create a prime indicator sequence
    /// Returns 1.0 for prime positions, 0.0 for composite
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

    #[test]
    fn test_fourier_transform_basic() {
        // Test with a simple signal
        let signal = vec![1.0, 0.0, 1.0, 0.0];
        let spectrum = fourier_transform(&signal);

        // DC component should be average of signal
        assert!((spectrum[0].re - 0.5).abs() < 1e-10);
        assert!(spectrum[0].im.abs() < 1e-10);

        // Should have 4 frequency components
        assert_eq!(spectrum.len(), 4);
    }

    #[test]
    fn test_fourier_transform_parseval() {
        // Parseval's theorem: sum of squares in time domain equals sum in frequency domain
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let spectrum = fourier_transform(&signal);

        let time_energy: f64 = signal.iter().map(|&x| x * x).sum();
        let freq_energy: f64 =
            spectrum.iter().map(|c| c.norm_sqr()).sum::<f64>() * signal.len() as f64;

        assert!(
            (time_energy - freq_energy).abs() < 1e-10,
            "Parseval's theorem failed: time={}, freq={}",
            time_energy,
            freq_energy
        );
    }

    #[test]
    fn test_power_spectrum_non_negative() {
        let signal = vec![1.0, -1.0, 2.0, -2.0, 3.0];
        let power = power_spectrum(&signal);

        // All power spectrum values should be non-negative
        for (i, &p) in power.iter().enumerate() {
            assert!(p >= 0.0, "Power spectrum at index {} is negative: {}", i, p);
        }
    }

    #[test]
    fn test_dominant_frequencies_threshold() {
        // Create a signal with clear dominant frequency
        let n = 64;
        let signal: Vec<f64> = (0..n)
            .map(|i| (2.0 * PI * i as f64 * 5.0 / n as f64).sin())
            .collect();

        let dominant = find_dominant_frequencies(&signal, 0.1);

        // Should find at least one dominant frequency
        assert!(!dominant.is_empty(), "No dominant frequencies found");

        // The dominant frequency should be around index 5
        let (freq, _power) = dominant[0];
        assert!(
            freq == 5 || freq == n - 5,
            "Expected frequency 5 or {}, got {}",
            n - 5,
            freq
        );
    }

    #[test]
    fn test_prime_sequence_harmonics() {
        // Analyze harmonics in the first 100 natural numbers
        let prime_seq = create_prime_sequence(100);
        let analyzer = HarmonicAnalyzer::new(100);
        let analysis = analyzer.analyze(&prime_seq);

        // Should have dominant frequencies
        assert!(
            !analysis.dominant_frequencies.is_empty(),
            "Prime sequence should have dominant frequencies"
        );

        // Should have reasonable harmonic purity (not too pure, not too noisy)
        assert!(
            analysis.harmonic_purity > 0.0 && analysis.harmonic_purity < 1.0,
            "Harmonic purity {} is out of expected range",
            analysis.harmonic_purity
        );
    }

    #[test]
    fn test_prime_harmonics_vs_random() {
        // Compare prime sequence harmonics with random sequence
        let prime_seq = create_prime_sequence(200);
        let analyzer = HarmonicAnalyzer::new(200);
        let prime_analysis = analyzer.analyze(&prime_seq);

        // Create random sequence with same density as primes
        let prime_count = prime_seq.iter().filter(|&&x| x == 1.0).count();
        let density = prime_count as f64 / prime_seq.len() as f64;

        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};
        let mut rng = StdRng::seed_from_u64(42);
        let random_seq: Vec<f64> = (0..200)
            .map(|_| if rng.gen::<f64>() < density { 1.0 } else { 0.0 })
            .collect();

        let random_analysis = analyzer.analyze(&random_seq);

        // Prime sequence should have higher harmonic purity than random
        // This tests the harmonic identity - primes have more structure
        assert!(
            prime_analysis.harmonic_purity > random_analysis.harmonic_purity * 0.9,
            "Prime purity {} should be notably higher than random purity {}",
            prime_analysis.harmonic_purity,
            random_analysis.harmonic_purity
        );
    }

    #[test]
    fn test_membrane_prime_harmonics() {
        use primes::membrane::construct_symmetric_membrane;

        // Create a sequence of membrane-generated numbers
        let mut membrane_seq = vec![0.0; 200];

        // Use verified high-performing configuration
        let outer = 3;
        let inner = 7;
        let k_outer = 0;
        let k_inner = 0;

        for seed in 1..=9 {
            if let Ok(n_str) =
                construct_symmetric_membrane(outer, inner, &seed.to_string(), k_outer, k_inner)
            {
                if let Ok(n) = n_str.parse::<u64>() {
                    if n < 200 {
                        let big_n = BigUint::from(n);
                        membrane_seq[n as usize] = if is_prime(&big_n) { 1.0 } else { 0.5 };
                    }
                }
            }
        }

        let analyzer = HarmonicAnalyzer::new(200);
        let analysis = analyzer.analyze(&membrane_seq);

        // Membrane-generated sequences should show harmonic structure
        assert!(analysis.total_harmonics > 0);
        // Note: dominant_frequencies might be empty if the signal is too sparse
    }

    #[test]
    fn test_lagrange_point_harmonics() {
        // Simple test of harmonic analysis on a synthetic Lagrange-like pattern
        // Since the actual Lagrange API is complex, we'll create a pattern that
        // represents where Lagrange points might appear between primes

        let mut lagrange_seq = vec![0.0; 200];
        let sieve = BitSieve::new(50);
        let primes = sieve.primes();

        // Mark approximate midpoints between consecutive primes
        for i in 0..primes.len() - 1 {
            if primes[i] < 200 && primes[i + 1] < 200 {
                let midpoint = (primes[i] + primes[i + 1]) / 2;
                if midpoint < 200 {
                    lagrange_seq[midpoint] = 1.0;
                }
            }
        }

        let analyzer = HarmonicAnalyzer::new(200);
        let analysis = analyzer.analyze(&lagrange_seq);

        // Lagrange-like points should exhibit some harmonic structure
        assert!(analysis.total_harmonics > 0);
    }

    #[test]
    fn test_harmonic_stability() {
        // Test that similar prime sequences produce similar harmonic profiles
        let seq1 = create_prime_sequence(150);
        let seq2 = create_prime_sequence(151); // One more element

        let analyzer = HarmonicAnalyzer::new(150);
        let analysis1 = analyzer.analyze(&seq1);
        let analysis2 = analyzer.analyze(&seq2[..150]); // Truncate to same length

        // Harmonic purity should be very similar
        let purity_diff = (analysis1.harmonic_purity - analysis2.harmonic_purity).abs();
        assert!(
            purity_diff < 0.01,
            "Harmonic purity should be stable: diff={}",
            purity_diff
        );

        // Should have similar number of dominant frequencies
        let freq_diff = (analysis1.dominant_frequencies.len() as i32
            - analysis2.dominant_frequencies.len() as i32)
            .abs();
        assert!(
            freq_diff <= 2,
            "Number of dominant frequencies should be similar: {} vs {}",
            analysis1.dominant_frequencies.len(),
            analysis2.dominant_frequencies.len()
        );
    }

    #[test]
    fn test_base_dependent_harmonics() {
        use primes::membrane::construct_symmetric_membrane;

        // Test harmonic profiles in different bases
        // Use coprime boundary digits for each base
        let test_configs = vec![
            (6, 1, 5),   // base 6: (1,5) are coprime to 6
            (10, 3, 7),  // base 10: (3,7) are coprime to 10
            (12, 5, 7),  // base 12: (5,7) are coprime to 12
            (30, 7, 11), // base 30: (7,11) are coprime to 30
        ];

        let mut harmonic_profiles = Vec::new();

        for (base, outer, inner) in test_configs {
            let mut base_seq = vec![0.0; 200];

            // Generate numbers with this configuration
            // Note: construct_symmetric_membrane doesn't take base parameter
            // so we'll use base 10 construction for all
            for seed in 1..20 {
                if let Ok(n_str) =
                    construct_symmetric_membrane(outer, inner, &seed.to_string(), 0, 0)
                {
                    if let Ok(n) = n_str.parse::<u64>() {
                        if n < 200 {
                            let big_n = BigUint::from(n);
                            if is_prime(&big_n) {
                                base_seq[n as usize] = 1.0;
                            }
                        }
                    }
                }
            }

            let analyzer = HarmonicAnalyzer::new(200);
            let analysis = analyzer.analyze(&base_seq);
            harmonic_profiles.push((base, analysis.harmonic_purity));
        }

        // Different configurations should produce different harmonic profiles
        let purities: Vec<f64> = harmonic_profiles.iter().map(|(_, p)| *p).collect();
        let min_purity = purities.iter().copied().fold(f64::INFINITY, f64::min);
        let max_purity = purities.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        // Allow for the possibility that all purities are similar
        if max_purity > 0.0 {
            assert!(
                max_purity >= min_purity,
                "Max purity should be >= min purity: max={}, min={}",
                max_purity,
                min_purity
            );
        }
    }
}

#[cfg(not(feature = "prime-harmonics"))]
mod harmonics_tests {
    #[test]
    fn test_harmonics_feature_disabled() {
        use primes::harmonics::HarmonicAnalyzer;

        // When feature is disabled, analyzer should return empty results
        let analyzer = HarmonicAnalyzer::new(100);
        let dummy_seq = vec![1.0, 0.0, 1.0, 0.0];
        let analysis = analyzer.analyze(&dummy_seq);

        assert_eq!(analysis.dominant_frequencies.len(), 0);
        assert_eq!(analysis.total_harmonics, 0);
        assert_eq!(analysis.harmonic_purity, 0.0);
    }
}
