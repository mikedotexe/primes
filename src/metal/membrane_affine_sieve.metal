//
// membrane_affine_sieve.metal
// Optimized GPU membrane sieve using affine residue patterns
//

#include <metal_stdlib>
using namespace metal;

// Configuration passed to kernel
struct Params {
    uint numPrimes;        // Number of small primes to test against
    uint candidateOffset;  // Starting index in candidate table
    uint base;            // Number base (6 or 12)
    uint width;           // Membrane width
    uint lDigit;          // Left boundary digit
    uint rDigit;          // Right boundary digit
    uint r1;              // Left zero padding
    uint r2;              // Right zero padding
};

// Packed signature row for affine test
// IMPORTANT: Keep 16-byte aligned as recommended
struct SigRow {
    uint s;    // signature
    uint g;    // generator
    uint p;    // prime
    uint pad;  // padding for 16-byte alignment
};

// Output: 256-bit atomic mask (8 x 32-bit words)
struct CandidateOutput {
    atomic_uint mask[8];
};

// Fast modular exponentiation for membrane computation
inline uint fast_mod_pow(uint base, uint exp, uint mod) {
    uint result = 1;
    uint b = base % mod;
    
    // Unroll for common small exponents
    switch(exp) {
        case 0: return 1;
        case 1: return b;
        case 2: return (b * b) % mod;
        case 3: return (b * b % mod * b) % mod;
        default:
            while (exp > 0) {
                if (exp & 1) result = (result * b) % mod;
                b = (b * b) % mod;
                exp >>= 1;
            }
            return result;
    }
}

// Compute membrane value modulo p
inline uint membrane_mod(uint c, uint base, uint w, uint l, uint r, 
                        uint r1, uint r2, uint p) {
    // Precompute powers of base mod p
    uint b1 = fast_mod_pow(base, w - 1, p);
    uint b2 = fast_mod_pow(base, w - 2 - r1, p);
    uint b3 = fast_mod_pow(base, w / 2, p);
    uint b4 = fast_mod_pow(base, r2 + 1, p);
    
    // Membrane formula: L*b^(w-1) + R*b^(w-2-r1) + C*b^(w/2) + R*b^(r2+1) + L
    uint result = 0;
    result = (result + l * b1) % p;
    result = (result + r * b2) % p;
    result = (result + c * b3) % p;
    result = (result + r * b4) % p;
    result = (result + l) % p;
    
    return result;
}

// Optimized affine sieve kernel
kernel void membrane_affine_sieve(
    constant Params&        params      [[ buffer(0) ]],
    constant SigRow*        sigTable    [[ buffer(1) ]],
    device   uint*          candidates  [[ buffer(2) ]],
    device   CandidateOutput* output    [[ buffer(3) ]],
    uint3    gid   [[ thread_position_in_grid ]],
    uint3    lid   [[ thread_position_in_threadgroup ]],
    uint3    tgSize [[ threads_per_threadgroup ]])
{
    // Each thread processes one candidate
    uint idx = params.candidateOffset + gid.x;
    uint c = candidates[idx];
    
    // Extract membrane params for readability
    uint base = params.base;
    uint w = params.width;
    uint l = params.lDigit;
    uint r = params.rDigit;
    uint r1 = params.r1;
    uint r2 = params.r2;
    
    // Start with candidate alive
    uint alive = 1;
    
    // Process primes in groups of 4 for ILP (Instruction Level Parallelism)
    uint numGroups = (params.numPrimes + 3) / 4;
    
    for (uint group = 0; group < numGroups && alive; group++) {
        uint baseIdx = group * 4;
        
        // Load 4 signature rows at once (16-byte aligned)
        SigRow sig0 = sigTable[baseIdx];
        SigRow sig1 = sigTable[baseIdx + 1];
        SigRow sig2 = sigTable[baseIdx + 2];
        SigRow sig3 = sigTable[baseIdx + 3];
        
        // Compute membrane value mod each prime
        uint v0 = membrane_mod(c, base, w, l, r, r1, r2, sig0.p);
        uint v1 = membrane_mod(c, base, w, l, r, r1, r2, sig1.p);
        uint v2 = membrane_mod(c, base, w, l, r, r1, r2, sig2.p);
        uint v3 = membrane_mod(c, base, w, l, r, r1, r2, sig3.p);
        
        // Apply affine test: (s + v*g) % p != 0
        uint test0 = (sig0.s + v0 * sig0.g) % sig0.p;
        uint test1 = (sig1.s + v1 * sig1.g) % sig1.p;
        uint test2 = (sig2.s + v2 * sig2.g) % sig2.p;
        uint test3 = (sig3.s + v3 * sig3.g) % sig3.p;
        
        // Check if any test failed (early exit)
        alive &= (test0 != 0) & (test1 != 0) & (test2 != 0) & (test3 != 0);
    }
    
    // Write result as single bit in 256-bit atomic mask
    uint wordIdx = lid.x >> 5;  // / 32
    uint bitIdx = lid.x & 31;   // % 32
    
    if (alive) {
        // Atomic OR to set our bit
        atomic_fetch_or_explicit(&output[gid.x >> 8].mask[wordIdx],
                               1u << bitIdx, 
                               memory_order_relaxed);
    }
}

// Base-6 optimized variant with packed nibbles
kernel void membrane_affine_base6_packed(
    constant Params&        params      [[ buffer(0) ]],
    constant SigRow*        sigTable    [[ buffer(1) ]],
    device   uint*          packedCands [[ buffer(2) ]],
    device   CandidateOutput* output    [[ buffer(3) ]],
    threadgroup uint*       sharedMem   [[ threadgroup(0) ]],
    uint3    gid   [[ thread_position_in_grid ]],
    uint3    lid   [[ thread_position_in_threadgroup ]],
    uint3    tgSize [[ threads_per_threadgroup ]])
{
    // Each thread processes 8 candidates (one packed uint)
    uint packedIdx = gid.x;
    uint packed = packedCands[packedIdx];
    
    // Process each 4-bit digit
    for (uint digit = 0; digit < 8; digit++) {
        uint c = (packed >> (digit * 4)) & 0xF;
        
        // Skip invalid base-6 digits
        if (c >= 6) continue;
        
        // Use shared memory for intermediate results
        uint localIdx = lid.x * 8 + digit;
        sharedMem[localIdx] = 1;  // Start alive
        
        // Process primes
        for (uint i = 0; i < params.numPrimes; i++) {
            SigRow sig = sigTable[i];
            uint v = membrane_mod(c, 6, params.width, params.lDigit, 
                                params.rDigit, params.r1, params.r2, sig.p);
            uint test = (sig.s + v * sig.g) % sig.p;
            
            if (test == 0) {
                sharedMem[localIdx] = 0;
                break;
            }
        }
        
        // Sync before writing output
        threadgroup_barrier(mem_flags::mem_threadgroup);
        
        // Write survivors to output mask
        if (sharedMem[localIdx]) {
            uint globalIdx = packedIdx * 8 + digit;
            uint wordIdx = (globalIdx % 256) >> 5;
            uint bitIdx = globalIdx & 31;
            
            atomic_fetch_or_explicit(&output[globalIdx >> 8].mask[wordIdx],
                                   1u << bitIdx,
                                   memory_order_relaxed);
        }
    }
}

// Performance monitoring kernel
kernel void membrane_affine_instrumented(
    constant Params&        params      [[ buffer(0) ]],
    constant SigRow*        sigTable    [[ buffer(1) ]],
    device   uint*          candidates  [[ buffer(2) ]],
    device   CandidateOutput* output    [[ buffer(3) ]],
    device   atomic_uint*   perfCounters [[ buffer(4) ]], // [coalesced, cache_miss, cycles]
    uint3    gid   [[ thread_position_in_grid ]],
    uint3    lid   [[ thread_position_in_threadgroup ]],
    uint3    tgSize [[ threads_per_threadgroup ]])
{
    // Simulate performance counting
    uint startCycle = lid.x;  // Pseudo cycle counter
    
    // Check for coalesced access
    if (lid.x == 0 || gid.x == 0 || candidates[gid.x] == candidates[gid.x-1] + 1) {
        atomic_fetch_add_explicit(&perfCounters[0], 1, memory_order_relaxed);
    } else {
        atomic_fetch_add_explicit(&perfCounters[1], 1, memory_order_relaxed);
    }
    
    // Run normal sieve
    uint idx = params.candidateOffset + gid.x;
    uint c = candidates[idx];
    uint alive = 1;
    
    // Simplified test loop for instrumentation
    for (uint i = 0; i < min(params.numPrimes, 16u); i++) {
        SigRow sig = sigTable[i];
        uint v = membrane_mod(c, params.base, params.width, params.lDigit,
                            params.rDigit, params.r1, params.r2, sig.p);
        if ((sig.s + v * sig.g) % sig.p == 0) {
            alive = 0;
            break;
        }
    }
    
    // Write result
    if (alive) {
        uint wordIdx = lid.x >> 5;
        uint bitIdx = lid.x & 31;
        atomic_fetch_or_explicit(&output[gid.x >> 8].mask[wordIdx],
                               1u << bitIdx, memory_order_relaxed);
    }
    
    // Record pseudo cycle count
    uint endCycle = lid.x + params.numPrimes;
    atomic_fetch_add_explicit(&perfCounters[2], endCycle - startCycle, memory_order_relaxed);
}