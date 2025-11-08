//! Prime Base Deep Dive - Focusing on bases 3, 5, 7, 11, 13
//! 
//! Prime bases have ALL non-zero digits coprime with the base.
//! This creates unique dynamics worth exploring.

use prime_physics_engine::{
    is_prime_miller_rabin,
    ascii_art::*,
};
use num_bigint::BigUint;
fn analyze_prime_base(base: u32) -> (Vec<DiscoveredPrime>, BaseProperties) {
    println!("\n{}", boxed_title(&format!("PRIME BASE {} ANALYSIS", base), 60));
    
    let mut discovered_primes = Vec::new();
    let mut pattern_stats = std::collections::HashMap::new();
    // In prime bases, EVERY non-zero digit is coprime!
    println!("All digits 1 through {} are coprime with base {}", base - 1, base);
    // Test all possible configurations more thoroughly
    for outer in 1..base {
        for inner in 1..base {
            for k_outer in 0..=3 {
                for k_inner in 0..=3 {
                    let config = MembraneConfig { outer, inner, k_outer, k_inner };
                    let pattern_key = format!("({},{})_k({},{})", outer, inner, k_outer, k_inner);
                    
                    let mut pattern_primes = 0;
                    // Test with various seed lengths
                    for seed_len in 1..=3 {
                        let test_seeds = generate_test_seeds(seed_len, base);
                        
                        for seed in test_seeds {
                            let candidate = generate_prime_candidate(&config, &seed, base);
                            
                            if is_prime_miller_rabin(&candidate, 20) {
                                pattern_primes += 1;
                                
                                if discovered_primes.len() < 50 {
                                    discovered_primes.push(DiscoveredPrime {
                                        value: candidate.clone(),
                                        config: config.clone(),
                                        seed: seed.clone(),
                                        base,
                                    });
                                }
                            }
                        }
                    }
                    if pattern_primes > 0 {
                        pattern_stats.insert(pattern_key, pattern_primes);
                }
            }
        }
    }
    // Find the best patterns
    let mut best_patterns: Vec<_> = pattern_stats.iter().collect();
    best_patterns.sort_by(|a, b| b.1.cmp(a.1));
    println!("\nTop 5 patterns for base {}:", base);
    for (pattern, count) in best_patterns.iter().take(5) {
        println!("  {} → {} primes", pattern, count);
    // Analyze special properties
    let properties = BaseProperties {
        base,
        allows_perfect_symmetry: true, // All digits work
        special_distances: find_special_distances(base),
        unique_behavior: match base {
            3 => "Minimal prime base - only digits 1,2 available".to_string(),
            5 => "Creates natural pentagonal symmetries".to_string(),
            7 => "Seven-fold symmetry aligns with many patterns".to_string(),
            11 => "First two-digit prime base".to_string(),
            13 => "Lucky prime - often produces unusual patterns".to_string(),
            _ => "Prime base with unique properties".to_string(),
        },
    };
    (discovered_primes, properties)
}
#[derive(Debug, Clone)]
struct DiscoveredPrime {
    value: BigUint,
    config: MembraneConfig,
    seed: String,
    base: u32,
#[derive(Debug)]
struct BaseProperties {
    allows_perfect_symmetry: bool,
    special_distances: Vec<u32>,
    unique_behavior: String,
fn generate_test_seeds(length: usize, base: u32) -> Vec<String> {
    let mut seeds = Vec::new();
    if length == 1 {
        for d in 0..base.min(10) {
            seeds.push(d.to_string());
    } else if length == 2 {
        for d1 in 0..base.min(5) {
            for d2 in 0..base.min(5) {
                seeds.push(format!("{}{}", d1, d2));
    } else if length == 3 {
        // Just a sample for length 3
        for d in 0..base.min(3) {
            seeds.push(format!("{}{}{}", d, d+1, d));
    seeds
fn find_special_distances(base: u32) -> Vec<u32> {
    let mut special = Vec::new();
    // Distance 1 is always special
    special.push(1);
    // Half the base (if integer)
    if base > 2 {
        special.push(base / 2);
    // One less than base
    special.push(base - 1);
    special
fn visualize_prime_base_symmetry(base: u32) -> String {
    match base {
        3 => r#"
    Base 3 Symmetry (Triangular)
    =============================
           1
          / \
         /   \
        2─────0
        
    All paths are equivalent!
    Each digit is 120° apart.
"#.to_string(),
        5 => r#"
    Base 5 Symmetry (Pentagonal)
        2     0
        |     |
        3─────4
    Pentagon creates natural
    golden ratio relationships!
        7 => r#"
    Base 7 Symmetry (Heptagonal)
          1
        /   \
       2     0
      /       \
     3         6
      \       /
       4─────5
       
    Seven-fold symmetry appears
    in many natural systems!
        _ => format!("Base {} has {}-fold rotational symmetry", base, base)
fn main() {
    println!("{}", banner("PRIME BASE DEEP DIVE", 70));
    println!("\nExploring the unique properties of prime number bases...\n");
    let prime_bases = vec![3, 5, 7, 11, 13];
    let mut all_discoveries = Vec::new();
    for base in &prime_bases {
        let (primes, properties) = analyze_prime_base(*base);
        all_discoveries.push((*base, primes, properties));
    // Create comparison visualization
    println!("\n{}", boxed_title("PRIME BASE COMPARISON", 70));
    println!("\nUnique Properties Summary:");
    println!("{}", separator("double", 70));
    for (base, _, props) in &all_discoveries {
        println!("\nBase {} (prime):", base);
        println!("  {}", props.unique_behavior);
        println!("  Special distances: {:?}", props.special_distances);
    // Show symmetry visualizations
    println!("\n{}", boxed_title("SYMMETRY PATTERNS", 70));
    for base in vec![3, 5, 7] {
        println!("{}", visualize_prime_base_symmetry(base));
    // Find cross-base patterns
    println!("\n{}", boxed_title("CROSS-BASE DISCOVERIES", 70));
    // Check if any configurations work across multiple prime bases
    let mut cross_base_configs = std::collections::HashMap::new();
    for (base1, primes1, _) in &all_discoveries {
        for prime in primes1 {
            let key = format!("({},{})_k({},{})", 
                prime.config.outer, 
                prime.config.inner,
                prime.config.k_outer,
                prime.config.k_inner
            );
            
            cross_base_configs.entry(key)
                .or_insert_with(Vec::new)
                .push(*base1);
    println!("\nConfigurations that work in multiple prime bases:");
    for (config, bases) in cross_base_configs {
        if bases.len() > 1 {
            println!("  {} works in bases: {:?}", config, bases);
    // Special investigation: "Atom-like" patterns
    println!("\n{}", boxed_title("PREPARING FOR ATOM-LIKE PATTERNS", 70));
    println!(r#"
Next Investigation: Multi-Membrane Atomic Structures
====================================================
Based on our prime base analysis, we should explore:
1. DOUBLE MEMBRANE patterns:
   outer₁ + k₁ + inner₁ + k₂ + NUCLEUS + k₂ + inner₁ + k₁ + outer₁
   
2. TRIPLE MEMBRANE patterns:
   Like electron shells around an atom
3. BREATHING DOUBLE MEMBRANES:
   Where each layer can breathe independently
4. CROSS-BASE ATOMIC PRIMES:
   Finding structures that remain prime across multiple bases
The prime bases (3,5,7,11,13) offer unique testing grounds
because ALL their digits participate equally in the patterns.
"#);
    // Save findings
    let filename = format!("prime_base_analysis_{}.txt", 
        Local::now().format("%Y%m%d_%H%M%S"));
    let mut file = File::create(&filename).expect("Failed to create file");
    writeln!(file, "{}", boxed_title("PRIME BASE DEEP DIVE RESULTS", 80)).unwrap();
    writeln!(file, "\nGenerated: {}\n", Local::now()).unwrap();
    for (base, discoveries, properties) in all_discoveries {
        writeln!(file, "\n{}", banner(&format!("BASE {} DISCOVERIES", base), 80)).unwrap();
        writeln!(file, "{}", properties.unique_behavior).unwrap();
        writeln!(file, "\nExample primes found:").unwrap();
        for (i, prime) in discoveries.iter().take(10).enumerate() {
            writeln!(file, "  {}. {} (seed: {})", i+1, prime.value, prime.seed).unwrap();
    writeln!(file, "\n{}", separator("wave", 80)).unwrap();
    writeln!(file, "\nReady for multi-membrane atomic pattern exploration!").unwrap();
    println!("\n✅ Prime base analysis complete!");
    println!("📄 Results saved to: {}", filename);
    println!("\n{}", simple_box(
        "Key Insight: Prime bases treat all digits equally,\n\
         making them ideal for exploring pure mathematical\n\
         patterns without base-factor interference!"
    ));
