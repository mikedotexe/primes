//! Metal Compute Shader for Membrane Prime Sieving on Apple Silicon
//! Optimized for M1/M2/M3 GPUs

#include <metal_stdlib>
using namespace metal;

// Constants passed via buffer
struct SieveParams {
    uint32_t num_primes;
    uint32_t batch_size;
    uint32_t w_half;        // w/2 for growth factor
};

// Pre-computed signature components
struct SignatureData {
    uint32_t signature;     // S_p value
    uint32_t growth;        // G_p value  
    uint32_t prime;         // The prime p itself
};

// Optimized modular multiplication to avoid 64-bit ops where possible
inline uint32_t mod_mul(uint32_t a, uint32_t b, uint32_t p) {
    // For small primes, we can use 64-bit to avoid overflow
    if (p < 65536) {
        return (uint64_t(a) * uint64_t(b)) % p;
    }
    // For larger primes, use iterative addition (slower but safe)
    uint32_t result = 0;
    a %= p;
    while (b > 0) {
        if (b & 1) {
            result = (result + a) % p;
        }
        a = (a * 2) % p;
        b >>= 1;
    }
    return result;
}

// Main sieving kernel - each thread handles one C value
kernel void membrane_sieve(
    constant SieveParams& params [[buffer(0)]],
    constant SignatureData* signatures [[buffer(1)]],
    device atomic_uint* survivor_count [[buffer(2)]],
    device uint64_t* survivors [[buffer(3)]],
    uint3 gid [[thread_position_in_grid]],
    uint3 tid [[thread_position_in_threadgroup]],
    uint3 tg_size [[threads_per_threadgroup]]
) {
    // Calculate C value for this thread
    uint64_t C = uint64_t(gid.x);
    
    // Shared memory for coalesced prime data access
    threadgroup SignatureData shared_sigs[256];
    
    // Load signatures into shared memory (coalesced)
    uint tid_flat = tid.x;
    uint tg_flat = tg_size.x;
    
    for (uint i = tid_flat; i < params.num_primes && i < 256; i += tg_flat) {
        shared_sigs[i] = signatures[i];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    
    // Check against all primes
    bool is_candidate = true;
    
    // Process small primes from shared memory
    uint shared_limit = min(params.num_primes, 256u);
    for (uint i = 0; i < shared_limit; i++) {
        uint32_t p = shared_sigs[i].prime;
        uint32_t c_mod_p = C % p;
        uint32_t val = (shared_sigs[i].signature + 
                       mod_mul(c_mod_p, shared_sigs[i].growth, p)) % p;
        
        if (val == 0) {
            is_candidate = false;
            break;
        }
    }
    
    // Process remaining primes from global memory if needed
    if (is_candidate && params.num_primes > 256) {
        for (uint i = 256; i < params.num_primes; i++) {
            uint32_t p = signatures[i].prime;
            uint32_t c_mod_p = C % p;
            uint32_t val = (signatures[i].signature + 
                           mod_mul(c_mod_p, signatures[i].growth, p)) % p;
            
            if (val == 0) {
                is_candidate = false;
                break;
            }
        }
    }
    
    // Store survivors using atomic counter
    if (is_candidate) {
        uint idx = atomic_fetch_add_explicit(survivor_count, 1, memory_order_relaxed);
        if (idx < params.batch_size / 4) { // Safety limit
            survivors[idx] = C;
        }
    }
}

// Optimized vectorized kernel from AI friend
// Process 32 C values per thread with improved memory access
kernel void membrane_sieve_vectorized(
    constant SieveParams&   params      [[ buffer(0) ]],
    constant SignatureData* sigs        [[ buffer(1) ]],
    device   uint*          outBits     [[ buffer(2) ]],
    uint     gid            [[ thread_position_in_grid ]]) {

    uint wordIdx = gid;                     // each thread handles 32 Cs
    uint32_t mask = 0;

    uint Cbase = params.batch_size * (wordIdx >> 5) + (wordIdx << 5);
    
    for (uint lane = 0; lane < 32; ++lane) {
        uint C = Cbase + lane;
        bool ok = true;

        #pragma unroll(4)
        for (uint i = 0; i < params.num_primes && ok; i++) {
            uint s = sigs[i].signature;
            uint g = sigs[i].growth;
            uint p = sigs[i].prime;
            
            // More efficient: (s + (C mod p)*g) mod p
            uint val = s + (C % p) * g;
            ok = (val % p) != 0;
        }
        if (ok) mask |= 1u << lane;
    }
    outBits[wordIdx] = mask;
}

// Kernel for phase 2: Miller-Rabin primality testing on GPU
kernel void miller_rabin_test(
    device uint64_t* candidates [[buffer(0)]],
    device uint32_t* results [[buffer(1)]],  // 1 = prime, 0 = composite
    constant uint32_t& num_candidates [[buffer(2)]],
    uint gid [[thread_position_in_grid]]
) {
    if (gid >= num_candidates) return;
    
    uint64_t n = candidates[gid];
    
    // Simplified Miller-Rabin for demonstration
    // In production, would need full implementation
    bool is_prime = true;
    
    // Test with first few primes as witnesses
    uint32_t witnesses[3] = {2, 3, 5};
    
    for (uint i = 0; i < 3; i++) {
        // Simplified test - full implementation needed
        if (n % witnesses[i] == 0) {
            is_prime = (n == witnesses[i]);
            break;
        }
    }
    
    results[gid] = is_prime ? 1 : 0;
}