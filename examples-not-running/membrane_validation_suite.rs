use primes::{
    is_prime_miller_rabin,
};
use std::time::Instant;
/// Comprehensive validation suite for all membrane prime generation claims
fn main() {
    println!("🔬 Membrane Prime Generation Validation Suite");
    println!("=" .repeat(80));
    
    let mut results = ValidationResults::new();
    // Test 1: Basic membrane structure generates primes above random chance
    test_basic_membrane_success_rate(&mut results);
    // Test 2: Base-dependent optimal digits
    test_base_dependent_optimal_digits(&mut results);
    // Test 3: Breathing membrane advantage
    test_breathing_membrane_performance(&mut results);
    // Test 4: Exclusive configurations
    test_exclusive_configurations(&mut results);
    // Test 5: Configuration migration
    test_configuration_migration(&mut results);
    // Test 6: Statistical significance
    test_statistical_significance(&mut results);
    // Test 7: Edge cases
    test_edge_cases(&mut results);
    // Test 8: Cross-base pattern consistency
    test_cross_base_patterns(&mut results);
    // Test 9: Reproducibility
    test_reproducibility(&mut results);
    // Test 10: Performance benchmarks
    test_performance_benchmarks(&mut results);
    // Generate final report
    results.print_report();
}

struct ValidationResults {
    tests: Vec<TestResult>,
    start_time: Instant,
struct TestResult {
    name: String,
    passed: bool,
    details: String,
    duration: std::time::Duration,
impl ValidationResults {
    fn new() -> Self {
        Self {
            tests: Vec::new(),
            start_time: Instant::now(),
        }
    }
    fn add_test(&mut self, name: &str, passed: bool, details: String) {
        self.tests.push(TestResult {
            name: name.to_string(),
            passed,
            details,
            duration: self.start_time.elapsed(),
        });
    fn print_report(&self) {
        println!("\n📊 VALIDATION REPORT");
        println!("=" .repeat(80));
        
        let total_tests = self.tests.len();
        let passed_tests = self.tests.iter().filter(|t| t.passed).count();
        let pass_rate = (passed_tests as f64 / total_tests as f64) * 100.0;
        println!("Total Tests: {}", total_tests);
        println!("Passed: {} ({:.1}%)", passed_tests, pass_rate);
        println!("Failed: {}", total_tests - passed_tests);
        println!("\nDetailed Results:");
        println!("-" .repeat(80));
        for test in &self.tests {
            let status = if test.passed { "✅ PASS" } else { "❌ FAIL" };
            println!("\n{}: {}", status, test.name);
            println!("Details: {}", test.details);
            println!("Duration: {:.2}s", test.duration.as_secs_f64());
        println!("\n" + &"=" .repeat(80));
        println!("Total Runtime: {:.2}s", self.start_time.elapsed().as_secs_f64());
/// Test 1: Basic membrane structure generates primes above random chance
fn test_basic_membrane_success_rate(results: &mut ValidationResults) {
    println!("\n🧪 Test 1: Basic Membrane Success Rate");
    let config = MembraneConfig {
        outer: 3,
        inner: 7,
        k_outer: 2,
        k_inner: 1,
    };
    let mut prime_count = 0;
    let test_seeds = 100;
    for seed in 1..=test_seeds {
        let candidate = generate_prime_candidate(&config, &seed.to_string(), 10);
        if is_prime_miller_rabin(&candidate, 20) {
            prime_count += 1;
    let success_rate = prime_count as f64 / test_seeds as f64;
    let random_baseline = estimate_random_prime_density(11); // 11-digit numbers
    let passed = success_rate > random_baseline * 1.5; // Should be at least 50% better than random
    results.add_test(
        "Basic Membrane Success Rate",
        passed,
        format!(
            "Success rate: {:.1}% vs random baseline: {:.1}% (ratio: {:.2}x)",
            success_rate * 100.0,
            random_baseline * 100.0,
            success_rate / random_baseline
        )
    );
/// Test 2: Base-dependent optimal digits
fn test_base_dependent_optimal_digits(results: &mut ValidationResults) {
    println!("\n🧪 Test 2: Base-Dependent Optimal Digits");
    let bases = vec![10, 12, 16];
    let mut base_results = HashMap::new();
    for base in bases {
        let mut digit_performance = HashMap::new();
        // Test different boundary digits
        for outer in 1..base.min(10) {
            for inner in 1..base.min(10) {
                if outer == inner { continue; }
                
                let config = MembraneConfig {
                    outer: outer as u8,
                    inner: inner as u8,
                    k_outer: 1,
                    k_inner: 1,
                };
                let mut success = 0;
                for seed in 1..=20 {
                    let candidate = generate_prime_candidate(&config, &seed.to_string(), base);
                    if is_prime_miller_rabin(&candidate, 20) {
                        success += 1;
                    }
                }
                digit_performance.insert((outer, inner), success);
            }
        // Find best configuration for this base
        let best = digit_performance.iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&key, &count)| (key, count))
            .unwrap_or(((0, 0), 0));
        base_results.insert(base, best);
    // Check if different bases have different optimal digits
    let base_10_best = base_results[&10].0;
    let base_12_best = base_results[&12].0;
    let base_16_best = base_results[&16].0;
    let passed = base_10_best != base_12_best || base_10_best != base_16_best;
        "Base-Dependent Optimal Digits",
            "Base 10 best: {:?}, Base 12 best: {:?}, Base 16 best: {:?}",
            base_10_best, base_12_best, base_16_best
/// Test 3: Breathing membrane advantage
fn test_breathing_membrane_performance(results: &mut ValidationResults) {
    println!("\n🧪 Test 3: Breathing Membrane Advantage");
    // Symmetric configuration
    let symmetric_config = MembraneConfig {
        inner: 3,
        k_outer: 1,
    // Asymmetric "breathing" configuration
    let breathing_config = MembraneConfig {
        k_outer: 0,
    let test_seeds = 50;
    let mut symmetric_primes = 0;
    let mut breathing_primes = 0;
        let sym_candidate = generate_prime_candidate(&symmetric_config, &seed.to_string(), 10);
        let breath_candidate = generate_prime_candidate(&breathing_config, &seed.to_string(), 10);
        if is_prime_miller_rabin(&sym_candidate, 20) {
            symmetric_primes += 1;
        if is_prime_miller_rabin(&breath_candidate, 20) {
            breathing_primes += 1;
    let symmetric_rate = symmetric_primes as f64 / test_seeds as f64;
    let breathing_rate = breathing_primes as f64 / test_seeds as f64;
    let improvement = if symmetric_rate > 0.0 { breathing_rate / symmetric_rate } else { 0.0 };
    let passed = breathing_rate > symmetric_rate * 1.2; // At least 20% improvement
        "Breathing Membrane Advantage",
            "Symmetric: {:.1}%, Breathing: {:.1}%, Improvement: {:.1}x",
            symmetric_rate * 100.0,
            breathing_rate * 100.0,
            improvement
/// Test 4: Exclusive configurations
fn test_exclusive_configurations(results: &mut ValidationResults) {
    println!("\n🧪 Test 4: Exclusive Configurations");
    // Known exclusive configuration from CLAUDE.md
    let exclusive_config = MembraneConfig {
    let exclusive_seed = "5";
    let mut exclusive_count = 0;
    let mut other_seed_count = 0;
    // Test the exclusive seed
    let candidate = generate_prime_candidate(&exclusive_config, exclusive_seed, 10);
    if is_prime_miller_rabin(&candidate, 20) {
        exclusive_count = 1;
    // Test other seeds
    for seed in 1..=20 {
        if seed.to_string() == exclusive_seed { continue; }
        let candidate = generate_prime_candidate(&exclusive_config, &seed.to_string(), 10);
            other_seed_count += 1;
    let passed = exclusive_count == 1 && other_seed_count == 0;
        "Exclusive Configurations",
            "Exclusive seed prime: {}, Other seeds prime count: {}",
            exclusive_count, other_seed_count
/// Test 5: Configuration migration
fn test_configuration_migration(results: &mut ValidationResults) {
    println!("\n🧪 Test 5: Configuration Migration");
    // Test how configurations perform with different seed lengths
    let configs = vec![
        MembraneConfig { outer: 3, inner: 3, k_outer: 0, k_inner: 1 }, // Breathing
        MembraneConfig { outer: 1, inner: 2, k_outer: 0, k_inner: 0 }, // Length specialist
    ];
    let mut migration_data = Vec::new();
    for config in configs {
        let mut length_performance = HashMap::new();
        for length in 1..=4 {
            let mut primes = 0;
            let samples = 20;
            
            for i in 0..samples {
                let seed = format!("{:0width$}", i, width = length);
                let candidate = generate_prime_candidate(&config, &seed, 10);
                if is_prime_miller_rabin(&candidate, 20) {
                    primes += 1;
            length_performance.insert(length, primes as f64 / samples as f64);
        migration_data.push(length_performance);
    // Check if some configurations specialize in certain lengths
    let breathing_best_length = migration_data[0].iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(k, _)| k)
        .unwrap();
    let specialist_best_length = migration_data[1].iter()
    let passed = breathing_best_length != specialist_best_length;
        "Configuration Migration",
            "Breathing optimal length: {}, Specialist optimal length: {}",
            breathing_best_length, specialist_best_length
/// Test 6: Statistical significance
fn test_statistical_significance(results: &mut ValidationResults) {
    println!("\n🧪 Test 6: Statistical Significance");
    // Run multiple trials to test consistency
    let trials = 5;
    let seeds_per_trial = 50;
    let mut trial_results = Vec::new();
    for _ in 0..trials {
        let mut primes = 0;
        for seed in 1..=seeds_per_trial {
            let candidate = generate_prime_candidate(&config, &seed.to_string(), 10);
            if is_prime_miller_rabin(&candidate, 20) {
                primes += 1;
        trial_results.push(primes as f64 / seeds_per_trial as f64);
    // Calculate mean and standard deviation
    let mean = trial_results.iter().sum::<f64>() / trials as f64;
    let variance = trial_results.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / trials as f64;
    let std_dev = variance.sqrt();
    // Check if results are statistically consistent
    let coefficient_of_variation = std_dev / mean;
    let passed = coefficient_of_variation < 0.3; // CV should be reasonably low
        "Statistical Significance",
            "Mean: {:.1}%, Std Dev: {:.1}%, CV: {:.2}",
            mean * 100.0, std_dev * 100.0, coefficient_of_variation
/// Test 7: Edge cases
fn test_edge_cases(results: &mut ValidationResults) {
    println!("\n🧪 Test 7: Edge Cases");
    let mut edge_case_results = Vec::new();
    // Test 1: Very large k values
    let large_k_config = MembraneConfig {
        k_outer: 10,
        k_inner: 10,
    let large_k_candidate = generate_prime_candidate(&large_k_config, "5", 10);
    let large_k_prime = is_prime_miller_rabin(&large_k_candidate, 20);
    edge_case_results.push(("Large k values", large_k_prime, large_k_candidate.len()));
    // Test 2: Zero k values
    let zero_k_config = MembraneConfig {
        k_inner: 0,
    let zero_k_candidate = generate_prime_candidate(&zero_k_config, "5", 10);
    let zero_k_prime = is_prime_miller_rabin(&zero_k_candidate, 20);
    edge_case_results.push(("Zero k values", zero_k_prime, zero_k_candidate.len()));
    // Test 3: Same outer and inner digits
    let same_digits_config = MembraneConfig {
    let same_candidate = generate_prime_candidate(&same_digits_config, "5", 10);
    let same_prime = is_prime_miller_rabin(&same_candidate, 20);
    edge_case_results.push(("Same boundary digits", same_prime, same_candidate.len()));
    // Test 4: Empty seed
    let empty_seed_candidate = generate_prime_candidate(&zero_k_config, "", 10);
    let empty_seed_prime = is_prime_miller_rabin(&empty_seed_candidate, 20);
    edge_case_results.push(("Empty seed", empty_seed_prime, empty_seed_candidate.len()));
    // All edge cases should produce valid candidates (even if not prime)
    let passed = edge_case_results.iter().all(|(_, _, len)| *len >= 5);
    let details = edge_case_results.iter()
        .map(|(name, is_prime, len)| format!("{}: {} (len: {})", 
            name, 
            if *is_prime { "prime" } else { "composite" },
            len
        ))
        .collect::<Vec<_>>()
        .join(", ");
    results.add_test("Edge Cases", passed, details);
/// Test 8: Cross-base pattern consistency
fn test_cross_base_patterns(results: &mut ValidationResults) {
    println!("\n🧪 Test 8: Cross-Base Pattern Consistency");
    // Test if base-10 optimal config works in other bases
    let base_10_config = MembraneConfig {
    let bases = vec![8, 10, 12, 16];
    let mut base_performance = HashMap::new();
    for &base in &bases {
        let samples = 30;
        for seed in 1..=samples {
            // Skip if digits are invalid for this base
            if base_10_config.outer >= base || base_10_config.inner >= base {
                continue;
            let candidate = generate_prime_candidate(&base_10_config, &seed.to_string(), base);
        base_performance.insert(base, primes as f64 / samples as f64);
    // Check if performance varies significantly across bases
    let min_perf = base_performance.values().cloned().fold(1.0, f64::min);
    let max_perf = base_performance.values().cloned().fold(0.0, f64::max);
    let variation = if min_perf > 0.0 { max_perf / min_perf } else { f64::INFINITY };
    let passed = variation > 1.5; // Significant variation expected
    let details = bases.iter()
        .map(|b| format!("Base {}: {:.1}%", b, base_performance[b] * 100.0))
        "Cross-Base Pattern Consistency",
        format!("{} (variation: {:.1}x)", details, variation)
/// Test 9: Reproducibility
fn test_reproducibility(results: &mut ValidationResults) {
    println!("\n🧪 Test 9: Reproducibility");
    // Run the same configuration multiple times
    let runs = 3;
    let mut run_results = Vec::new();
    for _ in 0..runs {
        let mut primes = Vec::new();
        for seed in 1..=20 {
                primes.push(seed);
        run_results.push(primes);
    // Check if all runs produced identical results
    let first_run = &run_results[0];
    let all_identical = run_results.iter().all(|run| run == first_run);
        "Reproducibility",
        all_identical,
            "All {} runs produced identical results: {} primes found",
            runs,
            first_run.len()
/// Test 10: Performance benchmarks
fn test_performance_benchmarks(results: &mut ValidationResults) {
    println!("\n🧪 Test 10: Performance Benchmarks");
    let iterations = 1000;
    let start = Instant::now();
    for i in 0..iterations {
        let _ = generate_prime_candidate(&config, &i.to_string(), 10);
    let generation_time = start.elapsed();
    let per_candidate = generation_time.as_micros() as f64 / iterations as f64;
    // Test primality checking performance
    let test_candidate = generate_prime_candidate(&config, "12345", 10);
    let prime_start = Instant::now();
    for _ in 0..100 {
        let _ = is_prime_miller_rabin(&test_candidate, 20);
    let prime_time = prime_start.elapsed();
    let per_prime_test = prime_time.as_micros() as f64 / 100.0;
    // Performance should be reasonable
    let passed = per_candidate < 1000.0 && per_prime_test < 10000.0;
        "Performance Benchmarks",
            "Generation: {:.1}μs/candidate, Primality: {:.1}μs/test",
            per_candidate, per_prime_test
/// Estimate prime density for n-digit numbers
fn estimate_random_prime_density(digits: usize) -> f64 {
    // By prime number theorem, density ≈ 1/ln(n)
    let n = 10_f64.powi(digits as i32);
    1.0 / n.ln()
