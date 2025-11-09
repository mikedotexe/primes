use std::str::FromStr;//! Atomic Prime Explorer - Finds "atom-looking" primes with 5 at the center
//! 
//! These are membrane configurations that create symmetric, atom-like structures:
//! - Single membrane: L 0...0 5 0...0 L
//! - Double membrane: L 0...0 R 0...0 5 0...0 R 0...0 L
//! - Triple membrane: L 0...0 R1 0...0 R2 0...0 5 0...0 R2 0...0 R1 0...0 L

use primes::{
    is_prime_miller_rabin,
};
use std::fs::{File, OpenOptions};
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AtomicPrime {
    structure_type: String,
    membrane_count: u32,
    configuration: String,
    visual_pattern: String,
    prime_value: String,
    digit_count: usize,
    verification_url: String,
    discovered_at: String,
}
struct AtomicFamily {
    center_value: u32,
    base: u32,
    families_found: Vec<AtomicPrime>,
    search_summary: SearchSummary,
struct SearchSummary {
    configurations_tested: u32,
    primes_found: u32,
    timestamp: String,
    most_beautiful: Option<String>,
// Single membrane: (L) 0...0 (5) 0...0 (L)
fn generate_single_membrane(outer: u32, k: u32) -> BigUint {
    let pattern = format!("{}{}{}{}{}", 
        outer, 
        "0".repeat(k as usize),
        "5",
        outer
    );
    pattern.parse().unwrap()
// Double membrane: (L) 0...0 (R) 0...0 (5) 0...0 (R) 0...0 (L)
fn generate_double_membrane(outer: u32, inner: u32, k_outer: u32, k_inner: u32) -> BigUint {
    let pattern = format!("{}{}{}{}{}{}{}{}{}", 
        outer,
        "0".repeat(k_outer as usize),
        inner,
        "0".repeat(k_inner as usize),
// Triple membrane: (L) 0...0 (R1) 0...0 (R2) 0...0 (5) 0...0 (R2) 0...0 (R1) 0...0 (L)
fn generate_triple_membrane(outer: u32, middle: u32, inner: u32, k_outer: u32, k_middle: u32, k_inner: u32) -> BigUint {
    let pattern = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}", 
        middle,
        "0".repeat(k_middle as usize),
fn create_visual_pattern(structure: &str, params: &[u32]) -> String {
    match structure {
        "single" => {
            let (outer, k) = (params[0], params[1]);
            format!("({}){}─(5)─{}({})", outer, "─".repeat(k as usize), "─".repeat(k as usize), outer)
        },
        "double" => {
            let (outer, inner, k_outer, k_inner) = (params[0], params[1], params[2], params[3]);
            format!("({}){}─({}){}─(5)─{}({})─{}({})", 
                outer, "─".repeat(k_outer as usize),
                inner, "─".repeat(k_inner as usize),
                "─".repeat(k_inner as usize), inner,
                "─".repeat(k_outer as usize), outer
            )
        "triple" => {
            let (outer, middle, inner, k_outer, k_middle, k_inner) = 
                (params[0], params[1], params[2], params[3], params[4], params[5]);
            format!("({}){}─({}){}─({}){}─(5)─{}({})─{}({})─{}({})", 
                middle, "─".repeat(k_middle as usize),
                "─".repeat(k_middle as usize), middle,
        _ => "Unknown".to_string()
    }
fn main() {
    println!("⚛️  Atomic Prime Explorer");
    println!("========================");
    println!("Searching for atom-looking primes with 5 at the center...\n");
    
    let mut atomic_primes = Vec::new();
    let mut configs_tested = 0;
    // Search single membrane structures
    println!("🔍 Searching single membrane structures...");
    for outer in [1, 3, 5, 7, 9].iter() {
        for k in 0..=5 {
            configs_tested += 1;
            let candidate = generate_single_membrane(*outer, k);
            
            if is_prime_miller_rabin(&candidate, 20) {
                let visual = create_visual_pattern("single", &[*outer, k]);
                println!("  ✨ Found: {} → {}", visual, candidate);
                
                atomic_primes.push(AtomicPrime {
                    structure_type: "Single Membrane".to_string(),
                    membrane_count: 1,
                    configuration: format!("outer={}, k={}", outer, k),
                    visual_pattern: visual,
                    prime_value: candidate.to_string(),
                    digit_count: candidate.to_string().len(),
                    verification_url: format!("https://www.wolframalpha.com/input/?i=isprime({})", candidate),
                    discovered_at: Local::now().to_rfc3339(),
                });
            }
        }
    // Search double membrane structures
    println!("\n🔍 Searching double membrane structures...");
        for inner in [1, 3, 5, 7, 9].iter() {
            if inner == outer { continue; } // Skip symmetric cases
            for k_outer in 0..=3 {
                for k_inner in 0..=3 {
                    configs_tested += 1;
                    let candidate = generate_double_membrane(*outer, *inner, k_outer, k_inner);
                    
                    if is_prime_miller_rabin(&candidate, 20) {
                        let visual = create_visual_pattern("double", &[*outer, *inner, k_outer, k_inner]);
                        println!("  ✨ Found: {} → {}", visual, candidate);
                        
                        atomic_primes.push(AtomicPrime {
                            structure_type: "Double Membrane".to_string(),
                            membrane_count: 2,
                            configuration: format!("outer={}, inner={}, k_outer={}, k_inner={}", 
                                outer, inner, k_outer, k_inner),
                            visual_pattern: visual,
                            prime_value: candidate.to_string(),
                            digit_count: candidate.to_string().len(),
                            verification_url: format!("https://www.wolframalpha.com/input/?i=isprime({})", candidate),
                            discovered_at: Local::now().to_rfc3339(),
                        });
                    }
                }
    // Search triple membrane structures
    println!("\n🔍 Searching triple membrane structures...");
        for middle in [1, 3, 5, 7, 9].iter() {
            for inner in [1, 3, 5, 7, 9].iter() {
                if inner == middle || middle == outer { continue; }
                for k_outer in 0..=2 {
                    for k_middle in 0..=2 {
                        for k_inner in 0..=2 {
                            configs_tested += 1;
                            let candidate = generate_triple_membrane(
                                *outer, *middle, *inner, k_outer, k_middle, k_inner
                            );
                            
                            if is_prime_miller_rabin(&candidate, 20) {
                                let visual = create_visual_pattern("triple", 
                                    &[*outer, *middle, *inner, k_outer, k_middle, k_inner]);
                                println!("  ✨ Found: {} → {}", visual, candidate);
                                
                                atomic_primes.push(AtomicPrime {
                                    structure_type: "Triple Membrane".to_string(),
                                    membrane_count: 3,
                                    configuration: format!(
                                        "outer={}, middle={}, inner={}, k_outer={}, k_middle={}, k_inner={}", 
                                        outer, middle, inner, k_outer, k_middle, k_inner
                                    ),
                                    visual_pattern: visual,
                                    prime_value: candidate.to_string(),
                                    digit_count: candidate.to_string().len(),
                                    verification_url: format!("https://www.wolframalpha.com/input/?i=isprime({})", candidate),
                                    discovered_at: Local::now().to_rfc3339(),
                                });
                            }
                        }
    // Find most beautiful (subjective: prefer symmetric k values and distinct digits)
    let most_beautiful = atomic_primes.iter()
        .filter(|p| p.membrane_count >= 2)
        .min_by_key(|p| {
            // Score based on pattern beauty (lower is better)
            let symmetry_score = if p.visual_pattern.contains("─(") && 
                                   p.visual_pattern.matches("─").count() % 2 == 0 { 0 } else { 10 };
            let length_score = p.digit_count;
            symmetry_score + length_score
        })
        .map(|p| p.visual_pattern.clone());
    // Build the atomic family report
    let atomic_family = AtomicFamily {
        center_value: 5,
        base: 10,
        families_found: atomic_primes.clone(),
        search_summary: SearchSummary {
            configurations_tested: configs_tested,
            primes_found: atomic_primes.len() as u32,
            timestamp: Local::now().to_rfc3339(),
            most_beautiful,
    };
    // Save/append to file
    let filename = "atomic_primes_catalog.json";
    let file_exists = std::path::Path::new(filename).exists();
    if file_exists {
        println!("\n📁 Appending to existing catalog...");
        // Read existing data
        let existing_data = std::fs::read_to_string(filename).unwrap_or_else(|_| "[]".to_string());
        let mut catalog: Vec<AtomicFamily> = serde_json::from_str(&existing_data).unwrap_or_else(|_| Vec::new());
        catalog.push(atomic_family);
        
        // Write updated catalog
        let json = serde_json::to_string_pretty(&catalog).expect("Failed to serialize catalog");
        let mut file = File::create(filename).expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write catalog");
    } else {
        println!("\n📁 Creating new catalog...");
        let catalog = vec![atomic_family];
    // Print summary
    println!("\n" + "=".repeat(60));
    println!("⚛️  ATOMIC PRIME SUMMARY");
    println!("=".repeat(60));
    println!("Configurations tested: {}", configs_tested);
    println!("Atomic primes found: {}", atomic_primes.len());
    println!("\n📊 By membrane count:");
    let single_count = atomic_primes.iter().filter(|p| p.membrane_count == 1).count();
    let double_count = atomic_primes.iter().filter(|p| p.membrane_count == 2).count();
    let triple_count = atomic_primes.iter().filter(|p| p.membrane_count == 3).count();
    println!("  Single membrane: {}", single_count);
    println!("  Double membrane: {}", double_count);
    println!("  Triple membrane: {}", triple_count);
    // Show some beautiful examples
    println!("\n✨ Most beautiful structures:");
    for prime in atomic_primes.iter()
        .take(5) {
        println!("\n  {}", prime.visual_pattern);
        println!("  → {} ({} digits)", prime.prime_value, prime.digit_count);
        println!("  Verify: {}", prime.verification_url);
    println!("\n✅ Results saved to: {}", filename);
    // Create a separate visual file for easy viewing
    let visual_filename = format!("atomic_primes_visual_{}.txt", Local::now().format("%Y%m%d_%H%M%S"));
    let mut visual_file = File::create(&visual_filename).expect("Failed to create visual file");
    writeln!(visual_file, "⚛️  ATOMIC PRIMES WITH CENTER 5").unwrap();
    writeln!(visual_file, "================================\n").unwrap();
    for (i, prime) in atomic_primes.iter().enumerate() {
        writeln!(visual_file, "{}. {}", i + 1, prime.visual_pattern).unwrap();
        writeln!(visual_file, "   Value: {}", prime.prime_value).unwrap();
        writeln!(visual_file, "   Type: {}", prime.structure_type).unwrap();
        writeln!(visual_file, "   Config: {}", prime.configuration).unwrap();
        writeln!(visual_file, "").unwrap();
    println!("📄 Visual representation saved to: {}", visual_filename);
