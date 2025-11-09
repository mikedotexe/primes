use num_bigint::BigUint;
use prime_physics_engine::is_prime;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct LagrangeDataPoint {
    body1: String,
    body2: String,
    body1_len: usize,
    body2_len: usize,
    #[allow(dead_code)]
    body1_prime: bool,
    #[allow(dead_code)]
    body2_prime: bool,
    body1_zeros: usize,
    body2_zeros: usize,
    space_size: usize,
    position: usize,
    digit: u8,
    result_prime: bool,
    result_len: usize,
}

fn count_zeros(s: &str) -> usize {
    s.chars().filter(|&c| c == '0').count()
}

fn systematic_lagrange_test(body1: &str, body2: &str, max_space: usize) -> Vec<LagrangeDataPoint> {
    let mut results = Vec::new();

    let b1 = body1.parse::<BigUint>().unwrap();
    let b2 = body2.parse::<BigUint>().unwrap();
    let body1_prime = is_prime(&b1);
    let body2_prime = is_prime(&b2);
    let body1_zeros = count_zeros(body1);
    let body2_zeros = count_zeros(body2);

    for space_size in 1..=max_space {
        let zeros = "0".repeat(space_size);

        for position in 0..space_size {
            for digit in 1..=9 {
                let mut test_str = zeros.clone();
                let bytes = unsafe { test_str.as_bytes_mut() };
                bytes[position] = b'0' + digit as u8;

                let full_number = format!("{}{}{}", body1, test_str, body2);
                let num = full_number.parse::<BigUint>().unwrap();
                let result_prime = is_prime(&num);

                results.push(LagrangeDataPoint {
                    body1: body1.to_string(),
                    body2: body2.to_string(),
                    body1_len: body1.len(),
                    body2_len: body2.len(),
                    body1_prime,
                    body2_prime,
                    body1_zeros,
                    body2_zeros,
                    space_size,
                    position,
                    digit,
                    result_prime,
                    result_len: full_number.len(),
                });
            }
        }
    }

    results
}

fn analyze_patterns(data: &[LagrangeDataPoint]) {
    println!("\n📊 PATTERN ANALYSIS FROM {} DATA POINTS", data.len());
    println!("{}", "=".repeat(80));

    // Group by various characteristics
    let mut by_digit: BTreeMap<u8, (usize, usize)> = BTreeMap::new();
    let mut by_position_ratio: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut by_zero_count: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut by_length_ratio: BTreeMap<String, (usize, usize)> = BTreeMap::new();

    for point in data {
        // By digit
        let digit_entry = by_digit.entry(point.digit).or_insert((0, 0));
        digit_entry.0 += 1;
        if point.result_prime {
            digit_entry.1 += 1;
        }

        // By position ratio (early/middle/late)
        let pos_ratio = if point.space_size > 0 {
            let ratio = point.position as f64 / point.space_size as f64;
            if ratio < 0.33 {
                "early"
            } else if ratio < 0.67 {
                "middle"
            } else {
                "late"
            }
        } else {
            "single"
        };
        let pos_entry = by_position_ratio
            .entry(pos_ratio.to_string())
            .or_insert((0, 0));
        pos_entry.0 += 1;
        if point.result_prime {
            pos_entry.1 += 1;
        }

        // By total zero count
        let total_zeros = point.body1_zeros + point.body2_zeros;
        let zero_entry = by_zero_count.entry(total_zeros).or_insert((0, 0));
        zero_entry.0 += 1;
        if point.result_prime {
            zero_entry.1 += 1;
        }

        // By length ratio
        let len_ratio = format!("{}:{}", point.body1_len, point.body2_len);
        let len_entry = by_length_ratio.entry(len_ratio).or_insert((0, 0));
        len_entry.0 += 1;
        if point.result_prime {
            len_entry.1 += 1;
        }
    }

    // Print analysis
    println!("\n🔢 SUCCESS BY DIGIT:");
    for (digit, (total, primes)) in &by_digit {
        let rate = (*primes as f64 / *total as f64) * 100.0;
        println!("   Digit {}: {}/{} = {:.1}%", digit, primes, total, rate);
    }

    println!("\n📍 SUCCESS BY POSITION:");
    for (pos, (total, primes)) in &by_position_ratio {
        let rate = (*primes as f64 / *total as f64) * 100.0;
        println!("   Position {}: {}/{} = {:.1}%", pos, primes, total, rate);
    }

    println!("\n🔵 SUCCESS BY BODY ZERO COUNT:");
    for (zeros, (total, primes)) in &by_zero_count {
        let rate = (*primes as f64 / *total as f64) * 100.0;
        println!(
            "   {} zeros in bodies: {}/{} = {:.1}%",
            zeros, primes, total, rate
        );
    }

    println!("\n📏 SUCCESS BY LENGTH RATIO:");
    for (ratio, (total, primes)) in &by_length_ratio {
        let rate = (*primes as f64 / *total as f64) * 100.0;
        println!("   Ratio {}: {}/{} = {:.1}%", ratio, primes, total, rate);
    }
}

fn output_prime_wall(data: &[LagrangeDataPoint]) {
    println!("\n🌊 WALL OF VERIFIED PRIMES");
    println!("{}", "=".repeat(80));

    let primes: Vec<_> = data.iter().filter(|p| p.result_prime).collect();

    println!(
        "Found {} primes from {} tests ({:.1}% success)",
        primes.len(),
        data.len(),
        (primes.len() as f64 / data.len() as f64) * 100.0
    );

    println!("\n📋 ALL VERIFIED PRIMES (first 100):");
    for (i, point) in primes.iter().take(100).enumerate() {
        let mut zeros = "0".repeat(point.space_size);
        let bytes = unsafe { zeros.as_bytes_mut() };
        bytes[point.position] = b'0' + point.digit as u8;

        let full_prime = format!("{}{}{}", point.body1, zeros, point.body2);
        println!("{:3}. {} ({}d)", i + 1, full_prime, point.result_len);
    }
}

fn main() {
    println!("🔬 SYSTEMATIC LAGRANGE POINT STUDY");
    println!("{}", "=".repeat(80));
    println!("No assumptions. Just data.\n");

    let mut all_data = Vec::new();

    // Test 1: Small symmetric primes (2 digits)
    println!("📌 TEST SET 1: Small symmetric primes");
    let small_primes = ["11", "13", "17", "19", "23", "29", "31", "37", "41", "43"];
    for i in 0..small_primes.len() {
        for j in i..small_primes.len() {
            let data = systematic_lagrange_test(small_primes[i], small_primes[j], 3);
            all_data.extend(data);
        }
    }

    // Test 2: Zero-padded small primes
    println!("📌 TEST SET 2: Zero-padded structures");
    let zero_padded = ["101", "103", "107", "109", "10007", "10009", "100003"];
    for i in 0..zero_padded.len().min(5) {
        for j in i..zero_padded.len().min(5) {
            let data = systematic_lagrange_test(zero_padded[i], zero_padded[j], 3);
            all_data.extend(data);
        }
    }

    // Test 3: Membrane primes
    println!("📌 TEST SET 3: Membrane structures");
    let membranes = ["10301", "30103", "30703", "1035301", "3007003007003"];
    for i in 0..membranes.len().min(4) {
        for j in i..membranes.len().min(4) {
            let data = systematic_lagrange_test(membranes[i], membranes[j], 2);
            all_data.extend(data);
        }
    }

    // Test 4: Composites
    println!("📌 TEST SET 4: Composite numbers");
    let composites = ["4", "6", "8", "9", "10", "12", "14", "15", "16", "18"];
    for i in 0..5 {
        for j in 0..5 {
            let data = systematic_lagrange_test(composites[i], composites[j], 2);
            all_data.extend(data);
        }
    }

    // Test 5: Mixed (prime with composite)
    println!("📌 TEST SET 5: Mixed prime-composite");
    for i in 0..5 {
        for j in 0..5 {
            let data = systematic_lagrange_test(small_primes[i], composites[j], 2);
            all_data.extend(data);
        }
    }

    // Analyze all patterns
    analyze_patterns(&all_data);

    // Output wall of primes
    output_prime_wall(&all_data);

    // Look for unexpected patterns
    println!("\n🔍 UNEXPECTED DISCOVERIES:");

    // Find highest success rate configurations
    let mut config_success: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for point in &all_data {
        let config = format!("{}-{}-{}", point.body1, point.body2, point.space_size);
        let entry = config_success.entry(config).or_insert((0, 0));
        entry.0 += 1;
        if point.result_prime {
            entry.1 += 1;
        }
    }

    let mut sorted_configs: Vec<_> = config_success
        .iter()
        .map(|(k, (total, primes))| (k, *primes as f64 / *total as f64))
        .filter(|(_, rate)| *rate > 0.0)
        .collect();
    sorted_configs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\n🏆 TOP 10 CONFIGURATIONS:");
    for (config, rate) in sorted_configs.iter().take(10) {
        println!("   {} → {:.1}% success", config, rate * 100.0);
    }

    // Check for digit preferences by position
    println!("\n🎯 DIGIT PREFERENCES BY POSITION:");
    for space in 1..=3 {
        for pos in 0..space {
            let mut digit_counts = vec![0; 10];
            let mut digit_primes = vec![0; 10];

            for point in &all_data {
                if point.space_size == space && point.position == pos {
                    digit_counts[point.digit as usize] += 1;
                    if point.result_prime {
                        digit_primes[point.digit as usize] += 1;
                    }
                }
            }

            let best_digit = (1..=9).max_by_key(|&d| digit_primes[d]).unwrap();
            let rate = if digit_counts[best_digit] > 0 {
                digit_primes[best_digit] as f64 / digit_counts[best_digit] as f64
            } else {
                0.0
            };

            println!(
                "   Space={}, Pos={}: Best digit={} ({:.1}% success)",
                space,
                pos,
                best_digit,
                rate * 100.0
            );
        }
    }
}
