//! Membrane Data Pump - Demonstrating massive prime generation and logging
//! Shows how much data we can pump into the event log with GPU-scale performance

use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use std::time::Instant;
use rayon::prelude::*;
use num_bigint::BigUint;
use primes::is_prime_miller_rabin;
use chrono::Local;
use sha2::{Sha256, Digest};
fn main() {
    println!("MEMBRANE DATA PUMP - MAXIMUM THROUGHPUT MODE");
    println!("===========================================\n");
    
    // Use buffered writer for maximum throughput
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("membrane_prime_events_pump.log")
        .expect("Failed to open log");
    let mut writer = BufWriter::with_capacity(1024 * 1024, file); // 1MB buffer
    // Show what we're about to do
    println!("Configuration:");
    println!("  ✓ Parallel processing across {} threads", rayon::current_num_threads());
    println!("  ✓ Buffered logging (1MB buffer)");
    println!("  ✓ Multiple bases with self-inverse digits");
    println!("  ✓ Aggressive C-value ranges\n");
    let start_time = Instant::now();
    let mut total_primes = 0;
    let mut total_bytes = 0;
    // Configuration sets optimized for prime density
    let configs = vec![
        // Base 10: Classic configurations
        (10, vec![(3, 7), (1, 9), (1, 5)], 10, 0..100_000),
        (10, vec![(3, 7), (1, 9)], 20, 0..50_000),
        
        // Base 12: Self-inverse powerhouse
        (12, vec![(5, 7), (1, 5), (1, 7), (5, 11), (7, 11)], 10, 0..100_000),
        (12, vec![(5, 7)], 15, 0..75_000),
        // Base 16: Binary-friendly
        (16, vec![(7, 9), (1, 15), (7, 15)], 10, 0..80_000),
        // Base 20: High density potential
        (20, vec![(9, 11), (1, 19), (9, 19)], 10, 0..60_000),
    ];
    // Process each configuration in parallel
    for (base, digit_pairs, w, c_range) in configs {
        println!("Pumping data for base {} configurations...", base);
        let config_start = Instant::now();
        let mut config_primes = 0;
        // Pre-compute all signatures for this base
        let primes = sieve_small_primes(10_000);
        for (l, r) in digit_pairs {
            // Vary r1, r2 for diversity
            let r_values = if w >= 10 { vec![(0, 0), (1, 0), (0, 1), (1, 1)] } else { vec![(0, 0)] };
            
            for (r1, r2) in r_values {
                let params = MembraneParams { b: base, w, r1, r2, l, r };
                
                // Pre-compute signatures
                let signatures: Vec<_> = primes.iter()
                    .map(|&p| compute_signature(&params, p))
                    .collect();
                // Parallel search with batching
                let batch_size = 10_000;
                let found_primes: Vec<_> = c_range.clone()
                    .collect::<Vec<_>>()
                    .par_chunks(batch_size)
                    .flat_map(|batch| {
                        let mut batch_primes = Vec::new();
                        
                        for &c in batch {
                            // Quick sieve
                            let mut is_candidate = true;
                            for (i, &p) in primes.iter().enumerate() {
                                let (sig, growth) = signatures[i];
                                if (sig + (c % p as u64) as u32 * growth) % p == 0 {
                                    is_candidate = false;
                                    break;
                                }
                            }
                            
                            if is_candidate {
                                let value = compute_membrane_value(&params, c);
                                if is_prime_miller_rabin(&value) {
                                    batch_primes.push((c, value));
                        }
                        batch_primes
                    })
                // Log all found primes
                for (c, value) in found_primes {
                    let log_entry = format_log_entry(&params, c, &value);
                    total_bytes += log_entry.len();
                    writeln!(writer, "{}", log_entry).unwrap();
                    config_primes += 1;
                    total_primes += 1;
                    
                    // Show progress every 1000 primes
                    if total_primes % 1000 == 0 {
                        print!("\r  Total primes logged: {} | Data written: {} | Rate: {}/sec",
                            format_number(total_primes),
                            format_bytes(total_bytes),
                            format_number((total_primes as f64 / start_time.elapsed().as_secs_f64()) as usize)
                        );
                        std::io::stdout().flush().unwrap();
                    }
                }
            }
        }
        println!("\n  Base {} complete: {} primes in {:?}", 
            base, format_number(config_primes), config_start.elapsed());
    }
    // Flush the buffer
    writer.flush().unwrap();
    let total_time = start_time.elapsed();
    println!("\n\n=== PUMP COMPLETE ===");
    println!("Total primes logged: {}", format_number(total_primes));
    println!("Total data written: {}", format_bytes(total_bytes));
    println!("Total time: {:?}", total_time);
    println!("Average rate: {} primes/second", 
        format_number((total_primes as f64 / total_time.as_secs_f64()) as usize));
    println!("Data throughput: {}/second", 
        format_bytes((total_bytes as f64 / total_time.as_secs_f64()) as usize));
    // Show some statistics about what we found
    analyze_log_diversity();
}
fn format_log_entry(params: &MembraneParams, c: u64, value: &BigUint) -> String {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let hash = compute_hash(params, c);
    let value_str = if value.to_string().len() > 100 {
        format!("{}...{} [{}d]", 
            &value.to_string()[..50],
            &value.to_string()[value.to_string().len()-20..],
            value.to_string().len())
    } else {
        value.to_string()
    };
    format!("[{}] EVT ADD_PRIME b={} w={} r1={} r2={} L={} R={} C={} hash={} value={}",
        timestamp, params.b, params.w, params.r1, params.r2, 
        params.l, params.r, c, hash, value_str)
fn analyze_log_diversity() {
    println!("\n\nLOG DIVERSITY ANALYSIS");
    println!("---------------------");
    // Read back a sample to show diversity
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("membrane_prime_events_pump.log").unwrap();
    let reader = BufReader::new(file);
    let mut base_counts = std::collections::HashMap::new();
    let mut digit_counts = std::collections::HashMap::new();
    let mut size_histogram = std::collections::HashMap::new();
    let mut sample_count = 0;
    for line in reader.lines().take(10000) {
        if let Ok(line) = line {
            if line.contains("EVT ADD_PRIME") {
                sample_count += 1;
                // Extract base
                if let Some(b_pos) = line.find("b=") {
                    if let Some(b_str) = line[b_pos+2..].split_whitespace().next() {
                        if let Ok(b) = b_str.parse::<u32>() {
                            *base_counts.entry(b).or_insert(0) += 1;
                // Extract L,R pair
                if let (Some(l_pos), Some(r_pos)) = (line.find("L="), line.find(" R=")) {
                    if let (Some(l_str), Some(r_str)) = (
                        line[l_pos+2..].split_whitespace().next(),
                        line[r_pos+3..].split_whitespace().next()
                    ) {
                        let pair = format!("({},{})", l_str, r_str);
                        *digit_counts.entry(pair).or_insert(0) += 1;
                // Extract size
                if let Some(d_pos) = line.find("[") {
                    if let Some(d_end) = line[d_pos..].find("d]") {
                        if let Ok(digits) = line[d_pos+1..d_pos+d_end].parse::<usize>() {
                            let bucket = (digits / 10) * 10;
                            *size_histogram.entry(bucket).or_insert(0) += 1;
    println!("\nBase distribution (sample of {}):", sample_count);
    let mut bases: Vec<_> = base_counts.iter().collect();
    bases.sort_by_key(|&(k, _)| k);
    for (base, count) in bases {
        println!("  Base {}: {} primes ({:.1}%)", 
            base, count, *count as f64 / sample_count as f64 * 100.0);
    println!("\nTop digit pairs:");
    let mut pairs: Vec<_> = digit_counts.iter().collect();
    pairs.sort_by_key(|&(_, v)| std::cmp::Reverse(v));
    for (pair, count) in pairs.iter().take(5) {
        println!("  {}: {} occurrences", pair, count);
    println!("\nSize distribution:");
    let mut sizes: Vec<_> = size_histogram.iter().collect();
    sizes.sort_by_key(|&(k, _)| k);
    for (bucket, count) in sizes {
        println!("  {}-{} digits: {} primes", bucket, bucket + 9, count);
// Helper structures and functions
#[derive(Clone)]
struct MembraneParams {
    b: u32, w: u32, r1: u32, r2: u32, l: u32, r: u32,
fn sieve_small_primes(limit: usize) -> Vec<u32> {
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..((limit as f64).sqrt() as usize + 1) {
        if is_prime[i] {
            for j in (i*i..limit).step_by(i) {
                is_prime[j] = false;
    is_prime.iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i as u32) } else { None })
        .collect()
fn compute_signature(params: &MembraneParams, p: u32) -> (u32, u32) {
    let sig1 = (params.l * (mod_pow(params.b, params.w - 1, p) + 1)) % p;
    let sig2 = (params.r * (mod_pow(params.b, params.w - 2 - params.r1, p) + 
                            mod_pow(params.b, params.r2 + 1, p))) % p;
    let signature = (sig1 + sig2) % p;
    let growth = mod_pow(params.b, params.w / 2, p);
    (signature, growth)
fn compute_membrane_value(params: &MembraneParams, c: u64) -> BigUint {
    let base = BigUint::from(params.b);
    let l = BigUint::from(params.l);
    let r = BigUint::from(params.r);
    let c = BigUint::from(c);
    &l * base.pow(params.w - 1) +
    &r * base.pow(params.w - 2 - params.r1) +
    &c * base.pow(params.w / 2) +
    &r * base.pow(params.r2 + 1) +
    &l
fn compute_hash(params: &MembraneParams, c: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(params.b.to_le_bytes());
    hasher.update(params.w.to_le_bytes());
    hasher.update(params.r1.to_le_bytes());
    hasher.update(params.r2.to_le_bytes());
    hasher.update(params.l.to_le_bytes());
    hasher.update(params.r.to_le_bytes());
    hasher.update(c.to_le_bytes());
    let result = hasher.finalize();
    format!("{:x}", result).chars().take(16).collect()
fn mod_pow(base: u32, exp: u32, modulus: u32) -> u32 {
    let mut result = 1u64;
    let mut base = base as u64;
    let mut exp = exp;
    let modulus = modulus as u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        base = (base * base) % modulus;
        exp >>= 1;
    result as u32
fn format_number(n: usize) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
        n.to_string()
fn format_bytes(bytes: usize) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.2} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{:.2} KB", bytes as f64 / 1_024.0)
        format!("{} bytes", bytes)
}
