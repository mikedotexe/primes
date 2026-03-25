//! Membrane Prime Showcase
//!
//! A beautiful demonstration of membrane prime patterns across different bases
//! Shows the symmetric zero-padded structure and statistical success

use num_bigint::BigUint;
use primes::{is_prime, MembraneConfig};

fn main() {
    println!("✨ Membrane Prime Showcase");
    println!("{}", "═".repeat(80));
    println!();

    // Part 1: Visual Structure Demo
    visual_structure_demo();

    // Part 2: Statistical Patterns
    statistical_patterns();

    // Part 3: Palindrome overlap vs distinction
    palindrome_overlap();

    // Part 4: Lagrange Points
    lagrange_showcase();

    // Part 5: Current Insights
    universal_insights();
}

fn visual_structure_demo() {
    println!("📐 PART 1: MEMBRANE STRUCTURE");
    println!("{}", "─".repeat(80));
    println!();

    println!("Membrane primes have the structure:");
    println!("outer + (k_outer zeros) + inner + (k_inner zeros) + SEED + (k_inner zeros) + inner + (k_outer zeros) + outer");
    println!();

    // Show different patterns
    let patterns = vec![
        ("Classic", MembraneConfig::new(10, 3, 7, 0, 0), 5, "37573"),
        (
            "Breathing",
            MembraneConfig::new(10, 3, 3, 0, 1),
            5,
            "3305033",
        ),
        (
            "Symmetric",
            MembraneConfig::new(10, 3, 3, 1, 1),
            5,
            "303050303",
        ),
        (
            "Exclusive",
            MembraneConfig::new(10, 3, 7, 1, 1),
            5,
            "307050703",
        ),
    ];

    for (name, config, seed, expected) in patterns {
        let membrane = build_membrane_string(&config, seed);
        let visual = visualize_membrane(&membrane);
        let decimal = BigUint::parse_bytes(membrane.as_bytes(), 10).unwrap();
        let is_p = is_prime(&decimal);

        println!("{} Pattern:", name);
        println!(
            "  Config: ({},{}) k=({},{}) seed={}",
            config.outer, config.inner, config.k_outer, config.k_inner, seed
        );
        println!("  String: {}", membrane);
        println!("  Visual: {}", visual);
        println!(
            "  Decimal: {} {}",
            decimal,
            if is_p { "✓ PRIME" } else { "✗" }
        );

        if membrane != expected {
            println!("  ⚠️  Expected: {}", expected);
        }
        println!();
    }
}

fn statistical_patterns() {
    println!("\n📊 PART 2: STATISTICAL PATTERNS");
    println!("{}", "─".repeat(80));
    println!();

    // Test configurations across bases
    let test_configs = vec![
        ("Base 6 Elite", 6, 1, 1, 0, 0),
        ("Base 10 Breathing", 10, 3, 3, 0, 1),
        ("Base 12 Champion", 12, 1, 1, 0, 0),
        ("Base 30 Giant", 30, 11, 19, 0, 0),
    ];

    for (name, base, outer, inner, k_outer, k_inner) in test_configs {
        let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);
        let mut found = 0;
        let mut examples = Vec::new();

        let test_count = base.min(20);
        for seed in 0..test_count {
            let membrane = build_membrane_string(&config, seed);
            if let Some(decimal) = BigUint::parse_bytes(membrane.as_bytes(), base) {
                if is_prime(&decimal) {
                    found += 1;
                    if examples.len() < 3 {
                        examples.push((seed, membrane, decimal));
                    }
                }
            }
        }

        println!(
            "{} - ({},{}) k=({},{}):",
            name, outer, inner, k_outer, k_inner
        );
        println!(
            "  Success rate: {}/{} = {:.1}%",
            found,
            test_count,
            found as f64 / test_count as f64 * 100.0
        );

        for (seed, membrane, decimal) in examples {
            println!(
                "  Seed {:2}: {} (base {}) = {}",
                seed, membrane, base, decimal
            );
        }
        println!();
    }
}

fn palindrome_overlap() {
    println!("\n🪞 PART 3: PALINDROME OVERLAP vs MEMBRANE DISTINCTION");
    println!("{}", "─".repeat(80));
    println!();

    println!("Some membrane examples are also palindromes, especially with a");
    println!("single central digit and fully symmetric padding.");
    println!("But the construction family is broader than palindromes.");
    println!();

    let examples = vec![
        ("Single-digit symmetric", "307050703"),
        ("Breathing example", "3305033"),
        ("Multi-digit membrane", "30301303"),
        ("Another multi-digit membrane", "10409401"),
    ];

    for (label, value) in examples {
        let reversed: String = value.chars().rev().collect();
        let palindrome = value == reversed;
        println!(
            "{:28} {}  {}",
            label,
            value,
            if palindrome {
                "(also a palindrome)"
            } else {
                "(not a palindrome)"
            }
        );
    }

    println!();
    println!("Takeaway:");
    println!("  - palindrome symmetry explains some headline examples");
    println!("  - membrane structure still includes non-palindromic candidates");
    println!("  - the more interesting question is which structural constraints");
    println!("    matter after controlling for coprimality and length");
}

fn lagrange_showcase() {
    println!("\n🌌 PART 4: LAGRANGE POINTS");
    println!("{}", "─".repeat(80));
    println!();

    println!("When two membrane primes are concatenated with a 'space' between them,");
    println!("special positions (Lagrange points) can hold non-zero digits to create primes!");
    println!();

    let prime1 = BigUint::from(303050303u64);
    let prime2 = BigUint::from(303070303u64);

    println!(
        "Prime 1 (Earth): {} = {}",
        prime1,
        visualize_number(&prime1.to_string())
    );
    println!(
        "Prime 2 (Moon):  {} = {}",
        prime2,
        visualize_number(&prime2.to_string())
    );
    println!();

    // Test a 7-zero buffer with known Lagrange points
    let lagrange_configs = vec![
        (2, 5, "L3"), // Position 2, digit 5
        (4, 2, "L1"), // Position 4, digit 2
        (5, 5, "L5"), // Position 5, digit 5
    ];

    println!("7-zero buffer with Lagrange points:");
    for (pos, digit, name) in lagrange_configs {
        let mut buffer = vec!['0'; 7];
        buffer[pos] = char::from_digit(digit, 10).unwrap();
        let buffer_str: String = buffer.into_iter().collect();

        let concatenated_str = format!("{}{}{}", prime1, buffer_str, prime2);
        let concatenated = concatenated_str.parse::<BigUint>().unwrap();

        println!("\n{}: Position {}, digit {}", name, pos, digit);
        println!(
            "  Buffer: {}",
            buffer_str
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == pos {
                        format!("[{}]", c)
                    } else {
                        c.to_string()
                    }
                })
                .collect::<String>()
        );
        println!(
            "  Full: {} = {} digits",
            concatenated,
            concatenated_str.len()
        );
        println!(
            "  Prime: {}",
            if is_prime(&concatenated) {
                "✓ YES!"
            } else {
                "✗"
            }
        );
    }
}

fn universal_insights() {
    println!("\n\n🔑 PART 5: CURRENT INSIGHTS");
    println!("{}", "═".repeat(80));
    println!();

    println!("1. COPRIMALITY IS ESSENTIAL");
    println!("   All successful configurations use boundary digits coprime to the base");
    println!();

    println!("2. SHORT / COMPACT CONSTRUCTIONS OFTEN WIN");
    println!("   Minimal padding often performs well, especially once seed length grows");
    println!();

    println!("3. MEMBRANES OVERLAP WITH PALINDROMES, BUT ARE NOT REDUCIBLE TO THEM");
    println!("   Multi-digit and asymmetric examples extend beyond the palindrome subset");
    println!();

    println!("4. BASE CHOICE AND BOUNDARY DIGITS MATTER A LOT");
    println!("   High-performing families depend strongly on coprimality to the base");
    println!();

    println!("5. THE MAIN CONTROL QUESTION IS STILL IMPORTANT");
    println!("   Dramatic gains over naive random are real, but the extra gain over");
    println!("   random-coprime controls appears smaller and may be negligible");
    println!();

    println!("🎯 CONCLUSION:");
    println!("Membrane constructions are a useful candidate family with real high-density");
    println!("configurations. The strongest supported claim today is about structured");
    println!("coprime candidate generation, not a proven new prime mechanism.");
}

// Helper functions

fn build_membrane_string(config: &MembraneConfig, seed: u32) -> String {
    let outer = to_base_string(config.outer, config.base);
    let inner = to_base_string(config.inner, config.base);
    let seed_str = to_base_string(seed, config.base);

    format!(
        "{}{}{}{}{}{}{}{}{}",
        outer,
        "0".repeat(config.k_outer as usize),
        inner,
        "0".repeat(config.k_inner as usize),
        seed_str,
        "0".repeat(config.k_inner as usize),
        inner,
        "0".repeat(config.k_outer as usize),
        outer
    )
}

fn to_base_string(mut n: u32, base: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    while n > 0 {
        let digit = n % base;
        let ch = if digit < 10 {
            char::from_digit(digit, 10).unwrap()
        } else {
            char::from_u32('A' as u32 + digit - 10).unwrap()
        };
        result.insert(0, ch);
        n /= base;
    }
    result
}

fn visualize_membrane(s: &str) -> String {
    s.chars()
        .map(|c| if c == '0' { '◯' } else { c })
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("-")
}

fn visualize_number(s: &str) -> String {
    s.chars()
        .map(|c| if c == '0' { '◯' } else { c })
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("-")
}
