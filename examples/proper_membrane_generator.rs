//! Proper Membrane Generator
//!
//! This creates a DETERMINISTIC membrane generator that uses seeds correctly
//! unlike the MembraneBuilder which randomizes or searches for primes.
//!
//! # Purpose
//!
//! Demonstrates symmetric "membrane" structures as base-aware candidate
//! generators. The point of this example is deterministic construction, not a
//! new density theorem.
//!
//! # Expected Output
//!
//! ```text
//! 🎯 Proper Membrane Prime Generator
//! ================================================================================
//!
//! Testing all configurations with all single-digit seeds:
//!
//! Breathing (3,3) k=(0,1)
//! Expected success rate: 30%
//!   Seed 4: 3304033 ✓
//!   Seed 5: 3305033 ✓
//!   Seed 7: 3307033 ✓
//! Actual success rate: 30% (3/10)
//!
//! Symmetric (3,3) k=(1,1)
//! Expected success rate: 10%
//!   Seed 5: 303050303 ✓
//! Actual success rate: 10% (1/10)
//!
//! [...]
//!
//! 📊 Statistical Generation (100 attempts)
//! Results:
//!   Success rate: 55%
//!
//! Breakdown by configuration:
//!   Breathing (3,3) k=(0,1): 37 primes
//!   Symmetric (3,3) k=(1,1): 10 primes
//! ```
//!
//! # Key Concepts
//!
//! **Membrane Structure**: outer + zeros + inner + zeros + SEED + zeros + inner + zeros + outer
//!
//! **Example**: 3-◯-3-◯-5-◯-3-◯-3 → 3305033 ✓ PRIME
//!
//! # Runtime
//!
//! Approximately 1 minute.
//!
//! # Success Indicator
//!
//! You should see multiple known seed/configuration witnesses marked ✓.

use num_bigint::BigUint;
use primes::{is_prime, MembraneConfig};
use rand::prelude::*;

/// Generate a membrane prime using the seed AS the middle digit(s)
fn generate_membrane_directly(config: &MembraneConfig, seed: u32) -> Option<BigUint> {
    let num = config.construct_number(seed).ok()?;

    if is_prime(&num) {
        Some(num)
    } else {
        None
    }
}

/// Statistical prime generator based on empirical data
struct StatisticalGenerator {
    configs: Vec<EmpiricalConfig>,
}

#[derive(Clone)]
struct EmpiricalConfig {
    config: MembraneConfig,
    success_rate: f64,
    known_seeds: Vec<u32>,
    description: String,
}

impl StatisticalGenerator {
    fn new() -> Self {
        Self {
            configs: vec![
                // Base 10 breathing pattern - 30% success
                EmpiricalConfig {
                    config: MembraneConfig::new(10, 3, 3, 0, 1),
                    success_rate: 0.30,
                    known_seeds: vec![4, 5, 7],
                    description: "Breathing (3,3) k=(0,1)".to_string(),
                },
                // Base 10 symmetric - 10% success
                EmpiricalConfig {
                    config: MembraneConfig::new(10, 3, 3, 1, 1),
                    success_rate: 0.10,
                    known_seeds: vec![5],
                    description: "Symmetric (3,3) k=(1,1)".to_string(),
                },
                // Base 10 exclusive - 10% success
                EmpiricalConfig {
                    config: MembraneConfig::new(10, 3, 7, 1, 1),
                    success_rate: 0.10,
                    known_seeds: vec![5],
                    description: "Exclusive (3,7) k=(1,1)".to_string(),
                },
                // Base 6 champion - 33% success
                EmpiricalConfig {
                    config: MembraneConfig::new(6, 1, 5, 0, 0),
                    success_rate: 1.0 / 6.0,
                    known_seeds: vec![4],
                    description: "Base 6 champion (1,5) k=(0,0)".to_string(),
                },
            ],
        }
    }

    /// Generate a prime by selecting among known witness seeds with report
    /// weights. This is a deterministic-witness demo, not a density sampler.
    fn generate_weighted(&self) -> Option<(BigUint, String)> {
        let mut rng = thread_rng();

        // Select config based on success rate
        let total_weight: f64 = self.configs.iter().map(|c| c.success_rate).sum();
        let mut roll = rng.gen::<f64>() * total_weight;

        for emp_config in &self.configs {
            roll -= emp_config.success_rate;
            if roll <= 0.0 {
                // Use a known good seed
                if let Some(&seed) = emp_config.known_seeds.choose(&mut rng) {
                    if let Some(prime) = generate_membrane_directly(&emp_config.config, seed) {
                        return Some((prime, emp_config.description.clone()));
                    }
                }
                break;
            }
        }

        None
    }

    /// Test all configurations systematically
    fn test_all_configs(&self) {
        println!("Testing all configurations with all single-digit seeds:");
        println!("{}", "-".repeat(80));

        for emp_config in &self.configs {
            println!("\n{}", emp_config.description);
            println!(
                "Known single-digit witness rate: {:.1}%",
                known_witness_rate(emp_config) * 100.0
            );
            println!("Search weight: {:.1}%", emp_config.success_rate * 100.0);

            let mut found = 0;
            let mut successful_seeds = Vec::new();
            let seed_space = single_digit_seed_space(&emp_config.config);

            for seed in 0..seed_space {
                if let Some(prime) = generate_membrane_directly(&emp_config.config, seed) {
                    found += 1;
                    successful_seeds.push(seed);
                    println!(
                        "  Seed {}: {} [{}] ✓",
                        seed,
                        prime,
                        membrane_digit_label(&emp_config.config, seed)
                    );
                }
            }

            println!(
                "Actual single-digit witness rate: {:.1}% ({}/{})",
                found as f64 * 100.0 / seed_space as f64,
                found,
                seed_space
            );
            println!("Successful seeds: {:?}", successful_seeds);

            if successful_seeds != emp_config.known_seeds {
                println!("⚠️  WARNING: Actual seeds differ from expected!");
            }
        }
    }
}

fn main() {
    println!("🎯 Proper Membrane Prime Generator");
    println!("{}", "=".repeat(80));
    println!();

    let generator = StatisticalGenerator::new();

    // First, verify our empirical data
    generator.test_all_configs();

    // Generate some primes from known witnesses.
    println!("\n\n📊 Known-Seed Weighted Generation (100 draws)");
    println!("{}", "-".repeat(80));

    let mut successes = 0;
    let mut by_config = std::collections::HashMap::new();

    for _ in 0..100 {
        if let Some((prime, config_name)) = generator.generate_weighted() {
            successes += 1;
            *by_config.entry(config_name).or_insert(0) += 1;

            if successes <= 5 {
                println!("Generated: {}", prime);
            }
        }
    }

    println!("\nResults:");
    println!("  Witness draws that produced primes: {}%", successes);
    println!("\nBreakdown by configuration:");
    for (config, count) in by_config.iter() {
        println!("  {}: {} primes", config, count);
    }

    // Demonstrate deterministic generation
    println!("\n\n🔒 Deterministic Generation");
    println!("{}", "-".repeat(80));
    println!("Using known seed-config pairs:");

    let deterministic_pairs = vec![
        (
            MembraneConfig::new(10, 3, 3, 0, 1),
            5,
            "Breathing pattern, seed 5",
        ),
        (
            MembraneConfig::new(10, 3, 7, 1, 1),
            5,
            "Exclusive config, seed 5",
        ),
        (
            MembraneConfig::new(10, 3, 3, 1, 1),
            5,
            "Symmetric pattern, seed 5",
        ),
        (
            MembraneConfig::new(6, 1, 5, 0, 0),
            4,
            "Base-6 witness, seed 4",
        ),
    ];

    for (config, seed, desc) in deterministic_pairs {
        if let Some(prime) = generate_membrane_directly(&config, seed) {
            println!(
                "\n{}: {} [{}] ✓",
                desc,
                prime,
                membrane_digit_label(&config, seed)
            );

            println!("  Structure: {}", membrane_structure_label(&config, seed));
        }
    }
}

fn single_digit_seed_space(config: &MembraneConfig) -> u32 {
    config.base
}

fn known_witness_rate(emp_config: &EmpiricalConfig) -> f64 {
    emp_config.known_seeds.len() as f64 / single_digit_seed_space(&emp_config.config) as f64
}

fn membrane_digit_label(config: &MembraneConfig, seed: u32) -> String {
    let middle_digits = match config.middle_digits_from_seed(seed) {
        Ok(digits) => digits,
        Err(_) => return "invalid middle".to_string(),
    };
    let digits = match config.construct_digits_from_middle_digits(&middle_digits) {
        Ok(digits) => digits,
        Err(_) => return "invalid template".to_string(),
    };

    format!(
        "{} (base {})",
        digits
            .iter()
            .map(|&digit| digit_symbol(digit))
            .collect::<Vec<_>>()
            .join(""),
        config.base
    )
}

fn digit_symbol(digit: u32) -> String {
    if digit < 10 {
        digit.to_string()
    } else {
        let offset = digit - 10;
        if offset < 26 {
            ((b'A' + offset as u8) as char).to_string()
        } else {
            format!("[{digit}]")
        }
    }
}

fn membrane_structure_label(config: &MembraneConfig, seed: u32) -> String {
    let middle_digits = match config.middle_digits_from_seed(seed) {
        Ok(digits) => digits,
        Err(_) => return "invalid middle".to_string(),
    };
    let digits = match config.construct_digits_from_middle_digits(&middle_digits) {
        Ok(digits) => digits,
        Err(_) => return "invalid template".to_string(),
    };

    digits
        .iter()
        .map(|&digit| {
            if digit == 0 {
                "◯".to_string()
            } else {
                digit_symbol(digit)
            }
        })
        .collect::<Vec<_>>()
        .join("")
}
