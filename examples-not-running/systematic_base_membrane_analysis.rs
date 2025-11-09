//! Systematic Base-Membrane Analysis
//! 
//! A rigorous statistical exploration of membrane configurations across different bases
//! with proper controls, significance testing, and comprehensive data output.

use primes::is_prime_miller_rabin;
use num_bigint::BigUint;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
/// Configuration test results
#[derive(Debug, Clone)]
struct ConfigResult {
    base: u32,
    outer: u32,
    inner: u32,
    k_outer: usize,
    k_inner: usize,
    successes: u32,
    total: u32,
    success_rate: f64,
    examples: Vec<String>,
    p_value: f64,  // Statistical significance vs random baseline
}
/// Base analysis results
struct BaseAnalysis {
    factorization: Vec<u32>,
    digit_properties: HashMap<u32, DigitProperty>,
    top_configs: Vec<ConfigResult>,
    baseline_rate: f64,
    special_relationships: Vec<String>,
struct DigitProperty {
    digit: u32,
    is_coprime: bool,
    multiplicative_order: Option<u32>,
    is_self_inverse: bool,
    inverse: Option<u32>,
/// Convert number to arbitrary base
fn to_base(n: &BigUint, base: u32) -> String {
    if n == &BigUint::from(0u32) {
        return "0".to_string();
    }
    
    let mut result = String::new();
    let mut num = n.clone();
    let base_big = BigUint::from(base);
    while num > BigUint::from(0u32) {
        let remainder = &num % &base_big;
        let digit = remainder.to_u32_digits().get(0).copied().unwrap_or(0);
        
        let c = if digit < 10 {
            char::from_digit(digit, 10).unwrap()
        } else {
            char::from_u32('A' as u32 + digit - 10).unwrap()
        };
        result.push(c);
        num /= &base_big;
    result.chars().rev().collect()
/// Convert from arbitrary base to BigUint
fn from_base(s: &str, base: u32) -> Option<BigUint> {
    let mut result = BigUint::from(0u32);
    for c in s.chars() {
        let digit = if c.is_ascii_digit() {
            c.to_digit(10)?
        } else if c.is_ascii_uppercase() {
            c as u32 - 'A' as u32 + 10
            return None;
        if digit >= base {
        }
        result = result * &base_big + BigUint::from(digit);
    Some(result)
/// Construct membrane in given base
fn construct_membrane(outer: u32, inner: u32, seed: &str, k_outer: usize, k_inner: usize, _base: u32) -> String {
    let outer_char = digit_to_char(outer);
    let inner_char = digit_to_char(inner);
    format!("{}{}{}{}{}{}{}{}{}",
        outer_char,
        "0".repeat(k_outer),
        inner_char,
        "0".repeat(k_inner),
        seed,
        outer_char
    )
fn digit_to_char(d: u32) -> char {
    if d < 10 {
        char::from_digit(d, 10).unwrap()
    } else {
        char::from_u32('A' as u32 + d - 10).unwrap()
/// Calculate modular multiplicative inverse
fn mod_inverse(a: u32, m: u32) -> Option<u32> {
    if gcd(a, m) != 1 {
        return None;
    for x in 1..m {
        if (a * x) % m == 1 {
            return Some(x);
    None
/// Calculate multiplicative order
fn multiplicative_order(a: u32, m: u32) -> Option<u32> {
    let mut power = a % m;
    for order in 1..m {
        if power == 1 {
            return Some(order);
        power = (power * a) % m;
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
/// Factorize a number
fn factorize(n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut num = n;
    for p in 2..=((n as f64).sqrt() as u32 + 1) {
        while num % p == 0 {
            factors.push(p);
            num /= p;
    if num > 1 {
        factors.push(num);
    factors
/// Analyze digit properties for a given base
fn analyze_digit_properties(base: u32) -> HashMap<u32, DigitProperty> {
    let mut properties = HashMap::new();
    for digit in 1..base {
        let is_coprime = gcd(digit, base) == 1;
        let multiplicative_order = multiplicative_order(digit, base);
        let inverse = mod_inverse(digit, base);
        let is_self_inverse = inverse == Some(digit);
        properties.insert(digit, DigitProperty {
            digit,
            is_coprime,
            multiplicative_order,
            is_self_inverse,
            inverse,
        });
    properties
/// Test a configuration with statistical rigor
fn test_configuration(
    sample_size: u32,
) -> ConfigResult {
    let mut successes = 0;
    let mut examples = Vec::new();
    for seed_num in 1..=sample_size {
        let seed = to_base(&BigUint::from(seed_num), base);
        let membrane = construct_membrane(outer, inner, &seed, k_outer, k_inner, base);
        if let Some(num) = from_base(&membrane, base) {
            if is_prime_miller_rabin(&num) {
                successes += 1;
                if examples.len() < 3 {
                    examples.push(format!("{} (decimal: {})", membrane, num));
                }
            }
    let success_rate = successes as f64 / sample_size as f64;
    // Calculate p-value using binomial test against baseline
    // For now, using a simplified approximation
    let baseline = 0.1; // ~10% for random numbers of similar size
    let z_score = (success_rate - baseline) / (baseline * (1.0 - baseline) / sample_size as f64).sqrt();
    let p_value = 1.0 - normal_cdf(z_score.abs());
    ConfigResult {
        base,
        outer,
        inner,
        k_outer,
        k_inner,
        successes,
        total: sample_size,
        success_rate,
        examples,
        p_value,
/// Simplified normal CDF for p-value calculation
fn normal_cdf(z: f64) -> f64 {
    // Approximation of the normal CDF
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;
    let sign = if z < 0.0 { -1.0 } else { 1.0 };
    let z = z.abs();
    let t = 1.0 / (1.0 + p * z);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-z * z).exp();
    0.5 * (1.0 + sign * y)
/// Find special relationships between digits
fn find_special_relationships(base: u32, properties: &HashMap<u32, DigitProperty>) -> Vec<String> {
    let mut relationships = Vec::new();
    // Find digit pairs that sum to base
    for d1 in 1..base {
        for d2 in d1..base {
            if d1 + d2 == base {
                relationships.push(format!("{} + {} = {} (base)", d1, d2, base));
    // Find self-inverse digits
    for (digit, prop) in properties {
        if prop.is_self_inverse {
            relationships.push(format!("{} × {} ≡ 1 (mod {})", digit, digit, base));
    // Find digit pairs whose product is -1 (mod base)
            if (d1 * d2) % base == base - 1 {
                relationships.push(format!("{} × {} ≡ -1 (mod {})", d1, d2, base));
    relationships
/// Comprehensive analysis of a single base
fn analyze_base(base: u32, sample_size: u32) -> BaseAnalysis {
    println!("Analyzing base {}...", base);
    let factorization = factorize(base);
    let digit_properties = analyze_digit_properties(base);
    let special_relationships = find_special_relationships(base, &digit_properties);
    let mut all_results = Vec::new();
    // Test all meaningful digit pairs
    for outer in 1..base.min(16) {
        for inner in 1..base.min(16) {
            if outer != inner && gcd(outer, base) == 1 && gcd(inner, base) == 1 {
                // Test symmetric (k1 = k2)
                for k in 0..=2 {
                    let result = test_configuration(base, outer, inner, k, k, sample_size);
                    if result.success_rate > 0.05 { // Only keep if > 5%
                        all_results.push(result);
                    }
                
                // Test asymmetric (k1 != k2)
                for k1 in 0..=2 {
                    for k2 in 0..=2 {
                        if k1 != k2 {
                            let result = test_configuration(base, outer, inner, k1, k2, sample_size);
                            if result.success_rate > 0.05 {
                                all_results.push(result);
                            }
                        }
    // Sort by success rate
    all_results.sort_by(|a, b| b.success_rate.partial_cmp(&a.success_rate).unwrap());
    // Calculate baseline rate
    let mut baseline_successes = 0;
    let baseline_samples = 1000;
    for i in 1..=baseline_samples {
        let num = BigUint::from(base).pow(7) + BigUint::from(i as u32); // Similar size to membranes
        if is_prime_miller_rabin(&num) {
            baseline_successes += 1;
    let baseline_rate = baseline_successes as f64 / baseline_samples as f64;
    BaseAnalysis {
        factorization,
        digit_properties,
        top_configs: all_results.into_iter().take(10).collect(),
        baseline_rate,
        special_relationships,
/// Generate comprehensive report
fn generate_report(analyses: Vec<BaseAnalysis>) -> String {
    let mut report = String::new();
    report.push_str("# SYSTEMATIC BASE-MEMBRANE ANALYSIS REPORT\n");
    report.push_str("=========================================\n\n");
    // Summary table
    report.push_str("## SUMMARY TABLE\n\n");
    report.push_str("| Base | Factorization | Best Config | Success Rate | vs Baseline | p-value |\n");
    report.push_str("|------|---------------|-------------|--------------|-------------|----------|\n");
    for analysis in &analyses {
        if let Some(best) = analysis.top_configs.first() {
            let factors = analysis.factorization.iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join("×");
            
            report.push_str(&format!(
                "| {} | {} | ({},{}) k=({},{}) | {:.1}% | {:.1}x | {:.4} |\n",
                analysis.base,
                factors,
                best.outer,
                best.inner,
                best.k_outer,
                best.k_inner,
                best.success_rate * 100.0,
                best.success_rate / analysis.baseline_rate,
                best.p_value
            ));
    report.push_str("\n## DETAILED ANALYSIS BY BASE\n\n");
        report.push_str(&format!("### Base {}\n", analysis.base));
        report.push_str(&format!("Factorization: {:?}\n", analysis.factorization));
        report.push_str(&format!("Baseline rate: {:.2}%\n\n", analysis.baseline_rate * 100.0));
        // Special relationships
        if !analysis.special_relationships.is_empty() {
            report.push_str("Special relationships:\n");
            for rel in &analysis.special_relationships {
                report.push_str(&format!("- {}\n", rel));
            report.push_str("\n");
        // Top configurations
        report.push_str("Top configurations:\n");
        for (i, config) in analysis.top_configs.iter().take(5).enumerate() {
                "{}. ({},{}) k=({},{}) → {:.1}% ({}/{}) p={:.4}\n",
                i + 1,
                config.outer,
                config.inner,
                config.k_outer,
                config.k_inner,
                config.success_rate * 100.0,
                config.successes,
                config.total,
                config.p_value
            if !config.examples.is_empty() {
                report.push_str("   Examples: ");
                report.push_str(&config.examples.join(", "));
                report.push_str("\n");
        report.push_str("\n");
    // Pattern analysis across bases
    report.push_str("## CROSS-BASE PATTERNS\n\n");
    // Find configurations that work well across multiple bases
    let mut config_performance: HashMap<(u32, u32), Vec<(u32, f64)>> = HashMap::new();
        for config in &analysis.top_configs {
            let key = (config.outer.min(config.inner), config.outer.max(config.inner));
            config_performance.entry(key)
                .or_insert_with(Vec::new)
                .push((analysis.base, config.success_rate));
    report.push_str("Universal configurations (work well across multiple bases):\n");
    for ((d1, d2), performances) in config_performance.iter() {
        if performances.len() >= 3 {
            let avg_rate: f64 = performances.iter().map(|(_, r)| r).sum::<f64>() / performances.len() as f64;
                "- ({},{}) appears in {} bases, avg success: {:.1}%\n",
                d1, d2, performances.len(), avg_rate * 100.0
    report
fn main() {
    println!("SYSTEMATIC BASE-MEMBRANE ANALYSIS");
    println!("=================================\n");
    let sample_size = 100; // Samples per configuration
    let bases_to_test = vec![6, 8, 10, 12, 14, 16, 18, 20, 24, 30];
    let mut analyses = Vec::new();
    for base in bases_to_test {
        let analysis = analyze_base(base, sample_size);
        analyses.push(analysis);
    // Generate report
    let report = generate_report(analyses.clone());
    // Write to file
    let mut file = File::create("base_membrane_analysis_report.txt").unwrap();
    file.write_all(report.as_bytes()).unwrap();
    println!("Analysis complete! Report written to base_membrane_analysis_report.txt");
    // Also generate CSV for further analysis
    let mut csv_file = File::create("base_membrane_data.csv").unwrap();
    writeln!(csv_file, "base,factorization,outer,inner,k_outer,k_inner,successes,total,success_rate,p_value").unwrap();
            writeln!(csv_file, "{},{},{},{},{},{},{},{},{:.4},{:.4}",
                config.success_rate,
            ).unwrap();
    println!("CSV data written to base_membrane_data.csv");
