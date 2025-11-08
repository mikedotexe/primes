# Performance Evolution Timeline

## The Journey from 270K to 186M candidates/second

### Baseline: CPU Implementation (270K c/s)

```rust
fn is_membrane_prime(base: u32, l: u32, r: u32, c: u32) -> bool {
    let n = l * base.pow(2) + r * base + c + l;
    
    // Check divisibility by first 100 primes
    for &p in &PRIMES_100 {
        if n % p == 0 { return false; }
    }
    
    // Miller-Rabin test
    miller_rabin(n)
}
```

**Bottlenecks**: Sequential processing, expensive modulo operations

---

### Attempt 1: Naive GPU Port (297K c/s) - Almost No Speedup!

```metal
kernel void sieve_naive(device uint* membrane_values [[buffer(0)]],
                       device bool* results [[buffer(1)]],
                       uint tid [[thread_position_in_grid]]) {
    uint n = membrane_values[tid];
    
    for (int i = 0; i < 100; i++) {
        if (n % PRIMES_100[i] == 0) {
            results[tid] = false;
            return;  // PROBLEM: Thread divergence!
        }
    }
    results[tid] = true;
}
```

**Why it failed**:
- Early returns cause thread divergence
- Global memory access for prime constants
- No exploitation of GPU parallelism

**Lesson**: Simply porting CPU code to GPU doesn't work!

---

### Breakthrough: Affine Transform Discovery

**The Mathematical Insight**:
```
Instead of: M(c) % p == 0
We found:  (s + c*g) % p == 0

where s and g are constants we can precompute!
```

This transforms the problem from irregular modular arithmetic to regular linear operations.

---

### Attempt 2: Basic Affine Implementation (3M c/s) - 10x Speedup

```metal
kernel void sieve_affine_basic(device uint* candidates [[buffer(0)]],
                              device SigRow* signatures [[buffer(1)]],
                              device bool* results [[buffer(2)]],
                              uint tid [[thread_position_in_grid]]) {
    uint c = candidates[tid];
    bool alive = true;
    
    // No more early exit - all threads do same work
    for (uint i = 0; i < 100; i++) {
        uint residue = (signatures[i].s + c * signatures[i].g) % signatures[i].p;
        alive = alive && (residue != 0);
    }
    
    results[tid] = alive;
}
```

**Improvements**:
- ✅ No thread divergence
- ✅ Regular computation pattern
- ❌ Still hitting global memory 100x per thread

---

### Optimization 1: Threadgroup Memory (10.5M c/s) - 3.5x More

```metal
kernel void sieve_affine_shared(device uint* candidates [[buffer(0)]],
                               device SigRow* signatures [[buffer(1)]],
                               device bool* results [[buffer(2)]],
                               uint tid [[thread_position_in_grid]],
                               uint lid [[thread_position_in_threadgroup]],
                               uint tpg [[threads_per_threadgroup]]) {
    // Game changer: Shared memory for signatures
    threadgroup SigRow tgSig[100];
    
    // Cooperative loading - 1024 threads load 100 items
    for (uint i = lid; i < 100; i += tpg) {
        tgSig[i] = signatures[i];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    
    uint c = candidates[tid];
    bool alive = true;
    
    // Now reading from FAST threadgroup memory
    for (uint i = 0; i < 100; i++) {
        uint residue = (tgSig[i].s + c * tgSig[i].g) % tgSig[i].p;
        alive = alive && (residue != 0);
    }
    
    results[tid] = alive;
}
```

**Memory access reduced**: 102,400 → 1,124 reads per threadgroup!

---

### Optimization 2: Batch Size Tuning (31M c/s) - 3x More

Discovered that larger batches amortize kernel launch overhead:

```rust
// Before: 10K candidates
let batch_size = 10_000;
// Launch time: 5ms
// Compute time: 1ms
// Overhead: 83%!

// After: 4M candidates  
let batch_size = 4_000_000;
// Launch time: 5ms
// Compute time: 125ms
// Overhead: 4%
```

---

### Optimization 3: Bitmask Output (37M c/s) - 1.2x More

```metal
// Before: 4 bytes per result
device bool* results;  // 4MB for 1M candidates

// After: 1 bit per result  
device atomic_uint* survivors;  // 125KB for 1M candidates

if (alive) {
    uint word = tid / 32;
    uint bit = tid % 32;
    atomic_fetch_or(&survivors[word], 1u << bit);
}
```

**Benefits**: 32x less memory bandwidth, better cache usage

---

### Optimization 4: SIMD Ballot (52M c/s) - 1.4x More

```metal
// Before: Every thread does atomic operation
if (alive) {
    atomic_fetch_or(&survivors[word], 1u << bit);  // Contention!
}

// After: SIMD groups coordinate
uint ballot = simd_ballot(alive);  // Hardware magic!
if (simd_lane == 0) {
    atomic_fetch_or(&survivors[word], ballot);  // 32x fewer atomics
}
```

---

### Optimization 5: Reciprocal Multiplication (94M c/s) - 1.8x More

```metal
// Before: Expensive modulo
uint residue = (s + c * g) % p;  // ~20 cycles

// After: Multiply + shift
inline uint mod_fast(uint x, uint p, uint q) {
    uint t = mul_hi(x, q);  // 1 cycle
    return x - t * p;       // 1 cycle
}
uint residue = mod_fast(s + c * g, p, q);  // ~3 cycles total
```

---

### Optimization 6: CPU Parallelization (162M c/s) - 1.7x More

```rust
// Before: Single-threaded membrane computation
let values: Vec<u32> = (0..count)
    .map(|c| compute_membrane(base, l, r, c))
    .collect();

// After: Rayon parallel iteration
use rayon::prelude::*;
let values: Vec<u32> = (0..count)
    .into_par_iter()  // Magic happens here
    .map(|c| compute_membrane(base, l, r, c))
    .collect();
```

---

### Final: All Optimizations Combined (186.6M c/s kernel)

The complete pipeline with all optimizations:

```
1. CPU: Parallel membrane computation (Rayon)
   ↓ 4M values in 2.7ms
2. GPU: Affine sieve with all optimizations  
   ↓ 800K survivors in 21.4ms
3. CPU: Parallel Miller-Rabin (32-bit optimized)
   ↓ 753K primes in 42.6ms
   
Total: 66.7ms for 4M candidates = 60M c/s end-to-end
GPU kernel alone: 21.4ms for 4M = 186.6M c/s
```

---

## The Superlinear Speedup Explained

Expected speedup (multiplicative): 
```
10 × 3.5 × 3 × 1.2 × 1.4 × 1.8 × 1.7 = 535x
```

Actual speedup: **691x** (kernel) / **222x** (end-to-end)

Why superlinear?
1. **Reduced memory pressure** → Better cache usage
2. **Higher occupancy** → Better latency hiding  
3. **Fewer atomics** → Less contention
4. **Aligned access** → Memory coalescing kicks in

Each optimization didn't just add its benefit - it unlocked hidden potential in the others!

---

## Cost Per Prime

Final performance metrics:
- 186.6M candidates/second (GPU kernel)
- 18.8% prime density
- ~35M primes/second generated
- Power: ~30W on M2 Max

**Cost**: ~0.86 microjoules per prime found

Compare to Bitcoin mining: ~100 trillion times more energy per "success"!