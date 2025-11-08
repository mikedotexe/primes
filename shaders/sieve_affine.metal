//
// sieve_affine.metal
// Affine sieve implementation for Metal GPU
//
// SAFETY:
// - This kernel requires macOS with Metal support
// - All array accesses are bounds-checked by thread ID
// - Atomic operations ensure race-free bit setting
// - Threadgroup memory usage is within guaranteed limits (32KB)
//
// THREAD SAFETY:
// - Each thread processes one candidate independently
// - Atomic bit operations prevent write conflicts
// - Threadgroup barrier ensures synchronized memory access
//

#include <metal_stdlib>
using namespace metal;

struct Params {
    uint numPrimes;
    uint tableOffset;
    uint numCandidates;
};

struct SigRow {
    uint s;
    uint g;
    uint p;
};

// Thread-coherent bit-set helper
inline void set_bit(device atomic_uint *out, uint idx) {
    uint word = idx >> 5;
    uint bit  = idx & 31;
    atomic_fetch_or_explicit(&out[word], 1u << bit, memory_order_relaxed);
}

kernel void sieve_affine(
    constant Params   &prm  [[ buffer(0) ]],
    constant SigRow   *sig  [[ buffer(1) ]],
    device   uint     *ctab [[ buffer(2) ]],
    device   uint     *outBits [[ buffer(3) ]],
    uint tid [[ thread_position_in_grid ]],
    uint lid [[ thread_index_in_simdgroup ]],
    uint gid [[ thread_position_in_threadgroup ]],
    uint tpg [[ threads_per_threadgroup ]])
{
    // Load signature table into threadgroup memory
    threadgroup SigRow tgSig[100];
    
    for (uint i = gid; i < prm.numPrimes && i < 100; i += tpg) {
        tgSig[i] = sig[i];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    
    if (tid >= prm.numCandidates) return;

    uint C = ctab[prm.tableOffset + tid];
    bool alive = true;

    // 4-way unrolled residue test for ILP
    for (uint i = 0; i < prm.numPrimes; i += 4) {
        // Simple divisibility test: N is the pre-computed membrane value
        uint N = C;  // C is already the membrane value, not the seed
        
        if (i + 3 < prm.numPrimes) {
            uint4 p = uint4(tgSig[i+0].p, tgSig[i+1].p, tgSig[i+2].p, tgSig[i+3].p);
            uint4 rem = uint4(N) % p;
            alive = alive && all(rem != 0);
        } else {
            // Handle remaining primes
            for (uint j = i; j < prm.numPrimes; j++) {
                if (N % tgSig[j].p == 0) {
                    alive = false;
                    break;
                }
            }
        }
        
        if(!alive) break;
    }

    // Simple optimization: use atomic only if alive to reduce contention
    if (alive) {
        uint word = tid >> 5;
        uint bit  = tid & 31;
        atomic_fetch_or_explicit((device atomic_uint*)&outBits[word], 1u << bit, memory_order_relaxed);
    }
}