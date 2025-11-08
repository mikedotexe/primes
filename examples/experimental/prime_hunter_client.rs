//! Prime Hunter Client - What contributors run to participate in the protocol
//! 
//! This demonstrates the client-side software for finding and submitting prime discoveries

use prime_physics_engine::is_prime_miller_rabin;
use num_bigint::BigUint;
use std::str::FromStr;
use std::time::{Instant, Duration};
use serde::{Serialize, Deserialize};
/// Client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HunterConfig {
    pub account_id: String,
    pub target_depth: usize,
    pub search_strategy: SearchStrategy,
    pub performance_mode: PerformanceMode,
    pub auto_submit: bool,
}
pub enum SearchStrategy {
    /// Focus on known high-yield patterns
    KnownPatterns,
    
    /// Explore new configurations
    Exploration { 
        base_range: (u32, u32),
        k_range: (usize, usize),
    },
    /// Deep search on specific configuration
    DeepDive {
        outer: u32,
        inner: u32,
        k_outer: usize,
        k_inner: usize,
    /// Follow up on existing discoveries
    ChainExtension {
        starting_primes: Vec<String>,
pub enum PerformanceMode {
    /// Use all CPU cores
    Maximum,
    /// Use half cores (for background running)
    Balanced,
    /// Single core (minimal impact)
    Minimal,
/// Local discovery before submission
#[derive(Debug, Clone)]
struct LocalDiscovery {
    prime: String,
    chain: Vec<(String, usize)>,
    config: Option<(u32, u32, usize, usize, String)>, // outer, inner, k1, k2, seed
    found_at: Instant,
    estimated_value: u128,
/// Main hunter client
pub struct PrimeHunter {
    config: HunterConfig,
    discoveries: Vec<LocalDiscovery>,
    search_stats: SearchStatistics,
#[derive(Debug, Default)]
struct SearchStatistics {
    numbers_checked: u64,
    primes_found: u64,
    chains_found: u64,
    deepest_chain: usize,
    start_time: Option<Instant>,
impl PrimeHunter {
    pub fn new(config: HunterConfig) -> Self {
        Self {
            config,
            discoveries: Vec::new(),
            search_stats: SearchStatistics::default(),
        }
    }
    /// Main hunting loop
    pub fn start_hunting(&mut self) {
        println!("🏹 PRIME HUNTER CLIENT v1.0");
        println!("===========================");
        println!("Account: {}", self.config.account_id);
        println!("Strategy: {:?}", self.config.search_strategy);
        println!("Target depth: {}", self.config.target_depth);
        println!();
        
        self.search_stats.start_time = Some(Instant::now());
        match &self.config.search_strategy {
            SearchStrategy::KnownPatterns => self.hunt_known_patterns(),
            SearchStrategy::Exploration { base_range, k_range } => {
                self.explore_new_patterns(*base_range, *k_range)
            },
            SearchStrategy::DeepDive { outer, inner, k_outer, k_inner } => {
                self.deep_dive_configuration(*outer, *inner, *k_outer, *k_inner)
            SearchStrategy::ChainExtension { starting_primes } => {
                self.extend_chains(starting_primes.clone())
        self.print_summary();
    fn hunt_known_patterns(&mut self) {
        println!("🎯 Hunting with known high-yield patterns...\n");
        // Known good patterns from our research
        let patterns = vec![
            (3, 7, 1, 1),   // Our legendary configuration
            (1, 3, 0, 2),   // 100% success rate!
            (1, 1, 0, 0),   // Another perfect one
            (3, 3, 0, 1),   // Breathing pattern
        ];
        for (outer, inner, k1, k2) in patterns {
            println!("Testing ({},{}) k=({},{})...", outer, inner, k1, k2);
            
            // Try different seeds
            for seed in 1..1000 {
                let seed_str = seed.to_string();
                let membrane = self.construct_membrane(outer, inner, &seed_str, k1, k2);
                
                self.search_stats.numbers_checked += 1;
                if self.check_prime(&membrane) {
                    self.search_stats.primes_found += 1;
                    
                    // Check for chain
                    if let Some(chain) = self.find_chain(&membrane) {
                        if chain.len() >= self.config.target_depth {
                            println!("  ✨ Found depth-{} chain: {} (seed {})", 
                                chain.len(), membrane, seed);
                            
                            self.record_discovery(
                                membrane,
                                chain,
                                Some((outer, inner, k1, k2, seed_str))
                            );
                        }
                    }
                }
                // Progress indicator
                if seed % 100 == 0 {
                    print!(".");
                    use std::io::{self, Write};
                    io::stdout().flush().unwrap();
            }
            println!();
    fn explore_new_patterns(&mut self, base_range: (u32, u32), k_range: (usize, usize)) {
        println!("🔍 Exploring new pattern space...\n");
        for outer in base_range.0..=base_range.1 {
            for inner in base_range.0..=base_range.1 {
                for k1 in k_range.0..=k_range.1 {
                    for k2 in k_range.0..=k_range.1 {
                        // Quick sampling
                        let mut successes = 0;
                        let samples = 20;
                        
                        for seed in 1..=samples {
                            let membrane = self.construct_membrane(
                                outer, inner, &seed.to_string(), k1, k2
                            if self.check_prime(&membrane) {
                                successes += 1;
                                
                                if let Some(chain) = self.find_chain(&membrane) {
                                    if chain.len() >= 2 {
                                        println!("  🎲 New pattern ({},{}) k=({},{}) → depth {}",
                                            outer, inner, k1, k2, chain.len());
                                    }
                                }
                            }
                        // If promising, do deeper search
                        if successes >= 3 {
                            println!("  📈 Promising pattern: ({},{}) k=({},{}) - {}/{}",
                                outer, inner, k1, k2, successes, samples);
    fn deep_dive_configuration(&mut self, outer: u32, inner: u32, k1: usize, k2: usize) {
        println!("🏊 Deep diving into ({},{}) k=({},{})...\n", outer, inner, k1, k2);
        let start = Instant::now();
        let mut best_chain_length = 0;
        // Exhaustive search for this configuration
        for seed in 1..10000 {
            let seed_str = seed.to_string();
            let membrane = self.construct_membrane(outer, inner, &seed_str, k1, k2);
            self.search_stats.numbers_checked += 1;
            if self.check_prime(&membrane) {
                self.search_stats.primes_found += 1;
                if let Some(chain) = self.find_chain(&membrane) {
                    if chain.len() > best_chain_length {
                        best_chain_length = chain.len();
                        println!("  🏆 New record! Seed {} → depth {} chain", seed, chain.len());
                        self.record_discovery(
                            membrane.clone(),
                            chain,
                            Some((outer, inner, k1, k2, seed_str))
                        );
            // Time check
            if start.elapsed() > Duration::from_secs(60) {
                println!("  ⏱️ Time limit reached");
                break;
    fn extend_chains(&mut self, starting_primes: Vec<String>) {
        println!("🔗 Extending existing prime chains...\n");
        for prime in starting_primes {
            println!("Starting from: {}", prime);
            // Try to extend the chain
            if let Some(extended_chain) = self.find_chain(&prime) {
                if extended_chain.len() > 0 {
                    println!("  Extended by {} levels", extended_chain.len());
                    self.record_discovery(prime.clone(), extended_chain, None);
            // Try variations
            self.try_variations(&prime);
    fn try_variations(&mut self, prime: &str) {
        // Try digit modifications that might yield chains
        let chars: Vec<char> = prime.chars().collect();
        for i in 0..chars.len() {
            if chars[i] != '0' {
                // Try incrementing/decrementing non-zero digits
                for delta in [-2, -1, 1, 2] {
                    let digit = chars[i].to_digit(10).unwrap() as i32;
                    let new_digit = (digit + delta).clamp(0, 9);
                    let mut variant = chars.clone();
                    variant[i] = char::from_digit(new_digit as u32, 10).unwrap();
                    let variant_str: String = variant.into_iter().collect();
                    if self.check_prime(&variant_str) {
                        if let Some(chain) = self.find_chain(&variant_str) {
                            if chain.len() >= 2 {
                                println!("  🎭 Variation found: {} → depth {}", 
                                    variant_str, chain.len());
    fn find_chain(&mut self, start: &str) -> Option<Vec<(String, usize)>> {
        let mut chain = Vec::new();
        let mut current = start.to_string();
        for _ in 0..10 { // Max depth
            let mut found = false;
            for n in 2..=10 {
                if let Some(sub) = self.extract_subharmonic(&current, n) {
                    if sub.len() >= 2 && self.check_prime(&sub) {
                        chain.push((sub.clone(), n));
                        current = sub;
                        found = true;
                        self.search_stats.chains_found += 1;
                        break;
            if !found {
        if chain.len() > self.search_stats.deepest_chain {
            self.search_stats.deepest_chain = chain.len();
        if chain.is_empty() {
            None
        } else {
            Some(chain)
    fn record_discovery(
        &mut self, 
        prime: String, 
        chain: Vec<(String, usize)>,
        config: Option<(u32, u32, usize, usize, String)>
    ) {
        let estimated_value = self.estimate_value(&prime, &chain);
        let discovery = LocalDiscovery {
            prime,
            chain,
            found_at: Instant::now(),
            estimated_value,
        };
        self.discoveries.push(discovery);
        if self.config.auto_submit {
            println!("  📤 Auto-submitting to blockchain...");
            // In real implementation, submit to NEAR
    fn estimate_value(&self, _prime: &str, chain: &[(String, usize)]) -> u128 {
        let base = 1_000_000_000_000_000_000_000_000u128; // 1 NEAR
        let depth_multiplier = chain.len() as u128;
        base * (1 + depth_multiplier)
    fn print_summary(&self) {
        let elapsed = self.search_stats.start_time
            .map(|t| t.elapsed())
            .unwrap_or_default();
        println!("\n📊 HUNTING SUMMARY");
        println!("==================");
        println!("Time: {:.1}s", elapsed.as_secs_f64());
        println!("Numbers checked: {}", self.search_stats.numbers_checked);
        println!("Primes found: {}", self.search_stats.primes_found);
        println!("Chains found: {}", self.search_stats.chains_found);
        println!("Deepest chain: {}", self.search_stats.deepest_chain);
        println!("Discoveries: {}", self.discoveries.len());
        if !self.discoveries.is_empty() {
            println!("\n💎 TOP DISCOVERIES:");
            let mut sorted = self.discoveries.clone();
            sorted.sort_by_key(|d| std::cmp::Reverse(d.chain.len()));
            for (i, discovery) in sorted.iter().take(5).enumerate() {
                println!("{}. {} (depth {}, ~{:.2} NEAR)",
                    i + 1,
                    discovery.prime,
                    discovery.chain.len(),
                    discovery.estimated_value as f64 / 1e24
                );
    // Helper methods
    fn construct_membrane(&self, outer: u32, inner: u32, seed: &str, k1: usize, k2: usize) -> String {
        format!("{}{}{}{}{}{}{}{}{}",
            outer,
            "0".repeat(k1),
            inner,
            "0".repeat(k2),
            seed,
            outer
        )
    fn check_prime(&self, s: &str) -> bool {
        if let Ok(num) = BigUint::from_str(s) {
            is_prime_miller_rabin(&num)
            false
    fn extract_subharmonic(&self, membrane: &str, n: usize) -> Option<String> {
        if n == 0 || membrane.is_empty() {
            return None;
        let chars: Vec<char> = membrane.chars().collect();
        if chars.len() < n {
        let subharmonic: String = chars.iter()
            .enumerate()
            .filter(|(i, _)| i % n == 0)
            .map(|(_, &c)| c)
            .collect();
        let trimmed = subharmonic.trim_start_matches('0');
        if trimmed.is_empty() {
            Some(trimmed.to_string())
fn main() {
    // Example configurations for different hunting strategies
    println!("Choose your hunting strategy:\n");
    println!("1. Known Patterns - Safe, reliable");
    println!("2. Exploration - Find new patterns");
    println!("3. Deep Dive - Exhaust one configuration");
    println!("4. Chain Extension - Build on existing discoveries\n");
    // Simulate option 1: Known patterns
    let config = HunterConfig {
        account_id: "alice.near".to_string(),
        target_depth: 2,
        search_strategy: SearchStrategy::KnownPatterns,
        performance_mode: PerformanceMode::Balanced,
        auto_submit: false,
    };
    let mut hunter = PrimeHunter::new(config);
    hunter.start_hunting();
    println!("\n💰 ESTIMATED EARNINGS");
    println!("====================");
    println!("Based on current protocol parameters:");
    println!("- Depth 2 chain: ~2 NEAR");
    println!("- Depth 3 chain: ~3 NEAR");
    println!("- Depth 4 chain: ~4 NEAR + record bonus");
    println!("\nHappy hunting! 🏹");
