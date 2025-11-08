//! Prime Discovery Protocol - Architecture for NEAR-based collaborative prime hunting
//! 
//! A decentralized system where contributors find deep harmonic chains and get rewarded
//! based on the beauty and depth of their discoveries.

use serde::{Deserialize, Serialize};
/// Core discovery submitted by a contributor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeDiscovery {
    /// The prime number discovered
    pub prime: String,
    
    /// The harmonic chain (if any)
    pub chain: Vec<HarmonicStep>,
    /// Membrane configuration used (if applicable)
    pub membrane_config: Option<MembraneConfig>,
    /// Discoverer's NEAR account
    pub discoverer: String,
    /// Block height at discovery
    pub block_height: u64,
    /// Computed beauty score
    pub beauty_score: f64,
}
pub struct HarmonicStep {
    pub value: String,
    pub extraction_ratio: usize,
    pub is_prime: bool,
pub struct MembraneConfig {
    pub outer: u32,
    pub inner: u32,
    pub k_outer: usize,
    pub k_inner: usize,
    pub seed: String,
/// Verification proof submitted by validators
pub struct VerificationProof {
    pub discovery_id: String,
    pub verifier: String,
    pub primality_checks: Vec<PrimalityCheck>,
    pub chain_verification: bool,
    pub timestamp: u64,
pub struct PrimalityCheck {
    pub number: String,
    pub method: String, // "miller-rabin", "lucas-lehmer", etc.
    pub rounds: u32,
/// On-chain state (minimal for gas efficiency)
pub struct ChainState {
    /// Active discoveries awaiting verification
    pub pending_discoveries: HashMap<String, PendingDiscovery>,
    /// Verified discoveries by beauty score (for leaderboard)
    pub verified_discoveries: BTreeMap<String, VerifiedDiscovery>,
    /// Contributor statistics
    pub contributors: HashMap<String, ContributorStats>,
    /// Global statistics
    pub global_stats: GlobalStats,
    /// Reward parameters (updatable by DAO)
    pub reward_params: RewardParameters,
pub struct PendingDiscovery {
    pub discovery: PrimeDiscovery,
    pub submitted_at: u64,
    pub verification_deadline: u64,
    pub verifications: Vec<String>, // Account IDs of verifiers
pub struct VerifiedDiscovery {
    pub id: String,
    pub chain_depth: usize,
    pub verified_at: u64,
    pub reward_paid: u128,
pub struct ContributorStats {
    pub discoveries: u64,
    pub verifications: u64,
    pub total_chain_depth: u64,
    pub highest_beauty_score: f64,
    pub total_rewards: u128,
pub struct GlobalStats {
    pub total_discoveries: u64,
    pub deepest_chain: usize,
    pub largest_prime_digits: usize,
    pub total_rewards_distributed: u128,
pub struct RewardParameters {
    /// Base reward for any verified prime
    pub base_reward: u128,
    /// Multiplier per chain depth level
    pub depth_multiplier: f64,
    /// Bonus for new record depths
    pub record_bonus: u128,
    /// Reward for verification work
    pub verification_reward: u128,
    /// Required verifications before acceptance
    pub required_verifications: u32,
/// External interface for the smart contract
pub trait PrimeDiscoveryProtocol {
    /// Submit a new discovery
    fn submit_discovery(&mut self, discovery: PrimeDiscovery) -> Result<String, String>;
    /// Submit verification for a pending discovery
    fn submit_verification(&mut self, proof: VerificationProof) -> Result<(), String>;
    /// Claim rewards for verified discoveries
    fn claim_rewards(&mut self, account_id: String) -> Result<u128, String>;
    /// Query functions (read-only, no gas)
    fn get_pending_discoveries(&self, limit: u32) -> Vec<PendingDiscovery>;
    fn get_leaderboard(&self, limit: u32) -> Vec<VerifiedDiscovery>;
    fn get_contributor_stats(&self, account_id: String) -> Option<ContributorStats>;
/// Beauty score calculation
fn calculate_beauty_score(discovery: &PrimeDiscovery) -> f64 {
    let mut score = 0.0;
    // Base score for prime size
    let digits = discovery.prime.len();
    score += (digits as f64).ln();
    // Chain depth is highly valued
    let chain_depth = discovery.chain.len();
    score += chain_depth as f64 * 10.0;
    // Bonus for all-prime chains
    let all_prime = discovery.chain.iter().all(|step| step.is_prime);
    if all_prime && chain_depth > 0 {
        score += 20.0;
    }
    // Bonus for elegant patterns (e.g., consistent extraction ratios)
    let ratios: Vec<usize> = discovery.chain.iter().map(|s| s.extraction_ratio).collect();
    if ratios.windows(2).all(|w| w[0] == w[1]) && !ratios.is_empty() {
        score += 5.0; // Consistent ratio bonus
    // Special bonus for certain aesthetic patterns
    if let Some(config) = &discovery.membrane_config {
        // The legendary (3,7) configuration
        if config.outer == 3 && config.inner == 7 {
            score += 3.7;
        }
        
        // Symmetric k values
        if config.k_outer == config.k_inner {
            score += 2.0;
    score
/// Reward calculation based on beauty score and depth
fn calculate_reward(discovery: &VerifiedDiscovery, params: &RewardParameters) -> u128 {
    let base = params.base_reward;
    let depth_bonus = (discovery.chain_depth as f64 * params.depth_multiplier) as u128;
    base + depth_bonus * base / 100
fn main() {
    println!("🌐 PRIME DISCOVERY PROTOCOL - NEAR Blockchain Architecture");
    println!("=======================================================\n");
    println!("📋 SYSTEM OVERVIEW");
    println!("=================");
    println!("Contributors discover primes → Submit on-chain → Validators verify → Rewards distributed\n");
    println!("🔄 WORKFLOW");
    println!("===========");
    println!("1. OFF-CHAIN: Contributors run compute-intensive prime searches");
    println!("2. ON-CHAIN: Submit discovery with proof data");
    println!("3. OFF-CHAIN: Multiple validators verify independently");
    println!("4. ON-CHAIN: Submit verification proofs");
    println!("5. AUTOMATIC: After threshold, discovery is accepted and rewards distributed\n");
    println!("💎 INCENTIVE STRUCTURE");
    println!("=====================");
    println!("- Base reward for any verified prime");
    println!("- Exponential bonus for chain depth");
    println!("- Special bonuses for aesthetic patterns");
    println!("- Small reward for verification work");
    println!("- Penalties for false submissions\n");
    // Simulate some discoveries
    simulate_protocol();
fn simulate_protocol() {
    println!("🎮 SIMULATION");
    println!("=============\n");
    // Initialize state
    let mut state = ChainState {
        pending_discoveries: HashMap::new(),
        verified_discoveries: BTreeMap::new(),
        contributors: HashMap::new(),
        global_stats: GlobalStats {
            total_discoveries: 0,
            deepest_chain: 0,
            largest_prime_digits: 0,
            total_rewards_distributed: 0,
        },
        reward_params: RewardParameters {
            base_reward: 1_000_000_000_000_000_000_000_000, // 1 NEAR
            depth_multiplier: 50.0, // 50% bonus per depth level
            record_bonus: 10_000_000_000_000_000_000_000_000, // 10 NEAR
            verification_reward: 100_000_000_000_000_000_000_000, // 0.1 NEAR
            required_verifications: 3,
    };
    // Simulate discovery 1: Our famous chain
    let discovery1 = PrimeDiscovery {
        prime: "307050703".to_string(),
        chain: vec![
            HarmonicStep { value: "37573".to_string(), extraction_ratio: 2, is_prime: true },
            HarmonicStep { value: "353".to_string(), extraction_ratio: 2, is_prime: true },
            HarmonicStep { value: "3".to_string(), extraction_ratio: 3, is_prime: true },
        ],
        membrane_config: Some(MembraneConfig {
            outer: 3,
            inner: 7,
            k_outer: 1,
            k_inner: 1,
            seed: "5".to_string(),
        }),
        discoverer: "alice.near".to_string(),
        block_height: 100000,
        beauty_score: 0.0, // Will be calculated
    // Calculate beauty score
    let mut discovery1 = discovery1;
    discovery1.beauty_score = calculate_beauty_score(&discovery1);
    println!("Discovery 1: {} by {}", discovery1.prime, discovery1.discoverer);
    println!("  Chain depth: {}", discovery1.chain.len());
    println!("  Beauty score: {:.2}", discovery1.beauty_score);
    println!("  Potential reward: {} yoctoNEAR", 
        calculate_reward(&VerifiedDiscovery {
            id: "disc1".to_string(),
            prime: discovery1.prime.clone(),
            chain_depth: discovery1.chain.len(),
            beauty_score: discovery1.beauty_score,
            discoverer: discovery1.discoverer.clone(),
            verified_at: 0,
            reward_paid: 0,
        }, &state.reward_params)
    );
    // Simulate discovery 2: A deeper chain
    let discovery2 = PrimeDiscovery {
        prime: "130030031".to_string(),
            HarmonicStep { value: "10301".to_string(), extraction_ratio: 2, is_prime: true },
            HarmonicStep { value: "131".to_string(), extraction_ratio: 2, is_prime: true },
            HarmonicStep { value: "11".to_string(), extraction_ratio: 2, is_prime: true },
            outer: 1,
            inner: 3,
            k_outer: 0,
            k_inner: 2,
            seed: "3".to_string(),
        discoverer: "bob.near".to_string(),
        block_height: 100500,
        beauty_score: 0.0,
    let mut discovery2 = discovery2;
    discovery2.beauty_score = calculate_beauty_score(&discovery2);
    println!("\nDiscovery 2: {} by {}", discovery2.prime, discovery2.discoverer);
    println!("  Chain depth: {}", discovery2.chain.len());
    println!("  Beauty score: {:.2}", discovery2.beauty_score);
    println!("\n🏆 LEADERBOARD PREVIEW");
    println!("======================");
    println!("Rank | Prime       | Depth | Beauty | Discoverer");
    println!("-----|-------------|-------|--------|------------");
    println!("1    | 130030031   | 3     | {:.1}  | bob.near", discovery2.beauty_score);
    println!("2    | 307050703   | 3     | {:.1}  | alice.near", discovery1.beauty_score);
    println!("\n💡 STATE MUTATIONS");
    println!("==================");
    println!("1. submit_discovery(): Add to pending_discoveries");
    println!("2. submit_verification(): Update verification count");
    println!("3. Auto-trigger on threshold: Move to verified, distribute rewards");
    println!("4. claim_rewards(): Update balances, mark as claimed");
    println!("\n🔍 VERIFICATION REQUIREMENTS");
    println!("============================");
    println!("- Independent primality verification of all numbers");
    println!("- Correct subharmonic extraction verification");
    println!("- Chain continuity verification");
    println!("- No double-spending of discoveries");
