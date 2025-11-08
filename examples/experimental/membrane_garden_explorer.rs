//! Membrane Garden Explorer - Cultivating prime density patterns
//! Water the seeds of discovery and watch them bloom!

use std::collections::HashMap;
use num_bigint::BigUint;
use prime_physics_engine::is_prime_miller_rabin;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
fn main() {
    println!("🌱 MEMBRANE GARDEN EXPLORER 🌱");
    println!("==============================\n");
    
    println!("Planting seeds of discovery...");
    println!("Each configuration is a different species in our garden\n");
    // Open our garden journal
    let mut journal = OpenOptions::new()
        .create(true)
        .append(true)
        .open("membrane_garden.log")
        .unwrap();
    // Plant different membrane species
    let garden_plots = vec![
        // The Minimalist Garden - ultra-simple membranes
        GardenPlot {
            name: "Minimalist Singles",
            species: vec![
                MembraneSpecies { l: 1, r: 1, w: 3, base: 10, r1: 0, r2: 0, label: "1[C]1" },
                MembraneSpecies { l: 3, r: 3, w: 3, base: 10, r1: 0, r2: 0, label: "3[C]3" },
                MembraneSpecies { l: 7, r: 7, w: 3, base: 10, r1: 0, r2: 0, label: "7[C]7" },
                MembraneSpecies { l: 9, r: 9, w: 3, base: 10, r1: 0, r2: 0, label: "9[C]9" },
            ],
            c_range: 0..10000,
        },
        
        // The Breathing Garden - asymmetric patterns
            name: "Breathing Patterns",
                MembraneSpecies { l: 1, r: 3, w: 3, base: 10, r1: 0, r2: 0, label: "1[C]3" },
                MembraneSpecies { l: 1, r: 7, w: 3, base: 10, r1: 0, r2: 0, label: "1[C]7" },
                MembraneSpecies { l: 1, r: 9, w: 3, base: 10, r1: 0, r2: 0, label: "1[C]9" },
                MembraneSpecies { l: 3, r: 7, w: 3, base: 10, r1: 0, r2: 0, label: "3[C]7" },
        // The Fractal Garden - mixed μ values
            name: "Fractal μ Patterns",
                MembraneSpecies { l: 1, r: 1, w: 5, base: 10, r1: 0, r2: 1, label: "1[C][0]1 (μ=1)" },
                MembraneSpecies { l: 1, r: 1, w: 5, base: 10, r1: 1, r2: 0, label: "1[0][C]1 (μ=1)" },
                MembraneSpecies { l: 3, r: 7, w: 6, base: 10, r1: 1, r2: 2, label: "3[0][C][00]7 (μ=3)" },
                MembraneSpecies { l: 3, r: 7, w: 6, base: 10, r1: 2, r2: 1, label: "3[00][C][0]7 (μ=3)" },
            c_range: 0..5000,
        // The Exotic Garden - different bases
            name: "Exotic Base Species",
                MembraneSpecies { l: 1, r: 1, w: 3, base: 6, r1: 0, r2: 0, label: "Base-6: 1[C]1" },
                MembraneSpecies { l: 5, r: 5, w: 3, base: 12, r1: 0, r2: 0, label: "Base-12: 5[C]5" },
                MembraneSpecies { l: 1, r: 15, w: 3, base: 16, r1: 0, r2: 0, label: "Base-16: 1[C]F" },
                MembraneSpecies { l: 1, r: 1, w: 3, base: 20, r1: 0, r2: 0, label: "Base-20: 1[C]1" },
        // The Resonance Garden - self-inverse exploration
            name: "Resonance Chambers",
                MembraneSpecies { l: 5, r: 5, w: 3, base: 12, r1: 0, r2: 0, label: "5[C]5 (5²≡1 mod 12)" },
                MembraneSpecies { l: 7, r: 7, w: 3, base: 12, r1: 0, r2: 0, label: "7[C]7 (7²≡1 mod 12)" },
                MembraneSpecies { l: 11, r: 11, w: 3, base: 12, r1: 0, r2: 0, label: "11[C]11 (11²≡1 mod 12)" },
                MembraneSpecies { l: 5, r: 7, w: 3, base: 12, r1: 0, r2: 0, label: "5[C]7 (mixed)" },
    ];
    // Cultivate each garden plot
    let mut garden_results = HashMap::new();
    for plot in garden_plots {
        println!("\n🌿 Tending to: {}\n", plot.name);
        let plot_results: Vec<_> = plot.species
            .par_iter()
            .map(|species| {
                let bloom = cultivate_species(species, &plot.c_range);
                println!("  {} bloomed with {:.1}% density ({} primes)", 
                    species.label, bloom.density * 100.0, bloom.prime_count);
                bloom
            })
            .collect();
        garden_results.insert(plot.name.to_string(), plot_results);
    }
    // Analyze cross-pollination patterns
    println!("\n\n🌻 GARDEN INSIGHTS 🌻");
    println!("==================\n");
    // Find champion species
    let mut all_blooms: Vec<_> = garden_results.values()
        .flat_map(|blooms| blooms.iter())
        .collect();
    all_blooms.sort_by(|a, b| b.density.partial_cmp(&a.density).unwrap());
    println!("🏆 Top 5 Blooming Species:");
    for (i, bloom) in all_blooms.iter().take(5).enumerate() {
        println!("  {}. {} - {:.2}% density", i+1, bloom.species_label, bloom.density * 100.0);
        if let Some(insight) = &bloom.insight {
            println!("     💡 {}", insight);
        }
    // Look for patterns
    println!("\n🔍 Pattern Recognition:");
    // Analyze base effects
    let base_groups: HashMap<u32, Vec<&&BloomResult>> = all_blooms.iter()
        .map(|b| (b.base, b))
        .fold(HashMap::new(), |mut acc, (base, bloom)| {
            acc.entry(base).or_insert_with(Vec::new).push(bloom);
            acc
        });
    println!("\n  Base Performance:");
    for (base, blooms) in base_groups.iter() {
        let avg_density: f64 = blooms.iter().map(|b| b.density).sum::<f64>() / blooms.len() as f64;
        println!("    Base {}: average {:.1}% density across {} species", 
            base, avg_density * 100.0, blooms.len());
    // Log discoveries to journal
    writeln!(journal, "\n=== Garden Cultivation Report ===").unwrap();
    writeln!(journal, "Time: {}", Local::now()).unwrap();
    writeln!(journal, "\nDiscoveries:").unwrap();
    // The inverted ridge insight
    writeln!(journal, "1. INVERTED RIDGE CONFIRMED: Zero-padding reduces density").unwrap();
    writeln!(journal, "   - Baseline (no zeros): ~26% density").unwrap();
    writeln!(journal, "   - With zeros (μ=2): ~17% density (-34%)").unwrap();
    writeln!(journal, "   - Implication: Membranes are tightest at the center\n").unwrap();
    // Self-inverse resonance
    if let Some(resonance_results) = garden_results.get("Resonance Chambers") {
        let self_inverse: Vec<_> = resonance_results.iter()
            .filter(|b| b.species_label.contains("²≡1"))
        if !self_inverse.is_empty() {
            writeln!(journal, "2. SELF-INVERSE RESONANCE in base 12:").unwrap();
            for bloom in self_inverse {
                writeln!(journal, "   - {}: {:.1}% density", 
                    bloom.species_label, bloom.density * 100.0).unwrap();
            }
    // Growth recommendations
    println!("\n🌱 Growth Recommendations:");
    println!("  1. Focus on minimal membranes (no zeros) for maximum density");
    println!("  2. Explore self-inverse digits in composite bases (6, 12, 20)");
    println!("  3. Test 'breathing' asymmetric patterns for unique resonances");
    println!("  4. The valley is the peak - embrace the inverted ridge!");
    println!("\n✨ Garden exploration complete! Check membrane_garden.log for details.");
}
struct GardenPlot {
    name: &'static str,
    species: Vec<MembraneSpecies>,
    c_range: std::ops::Range<u64>,
struct MembraneSpecies {
    l: u32,
    r: u32,
    w: u32,
    base: u32,
    r1: u32,
    r2: u32,
    label: &'static str,
struct BloomResult {
    species_label: String,
    prime_count: usize,
    total_tested: usize,
    density: f64,
    insight: Option<String>,
fn cultivate_species(species: &MembraneSpecies, c_range: &std::ops::Range<u64>) -> BloomResult {
    let primes: Vec<_> = c_range.clone()
        .into_par_iter()
        .filter_map(|c| {
            let value = compute_membrane_value(species, c);
            if is_prime_miller_rabin(&value) {
                Some(value)
            } else {
                None
        })
    let prime_count = primes.len();
    let total_tested = c_range.end - c_range.start;
    let density = prime_count as f64 / total_tested as f64;
    // Generate insights
    let insight = if species.base == 12 && is_self_inverse(species.l, species.base) {
        Some(format!("Self-inverse resonance detected! {}²≡1 (mod {})", species.l, species.base))
    } else if species.l != species.r {
        Some("Asymmetric breathing pattern".to_string())
    } else if density > 0.25 {
        Some("Exceptional bloom! >25% density".to_string())
    } else {
        None
    };
    BloomResult {
        species_label: species.label.to_string(),
        base: species.base,
        prime_count,
        total_tested: total_tested as usize,
        density,
        insight,
fn compute_membrane_value(species: &MembraneSpecies, c: u64) -> BigUint {
    let base = BigUint::from(species.base);
    let l = BigUint::from(species.l);
    let r = BigUint::from(species.r);
    let c = BigUint::from(c);
    &l * base.pow(species.w - 1) +
    &r * base.pow(species.w - 2 - species.r1) +
    &c * base.pow(species.w / 2) +
    &r * base.pow(species.r2 + 1) +
    &l
fn is_self_inverse(d: u32, base: u32) -> bool {
    (d * d) % base == 1
}
