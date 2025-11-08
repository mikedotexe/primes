# GPU Memory Hierarchy Explained

## The Memory Pyramid

```
┌─────────────────────────────────┐
│      Registers (4 bytes)        │ <- 1 cycle
│   Per thread, ultra-fast        │
├─────────────────────────────────┤
│  Threadgroup Memory (32 KB)     │ <- ~10 cycles  
│  Shared within threadgroup      │
├─────────────────────────────────┤
│   L2 Cache (8 MB on M2)        │ <- ~50 cycles
│    Shared across GPU           │
├─────────────────────────────────┤
│  Global Memory (up to 96 GB)   │ <- ~500 cycles
│   Main GPU memory (unified)    │
└─────────────────────────────────┘
```

## Access Time Comparison

Imagine registers are like items in your pocket (instant access), threadgroup memory is like a whiteboard in your classroom (quick look), and global memory is like going to the library (long walk).

```
Operation                          Cycles    Time (at 1.4 GHz)
-----------------------------------------------------------------
Read from register                 1         0.7 ns
Read from threadgroup memory       10        7 ns  
Read from L2 cache                 50        35 ns
Read from global memory            500       350 ns
```

## Our Signature Table Journey

### Before Optimization: Global Memory Thrashing
```
Each thread does:
for (i = 0; i < 100; i++) {
    read signatures[i] from global  // 350ns × 100 = 35,000 ns
}

Total: 1024 threads × 100 reads = 102,400 global reads
Time: 35 microseconds of just memory latency!
```

### After Optimization: Threadgroup Memory Bliss
```
Phase 1 - Cooperative Load (100 ns total):
Thread 0:   Load signatures[0]
Thread 1:   Load signatures[1]
...
Thread 99:  Load signatures[99]
Threads 100-1023: Wait

Phase 2 - Everyone Uses Shared Memory:
All threads: Read from threadgroup memory // 7ns × 100 = 700 ns

Total: 100 global reads + (1024 × 100 threadgroup reads)
Time: 35ns + 700ns = 735 ns (47x faster!)
```

## Memory Coalescing Magic

GPUs read memory in 128-byte chunks. When threads access consecutive addresses, magic happens:

### Bad Pattern (Random Access):
```
Thread 0: Read candidates[1000]  ┐
Thread 1: Read candidates[47]    ├─ 32 separate memory transactions!
Thread 2: Read candidates[823]   │
...                              ┘
```

### Good Pattern (Sequential Access):
```
Thread 0: Read candidates[0]  ┐
Thread 1: Read candidates[1]  ├─ 1 coalesced memory transaction!
Thread 2: Read candidates[2]  │  (All 32 values in one 128-byte read)
...                          ┘
```

Our implementation ensures perfect coalescing:
```metal
uint C = candidates[params.tableOffset + tid];  // tid is sequential!
```

## The Atomic Bottleneck Solution

### Problem: Atomic Contention
When many threads try to update the same memory location:
```
Thread 0: atomic_or(&word, bit0)  ┐
Thread 1: atomic_or(&word, bit1)  ├─ Serialized! Each waits for previous
Thread 2: atomic_or(&word, bit2)  │
...                               ┘
```

### Solution: SIMD Ballot
```
Threads 0-31: Calculate locally
SIMD Hardware: Combine all 32 results into one ballot
Thread 0 only: atomic_or(&word, ballot)  // 32x fewer atomics!
```

## Bandwidth Calculation

Let's calculate the actual memory bandwidth:

```
Per candidate:
- Read: 4 bytes (uint candidate)
- Write: 0.125 bytes (1 bit in output)
- Signatures: 1.2 KB / 1024 threads = 1.17 bytes amortized

Total per candidate: 5.3 bytes

At 186.6M candidates/sec:
Bandwidth = 186.6M × 5.3 = 989 MB/s

M2 Max Memory Bandwidth: 400 GB/s
Utilization: 0.25% (!!)
```

We're compute-bound, not memory-bound - perfect GPU utilization!

## Register Allocation

Each thread needs:
```
uint C           // 1 register - our candidate
bool alive       // 1 register - our result
uint i           // 1 register - loop counter
uint residue     // 1 register - temporary

Total: 4 registers per thread
M2 GPU: 256 registers available per thread
Usage: 1.6% - plenty of room!
```

## The Critical Insight

The GPU has a **memory hierarchy** just like a CPU, but the ratios are different:

- CPU: Optimizes for **latency** (make one thread fast)
- GPU: Optimizes for **throughput** (make 30,000 threads fast in aggregate)

By understanding and exploiting each level of the hierarchy:
1. **Registers**: Keep loop variables and results
2. **Threadgroup**: Share the signature table
3. **Coalescing**: Access candidates sequentially  
4. **Atomics**: Minimize with SIMD ballot

We transform a memory-bound problem into a compute-bound one, allowing the GPU to reach its full potential of 186.6M candidates per second!