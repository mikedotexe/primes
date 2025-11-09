//! Quick Seed Exclusivity Discovery
//! =================================
//! 
//! Focused exploration of the most interesting exclusive configurations

use primes::membrane::MembraneConfig;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Quick Seed Exclusivity Discovery");
    println!("===================================\n");
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    // Focus on the most promising configurations
    println!("Testing known exclusive patterns...\n");
    let mut all_exclusives = Vec::new();
    // Pattern 1: Twin boundaries with k=(1,1)
    println!("Pattern 1: Twin boundaries with minimal k");
    let twin_configs = vec![
        (3, 3, 1, 1),
        (5, 5, 1, 1),
        (7, 7, 1, 1),
        (9, 9, 1, 1),
    ];
    for (outer, inner, k_outer, k_inner) in twin_configs {
        test_exclusivity(outer, inner, k_outer, k_inner, &mut all_exclusives);
    }
    // Pattern 2: Classic (3,7) with different k values
    println!("\nPattern 2: The (3,7) family");
    for k in 1..=3 {
        test_exclusivity(3, 7, k, k, &mut all_exclusives);
        test_exclusivity(7, 3, k, k, &mut all_exclusives);
    // Pattern 3: Asymmetric k values
    println!("\nPattern 3: Asymmetric k values");
    let asymmetric = vec![
        (3, 3, 1, 2),
        (3, 3, 2, 1),
        (5, 5, 1, 2),
        (3, 7, 1, 2),
    for (outer, inner, k_outer, k_inner) in asymmetric {
    // Create output files
    let summary_file = format!("seed_exclusivity_summary_{}.txt", timestamp);
    let csv_file = format!("exclusive_primes_{}.csv", timestamp);
    write_summary(&all_exclusives, &summary_file)?;
    write_csv(&all_exclusives, &csv_file)?;
    println!("\n✅ Discovery complete!");
    println!("Found {} exclusive configurations", all_exclusives.len());
    println!("\nFiles created:");
    println!("  Summary: {}", summary_file);
    println!("  CSV data: {}", csv_file);
    println!("\nSome fascinating examples:");
    // Show a few interesting ones
    for (i, (config, seed, prime)) in all_exclusives.iter().take(5).enumerate() {
        println!("  {}) {} → ONLY seed {} → {}", 
            i+1, format_config(config), seed, prime);
    if all_exclusives.len() > 5 {
        println!("  ... and {} more!", all_exclusives.len() - 5);
    Ok(())
}
fn test_exclusivity(
    outer: u32, 
    inner: u32, 
    k_outer: u32, 
    k_inner: u32,
    results: &mut Vec<((u32, u32, u32, u32), u32, BigUint)>
) {
    let config = MembraneConfig::new(10, outer, inner, k_outer, k_inner);
    let mut working_seeds = Vec::new();
    let mut primes = Vec::new();
    // Test single-digit seeds
    for seed in 0..=9 {
        if let Ok(num) = config.construct_number(seed) {
            if is_prime(&num) {
                working_seeds.push(seed);
                primes.push(num);
            }
        }
    if working_seeds.len() == 1 {
        let seed = working_seeds[0];
        let prime = primes[0].clone();
        
        println!("  ✓ ({},{}) k=({},{}) → ONLY seed {} → {}", 
            outer, inner, k_outer, k_inner, seed, prime);
        results.push(((outer, inner, k_outer, k_inner), seed, prime));
    } else if working_seeds.is_empty() {
        println!("  ✗ ({},{}) k=({},{}) → No primes", 
            outer, inner, k_outer, k_inner);
    } else {
        println!("  ~ ({},{}) k=({},{}) → {} seeds work (not exclusive)", 
            outer, inner, k_outer, k_inner, working_seeds.len());
fn write_summary(
    exclusives: &[((u32, u32, u32, u32), u32, BigUint)],
    filename: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    writeln!(file, "Seed Exclusivity Discovery Summary")?;
    writeln!(file, "==================================")?;
    writeln!(file)?;
    writeln!(file, "Found {} configurations that produce primes with EXACTLY ONE seed", 
        exclusives.len())?;
    // Group by exclusive seed
    let mut by_seed = std::collections::HashMap::new();
    for (config, seed, prime) in exclusives {
        by_seed.entry(*seed).or_insert_with(Vec::new).push((config, prime));
    writeln!(file, "Distribution by exclusive seed:")?;
        if let Some(configs) = by_seed.get(&seed) {
            writeln!(file, "  Seed {}: {} configurations", seed, configs.len())?;
    writeln!(file, "All exclusive configurations:")?;
        writeln!(file, "{} → seed {} → {}", 
            format_config(config), seed, prime)?;
        // Add some analysis
        let prime_str = prime.to_string();
        writeln!(file, "  Length: {} digits", prime_str.len())?;
        writeln!(file, "  Digit sum: {}", 
            prime_str.chars().filter_map(|c| c.to_digit(10)).sum::<u32>())?;
        writeln!(file)?;
fn write_csv(
    writeln!(file, "configuration,outer,inner,k_outer,k_inner,exclusive_seed,prime_number,digit_count,wolfram_url")?;
    for ((outer, inner, k_outer, k_inner), seed, prime) in exclusives {
        let wolfram_url = format!(
            "https://www.wolframalpha.com/input?i=is+{}+prime",
            prime_str
        );
        writeln!(file, "\"({},{})_k({},{})\",{},{},{},{},{},{},{},\"{}\"",
            outer, inner, k_outer, k_inner,
            seed,
            prime_str,
            prime_str.len(),
            wolfram_url
        )?;
fn format_config(config: &(u32, u32, u32, u32)) -> String {
    format!("({},{}) k=({},{})", config.0, config.1, config.2, config.3)
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
