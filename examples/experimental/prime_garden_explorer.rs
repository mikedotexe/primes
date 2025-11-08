//! Prime Garden Explorer - Growing primes from different membrane seeds
//! 
//! This creates a beautiful "garden" visualization showing how different
//! configurations bloom into prime numbers, with ASCII art flowers!

use prime_physics_engine::{is_prime_miller_rabin, ascii_art::*};
use num_bigint::BigUint;
use std::fs::File;
use std::io::Write;
use chrono::Local;
use std::collections::HashMap;
#[derive(Debug, Clone)]
struct PrimeFlower {
    base: u32,
    config: String,
    seeds: Vec<u32>,
    primes: Vec<BigUint>,
    density: f64,
    flower_type: FlowerType,
}
enum FlowerType {
    Rose,      // High density (>40%)
    Tulip,     // Medium density (20-40%)
    Daisy,     // Low density (10-20%)
    Weed,      // Very low (<10%)
impl PrimeFlower {
    fn new(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32) -> Self {
        let config = format!("({},{}) k=({},{})", outer, inner, k_outer, k_inner);
        let mut seeds = Vec::new();
        let mut primes = Vec::new();
        
        // Test seeds 0-99
        for seed in 0..100 {
            let number = generate_membrane_number(base, outer, inner, k_outer, k_inner, seed);
            
            if is_prime_miller_rabin(&number) {
                seeds.push(seed);
                if primes.len() < 10 {
                    primes.push(number);
                }
            }
        }
        let density = seeds.len() as f64 / 100.0;
        let flower_type = match (density * 100.0) as u32 {
            40..=100 => FlowerType::Rose,
            20..=39 => FlowerType::Tulip,
            10..=19 => FlowerType::Daisy,
            _ => FlowerType::Weed,
        };
        Self {
            base,
            config,
            seeds,
            primes,
            density,
            flower_type,
    }
    
    fn bloom(&self) -> String {
        match self.flower_type {
            FlowerType::Rose => self.draw_rose(),
            FlowerType::Tulip => self.draw_tulip(),
            FlowerType::Daisy => self.draw_daisy(),
            FlowerType::Weed => self.draw_weed(),
    fn draw_rose(&self) -> String {
        format!(r#"
        🌹 ROSE ({:.0}% density)
           @@@@@@@
         @@@@@@@@@@@ 
        @@@@@@@@@@@@@
        @@@@@@@@@@@@@ Config: {}
         @@@@@@@@@@@  Base: {}
          |@@@@@@@|   Seeds: {} primes
          |   |   |   
         /|\  |  /|\  Example: {}
        "#, 
        self.density * 100.0,
        self.config,
        self.base,
        self.seeds.len(),
        if !self.primes.is_empty() { self.primes[0].to_string() } else { "none".to_string() }
        )
    fn draw_tulip(&self) -> String {
        🌷 TULIP ({:.0}% density)
          (\___/)
          ( o.o )    Config: {}
           > ^ <     Base: {}
            | |      Seeds: {} primes
           /| |\     
          //| |\\    Example: {}
        "#,
    fn draw_daisy(&self) -> String {
        🌼 DAISY ({:.0}% density)
         _ _ _ _
        |_|_|_|_|    Config: {}
        \_|_|_|_/    Base: {}
          |_|_|      Seeds: {} primes
            |        
           /|\       Example: {}
    fn draw_weed(&self) -> String {
        🌱 WEED ({:.0}% density)
          \|/       Config: {}
           |        Base: {}
          /|\       Seeds: {} primes
        self.seeds.len()
fn generate_membrane_number(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, middle: u32) -> BigUint {
    let mut digits = vec![outer];
    for _ in 0..k_outer { digits.push(0); }
    digits.push(inner);
    for _ in 0..k_inner { digits.push(0); }
    digits.push(middle);
    digits.push(outer);
    let mut value = BigUint::from(0u32);
    let base_big = BigUint::from(base);
    for digit in digits {
        value = value * &base_big + BigUint::from(digit);
    value
fn plant_garden(base: u32) -> Vec<PrimeFlower> {
    let mut garden = Vec::new();
    println!("\n{}", boxed_title(&format!("PLANTING BASE {} GARDEN", base), 60));
    // Plant various configurations
    let configs = vec![
        (3, 7, 0, 1),  // Breathing pattern
        (3, 7, 1, 1),  // Symmetric pattern
        (5, 7, 0, 1),  // 5-7 breathing
        (5, 7, 1, 1),  // 5-7 symmetric
        (1, 3, 0, 0),  // Minimal
        (1, base-1, 0, 0),  // Extreme spread
    ];
    for (outer, inner, k_outer, k_inner) in configs {
        if outer < base && inner < base {
            let flower = PrimeFlower::new(base, outer, inner, k_outer, k_inner);
            if flower.seeds.len() > 0 {  // Only plant if it produces primes
                garden.push(flower);
    // Sort by density (best flowers first)
    garden.sort_by(|a, b| b.density.partial_cmp(&a.density).unwrap());
    garden
fn create_garden_map(gardens: &HashMap<u32, Vec<PrimeFlower>>) -> String {
    let mut map = String::new();
    map.push_str(&format!("\n{}", boxed_title("PRIME GARDEN MAP", 80)));
    map.push_str("\n\nA botanical guide to prime-generating configurations\n\n");
    // Legend
    map.push_str("LEGEND:\n");
    map.push_str("🌹 Rose  = 40%+ density (exceptional)\n");
    map.push_str("🌷 Tulip = 20-40% density (good)\n");
    map.push_str("🌼 Daisy = 10-20% density (moderate)\n");
    map.push_str("🌱 Weed  = <10% density (poor)\n\n");
    // Garden layout
    map.push_str("THE GARDENS:\n");
    map.push_str("═══════════════════════════════════════════════════════════\n\n");
    for (base, flowers) in gardens {
        map.push_str(&format!("Base {} Garden:\n", base));
        map.push_str("─────────────────\n");
        let mut row = String::new();
        for (i, flower) in flowers.iter().enumerate() {
            let symbol = match flower.flower_type {
                FlowerType::Rose => "🌹",
                FlowerType::Tulip => "🌷",
                FlowerType::Daisy => "🌼",
                FlowerType::Weed => "🌱",
            };
            row.push_str(&format!("{} ", symbol));
            if (i + 1) % 6 == 0 {
                map.push_str(&format!("{}\n", row));
                row.clear();
        if !row.is_empty() {
            map.push_str(&format!("{}\n", row));
        map.push_str("\n");
    map
fn main() {
    println!("{}", banner("PRIME GARDEN EXPLORER", 80));
    println!("\nCultivating prime numbers from membrane configuration seeds\n");
    // Plant gardens in different bases
    let bases = vec![6, 8, 10, 12];
    let mut all_gardens = HashMap::new();
    for base in &bases {
        let garden = plant_garden(*base);
        println!("\nBase {} garden has {} flowering configurations", base, garden.len());
        // Show the best flowers
        if let Some(best) = garden.first() {
            println!("\nPrize flower:");
            println!("{}", best.bloom());
        all_gardens.insert(*base, garden);
    // Cross-pollination analysis
    println!("\n{}", boxed_title("CROSS-POLLINATION STUDY", 80));
    let mut cross_patterns: HashMap<String, Vec<u32>> = HashMap::new();
    for (base, garden) in &all_gardens {
        for flower in garden {
            cross_patterns.entry(flower.config.clone())
                .or_insert_with(Vec::new)
                .push(*base);
    println!("\nPatterns that bloom across multiple bases:");
    for (pattern, bases) in cross_patterns.iter() {
        if bases.len() > 1 {
            println!("  {} grows in bases: {:?}", pattern, bases);
    // Create beautiful output file
    let filename = format!("prime_garden_{}.txt", 
        Local::now().format("%Y%m%d_%H%M%S"));
    let mut file = File::create(&filename).expect("Failed to create file");
    writeln!(file, "{}", boxed_title("THE PRIME GARDEN", 100)).unwrap();
    writeln!(file, "\nA botanical exploration of prime-generating membrane configurations\n").unwrap();
    // Garden map
    writeln!(file, "{}", create_garden_map(&all_gardens)).unwrap();
    // Detailed flower catalog
    writeln!(file, "\n{}", banner("FLOWER CATALOG", 100)).unwrap();
        writeln!(file, "\n{}", separator("wave", 100)).unwrap();
        writeln!(file, "BASE {} SPECIMENS", base).unwrap();
        writeln!(file, "{}", separator("wave", 100)).unwrap();
        for flower in garden.iter().take(3) {
            writeln!(file, "{}", flower.bloom()).unwrap();
            // Show seed distribution
            writeln!(file, "\nSeed distribution:").unwrap();
            write!(file, "Seeds that bloom: ").unwrap();
            for (i, seed) in flower.seeds.iter().enumerate() {
                if i > 0 { write!(file, ", ").unwrap(); }
                write!(file, "{}", seed).unwrap();
                if i >= 10 { 
                    write!(file, "...").unwrap(); 
                    break; 
            writeln!(file).unwrap();
    // Growing guide
    writeln!(file, "\n{}", boxed_title("GARDENING GUIDE", 100)).unwrap();
    writeln!(file, r#"
How to Grow Prime Flowers
=========================
1. CHOOSE YOUR BASE (Soil Type):
   - Even bases (6,8,10,12): Rich, fertile soil
   - Odd bases (5,7,9,11): Challenging terrain
   - Prime bases: Unique mineral content
2. SELECT YOUR CONFIGURATION (Seed Type):
   - (3,7): Classic variety, reliable bloomer
   - (5,7): Twin prime special, needs space
   - (1,3): Compact variety, good for small gardens
3. ADJUST YOUR PADDING (Watering Schedule):
   - k=(0,0): Drought resistant
   - k=(0,1): Breathing pattern, needs alternating water
   - k=(1,1): Symmetric watering, steady growth
   - k=(2,2): Deep watering, slow growth
4. NURTURE YOUR SEEDS (Middle Values):
   - Seeds 0-9: Test each one
   - Some configs only bloom with specific seeds
   - Document which seeds produce primes
5. HARVEST YOUR PRIMES:
   - Roses (40%+): Prize winners!
   - Tulips (20-40%): Reliable producers
   - Daisies (10-20%): Modest but pretty
   - Weeds (<10%): Consider replanting
"#).unwrap();
    // ASCII art garden scene
    writeln!(file, "\n{}", banner("THE PRIME GARDEN AT SUNSET", 100)).unwrap();
                                    ☀️
                                   ╱  ╲
                                  ╱    ╲
    🌹    🌷    🌼         🌹    🌷         🌼    🌱
    /|\   /|\   /|\       /|\   /|\       /|\   /|\
   / | \ / | \ / | \     / | \ / | \     / | \ / | \
  ───┴───┴───┴───┴───────┴───┴───┴───────┴───┴───┴───
  Base 6 Section      Base 10 Section   Base 12 Section
  
  "In the garden of mathematics, prime configurations bloom eternal"
    // Summary statistics
    writeln!(file, "\n{}", boxed_title("GARDEN STATISTICS", 100)).unwrap();
    let total_flowers: usize = all_gardens.values().map(|g| g.len()).sum();
    let total_roses: usize = all_gardens.values()
        .flat_map(|g| g.iter())
        .filter(|f| matches!(f.flower_type, FlowerType::Rose))
        .count();
    writeln!(file, "\nTotal configurations planted: {}", total_flowers).unwrap();
    writeln!(file, "Prize roses grown: {}", total_roses).unwrap();
    writeln!(file, "Most fertile base: {}", 
        all_gardens.iter()
            .max_by_key(|(_, g)| g.len())
            .map(|(b, _)| b)
            .unwrap()
    ).unwrap();
    println!("\n✅ Prime garden exploration complete!");
    println!("🌻 Garden catalog saved to: {}", filename);
    println!("\n{}", simple_box(
        "WISDOM FROM THE GARDEN:\n\
         - Even bases are fertile soil for primes\n\
         - Breathing patterns help flowers bloom\n\
         - The 5-7 configuration is a prize variety\n\
         - Every base has its own unique ecosystem"
    ));
