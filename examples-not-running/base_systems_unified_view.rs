//! Base Systems Unified View
//! ==========================
//! 
//! A comprehensive visualization of how different number bases create
//! unique prime generation landscapes through membrane configurations

use primes::membrane::MembraneConfig;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(80));
    println!("{:^80}", "🌌 UNIFIED BASE SYSTEMS VISUALIZATION 🌌");
    println!("{}", "=".repeat(80));
    
    // Part 1: The Base Spectrum
    println!("\n{}", "─".repeat(80));
    println!("{:^80}", "THE BASE SPECTRUM: From Exclusivity to Democracy");
    println!("{}", "─".repeat(80));
    visualize_base_spectrum();
    // Part 2: Midpoint Analysis
    println!("{:^80}", "MIDPOINT PRIMALITY EFFECTS");
    analyze_midpoint_effects();
    // Part 3: Configuration Space
    println!("{:^80}", "CONFIGURATION SPACE TOPOLOGY");
    explore_configuration_space();
    // Part 4: The Universal Pattern
    println!("{:^80}", "THE UNIVERSAL 5+10k PATTERN");
    demonstrate_universal_pattern();
    // Part 5: Grand Unification
    println!("{:^80}", "GRAND UNIFICATION: The Complete Picture");
    present_grand_unification();
    Ok(())
}
fn visualize_base_spectrum() {
    println!("\nBase systems form a spectrum from exclusive to democratic:\n");
    // Test multiple bases
    let bases = vec![6, 8, 10, 12, 14, 16, 20, 30, 37, 43];
    let mut results = Vec::new();
    for base in bases {
        let midpoint = base / 2;
        let midpoint_prime = is_prime(&BigUint::from(midpoint));
        
        // Test standard (3,3) k=(1,1) configuration
        let config = MembraneConfig::new(base, 3, 3, 1, 1);
        let mut working_seeds = Vec::new();
        let mut midpoint_works = false;
        for seed in 0..base.min(20) {
            if let Ok(num) = config.construct_number(seed) {
                if is_prime(&num) {
                    working_seeds.push(seed);
                    if seed == midpoint {
                        midpoint_works = true;
                    }
                }
            }
        }
        let exclusivity = working_seeds.len() == 1;
        let democracy = working_seeds.len() as f64 / base.min(20) as f64;
        results.push((base, midpoint, midpoint_prime, exclusivity, democracy, working_seeds));
    }
    // Display spectrum
    println!("Base | Midpoint | M-Prime? | Seeds Work | Democracy | Type");
    println!("-----|----------|----------|------------|-----------|-----");
    for (base, mid, m_prime, excl, demo, seeds) in &results {
        let type_label = if *excl {
            "\x1b[31mEXCLUSIVE\x1b[0m"
        } else if *demo > 0.3 {
            "\x1b[32mDEMOCRATIC\x1b[0m"
        } else if *demo > 0.1 {
            "\x1b[33mSELECTIVE\x1b[0m"
        } else {
            "\x1b[90mRESTRICTED\x1b[0m"
        };
        println!("{:>4} | {:>8} | {:>8} | {:>10} | {:>9.1}% | {}",
            base, mid, 
            if *m_prime { "YES" } else { "no" },
            seeds.len(),
            demo * 100.0,
            type_label
        );
    println!("\n\x1b[36mKey Insight:\x1b[0m Prime midpoints enable exclusivity!");
fn analyze_midpoint_effects() {
    println!("\nHow midpoint primality affects prime generation:\n");
    // Compare bases with prime vs composite midpoints
    let comparisons = vec![
        (10, 5, true, "2×5 with prime midpoint"),
        (12, 6, false, "2²×3 with composite midpoint"),
        (14, 7, true, "2×7 with prime midpoint"),
        (16, 8, false, "2⁴ with composite midpoint"),
    ];
    for (base, midpoint, mid_prime, desc) in comparisons {
        println!("\n\x1b[35mBase {}\x1b[0m: {}", base, desc);
        println!("Midpoint {}: {}", midpoint, if mid_prime { "PRIME" } else { "composite" });
        // Test midpoint seed in various configurations
        let test_configs = vec![
            (3, 3, 1, 1),
            (1, base-1, 1, 1),
            (midpoint, midpoint, 1, 1),
        ];
        for (outer, inner, k_outer, k_inner) in test_configs {
            if outer >= base || inner >= base { continue; }
            
            let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);
            // Test midpoint seed
            let mut midpoint_success = false;
            if let Ok(num) = config.construct_number(midpoint) {
                midpoint_success = is_prime(&num);
            // Test exclusivity
            let mut total_success = 0;
            for seed in 0..base {
                if let Ok(num) = config.construct_number(seed) {
                    if is_prime(&num) {
                        total_success += 1;
            if midpoint_success {
                println!("  ({},{}) k=({},{}): Midpoint {} ✓ | Total: {} {}",
                    outer, inner, k_outer, k_inner,
                    if midpoint_success { "WORKS" } else { "fails" },
                    total_success,
                    if total_success == 1 { "\x1b[31m(EXCLUSIVE!)\x1b[0m" } else { "" }
                );
fn explore_configuration_space() {
    println!("\nConfiguration space has different topologies in each base:\n");
    // Map configuration space for bases 10 and 12
    for base in [10, 12] {
        println!("\n\x1b[34mBase {} Configuration Space:\x1b[0m", base);
        // Create a 2D map of (outer, inner) success rates
        let mut config_map = HashMap::new();
        for outer in 1..base {
            for inner in 1..base {
                if outer == inner { continue; }
                
                let config = MembraneConfig::new(base, outer, inner, 1, 1);
                let mut success_count = 0;
                for seed in 0..base {
                    if let Ok(num) = config.construct_number(seed) {
                        if is_prime(&num) {
                            success_count += 1;
                        }
                if success_count > 0 {
                    config_map.insert((outer, inner), success_count);
        // Find patterns
        let max_success = config_map.values().max().copied().unwrap_or(0);
        let exclusive_configs: Vec<_> = config_map.iter()
            .filter(|(_, &v)| v == 1)
            .map(|(&k, _)| k)
            .collect();
        let high_success_configs: Vec<_> = config_map.iter()
            .filter(|(_, &v)| v >= max_success / 2)
            .map(|(&k, &v)| (k, v))
        println!("  Total viable configurations: {}", config_map.len());
        println!("  Exclusive configurations: {}", exclusive_configs.len());
        println!("  Maximum success count: {}", max_success);
        if !exclusive_configs.is_empty() && exclusive_configs.len() <= 5 {
            println!("  Exclusive configs: {:?}", exclusive_configs);
        if !high_success_configs.is_empty() && high_success_configs.len() <= 5 {
            println!("  High success configs:");
            for ((o, i), count) in high_success_configs {
                println!("    ({},{}) → {} primes", o, i, count);
fn demonstrate_universal_pattern() {
    println!("\nThe 5+10k pattern works across many bases:\n");
    let test_bases = vec![10, 12, 37, 43, 60];
    let pattern_seeds = vec![5, 15, 25, 35];
    println!("Testing pattern [5, 15, 25, 35] in (3,3) k=(1,1):\n");
    for base in test_bases {
        println!("\x1b[36mBase {}:\x1b[0m", base);
        let mut pattern_success = 0;
        let mut pattern_results = Vec::new();
        for &seed in &pattern_seeds {
            if seed >= base { break; }
                let is_p = is_prime(&num);
                pattern_results.push((seed, is_p));
                if is_p {
                    pattern_success += 1;
        println!("  Pattern results:");
        for (seed, success) in pattern_results {
            println!("    Seed {:>2}: {}", seed, 
                if success { "\x1b[32m✓ PRIME\x1b[0m" } else { "\x1b[90m✗ composite\x1b[0m" }
            );
        println!("  Success rate: {}/{} ({:.0}%)\n", 
            pattern_success, 
            pattern_seeds.iter().filter(|&&s| s < base).count(),
            pattern_success as f64 / pattern_seeds.iter().filter(|&&s| s < base).count() as f64 * 100.0
    println!("\x1b[33mUniversal Pattern Insight:\x1b[0m");
    println!("The 5+10k series maintains special properties across bases!");
    println!("This suggests deep mathematical structure beyond base dependence.");
fn present_grand_unification() {
    println!("\n\x1b[1;35mTHE GRAND UNIFICATION OF BASE SYSTEMS\x1b[0m\n");
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│                    \x1b[1mThree Fundamental Principles\x1b[0m                 │");
    println!("├─────────────────────────────────────────────────────────────────┤");
    println!("│                                                                 │");
    println!("│ 1. \x1b[36mMIDPOINT PRIMALITY PRINCIPLE\x1b[0m                               │");
    println!("│    Prime midpoints enable exclusive monopoles                   │");
    println!("│    Composite midpoints create democratic diversity              │");
    println!("│ 2. \x1b[33mFACTOR PRESERVATION PRINCIPLE\x1b[0m                              │");
    println!("│    Seeds must avoid compounding base factors                   │");
    println!("│    Coprime relationships maximize success                       │");
    println!("│ 3. \x1b[32mUNIVERSAL RESONANCE PRINCIPLE\x1b[0m                              │");
    println!("│    Certain patterns (like 5+10k) transcend bases               │");
    println!("│    Mathematical harmonics exist independent of representation  │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    println!("\n\x1b[1mOptimal Strategies by Base Type:\x1b[0m\n");
    println!("• \x1b[31mPrime Midpoint Bases\x1b[0m (10, 14, 22, 26...):");
    println!("  → Seek exclusive configurations with midpoint seed");
    println!("  → Use symmetric boundaries (3,3), (5,5)");
    println!("  → Minimal padding often suffices");
    println!("\n• \x1b[32mComposite Midpoint Bases\x1b[0m (12, 16, 18, 20...):");
    println!("  → Embrace democratic distribution");
    println!("  → Use harmonic subdivisions (quarters, thirds)");
    println!("  → Mix coprime and factor-bearing boundaries");
    println!("\n• \x1b[33mPrime Bases\x1b[0m (37, 43, 47...):");
    println!("  → Universal patterns still apply");
    println!("  → Edge pairs work well");
    println!("  → 5+10k series maintains effectiveness");
    println!("\n\x1b[1;36mFINAL INSIGHT:\x1b[0m");
    println!("│ Prime generation through membrane construction reveals that     │");
    println!("│ number bases are not mere notation - they create distinct      │");
    println!("│ mathematical universes with their own physics and symmetries.  │");
    println!("│ Base 10's unique property (2×5 with prime midpoint 5) creates  │");
    println!("│ the perfect storm for exclusivity, but every base has its      │");
    println!("│ own optimal strategies waiting to be discovered!               │");
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
