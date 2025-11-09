//! Comprehensive Claim Verifier
//! 
//! This script systematically checks ALL claims found in documentation files
//! and outputs a detailed report showing which are true and which are false.

use primes::{is_prime_miller_rabin, ascii_art::*};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::io::Write;
#[derive(Debug, Clone)]
struct Claim {
    description: String,
    file: String,
    line: usize,
    claim_type: ClaimType,
    verified: Option<bool>,
    actual_value: Option<String>,
}
enum ClaimType {
    PerformancePercentage { base: u32, config: (u32, u32), k: (u32, u32), claimed: f64 },
    SpeedupFactor { claimed: f64 },
    UniversalPattern { pattern: (u32, u32) },
    BreathingAdvantage { claimed_boost: f64 },
    EvenOddAdvantage { claimed: f64 },
fn main() {
    println!("{}", banner("COMPREHENSIVE CLAIM VERIFICATION", 100));
    println!("\nSystematically checking all claims found in documentation...\n");
    
    let mut claims = collect_all_claims();
    let total_claims = claims.len();
    println!("Found {} claims to verify:\n", total_claims);
    // Verify each claim
    for (i, claim) in claims.iter_mut().enumerate() {
        print!("Verifying claim {}/{}: ", i + 1, total_claims);
        std::io::stdout().flush().unwrap();
        
        verify_claim(claim);
        if claim.verified == Some(true) {
            println!("✓ VERIFIED");
        } else {
            println!("✗ FALSE");
        }
    }
    // Generate report
    generate_verification_report(&claims);
fn collect_all_claims() -> Vec<Claim> {
    let mut claims = Vec::new();
    // From THREAD_SUMMARY.md
    claims.push(Claim {
        description: "Base 6 champion: 41% density".to_string(),
        file: "THREAD_SUMMARY.md".to_string(),
        line: 11,
        claim_type: ClaimType::PerformancePercentage { 
            base: 6, 
            config: (1, 3), 
            k: (0, 0), 
            claimed: 41.0 
        },
        verified: None,
        actual_value: None,
    });
        description: "Even bases generate 44% more primes".to_string(),
        line: 6,
        claim_type: ClaimType::EvenOddAdvantage { claimed: 44.0 },
        description: "k=(0,1) outperforms k=(1,1) by up to 42%".to_string(),
        line: 14,
        claim_type: ClaimType::BreathingAdvantage { claimed_boost: 42.0 },
        description: "691x speedup explained".to_string(),
        line: 33,
        claim_type: ClaimType::SpeedupFactor { claimed: 691.0 },
    // From VISUAL_DISCOVERIES.md
        description: "Base 6: (3,3) k=(0,1) achieves 30.2% primes".to_string(),
        file: "VISUAL_DISCOVERIES.md".to_string(),
        line: 83,
            config: (3, 3), 
            k: (0, 1), 
            claimed: 30.2 
        description: "Breathing effect enhances prime generation by 42%".to_string(),
        line: 86,
    // From BASE12_DISCOVERIES.md
        description: "Base 12: (5,7) k=(0,1) achieves 28.9% density".to_string(),
        file: "BASE12_DISCOVERIES.md".to_string(),
        line: 37,
            base: 12, 
            config: (5, 7), 
            claimed: 28.9 
    // From QUICK_REFERENCE_CARD.md (these should be accurate)
        description: "Base 6: (1,5) achieves 31% success".to_string(),
        file: "QUICK_REFERENCE_CARD.md".to_string(),
        line: 13,
            config: (1, 5), 
            claimed: 31.0 
        description: "Base 4: (3,1) achieves 28% success".to_string(),
            base: 4, 
            config: (3, 1), 
            claimed: 28.0 
        description: "vs Random: Always 3-7x better".to_string(),
        line: 61,
        claim_type: ClaimType::SpeedupFactor { claimed: 3.0 }, // Using lower bound
    // From MEGA_ANALYSIS_FINDINGS.md
        description: "Base 6 (1,5) k=(0,0) achieves 33% success rate".to_string(),
        file: "MEGA_ANALYSIS_FINDINGS.md".to_string(),
        line: 15,
            claimed: 33.0 
        description: "Universal pattern (1,5) k=(0,0) works across multiple bases".to_string(),
        line: 25,
        claim_type: ClaimType::UniversalPattern { pattern: (1, 5) },
    claims
fn verify_claim(claim: &mut Claim) {
    match &claim.claim_type {
        ClaimType::PerformancePercentage { base, config, k, claimed } => {
            let actual = test_configuration(*base, *config, *k, 1000);
            claim.actual_value = Some(format!("{:.1}%", actual));
            claim.verified = Some((actual - claimed).abs() < 3.0); // Allow 3% tolerance
        ClaimType::BreathingAdvantage { claimed_boost } => {
            // Test breathing vs symmetric for base 6 (3,3)
            let symmetric = test_configuration(6, (3, 3), (1, 1), 1000);
            let breathing = test_configuration(6, (3, 3), (0, 1), 1000);
            let actual_boost = ((breathing - symmetric) / symmetric) * 100.0;
            
            claim.actual_value = Some(format!("{:.1}% (symmetric: {:.1}%, breathing: {:.1}%)", 
                actual_boost, symmetric, breathing));
            claim.verified = Some(actual_boost > 0.0 && (actual_boost - claimed_boost).abs() < 10.0);
        ClaimType::EvenOddAdvantage { claimed } => {
            // Test several even vs odd bases
            let even_bases = vec![4, 6, 8, 10, 12];
            let odd_bases = vec![3, 5, 7, 9, 11];
            let even_avg = even_bases.iter()
                .map(|&b| find_best_config(b))
                .sum::<f64>() / even_bases.len() as f64;
                
            let odd_avg = odd_bases.iter()
                .sum::<f64>() / odd_bases.len() as f64;
            let advantage = ((even_avg - odd_avg) / odd_avg) * 100.0;
            claim.actual_value = Some(format!("{:.1}% (even: {:.1}%, odd: {:.1}%)", 
                advantage, even_avg, odd_avg));
            claim.verified = Some((advantage - claimed).abs() < 5.0);
        ClaimType::UniversalPattern { pattern } => {
            // Test pattern across multiple bases
            let bases = vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 18, 20];
            let mut working_count = 0;
            let mut results = Vec::new();
            for &base in &bases {
                let success = test_configuration(base, *pattern, (0, 0), 100);
                if success > 5.0 { // Consider >5% as "working"
                    working_count += 1;
                }
                results.push((base, success));
            }
            claim.actual_value = Some(format!("Works in {}/{} bases", working_count, bases.len()));
            claim.verified = Some(working_count >= 10); // Works in at least 10 bases
        ClaimType::SpeedupFactor { claimed: _ } => {
            // For speedup claims, we'll mark as "needs context"
            claim.actual_value = Some("Context-dependent".to_string());
            claim.verified = None; // Can't verify without specific context
fn test_configuration(base: u32, config: (u32, u32), k: (u32, u32), samples: u32) -> f64 {
    let mut successes = 0;
    for seed in 0..samples {
        let membrane = construct_membrane(base, config.0, config.1, k.0, k.1, seed);
        if is_prime_miller_rabin(&membrane) {
            successes += 1;
    (successes as f64 / samples as f64) * 100.0
fn find_best_config(base: u32) -> f64 {
    let mut best = 0.0;
    // Test common configurations
    let configs = vec![(1, 5), (1, 7), (1, 3), (3, 7), (3, 5)];
    for config in configs {
        if gcd(config.0, base) == 1 && gcd(config.1, base) == 1 {
            let rate = test_configuration(base, config, (0, 0), 100);
            if rate > best {
                best = rate;
    best
fn construct_membrane(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, seed: u32) -> BigUint {
    let width = 2 * (1 + k_outer + 1 + k_inner) + 1;
    let base_big = BigUint::from(base);
    let mut value = BigUint::from(0u32);
    // Build membrane polynomial
    value += BigUint::from(outer) * base_big.pow(width - 1);
    value += BigUint::from(inner) * base_big.pow(width - 2 - k_outer);
    value += BigUint::from(seed) * base_big.pow(width / 2);
    value += BigUint::from(inner) * base_big.pow(k_inner + 1);
    value += BigUint::from(outer);
    value
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
fn generate_verification_report(claims: &[Claim]) {
    println!("\n{}", boxed_title("VERIFICATION REPORT", 100));
    let total = claims.len();
    let verified = claims.iter().filter(|c| c.verified == Some(true)).count();
    let false_claims = claims.iter().filter(|c| c.verified == Some(false)).count();
    let unverifiable = claims.iter().filter(|c| c.verified.is_none()).count();
    println!("\nSummary:");
    println!("  Total claims checked: {}", total);
    println!("  ✓ Verified as true:  {} ({:.1}%)", verified, (verified as f64 / total as f64) * 100.0);
    println!("  ✗ Proven false:      {} ({:.1}%)", false_claims, (false_claims as f64 / total as f64) * 100.0);
    println!("  ? Unverifiable:      {} ({:.1}%)", unverifiable, (unverifiable as f64 / total as f64) * 100.0);
    println!("\n{}", boxed_title("FALSE CLAIMS REQUIRING CORRECTION", 100));
    for claim in claims.iter().filter(|c| c.verified == Some(false)) {
        println!("\n❌ File: {}, Line: {}", claim.file, claim.line);
        println!("   Claim: {}", claim.description);
        println!("   Actual: {}", claim.actual_value.as_ref().unwrap_or(&"Unknown".to_string()));
    println!("\n{}", boxed_title("VERIFIED CLAIMS", 100));
    for claim in claims.iter().filter(|c| c.verified == Some(true)) {
        println!("\n✅ File: {}, Line: {}", claim.file, claim.line);
        println!("   Verified: {}", claim.actual_value.as_ref().unwrap_or(&"Correct".to_string()));
    println!("\n{}", boxed_title("FILES NEEDING UPDATES", 100));
    let mut files_to_update: HashMap<String, usize> = HashMap::new();
        *files_to_update.entry(claim.file.clone()).or_insert(0) += 1;
    for (file, count) in files_to_update {
        println!("\n  {} - {} false claims", file, count);
    println!("\n{}", simple_box(
        "RECOMMENDATION:\n\n\
         1. Update THREAD_SUMMARY.md with correct percentages\n\
         2. Fix VISUAL_DISCOVERIES.md breathing pattern claims\n\
         3. Correct BASE12_DISCOVERIES.md performance numbers\n\
         4. Add verification timestamps to all claims\n\
         5. Link claims to verification scripts"
    ));
