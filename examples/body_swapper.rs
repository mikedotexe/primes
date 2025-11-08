
use num_bigint::BigUint;
use prime_physics_engine::is_prime;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run --release --example body_swapper -- <body1> <body2>");
        return Ok(());
    }
    let body1 = &args[1];
    let body2 = &args[2];
    let max_space_size = 100; // A smaller range for faster comparative runs

    println!("space_size,prime_yield");

    for space_size in 1..=max_space_size {
        let mut prime_yield = 0;
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
                        prime_yield += 1;
                    }
                }
            }
        }
        println!("{},{}", space_size, prime_yield);
    }

    Ok(())
}
