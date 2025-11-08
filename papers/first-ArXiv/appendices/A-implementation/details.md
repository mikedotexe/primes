# Appendix A: Implementation Details

## Metal Shader Code

Complete GPU kernel for affine sieving:

```metal
kernel void sieve_affine(
    device const uint* candidates [[buffer(0)]],
    device const SigRow* signatures [[buffer(1)]],
    device atomic_uint* survivors [[buffer(2)]],
    constant SieveParams& params [[buffer(3)]],
    uint tid [[thread_position_in_grid]],
    uint tpg [[threads_per_threadgroup]],
    uint gid [[threadgroup_position_in_grid]]
) {
    if (tid >= params.numCandidates) return;
    
    // Load signatures into threadgroup memory
    threadgroup SigRow tgSig[100];
    for (uint i = gid; i < params.numPrimes && i < 100; i += tpg) {
        tgSig[i] = signatures[i];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    
    // Affine divisibility test
    uint C = candidates[tid];
    bool alive = true;
    
    for (uint i = 0; i < params.numPrimes && alive; i++) {
        uint residue = (tgSig[i].s + C * tgSig[i].g) % tgSig[i].p;
        alive = (residue != 0);
    }
    
    // Atomic write survivors
    if (alive) {
        uint word = tid / 32;
        uint bit = tid % 32;
        atomic_fetch_or(&survivors[word], 1u << bit);
    }
}
```

[Stub: Add Rust FFI bridge, CPU implementation, optimization progression]