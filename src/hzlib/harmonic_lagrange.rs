//! Lagrange polynomial fitting for lineouts

use super::grid_analysis::{Axis, JoinedGrid, lineout};

/// Fit a polynomial to a lineout and return (x, y_obs, y_fit)
pub fn fit_lineout_poly(
    grid: &JoinedGrid,
    axis: Axis,
    fixed_mid: usize,
    fixed_iz: usize,
    degree: usize,
    quantity: &str,
) -> Vec<(usize, f64, f64)> {
    let line = lineout(grid, axis, fixed_mid, fixed_iz);
    if line.is_empty() {
        return vec![];
    }

    // Extract x and y values
    let xs: Vec<f64> = line.iter().map(|(x, _, _)| *x as f64).collect();
    let ys: Vec<f64> = line
        .iter()
        .map(|(_, obs, pred)| match quantity {
            "obs" => *obs,
            "pred" => *pred,
            "enrichment" => if *pred > 0.0 { obs / pred - 1.0 } else { 0.0 },
            _ => 0.0,
        })
        .collect();

    // Fit polynomial
    let coeffs = fit_polynomial(&xs, &ys, degree);

    // Evaluate polynomial at each x
    line.iter()
        .map(|(x, obs, pred)| {
            let y_obs = match quantity {
                "obs" => *obs,
                "pred" => *pred,
                "enrichment" => if *pred > 0.0 { obs / pred - 1.0 } else { 0.0 },
                _ => 0.0,
            };
            let y_fit = eval_polynomial(&coeffs, *x as f64);
            (*x, y_obs, y_fit)
        })
        .collect()
}

/// Fit polynomial using least squares (Vandermonde matrix approach)
/// Returns coefficients [c0, c1, c2, ...] for c0 + c1*x + c2*x^2 + ...
fn fit_polynomial(xs: &[f64], ys: &[f64], degree: usize) -> Vec<f64> {
    let n = xs.len();
    if n == 0 || degree >= n {
        return vec![];
    }

    // Build Vandermonde matrix A and solve A^T A c = A^T y
    // This is a simple least squares approach (not numerically optimal for high degrees)
    let d = degree + 1;
    let mut ata = vec![vec![0.0; d]; d];
    let mut aty = vec![0.0; d];

    for i in 0..n {
        let x = xs[i];
        let y = ys[i];
        let mut x_pow = vec![1.0];
        for _ in 1..d {
            x_pow.push(x_pow.last().unwrap() * x);
        }

        for j in 0..d {
            aty[j] += x_pow[j] * y;
            for k in 0..d {
                ata[j][k] += x_pow[j] * x_pow[k];
            }
        }
    }

    // Solve using Gaussian elimination (simple implementation)
    solve_linear_system(&ata, &aty)
}

/// Evaluate polynomial at x
fn eval_polynomial(coeffs: &[f64], x: f64) -> f64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(i, &c)| c * x.powi(i as i32))
        .sum()
}

/// Simple Gaussian elimination solver for A*x = b
fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = a.len();
    let mut a = a.to_vec();
    let mut b = b.to_vec();

    // Forward elimination
    for i in 0..n {
        // Find pivot
        let mut max_row = i;
        for k in (i + 1)..n {
            if a[k][i].abs() > a[max_row][i].abs() {
                max_row = k;
            }
        }

        // Swap rows
        a.swap(i, max_row);
        b.swap(i, max_row);

        // Eliminate column
        for k in (i + 1)..n {
            if a[i][i].abs() < 1e-10 {
                continue;
            }
            let factor = a[k][i] / a[i][i];
            for j in i..n {
                a[k][j] -= factor * a[i][j];
            }
            b[k] -= factor * b[i];
        }
    }

    // Back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        if a[i][i].abs() < 1e-10 {
            x[i] = 0.0;
            continue;
        }
        let mut sum = b[i];
        for j in (i + 1)..n {
            sum -= a[i][j] * x[j];
        }
        x[i] = sum / a[i][i];
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_fit() {
        // Fit y = 2x + 1
        let xs = vec![0.0, 1.0, 2.0, 3.0];
        let ys = vec![1.0, 3.0, 5.0, 7.0];
        let coeffs = fit_polynomial(&xs, &ys, 1);
        assert_eq!(coeffs.len(), 2);
        assert!((coeffs[0] - 1.0).abs() < 1e-6);
        assert!((coeffs[1] - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_quadratic_fit() {
        // Fit y = x^2
        let xs = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let ys = vec![0.0, 1.0, 4.0, 9.0, 16.0];
        let coeffs = fit_polynomial(&xs, &ys, 2);
        assert_eq!(coeffs.len(), 3);
        assert!(coeffs[0].abs() < 1e-6);
        assert!(coeffs[1].abs() < 1e-6);
        assert!((coeffs[2] - 1.0).abs() < 1e-6);
    }
}
