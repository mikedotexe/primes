#include <metal_stdlib>
using namespace metal;

struct Params {
    uint numPrimes;
    uint numCandidates;
    uint base;
    uint l;      // left boundary
    uint r;      // right boundary  
    uint width;  // membrane width
};

struct SigRowRecip {
    uint s;  // signature
    uint g;  // generator
    uint p;  // prime
    uint q;  // reciprocal
};

// Fast modulo using reciprocal multiplication
inline uint mod_fast(uint x, uint p, uint q) {
    // mul_hi: upper 32 bits of 32×32→64
    ulong prod = (ulong)x * (ulong)q;
    uint t = prod >> 32;
    return x - t * p;  // Result in [0, 2p)
}

// Compute membrane value directly on GPU
inline uint membrane(uint c, constant Params& prm) {
    uint b2 = prm.base * prm.base;
    return prm.l * b2 +
           prm.r * prm.base +
           c * prm.base +      // w=3 → b¹
           prm.r * prm.base +
           prm.l;
}

// Fast 32-bit modular exponentiation for Fermat test
inline uint mod_pow32(uint base, uint exp, uint mod) {
    uint result = 1;
    base %= mod;
    
    while (exp > 0) {
        if (exp & 1) {
            result = ((ulong)result * base) % mod;
        }
        base = ((ulong)base * base) % mod;
        exp >>= 1;
    }
    return result;
}

kernel void sieve_optimized(
    constant Params &prm [[ buffer(0) ]],
    constant SigRowRecip *sig [[ buffer(1) ]],
    device uint *outBits [[ buffer(2) ]],
    uint tid [[ thread_position_in_grid ]],
    uint lid [[ thread_index_in_simdgroup ]],
    uint gid [[ thread_position_in_threadgroup ]],
    uint tpg [[ threads_per_threadgroup ]])
{
    if (tid >= prm.numCandidates) return;
    
    // 1. Compute membrane value directly (no memory read)
    uint N = membrane(tid, prm);
    
    // 2. Load signature table into threadgroup memory
    threadgroup SigRowRecip tgSig[100];
    for (uint i = gid; i < prm.numPrimes && i < 100; i += tpg) {
        tgSig[i] = sig[i];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    
    // 3. Affine sieve with reciprocal multiplication
    bool alive = true;
    
    for (uint i = 0; i < prm.numPrimes && alive; i += 4) {
        if (i + 3 < prm.numPrimes) {
            // 4-way unrolled with fast modulo
            uint4 s = uint4(tgSig[i+0].s, tgSig[i+1].s, tgSig[i+2].s, tgSig[i+3].s);
            uint4 g = uint4(tgSig[i+0].g, tgSig[i+1].g, tgSig[i+2].g, tgSig[i+3].g);
            uint4 p = uint4(tgSig[i+0].p, tgSig[i+1].p, tgSig[i+2].p, tgSig[i+3].p);
            uint4 q = uint4(tgSig[i+0].q, tgSig[i+1].q, tgSig[i+2].q, tgSig[i+3].q);
            
            // Fast modulo for N mod p
            uint4 n_mod_p = uint4(
                mod_fast(N, p.x, q.x),
                mod_fast(N, p.y, q.y),
                mod_fast(N, p.z, q.z),
                mod_fast(N, p.w, q.w)
            );
            
            // Affine test: (s + n_mod_p * g) % p
            uint4 val = (s + n_mod_p * g) % p;  // p is small, % compiles to mad24
            alive = alive && all(val != 0);
        } else {
            // Handle remaining primes
            for (uint j = i; j < prm.numPrimes; j++) {
                uint n_mod_p = mod_fast(N, tgSig[j].p, tgSig[j].q);
                if ((tgSig[j].s + n_mod_p * tgSig[j].g) % tgSig[j].p == 0) {
                    alive = false;
                    break;
                }
            }
        }
    }
    
    // 4. Fermat test base-2 (optional but recommended)
    if (alive && N > 2) {
        // Quick compositeness check: 2^(N-1) ≡ 1 (mod N)
        alive = (mod_pow32(2, N - 1, N) == 1);
    }
    
    // 5. Output survivors
    if (alive) {
        uint word = tid >> 5;
        uint bit = tid & 31;
        atomic_fetch_or_explicit((device atomic_uint*)&outBits[word], 1u << bit, memory_order_relaxed);
    }
}

// Alternative: SIMD-ballot version for newer Metal
kernel void sieve_optimized_ballot(
    constant Params &prm [[ buffer(0) ]],
    constant SigRowRecip *sig [[ buffer(1) ]],
    device uint *outBits [[ buffer(2) ]],
    uint tid [[ thread_position_in_grid ]],
    uint lid [[ thread_index_in_simdgroup ]],
    uint gid [[ thread_position_in_threadgroup ]],
    uint tpg [[ threads_per_threadgroup ]])
{
    if (tid >= prm.numCandidates) return;
    
    uint N = membrane(tid, prm);
    
    // ... same sieve logic ...
    bool alive = true;
    // ... affine sieve + Fermat ...
    
    // SIMD ballot write (if supported)
    uint ballot = simd_ballot(alive);
    if (lid == 0 && ballot) {
        uint word = (tid >> 5);
        outBits[word] = ballot;  // Non-atomic write
    }
}