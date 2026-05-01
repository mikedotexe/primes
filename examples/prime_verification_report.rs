//! Generate a verification report for maintained prime anchors and audit cases
//!
//! Lists common membrane prime anchors, verifies each one, and keeps known
//! composite patterns in a separate audit section.
//!
//! # Purpose
//!
//! Independently verifies maintained prime anchors. Known composite examples
//! stay visible as doc-audit guardrails instead of being counted as failed
//! prime claims.
//!
//! # Expected Output
//!
//! ```text
//! === Membrane Prime Toolkit - Verification Report ===
//!
//! Small membrane primes:
//! 1-5-1                                    151                 ✅ PRIME
//! 1-0-3-0-1                                10301               ✅ PRIME
//! 3-0-7-0-3                                30703               ✅ PRIME
//! 1-00-3-00-1                              1003001             ✅ PRIME
//!
//! Larger membrane structures:
//! 3-03-05-03-03                            303050303           ✅ PRIME
//! 33-05-033                                3305033             ✅ PRIME
//!
//! Lagrange point examples:
//! 10301 + 8 at pos 0 + [...]              [25-digit number]   ✅ PRIME
//! ```
//!
//! # Key Features
//!
//! **Independent Verification**: Uses Miller-Rabin with 20 rounds (>99.99% confidence)
//!
//! **Factor Finding**: Shows small factors for composite numbers
//!
//! **Documentation Audit**: Identifies any false claims in documentation
//!
//! # Runtime
//!
//! Approximately 1 minute.
//!
//! # Success Indicator
//!
//! All maintained anchors show ✅ PRIME and all audit cases show ✅ COMPOSITE.

use num_bigint::BigUint;
use primes::is_prime;
use std::str::FromStr;

fn is_prime_decimal(num_str: &str) -> bool {
    let num = BigUint::from_str(num_str).unwrap();
    is_prime(&num)
}

fn print_prime_anchor(num_str: &str, description: &str) -> bool {
    print!("{:<40} {:<30}  ", description, num_str);

    if is_prime_decimal(num_str) {
        println!("✅ PRIME");
        true
    } else {
        println!("❌ UNEXPECTED COMPOSITE ({})", small_factor_label(num_str));
        false
    }
}

fn print_composite_audit(num_str: &str, description: &str) -> bool {
    print!("{:<40} {:<30}  ", description, num_str);

    if is_prime_decimal(num_str) {
        println!("❌ UNEXPECTED PRIME");
        false
    } else {
        println!("✅ COMPOSITE ({})", small_factor_label(num_str));
        true
    }
}

fn small_factor_label(num_str: &str) -> String {
    let num = BigUint::from_str(num_str).unwrap();
    let mut factors = Vec::new();

    for p in &[2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47] {
        if &num % p == 0u32.into() && num != (*p).into() {
            factors.push(*p);
        }
    }

    if factors.is_empty() {
        "no small factor <= 47".to_string()
    } else {
        format!("divisible by {:?}", factors)
    }
}

fn main() {
    println!("=== Membrane Prime Toolkit - Verification Report ===\n");

    let mut anchors_ok = true;
    let mut audit_ok = true;

    println!("Maintained prime anchors:\n");

    // Small verified primes
    println!("Small membrane primes:");
    anchors_ok &= print_prime_anchor("151", "1-5-1");
    anchors_ok &= print_prime_anchor("10301", "1-0-3-0-1");
    anchors_ok &= print_prime_anchor("30703", "3-0-7-0-3");
    anchors_ok &= print_prime_anchor("1003001", "1-00-3-00-1");

    println!("\nLarger membrane structures:");
    anchors_ok &= print_prime_anchor("303050303", "3-03-05-03-03");
    anchors_ok &= print_prime_anchor("3305033", "33-05-033");
    anchors_ok &= print_prime_anchor("300700303", "3-00-7-00-3-03");
    anchors_ok &= print_prime_anchor("2551", "15451 (base 6)");

    // Lagrange point examples
    println!("\nLagrange point examples:");
    anchors_ok &= print_prime_anchor(
        "103018000030305070305070303",
        "10301 + 8 at pos 0 + 30305070305070303",
    );

    println!("\nKnown composite doc-audit cases:");
    audit_ok &= print_composite_audit("15651", "1-5-6-5-1 (decimal)");
    audit_ok &= print_composite_audit("300700300703", "3-007-003-007-03");
    audit_ok &= print_composite_audit("30070030703", "3-00-7-00-3-07-03");
    audit_ok &= print_composite_audit("37373", "(3,7) k=(0,0) seed=3");
    audit_ok &= print_composite_audit("3070703", "(3,7) k=(1,0) seed=3");
    audit_ok &= print_composite_audit("3703073", "(3,7) k=(0,1) seed=3");
    audit_ok &= print_composite_audit("300703070003", "(3,7) k=(2,1) seed=3");
    audit_ok &= print_composite_audit("9700005303050303", "97 + 5 zeros + 303050303");

    println!("\n{}", "=".repeat(50));
    if anchors_ok && audit_ok {
        println!("✅ Maintained prime anchors and composite audit cases verified successfully!");
    } else {
        println!("⚠️  Verification drift detected!");
        println!("Action needed: inspect anchor/audit sections above and update docs or examples.");
    }

    // Additional investigation
    println!("\n{}", "=".repeat(50));
    println!("Investigation: Why might 300700300703 appear in docs?\n");

    println!("It could be a typo or formatting error of valid patterns:");
    println!("- 30703 (valid prime)");
    println!("- 3007003 (checking...)");
    print_prime_anchor("3007003", "3-00-7-00-3");
    println!("- 30070030703 (checking...)");
    print_composite_audit("30070030703", "3-00-7-00-3-07-03");

    println!("\nRecommendation: Search docs for '3007' or '30070' patterns");
}
