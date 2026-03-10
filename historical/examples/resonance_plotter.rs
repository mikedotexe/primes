use plotters::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_file = "resonance_data.csv";
    let file = File::open(data_file)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        if index == 0 {
            continue;
        } // Skip header row
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let space_size: u32 = parts[0].parse()?;
            let prime_yield: u32 = parts[1].parse()?;
            data.push((space_size, prime_yield));
        }
    }

    let out_file_name = "resonance_chart.png";
    let root = BitMapBackend::new(out_file_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (max_x, max_y) = data
        .iter()
        .fold((0, 0), |(mx, my), (x, y)| (mx.max(*x), my.max(*y)));

    let mut chart = ChartBuilder::on(&root)
        .caption("Prime Yield vs. Space Size", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0u32..max_x, 0u32..max_y)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(data, &RED))?;

    root.present()?;
    println!("Resonance chart saved to {}", out_file_name);

    Ok(())
}
