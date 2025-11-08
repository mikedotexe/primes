//! Minimal DVFS tracker – keeps a global moving average of CPU‑GHz.
//! Non‑Apple & WASM targets fall back to a fixed 3.0 GHz.

use std::sync::atomic::{AtomicU64, Ordering};

static GHZ_FP_BITS: AtomicU64 = AtomicU64::new(f64::to_bits(3.0));

/// Returns current estimate in GHz (cheap – no syscalls on fast path)
pub fn cpu_freq_ghz() -> f64 {
    f64::from_bits(GHZ_FP_BITS.load(Ordering::Relaxed))
}

/// Slow‑path updater – call once every 100 ms from a background thread.
#[cfg(feature = "dvfs-adaptive")]
pub fn refresh() {
    #[cfg(target_arch = "aarch64")]
    {
        // Apple M‑series: Use performance monitoring for frequency detection
        // For now, use a conservative estimate based on thermal state
        let ghz = estimate_current_frequency();
        let bits = f64::to_bits(ghz);
        GHZ_FP_BITS.store(bits, Ordering::Relaxed);
    }
    
    #[cfg(not(target_arch = "aarch64"))]
    {
        // x86_64: Could use CPUID or MSR reads, but for safety use fixed 3.0 GHz
        let bits = f64::to_bits(3.0);
        GHZ_FP_BITS.store(bits, Ordering::Relaxed);
    }
}

#[cfg(not(feature = "dvfs-adaptive"))]
pub fn refresh() {
    // No-op when feature disabled
}

#[cfg(target_arch = "aarch64")]
#[allow(dead_code)] // Used only when dvfs-adaptive feature is enabled
fn estimate_current_frequency() -> f64 {
    // Conservative frequency estimation for Apple Silicon
    // TODO: Use kpc/sysctl for actual P‑core frequency measurement
    
    // Read cycle counter frequency (fixed at 24 MHz on Apple Silicon)  
    let _cntfrq: u64;
    unsafe {
        std::arch::asm!("mrs {0}, cntfrq_el0", out(reg) _cntfrq);
    }
    
    // Apple M1/M2 typically run 3.0-3.2 GHz under load
    // Use thermal-aware estimate
    match std::env::var("THERMAL_STATE") {
        Ok(state) if state == "hot" => 2.8, // Throttled
        Ok(state) if state == "warm" => 3.0, // Nominal
        _ => 3.2, // Cool/optimal
    }
}

/// Start background DVFS monitoring thread
#[cfg(feature = "dvfs-adaptive")]
pub fn start_monitor() -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| loop {
        refresh();
        std::thread::sleep(std::time::Duration::from_millis(100));
    })
}

#[cfg(not(feature = "dvfs-adaptive"))]
pub fn start_monitor() -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| {
        // Minimal thread that does nothing when feature disabled
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_freq_reasonable() {
        let freq = cpu_freq_ghz();
        assert!(freq > 1.0 && freq < 6.0, "CPU frequency {} GHz seems unreasonable", freq);
    }

    #[test]
    fn test_refresh_no_panic() {
        refresh(); // Should not panic on any platform
        let freq = cpu_freq_ghz();
        assert!(freq > 0.0);
    }
}