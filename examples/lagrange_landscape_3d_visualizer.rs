use num_bigint::BigUint;
use plotters::prelude::*;
use primes::is_prime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body1 = "7";
    let body2 = "11";
    let max_space_size = 20; // Keep this reasonably small for faster execution

    let out_file_name = "lagrange_landscape_3d.png";
    let root = BitMapBackend::new(out_file_name, (1024, 768)).into_drawing_area();
    root.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root).margin(20).build_cartesian_3d(
        0..max_space_size,
        1..10,
        0..max_space_size,
    )?;

    chart.configure_axes().draw()?;

    let mut prime_points = Vec::new();

    for space_size in 1..=max_space_size {
        let zeros = "0".repeat(space_size);
        for position in 0..space_size {
            for digit in 1..=9 {
                let mut test_str = zeros.clone();
                unsafe {
                    test_str.as_bytes_mut()[position] = b'0' + digit as u8;
                }

                let full_number = format!("{}{}{}", body1, test_str, body2);
                if let Ok(num) = full_number.parse::<BigUint>() {
                    if is_prime(&num) {
                        prime_points.push((position, digit, space_size));
                    }
                }
            }
        }
    }

    chart.draw_series(PointSeries::of_element(
        prime_points,
        2, // size of the point
        &WHITE.mix(0.8),
        &|c, s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
        },
    ))?;

    root.present()?;
    println!(
        "3D Lagrange landscape visualization saved to {}",
        out_file_name
    );

    Ok(())
}
