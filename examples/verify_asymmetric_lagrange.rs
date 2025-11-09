//! Quick verification of asymmetric Lagrange points

use num_bigint::BigUint;
use prime_physics_engine::is_prime;
use std::str::FromStr;

fn main() {
    let p1 = "97";
    let p2 = "30305070305070303";

    println!("Testing {} + 7 zeros + {}", p1, p2);
    println!();

    for pos in 0..7 {
        for digit in 1..=9 {
            let mut buffer = vec!['0'; 7];
            buffer[pos] = char::from_digit(digit, 10).unwrap();
            let buffer_str: String = buffer.into_iter().collect();

            let full = format!("{}{}{}", p1, buffer_str, p2);
            let num = BigUint::from_str(&full).unwrap();

            if is_prime(&num) {
                println!("FOUND: Position {}, Digit {} → PRIME!", pos, digit);
                println!("       {}", full);
            }
        }
    }
}
