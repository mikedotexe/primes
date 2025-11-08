//! Fourier Analysis of Membrane Patterns
//! 
//! This analyzes successful prime-generating membranes in frequency space
//! to discover hidden harmonic patterns.

use prime_physics_engine::{is_prime_miller_rabin, ascii_art::*};
use num_bigint::BigUint;
use num_complex::Complex64;
use std::f64::consts::PI;
fn main() {
    println!("{}", banner("MEMBRANE FOURIER ANALYSIS", 100));
    println!("\nAnalyzing successful patterns in frequency space...\n");
    
    // First, collect successful membrane sequences
    let patterns = collect_successful_patterns();
    // Analyze frequency components
    analyze_frequency_components(&patterns);
    // Look for harmonic relationships
    find_harmonic_relationships(&patterns);
    // Visualize the frequency spectrum
    visualize_frequency_spectrum(&patterns);
}
#[derive(Debug, Clone)]
struct MembranePattern {
    base: u32,
    config: (u32, u32),
    k: (u32, u32),
    success_rate: f64,
    digit_sequence: Vec<u32>,
    prime_positions: Vec<usize>,
fn collect_successful_patterns() -> Vec<MembranePattern> {
    println!("Collecting successful membrane patterns...\n");
    let mut patterns = Vec::new();
    // Test top configurations we know work well
    let configs = vec![
        (6, (1, 5), (0, 0)),
        (6, (5, 1), (0, 0)),
        (30, (1, 17), (0, 0)),
        (12, (1, 5), (0, 0)),
        (4, (1, 3), (0, 0)),
        (10, (1, 7), (0, 0)),
    ];
    for (base, config, k) in configs {
        print!("Analyzing base {} config ({},{})... ", base, config.0, config.1);
        
        let mut digit_sequence = Vec::new();
        let mut prime_positions = Vec::new();
        let samples = 1000;
        let mut successes = 0;
        for seed in 0..samples {
            let membrane = construct_membrane(base, config.0, config.1, k.0, k.1, seed);
            let digits = extract_digits(&membrane, base);
            digit_sequence.extend(&digits);
            
            if is_prime_miller_rabin(&membrane) {
                successes += 1;
                prime_positions.push(seed as usize);
            }
        }
        let success_rate = (successes as f64 / samples as f64) * 100.0;
        println!("{:.1}% success", success_rate);
        patterns.push(MembranePattern {
            base,
            config,
            k,
            success_rate,
            digit_sequence,
            prime_positions,
        });
    }
    patterns
fn analyze_frequency_components(patterns: &[MembranePattern]) {
    println!("\n{}", boxed_title("FREQUENCY COMPONENT ANALYSIS", 80));
    for pattern in patterns {
        println!("\nBase {} ({},{}):", pattern.base, pattern.config.0, pattern.config.1);
        // Convert digit sequence to signal
        let signal: Vec<f64> = pattern.digit_sequence.iter()
            .map(|&d| d as f64 / pattern.base as f64)
            .collect();
        // Compute DFT
        let spectrum = compute_dft(&signal);
        // Find dominant frequencies
        let dominant = find_dominant_frequencies(&spectrum, 5);
        println!("  Dominant frequencies (normalized):");
        for (freq, magnitude) in &dominant {
            let wavelength = if *freq > 0.0 { 1.0 / freq } else { f64::INFINITY };
            println!("    f={:.4}, λ={:.2}, |A|={:.2}", freq, wavelength, magnitude);
        // Check for special relationships
        check_frequency_relationships(&dominant, pattern.base);
fn find_harmonic_relationships(patterns: &[MembranePattern]) {
    println!("\n{}", boxed_title("HARMONIC RELATIONSHIPS", 80));
    // Analyze prime position patterns
        // Convert prime positions to intervals
        let mut intervals = Vec::new();
        for i in 1..pattern.prime_positions.len() {
            intervals.push(pattern.prime_positions[i] - pattern.prime_positions[i-1]);
        if intervals.is_empty() {
            continue;
        // Analyze interval spectrum
        let interval_signal: Vec<f64> = intervals.iter().map(|&i| i as f64).collect();
        let interval_spectrum = compute_dft(&interval_signal);
        println!("  Prime interval spectrum:");
        let dominant_intervals = find_dominant_frequencies(&interval_spectrum, 3);
        for (freq, mag) in dominant_intervals {
            println!("    Interval frequency: {:.4} (magnitude: {:.2})", freq, mag);
        // Look for beat frequencies
        find_beat_frequencies(pattern);
fn visualize_frequency_spectrum(patterns: &[MembranePattern]) {
    println!("\n{}", boxed_title("FREQUENCY SPECTRUM VISUALIZATION", 80));
        println!("\nBase {} ({},{}) - {:.1}% success:", 
            pattern.base, pattern.config.0, pattern.config.1, pattern.success_rate);
        // Create frequency bins
            .take(256) // Use first 256 for visualization
        // Visualize as ASCII histogram
        println!("\n  Frequency Spectrum (0 to Nyquist):");
        visualize_spectrum(&spectrum, 40);
        // Show phase relationships
        analyze_phase_relationships(&spectrum, pattern);
fn compute_dft(signal: &[f64]) -> Vec<Complex64> {
    let n = signal.len();
    let mut spectrum = vec![Complex64::new(0.0, 0.0); n];
    for k in 0..n {
        for (t, &x) in signal.iter().enumerate() {
            let angle = -2.0 * PI * k as f64 * t as f64 / n as f64;
            spectrum[k] += Complex64::new(angle.cos(), angle.sin()) * x;
    spectrum
fn find_dominant_frequencies(spectrum: &[Complex64], count: usize) -> Vec<(f64, f64)> {
    let n = spectrum.len();
    let mut freq_mag: Vec<(f64, f64)> = Vec::new();
    // Only look at first half (Nyquist)
    for k in 1..n/2 {
        let magnitude = spectrum[k].norm();
        let frequency = k as f64 / n as f64;
        freq_mag.push((frequency, magnitude));
    // Sort by magnitude
    freq_mag.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    freq_mag.into_iter().take(count).collect()
fn check_frequency_relationships(frequencies: &[(f64, f64)], base: u32) {
    println!("\n  Checking for special relationships:");
    // Check if frequencies relate to base factors
    let factors = factorize(base);
    for (freq, _) in frequencies {
        for &factor in &factors {
            let ratio = freq * factor as f64;
            if (ratio - ratio.round()).abs() < 0.05 {
                println!("    Frequency {:.4} ≈ 1/{} (base factor resonance)", freq, factor);
    // Check for golden ratio relationships
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    for i in 0..frequencies.len() {
        for j in i+1..frequencies.len() {
            let ratio = frequencies[i].0 / frequencies[j].0;
            if (ratio - phi).abs() < 0.1 {
                println!("    Frequencies {:.4} and {:.4} have golden ratio relationship", 
                    frequencies[i].0, frequencies[j].0);
fn find_beat_frequencies(pattern: &MembranePattern) {
    // Look for interference patterns between outer and inner digits
    let outer_freq = pattern.config.0 as f64 / pattern.base as f64;
    let inner_freq = pattern.config.1 as f64 / pattern.base as f64;
    let beat_freq = (outer_freq - inner_freq).abs();
    let sum_freq = outer_freq + inner_freq;
    println!("\n  Beat frequency analysis:");
    println!("    Outer frequency: {:.4}", outer_freq);
    println!("    Inner frequency: {:.4}", inner_freq);
    println!("    Beat frequency:  {:.4}", beat_freq);
    println!("    Sum frequency:   {:.4}", sum_freq);
    // Check if beat frequency relates to success rate
    let normalized_success = pattern.success_rate / 100.0;
    if (beat_freq - normalized_success).abs() < 0.1 {
        println!("    🎯 Beat frequency correlates with success rate!");
fn visualize_spectrum(spectrum: &[Complex64], width: usize) {
    let max_mag = spectrum.iter()
        .take(spectrum.len() / 2)
        .map(|c| c.norm())
        .fold(0.0, f64::max);
    for k in 0..spectrum.len().min(20) {
        let mag = spectrum[k].norm();
        let bar_length = ((mag / max_mag) * width as f64) as usize;
        let bar = "█".repeat(bar_length);
        let freq = k as f64 / spectrum.len() as f64;
        println!("  {:.3} │{:<width$} {:.1}", freq, bar, mag, width = width);
fn analyze_phase_relationships(spectrum: &[Complex64], _pattern: &MembranePattern) {
    println!("\n  Phase relationships:");
    // Find phases of dominant frequencies
    let dominant = find_dominant_frequencies(spectrum, 3);
    for (_i, (freq, _)) in dominant.iter().enumerate() {
        let k = (freq * spectrum.len() as f64) as usize;
        if k < spectrum.len() {
            let phase = spectrum[k].arg();
            println!("    Frequency {:.4}: phase = {:.2}°", freq, phase * 180.0 / PI);
    // Check for phase locking
    if dominant.len() >= 2 {
        let k1 = (dominant[0].0 * spectrum.len() as f64) as usize;
        let k2 = (dominant[1].0 * spectrum.len() as f64) as usize;
        if k1 < spectrum.len() && k2 < spectrum.len() {
            let phase_diff = (spectrum[k1].arg() - spectrum[k2].arg()).abs();
            if phase_diff < 0.1 || (phase_diff - PI).abs() < 0.1 {
                println!("    🔒 Phase locked frequencies detected!");
fn construct_membrane(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, seed: u32) -> BigUint {
    let width = 2 * (1 + k_outer + 1 + k_inner) + 1;
    let base_big = BigUint::from(base);
    let mut value = BigUint::from(0u32);
    value += BigUint::from(outer) * base_big.pow(width - 1);
    value += BigUint::from(inner) * base_big.pow(width - 2 - k_outer);
    value += BigUint::from(seed) * base_big.pow(width / 2);
    value += BigUint::from(inner) * base_big.pow(k_inner + 1);
    value += BigUint::from(outer);
    value
fn extract_digits(num: &BigUint, base: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut n = num.clone();
    while n > BigUint::from(0u32) {
        let digit = (&n % &base_big).to_u32().unwrap_or(0);
        digits.push(digit);
        n /= &base_big;
    digits.reverse();
    digits
fn factorize(n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut num = n;
    for p in 2..=(n as f64).sqrt() as u32 {
        while num % p == 0 {
            factors.push(p);
            num /= p;
    if num > 1 {
        factors.push(num);
    factors.sort();
    factors.dedup();
    factors
use num_traits::ToPrimitive;
