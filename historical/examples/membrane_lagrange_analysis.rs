//! Deep analysis of Lagrange points for membrane primes
//!
//! This explores the hypothesis that Lagrange points between symmetric
//! zero-padded primes might have special structural properties.

use num_bigint::BigUint;
use primes::is_prime;

fn main() {
    println!("🔬 Membrane Lagrange Point Deep Analysis");
    println!("{}", "=".repeat(60));
    println!();

    // Test 1: Same outer structure, different middle digits
    println!("Test 1: Primes with identical membrane structure");
    println!("{}", "-".repeat(60));

    let tests = vec![
        (303050303u64, 303070303u64, "Same structure, middle: 5 vs 7"),
        (303010303u64, 303050303u64, "Same structure, middle: 1 vs 5"),
        (
            303050303u64,
            303110303u64,
            "Same structure, middle: 5 vs 11",
        ),
    ];

    for (p1_val, p2_val, desc) in tests {
        let p1 = BigUint::from(p1_val);
        let p2 = BigUint::from(p2_val);

        println!("\n{}", desc);
        println!("P1: {} = {}", p1, visualize(&p1.to_string()));
        println!("P2: {} = {}", p2, visualize(&p2.to_string()));

        // Numeric midpoint
        let mid = (&p1 + &p2) / 2u8;
        println!(
            "Numeric midpoint: {} = {}",
            mid,
            visualize(&mid.to_string())
        );
        println!(
            "                  Prime: {}",
            if is_prime(&mid) { "✓" } else { "✗" }
        );

        // Analyze structure
        analyze_midpoint_structure(&p1, &p2, &mid);
    }

    // Test 2: The "all zeros" hypothesis
    println!("\n\nTest 2: All-Zero Middle Hypothesis");
    println!("{}", "-".repeat(60));

    let zero_candidates = vec![
        303000303u64, // All zeros in middle positions
        303000003u64, // Even more zeros
        300000003u64, // Maximum zeros
    ];

    for val in zero_candidates {
        let num = BigUint::from(val);
        println!("\nCandidate: {} = {}", num, visualize(&num.to_string()));
        println!(
            "           Prime: {}",
            if is_prime(&num) { "✓" } else { "✗" }
        );

        // Factor it
        if !is_prime(&num) {
            print_factors(&num);
        }
    }

    // Test 3: Structural interpolation
    println!("\n\nTest 3: Structural Lagrange Points");
    println!("{}", "-".repeat(60));

    // For two primes: 3-0-3-0-X-0-3-0-3 and 3-0-3-0-Y-0-3-0-3
    // What should the Lagrange point be?

    let p1 = BigUint::from(303050303u64); // X = 5
    let p2 = BigUint::from(303070303u64); // Y = 7

    println!("\nBetween {} and {}", p1, p2);
    println!("Structure: 3-0-3-0-X-0-3-0-3 where X ∈ {{5, 7}}");

    // Different interpolation methods
    let methods = vec![
        ("Numeric average", (&p1 + &p2) / 2u8),
        ("Zero middle", BigUint::from(303000303u64)),
        ("Pattern 3030[6]0303", BigUint::from(303060303u64)),
        ("Pattern 3030[0]0303", BigUint::from(303000303u64)),
    ];

    for (method, value) in methods {
        println!(
            "\n{}: {} = {}",
            method,
            value,
            visualize(&value.to_string())
        );
        println!(
            "{} Prime: {}",
            " ".repeat(method.len()),
            if is_prime(&value) { "✓" } else { "✗" }
        );
    }

    // Test 4: Larger membranes
    println!("\n\nTest 4: Larger Membrane Structures");
    println!("{}", "-".repeat(60));

    // These have more padding: 3-00-7-0-X-0-7-00-3
    let large1 = BigUint::from(30070050700703u64); // X = 5
    let large2 = BigUint::from(30070170700703u64); // X = 17

    println!("\nP1: {} = {}", large1, visualize(&large1.to_string()));
    println!("P2: {} = {}", large2, visualize(&large2.to_string()));

    let large_mid = (&large1 + &large2) / 2u8;
    println!(
        "\nMidpoint: {} = {}",
        large_mid,
        visualize(&large_mid.to_string())
    );

    // What if we zero out the middle section?
    let zero_middle = BigUint::from(30070000700703u64);
    println!(
        "All-zero: {} = {}",
        zero_middle,
        visualize(&zero_middle.to_string())
    );
    println!(
        "          Prime: {}",
        if is_prime(&zero_middle) { "✓" } else { "✗" }
    );

    // Key insight check
    println!("\n\n🎯 KEY INSIGHT");
    println!("{}", "=".repeat(60));
    println!("For membrane primes with structure: outer-pad-inner-pad-MIDDLE-pad-inner-pad-outer");
    println!("The Lagrange points may have special properties when MIDDLE → 0");
    println!("\nThis aligns with the gravitational interpretation:");
    println!("- The 'mass' is concentrated in the boundary digits");
    println!("- The middle is the variable 'field' region");
    println!("- At equilibrium (Lagrange points), the field could be zero!");
}

fn visualize(s: &str) -> String {
    s.chars()
        .map(|c| if c == '0' { '◯' } else { c })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("-")
}

fn analyze_midpoint_structure(p1: &BigUint, p2: &BigUint, mid: &BigUint) {
    let s1 = p1.to_string();
    let s2 = p2.to_string();
    let sm = mid.to_string();

    if s1.len() == s2.len() && s1.len() == sm.len() {
        // Find differing positions
        let mut diffs = Vec::new();
        for (i, (c1, c2)) in s1.chars().zip(s2.chars()).enumerate() {
            if c1 != c2 {
                diffs.push(i);
            }
        }

        if diffs.len() == 1 {
            let pos = diffs[0];
            let c1 = s1.chars().nth(pos).unwrap();
            let c2 = s2.chars().nth(pos).unwrap();
            let cm = sm.chars().nth(pos).unwrap();

            println!(
                "                  Structure preserved except position {}: {} ↔ {} → {}",
                pos, c1, c2, cm
            );

            // Check if it's the average
            if let (Some(d1), Some(d2), Some(dm)) =
                (c1.to_digit(10), c2.to_digit(10), cm.to_digit(10))
            {
                let avg = (d1 + d2) / 2;
                if dm == avg {
                    println!("                  ✓ Middle digit is arithmetic average");
                }
            }
        }
    }
}

fn print_factors(n: &BigUint) {
    let mut factors = Vec::new();
    let mut remaining = n.clone();
    let mut d = BigUint::from(2u32);

    while &d * &d <= remaining && factors.len() < 10 {
        while &remaining % &d == BigUint::from(0u32) {
            factors.push(d.clone());
            remaining = &remaining / &d;
        }
        d += 1u32;
    }

    if remaining > BigUint::from(1u32) {
        factors.push(remaining);
    }

    let factor_strs: Vec<String> = factors.iter().map(|f| f.to_string()).collect();
    println!("           Factors: {}", factor_strs.join(" × "));
}
