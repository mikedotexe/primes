use num_bigint::BigUint;
use prime_physics_engine::is_prime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body1 = "7";
    let body2 = "11";
    let max_space_size = 200;

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
