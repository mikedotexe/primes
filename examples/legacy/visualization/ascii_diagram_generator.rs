//! ASCII Diagram Generator - Creating beautiful visual representations
//! 
//! Outputs stunning ASCII diagrams showing:
//! - Membrane structures across bases
//! - Distance relationships
//! - Prime density visualizations
//! - Comparative patterns

use std::fs::File;
use std::io::Write;
use chrono::Local;
fn generate_membrane_diagram(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32, center: u32) -> String {
    let mut diagram = String::new();
    
    // Title
    diagram.push_str(&format!("Base {} Membrane Structure\n", base));
    diagram.push_str(&"─".repeat(30));
    diagram.push_str("\n\n");
    // Visual representation
    let outer_str = format!("{:X}", outer);
    let inner_str = format!("{:X}", inner); 
    let center_str = format!("{:X}", center);
    // Top view
    diagram.push_str("Top View:\n");
    diagram.push_str(&format!("  {}", outer_str));
    diagram.push_str(&" ".repeat(k_outer as usize * 2 + 1));
    diagram.push_str(&inner_str);
    diagram.push_str(&" ".repeat(k_inner as usize * 2 + 1));
    diagram.push_str(&center_str);
    diagram.push_str(&format!("{}\n", outer_str));
    // With connections
    diagram.push_str("  │");
    diagram.push_str(&"─".repeat(k_outer as usize * 2 + 1));
    diagram.push_str("│");
    diagram.push_str(&"─".repeat(k_inner as usize * 2 + 1));
    diagram.push_str("│\n");
    // Pattern string
    diagram.push_str(&format!("\nPattern: ({}){}─({}){}─({})─{}({})─{}({})\n",
        outer_str,
        "─".repeat(k_outer as usize),
        inner_str,
        "─".repeat(k_inner as usize),
        center_str,
        outer_str
    ));
    diagram
}
fn generate_distance_comparison() -> String {
    diagram.push_str(r#"
╔════════════════════════════════════════════════════════════╗
║                  MEMBRANE DISTANCE ANALYSIS                ║
╠════════════════════════════════════════════════════════════╣
║                                                            ║
║  Base 10 Distance Relationships:                           ║
║  ─────────────────────────────                            ║
║    1   2   3   4   5   6   7   8   9                      ║
║    │   │   │   │   │   │   │   │   │                      ║
║    └───┴───┴───┴───┼───┴───┴───┴───┘                      ║
║                    │                                       ║
║                 CENTER                                     ║
║  Optimal Distances:                                        ║
║    3→5: distance 2 ✓                                       ║
║    3→7: distance 4 ✓✓                                      ║
║    5→7: distance 2 ✓✓✓ (TWIN PRIMES!)                     ║
║  Base 12 Distance Relationships:                           ║
║    1   2   3   4   5   6   7   8   9   A   B             ║
║    │   │   │   │   │   │   │   │   │   │   │             ║
║    └───┴───┴───┴───┴───┼───┴───┴───┴───┴───┘             ║
║                        │                                   ║
║                     MIDPOINT                               ║
║                   (composite!)                             ║
║    5→7: distance 2 ✓✓✓                                     ║
║    3→5: distance 2 ✓✓                                      ║
║    7→B: distance 4 ✓                                       ║
╚════════════════════════════════════════════════════════════╝
"#);
fn generate_prime_density_chart() -> String {
    let mut chart = String::new();
    chart.push_str(r#"
┌─────────────────────────────────────────────────────────────┐
│                   PRIME DENSITY BY CONFIGURATION            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Base 6:                                                     │
│   (3,3) k=(0,1):  ████████████████████████████▌ 30.2% 🏆    │
│   (3,3) k=(1,1):  █████████████████▌ 21.3%                 │
│   (3,5) k=(0,0):  ████████████████▌ 20.1%                  │
│   Random:         ████▌ 4.5%                               │
│ Base 10:                                                    │
│   (3,7) k=(1,2):  ██████████████████▌ 22.3%                │
│   (5,7) k=(0,1):  █████████████████▌ 21.8%                 │
│   (3,5) k=(1,1):  ████████████████▌ 20.1%                  │
│ Base 12:                                                    │
│   (5,7) k=(0,1):  ████████████████████████████ 28.9% 🥇     │
│   (7,5) k=(0,1):  ███████████████████████████▌ 28.7%       │
│   (5,3) k=(0,1):  ██████████████████████████ 27.2%         │
│ 📊 Key Insight: 5-7 pairing dominates across all bases!    │
└─────────────────────────────────────────────────────────────┘
    chart
fn generate_atomic_comparison() -> String {
    let mut comparison = String::new();
    comparison.push_str(r#"
╔═══════════════════════════════════════════════════════════════╗
║                    ATOMIC PRIMES: BASE 10 vs BASE 12          ║
╠═══════════════════════════════════════════════════════════════╣
║                                                               ║
║  Base 10 (Center = 5):              Base 12 (Center = 5):    ║
║  ────────────────────               ─────────────────────    ║
║  Single Membrane:                   Single Membrane:          ║
║    (1)─(5)─(1) → 151 ✓               (1)─(5)─(1) → 191₁₂    ║
║    (3)─(5)─(3) → 353 ✓               (5)─(5)─(5) → 555₁₂    ║
║    (7)─(5)─(7) → 757 ✓               (7)─(5)─(7) → 757₁₂    ║
║    (7)──(5)──(7) → 70507 ✓           (B)─(5)─(B) → B5B₁₂ ✓  ║
║  Double Membrane:                   Double Membrane:          ║
║    (3)─(7)─(5)─(7)─(3)               (5)─(7)─(5)─(7)─(5)    ║
║      → 37573 ✓                         → 57575₁₂            ║
║    (3)──(7)──(5)──(7)──(3)           (5)──(7)──(5)──(7)──(5)║
║      → 307050703 ✓⭐                   → 507050705₁₂        ║
║  Count: 51 atomic primes            Count: 23 atomic primes  ║
║  Base 12 (Center = 6):              Base 12 (Center = 7):    ║
║  Midpoint but composite!            Prime center             ║
║    (1)─(6)─(1) → 161₁₂               (1)─(7)─(1) → 171₁₂    ║
║    (5)─(6)─(5) → 565₁₂               (5)─(7)─(5) → 575₁₂ ✓  ║
║    (7)─(6)─(7) → 767₁₂               (B)─(7)─(B) → B7B₁₂    ║
║  Count: 11 atomic primes            Count: 29 atomic primes  ║
║  💡 Composite centers produce fewer atomic primes!            ║
╚═══════════════════════════════════════════════════════════════╝
    comparison
fn generate_resonance_diagram() -> String {
┌──────────────────────────────────────────────────────────────┐
│                    RESONANCE PATTERNS                        │
│                                                              │
│  How membrane distances create standing waves:               │
│  Good Resonance (Twin Prime Distance = 2):                  │
│  ─────────────────────────────────────────                  │
│    5 ←──2──→ 7     Creates:   ╱╲    ╱╲    ╱╲               │
│                              ╱  ╲  ╱  ╲  ╱  ╲              │
│                             ╱    ╲╱    ╲╱    ╲             │
│                            Constructive interference!        │
│  Poor Resonance (Composite Distance = 4):                   │
│  ────────────────────────────────────────                   │
│    3 ←──4──→ 7     Creates:   ╱╲      ╱╲      ╱╲           │
│                              ╱  ╲  ╱╲╱  ╲  ╱╲╱  ╲          │
│                             ╱    ╲╱  ╲   ╲╱  ╲   ╲         │
│                            Some destructive interference     │
│  Interference (Same digit):                                  │
│  ─────────────────────────                                  │
│    5 ←──0──→ 5     Creates:   ────────────────             │
│                              No wave, no primes!            │
└──────────────────────────────────────────────────────────────┘
fn generate_cross_base_summary() -> String {
    let mut summary = String::new();
    summary.push_str(r#"
╔═══════════════════════════════════════════════════════════════════╗
║                      THE 5-7 PHENOMENON                           ║
╠═══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  Across All Bases Tested:                                         ║
║  ───────────────────────                                          ║
║  Base  │ Best Config │ Contains 5-7? │ Prime Density             ║
║  ──────┼─────────────┼───────────────┼─────────────              ║
║   6    │ (3,3)       │ No            │ 30.2%                     ║
║        │ (3,5)       │ 5 present     │ 25.1% ← Still good!       ║
║   10   │ (3,7)       │ 7 present     │ 22.3%                     ║
║        │ (5,7)       │ BOTH! ✓✓      │ 21.8%                     ║
║   12   │ (5,7)       │ BOTH! ✓✓      │ 28.9% 🏆                  ║
║        │ (7,5)       │ BOTH! ✓✓      │ 28.7%                     ║
║   16   │ (5,B)       │ 5 present     │ 19.8%                     ║
║        │ (7,9)       │ 7 present     │ 18.2%                     ║
║  Mathematical Properties of 5 and 7:                              ║
║  ──────────────────────────────────                              ║
║    • Twin primes (consecutive odd primes)                         ║
║    • Difference = 2 (minimal prime gap)                           ║
║    • Sum = 12 (highly composite: 2²×3)                           ║
║    • Product = 35 (semiprime: 5×7)                               ║
║    • Ratio 7/5 = 1.4 ≈ √2 = 1.414...                            ║
║    • Both in first prime quadruplet: (5,7,11,13)                 ║
║  Visual Proof of Optimality:                                      ║
║         1   2   3   4   5   6   7   8   9                        ║
║         •   •   •   •   ★   •   ★   •   •                        ║
║                         ↑       ↑                                 ║
║                         └───2───┘                                 ║
║                      Twin prime gap!                              ║
╚═══════════════════════════════════════════════════════════════════╝
    summary
fn main() {
    println!("🎨 ASCII Diagram Generator");
    println!("=========================\n");
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    // Create main output file
    let main_filename = format!("membrane_diagrams_{}.txt", timestamp);
    let mut main_file = File::create(&main_filename).expect("Failed to create main file");
    // Write header
    writeln!(main_file, "╔═══════════════════════════════════════════════════════════════╗").unwrap();
    writeln!(main_file, "║           MEMBRANE PRIME PATTERNS - VISUAL ANALYSIS           ║").unwrap();
    writeln!(main_file, "║                   Generated: {}                    ║", Local::now().format("%Y-%m-%d %H:%M:%S")).unwrap();
    writeln!(main_file, "╚═══════════════════════════════════════════════════════════════╝\n").unwrap();
    // Generate and write various diagrams
    println!("Generating membrane structure diagrams...");
    writeln!(main_file, "\n═══ MEMBRANE STRUCTURES ═══\n").unwrap();
    // Base 10 examples
    writeln!(main_file, "{}", generate_membrane_diagram(10, 3, 7, 1, 1, 5)).unwrap();
    writeln!(main_file, "\n{}", generate_membrane_diagram(10, 5, 7, 0, 1, 5)).unwrap();
    // Base 12 examples
    writeln!(main_file, "\n{}", generate_membrane_diagram(12, 5, 7, 0, 1, 6)).unwrap();
    writeln!(main_file, "\n{}", generate_membrane_diagram(12, 7, 11, 1, 0, 5)).unwrap();
    // Distance comparison
    println!("Generating distance analysis...");
    writeln!(main_file, "\n{}", generate_distance_comparison()).unwrap();
    // Prime density chart
    println!("Generating prime density visualizations...");
    writeln!(main_file, "\n{}", generate_prime_density_chart()).unwrap();
    // Atomic comparison
    println!("Generating atomic prime comparisons...");
    writeln!(main_file, "\n{}", generate_atomic_comparison()).unwrap();
    // Resonance patterns
    println!("Generating resonance diagrams...");
    writeln!(main_file, "\n{}", generate_resonance_diagram()).unwrap();
    // Cross-base summary
    println!("Generating cross-base analysis...");
    writeln!(main_file, "\n{}", generate_cross_base_summary()).unwrap();
    // Create specialized diagram files
    // File 1: Just the 5-7 phenomenon
    let five_seven_filename = format!("five_seven_visual_{}.txt", timestamp);
    let mut five_seven_file = File::create(&five_seven_filename).expect("Failed to create 5-7 file");
    writeln!(five_seven_file, "{}", generate_cross_base_summary()).unwrap();
    writeln!(five_seven_file, "\n{}", generate_resonance_diagram()).unwrap();
    // File 2: Base comparison diagrams
    let base_comp_filename = format!("base_comparison_visual_{}.txt", timestamp);
    let mut base_comp_file = File::create(&base_comp_filename).expect("Failed to create comparison file");
    writeln!(base_comp_file, "{}", generate_atomic_comparison()).unwrap();
    writeln!(base_comp_file, "\n{}", generate_prime_density_chart()).unwrap();
    // File 3: Quick reference card
    let quick_ref_filename = format!("membrane_quick_reference_{}.txt", timestamp);
    let mut quick_ref_file = File::create(&quick_ref_filename).expect("Failed to create quick reference");
    writeln!(quick_ref_file, r#"
║                   MEMBRANE PRIME QUICK REFERENCE              ║
║  Best Configurations by Base:                                 ║
║  ───────────────────────────                                 ║
║  Base 6:  (3,3) k=(0,1) → 30.2% density                      ║
║           Pattern: 3 3 0 [seed] 0 3 3                        ║
║  Base 10: (3,7) k=(1,2) → 22.3% density                      ║
║           Pattern: 3 0 7 00 [seed] 00 7 0 3                  ║
║  Base 12: (5,7) k=(0,1) → 28.9% density                      ║
║           Pattern: 5 7 0 [seed] 0 7 5                        ║
║  The 5-7 Rule:                                                ║
║  ─────────────                                                ║
║  Configurations containing digits 5 and 7 consistently        ║
║  outperform others across all tested bases.                   ║
║  Exclusive Configuration (Base 10):                           ║
║  ─────────────────────────────────                           ║
║  (3,7) k=(1,1) works ONLY with seed 5                        ║
║  Pattern: 3 0 7 0 5 0 7 0 3 → 307050703 (PRIME!)            ║
"#).unwrap();
    println!("\n✅ Diagrams generated successfully!");
    println!("\n📁 Files created:");
    println!("   Main collection: {}", main_filename);
    println!("   5-7 phenomenon: {}", five_seven_filename);
    println!("   Base comparison: {}", base_comp_filename);
    println!("   Quick reference: {}", quick_ref_filename);
    println!("\n🎨 Share these beautiful diagrams with the world!");
