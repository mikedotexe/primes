# GPU Implementation Troubleshooting Guide

## Common Pitfalls and How to Avoid Them

### 1. The "GPU is Slower" Paradox

**Symptom**: Your GPU version runs slower than CPU

**Common Causes**:
```metal
// PITFALL 1: Thread divergence
if (n % prime == 0) {
    return;  // Some threads exit early, others continue
}

// FIX: All threads complete all work
alive = alive && (n % prime != 0);
```

```metal
// PITFALL 2: Global memory thrashing
for (i = 0; i < 100; i++) {
    p = global_primes[i];  // 100 global reads per thread!
}

// FIX: Use threadgroup memory
threadgroup uint primes[100];
// ... cooperative load ...
p = primes[i];  // 100x faster!
```

```rust
// PITFALL 3: Tiny batches
let batch = 1000;  // GPU spends 90% time on overhead!

// FIX: Larger batches
let batch = 4_000_000;  // Amortize launch cost
```

### 2. Wrong Answer Bugs

**Symptom**: GPU gives different results than CPU

**Common Causes**:
```metal
// BUG 1: Forgetting bounds check
kernel void sieve(uint tid [[thread_position_in_grid]]) {
    uint value = data[tid];  // What if tid >= data.size?
}

// FIX: Always check
if (tid >= num_elements) return;
```

```metal
// BUG 2: Race condition
threadgroup float sum = 0;
sum += my_value;  // Multiple threads writing!

// FIX: Use atomics or reduction
atomic_fetch_add(&sum, my_value);
```

```metal
// BUG 3: Missing barrier
if (lid == 0) shared_data[0] = result;
float value = shared_data[0];  // Might read before write!

// FIX: Synchronize
if (lid == 0) shared_data[0] = result;
threadgroup_barrier(mem_flags::mem_threadgroup);
float value = shared_data[0];
```

### 3. Performance Mysteries

**Symptom**: Not getting expected speedup

**Diagnosis Tools**:

1. **Profile kernel time vs total time**:
```rust
let start = Instant::now();
gpu_sieve(&candidates);  
let gpu_time = start.elapsed();

let start = Instant::now();
// Include memory transfers
let values = compute_membranes();
gpu_sieve(&values);
let survivors = read_results();
let total_time = start.elapsed();

println!("GPU kernel: {:?}, Total: {:?}", gpu_time, total_time);
```

2. **Check occupancy**:
```metal
// Too many registers reduces occupancy
float array[100];  // 400 bytes of registers!

// Better: Use threadgroup memory
threadgroup float array[100];
```

3. **Memory access pattern**:
```metal
// Bad: Strided access
uint value = data[tid * stride];  // Cache misses!

// Good: Sequential access  
uint value = data[tid];  // Coalesced!
```

### 4. The Affine Transform Confusion

**Q: Why does (s + c*g) % p work?**

Think of it like this:
```
Original membrane: M(c) = 5*6² + 5*6 + c + 5
                        = 180 + 30 + c + 5
                        = 215 + c

For prime p=7:
M(0) % 7 = 215 % 7 = 5  (this is s)
M(1) % 7 = 216 % 7 = 6
M(2) % 7 = 217 % 7 = 0  (divisible!)

Pattern: Each increment of c adds 1 (mod 7)
So M(c) ≡ 5 + c*1 (mod 7)
       ≡ s + c*g (mod 7)  where s=5, g=1
```

### 5. Metal-Specific Gotchas

**Buffer Alignment**:
```rust
// Wrong: Odd-sized structure
#[repr(C)]
struct SigRow {
    s: u32,
    g: u32,
    p: u16,  // Total: 10 bytes (not aligned!)
}

// Right: Pad to multiple of 4
#[repr(C)]
struct SigRow {
    s: u32,
    g: u32, 
    p: u32,  // Total: 12 bytes (aligned!)
}
```

**Function Constants**:
```metal
// Wrong: Array in kernel
kernel void sieve(...) {
    uint primes[100] = {2,3,5,7...};  // Allocated per thread!
}

// Right: Constant address space
constant uint primes[100] = {2,3,5,7...};  // Shared, cached
```

### 6. Debugging Techniques

**Printf Debugging** (Yes, it works in Metal!):
```metal
#include <metal_stdlib>
using namespace metal;

kernel void debug_kernel(device uint* data [[buffer(0)]],
                        uint tid [[thread_position_in_grid]]) {
    if (tid == 0) {  // Limit output!
        printf("First value: %u\n", data[0]);
    }
}
```

**Binary Search for Bugs**:
```metal
// Simplify until it works, then add back
kernel void test(device uint* out [[buffer(0)]],
                uint tid [[thread_position_in_grid]]) {
    // Step 1: Just write thread ID
    out[tid] = tid;
    
    // Step 2: Add computation
    // out[tid] = tid * 2;
    
    // Step 3: Add memory access
    // out[tid] = signatures[0].s;
    
    // etc...
}
```

### 7. The "It Should Be Faster" Checklist

When your GPU kernel isn't as fast as expected:

- [ ] **Threadgroup size**: Using 1024 (maximum)?
- [ ] **Threadgroup memory**: Signatures loaded once?
- [ ] **Memory coalescing**: Sequential access pattern?
- [ ] **Atomics minimized**: Using SIMD ballot?
- [ ] **Batch size**: At least 1M candidates?
- [ ] **Compute bound**: Not waiting on memory?
- [ ] **Occupancy**: All SMs busy?
- [ ] **Register pressure**: Under 32 registers/thread?
- [ ] **Bank conflicts**: Stride-1 access to threadgroup memory?
- [ ] **CPU side**: Parallel membrane computation?

### 8. Understanding the Full Pipeline

Remember the data flow:

```
1. CPU generates seed values (0, 1, 2, ...)
   ↓
2. CPU computes membrane polynomials M(c) in parallel
   ↓
3. GPU receives array of M(c) values
   ↓
4. GPU tests each M(c) against 100 primes using affine method
   ↓
5. GPU outputs bitmask of survivors
   ↓
6. CPU reads survivors and runs Miller-Rabin
   ↓
7. Verified primes!
```

The genius is that each component plays to its strengths:
- CPU: Complex branchy logic (Miller-Rabin)
- GPU: Simple parallel arithmetic (affine sieving)

### Final Wisdom

The GPU is not magic - it's a very specific tool that excels when:
1. You have thousands of independent computations
2. Each computation follows the same control flow
3. Memory access is predictable and coalesced
4. You minimize synchronization and atomics

The membrane + affine transform creates exactly these conditions. That's why a mathematical insight (linear patterns in residue space) combined with engineering insight (GPU architecture) produces a 1000x speedup.

When debugging, always ask: "Am I fighting the hardware or flowing with it?"