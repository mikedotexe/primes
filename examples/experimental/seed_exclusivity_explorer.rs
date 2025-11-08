//! Seed Exclusivity Explorer
//! =========================
//! 
//! Discovers membrane configurations that produce primes with
//! exactly ONE seed - the ultimate in selectivity!

use prime_physics_engine::membrane::MembraneConfig;
#[derive(Debug, Clone)]
struct ExclusiveConfig {
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    exclusive_seed: u32,
    prime: BigUint,
    digit_sum: u32,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Seed Exclusivity Explorer");
    println!("============================\n");
    
    println!("Searching for configurations that produce primes");
    println!("with EXACTLY ONE seed (ultimate selectivity)...\n");
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    let output_filename = format!("seed_exclusivity_{}.txt", timestamp);
    let mut output = File::create(&output_filename)?;
    writeln!(output, "Seed Exclusivity Explorer Results")?;
    writeln!(output, "=================================")?;
    writeln!(output, "Timestamp: {}", timestamp)?;
    writeln!(output)?;
    // Phase 1: Find all exclusive configurations
    println!("Phase 1: Discovering exclusive configurations...");
    let exclusive_configs = find_exclusive_configurations(&mut output)?;
    println!("Found {} exclusive configurations!\n", exclusive_configs.len());
    // Phase 2: Analyze patterns
    println!("Phase 2: Analyzing exclusivity patterns...");
    analyze_exclusivity_patterns(&exclusive_configs, &mut output)?;
    // Phase 3: Multi-base analysis
    println!("\nPhase 3: Testing exclusivity across bases...");
    test_cross_base_exclusivity(&exclusive_configs, &mut output)?;
    // Phase 4: The mystery of seed 5
    println!("\nPhase 4: Investigating the dominance of seed 5...");
    investigate_seed_5_mystery(&mut output)?;
    // Phase 5: Extreme exclusivity
    println!("\nPhase 5: Searching for extreme exclusivity...");
    find_extreme_exclusivity(&mut output)?;
    // Create verification file
    let verify_filename = format!("exclusive_primes_verify_{}.csv", timestamp);
    create_verification_file(&exclusive_configs, &verify_filename)?;
    println!("\n✅ Analysis complete!");
    println!("Results written to: {}", output_filename);
    println!("Verification data: {}", verify_filename);
    println!("\nTo verify: Use any primality test on the numbers in the CSV file");
    Ok(())
fn find_exclusive_configurations(output: &mut File) -> Result<Vec<ExclusiveConfig>, Box<dyn std::error::Error>> {
    writeln!(output, "Phase 1: Discovering Exclusive Configurations")?;
    writeln!(output, "=============================================")?;
    let mut exclusive_configs = Vec::new();
    // Test parameter space
    let boundaries = vec![1, 3, 5, 7, 9];
    let k_values = vec![1, 2, 3];
    for &outer in &boundaries {
        for &inner in &boundaries {
            for &k_outer in &k_values {
                for &k_inner in &k_values {
                    let config = MembraneConfig::new(10, outer, inner, k_outer, k_inner);
                    
                    // Test all single-digit seeds
                    let mut prime_seeds = Vec::new();
                    let mut primes_found = Vec::new();
                    for seed in 0..=9 {
                        if let Ok(num) = config.construct_number(seed) {
                            if is_prime(&num) {
                                prime_seeds.push(seed);
                                primes_found.push(num);
                            }
                        }
                    }
                    // Check if exactly one seed works
                    if prime_seeds.len() == 1 {
                        let seed = prime_seeds[0];
                        let prime = primes_found[0].clone();
                        let digit_sum = prime.to_string().chars()
                            .filter_map(|c| c.to_digit(10))
                            .sum();
                        
                        exclusive_configs.push(ExclusiveConfig {
                            outer,
                            inner,
                            k_outer,
                            k_inner,
                            exclusive_seed: seed,
                            prime: prime.clone(),
                            digit_sum,
                        });
                        writeln!(output, "Found: ({},{}) k=({},{}) → ONLY seed {} → {}", 
                            outer, inner, k_outer, k_inner, seed, prime)?;
                }
            }
        }
    }
    writeln!(output, "Total exclusive configurations found: {}", exclusive_configs.len())?;
    Ok(exclusive_configs)
fn analyze_exclusivity_patterns(
    configs: &[ExclusiveConfig], 
    output: &mut File
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(output, "Phase 2: Exclusivity Pattern Analysis")?;
    writeln!(output, "=====================================")?;
    // Group by exclusive seed
    let mut by_seed: HashMap<u32, Vec<&ExclusiveConfig>> = HashMap::new();
    for config in configs {
        by_seed.entry(config.exclusive_seed).or_default().push(config);
    writeln!(output, "Configurations grouped by exclusive seed:")?;
    for seed in 0..=9 {
        if let Some(configs) = by_seed.get(&seed) {
            writeln!(output, "Seed {}: {} configurations exclusively use this seed", 
                seed, configs.len())?;
            
            // Show a few examples
            for (i, config) in configs.iter().take(3).enumerate() {
                writeln!(output, "  {}) ({},{}) k=({},{}) → {}", 
                    i+1, config.outer, config.inner, config.k_outer, config.k_inner,
                    config.prime)?;
            if configs.len() > 3 {
                writeln!(output, "  ... and {} more", configs.len() - 3)?;
    // Analyze digit sum patterns
    writeln!(output, "Digit sum analysis of exclusive primes:")?;
    let mut sum_groups: HashMap<u32, Vec<&ExclusiveConfig>> = HashMap::new();
        sum_groups.entry(config.digit_sum).or_default().push(config);
    let mut sums: Vec<_> = sum_groups.keys().cloned().collect();
    sums.sort();
    for sum in sums.iter().take(5) {
        let configs = &sum_groups[sum];
        writeln!(output, "  Digit sum {}: {} primes", sum, configs.len())?;
    // Special patterns
    writeln!(output, "Special patterns discovered:")?;
    // Twin boundaries
    let twin_boundaries: Vec<_> = configs.iter()
        .filter(|c| c.outer == c.inner)
        .collect();
    writeln!(output, "  Twin boundaries (outer=inner): {} configs", twin_boundaries.len())?;
    // Symmetric k values
    let symmetric_k: Vec<_> = configs.iter()
        .filter(|c| c.k_outer == c.k_inner)
    writeln!(output, "  Symmetric k values: {} configs", symmetric_k.len())?;
    // The 3-7 connection
    let has_3_or_7: Vec<_> = configs.iter()
        .filter(|c| c.outer == 3 || c.outer == 7 || c.inner == 3 || c.inner == 7)
    writeln!(output, "  Contains 3 or 7: {} configs ({:.1}%)", 
        has_3_or_7.len(), 
        has_3_or_7.len() as f64 / configs.len() as f64 * 100.0)?;
fn test_cross_base_exclusivity(
    base10_configs: &[ExclusiveConfig],
    writeln!(output, "Phase 3: Cross-Base Exclusivity Analysis")?;
    writeln!(output, "========================================")?;
    // Test a few interesting exclusive configs in other bases
    let test_configs = base10_configs.iter()
        .filter(|c| c.exclusive_seed == 5) // Focus on seed 5 configs
        .take(3)
        .collect::<Vec<_>>();
    for config in test_configs {
        writeln!(output)?;
        writeln!(output, "Testing ({},{}) k=({},{}) across bases:", 
            config.outer, config.inner, config.k_outer, config.k_inner)?;
        writeln!(output, "  Base 10: Exclusive to seed {} → {}", 
            config.exclusive_seed, config.prime)?;
        
        // Test in other bases
        for base in [11, 12, 16] {
            let new_config = MembraneConfig::new(
                base, config.outer, config.inner, config.k_outer, config.k_inner
            );
            let mut working_seeds = Vec::new();
            for seed in 0..=9 {
                if let Ok(num) = new_config.construct_number(seed) {
                    if is_prime(&num) {
                        working_seeds.push(seed);
            match working_seeds.len() {
                0 => writeln!(output, "  Base {}: No seeds produce primes", base)?,
                1 => writeln!(output, "  Base {}: Exclusive to seed {} ({})", 
                    base, working_seeds[0], 
                    if working_seeds[0] == config.exclusive_seed { "SAME!" } else { "different" })?,
                n => writeln!(output, "  Base {}: {} seeds work (not exclusive)", base, n)?,
fn investigate_seed_5_mystery(output: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(output, "Phase 4: The Mystery of Seed 5")?;
    writeln!(output, "==============================")?;
    writeln!(output, "Seed 5 appears to dominate exclusivity. Why?")?;
    // Test hypothesis: 5 is special because it's the center of base 10
    writeln!(output, "Hypothesis: 5 is the center digit of base 10 (0-9)")?;
    // Find all configs where seed 5 is exclusive
    let mut seed_5_configs = Vec::new();
    for outer in [1, 3, 5, 7, 9] {
        for inner in [1, 3, 5, 7, 9] {
            for k_outer in 1..=3 {
                for k_inner in 1..=3 {
                    let mut prime_count = 0;
                    let mut seed_5_prime = None;
                                prime_count += 1;
                                if seed == 5 {
                                    seed_5_prime = Some(num);
                                }
                    if prime_count == 1 && seed_5_prime.is_some() {
                        seed_5_configs.push((outer, inner, k_outer, k_inner, seed_5_prime.unwrap()));
    writeln!(output, "Configurations where ONLY seed 5 produces a prime:")?;
    writeln!(output, "Found {} such configurations", seed_5_configs.len())?;
    // Look for patterns in these configs
    let mut pattern_counts = HashMap::new();
    for (outer, inner, k_outer, k_inner, prime) in &seed_5_configs {
        // Pattern 1: Twin boundaries
        if outer == inner {
            *pattern_counts.entry("Twin boundaries").or_insert(0) += 1;
        // Pattern 2: Contains 3
        if *outer == 3 || *inner == 3 {
            *pattern_counts.entry("Contains 3").or_insert(0) += 1;
        // Pattern 3: k_outer = k_inner = 1
        if *k_outer == 1 && *k_inner == 1 {
            *pattern_counts.entry("Minimal k (1,1)").or_insert(0) += 1;
        // Pattern 4: Prime ends in 3
        if prime.to_string().ends_with('3') {
            *pattern_counts.entry("Prime ends in 3").or_insert(0) += 1;
    writeln!(output, "Patterns in seed-5-exclusive configurations:")?;
    for (pattern, count) in pattern_counts {
        writeln!(output, "  {}: {} ({:.1}%)", 
            pattern, count, count as f64 / seed_5_configs.len() as f64 * 100.0)?;
    // Show some examples
    writeln!(output, "Examples of seed-5-exclusive primes:")?;
    for (outer, inner, k_outer, k_inner, prime) in seed_5_configs.iter().take(5) {
        writeln!(output, "  ({},{}) k=({},{}) → {}", outer, inner, k_outer, k_inner, prime)?;
fn find_extreme_exclusivity(output: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(output, "Phase 5: Extreme Exclusivity Search")?;
    writeln!(output, "===================================")?;
    writeln!(output, "Searching for configurations with extreme selectivity...")?;
    // Test with 2-digit seeds (00-99)
    writeln!(output, "Testing 100 seeds (00-99) for ultra-exclusive configs:")?;
    let mut ultra_exclusive = Vec::new();
    // Focus on promising configurations
    let test_configs = vec![
        (3, 3, 1, 1),
        (5, 5, 1, 1),
        (7, 7, 1, 1),
        (3, 7, 1, 1),
        (1, 9, 1, 1),
    ];
    for (outer, inner, k_outer, k_inner) in test_configs {
        let config = MembraneConfig::new(10, outer, inner, k_outer, k_inner);
        let mut working_seeds = Vec::new();
        for seed in 0..100 {
            if let Ok(num) = config.construct_number(seed) {
                if is_prime(&num) && working_seeds.len() < 5 {
                    working_seeds.push((seed, num));
        if working_seeds.len() == 1 {
            ultra_exclusive.push((outer, inner, k_outer, k_inner, working_seeds[0].clone()));
        writeln!(output, "({},{}) k=({},{}) → {} seeds work out of 100", 
            outer, inner, k_outer, k_inner, working_seeds.len())?;
        if working_seeds.len() <= 3 && !working_seeds.is_empty() {
            writeln!(output, "  Seeds: {:?}", 
                working_seeds.iter().map(|(s, _)| s).collect::<Vec<_>>())?;
    if !ultra_exclusive.is_empty() {
        writeln!(output, "🏆 ULTRA-EXCLUSIVE: Only 1 seed in 100 works!")?;
        for (outer, inner, k_outer, k_inner, (seed, prime)) in ultra_exclusive {
            writeln!(output, "  ({},{}) k=({},{}) → ONLY seed {} → {}", 
                outer, inner, k_outer, k_inner, seed, prime)?;
    // The ultimate test: 1000 seeds
    writeln!(output, "Ultimate exclusivity test: 1000 seeds (000-999):")?;
    let config = MembraneConfig::new(10, 3, 3, 1, 1); // Our most selective config
    let mut count = 0;
    let mut examples = Vec::new();
    for seed in 0..1000 {
        if let Ok(num) = config.construct_number(seed) {
            if is_prime(&num) {
                count += 1;
                if examples.len() < 3 {
                    examples.push((seed, num));
    writeln!(output, "Configuration (3,3) k=(1,1) with 1000 seeds:")?;
    writeln!(output, "  Success rate: {}/1000 = {:.2}%", count, count as f64 / 10.0)?;
    writeln!(output, "  Examples: {:?}", examples)?;
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
fn create_verification_file(
    filename: &str
    let mut file = File::create(filename)?;
    // CSV header
    writeln!(file, "configuration,outer,inner,k_outer,k_inner,exclusive_seed,prime_number,digit_count,digit_sum,verification_url")?;
        let prime_str = config.prime.to_string();
        let digit_count = prime_str.len();
        // Create Wolfram Alpha verification URL
        let wolfram_url = format!(
            "https://www.wolframalpha.com/input?i=is+{}+prime",
            prime_str
        );
        writeln!(file, "\"({},{})_k({},{})\",{},{},{},{},{},{},{},{},\"{}\"",
            config.outer, config.inner, config.k_outer, config.k_inner,
            config.outer,
            config.inner,
            config.k_outer,
            config.k_inner,
            config.exclusive_seed,
            prime_str,
            digit_count,
            config.digit_sum,
            wolfram_url
        )?;
    writeln!(file)?;
    writeln!(file, "# To verify these primes:")?;
    writeln!(file, "# 1. Use any primality testing tool or library")?;
    writeln!(file, "# 2. Click the Wolfram Alpha URLs for online verification")?;
    writeln!(file, "# 3. All numbers listed should be confirmed as prime")?;
    writeln!(file, "# 4. Each configuration should produce a prime ONLY with the listed seed")?;
