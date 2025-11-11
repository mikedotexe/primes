pub mod verification;
pub mod harmonic_overtones;
pub mod harmonic_lagrange;
pub mod symmetry_breaking;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Map from (mid_len, inner_zero) to list of (modulus, probability) pairs
pub type ExplainMap = HashMap<(usize, usize), Vec<(u32, f64)>>;

#[derive(Clone, Debug)]
pub struct Pair {
    pub base: u32,
    pub mid_len: usize,
    pub inner_zero: usize,
    pub obs_density: Option<f64>,
    pub ci_lo: Option<f64>,
    pub ci_hi: Option<f64>,
    pub pred_local: Option<f64>,
    pub pred_local_exact: Option<f64>,
    pub tracked_moduli: Vec<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Mid,
    InnerZero,
}

#[derive(Clone, Debug)]
pub struct JoinedGrid {
    pub mids: Vec<usize>,
    pub izs: Vec<usize>,
    pub pairs: Vec<Pair>, // row-major over iz × mid
}

impl JoinedGrid {
    pub fn idx(&self, mid: usize, iz: usize) -> Option<usize> {
        let i = self.mids.iter().position(|&m| m == mid)?;
        let j = self.izs.iter().position(|&z| z == iz)?;
        Some(j * self.mids.len() + i)
    }
}

pub fn load_sample_csv<P: AsRef<Path>>(path: P) -> io::Result<Vec<HashMap<String, String>>> {
    load_csv_internal(path)
}
pub fn load_model_csv<P: AsRef<Path>>(path: P) -> io::Result<Vec<HashMap<String, String>>> {
    load_csv_internal(path)
}

/// Robust single-line CSV splitter that keeps bracketed lists `[ ... ]` intact (no quotes used by producer).
fn split_csv_line_keep_brackets(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    for ch in line.chars() {
        match ch {
            '[' => {
                depth += 1;
                cur.push(ch);
            }
            ']' => {
                depth -= 1;
                cur.push(ch);
            }
            ',' if depth == 0 => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    out.push(cur.trim().to_string());
    out
}

fn load_csv_internal<P: AsRef<Path>>(path: P) -> io::Result<Vec<HashMap<String, String>>> {
    let f = File::open(path)?;
    let mut it = BufReader::new(f).lines();
    let header = if let Some(Ok(h)) = it.next() { h } else { return Ok(vec![]) };
    let cols: Vec<String> = header.split(',').map(|s| s.trim().to_string()).collect();
    let mut rows = Vec::new();
    for line in it {
        let line = line?;
        if line.trim().is_empty() { continue; }
        let parts = split_csv_line_keep_brackets(&line);
        let mut map = HashMap::new();
        for (i, v) in parts.into_iter().enumerate() {
            if i < cols.len() { map.insert(cols[i].clone(), v); }
        }
        rows.push(map);
    }
    Ok(rows)
}

fn parse_u32_list_field(v: &str) -> Vec<u32> {
    // Accept "[]" or "[1, 3, 7]" or "".
    let s = v.trim();
    if s.is_empty() { return vec![]; }
    let t = s.trim_matches(|c| c=='[' || c==']' || c==' ' );
    if t.is_empty() { return vec![]; }
    t.split(',')
        .filter_map(|x| x.trim().parse::<u32>().ok())
        .collect()
}

fn as_usize(map: &HashMap<String,String>, key: &str) -> Option<usize> {
    map.get(key)?.parse::<usize>().ok()
}
fn as_u32(map: &HashMap<String,String>, key: &str) -> Option<u32> {
    map.get(key)?.parse::<u32>().ok()
}
fn as_f64(map: &HashMap<String,String>, key: &str) -> Option<f64> {
    map.get(key)?.parse::<f64>().ok()
}

pub fn join_sample_and_model(
    sample_rows: &[HashMap<String,String>],
    model_rows: &[HashMap<String,String>],
) -> JoinedGrid {
    let mut mids = std::collections::BTreeSet::new();
    let mut izs = std::collections::BTreeSet::new();

    for r in sample_rows {
        if let (Some(m), Some(z)) = (as_usize(r, "mid_len"), as_usize(r, "inner_zero")) {
            mids.insert(m); izs.insert(z);
        }
    }
    for r in model_rows {
        if let (Some(m), Some(z)) = (as_usize(r, "mid_len"), as_usize(r, "inner_zero")) {
            mids.insert(m); izs.insert(z);
        }
    }

    let mids: Vec<usize> = mids.into_iter().collect();
    let izs: Vec<usize> = izs.into_iter().collect();

    let key = |m: usize, z: usize| format!("{m}_{z}");

    let mut smap: HashMap<String, &HashMap<String,String>> = HashMap::new();
    for r in sample_rows {
        if let (Some(m), Some(z)) = (as_usize(r, "mid_len"), as_usize(r, "inner_zero")) {
            smap.insert(key(m,z), r);
        }
    }
    let mut mmap: HashMap<String, &HashMap<String,String>> = HashMap::new();
    for r in model_rows {
        if let (Some(m), Some(z)) = (as_usize(r, "mid_len"), as_usize(r, "inner_zero")) {
            mmap.insert(key(m,z), r);
        }
    }

    let mut pairs = Vec::with_capacity(mids.len() * izs.len());
    for &z in &izs {
        for &m in &mids {
            let k = key(m,z);
            let s = smap.get(&k).copied();
            let t = mmap.get(&k).copied();
            let base = s.and_then(|x| as_u32(x,"base")).or_else(|| t.and_then(|x| as_u32(x,"base"))).unwrap_or(10);
            let obs = s.and_then(|x| as_f64(x,"prime_density"));
            let ci_lo = s.and_then(|x| as_f64(x,"ci_lo"));
            let ci_hi = s.and_then(|x| as_f64(x,"ci_hi"));
            let pred_exact = s.and_then(|x| as_f64(x,"expected_density_local_exact"))
                .or_else(|| t.and_then(|x| as_f64(x,"expected_density_local_exact")));
            let pred_local = s.and_then(|x| as_f64(x,"expected_density_local"))
                .or_else(|| t.and_then(|x| as_f64(x,"expected_density_local")));
            let tracked = s.and_then(|x| x.get("tracked_moduli").map(|v| parse_u32_list_field(v)))
                .or_else(|| t.and_then(|x| x.get("tracked_moduli").map(|v| parse_u32_list_field(v))))
                .unwrap_or_default();

            pairs.push(Pair{
                base,
                mid_len: m,
                inner_zero: z,
                obs_density: obs,
                ci_lo,
                ci_hi,
                pred_local,
                pred_local_exact: pred_exact,
                tracked_moduli: tracked,
            });
        }
    }

    JoinedGrid { mids, izs, pairs }
}

/// Extract a 1D series along an axis from a `JoinedGrid`.
pub fn lineout(grid: &JoinedGrid, axis: Axis, fixed_mid: usize, fixed_iz: usize) -> Vec<(usize, f64, f64)> {
    match axis {
        Axis::Mid => {
            let mut v = Vec::with_capacity(grid.mids.len());
            for &m in &grid.mids {
                let idx = grid.idx(m, fixed_iz).unwrap();
                let p = &grid.pairs[idx];
                let pred = p.pred_local_exact.or(p.pred_local).unwrap_or(0.0);
                let obs = p.obs_density.unwrap_or(0.0);
                v.push((m, obs, pred));
            }
            v
        }
        Axis::InnerZero => {
            let mut v = Vec::with_capacity(grid.izs.len());
            for &z in &grid.izs {
                let idx = grid.idx(fixed_mid, z).unwrap();
                let p = &grid.pairs[idx];
                let pred = p.pred_local_exact.or(p.pred_local).unwrap_or(0.0);
                let obs = p.obs_density.unwrap_or(0.0);
                v.push((z, obs, pred));
            }
            v
        }
    }
}

/// Convenience: enrichment = obs/pred - 1 (clamped if pred≈0).
pub fn enrichment(obs: f64, pred: f64) -> f64 {
    if pred <= 0.0 { if obs <= 0.0 { 0.0 } else { f64::INFINITY } } else { obs / pred - 1.0 }
}

/// Parse ExplainGrid JSON array (optional) and return a map (mid,iz) -> top (p, p0).
pub fn load_explain_json<P: AsRef<Path>>(path: P) -> io::Result<ExplainMap> {
    let f = File::open(path)?;
    let vals: serde_json::Value = serde_json::from_reader(f).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut out = HashMap::new();
    let arr = vals.as_array().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected array"))?;
    for v in arr {
        let mid = v.get("mid_len").and_then(|x| x.as_u64()).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "mid_len"))? as usize;
        let iz  = v.get("inner_zero").and_then(|x| x.as_u64()).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "inner_zero"))? as usize;
        let list_opt = v.get("model_p0").and_then(|x| x.as_array());
        let mut pairs = Vec::new();
        if let Some(list) = list_opt {
            for it in list {
                if let (Some(p), Some(p0)) = (it.get(0).and_then(|x| x.as_u64()), it.get(1).and_then(|x| x.as_f64())) {
                    pairs.push((p as u32, p0));
                }
            }
        }
        pairs.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
        out.insert((mid,iz), pairs);
    }
    Ok(out)
}
