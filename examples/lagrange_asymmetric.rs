//! Lagrange Points with Asymmetric Primes
//! 
//! Demonstrates Lagrange points between primes of very different sizes,
//! like a massive star and a small planet

use prime_physics_engine::is_prime;
use num_bigint::BigUint;
use std::str::FromStr;

fn main() {
    println!("🌌 LAGRANGE POINTS WITH ASYMMETRIC PRIMES");
    println!("{}", "=".repeat(80));
    println!();
    
    // Test different size ratios
    test_small_large_pair();
    test_tiny_giant_pair();
    test_membrane_pairs();
}

fn test_small_large_pair() {
    println!("📍 TEST 1: Small Prime + Large Prime (like Earth & Sun)");
    println!("{}", "-".repeat(80));
    println!();
    
    let small_prime = "97";           // 2-digit prime
    let large_prime = "3030507030703"; // 13-digit prime (membrane prime)
    
    println!("Small body: {} ({})", small_prime, prime_status(small_prime));
    println!("Large body: {} ({})", large_prime, prime_status(large_prime));
    println!("Size ratio: 1:{}", large_prime.len() / small_prime.len());
    println!();
    
    // Test different buffer sizes
    for buffer_size in [3, 5, 7, 9] {
        println!("\n🔭 Buffer size: {} zeros", buffer_size);
        println!("Full system: {}|{}|{}", small_prime, "0".repeat(buffer_size), large_prime);
        
        let mut found_any = false;
        
        for position in 0..buffer_size {
            for digit in 1..=9 {
                let mut buffer = vec!['0'; buffer_size];
                buffer[position] = char::from_digit(digit, 10).unwrap();
                let buffer_str: String = buffer.into_iter().collect();
                
                let full = format!("{}{}{}", small_prime, buffer_str, large_prime);
                let num = BigUint::from_str(&full).unwrap();
                
                if is_prime(&num) {
                    if !found_any {
                        println!("  ✓ Lagrange points found!");
                        found_any = true;
                    }
                    
                    // Visualize the discovery
                    let visual = create_visual(small_prime, &buffer_str, large_prime, position);
                    println!("    L{}: pos {}, digit {} → {}", 
                        position + 1, position, digit, visual);
                    println!("    Full: {} ({} digits) → PRIME!", full, full.len());
                }
            }
        }
        
        if !found_any {
            println!("  ✗ No Lagrange points with this buffer size");
        }
    }
}

fn test_tiny_giant_pair() {
    println!("\n\n📍 TEST 2: Tiny Prime + Giant Prime (like asteroid & Jupiter)");
    println!("{}", "-".repeat(80));
    println!();
    
    let tiny = "11";                      // Twin prime
    let giant = "30305070305070303";      // 17-digit membrane prime
    
    println!("Tiny body:  {} ({})", tiny, prime_status(tiny));
    println!("Giant body: {} ({})", giant, prime_status(giant));
    println!("Size ratio: 1:{}", giant.len() / tiny.len());
    println!();
    
    // Focus on one buffer size for clarity
    let buffer_size = 7;
    println!("Testing with {}-zero buffer:", buffer_size);
    println!();
    
    let mut lagrange_points = Vec::new();
    
    for position in 0..buffer_size {
        for digit in 1..=9 {
            let mut buffer = vec!['0'; buffer_size];
            buffer[position] = char::from_digit(digit, 10).unwrap();
            let buffer_str: String = buffer.into_iter().collect();
            
            let full = format!("{}{}{}", tiny, buffer_str, giant);
            let num = BigUint::from_str(&full).unwrap();
            
            if is_prime(&num) {
                lagrange_points.push((position, digit));
            }
        }
    }
    
    if lagrange_points.is_empty() {
        println!("No Lagrange points found");
    } else {
        println!("Found {} Lagrange points:", lagrange_points.len());
        for (pos, digit) in &lagrange_points {
            let mut buffer = vec!['0'; buffer_size];
            buffer[*pos] = char::from_digit(*digit, 10).unwrap();
            let buffer_str: String = buffer.into_iter().collect();
            
            println!("\n  Position {}, Digit {}:", pos, digit);
            println!("  Visual: {}", create_detailed_visual(tiny, &buffer_str, giant, *pos));
            
            let full = format!("{}{}{}", tiny, buffer_str, giant);
            println!("  Result: {} → PRIME! ✓", full);
            println!("  Total: {} digits", full.len());
        }
    }
}

fn test_membrane_pairs() {
    println!("\n\n📍 TEST 3: Different Sized Membrane Primes");
    println!("{}", "-".repeat(80));
    println!();
    
    // Use membrane primes of different sizes
    let pairs = vec![
        ("151", "303050303", "Small vs Medium membrane"),
        ("303050303", "30305070305070303", "Medium vs Large membrane"),
        ("3305033", "330500305003305003033", "Breathing patterns"),
    ];
    
    for (p1, p2, description) in pairs {
        println!("\n{}: {} (len {}) vs {} (len {})", 
            description, p1, p1.len(), p2, p2.len());
        
        // Quick scan with 5-zero buffer
        let buffer_size = 5;
        let mut found_count = 0;
        
        for position in 0..buffer_size {
            for digit in 1..=9 {
                let mut buffer = vec!['0'; buffer_size];
                buffer[position] = char::from_digit(digit, 10).unwrap();
                let buffer_str: String = buffer.into_iter().collect();
                
                let full = format!("{}{}{}", p1, buffer_str, p2);
                if let Ok(num) = BigUint::from_str(&full) {
                    if is_prime(&num) {
                        found_count += 1;
                        if found_count == 1 {
                            println!("  Example L-point: pos {}, digit {} → {}-digit PRIME",
                                position, digit, full.len());
                        }
                    }
                }
            }
        }
        
        if found_count > 0 {
            println!("  Total: {} Lagrange points found", found_count);
        } else {
            println!("  No Lagrange points with 5-zero buffer");
        }
    }
}

// Helper functions

fn prime_status(n: &str) -> &'static str {
    if let Ok(num) = BigUint::from_str(n) {
        if is_prime(&num) {
            "✓ prime"
        } else {
            "✗ not prime"
        }
    } else {
        "? invalid"
    }
}

fn create_visual(p1: &str, buffer: &str, p2: &str, highlight_pos: usize) -> String {
    let mut visual = String::new();
    
    // Small prime
    visual.push_str(p1);
    visual.push('|');
    
    // Buffer with highlight
    for (i, c) in buffer.chars().enumerate() {
        if i == highlight_pos && c != '0' {
            visual.push('[');
            visual.push(c);
            visual.push(']');
        } else {
            visual.push(c);
        }
    }
    
    visual.push('|');
    // Large prime (truncate if too long for display)
    if p2.len() > 10 {
        visual.push_str(&p2[..6]);
        visual.push_str("...");
        visual.push_str(&p2[p2.len()-3..]);
    } else {
        visual.push_str(p2);
    }
    
    visual
}

fn create_detailed_visual(p1: &str, buffer: &str, p2: &str, pos: usize) -> String {
    let mut lines = Vec::new();
    
    // Top line: labels
    lines.push(format!("  tiny    space    giant"));
    
    // Middle line: the system
    let mut middle = String::from("  ");
    middle.push_str(p1);
    middle.push_str("  ");
    
    for (i, c) in buffer.chars().enumerate() {
        if i == pos && c != '0' {
            middle.push('[');
            middle.push(c);
            middle.push(']');
        } else {
            middle.push(c);
        }
    }
    
    middle.push_str("  ");
    if p2.len() > 15 {
        middle.push_str(&p2[..8]);
        middle.push_str("...");
        middle.push_str(&p2[p2.len()-4..]);
    } else {
        middle.push_str(p2);
    }
    
    lines.push(middle);
    
    // Bottom line: pointer
    let mut pointer = String::from("  ");
    pointer.push_str(&" ".repeat(p1.len() + 2 + pos));
    pointer.push('↑');
    lines.push(pointer);
    lines.push(format!("  {}L-point", " ".repeat(p1.len() + 2 + pos - 2)));
    
    lines.join("\n")
}