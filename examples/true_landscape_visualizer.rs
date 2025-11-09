use num_bigint::BigUint;
use plotters::prelude::*;
use primes::is_prime;
use std::f64::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body1 = "7";
    let body2 = "11";
    let space_size = 100;

    let out_file_name = "true_complex_landscape.png";
    let root = BitMapBackend::new(out_file_name, (800, 800)).into_drawing_area();
    root.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root).build_cartesian_2d(-10.0..10.0, -10.0..10.0)?;

    chart.configure_mesh().draw()?;

    let mut prime_points = Vec::new();

    for position in 0..space_size {
        for digit in 1..=9 {
            let mut test_str = "0".repeat(space_size);
            unsafe {
                test_str.as_bytes_mut()[position] = b'0' + digit as u8;
            }

            let full_number = format!("{}{}{}", body1, test_str, body2);
            if let Ok(num) = full_number.parse::<BigUint>() {
                if is_prime(&num) {
                    let r = digit as f64;
                    let theta = 2.0 * PI * (position as f64) / (space_size as f64);
                    let x = r * theta.cos();
                    let y = r * theta.sin();
                    prime_points.push((x, y));
                }
            }
        }
    }

    chart.draw_series(PointSeries::of_element(
        prime_points,
        2,
        &WHITE.mix(0.8),
        &|c, s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
        },
    ))?;

    root.present()?;
    println!(
        "True complex landscape visualization saved to {}",
        out_file_name
    );

    Ok(())
}
