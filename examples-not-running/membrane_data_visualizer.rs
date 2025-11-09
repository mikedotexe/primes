use primes::{
    generator::Generator,
    prime_checker::is_prime_miller_rabin,
};
use std::collections::{HashMap, HashSet};
#[derive(Debug, Clone)]
struct MembraneResult {
    base: u32,
    outer: u32,
    inner: u32,
    k_outer: usize,
    k_inner: usize,
    seed: String,
    success: bool,
    prime_generated: Option<String>,
}

struct ConfigStats {
    success_count: usize,
    total_count: usize,
    success_rate: f64,
    confidence_lower: f64,
    confidence_upper: f64,
    effect_size: f64,
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Membrane Data Visualizer");
    println!("========================");
    // Read data from CSV files or generate fresh data
    let results = if Path::new("membrane_results.csv").exists() {
        println!("Reading existing CSV data...");
        read_csv_data("membrane_results.csv")?
    } else {
        println!("Generating fresh data...");
        generate_membrane_data()?
    };
    println!("Loaded {} data points", results.len());
    // Generate all visualization outputs
    generate_gnuplot_data(&results)?;
    generate_matplotlib_data(&results)?;
    generate_latex_tables(&results)?;
    generate_summary_statistics(&results)?;
    generate_r_dataframes(&results)?;
    println!("\nAll visualization files generated successfully!");
    Ok(())
fn read_csv_data(filename: &str) -> Result<Vec<MembraneResult>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut results = Vec::new();
    
    for (i, line) in reader.lines().enumerate() {
        if i == 0 { continue; } // Skip header
        
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 8 {
            results.push(MembraneResult {
                base: parts[0].parse()?,
                outer: parts[1].parse()?,
                inner: parts[2].parse()?,
                k_outer: parts[3].parse()?,
                k_inner: parts[4].parse()?,
                seed: parts[5].to_string(),
                success: parts[6].parse()?,
                prime_generated: if parts[7] != "None" { Some(parts[7].to_string()) } else { None },
            });
        }
    }
    Ok(results)
fn generate_membrane_data() -> Result<Vec<MembraneResult>, Box<dyn std::error::Error>> {
    let bases = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let k_values = vec![(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (0, 2)];
    let seeds = vec!["1", "5", "7", "11", "13", "17", "19", "23", "29", "31"];
    for &base in &bases {
        println!("Processing base {}...", base);
        for outer in 1..base {
            for inner in 1..base {
                if outer == inner { continue; }
                
                for &(k_outer, k_inner) in &k_values {
                    for seed in &seeds {
                        let config = Config::new(base, vec![outer, inner], vec![k_outer, k_inner]);
                        let mut generator = Generator::new(config);
                        
                        match generator.generate_number(seed) {
                            Ok(generated) => {
                                let is_prime = is_prime_miller_rabin(&generated, 20);
                                results.push(MembraneResult {
                                    base,
                                    outer,
                                    inner,
                                    k_outer,
                                    k_inner,
                                    seed: seed.to_string(),
                                    success: is_prime,
                                    prime_generated: if is_prime { Some(generated.to_str_radix(10)) } else { None },
                                });
                            }
                            Err(_) => {
                                    success: false,
                                    prime_generated: None,
                        }
                    }
                }
            }
    // Save to CSV
    save_to_csv(&results, "membrane_results.csv")?;
fn save_to_csv(results: &[MembraneResult], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    writeln!(file, "base,outer,inner,k_outer,k_inner,seed,success,prime_generated")?;
    for result in results {
        writeln!(
            file,
            "{},{},{},{},{},{},{},{}",
            result.base,
            result.outer,
            result.inner,
            result.k_outer,
            result.k_inner,
            result.seed,
            result.success,
            result.prime_generated.as_deref().unwrap_or("None")
        )?;
fn calculate_stats(results: &[MembraneResult]) -> HashMap<(u32, u32, u32), ConfigStats> {
    let mut stats_map = HashMap::new();
    // Group by configuration
    let mut config_groups: HashMap<(u32, u32, u32), Vec<&MembraneResult>> = HashMap::new();
        config_groups.entry((result.base, result.outer, result.inner))
            .or_insert_with(Vec::new)
            .push(result);
    // Calculate baseline success rate for each base
    let mut base_baselines: HashMap<u32, f64> = HashMap::new();
    for (base, group) in results.iter().fold(HashMap::new(), |mut acc: HashMap<u32, Vec<&MembraneResult>>, r| {
        acc.entry(r.base).or_insert_with(Vec::new).push(r);
        acc
    }) {
        let successes = group.iter().filter(|r| r.success).count();
        base_baselines.insert(base, successes as f64 / group.len() as f64);
    // Calculate stats for each configuration
    for ((base, outer, inner), group) in config_groups {
        let success_count = group.iter().filter(|r| r.success).count();
        let total_count = group.len();
        let success_rate = success_count as f64 / total_count as f64;
        // Wilson score confidence interval
        let z = 1.96; // 95% confidence
        let n = total_count as f64;
        let p_hat = success_rate;
        let denominator = 1.0 + z * z / n;
        let center = (p_hat + z * z / (2.0 * n)) / denominator;
        let margin = z * ((p_hat * (1.0 - p_hat) / n + z * z / (4.0 * n * n)).sqrt()) / denominator;
        let confidence_lower = (center - margin).max(0.0);
        let confidence_upper = (center + margin).min(1.0);
        // Cohen's h effect size vs baseline
        let baseline = base_baselines.get(&base).copied().unwrap_or(0.1);
        let h1 = 2.0 * success_rate.sqrt().asin();
        let h2 = 2.0 * baseline.sqrt().asin();
        let effect_size = h1 - h2;
        stats_map.insert((base, outer, inner), ConfigStats {
            success_count,
            total_count,
            success_rate,
            confidence_lower,
            confidence_upper,
            effect_size,
        });
    stats_map
fn generate_gnuplot_data(results: &[MembraneResult]) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("visualization_data/gnuplot")?;
    // 1. Success rate vs base (grouped by base type)
    let mut base_stats: HashMap<u32, (usize, usize)> = HashMap::new();
        let entry = base_stats.entry(result.base).or_insert((0, 0));
        entry.1 += 1;
        if result.success {
            entry.0 += 1;
    let mut gnuplot_file = File::create("visualization_data/gnuplot/success_rate_by_base.dat")?;
    writeln!(gnuplot_file, "# Base Type SuccessRate")?;
    for (base, (successes, total)) in base_stats.iter() {
        let base_type = if is_prime_miller_rabin(&(*base).into(), 20) {
            "Prime"
        } else if base % 2 == 0 {
            "EvenComposite"
        } else {
            "OddComposite"
        };
        let success_rate = *successes as f64 / *total as f64;
        writeln!(gnuplot_file, "{} {} {:.4}", base, base_type, success_rate)?;
    // 2. Effect size distribution
    let stats = calculate_stats(results);
    let mut effect_sizes: Vec<f64> = stats.values().map(|s| s.effect_size).collect();
    effect_sizes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut effect_file = File::create("visualization_data/gnuplot/effect_size_distribution.dat")?;
    writeln!(effect_file, "# EffectSize Frequency")?;
    // Create histogram bins
    let min_effect = effect_sizes.first().copied().unwrap_or(0.0);
    let max_effect = effect_sizes.last().copied().unwrap_or(1.0);
    let bin_width = (max_effect - min_effect) / 20.0;
    for i in 0..20 {
        let bin_start = min_effect + i as f64 * bin_width;
        let bin_end = bin_start + bin_width;
        let count = effect_sizes.iter().filter(|&&e| e >= bin_start && e < bin_end).count();
        writeln!(effect_file, "{:.4} {}", (bin_start + bin_end) / 2.0, count)?;
    // 3. Heatmaps for each base
    for base in 2..=16 {
        let mut heatmap_file = File::create(format!("visualization_data/gnuplot/heatmap_base_{}.dat", base))?;
        writeln!(heatmap_file, "# Outer Inner SuccessRate")?;
                if let Some(stat) = stats.get(&(base, outer, inner)) {
                    writeln!(heatmap_file, "{} {} {:.4}", outer, inner, stat.success_rate)?;
                } else {
                    writeln!(heatmap_file, "{} {} 0.0", outer, inner)?;
            writeln!(heatmap_file)?; // Empty line for gnuplot pm3d
    // 4. Confidence intervals for top configurations
    let mut top_configs: Vec<((u32, u32, u32), &ConfigStats)> = stats.iter().collect();
    top_configs.sort_by(|a, b| b.1.success_rate.partial_cmp(&a.1.success_rate).unwrap());
    top_configs.truncate(20);
    let mut ci_file = File::create("visualization_data/gnuplot/confidence_intervals.dat")?;
    writeln!(ci_file, "# Config SuccessRate Lower Upper")?;
    for ((base, outer, inner), stat) in top_configs {
        let config_str = format!("B{}:({},{})", base, outer, inner);
        writeln!(ci_file, "{} {:.4} {:.4} {:.4}", 
            config_str, stat.success_rate, stat.confidence_lower, stat.confidence_upper)?;
fn generate_matplotlib_data(results: &[MembraneResult]) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("visualization_data/matplotlib")?;
    // Convert data to Python-friendly CSV format
    // 1. Complete stats for all configurations
    let mut stats_file = File::create("visualization_data/matplotlib/complete_stats.csv")?;
    writeln!(stats_file, "base,outer,inner,success_rate,confidence_lower,confidence_upper,effect_size,total_trials")?;
    for ((base, outer, inner), stat) in &stats {
        writeln!(stats_file, "{},{},{},{:.6},{:.6},{:.6},{:.6},{}",
            base, outer, inner, stat.success_rate, stat.confidence_lower, 
            stat.confidence_upper, stat.effect_size, stat.total_count)?;
    // 2. Base-aggregated data
    let mut base_file = File::create("visualization_data/matplotlib/base_aggregated.csv")?;
    writeln!(base_file, "base,base_type,total_configs,avg_success_rate,max_success_rate,std_dev")?;
        let base_configs: Vec<&ConfigStats> = stats.iter()
            .filter(|((b, _, _), _)| *b == base)
            .map(|(_, s)| s)
            .collect();
        if base_configs.is_empty() { continue; }
        let base_type = if is_prime_miller_rabin(&base.into(), 20) {
            "prime"
            "even_composite"
            "odd_composite"
        let avg_rate = base_configs.iter().map(|s| s.success_rate).sum::<f64>() / base_configs.len() as f64;
        let max_rate = base_configs.iter().map(|s| s.success_rate).fold(0.0, f64::max);
        let variance = base_configs.iter()
            .map(|s| (s.success_rate - avg_rate).powi(2))
            .sum::<f64>() / base_configs.len() as f64;
        let std_dev = variance.sqrt();
        writeln!(base_file, "{},{},{},{:.6},{:.6},{:.6}",
            base, base_type, base_configs.len(), avg_rate, max_rate, std_dev)?;
    // 3. K-value analysis
    let mut k_file = File::create("visualization_data/matplotlib/k_value_analysis.csv")?;
    writeln!(k_file, "k_outer,k_inner,avg_success_rate,count")?;
    let mut k_groups: HashMap<(usize, usize), Vec<&MembraneResult>> = HashMap::new();
        k_groups.entry((result.k_outer, result.k_inner))
    for ((k_outer, k_inner), group) in k_groups {
        let avg_rate = success_count as f64 / group.len() as f64;
        writeln!(k_file, "{},{},{:.6},{}", k_outer, k_inner, avg_rate, group.len())?;
fn generate_latex_tables(results: &[MembraneResult]) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("visualization_data/latex")?;
    // 1. Top configurations table
    top_configs.truncate(15);
    let mut top_file = File::create("visualization_data/latex/top_configurations.tex")?;
    writeln!(top_file, "\\begin{{table}}[h]")?;
    writeln!(top_file, "\\centering")?;
    writeln!(top_file, "\\caption{{Top 15 Membrane Configurations by Success Rate}}")?;
    writeln!(top_file, "\\begin{{tabular}}{{|c|c|c|c|c|c|}}")?;
    writeln!(top_file, "\\hline")?;
    writeln!(top_file, "Base & Outer & Inner & Success Rate & 95\\% CI & Effect Size \\\\")?;
    for ((base, outer, inner), stat) in &top_configs {
        writeln!(top_file, "{} & {} & {} & {:.2}\\% & [{:.2}\\%, {:.2}\\%] & {:.3} \\\\",
            base, outer, inner, 
            stat.success_rate * 100.0,
            stat.confidence_lower * 100.0,
            stat.confidence_upper * 100.0,
            stat.effect_size)?;
    writeln!(top_file, "\\end{{tabular}}")?;
    writeln!(top_file, "\\end{{table}}")?;
    // 2. Base comparison table
    let mut base_file = File::create("visualization_data/latex/base_comparison.tex")?;
    writeln!(base_file, "\\begin{{table}}[h]")?;
    writeln!(base_file, "\\centering")?;
    writeln!(base_file, "\\caption{{Success Rates by Number Base}}")?;
    writeln!(base_file, "\\begin{{tabular}}{{|c|c|c|c|c|}}")?;
    writeln!(base_file, "\\hline")?;
    writeln!(base_file, "Base & Type & Configurations & Avg Success & Max Success \\\\")?;
        let base_stats: Vec<&ConfigStats> = stats.iter()
        if base_stats.is_empty() { continue; }
            "Even"
            "Odd"
        let avg_rate = base_stats.iter().map(|s| s.success_rate).sum::<f64>() / base_stats.len() as f64;
        let max_rate = base_stats.iter().map(|s| s.success_rate).fold(0.0, f64::max);
        writeln!(base_file, "{} & {} & {} & {:.2}\\% & {:.2}\\% \\\\",
            base, base_type, base_stats.len(), avg_rate * 100.0, max_rate * 100.0)?;
    writeln!(base_file, "\\end{{tabular}}")?;
    writeln!(base_file, "\\end{{table}}")?;
    // 3. K-value effects table
    let mut k_file = File::create("visualization_data/latex/k_value_effects.tex")?;
    writeln!(k_file, "\\begin{{table}}[h]")?;
    writeln!(k_file, "\\centering")?;
    writeln!(k_file, "\\caption{{Effect of Zero Padding (k-values) on Success Rate}}")?;
    writeln!(k_file, "\\begin{{tabular}}{{|c|c|c|c|}}")?;
    writeln!(k_file, "\\hline")?;
    writeln!(k_file, "$k_{{outer}}$ & $k_{{inner}}$ & Avg Success & Sample Size \\\\")?;
    let mut k_stats: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let entry = k_stats.entry((result.k_outer, result.k_inner)).or_insert((0, 0));
    let mut k_vec: Vec<((usize, usize), (usize, usize))> = k_stats.into_iter().collect();
    k_vec.sort_by_key(|((ko, ki), _)| (*ko, *ki));
    for ((k_outer, k_inner), (successes, total)) in k_vec {
        let success_rate = successes as f64 / total as f64;
        writeln!(k_file, "{} & {} & {:.2}\\% & {} \\\\",
            k_outer, k_inner, success_rate * 100.0, total)?;
    writeln!(k_file, "\\end{{tabular}}")?;
    writeln!(k_file, "\\end{{table}}")?;
fn generate_summary_statistics(results: &[MembraneResult]) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("visualization_data/summary")?;
    let mut summary_file = File::create("visualization_data/summary/statistics.txt")?;
    writeln!(summary_file, "MEMBRANE DATA VISUALIZATION SUMMARY")?;
    writeln!(summary_file, "===================================")?;
    writeln!(summary_file)?;
    writeln!(summary_file, "Dataset Overview:")?;
    writeln!(summary_file, "- Total data points: {}", results.len())?;
    writeln!(summary_file, "- Total successes: {}", results.iter().filter(|r| r.success).count())?;
    writeln!(summary_file, "- Overall success rate: {:.2}%", 
        results.iter().filter(|r| r.success).count() as f64 / results.len() as f64 * 100.0)?;
    writeln!(summary_file, "Base Coverage:")?;
    let unique_bases: HashSet<u32> = results.iter().map(|r| r.base).collect();
    writeln!(summary_file, "- Bases tested: {:?}", {
        let mut bases: Vec<u32> = unique_bases.into_iter().collect();
        bases.sort();
        bases
    })?;
    writeln!(summary_file, "Configuration Analysis:")?;
    writeln!(summary_file, "- Total unique configurations: {}", stats.len())?;
    writeln!(summary_file, "- Configurations with >50% success: {}", 
        stats.values().filter(|s| s.success_rate > 0.5).count())?;
    writeln!(summary_file, "- Configurations with >75% success: {}", 
        stats.values().filter(|s| s.success_rate > 0.75).count())?;
    writeln!(summary_file, "Effect Size Distribution:")?;
    let effect_sizes: Vec<f64> = stats.values().map(|s| s.effect_size).collect();
    let mean_effect = effect_sizes.iter().sum::<f64>() / effect_sizes.len() as f64;
    let positive_effects = effect_sizes.iter().filter(|&&e| e > 0.0).count();
    writeln!(summary_file, "- Mean effect size: {:.3}", mean_effect)?;
    writeln!(summary_file, "- Positive effects: {} ({:.1}%)", 
        positive_effects, positive_effects as f64 / effect_sizes.len() as f64 * 100.0)?;
    writeln!(summary_file, "Top 5 Configurations:")?;
    for (i, ((base, outer, inner), stat)) in top_configs.iter().take(5).enumerate() {
        writeln!(summary_file, "{}. Base {}: ({},{}) - {:.1}% success ({}/{})",
            i + 1, base, outer, inner, stat.success_rate * 100.0, 
            stat.success_count, stat.total_count)?;
fn generate_r_dataframes(results: &[MembraneResult]) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("visualization_data/r_data")?;
    // 1. Complete results dataframe
    let mut results_file = File::create("visualization_data/r_data/membrane_results.csv")?;
    writeln!(results_file, "base,outer,inner,k_outer,k_inner,seed,success,prime_generated,base_type")?;
        let base_type = if is_prime_miller_rabin(&result.base.into(), 20) {
        } else if result.base % 2 == 0 {
        writeln!(results_file, "{},{},{},{},{},{},{},{},{}",
            result.base, result.outer, result.inner, result.k_outer, result.k_inner,
            result.seed, if result.success { 1 } else { 0 },
            result.prime_generated.as_deref().unwrap_or("NA"),
            base_type)?;
    // 2. Aggregated statistics dataframe
    let mut stats_file = File::create("visualization_data/r_data/config_statistics.csv")?;
    writeln!(stats_file, "base,outer,inner,success_count,total_count,success_rate,ci_lower,ci_upper,effect_size")?;
        writeln!(stats_file, "{},{},{},{},{},{:.6},{:.6},{:.6},{:.6}",
            base, outer, inner, stat.success_count, stat.total_count,
            stat.success_rate, stat.confidence_lower, stat.confidence_upper, stat.effect_size)?;
    // 3. Pairwise comparisons dataframe (for statistical tests)
    let mut pairs_file = File::create("visualization_data/r_data/pairwise_comparisons.csv")?;
    writeln!(pairs_file, "base,config1_outer,config1_inner,config2_outer,config2_inner,rate_diff,p_value")?;
    // Generate pairwise comparisons for each base
        let base_configs: Vec<((u32, u32), &ConfigStats)> = stats.iter()
            .map(|((_, o, i), s)| ((*o, *i), s))
        for i in 0..base_configs.len() {
            for j in i+1..base_configs.len() {
                let ((o1, i1), s1) = &base_configs[i];
                let ((o2, i2), s2) = &base_configs[j];
                let rate_diff = s1.success_rate - s2.success_rate;
                // Simple chi-square test p-value approximation
                let n1 = s1.total_count as f64;
                let n2 = s2.total_count as f64;
                let p1 = s1.success_rate;
                let p2 = s2.success_rate;
                let p_pooled = (s1.success_count + s2.success_count) as f64 / (n1 + n2);
                let se = (p_pooled * (1.0 - p_pooled) * (1.0/n1 + 1.0/n2)).sqrt();
                let z = if se > 0.0 { (p1 - p2) / se } else { 0.0 };
                let p_value = 2.0 * (1.0 - normal_cdf(z.abs()));
                writeln!(pairs_file, "{},{},{},{},{},{:.6},{:.6}",
                    base, o1, i1, o2, i2, rate_diff, p_value)?;
    // 4. Time series style data (if seeds represent some ordering)
    let mut time_file = File::create("visualization_data/r_data/seed_progression.csv")?;
    writeln!(time_file, "base,outer,inner,seed_index,seed,success")?;
    let seed_order = vec!["1", "5", "7", "11", "13", "17", "19", "23", "29", "31"];
        if let Some(seed_idx) = seed_order.iter().position(|&s| s == result.seed) {
            writeln!(time_file, "{},{},{},{},{},{}",
                result.base, result.outer, result.inner, seed_idx, result.seed,
                if result.success { 1 } else { 0 })?;
    println!("\nGenerated R-compatible data files:");
    println!("- membrane_results.csv: Complete raw data");
    println!("- config_statistics.csv: Aggregated statistics per configuration");
    println!("- pairwise_comparisons.csv: For statistical hypothesis testing");
    println!("- seed_progression.csv: For analyzing patterns across seeds");
// Simple normal CDF approximation for p-value calculation
fn normal_cdf(z: f64) -> f64 {
    0.5 * (1.0 + erf(z / std::f64::consts::SQRT_2))
fn erf(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    sign * y
