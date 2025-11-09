// 3D Visualization of Coordinate Constellation Space
//
// This example creates ASCII 3D visualizations of the (x,y,z) coordinate
// space for septuplet constellations, showing where primes cluster.
//
// VISUALIZATION TECHNIQUES:
// 1. Slice views: Show z-layers as 2D grids
// 2. Projection views: xy, xz, yz planes
// 3. Density heatmaps: Count primes per coordinate
// 4. Coprimality highlighting: Mark coprime-to-base coords
//
// LOOKING FOR PATTERN OF 6:
// - Base 14 has φ(14) = 6 constrained outer coords
// - Base 18 also has φ(18) = 6
// - Is there clustering at specific coordinate values related to 6?

use num_bigint::BigUint;
use num_traits::Zero;
use primes::is_prime;
use std::collections::HashMap;

fn septuplet_membrane(middle: u32, x: u32, y: u32, z: u32, base: u32) -> BigUint {
    let mut result = BigUint::zero();
    let base_big = BigUint::from(base);

    result = result * &base_big + BigUint::from(z);
    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(middle);
    result = result * &base_big + BigUint::from(x);
    result = result * &base_big + BigUint::from(y);
    result = result * &base_big + BigUint::from(z);

    result
}

fn is_coprime(a: u32, b: u32) -> bool {
    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    gcd(a, b) == 1
}

fn collect_septuplet_coords(base: u32, middle: u32, limit: u64) -> Vec<(u32, u32, u32)> {
    let mut coords = Vec::new();

    for z in 1..base {
        for y in 1..base {
            for x in 1..base {
                let candidate = septuplet_membrane(middle, x, y, z, base);

                if candidate > BigUint::from(limit) {
                    continue;
                }

                if is_prime(&candidate) {
                    coords.push((x, y, z));
                }
            }
        }
    }

    coords
}

fn visualize_z_slice(coords: &[(u32, u32, u32)], z_value: u32, base: u32) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("Z-SLICE: z = {} (3rd neighbor distance)", z_value);
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let coprime_mark = if is_coprime(z_value, base) {
        "✓"
    } else {
        "✗"
    };
    println!(
        "  z={} is {} coprime to base {}",
        z_value, coprime_mark, base
    );
    println!();

    // Build frequency map for this z-slice
    let mut freq_map: HashMap<(u32, u32), usize> = HashMap::new();
    for &(x, y, z) in coords {
        if z == z_value {
            *freq_map.entry((x, y)).or_insert(0) += 1;
        }
    }

    if freq_map.is_empty() {
        println!("  (No primes found in this z-slice)");
        println!();
        return;
    }

    let max_freq = *freq_map.values().max().unwrap_or(&1);

    println!("  y (2nd neighbor) ↑");
    println!("                   │");

    // Print grid
    for y in (1..base).rev() {
        let coprime_y = if is_coprime(y, base) { "▌" } else { " " };
        print!("  {:2}{} │", y, coprime_y);

        for x in 1..base {
            let freq = freq_map.get(&(x, y)).unwrap_or(&0);
            let symbol = if *freq == 0 {
                "·"
            } else if *freq == max_freq {
                "█"
            } else if *freq > max_freq / 2 {
                "▓"
            } else if *freq > max_freq / 4 {
                "▒"
            } else {
                "░"
            };
            print!(" {}", symbol);
        }
        println!();
    }

    print!("     └");
    for _ in 1..base {
        print!("──");
    }
    println!("─→ x (1st neighbor)");

    print!("      ");
    for x in 1..base {
        let coprime_x = if is_coprime(x, base) { "▌" } else { " " };
        print!("{:1}{}", x % 10, coprime_x);
    }
    println!();
    println!();

    println!("  Legend: · = 0  ░ = low  ▒ = medium  ▓ = high  █ = max");
    println!("  Side marks (▌) indicate coprime values");
    println!();

    let total_in_slice = freq_map.values().sum::<usize>();
    println!("  Total primes in this slice: {}", total_in_slice);
    println!();
}

fn visualize_projection(coords: &[(u32, u32, u32)], projection: &str, base: u32) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("{} PROJECTION", projection.to_uppercase());
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Build frequency map
    let mut freq_map: HashMap<(u32, u32), usize> = HashMap::new();

    for &(x, y, z) in coords {
        let key = match projection {
            "xy" => (x, y),
            "xz" => (x, z),
            "yz" => (y, z),
            _ => panic!("Unknown projection"),
        };
        *freq_map.entry(key).or_insert(0) += 1;
    }

    let max_freq = *freq_map.values().max().unwrap_or(&1);

    let (axis1_label, axis2_label) = match projection {
        "xy" => ("x (1st)", "y (2nd)"),
        "xz" => ("x (1st)", "z (3rd)"),
        "yz" => ("y (2nd)", "z (3rd)"),
        _ => ("?", "?"),
    };

    println!("  {} ↑", axis2_label);
    println!("     │");

    for v in (1..base).rev() {
        let coprime_mark = if is_coprime(v, base) { "▌" } else { " " };
        print!("  {:2}{} │", v, coprime_mark);

        for u in 1..base {
            let key = if projection == "xy" {
                (u, v)
            } else if projection == "xz" {
                (u, v)
            } else {
                (u, v)
            };

            let freq = freq_map.get(&key).unwrap_or(&0);
            let symbol = if *freq == 0 {
                "·"
            } else if *freq == max_freq {
                "█"
            } else if *freq > max_freq / 2 {
                "▓"
            } else if *freq > max_freq / 4 {
                "▒"
            } else {
                "░"
            };
            print!(" {}", symbol);
        }
        println!();
    }

    print!("     └");
    for _ in 1..base {
        print!("──");
    }
    println!("─→ {}", axis1_label);

    print!("      ");
    for u in 1..base {
        let coprime_mark = if is_coprime(u, base) { "▌" } else { " " };
        print!("{:1}{}", u % 10, coprime_mark);
    }
    println!();
    println!();
}

fn analyze_sixes(coords: &[(u32, u32, u32)], base: u32) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("PATTERN OF 6 ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Look for appearances of 6 in coordinates
    let sixes_x = coords.iter().filter(|(x, _, _)| *x == 6).count();
    let sixes_y = coords.iter().filter(|(_, y, _)| *y == 6).count();
    let sixes_z = coords.iter().filter(|(_, _, z)| *z == 6).count();

    println!("Coordinate value 6 appearances:");
    println!(
        "  x=6: {} primes ({:.1}%)",
        sixes_x,
        sixes_x as f64 / coords.len() as f64 * 100.0
    );
    println!(
        "  y=6: {} primes ({:.1}%)",
        sixes_y,
        sixes_y as f64 / coords.len() as f64 * 100.0
    );
    println!(
        "  z=6: {} primes ({:.1}%)",
        sixes_z,
        sixes_z as f64 / coords.len() as f64 * 100.0
    );
    println!();

    // Sum to 6?
    let sum_to_6 = coords.iter().filter(|(x, y, z)| x + y + z == 6).count();
    println!(
        "Coordinates summing to 6 (x+y+z=6): {} primes ({:.1}%)",
        sum_to_6,
        sum_to_6 as f64 / coords.len() as f64 * 100.0
    );
    println!();

    // Multiples of 6?
    let x_mult_6 = coords
        .iter()
        .filter(|(x, _, _)| *x % 6 == 0 && *x > 0)
        .count();
    let y_mult_6 = coords
        .iter()
        .filter(|(_, y, _)| *y % 6 == 0 && *y > 0)
        .count();
    let z_mult_6 = coords
        .iter()
        .filter(|(_, _, z)| *z % 6 == 0 && *z > 0)
        .count();

    println!("Multiples of 6:");
    println!(
        "  x divisible by 6: {} primes ({:.1}%)",
        x_mult_6,
        x_mult_6 as f64 / coords.len() as f64 * 100.0
    );
    println!(
        "  y divisible by 6: {} primes ({:.1}%)",
        y_mult_6,
        y_mult_6 as f64 / coords.len() as f64 * 100.0
    );
    println!(
        "  z divisible by 6: {} primes ({:.1}%)",
        z_mult_6,
        z_mult_6 as f64 / coords.len() as f64 * 100.0
    );
    println!();

    // Distance from origin (1,1,1) = 6?
    let manhattan_6 = coords
        .iter()
        .filter(|(x, y, z)| (x.abs_diff(1) + y.abs_diff(1) + z.abs_diff(1)) == 6)
        .count();
    println!(
        "Manhattan distance 6 from (1,1,1): {} primes ({:.1}%)",
        manhattan_6,
        manhattan_6 as f64 / coords.len() as f64 * 100.0
    );
    println!();

    // Perfect number connection? (6 = 1+2+3)
    let perfect_combo = coords
        .iter()
        .filter(|(x, y, z)| {
            let mut vals = vec![*x, *y, *z];
            vals.sort();
            vals == vec![1, 2, 3]
        })
        .count();
    println!("Coordinates (1,2,3) in any order: {} primes", perfect_combo);
    println!();
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║    3D COORDINATE CONSTELLATION VISUALIZATION                 ║");
    println!("║    Exploring the (x,y,z) Space of Septuplets                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let base = 14u32;
    let middle = 1u32;
    let limit = 1_000_000_000_000u64;

    println!("CONFIGURATION:");
    println!("  Base: {}", base);
    println!("  Middle value: {}", middle);
    println!("  Structure: z-y-x-{}-x-y-z", middle);
    println!();

    println!("Collecting coordinate data...");
    let coords = collect_septuplet_coords(base, middle, limit);
    println!("  Found {} septuplet primes", coords.len());
    println!();

    // Analyze the pattern of 6
    analyze_sixes(&coords, base);

    // Show z-slices for coprime values (the constrained outer coords)
    let coprime_z_values: Vec<u32> = (1..base).filter(|&z| is_coprime(z, base)).collect();

    println!("═══════════════════════════════════════════════════════════════");
    println!("Z-SLICE VISUALIZATIONS (Coprime z-values only)");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!(
        "Showing z ∈ {:?} (φ({}) = {} coprime values)",
        coprime_z_values,
        base,
        coprime_z_values.len()
    );
    println!();

    for &z_val in &coprime_z_values {
        visualize_z_slice(&coords, z_val, base);
    }

    // Projection views
    visualize_projection(&coords, "xy", base);
    visualize_projection(&coords, "xz", base);
    visualize_projection(&coords, "yz", base);

    println!("═══════════════════════════════════════════════════════════════");
    println!("INTERPRETATION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("The 3D coordinate space reveals:");
    println!("  1. Outer shell (z) constraint: only φ(base) values active");
    println!("  2. Inner coordinates (x,y) distribute across coprime z-layers");
    println!("  3. Projections show correlation structure between dimensions");
    println!("  4. Pattern of 6 shows up in various geometric relationships");
    println!();
    println!("This visualization helps understand how symmetry and coprimality");
    println!("create structured patterns in 3D prime constellation space.");
    println!();
}
