use super::{JoinedGrid, enrichment, load_explain_json};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct VerifyRow {
    pub base: u32,
    pub mid_len: usize,
    pub inner_zero: usize,
    pub obs: Option<f64>,
    pub pred: Option<f64>,
    pub ci_width: Option<f64>,
    pub delta_abs: Option<f64>,
    pub enrichment: Option<f64>,
    pub union_any: Option<f64>,
    pub top_moduli: Vec<(u32,f64)>,
}

pub fn verify_to_csv<P: AsRef<Path>>(
    grid: &JoinedGrid,
    explain_map: Option<&std::collections::HashMap<(usize,usize), Vec<(u32,f64)>>>,
    out_csv: P,
) -> io::Result<()> {
    if let Some(dir) = out_csv.as_ref().parent() { if !dir.exists() { fs::create_dir_all(dir)?; } }
    let mut w = File::create(out_csv)?;
    writeln!(w, "base,mid_len,inner_zero,obs,pred,ci_width,delta_abs,enrichment,union_any,top_moduli")?;
    for p in &grid.pairs {
        let pred = p.pred_local_exact.or(p.pred_local);
        let obs = p.obs_density;
        let ciw = match (p.ci_lo, p.ci_hi) { (Some(a), Some(b)) => Some((b-a).abs()), _ => None };
        let dabs = match (obs, pred) { (Some(o), Some(pr)) => Some(o - pr), _ => None };
        let enr  = match (obs, pred) { (Some(o), Some(pr)) => Some(super::enrichment(o, pr)), _ => None };

        let top = explain_map
            .and_then(|m| m.get(&(p.mid_len, p.inner_zero)).cloned())
            .unwrap_or_default();

        // union_any lives in Explain JSON as union_p_any; we surface it by summing top moduli as heuristic if not included
        let union_any = explain_map
            .and_then(|m| m.get(&(p.mid_len, p.inner_zero)))
            .map(|list| list.iter().map(|(_,q)| *q).fold(0.0f64, |a: f64, b: f64| a.max(b))); // weak proxy if not included

        writeln!(
            w,
            "{},{},{},{},{},{},{},{},{},{}",
            p.base,
            p.mid_len,
            p.inner_zero,
            fmt_opt(obs),
            fmt_opt(pred),
            fmt_opt(ciw),
            fmt_opt(dabs),
            fmt_opt(enr),
            fmt_opt(union_any),
            fmt_top(&top)
        )?;
    }
    Ok(())
}

fn fmt_opt(x: Option<f64>) -> String {
    match x { Some(v) => format!("{:.8}", v), None => String::from("") }
}
fn fmt_top(v: &[(u32,f64)]) -> String {
    if v.is_empty() { String::from("[]") }
    else {
        let items: Vec<String> = v.iter().take(6).map(|(p,q)| format!("(p={},p0={:.4})",p,q)).collect();
        format!("[{}]", items.join(","))
    }
}
