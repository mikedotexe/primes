use std::str::FromStr;//! Membrane Visualization: Shows all valid configurations for a given base/seed combination
//! 
//! This program demonstrates how a single seed digit can generate multiple primes through
//! different membrane configurations - like an electron existing in different orbitals.
//! The visualization emphasizes that primes are "groups" - different numbers in different
//! layers that together form a prime.

use prime_physics_engine::resonance_profiles::{
    BaseResonanceProfile, MembraneConfig, ConfigurationProfile
};
use prime_physics_engine::membrane::symmetric::construct_symmetric_membrane;
use num_bigint::BigUint;
use num_traits::Num;
use std::collections::HashSet;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about = "Visualize membrane configurations for a specific base and seed")]
struct Args {
    /// Base to analyze (e.g., 10 for decimal)
    #[arg(short, long, default_value_t = 10)]
    base: u32,
    
    /// Seed digit to focus on (0-9)
    #[arg(short, long, default_value_t = 5)]
    seed: u8,
    /// Maximum k-value to test
    #[arg(short = 'k', long, default_value_t = 4)]
    max_k: u8,
    /// Show all seeds, not just the specified one
    #[arg(short = 'a', long)]
    all_seeds: bool,
    /// Number of example primes to show per configuration
    #[arg(short = 'e', long, default_value_t = 3)]
    examples: usize,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║           MEMBRANE CONFIGURATION VISUALIZATION                    ║");
    println!("║                                                                  ║");
    println!("║  Showing how seed digits create primes through different        ║");
    println!("║  membrane configurations - like electrons in various orbitals    ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    // Discover all resonant configurations for the base
    let mut profile = BaseResonanceProfile::new(args.base);
    // Test range of digits based on base
    let max_digit = (args.base - 1).min(9) as u8;
    let outer_range: Vec<u8> = (1..=max_digit).collect();
    let inner_range: Vec<u8> = (1..=max_digit).collect();
    let k_range: Vec<u8> = (0..=args.max_k).collect();
    println!("Discovering resonant configurations for base {}...", args.base);
    println!("Testing ranges:");
    println!("  Outer digits: {:?}", outer_range);
    println!("  Inner digits: {:?}", inner_range);
    println!("  K-values: {:?}", k_range);
    println!();
    profile.discover_resonances(&outer_range, &inner_range, &k_range, 1)?;
    // Display results
    if args.all_seeds {
        visualize_all_seeds(&profile, args.examples)?;
    } else {
        visualize_single_seed(&profile, args.seed, args.base, args.examples)?;
    }
    // Show dead seeds
    if !profile.stats.dead_seeds.is_empty() {
        println!("\n⚠️  DEAD SEEDS (no configurations work): {:?}", profile.stats.dead_seeds);
    Ok(())
/// Visualize configurations for a single seed
fn visualize_single_seed(
    profile: &BaseResonanceProfile,
    max_examples: usize
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌟 CONFIGURATIONS FOR SEED {} IN BASE {} 🌟", seed, base);
    println!("═══════════════════════════════════════════");
    // Get all configurations that work with this seed
    let seed_configs: Vec<&ConfigurationProfile> = profile.configurations.iter()
        .filter(|c| c.successful_seeds.contains(&seed))
        .collect();
    if seed_configs.is_empty() {
        println!("\n❌ Seed {} is DEAD - no configurations generate primes!", seed);
        return Ok(());
    println!("\nFound {} configurations that work with seed {}:", seed_configs.len(), seed);
    for (idx, config_profile) in seed_configs.iter().enumerate() {
        println!("\n┌─ Configuration {} {}─┐", 
            idx + 1,
            if config_profile.is_exclusive { "(EXCLUSIVE) " } else { "" }
        );
        
        let config = &config_profile.config;
        // Draw the membrane structure
        draw_membrane_structure(config, seed);
        // Show statistics
        println!("\n│ Statistics:");
        println!("│   Success rate: {:.1}%", config_profile.success_rate * 100.0);
        println!("│   Prime count: {}", config_profile.prime_count);
        println!("│   Avg length: {:.1} digits", config_profile.avg_prime_length);
        if config_profile.is_exclusive {
            println!("│   ⚡ EXCLUSIVE: Only seed {} works with this configuration!", seed);
        } else {
            println!("│   Other seeds that work: {:?}", 
                config_profile.successful_seeds.iter()
                    .filter(|&&s| s != seed)
                    .collect::<Vec<_>>()
            );
        }
        // Generate and show example primes
        println!("│\n│ Example primes:");
        let examples = generate_prime_examples(config, seed, base, max_examples)?;
        for (i, (prime_str, prime_val)) in examples.iter().enumerate() {
            println!("│   {}. {} ", i + 1, prime_str);
            if prime_str.len() <= 20 {
                println!("│      = {}", prime_val);
            }
        println!("└{:─<60}┘", "");
    // Show orbital diagram for this seed
    println!("\n📊 ORBITAL FILLING PATTERN");
    println!("Each configuration is like an electron orbital:");
    draw_orbital_analogy(&seed_configs);
/// Visualize all seeds and their configurations
fn visualize_all_seeds(
    _max_examples: usize
    println!("\n🎯 ALL SEEDS ORBITAL DIAGRAM FOR BASE {} 🎯", profile.base);
    println!("════════════════════════════════════════════");
    let orbital_diagram = profile.seed_map.orbital_diagram();
    for seed in 0..=9 {
        let config_count = orbital_diagram.get(&seed).copied().unwrap_or(0);
        print!("Seed {}: ", seed);
        if config_count == 0 {
            println!("💀 DEAD (no configurations work)");
            // Draw orbital boxes
            let boxes = "▓".repeat(config_count.min(20));
            println!("{} ({} configs)", boxes, config_count);
            
            // Show exclusive configurations for this seed
            let exclusive_configs: Vec<&ConfigurationProfile> = profile.configurations.iter()
                .filter(|c| c.is_exclusive && c.exclusive_seed() == Some(seed))
                .collect();
            if !exclusive_configs.is_empty() {
                for ec in exclusive_configs {
                    println!("       ⚡ EXCLUSIVE: ({},{}) k=({},{})",
                        ec.config.outer_digit,
                        ec.config.inner_digit,
                        ec.config.k_outer,
                        ec.config.k_inner
                    );
                }
    println!("\n📈 STATISTICS:");
    println!("Total configurations: {}", profile.stats.total_configurations);
    println!("Exclusive configurations: {}", profile.stats.exclusive_configurations);
    println!("Active seeds: {}/10", profile.stats.seeds_with_configs);
/// Draw ASCII art representation of membrane structure
fn draw_membrane_structure(config: &MembraneConfig, seed: u8) {
    let k_outer_str = "0".repeat(config.k_outer);
    let k_inner_str = "0".repeat(config.k_inner);
    println!("│");
    println!("│ Membrane Structure:");
    // Top line showing the structure
    println!("│   ╭─{}─╮ ╭─{}─╮   ╭─{}─╮ ╭─{}─╮", 
        "─".repeat(config.k_outer.max(1)), 
        "─".repeat(config.k_inner.max(1)),
        "─".repeat(config.k_inner.max(1)), 
        "─".repeat(config.k_outer.max(1))
    );
    // The membrane itself
    print!("│   │ {} │ {} │ {} │ {} │ {} │ {} │ {} │", 
        config.outer_digit,
        if config.k_outer > 0 { &k_outer_str } else { "" },
        config.inner_digit,
        if config.k_inner > 0 { &k_inner_str } else { "" },
        seed,
        config.inner_digit
    if config.k_outer > 0 {
        print!(" {} │ {}", k_outer_str, config.outer_digit);
        print!(" {}", config.outer_digit);
    // Bottom line
    println!("│   ╰─{}─╯ ╰─{}─╯   ╰─{}─╯ ╰─{}─╯", 
    // Labels
    println!("│    outer   inner  seed  inner   outer");
    println!("│    shell   shell  core  shell   shell");
    // Show as a group
    println!("│ This is a GROUP of digits that together form primes!");
    println!("│ The seed {} resonates within the ({},{}) membrane", seed, config.outer_digit, config.inner_digit);
    println!("│ with k-padding ({},{}) creating a specific cavity shape.", config.k_outer, config.k_inner);
/// Generate example primes for a configuration
fn generate_prime_examples(
    config: &MembraneConfig,
    max_count: usize
) -> Result<Vec<(String, BigUint)>, Box<dyn std::error::Error>> {
    let mut examples = Vec::new();
    let mut tested = HashSet::new();
    // Try different patterns with the seed
    let patterns = vec![
        seed.to_string(),
        format!("{}{}", seed, seed),
        format!("{}{}{}", seed, seed, seed),
    ];
    for pattern in patterns {
        if examples.len() >= max_count {
            break;
        let membrane_str = construct_symmetric_membrane(
            config.outer_digit as u32,
            config.inner_digit as u32,
            &pattern,
            config.k_outer as u32,
            config.k_inner as u32,
        )?;
        if tested.contains(&membrane_str) {
            continue;
        tested.insert(membrane_str.clone());
        if let Ok(num) = BigUint::from_str_radix(&membrane_str, base) {
            if is_prime(&num) {
                examples.push((membrane_str, num));
    Ok(examples)
/// Draw an orbital analogy visualization
fn draw_orbital_analogy(configs: &[&ConfigurationProfile]) {
    println!("\n┌{:─<60}┐", "");
    println!("│ Orbital Type    Configuration         Success Rate         │");
    println!("├{:─<60}┤", "");
    for (_idx, config) in configs.iter().enumerate() {
        let orbital_type = match (config.config.k_outer, config.config.k_inner) {
            (0, 0) | (0, 1) | (1, 0) => "s-orbital",
            (1, 1) => "p-orbital",
            (2, 2) => "d-orbital",
            (3, 3) => "f-orbital",
            _ => "hybrid",
        };
        let config_str = format!("({},{}) k=({},{})",
            config.config.outer_digit,
            config.config.inner_digit,
            config.config.k_outer,
            config.config.k_inner
        let success_bar = "█".repeat((config.success_rate * 20.0) as usize);
        println!("│ {:12} {:20} {:20} │", orbital_type, config_str, success_bar);
    println!("└{:─<60}┘", "");
    println!("\nJust as electrons fill atomic orbitals in order of increasing");
    println!("energy, seeds 'fill' membrane configurations based on resonance!");
