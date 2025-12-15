//! Meatball Sandwich Prime Finder
//!
//! Extends Matt Parker's "zero sandwiches" (1[zeros]1) with a center digit.
//! Structure: 1 [k zeros] MEATBALL [k zeros] 1
//!
//! Matt's zero sandwiches are constrained: 10^n + 1 can only be prime
//! when n is a power of 2 (Fermat-like). The meatball breaks this!

use num_bigint::BigUint;
use primes::is_prime;

fn main() {
    println!("🥪 MEATBALL SANDWICH PRIMES");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Structure: 1[k zeros]M[k zeros]1");
    println!("Matt Parker's zero sandwich 10^n+1 requires n = 2^k for primality.");
    println!("Adding a meatball M breaks the algebraic constraint!\n");

    let mut results: Vec<(usize, u8, String)> = vec![];

    for k in 0..=20 {
        for meatball in 1u8..=9 {
            let sandwich = format!(
                "1{}{}{}1",
                "0".repeat(k),
                meatball,
                "0".repeat(k)
            );

            let num: BigUint = sandwich.parse().unwrap();
            if is_prime(&num) {
                results.push((k, meatball, sandwich.clone()));
                println!("k={:2} M={}: {} ✓ PRIME", k, meatball, sandwich);
            }
        }
    }

    // Summary for YouTube comment
    println!("\n━━━ YOUTUBE COMMENT FORMAT ━━━\n");
    print_youtube_summary(&results);

    // Comparison with Matt's zero sandwiches
    println!("\n━━━ COMPARISON ━━━\n");
    print_comparison();
}

fn print_youtube_summary(results: &[(usize, u8, String)]) {
    println!("I love thinking about this. Now I'm going to be annoying and talk");
    println!("about my journey since February, where I might be chasing Sandwich");
    println!("Prime Numbers where there's a central meatball, and equidistant");
    println!("space (0, 00, 000) between lettuce and tomato. Filling my sandwich:\n");

    println!("🥪 1[k zeros]M[k zeros]1\n");

    for k in 0..=20 {
        let primes: Vec<_> = results
            .iter()
            .filter(|(kk, _, _)| *kk == k)
            .map(|(_, m, _)| m.to_string())
            .collect();

        if !primes.is_empty() {
            println!("k={:2}: M∈{{{}}}", k, primes.join(","));
        } else {
            println!("k={:2}: (none)", k);
        }
    }

    println!("\nTotal: {} meatball sandwich primes found!", results.len());
    println!("\nYour 10^n+1 is constrained by algebraic factorization.");
    println!("The meatball breaks it! 🍖");
}

fn print_comparison() {
    println!("Matt's Zero Sandwiches (10^n + 1):");
    println!("  n=1: 11 ✓ prime (trivial)");
    println!("  n=2: 101 ✓ prime");
    println!("  n=3: 1001 = 7×11×13 ✗");
    println!("  n=4: 10001 = 73×137 ✗");
    println!("  n=5: 100001 = 11×9091 ✗");
    println!("  n=6: 1000001 = 101×9901 ✗");
    println!("  n=7: 10000001 = 11×909091 ✗");
    println!("  n=8: 100000001 = 17×5882353 ✗ (the video's example!)");
    println!();
    println!("Only 10^(2^k) + 1 CAN be prime (generalized Fermat in base 10).");
    println!("But with a meatball center, we escape this constraint!");
}
