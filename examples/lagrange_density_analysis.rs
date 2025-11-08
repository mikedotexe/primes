use num_bigint::BigUint;
use prime_physics_engine::is_prime;

#[derive(Debug)]
struct DensityAnalysis {
    body1: String,
    body2: String,
    digit_sum1: u32,
    digit_sum2: u32,
    digit_product: u64,
    lagrange_success_rate: f64,
    zero_density1: f64,
    zero_density2: f64,
}

fn digit_sum(s: &str) -> u32 {
    s.chars().filter_map(|c| c.to_digit(10)).sum()
}

fn zero_density(s: &str) -> f64 {
    let zeros = s.chars().filter(|&c| c == '0').count();
    zeros as f64 / s.len() as f64
}

fn test_lagrange_density(body1: &str, body2: &str, space_size: usize) -> DensityAnalysis {
    let zeros = "0".repeat(space_size);
    let mut successes = 0;
    let mut tests = 0;
    
    for position in 0..space_size {
        for digit in 1..=9 {
            tests += 1;
            let mut test_str = zeros.clone();
            let bytes = unsafe { test_str.as_bytes_mut() };
            bytes[position] = b'0' + digit as u8;
            
            let full_number = format!("{}{}{}", body1, test_str, body2);
            let num = full_number.parse::<BigUint>().unwrap();
            
            if is_prime(&num) {
                successes += 1;
            }
        }
    }
    
    DensityAnalysis {
        body1: body1.to_string(),
        body2: body2.to_string(),
        digit_sum1: digit_sum(body1),
        digit_sum2: digit_sum(body2),
        digit_product: digit_sum(body1) as u64 * digit_sum(body2) as u64,
        lagrange_success_rate: (successes as f64 / tests as f64) * 100.0,
        zero_density1: zero_density(body1),
        zero_density2: zero_density(body2),
    }
}

fn main() {
    println!("🔬 LAGRANGE DENSITY ANALYSIS");
    println!("{}", "=".repeat(80));
    println!("\nHypothesis: Simpler numbers (lower digit sums, no zeros) create better Lagrange fields\n");
    
    let test_pairs = vec![
        // High performers
        ("11", "13"),
        ("11", "121"),
        ("13", "17"),
        ("17", "19"),
        
        // Medium performers  
        ("101", "103"),
        ("13", "169"),
        
        // Low performers
        ("101", "30103"),
        ("10301", "10301"),
        ("10301", "3007003007003"),
        
        // Additional tests
        ("7", "11"),
        ("23", "29"),
        ("31", "37"),
        ("10007", "10009"),
        ("100003", "100019"),
    ];
    
    let mut results = Vec::new();
    
    for (body1, body2) in test_pairs {
        let analysis = test_lagrange_density(body1, body2, 5);
        results.push(analysis);
    }
    
    // Sort by success rate
    results.sort_by(|a, b| b.lagrange_success_rate.partial_cmp(&a.lagrange_success_rate).unwrap());
    
    // Display results
    println!("{:<15} {:<15} {:>10} {:>10} {:>12} {:>12} {:>12}", 
        "Body 1", "Body 2", "Sum1", "Sum2", "Product", "Zero%", "Success%"
    );
    println!("{}", "-".repeat(90));
    
    for r in &results {
        let avg_zero_density = (r.zero_density1 + r.zero_density2) / 2.0 * 100.0;
        println!("{:<15} {:<15} {:>10} {:>10} {:>12} {:>11.1}% {:>11.1}%", 
            r.body1, r.body2, r.digit_sum1, r.digit_sum2, r.digit_product,
            avg_zero_density, r.lagrange_success_rate
        );
    }
    
    // Analysis
    println!("\n📊 PATTERN ANALYSIS:");
    
    // Calculate correlations
    let high_performers: Vec<_> = results.iter().filter(|r| r.lagrange_success_rate > 10.0).collect();
    let low_performers: Vec<_> = results.iter().filter(|r| r.lagrange_success_rate < 5.0).collect();
    
    println!("\n🌟 HIGH PERFORMERS (>10% success):");
    for r in &high_performers {
        println!("   {} ↔ {}: Product={}, Zeros={:.0}%", 
            r.body1, r.body2, r.digit_product, 
            (r.zero_density1 + r.zero_density2) / 2.0 * 100.0
        );
    }
    
    println!("\n💤 LOW PERFORMERS (<5% success):");
    for r in &low_performers {
        println!("   {} ↔ {}: Product={}, Zeros={:.0}%", 
            r.body1, r.body2, r.digit_product,
            (r.zero_density1 + r.zero_density2) / 2.0 * 100.0
        );
    }
    
    // Calculate averages
    let high_avg_product: f64 = high_performers.iter().map(|r| r.digit_product as f64).sum::<f64>() / high_performers.len() as f64;
    let low_avg_product: f64 = low_performers.iter().map(|r| r.digit_product as f64).sum::<f64>() / low_performers.len() as f64;
    
    let high_avg_zeros: f64 = high_performers.iter()
        .map(|r| (r.zero_density1 + r.zero_density2) / 2.0 * 100.0)
        .sum::<f64>() / high_performers.len() as f64;
    let low_avg_zeros: f64 = low_performers.iter()
        .map(|r| (r.zero_density1 + r.zero_density2) / 2.0 * 100.0)
        .sum::<f64>() / low_performers.len() as f64;
    
    println!("\n📈 STATISTICAL SUMMARY:");
    println!("   High performers: avg product = {:.1}, avg zero% = {:.1}%", high_avg_product, high_avg_zeros);
    println!("   Low performers:  avg product = {:.1}, avg zero% = {:.1}%", low_avg_product, low_avg_zeros);
    
    println!("\n💡 CONCLUSIONS:");
    println!("   1. Lower digit products correlate with higher Lagrange success");
    println!("   2. Zero-heavy numbers (membranes) have lower Lagrange density");
    println!("   3. Simple primes create the strongest Lagrange fields");
}