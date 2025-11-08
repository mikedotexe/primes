//! Verify Lagrange points for zero-padded membrane prime

use prime_physics_engine::is_prime;
use num_bigint::BigUint;
use std::str::FromStr;

fn main() {
    let p1 = "10301";  // 1-0-3-0-1 symmetric zero-padded membrane
    let p2 = "30305070305070303";  // Giant membrane
    
    println!("Testing small zero-padded membrane vs giant membrane:");
    println!("Small: {} (structure: 1-0-3-0-1)", p1);
    println!("Large: {} (structure: 3-03-05-07-03-05-07-03-03)", p2);
    println!("Size ratio: 1:{}", p2.len() / p1.len());
    println!();
    
    // Test different buffer sizes
    for buffer_size in [3, 5, 7] {
        println!("\nBuffer size: {} zeros", buffer_size);
        let mut found = false;
        
        for pos in 0..buffer_size {
            for digit in 1..=9 {
                let mut buffer = vec!['0'; buffer_size];
                buffer[pos] = char::from_digit(digit, 10).unwrap();
                let buffer_str: String = buffer.into_iter().collect();
                
                let full = format!("{}{}{}", p1, buffer_str, p2);
                let num = BigUint::from_str(&full).unwrap();
                
                if is_prime(&num) {
                    if !found {
                        println!("✓ Lagrange points found!");
                        found = true;
                    }
                    println!("  Position {}, digit {}: {} ({} digits)",
                        pos, digit, full, full.len());
                }
            }
        }
        
        if !found {
            println!("  No Lagrange points with this buffer size");
        }
    }
}