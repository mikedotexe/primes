use std::io;use std::env;//! Educational Explorer - Colored Edition
//! 
//! Same as educational_explorer.rs but with unified color scheme

use primes::{
    is_prime,
};
use num_bigint::BigUint;
fn main() {
    let args: Vec<String> = env::args().collect();
    let interactive = args.contains(&"--interactive".to_string());
    
    println!("{}", "🔬 Membrane Prime Generation - Educational Explorer".bright_cyan().bold());
    println!("{}", "For the adventurous and curious only.".bright_white());
    if interactive {
        println!("{}", "💡 Tip: Run without --interactive for a guided demo first!".yellow());
    } else {
        println!("{}", "💡 Tip: Run with --interactive flag for hands-on exploration!".yellow());
    }
        run_interactive_mode();
        run_demo_mode();
}
fn run_demo_mode() {
    // The core discovery
    println!("\n{}", "=== The Core Discovery ===".bright_cyan().bold());
    println!("{}", "Symmetric number constructions with coprime boundary digits".bright_white());
    println!("{}", "generate primes at higher rates than random chance.".bright_white());
    println!("\n{}", "• Symmetric: patterns like 3-7-5-7-3 that mirror themselves".green());
    println!("{}", "• Constructions: we build primes with architectural intention".green());
    println!("{}", "• Coprime digits: gcd(digit, base) = 1 is absolutely essential".green());
    println!("{}", "• Higher rates: 3-7x improvement, not miracles".green());
    println!("{}", "• Than random: our honest baseline - systematic advantage".green());
    // Start with verified examples that actually work
    println!("\n{}", "=== Real Examples That Work ===".bright_cyan().bold());
    // Test some actual constructions
    let examples = [
        ("Base 10, (3,7) k=(0,0)", 10, 3, 7, 0, 0),
        ("Base 6, (1,5) k=(0,0)", 6, 1, 5, 0, 0),
        ("Base 12, (5,7) k=(0,0)", 12, 5, 7, 0, 0),
    ];
    for (desc, base, outer, inner, k_outer, k_inner) in examples {
        println!("\n{}", desc.bright_yellow().bold());
        let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);
        
        let mut prime_count = 0;
        let seeds = [1, 2, 3, 4, 5];
        for seed in seeds {
            match MembraneBuilder::new(config.clone()).with_seed(seed).build() {
                Ok(particle) => {
                    let is_prime_result = is_prime(&particle.value);
                    if is_prime_result {
                        prime_count += 1;
                        println!("Seed {}: {} {}", seed, particle.value.to_string().bright_green(), "✓ PRIME".bright_green().bold());
                    } else {
                        println!("Seed {}: {} {}", seed, particle.value.to_string().red(), "✗ composite".red());
                    }
                }
                Err(_) => println!("Seed {}: {}", seed, "construction failed".red()),
            }
        }
        let rate = (prime_count as f64 / seeds.len() as f64) * 100.0;
        let rate_color = match rate {
            r if r >= 30.0 => "bright_green",
            r if r >= 20.0 => "green", 
            r if r >= 10.0 => "yellow",
            _ => "red",
        };
        println!("Result: {}/{} = {:.1}% prime rate", 
            prime_count, 
            seeds.len(), 
            match rate_color {
                "bright_green" => format!("{:.1}%", rate).bright_green().bold(),
                "green" => format!("{:.1}%", rate).green().bold(),
                "yellow" => format!("{:.1}%", rate).yellow().bold(),
                _ => format!("{:.1}%", rate).red().bold(),
        );
    // Show coprimality violation
    println!("\n{}", "=== What Happens When You Break Coprimality ===".bright_red().bold());
    println!("{}", "Base 6, using (2,4) - both share factor 2 with base 6".red());
    let bad_config = MembraneConfig::new(6, 2, 4, 0, 0);
    let mut bad_prime_count = 0;
    let seeds = [1, 2, 3];
    for seed in seeds {
        match MembraneBuilder::new(bad_config.clone()).with_seed(seed).build() {
            Ok(particle) => {
                let is_prime_result = is_prime(&particle.value);
                if is_prime_result {
                    bad_prime_count += 1;
                    println!("Seed {}: {} {}", seed, particle.value.to_string().bright_green(), "✓ PRIME".bright_green().bold());
                } else {
                    println!("Seed {}: {} {}", seed, particle.value.to_string().red(), "✗ composite".red());
            Err(_) => println!("Seed {}: {}", seed, "construction failed".red()),
    println!("Result: {}/{} = {:.1}% prime rate", 
        bad_prime_count, 
        seeds.len(), 
        format!("{:.1}%", (bad_prime_count as f64 / seeds.len() as f64) * 100.0).red().bold()
    );
    // Key insights
    println!("\n{}", "=== Key Insights ===".bright_cyan().bold());
    println!("1. {} is essential (digits must not share factors with base)", "Coprimality".bright_green().bold());
    println!("2. {} works best (k=0,0)", "Minimal padding".bright_green().bold());
    println!("3. {} work across many bases", "Some patterns".bright_green().bold());
    println!("4. It's {} better than random chance", "3-7x".bright_green().bold());
    println!("\n{}", "=== For the Adventurous ===".bright_yellow().bold());
    println!("• Try: {}", "cargo run --example membrane_lab_tui".cyan());
    println!("• Try: {}", "cargo run --example prime_discovery_dashboard".cyan());
    println!("• Read: {} for 286,200 test results", "CLAUDE.md".cyan());
    println!("\n{}", "This is just the beginning. There are Lagrange points,".bright_white());
    println!("{}", "gravitational membrane dynamics, and much more to discover.".bright_white());
fn run_interactive_mode() {
    println!("{}", "=== Interactive Mode ===".bright_cyan().bold());
    println!("{}", "Let's explore membrane prime generation together!".bright_white());
    println!("{}", "(Type 'help' for commands or 'quit' to exit)".yellow());
    loop {
        print!("{}", "> ".bright_cyan().bold());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "quit" | "exit" => {
                println!("{}", "Thanks for exploring! 🔬".bright_green());
                break;
            "help" => show_help(),
            "test" => interactive_test(),
            "learn" => show_concepts(),
            "challenge" => run_challenge(),
            _ => {
                if input.starts_with("check ") {
                    check_configuration(input);
                } else if input.is_empty() {
                    continue;
                    println!("{}", "Unknown command. Type 'help' for available commands.".red());
fn show_help() {
    println!("\n{}", "Available commands:".bright_cyan().bold());
    println!("  {}      - Test your own membrane configuration", "test".green());
    println!("  {}     - Learn about key concepts", "learn".green());
    println!("  {} - Try a prime-finding challenge", "challenge".green());
    println!("  {} <base> <outer> <inner> - Quick check a configuration", "check".green());
    println!("  {}      - Show this help", "help".green());
    println!("  {}      - Exit interactive mode", "quit".green());
fn show_concepts() {
    println!("\n{}", "📚 Key Concepts:".bright_cyan().bold());
    println!("\n{}", "1. COPRIMALITY (most important!)".bright_green().bold());
    println!("   - Your boundary digits must not share factors with the base");
    println!("   - Example: In base 10, use {} (not {})", "1,3,7,9".green(), "2,4,5,6,8".red());
    println!("   - Why? Shared factors create divisibility patterns");
    println!("\n{}", "2. SYMMETRY".bright_green().bold());
    println!("   - Membrane structure: {}", "outer-inner-seed-inner-outer".yellow());
    println!("   - Example: {} → {}", "3-7-5-7-3".yellow(), "37573".bright_green());
    println!("   - The pattern mirrors itself");
    println!("\n{}", "3. MINIMAL PADDING".bright_green().bold());
    println!("   - {} works best - no extra zeros", "k=(0,0)".green());
    println!("   - Adding zeros dilutes the pattern");
    println!("\n{}", "4. BASE MATTERS".bright_green().bold());
    println!("   - Different bases have different optimal digits");
    println!("   - Base 6: {} is champion", "(1,5)".bright_green());
    println!("   - Base 10: {} or {} work well", "(3,7)".green(), "(1,7)".green());
    println!("\n{}", "Press Enter to continue...".bright_black());
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
// Additional functions would continue with the same color scheme...
// This is a demonstration of how to apply unified colors
fn interactive_test() {
    println!("\n{}", "🔬 Configuration Tester".bright_cyan().bold());
    println!("{}", "Let's test your membrane configuration!".bright_white());
    // Implementation would use the same color scheme
fn check_configuration(input: &str) {
    println!("{}", "Configuration check not implemented in this demo".yellow());
fn run_challenge() {
    println!("\n{}", "🏆 CHALLENGE MODE".bright_yellow().bold());
    println!("{}", "Challenge mode not implemented in this demo".yellow());
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
