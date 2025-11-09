use std::io;use primes::{
    analysis::{ConfigurationAnalyzer, AnalysisResult},
};
use num_bigint::BigUint;
const VERSION: &str = "1.0.0";
const TOP_CONFIGS_PER_BASE: usize = 50;
const MIN_SEEDS_TO_TEST: usize = 100;
const CONFIDENCE_LEVEL: f64 = 0.95;

struct DatasetGenerator {
    analyzer: ConfigurationAnalyzer,
    results: Vec<ConfigResult>,
}
#[derive(Clone)]
struct ConfigResult {
    base: u32,
    factorization: String,
    outer: u8,
    inner: u8,
    k_outer: usize,
    k_inner: usize,
    seeds_tested: usize,
    primes_found: usize,
    success_rate: f64,
    baseline_rate: f64,
    p_value: f64,
    effect_size: f64,
    confidence_interval_lower: f64,
    confidence_interval_upper: f64,
impl DatasetGenerator {
    fn new() -> Self {
        Self {
            analyzer: ConfigurationAnalyzer::new(),
            results: Vec::new(),
        }
    }
    fn generate_dataset(&mut self) {
        println!("=== Prime Configuration Verification Dataset Generator v{} ===", VERSION);
        println!("Timestamp: {}", Utc::now().to_rfc3339());
        println!();
        // Test bases 2 through 20
        for base in 2..=20 {
            println!("Testing base {}...", base);
            self.test_base(base);
        // Sort results by success rate descending
        self.results.sort_by(|a, b| b.success_rate.partial_cmp(&a.success_rate).unwrap());
    fn test_base(&mut self, base: u32) {
        let factorization = self.get_factorization(base);
        let baseline = self.calculate_baseline(base);
        
        let mut base_results = Vec::new();
        // Test all combinations of boundary digits and k-values
        for outer in 1..base as u8 {
            for inner in 1..base as u8 {
                for k_outer in 0..=3 {
                    for k_inner in 0..=3 {
                        let config = MembraneConfig::new(base, outer, inner, k_outer, k_inner);
                        
                        // Test this configuration
                        let (seeds_tested, primes_found) = self.test_configuration(&config);
                        if seeds_tested >= 10 { // Only include configs with sufficient data
                            let success_rate = primes_found as f64 / seeds_tested as f64;
                            
                            // Calculate statistical measures
                            let (p_value, effect_size) = self.calculate_statistics(
                                primes_found, seeds_tested, baseline
                            );
                            let (ci_lower, ci_upper) = self.calculate_confidence_interval(
                                primes_found, seeds_tested
                            base_results.push(ConfigResult {
                                base,
                                factorization: factorization.clone(),
                                outer,
                                inner,
                                k_outer,
                                k_inner,
                                seeds_tested,
                                primes_found,
                                success_rate,
                                baseline_rate: baseline,
                                p_value,
                                effect_size,
                                confidence_interval_lower: ci_lower,
                                confidence_interval_upper: ci_upper,
                            });
                        }
                    }
                }
            }
        // Keep only top configurations for this base
        base_results.sort_by(|a, b| b.success_rate.partial_cmp(&a.success_rate).unwrap());
        base_results.truncate(TOP_CONFIGS_PER_BASE);
        self.results.extend(base_results);
    fn test_configuration(&self, config: &MembraneConfig) -> (usize, usize) {
        let mut seeds_tested = 0;
        let mut primes_found = 0;
        // Test single-digit seeds
        for seed in 1..10 {
            let seed_str = seed.to_string();
            let membrane = MembraneStructure::new(config.clone(), seed_str);
            if let Ok(number) = membrane.generate_number() {
                seeds_tested += 1;
                if self.is_prime(&number) {
                    primes_found += 1;
        // Test two-digit seeds
        for seed in 10..100 {
                
                // Stop early if we have enough samples
                if seeds_tested >= MIN_SEEDS_TO_TEST {
                    break;
        (seeds_tested, primes_found)
    fn is_prime(&self, n: &BigUint) -> bool {
        if n <= &BigUint::one() {
            return false;
        // Convert to u64 if possible for faster primality testing
        if let Some(num) = n.to_u64_digits().first() {
            if n.to_u64_digits().len() == 1 {
                return miller_rabin(*num, 20);
        // For larger numbers, use BigUint primality test
        miller_rabin_big(n, 20)
    fn calculate_baseline(&self, base: u32) -> f64 {
        // Approximate prime density for numbers in the typical range
        // generated by our configurations
        let typical_size = base.pow(6) as f64; // Rough estimate
        1.0 / typical_size.ln()
    fn get_factorization(&self, n: u32) -> String {
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
        if factors.len() == 1 {
            "prime".to_string()
        } else {
            factors.iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join("×")
    fn calculate_statistics(&self, successes: usize, trials: usize, baseline: f64) -> (f64, f64) {
        let observed_rate = successes as f64 / trials as f64;
        // Calculate z-score for binomial test
        let expected = trials as f64 * baseline;
        let variance = trials as f64 * baseline * (1.0 - baseline);
        let z = (successes as f64 - expected) / variance.sqrt();
        // Calculate p-value (two-tailed)
        let normal = Normal::new(0.0, 1.0).unwrap();
        let p_value = 2.0 * (1.0 - normal.cdf(z.abs()));
        // Calculate Cohen's h effect size
        let h1 = 2.0 * (observed_rate.sqrt()).asin();
        let h2 = 2.0 * (baseline.sqrt()).asin();
        let effect_size = (h1 - h2).abs();
        (p_value, effect_size)
    fn calculate_confidence_interval(&self, successes: usize, trials: usize) -> (f64, f64) {
        let p = successes as f64 / trials as f64;
        let z = 1.96; // 95% confidence
        // Wilson score interval
        let denominator = 1.0 + z * z / trials as f64;
        let center = (p + z * z / (2.0 * trials as f64)) / denominator;
        let margin = z * ((p * (1.0 - p) / trials as f64 + z * z / (4.0 * trials as f64 * trials as f64)).sqrt()) / denominator;
        (center - margin, center + margin)
    fn write_csv(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        // Write metadata
        writeln!(file, "# Prime Configuration Verification Dataset")?;
        writeln!(file, "# Version: {}", VERSION)?;
        writeln!(file, "# Generated: {}", Utc::now().to_rfc3339())?;
        writeln!(file, "# Total configurations tested: {}", self.results.len())?;
        writeln!(file)?;
        // Write header
        writeln!(file, "base,factorization,outer,inner,k_outer,k_inner,seeds_tested,primes_found,success_rate,baseline_rate,p_value,effect_size,confidence_interval_lower,confidence_interval_upper")?;
        // Write data
        for result in &self.results {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}",
                result.base,
                result.factorization,
                result.outer,
                result.inner,
                result.k_outer,
                result.k_inner,
                result.seeds_tested,
                result.primes_found,
                result.success_rate,
                result.baseline_rate,
                result.p_value,
                result.effect_size,
                result.confidence_interval_lower,
                result.confidence_interval_upper
            )?;
        Ok(())
    fn print_summary(&self) {
        println!("\n=== Summary Statistics ===");
        println!("Total configurations tested: {}", self.results.len());
        // Group by base for summary
        let mut base_summaries = std::collections::HashMap::new();
            base_summaries.entry(result.base)
                .or_insert(Vec::new())
                .push(result.clone());
        println!("\nTop configurations by base:");
            if let Some(configs) = base_summaries.get(&base) {
                if let Some(best) = configs.iter().max_by(|a, b| a.success_rate.partial_cmp(&b.success_rate).unwrap()) {
                    println!(
                        "Base {} ({}): ({},{}) k=({},{}) - {:.1}% success rate (p={:.4})",
                        base,
                        best.factorization,
                        best.outer,
                        best.inner,
                        best.k_outer,
                        best.k_inner,
                        best.success_rate * 100.0,
                        best.p_value
                    );
        // Overall statistics
        let significant_configs: Vec<_> = self.results.iter()
            .filter(|r| r.p_value < 0.05)
            .collect();
        println!("\n=== Statistical Summary ===");
        println!("Statistically significant configurations (p < 0.05): {}", significant_configs.len());
        if !significant_configs.is_empty() {
            let avg_effect_size: f64 = significant_configs.iter()
                .map(|r| r.effect_size)
                .sum::<f64>() / significant_configs.len() as f64;
            
            println!("Average effect size (Cohen's h): {:.3}", avg_effect_size);
        // Best overall configuration
        if let Some(best) = self.results.first() {
            println!("\n=== Best Overall Configuration ===");
            println!("Base {}: ({},{}) k=({},{})", best.base, best.outer, best.inner, best.k_outer, best.k_inner);
            println!("Success rate: {:.1}% (95% CI: {:.1}%-{:.1}%)", 
                best.success_rate * 100.0,
                best.confidence_interval_lower * 100.0,
                best.confidence_interval_upper * 100.0
            );
            println!("vs baseline: {:.1}%", best.baseline_rate * 100.0);
            println!("Effect size: {:.3} (p = {:.6})", best.effect_size, best.p_value);
// Miller-Rabin primality test for u64
fn miller_rabin(n: u64, k: usize) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 { return false; }
    
    // Write n-1 as 2^r * d
    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    // Witnesses to test
    let witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for _ in 0..k.min(witnesses.len()) {
        let a = witnesses[rand::random::<usize>() % witnesses.len()];
        if !miller_rabin_witness(n, a as u64, d, r) {
    true
fn miller_rabin_witness(n: u64, a: u64, d: u64, r: u32) -> bool {
    let mut x = mod_pow(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    for _ in 0..r-1 {
        x = (x as u128 * x as u128 % n as u128) as u64;
        if x == n - 1 {
            return true;
    false
fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1u128;
    let mut base = base as u128;
    let mut exp = exp;
    let modulus = modulus as u128;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        base = (base * base) % modulus;
        exp /= 2;
    result as u64
// Miller-Rabin for BigUint (simplified version)
fn miller_rabin_big(n: &BigUint, k: usize) -> bool {
    use num_bigint::RandBigInt;
    use rand::thread_rng;
    if n <= &BigUint::one() { return false; }
    if n == &BigUint::from(2u32) { return true; }
    if n.is_even() { return false; }
    let one = BigUint::one();
    let two = BigUint::from(2u32);
    let n_minus_1 = n - &one;
    let mut d = n_minus_1.clone();
    while d.is_even() {
        d /= &two;
    let mut rng = thread_rng();
    for _ in 0..k {
        let a = rng.gen_biguint_range(&two, &n_minus_1);
        let mut x = a.modpow(&d, n);
        if x == one || x == n_minus_1 {
            continue;
        let mut continue_witness = false;
        for _ in 0..r-1 {
            x = (&x * &x) % n;
            if x == n_minus_1 {
                continue_witness = true;
                break;
        if !continue_witness {
fn main() {
    let mut generator = DatasetGenerator::new();
    // Generate the dataset
    generator.generate_dataset();
    // Print human-readable summary
    generator.print_summary();
    // Write machine-readable CSV
    let csv_filename = format!("prime_verification_dataset_{}.csv", 
        Utc::now().format("%Y%m%d_%H%M%S"));
    match generator.write_csv(&csv_filename) {
        Ok(_) => println!("\n✓ Dataset written to: {}", csv_filename),
        Err(e) => println!("\n✗ Error writing CSV: {}", e),
