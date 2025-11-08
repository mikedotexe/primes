use num_bigint::BigUint;
use prime_physics_engine::is_prime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body1 = "7";
    let body2 = "11";
    let space_size = 100; // A resonant peak identified in Phase 1

    let mut both_prime = 0;
    let mut a_only = 0;
    let mut b_only = 0;
    let mut total_primes = 0;

    for digit in 1..=9 {
        for pos_a in 0..(space_size / 2) {
            let pos_b = space_size - 1 - pos_a;

            // Construct and test Number A
            let mut test_str_a = "0".repeat(space_size);
            unsafe { test_str_a.as_bytes_mut()[pos_a] = b'0' + digit as u8; }
            let full_number_a = format!("{}{}{}", body1, test_str_a, body2);
            let is_prime_a = full_number_a.parse::<BigUint>().map_or(false, |num| is_prime(&num));

            // Construct and test Number B
            let mut test_str_b = "0".repeat(space_size);
            unsafe { test_str_b.as_bytes_mut()[pos_b] = b'0' + digit as u8; }
            let full_number_b = format!("{}{}{}", body1, test_str_b, body2);
            let is_prime_b = full_number_b.parse::<BigUint>().map_or(false, |num| is_prime(&num));

            match (is_prime_a, is_prime_b) {
                (true, true) => {
                    // If pos_a is its own reflection (middle point in odd space_size), count it once.
                    if pos_a == pos_b { 
                        total_primes += 1; 
                    } else {
                        both_prime += 1;
                        total_primes += 2;
                    }
                },
                (true, false) => {
                    a_only += 1;
                    total_primes += 1;
                },
                (false, true) => {
                    b_only += 1;
                    total_primes += 1;
                },
                (false, false) => {},
            }
        }
    }

    let symmetric_pairs = both_prime;
    let asymmetric_primes = a_only + b_only;
    let total_prime_events = (symmetric_pairs * 2) + asymmetric_primes;

    // This factor measures: "Of all primes found, what percentage came in a symmetric pair?"
    let symmetry_strength = (symmetric_pairs as f64 * 2.0) / total_prime_events as f64 * 100.0;

    // This factor measures: "Of all prime *events* (a prime appearing), what percentage were part of a symmetric pair event?"
    // An "event" is finding a prime, a "symmetric pair event" is finding two primes at mirror positions.
    let correlation_factor = symmetric_pairs as f64 / (symmetric_pairs + asymmetric_primes) as f64;

    println!("--- Symmetry Verification Report ---");
    println!("Analysis for space_size = {}", space_size);
    println!("------------------------------------",);
    println!("Total Primes Found: {}", total_primes);
    println!("Symmetric Pairs (A and B prime): {}", symmetric_pairs);
    println!("Asymmetric Primes (A or B, but not both): {}", asymmetric_primes);
    println!("\n--- Analysis ---");
    println!("Symmetry Strength: {:.2}% of primes found were part of a symmetric pair.", symmetry_strength);
    println!("Symmetry Correlation Factor: {:.4}", correlation_factor);
    println!("\nInterpretation:");
    println!("A correlation factor of 0.0 would indicate no correlation (randomness).");
    println!("A correlation factor of 1.0 would indicate perfect symmetry (a prime never appears alone).");

    Ok(())
}
