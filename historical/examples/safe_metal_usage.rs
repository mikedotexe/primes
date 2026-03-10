//! Safe Metal GPU Usage Example
//! 
//! Demonstrates proper error handling and platform detection for Metal GPU acceleration.
//! This example shows how to gracefully handle all failure modes and provide
//! meaningful feedback to users.

use primes::{
    membrane::MembraneConfig,
    is_prime_miller_rabin,
};
use num_bigint::BigUint;
use std::time::Instant;

// Only import GPU types on macOS
#[cfg(feature = "metal")]
use primes::gpu::GpuSieve;

fn main() {
    println!("🛡️  Safe Metal GPU Usage Demo");
    println!("==============================\n");
    
    // Display platform information
    display_platform_info();
    
    // Test GPU availability
    test_gpu_availability();
    
    // Run safe computation with fallback
    let config = MembraneConfig::new(6, 1, 5, 0, 0);
    let candidate_count = 10_000;
    
    println!("\n📊 Testing {} membrane candidates with config: Base {} ({},{})",
        candidate_count, config.base, config.outer, config.inner
    );
    
    let primes = safe_prime_discovery(&config, candidate_count);
    
    println!("\n✅ Found {} primes ({:.1}% success rate)",
        primes.len(),
        (primes.len() as f64 / candidate_count as f64) * 100.0
    );
    
    // Show some example primes
    if !primes.is_empty() {
        println!("\nExample primes found:");
        for (i, prime) in primes.iter().take(5).enumerate() {
            println!("  {}: {}", i + 1, prime);
        }
        if primes.len() > 5 {
            println!("  ... and {} more", primes.len() - 5);
        }
    }
}

/// Display current platform information
fn display_platform_info() {
    println!("🖥️  Platform Information:");
    println!("  OS: {}", std::env::consts::OS);
    println!("  Architecture: {}", std::env::consts::ARCH);
    
    #[cfg(target_os = "macos")]
    {
        println!("  ✅ Running on macOS - Metal support available");
        
        // Check macOS version
        if let Ok(output) = std::process::Command::new("sw_vers")
            .arg("-productVersion")
            .output()
        {
            if let Ok(version) = String::from_utf8(output.stdout) {
                println!("  macOS version: {}", version.trim());
            }
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        println!("  ⚠️  Not running on macOS - Metal GPU acceleration unavailable");
        println!("  💡 CPU implementation will be used instead");
    }
}

/// Test GPU availability with detailed diagnostics
fn test_gpu_availability() {
    println!("\n🔍 Testing GPU Availability:");
    
    #[cfg(feature = "metal")]
    {
        match test_metal_gpu() {
            Ok(info) => {
                println!("  ✅ Metal GPU initialized successfully!");
                println!("  {}", info);
            }
            Err(e) => {
                println!("  ❌ Metal GPU initialization failed:");
                println!("  Error: {}", e);
                println!("  💡 Tip: Ensure you've compiled Metal shaders with build_metal.sh");
            }
        }
    }
    
    #[cfg(not(feature = "metal"))]
    {
        println!("  ⚠️  Metal feature not enabled during compilation");
        println!("  💡 To enable: cargo run --example safe_metal_usage --features metal");
    }
}

/// Test Metal GPU initialization and return diagnostic info
#[cfg(feature = "metal")]
fn test_metal_gpu() -> Result<String, String> {
    use std::env;
    
    // Check if we're on macOS first
    if cfg!(not(target_os = "macos")) {
        return Err("Metal is only available on macOS".to_string());
    }
    
    // Check for METALLIB_PATH
    if env::var("METALLIB_PATH").is_err() {
        return Err("METALLIB_PATH environment variable not set. Run: export METALLIB_PATH=src/metal/default.metallib".to_string());
    }
    
    // Try to create GPU sieve
    match GpuSieve::new() {
        Ok(_) => {
            // Get GPU info if possible
            let mut info = String::from("GPU Details:");
            
            // Try to get GPU name (this is macOS specific)
            if let Ok(output) = std::process::Command::new("system_profiler")
                .args(&["SPDisplaysDataType", "-json"])
                .output()
            {
                if let Ok(json_str) = String::from_utf8(output.stdout) {
                    if let Some(gpu_line) = json_str.lines()
                        .find(|line| line.contains("sppci_model"))
                    {
                        info.push_str(&format!("\n  {}", gpu_line.trim()));
                    }
                }
            }
            
            Ok(info)
        }
        Err(e) => Err(format!("GPU initialization error: {}", e))
    }
}

/// Safe prime discovery with automatic GPU/CPU fallback
fn safe_prime_discovery(config: &MembraneConfig, count: usize) -> Vec<BigUint> {
    let start = Instant::now();
    let mut primes = Vec::new();
    
    // Try GPU acceleration first
    #[cfg(feature = "metal")]
    {
        match try_gpu_discovery(config, count) {
            Ok(gpu_primes) => {
                let elapsed = start.elapsed();
                println!("\n🚀 GPU processing completed in {:.3}s", elapsed.as_secs_f64());
                return gpu_primes;
            }
            Err(e) => {
                println!("\n⚠️  GPU processing failed: {}", e);
                println!("📱 Falling back to CPU implementation...");
            }
        }
    }
    
    // CPU fallback
    println!("\n💻 Using CPU implementation...");
    let cpu_start = Instant::now();
    
    for seed in 0..count {
        let number = generate_membrane_number(config, seed as u64);
        if is_prime_miller_rabin(&number) {
            primes.push(number);
        }
        
        // Progress indicator
        if seed % 1000 == 999 {
            print!(".");
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        }
    }
    
    let cpu_elapsed = cpu_start.elapsed();
    println!("\n✅ CPU processing completed in {:.3}s", cpu_elapsed.as_secs_f64());
    
    primes
}

/// Try GPU-accelerated prime discovery
#[cfg(feature = "metal")]
fn try_gpu_discovery(config: &MembraneConfig, count: usize) -> Result<Vec<BigUint>, String> {
    use primes::metal_host::build_packed6;
    
    // Initialize GPU
    let gpu = GpuSieve::new()?;
    
    // Prepare candidates (simplified for demo)
    let mut candidates = Vec::with_capacity(count);
    for i in 0..count {
        // Simple membrane value calculation that fits in u32
        let value = compute_simple_membrane(config, i as u32);
        candidates.push(value);
    }
    
    // Run GPU sieve
    let survivors = gpu.sieve(&candidates, config.base)?;
    
    // Verify survivors with Miller-Rabin
    let mut primes = Vec::new();
    for &idx in &survivors {
        if (idx as usize) < candidates.len() {
            let value = BigUint::from(candidates[idx as usize]);
            if is_prime_miller_rabin(&value) {
                primes.push(value);
            }
        }
    }
    
    Ok(primes)
}

/// Simple membrane calculation that fits in u32
fn compute_simple_membrane(config: &MembraneConfig, seed: u32) -> u32 {
    let base = config.base;
    let outer = config.outer as u32;
    let inner = config.inner as u32;
    let middle = seed % 10;
    
    // Simplified calculation to avoid overflow
    let mut result = outer;
    result = result.saturating_mul(base).saturating_add(inner);
    result = result.saturating_mul(base).saturating_add(middle);
    result = result.saturating_mul(base).saturating_add(inner);
    result = result.saturating_mul(base).saturating_add(outer);
    
    result
}

/// Generate membrane number for CPU processing
fn generate_membrane_number(config: &MembraneConfig, seed: u64) -> BigUint {
    let base = BigUint::from(config.base);
    let outer = BigUint::from(config.outer);
    let inner = BigUint::from(config.inner);
    let middle = BigUint::from(seed % 10);
    
    let mut result = outer.clone();
    
    for _ in 0..config.k_outer {
        result = &result * &base;
    }
    
    result = &result * &base + &inner;
    
    for _ in 0..config.k_inner {
        result = &result * &base;
    }
    
    result = &result * &base + &middle;
    
    for _ in 0..config.k_inner {
        result = &result * &base;
    }
    
    result = &result * &base + &inner;
    
    for _ in 0..config.k_outer {
        result = &result * &base;
    }
    
    result = &result * &base + &outer;
    
    result
}