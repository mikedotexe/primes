//! prime_physics_wasm – Interactive membrane prime generation & cache-aware sieve
//! 
//! This WebAssembly module exposes two main features:
//! 1. Membrane prime generation with configurable parameters
//! 2. Cache-aware bit-packed prime sieve with real-time performance metrics

use wasm_bindgen::prelude::*;
use prime_physics_engine::{MembraneConfig, PrimeCandidate};
use std::convert::TryInto;

// Better error handling for WASM
#[wasm_bindgen]
#[derive(Debug)]
pub struct WasmError {
    message: String,
}

#[wasm_bindgen]
impl WasmError {
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl From<String> for WasmError {
    fn from(msg: String) -> Self {
        WasmError { message: msg }
    }
}

impl From<&str> for WasmError {
    fn from(msg: &str) -> Self {
        WasmError { message: msg.to_string() }
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    // Better error messages in the browser console
    console_error_panic_hook::set_once();
}

/* --------------------------------------------------------------------- */
/* 1.  Membrane Prime Generation                                         */
/* --------------------------------------------------------------------- */

/// Generate membrane primes with given configuration
/// Returns array of [candidate_value, is_prime] pairs
/// Throws error if parameters are invalid
#[wasm_bindgen]
pub fn generate_membrane_primes(
    base: u32,
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    middle_start: u32,
    count: u32,
) -> Result<Vec<u32>, WasmError> {
    // Validate inputs
    if base < 2 || base > 36 {
        return Err("Base must be between 2 and 36".into());
    }
    if outer >= base || inner >= base {
        return Err("Boundary digits must be less than base".into());
    }
    if k_outer > 10 || k_inner > 10 {
        return Err("Padding values too large (max 10)".into());
    }
    if count > 1000 {
        return Err("Count too large (max 1000 for performance)".into());
    }
    
    let config = MembraneConfig {
        base,
        outer,
        inner, 
        k_outer,
        k_inner,
    };
    
    let mut results = Vec::with_capacity(count as usize * 3);
    
    for i in 0..count {
        let middle = middle_start.saturating_add(i);
        let candidate = config.generate(middle);
        
        // For WASM, we'll use a simple primality test for numbers that fit in u64
        let is_prime = if let Ok(n) = candidate.to_u64() {
            is_prime_simple(n)
        } else {
            // For very large numbers, use Miller-Rabin
            candidate.is_prime()
        };
        
        // Pack result as [value_low32, value_high32, is_prime]
        if let Ok(n) = candidate.to_u64() {
            results.push((n & 0xFFFFFFFF) as u32);
            results.push((n >> 32) as u32);
            results.push(if is_prime { 1 } else { 0 });
        } else {
            // Number too large for u64, return special marker
            results.push(0xFFFFFFFF);
            results.push(0xFFFFFFFF);
            results.push(if is_prime { 1 } else { 0 });
        }
    }
    
    Ok(results)
}

/// Check if a configuration uses coprime boundary digits
#[wasm_bindgen]
pub fn is_coprime_config(base: u32, outer: u32, inner: u32) -> bool {
    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    
    gcd(outer, base) == 1 && gcd(inner, base) == 1
}

/// Simple primality test for small numbers
fn is_prime_simple(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

/* --------------------------------------------------------------------- */
/* 2.  Cache-aware Prime Sieve                                           */
/* --------------------------------------------------------------------- */

/// Count primes up to limit using bit-packed sieve
#[wasm_bindgen]
pub fn sieve_count_primes(limit: u32) -> Result<u32, WasmError> {
    if limit < 2 { 
        return Ok(0); 
    }
    if limit > 100_000_000 {
        return Err("Limit too large (max 100M for browser performance)".into());
    }
    
    let limit = limit as usize;
    let sieve = prime_physics_engine::prime_sieve::BitSieve::new(limit);
    Ok(sieve.primes().len() as u32)
}

/// Benchmark sieve performance - returns [count, milliseconds]
#[wasm_bindgen]
pub fn sieve_benchmark(limit: u32) -> Result<Vec<f64>, WasmError> {
    if limit > 50_000_000 {
        return Err("Limit too large for benchmark (max 50M)".into());
    }
    
    let start = web_sys::window()
        .ok_or("No window object")?
        .performance()
        .ok_or("No performance API")?
        .now();
    
    let count = sieve_count_primes(limit)?;
    
    let elapsed = web_sys::window()
        .unwrap()
        .performance()
        .unwrap()
        .now() - start;
    
    Ok(vec![count as f64, elapsed])
}

/// Get detailed sieve statistics including performance metrics
#[wasm_bindgen]
pub fn sieve_with_stats(limit: u32) -> Result<JsValue, WasmError> {
    use serde_json::json;
    
    if limit == 0 || limit > 10_000_000 {
        return Err("Limit must be between 1 and 10,000,000".into());
    }
    
    let start = web_sys::window()
        .ok_or("No window object")?
        .performance()
        .ok_or("No performance API")?
        .now();
    
    let count = sieve_count_primes(limit)?;
    
    let elapsed_ms = web_sys::window()
        .unwrap()
        .performance()
        .unwrap()
        .now() - start;
    
    let ns_per_prime = if count > 0 { 
        (elapsed_ms * 1_000_000.0) / count as f64 
    } else { 
        0.0 
    };
    
    let result = json!({
        "limit": limit,
        "primeCount": count,
        "elapsedMs": elapsed_ms,
        "nsPerPrime": ns_per_prime,
        "throughput": {
            "primesPerSecond": if ns_per_prime > 0.0 { 
                1_000_000_000.0 / ns_per_prime 
            } else { 
                0.0 
            },
            "candidatesPerSecond": if elapsed_ms > 0.0 { 
                limit as f64 / elapsed_ms * 1000.0 
            } else { 
                0.0 
            }
        },
        "memory": {
            "bytesUsed": (limit / 16), // Bit-packed estimate
            "efficiency": "1 bit per odd number"
        }
    });
    
    Ok(JsValue::from_serde(&result).map_err(|e| WasmError::from(e.to_string()))?)
}

/* --------------------------------------------------------------------- */
/* 3.  Phase 4 Neural Network Demo                                       */
/* --------------------------------------------------------------------- */

/// Simple 8→16→1 neural network prediction
#[wasm_bindgen]
pub fn neural_predict(input: &[i8]) -> i32 {
    if input.len() < 8 {
        return 0;
    }
    
    // Pad to 16 for the SME-style interface
    let mut padded = [0i8; 16];
    padded[..8].copy_from_slice(&input[..8]);
    
    // Use the safe wrapper from phase4
    prime_physics_engine::phase4::predict_sme_padded_safe(padded)
}

/* --------------------------------------------------------------------- */
/* 4.  Utilities                                                         */
/* --------------------------------------------------------------------- */

/// Get optimal configurations for a given base
#[wasm_bindgen]
pub fn get_optimal_configs(base: u32) -> Vec<u32> {
    let mut configs = Vec::new();
    
    // Return top 3 configurations for the base
    // Format: [outer1, inner1, k_outer1, k_inner1, outer2, inner2, ...]
    match base {
        6 => configs.extend_from_slice(&[1, 5, 0, 0, 5, 1, 0, 0, 1, 5, 0, 1]),
        10 => configs.extend_from_slice(&[3, 7, 0, 0, 7, 3, 0, 0, 1, 3, 0, 0]),
        12 => configs.extend_from_slice(&[5, 7, 0, 0, 7, 5, 0, 0, 1, 5, 0, 0]),
        _ => configs.extend_from_slice(&[1, base-1, 0, 0, 1, base/2, 0, 0, 1, 2, 0, 0]),
    }
    
    configs
}

/// Get statistics about membrane generation
#[wasm_bindgen]
pub fn get_membrane_stats(
    base: u32,
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    samples: u32,
) -> Result<Vec<f64>, WasmError> {
    // Validate inputs
    if base < 2 || base > 36 {
        return Err("Invalid base".into());
    }
    if samples == 0 || samples > 10000 {
        return Err("Samples must be between 1 and 10000".into());
    }
    
    let config = MembraneConfig {
        base,
        outer,
        inner,
        k_outer,
        k_inner,
    };
    
    let mut prime_count = 0;
    let mut total_digits = 0;
    let mut largest_prime = 0u64;
    
    for middle in 1..=samples {
        let candidate = config.generate(middle);
        if candidate.is_prime() {
            prime_count += 1;
            if let Ok(n) = candidate.to_u64() {
                largest_prime = largest_prime.max(n);
            }
        }
        total_digits += candidate.to_string().len();
    }
    
    let success_rate = prime_count as f64 / samples as f64;
    let avg_digits = total_digits as f64 / samples as f64;
    
    Ok(vec![
        prime_count as f64, 
        success_rate * 100.0, 
        avg_digits,
        largest_prime as f64
    ])
}