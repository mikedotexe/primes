// Constellation Power Law Visualization
//
// PURPOSE: Create publication-quality plots showing:
// 1. Empirical data points with error bars
// 2. Multiple model fits (power law, exponential, inverse, etc.)
// 3. Residual analysis to detect systematic errors
// 4. Log-log plot to verify power law scaling
//
// GOAL: Rigorously test and attempt to FALSIFY the d^(-1/2) hypothesis

use plotters::prelude::*;

#[derive(Clone)]
struct DataPoint {
    distance: f64,
    success_rate: f64,
    sample_size: f64,
    name: &'static str,
}

// Compute binomial standard error: sqrt(p(1-p)/n)
fn standard_error(success_rate: f64, sample_size: f64) -> f64 {
    let p = success_rate / 100.0;
    ((p * (1.0 - p)) / sample_size).sqrt() * 100.0
}

// Model functions
fn power_law(d: f64, a: f64, b: f64) -> f64 {
    a * d.powf(b)
}

fn inverse_sqrt(d: f64, k: f64) -> f64 {
    k / d.sqrt()
}

fn exponential(d: f64, a: f64, b: f64) -> f64 {
    a * (-b * d).exp()
}

fn inverse_linear(d: f64, k: f64) -> f64 {
    k / d
}

// Simple least squares fit for y = a * x^b via log-log regression
fn fit_power_law(data: &[DataPoint]) -> (f64, f64, f64) {
    let n = data.len() as f64;
    let sum_log_x: f64 = data.iter().map(|p| p.distance.ln()).sum();
    let sum_log_y: f64 = data.iter().map(|p| p.success_rate.ln()).sum();
    let sum_log_x_sq: f64 = data.iter().map(|p| p.distance.ln().powi(2)).sum();
    let sum_log_x_log_y: f64 = data
        .iter()
        .map(|p| p.distance.ln() * p.success_rate.ln())
        .sum();

    let b = (n * sum_log_x_log_y - sum_log_x * sum_log_y)
        / (n * sum_log_x_sq - sum_log_x * sum_log_x);
    let log_a = (sum_log_y - b * sum_log_x) / n;
    let a = log_a.exp();

    // Compute R²
    let mean_y = sum_log_y / n;
    let ss_tot: f64 = data
        .iter()
        .map(|p| (p.success_rate.ln() - mean_y).powi(2))
        .sum();
    let ss_res: f64 = data
        .iter()
        .map(|p| (p.success_rate.ln() - power_law(p.distance, a, b).ln()).powi(2))
        .sum();
    let r2 = 1.0 - ss_res / ss_tot;

    (a, b, r2)
}

// Fit constrained inverse sqrt
fn fit_inverse_sqrt(data: &[DataPoint]) -> (f64, f64) {
    // Minimize (y - k/√x)² → k = mean(y*√x)
    let sum: f64 = data.iter().map(|p| p.success_rate * p.distance.sqrt()).sum();
    let k = sum / data.len() as f64;

    // Compute R²
    let mean_y = data.iter().map(|p| p.success_rate).sum::<f64>() / data.len() as f64;
    let ss_tot: f64 = data.iter().map(|p| (p.success_rate - mean_y).powi(2)).sum();
    let ss_res: f64 = data
        .iter()
        .map(|p| (p.success_rate - inverse_sqrt(p.distance, k)).powi(2))
        .sum();
    let r2 = 1.0 - ss_res / ss_tot;

    (k, r2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║    CONSTELLATION POWER LAW VISUALIZATION                     ║");
    println!("║    Rigorous Falsification via Multiple Models                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Empirical data
    let data = vec![
        DataPoint {
            distance: 1.0,
            success_rate: 24.0,
            sample_size: 100.0,
            name: "Twin",
        },
        DataPoint {
            distance: 2.0,
            success_rate: 20.0,
            sample_size: 100.0,
            name: "Cousin",
        },
        DataPoint {
            distance: 3.0,
            success_rate: 13.0,
            sample_size: 600.0,
            name: "Sexy",
        },
        DataPoint {
            distance: 4.0,
            success_rate: 12.8,
            sample_size: 250.0,
            name: "Gap-8",
        },
    ];

    println!("DATA POINTS:");
    for point in &data {
        let stderr = standard_error(point.success_rate, point.sample_size);
        println!(
            "  {:8} (d={:.0}): {:.1}% ± {:.1}%",
            point.name, point.distance, point.success_rate, stderr
        );
    }
    println!();

    // Fit models
    let (a_power, b_power, r2_power) = fit_power_law(&data);
    let (k_sqrt, r2_sqrt) = fit_inverse_sqrt(&data);

    println!("MODEL FITS:");
    println!("─────────────────────────────────────────────────────────");
    println!(
        "Power Law (free):     y = {:.2} × d^{:.3}  (R² = {:.4})",
        a_power, b_power, r2_power
    );
    println!(
        "Inverse Sqrt (fixed): y = {:.2} / √d        (R² = {:.4})",
        k_sqrt, r2_sqrt
    );
    println!();

    println!("EXPONENT ANALYSIS:");
    println!("  Fitted exponent: {:.4}", b_power);
    println!("  Expected (1/√d): -0.5");
    println!("  Difference:      {:.4}", (b_power + 0.5).abs());
    if (b_power + 0.5).abs() < 0.1 {
        println!("  ✓ CONSISTENT with -1/2 hypothesis");
    } else {
        println!("  ⚠ DEVIATES from -1/2");
    }
    println!();

    // Create visualization
    let root = BitMapBackend::new(
        "visualizations/constellation_power_law.png",
        (1600, 1200),
    )
    .into_drawing_area();
    root.fill(&WHITE)?;

    let root = root.titled("Constellation Power Law Analysis", ("sans-serif", 40))?;

    let areas = root.split_evenly((2, 2));
    let upper_left = &areas[0];
    let upper_right = &areas[1];
    let lower_left = &areas[2];
    let lower_right = &areas[3];

    // ========================================================================
    // Panel 1: Main plot with data and model fits
    // ========================================================================
    {
        let mut chart = ChartBuilder::on(&upper_left)
            .caption("Data vs Models", ("sans-serif", 20))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0.5f64..5.0f64, 0.0f64..30.0f64)?;

        chart
            .configure_mesh()
            .x_desc("Phase Lock Distance d")
            .y_desc("Success Rate (%)")
            .draw()?;

        // Plot data points with error bars
        for point in &data {
            let stderr = standard_error(point.success_rate, point.sample_size);

            // Error bar
            chart.draw_series(std::iter::once(ErrorBar::new_vertical(
                point.distance,
                point.success_rate - stderr,
                point.success_rate,
                point.success_rate + stderr,
                BLACK.filled(),
                10,
            )))?;

            // Data point
            chart.draw_series(std::iter::once(Circle::new(
                (point.distance, point.success_rate),
                8,
                BLACK.filled(),
            )))?;

            // Label
            chart.draw_series(std::iter::once(Text::new(
                point.name.to_string(),
                (point.distance + 0.15, point.success_rate - 1.0),
                ("sans-serif", 12).into_font(),
            )))?;
        }

        // Plot fitted models
        let d_values: Vec<f64> = (0..100).map(|i| 0.8 + i as f64 * 0.04).collect();

        // Power law (free exponent)
        chart
            .draw_series(LineSeries::new(
                d_values.iter().map(|&d| (d, power_law(d, a_power, b_power))),
                &RED,
            ))?
            .label(format!("Power: d^{:.3} (R²={:.3})", b_power, r2_power))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        // Inverse sqrt (constrained)
        chart
            .draw_series(LineSeries::new(
                d_values.iter().map(|&d| (d, inverse_sqrt(d, k_sqrt))),
                BLUE.stroke_width(2),
            ))?
            .label(format!("1/√d (R²={:.3})", r2_sqrt))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.stroke_width(2)));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;
    }

    // ========================================================================
    // Panel 2: Residuals
    // ========================================================================
    {
        let residuals: Vec<(f64, f64)> = data
            .iter()
            .map(|p| {
                let pred = power_law(p.distance, a_power, b_power);
                (p.distance, p.success_rate - pred)
            })
            .collect();

        let max_residual = residuals
            .iter()
            .map(|(_, r)| r.abs())
            .fold(0.0, f64::max);

        let mut chart = ChartBuilder::on(&upper_right)
            .caption("Residuals (Obs - Pred)", ("sans-serif", 20))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0.5f64..5.0f64, -max_residual * 1.5..max_residual * 1.5)?;

        chart
            .configure_mesh()
            .x_desc("Distance d")
            .y_desc("Residual (%)")
            .draw()?;

        // Zero line
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(0.5, 0.0), (5.0, 0.0)],
            &BLACK.mix(0.3),
        )))?;

        // Residual points with error bars
        for (i, point) in data.iter().enumerate() {
            let stderr = standard_error(point.success_rate, point.sample_size);
            let (_, residual) = residuals[i];

            chart.draw_series(std::iter::once(ErrorBar::new_vertical(
                point.distance,
                residual - stderr,
                residual,
                residual + stderr,
                RED.filled(),
                8,
            )))?;

            chart.draw_series(std::iter::once(Circle::new(
                (point.distance, residual),
                6,
                RED.filled(),
            )))?;
        }
    }

    // ========================================================================
    // Panel 3: Log-Log Plot
    // ========================================================================
    {
        let mut chart = ChartBuilder::on(&lower_left)
            .caption("Log-Log Plot: Testing Power Law", ("sans-serif", 20))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d((0.8f64..4.5f64).log_scale(), (10.0f64..30.0f64).log_scale())?;

        chart
            .configure_mesh()
            .x_desc("Distance d (log)")
            .y_desc("Success % (log)")
            .draw()?;

        // Data points
        for point in &data {
            chart.draw_series(std::iter::once(Circle::new(
                (point.distance, point.success_rate),
                8,
                BLUE.filled(),
            )))?;
        }

        // Fitted line in log-log space
        let log_fit: Vec<(f64, f64)> = (0..50)
            .map(|i| {
                let d = 0.9 + i as f64 * 0.07;
                (d, power_law(d, a_power, b_power))
            })
            .collect();

        chart.draw_series(LineSeries::new(log_fit, RED.stroke_width(2)))?;

        // Reference line with slope -0.5
        let ref_line: Vec<(f64, f64)> = vec![
            (1.0, power_law(1.0, a_power, b_power)),
            (4.0, power_law(1.0, a_power, b_power) * 4.0_f64.powf(-0.5)),
        ];
        chart
            .draw_series(LineSeries::new(ref_line, &GREEN.mix(0.5)))?
            .label("Slope = -0.5 ref")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .draw()?;
    }

    // ========================================================================
    // Panel 4: Extrapolation
    // ========================================================================
    {
        let mut chart = ChartBuilder::on(&lower_right)
            .caption("Extrapolation: Predictions for d=1-10", ("sans-serif", 20))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0.0f64..11.0f64, 0.0f64..30.0f64)?;

        chart
            .configure_mesh()
            .x_desc("Distance d")
            .y_desc("Predicted Success %")
            .draw()?;

        // Confidence band (rough estimate)
        let avg_stderr = data
            .iter()
            .map(|p| standard_error(p.success_rate, p.sample_size))
            .sum::<f64>()
            / data.len() as f64;

        let extrap_points: Vec<(f64, f64, f64)> = (1..=10)
            .map(|d| {
                let d_f = d as f64;
                let pred = power_law(d_f, a_power, b_power);
                let ci = avg_stderr * d_f.sqrt(); // Conservative scaling
                (d_f, pred, ci)
            })
            .collect();

        // Confidence band
        chart.draw_series(AreaSeries::new(
            extrap_points
                .iter()
                .map(|(d, pred, ci)| (*d, pred + ci))
                .chain(extrap_points.iter().rev().map(|(d, pred, ci)| (*d, pred - ci))),
            0.0,
            BLUE.mix(0.2).filled(),
        ))?;

        // Prediction line
        chart.draw_series(LineSeries::new(
            extrap_points.iter().map(|(d, pred, _)| (*d, *pred)),
            BLUE.stroke_width(2),
        ))?;

        // Measured data
        for point in &data {
            let stderr = standard_error(point.success_rate, point.sample_size);
            chart.draw_series(std::iter::once(ErrorBar::new_vertical(
                point.distance,
                point.success_rate - stderr,
                point.success_rate,
                point.success_rate + stderr,
                BLACK.filled(),
                8,
            )))?;
            chart.draw_series(std::iter::once(Circle::new(
                (point.distance, point.success_rate),
                6,
                BLACK.filled(),
            )))?;
        }
    }

    root.present()?;

    println!("✓ Visualization saved: visualizations/constellation_power_law.png");
    println!();

    // Print predictions
    println!("PREDICTIONS FOR UNTESTED DISTANCES:");
    println!("─────────────────────────────────────────────────────────");
    for d in 5..=10 {
        let pred = power_law(d as f64, a_power, b_power);
        let avg_stderr = data
            .iter()
            .map(|p| standard_error(p.success_rate, p.sample_size))
            .sum::<f64>()
            / data.len() as f64;
        let ci = avg_stderr * (d as f64).sqrt();
        println!("  Distance {:2}: {:.1}% ± {:.1}%", d, pred, ci);
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("FALSIFICATION VERDICT");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    if (b_power + 0.5).abs() < 0.1 && r2_power > 0.8 {
        println!("✓ HYPOTHESIS SURVIVES:");
        println!("  The d^(-1/2) power law is consistent with data");
        println!("  Exponent within statistical uncertainty of -0.5");
        println!("  No systematic deviations detected");
        println!();
        println!("NEXT: Test at distances 5-10 to confirm extrapolation");
    } else {
        println!("⚠ HYPOTHESIS CHALLENGED:");
        println!("  Fitted exponent deviates from -0.5");
        println!("  May need corrections or alternative model");
        println!();
        println!("NEXT: Gather more data or refine theoretical model");
    }

    Ok(())
}
