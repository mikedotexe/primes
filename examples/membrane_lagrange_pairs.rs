//! Lagrange Points Between Different-Sized Membrane Primes
//! 
//! Shows Lagrange points when BOTH bodies are membrane primes with symmetric zero-padding,
//! but of very different sizes

use prime_physics_engine::is_prime;
use num_bigint::BigUint;
use std::str::FromStr;

fn main() {
    println!("🌌 LAGRANGE POINTS: SMALL MEMBRANE vs GIANT MEMBRANE");
    println!("{}", "=".repeat(80));
    println!();
    
    test_membrane_pairs();
}

fn test_membrane_pairs() {
    println!("Both bodies are membrane primes with symmetric zero-padding!");
    println!("This shows the gravitational interaction between structured primes.\n");
    
    // Different membrane prime pairs
    let membrane_pairs = vec![
        (
            "151",  // 1-5-1 minimal membrane
            "30305070305070303",  // 3-03-05-07-03-05-07-03-03 complex membrane
            "Tiny membrane vs Giant membrane (1:6 ratio)"
        ),
        (
            "30503",  // 3-05-03 small symmetric
            "303050303",  // 3-03-05-03-03 medium symmetric
            "Small vs Medium membrane (1:2 ratio)"
        ),
        (
            "3305033",  // 33-05-033 breathing pattern
            "30305070305070303",  // giant membrane
            "Breathing vs Giant (1:2.5 ratio)"
        ),
        (
            "1510151",  // 151-0-151 symmetric with zero
            "303050703050703",  // complex alternating membrane
            "Symmetric padded vs Alternating (1:2 ratio)"
        ),
    ];
    
    for (small, large, description) in membrane_pairs {
        println!("\n{}", "=".repeat(70));
        println!("📍 {}", description);
        println!("{}", "=".repeat(70));
        
        // Show structure breakdown
        println!("\nSmall membrane: {}", small);
        println!("Structure: {}", visualize_membrane(small));
        println!("Is prime: {}", if is_prime(&BigUint::from_str(small).unwrap()) { "✓ YES" } else { "✗ NO" });
        
        println!("\nLarge membrane: {}", large);
        println!("Structure: {}", visualize_membrane(large));
        println!("Is prime: {}", if is_prime(&BigUint::from_str(large).unwrap()) { "✓ YES" } else { "✗ NO" });
        
        println!("\nSize comparison: {} digits vs {} digits (1:{})", 
            small.len(), large.len(), large.len() / small.len());
        
        // Test with different buffer sizes
        println!("\nSearching for Lagrange points...");
        
        for buffer_size in [3, 5, 7] {
            println!("\n🔭 Buffer size: {} zeros", buffer_size);
            let mut found_points = Vec::new();
            
            for position in 0..buffer_size {
                for digit in 1..=9 {
                    let mut buffer = vec!['0'; buffer_size];
                    buffer[position] = char::from_digit(digit, 10).unwrap();
                    let buffer_str: String = buffer.into_iter().collect();
                    
                    let full = format!("{}{}{}", small, buffer_str, large);
                    if let Ok(num) = BigUint::from_str(&full) {
                        if is_prime(&num) {
                            found_points.push((position, digit, full.len()));
                        }
                    }
                }
            }
            
            if found_points.is_empty() {
                println!("  No Lagrange points found");
            } else {
                println!("  ✓ Found {} Lagrange points!", found_points.len());
                for (pos, digit, total_len) in found_points.iter().take(3) {
                    let mut buffer = vec!['0'; buffer_size];
                    buffer[*pos] = char::from_digit(*digit, 10).unwrap();
                    let buffer_str: String = buffer.into_iter().collect();
                    
                    println!("    Position {}, digit {}: {} | {} | {}",
                        pos, digit,
                        small,
                        highlight_buffer(&buffer_str, *pos),
                        if large.len() > 10 { 
                            format!("{}...{}", &large[..6], &large[large.len()-4..])
                        } else {
                            large.to_string()
                        }
                    );
                    println!("    → Creates {}-digit PRIME!", total_len);
                }
            }
        }
    }
    
    println!("\n\n🔑 KEY INSIGHTS:");
    println!("• Both bodies can be membrane primes with internal structure");
    println!("• The symmetric zero-padding creates 'gravitational fields'");
    println!("• Lagrange points exist in the space between structured primes");
    println!("• Size asymmetry enhances the two-body dynamics");
}

fn visualize_membrane(s: &str) -> String {
    // Simple visualization showing zero patterns
    s.chars().map(|c| {
        if c == '0' {
            '◯'
        } else {
            c
        }
    }).collect::<String>()
}

fn highlight_buffer(buffer: &str, pos: usize) -> String {
    buffer.chars().enumerate().map(|(i, c)| {
        if i == pos && c != '0' {
            format!("[{}]", c)
        } else {
            c.to_string()
        }
    }).collect::<Vec<_>>().join("")
}