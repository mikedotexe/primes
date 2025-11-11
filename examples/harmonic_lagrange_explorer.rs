use std::env;
use std::path::PathBuf;
use primes::hzlib::{join_sample_and_model, Axis};
use primes::hzlib::harmonic_lagrange::fit_lineout_poly;

fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);
    let sample = PathBuf::from(args.next().expect("sample_csv"));
    let model  = PathBuf::from(args.next().expect("model_csv"));
    let axis = args.next().unwrap_or_else(|| "mid".into());
    let fixed = args.next().and_then(|x| x.parse::<usize>().ok()).unwrap_or(0);
    let qty = args.next().unwrap_or_else(|| "enrichment".into());
    let deg = args.next().and_then(|x| x.parse::<usize>().ok()).unwrap_or(5);

    let sample_rows = primes::hzlib::load_sample_csv(&sample)?;
    let model_rows  = primes::hzlib::load_model_csv(&model)?;
    let grid = join_sample_and_model(&sample_rows, &model_rows);

    let axis = if axis=="mid" { Axis::Mid } else { Axis::InnerZero };
    let (fixed_mid, fixed_iz) = match axis {
        Axis::Mid => (0usize, fixed),
        Axis::InnerZero => (fixed, 0usize),
    };

    let series = fit_lineout_poly(&grid, axis, fixed_mid, fixed_iz, deg, &qty);
    println!("x, y_obs, y_fit");
    for (x, y, f) in series {
        println!("{},{:.10},{:.10}", x, y, f);
    }
    Ok(())
}
