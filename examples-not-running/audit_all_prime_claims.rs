//! Audit all prime claims in documentation
//! 
//! Searches for numbers claimed to be prime and verifies them

use primes::is_prime;
use num_bigint::BigUint;
use std::str::FromStr;
use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {
    println!("Auditing all prime claims in documentation...\n");
    
    // Look for patterns that might indicate a prime claim
    let patterns = vec![
        r"prime[:\s]+(\d+)",
        r"(\d+).*prime",
        r"✓\s*(\d+)",
        r"→\s*(\d+).*PRIME",
        r"Example:.*?(\d{5,})",  // Large numbers in examples
        r"membrane.*?(\d{5,})",   // Large numbers near membrane mentions
    ];
    
    let mut all_numbers = std::collections::HashSet::new();
    let mut false_primes = Vec::new();
    
    // Search documentation files
    let doc_files = vec![
        "CLAUDE.md",
        "EVIDENCE.md", 
        "README.md",
        "MEMBRANE_PRIME_README.md",
        "LAGRANGE_POINTS.md",
        "prime-physics-engine/README.md",
        "prime-physics-engine/CLAUDE.md",
    ];
    
    for file_path in &doc_files {
        let full_path = format!("/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/{}", file_path);
        if Path::new(&full_path).exists() {
            if let Ok(content) = fs::read_to_string(&full_path) {
                println!("Checking {}...", file_path);
                
                for pattern in &patterns {
                    let re = Regex::new(pattern).unwrap();
                    for cap in re.captures_iter(&content) {
                        if let Some(num_str) = cap.get(1) {
                            let num_text = num_str.as_str();
                            // Only check numbers with 5+ digits
                            if num_text.len() >= 5 {
                                all_numbers.insert(num_text.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Also check for the specific problematic number
    all_numbers.insert("300700300703".to_string());
    
    println!("\nFound {} numbers to verify", all_numbers.len());
    println!("Checking each one...\n");
    
    for num_str in &all_numbers {
        if let Ok(num) = BigUint::from_str(num_str) {
            let is_prime_result = is_prime(&num);
            
            // For large numbers, do a quick divisibility check
            let mut factors = Vec::new();
            for p in &[2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47] {
                if &num % p == 0u32.into() && &num != &(*p).into() {
                    factors.push(*p);
                }
            }
            
            if !is_prime_result && num_str.len() > 4 {
                false_primes.push((num_str.clone(), factors.clone()));
                println!("❌ {} is COMPOSITE!", num_str);
                if !factors.is_empty() {
                    println!("   Divisible by: {:?}", factors);
                }
            } else if is_prime_result && num_str.len() > 6 {
                println!("✓ {} is prime", num_str);
            }
        }
    }
    
    if false_primes.is_empty() {
        println!("\n✅ All verified numbers are correctly identified!");
    } else {
        println!("\n⚠️  Found {} false prime claims:", false_primes.len());
        for (num, factors) in &false_primes {
            println!("  {} - divisible by {:?}", num, factors);
        }
    }
    
    // Now let's specifically search for patterns that might generate our problem number
    println!("\n\nSearching for patterns that might generate 300700300703...");
    
    // Check if any membrane configs could generate this
    let problem_str = "300700300703";
    let patterns_to_check = vec![
        ("3", "7", "3", "7", "3"),  // Various arrangements
        ("30", "70", "03", "00", "703"),
        ("300", "700", "300", "70", "3"),
    ];
    
    for (a, b, c, d, e) in patterns_to_check {
        let combined = format!("{}{}{}{}{}", a, b, c, d, e);
        if combined == problem_str {
            println!("Pattern {}-{}-{}-{}-{} generates the problem number!", a, b, c, d, e);
        }
    }
}