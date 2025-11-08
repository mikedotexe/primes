# M1 Max Specific Performance Guide

## Optimizing for Apple M1 Max

The M1 Max features:
- 32 GPU cores (compute units)
- 400 GB/s memory bandwidth
- 32KB threadgroup memory per CU
- Max 1024 threads per threadgroup

### Expected Performance

On M1 Max with our optimized implementation:
```
Kernel throughput: 186.9M candidates/second
End-to-end: ~60M candidates/second
Power draw: ~25-30W under load
```

### Optimal Parameters for M1 Max

```rust
// Best configuration for M1 Max
const BATCH_SIZE: usize = 4_000_000;  // 4M candidates
const THREADS_PER_THREADGROUP: usize = 1024;  // Maximum
const SIGNATURE_COUNT: usize = 100;  // First 100 primes

// Metal dispatch
let threadgroups = (BATCH_SIZE + 1023) / 1024;  // 3,907
let threads_per_threadgroup = MTLSize { 
    width: 1024, 
    height: 1, 
    depth: 1 
};
```

### Performance Tuning Tips

1. **Batch Size**: M1 Max benefits from large batches
   - 1M: ~150M c/s
   - 4M: ~187M c/s (optimal)
   - 10M: ~185M c/s (diminishing returns)

2. **Threadgroup Size**: Always use 1024
   - 256: ~140M c/s (underutilized)
   - 512: ~165M c/s (better)
   - 1024: ~187M c/s (optimal)

3. **Memory Pressure**: Monitor with `sudo powermetrics`
   ```bash
   sudo powermetrics --samplers gpu_power -i 1000 -n 10
   ```

### M1 Max Specific Optimizations

1. **Unified Memory Advantage**
   ```rust
   // No need for explicit GPU memory allocation
   let buffer = device.new_buffer_with_data(
       data.as_ptr() as *const _,
       data.len() * size_of::<u32>(),
       MTLResourceOptions::StorageModeShared
   );
   ```

2. **Power Efficiency Mode**
   ```rust
   // For battery operation, reduce batch size
   let eco_batch = if on_battery { 1_000_000 } else { 4_000_000 };
   ```

3. **Thermal Management**
   The M1 Max can sustain 186M c/s indefinitely with proper cooling.
   If thermal throttling occurs:
   - Reduce batch size to 2M
   - Add 100ms delay between batches
   - Ensure adequate ventilation

### Verification Commands

Test your M1 Max performance:

```bash
# Quick benchmark
cargo run --release --features metal --bin membrane-prime-gpu-fast \
    -- --gpu --base 6 --count 4000000 --benchmark

# Extended run (thermal test)
cargo run --release --features metal --bin membrane-prime-gpu-fast \
    -- --gpu --base 6 --count 40000000 --benchmark

# Power monitoring during run
sudo powermetrics --samplers gpu_power -i 1000 | grep "GPU Power"
```

### Expected Output on M1 Max
```
=== Membrane Prime Generator (GPU-Optimized) ===
Configuration: Base 6, (5,5), k=(0,0)
Generating 4000000 membrane values...
Membrane generation: 2.7ms (1481.5M values/s)

GPU Sieving...
GPU sieve time: 21.4ms (186.9M candidates/s)
Survivors: 753897 (18.8%)

Running parallel Miller-Rabin on survivors...
Miller-Rabin: 38.2ms

Found 189,234 primes (4.73% of 4000000)
Total time: 62.3ms
Overall throughput: 64.2M candidates/s
```

### Troubleshooting M1 Max Issues

1. **Lower than expected performance**
   - Check Activity Monitor for other GPU usage
   - Ensure Release build: `--release` flag
   - Verify Metal feature: `--features metal`

2. **Thermal throttling**
   - Use `sudo powermetrics` to check GPU frequency
   - Normal: 1296 MHz
   - Throttled: <1000 MHz
   - Solution: Improve cooling or reduce batch size

3. **Memory pressure**
   - M1 Max shares memory with CPU
   - Close memory-heavy applications
   - Monitor with `vm_stat 1`

### Comparison with Other M1 Family

| Chip | GPU Cores | Expected Throughput | Efficiency |
|------|-----------|-------------------|------------|
| M1 | 8 | ~56M c/s | 100% |
| M1 Pro | 16 | ~109M c/s | 97% |
| M1 Max | 32 | ~187M c/s | 83% |
| M1 Ultra | 64 | ~350M c/s* | ~78%* |

*M1 Ultra projected based on scaling

The M1 Max hits a sweet spot for membrane prime generation - enough compute units for massive parallelism while maintaining good efficiency.