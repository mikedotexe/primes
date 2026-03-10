use primes::hzlib::verification::verify_to_csv;
use primes::hzlib::{join_sample_and_model, load_explain_json};
use std::env;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);
    let sample = PathBuf::from(args.next().expect("sample_csv"));
    let model = PathBuf::from(args.next().expect("model_csv"));
    let explain_json = args.next().map(PathBuf::from);
    let out = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("hz_out/verification_results.csv"));

    let sample_rows = primes::hzlib::load_sample_csv(&sample)?;
    let model_rows = primes::hzlib::load_model_csv(&model)?;
    let grid = join_sample_and_model(&sample_rows, &model_rows);

    let explain_map = match explain_json {
        Some(p) => Some(load_explain_json(p)?),
        None => None,
    };

    verify_to_csv(&grid, explain_map.as_ref(), out)?;
    Ok(())
}
