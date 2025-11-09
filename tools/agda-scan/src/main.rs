use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let root = env::args().nth(1).unwrap_or_else(|| ".".into());
    let mut files = Vec::new();
    walk(Path::new(&root), &mut files)?;

    let mut info = Vec::new();
    for f in files {
        if let Some(ext) = f.extension() {
            if ext == "agda" || ext == "lagda" {
                let text = fs::read_to_string(&f)?;
                let (module, imports, posts) = analyze(&text);
                info.push((f, module, imports, posts));
            }
        }
    }

    // Map declared module -> file
    let mut name_to_file: BTreeMap<String, PathBuf> = BTreeMap::new();
    for (f, m, _, _) in &info {
        if let Some(m) = m {
            name_to_file.insert(m.clone(), f.clone());
        }
    }

    // Mismatches
    let mut mismatches = Vec::new();
    for (f, m, _, _) in &info {
        if let Some(m) = m {
            let expected = PathBuf::from(m.replace('.', "/") + ".agda");
            if !f.to_string_lossy().ends_with(expected.to_string_lossy().as_ref()) {
                mismatches.push((f.clone(), m.clone(), expected));
            }
        }
    }

    // Missing internal modules
    let std_prefixes = ["Data", "Relation", "Function", "Agda", "IO", "Level", "Axiom"];
    let mut internal_missing: BTreeMap<String, BTreeSet<PathBuf>> = BTreeMap::new();
    let mut import_hist: BTreeMap<String, usize> = BTreeMap::new();

    for (f, _m, imports, _posts) in &info {
        for imp in imports {
            *import_hist.entry(imp.clone()).or_default() += 1;
            let is_std = std_prefixes.iter().any(|p| imp.starts_with(p));
            if !is_std && !name_to_file.contains_key(imp) {
                internal_missing.entry(imp.clone()).or_default().insert(f.clone());
            }
        }
    }

    // Postulate counts
    let mut post_by_file: BTreeMap<PathBuf, usize> = BTreeMap::new();
    let mut post_total = 0usize;
    let mut post_by_dir: BTreeMap<String, usize> = BTreeMap::new();
    for (f, _m, _i, posts) in &info {
        post_by_file.insert(f.clone(), *posts);
        post_total += posts;
        if let Some(dir) = f.parent() {
            let key = dir.file_name().unwrap_or_default().to_string_lossy().to_string();
            *post_by_dir.entry(key).or_default() += posts;
        }
    }

    println!("=== Module ↔ path mismatches ({}) ===", mismatches.len());
    for (f, m, exp) in &mismatches {
        println!("- {} declares {} (expected suffix {})",
            f.to_string_lossy(), m, exp.to_string_lossy());
    }

    println!("\n=== Missing internal modules ({} kinds) ===", internal_missing.len());
    for (imp, sites) in &internal_missing {
        println!("- {}  (referenced in {} files)", imp, sites.len());
        for f in sites {
            println!("    - {}", f.to_string_lossy());
        }
    }

    println!("\n=== Top external imports ===");
    for (imp, n) in import_hist.iter().rev().take(20) {
        println!("- {imp}: {n}");
    }

    println!("\n=== Postulates ===");
    println!("Total: {}", post_total);
    println!("By directory:");
    for (d, n) in &post_by_dir {
        println!("- {d}: {n}");
    }
    println!("Top files with postulates:");
    let mut files_sorted: Vec<_> = post_by_file.iter().collect();
    files_sorted.sort_by_key(|(_, n)| *n);
    for (f, n) in files_sorted.iter().rev().take(10) {
        println!("- {}: {}", f.to_string_lossy(), n);
    }

    Ok(())
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            walk(&p, out)?;
        } else {
            out.push(p);
        }
    }
    Ok(())
}

fn analyze(src: &str) -> (Option<String>, Vec<String>, usize) {
    let s = strip_comments(src);
    let mut module: Option<String> = None;
    let mut imports = Vec::new();
    let mut postulates = 0usize;

    for line in s.lines() {
        let line = line.trim();
        if line.starts_with("module ") && line.ends_with(" where") && module.is_none() {
            let name = line.strip_prefix("module ").unwrap().strip_suffix(" where").unwrap().trim();
            module = Some(name.to_string());
        }
        if (line.starts_with("import ") || line.starts_with("open import ")) && !line.contains("--") {
            // capture token after "import " or "open import "
            let after = if let Some(rest) = line.strip_prefix("open import ") {
                rest
            } else {
                line.strip_prefix("import ").unwrap_or(line)
            };
            // stop at whitespace
            if let Some(tok) = after.split_whitespace().next() {
                imports.push(tok.to_string());
            }
        }
        if line.starts_with("postulate") {
            postulates += 1;
        }
    }
    (module, imports, postulates)
}

fn strip_comments(src: &str) -> String {
    // Remove line comments and nested block comments.
    let mut out = String::with_capacity(src.len());
    let mut i = 0usize;
    let b = src.as_bytes();
    let mut nest = 0i32;

    while i < b.len() {
        if nest == 0 && i + 1 < b.len() && b[i] == b'-' && b[i + 1] == b'-' {
            // skip to end of line
            while i < b.len() && b[i] != b'\n' { i += 1; }
            continue;
        }
        if i + 1 < b.len() && b[i] == b'{' && b[i + 1] == b'-' {
            nest += 1;
            i += 2;
            continue;
        }
        if nest > 0 && i + 1 < b.len() && b[i] == b'-' && b[i + 1] == b'}' {
            nest -= 1;
            i += 2;
            continue;
        }
        if nest == 0 {
            out.push(b[i] as char);
        }
        i += 1;
    }
    out
}
