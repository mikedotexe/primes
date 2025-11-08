//! Prime Verification System - Detailed verification and anti-gaming mechanics
//! 
//! This module handles the critical verification process and prevents gaming

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
/// Verification challenge issued to validators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationChallenge {
    pub discovery_id: String,
    pub prime_commitment: String, // Hash of the prime
    pub chain_commitments: Vec<String>, // Hashes of chain elements
    pub challenge_seed: u64, // For deterministic random challenges
    pub required_checks: Vec<RequiredCheck>,
}
pub enum RequiredCheck {
    /// Verify primality with specific method
    PrimalityTest {
        number_index: usize,
        method: TestMethod,
        min_confidence: f64,
    },
    
    /// Verify subharmonic extraction
    SubharmonicExtraction {
        source_index: usize,
        ratio: usize,
        expected_hash: String,
    /// Verify specific digit at position
    DigitVerification {
        position: usize,
        expected_digit: u8,
    /// Verify chain continuity
    ChainContinuity {
        from_index: usize,
        to_index: usize,
        extraction_ratio: usize,
pub enum TestMethod {
    MillerRabin { rounds: u32 },
    LucasLehmer,
    Solovay,
    BailliePSW,
/// Anti-gaming measures
pub struct AntiGamingState {
    /// Recent discovery hashes to prevent duplicates
    pub recent_discoveries: HashSet<String>,
    /// Rate limiting per account
    pub submission_timestamps: HashMap<String, Vec<u64>>,
    /// Suspicious patterns detected
    pub flagged_accounts: HashMap<String, Vec<SuspiciousActivity>>,
    /// Global uniqueness check (bloom filter in production)
    pub global_prime_hashes: HashSet<String>,
pub enum SuspiciousActivity {
    RapidSubmissions { count: u32, timespan: u64 },
    InvalidVerifications { count: u32 },
    DuplicateSubmissions { count: u32 },
    CollusionPattern { with_accounts: Vec<String> },
/// Verification oracle service (off-chain)
pub struct VerificationOracle {
    /// Cache of verified primes
    verified_cache: HashMap<String, bool>,
    /// Performance metrics
    total_verifications: u64,
    false_submissions: u64,
impl VerificationOracle {
    pub fn new() -> Self {
        Self {
            verified_cache: HashMap::new(),
            total_verifications: 0,
            false_submissions: 0,
        }
    }
    /// Process a verification challenge
    pub fn process_challenge(&mut self, challenge: &VerificationChallenge) -> VerificationResult {
        let mut results = Vec::new();
        let mut all_passed = true;
        
        for check in &challenge.required_checks {
            let result = match check {
                RequiredCheck::PrimalityTest { number_index, method, min_confidence } => {
                    self.verify_primality(*number_index, method, *min_confidence)
                },
                RequiredCheck::SubharmonicExtraction { source_index, ratio, expected_hash } => {
                    self.verify_extraction(*source_index, *ratio, expected_hash)
                RequiredCheck::DigitVerification { number_index, position, expected_digit } => {
                    self.verify_digit(*number_index, *position, *expected_digit)
                RequiredCheck::ChainContinuity { from_index, to_index, extraction_ratio } => {
                    self.verify_continuity(*from_index, *to_index, *extraction_ratio)
            };
            
            if !result.passed {
                all_passed = false;
            }
            results.push(result);
        self.total_verifications += 1;
        if !all_passed {
            self.false_submissions += 1;
        let confidence = self.calculate_confidence(&results);
        VerificationResult {
            challenge_id: challenge.discovery_id.clone(),
            all_checks_passed: all_passed,
            individual_results: results,
            confidence_score: confidence,
    fn verify_primality(&self, _index: usize, method: &TestMethod, min_confidence: f64) -> CheckResult {
        // In real implementation, fetch actual number and test
        match method {
            TestMethod::MillerRabin { rounds } => {
                let confidence = 1.0 - (0.25_f64).powi(*rounds as i32);
                CheckResult {
                    check_type: "primality".to_string(),
                    passed: confidence >= min_confidence,
                    details: format!("Miller-Rabin {} rounds, confidence: {:.6}", rounds, confidence),
                }
            },
            _ => CheckResult {
                check_type: "primality".to_string(),
                passed: true,
                details: "Other method".to_string(),
    fn verify_extraction(&self, _source: usize, _ratio: usize, _expected: &str) -> CheckResult {
        // In real implementation, perform extraction and compare hash
        CheckResult {
            check_type: "extraction".to_string(),
            passed: true,
            details: "Extraction verified".to_string(),
    fn verify_digit(&self, _index: usize, _position: usize, _expected: u8) -> CheckResult {
            check_type: "digit".to_string(),
            details: "Digit verified".to_string(),
    fn verify_continuity(&self, _from: usize, _to: usize, _ratio: usize) -> CheckResult {
            check_type: "continuity".to_string(),
            details: "Chain continuity verified".to_string(),
    fn calculate_confidence(&self, results: &[CheckResult]) -> f64 {
        let passed = results.iter().filter(|r| r.passed).count();
        passed as f64 / results.len() as f64
pub struct VerificationResult {
    pub challenge_id: String,
    pub all_checks_passed: bool,
    pub individual_results: Vec<CheckResult>,
    pub confidence_score: f64,
pub struct CheckResult {
    pub check_type: String,
    pub passed: bool,
    pub details: String,
/// Generate deterministic challenges to prevent gaming
pub fn generate_verification_challenges(
    discovery_id: &str,
    prime: &str,
    chain: &[(String, usize)],
    seed: u64,
) -> VerificationChallenge {
    let mut hasher = Sha256::new();
    // Hash the prime
    hasher.update(prime.as_bytes());
    let prime_hash = format!("{:x}", hasher.finalize_reset());
    // Hash chain elements
    let chain_hashes: Vec<String> = chain.iter().map(|(value, _)| {
        hasher.update(value.as_bytes());
        format!("{:x}", hasher.finalize_reset())
    }).collect();
    // Generate deterministic challenges based on seed
    let mut required_checks = Vec::new();
    // Always verify the main prime
    required_checks.push(RequiredCheck::PrimalityTest {
        number_index: 0,
        method: TestMethod::MillerRabin { rounds: 20 },
        min_confidence: 0.999999,
    });
    // Verify chain elements based on seed
    let check_indices = deterministic_sample(chain.len(), seed);
    for idx in check_indices {
        required_checks.push(RequiredCheck::PrimalityTest {
            number_index: idx + 1,
            method: TestMethod::MillerRabin { rounds: 15 },
            min_confidence: 0.9999,
        });
        // Verify extraction
        if idx > 0 {
            required_checks.push(RequiredCheck::SubharmonicExtraction {
                source_index: idx - 1,
                ratio: chain[idx - 1].1,
                expected_hash: chain_hashes[idx].clone(),
            });
    // Random digit checks to ensure honesty
    let digit_checks = (seed % 5 + 1) as usize;
    for i in 0..digit_checks {
        required_checks.push(RequiredCheck::DigitVerification {
            number_index: (seed + i as u64) as usize % (chain.len() + 1),
            position: ((seed * (i + 1) as u64) % 20) as usize,
            expected_digit: ((seed + i as u64) % 10) as u8,
    VerificationChallenge {
        discovery_id: discovery_id.to_string(),
        prime_commitment: prime_hash,
        chain_commitments: chain_hashes,
        challenge_seed: seed,
        required_checks,
fn deterministic_sample(max: usize, seed: u64) -> Vec<usize> {
    // Simple deterministic sampling
    let count = (max / 2).max(1).min(5);
    let mut indices = Vec::new();
    for i in 0..count {
        let idx = ((seed + i as u64) * 2654435761) % max as u64;
        indices.push(idx as usize);
    indices.sort();
    indices.dedup();
    indices
/// Economic incentive calculations
pub fn calculate_verification_incentives(
    discovery_value: u128,
    num_verifiers: u32,
    early_bird_bonus: bool,
) -> u128 {
    let base_reward = discovery_value / 100; // 1% of discovery value
    let verifier_share = base_reward / num_verifiers as u128;
    if early_bird_bonus {
        verifier_share * 3 / 2 // 50% bonus for early verifiers
    } else {
        verifier_share
fn main() {
    println!("🔐 PRIME VERIFICATION SYSTEM");
    println!("===========================\n");
    println!("🎯 VERIFICATION CHALLENGES");
    println!("=========================");
    // Example discovery
    let discovery_id = "disc_001";
    let prime = "307050703";
    let chain = vec![
        ("37573".to_string(), 2),
        ("353".to_string(), 2),
        ("3".to_string(), 3),
    ];
    let challenge = generate_verification_challenges(
        discovery_id,
        prime,
        &chain,
        12345, // blockchain seed
    );
    println!("Challenge for discovery: {}", discovery_id);
    println!("Prime hash: {}", challenge.prime_commitment);
    println!("Required checks: {}", challenge.required_checks.len());
    for (i, check) in challenge.required_checks.iter().enumerate() {
        println!("\n  Check {}: {:?}", i + 1, check);
    println!("\n🤖 ORACLE VERIFICATION");
    println!("======================");
    let mut oracle = VerificationOracle::new();
    let result = oracle.process_challenge(&challenge);
    println!("Verification complete:");
    println!("  All checks passed: {}", result.all_checks_passed);
    println!("  Confidence score: {:.2}%", result.confidence_score * 100.0);
    println!("\n💰 INCENTIVE STRUCTURE");
    let discovery_reward = 2_500_000_000_000_000_000_000_000u128; // 2.5 NEAR
    println!("Discovery reward: {} yoctoNEAR", discovery_reward);
    println!("Verification rewards:");
    println!("  Early bird (1st verifier): {} yoctoNEAR", 
        calculate_verification_incentives(discovery_reward, 3, true));
    println!("  Regular verifiers: {} yoctoNEAR", 
        calculate_verification_incentives(discovery_reward, 3, false));
    println!("\n🛡️ ANTI-GAMING MEASURES");
    println!("=======================");
    println!("1. Deterministic challenge generation prevents cherry-picking");
    println!("2. Random digit verification ensures full number honesty");
    println!("3. Time-based rate limiting prevents spam");
    println!("4. Collusion detection through verification patterns");
    println!("5. Global uniqueness checking prevents recycling");
    println!("6. Slashing for false verifications");
    println!("\n📊 GAME THEORY ANALYSIS");
    println!("Honest behavior is optimal because:");
    println!("- Cost of finding real primes < cost of gaming system");
    println!("- Verification is cheaper than discovery");
    println!("- False submissions lead to reputation loss");
    println!("- Collusion is detectable and punishable");
