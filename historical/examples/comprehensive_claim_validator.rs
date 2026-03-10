use std::io;//! Comprehensive Claim Validator - Verifies all major claims with detailed output
//! 
//! This validates:
//! 1. 30% prime density for specific configurations
//! 2. Exclusive configurations that work with only one seed
//! 3. Breathing pattern advantages
//! 4. Cross-base performance
//! 5. Outputs complete verification data with timestamps

use primes::{
    is_prime_miller_rabin,
};
use std::fs::File;
#[derive(Debug, Serialize, Deserialize)]
struct ValidationReport {
    timestamp: String,
    runtime_seconds: f64,
    claims_validated: Vec<ClaimValidation>,
    overall_result: ValidationSummary,
}
struct ClaimValidation {
    claim_id: String,
    claim_description: String,
    test_performed: String,
    expected_result: String,
    actual_result: String,
    passed: bool,
    evidence: serde_json::Value,
struct ValidationSummary {
    total_claims: usize,
    passed_claims: usize,
    failed_claims: usize,
    success_rate: f64,
    verification_urls: Vec<String>,
fn validate_30_percent_density() -> ClaimValidation {
    let claim_id = "CLAIM_001".to_string();
    let claim_description = "Base-6 configuration (3,3) k=(0,1) achieves 30% prime density".to_string();
    
    println!("\n🔬 Validating Claim 001: 30% Prime Density");
    println!("Testing configuration (3,3) k=(0,1) base 6...");
    let config = MembraneConfig {
        outer: 3,
        inner: 3,
        k_outer: 0,
        k_inner: 1,
    };
    let num_seeds = 10000;
    let mut primes_found = 0;
    let mut prime_examples = Vec::new();
    let start = Instant::now();
    for seed in 1..=num_seeds {
        let candidate = generate_prime_candidate(&config, &seed.to_string(), 6);
        if is_prime_miller_rabin(&candidate, 20) {
            primes_found += 1;
            if prime_examples.len() < 5 {
                prime_examples.push(json!({
                    "seed": seed,
                    "prime": candidate.to_string(),
                    "verification_url": format!("https://www.wolframalpha.com/input/?i=isprime({})", candidate)
                }));
            }
        }
        
        if seed % 1000 == 0 {
            print!(".");
            std::io::stdout().flush().unwrap();
    }
    let duration = start.elapsed();
    let actual_density = primes_found as f64 / num_seeds as f64;
    let passed = actual_density >= 0.28 && actual_density <= 0.32; // Allow ±2% variance
    println!("\nFound {} primes in {} seeds ({:.1}%) in {:.2}s", 
        primes_found, num_seeds, actual_density * 100.0, duration.as_secs_f64());
    ClaimValidation {
        claim_id,
        claim_description,
        test_performed: format!("Generated {} membrane candidates with base 6, (3,3) k=(0,1)", num_seeds),
        expected_result: "30% ± 2% prime density".to_string(),
        actual_result: format!("{:.2}% prime density", actual_density * 100.0),
        passed,
        evidence: json!({
            "seeds_tested": num_seeds,
            "primes_found": primes_found,
            "density": actual_density,
            "confidence_interval": {
                "lower": actual_density - 0.0046,  // 95% CI approximation
                "upper": actual_density + 0.0046
            },
            "prime_examples": prime_examples,
            "test_duration_seconds": duration.as_secs_f64()
        }),
fn validate_exclusive_configuration() -> ClaimValidation {
    let claim_id = "CLAIM_002".to_string();
    let claim_description = "Configuration (3,7) k=(1,1) base 10 works exclusively with seed 5".to_string();
    println!("\n🔬 Validating Claim 002: Exclusive Configuration");
    println!("Testing all seeds 0-9 for configuration (3,7) k=(1,1) base 10...");
        inner: 7,
        k_outer: 1,
    let mut results = Vec::new();
    let mut prime_count = 0;
    for seed in 0..=9 {
        let candidate = generate_prime_candidate(&config, &seed.to_string(), 10);
        let is_prime = is_prime_miller_rabin(&candidate, 20);
        if is_prime {
            prime_count += 1;
        results.push(json!({
            "seed": seed,
            "value": candidate.to_string(),
            "is_prime": is_prime,
            "verification_url": format!("https://www.wolframalpha.com/input/?i=isprime({})", candidate)
        }));
        println!("  Seed {}: {} {}", seed, candidate, if is_prime { "✓ PRIME" } else { "✗ composite" });
    let passed = prime_count == 1 && results[5]["is_prime"] == true;
        test_performed: "Tested all seeds 0-9 for primality".to_string(),
        expected_result: "Exactly one prime (seed 5)".to_string(),
        actual_result: format!("{} prime(s) found", prime_count),
            "all_results": results,
            "prime_count": prime_count,
            "exclusive_seed": if passed { Some(5) } else { None }
fn validate_breathing_advantage() -> ClaimValidation {
    let claim_id = "CLAIM_003".to_string();
    let claim_description = "Asymmetric (breathing) patterns outperform symmetric patterns by >40%".to_string();
    println!("\n🔬 Validating Claim 003: Breathing Pattern Advantage");
    let base = 6;
    let num_seeds = 5000;
    // Test symmetric k=(1,1)
    print!("Testing symmetric k=(1,1)...");
    std::io::stdout().flush().unwrap();
    let symmetric_config = MembraneConfig {
    let mut symmetric_primes = 0;
        let candidate = generate_prime_candidate(&symmetric_config, &seed.to_string(), base);
            symmetric_primes += 1;
    let symmetric_density = symmetric_primes as f64 / num_seeds as f64;
    println!(" {:.1}%", symmetric_density * 100.0);
    // Test breathing k=(0,1)
    print!("Testing breathing k=(0,1)...");
    let breathing_config = MembraneConfig {
    let mut breathing_primes = 0;
        let candidate = generate_prime_candidate(&breathing_config, &seed.to_string(), base);
            breathing_primes += 1;
    let breathing_density = breathing_primes as f64 / num_seeds as f64;
    println!(" {:.1}%", breathing_density * 100.0);
    let improvement = (breathing_density - symmetric_density) / symmetric_density * 100.0;
    let passed = improvement >= 40.0;
        test_performed: format!("Compared {} seeds for both patterns", num_seeds),
        expected_result: ">40% improvement".to_string(),
        actual_result: format!("{:.1}% improvement", improvement),
            "symmetric_results": {
                "configuration": "k=(1,1)",
                "primes_found": symmetric_primes,
                "density": symmetric_density
            "breathing_results": {
                "configuration": "k=(0,1)",
                "primes_found": breathing_primes,
                "density": breathing_density
            "improvement_percentage": improvement,
            "seeds_tested": num_seeds
fn validate_cross_base_performance() -> ClaimValidation {
    let claim_id = "CLAIM_004".to_string();
    let claim_description = "Membrane patterns work effectively across multiple bases".to_string();
    println!("\n🔬 Validating Claim 004: Cross-Base Performance");
    let bases = vec![2, 6, 10, 12];
    let num_seeds = 1000;
    let mut base_results = Vec::new();
    for base in bases {
        print!("Testing base {}...", base);
        std::io::stdout().flush().unwrap();
        // Find a good configuration for this base
        let config = match base {
            2 => MembraneConfig { outer: 1, inner: 1, k_outer: 0, k_inner: 1 },
            6 => MembraneConfig { outer: 3, inner: 3, k_outer: 0, k_inner: 1 },
            10 => MembraneConfig { outer: 3, inner: 7, k_outer: 1, k_inner: 2 },
            12 => MembraneConfig { outer: 5, inner: 7, k_outer: 0, k_inner: 1 },
            _ => MembraneConfig { outer: 1, inner: 1, k_outer: 0, k_inner: 0 }
        };
        let mut primes = 0;
        for seed in 1..=num_seeds {
            let candidate = generate_prime_candidate(&config, &seed.to_string(), base);
            if is_prime_miller_rabin(&candidate, 20) {
                primes += 1;
        let density = primes as f64 / num_seeds as f64;
        println!(" {:.1}%", density * 100.0);
        base_results.push(json!({
            "base": base,
            "configuration": format!("({},{}) k=({},{})", config.outer, config.inner, config.k_outer, config.k_inner),
            "primes_found": primes,
            "density": density
    let min_density = base_results.iter()
        .map(|r| r["density"].as_f64().unwrap())
        .fold(f64::INFINITY, f64::min);
    let passed = min_density >= 0.15; // All bases should achieve at least 15%
        test_performed: "Tested optimal configurations across bases 2, 6, 10, 12".to_string(),
        expected_result: "All bases achieve >15% prime density".to_string(),
        actual_result: format!("Minimum density: {:.1}%", min_density * 100.0),
            "base_results": base_results,
            "seeds_per_base": num_seeds
fn main() {
    println!("🔬 Comprehensive Claim Validator");
    println!("================================");
    println!("Validating all major claims about membrane prime generation...\n");
    let start_time = Instant::now();
    let mut claims = Vec::new();
    // Run all validations
    claims.push(validate_30_percent_density());
    claims.push(validate_exclusive_configuration());
    claims.push(validate_breathing_advantage());
    claims.push(validate_cross_base_performance());
    let total_runtime = start_time.elapsed().as_secs_f64();
    // Calculate summary
    let total_claims = claims.len();
    let passed_claims = claims.iter().filter(|c| c.passed).count();
    let failed_claims = total_claims - passed_claims;
    let success_rate = passed_claims as f64 / total_claims as f64;
    // Collect verification URLs
    let mut verification_urls = Vec::new();
    for claim in &claims {
        if let Some(examples) = claim.evidence.get("prime_examples") {
            if let Some(array) = examples.as_array() {
                for example in array {
                    if let Some(url) = example.get("verification_url") {
                        if let Some(url_str) = url.as_str() {
                            verification_urls.push(url_str.to_string());
                        }
                    }
                }
    let report = ValidationReport {
        timestamp: Local::now().to_rfc3339(),
        runtime_seconds: total_runtime,
        claims_validated: claims,
        overall_result: ValidationSummary {
            total_claims,
            passed_claims,
            failed_claims,
            success_rate,
            verification_urls,
        },
    // Save report
    let filename = format!("validation_report_{}.json", Local::now().format("%Y%m%d_%H%M%S"));
    let mut file = File::create(&filename).expect("Failed to create output file");
    let json = serde_json::to_string_pretty(&report).expect("Failed to serialize report");
    file.write_all(json.as_bytes()).expect("Failed to write report");
    // Print summary
    println!("\n" + "=".repeat(60));
    println!("📊 VALIDATION SUMMARY");
    println!("=".repeat(60));
    println!("Total claims tested: {}", total_claims);
    println!("Passed: {} ✅", passed_claims);
    println!("Failed: {} ❌", failed_claims);
    println!("Success rate: {:.0}%", success_rate * 100.0);
    println!("Total runtime: {:.2}s", total_runtime);
    println!("\n📋 Individual Results:");
    for claim in &report.claims_validated {
        let status = if claim.passed { "✅ PASS" } else { "❌ FAIL" };
        println!("\n{}: {}", claim.claim_id, status);
        println!("  Description: {}", claim.claim_description);
        println!("  Expected: {}", claim.expected_result);
        println!("  Actual: {}", claim.actual_result);
    println!("\n✅ Validation complete! Full report saved to: {}", filename);
    if !verification_urls.is_empty() {
        println!("\n🔍 Sample verification URLs:");
        for url in verification_urls.iter().take(3) {
            println!("  {}", url);
// Helper macro for JSON creation
#[macro_use]
extern crate serde_json;
