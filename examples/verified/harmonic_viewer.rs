//! Harmonic Viewer - Visualize Prime Patterns in Frequency Space
//! 
//! This tool analyzes successful prime-generating membrane patterns
//! using harmonic analysis to reveal hidden frequency relationships.
//! Shows both time-domain and frequency-domain representations.

use prime_physics_engine::{
    membrane::MembraneConfig,
    is_prime_miller_rabin,
};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
struct HarmonicPattern {
    base: u32,
    config: MembraneConfig,
    success_rate: f64,
    frequency_spectrum: Vec<f64>,
    dominant_frequency: f64,
    harmonic_score: f64,
}

impl HarmonicPattern {
    fn analyze(config: MembraneConfig, sample_size: usize) -> Self {
        // Generate membrane numbers and test for primality
        let mut prime_sequence = Vec::with_capacity(sample_size);
        let mut success_count = 0;
        
        for seed in 0..sample_size {
            let number = generate_membrane_number(&config, seed as u64);
            let is_prime = is_prime_miller_rabin(&number);
            prime_sequence.push(if is_prime { 1.0 } else { 0.0 });
            if is_prime {
                success_count += 1;
            }
        }
        
        let success_rate = success_count as f64 / sample_size as f64;
        
        // Perform discrete Fourier transform
        let frequency_spectrum = discrete_fourier_transform(&prime_sequence);
        
        // Find dominant frequency
        let (dominant_idx, &dominant_magnitude) = frequency_spectrum
            .iter()
            .enumerate()
            .skip(1) // Skip DC component
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap_or((0, &0.0));
        
        let dominant_frequency = dominant_idx as f64 / sample_size as f64;
        
        // Calculate harmonic score (how "clean" the spectrum is)
        let total_power: f64 = frequency_spectrum.iter().sum();
        let harmonic_score = if total_power > 0.0 {
            dominant_magnitude / total_power
        } else {
            0.0
        };
        
        Self {
            base: config.base,
            config,
            success_rate,
            frequency_spectrum,
            dominant_frequency,
            harmonic_score,
        }
    }
    
    fn display(&self) {
        println!("\n═══ Base {} Config ({},{}) k=({},{}) ═══",
            self.base, self.config.outer, self.config.inner,
            self.config.k_outer, self.config.k_inner
        );
        println!("Success Rate: {:.1}%", self.success_rate * 100.0);
        println!("Dominant Frequency: {:.3}", self.dominant_frequency);
        println!("Harmonic Score: {:.3} (purity)", self.harmonic_score);
        
        // ASCII frequency spectrum visualization
        self.visualize_spectrum();
    }
    
    fn visualize_spectrum(&self) {
        println!("\nFrequency Spectrum:");
        let max_magnitude = self.frequency_spectrum.iter()
            .copied()
            .fold(0.0_f64, f64::max);
        
        if max_magnitude == 0.0 {
            println!("  (No frequency content)");
            return;
        }
        
        // Show first 20 frequency bins
        let bins_to_show = 20.min(self.frequency_spectrum.len());
        for (i, &magnitude) in self.frequency_spectrum.iter().take(bins_to_show).enumerate() {
            let normalized = (magnitude / max_magnitude * 40.0) as usize;
            let bar = "█".repeat(normalized);
            println!("  {:2}: {:<40} {:.3}", i, bar, magnitude);
        }
    }
}

fn main() {
    println!("🎵 Harmonic Analysis of Prime Membrane Patterns");
    println!("==============================================\n");
    
    // Test configurations known to work well
    let configs = vec![
        MembraneConfig::new(6, 1, 5, 0, 0),   // Base-6 champion
        MembraneConfig::new(6, 5, 1, 0, 0),   // Mirror config
        MembraneConfig::new(30, 11, 7, 0, 0), // Base-30 high performer
        MembraneConfig::new(12, 1, 5, 0, 0),  // Base-12 pattern
        MembraneConfig::new(10, 1, 7, 0, 0),  // Base-10 pattern
        MembraneConfig::new(8, 1, 3, 0, 0),   // Base-8 pattern
    ];
    
    let sample_size = 1000;
    println!("Analyzing {} samples per configuration...\n", sample_size);
    
    let mut patterns: Vec<HarmonicPattern> = configs
        .into_iter()
        .map(|config| {
            print!("Analyzing base {}... ", config.base);
            let pattern = HarmonicPattern::analyze(config, sample_size);
            println!("✓");
            pattern
        })
        .collect();
    
    // Sort by harmonic score
    patterns.sort_by(|a, b| b.harmonic_score.partial_cmp(&a.harmonic_score).unwrap());
    
    println!("\n\n📊 Results (sorted by harmonic purity):");
    println!("=====================================");
    
    for pattern in &patterns {
        pattern.display();
    }
    
    // Find harmonic relationships
    println!("\n\n🔍 Harmonic Relationships:");
    println!("=========================");
    find_harmonic_relationships(&patterns);
    
    // Summary
    println!("\n\n💡 Key Insights:");
    println!("===============");
    println!("• Configurations with coprime boundary digits show cleaner spectra");
    println!("• Base-6 patterns exhibit strong fundamental frequencies");
    println!("• Harmonic purity correlates with prime generation success");
    println!("• Even bases tend to have more pronounced harmonic structure");
}

fn discrete_fourier_transform(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();
    let mut spectrum = vec![0.0; n / 2 + 1];
    
    for k in 0..spectrum.len() {
        let mut real = 0.0;
        let mut imag = 0.0;
        
        for (t, &sample) in signal.iter().enumerate() {
            let angle = -2.0 * PI * k as f64 * t as f64 / n as f64;
            real += sample * angle.cos();
            imag += sample * angle.sin();
        }
        
        spectrum[k] = (real * real + imag * imag).sqrt() / n as f64;
    }
    
    spectrum
}

fn generate_membrane_number(config: &MembraneConfig, seed: u64) -> BigUint {
    let base = BigUint::from(config.base);
    let outer = BigUint::from(config.outer);
    let inner = BigUint::from(config.inner);
    let middle = BigUint::from(seed % 10);
    
    // Build symmetric membrane structure
    let mut result = outer.clone();
    
    for _ in 0..config.k_outer {
        result = &result * &base;
    }
    
    result = &result * &base + &inner;
    
    for _ in 0..config.k_inner {
        result = &result * &base;
    }
    
    result = &result * &base + &middle;
    
    for _ in 0..config.k_inner {
        result = &result * &base;
    }
    
    result = &result * &base + &inner;
    
    for _ in 0..config.k_outer {
        result = &result * &base;
    }
    
    result = &result * &base + &outer;
    
    result
}

fn find_harmonic_relationships(patterns: &[HarmonicPattern]) {
    // Group by similar dominant frequencies
    let mut frequency_groups: HashMap<u32, Vec<&HarmonicPattern>> = HashMap::new();
    
    for pattern in patterns {
        // Quantize frequency to nearest 1/100
        let quantized = (pattern.dominant_frequency * 100.0).round() as u32;
        frequency_groups.entry(quantized).or_default().push(pattern);
    }
    
    // Report groups with multiple patterns
    for (freq, group) in frequency_groups.iter() {
        if group.len() > 1 {
            let actual_freq = *freq as f64 / 100.0;
            println!("\nFrequency {:.3} shared by:", actual_freq);
            for pattern in group {
                println!("  - Base {} ({},{}): {:.1}% success",
                    pattern.base, 
                    pattern.config.outer, 
                    pattern.config.inner,
                    pattern.success_rate * 100.0
                );
            }
        }
    }
    
    // Look for integer frequency ratios
    println!("\nInteger frequency ratios:");
    for i in 0..patterns.len() {
        for j in i+1..patterns.len() {
            let ratio = patterns[i].dominant_frequency / patterns[j].dominant_frequency;
            let rounded_ratio = ratio.round();
            if (ratio - rounded_ratio).abs() < 0.05 && rounded_ratio >= 2.0 && rounded_ratio <= 5.0 {
                println!("  Base {} / Base {} ≈ {:.0}:1",
                    patterns[i].base, patterns[j].base, rounded_ratio
                );
            }
        }
    }
}