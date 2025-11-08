
use num_bigint::BigUint;
use prime_physics_engine::is_prime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body1 = "7";
    let body2 = "11";
    let space_size = 100;

    println!("--- Symmetry Debugger ---");
    println!("Searching for primes at space_size = {}", space_size);
    println!("------------------------------------");

    for position in 0..space_size {
        for digit in 1..=9 {
            let mut test_str = "0".repeat(space_size);
            unsafe {
                test_str.as_bytes_mut()[position] = b'0' + digit as u8;
            }

            let full_number = format!("{}{}{}", body1, test_str, body2);
            if let Ok(num) = full_number.parse::<BigUint>() {
                if is_prime(&num) {
                    println!("Prime found: position={}, digit={}, number={}", position, digit, full_number);
                }
            }
        }
    }

    println!("------------------------------------");
    println!("Debug run complete.");

    Ok(())
}
