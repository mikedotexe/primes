use num_bigint::BigUint;
use primes::is_prime;
use std::time::Instant;

fn fertility_scan(body1: &str, body2: &str) -> u32 {
    let max_space_size = 30; // A quick scan is sufficient
    let mut total_yield = 0;

    for space_size in 1..=max_space_size {
        let zeros = "0".repeat(space_size);
        for position in 0..space_size {
            for digit in 1..=9 {
                let mut test_str = zeros.clone();
                unsafe {
                    test_str.as_bytes_mut()[position] = b'0' + digit as u8;
                }
                let full_number = format!("{}{}{}", body1, test_str, body2);
                if let Ok(num) = full_number.parse::<BigUint>() {
                    if is_prime(&num) {
                        total_yield += 1;
                    }
                }
            }
        }
    }
    total_yield
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let primes_to_test = vec!["3", "5", "7", "11", "13", "17", "19", "23", "29", "31"];
    let num_primes = primes_to_test.len();

    println!("--- Pair Scanner ---");
    println!(
        "Scanning {} unique pairs of the first {} primes...",
        (num_primes * (num_primes - 1)) / 2,
        num_primes
    );
    println!("--------------------------------------------------");

    let start_time = Instant::now();

    for i in 0..num_primes {
        for j in (i + 1)..num_primes {
            let p1 = primes_to_test[i];
            let p2 = primes_to_test[j];

            let yield_result = fertility_scan(p1, p2);

            let classification = if yield_result > 0 {
                "Fertile"
            } else {
                "Sterile"
            };
            println!(
                "Pair: ({}, {}), Result: {}, Yield: {}",
                p1, p2, classification, yield_result
            );
        }
    }

    let duration = start_time.elapsed();
    println!("--------------------------------------------------");
    println!("Scan complete in {:.2?}.", duration);

    Ok(())
}
