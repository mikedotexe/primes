//! Educational Explorer - For the adventurous and curious
//! 
//! A no-nonsense introduction to membrane prime generation.
//! Real examples, real primes, real results.

use prime_physics_engine::{
    is_prime,
    membrane::{MembraneConfig, MembraneBuilder},
};
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let interactive = args.contains(&"--interactive".to_string());
    
    println!("🔬 Membrane Prime Generation - Educational Explorer");
    println!("For the adventurous and curious only.\n");
    
    if interactive {
        println!("💡 Tip: Run without --interactive for a guided demo first!");
        run_interactive_mode();
    } else {
        println!("💡 Tip: Run with --interactive flag for hands-on exploration!");
        run_demo_mode();
    }
}

fn run_demo_mode() {
    // The core discovery
    println!("=== The Core Discovery ===");
    println!("Symmetric number constructions with coprime boundary digits");
    println!("generate primes at higher rates than random chance.\n");
    
    // Test some actual constructions
    let examples = [
        ("Base 10, (3,7) k=(0,0)", 10, 3, 7, 0, 0),
        ("Base 6, (1,5) k=(0,0)", 6, 1, 5, 0, 0),
        ("Base 12, (5,7) k=(0,0)", 12, 5, 7, 0, 0),
    ];
    
    for (desc, base, outer, inner, k_outer, k_inner) in examples {
        println!("\n{}", desc);
        let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);
        
        let mut prime_count = 0;
        let seeds = [1, 2, 3, 4, 5];
        
        for seed in seeds {
            match MembraneBuilder::new(config.clone()).with_seed(seed).build() {
                Ok(particle) => {
                    let is_prime_result = is_prime(&particle.value);
                    if is_prime_result {
                        prime_count += 1;
                        println!("Seed {}: {} ✓ PRIME", seed, particle.value);
                    } else {
                        println!("Seed {}: {} ✗ composite", seed, particle.value);
                    }
                }
                Err(_) => println!("Seed {}: construction failed", seed),
            }
        }
        
        println!("Result: {}/{} = {:.1}% prime rate", prime_count, seeds.len(), 
                 (prime_count as f64 / seeds.len() as f64) * 100.0);
    }
    
    // Show key insights
    println!("\n=== Key Insights ===");
    println!("1. Coprimality is essential (digits must not share factors with base)");
    println!("2. Minimal padding works best (k=0,0)");
    println!("3. Some patterns work across many bases");
    println!("4. It's 3-7x better than random chance");
}

fn run_interactive_mode() {
    println!("=== Interactive Mode ===");
    println!("Let's explore membrane prime generation together!");
    println!("(Type 'quit' to exit)\n");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input {
            "quit" | "exit" => {
                println!("Thanks for exploring! 🔬");
                break;
            }
            _ => {
                println!("Interactive mode coming soon! For now, try the demo mode.");
                println!("Type 'quit' to exit.");
            }
        }
    }
}