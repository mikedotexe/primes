use num_bigint::BigUint;
use primes::is_prime;

fn deep_dive_configuration(body1: &str, body2: &str, max_space: usize) {
    let b1 = body1.parse::<BigUint>().unwrap();
    let b2 = body2.parse::<BigUint>().unwrap();

    println!("\n{}", "=".repeat(60));
    println!("DEEP DIVE: {} ↔ {}", body1, body2);
    println!("{}", "=".repeat(60));
    println!(
        "Body 1: {} ({})",
        body1,
        if is_prime(&b1) { "PRIME" } else { "COMPOSITE" }
    );
    println!(
        "Body 2: {} ({})",
        body2,
        if is_prime(&b2) { "PRIME" } else { "COMPOSITE" }
    );

    // Factor the composites if they're composite
    if !is_prime(&b1) {
        print!("   Factorization of {}: ", body1);
        factor_simple(&b1);
    }
    if !is_prime(&b2) {
        print!("   Factorization of {}: ", body2);
        factor_simple(&b2);
    }

    println!("\n📊 RESULTS BY SPACE SIZE:");

    for space_size in 1..=max_space {
        let zeros = "0".repeat(space_size);
        let mut primes_found = Vec::new();

        for position in 0..space_size {
            for digit in 1..=9 {
                let mut test_str = zeros.clone();
                let bytes = unsafe { test_str.as_bytes_mut() };
                bytes[position] = b'0' + digit as u8;

                let full_number = format!("{}{}{}", body1, test_str, body2);
                let num = full_number.parse::<BigUint>().unwrap();

                if is_prime(&num) {
                    primes_found.push((position, digit, full_number));
                }
            }
        }

        let total_tests = space_size * 9;
        let success_rate = (primes_found.len() as f64 / total_tests as f64) * 100.0;

        println!(
            "\nSpace size {}: {}/{} = {:.1}% success",
            space_size,
            primes_found.len(),
            total_tests,
            success_rate
        );

        if !primes_found.is_empty() && space_size <= 3 {
            println!("   Primes found:");
            for (pos, digit, prime) in &primes_found {
                println!("   - Position {}, digit {}: {} ✓", pos, digit, prime);
            }
        }
    }
}

fn factor_simple(n: &BigUint) {
    let mut factors = Vec::new();
    let mut num = n.clone();
    let two = BigUint::from(2u32);
    let mut divisor = two.clone();

    while &divisor * &divisor <= num {
        while &num % &divisor == BigUint::from(0u32) {
            factors.push(divisor.clone());
            num = num / &divisor;
        }
        divisor = divisor + BigUint::from(1u32);

        // Bail out for large numbers
        if factors.len() > 10 || divisor > BigUint::from(1000u32) {
            print!("... (factorization stopped)");
            return;
        }
    }

    if num > BigUint::from(1u32) {
        factors.push(num);
    }

    print!(
        "{}",
        factors
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(" × ")
    );
    println!();
}

fn test_composite_patterns() {
    println!("\n🔍 TESTING COMPOSITE PATTERNS");
    println!("{}", "=".repeat(80));

    // Test perfect squares
    println!("\n📐 PERFECT SQUARES:");
    let squares = [
        ("4", "2²"),
        ("9", "3²"),
        ("16", "4²"),
        ("25", "5²"),
        ("36", "6²"),
    ];
    for (num, desc) in &squares {
        let n = num.parse::<BigUint>().unwrap();
        println!(
            "{} = {} ({})",
            num,
            desc,
            if is_prime(&n) { "prime" } else { "composite" }
        );
    }

    // Test products of primes
    println!("\n✖️ PRODUCTS OF PRIMES:");
    let products = [
        ("6", "2×3"),
        ("10", "2×5"),
        ("14", "2×7"),
        ("15", "3×5"),
        ("21", "3×7"),
    ];
    for (num, desc) in &products {
        let n = num.parse::<BigUint>().unwrap();
        println!(
            "{} = {} ({})",
            num,
            desc,
            if is_prime(&n) { "prime" } else { "composite" }
        );
    }
}

fn systematic_composite_test() {
    println!("\n📊 SYSTEMATIC COMPOSITE LAGRANGE ANALYSIS");
    println!("{}", "=".repeat(80));

    // Group 1: Small composites with each other
    println!("\n1️⃣ COMPOSITE ↔ COMPOSITE:");
    let composites = [
        "4", "6", "8", "9", "10", "12", "14", "15", "16", "18", "20", "21",
    ];
    let mut results = Vec::new();

    for i in 0..composites.len().min(8) {
        for j in i..composites.len().min(8) {
            let _zeros = "0";
            let mut success_count = 0;

            for digit in 1..=9 {
                let test = format!("{}{}{}", composites[i], digit, composites[j]);
                let num = test.parse::<BigUint>().unwrap();
                if is_prime(&num) {
                    success_count += 1;
                }
            }

            let rate = (success_count as f64 / 9.0) * 100.0;
            if rate > 0.0 {
                results.push((composites[i], composites[j], success_count, rate));
            }
        }
    }

    results.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap());

    println!("\nTop composite pairs (1 space):");
    for (b1, b2, count, rate) in results.iter().take(10) {
        println!("   {} ↔ {}: {}/9 = {:.1}%", b1, b2, count, rate);
    }

    // Group 2: Primes with their squares
    println!("\n2️⃣ PRIME ↔ PRIME²:");
    let prime_squares = [
        ("2", "4"),
        ("3", "9"),
        ("5", "25"),
        ("7", "49"),
        ("11", "121"),
    ];

    for (prime, square) in &prime_squares {
        let mut count = 0;
        for digit in 1..=9 {
            let test = format!("{}{}{}", prime, digit, square);
            let num = test.parse::<BigUint>().unwrap();
            if is_prime(&num) {
                count += 1;
            }
        }
        println!(
            "   {} ↔ {}: {}/9 = {:.1}%",
            prime,
            square,
            count,
            (count as f64 / 9.0) * 100.0
        );
    }
}

fn main() {
    println!("{}", "=".repeat(80));
    println!("🧪 LAGRANGE COMPOSITE INVESTIGATION");
    println!("{}", "=".repeat(80));
    println!("Why do composite numbers create such strong Lagrange fields?");

    // Test basic patterns
    test_composite_patterns();

    // Systematic test
    systematic_composite_test();

    // Deep dive into top performers
    println!("\n🔬 DEEP DIVES INTO TOP PERFORMERS:");

    deep_dive_configuration("4", "9", 5);
    deep_dive_configuration("10", "9", 5);
    deep_dive_configuration("41", "43", 3);
    deep_dive_configuration("4", "4", 3);
    deep_dive_configuration("6", "9", 3);

    println!("\n💡 HYPOTHESIS GENERATION:");
    println!("   1. Small composites have simple factors that create patterns");
    println!("   2. Digit sum relationships may play a role");
    println!("   3. Modular arithmetic patterns emerge from factorizations");
    println!("   4. The 'weight' of bodies matters less than their interaction");
}
