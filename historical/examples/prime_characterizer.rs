use num_bigint::BigUint;
use num_traits::{One, Zero};
use primes::is_prime;
use std::env;

// Function to calculate the sum of digits of a BigUint
fn digit_sum(n: &BigUint) -> u32 {
    n.to_string().chars().filter_map(|c| c.to_digit(10)).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --release --example prime_characterizer -- <space_size>");
        return Ok(());
    }
    let space_size: usize = args[1].parse()?;

    let body1 = "7";
    let body2 = "11";

    println!("space_size,position,digit,digit_sum,form");

    for position in 0..space_size {
        for digit in 1..=9 {
            let mut test_str = "0".repeat(space_size);
            unsafe {
                test_str.as_bytes_mut()[position] = b'0' + digit as u8;
            }

            let full_number_str = format!("{}{}{}", body1, test_str, body2);
            if let Ok(num) = full_number_str.parse::<BigUint>() {
                if is_prime(&num) {
                    let sum_of_digits = digit_sum(&num);

                    let six = BigUint::from(6u32);
                    let one = BigUint::one();

                    let form = if (&num + &one) % &six == BigUint::zero() {
                        "6k-1"
                    } else if (&num - &one) % &six == BigUint::zero() {
                        "6k+1"
                    } else {
                        "other" // Should only be 2 or 3, not possible here
                    };

                    println!(
                        "{},{},{},{},{}",
                        space_size, position, digit, sum_of_digits, form
                    );
                }
            }
        }
    }

    Ok(())
}
