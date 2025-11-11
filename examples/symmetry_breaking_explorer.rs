use std::env;
use std::path::PathBuf;
use primes::hzlib::{join_sample_and_model, Axis};
use primes::hzlib::symmetry_breaking::ridge_trough;

fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);
    let sample = PathBuf::from(args.next().expect("sample_csv"));
    let model  = PathBuf::from(args.next().expect("model_csv"));
    let axis = args.next().unwrap_or_else(|| "mid".into());
    let qty = args.next().unwrap_or_else(|| "pred".into());

    let sample_rows = primes::hzlib::load_sample_csv(&sample)?;
    let model_rows  = primes::hzlib::load_model_csv(&model)?;
    let grid = join_sample_and_model(&sample_rows, &model_rows);

    let axis = if axis=="mid" { Axis::Mid } else { Axis::InnerZero };
    let ridges = ridge_trough(&grid, axis, &qty);

    println!("# key,argmin,value");
    for r in ridges {
        println!("{},{},{}", r.key, r.argmin, r.value);
    }
    Ok(())
}
