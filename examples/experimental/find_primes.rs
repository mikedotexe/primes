//! Simple example: Finding primes with membrane configurations
//! 
//! This example shows the basic usage of the prime generator library

use prime_physics_engine::core::membrane::MembraneConfig;
use prime_physics_engine::patterns::exclusivity::ExclusivityAnalyzer;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    println!("Prime Generator - Basic Example");
    println!("==============================\n");
    
    // Example 1: Generate a specific prime
    println!("1. Generating a specific prime:");
    let config = MembraneConfig::new(10, 3, 7, 2, 2)?;
    let prime = config.generate(3)?;
    println!("   Configuration: {}", config.description());
    println!("   Seed: 3");
    println!("   Generated: {}", prime);
    println!("   Verify at: https://www.wolframalpha.com/input?i=is+{}+prime\n", prime);
    // Example 2: Test configuration viability
    println!("2. Testing configuration viability:");
    let report = config.test_viability();
    println!("   Success rate: {:.1}%", report.success_rate * 100.0);
    println!("   Working seeds: {:?}", report.working_seeds);
    println!("   Example primes: {} found\n", report.example_primes.len());
    // Example 3: Find exclusive configuration
    println!("3. Finding exclusive configurations:");
    let exclusive_config = MembraneConfig::new(10, 3, 3, 1, 1)?;
    let exclusive_report = exclusive_config.test_viability();
    if exclusive_report.is_exclusive {
        println!("   Found exclusive configuration: {}", exclusive_config.description());
        println!("   Only seed {} works!", exclusive_report.exclusive_seed.unwrap());
        if let Some((seed, prime)) = exclusive_report.example_primes.first() {
            println!("   Generates: {} (from seed {})", prime, seed);
        }
    }
    println!();
    // Example 4: Breathing membrane
    println!("4. Testing breathing membrane:");
    let breathing_config = MembraneConfig::new(10, 3, 3, 0, 1)?;
    println!("   Configuration: {}", breathing_config.description());
    println!("   Is breathing: {}", breathing_config.is_breathing());
    println!("   Breathing ratio: {:?}", breathing_config.breathing_ratio());
    let breathing_report = breathing_config.test_viability();
    println!("   Success rate: {:.1}%", breathing_report.success_rate * 100.0);
    println!("   Working seeds: {:?}\n", breathing_report.working_seeds);
    // Example 5: Pattern search
    println!("5. Searching for patterns:");
    let mut analyzer = ExclusivityAnalyzer::new();
    // Search a small space
    let exclusive_configs = analyzer.find_exclusive_configurations(
        10,                    // base
        &[1, 3, 7, 9],        // outer digits
        &[1, 3, 5, 7, 9],     // inner digits  
        &[0, 1, 2],           // k values
    );
    println!("   Found {} exclusive configurations", exclusive_configs.len());
    // Show top 3
    for (i, analysis) in exclusive_configs.iter().take(3).enumerate() {
        if let Some(seed) = analysis.viability.exclusive_seed {
            println!("   {}. {} → only seed {} works", 
                i + 1,
                analysis.config.description(),
                seed
            );
    // Pattern analysis
    let patterns = analyzer.analyze_exclusivity_patterns();
    if let Some(common_seed) = patterns.most_common_seed {
        println!("\n   Most common exclusive seed: {}", common_seed);
    Ok(())
}
