use num_bigint::BigUint;
use plotters::prelude::*;
use prime_physics_engine::is_prime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body1 = "7";
    let body2 = "11";
    let space_size = 50; // Increase space size for a more detailed landscape

    let out_file_name = "lagrange_landscape.png";
    let root = BitMapBackend::new(out_file_name, (1024, 768)).into_drawing_area();
    root.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .margin(5)
        .build_cartesian_2d(0..space_size, 1..10)?;

    chart
        .configure_mesh()
        .x_desc("Position")
        .y_desc("Digit")
        .draw()?;

    let zeros = "0".repeat(space_size);

    for position in 0..space_size {
        for digit in 1..=9 {
            let mut test_str = zeros.clone();
            unsafe {
                test_str.as_bytes_mut()[position] = b'0' + digit as u8;
            }

            let full_number = format!("{}{}{}", body1, test_str, body2);
            let num = full_number.parse::<BigUint>().unwrap();

            if is_prime(&num) {
                chart.draw_series(PointSeries::of_element(
                    [(position, digit)],
                    5,
                    &WHITE.mix(0.8),
                    &|c, s, st| {
                        return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                        + Circle::new((0,0),s,st.filled()) // At backend position c, draw a circle with radius s
                        + Text::new(format!("({},{})", c.0, c.1), (10, 0), ("sans-serif", 10).into_font());
                    }
                ))?;
            } else {
                chart.draw_series(PointSeries::of_element(
                    [(position, digit)],
                    1,
                    &BLUE.mix(0.2),
                    &|c, s, st| {
                        return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                        + Circle::new((0,0),s,st.filled()); // At backend position c, draw a circle with radius s
                    },
                ))?;
            }
        }
    }

    root.present()?;
    println!(
        "Lagrange landscape visualization saved to {}",
        out_file_name
    );

    Ok(())
}
