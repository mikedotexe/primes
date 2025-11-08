# Prime Physics Engine - Advanced Optimization Stretch Goals 🚀

## Executive Summary

Two non-trivial optimizations that complement the existing roadmap and deliver measurable performance improvements through sophisticated systems engineering.

## Goal 1: Hierarchical Wheel + SIMD "Bit-Blast" 🎯

### Overview
Replace the classic odd-only bitset with a hierarchically-compressed wheel (30- or 210-wheel) and eliminate all remaining per-prime loop branches by SIMD blasting entire cache-line-sized "mask packets" at once.

### Performance Impact

| Metric | Current State | Target State | Expected Gain |
|--------|--------------|--------------|---------------|
| Bit Density | 1 bit per odd number | 1 bit per number not divisible by 2·3·5 | **2.5× denser** |
| Inner Loop | `while p*p ≤ n { mark += step }` per prime | Pre-computed per-wheel-sector masks → branch-free store | **~30% fewer branches** |
| Memory BW | Full scan each segment | 40-60% fewer bytes touched | **1.4GB saved for n=1B** |
| Registers | Scalar add | 128/256-bit "sieve mask" vectors | **16× throughput** |

### Technical Challenges

1. **Cache Residency**: The wheel mask must stay cache-resident; naïve pre-compute spills out of L1
2. **State Management**: For segmented sieve, must keep per-prime rolling "offset" state mod wheel without branches
3. **Platform Diversity**: Different SIMD backends (NEON vs AVX2/AVX-512) need separate intrinsics

### Implementation Sketch

```rust
/// 30-wheel: pattern length = 8 bytes (for 30 numbers, 8 odd survivors)
const WHEEL30: [u8; 8] = [  // 1 bit = prime candidate
    0b10101010, // sector 0: numbers ≡ 1, 7, 11, 13, 17, 19, 23, 29 (mod 30)
    0b01010100, // sector 1
    0b00101010, // sector 2
    0b10010100, // sector 3
    0b01001010, // sector 4
    0b10100100, // sector 5
    0b01010010, // sector 6
    0b00101001, // sector 7
];

#[cfg(all(target_arch = "aarch64", feature = "wheel30"))]
#[inline(always)]
unsafe fn blast_lane_neon(ptr: *mut u8) {
    use std::arch::aarch64::*;
    // WHEEL30 repeated 8× fills one 64-B cache line
    let pattern = vld1q_u8(WHEEL30.as_ptr());
    for offset in (0..64).step_by(16) {
        vst1q_u8(ptr.add(offset), pattern);
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2", feature = "wheel30"))]
#[inline(always)]
unsafe fn blast_lane_avx2(ptr: *mut u8) {
    use std::arch::x86_64::*;
    // Broadcast wheel pattern to 256-bit vector
    let pattern = _mm256_loadu_si256(WHEEL30.as_ptr() as *const __m256i);
    _mm256_storeu_si256(ptr as *mut __m256i, pattern);
    _mm256_storeu_si256(ptr.add(32) as *mut __m256i, pattern);
}

/// Mark a full 64-byte chunk in one go
#[inline(always)]
pub fn mark_chunk(ptr: *mut u8) {
    #[cfg(all(target_arch = "aarch64", feature = "wheel30"))]
    unsafe { blast_lane_neon(ptr) }
    
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2", feature = "wheel30"))]
    unsafe { blast_lane_avx2(ptr) }
    
    #[cfg(not(any(
        all(target_arch = "aarch64", feature = "wheel30"),
        all(target_arch = "x86_64", target_feature = "avx2", feature = "wheel30")
    )))]
    {
        // Scalar fallback
        for i in 0..64 {
            unsafe { *ptr.add(i) = WHEEL30[i % 8]; }
        }
    }
}
```

### Measurement Strategy

1. **Criterion Benchmark**: Before/after on L2-sized segment (~256 KiB)
2. **Key Metrics**: 
   - ns/prime or cycles/bit-clear
   - Memory bandwidth utilization
   - L3 cache miss rate
3. **Expected Results**: 20-30% win on Apple M-cores, ~15% on x86-64 when memory-bound

### Integration Path

```rust
// In BitSieve::run_segment()
for chunk in segment.chunks_mut(64) {  // 64-byte aligned chunks
    if chunk.len() == 64 {
        mark_chunk(chunk.as_mut_ptr());
    } else {
        // Handle tail
        chunk.fill(WHEEL30[0]);  // simplified
    }
}
```

## Goal 2: Dynamic DVFS-Aware Scheduling 🔄

### Overview
Sample instantaneous core frequency every N segments and adaptively grow/shrink segment size so sieve-worker's inner loop fits inside the turbo window (~30ms on M-series).

### Performance Impact

| Metric | Current State | Target State | Expected Gain |
|--------|--------------|--------------|---------------|
| Segment Size | Compile-time / CLI flag | Adaptive: `usize → usize` | **Dynamic optimization** |
| Throughput Variance | ±25% across long runs | <±10% | **2.5× more stable** |
| Energy/prime | Baseline | -5% to -10% | **Better perf/watt** |

### Technical Challenges

1. **Platform Differences**: Cycle counters on Apple Silicon run at fixed 24 MHz, but core frequency varies 600 MHz – 3.4 GHz
2. **Bookkeeping Complexity**: Adapting segment size mid-run must not break the segmented sieve's "pending offset" bookkeeping
3. **Zero-Cost Abstraction**: Needs lock-free, O(1) cost: any branch inside tight loop is too expensive

### Implementation Design

```rust
use std::sync::atomic::{AtomicU64, Ordering};

/// Moving average of microseconds per prime (Q32.32 fixed-point)
static GLOBAL_US_PER_PRIME: AtomicU64 = AtomicU64::new(0);

/// Every N segments update a cheap global moving-average in µs/prime
fn dvfs_feedback(segment_cycles: u64, segment_primes: usize) {
    const HISTORY: usize = 16;  // Moving average window
    
    // Convert to Q32.32 fixed-point for atomic operations
    let new_sample = ((segment_cycles as u128) << 32) / segment_primes as u128;
    let old = GLOBAL_US_PER_PRIME.load(Ordering::Relaxed);
    let blended = ((old * (HISTORY as u64 - 1)) + new_sample as u64) / HISTORY as u64;
    GLOBAL_US_PER_PRIME.store(blended, Ordering::Relaxed);
}

/// Called by each worker before fetching the next segment
fn choose_next_segment_size() -> usize {
    const DEFAULT_SEG: usize = 64 * 1024;  // 64 KiB baseline
    
    let q32 = GLOBAL_US_PER_PRIME.load(Ordering::Relaxed);
    let us_per_prime = (q32 >> 32) as f64 + (q32 & 0xFFFFFFFF) as f64 / 4294967296.0;
    
    // Adaptive sizing based on current performance
    let seg = if us_per_prime > 0.50 {      // ~2× slower than turbo
        DEFAULT_SEG << 2                    // 256 KiB segment
    } else if us_per_prime < 0.20 {         // Running at turbo
        DEFAULT_SEG >> 1                    // 32 KiB for better L1 reuse
    } else {
        DEFAULT_SEG                         // Standard 64 KiB
    };
    
    seg.clamp(32 * 1024, 512 * 1024)      // Sane bounds
}

/// Enhanced segmented sieve with DVFS awareness
pub fn adaptive_segmented_sieve(limit: usize) -> Vec<usize> {
    use rayon::prelude::*;
    use crate::performance::CycleTimer;
    
    let sqrt_limit = (limit as f64).sqrt() as usize + 1;
    let base_sieve = BitSieve::new(sqrt_limit);
    let base_primes = base_sieve.primes();
    
    let mut all_primes = vec![2];
    let mut current_start = 3;
    
    while current_start <= limit {
        let segment_size = choose_next_segment_size();
        let segment_end = (current_start + segment_size - 1).min(limit);
        
        let mut timer = CycleTimer::new();
        timer.start();
        
        // Process segment (existing logic)
        let segment_primes = process_segment(current_start, segment_end, &base_primes);
        let prime_count = segment_primes.len();
        
        let cycles = timer.stop();
        dvfs_feedback(cycles, prime_count);
        
        all_primes.extend(segment_primes);
        current_start = segment_end + 2;  // Next odd
    }
    
    all_primes
}
```

### Measurement Strategy

1. **Long-Running Test**: 60-second continuous sieve run
2. **Metrics to Track**:
   - Standard deviation of ns/prime
   - CPU frequency samples via `spawn_dvfs_sampler()`
   - Energy consumption (if platform supports)
3. **Success Criteria**: >50% reduction in throughput variance

### Platform-Specific Considerations

```rust
#[cfg(target_os = "macos")]
fn get_instant_freq_ghz() -> f64 {
    // Use IOKit or powermetrics API
    // Fallback: estimate from instruction retirement rate
}

#[cfg(target_os = "linux")]
fn get_instant_freq_ghz() -> f64 {
    // Read from /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq
    // Or use perf_event_open with PERF_COUNT_HW_CPU_CYCLES
}
```

## Integration & Testing Plan

### Phase 1: Wheel + SIMD (2-3 weeks)
1. Implement behind `#[cfg(feature="wheel30")]`
2. Create comprehensive benchmarks with Criterion
3. Measure with platform-specific tools:
   - macOS: `pmc` counters
   - Linux: `perf stat -e mem_load_retired.l3_miss`
4. Target: ≥20% reduction in memory reads

### Phase 2: DVFS Adaptation (1-2 weeks)
1. Integrate with existing `spawn_dvfs_sampler()`
2. Add `adaptive_segmented_sieve` alongside existing implementation
3. Create stress test that triggers frequency scaling
4. Target: <10% throughput variance over 60s runs

### Phase 3: Combined Optimization (1 week)
1. Enable both features simultaneously
2. Tune interactions (wheel density affects optimal segment size)
3. Create production benchmarks on various platforms
4. Document configuration recommendations

## Expected Combined Impact

### Large-Scale Prime Counting (n = 10M...1B)
- Memory bandwidth reduction: **25-30%** wall-time improvement
- Especially significant on bandwidth-constrained systems

### Thermally-Constrained Devices
- MacBook Air, Raspberry Pi, mobile processors
- Recovers **~10%** wall-time lost to DVFS throttling
- Better sustained performance under load

### Energy Efficiency
- Fewer memory accesses = less power
- Adaptive sizing = more time at efficient frequency
- Combined: **10-15%** better performance/watt

## Risk Mitigation

1. **Feature Flags**: All optimizations behind compile-time flags
2. **Fallback Paths**: Scalar implementations for all SIMD code
3. **Gradual Rollout**: Test on CI matrix before enabling by default
4. **Performance Regression Tests**: Criterion benchmarks in CI

## Conclusion

These stretch goals represent sophisticated systems optimizations that:
- Attack different bottlenecks (memory bandwidth vs CPU frequency)
- Are orthogonal to existing work (GPU, BigInt, etc.)
- Provide measurable, significant improvements
- Demonstrate advanced engineering capabilities

The hierarchical wheel reduces the fundamental memory pressure of prime generation, while DVFS-aware scheduling adapts to modern CPU behavior. Together, they push the Prime Physics Engine into territory typically reserved for HPC applications.

Ready for implementation behind feature flags! 🚀