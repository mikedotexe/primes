use std::io;//! Visual Membrane Explorer - Interactive exploration with beautiful ASCII output
//! 
//! Uses the ascii_art module to create stunning visualizations

use prime_physics_engine::{
    is_prime_miller_rabin,
    ascii_art::*,
};
use std::fs::File;
fn main() {
    // Create a beautiful header
    print!("{}", boxed_title("VISUAL MEMBRANE EXPLORER", 60));
    println!("{}", banner("Discovering Prime Patterns with ASCII Art", 60));
    
    // Show the 5-7 phenomenon
    println!("{}", five_seven_diagram());
    println!("{}", separator("wave", 60));
    // Test some configurations and visualize them
    let configs = vec![
        (10, 3, 7, 1, 1, 5, "The Exclusive Configuration"),
        (6, 3, 3, 0, 1, 5, "Base 6 Champion"),
        (12, 5, 7, 0, 1, 5, "Base 12 Optimal"),
    ];
    let mut results = Vec::new();
    for (base, outer, inner, k_outer, k_inner, center, name) in configs {
        println!("\n{}", simple_box(&format!("Testing: {}", name)));
        
        // Show the membrane structure
        println!("\nStructure (base {}):", base);
        println!("{}", membrane_diagram(base, outer, inner, k_outer, k_inner, center));
        // Test for primality
        let config = MembraneConfig { outer, inner, k_outer, k_inner };
        let mut primes_found = 0;
        let mut total_tested = 0;
        let mut example_prime = None;
        // Show breathing pattern if applicable
        if k_outer != k_inner {
            println!("\n{}", breathing_diagram(k_outer, k_inner, false));
        }
        // Test multiple seeds
        print!("\nTesting seeds: ");
        for seed in 0..=100 {
            total_tested += 1;
            let candidate = generate_prime_candidate(&config, &seed.to_string(), base);
            
            if is_prime_miller_rabin(&candidate, 20) {
                primes_found += 1;
                if example_prime.is_none() && seed < 10 {
                    example_prime = Some((seed, candidate.clone()));
                }
            }
            // Progress indicator
            if seed % 10 == 0 {
                print!(".");
                std::io::stdout().flush().unwrap();
        println!(" Done!");
        let density = primes_found as f64 / total_tested as f64;
        results.push((name, density));
        // Show results with progress bar
        println!("\n{}", progress_bar(density, 0.35, 40, "Density"));
        // Show example if found
        if let Some((seed, prime)) = example_prime {
            let pattern = format!("seed {} → {}", seed, prime);
            println!("{}", atomic_prime(&pattern, &prime.to_string(), true));
        // Show resonance quality
        let quality = if density > 0.25 { "good" } else if density > 0.15 { "moderate" } else { "poor" };
        println!("{}", resonance_wave(k_inner, quality));
    }
    // Create comparison chart
    println!("\n{}", boxed_title("DENSITY COMPARISON", 60));
    println!("{}", comparison_chart(results.clone(), 40));
    // Show speedup visualization
    println!("\n{}", boxed_title("GPU ACCELERATION", 60));
    println!("{}", speedup_meter(270_000.0, 186_900_000.0));
    // Create summary statistics
    let best_config = results.iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();
    let stats = vec![
        ("Best Configuration", best_config.0.to_string()),
        ("Best Density", format!("{:.1}%", best_config.1 * 100.0)),
        ("Improvement vs Random", format!("{:.1}x", best_config.1 / 0.045)),
        ("GPU Speedup", "691x".to_string()),
    println!("\n{}", stats_box("SUMMARY STATISTICS", stats));
    // Save beautiful output to file
    let filename = format!("visual_exploration_{}.txt", Local::now().format("%Y%m%d_%H%M%S"));
    let mut file = File::create(&filename).expect("Failed to create file");
    writeln!(file, "{}", boxed_title("MEMBRANE PRIME VISUAL EXPLORATION", 70)).unwrap();
    writeln!(file, "\nGenerated: {}\n", Local::now().format("%Y-%m-%d %H:%M:%S")).unwrap();
    // Write the exclusive configuration in all its glory
    writeln!(file, "{}", simple_box(r#"
    The Crown Jewel: Configuration (3,7) k=(1,1) base 10
         3   0   7   0   5   0   7   0   3
         │   │   │   │   │   │   │   │   │
         └───┴───┴───┴───┼───┴───┴───┴───┘
                         │
                   WORKS ONLY WITH
                      SEED 5!
                         ↓
                    307050703
                    (PRIME!)"#)).unwrap();
    writeln!(file, "\n{}", separator("double", 70)).unwrap();
    writeln!(file, "\nDensity Results:").unwrap();
    writeln!(file, "{}", comparison_chart(results, 50)).unwrap();
    writeln!(file, "\n{}", five_seven_diagram()).unwrap();
    println!("\n✅ Visual exploration complete!");
    println!("📁 Results saved to: {}", filename);
    // Final flourish
    println!("\n{}", banner("Mathematics is Beautiful When Visualized", 60));
}
