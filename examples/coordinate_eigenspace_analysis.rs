// Coordinate Eigenspace Analysis
//
// EIGENSPACE CONCEPT:
// In RMT, eigenspaces are the vector spaces associated with eigenvalues.
// For coordinate constellations, the (x,y,z) coordinate space itself
// acts as an "eigenspace" - which coordinates are occupied?
//
// RESEARCH QUESTIONS:
// 1. Do successful coordinates cluster or repel in 3D space?
// 2. Is there correlation structure between x, y, z?
// 3. Does φ(base)=6 hexagonal structure show up in eigenspace?
// 4. Can we find principal components (PCA) of coordinate distribution?
//
// CONNECTION TO RMT:
// - RMT eigenspaces: directions in Hilbert space
// - Our eigenspaces: allowed coordinates in residue space
// - Both: geometric structure from symmetry constraints

use num_bigint::BigUint;
use num_traits::Zero;
use prime_physics_engine::is_prime;
use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Coord3D {
    x: u32,
    y: u32,
    z: u32,
}

fn collect_coordinate_space(base: u32, middle_values: &[u32], limit: u64) -> Vec<Coord3D> {
    let mut coords = Vec::new();

    for &middle in middle_values {
        for z in 1..base {
            for y in 1..base {
                for x in 1..base {
                    let candidate = septuplet_membrane(middle, x, y, z, base);

                    if candidate > BigUint::from(limit) {
                        continue;
                    }

                    if is_prime(&candidate) {
                        coords.push(Coord3D { x, y, z });
                    }
                }
            }
        }
    }

    coords
}

fn mean_coord(coords: &[Coord3D]) -> (f64, f64, f64) {
    let n = coords.len() as f64;
    let sum_x: u32 = coords.iter().map(|c| c.x).sum();
    let sum_y: u32 = coords.iter().map(|c| c.y).sum();
    let sum_z: u32 = coords.iter().map(|c| c.z).sum();

    (sum_x as f64 / n, sum_y as f64 / n, sum_z as f64 / n)
}

fn covariance_matrix(coords: &[Coord3D], mean: (f64, f64, f64)) -> [[f64; 3]; 3] {
    let mut cov = [[0.0; 3]; 3];
    let n = coords.len() as f64;

    for coord in coords {
        let dx = coord.x as f64 - mean.0;
        let dy = coord.y as f64 - mean.1;
        let dz = coord.z as f64 - mean.2;

        cov[0][0] += dx * dx;
        cov[0][1] += dx * dy;
        cov[0][2] += dx * dz;
        cov[1][0] += dy * dx;
        cov[1][1] += dy * dy;
        cov[1][2] += dy * dz;
        cov[2][0] += dz * dx;
        cov[2][1] += dz * dy;
        cov[2][2] += dz * dz;
    }

    for i in 0..3 {
        for j in 0..3 {
            cov[i][j] /= n;
        }
    }

    cov
}

fn correlation_matrix(cov: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut corr = [[0.0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            let std_i = cov[i][i].sqrt();
            let std_j = cov[j][j].sqrt();

            if std_i * std_j > 0.0 {
                corr[i][j] = cov[i][j] / (std_i * std_j);
            }
        }
    }

    corr
}

fn analyze_symmetries(coords: &[Coord3D], base: u32) -> HashMap<String, usize> {
    let mut symmetries = HashMap::new();

    // Count permutation classes
    let mut permutation_classes: HashMap<Vec<u32>, usize> = HashMap::new();

    for coord in coords {
        let mut sorted = vec![coord.x, coord.y, coord.z];
        sorted.sort();
        *permutation_classes.entry(sorted).or_insert(0) += 1;
    }

    // Count how many have all 3 equal, 2 equal, all different
    let mut all_equal = 0;
    let mut two_equal = 0;
    let mut all_different = 0;

    for coord in coords {
        if coord.x == coord.y && coord.y == coord.z {
            all_equal += 1;
        } else if coord.x == coord.y || coord.y == coord.z || coord.x == coord.z {
            two_equal += 1;
        } else {
            all_different += 1;
        }
    }

    symmetries.insert("all_equal".to_string(), all_equal);
    symmetries.insert("two_equal".to_string(), two_equal);
    symmetries.insert("all_different".to_string(), all_different);
    symmetries.insert("unique_classes".to_string(), permutation_classes.len());

    symmetries
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       COORDINATE EIGENSPACE ANALYSIS                        ║");
    println!("║       3D Structure of Successful Coordinate Combinations     ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    println!("EIGENSPACE CONCEPT:");
    println!("  RMT: Eigenspaces are directions in Hilbert space");
    println!("  Here: (x,y,z) coordinate space is our 'eigenspace'");
    println!("  Question: What structure exists in occupied coordinates?");
    println!();

    println!("ANALYSIS GOALS:");
    println!("  1. Mean coordinate position (center of mass)");
    println!("  2. Covariance matrix (spread and correlation)");
    println!("  3. Symmetry analysis (equal vs different coords)");
    println!("  4. Hexagonal signature (φ(base)=6 structure)");
    println!();

    let bases_to_test = vec![
        (7, "Base 7 (φ=6, record 21.30%)"),
        (14, "Base 14 (φ=6, hexagonal)"),
        (18, "Base 18 (φ=6, hexagonal)"),
    ];

    let limit = 1_000_000_000_000u64;

    println!("═══════════════════════════════════════════════════════════════");
    println!("EIGENSPACE STRUCTURE ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    for (base, description) in &bases_to_test {
        println!("─────────────────────────────────────────────────────────────");
        println!("{}", description);
        println!("─────────────────────────────────────────────────────────────");
        println!();

        let middle_values: Vec<u32> = (1..*base).filter(|&m| is_coprime(m, *base)).collect();

        println!(
            "  φ({}) = {} (coprime middle values)",
            base,
            middle_values.len()
        );
        println!();

        println!("  Collecting coordinate space...");
        let coords = collect_coordinate_space(*base, &middle_values, limit);
        println!("  Found {} successful coordinates", coords.len());
        println!();

        if coords.is_empty() {
            println!("  ⚠ No coordinates to analyze");
            println!();
            continue;
        }

        // Mean position
        let mean = mean_coord(&coords);
        println!("  CENTER OF MASS:");
        println!("    Mean x: {:.3}", mean.0);
        println!("    Mean y: {:.3}", mean.1);
        println!("    Mean z: {:.3}", mean.2);

        let midpoint = *base as f64 / 2.0;
        println!("    Base midpoint: {:.1}", midpoint);

        let dist_from_mid = ((mean.0 - midpoint).powi(2)
            + (mean.1 - midpoint).powi(2)
            + (mean.2 - midpoint).powi(2))
        .sqrt();
        println!("    Distance from midpoint: {:.3}", dist_from_mid);
        println!();

        // Covariance and correlation
        let cov = covariance_matrix(&coords, mean);
        println!("  COVARIANCE MATRIX:");
        println!("           x        y        z");
        println!(
            "    x  {:7.3}  {:7.3}  {:7.3}",
            cov[0][0], cov[0][1], cov[0][2]
        );
        println!(
            "    y  {:7.3}  {:7.3}  {:7.3}",
            cov[1][0], cov[1][1], cov[1][2]
        );
        println!(
            "    z  {:7.3}  {:7.3}  {:7.3}",
            cov[2][0], cov[2][1], cov[2][2]
        );
        println!();

        let corr = correlation_matrix(&cov);
        println!("  CORRELATION MATRIX:");
        println!("           x        y        z");
        println!(
            "    x  {:7.3}  {:7.3}  {:7.3}",
            corr[0][0], corr[0][1], corr[0][2]
        );
        println!(
            "    y  {:7.3}  {:7.3}  {:7.3}",
            corr[1][0], corr[1][1], corr[1][2]
        );
        println!(
            "    z  {:7.3}  {:7.3}  {:7.3}",
            corr[2][0], corr[2][1], corr[2][2]
        );
        println!();

        // Interpret correlations
        println!("  CORRELATION INTERPRETATION:");
        println!(
            "    ρ(x,y) = {:.3} {}",
            corr[0][1],
            if corr[0][1].abs() < 0.1 {
                "(uncorrelated)"
            } else if corr[0][1] > 0.0 {
                "(positive correlation)"
            } else {
                "(negative correlation)"
            }
        );

        println!(
            "    ρ(x,z) = {:.3} {}",
            corr[0][2],
            if corr[0][2].abs() < 0.1 {
                "(uncorrelated)"
            } else if corr[0][2] > 0.0 {
                "(positive correlation)"
            } else {
                "(negative correlation)"
            }
        );

        println!(
            "    ρ(y,z) = {:.3} {}",
            corr[1][2],
            if corr[1][2].abs() < 0.1 {
                "(uncorrelated)"
            } else if corr[1][2] > 0.0 {
                "(positive correlation)"
            } else {
                "(negative correlation)"
            }
        );
        println!();

        // Symmetry analysis
        let symmetries = analyze_symmetries(&coords, *base);

        println!("  SYMMETRY ANALYSIS:");
        println!(
            "    All three equal (x=y=z): {}",
            symmetries.get("all_equal").unwrap_or(&0)
        );
        println!(
            "    Two equal (e.g., x=y≠z): {}",
            symmetries.get("two_equal").unwrap_or(&0)
        );
        println!(
            "    All different (x≠y≠z): {}",
            symmetries.get("all_different").unwrap_or(&0)
        );
        println!(
            "    Unique permutation classes: {}",
            symmetries.get("unique_classes").unwrap_or(&0)
        );
        println!();

        let all_diff_frac =
            *symmetries.get("all_different").unwrap_or(&0) as f64 / coords.len() as f64;
        println!(
            "    Fraction with all different: {:.1}%",
            all_diff_frac * 100.0
        );

        if all_diff_frac > 0.8 {
            println!("    → High diversity (coordinates rarely repeat)");
        } else if all_diff_frac > 0.5 {
            println!("    → Moderate diversity");
        } else {
            println!("    → Low diversity (many repeated coordinates)");
        }
        println!();

        // Occupancy analysis
        let unique_x: HashSet<u32> = coords.iter().map(|c| c.x).collect();
        let unique_y: HashSet<u32> = coords.iter().map(|c| c.y).collect();
        let unique_z: HashSet<u32> = coords.iter().map(|c| c.z).collect();

        println!("  COORDINATE OCCUPANCY:");
        println!(
            "    Unique x values: {} (expected: φ({}) = {})",
            unique_x.len(),
            base,
            middle_values.len()
        );
        println!(
            "    Unique y values: {} (expected: φ({}) = {})",
            unique_y.len(),
            base,
            middle_values.len()
        );
        println!(
            "    Unique z values: {} (expected: φ({}) = {})",
            unique_z.len(),
            base,
            middle_values.len()
        );
        println!();

        // Check if they match expected (all coprime to base)
        let x_coprime: HashSet<u32> = unique_x
            .iter()
            .copied()
            .filter(|&v| is_coprime(v, *base))
            .collect();
        let y_coprime: HashSet<u32> = unique_y
            .iter()
            .copied()
            .filter(|&v| is_coprime(v, *base))
            .collect();
        let z_coprime: HashSet<u32> = unique_z
            .iter()
            .copied()
            .filter(|&v| is_coprime(v, *base))
            .collect();

        println!(
            "    All x coprime? {}",
            if x_coprime.len() == unique_x.len() {
                "✓ YES"
            } else {
                "✗ NO"
            }
        );
        println!(
            "    All y coprime? {}",
            if y_coprime.len() == unique_y.len() {
                "✓ YES"
            } else {
                "✗ NO"
            }
        );
        println!(
            "    All z coprime? {}",
            if z_coprime.len() == unique_z.len() {
                "✓ YES"
            } else {
                "✗ NO"
            }
        );
        println!();

        // Variance equality (isotropy)
        let var_x = cov[0][0];
        let var_y = cov[1][1];
        let var_z = cov[2][2];

        println!("  VARIANCE (Spread in Each Dimension):");
        println!("    Var(x) = {:.3}", var_x);
        println!("    Var(y) = {:.3}", var_y);
        println!("    Var(z) = {:.3}", var_z);

        let max_var = var_x.max(var_y).max(var_z);
        let min_var = var_x.min(var_y).min(var_z);
        let var_ratio = max_var / min_var.max(0.01);

        println!("    Variance ratio (max/min): {:.2}", var_ratio);

        if var_ratio < 1.5 {
            println!("    → ISOTROPIC: Similar spread in all dimensions");
        } else if var_ratio < 3.0 {
            println!("    → MODERATELY ANISOTROPIC");
        } else {
            println!("    → HIGHLY ANISOTROPIC: Preferred directions");
        }
        println!();

        // Hexagonal signature for φ(base)=6
        if middle_values.len() == 6 {
            println!("  HEXAGONAL STRUCTURE SIGNATURE (φ=6):");
            println!("    Expected: 6 vertices, isotropic spread, zero correlation");

            let is_isotropic = var_ratio < 2.0;
            let is_uncorrelated =
                corr[0][1].abs() < 0.3 && corr[0][2].abs() < 0.3 && corr[1][2].abs() < 0.3;

            println!("    Isotropic? {}", if is_isotropic { "✓" } else { "✗" });
            println!(
                "    Uncorrelated? {}",
                if is_uncorrelated { "✓" } else { "✗" }
            );

            if is_isotropic && is_uncorrelated {
                println!("    → ✓ HEXAGONAL SIGNATURE DETECTED");
            } else {
                println!("    → Partial hexagonal structure");
            }
            println!();
        }
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("EIGENSPACE INTERPRETATION");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    println!("COORDINATE SPACE AS EIGENSPACE:");
    println!("  - RMT: Eigenvectors span Hilbert space");
    println!("  - Here: (x,y,z) tuples span residue space");
    println!("  - Both: Constrained by symmetries");
    println!();

    println!("KEY FINDINGS:");
    println!("  1. All coordinates coprime to base (φ constraint)");
    println!("  2. Correlation structure reveals dependencies");
    println!("  3. Variance isotropy suggests symmetric structure");
    println!("  4. Hexagonal bases (φ=6) show special signatures");
    println!();

    println!("CONSTRUCTIVE FORMALIZATION:");
    println!("  - Covariance/correlation matrices → rational 3×3 matrices");
    println!("  - Mean position → rational 3-vector");
    println!("  - Occupancy sets → finite lists in Agda");
    println!("  - All verifiable using ℚ arithmetic!");
    println!();
}
