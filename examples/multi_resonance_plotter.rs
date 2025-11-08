
use plotters::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_data_file(file_path: &str) -> Result<Vec<(u32, u32)>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        if index == 0 { continue; } // Skip header
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let space_size: u32 = parts[0].parse()?;
            let prime_yield: u32 = parts[1].parse()?;
            data.push((space_size, prime_yield));
        }
    }
    Ok(data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let datasets_info = vec![
        ("body_resonance_7_11.csv", "7-11 (Control)", &RED),
        ("body_resonance_4_25.csv", "4-25 (Composite Test)", &BLUE),
    ];

    let out_file_name = "multi_resonance_chart.png";
    let root = BitMapBackend::new(out_file_name, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut max_x = 0;
    let mut max_y = 0;
    let mut all_data = Vec::new();

    for (file_path, name, _) in &datasets_info {
        let data = read_data_file(file_path)?;
        let (local_max_x, local_max_y) = data.iter().fold((0, 0), |(mx, my), (x, y)| (mx.max(*x), my.max(*y)));
        max_x = max_x.max(local_max_x);
        max_y = max_y.max(local_max_y);
        all_data.push((name.to_string(), data));
    }

    let mut chart = ChartBuilder::on(&root)
        .caption("Resonance Profiles by Body Type", ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0u32..max_x, 0u32..max_y)?;

    chart.configure_mesh()
        .x_desc("Space Size")
        .y_desc("Prime Yield")
        .draw()?;

    for (index, (name, data)) in all_data.iter().enumerate() {
        let color = datasets_info[index].2;
        chart.draw_series(LineSeries::new(data.clone(), color.stroke_width(2)))?
            .label(name.clone())
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    println!("Multi-resonance chart saved to {}", out_file_name);

    Ok(())
}
