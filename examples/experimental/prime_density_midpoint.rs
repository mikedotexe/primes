use std::cmp::{max, min};
use std::env;
use std::f64;
use std::fs::File;
use std::io::{BufWriter, Write};

// -------------------------- parsing --------------------------
fn parse_csv_u64(s: &str) -> Vec<u64> {
    s.split(',')
        .filter(|t| !t.trim().is_empty())
        .map(|t| t.trim().parse::<u64>().expect("parse u64"))
        .collect()
}
fn get_arg(flag: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
        if args[i].starts_with(&(flag.to_string() + "=")) {
            let v = args[i].splitn(2, '=').nth(1).unwrap().to_string();
            return Some(v);
        }
    }
    None
}

// -------------------------- small primes --------------------------
fn sieve_upto(n: u64) -> Vec<u64> {
    if n < 2 { return vec![]; }
    let nn = n as usize;
    let mut is_prime = vec![true; nn + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut p = 2usize;
    while p * p <= nn {
        if is_prime[p] {
            let mut m = p * p;
            while m <= nn {
                is_prime[m] = false;
                m += p;
            }
        }
        p += 1;
    }
    let mut res = Vec::new();
    for i in 2..=nn {
        if is_prime[i] {
            res.push(i as u64);
        }
    }
    res
}

// -------------------------- segmented sieve count --------------------------
fn count_primes_interval(l: u64, r: u64, small: &Vec<u64>) -> u64 {
    if r < 2 || l > r { return 0; }
    let ll = max(l, 2);
    let len = (r - ll + 1) as usize;
    let mut seg = vec![true; len];

    for &p in small.iter() {
        let pp = p * p;
        if pp > r { break; }
        let mut start = if ll % p == 0 { ll } else { ll + (p - (ll % p)) };
        if start == p { start += p; }
        let mut x = start;
        while x <= r {
            let idx = (x - ll) as usize;
            seg[idx] = false;
            match x.checked_add(p) {
                Some(v) => x = v,
                None => break,
            }
        }
    }
    seg.into_iter().filter(|&b| b).count() as u64
}

// -------------------------- math helpers --------------------------
fn ln_u64(x: u64) -> f64 {
    (x as f64).ln()
}
fn mean(xs: &Vec<f64>) -> f64 {
    if xs.is_empty() { return 0.0; }
    xs.iter().sum::<f64>() / (xs.len() as f64)
}
fn std_sample(xs: &Vec<f64>) -> f64 {
    let n = xs.len();
    if n <= 1 { return 0.0; }
    let m = mean(xs);
    let var = xs.iter().map(|&v| (v - m)*(v - m)).sum::<f64>() / ((n - 1) as f64);
    var.sqrt()
}
fn corr(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    let n = x.len();
    if n == 0 || n != y.len() { return 0.0; }
    let mx = mean(x); let my = mean(y);
    let sx = std_sample(x); let sy = std_sample(y);
    if sx == 0.0 || sy == 0.0 { return 0.0; }
    let mut c = 0.0;
    for i in 0..n { c += (x[i]-mx)*(y[i]-my); }
    c / ((n as f64 - 1.0) * sx * sy)
}
fn regress_slope(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    let n = x.len();
    if n == 0 || n != y.len() { return 0.0; }
    let mx = mean(x); let my = mean(y);
    let mut sxx = 0.0; let mut sxy = 0.0;
    for i in 0..n {
        let dx = x[i] - mx;
        sxx += dx*dx;
        sxy += dx*(y[i]-my);
    }
    if sxx == 0.0 { 0.0 } else { sxy / sxx }
}

// -------------------------- search w* --------------------------
fn find_min_w_for_target(m: u64, target: u64, small: &Vec<u64>, max_w_cap: u64) -> (u64, u64) {
    if target == 0 { return (0, 0); }
    let ln_m = ln_u64(m).max(1.0);
    let mut w = ((target as f64 * 0.5 * ln_m).round() as u64).max(10);
    let mut count = count_primes_interval(m.saturating_sub(w), m.saturating_add(w), small);

    if count < target {
        let mut step = w;
        while count < target {
            w = match w.checked_mul(2) { Some(v) => v, None => max_w_cap };
            if w > max_w_cap { break; }
            count = count_primes_interval(m.saturating_sub(w), m.saturating_add(w), small);
            step = step.saturating_mul(2);
            if step > max_w_cap { break; }
        }
        if count < target || w > max_w_cap {
            return (w.min(max_w_cap), count);
        }
        let mut lo = w / 2;
        let mut hi = w;
        while hi > lo {
            let mid = lo + (hi - lo) / 2;
            let c = count_primes_interval(m.saturating_sub(mid), m.saturating_add(mid), small);
            if c >= target {
                hi = mid;
                count = c;
            } else {
                lo = mid + 1;
            }
        }
        w = hi;
    } else {
        let mut lo = 0u64;
        let mut hi = w;
        while hi > lo {
            let mid = lo + (hi - lo) / 2;
            let c = count_primes_interval(m.saturating_sub(mid), m.saturating_add(mid), small);
            if c >= target {
                hi = mid;
                count = c;
            } else {
                lo = mid + 1;
            }
        }
        w = hi;
    }
    (w, count)
}

// -------------------------- main analysis --------------------------
#[derive(Clone)]
struct Row {
    base: u64,
    k: u64,
    low: u64,
    high: u64,
    mid: u64,
    ln_mid: f64,
    target: u64,
    w_min: u64,
    two_w: u64,
    prime_count: u64,
    density: f64,
    theory_density: f64,
    w_pred: f64,
    ratio_w_over_pred: f64,
    zscore: f64, // (count - E)/sqrt(E) with E≈2w/ln mid
}

fn pow_u64(mut a: u64, mut e: u64, cap: u64) -> u64 {
    let mut r: u64 = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = r.saturating_mul(a);
            if r > cap { return cap; }
        }
        if e > 1 {
            a = a.saturating_mul(a);
            if a > cap { a = cap; }
        }
        e >>= 1;
    }
    r.min(cap)
}

fn main() {
    let bases: Vec<u64> = parse_csv_u64(&get_arg("--bases").unwrap_or_else(|| "10".to_string()));
    let targets: Vec<u64> = parse_csv_u64(&get_arg("--targets").unwrap_or_else(|| "16,32".to_string()));
    let limit: u64 = get_arg("--limit").unwrap_or_else(|| "200000000".to_string()).parse().expect("limit u64");
    let max_w_cap: u64 = limit / 2;

    println!("MIDPOINT PRIME DENSITY (symmetric windows)");
    println!("bases={:?} targets={:?} limit={}", bases, targets, limit);

    let small_limit = (limit as f64).sqrt().ceil() as u64 + 100;
    let small_primes = sieve_upto(small_limit);
    println!("small primes up to {}: {}", small_limit, small_primes.len());

    let mut rows: Vec<Row> = Vec::new();

    for &b in &bases {
        if b < 2 { continue; }
        let ln_b = (b as f64).ln().max(1e-12);
        let mut k: u64 = 1;
        loop {
            let low = pow_u64(b, k - 1, limit);
            let high_full = pow_u64(b, k, limit);
            if low >= limit { break; }
            let high = high_full.saturating_sub(1).min(limit);
            if high < 2 || high <= low { break; }
            let mid = (low / 2).saturating_add(high / 2) + ((low & 1) & (high & 1)); // floor((low+high)/2) safely
            if mid < 3 { k += 1; continue; }

            let ln_mid = ln_u64(mid);
            let theory_density = 1.0 / ln_mid;

            for &t in &targets {
                let (w_min, c) = find_min_w_for_target(mid, t, &small_primes, max_w_cap);
                let two_w = w_min.saturating_mul(2);
                let density = if two_w > 0 { (c as f64) / (two_w as f64) } else { 0.0 };
                let w_pred = 0.5 * (t as f64) * ln_mid;
                let ratio_w_over_pred = if w_pred > 0.0 { (w_min as f64) / w_pred } else { 0.0 };
                let expect = (two_w as f64) * theory_density;
                let zscore = if expect > 1e-9 { ((c as f64) - expect) / expect.sqrt() } else { 0.0 };

                rows.push(Row {
                    base: b,
                    k,
                    low,
                    high,
                    mid,
                    ln_mid,
                    target: t,
                    w_min,
                    two_w,
                    prime_count: c,
                    density,
                    theory_density,
                    w_pred,
                    ratio_w_over_pred,
                    zscore,
                });
            }
            k += 1;
        }

        // summary per base & per target
        for &t in &targets {
            let rbt: Vec<&Row> = rows.iter().filter(|r| r.base == b && r.target == t).collect();
            if rbt.len() >= 2 {
                let wmins: Vec<f64> = rbt.iter().map(|r| r.w_min as f64).collect();
                let _ks: Vec<f64> = rbt.iter().map(|r| r.k as f64).collect();
                let lnms: Vec<f64> = rbt.iter().map(|r| r.ln_mid).collect();

                let mut deltas = Vec::new();
                for i in 1..rbt.len() {
                    deltas.push((rbt[i].w_min as f64) - (rbt[i-1].w_min as f64));
                }
                let mean_delta = mean(&deltas);
                let std_delta = std_sample(&deltas);
                let pred_per_digit = 0.5 * (t as f64) * ln_b;
                let slope_vs_ln = regress_slope(&lnms, &wmins);
                let corr_ln = corr(&lnms, &wmins);
                let avg_ratio = mean(&rbt.iter().map(|r| r.ratio_w_over_pred).collect());

                println!(
                    "base={} T={}  Δw/digit ≈ {:.3} ± {:.3}  | pred {:.3}  | slope(d w / d ln m)≈{:.3}  corr≈{:.3}  mean(w/pred)≈{:.3} (n={})",
                    b, t, mean_delta, std_delta, pred_per_digit, slope_vs_ln, corr_ln, avg_ratio, rbt.len()
                );
            } else {
                println!("base={} T={}  (insufficient digit blocks for summary)", b, t);
            }
        }
    }

    // CSV
    let mut w = BufWriter::new(File::create("midpoint_density_results.csv").expect("create csv"));
    writeln!(
        w,
        "base,k,low,high,mid,ln_mid,target,w_min,window,prime_count,density,theory_density,w_pred,ratio_w_over_pred,zscore"
    ).unwrap();
    for r in &rows {
        writeln!(
            w,
            "{},{},{},{},{},{:.9},{},{},{},{},{:.9},{:.9},{:.9},{:.6},{:.6}",
            r.base, r.k, r.low, r.high, r.mid, r.ln_mid, r.target, r.w_min, r.two_w, r.prime_count,
            r.density, r.theory_density, r.w_pred, r.ratio_w_over_pred, r.zscore
        ).unwrap();
    }
    w.flush().unwrap();
    println!("CSV written: midpoint_density_results.csv");
}
