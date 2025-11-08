use num_bigint::BigUint;
use prime_physics_engine::is_prime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body1 = "7";
    let body2 = "11";
    let space_size = 100;
    
    // Configuration for a known prime from our debugger run
    let prime_pos = 5;
    let prime_digit = 5;

    println!("--- Perturbation Analysis ---");
    println!("Testing stability of prime at space_size={}, position={}, digit={}", space_size, prime_pos, prime_digit);

    let mut base_str = "0".repeat(space_size);
    unsafe {
        base_str.as_bytes_mut()[prime_pos] = b'0' + prime_digit as u8;
    }

    let mut perturbations_survived = 0;
    let mut total_perturbations = 0;

    // Introduce a second, small perturbation
    for perturb_pos in 0..space_size {
        // Skip the position of our main digit
        if perturb_pos == prime_pos { continue; }

        let mut perturbed_str = base_str.clone();
        // The perturbation will be to change a '0' to a '1'
        unsafe {
            perturbed_str.as_bytes_mut()[perturb_pos] = b'1';
        }

        total_perturbations += 1;
        let full_number = format!("{}{}{}", body1, perturbed_str, body2);
        if let Ok(num) = full_number.parse::<BigUint>() {
            if is_prime(&num) {
                perturbations_survived += 1;
            }
        }
    }

    let stability_score = perturbations_survived as f64 / total_perturbations as f64;

    println!("\n--- Results ---");
    println!("Total Perturbations Tested: {}", total_perturbations);
    println!("Perturbations Survived (remained prime): {}", perturbations_survived);
    println!("Stability Score: {:.4}", stability_score);
    println!("\nInterpretation:");
    println!("A score of 0.0 means the prime state is fragile and any small change destroys it.");
    println!("A higher score indicates the state is robust and exists in a stable 'potential well'.");

    Ok(())
}
