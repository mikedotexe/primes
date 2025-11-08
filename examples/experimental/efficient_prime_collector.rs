//! Efficient Prime Collector
//! =========================
//! 
//! Collects primes efficiently without testing overly large configurations

use num_bigint::BigUint;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 EFFICIENT PRIME COLLECTOR");
    println!("============================\n");
    
    let mut output = File::create("verified_primes_collection.csv")?;
    writeln!(output, "Configuration,Outer,Inner,K_outer,K_inner,Seed,Prime,Digits,VerificationURL")?;
    let mut total_primes = 0;
    let mut primes_by_config = std::collections::HashMap::new();
    // Test configurations systematically
    let test_ranges = vec![
        // (outer_range, inner_range, k_outer_range, k_inner_range)
        (vec![1, 3, 5, 7, 9], vec![1, 3, 5, 7, 9], vec![0, 1, 2], vec![0, 1, 2]),  // Small k
        (vec![3, 7], vec![3, 7], vec![3, 5, 7], vec![3, 5, 7]),  // Medium k
    ];
    for (outer_range, inner_range, k_outer_range, k_inner_range) in test_ranges {
        for &outer in &outer_range {
            for &inner in &inner_range {
                for &k_outer in &k_outer_range {
                    for &k_inner in &k_inner_range {
                        let config = MembraneConfig::new(10, outer, inner, k_outer, k_inner);
                        let config_str = format!("({},{}) k=({},{})", outer, inner, k_outer, k_inner);
                        
                        let mut config_primes = 0;
                        for seed in 0..=9 {
                            if let Ok(num) = config.construct_number(seed) {
                                if is_prime(&num) {
                                    let num_str = num.to_string();
                                    let digit_count = num_str.len();
                                    
                                    // Only output if not too large (for efficiency)
                                    if digit_count <= 20 {
                                        let verification_url = format!(
                                            "https://www.wolframalpha.com/input?i=is+{}+prime",
                                            num_str
                                        );
                                        
                                        writeln!(output, "{},{},{},{},{},{},{},{},{}",
                                            config_str, outer, inner, k_outer, k_inner, 
                                            seed, num_str, digit_count, verification_url
                                        )?;
                                        config_primes += 1;
                                        total_primes += 1;
                                        if digit_count >= 10 {
                                            println!("  Found {}-digit prime: {} (config: {}, seed: {})", 
                                                digit_count, num_str, config_str, seed);
                                        }
                                    }
                                }
                            }
                        }
                        if config_primes > 0 {
                            primes_by_config.insert(config_str.clone(), config_primes);
                    }
                }
            }
        }
    }
    // Test some specific interesting configurations
    println!("\nTesting special configurations...");
    let special_configs = vec![
        // Exclusive configs
        (3, 3, 1, 1, "Exclusive (3,3)"),
        (3, 7, 1, 1, "Classic (3,7)"),
        (5, 5, 1, 1, "Twin 5s"),
        
        // Breathing membranes
        (3, 3, 0, 1, "Minimal breathing"),
        (3, 3, 1, 3, "Asymmetric breathing"),
        // Edge pairs
        (1, 9, 1, 1, "Edge pair (1,9)"),
        (2, 8, 1, 1, "Edge pair (2,8)"),
        (4, 6, 1, 1, "Edge pair (4,6)"),
        // 2-digit seeds
        (3, 7, 2, 2, "Classic for 2-digit seeds"),
    for (outer, inner, k_outer, k_inner, name) in special_configs {
        let config = MembraneConfig::new(10, outer, inner, k_outer, k_inner);
        let config_str = format!("{}: ({},{}) k=({},{})", name, outer, inner, k_outer, k_inner);
        println!("\nTesting {}...", name);
        // Test 1-digit seeds
        for seed in 0..=9 {
            test_and_record(&config, seed, &config_str, &mut output, &mut total_primes)?;
        // Test some 2-digit seeds for variety
        if k_outer >= 2 && k_inner >= 2 {
            for seed in [10, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47] {
                test_and_record(&config, seed, &config_str, &mut output, &mut total_primes)?;
    // Summary
    println!("\n📊 COLLECTION SUMMARY");
    println!("====================");
    println!("Total primes collected: {}", total_primes);
    // Show configurations with most primes
    let mut config_vec: Vec<_> = primes_by_config.into_iter().collect();
    config_vec.sort_by(|a, b| b.1.cmp(&a.1));
    println!("\nTop configurations by prime count:");
    for (config, count) in config_vec.iter().take(10) {
        println!("  {}: {} primes", config, count);
    println!("\n✅ All primes saved to: verified_primes_collection.csv");
    println!("🔗 Each prime includes a Wolfram Alpha verification link");
    Ok(())
}
fn test_and_record(
    config: &MembraneConfig,
    seed: u32,
    config_str: &str,
    output: &mut File,
    total_primes: &mut usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(num) = config.construct_number(seed) {
        if is_prime(&num) {
            let num_str = num.to_string();
            let digit_count = num_str.len();
            
            if digit_count <= 25 {  // Reasonable size limit
                let verification_url = format!(
                    "https://www.wolframalpha.com/input?i=is+{}+prime",
                    num_str
                );
                
                writeln!(output, "{},{},{},{},{},{},{},{},{}",
                    config_str, config.outer, config.inner, 
                    config.k_outer, config.k_inner, 
                    seed, num_str, digit_count, verification_url
                )?;
                *total_primes += 1;
                if digit_count >= 9 {
                    println!("  Found: {} ({} digits, seed {})", 
                        num_str, digit_count, seed);
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
