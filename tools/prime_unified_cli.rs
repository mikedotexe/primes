use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;
use std::f64;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;

// --------------------------- CLI ---------------------------
fn arg(flag: &str, default: &str) -> String {
    let a: Vec<String> = env::args().collect();
    for i in 0..a.len() {
        if a[i] == flag && i + 1 < a.len() { return a[i + 1].clone(); }
        let key = format!("{}=", flag);
        if a[i].starts_with(&key) { return a[i][key.len()..].to_string(); }
    }
    default.to_string()
}
fn parse_csv_u64(s: &str) -> Vec<u64> {
    s.split(',').filter(|t| !t.trim().is_empty()).map(|t| t.trim().parse::<u64>().expect("parse u64")).collect()
}
fn parse_csv_u32(s: &str) -> Vec<u32> {
    s.split(',').filter(|t| !t.trim().is_empty()).map(|t| t.trim().parse::<u32>().expect("parse u32")).collect()
}
fn parse_bool(s: &str) -> bool {
    matches!(s.to_lowercase().as_str(), "1"|"true"|"yes"|"y")
}

// --------------------------- math utils ---------------------------
fn ln_u64(x: u64) -> f64 { (x as f64).ln() }
fn mean(xs: &[f64]) -> f64 { if xs.is_empty() { 0.0 } else { xs.iter().sum::<f64>() / xs.len() as f64 } }
fn std_sample(xs: &[f64]) -> f64 {
    let n = xs.len(); if n <= 1 { return 0.0; }
    let m = mean(xs);
    let s = xs.iter().map(|&v| (v - m)*(v - m)).sum::<f64>() / ((n - 1) as f64);
    s.sqrt()
}
fn corr(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len(); if n == 0 || n != y.len() { return 0.0; }
    let mx = mean(x); let my = mean(y);
    let sx = std_sample(x); let sy = std_sample(y);
    if sx == 0.0 || sy == 0.0 { return 0.0; }
    let mut c = 0.0; for i in 0..n { c += (x[i]-mx)*(y[i]-my); }
    c / ((n as f64 - 1.0) * sx * sy)
}
fn regress_slope(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len(); if n == 0 || n != y.len() { return 0.0; }
    let mx = mean(x); let my = mean(y);
    let mut sxx = 0.0; let mut sxy = 0.0;
    for i in 0..n { let dx = x[i]-mx; sxx += dx*dx; sxy += dx*(y[i]-my); }
    if sxx == 0.0 { 0.0 } else { sxy/sxx }
}
fn gcd_u64(mut a: u64, mut b: u64) -> u64 { while b != 0 { let t=a%b; a=b; b=t; } a }

// Extended GCD for modular inverse
fn egcd_i64(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (g, x1, y1) = egcd_i64(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

// Modular inverse: returns Some(inv) where a*inv ≡ 1 (mod n), or None if gcd(a,n) ≠ 1
fn inv_mod(a: u64, n: u64) -> Option<u64> {
    let a_signed = (a % n) as i64;
    let n_signed = n as i64;
    let (g, x, _) = egcd_i64(a_signed, n_signed);
    if g != 1 { return None; }
    Some(((x % n_signed + n_signed) % n_signed) as u64)
}

// Prime factorization
fn prime_factors(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        let mut count = 0u32;
        while n % d == 0 {
            count += 1;
            n /= d;
        }
        if count > 0 { out.push((d, count)); }
        d += if d == 2 { 1 } else { 2 };
    }
    if n > 1 { out.push((n, 1)); }
    out
}

// Check if fraction num/den terminates in given base
// Terminates iff prime factors of den are subset of prime factors of base
#[allow(dead_code)]
fn terminates_in_base(_num: u64, den: u64, base: u64) -> bool {
    if den == 1 { return true; }
    let pf_den: std::collections::HashSet<u64> = prime_factors(den).into_iter().map(|(p,_)| p).collect();
    let pf_base: std::collections::HashSet<u64> = prime_factors(base).into_iter().map(|(p,_)| p).collect();
    pf_den.is_subset(&pf_base)
}
fn pow_u64(mut a: u64, mut e: u64, cap: u64) -> u64 {
    let mut r=1u64;
    while e>0 {
        if e&1==1 { r = r.saturating_mul(a); if r>cap { return cap; } }
        if e>1 { a = a.saturating_mul(a); if a>cap { a=cap; } }
        e >>= 1;
    }
    r.min(cap)
}

// --------------------------- inverse normal CDF (Acklam) ---------------------------
fn inv_norm_cdf(p: f64) -> f64 {
    let a = [-3.969683028665376e+01, 2.209460984245205e+02,-2.759285104469687e+02,1.383577518672690e+02,-3.066479806614716e+01,2.506628277459239e+00];
    let b = [-5.447609879822406e+01, 1.615858368580409e+02,-1.556989798598866e+02,6.680131188771972e+01,-1.328068155288572e+01];
    let c = [-7.784894002430293e-03,-3.223964580411365e-01,-2.400758277161838e+00,-2.549732539343734e+00, 4.374664141464968e+00, 2.938163982698783e+00];
    let d = [ 7.784695709041462e-03, 3.224671290700398e-01, 2.445134137142996e+00, 3.754408661907416e+00];
    let plow = 0.02425; let phigh = 1.0 - plow;
    if !(p>0.0 && p<1.0) { return if p<=0.0 { f64::NEG_INFINITY } else { f64::INFINITY }; }
    if p < plow {
        let q = (-2.0*p.ln()).sqrt();
        (((((c[0]*q+c[1])*q+c[2])*q+c[3])*q+c[4])*q+c[5]) /
        ((((d[0]*q+d[1])*q+d[2])*q+d[3])*q+1.0)
    } else if p > phigh {
        let q = (-2.0*(1.0-p).ln()).sqrt();
        -(((((c[0]*q+c[1])*q+c[2])*q+c[3])*q+c[4])*q+c[5]) /
        ((((d[0]*q+d[1])*q+d[2])*q+d[3])*q+1.0)
    } else {
        let q = p - 0.5; let r = q*q;
        (((((a[0]*r+a[1])*r+a[2])*r+a[3])*r+a[4])*r+a[5])*q /
        (((((b[0]*r+b[1])*r+b[2])*r+b[3])*r+b[4])*r+1.0)
    }
}

// --------------------------- primes: sieve + segmented ---------------------------
fn sieve_upto(n: u64) -> Vec<u64> {
    if n < 2 { return vec![]; }
    let nn = n as usize;
    let mut is_prime = vec![true; nn+1];
    is_prime[0]=false; is_prime[1]=false;
    let mut p=2usize;
    while p*p<=nn {
        if is_prime[p] {
            let mut m = p*p;
            while m<=nn { is_prime[m]=false; m+=p; }
        }
        p+=1;
    }
    let mut out = Vec::new();
    for i in 2..=nn { if is_prime[i] { out.push(i as u64); } }
    out
}
fn segmented_mark(l: u64, r: u64, small: &Vec<u64>) -> Vec<bool> {
    if r < l { return vec![]; }
    let ll = max(l, 2);
    let len = (r - ll + 1) as usize;
    let mut seg = vec![true; len];
    for &p in small.iter() {
        let pp = p*p; if pp > r { break; }
        let mut start = if ll%p==0 { ll } else { ll + (p - (ll%p)) };
        if start < pp { start = pp; }
        let mut x = start;
        while x <= r {
            seg[(x-ll) as usize] = false;
            match x.checked_add(p) { Some(v)=>x=v, None=>break }
        }
    }
    if l < 2 {
        for n in l..min(r,1) {
            let idx = (n - ll) as i64;
            if idx >= 0 { seg[idx as usize] = false; }
        }
    }
    seg
}
fn count_primes_interval(l: u64, r: u64, small: &Vec<u64>) -> u64 {
    if r < 2 || l > r { return 0; }
    let ll = max(l,2);
    let seg = segmented_mark(ll, r, small);
    seg.into_iter().filter(|&b| b).count() as u64
}
fn residue_counts(l:u64, r:u64, q:u64)->Vec<u64> {
    let mut out = vec![0u64; q as usize];
    if l > r { return out; }
    for a in 0..q {
        let first = if l%q <= a { l - (l%q) + a } else { l - (l%q) + q + a };
        if first > r { out[a as usize] = 0; } else { out[a as usize] = 1 + (r - first) / q; }
    }
    out
}
fn count_primes_by_mod(l: u64, r: u64, q: u64, small: &Vec<u64>) -> (u64, Vec<u64>) {
    if r < 2 || l > r { return (0, vec![0; q as usize]); }
    let ll = max(l,2);
    let seg = segmented_mark(ll, r, small);
    let mut cnt=0u64; let mut by = vec![0u64; q as usize];
    for (i, &is_p) in seg.iter().enumerate() {
        if is_p { let n = ll + i as u64; cnt += 1; by[(n%q) as usize] += 1; }
    }
    (cnt, by)
}

// --------------------------- CCRT (Complementary CRT zero pairs) ---------------------------
struct CCRTRow {
    base: u64,
    pattern: String,
    honorary_zero: u64,
    zeros: Vec<u64>,
    zero_count: u64,
    n_evens: u64,
    covered: u64,
    total_pairs: u64,
    coverage_rate: f64,
    avg_pairs: f64,
}
fn gcd(a: u64, b: u64) -> u64 { gcd_u64(a,b) }
fn lcm(xs:&[u64]) -> u64 {
    let mut g=1u64;
    for &v in xs { g = g / gcd(g, v) * v; }
    g
}
fn generate_bases_for_zero_set(zeros: &[u64], small:&[u64], min_b:u64, max_b:u64) -> Vec<u64> {
    if zeros.is_empty() { return vec![]; }
    let zprod = lcm(zeros);
    let others: Vec<u64> = small.iter().cloned().filter(|q| !zeros.contains(q)).collect();
    let mut out = Vec::new();
    let mut k=1u64;
    loop {
        let hz = zprod*k;
        let b = 2*hz;
        if b > max_b { break; }
        if b >= min_b && others.iter().all(|&q| hz%q != 0) { out.push(b); }
        k+=1;
    }
    out
}
fn ccrt_metrics(base:u64, pattern:&str, primes:&Vec<u64>, is_prime:&Vec<bool>, window:u64, require_p_prime:bool, small_set:&[u64]) -> Option<CCRTRow> {
    let hz = base/2;
    if require_p_prime {
        if hz as usize >= is_prime.len() || !is_prime[hz as usize] { return None; }
    }
    let mut zeros=Vec::new();
    for &m in small_set { if hz % m == 0 { zeros.push(m); } }
    let start = 2*base;
    let max_needed = (2*base + window + 10).min((is_prime.len()-1) as u64);
    if start >= max_needed { return None; }
    let end = (2*base + window).min(max_needed);
    let mut evens = Vec::new();
    let mut n = if start%2==0 { start } else { start+1 };
    while n <= end { evens.push(n); n += 2; }
    if evens.is_empty() { return None; }
    let max_n = *evens.last().unwrap();
    let mut left = Vec::new();
    for &p in primes { if p > max_n/2 { break; } left.push(p); }

    let mut covered=0u64; let mut total_pairs=0u64;
    for &even_n in &evens {
        let mut cnt=0u64;
        for &p in &left {
            let q = even_n - p;
            if q < p { break; }
            if (q as usize) < is_prime.len() && is_prime[q as usize] { cnt += 1; }
        }
        if cnt > 0 { covered += 1; }
        total_pairs += cnt;
    }
    let n_evens = evens.len() as u64;
    let cov = if n_evens>0 { covered as f64 / n_evens as f64 } else { 0.0 };
    let avg = if n_evens>0 { total_pairs as f64 / n_evens as f64 } else { 0.0 };
    let zero_count = zeros.len() as u64;
    Some(CCRTRow{
        base, pattern: pattern.to_string(), honorary_zero: hz, zeros, zero_count,
        n_evens, covered, total_pairs, coverage_rate: cov, avg_pairs: avg
    })
}

// --------------------------- Midpoint Density (quantile + wheel) ---------------------------
struct MDRRow {
    base: u64, k: u64, low: u64, high: u64, mid: u64, ln_mid: f64, target: u64,
    w_pred_plain: f64, w_pred_tau: f64, tau: f64, ztau: f64,
    q: u64, f_q_at_wpred: f64, w_pred_wheel: f64,
    w_min: u64, prime_count_min: u64,
    count_at_wpred: u64, expect_pnt_at_wpred: f64, expect_wheel_at_wpred: f64,
    ratio_w_over_pred: f64, ratio_w_over_wheel: f64,
    chi2_int_res: f64, chi2_prime_res: f64,
}
fn count_multiples(l:u64, r:u64, d:u64)->u64 {
    if r<d { return 0; }
    if l==0 { return r/d + 1; }
    r/d - (l-1)/d
}
fn count_coprime_interval(l:u64, r:u64, ps:&[u64]) -> u64 {
    if l>r { return 0; }
    let total = r - l + 1;
    let m = ps.len();
    let mut bad = 0u64;
    for mask in 1u64..(1u64<<m) {
        let mut mult=1u64; let mut bits=0; let mut ok=true;
        for i in 0..m {
            if (mask>>i)&1==1 {
                bits += 1;
                match mult.checked_mul(ps[i]) { Some(v)=>mult=v, None=>{ ok=false; break; } }
            }
        }
        if !ok || mult==0 { continue; }
        let c = count_multiples(l, r, mult);
        if bits%2==1 { bad = bad.saturating_add(c); } else { bad = bad.saturating_sub(c); }
    }
    total.saturating_sub(bad)
}
fn find_min_w_for_target(m:u64, target:u64, small:&Vec<u64>, cap:u64)->(u64,u64){
    if target==0 { return (0,0); }
    let ln_m = ln_u64(m).max(1.0);
    let mut w = ((target as f64 * 0.5 * ln_m).round() as u64).max(10);
    let mut cnt = count_primes_interval(m.saturating_sub(w), m.saturating_add(w), small);
    if cnt < target {
        while cnt < target {
            w = match w.checked_mul(2) { Some(v)=>v, None=>cap };
            if w > cap { break; }
            cnt = count_primes_interval(m.saturating_sub(w), m.saturating_add(w), small);
        }
        if cnt < target || w > cap { return (w.min(cap), cnt); }
        let mut lo = w/2; let mut hi = w;
        while hi > lo {
            let mid = lo + (hi-lo)/2;
            let c = count_primes_interval(m.saturating_sub(mid), m.saturating_add(mid), small);
            if c >= target { hi = mid; cnt = c; } else { lo = mid + 1; }
        }
        (hi, cnt)
    } else {
        let mut lo=0u64; let mut hi=w;
        while hi>lo {
            let mid = lo + (hi-lo)/2;
            let c = count_primes_interval(m.saturating_sub(mid), m.saturating_add(mid), small);
            if c >= target { hi = mid; cnt = c; } else { lo = mid + 1; }
        }
        (hi, cnt)
    }
}

// --------------------------- Summary helpers ---------------------------
fn welch_t(x:&[f64], y:&[f64])->(f64,f64){
    let nx=x.len() as f64; let ny=y.len() as f64;
    if nx<2.0 || ny<2.0 { return (0.0,0.0); }
    let mx=mean(x); let my=mean(y);
    let sx=std_sample(x); let sy=std_sample(y);
    let vx=sx*sx; let vy=sy*sy;
    let denom = vx/nx + vy/ny; if denom==0.0 { return (0.0,0.0); }
    let t = (mx-my)/denom.sqrt();
    let dx=vx/nx; let dy=vy/ny;
    let num = (dx+dy)*(dx+dy);
    let den = (dx*dx)/(nx-1.0) + (dy*dy)/(ny-1.0);
    let dof = if den>0.0 { num/den } else { 0.0 };
    (t,dof)
}

// --------------------------- N× TRANSFORM ---------------------------
#[derive(Debug, Clone)]
struct NTransformDetail {
    base: u64,
    n_val: u64,
    r: u64,
    gcd_bn: u64,
    k0_residue: u64,
    k1_residue: u64,
    k2_residue: u64,
    integer_vertex_k: Option<u64>,
    distinct_residue_count: u64,
}

#[derive(Debug, Clone)]
struct NTransformSummary {
    base: u64,
    n_val: u64,
    modulo: u64,
    gcd_bn: u64,
    integer_k_entropy_bits: f64,
    integer_k_support: u64,
    integer_k_uniformity: bool,
    n3_trio_universal: bool,
}

// Compute residues (r + kB) % N for k=0..N-1
fn residues_after_transform(b: u64, n_val: u64, r: u64) -> Vec<u64> {
    let a = r % n_val;
    let b_mod = b % n_val;
    (0..n_val).map(|k| (a + k * b_mod) % n_val).collect()
}

// Find k in [0..N-1] where (r + kB) ≡ 0 (mod N)
fn vertex_integer_k(b: u64, n_val: u64, r: u64) -> Option<u64> {
    let g = gcd_u64(b % n_val, n_val);
    if r % g != 0 { return None; }
    if g == n_val { return Some(0); }

    let b_reduced = (b / g) % (n_val / g);
    let r_reduced = (r / g) % (n_val / g);
    let n_reduced = n_val / g;

    let inv = inv_mod(b_reduced, n_reduced)?;
    let k0 = ((n_reduced - r_reduced) * inv) % n_reduced;
    Some(k0)
}

// Analyze single (B, N, r) combination
fn analyze_ntransform_single(b: u64, n_val: u64, r: u64) -> NTransformDetail {
    let gcd_bn = gcd_u64(b, n_val);
    let res = residues_after_transform(b, n_val, r);
    let integer_k = vertex_integer_k(b, n_val, r);
    let distinct = res.iter().collect::<HashSet<_>>().len() as u64;

    // For N=3, extract k=0,1,2 residues
    let (k0, k1, k2) = if res.len() >= 3 { (res[0], res[1], res[2]) } else { (0, 0, 0) };

    NTransformDetail {
        base: b,
        n_val,
        r,
        gcd_bn,
        k0_residue: k0,
        k1_residue: k1,
        k2_residue: k2,
        integer_vertex_k: integer_k,
        distinct_residue_count: distinct,
    }
}

// Analyze all r in [0..modulo) for given base
fn analyze_ntransform_sweep(b: u64, n_val: u64, modulo: u64) -> NTransformSummary {
    let gcd_bn = gcd_u64(b, n_val);
    let details: Vec<NTransformDetail> = (0..modulo)
        .map(|r| analyze_ntransform_single(b, n_val, r))
        .collect();

    // Check N=3 universal trio property: 3∤B ⇒ every r yields {0,1,2}
    let trio_universal = if n_val == 3 && b % 3 != 0 {
        details.iter().all(|d| {
            let mut res_set = vec![d.k0_residue, d.k1_residue, d.k2_residue];
            res_set.sort();
            res_set == vec![0, 1, 2]
        })
    } else {
        false
    };

    // Compute k_int distribution
    let ks: Vec<u64> = details.iter()
        .filter_map(|d| d.integer_vertex_k)
        .collect();

    let mut hist: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    for k in &ks {
        *hist.entry(*k).or_insert(0) += 1;
    }

    let support = hist.len() as u64;
    let uniformity = if support > 0 {
        let counts: Vec<usize> = hist.values().copied().collect();
        counts.iter().min() == counts.iter().max()
    } else {
        false
    };

    // Entropy
    let total = ks.len() as f64;
    let entropy = if total > 0.0 {
        hist.values().map(|&count| {
            let p = count as f64 / total;
            if p > 0.0 { -p * p.log2() } else { 0.0 }
        }).sum()
    } else {
        0.0
    };

    NTransformSummary {
        base: b,
        n_val,
        modulo,
        gcd_bn,
        integer_k_entropy_bits: entropy,
        integer_k_support: support,
        integer_k_uniformity: uniformity,
        n3_trio_universal: trio_universal,
    }
}

// --------------------------- MAIN ---------------------------
fn main() {
    // ---- Args
    let out_dir = arg("--out-dir", "/mnt/user-data/outputs");
    let run = arg("--run", "all"); // all|ccrt|mdr|ntransform
    // CCRT args
    let ccrt_min_base: u64 = arg("--ccrt-min-base","10").parse().unwrap();
    let ccrt_max_base: u64 = arg("--ccrt-max-base","500").parse().unwrap();
    let ccrt_window:   u64 = arg("--ccrt-window","400").parse().unwrap();
    let ccrt_require_p_prime = parse_bool(&arg("--require-p-prime","0"));
    // MDR args
    let mdr_bases: Vec<u64> = parse_csv_u64(&arg("--mdr-bases","6,10,30"));
    let mdr_targets: Vec<u64> = parse_csv_u64(&arg("--mdr-targets","8,16,32"));
    let mdr_limit: u64 = arg("--mdr-limit","200000000").parse().unwrap();
    let q_mods: Vec<u32> = parse_csv_u32(&arg("--q","30,210"));
    let tau: f64 = arg("--tau","0.80").parse::<f64>().unwrap_or(0.80).clamp(0.50,0.99);
    let small_mods: [u64;4] = [3,5,7,11];
    // N× transform args
    let ntransform_bases: Vec<u64> = parse_csv_u64(&arg("--ntransform-bases","106,998"));
    let ntransform_n: u64 = arg("--ntransform-N","3").parse().unwrap();
    let ntransform_detail = parse_bool(&arg("--ntransform-detail","0"));

    // ---- prep
    create_dir_all(&out_dir).ok();
    // CCRT sieve bound exactly mirrors previous scripts
    let ccrt_max_needed = (2*ccrt_max_base + ccrt_window + 10) as u64;
    let ccrt_primes_vec = sieve_upto(ccrt_max_needed);
    let mut ccrt_is_prime = vec![false; (ccrt_max_needed as usize)+1];
    for &p in &ccrt_primes_vec { ccrt_is_prime[p as usize] = true; }

    // MDR small primes for segmentation
    let small_limit = (mdr_limit as f64).sqrt().ceil() as u64 + 100;
    let small_primes = sieve_upto(small_limit);

    // Outputs
    let ccrt_csv = Path::new(&out_dir).join("ccrt_results.csv");
    let mdr_csv  = Path::new(&out_dir).join("mdr_results.csv");
    let ntrans_detail_csv = Path::new(&out_dir).join("ntransform_detail.csv");
    let ntrans_summary_csv = Path::new(&out_dir).join("ntransform_summary.csv");
    let mut summary_lines: Vec<String> = Vec::new();

    // ==================== CCRT ====================
    if run=="all" || run=="ccrt" {
        let mut rows: Vec<CCRTRow> = Vec::new();
        // patterns
        let patterns: [(&str, Vec<u64>);14] = [
            ("only_3",vec![3]),("only_5",vec![5]),("only_7",vec![7]),("only_11",vec![11]),
            ("3_and_5",vec![3,5]),("3_and_7",vec![3,7]),("3_and_11",vec![3,11]),("5_and_7",vec![5,7]),
            ("5_and_11",vec![5,11]),("7_and_11",vec![7,11]),
            ("3_5_7",vec![3,5,7]),("3_5_11",vec![3,5,11]),("3_7_11",vec![3,7,11]),("5_7_11",vec![5,7,11]),
        ];
        for (name, zs) in patterns {
            let bases = generate_bases_for_zero_set(&zs, &small_mods, ccrt_min_base, ccrt_max_base);
            for b in bases {
                if let Some(r) = ccrt_metrics(b, name, &ccrt_primes_vec, &ccrt_is_prime, ccrt_window, ccrt_require_p_prime, &small_mods) {
                    rows.push(r);
                }
            }
        }
        // sort by coverage then avg_pairs
        rows.sort_by(|a,b| b.coverage_rate.partial_cmp(&a.coverage_rate).unwrap()
            .then(b.avg_pairs.partial_cmp(&a.avg_pairs).unwrap()));
        // write csv
        let mut w = BufWriter::new(File::create(&ccrt_csv).expect("create ccrt csv"));
        writeln!(w, "base,pattern,honorary_zero,zeros,zero_count,n_evens,covered,total_pairs,coverage_rate,avg_pairs").unwrap();
        for r in &rows {
            let zeros_str = {
                let mut s = String::from("[");
                for (i,z) in r.zeros.iter().enumerate() { if i>0 { s.push_str(", "); } s.push_str(&z.to_string()); }
                s.push(']'); s
            };
            writeln!(w, "{},{},{},{},{},{},{},{},{:.6},{:.6}", r.base, r.pattern, r.honorary_zero, zeros_str, r.zero_count, r.n_evens, r.covered, r.total_pairs, r.coverage_rate, r.avg_pairs).unwrap();
        }
        w.flush().unwrap();

        // quick summary (complementary vs single)
        let mut singles: Vec<f64> = Vec::new();
        let mut pairs:   Vec<f64> = Vec::new();
        let mut triples: Vec<f64> = Vec::new();
        let mut comp:    Vec<f64> = Vec::new();
        let mut noncomp: Vec<f64> = Vec::new();
        for r in &rows {
            match r.zero_count {
                1 => singles.push(r.coverage_rate),
                2 => {
                    pairs.push(r.coverage_rate);
                    if r.pattern=="3_and_11" || r.pattern=="5_and_7" { comp.push(r.coverage_rate); }
                    else { noncomp.push(r.coverage_rate); }
                },
                _ => triples.push(r.coverage_rate),
            }
        }
        let (t_cs, dof_cs) = welch_t(&comp, &singles);
        let (t_cn, dof_cn) = welch_t(&comp, &noncomp);
        summary_lines.push(format!("CCRT: n={}  comp_mean={:.4}  singles_mean={:.4}  noncomp_mean={:.4}  t(comp vs single)={:.3} dof≈{:.1}  t(comp vs noncomp)={:.3} dof≈{:.1}",
            rows.len(),
            if comp.is_empty(){f64::NAN}else{mean(&comp)},
            if singles.is_empty(){f64::NAN}else{mean(&singles)},
            if noncomp.is_empty(){f64::NAN}else{mean(&noncomp)},
            t_cs, dof_cs, t_cn, dof_cn
        ));
    }

    // ==================== Midpoint Density ====================
    if run=="all" || run=="mdr" {
        let mut rows: Vec<MDRRow> = Vec::new();
        for &b in &mdr_bases {
            if b < 2 { continue; }
            let mut k=1u64;
            loop {
                let low  = pow_u64(b, k-1, mdr_limit);
                let highf= pow_u64(b, k,   mdr_limit);
                if low >= mdr_limit { break; }
                let high = highf.saturating_sub(1).min(mdr_limit);
                if high <= low || high < 3 { break; }
                let mid = (low/2).saturating_add(high/2) + ((low&1) & (high&1));
                let ln_mid = ln_u64(mid);
                if ln_mid <= 0.0 { k += 1; continue; }

                for &t in &mdr_targets {
                    let w_pred_plain = 0.5 * (t as f64) * ln_mid;
                    let z = inv_norm_cdf(tau);
                    // Solve mu + z*sqrt(mu) = T -> s = (-z + sqrt(z^2 + 4T))/2
                    let s = (-z + (z*z + 4.0*(t as f64)).sqrt())/2.0;
                    let mu = s*s;
                    let w_pred_tau = 0.5 * mu * ln_mid;

                    // pick best wheel q from provided list
                    let mut best_q = 1u64; let mut best_fq = 1.0f64; let mut best_wheel = w_pred_plain;
                    for &q32 in &q_mods {
                        let q = q32 as u64;
                        let w0 = w_pred_plain.max(10.0).round() as u64;
                        let l0 = mid.saturating_sub(w0);
                        let r0 = mid.saturating_add(w0);
                        // primes dividing q (subset of small primes list)
                        let mut ps: Vec<u64> = Vec::new();
                        for &pp in &[2u64,3,5,7,11,13,17,19] {
                            if q % pp == 0 { ps.push(pp); }
                        }
                        let cop = count_coprime_interval(l0, r0, &ps) as f64;
                        let len = (r0 - l0 + 1) as f64;
                        let fq = if len>0.0 { cop/len } else { 1.0 };
                        let w_wheel = if fq>0.0 { w_pred_plain/fq } else { w_pred_plain*10.0 };
                        if (w_wheel - w_pred_tau).abs() < (best_wheel - w_pred_tau).abs() {
                            best_q = q; best_fq = fq; best_wheel = w_wheel;
                        }
                    }

                    // minimal w*
                    let cap = (high - low + 1)/2;
                    let (w_min, c_min) = find_min_w_for_target(mid, t, &small_primes, cap.max(100));

                    // counts and expectations at w_pred_plain
                    let w0 = w_pred_plain.max(10.0).round() as u64;
                    let l0 = mid.saturating_sub(w0);
                    let r0 = mid.saturating_add(w0);
                    let c0 = count_primes_interval(l0, r0, &small_primes);
                    let expect_pnt = (2.0*(w0 as f64))/ln_mid;
                    let expect_wheel = (2.0*(w0 as f64)*best_fq)/ln_mid;

                    // residue diagnostics at w_min
                    let l1 = mid.saturating_sub(w_min);
                    let r1 = mid.saturating_add(w_min);
                    let q = if best_q==1 { 30 } else { best_q };
                    let vec_res = residue_counts(l1, r1, q);
                    let mut totatives: Vec<usize> = Vec::new();
                    for a in 0..q { if gcd_u64(a,q)==1 { totatives.push(a as usize); } }
                    let mut int_counts: Vec<f64> = Vec::new(); let mut total_int=0u64;
                    for &a in &totatives { let c = vec_res[a]; total_int += c; int_counts.push(c as f64); }
                    let mean_int = if !int_counts.is_empty() { (total_int as f64)/(int_counts.len() as f64) } else { 0.0 };
                    let mut chi2_int = 0.0;
                    if mean_int > 0.0 { for &c in &int_counts { chi2_int += (c-mean_int)*(c-mean_int)/mean_int; } }
                    let (pcount, by_mod) = count_primes_by_mod(l1, r1, q, &small_primes);
                    let kclasses = totatives.len();
                    let expect_per = if kclasses>0 { (pcount as f64)/(kclasses as f64) } else { 0.0 };
                    let mut chi2_prime = 0.0;
                    if expect_per > 0.0 {
                        for &a in &totatives {
                            let c = by_mod[a] as f64;
                            chi2_prime += (c - expect_per)*(c - expect_per)/expect_per;
                        }
                    }

                    rows.push(MDRRow{
                        base:b, k, low, high, mid, ln_mid,
                        target:t,
                        w_pred_plain, w_pred_tau, tau, ztau:z,
                        q, f_q_at_wpred: best_fq, w_pred_wheel: best_wheel,
                        w_min, prime_count_min: c_min,
                        count_at_wpred: c0, expect_pnt_at_wpred: expect_pnt, expect_wheel_at_wpred: expect_wheel,
                        ratio_w_over_pred: (w_min as f64)/w_pred_plain.max(1e-9),
                        ratio_w_over_wheel: (w_min as f64)/best_wheel.max(1e-9),
                        chi2_int_res: chi2_int, chi2_prime_res: chi2_prime
                    });
                }
                k += 1;
                if highf >= mdr_limit { break; }
            }

            // base‑wise summaries
            for &t in &mdr_targets {
                let rbt: Vec<&MDRRow> = rows.iter().filter(|r| r.base==b && r.target==t).collect();
                if rbt.len() >= 2 {
                    let lnms: Vec<f64> = rbt.iter().map(|r| r.ln_mid).collect();
                    let wmins: Vec<f64> = rbt.iter().map(|r| r.w_min as f64).collect();
                    let slope = regress_slope(&lnms, &wmins);
                    let crr = corr(&lnms, &wmins);
                    let ratios: Vec<f64> = rbt.iter().map(|r| r.ratio_w_over_pred).collect();
                    let ratios_w: Vec<f64> = rbt.iter().map(|r| r.ratio_w_over_wheel).collect();
                    let chi_int: Vec<f64> = rbt.iter().map(|r| r.chi2_int_res).collect();
                    let chi_prime: Vec<f64> = rbt.iter().map(|r| r.chi2_prime_res).collect();
                    let corr_ratio_chi_int = corr(&ratios, &chi_int);
                    let corr_ratio_chi_prime = corr(&ratios, &chi_prime);
                    summary_lines.push(format!("MDR base={} T={}  slope≈{:.3} corr≈{:.3} mean(w/pred)={:.3} wheel={:.3} corr(ratio,χ2int)={:.3} corr(ratio,χ2prime)={:.3} n={}",
                        b, t, slope, crr, mean(&ratios), mean(&ratios_w), corr_ratio_chi_int, corr_ratio_chi_prime, rbt.len()));
                } else {
                    summary_lines.push(format!("MDR base={} T={} insufficient digit blocks", b, t));
                }
            }
        }

        // write csv
        let mut w = BufWriter::new(File::create(&mdr_csv).expect("create mdr csv"));
        writeln!(w, "base,k,low,high,mid,ln_mid,target,w_pred_plain,w_pred_tau,tau,ztau,q,f_q_at_wpred,w_pred_wheel,w_min,prime_count_min,count_at_wpred,expect_pnt_at_wpred,expect_wheel_at_wpred,ratio_w_over_pred,ratio_w_over_wheel,chi2_int_res,chi2_prime_res").unwrap();
        for r in &rows {
            writeln!(w,
                "{},{},{},{},{},{:.9},{},{:.6},{:.6},{:.4},{:.6},{},{:.9},{:.6},{},{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}",
                r.base, r.k, r.low, r.high, r.mid, r.ln_mid, r.target,
                r.w_pred_plain, r.w_pred_tau, r.tau, r.ztau,
                r.q, r.f_q_at_wpred, r.w_pred_wheel,
                r.w_min, r.prime_count_min, r.count_at_wpred,
                r.expect_pnt_at_wpred, r.expect_wheel_at_wpred,
                r.ratio_w_over_pred, r.ratio_w_over_wheel,
                r.chi2_int_res, r.chi2_prime_res
            ).unwrap();
        }
        w.flush().unwrap();
    }

    // ==================== N× TRANSFORM ====================
    if run == "ntransform" {
        let mut detail_rows: Vec<NTransformDetail> = Vec::new();
        let mut summary_rows: Vec<NTransformSummary> = Vec::new();

        for &base in &ntransform_bases {
            // Determine modulo: use p = B/2 for even bases
            let modulo = if base % 2 == 0 { base / 2 } else { base };

            // Generate summary for this base
            let summary = analyze_ntransform_sweep(base, ntransform_n, modulo);
            summary_rows.push(summary.clone());

            // Add to summary lines
            summary_lines.push(format!(
                "NTrans B={} N={} mod={}  gcd(B,N)={}  k_ent={:.3} k_sup={}  uniform={}  trio={}",
                base, ntransform_n, modulo, summary.gcd_bn,
                summary.integer_k_entropy_bits, summary.integer_k_support,
                if summary.integer_k_uniformity { "Y" } else { "N" },
                if summary.n3_trio_universal { "Y" } else { "N" }
            ));

            // Optionally generate detail rows
            if ntransform_detail {
                for r in 0..modulo {
                    detail_rows.push(analyze_ntransform_single(base, ntransform_n, r));
                }
            }
        }

        // Write detail CSV if requested
        if ntransform_detail && !detail_rows.is_empty() {
            let mut wd = BufWriter::new(File::create(&ntrans_detail_csv).expect("create ntransform detail csv"));
            writeln!(wd, "B,N,r,gcd_BN,k0_residue,k1_residue,k2_residue,integer_vertex_k,distinct_residue_count").unwrap();
            for d in &detail_rows {
                writeln!(wd, "{},{},{},{},{},{},{},{},{}",
                    d.base, d.n_val, d.r, d.gcd_bn,
                    d.k0_residue, d.k1_residue, d.k2_residue,
                    d.integer_vertex_k.map(|k| k.to_string()).unwrap_or_else(|| "".to_string()),
                    d.distinct_residue_count
                ).unwrap();
            }
            wd.flush().unwrap();
        }

        // Write summary CSV
        let mut ws = BufWriter::new(File::create(&ntrans_summary_csv).expect("create ntransform summary csv"));
        writeln!(ws, "B,N,modulo,gcd_BN,integer_k_entropy_bits,integer_k_support,integer_k_uniformity,N3_trio_universal").unwrap();
        for s in &summary_rows {
            writeln!(ws, "{},{},{},{},{:.6},{},{},{}",
                s.base, s.n_val, s.modulo, s.gcd_bn,
                s.integer_k_entropy_bits, s.integer_k_support,
                if s.integer_k_uniformity { 1 } else { 0 },
                if s.n3_trio_universal { 1 } else { 0 }
            ).unwrap();
        }
        ws.flush().unwrap();
    }

    // Write concise SUMMARY.txt
    let summary_path = Path::new(&out_dir).join("SUMMARY.txt");
    let mut ws = BufWriter::new(File::create(&summary_path).expect("create summary"));
    for line in &summary_lines { writeln!(ws, "{}", line).unwrap(); }
    ws.flush().unwrap();

    println!("Wrote:");
    if run=="all" || run=="ccrt" { println!("  {}", ccrt_csv.display()); }
    if run=="all" || run=="mdr"  { println!("  {}", mdr_csv.display()); }
    if run=="ntransform" {
        println!("  {}", ntrans_summary_csv.display());
        if ntransform_detail {
            println!("  {}", ntrans_detail_csv.display());
        }
    }
    println!("  {}", summary_path.display());
}
