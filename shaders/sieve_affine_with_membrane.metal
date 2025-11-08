#include <metal_stdlib>
using namespace metal;

struct Params {
    uint numPrimes;
    uint numCandidates;
    uint base;
    uint l;      // left boundary
    uint r;      // right boundary  
    uint width;
};

struct SigRow {
    uint s;
    uint g;
    uint p;
};

// Compute membrane value directly on GPU
inline uint compute_membrane(uint c, uint base, uint width, uint l, uint r) {
    // For width=3: L*b² + R*b + C*b + R*b + L
    uint b2 = base * base;
    return l * b2 + r * base + c * base + r * base + l;
}

kernel void sieve_affine_with_membrane(
    constant Params   &prm  [[ buffer(0) ]],
    constant SigRow   *sig  [[ buffer(1) ]],
    device   uint     *outBits [[ buffer(2) ]],
    uint tid [[ thread_position_in_grid ]],
    uint lid [[ thread_index_in_simdgroup ]],
    uint gid [[ thread_position_in_threadgroup ]],
    uint tpg [[ threads_per_threadgroup ]])
{
    if (tid >= prm.numCandidates) return;
    
    // NEW: Compute membrane value directly instead of reading from buffer
    uint N = compute_membrane(tid, prm.base, prm.width, prm.l, prm.r);
    
    // Load signature table into threadgroup memory (unchanged)
    threadgroup SigRow tgSig[100];
    for (uint i = gid; i < prm.numPrimes && i < 100; i += tpg) {
        tgSig[i] = sig[i];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    
    // Sieve test (unchanged)
    bool alive = true;
    for (uint i = 0; i < prm.numPrimes; i += 4) {
        if (i + 3 < prm.numPrimes) {
            uint4 p = uint4(tgSig[i+0].p, tgSig[i+1].p, tgSig[i+2].p, tgSig[i+3].p);
            uint4 rem = uint4(N) % p;
            alive = alive && all(rem != 0);
        } else {
            for (uint j = i; j < prm.numPrimes; j++) {
                if (N % tgSig[j].p == 0) {
                    alive = false;
                    break;
                }
            }
        }
        if(!alive) break;
    }
    
    // Output (unchanged)
    if (alive) {
        uint word = tid >> 5;
        uint bit  = tid & 31;
        atomic_fetch_or_explicit((device atomic_uint*)&outBits[word], 1u << bit, memory_order_relaxed);
    }
}

// Optional: Fermat pre-test on GPU
inline bool fermat_test_32(uint n, uint base) {
    if (n <= 1 || (n % 2 == 0 && n != 2)) return false;
    if (n <= 3) return true;
    
    // Compute base^(n-1) mod n
    uint result = 1;
    uint exp = n - 1;
    uint b = base % n;
    
    while (exp > 0) {
        if (exp & 1) {
            result = ((ulong)result * b) % n;
        }
        b = ((ulong)b * b) % n;
        exp >>= 1;
    }
    
    return result == 1;
}

kernel void sieve_with_fermat(
    constant Params   &prm  [[ buffer(0) ]],
    constant SigRow   *sig  [[ buffer(1) ]],
    device   uint     *candidates [[ buffer(2) ]],
    device   uint     *survivors [[ buffer(3) ]],
    device   atomic_uint *counter [[ buffer(4) ]],
    uint tid [[ thread_position_in_grid ]])
{
    if (tid >= prm.numCandidates) return;
    
    uint N = candidates[tid];
    
    // First do affine sieve (reuse logic from above)
    bool passed_sieve = true;
    // ... sieve test ...
    
    if (passed_sieve) {
        // Then do Fermat test
        if (fermat_test_32(N, 2)) {
            // Atomically add to survivors list
            uint idx = atomic_fetch_add_explicit(counter, 1, memory_order_relaxed);
            survivors[idx] = N;
        }
    }
}