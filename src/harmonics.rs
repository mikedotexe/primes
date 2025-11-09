//! Harmonic analysis module for prime patterns
//!
//! Enabled with the `prime-harmonics` feature flag.

#[cfg(feature = "prime-harmonics")]
use num_complex::Complex64;
#[cfg(feature = "prime-harmonics")]
use std::f64::consts::PI;

/// Performs discrete Fourier transform on a sequence of prime indicators
#[cfg(feature = "prime-harmonics")]
pub fn fourier_transform(signal: &[f64]) -> Vec<Complex64> {
    let n = signal.len();
    let mut spectrum = vec![Complex64::new(0.0, 0.0); n];

    for (k, freq_bin) in spectrum.iter_mut().enumerate().take(n) {
        for (t, &sample) in signal.iter().enumerate() {
            let angle = -2.0 * PI * k as f64 * t as f64 / n as f64;
            *freq_bin += Complex64::new(angle.cos(), angle.sin()) * sample;
        }
        *freq_bin /= n as f64;
    }

    spectrum
}

/// Computes the power spectrum from Fourier transform
#[cfg(feature = "prime-harmonics")]
pub fn power_spectrum(signal: &[f64]) -> Vec<f64> {
    fourier_transform(signal)
        .iter()
        .map(|c| c.norm_sqr())
        .collect()
}

/// Finds dominant frequencies in a prime sequence
#[cfg(feature = "prime-harmonics")]
pub fn find_dominant_frequencies(signal: &[f64], threshold: f64) -> Vec<(usize, f64)> {
    let spectrum = power_spectrum(signal);
    let max_power = spectrum.iter().copied().fold(0.0_f64, f64::max);

    spectrum
        .iter()
        .enumerate()
        .filter(|(_, &power)| power > max_power * threshold)
        .map(|(freq, &power)| (freq, power))
        .collect()
}

/// Analyzes harmonic relationships between prime patterns
pub struct HarmonicAnalyzer {
    pub sample_size: usize,
}

impl HarmonicAnalyzer {
    pub fn new(sample_size: usize) -> Self {
        Self { sample_size }
    }

    /// Analyze a sequence of prime indicators (1.0 for prime, 0.0 for composite)
    #[cfg(feature = "prime-harmonics")]
    pub fn analyze(&self, prime_sequence: &[f64]) -> HarmonicAnalysis {
        let dominant = find_dominant_frequencies(prime_sequence, 0.1);
        let spectrum = power_spectrum(prime_sequence);

        HarmonicAnalysis {
            dominant_frequencies: dominant,
            total_harmonics: spectrum.len(),
            harmonic_purity: calculate_purity(&spectrum),
        }
    }

    /// Stub implementation when harmonics feature is disabled
    #[cfg(not(feature = "prime-harmonics"))]
    pub fn analyze(&self, _prime_sequence: &[f64]) -> HarmonicAnalysis {
        HarmonicAnalysis {
            dominant_frequencies: vec![],
            total_harmonics: 0,
            harmonic_purity: 0.0,
        }
    }
}

pub struct HarmonicAnalysis {
    pub dominant_frequencies: Vec<(usize, f64)>,
    pub total_harmonics: usize,
    pub harmonic_purity: f64,
}

#[cfg(feature = "prime-harmonics")]
fn calculate_purity(spectrum: &[f64]) -> f64 {
    if spectrum.is_empty() {
        return 0.0;
    }

    let max_power = spectrum.iter().copied().fold(0.0_f64, f64::max);
    let total_power: f64 = spectrum.iter().sum();

    if total_power > 0.0 {
        max_power / total_power
    } else {
        0.0
    }
}
