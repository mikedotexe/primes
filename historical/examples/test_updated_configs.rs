//! Test the updated membrane configurations based on empirical verification

use primes::membrane::MembraneConfig;
fn main() {
    println!("Testing updated membrane configurations...\n");
    
    // Test the champion configuration
    println!("🏆 CHAMPION: Base 6 (1,5) k=(0,0)");
    let champion_config = MembraneConfig::best_for_base(6)[0].clone();
    println!("   {}", champion_config.summary());
    println!("   Expected density: {:.1}%", champion_config.expected_density * 100.0);
    println!("   Is high performance: {}", champion_config.is_high_performance());
    // Test a few other top performers
    println!("\n🥈 Other top performers:");
    let configs = [
        (14, "Base 14 (1,9)"),
        (9, "Base 9 (1,2)"),
        (4, "Base 4 (1,3)"),
        (11, "Base 11 (1,10)"),
    ];
    for (base, name) in configs {
        let config = MembraneConfig::best_for_base(base)[0].clone();
        println!("   {}: {:.1}% density", name, config.expected_density * 100.0);
    }
    // Test non-coprime warning
    println!("\n⚠️  Testing non-coprime warning:");
    let _bad_config = MembraneConfig::new(6, 3, 3, 0, 0); // Should warn
    // Test the original (3,7) k=(2,2) vs optimized (3,7) k=(0,0)
    println!("\n📊 Comparison: Original vs Optimized");
    let old_config = MembraneConfig::new(10, 3, 7, 2, 2);
    let new_config = MembraneConfig::new(10, 3, 7, 0, 0);
    println!("   Original (3,7) k=(2,2): {:.1}% density", old_config.expected_density * 100.0);
    println!("   Optimized (3,7) k=(0,0): {:.1}% density", new_config.expected_density * 100.0);
    println!("   Improvement: {:.1}x", new_config.expected_density / old_config.expected_density);
}
