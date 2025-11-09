//! Multi-Base Configuration Optimizer
//! ==================================
//! 
//! Finds optimal membrane configurations for different number bases
//! Based on verified findings about base-specific strategies

use primes::membrane::MembraneConfig;
use num_bigint::BigUint;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
#[derive(Debug, Clone)]
struct BaseStrategy {
    base: u32,
    base_properties: BaseProperties,
    optimal_configs: Vec<ConfigResult>,
    edge_pairs: Vec<(u32, u32)>,
    midpoint_analysis: MidpointInfo,
    success_metrics: SuccessMetrics,
}
struct BaseProperties {
    factorization: Vec<u32>,
    is_prime: bool,
    is_even: bool,
    midpoint: f64,
    quarter_points: (f64, f64),
struct ConfigResult {
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    primes_found: Vec<(u32, BigUint)>,
    success_rate: f64,
    avg_prime_length: f64,
    working_seeds: Vec<u32>,
struct MidpointInfo {
    midpoint_floor: u32,
    midpoint_ceil: u32,
    floor_is_prime: bool,
    ceil_is_prime: bool,
    midpoint_divides_base: bool,
struct SuccessMetrics {
    total_configs_tested: usize,
    productive_configs: usize,
    total_primes_found: usize,
    best_success_rate: f64,
    avg_success_rate: f64,
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 MULTI-BASE CONFIGURATION OPTIMIZER");
    println!("====================================\n");
    
    let mut csv_output = File::create("multi_base_optimization_results.csv")?;
    writeln!(csv_output, "Base,Factorization,IsPrime,IsEven,Midpoint,BestConfig,BestRate,TotalPrimes,EdgePairs,Examples,URLs")?;
    let mut strategy_output = File::create("base_strategies.json")?;
    writeln!(strategy_output, "{{")?;
    // Test a variety of bases
    let test_bases = vec![
        6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 24, 30, 37, 43, 60
    ];
    let mut all_strategies = Vec::new();
    for (i, &base) in test_bases.iter().enumerate() {
        println!("Analyzing Base {}...", base);
        let strategy = analyze_base(base)?;
        
        // Display key findings
        println!("  Properties: {:?}", strategy.base_properties.factorization);
        println!("  Midpoint: {:.1} (floor={}, ceil={})", 
            strategy.base_properties.midpoint,
            strategy.midpoint_analysis.midpoint_floor,
            strategy.midpoint_analysis.midpoint_ceil);
        if let Some(best) = strategy.optimal_configs.first() {
            println!("  Best config: ({},{}) k=({},{}) → {:.1}% success",
                best.outer, best.inner, best.k_outer, best.k_inner,
                best.success_rate * 100.0);
            
            if !best.primes_found.is_empty() {
                println!("  Example prime: {}", best.primes_found[0].1);
            }
        }
        println!("  Edge pairs: {:?}", strategy.edge_pairs);
        println!("  Success metrics: {}/{} configs productive, {} total primes",
            strategy.success_metrics.productive_configs,
            strategy.success_metrics.total_configs_tested,
            strategy.success_metrics.total_primes_found);
        write_to_csv(&mut csv_output, &strategy)?;
        write_strategy_json(&mut strategy_output, &strategy, i == 0)?;
        all_strategies.push(strategy);
        println!();
    }
    writeln!(strategy_output, "}}")?;
    // Comparative analysis
    println!("\n📊 COMPARATIVE ANALYSIS");
    println!("=======================\n");
    analyze_base_patterns(&all_strategies);
    // Find universal configurations
    println!("\n🌟 UNIVERSAL CONFIGURATIONS");
    println!("===========================\n");
    find_universal_configs(&all_strategies);
    // Base recommendations
    println!("\n💡 BASE-SPECIFIC RECOMMENDATIONS");
    println!("=================================\n");
    generate_recommendations(&all_strategies);
    println!("\n✅ Results saved to:");
    println!("  - multi_base_optimization_results.csv");
    println!("  - base_strategies.json");
    Ok(())
fn analyze_base(base: u32) -> Result<BaseStrategy, Box<dyn std::error::Error>> {
    let base_props = analyze_base_properties(base);
    let midpoint_info = analyze_midpoint(base, &base_props);
    let edge_pairs = find_edge_pairs(base);
    let mut all_configs = Vec::new();
    let mut total_primes = 0;
    // Test comprehensive configuration space
    let digit_range: Vec<u32> = (1..base).collect();
    let k_range = vec![0, 1, 2, 3];
    for &outer in &digit_range {
        for &inner in &digit_range {
            // Skip if both are even in even base (likely dead)
            if base % 2 == 0 && outer % 2 == 0 && inner % 2 == 0 {
                continue;
            for &k_outer in &k_range {
                for &k_inner in &k_range {
                    match test_configuration(base, outer, inner, k_outer, k_inner) {
                        Ok(result) => {
                            if !result.primes_found.is_empty() {
                                total_primes += result.primes_found.len();
                                all_configs.push(result);
                            }
                        }
                        Err(_) => continue,
                    }
                }
    // Sort by success rate
    all_configs.sort_by(|a, b| b.success_rate.partial_cmp(&a.success_rate).unwrap());
    // Calculate metrics
    let total_tested = digit_range.len() * digit_range.len() * k_range.len() * k_range.len();
    let avg_success = if all_configs.is_empty() {
        0.0
    } else {
        all_configs.iter().map(|c| c.success_rate).sum::<f64>() / all_configs.len() as f64
    };
    let best_rate = all_configs.first().map(|c| c.success_rate).unwrap_or(0.0);
    let metrics = SuccessMetrics {
        total_configs_tested: total_tested,
        productive_configs: all_configs.len(),
        total_primes_found: total_primes,
        best_success_rate: best_rate,
        avg_success_rate: avg_success,
    // Take top 10 configurations
    let optimal_configs = all_configs.into_iter().take(10).collect();
    Ok(BaseStrategy {
        base,
        base_properties: base_props,
        optimal_configs,
        edge_pairs,
        midpoint_analysis: midpoint_info,
        success_metrics: metrics,
    })
fn analyze_base_properties(base: u32) -> BaseProperties {
    let factorization = factorize(base);
    let is_prime = factorization.len() == 1 && factorization[0] == base;
    let is_even = base % 2 == 0;
    let midpoint = base as f64 / 2.0;
    let quarter_points = (base as f64 / 4.0, base as f64 * 3.0 / 4.0);
    BaseProperties {
        factorization,
        is_prime,
        is_even,
        midpoint,
        quarter_points,
fn analyze_midpoint(base: u32, props: &BaseProperties) -> MidpointInfo {
    let midpoint_floor = props.midpoint.floor() as u32;
    let midpoint_ceil = props.midpoint.ceil() as u32;
    MidpointInfo {
        midpoint_floor,
        midpoint_ceil,
        floor_is_prime: is_prime(&BigUint::from(midpoint_floor)),
        ceil_is_prime: is_prime(&BigUint::from(midpoint_ceil)),
        midpoint_divides_base: base % midpoint_floor == 0 || 
                               (midpoint_ceil != midpoint_floor && base % midpoint_ceil == 0),
fn find_edge_pairs(base: u32) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();
    // Find digits equidistant from boundaries
    for d1 in 1..base {
        let dist_from_zero = d1;
        let dist_from_base = base - d1;
        // Find complement
        for d2 in 1..base {
            if d2 != d1 && dist_from_zero == base - d2 {
                pairs.push((d1.min(d2), d1.max(d2)));
    // Remove duplicates
    pairs.sort();
    pairs.dedup();
    pairs
fn factorize(n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut num = n;
    let mut d = 2;
    while d * d <= num {
        while num % d == 0 {
            factors.push(d);
            num /= d;
        d += 1;
    if num > 1 {
        factors.push(num);
    factors
fn test_configuration(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32)
    -> Result<ConfigResult, Box<dyn std::error::Error>> {
    let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);
    let mut primes_found = Vec::new();
    let mut working_seeds = Vec::new();
    let mut total_length = 0;
    // Test single-digit seeds
    for seed in 0..base.min(10) {
        match config.construct_number(seed) {
            Ok(num) => {
                if is_prime(&num) {
                    let length = num.to_string().len();
                    total_length += length;
                    working_seeds.push(seed);
                    primes_found.push((seed, num));
            Err(_) => {}
    let success_rate = primes_found.len() as f64 / base.min(10) as f64;
    let avg_length = if !primes_found.is_empty() {
        total_length as f64 / primes_found.len() as f64
    Ok(ConfigResult {
        outer,
        inner,
        k_outer,
        k_inner,
        primes_found,
        success_rate,
        avg_prime_length: avg_length,
        working_seeds,
fn write_to_csv(output: &mut File, strategy: &BaseStrategy) 
    -> Result<(), Box<dyn std::error::Error>> {
    let factors_str = strategy.base_properties.factorization.iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join("×");
    let best_config_str = if let Some(best) = strategy.optimal_configs.first() {
        format!("({},{},{},{})", best.outer, best.inner, best.k_outer, best.k_inner)
        "None".to_string()
    let best_rate = strategy.success_metrics.best_success_rate;
    let edge_pairs_str = strategy.edge_pairs.iter()
        .map(|(a, b)| format!("({},{})", a, b))
        .join(";");
    let examples = strategy.optimal_configs.iter()
        .flat_map(|c| &c.primes_found)
        .take(2)
        .map(|(s, p)| format!("s{}={}", s, p))
    let urls = strategy.optimal_configs.iter()
        .map(|(_, p)| format!("https://www.wolframalpha.com/input?i=is+{}+prime", p))
    writeln!(output, "{},\"{}\",{},{},{:.1},\"{}\",{:.3},{},\"{}\",\"{}\",\"{}\"",
        strategy.base,
        factors_str,
        strategy.base_properties.is_prime,
        strategy.base_properties.is_even,
        strategy.base_properties.midpoint,
        best_config_str,
        best_rate,
        strategy.success_metrics.total_primes_found,
        edge_pairs_str,
        examples,
        urls
    )?;
fn write_strategy_json(output: &mut File, strategy: &BaseStrategy, first: bool) 
    if !first {
        writeln!(output, ",")?;
    write!(output, "  \"{}\": {{", strategy.base)?;
    write!(output, "\n    \"properties\": {{")?;
    write!(output, "\n      \"factorization\": {:?},", strategy.base_properties.factorization)?;
    write!(output, "\n      \"is_prime\": {},", strategy.base_properties.is_prime)?;
    write!(output, "\n      \"is_even\": {},", strategy.base_properties.is_even)?;
    write!(output, "\n      \"midpoint\": {:.1}", strategy.base_properties.midpoint)?;
    write!(output, "\n    }},")?;
    write!(output, "\n    \"midpoint_analysis\": {{")?;
    write!(output, "\n      \"floor\": {},", strategy.midpoint_analysis.midpoint_floor)?;
    write!(output, "\n      \"ceil\": {},", strategy.midpoint_analysis.midpoint_ceil)?;
    write!(output, "\n      \"floor_is_prime\": {},", strategy.midpoint_analysis.floor_is_prime)?;
    write!(output, "\n      \"ceil_is_prime\": {},", strategy.midpoint_analysis.ceil_is_prime)?;
    write!(output, "\n      \"divides_base\": {}", strategy.midpoint_analysis.midpoint_divides_base)?;
    write!(output, "\n    \"edge_pairs\": {:?},", strategy.edge_pairs)?;
    write!(output, "\n    \"success_metrics\": {{")?;
    write!(output, "\n      \"configs_tested\": {},", strategy.success_metrics.total_configs_tested)?;
    write!(output, "\n      \"productive_configs\": {},", strategy.success_metrics.productive_configs)?;
    write!(output, "\n      \"total_primes\": {},", strategy.success_metrics.total_primes_found)?;
    write!(output, "\n      \"best_rate\": {:.3},", strategy.success_metrics.best_success_rate)?;
    write!(output, "\n      \"avg_rate\": {:.3}", strategy.success_metrics.avg_success_rate)?;
    write!(output, "\n    \"top_configs\": [")?;
    for (i, config) in strategy.optimal_configs.iter().take(3).enumerate() {
        if i > 0 { write!(output, ",")?; }
        write!(output, "\n      {{")?;
        write!(output, "\"config\": [{},{},{},{}], ", 
            config.outer, config.inner, config.k_outer, config.k_inner)?;
        write!(output, "\"rate\": {:.3}, ", config.success_rate)?;
        write!(output, "\"seeds\": {:?}", config.working_seeds)?;
        write!(output, "}}")?;
    write!(output, "\n    ]")?;
    write!(output, "\n  }}")?;
fn analyze_base_patterns(strategies: &[BaseStrategy]) {
    // Group by properties
    let mut by_parity: HashMap<&str, Vec<&BaseStrategy>> = HashMap::new();
    let mut by_primality: HashMap<&str, Vec<&BaseStrategy>> = HashMap::new();
    for strategy in strategies {
        let parity = if strategy.base_properties.is_even { "Even" } else { "Odd" };
        let primality = if strategy.base_properties.is_prime { "Prime" } else { "Composite" };
        by_parity.entry(parity).or_insert_with(Vec::new).push(strategy);
        by_primality.entry(primality).or_insert_with(Vec::new).push(strategy);
    println!("Success by base parity:");
    for (parity, strats) in by_parity {
        let avg_best = strats.iter()
            .map(|s| s.success_metrics.best_success_rate)
            .sum::<f64>() / strats.len() as f64;
        let total_primes: usize = strats.iter()
            .map(|s| s.success_metrics.total_primes_found)
            .sum();
        println!("  {} bases: {:.1}% avg best rate, {} total primes",
            parity, avg_best * 100.0, total_primes);
    println!("\nSuccess by base primality:");
    for (primality, strats) in by_primality {
        println!("  {} bases: {:.1}% avg best rate",
            primality, avg_best * 100.0);
    // Midpoint analysis
    println!("\nMidpoint primality effects:");
    let with_prime_midpoint: Vec<_> = strategies.iter()
        .filter(|s| s.midpoint_analysis.floor_is_prime || s.midpoint_analysis.ceil_is_prime)
        .collect();
    if !with_prime_midpoint.is_empty() {
        let avg_success = with_prime_midpoint.iter()
            .sum::<f64>() / with_prime_midpoint.len() as f64;
        println!("  Bases with prime midpoint: {} ({:.1}% avg best rate)",
            with_prime_midpoint.len(), avg_success * 100.0);
        for strategy in with_prime_midpoint {
            println!("    Base {}: midpoint {:.1} (floor={} prime:{}, ceil={} prime:{})",
                strategy.base,
                strategy.base_properties.midpoint,
                strategy.midpoint_analysis.midpoint_floor,
                strategy.midpoint_analysis.floor_is_prime,
                strategy.midpoint_analysis.midpoint_ceil,
                strategy.midpoint_analysis.ceil_is_prime);
fn find_universal_configs(strategies: &[BaseStrategy]) {
    // Find configurations that work across multiple bases
    let mut config_appearances: HashMap<(u32, u32, u32, u32), Vec<u32>> = HashMap::new();
        for config in &strategy.optimal_configs {
            let key = (config.outer, config.inner, config.k_outer, config.k_inner);
            config_appearances.entry(key).or_insert_with(Vec::new).push(strategy.base);
    // Find configs that work in 3+ bases
    let universal: Vec<_> = config_appearances.iter()
        .filter(|(_, bases)| bases.len() >= 3)
    if !universal.is_empty() {
        println!("Configurations working in 3+ bases:");
        for ((outer, inner, k_outer, k_inner), bases) in universal {
            println!("  ({},{}) k=({},{}) works in bases: {:?}",
                outer, inner, k_outer, k_inner, bases);
        println!("No truly universal configurations found.");
fn generate_recommendations(strategies: &[BaseStrategy]) {
        if strategy.success_metrics.productive_configs == 0 {
            continue;
        println!("\nBase {}:", strategy.base);
        // Recommend based on base properties
        if strategy.base_properties.is_even {
            println!("  ⚠️  Even base - expect lower success rates");
            println!("  💡 Use odd boundary digits to avoid factor conflicts");
        if strategy.base_properties.is_prime {
            println!("  ✓ Prime base - good potential for diverse configurations");
        // Midpoint recommendations
        if strategy.midpoint_analysis.floor_is_prime && 
           strategy.midpoint_analysis.midpoint_divides_base {
            println!("  🎯 Prime midpoint {} divides base - try exclusive seeds!",
                strategy.midpoint_analysis.midpoint_floor);
        // Edge pair recommendations
        if !strategy.edge_pairs.is_empty() {
            println!("  📐 Try edge pairs: {:?}", 
                strategy.edge_pairs.iter().take(3).collect::<Vec<_>>());
        // Best configuration
            println!("  🏆 Best: ({},{}) k=({},{}) → {:.0}% success",
            if best.working_seeds.len() == 1 {
                println!("     Exclusive to seed {}!", best.working_seeds[0]);
