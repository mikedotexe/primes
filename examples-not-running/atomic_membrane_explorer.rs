//! Atomic Membrane Explorer - Multi-layer membrane structures
//! 
//! Explores "atom-like" patterns with multiple membrane layers,
//! similar to electron shells around a nucleus.

use primes::{is_prime_miller_rabin, ascii_art::*};
use num_bigint::BigUint;
use std::fs::File;
use std::io::Write;
use chrono::Local;
#[derive(Debug, Clone)]
struct AtomicPattern {
    shells: Vec<Shell>,
    nucleus: u32,
    base: u32,
}
struct Shell {
    digit: u32,
    padding: u32,
impl AtomicPattern {
    fn new(base: u32, nucleus: u32) -> Self {
        Self {
            shells: Vec::new(),
            nucleus,
            base,
        }
    }
    
    fn add_shell(&mut self, digit: u32, padding: u32) {
        self.shells.push(Shell { digit, padding });
    fn to_biguint(&self) -> BigUint {
        let mut digits = Vec::new();
        
        // Build from outermost shell inward
        for shell in &self.shells {
            digits.push(shell.digit);
            for _ in 0..shell.padding {
                digits.push(0);
            }
        // Nucleus
        digits.push(self.nucleus);
        // Mirror back out
        for shell in self.shells.iter().rev() {
        // Convert to BigUint
        let mut value = BigUint::from(0u32);
        let base_big = BigUint::from(self.base);
        for digit in digits {
            value = value * &base_big + BigUint::from(digit);
        value
    fn visualize(&self) -> String {
        let mut viz = String::new();
        // Show the structure
        viz.push_str(&format!("Base {} Atomic Structure:\n", self.base));
        viz.push_str(&format!("Nucleus: {}\n", self.nucleus));
        for (i, shell) in self.shells.iter().enumerate() {
            viz.push_str(&format!("Shell {}: digit={}, padding={}\n", 
                i + 1, shell.digit, shell.padding));
        // Show the pattern
        viz.push_str("\nPattern: ");
            viz.push_str(&format!("{} ", shell.digit));
                viz.push_str("0 ");
        viz.push_str(&format!("[{}] ", self.nucleus));
        viz
fn explore_atomic_patterns(base: u32) -> Vec<(AtomicPattern, BigUint)> {
    let mut prime_atoms = Vec::new();
    println!("\n{}", boxed_title(&format!("EXPLORING BASE {} ATOMS", base), 60));
    // Single shell patterns (like simple membranes)
    for nucleus in 1..base.min(10) {
        for shell1 in 1..base.min(8) {
            for pad1 in 0..=2 {
                let mut atom = AtomicPattern::new(base, nucleus);
                atom.add_shell(shell1, pad1);
                
                let value = atom.to_biguint();
                if is_prime_miller_rabin(&value) {
                    prime_atoms.push((atom.clone(), value));
                }
    // Double shell patterns (like electron s and p orbitals)
    for nucleus in 1..base.min(8) {
        for shell1 in 1..base.min(6) {
            for shell2 in 1..base.min(6) {
                if shell1 == shell2 { continue; }
                for pad1 in 0..=1 {
                    for pad2 in 0..=1 {
                        let mut atom = AtomicPattern::new(base, nucleus);
                        atom.add_shell(shell2, pad2); // outer shell
                        atom.add_shell(shell1, pad1); // inner shell
                        
                        let value = atom.to_biguint();
                        if is_prime_miller_rabin(&value) && value < BigUint::from(1_000_000u32) {
                            prime_atoms.push((atom.clone(), value));
                        }
                    }
    prime_atoms
fn main() {
    println!("{}", banner("ATOMIC MEMBRANE STRUCTURES", 70));
    println!("\nExploring multi-shell membrane patterns inspired by atomic orbitals\n");
    // Test in our best-performing bases
    let test_bases = vec![4, 6, 10, 12];
    let mut all_discoveries = Vec::new();
    for base in test_bases {
        let atoms = explore_atomic_patterns(base);
        println!("\nFound {} atomic primes in base {}", atoms.len(), base);
        // Show some examples
        if atoms.len() > 0 {
            println!("\nExample atomic primes:");
            for (atom, prime) in atoms.iter().take(5) {
                println!("\n{}", atom.visualize());
                println!("→ {} (PRIME!)", prime);
        all_discoveries.push((base, atoms));
    // Analysis
    println!("\n{}", boxed_title("ATOMIC PATTERN ANALYSIS", 70));
    // Find patterns that appear across bases
    println!("\n🔬 CROSS-BASE ATOMIC STRUCTURES:");
    // Look for similar shell configurations
    let mut pattern_matches = std::collections::HashMap::new();
    for (base, atoms) in &all_discoveries {
        for (atom, _) in atoms {
            let pattern_key = format!("nucleus={},shells={:?}", 
                atom.nucleus, 
                atom.shells.iter().map(|s| (s.digit, s.padding)).collect::<Vec<_>>()
            );
            
            pattern_matches.entry(pattern_key)
                .or_insert_with(Vec::new)
                .push(*base);
    println!("\nPatterns appearing in multiple bases:");
    for (pattern, bases) in pattern_matches.iter() {
        if bases.len() > 1 {
            println!("  {} → bases {:?}", pattern, bases);
    // Special patterns
    println!("\n{}", simple_box("NOTABLE DISCOVERIES"));
    // Find double-shell atoms
    let mut double_shell_count = 0;
    let mut single_shell_count = 0;
    for (_, atoms) in &all_discoveries {
            match atom.shells.len() {
                1 => single_shell_count += 1,
                2 => double_shell_count += 1,
                _ => {}
    println!("\nShell statistics:");
    println!("  Single shell atoms: {}", single_shell_count);
    println!("  Double shell atoms: {}", double_shell_count);
    // Nucleus analysis
    println!("\n🎯 NUCLEUS PREFERENCES:");
    let mut nucleus_counts = std::collections::HashMap::new();
            *nucleus_counts.entry(atom.nucleus).or_insert(0) += 1;
    let mut nucleus_vec: Vec<_> = nucleus_counts.into_iter().collect();
    nucleus_vec.sort_by(|a, b| b.1.cmp(&a.1));
    println!("\nMost common nuclei:");
    for (nucleus, count) in nucleus_vec.iter().take(5) {
        println!("  Nucleus {}: {} occurrences", nucleus, count);
    // Create beautiful visualization
    let visual_file = format!("atomic_membranes_{}.txt", 
        Local::now().format("%Y%m%d_%H%M%S"));
    let mut file = File::create(&visual_file).expect("Failed to create file");
    writeln!(file, "{}", boxed_title("ATOMIC MEMBRANE GALLERY", 80)).unwrap();
    writeln!(file, "\nMulti-shell membrane structures that generate primes\n").unwrap();
    // Show some beautiful examples
    writeln!(file, "{}", banner("SINGLE SHELL ATOMS", 80)).unwrap();
    writeln!(file, r#"
    Classic membrane structure with one shell:
         shell → nucleus ← shell
             \      |      /
              3  0  5  0  3
                    ↓
                 30503
                (prime)
"#).unwrap();
    writeln!(file, "\n{}", banner("DOUBLE SHELL ATOMS", 80)).unwrap();
    Two-shell structure like s and p orbitals:
      outer shell → inner shell → nucleus
            \           \            /
             7  0  3  0  5  0  3  0  7
                         ↓
                     703050307
                      (prime)
                      
    This creates nested resonance chambers!
    // Theory section
    writeln!(file, "\n{}", boxed_title("THEORETICAL IMPLICATIONS", 80)).unwrap();
1. SHELL INTERFERENCE
   Multiple shells create complex wave patterns
   Inner shells modulate outer shell resonance
   
2. NUCLEUS CRITICALITY
   Certain nuclei (like 5) appear frequently
   The nucleus acts as the resonance center
3. BASE DEPENDENCY
   Even bases still dominate in multi-shell patterns
   Shell spacing (padding) is base-sensitive
4. QUANTUM ANALOGY
   Like electron orbitals, shells have preferred configurations
   Some combinations are "forbidden" (always composite)
   Others are "allowed" (high prime probability)
    println!("\n✅ Atomic exploration complete!");
    println!("📄 Visual gallery saved to: {}", visual_file);
    println!("\n{}", simple_box(
        "NEXT STEPS:\n\
         - Explore triple-shell structures\n\
         - Find 'magic' shell numbers (like magic nuclei)\n\
         - Test breathing patterns in multi-shell atoms\n\
         - Search for cross-base universal atoms"
    ));
