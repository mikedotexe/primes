use std::io;//! Breathing Pattern Analyzer - Demonstrates why asymmetric k-values outperform symmetric ones
//! 
//! This example provides detailed analysis showing:
//! 1. Prime density differences between symmetric and asymmetric patterns
//! 2. Resonance analysis showing which primes "kill" the most candidates
//! 3. Statistical significance testing
//! 4. Outputs results to a timestamped JSON file for verification

use prime_physics_engine::{
    is_prime_miller_rabin,
};
use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize)]
struct BreathingAnalysisResults {
    timestamp: String,
    base: u32,
    boundary_digits: (u32, u32),
    patterns_tested: Vec<PatternResult>,
    resonance_analysis: HashMap<String, ResonanceData>,
    statistical_summary: StatisticalSummary,
    conclusions: Vec<String>,
}
struct PatternResult {
    k_values: (u32, u32),
    pattern_type: String,
    seeds_tested: u32,
    primes_found: u32,
    prime_density: f64,
    prime_examples: Vec<String>,
struct ResonanceData {
    pattern: (u32, u32),
    prime_kill_rates: Vec<(u32, f64)>, // (prime, kill_rate)
    total_survival_rate: f64,
struct StatisticalSummary {
    best_symmetric: PatternResult,
    best_asymmetric: PatternResult,
    improvement_percentage: f64,
    chi_square_statistic: f64,
    p_value_estimate: String,
fn analyze_pattern(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, num_seeds: u32) -> PatternResult {
    let config = MembraneConfig { outer, inner, k_outer, k_inner };
    let pattern_type = if k_outer == k_inner { "Symmetric" } else { "Asymmetric (Breathing)" };
    
    let mut primes_found = 0;
    let mut prime_examples = Vec::new();
    for seed in 1..=num_seeds {
        let candidate = generate_prime_candidate(&config, &seed.to_string(), base);
        if is_prime_miller_rabin(&candidate, 20) {
            primes_found += 1;
            if prime_examples.len() < 5 {
                prime_examples.push(format!("seed {} → {}", seed, candidate));
            }
        }
    }
    let prime_density = primes_found as f64 / num_seeds as f64;
    PatternResult {
        k_values: (k_outer, k_inner),
        pattern_type: pattern_type.to_string(),
        seeds_tested: num_seeds,
        primes_found,
        prime_density,
        prime_examples,
fn analyze_resonance(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, num_seeds: u32) -> ResonanceData {
    let test_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let mut kill_counts: HashMap<u32, u32> = HashMap::new();
    let mut total_candidates = 0;
    let mut survivors = 0;
        total_candidates += 1;
        
        let mut killed_by = None;
        for &prime in &test_primes {
            if &candidate % prime == 0u32 {
                killed_by = Some(prime);
                break;
        if let Some(prime) = killed_by {
            *kill_counts.entry(prime).or_insert(0) += 1;
        } else {
            survivors += 1;
    let mut prime_kill_rates: Vec<(u32, f64)> = kill_counts.into_iter()
        .map(|(prime, count)| (prime, count as f64 / total_candidates as f64))
        .collect();
    prime_kill_rates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    ResonanceData {
        pattern: (k_outer, k_inner),
        prime_kill_rates,
        total_survival_rate: survivors as f64 / total_candidates as f64,
fn calculate_chi_square(observed1: u32, total1: u32, observed2: u32, total2: u32) -> f64 {
    let expected1 = (observed1 + observed2) as f64 * total1 as f64 / (total1 + total2) as f64;
    let expected2 = (observed1 + observed2) as f64 * total2 as f64 / (total1 + total2) as f64;
    let chi1 = (observed1 as f64 - expected1).powi(2) / expected1;
    let chi2 = (observed2 as f64 - expected2).powi(2) / expected2;
    chi1 + chi2
fn main() {
    println!("🫁 Breathing Pattern Analysis");
    println!("============================\n");
    let base = 6u32;
    let (outer, inner) = (3, 3);
    let num_seeds = 1000;
    println!("Configuration:");
    println!("  Base: {}", base);
    println!("  Boundary digits: ({}, {})", outer, inner);
    println!("  Seeds to test: {}\n", num_seeds);
    // Test various k-patterns
    let k_patterns = vec![
        (0, 0),  // Tight symmetric
        (1, 1),  // Standard symmetric
        (2, 2),  // Loose symmetric
        (0, 1),  // Breathing right
        (1, 0),  // Breathing left
        (0, 2),  // Deep breathing right
        (2, 0),  // Deep breathing left
        (1, 2),  // Asymmetric loose
    ];
    let mut pattern_results = Vec::new();
    let mut resonance_data = HashMap::new();
    println!("📊 Testing Patterns:");
    println!("==================");
    for (k_outer, k_inner) in &k_patterns {
        print!("Testing k=({},{})... ", k_outer, k_inner);
        std::io::stdout().flush().unwrap();
        let result = analyze_pattern(base, outer, inner, *k_outer, *k_inner, num_seeds);
        let resonance = analyze_resonance(base, outer, inner, *k_outer, *k_inner, num_seeds);
        println!("{:.1}% primes ({})", result.prime_density * 100.0, 
            if k_outer == k_inner { "symmetric" } else { "breathing" });
        pattern_results.push(result);
        resonance_data.insert(format!("k_{}_{}", k_outer, k_inner), resonance);
    // Find best symmetric and asymmetric
    let best_symmetric = pattern_results.iter()
        .filter(|r| r.k_values.0 == r.k_values.1)
        .max_by(|a, b| a.prime_density.partial_cmp(&b.prime_density).unwrap())
        .unwrap()
        .clone();
    let best_asymmetric = pattern_results.iter()
        .filter(|r| r.k_values.0 != r.k_values.1)
    let improvement = (best_asymmetric.prime_density - best_symmetric.prime_density) / best_symmetric.prime_density * 100.0;
    // Calculate chi-square
    let chi_square = calculate_chi_square(
        best_symmetric.primes_found, best_symmetric.seeds_tested,
        best_asymmetric.primes_found, best_asymmetric.seeds_tested
    );
    let p_value = if chi_square > 10.83 { "< 0.001" } 
                  else if chi_square > 6.64 { "< 0.01" }
                  else if chi_square > 3.84 { "< 0.05" }
                  else { "> 0.05" };
    println!("\n📊 Resonance Analysis:");
    println!("====================");
    // Show top killers for best patterns
    println!("\nBest Symmetric k={:?} - Top Prime Killers:", best_symmetric.k_values);
    if let Some(res) = resonance_data.get(&format!("k_{}_{}", best_symmetric.k_values.0, best_symmetric.k_values.1)) {
        for (prime, rate) in res.prime_kill_rates.iter().take(5) {
            println!("  Prime {}: kills {:.1}% of candidates", prime, rate * 100.0);
    println!("\nBest Breathing k={:?} - Top Prime Killers:", best_asymmetric.k_values);
    if let Some(res) = resonance_data.get(&format!("k_{}_{}", best_asymmetric.k_values.0, best_asymmetric.k_values.1)) {
    // Build conclusions
    let mut conclusions = vec![
        format!("Breathing patterns show {:.1}% improvement over symmetric patterns", improvement),
        format!("Best symmetric: k={:?} with {:.1}% prime density", best_symmetric.k_values, best_symmetric.prime_density * 100.0),
        format!("Best breathing: k={:?} with {:.1}% prime density", best_asymmetric.k_values, best_asymmetric.prime_density * 100.0),
        format!("Statistical significance: χ² = {:.2}, p {}", chi_square, p_value),
    if improvement > 30.0 {
        conclusions.push("The breathing advantage is substantial and highly significant".to_string());
    // Build final results
    let results = BreathingAnalysisResults {
        timestamp: Local::now().to_rfc3339(),
        base,
        boundary_digits: (outer, inner),
        patterns_tested: pattern_results,
        resonance_analysis: resonance_data,
        statistical_summary: StatisticalSummary {
            best_symmetric,
            best_asymmetric,
            improvement_percentage: improvement,
            chi_square_statistic: chi_square,
            p_value_estimate: p_value.to_string(),
        },
        conclusions,
    };
    // Save to file
    let filename = format!("breathing_analysis_{}.json", Local::now().format("%Y%m%d_%H%M%S"));
    let mut file = File::create(&filename).expect("Failed to create output file");
    let json = serde_json::to_string_pretty(&results).expect("Failed to serialize results");
    file.write_all(json.as_bytes()).expect("Failed to write results");
    println!("\n📊 Summary:");
    println!("==========");
    for conclusion in &results.conclusions {
        println!("• {}", conclusion);
    println!("\n✅ Analysis complete! Results saved to: {}", filename);
    // Show sample primes from best breathing pattern
    println!("\n🔍 Sample primes from best breathing pattern:");
    for example in results.statistical_summary.best_asymmetric.prime_examples.iter().take(3) {
        println!("  {}", example);
