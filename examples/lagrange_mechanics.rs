use num_bigint::BigUint;
use prime_physics_engine::is_prime;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct LagrangePoint {
    position: usize,
    digit: u8,
    creates_prime: bool,
}

#[derive(Debug)]
struct LagrangeAnalysis {
    body1: String,
    body2: String,
    body1_prime: bool,
    body2_prime: bool,
    space_size: usize,
    lagrange_points: Vec<LagrangePoint>,
    success_rate: f64,
}

fn analyze_lagrange_mechanics(prime1: &str, prime2: &str, space_size: usize) -> LagrangeAnalysis {
    let p1 = prime1.parse::<BigUint>().unwrap();
    let p2 = prime2.parse::<BigUint>().unwrap();
    
    let body1_prime = is_prime(&p1);
    let body2_prime = is_prime(&p2);
    
    let mut lagrange_points = Vec::new();
    let zeros = "0".repeat(space_size);
    
    // Test each position and digit
    for position in 0..space_size {
        for digit in 1..=9 {
            let mut test_str = zeros.clone();
            let bytes = unsafe { test_str.as_bytes_mut() };
            bytes[position] = b'0' + digit as u8;
            
            let full_number = format!("{}{}{}", prime1, test_str, prime2);
            let num = full_number.parse::<BigUint>().unwrap();
            let creates_prime = is_prime(&num);
            
            lagrange_points.push(LagrangePoint {
                position,
                digit,
                creates_prime,
            });
        }
    }
    
    let successful = lagrange_points.iter().filter(|lp| lp.creates_prime).count();
    let total = lagrange_points.len();
    let success_rate = (successful as f64 / total as f64) * 100.0;
    
    LagrangeAnalysis {
        body1: prime1.to_string(),
        body2: prime2.to_string(),
        body1_prime,
        body2_prime,
        space_size,
        lagrange_points,
        success_rate,
    }
}

fn visualize_lagrange_field(analysis: &LagrangeAnalysis) {
    println!("\n{}", "=".repeat(80));
    println!("🌌 LAGRANGE FIELD VISUALIZATION");
    println!("{}", "=".repeat(80));
    
    println!("\n📍 Body 1: {} ({})", 
        analysis.body1, 
        if analysis.body1_prime { "PRIME ✓" } else { "COMPOSITE ✗" }
    );
    println!("📍 Body 2: {} ({})", 
        analysis.body2,
        if analysis.body2_prime { "PRIME ✓" } else { "COMPOSITE ✗" }
    );
    println!("🌌 Space: {} positions", analysis.space_size);
    
    // Create position map
    let mut position_map: HashMap<usize, Vec<u8>> = HashMap::new();
    for lp in &analysis.lagrange_points {
        if lp.creates_prime {
            position_map.entry(lp.position).or_insert(Vec::new()).push(lp.digit);
        }
    }
    
    // Visualize the field
    println!("\n🎯 LAGRANGE FIELD MAP:");
    println!("   Position: 0{}",
        (1..analysis.space_size).map(|i| format!("{:>10}", i)).collect::<String>()
    );
    print!("   Body1 → ");
    
    for pos in 0..analysis.space_size {
        if let Some(digits) = position_map.get(&pos) {
            if digits.len() == 1 {
                print!(" [{}]      ", digits[0]);
            } else {
                print!(" [{} pts]  ", digits.len());
            }
        } else {
            print!(" ·        ");
        }
    }
    println!(" → Body2");
    
    // Success statistics
    println!("\n📊 FIELD STATISTICS:");
    println!("   Total test points: {}", analysis.lagrange_points.len());
    println!("   Prime-creating points: {}", 
        analysis.lagrange_points.iter().filter(|lp| lp.creates_prime).count()
    );
    println!("   Success rate: {:.1}%", analysis.success_rate);
    
    // Position analysis
    println!("\n🔍 POSITION ANALYSIS:");
    for pos in 0..analysis.space_size {
        if let Some(digits) = position_map.get(&pos) {
            println!("   Position {}: {} Lagrange points (digits: {:?})", 
                pos, digits.len(), digits
            );
        }
    }
}

fn compare_configurations() {
    println!("\n{}", "=".repeat(80));
    println!("⚡ COMPARATIVE LAGRANGE MECHANICS");
    println!("{}", "=".repeat(80));
    
    let test_cases = vec![
        // (body1, body2, space_size, description)
        ("11", "13", 5, "Simple twin primes"),
        ("101", "103", 5, "Twin primes (3-digit)"),
        ("101", "30103", 5, "Prime to membrane prime"),
        ("10301", "10301", 5, "Same membrane prime"),
        ("10301", "3007003007003", 5, "Different membrane primes"),
        ("11", "121", 5, "Prime to composite (11²)"),
        ("13", "169", 5, "Prime to composite (13²)"),
        ("121", "169", 5, "Composite to composite"),
    ];
    
    let mut results = Vec::new();
    
    for (body1, body2, space, desc) in test_cases {
        println!("\n🔬 Testing: {} ({} ↔ {} with {} spaces)", desc, body1, body2, space);
        let analysis = analyze_lagrange_mechanics(body1, body2, space);
        visualize_lagrange_field(&analysis);
        results.push((desc, analysis));
    }
    
    // Summary comparison
    println!("\n\n{}", "=".repeat(80));
    println!("📊 SUMMARY COMPARISON");
    println!("{}", "=".repeat(80));
    println!("\n{:<30} {:>15} {:>15} {:>15}", 
        "Configuration", "Both Prime?", "L-Points", "Success Rate"
    );
    println!("{}", "-".repeat(80));
    
    for (desc, analysis) in &results {
        let both_prime = analysis.body1_prime && analysis.body2_prime;
        let l_points = analysis.lagrange_points.iter().filter(|lp| lp.creates_prime).count();
        
        println!("{:<30} {:>15} {:>15} {:>14.1}%", 
            desc,
            if both_prime { "Yes" } else { "No" },
            l_points,
            analysis.success_rate
        );
    }
}

fn main() {
    println!("{}", "=".repeat(80));
    println!("🚀 LAGRANGE MECHANICS EXPLORER");
    println!("{}", "=".repeat(80));
    
    // Detailed single analysis
    println!("\n1️⃣ DETAILED ANALYSIS: Classic membrane configuration");
    let analysis = analyze_lagrange_mechanics("10301", "3007003007003", 7);
    visualize_lagrange_field(&analysis);
    
    // Show specific successes
    println!("\n✨ SUCCESSFUL LAGRANGE POINTS:");
    for lp in &analysis.lagrange_points {
        if lp.creates_prime {
            let mut zeros = "0".repeat(analysis.space_size);
            let bytes = unsafe { zeros.as_bytes_mut() };
            bytes[lp.position] = b'0' + lp.digit as u8;
            
            let prime = format!("{}{}{}", analysis.body1, zeros, analysis.body2);
            println!("\n   Position {}, Digit {}: ", lp.position, lp.digit);
            println!("   → {}", prime);
            println!("   → {}-digit PRIME!", prime.len());
        }
    }
    
    // Comparative analysis
    println!("\n\n2️⃣ COMPARATIVE ANALYSIS:");
    compare_configurations();
    
    println!("\n{}", "=".repeat(80));
    println!("✨ EXPLORATION COMPLETE");
    println!("{}", "=".repeat(80));
}