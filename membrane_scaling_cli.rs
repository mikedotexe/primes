// Membrane Scaling Law Analyzer
// ==============================
//
// Historical Research Instrument - November 2025
//
// This tool was developed to test the hypothesis that optimal membrane padding scales
// as k* ∝ M^(1/2), potentially connecting to the Riemann zeta critical line ζ(1/2 + it).
//
// FINDINGS (Nov 22, 2025):
// ✅ Hypothesis REFUTED: β ≈ 0 (not 0.5)
// ✅ Minimal Padding Principle CONFIRMED: k* = 0 for M ≥ 2 across all bases
// 🎯 NEW DISCOVERY: Diameter-Density Law (ρ > 0.77, p < 10^-20)
//    - Compactness (1/total_digits) strongly predicts prime density
//    - Aligns with k-tuple minimal constellation theory
// 🎯 Cross-base universality: Pattern holds for bases 6, 10, 30
//
// See MEMBRANE_SCALING_EXPLORATION.md for complete analysis.
//
// Research Objective (Original):
// Determine if optimal membrane spacing exhibits power-law behavior with exponents
// that correlate to Riemann zeta function critical parameters.
//
// Theoretical Framework:
// - Membranes represent geometric templates with enhanced prime density
// - Optimal spacing k minimizes padding while maximizing primality
// - Scaling law k_opt(M) may reveal fundamental geometric principles
// - Connection to ζ(s) critical line (Re(s)=1/2) via information geometry
//
// Methodology:
// 1. Systematic parameter sweep across (base, outer, inner, k_outer, k_inner, middle_length)
// 2. Primality testing with statistical rigor (Miller-Rabin + deterministic verification)
// 3. Density measurement with gap statistics
// 4. Regression analysis to extract scaling exponents
// 5. Multi-dimensional signal hunting (loosely held hypotheses)
//
// Technical Achievement:
// - Upgraded to u128 arithmetic to handle large bases (base-10, base-30)
// - Processes ~870M candidates for base-30 full sweep
// - Gap statistics and example prime collection

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

// ============================================================================
// CORE DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct MembraneConfig {
    base: u32,
    outer: u32,
    inner: u32,
    k_outer: usize,
    k_inner: usize,
    middle_length: usize,
}

impl MembraneConfig {
    fn total_padding(&self) -> usize {
        2 * (self.k_outer + self.k_inner)
    }
    
    fn total_digits(&self) -> usize {
        2 + // outer boundaries
        self.total_padding() +
        self.middle_length
    }
    
    fn descriptor(&self) -> String {
        format!("B{}({},{})_k({},{})_M{}", 
                self.base, self.outer, self.inner,
                self.k_outer, self.k_inner, self.middle_length)
    }
}

#[derive(Debug, Clone)]
struct DensityMeasurement {
    config: MembraneConfig,
    total_tested: u64,
    primes_found: u64,
    composites_found: u64,
    density: f64,
    
    // Gap statistics
    gaps: Vec<u64>,
    mean_gap: f64,
    std_gap: f64,
    min_gap: u64,
    max_gap: u64,
    
    // Timing
    elapsed_ms: u128,
    
    // Example primes (first 10)
    example_primes: Vec<u64>,
}

impl DensityMeasurement {
    fn gap_ratio(&self) -> f64 {
        if self.min_gap > 0 {
            self.max_gap as f64 / self.min_gap as f64
        } else {
            0.0
        }
    }
    
    fn relative_std(&self) -> f64 {
        if self.mean_gap > 0.0 {
            self.std_gap / self.mean_gap
        } else {
            0.0
        }
    }
}

#[derive(Debug)]
struct ScalingAnalysis {
    base: u32,
    outer: u32,
    inner: u32,
    measurements: Vec<DensityMeasurement>,
    optimal_configs: HashMap<usize, (usize, usize, f64)>, // middle_length -> (k_outer, k_inner, density)
}

// ============================================================================
// PRIMALITY TESTING - Production Grade
// ============================================================================

/// Miller-Rabin primality test with deterministic witnesses for u64 range
fn is_prime_miller_rabin(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 { return false; }
    
    // Write n-1 as d * 2^r
    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    
    // Deterministic witnesses for u64 range
    let witnesses = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    
    'witness: for &a in &witnesses {
        if a >= n { continue; }
        
        let mut x = mod_pow(a, d, n);
        
        if x == 1 || x == n - 1 {
            continue 'witness;
        }
        
        for _ in 0..r-1 {
            x = mod_mul(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        
        return false;
    }
    
    true
}

/// Modular exponentiation: (base^exp) mod m
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0; }
    
    let mut result = 1u64;
    base %= modulus;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, modulus);
        }
        exp >>= 1;
        base = mod_mul(base, base, modulus);
    }
    
    result
}

/// Modular multiplication avoiding overflow
fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

/// Trial division for small primes (optimization)
fn has_small_prime_factor(n: u64) -> bool {
    const SMALL_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    
    for &p in &SMALL_PRIMES {
        if n == p { return false; }
        if n % p == 0 { return true; }
    }
    false
}

/// Combined primality test (trial division + Miller-Rabin)
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if has_small_prime_factor(n) { return false; }
    is_prime_miller_rabin(n)
}

// ============================================================================
// MEMBRANE CONSTRUCTION
// ============================================================================

struct MembraneBuilder {
    config: MembraneConfig,
}

impl MembraneBuilder {
    fn new(config: MembraneConfig) -> Self {
        Self { config }
    }
    
    /// Construct membrane value from middle seed
    ///
    /// Structure: outer [k_outer×0] inner [k_inner×0] MIDDLE [k_inner×0] inner [k_outer×0] outer
    ///
    /// Example (Base 6, outer=1, inner=5, k_outer=0, k_inner=0, middle=3):
    ///   1 5 3 5 1  →  15351 in base 6
    ///
    /// Note: Uses u128 internally to avoid overflow with larger bases,
    /// but returns u64 (None if value exceeds u64::MAX)
    fn construct(&self, middle: u64) -> Option<u64> {
        let base = self.config.base as u128;
        let mut value = 0u128;
        let mut position = 0u32;

        // Build from right to left (least significant to most significant)

        // Right outer boundary
        value = value.checked_add(
            self.config.outer as u128 * base.checked_pow(position)?
        )?;
        position += 1;

        // Right k_outer zeros (just advance position)
        position += self.config.k_outer as u32;

        // Right inner boundary
        value = value.checked_add(
            self.config.inner as u128 * base.checked_pow(position)?
        )?;
        position += 1;

        // Right k_inner zeros
        position += self.config.k_inner as u32;

        // MIDDLE (variable seed)
        value = value.checked_add(
            middle as u128 * base.checked_pow(position)?
        )?;
        position += self.config.middle_length as u32;

        // Left k_inner zeros
        position += self.config.k_inner as u32;

        // Left inner boundary
        value = value.checked_add(
            self.config.inner as u128 * base.checked_pow(position)?
        )?;
        position += 1;

        // Left k_outer zeros
        position += self.config.k_outer as u32;

        // Left outer boundary
        value = value.checked_add(
            self.config.outer as u128 * base.checked_pow(position)?
        )?;

        // Convert to u64 for primality testing (return None if too large)
        u64::try_from(value).ok()
    }
    
    /// Generate all valid membranes for this configuration
    fn generate_all(&self) -> Vec<u64> {
        // Calculate maximum middle value (base^middle_length)
        // Use u128 to avoid overflow, but cap at u64::MAX for iteration
        let middle_max_128 = (self.config.base as u128).pow(self.config.middle_length as u32);
        let middle_max = middle_max_128.min(u64::MAX as u128) as u64;

        (0..middle_max)
            .filter_map(|m| self.construct(m))
            .collect()
    }
}

// ============================================================================
// SCALING LAW ANALYZER
// ============================================================================

struct ScalingLawAnalyzer {
    base: u32,
    outer: u32,
    inner: u32,
    max_middle_length: usize,
    max_k: usize,
    verbose: bool,
}

impl ScalingLawAnalyzer {
    fn new(base: u32, outer: u32, inner: u32, max_middle_length: usize, max_k: usize) -> Self {
        Self {
            base,
            outer,
            inner,
            max_middle_length,
            max_k,
            verbose: true,
        }
    }
    
    /// Measure prime density for a single configuration
    fn measure_density(&self, config: &MembraneConfig) -> DensityMeasurement {
        let start = Instant::now();
        let builder = MembraneBuilder::new(config.clone());
        
        let candidates = builder.generate_all();
        let total_tested = candidates.len() as u64;
        
        let mut primes_found = 0u64;
        let mut gaps = Vec::new();
        let mut last_prime = 0u64;
        let mut example_primes = Vec::new();
        
        for &candidate in &candidates {
            if is_prime(candidate) {
                primes_found += 1;
                
                if example_primes.len() < 10 {
                    example_primes.push(candidate);
                }
                
                if last_prime > 0 {
                    gaps.push(candidate - last_prime);
                }
                last_prime = candidate;
            }
        }
        
        let composites_found = total_tested - primes_found;
        let density = if total_tested > 0 {
            primes_found as f64 / total_tested as f64
        } else {
            0.0
        };
        
        let (mean_gap, std_gap, min_gap, max_gap) = if !gaps.is_empty() {
            let mean = gaps.iter().sum::<u64>() as f64 / gaps.len() as f64;
            let variance = gaps.iter()
                .map(|&g| {
                    let diff = g as f64 - mean;
                    diff * diff
                })
                .sum::<f64>() / gaps.len() as f64;
            let std = variance.sqrt();
            let min = *gaps.iter().min().unwrap();
            let max = *gaps.iter().max().unwrap();
            (mean, std, min, max)
        } else {
            (0.0, 0.0, 0, 0)
        };
        
        let elapsed_ms = start.elapsed().as_millis();
        
        DensityMeasurement {
            config: config.clone(),
            total_tested,
            primes_found,
            composites_found,
            density,
            gaps,
            mean_gap,
            std_gap,
            min_gap,
            max_gap,
            elapsed_ms,
            example_primes,
        }
    }
    
    /// Find optimal k configuration for given middle length
    fn find_optimal_spacing(&self, middle_length: usize) -> (usize, usize, f64, DensityMeasurement) {
        if self.verbose {
            println!("  Searching optimal k for M={}...", middle_length);
        }
        
        let mut best_density = 0.0;
        let mut best_measurement = None;
        let mut best_k_outer = 0;
        let mut best_k_inner = 0;
        
        for k_outer in 0..=self.max_k {
            for k_inner in 0..=self.max_k {
                let config = MembraneConfig {
                    base: self.base,
                    outer: self.outer,
                    inner: self.inner,
                    k_outer,
                    k_inner,
                    middle_length,
                };
                
                let measurement = self.measure_density(&config);
                
                if measurement.density > best_density {
                    best_density = measurement.density;
                    best_k_outer = k_outer;
                    best_k_inner = k_inner;
                    best_measurement = Some(measurement);
                }
            }
        }
        
        (best_k_outer, best_k_inner, best_density, best_measurement.unwrap())
    }
    
    /// Comprehensive analysis across all configurations
    fn analyze_all(&self) -> ScalingAnalysis {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║  MEMBRANE SCALING LAW ANALYSIS                        ║");
        println!("╚════════════════════════════════════════════════════════╝\n");
        println!("Configuration: Base-{} with boundaries ({}, {})", 
                 self.base, self.outer, self.inner);
        println!("Parameter Space: M ∈ [1,{}], k ∈ [0,{}]", 
                 self.max_middle_length, self.max_k);
        println!();
        
        let mut measurements = Vec::new();
        let mut optimal_configs = HashMap::new();
        
        println!("┌────────┬────────────┬────────────┬──────────┬──────────┬────────────┐");
        println!("│ Middle │ Optimal    │ Optimal    │ Density  │ Primes   │ Mean Gap   │");
        println!("│ Length │ k_outer    │ k_inner    │ (%)      │ Found    │            │");
        println!("├────────┼────────────┼────────────┼──────────┼──────────┼────────────┤");
        
        for middle_len in 1..=self.max_middle_length {
            let (k_out, k_in, density, measurement) = self.find_optimal_spacing(middle_len);
            
            println!("│ {:6} │ {:10} │ {:10} │ {:7.2}% │ {:8} │ {:10.2} │",
                     middle_len, k_out, k_in, density * 100.0,
                     measurement.primes_found, measurement.mean_gap);
            
            optimal_configs.insert(middle_len, (k_out, k_in, density));
            measurements.push(measurement);
        }
        
        println!("└────────┴────────────┴────────────┴──────────┴──────────┴────────────┘\n");
        
        ScalingAnalysis {
            base: self.base,
            outer: self.outer,
            inner: self.inner,
            measurements,
            optimal_configs,
        }
    }
    
    /// Full parameter sweep for regression analysis
    fn comprehensive_sweep(&self) -> Vec<DensityMeasurement> {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║  COMPREHENSIVE PARAMETER SWEEP                        ║");
        println!("╚════════════════════════════════════════════════════════╝\n");
        
        let total_configs = self.max_middle_length * (self.max_k + 1) * (self.max_k + 1);
        println!("Total configurations: {}", total_configs);
        println!("Estimated runtime: ~{} minutes\n", total_configs / 100);
        
        let mut measurements = Vec::new();
        let mut progress = 0;
        
        for middle_len in 1..=self.max_middle_length {
            for k_outer in 0..=self.max_k {
                for k_inner in 0..=self.max_k {
                    let config = MembraneConfig {
                        base: self.base,
                        outer: self.outer,
                        inner: self.inner,
                        k_outer,
                        k_inner,
                        middle_length: middle_len,
                    };
                    
                    let measurement = self.measure_density(&config);
                    measurements.push(measurement);
                    
                    progress += 1;
                    if progress % 10 == 0 {
                        print!("\rProgress: {}/{} ({:.1}%)", 
                               progress, total_configs, 
                               100.0 * progress as f64 / total_configs as f64);
                        std::io::stdout().flush().unwrap();
                    }
                }
            }
        }
        
        println!("\n\nSweep complete! {} measurements collected.\n", measurements.len());
        measurements
    }
}

// ============================================================================
// DATA EXPORT
// ============================================================================

fn export_measurements(measurements: &[DensityMeasurement], filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    
    // CSV Header
    writeln!(file, "base,outer,inner,k_outer,k_inner,middle_length,total_padding,total_digits,total_tested,primes_found,composites_found,density,mean_gap,std_gap,min_gap,max_gap,gap_ratio,relative_std,elapsed_ms,examples")?;
    
    // Data rows
    for m in measurements {
        let examples_str = m.example_primes.iter()
            .take(5)
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(";");
        
        writeln!(file, "{},{},{},{},{},{},{},{},{},{},{},{:.8},{:.4},{:.4},{},{},{:.4},{:.4},{},\"{}\"",
                 m.config.base,
                 m.config.outer,
                 m.config.inner,
                 m.config.k_outer,
                 m.config.k_inner,
                 m.config.middle_length,
                 m.config.total_padding(),
                 m.config.total_digits(),
                 m.total_tested,
                 m.primes_found,
                 m.composites_found,
                 m.density,
                 m.mean_gap,
                 m.std_gap,
                 m.min_gap,
                 m.max_gap,
                 m.gap_ratio(),
                 m.relative_std(),
                 m.elapsed_ms,
                 examples_str)?;
    }
    
    println!("✓ Exported {} measurements to {}", measurements.len(), filename);
    Ok(())
}

fn export_optimal_summary(analysis: &ScalingAnalysis, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    
    writeln!(file, "# Membrane Scaling Analysis Summary")?;
    writeln!(file, "# Base: {}, Boundaries: ({}, {})\n", 
             analysis.base, analysis.outer, analysis.inner)?;
    
    writeln!(file, "middle_length,optimal_k_outer,optimal_k_inner,optimal_k_total,max_density,primes_found,mean_gap")?;
    
    let mut keys: Vec<_> = analysis.optimal_configs.keys().collect();
    keys.sort();
    
    for &m_len in keys {
        if let Some(&(k_out, k_in, density)) = analysis.optimal_configs.get(&m_len) {
            if let Some(measurement) = analysis.measurements.iter()
                .find(|m| m.config.middle_length == m_len && 
                         m.config.k_outer == k_out && 
                         m.config.k_inner == k_in) {
                writeln!(file, "{},{},{},{},{:.8},{},{:.4}",
                         m_len, k_out, k_in, k_out + k_in,
                         density, measurement.primes_found, measurement.mean_gap)?;
            }
        }
    }
    
    println!("✓ Exported optimal configuration summary to {}", filename);
    Ok(())
}

// ============================================================================
// MAIN EXECUTION
// ============================================================================

fn main() -> std::io::Result<()> {
    println!("\n");
    println!("█████████████████████████████████████████████████████████████");
    println!("█                                                           █");
    println!("█  MEMBRANE SCALING LAW ANALYZER                          █");
    println!("█  Empirical Investigation of Prime Generation Geometry    █");
    println!("█                                                           █");
    println!("█████████████████████████████████████████████████████████████\n");
    
    // Configuration: Test the known optimal configurations
    let configurations = vec![
        (6, 1, 5, "base6_1_5"),     // Base-6 optimal from your research
        (10, 3, 7, "base10_3_7"),   // Base-10 configuration
        (30, 11, 7, "base30_11_7"), // Base-30 high-performance
    ];
    
    let max_middle_length = 6;  // Analyze M = 1 through 6
    let max_k = 3;               // Test k = 0,1,2,3
    
    for (base, outer, inner, label) in configurations {
        println!("\n{}", "=".repeat(60));
        println!("ANALYZING: Base-{} ({}, {}) - {}", base, outer, inner, label);
        println!("{}\n", "=".repeat(60));
        
        let analyzer = ScalingLawAnalyzer::new(base, outer, inner, max_middle_length, max_k);
        
        // Phase 1: Find optimal configurations
        let analysis = analyzer.analyze_all();
        
        // Phase 2: Comprehensive sweep for regression
        let all_measurements = analyzer.comprehensive_sweep();
        
        // Export results
        let csv_filename = format!("membrane_scaling_{}.csv", label);
        let summary_filename = format!("membrane_optimal_{}.txt", label);
        
        export_measurements(&all_measurements, &csv_filename)?;
        export_optimal_summary(&analysis, &summary_filename)?;
        
        println!("\n✓ Analysis complete for {}", label);
        println!("  - Full dataset: {}", csv_filename);
        println!("  - Optimal summary: {}", summary_filename);
    }
    
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║  ANALYSIS PIPELINE COMPLETE                           ║");
    println!("╠════════════════════════════════════════════════════════╣");
    println!("║                                                        ║");
    println!("║  Next Steps:                                           ║");
    println!("║  1. Run statistical_analysis.py for regression        ║");
    println!("║  2. Review scaling_laws_*.png visualizations          ║");
    println!("║  3. Examine zeta_connection_report.txt                ║");
    println!("║                                                        ║");
    println!("╚════════════════════════════════════════════════════════╝\n");
    
    Ok(())
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_primality() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(37573)); // From your base-10 example
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(100));
    }
    
    #[test]
    fn test_membrane_construction_base6() {
        let config = MembraneConfig {
            base: 6,
            outer: 1,
            inner: 5,
            k_outer: 0,
            k_inner: 0,
            middle_length: 1,
        };
        
        let builder = MembraneBuilder::new(config);
        
        // Middle = 3 should give: 1 5 3 5 1
        // In base 6: 1*6^4 + 5*6^3 + 3*6^2 + 5*6 + 1 = 1296 + 1080 + 108 + 30 + 1 = 2515
        let result = builder.construct(3).unwrap();
        let expected = 1*1296 + 5*216 + 3*36 + 5*6 + 1;
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_optimal_finding_base6() {
        let analyzer = ScalingLawAnalyzer::new(6, 1, 5, 2, 1);
        let (k_out, k_in, density, _) = analyzer.find_optimal_spacing(1);
        
        // Based on your research, k=(0,0) should be optimal
        assert_eq!(k_out, 0);
        assert_eq!(k_in, 0);
        assert!(density > 0.25); // Should achieve >25% density
    }
}
