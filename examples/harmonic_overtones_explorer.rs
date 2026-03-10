use primes::hzlib::harmonic_overtones::overtone_spectrum;
use primes::hzlib::{join_sample_and_model, Axis};
use std::env;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);
    let sample = PathBuf::from(args.next().expect("sample_csv"));
    let model = PathBuf::from(args.next().expect("model_csv"));
    let axis = args.next().unwrap_or_else(|| "mid".into());
    let fixed = args
        .next()
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(0);
    let qty = args.next().unwrap_or_else(|| "enrichment".into());
    let topk = args
        .next()
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(6);

    let sample_rows = primes::hzlib::load_sample_csv(&sample)?;
    let model_rows = primes::hzlib::load_model_csv(&model)?;
    let grid = join_sample_and_model(&sample_rows, &model_rows);

    let axis = if axis == "mid" {
        Axis::Mid
    } else {
        Axis::InnerZero
    };
    let (fixed_mid, fixed_iz) = match axis {
        Axis::Mid => (0usize, fixed),
        Axis::InnerZero => (fixed, 0usize),
    };

    let spec = overtone_spectrum(&grid, axis, fixed_mid, fixed_iz, &qty);
    for (k, amp) in spec.into_iter().take(topk) {
        println!("k={}, amp={:.6}", k, amp);
    }
    Ok(())
}
