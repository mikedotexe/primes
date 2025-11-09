use primes::{MembraneConfig, is_prime_miller_rabin};
use num_bigint::BigUint;
use std::str::FromStr;

fn find_primes_for_seed(seed: &str) {
    println!("\nSeed: '{}' (length {})", seed, seed.len());
    println!("{}", "─".repeat(50));
    
    let mut all_results = Vec::new();
    // Test different k configurations
    for k_outer in 0..=3 {
        for k_inner in 0..=3 {
            let mut primes_found = Vec::new();
            
            // For the given seed, try different test patterns
            // First try the seed itself
            let membrane_str = format!(
                "3{}7{}{}{}7{}3",
                "0".repeat(k_outer),
                "0".repeat(k_inner),
                seed,
                "0".repeat(k_outer)
            );
            if let Ok(num) = BigUint::from_str(&membrane_str) {
                if is_prime_miller_rabin(&num) {
                    primes_found.push(membrane_str.clone());
                }
            }
            // Also try variations if seed is all the same digit
            if seed.chars().all(|c| c == seed.chars().next().unwrap()) {
                let digit = seed.chars().next().unwrap();
                
                // Try other repeating digits of same length
                for d in '1'..='9' {
                    if d != digit {
                        let alt_seed = d.to_string().repeat(seed.len());
                        let alt_membrane = format!(
                            "3{}7{}{}{}7{}3",
                            "0".repeat(k_outer),
                            "0".repeat(k_inner),
                            alt_seed,
                            "0".repeat(k_outer)
                        );
                        
                        if let Ok(num) = BigUint::from_str(&alt_membrane) {
                            if is_prime_miller_rabin(&num) {
                                primes_found.push(alt_membrane);
                            }
                        }
                    }
            if !primes_found.is_empty() {
                all_results.push((k_outer, k_inner, primes_found));
        }
    }
    // Sort by number of primes found
    all_results.sort_by(|a, b| b.2.len().cmp(&a.2.len()));
    if all_results.is_empty() {
        println!("No primes found with any configuration!");
    } else {
        println!("Best configurations:");
        for (k_out, k_in, primes) in all_results.iter().take(3) {
            println!("  k=({},{}) → {} primes found", k_out, k_in, primes.len());
            for prime in primes.iter().take(3) {
                println!("    {} ✓", prime);
                let url = format!("https://www.wolframalpha.com/input/?i=isprime({})", prime);
                println!("    Verify: {}", url);
}
fn test_large_desert_hypothesis() {
    println!("\n\n🏜️ TESTING LARGE DESERT HYPOTHESIS");
    println!("===================================");
    println!("\nWhen k values are large, do the left and right sides become independent primes?\n");
    // Test with large k values
    for (k_outer, k_inner, seed) in vec![(3, 3, "5"), (4, 4, "5"), (2, 4, "555")] {
        println!("\nConfiguration: (3,7) k=({},{}) seed='{}'", k_outer, k_inner, seed);
        
        let full_membrane = format!(
            "3{}7{}{}{}7{}3",
            "0".repeat(k_outer),
            "0".repeat(k_inner),
            seed,
            "0".repeat(k_outer)
        );
        println!("Full membrane: {}", full_membrane);
        // Check if full membrane is prime
        if let Ok(num) = BigUint::from_str(&full_membrane) {
            let is_prime = is_prime_miller_rabin(&num);
            println!("Is prime? {}", if is_prime { "YES ✓" } else { "NO" });
            if is_prime {
                let url = format!("https://www.wolframalpha.com/input/?i=isprime({})", full_membrane);
                println!("Verify: {}", url);
        // Now check left and right halves
        let left_part = format!("3{}7", "0".repeat(k_outer));
        let right_part = format!("7{}3", "0".repeat(k_outer));
        println!("\nChecking if parts are independently prime:");
        println!("  Left part: {}", left_part);
        if let Ok(num) = BigUint::from_str(&left_part) {
            println!("  Is prime? {}", if is_prime { "YES ✓" } else { "NO" });
        println!("  Right part: {}", right_part);
        if let Ok(num) = BigUint::from_str(&right_part) {
        // Check the "atom cores" without seed
        let left_atom = format!("3{}7{}", "0".repeat(k_outer), "0".repeat(k_inner));
        let right_atom = format!("{}7{}3", "0".repeat(k_inner), "0".repeat(k_outer));
        println!("\nChecking membrane 'atoms' without seed:");
        println!("  Left atom: {}", left_atom);
        if let Ok(num) = BigUint::from_str(&left_atom) {
        println!("  Right atom: {}", right_atom);
        if let Ok(num) = BigUint::from_str(&right_atom) {
fn main() {
    println!("🔬 VERIFYING SEED LENGTH EXAMPLES");
    println!("=================================");
    // Test the exact seeds mentioned in the document
    find_primes_for_seed("5");
    find_primes_for_seed("55");
    find_primes_for_seed("555");
    find_primes_for_seed("5555");
    find_primes_for_seed("55555");
    // Test the large desert hypothesis
    test_large_desert_hypothesis();
