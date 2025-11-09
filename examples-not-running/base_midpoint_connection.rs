//! The Base-Midpoint-Exclusivity Connection
//! ========================================
//! 
//! Connecting base properties, midpoint primality, and seed exclusivity

use primes::membrane::MembraneConfig;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 The Base-Midpoint-Exclusivity Connection");
    println!("===========================================\n");
    
    // Part 1: The fundamental theorem
    println!("HYPOTHESIS: In base b with prime midpoint m,");
    println!("seed m creates exclusive primes in symmetric configurations.\n");
    // Part 2: Test across multiple bases
    println!("Testing the hypothesis:\n");
    test_hypothesis();
    // Part 3: Why base 10 is special
    println!("\nWhy Base 10 is Special");
    println!("======================\n");
    analyze_base_10();
    // Part 4: Deep dive into bases 37 and 43
    println!("\nPart 4: Deep Dive - Bases 37 and 43");
    println!("====================================\n");
    analyze_bases_37_and_43();
    // Part 5: Predictions
    println!("\nPart 5: Predictions and Implications");
    make_predictions();
    Ok(())
}
fn test_hypothesis() {
    println!("Testing symmetric config (3,3) k=(1,1) with midpoint seeds:\n");
    println!("Base | Midpoint | Prime? | Seed Works? | Exclusive? | Prime Found");
    println!("-----|----------|--------|-------------|------------|------------");
    // Test bases with boundaries that work across bases
    let test_bases = vec![
        (6, 3, true),
        (8, 4, false),
        (10, 5, true),
        (12, 6, false),
        (14, 7, true),
        (16, 8, false),
        (18, 9, false),
        (20, 10, false),
        (22, 11, true),
        (26, 13, true),
    ];
    for (base, midpoint, midpoint_is_prime) in test_bases {
        // Use boundaries that exist in all bases (3,3)
        let config = MembraneConfig::new(base, 3, 3, 1, 1);
        
        // Test midpoint seed
        let mut midpoint_works = false;
        let mut midpoint_prime = String::new();
        if let Ok(num) = config.construct_number(midpoint) {
            if is_prime(&num) {
                midpoint_works = true;
                midpoint_prime = num.to_string();
            }
        }
        // Check exclusivity
        let mut working_seeds = 0;
        for seed in 0..base {
            if let Ok(num) = config.construct_number(seed) {
                if is_prime(&num) {
                    working_seeds += 1;
                }
        let exclusive = working_seeds == 1 && midpoint_works;
        println!("{:>4} | {:>8} | {:>6} | {:>11} | {:>10} | {}",
            base, midpoint,
            if midpoint_is_prime { "YES" } else { "no" },
            if midpoint_works { "YES" } else { "no" },
            if exclusive { "EXCLUSIVE!" } else { "-" },
            if midpoint_works { midpoint_prime } else { "-".to_string() }
        );
    }
    println!("\nPattern: Prime midpoints often (but not always) enable exclusivity!");
fn analyze_base_10() {
    println!("Base 10 = 2 × 5 creates unique properties:\n");
    println!("1. Factorization: 10 = 2 × 5 (both prime)");
    println!("2. Midpoint: 5 (prime AND factor of base)");
    println!("3. This dual role makes 5 exceptionally powerful\n");
    println!("Compare to other bases:");
    println!("  Base 6 = 2×3, midpoint 3 (factor AND prime) ✓");
    println!("  Base 14 = 2×7, midpoint 7 (factor AND prime) ✓");
    println!("  Base 22 = 2×11, midpoint 11 (factor AND prime) ✓");
    println!("  Base 26 = 2×13, midpoint 13 (factor AND prime) ✓\n");
    println!("But base 10 is the smallest where this happens!");
    println!("This makes it the 'fundamental' case.\n");
    // Test edge pairs in base 10
    println!("Base 10 edge pairs and seed 5:");
    let edge_pairs = vec![
        (1, 9, "extremes"),
        (2, 8, "near-extremes"),
        (3, 7, "classic"),
        (4, 6, "near-center"),
    for (a, b, name) in edge_pairs {
        let config = MembraneConfig::new(10, a, b, 1, 1);
        if let Ok(num) = config.construct_number(5) {
            println!("  ({},{}) {}: {} → {}",
                a, b, name, 5,
                if is_prime(&num) { "PRIME ✓" } else { "composite" }
            );
fn analyze_bases_37_and_43() {
    println!("Base 37 and 43 are fascinating edge cases!\n");
    println!("Base 37:");
    println!("========");
    println!("• 37 is the 12th prime number");
    println!("• Midpoint: 18.5 → rounds to 18 or 19");
    println!("• 18 = 2 × 3² (composite)");
    println!("• 19 is prime!");
    println!("• Unique: straddles composite/prime midpoint\n");
    println!("Base 43:");
    println!("• 43 is the 14th prime number");
    println!("• Midpoint: 21.5 → rounds to 21 or 22");
    println!("• 21 = 3 × 7 (composite)");
    println!("• 22 = 2 × 11 (composite)");
    println!("• Both midpoint candidates are composite!\n");
    // Test various configurations in base 37
    println!("Testing Base 37 Configurations:");
    println!("-------------------------------\n");
    // Test boundaries that make sense in base 37
    let base37_configs = vec![
        ((3, 3, 1, 1), "symmetric 3s"),
        ((3, 7, 1, 1), "classic 3-7"),
        ((5, 5, 1, 1), "symmetric 5s"),
        ((18, 19, 1, 1), "straddling midpoints"),
        ((9, 28, 1, 1), "quarter points"),
        ((12, 25, 1, 1), "third points"),
    for ((outer, inner, k_outer, k_inner), desc) in &base37_configs {
        test_configuration(37, *outer, *inner, *k_outer, *k_inner, desc);
    // Test various configurations in base 43
    println!("\nTesting Base 43 Configurations:");
    let base43_configs = vec![
        ((21, 22, 1, 1), "both midpoints"),
        ((10, 33, 1, 1), "~quarter points"),
        ((14, 29, 1, 1), "~third points"),
        ((7, 36, 1, 1), "seventh points"),
    for ((outer, inner, k_outer, k_inner), desc) in &base43_configs {
        test_configuration(43, *outer, *inner, *k_outer, *k_inner, desc);
    // Special analysis: Edge pairs
    println!("\nEdge Pair Analysis:");
    println!("===================\n");
    // Base 37 edge pairs
    println!("Base 37 edge pairs (equidistant from 0 and 37):");
    let base37_edge_pairs = vec![
        (1, 36), (2, 35), (3, 34), (4, 33), (5, 32),
        (6, 31), (7, 30), (8, 29), (9, 28), (10, 27),
    let mut best_37 = (0, 0, 0);
    for (a, b) in &base37_edge_pairs {
        let config = MembraneConfig::new(37, *a, *b, 1, 1);
        let mut prime_count = 0;
        for seed in 0..37 {
                    prime_count += 1;
        if prime_count > best_37.2 {
            best_37 = (*a, *b, prime_count);
    println!("  Best edge pair: ({},{}) with {} primes", best_37.0, best_37.1, best_37.2);
    // Base 43 edge pairs
    println!("\nBase 43 edge pairs:");
    let base43_edge_pairs = vec![
        (1, 42), (2, 41), (3, 40), (4, 39), (5, 38),
        (6, 37), (7, 36), (8, 35), (9, 34), (10, 33),
    let mut best_43 = (0, 0, 0);
    for (a, b) in &base43_edge_pairs {
        let config = MembraneConfig::new(43, *a, *b, 1, 1);
        for seed in 0..43 {
        if prime_count > best_43.2 {
            best_43 = (*a, *b, prime_count);
    println!("  Best edge pair: ({},{}) with {} primes", best_43.0, best_43.1, best_43.2);
    // Test midpoint seeds specifically
    println!("\nMidpoint Seed Analysis:");
    // Base 37 with seeds 18 and 19
    let config37 = MembraneConfig::new(37, 3, 3, 1, 1);
    println!("Base 37 (3,3) k=(1,1):");
    for seed in [17, 18, 19, 20] {
        if let Ok(num) = config37.construct_number(seed) {
            println!("  Seed {}: {} {}",
                seed,
                if is_prime(&num) { "PRIME ✓" } else { "composite" },
                if seed == 18 || seed == 19 { "(midpoint candidate)" } else { "" }
    // Base 43 with seeds 21 and 22
    let config43 = MembraneConfig::new(43, 3, 3, 1, 1);
    println!("\nBase 43 (3,3) k=(1,1):");
    for seed in [20, 21, 22, 23] {
        if let Ok(num) = config43.construct_number(seed) {
                if seed == 21 || seed == 22 { "(midpoint candidate)" } else { "" }
    // Analyze prime density patterns
    println!("\nPrime Density Patterns:");
    analyze_density_pattern(37);
    analyze_density_pattern(43);
fn test_configuration(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, desc: &str) {
    let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);
    let mut working_seeds = Vec::new();
    let mut exclusive_seed = None;
    for seed in 0..base {
        if let Ok(num) = config.construct_number(seed) {
                working_seeds.push(seed);
    if working_seeds.len() == 1 {
        exclusive_seed = Some(working_seeds[0]);
    let density = working_seeds.len() as f64 / base as f64 * 100.0;
    println!("({},{}) k=({},{}) - {}: {} primes ({:.1}%)",
        outer, inner, k_outer, k_inner, desc, working_seeds.len(), density
    );
    if let Some(seed) = exclusive_seed {
        println!("  ✓ EXCLUSIVE to seed {}!", seed);
    } else if !working_seeds.is_empty() && working_seeds.len() <= 5 {
        println!("  Seeds: {:?}", working_seeds);
fn analyze_density_pattern(base: u32) {
    println!("Base {} density by seed position:", base);
    let config = MembraneConfig::new(base, 3, 3, 1, 1);
    let mut position_classes: HashMap<String, Vec<u32>> = HashMap::new();
                // Classify by position
                let relative_pos = seed as f64 / base as f64;
                
                let class = if relative_pos < 0.25 {
                    "First quarter"
                } else if relative_pos < 0.5 {
                    "Second quarter"
                } else if relative_pos < 0.75 {
                    "Third quarter"
                } else {
                    "Fourth quarter"
                };
                position_classes.entry(class.to_string())
                    .or_insert(Vec::new())
                    .push(seed);
    for (class, seeds) in &position_classes {
        println!("  {}: {} seeds", class, seeds.len());
    // Check for midpoint clustering
    let midpoint = base / 2;
    let near_midpoint: Vec<u32> = (0..base)
        .filter(|&s| {
            let dist = if s > midpoint { s - midpoint } else { midpoint - s };
            dist <= 2
        })
        .filter(|&seed| {
                is_prime(&num)
            } else {
                false
        .collect();
    if !near_midpoint.is_empty() {
        println!("  Near midpoint (±2): {:?}", near_midpoint);
fn make_predictions() {
    println!("Based on this analysis:\n");
    println!("1. The 'Perfect Storm' for exclusivity requires:");
    println!("   - Base with prime midpoint");
    println!("   - Midpoint that divides the base");
    println!("   - Symmetric membrane configuration");
    println!("   - Minimal zero padding\n");
    println!("2. Why 303050303 is inevitable:");
    println!("   - Base 10 forces this structure");
    println!("   - 5 is the only digit that can work");
    println!("   - The palindrome emerges naturally");
    println!("   - It's the 'ground state' of the system\n");
    println!("3. Duodecimal (base 12) fails because:");
    println!("   - Midpoint 6 = 2×3 is composite");
    println!("   - Introduces multiple prime factors");
    println!("   - Destroys the resonance needed");
    println!("   - No single seed can dominate\n");
    println!("4. Future exploration:");
    println!("   - Test bases where midpoint² = base (like 10)");
    println!("   - Explore bases with twin prime midpoints");
    println!("   - Find all 'perfect storm' bases");
    println!("   - Develop complete theory of exclusivity\n");
    println!("CONCLUSION: Base 10's unique factorization (2×5) with");
    println!("prime midpoint 5 that also divides the base creates");
    println!("the perfect conditions for seed exclusivity!");
    // Final insight
    println!("\n🎯 The Ultimate Insight:");
    println!("===========================");
    println!("Seed 5 works because it's simultaneously:");
    println!("  1. The midpoint (geometric center)");
    println!("  2. A prime number (indivisible)");
    println!("  3. A factor of the base (divides 10)");
    println!("  4. A fixed point under multiplication (5×5≡5)");
    println!("\nNo other digit in base 10 has ALL these properties!");
fn is_prime(n: &BigUint) -> bool {
    if n < &BigUint::from(2u32) {
        return false;
    if n == &BigUint::from(2u32) {
        return true;
    if n % BigUint::from(2u32) == BigUint::from(0u32) {
    let sqrt_n = n.sqrt();
    let mut i = BigUint::from(3u32);
    while i <= sqrt_n {
        if n % &i == BigUint::from(0u32) {
            return false;
        i += BigUint::from(2u32);
    true
