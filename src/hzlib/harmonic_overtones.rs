//! Fourier analysis of lineouts (overtone spectrum)

use super::grid_analysis::{Axis, JoinedGrid, lineout};

/// Compute discrete Fourier spectrum of a lineout
/// Returns vec of (frequency_index, magnitude) sorted by magnitude descending
pub fn overtone_spectrum(
    grid: &JoinedGrid,
    axis: Axis,
    fixed_mid: usize,
    fixed_iz: usize,
    quantity: &str,
) -> Vec<(usize, f64)> {
    let line = lineout(grid, axis, fixed_mid, fixed_iz);
    if line.is_empty() {
        return vec![];
    }

    // Extract the requested quantity
    let values: Vec<f64> = line
        .iter()
        .map(|(_, obs, pred)| match quantity {
            "obs" => *obs,
            "pred" => *pred,
            "enrichment" => if *pred > 0.0 { obs / pred - 1.0 } else { 0.0 },
            _ => 0.0,
        })
        .collect();

    // Simple DFT (magnitude spectrum only)
    naive_dft_spectrum(&values)
}

/// Naive DFT implementation (magnitude spectrum)
/// Returns (k, |X[k]|) for k = 0..N/2
fn naive_dft_spectrum(signal: &[f64]) -> Vec<(usize, f64)> {
    let n = signal.len();
    if n == 0 {
        return vec![];
    }

    let mut spectrum = Vec::new();
    let two_pi = 2.0 * std::f64::consts::PI;

    // Compute DFT for frequencies k = 0..N/2
    for k in 0..=n / 2 {
        let mut real = 0.0;
        let mut imag = 0.0;

        for (j, &x) in signal.iter().enumerate() {
            let angle = -two_pi * (k as f64) * (j as f64) / (n as f64);
            real += x * angle.cos();
            imag += x * angle.sin();
        }

        let magnitude = (real * real + imag * imag).sqrt();
        spectrum.push((k, magnitude));
    }

    // Sort by magnitude descending
    spectrum.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    spectrum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dft_dc_component() {
        let signal = vec![1.0, 1.0, 1.0, 1.0];
        let spectrum = naive_dft_spectrum(&signal);
        // DC component (k=0) should dominate
        assert_eq!(spectrum[0].0, 0);
        assert!(spectrum[0].1 > 3.0); // ~4.0
    }

    #[test]
    fn test_dft_single_freq() {
        // Pure sine wave at frequency 1
        let n = 16;
        let signal: Vec<f64> = (0..n)
            .map(|i| (2.0 * std::f64::consts::PI * (i as f64) / (n as f64)).sin())
            .collect();
        let spectrum = naive_dft_spectrum(&signal);

        // Frequency 1 should have large magnitude
        let freq1_mag = spectrum.iter().find(|(k, _)| *k == 1).map(|(_, m)| *m).unwrap_or(0.0);
        assert!(freq1_mag > 5.0);
    }
}
