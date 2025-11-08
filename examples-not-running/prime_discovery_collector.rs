//! Prime Discovery Collector
//! =========================
//! 
//! This script runs through various configurations and collects ALL
//! primes discovered, outputting them with verification links.

use num_bigint::BigUint;
#[derive(Debug, Clone)]
struct DiscoveredPrime {
    config: String,
    seed: u32,
    prime: BigUint,
    digit_count: usize,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 PRIME DISCOVERY COLLECTOR");
    println!("============================\n");
    
    let mut output = File::create("discovered_primes_master_list.csv")?;
    writeln!(output, "Configuration,Seed,Prime,Digits,VerificationURL")?;
    let mut all_primes = Vec::new();
    let mut unique_primes = HashSet::new();
    // Test 1: Classic configurations
    println!("Testing classic configurations...");
    test_classic_configs(&mut all_primes, &mut unique_primes)?;
    // Test 2: Breathing membranes
    println!("\nTesting breathing membranes...");
    test_breathing_membranes(&mut all_primes, &mut unique_primes)?;
    // Test 3: Edge pairs
    println!("\nTesting edge pairs...");
    test_edge_pairs(&mut all_primes, &mut unique_primes)?;
    // Test 4: Exclusive configurations
    println!("\nTesting exclusive configurations...");
    test_exclusive_configs(&mut all_primes, &mut unique_primes)?;
    // Test 5: Large k-values
    println!("\nTesting large k-values for big primes...");
    test_large_k_values(&mut all_primes, &mut unique_primes)?;
    // Write all discovered primes to CSV
    for discovered in &all_primes {
        let verification_url = format!(
            "https://www.wolframalpha.com/input?i=is+{}+prime",
            discovered.prime
        );
        
        writeln!(output, "{},{},{},{},{}",
            discovered.config,
            discovered.seed,
            discovered.prime,
            discovered.digit_count,
            verification_url
        )?;
    }
    // Summary statistics
    println!("\n📊 DISCOVERY SUMMARY");
    println!("====================");
    println!("Total primes found: {}", all_primes.len());
    println!("Unique primes: {}", unique_primes.len());
    // Group by digit count
    let mut by_digits = std::collections::HashMap::new();
    for p in &all_primes {
        *by_digits.entry(p.digit_count).or_insert(0) += 1;
    println!("\nPrimes by digit count:");
    let mut digit_counts: Vec<_> = by_digits.into_iter().collect();
    digit_counts.sort_by_key(|&(k, _)| k);
    for (digits, count) in digit_counts {
        println!("  {:2} digits: {} primes", digits, count);
    // Show some examples
    println!("\n🌟 EXAMPLE PRIMES DISCOVERED:");
    // Show smallest
    if let Some(smallest) = all_primes.iter().min_by_key(|p| &p.prime) {
        println!("\nSmallest: {} ({} digits)", smallest.prime, smallest.digit_count);
        println!("  Config: {}, Seed: {}", smallest.config, smallest.seed);
    // Show largest
    if let Some(largest) = all_primes.iter().max_by_key(|p| &p.prime) {
        println!("\nLargest: {} ({} digits)", largest.prime, largest.digit_count);
        println!("  Config: {}, Seed: {}", largest.config, largest.seed);
    // Show some interesting ones
    println!("\nInteresting primes:");
    for p in all_primes.iter().filter(|p| p.digit_count >= 13).take(5) {
        println!("  {} ({} digits, {})", p.prime, p.digit_count, p.config);
    println!("\n✅ All primes saved to: discovered_primes_master_list.csv");
    println!("🔗 Each prime includes a Wolfram Alpha verification link");
    Ok(())
fn test_classic_configs(
    all_primes: &mut Vec<DiscoveredPrime>,
    unique_primes: &mut HashSet<BigUint>
) -> Result<(), Box<dyn std::error::Error>> {
    let configs = vec![
        ((3, 7, 2, 2), "(3,7) k=(2,2)"),
        ((7, 3, 2, 2), "(7,3) k=(2,2)"),
        ((3, 3, 1, 1), "(3,3) k=(1,1)"),
        ((5, 5, 1, 1), "(5,5) k=(1,1)"),
        ((1, 9, 2, 2), "(1,9) k=(2,2)"),
        ((3, 5, 2, 2), "(3,5) k=(2,2)"),
    ];
    for ((outer, inner, k_outer, k_inner), name) in configs {
        let config = MembraneConfig::new(10, outer, inner, k_outer, k_inner);
        for seed in 0..=9 {
            if let Ok(num) = config.construct_number(seed) {
                if is_prime(&num) && unique_primes.insert(num.clone()) {
                    let discovered = DiscoveredPrime {
                        config: name.to_string(),
                        seed,
                        prime: num.clone(),
                        digit_count: num.to_string().len(),
                    };
                    
                    println!("  Found: {} (seed {})", num, seed);
                    all_primes.push(discovered);
                }
            }
        }
fn test_breathing_membranes(
        ((3, 3, 0, 1), "(3,3) k=(0,1)"),
        ((3, 3, 1, 0), "(3,3) k=(1,0)"),
        ((3, 3, 1, 3), "(3,3) k=(1,3)"),
        ((3, 3, 3, 1), "(3,3) k=(3,1)"),
        ((3, 7, 1, 3), "(3,7) k=(1,3)"),
        ((7, 3, 3, 1), "(7,3) k=(3,1)"),
fn test_edge_pairs(
    // Edge pairs for base 10
        ((1, 9, 1, 1), "(1,9) edge pair"),
        ((2, 8, 1, 1), "(2,8) edge pair"),
        ((3, 7, 1, 1), "(3,7) edge pair"),
        ((4, 6, 1, 1), "(4,6) edge pair"),
fn test_exclusive_configs(
    // Configurations known to have seed exclusivity
        ((3, 3, 1, 1), "(3,3) k=(1,1) exclusive"),
        ((3, 7, 1, 1), "(3,7) k=(1,1)"),
        ((3, 5, 1, 1), "(3,5) k=(1,1)"),
fn test_large_k_values(
    // Test larger k values for bigger primes
        ((3, 7, 5, 5), "(3,7) k=(5,5)"),
        ((3, 7, 7, 7), "(3,7) k=(7,7)"),
        ((3, 3, 10, 1), "(3,3) k=(10,1)"),
        ((1, 9, 10, 10), "(1,9) k=(10,10)"),
        // Test fewer seeds for large numbers
        for seed in [0, 1, 3, 5, 7, 9] {
                    println!("  Found large prime: {} digits (seed {})", 
                        discovered.digit_count, seed);
fn is_prime(n: &BigUint) -> bool {
    if n < &BigUint::from(2u32) {
        return false;
    if n == &BigUint::from(2u32) {
        return true;
    if n % BigUint::from(2u32) == BigUint::from(0u32) {
    let sqrt_n = n.sqrt();
    let mut i = BigUint::from(3u32);
    while i <= sqrt_n {
        if n % &i == BigUint::from(0u32) {
            return false;
        i += BigUint::from(2u32);
    true
