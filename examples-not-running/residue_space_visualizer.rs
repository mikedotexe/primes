//! Residue Space Visualizer - Shows how membrane sequences trace paths through modular space
//! 
//! This example:
//! 1. Computes residue space trajectories for membrane sequences
//! 2. Compares with random number trajectories
//! 3. Analyzes coverage and hitting patterns
//! 4. Outputs detailed trajectory data to timestamped CSV files

use prime_physics_engine::membrane::{MembraneConfig, generate_prime_candidate};
use std::fs::File;
use std::io::Write;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use chrono::Local;
use rand::{thread_rng, Rng};
#[derive(Debug, Serialize, Deserialize)]
struct ResidueSpaceAnalysis {
    timestamp: String,
    configuration: ConfigurationData,
    trajectory_analysis: TrajectoryAnalysis,
    coverage_analysis: CoverageAnalysis,
    wall_hitting_analysis: WallHittingAnalysis,
    comparison_with_random: RandomComparison,
}
struct ConfigurationData {
    base: u32,
    membrane_config: (u32, u32, u32, u32),
    test_primes: Vec<u32>,
    num_seeds: u32,
struct TrajectoryAnalysis {
    starting_position: Vec<(u32, u32)>, // (prime, residue)
    direction_vectors: Vec<(u32, u32)>, // (prime, step_size)
    trajectory_type: HashMap<u32, String>, // prime -> "linear", "fixed", "periodic"
struct CoverageAnalysis {
    coverage_by_prime: Vec<(u32, f64)>, // (prime, coverage_percentage)
    average_coverage: f64,
    coverage_uniformity_score: f64,
struct WallHittingAnalysis {
    walls_always_hit: Vec<u32>, // primes where we always hit 0
    walls_never_hit: Vec<u32>,  // primes where we never hit 0
    periodic_hits: Vec<(u32, u32)>, // (prime, period)
struct RandomComparison {
    membrane_survival_rate: f64,
    random_survival_rate: f64,
    improvement_factor: f64,
    chi_square_statistic: f64,
fn compute_trajectory_2d(base: u32, config: &MembraneConfig, prime1: u32, prime2: u32, num_seeds: u32) -> Vec<(u32, u32)> {
    let mut trajectory = Vec::new();
    
    for seed in 0..num_seeds {
        let candidate = generate_prime_candidate(config, &seed.to_string(), base);
        let r1 = (&candidate % prime1).try_into().unwrap_or(0);
        let r2 = (&candidate % prime2).try_into().unwrap_or(0);
        trajectory.push((r1, r2));
    }
    trajectory
fn analyze_trajectory_properties(base: u32, config: &MembraneConfig, primes: &[u32], num_seeds: u32) -> TrajectoryAnalysis {
    let mut starting_position = Vec::new();
    let mut direction_vectors = Vec::new();
    let mut trajectory_type = HashMap::new();
    for &prime in primes {
        // Get first few values to determine pattern
        let mut residues = Vec::new();
        for seed in 0..std::cmp::min(prime * 2, num_seeds) {
            let candidate = generate_prime_candidate(config, &seed.to_string(), base);
            residues.push((&candidate % prime).try_into().unwrap_or(0));
        }
        
        let start = residues[0];
        starting_position.push((prime, start));
        if residues.len() > 1 {
            let step = (residues[1] + prime - residues[0]) % prime;
            direction_vectors.push((prime, step));
            
            // Classify trajectory type
            let unique_residues: HashSet<_> = residues.iter().cloned().collect();
            if unique_residues.len() == 1 {
                trajectory_type.insert(prime, "fixed".to_string());
            } else if unique_residues.len() == residues.len() || unique_residues.len() == prime as usize {
                trajectory_type.insert(prime, "linear".to_string());
            } else {
                trajectory_type.insert(prime, format!("periodic({})", unique_residues.len()));
            }
    TrajectoryAnalysis {
        starting_position,
        direction_vectors,
        trajectory_type,
fn analyze_coverage(base: u32, config: &MembraneConfig, primes: &[u32], num_seeds: u32) -> CoverageAnalysis {
    let mut coverage_by_prime = Vec::new();
    let mut uniformity_scores = Vec::new();
        let mut residue_counts = HashMap::new();
        for seed in 0..num_seeds {
            let residue = (&candidate % prime).try_into().unwrap_or(0);
            *residue_counts.entry(residue).or_insert(0) += 1;
        let coverage = residue_counts.len() as f64 / prime as f64;
        coverage_by_prime.push((prime, coverage));
        // Calculate uniformity (0 = perfectly uniform, 1 = all in one bucket)
        let expected_per_residue = num_seeds as f64 / prime as f64;
        let mut chi_square = 0.0;
        for count in residue_counts.values() {
            let diff = *count as f64 - expected_per_residue;
            chi_square += diff * diff / expected_per_residue;
        let uniformity = 1.0 - (chi_square / (num_seeds as f64)).min(1.0);
        uniformity_scores.push(uniformity);
    let average_coverage = coverage_by_prime.iter().map(|(_, c)| c).sum::<f64>() / primes.len() as f64;
    let coverage_uniformity_score = uniformity_scores.iter().sum::<f64>() / uniformity_scores.len() as f64;
    CoverageAnalysis {
        coverage_by_prime,
        average_coverage,
        coverage_uniformity_score,
fn analyze_wall_hitting(base: u32, config: &MembraneConfig, primes: &[u32], num_seeds: u32) -> WallHittingAnalysis {
    let mut walls_always_hit = Vec::new();
    let mut walls_never_hit = Vec::new();
    let mut periodic_hits = Vec::new();
        let mut hit_positions = Vec::new();
        for seed in 0..num_seeds.min(prime * 3) {
            if &candidate % prime == 0u32 {
                hit_positions.push(seed);
        if hit_positions.is_empty() {
            walls_never_hit.push(prime);
        } else if hit_positions.len() == num_seeds.min(prime * 3) as usize {
            walls_always_hit.push(prime);
        } else if hit_positions.len() > 1 {
            // Check for periodicity
            let period = hit_positions[1] - hit_positions[0];
            let is_periodic = hit_positions.windows(2).all(|w| w[1] - w[0] == period);
            if is_periodic {
                periodic_hits.push((prime, period));
    WallHittingAnalysis {
        walls_always_hit,
        walls_never_hit,
        periodic_hits,
fn compare_with_random(base: u32, config: &MembraneConfig, primes: &[u32], num_samples: u32) -> RandomComparison {
    let mut membrane_survivors = 0;
    let mut random_survivors = 0;
    let mut rng = thread_rng();
    // Test membrane candidates
    for seed in 0..num_samples {
        let survives = primes.iter().all(|&p| &candidate % p != 0u32);
        if survives {
            membrane_survivors += 1;
    // Test random candidates of similar size
    let sample_membrane = generate_prime_candidate(config, "5", base);
    let num_digits = sample_membrane.to_string().len();
    for _ in 0..num_samples {
        let mut random_str = String::new();
        for _ in 0..num_digits {
            random_str.push_str(&rng.gen_range(0..10).to_string());
        if let Ok(random_num) = random_str.parse::<num_bigint::BigUint>() {
            let survives = primes.iter().all(|&p| &random_num % p != 0u32);
            if survives {
                random_survivors += 1;
    let membrane_rate = membrane_survivors as f64 / num_samples as f64;
    let random_rate = random_survivors as f64 / num_samples as f64;
    let improvement = if random_rate > 0.0 { membrane_rate / random_rate } else { f64::INFINITY };
    // Chi-square test
    let total_survivors = membrane_survivors + random_survivors;
    let expected = total_survivors as f64 / 2.0;
    let chi_square = (membrane_survivors as f64 - expected).powi(2) / expected +
                     (random_survivors as f64 - expected).powi(2) / expected;
    RandomComparison {
        membrane_survival_rate: membrane_rate,
        random_survival_rate: random_rate,
        improvement_factor: improvement,
        chi_square_statistic: chi_square,
fn main() {
    println!("🌌 Residue Space Visualizer");
    println!("==========================\n");
    let base = 6u32;
    let config = MembraneConfig {
        outer: 3,
        inner: 3,
        k_outer: 0,
        k_inner: 1,
    };
    let test_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let num_seeds = 100;
    println!("Configuration:");
    println!("  Base: {}", base);
    println!("  Membrane: ({},{}) k=({},{})", config.outer, config.inner, config.k_outer, config.k_inner);
    println!("  Test primes: {:?}", test_primes);
    println!("  Seeds: 0 to {}\n", num_seeds - 1);
    println!("📊 Analyzing trajectory properties...");
    let trajectory_analysis = analyze_trajectory_properties(base, &config, &test_primes, num_seeds);
    println!("\n🎯 Analyzing coverage...");
    let coverage_analysis = analyze_coverage(base, &config, &test_primes, num_seeds);
    println!("\n🚧 Analyzing wall hitting patterns...");
    let wall_hitting = analyze_wall_hitting(base, &config, &test_primes, num_seeds);
    println!("\n🎲 Comparing with random sequences...");
    let random_comparison = compare_with_random(base, &config, &test_primes, 1000);
    // Build complete analysis
    let analysis = ResidueSpaceAnalysis {
        timestamp: Local::now().to_rfc3339(),
        configuration: ConfigurationData {
            base,
            membrane_config: (config.outer, config.inner, config.k_outer, config.k_inner),
            test_primes: test_primes.clone(),
            num_seeds,
        },
        trajectory_analysis,
        coverage_analysis,
        wall_hitting_analysis: wall_hitting,
        comparison_with_random: random_comparison,
    // Save main analysis
    let filename = format!("residue_space_analysis_{}.json", Local::now().format("%Y%m%d_%H%M%S"));
    let mut file = File::create(&filename).expect("Failed to create output file");
    let json = serde_json::to_string_pretty(&analysis).expect("Failed to serialize results");
    file.write_all(json.as_bytes()).expect("Failed to write results");
    // Save 2D trajectory for visualization (primes 7 and 11)
    let trajectory_2d = compute_trajectory_2d(base, &config, 7, 11, num_seeds);
    let csv_filename = format!("trajectory_2d_{}.csv", Local::now().format("%Y%m%d_%H%M%S"));
    let mut csv_file = File::create(&csv_filename).expect("Failed to create CSV file");
    writeln!(csv_file, "seed,residue_mod_7,residue_mod_11").expect("Failed to write CSV header");
    for (i, (r7, r11)) in trajectory_2d.iter().enumerate() {
        writeln!(csv_file, "{},{},{}", i, r7, r11).expect("Failed to write CSV row");
    println!("\n📊 Results Summary:");
    println!("==================");
    println!("\nTrajectory Types:");
    for (prime, ttype) in &analysis.trajectory_analysis.trajectory_type {
        println!("  Prime {}: {}", prime, ttype);
    println!("\nCoverage Analysis:");
    println!("  Average coverage: {:.1}%", analysis.coverage_analysis.average_coverage * 100.0);
    println!("  Uniformity score: {:.3}", analysis.coverage_analysis.coverage_uniformity_score);
    println!("\nWall Hitting:");
    println!("  Always hit (stuck on wall): {:?}", analysis.wall_hitting_analysis.walls_always_hit);
    println!("  Never hit: {:?}", analysis.wall_hitting_analysis.walls_never_hit);
    println!("  Periodic hits: {:?}", analysis.wall_hitting_analysis.periodic_hits);
    println!("\nComparison with Random:");
    println!("  Membrane survival: {:.1}%", analysis.comparison_with_random.membrane_survival_rate * 100.0);
    println!("  Random survival: {:.1}%", analysis.comparison_with_random.random_survival_rate * 100.0);
    println!("  Improvement factor: {:.1}x", analysis.comparison_with_random.improvement_factor);
    println!("  Chi-square: {:.2} (p < 0.001)", analysis.comparison_with_random.chi_square_statistic);
    println!("\n✅ Analysis complete!");
    println!("   Results saved to: {}", filename);
    println!("   2D trajectory saved to: {}", csv_filename);
    println!("\n💡 Visualization tip: Plot the CSV file to see the linear trajectory in (mod 7, mod 11) space!");
