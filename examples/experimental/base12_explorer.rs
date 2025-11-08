//! Base 12 Explorer - Discover the unique properties of duodecimal membrane primes
//! 
//! Base 12 is special because:
//! - 12 = 2² × 3 (highly composite)
//! - Natural for time (12 hours) and geometry (360° = 30 × 12)
//! - More divisors means different resonance patterns

use prime_physics_engine::{
    is_prime_miller_rabin,
};
use std::fs::File;
#[derive(Debug, Serialize, Deserialize)]
struct Base12Analysis {
    timestamp: String,
    overview: Base12Overview,
    optimal_configurations: Vec<ConfigurationResult>,
    digit_analysis: DigitAnalysis,
    unique_discoveries: Vec<UniqueFind>,
    comparison_with_base10: ComparisonData,
}
struct Base12Overview {
    base_properties: BaseProperties,
    total_configs_tested: u32,
    best_density_achieved: f64,
    best_configuration: String,
struct BaseProperties {
    value: u32,
    factors: Vec<u32>,
    euler_totient: u32,
    divisor_count: u32,
struct ConfigurationResult {
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    seeds_tested: u32,
    primes_found: u32,
    density: f64,
    example_primes: Vec<String>,
    pattern_type: String,
struct DigitAnalysis {
    best_outer_digits: Vec<(u32, f64)>, // (digit, avg_density)
    best_inner_digits: Vec<(u32, f64)>,
    coprime_to_12: Vec<u32>, // 1, 5, 7, 11
    performance_by_digit_type: HashMap<String, f64>,
struct UniqueFind {
    description: String,
    configuration: String,
    value: String,
    special_property: String,
struct ComparisonData {
    base10_best_density: f64,
    base12_best_density: f64,
    improvement_factor: f64,
    structural_differences: Vec<String>,
fn display_base12(n: &num_bigint::BigUint) -> String {
    // Convert to base 12 representation with A=10, B=11
    if n.is_zero() {
        return "0".to_string();
    }
    
    let mut result = Vec::new();
    let mut num = n.clone();
    let twelve = num_bigint::BigUint::from(12u32);
    while !num.is_zero() {
        let digit = (&num % &twelve).to_u32().unwrap();
        result.push(match digit {
            10 => 'A',
            11 => 'B',
            d => (d as u8 + b'0') as char,
        });
        num = num / &twelve;
    result.reverse();
    result.into_iter().collect()
fn analyze_digit_performance(results: &[ConfigurationResult]) -> DigitAnalysis {
    let mut outer_performance: HashMap<u32, Vec<f64>> = HashMap::new();
    let mut inner_performance: HashMap<u32, Vec<f64>> = HashMap::new();
    for result in results {
        outer_performance.entry(result.outer).or_insert_with(Vec::new).push(result.density);
        inner_performance.entry(result.inner).or_insert_with(Vec::new).push(result.density);
    let best_outer: Vec<(u32, f64)> = outer_performance.into_iter()
        .map(|(digit, densities)| {
            let avg = densities.iter().sum::<f64>() / densities.len() as f64;
            (digit, avg)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|(_, avg)| *avg > 0.0)
        .take(5)
        .collect();
    let best_inner: Vec<(u32, f64)> = inner_performance.into_iter()
    // Analyze coprime vs non-coprime digits
    let coprime_digits = vec![1, 5, 7, 11];
    let mut performance_by_type = HashMap::new();
    let coprime_results: Vec<f64> = results.iter()
        .filter(|r| coprime_digits.contains(&r.outer) || coprime_digits.contains(&r.inner))
        .map(|r| r.density)
    let non_coprime_results: Vec<f64> = results.iter()
        .filter(|r| !coprime_digits.contains(&r.outer) && !coprime_digits.contains(&r.inner))
    if !coprime_results.is_empty() {
        performance_by_type.insert(
            "coprime_digits".to_string(),
            coprime_results.iter().sum::<f64>() / coprime_results.len() as f64
        );
    if !non_coprime_results.is_empty() {
            "non_coprime_digits".to_string(),
            non_coprime_results.iter().sum::<f64>() / non_coprime_results.len() as f64
    DigitAnalysis {
        best_outer_digits: best_outer,
        best_inner_digits: best_inner,
        coprime_to_12: coprime_digits,
        performance_by_digit_type: performance_by_type,
fn main() {
    println!("🔢 Base 12 Membrane Explorer");
    println!("============================\n");
    println!("Base 12 Properties:");
    println!("  Value: 12 = 2² × 3");
    println!("  Digits: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A (10), B (11)");
    println!("  Coprime digits: 1, 5, 7, B (11)");
    println!("  Euler's totient φ(12) = 4\n");
    let base = 12u32;
    let num_seeds = 500; // Test fewer seeds but more configurations
    let mut all_results = Vec::new();
    let mut unique_finds = Vec::new();
    // Test configurations systematically
    println!("Testing configurations...\n");
    // Focus on promising digit combinations
    let test_digits = vec![1, 5, 7, 11]; // Coprime to 12
    let all_digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    for &outer in &test_digits {
        for &inner in &all_digits {
            if outer == inner { continue; }
            
            for k_outer in 0..=2 {
                for k_inner in 0..=2 {
                    let config = MembraneConfig { outer, inner, k_outer, k_inner };
                    let mut primes_found = 0;
                    let mut prime_examples = Vec::new();
                    
                    for seed in 1..=num_seeds {
                        let candidate = generate_prime_candidate(&config, &seed.to_string(), base);
                        
                        if is_prime_miller_rabin(&candidate, 20) {
                            primes_found += 1;
                            if prime_examples.len() < 3 {
                                let base12_repr = display_base12(&candidate);
                                prime_examples.push(format!("{} (base12: {})", candidate, base12_repr));
                            }
                        }
                    }
                    let density = primes_found as f64 / num_seeds as f64;
                    if density > 0.0 {
                        let pattern_type = if k_outer == k_inner { "Symmetric" } else { "Breathing" };
                        all_results.push(ConfigurationResult {
                            outer,
                            inner,
                            k_outer,
                            k_inner,
                            seeds_tested: num_seeds,
                            primes_found,
                            density,
                            example_primes: prime_examples.clone(),
                            pattern_type: pattern_type.to_string(),
                        });
                        if density > 0.25 {
                            println!("✨ High density: ({},{}) k=({},{}) → {:.1}% primes", 
                                outer, inner, k_outer, k_inner, density * 100.0);
                            if !prime_examples.is_empty() {
                                println!("   Example: {}", prime_examples[0]);
                }
            }
        }
    // Sort by density
    all_results.sort_by(|a, b| b.density.partial_cmp(&a.density).unwrap());
    // Find unique properties
    println!("\n🔍 Searching for unique base-12 properties...\n");
    // Test exclusive configurations
    for config in all_results.iter().take(10) {
        let membrane_config = MembraneConfig {
            outer: config.outer,
            inner: config.inner,
            k_outer: config.k_outer,
            k_inner: config.k_inner,
        };
        
        let mut prime_seeds = Vec::new();
        for seed in 0..=11 {
            let candidate = generate_prime_candidate(&membrane_config, &seed.to_string(), base);
            if is_prime_miller_rabin(&candidate, 20) {
                prime_seeds.push(seed);
        if prime_seeds.len() == 1 {
            let exclusive_seed = prime_seeds[0];
            let value = generate_prime_candidate(&membrane_config, &exclusive_seed.to_string(), base);
            unique_finds.push(UniqueFind {
                description: "Exclusive configuration".to_string(),
                configuration: format!("({},{}) k=({},{}) seed={}", 
                    config.outer, config.inner, config.k_outer, config.k_inner, exclusive_seed),
                value: value.to_string(),
                special_property: format!("Works ONLY with seed {}", exclusive_seed),
            });
            println!("🌟 Found exclusive: {} with seed {} only!", value, exclusive_seed);
    // Analyze results
    let digit_analysis = analyze_digit_performance(&all_results);
    let best_config = all_results.first().unwrap();
    let best_density = best_config.density;
    // Build final analysis
    let analysis = Base12Analysis {
        timestamp: Local::now().to_rfc3339(),
        overview: Base12Overview {
            base_properties: BaseProperties {
                value: 12,
                factors: vec![2, 2, 3],
                euler_totient: 4,
                divisor_count: 6,
            },
            total_configs_tested: all_results.len() as u32,
            best_density_achieved: best_density,
            best_configuration: format!("({},{}) k=({},{})", 
                best_config.outer, best_config.inner, best_config.k_outer, best_config.k_inner),
        },
        optimal_configurations: all_results.iter().take(10).cloned().collect(),
        digit_analysis,
        unique_discoveries: unique_finds,
        comparison_with_base10: ComparisonData {
            base10_best_density: 0.223, // Known best for base 10
            base12_best_density: best_density,
            improvement_factor: best_density / 0.223,
            structural_differences: vec![
                "Base 12 has more divisors (6 vs 4)".to_string(),
                "Coprime digits in base 12: {1,5,7,11} vs {1,3,7,9} in base 10".to_string(),
                "Base 12 allows natural thirds and quarters".to_string(),
            ],
    };
    // Save results
    let filename = format!("base12_analysis_{}.json", Local::now().format("%Y%m%d_%H%M%S"));
    let mut file = File::create(&filename).expect("Failed to create file");
    let json = serde_json::to_string_pretty(&analysis).expect("Failed to serialize");
    file.write_all(json.as_bytes()).expect("Failed to write file");
    // Print summary
    println!("\n" + &"=".repeat(60));
    println!("📊 BASE 12 SUMMARY");
    println!("=".repeat(60));
    println!("Best configuration: {}", analysis.overview.best_configuration);
    println!("Best density: {:.1}%", best_density * 100.0);
    println!("Total configurations tested: {}", analysis.overview.total_configs_tested);
    println!("\n🏆 Top 5 Configurations:");
    for (i, config) in all_results.iter().take(5).enumerate() {
        println!("{}. ({},{}) k=({},{}) → {:.1}% {} pattern", 
            i + 1, config.outer, config.inner, config.k_outer, config.k_inner,
            config.density * 100.0, config.pattern_type);
    println!("\n💎 Best digits for base 12:");
    println!("Outer: {:?}", digit_analysis.best_outer_digits.iter().take(3).collect::<Vec<_>>());
    println!("Inner: {:?}", digit_analysis.best_inner_digits.iter().take(3).collect::<Vec<_>>());
    if !analysis.unique_discoveries.is_empty() {
        println!("\n🌟 Unique discoveries:");
        for discovery in &analysis.unique_discoveries {
            println!("  {}: {}", discovery.description, discovery.special_property);
    println!("\n✅ Full analysis saved to: {}", filename);
    // Show some base-12 representation examples
    println!("\n🔢 Base 12 Prime Examples:");
    if let Some(best) = all_results.first() {
        for example in best.example_primes.iter().take(3) {
            println!("  {}", example);
