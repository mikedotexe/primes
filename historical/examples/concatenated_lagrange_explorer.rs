//! Explore Lagrange points in the space BETWEEN concatenated primes
//!
//! Like Earth and Moon separated by space, we concatenate two primes
//! with a buffer of zeros between them and look for special properties.

use num_bigint::BigUint;
use primes::is_prime;
use std::str::FromStr;

fn main() {
    println!("🌌 Concatenated Prime Lagrange Explorer");
    println!("{}", "=".repeat(80));
    println!();
    println!("Concept: Two primes separated by a 'space' of zeros");
    println!("The entire concatenated number may have special properties");
    println!();

    // Test 1: Basic concatenation
    println!("Test 1: Simple Concatenation");
    println!("{}", "-".repeat(80));

    let prime1 = BigUint::from(303050303u64); // "Earth"
    let prime2 = BigUint::from(303070303u64); // "Moon"

    // Try different buffer sizes
    for buffer_size in [0, 1, 3, 5, 7, 9, 11] {
        let zeros = "0".repeat(buffer_size);
        let concatenated_str = format!("{}{}{}", prime1, zeros, prime2);
        let concatenated = BigUint::from_str(&concatenated_str).unwrap();

        println!(
            "\nBuffer size {}: {}",
            buffer_size,
            visualize_concatenation(&prime1.to_string(), &zeros, &prime2.to_string())
        );
        println!("Full number: {}", concatenated_str);
        println!(
            "Is prime: {}",
            if is_prime(&concatenated) {
                "✓ YES!"
            } else {
                "✗ No"
            }
        );

        if !is_prime(&concatenated) && buffer_size > 0 {
            // Check if changing any zero to another digit makes it prime
            check_lagrange_points(&prime1.to_string(), buffer_size, &prime2.to_string());
        }
    }

    // Test 2: Symmetric buffer exploration
    println!("\n\nTest 2: Lagrange Point Search in Buffer");
    println!("{}", "-".repeat(80));

    // For a specific buffer size, try different configurations
    let buffer_size = 7; // Lucky 7
    println!("Fixed buffer size: {}", buffer_size);
    println!("Testing different 'masses' at positions in the buffer\n");

    for position in 0..buffer_size {
        for digit in 1..=9 {
            let mut buffer = vec!['0'; buffer_size];
            buffer[position] = char::from_digit(digit, 10).unwrap();
            let buffer_str: String = buffer.into_iter().collect();

            let concatenated_str = format!("{}{}{}", prime1, buffer_str, prime2);
            let concatenated = BigUint::from_str(&concatenated_str).unwrap();

            if is_prime(&concatenated) {
                println!(
                    "🌟 FOUND! Position {}, digit {}: {}",
                    position,
                    digit,
                    visualize_with_highlight(
                        &prime1.to_string(),
                        &buffer_str,
                        &prime2.to_string(),
                        position
                    )
                );
                println!("   Full: {}", concatenated_str);
            }
        }
    }

    // Test 3: Multiple masses in buffer
    println!("\n\nTest 3: Multiple Lagrange Points");
    println!("{}", "-".repeat(80));

    // Try patterns like 0010100 - multiple "masses" in the buffer
    let patterns = vec![
        "0010100", // Two symmetric points
        "0001000", // Single central point
        "1000001", // Two edge points
        "0101010", // Alternating pattern
        "0003000", // Different mass at center
    ];

    for pattern in patterns {
        let concatenated_str = format!("{}{}{}", prime1, pattern, prime2);
        let concatenated = BigUint::from_str(&concatenated_str).unwrap();

        println!(
            "\nPattern: {}",
            visualize_concatenation(&prime1.to_string(), pattern, &prime2.to_string())
        );
        println!(
            "Is prime: {}",
            if is_prime(&concatenated) {
                "✓ YES!"
            } else {
                "✗ No"
            }
        );
    }

    // Test 4: Physical interpretation
    println!("\n\n🔬 Physical Interpretation");
    println!("{}", "-".repeat(80));
    println!("In our gravitational model:");
    println!("- The two membrane primes are like massive bodies (Earth & Moon)");
    println!("- The zeros between them represent empty space");
    println!("- Lagrange points are positions where a 'test mass' (non-zero digit) can exist");
    println!("- When the entire system forms a prime, we have gravitational harmony!");
}

fn visualize_concatenation(p1: &str, buffer: &str, p2: &str) -> String {
    format!(
        "{} [{}] {}",
        p1.chars()
            .map(|c| if c == '0' { '◯' } else { c })
            .collect::<String>(),
        buffer
            .chars()
            .map(|c| if c == '0' { '◯' } else { c })
            .collect::<String>(),
        p2.chars()
            .map(|c| if c == '0' { '◯' } else { c })
            .collect::<String>()
    )
}

fn visualize_with_highlight(p1: &str, buffer: &str, p2: &str, pos: usize) -> String {
    let mut buffer_visual = String::new();
    for (i, c) in buffer.chars().enumerate() {
        if i == pos {
            buffer_visual.push_str(&format!("【{}】", c));
        } else {
            buffer_visual.push(if c == '0' { '◯' } else { c });
        }
    }

    format!(
        "{} [{}] {}",
        p1.chars()
            .map(|c| if c == '0' { '◯' } else { c })
            .collect::<String>(),
        buffer_visual,
        p2.chars()
            .map(|c| if c == '0' { '◯' } else { c })
            .collect::<String>()
    )
}

fn check_lagrange_points(p1: &str, buffer_size: usize, p2: &str) {
    let mut found_any = false;

    for position in 0..buffer_size {
        for digit in 1..=9 {
            let mut buffer = vec!['0'; buffer_size];
            buffer[position] = char::from_digit(digit, 10).unwrap();
            let buffer_str: String = buffer.into_iter().collect();

            let concatenated_str = format!("{}{}{}", p1, buffer_str, p2);
            let concatenated = BigUint::from_str(&concatenated_str).unwrap();

            if is_prime(&concatenated) {
                if !found_any {
                    println!("  Lagrange points found:");
                    found_any = true;
                }
                println!(
                    "    L{}: position {}, mass {}",
                    position + 1,
                    position,
                    digit
                );
            }
        }
    }

    if !found_any {
        println!("  No single-digit Lagrange points found in this configuration");
    }
}
