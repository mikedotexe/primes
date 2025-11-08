//! Known Atomic Prime Finder - Searches for specific atom-looking configurations
//! 
//! Based on the user's memory of finding attractive "atom-looking" primes with 5 in the middle

use prime_physics_engine::is_prime_miller_rabin;
use num_bigint::BigUint;
use std::fs::{File, OpenOptions};
use std::io::Write;
use chrono::Local;
fn main() {
    println!("⚛️  Finding Known Atomic Prime Patterns");
    println!("======================================\n");
    
    // Common attractive patterns that might be what was found
    let known_patterns = vec![
        // Single membrane patterns
        ("305", "Single: (3)─(5)"),
        ("30503", "Single: (3)─(5)─(3)"),
        ("3050503", "Single: (3)─(5)─(5)─(3)"),
        ("305050503", "Single: (3)─(5)─(5)─(5)─(3)"),
        ("70507", "Single: (7)─(5)─(7)"),
        ("7050507", "Single: (7)─(5)─(5)─(7)"),
        ("90509", "Single: (9)─(5)─(9)"),
        
        // Double membrane patterns  
        ("30705073", "Double: (3)─(7)─(5)─(7)─(3)"),
        ("307050703", "Double: (3)─(7)─(5)─(7)─(3)"),
        ("3070050703", "Double: (3)─(7)──(5)──(7)─(3)"),
        ("30700507003", "Double: (3)─(7)──(5)──(7)──(3)"),
        ("3070050007003", "Double: (3)─(7)──(5)───(7)──(3)"),
        ("30905093", "Double: (3)─(9)─(5)─(9)─(3)"),
        ("309050903", "Double: (3)─(9)─(5)─(9)─(3)"),
        ("7030503077", "Double: (7)─(3)─(5)─(3)─(7)"),
        ("703050307", "Double: (7)─(3)─(5)─(3)─(7)"),
        ("70305050307", "Double: (7)─(3)─(5)─(5)─(3)─(7)"),
        ("90105010919", "Double: (9)─(1)─(5)─(1)─(9)"),
        ("901050109", "Double: (9)─(1)─(5)─(1)─(9)"),
        // Triple membrane patterns
        ("307090509070303", "Triple: (3)─(7)─(9)─(5)─(9)─(7)─(3)"),
        ("30709050907003", "Triple: (3)─(7)─(9)─(5)─(9)─(7)──(3)"),
        ("3070905090703", "Triple: (3)─(7)─(9)─(5)─(9)─(7)─(3)"),
        ("7030905090307", "Triple: (7)─(3)─(9)─(5)─(9)─(3)─(7)"),
        ("703090509030717", "Triple: (7)─(3)─(9)─(5)─(9)─(3)─(7)"),
        ("90307050703091", "Triple: (9)─(3)─(7)─(5)─(7)─(3)─(9)"),
        ("903070507030919", "Triple: (9)─(3)─(7)─(5)─(7)─(3)─(9)"),
        // Special symmetric patterns
        ("1050501", "Symmetric: (1)─(5)─(5)─(1)"),
        ("10505050501", "Symmetric: (1)─(5)─(5)─(5)─(5)─(1)"),
        ("30505050503", "Symmetric: (3)─(5)─(5)─(5)─(5)─(3)"),
        ("3050505050503", "Symmetric: (3)─(5)─(5)─(5)─(5)─(5)─(3)"),
        ("505050505", "All Fives: (5)─(5)─(5)─(5)─(5)"),
        ("50505050505", "All Fives: (5)─(5)─(5)─(5)─(5)─(5)"),
    ];
    let mut found_primes = Vec::new();
    let mut output = String::new();
    output.push_str(&format!("⚛️  ATOMIC PRIME SEARCH - {}\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
    output.push_str("=" .repeat(60).as_str());
    output.push_str("\n\n");
    println!("Testing {} known atomic patterns...\n", known_patterns.len());
    for (pattern, description) in &known_patterns {
        if let Ok(num) = pattern.parse::<BigUint>() {
            let is_prime = is_prime_miller_rabin(&num, 20);
            
            if is_prime {
                println!("✨ PRIME FOUND: {} → {}", description, pattern);
                found_primes.push((pattern.to_string(), description.to_string()));
                
                output.push_str(&format!("✨ ATOMIC PRIME #{}\n", found_primes.len()));
                output.push_str(&format!("   Pattern: {}\n", description));
                output.push_str(&format!("   Value: {}\n", pattern));
                output.push_str(&format!("   Digits: {}\n", pattern.len()));
                output.push_str(&format!("   Verify: https://www.wolframalpha.com/input/?i=isprime({})\n", pattern));
                output.push_str("\n");
            } else {
                println!("   Testing: {} → {} (composite)", description, pattern);
            }
        }
    }
    // Also check some systematic double membrane patterns
    println!("\n🔍 Checking systematic double membrane patterns...\n");
    output.push_str("\n🔍 SYSTEMATIC DOUBLE MEMBRANE SEARCH\n");
    output.push_str("=" .repeat(40).as_str());
    for outer in [1, 3, 7, 9].iter() {
        for inner in [1, 3, 7, 9].iter() {
            if outer == inner { continue; }
            // Pattern: outer-inner-5-inner-outer
            let pattern1 = format!("{}{}{}{}{}", outer, inner, 5, inner, outer);
            if let Ok(num) = pattern1.parse::<BigUint>() {
                if is_prime_miller_rabin(&num, 20) {
                    let desc = format!("Double: ({})─({})─(5)─({})─({})", outer, inner, inner, outer);
                    println!("✨ PRIME: {} → {}", desc, pattern1);
                    found_primes.push((pattern1.clone(), desc.clone()));
                    
                    output.push_str(&format!("✨ Pattern: {}\n", desc));
                    output.push_str(&format!("   Value: {}\n", pattern1));
                    output.push_str("\n");
                }
            // Pattern: outer-0-inner-0-5-0-inner-0-outer
            let pattern2 = format!("{}0{}0{}0{}0{}", outer, inner, 5, inner, outer);
            if let Ok(num) = pattern2.parse::<BigUint>() {
                    let desc = format!("Double: ({})─({})──(5)──({})─({})", outer, inner, inner, outer);
                    println!("✨ PRIME: {} → {}", desc, pattern2);
                    found_primes.push((pattern2.clone(), desc.clone()));
                    output.push_str(&format!("   Value: {}\n", pattern2));
    // Summary
    output.push_str("\n" + &"=" .repeat(60) + "\n");
    output.push_str(&format!("SUMMARY: Found {} atomic primes with center 5\n", found_primes.len()));
    output.push_str(&"=" .repeat(60) + "\n\n");
    println!("\n" + &"=" .repeat(60));
    println!("⚛️  ATOMIC PRIME SUMMARY");
    println!("=" .repeat(60));
    println!("Total patterns tested: {}", known_patterns.len() + 32);
    println!("Atomic primes found: {}", found_primes.len());
    // Group by type
    let single_count = found_primes.iter().filter(|(_, d)| d.starts_with("Single")).count();
    let double_count = found_primes.iter().filter(|(_, d)| d.starts_with("Double")).count();
    let triple_count = found_primes.iter().filter(|(_, d)| d.starts_with("Triple")).count();
    let special_count = found_primes.len() - single_count - double_count - triple_count;
    println!("\nBy type:");
    println!("  Single membrane: {}", single_count);
    println!("  Double membrane: {}", double_count);
    println!("  Triple membrane: {}", triple_count);
    println!("  Special patterns: {}", special_count);
    // Show all found primes
    println!("\n🌟 ALL ATOMIC PRIMES FOUND:");
    for (value, description) in &found_primes {
        println!("\n  {}", description);
        println!("  → {}", value);
    // Save to file
    let filename = "atomic_primes_collection.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .expect("Failed to open file");
    file.write_all(output.as_bytes()).expect("Failed to write to file");
    println!("\n✅ Results appended to: {}", filename);
    // Create a JSON summary too
    let json_filename = format!("atomic_primes_summary_{}.json", Local::now().format("%Y%m%d_%H%M%S"));
    let json_data = serde_json::json!({
        "search_time": Local::now().to_rfc3339(),
        "center_value": 5,
        "total_found": found_primes.len(),
        "by_type": {
            "single": single_count,
            "double": double_count,
            "triple": triple_count,
            "special": special_count
        },
        "primes": found_primes.iter().map(|(value, desc)| {
            serde_json::json!({
                "value": value,
                "description": desc,
                "digits": value.len(),
                "verification_url": format!("https://www.wolframalpha.com/input/?i=isprime({})", value)
            })
        }).collect::<Vec<_>>()
    });
    let mut json_file = File::create(&json_filename).expect("Failed to create JSON file");
    json_file.write_all(serde_json::to_string_pretty(&json_data).unwrap().as_bytes())
        .expect("Failed to write JSON");
    println!("📊 JSON summary saved to: {}", json_filename);
    // Highlight the most "atom-like" ones
    println!("\n⚛️  Most Atom-Like Structures:");
    if found_primes.iter().any(|(v, _)| v == "307050703") {
        println!("\n  ★ (3)─(7)──(5)──(7)─(3) → 307050703");
        println!("    This is the famous exclusive configuration!");
    if found_primes.iter().any(|(v, _)| v == "30705073") {
        println!("\n  ★ (3)─(7)─(5)─(7)─(3) → 30705073");
        println!("    Compact double membrane");
}
