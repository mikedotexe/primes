use primes::{
    miller_rabin_test,
};
use num_bigint::BigUint;
// Helper function to construct membrane numbers
fn construct_membrane_with_config(config: &MembraneConfig, middle: &str, _base: u32) -> String {
    // Build the membrane structure manually
    let mut result = String::new();
    
    // Left side
    result.push_str(&config.outer.to_string());
    for _ in 0..config.k_outer {
        result.push('0');
    }
    result.push_str(&config.inner.to_string());
    for _ in 0..config.k_inner {
    // Middle
    result.push_str(middle);
    // Right side (mirror)
    result
}

// Helper function to check primality
fn is_prime_string(s: &str) -> bool {
    if let Ok(n) = s.parse::<BigUint>() {
        miller_rabin_test(&n, 20)
    } else {
        false
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct ConfigKey {
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
impl From<&MembraneConfig> for ConfigKey {
    fn from(config: &MembraneConfig) -> Self {
        ConfigKey {
            outer: config.outer,
            inner: config.inner,
            k_outer: config.k_outer,
            k_inner: config.k_inner,
        }
#[derive(Debug, Clone)]
struct ResonanceResult {
    source_config: ConfigKey,
    target_config: ConfigKey,
    source_primes: Vec<String>,
    amplified_primes: Vec<String>,
    base_success_rate: f64,
    resonance_success_rate: f64,
    resonance_strength: f64,
#[derive(Debug)]
struct ResonanceChain {
    configs: Vec<ConfigKey>,
    stage_results: Vec<Vec<String>>, // Primes at each stage
    final_success_rate: f64,
    chain_amplification: f64,
fn test_config_resonance(
    source_config: &MembraneConfig,
    target_config: &MembraneConfig,
    initial_seeds: &[&str],
    base: u32,
) -> ResonanceResult {
    // First, generate primes using source config
    let mut source_primes = Vec::new();
    for seed in initial_seeds {
        // Construct membrane number directly
        let middle = seed.to_string();
        let number_str = construct_membrane_with_config(source_config, &middle, base);
        if is_prime_string(&number_str) {
            source_primes.push(number_str);
    // Calculate base success rate for target config with original seeds
    let mut base_successes = 0;
        let number_str = construct_membrane_with_config(target_config, &middle, base);
            base_successes += 1;
    let base_success_rate = base_successes as f64 / initial_seeds.len() as f64;
    // Now use source primes as seeds for target config (extract middle portion)
    let mut amplified_primes = Vec::new();
    for prime in &source_primes {
        // Extract the middle portion from the source prime
        let seed = extract_seed_from_membrane(prime, source_config);
        if !seed.is_empty() {
            let number_str = construct_membrane_with_config(target_config, &seed, base);
            if is_prime_string(&number_str) {
                amplified_primes.push(number_str);
            }
    // Calculate resonance success rate
    let resonance_success_rate = if !source_primes.is_empty() {
        amplified_primes.len() as f64 / source_primes.len() as f64
        0.0
    };
    // Calculate resonance strength (how much better than base rate)
    let resonance_strength = if base_success_rate > 0.0 {
        resonance_success_rate / base_success_rate
    } else if resonance_success_rate > 0.0 {
        f64::INFINITY
        1.0
    ResonanceResult {
        source_config: source_config.into(),
        target_config: target_config.into(),
        source_primes,
        amplified_primes,
        base_success_rate,
        resonance_success_rate,
        resonance_strength,
fn extract_seed_from_membrane(membrane_number: &str, config: &MembraneConfig) -> String {
    // Calculate the positions where the seed should be
    let total_boundary_len = 2 + 2 * config.k_outer + 2 * config.k_inner;
    if membrane_number.len() <= 2 * total_boundary_len as usize {
        return String::new(); // No seed to extract
    let start = total_boundary_len as usize;
    let end = membrane_number.len() - total_boundary_len as usize;
    membrane_number[start..end].to_string()
fn discover_resonance_chains(
    configs: &[MembraneConfig],
    max_chain_length: usize,
) -> Vec<ResonanceChain> {
    let mut chains = Vec::new();
    // Try all possible starting configurations
    for start_config in configs {
        let mut current_chain = vec![start_config.into()];
        let mut current_seeds: Vec<String> = initial_seeds.iter().map(|s| s.to_string()).collect();
        let mut stage_results = vec![];
        
        // Build chain up to max length
        for _chain_pos in 0..max_chain_length {
            let mut best_next_config = None;
            let mut best_primes = Vec::new();
            let mut best_rate = 0.0;
            
            // Try each config as next in chain
            for next_config in configs {
                let next_key: ConfigKey = next_config.into();
                
                // Skip if already in chain
                if current_chain.contains(&next_key) {
                    continue;
                }
                // Generate primes with current seeds
                let mut primes = Vec::new();
                for seed in &current_seeds {
                    let number_str = construct_membrane_with_config(next_config, seed, base);
                    if is_prime_string(&number_str) {
                        primes.push(number_str);
                    }
                let success_rate = primes.len() as f64 / current_seeds.len() as f64;
                if success_rate > best_rate && !primes.is_empty() {
                    best_rate = success_rate;
                    best_primes = primes;
                    best_next_config = Some(next_config);
            // Add best config to chain if found
            if let Some(config) = best_next_config {
                current_chain.push(config.into());
                stage_results.push(best_primes.clone());
                // Extract seeds from primes for next iteration
                current_seeds = best_primes.iter()
                    .map(|p| extract_seed_from_membrane(p, config))
                    .filter(|s| !s.is_empty())
                    .collect();
                if current_seeds.is_empty() {
                    break; // Can't continue chain
            } else {
                break; // No good next config found
        // Calculate chain metrics
        if stage_results.len() > 1 {
            let final_count = stage_results.last().unwrap().len();
            let final_success_rate = final_count as f64 / initial_seeds.len() as f64;
            // Calculate cumulative amplification
            let mut amplification = 1.0;
            for i in 0..stage_results.len() {
                let stage_rate = stage_results[i].len() as f64 / 
                    if i == 0 { initial_seeds.len() } else { stage_results[i-1].len() } as f64;
                amplification *= stage_rate;
            chains.push(ResonanceChain {
                configs: current_chain,
                stage_results,
                final_success_rate,
                chain_amplification: amplification,
            });
    // Sort by chain amplification
    chains.sort_by(|a, b| b.chain_amplification.partial_cmp(&a.chain_amplification).unwrap());
    chains
fn visualize_resonance_network(results: &[ResonanceResult], min_strength: f64) {
    println!("\n=== RESONANCE NETWORK VISUALIZATION ===");
    println!("(Showing connections with resonance strength >= {:.1})", min_strength);
    println!();
    // Build adjacency map
    let mut network: HashMap<ConfigKey, Vec<(ConfigKey, f64)>> = HashMap::new();
    for result in results {
        if result.resonance_strength >= min_strength {
            network.entry(result.source_config.clone())
                .or_insert_with(Vec::new)
                .push((result.target_config.clone(), result.resonance_strength));
    // Print network
    for (source, targets) in &network {
        println!("({},{}) k=({},{})", source.outer, source.inner, source.k_outer, source.k_inner);
        for (target, strength) in targets {
            println!("  → ({},{}) k=({},{}) [strength: {:.2}x]", 
                target.outer, target.inner, target.k_outer, target.k_inner, strength);
        println!();
    // Find strongly connected components
    println!("\n=== RESONANCE CLUSTERS ===");
    let mut visited = HashSet::new();
    let mut cluster_id = 0;
    for source in network.keys() {
        if !visited.contains(source) {
            let mut cluster = HashSet::new();
            let mut to_visit = vec![source.clone()];
            while let Some(current) = to_visit.pop() {
                if visited.insert(current.clone()) {
                    cluster.insert(current.clone());
                    
                    // Add all targets
                    if let Some(targets) = network.get(&current) {
                        for (target, _) in targets {
                            if !visited.contains(target) {
                                to_visit.push(target.clone());
                            }
                        }
                    // Add all sources that target this
                    for (src, targets) in &network {
                        if targets.iter().any(|(t, _)| t == &current) && !visited.contains(src) {
                            to_visit.push(src.clone());
            if cluster.len() > 1 {
                cluster_id += 1;
                println!("Cluster {}: {} configurations", cluster_id, cluster.len());
                for config in &cluster {
                    println!("  - ({},{}) k=({},{})", 
                        config.outer, config.inner, config.k_outer, config.k_inner);
                println!();
fn main() {
    println!("=== MEMBRANE RESONANCE NETWORK DISCOVERY ===");
    println!("Testing configuration pairs for resonance effects...\n");
    let base = 10;
    let test_seeds = vec!["1", "3", "5", "7", "9", "11", "13", "17", "19", "23"];
    // Define test configurations (including the legendary (3,7))
    let configs = vec![
        MembraneConfig::new(base, 3, 7, 0, 0),
        MembraneConfig::new(base, 3, 7, 1, 1),
        MembraneConfig::new(base, 3, 7, 0, 1),
        MembraneConfig::new(base, 3, 3, 0, 1),
        MembraneConfig::new(base, 7, 3, 0, 0),
        MembraneConfig::new(base, 1, 3, 0, 0),
        MembraneConfig::new(base, 1, 7, 0, 0),
        MembraneConfig::new(base, 9, 1, 0, 0),
        MembraneConfig::new(base, 3, 9, 0, 1),
        MembraneConfig::new(base, 7, 7, 1, 0),
    ];
    // Test all configuration pairs
    let mut all_results = Vec::new();
    let mut strong_resonances = Vec::new();
    println!("Testing {} configuration pairs...\n", configs.len() * configs.len());
    for source in &configs {
        for target in &configs {
            let result = test_config_resonance(source, target, &test_seeds, base);
            // Report strong resonances (>2x amplification)
            if result.resonance_strength > 2.0 {
                strong_resonances.push(result.clone());
            all_results.push(result);
    // Report strong resonances
    println!("\n=== STRONG RESONANCES (>2x amplification) ===");
    strong_resonances.sort_by(|a, b| b.resonance_strength.partial_cmp(&a.resonance_strength).unwrap());
    for (i, result) in strong_resonances.iter().take(10).enumerate() {
        println!("\n{}. ({},{}) k=({},{}) → ({},{}) k=({},{})", 
            i + 1,
            result.source_config.outer, result.source_config.inner, 
            result.source_config.k_outer, result.source_config.k_inner,
            result.target_config.outer, result.target_config.inner,
            result.target_config.k_outer, result.target_config.k_inner
        );
        println!("   Base success rate: {:.1}%", result.base_success_rate * 100.0);
        println!("   Resonance success rate: {:.1}%", result.resonance_success_rate * 100.0);
        println!("   Resonance strength: {:.2}x", result.resonance_strength);
        println!("   Source primes: {} generated", result.source_primes.len());
        println!("   Amplified primes: {} generated", result.amplified_primes.len());
        // Show example cascade
        if !result.source_primes.is_empty() && !result.amplified_primes.is_empty() {
            println!("   Example cascade: {} → {}", 
                result.source_primes[0], 
                result.amplified_primes[0]
            );
    // Test legendary (3,7) specifically
    println!("\n=== LEGENDARY (3,7) CONFIGURATION ANALYSIS ===");
    let legendary_config = MembraneConfig::new(base, 3, 7, 1, 1);
    println!("\nConfigurations that resonate INTO (3,7) k=(1,1):");
    for result in &all_results {
        if result.target_config == ConfigKey::from(&legendary_config) && result.resonance_strength > 1.5 {
            println!("  ({},{}) k=({},{}) → {:.2}x amplification",
                result.source_config.outer, result.source_config.inner,
                result.source_config.k_outer, result.source_config.k_inner,
                result.resonance_strength
    println!("\nConfigurations that (3,7) k=(1,1) resonates WITH:");
        if result.source_config == ConfigKey::from(&legendary_config) && result.resonance_strength > 1.5 {
            println!("  → ({},{}) k=({},{}) gives {:.2}x amplification",
                result.target_config.outer, result.target_config.inner,
                result.target_config.k_outer, result.target_config.k_inner,
    // Discover resonance chains
    println!("\n=== RESONANCE CHAINS ===");
    println!("Discovering multi-stage amplification chains...\n");
    let chains = discover_resonance_chains(&configs, &test_seeds, base, 4);
    for (i, chain) in chains.iter().take(5).enumerate() {
        println!("Chain {}: {:.2}x total amplification", i + 1, chain.chain_amplification);
        for (j, config) in chain.configs.iter().enumerate() {
            print!("  Stage {}: ({},{}) k=({},{})", 
                j + 1, config.outer, config.inner, config.k_outer, config.k_inner);
            if j < chain.stage_results.len() {
                println!(" → {} primes", chain.stage_results[j].len());
        println!("  Final success rate: {:.1}%", chain.final_success_rate * 100.0);
        // Show example chain execution
        if !chain.stage_results.is_empty() && !chain.stage_results[0].is_empty() {
            println!("  Example: seed '5' → {}", chain.stage_results[0][0]);
            for i in 1..chain.stage_results.len().min(3) {
                if !chain.stage_results[i].is_empty() {
                    println!("           → {}", chain.stage_results[i][0]);
    // Visualize the resonance network
    visualize_resonance_network(&all_results, 1.5);
    // Summary statistics
    println!("\n=== SUMMARY STATISTICS ===");
    let total_pairs = all_results.len();
    let resonant_pairs = all_results.iter().filter(|r| r.resonance_strength > 1.5).count();
    let strong_pairs = all_results.iter().filter(|r| r.resonance_strength > 2.0).count();
    let super_pairs = all_results.iter().filter(|r| r.resonance_strength > 3.0).count();
    println!("Total configuration pairs tested: {}", total_pairs);
    println!("Resonant pairs (>1.5x): {} ({:.1}%)", resonant_pairs, resonant_pairs as f64 / total_pairs as f64 * 100.0);
    println!("Strong resonances (>2x): {} ({:.1}%)", strong_pairs, strong_pairs as f64 / total_pairs as f64 * 100.0);
    println!("Super resonances (>3x): {} ({:.1}%)", super_pairs, super_pairs as f64 / total_pairs as f64 * 100.0);
    // Find the most "connective" configurations
    println!("\n=== MOST CONNECTIVE CONFIGURATIONS ===");
    let mut outgoing_connections: HashMap<ConfigKey, usize> = HashMap::new();
    let mut incoming_connections: HashMap<ConfigKey, usize> = HashMap::new();
        if result.resonance_strength > 1.5 {
            *outgoing_connections.entry(result.source_config.clone()).or_insert(0) += 1;
            *incoming_connections.entry(result.target_config.clone()).or_insert(0) += 1;
    let mut outgoing_vec: Vec<_> = outgoing_connections.iter().collect();
    outgoing_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("\nBest source configurations (feed many others):");
    for (config, count) in outgoing_vec.iter().take(3) {
        println!("  ({},{}) k=({},{}) → resonates with {} configurations",
            config.outer, config.inner, config.k_outer, config.k_inner, count);
    let mut incoming_vec: Vec<_> = incoming_connections.iter().collect();
    incoming_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("\nBest amplifier configurations (receive from many):");
    for (config, count) in incoming_vec.iter().take(3) {
        println!("  ({},{}) k=({},{}) ← amplifies {} configurations",
