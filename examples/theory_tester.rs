use num_bigint::BigUint;
use primes::is_prime;
use std::env;

// A simple primality test for small numbers, as the main is_prime is for BigUint
fn is_small_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run --release --example theory_tester -- <body1> <body2>");
        return Ok(());
    }
    let body1_str = &args[1];
    let body2_str = &args[2];

    println!("--- Lagrange Field Theory Tester v1.0 ---");
    println!("Bodies under test: ({}, {})", body1_str, body2_str);

    // --- 1. Prediction Step ---
    println!("\n[Phase 1: Prediction]");
    println!("Applying Theory: The Primality Filter (Phase 4 discovery).");
    println!("Rule: A prime-generating field requires both bodies to be prime.");

    let body1_num: u64 = body1_str.parse()?;
    let body2_num: u64 = body2_str.parse()?;

    let prediction = if is_small_prime(body1_num) && is_small_prime(body2_num) {
        println!("Prediction: Bodies are both prime. Expecting POTENTIAL prime yield.");
        true
    } else {
        println!("Prediction: At least one body is composite. Expecting ZERO prime yield.");
        false
    };

    // --- 2. Experiment Step ---
    println!("\n[Phase 2: Experiment]");
    let max_space_size = 30; // A quick scan to test the prediction
    let mut total_yield = 0;
    println!(
        "Running limited resonance scan for space_size 1 to {}...",
        max_space_size
    );

    for space_size in 1..=max_space_size {
        let zeros = "0".repeat(space_size);
        for position in 0..space_size {
            for digit in 1..=9 {
                let mut test_str = zeros.clone();
                unsafe {
                    test_str.as_bytes_mut()[position] = b'0' + digit as u8;
                }
                let full_number = format!("{}{}{}", body1_str, test_str, body2_str);
                if let Ok(num) = full_number.parse::<BigUint>() {
                    if is_prime(&num) {
                        total_yield += 1;
                    }
                }
            }
        }
    }
    println!(
        "Experimental Result: Total prime yield was {}.",
        total_yield
    );

    // --- 3. Verification Step ---
    println!("\n[Phase 3: Verification]");
    let success = if prediction {
        total_yield > 0
    } else {
        total_yield == 0
    };

    if success {
        println!("SUCCESS: The experimental result matched the theoretical prediction.");
    } else {
        println!("FAILURE: The experimental result contradicted the theoretical prediction.");
    }
    println!("-----------------------------------------");

    Ok(())
}
