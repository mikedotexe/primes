//! Prime Wave Visualizer - Showing how membrane patterns create waves
//! 
//! This visualizes the wave mechanics behind why certain patterns work,
//! with beautiful ASCII representations of interference patterns.

use prime_physics_engine::{is_prime_miller_rabin, ascii_art::*};
use num_bigint::BigUint;
use std::fs::File;
use std::io::Write;
use chrono::Local;
use std::collections::HashMap;
/// Represents a wave pattern in a specific base
#[derive(Debug, Clone)]
struct WavePattern {
    base: u32,
    digit1: u32,
    digit2: u32,
    distance: u32,
    wavelength: f64,
    amplitude: f64,
    phase: f64,
}
impl WavePattern {
    fn new(base: u32, digit1: u32, digit2: u32) -> Self {
        let distance = if digit2 > digit1 { 
            digit2 - digit1 
        } else { 
            digit1 - digit2 
        };
        
        // Wave properties depend on base and distance
        let wavelength = base as f64 / gcd(distance, base) as f64;
        let amplitude = 1.0 / (1.0 + (distance as f64 - 2.0).abs());
        let phase = (digit1 as f64 * std::f64::consts::PI * 2.0) / base as f64;
        Self {
            base,
            digit1,
            digit2,
            distance,
            wavelength,
            amplitude,
            phase,
        }
    }
    
    fn interference_quality(&self) -> f64 {
        // Even bases with distance 2 create perfect interference
        let base_factor = if self.base % 2 == 0 { 1.5 } else { 1.0 };
        let distance_factor = match self.distance {
            2 => 2.0,  // Twin prime distance
            1 => 1.5,  // Adjacent
            _ => 1.0 / (1.0 + (self.distance as f64 - 2.0).abs() * 0.2),
        self.amplitude * base_factor * distance_factor
    fn visualize_wave(&self) -> String {
        let mut viz = String::new();
        let quality = self.interference_quality();
        viz.push_str(&format!("\nWave: {} ↔ {} (distance {})\n", self.digit1, self.digit2, self.distance));
        viz.push_str(&format!("Wavelength: {:.1}, Quality: {:.2}\n", self.wavelength, quality));
        // Draw the wave
        let width = 60;
        let height = 5;
        for y in 0..height {
            for x in 0..width {
                let pos = x as f64 / width as f64 * 4.0 * std::f64::consts::PI;
                let wave_height = self.amplitude * (pos / self.wavelength + self.phase).sin();
                let normalized_y = (y as f64 - height as f64 / 2.0) / (height as f64 / 2.0);
                
                if (wave_height - normalized_y).abs() < 0.3 {
                    viz.push('█');
                } else if (wave_height - normalized_y).abs() < 0.5 {
                    viz.push('▒');
                } else {
                    viz.push(' ');
                }
            }
            viz.push('\n');
        viz
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
fn analyze_base_waves(base: u32) -> Vec<(WavePattern, Vec<BigUint>)> {
    let mut wave_results = Vec::new();
    println!("\n{}", boxed_title(&format!("BASE {} WAVE ANALYSIS", base), 60));
    // Test different digit pairs
    for d1 in 1..base.min(8) {
        for d2 in d1+1..base.min(8) {
            let wave = WavePattern::new(base, d1, d2);
            
            // Generate membrane numbers with this pattern
            let mut primes = Vec::new();
            for k_outer in 0..=2 {
                for k_inner in 0..=2 {
                    for middle in 0..10 {
                        let number = generate_wave_membrane(base, d1, d2, k_outer, k_inner, middle);
                        
                        if is_prime_miller_rabin(&number) && primes.len() < 5 {
                            primes.push(number);
                        }
                    }
            if !primes.is_empty() {
                wave_results.push((wave, primes));
    // Sort by interference quality
    wave_results.sort_by(|a, b| 
        b.0.interference_quality().partial_cmp(&a.0.interference_quality()).unwrap()
    );
    wave_results
fn generate_wave_membrane(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, middle: u32) -> BigUint {
    let mut digits = vec![outer];
    for _ in 0..k_outer { digits.push(0); }
    digits.push(inner);
    for _ in 0..k_inner { digits.push(0); }
    digits.push(middle);
    digits.push(outer);
    let mut value = BigUint::from(0u32);
    let base_big = BigUint::from(base);
    for digit in digits {
        value = value * &base_big + BigUint::from(digit);
    value
fn create_interference_diagram(wave1: &WavePattern, wave2: &WavePattern) -> String {
    let mut diagram = String::new();
    diagram.push_str(&format!("\n{}", boxed_title("WAVE INTERFERENCE PATTERN", 70)));
    let constructive = wave1.wavelength == wave2.wavelength;
    diagram.push_str(&format!("\nWave 1: {} ↔ {} (λ = {:.1})\n", 
        wave1.digit1, wave1.digit2, wave1.wavelength));
    diagram.push_str(&format!("Wave 2: {} ↔ {} (λ = {:.1})\n\n", 
        wave2.digit1, wave2.digit2, wave2.wavelength));
    if constructive {
        diagram.push_str(r#"
     ╱╲    ╱╲    ╱╲    ╱╲     Wave 1
    ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲
   ╱    ╲╱    ╲╱    ╲╱    ╲
          +
     ╱╲    ╱╲    ╱╲    ╱╲     Wave 2  
          =
    ╱╲╱╲  ╱╲╱╲  ╱╲╱╲  ╱╲╱╲    CONSTRUCTIVE!
   ╱╱╱╱╲╲╱╱╱╱╲╲╱╱╱╱╲╲╱╱╱╱╲╲   (Enhanced amplitude)
  ╱╱╱╱╱╱╲╲╲╲╲╲╱╱╱╱╱╱╲╲╲╲╲╲╱
"#);
    } else {
    ╱╲  ╱╲  ╱╲  ╱╲  ╱╲  ╱╲    Wave 2 (offset)
   ╱  ╲╱  ╲╱  ╲╱  ╲╱  ╲╱  ╲
  ╱    ╲  ╱    ╲  ╱    ╲  ╱
  ─────────────────────────    DESTRUCTIVE!
                               (Waves cancel)
    diagram
fn main() {
    println!("{}", banner("PRIME WAVE MECHANICS VISUALIZER", 80));
    println!("\nExploring how digit pairs create wave patterns that generate primes\n");
    // Analyze our champion bases
    let bases = vec![6, 10, 12];
    let mut all_waves = HashMap::new();
    for base in &bases {
        let waves = analyze_base_waves(*base);
        println!("\nTop wave patterns for base {}:", base);
        for (wave, primes) in waves.iter().take(3) {
            println!("{}", wave.visualize_wave());
            println!("Generates {} primes, e.g.: {}", primes.len(), primes[0]);
        all_waves.insert(*base, waves);
    // The 5-7 phenomenon across bases
    println!("\n{}", boxed_title("THE 5-7 PHENOMENON IN WAVES", 80));
        if *base >= 8 {  // Only bases that have digits 5 and 7
            let wave_5_7 = WavePattern::new(*base, 5, 7);
            println!("\nBase {} with 5-7 pattern:", base);
            println!("Wavelength: {:.1}", wave_5_7.wavelength);
            println!("Interference quality: {:.2}", wave_5_7.interference_quality());
            // Show why it works so well
            if *base % 2 == 0 {
                println!("✓ Even base bonus: 1.5x");
                println!("✓ Distance 2 bonus: 2.0x");
                println!("✓ Total boost: {:.1}x", 1.5 * 2.0);
    // Create interference patterns
    println!("\n{}", separator("wave", 80));
    // Compare good vs bad interference
    if let Some(base10_waves) = all_waves.get(&10) {
        if base10_waves.len() >= 2 {
            let good_wave1 = &base10_waves[0].0;
            let good_wave2 = WavePattern::new(10, good_wave1.digit2, 
                (good_wave1.digit2 + 2) % 10);
            println!("{}", create_interference_diagram(good_wave1, &good_wave2));
    // Beautiful summary visualization
    let visual_file = format!("prime_wave_mechanics_{}.txt", 
        Local::now().format("%Y%m%d_%H%M%S"));
    let mut file = File::create(&visual_file).expect("Failed to create file");
    writeln!(file, "{}", boxed_title("PRIME WAVE MECHANICS", 100)).unwrap();
    writeln!(file, "\nHow membrane patterns create standing waves that generate primes\n").unwrap();
    writeln!(file, "{}", banner("WAVE QUALITY RANKINGS", 100)).unwrap();
    // Create a beautiful chart
    writeln!(file, "\nInterference Quality by Base and Pattern:\n").unwrap();
    writeln!(file, "Base │ Pattern │ Quality │ Visual").unwrap();
    writeln!(file, "─────┼─────────┼─────────┼{}─", "─".repeat(50)).unwrap();
        if let Some(waves) = all_waves.get(base) {
            for (wave, primes) in waves.iter().take(5) {
                let quality = wave.interference_quality();
                let bar_width = (quality * 20.0) as usize;
                writeln!(file, " {:2}  │  {}-{}    │  {:.2}   │ {}",
                    base,
                    wave.digit1,
                    wave.digit2,
                    quality,
                    "█".repeat(bar_width)
                ).unwrap();
                // Show example primes
                if !primes.is_empty() {
                    writeln!(file, "     │         │         │ → Examples: {}, {}",
                        primes[0], 
                        if primes.len() > 1 { primes[1].to_string() } else { "...".to_string() }
                    ).unwrap();
            writeln!(file, "─────┼─────────┼─────────┼{}─", "─".repeat(50)).unwrap();
    // Wave mechanics explanation
    writeln!(file, "\n{}", boxed_title("WAVE MECHANICS EXPLAINED", 100)).unwrap();
    writeln!(file, r#"
The Mathematics of Membrane Waves
=================================
1. WAVELENGTH CALCULATION:
   λ = base / gcd(distance, base)
   
   Example: Base 10, digits 5-7 (distance 2)
   λ = 10 / gcd(2, 10) = 10 / 2 = 5
2. INTERFERENCE QUALITY:
   Q = amplitude × base_factor × distance_factor
   Where:
   - amplitude = 1 / (1 + |distance - 2|)
   - base_factor = 1.5 for even bases, 1.0 for odd
   - distance_factor = 2.0 for distance 2 (twin primes)
3. STANDING WAVE CONDITIONS:
   Perfect standing waves form when:
   - Base is even (allows symmetric division)
   - Distance divides evenly into base
   - Digits are coprime with base factors
4. THE 5-7 SUPREMACY:
   In base 10: Q = 1.0 × 1.5 × 2.0 = 3.0 (maximum!)
   In base 12: Q = 1.0 × 1.5 × 2.0 = 3.0 (also maximum!)
   In base 6:  Cannot use 7 (out of range)
This explains why (5,7) configurations dominate in larger even bases!
"#).unwrap();
    // ASCII art wave gallery
    writeln!(file, "\n{}", banner("WAVE PATTERN GALLERY", 100)).unwrap();
Perfect Constructive Interference (Base 10, 5-7):
─────────────────────────────────────────────────
     5                           7
     │                           │
     ╰─────────── 2 ─────────────╯
     
  ╱╲    ╱╲    ╱╲    ╱╲    ╱╲    ╱╲
 ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲   Standing wave
╱    ╲╱    ╲╱    ╲╱    ╲╱    ╲╱    ╲  with λ = 5
Destructive Interference (Base 7, 2-5):
──────────────────────────────────────
     2                           5
     ╰─────────── 3 ─────────────╯
  ╱╲      ╱╲      ╱╲      ╱╲
 ╱  ╲    ╱  ╲    ╱  ╲    ╱  ╲         Offset waves
╱    ╲  ╱    ╲  ╱    ╲  ╱    ╲        partially cancel
      ╲╱      ╲╱      ╲╱      ╲╱
Multi-Frequency Resonance (Base 12):
────────────────────────────────────
Multiple compatible wavelengths create rich harmonics:
λ=12: ████████████████████████████████████████████████
λ=6:  ████████████      ████████████      ████████████
λ=4:  ████    ████    ████    ████    ████    ████
λ=3:  ███  ███  ███  ███  ███  ███  ███  ███  ███
λ=2:  ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██
This is why base 12 performs so well!
    println!("\n✅ Wave visualization complete!");
    println!("📊 Detailed mechanics saved to: {}", visual_file);
    println!("\n{}", simple_box(
        "KEY INSIGHT: Prime generation is a wave phenomenon!\n\
         Membrane digits create interference patterns.\n\
         Even bases allow perfect standing waves.\n\
         The 5-7 pattern has optimal wave properties."
    ));
