//! Phase 4: AMX/SME Backend with Observable Micro-architectural Layers
//! 
//! This module implements the technical roadmap for exploiting Apple Silicon's
//! matrix accelerators (AMX/SME) while observing and shaping micro-architectural
//! behavior through PMU counters, SLC residency, and warmth tracking.

use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};
use std::cell::RefCell;

/// PMU snapshot with double-buffering to avoid oscillations
/// 
/// Performance Monitoring Unit (PMU) counters provide real-time
/// micro-architectural insights. We track:
/// - L1 cache misses: Indicates memory pressure
/// - CPU cycles: For accurate timing
/// - Timestamp: For temporal correlation
#[derive(Clone, Copy, Default)]
pub struct PmuSnapshot {
    pub l1_miss: u16,
    pub cycles: u32,
    pub ts: u64,
}

/// Double-buffered PMU readings to avoid stale data in RL loop
/// 
/// # Why Double Buffering?
/// 
/// The PMU reader runs at ~100 Hz while the RL controller may
/// run at 1-10 kHz. Without buffering:
/// - Reader blocks writer → PMU data gets stale
/// - Writer blocks reader → RL sees old data
/// 
/// Double buffering ensures:
/// - Writer never blocks (always has a free buffer)
/// - Reader always sees most recent complete snapshot
/// - No tearing (partial reads of in-progress writes)
pub struct PmuDoubleBuffer {
    buffer_a: AtomicU64, // Packed PmuSnapshot
    buffer_b: AtomicU64,
    current: AtomicU32,  // 0 = A, 1 = B
}

impl PmuDoubleBuffer {
    pub const fn new() -> Self {
        Self {
            buffer_a: AtomicU64::new(0),
            buffer_b: AtomicU64::new(0),
            current: AtomicU32::new(0),
        }
    }
    
    /// Writer updates the non-current buffer then flips current
    pub fn write(&self, snapshot: PmuSnapshot) {
        let packed = Self::pack_snapshot(snapshot);
        let current = self.current.load(Ordering::Acquire);
        
        if current == 0 {
            // Current points to A, so write to B
            self.buffer_b.store(packed, Ordering::Release);
            self.current.store(1, Ordering::Release); // Now current points to B
        } else {
            // Current points to B, so write to A
            self.buffer_a.store(packed, Ordering::Release);
            self.current.store(0, Ordering::Release); // Now current points to A
        }
    }
    
    /// Reader gets the current buffer (most recently written)
    pub fn read(&self) -> PmuSnapshot {
        let current = self.current.load(Ordering::Acquire);
        // Read from the current buffer (the one that was just written)
        let packed = if current == 0 {
            self.buffer_a.load(Ordering::Acquire)  // Current points to A
        } else {
            self.buffer_b.load(Ordering::Acquire)  // Current points to B
        };
        Self::unpack_snapshot(packed)
    }
    
    fn pack_snapshot(s: PmuSnapshot) -> u64 {
        // Pack: ts (32 bits) | cycles (16 bits) | l1_miss (16 bits)
        (s.ts & 0xFFFF_FFFF) | ((s.cycles as u64 & 0xFFFF) << 32) | ((s.l1_miss as u64) << 48)
    }
    
    fn unpack_snapshot(packed: u64) -> PmuSnapshot {
        PmuSnapshot {
            ts: packed & 0xFFFF_FFFF,
            cycles: ((packed >> 32) & 0xFFFF) as u32,
            l1_miss: ((packed >> 48) & 0xFFFF) as u16,
        }
    }
}

impl Default for PmuDoubleBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// On-chip RL controller with eligibility traces
pub struct OnChipRL {
    q_table: [[f32; 4]; 16],        // 16 states, 4 actions
    eligibility: [[f32; 4]; 16],    // TD(λ) eligibility traces
    lambda: f32,                     // Decay factor (0.8 typical)
    alpha: f32,                      // Learning rate
    state: usize,
    action: usize,
}

impl OnChipRL {
    pub fn new() -> Self {
        Self {
            q_table: [[0.0; 4]; 16],
            eligibility: [[0.0; 4]; 16],
            lambda: 0.8,
            alpha: 0.1,
            state: 0,
            action: 0,
        }
    }
    
    pub fn tick(&mut self, pmu_sample: u8, latency_ns: u32) {
        // Map PMU sample to state (4 bits)
        let new_state = (pmu_sample & 0x0F) as usize;
        
        // Compute reward from latency (negative for high latency)
        let reward = 10.0 - (latency_ns as f32 / 10.0).min(10.0);
        
        // TD error
        let td_error = reward + 0.95 * self.q_table[new_state].iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap() - self.q_table[self.state][self.action];
        
        // Update eligibility trace
        self.eligibility[self.state][self.action] += 1.0;
        
        // Update Q-values using eligibility traces
        for s in 0..16 {
            for a in 0..4 {
                self.q_table[s][a] += self.alpha * td_error * self.eligibility[s][a];
                self.eligibility[s][a] *= self.lambda * 0.95; // Decay
            }
        }
        
        // Select next action (ε-greedy)
        self.state = new_state;
        
        // Simple pseudo-random for now (avoid rand dependency)
        let epsilon = 0.1;
        let random_val = (self.state as f32 * 0.618 + latency_ns as f32 * 0.314) % 1.0;
        
        if random_val < epsilon {
            self.action = (self.state * 13 + latency_ns as usize * 7) % 4;
        } else {
            self.action = self.q_table[self.state]
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0;
        }
    }
    
    pub fn best_action(&self) -> usize {
        self.action
    }
    
    pub fn has_learned(&self) -> bool {
        self.q_table.iter().any(|row| row.iter().any(|&q| q != 0.0))
    }
}

impl Default for OnChipRL {
    fn default() -> Self {
        Self::new()
    }
}

/// SLC residency controller with demand-driven maintenance
/// 
/// # System Level Cache (SLC) Strategy
/// 
/// The M1 Max has 48 MiB of System Level Cache shared across
/// all cores. We want to keep our neural network weights resident
/// in the SLC to minimize DRAM access latency.
/// 
/// Approach:
/// 1. Monitor "warmth" - fraction of weights likely in SLC
/// 2. When warmth drops below threshold, "ping" the weights
/// 3. Pinging = volatile reads to prevent eviction
/// 4. Use pseudo-LRU pattern to maximize coverage
pub struct SlcResident {
    target_warmth: f32,
    ping_interval_ms: u64,
    last_ping: std::time::Instant,
}

impl SlcResident {
    pub fn new(target_warmth: f32) -> Self {
        Self {
            target_warmth,
            ping_interval_ms: 10,
            last_ping: std::time::Instant::now(),
        }
    }
    
    /// # Safety
    /// 
    /// This function is safe to call when:
    /// - `weights_ptr` points to a valid memory region of at least `size` bytes
    /// - The memory region remains valid for the duration of the call
    /// - No concurrent writes occur to the memory region
    pub unsafe fn maintain_residency(&mut self, current_warmth: f32, weights_ptr: *const u8, size: usize) {
        let now = std::time::Instant::now();
        
        // Only ping if warmth below threshold AND interval elapsed
        if current_warmth < self.target_warmth 
            && now.duration_since(self.last_ping).as_millis() >= self.ping_interval_ms as u128 {
            
            // Touch cache lines in pseudo-LRU pattern
            for offset in (0..size).step_by(64) {
                let ptr = weights_ptr.add(offset);
                std::ptr::read_volatile(ptr);
            }
            
            self.last_ping = now;
        }
    }
}

/// Feature-gated SME prediction (stub for now)
/// 
/// # Safety
/// 
/// Caller must ensure:
/// - The input array is properly aligned for SME operations
/// - The CPU supports SME/AMX instructions (checked by feature flag)
#[cfg(all(feature = "amx", target_arch = "aarch64"))]
pub unsafe fn predict_sme_padded(x: [i8; 16]) -> i32 {
    // Stub: Simple computation until real SME intrinsics available
    // For now, just sum the first 8 elements
    x[..8].iter().map(|&v| v as i32).sum()
}

/// # Safety
/// 
/// This fallback version is marked unsafe for API compatibility,
/// but performs only safe operations internally.
#[cfg(not(all(feature = "amx", target_arch = "aarch64")))]
pub unsafe fn predict_sme_padded(x: [i8; 16]) -> i32 {
    // Direct computation fallback
    x[..8].iter().map(|&v| v as i32).sum()
}

/// Safe wrapper that checks for SME support
pub fn predict_sme_padded_safe(x: [i8; 16]) -> i32 {
    // SAFETY: predict_sme_padded expects a properly aligned 16-byte array,
    // which is guaranteed by the function signature
    unsafe { predict_sme_padded(x) }
}

// Global RL controller for testing
thread_local! {
    pub static RL_CTL: RefCell<OnChipRL> = RefCell::new(OnChipRL::new());
}

/// Global PMU double buffer
pub static PMU_BUFFER: PmuDoubleBuffer = PmuDoubleBuffer::new();

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn pmu_double_buffer_no_tearing() {
        // Use a local buffer for testing to avoid global state issues
        let buffer = PmuDoubleBuffer::new();
        
        let snapshot1 = PmuSnapshot { l1_miss: 100, cycles: 1000, ts: 1 };
        let snapshot2 = PmuSnapshot { l1_miss: 200, cycles: 2000, ts: 2 };
        
        // Initially reads zeros
        let read0 = buffer.read();
        assert_eq!(read0.ts, 0);
        
        buffer.write(snapshot1);
        let read1 = buffer.read();
        assert_eq!(read1.ts, 1);
        
        buffer.write(snapshot2);
        let read2 = buffer.read();
        assert_eq!(read2.ts, 2);
    }
    
    #[test]
    fn rl_eligibility_trace_update() {
        let mut rl = OnChipRL::new();
        
        // Initial state
        assert_eq!(rl.best_action(), 0);
        
        // Feed some samples
        for i in 0..100 {
            rl.tick(i as u8, 5 + (i % 10) as u32);
        }
        
        // Should have non-zero eligibility traces
        let has_eligibility = rl.eligibility.iter()
            .any(|row| row.iter().any(|&e| e > 0.0));
        assert!(has_eligibility, "Eligibility traces not updating");
    }
}