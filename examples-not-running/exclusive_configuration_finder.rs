//! Exclusive Configuration Finder - Discovers configurations that work with exactly one seed
//! 
//! This example:
//! 1. Systematically searches for exclusive configurations
//! 2. Verifies exclusivity by testing all seeds in range
//! 3. Provides factorizations for non-prime results
//! 4. Outputs detailed results to timestamped JSON file

use primes::{
    is_prime_miller_rabin,
};
use std::fs::File;
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExclusiveConfiguration {
    base: u32,
    config: (u32, u32, u32, u32), // (outer, inner, k_outer, k_inner)
    exclusive_seed: u32,
    prime_value: String,
    all_results: Vec<SeedResult>,
}
struct SeedResult {
    seed: u32,
    value: String,
    is_prime: bool,
    factors: Option<Vec<u32>>,
#[derive(Debug, Serialize, Deserialize)]
struct ExclusivityAnalysis {
    timestamp: String,
    search_parameters: SearchParameters,
    exclusive_configs_found: Vec<ExclusiveConfiguration>,
    statistics: Statistics,
struct SearchParameters {
    bases_tested: Vec<u32>,
    outer_range: (u32, u32),
    inner_range: (u32, u32),
    k_range: (u32, u32),
    max_seed: u32,
struct Statistics {
    total_configurations_tested: u32,
    exclusive_configurations_found: u32,
    exclusive_percentage: f64,
    most_common_exclusive_seed: Option<u32>,
fn find_small_factors(n: &BigUint, max_factor: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut remaining = n.clone();
    
    for p in 2..=max_factor {
        while &remaining % p == 0u32 {
            factors.push(p);
            remaining = remaining / p;
        }
        if remaining == BigUint::one() {
            break;
    }
    factors
fn test_configuration_exclusivity(
    base: u32, 
    outer: u32, 
    inner: u32, 
    k_outer: u32, 
    k_inner: u32, 
    max_seed: u32
) -> Option<ExclusiveConfiguration> {
    let config = MembraneConfig { outer, inner, k_outer, k_inner };
    let mut prime_seeds = Vec::new();
    let mut all_results = Vec::new();
    for seed in 0..=max_seed {
        let candidate = generate_prime_candidate(&config, &seed.to_string(), base);
        let is_prime = is_prime_miller_rabin(&candidate, 20);
        
        let factors = if !is_prime {
            Some(find_small_factors(&candidate, 1000))
        } else {
            None
        };
        all_results.push(SeedResult {
            seed,
            value: candidate.to_string(),
            is_prime,
            factors,
        });
        if is_prime {
            prime_seeds.push(seed);
    // Check if exactly one seed produces a prime
    if prime_seeds.len() == 1 {
        let exclusive_seed = prime_seeds[0];
        let prime_value = all_results[exclusive_seed as usize].value.clone();
        Some(ExclusiveConfiguration {
            base,
            config: (outer, inner, k_outer, k_inner),
            exclusive_seed,
            prime_value,
            all_results,
        })
    } else {
        None
fn main() {
    println!("🔍 Exclusive Configuration Finder");
    println!("================================\n");
    // Search parameters
    let bases = vec![6, 10, 12];
    let outer_range = (1, 9);
    let inner_range = (1, 9);
    let k_range = (0, 3);
    let max_seed = 9;
    println!("Search Parameters:");
    println!("  Bases: {:?}", bases);
    println!("  Outer digits: {} to {}", outer_range.0, outer_range.1);
    println!("  Inner digits: {} to {}", inner_range.0, inner_range.1);
    println!("  k values: {} to {}", k_range.0, k_range.1);
    println!("  Seeds tested: 0 to {}\n", max_seed);
    let mut exclusive_configs = Vec::new();
    let mut total_tested = 0;
    println!("🔎 Searching for exclusive configurations...\n");
    for base in &bases {
        println!("Base {}:", base);
        let mut base_exclusive_count = 0;
        for outer in outer_range.0..=outer_range.1 {
            for inner in inner_range.0..=inner_range.1 {
                for k_outer in k_range.0..=k_range.1 {
                    for k_inner in k_range.0..=k_range.1 {
                        total_tested += 1;
                        
                        if let Some(exclusive) = test_configuration_exclusivity(
                            *base, outer, inner, k_outer, k_inner, max_seed
                        ) {
                            base_exclusive_count += 1;
                            println!("  ✨ Found: ({},{}) k=({},{}) → seed {} only → {}", 
                                outer, inner, k_outer, k_inner, 
                                exclusive.exclusive_seed, exclusive.prime_value);
                            
                            exclusive_configs.push(exclusive);
                        }
                    }
                }
            }
        println!("  Total exclusive for base {}: {}\n", base, base_exclusive_count);
    // Analyze exclusive seeds
    let mut seed_counts = std::collections::HashMap::new();
    for config in &exclusive_configs {
        *seed_counts.entry(config.exclusive_seed).or_insert(0) += 1;
    let most_common_seed = seed_counts.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(seed, _)| *seed);
    // Build analysis results
    let analysis = ExclusivityAnalysis {
        timestamp: Local::now().to_rfc3339(),
        search_parameters: SearchParameters {
            bases_tested: bases,
            outer_range,
            inner_range,
            k_range,
            max_seed,
        },
        exclusive_configs_found: exclusive_configs.clone(),
        statistics: Statistics {
            total_configurations_tested: total_tested,
            exclusive_configurations_found: exclusive_configs.len() as u32,
            exclusive_percentage: (exclusive_configs.len() as f64 / total_tested as f64) * 100.0,
            most_common_exclusive_seed: most_common_seed,
    };
    // Save to file
    let filename = format!("exclusive_configs_{}.json", Local::now().format("%Y%m%d_%H%M%S"));
    let mut file = File::create(&filename).expect("Failed to create output file");
    let json = serde_json::to_string_pretty(&analysis).expect("Failed to serialize results");
    file.write_all(json.as_bytes()).expect("Failed to write results");
    println!("📊 Summary:");
    println!("==========");
    println!("Total configurations tested: {}", total_tested);
    println!("Exclusive configurations found: {}", exclusive_configs.len());
    println!("Exclusivity rate: {:.2}%", analysis.statistics.exclusive_percentage);
    if let Some(seed) = most_common_seed {
        println!("Most common exclusive seed: {}", seed);
    // Show detailed example if we found any
    if let Some(example) = exclusive_configs.first() {
        println!("\n📋 Detailed Example:");
        println!("==================");
        println!("Configuration: base {}, ({},{}) k=({},{})", 
            example.base, example.config.0, example.config.1, example.config.2, example.config.3);
        println!("Exclusive seed: {}", example.exclusive_seed);
        println!("\nAll seed results:");
        for result in &example.all_results {
            if result.is_prime {
                println!("  Seed {}: {} ✓ PRIME", result.seed, result.value);
            } else if let Some(factors) = &result.factors {
                let factor_str = factors.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(" × ");
                println!("  Seed {}: {} = {} (composite)", result.seed, result.value, factor_str);
    println!("\n✅ Analysis complete! Results saved to: {}", filename);
    // Verification message
    println!("\n🔍 To verify any prime, visit:");
    println!("   https://www.wolframalpha.com/input/?i=isprime(NUMBER)");
