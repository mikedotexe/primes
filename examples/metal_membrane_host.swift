#!/usr/bin/env swift
//! Host program for Metal GPU membrane sieving on Apple Silicon

import Metal
import MetalKit
import Foundation

// Configuration for membrane polynomial
struct MembraneConfig {
    let L: UInt32
    let R: UInt32
    let w: UInt32
    let r1: UInt32
    let r2: UInt32
    let base: UInt32 = 10
}

// Match the Metal shader structure
struct SieveParams {
    let numPrimes: UInt32
    let batchSize: UInt32
    let wHalf: UInt32
}

struct SignatureData {
    let signature: UInt32
    let growth: UInt32
    let prime: UInt32
}

class MetalMembraneSieve {
    let device: MTLDevice
    let commandQueue: MTLCommandQueue
    let sieveKernel: MTLComputePipelineState
    let vectorizedKernel: MTLComputePipelineState
    
    init() throws {
        // Get the default GPU
        guard let device = MTLCreateSystemDefaultDevice() else {
            throw NSError(domain: "MetalMembrane", code: 1, 
                         userInfo: [NSLocalizedDescriptionKey: "Metal is not supported"])
        }
        self.device = device
        
        guard let queue = device.makeCommandQueue() else {
            throw NSError(domain: "MetalMembrane", code: 2,
                         userInfo: [NSLocalizedDescriptionKey: "Cannot create command queue"])
        }
        self.commandQueue = queue
        
        // Load the shader library
        let libraryPath = "src/metal_membrane_sieve.metal"
        let source = try String(contentsOfFile: libraryPath)
        
        let library = try device.makeLibrary(source: source, options: nil)
        
        // Create compute pipelines
        guard let sieveFunction = library.makeFunction(name: "membrane_sieve"),
              let vectorFunction = library.makeFunction(name: "membrane_sieve_vectorized") else {
            throw NSError(domain: "MetalMembrane", code: 3,
                         userInfo: [NSLocalizedDescriptionKey: "Cannot find kernel functions"])
        }
        
        self.sieveKernel = try device.makeComputePipelineState(function: sieveFunction)
        self.vectorizedKernel = try device.makeComputePipelineState(function: vectorFunction)
        
        print("Metal device: \(device.name)")
        print("Max threads per threadgroup: \(sieveKernel.maxTotalThreadsPerThreadgroup)")
    }
    
    // Generate small primes using sieve of Eratosthenes
    func generatePrimes(upTo limit: Int) -> [UInt32] {
        var isPrime = Array(repeating: true, count: limit)
        isPrime[0] = false
        isPrime[1] = false
        
        for i in 2..<Int(sqrt(Double(limit))) + 1 {
            if isPrime[i] {
                for j in stride(from: i*i, to: limit, by: i) {
                    isPrime[j] = false
                }
            }
        }
        
        return isPrime.enumerated().compactMap { $1 ? UInt32($0) : nil }
    }
    
    // Modular exponentiation
    func modPow(_ base: UInt32, _ exp: UInt32, _ mod: UInt32) -> UInt32 {
        var result: UInt64 = 1
        var base = UInt64(base)
        var exp = exp
        let mod = UInt64(mod)
        
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result * base) % mod
            }
            base = (base * base) % mod
            exp >>= 1
        }
        
        return UInt32(result)
    }
    
    // Pre-compute signatures for the membrane configuration
    func computeSignatures(config: MembraneConfig, primes: [UInt32]) -> [SignatureData] {
        return primes.map { p in
            let sig1 = (config.L * (modPow(config.base, config.w - 1, p) + 1)) % p
            let sig2 = (config.R * (modPow(config.base, config.w - 2 - config.r1, p) + 
                                   modPow(config.base, config.r2 + 1, p))) % p
            let signature = (sig1 + sig2) % p
            let growth = modPow(config.base, config.w / 2, p)
            
            return SignatureData(signature: signature, growth: growth, prime: p)
        }
    }
    
    // Run the vectorized sieve
    func sieveBatch(config: MembraneConfig, 
                    startC: UInt64, 
                    batchSize: UInt32,
                    primeLimit: UInt32 = 10000) -> [UInt64] {
        
        let startTime = Date()
        
        // Generate primes and signatures
        let primes = generatePrimes(upTo: Int(primeLimit))
        let signatures = computeSignatures(config: config, primes: primes)
        
        print("Using \(primes.count) primes for sieving")
        
        // Create buffers
        let params = SieveParams(
            numPrimes: UInt32(primes.count),
            batchSize: batchSize,
            wHalf: config.w / 2
        )
        
        guard let paramsBuffer = device.makeBuffer(
            bytes: [params], 
            length: MemoryLayout<SieveParams>.size,
            options: .storageModeShared
        ) else { return [] }
        
        guard let sigBuffer = device.makeBuffer(
            bytes: signatures,
            length: MemoryLayout<SignatureData>.size * signatures.count,
            options: .storageModeShared
        ) else { return [] }
        
        // Results buffer - bit packed, 32 candidates per uint32
        let resultCount = (batchSize + 31) / 32
        guard let resultBuffer = device.makeBuffer(
            length: MemoryLayout<UInt32>.size * Int(resultCount),
            options: .storageModeShared
        ) else { return [] }
        
        // Create command buffer and encoder
        guard let commandBuffer = commandQueue.makeCommandBuffer(),
              let encoder = commandBuffer.makeComputeCommandEncoder() else { return [] }
        
        encoder.setComputePipelineState(vectorizedKernel)
        encoder.setBuffer(paramsBuffer, offset: 0, index: 0)
        encoder.setBuffer(sigBuffer, offset: 0, index: 1)
        encoder.setBuffer(resultBuffer, offset: 0, index: 2)
        
        // Calculate thread groups
        let threadsPerGroup = vectorizedKernel.maxTotalThreadsPerThreadgroup
        let numGroups = (Int(resultCount) + threadsPerGroup - 1) / threadsPerGroup
        
        encoder.dispatchThreadgroups(
            MTLSize(width: numGroups, height: 1, depth: 1),
            threadsPerThreadgroup: MTLSize(width: threadsPerGroup, height: 1, depth: 1)
        )
        
        encoder.endEncoding()
        commandBuffer.commit()
        commandBuffer.waitUntilCompleted()
        
        // Extract results
        let resultPtr = resultBuffer.contents().bindMemory(
            to: UInt32.self, 
            capacity: Int(resultCount)
        )
        
        var survivors: [UInt64] = []
        for i in 0..<Int(resultCount) {
            let bits = resultPtr[i]
            for bit in 0..<32 {
                if bits & (1 << bit) != 0 {
                    let C = startC + UInt64(i * 32 + bit)
                    if C < startC + UInt64(batchSize) {
                        survivors.append(C)
                    }
                }
            }
        }
        
        let elapsed = Date().timeIntervalSince(startTime)
        let rate = Double(batchSize) / elapsed
        
        print("Sieved \(batchSize) candidates in \(elapsed)s")
        print("Rate: \(String(format: "%.2e", rate)) candidates/second")
        print("Survivors: \(survivors.count) (\(String(format: "%.1f", Double(survivors.count) / Double(batchSize) * 100))%)")
        
        return survivors
    }
}

// Membrane value calculation
func membraneValue(L: UInt64, R: UInt64, C: UInt64, r1: UInt64, r2: UInt64, w: UInt64, b: UInt64) -> String {
    // For large numbers, we'll just show the structure
    if w > 20 {
        return "L=\(L), R=\(R), C=\(C), w=\(w) → ~\(w)-digit number"
    }
    
    // For smaller w, calculate actual value
    let val = L * UInt64(pow(Double(b), Double(w-1))) +
              R * UInt64(pow(Double(b), Double(w-2-r1))) +
              C * UInt64(pow(Double(b), Double(w/2))) +
              R * UInt64(pow(Double(b), Double(r2+1))) +
              L
    
    return String(val)
}

// Main execution
do {
    print("Metal Membrane Sieve on Apple Silicon")
    print("=====================================\n")
    
    let sieve = try MetalMembraneSieve()
    
    // Test configuration - matching the verified example
    let config = MembraneConfig(L: 3, R: 7, w: 10, r1: 1, r2: 2)
    
    print("Configuration: L=\(config.L), R=\(config.R), w=\(config.w), r1=\(config.r1), r2=\(config.r2)\n")
    
    // Small test first
    print("1. SMALL BATCH TEST (Verifying correctness)")
    print("--------------------------------------------")
    let smallSurvivors = sieve.sieveBatch(
        config: config,
        startC: 0,
        batchSize: 1000,
        primeLimit: 1000
    )
    
    print("\nFirst 10 survivors:")
    for C in smallSurvivors.prefix(10) {
        let value = membraneValue(
            L: UInt64(config.L), R: UInt64(config.R), C: C,
            r1: UInt64(config.r1), r2: UInt64(config.r2),
            w: UInt64(config.w), b: UInt64(config.base)
        )
        print("  C=\(C): \(value)")
    }
    
    // Large batch test
    print("\n\n2. LARGE BATCH TEST (Performance)")
    print("----------------------------------")
    let largeSurvivors = sieve.sieveBatch(
        config: config,
        startC: 1_000_000,
        batchSize: 10_000_000,
        primeLimit: 50000
    )
    
    // Test with higher degree polynomial
    print("\n\n3. HIGH DEGREE POLYNOMIAL TEST")
    print("------------------------------")
    let largeConfig = MembraneConfig(L: 3, R: 7, w: 100, r1: 10, r2: 20)
    print("Configuration: L=\(largeConfig.L), R=\(largeConfig.R), w=\(largeConfig.w)")
    print("Target: ~100-digit primes\n")
    
    let largeDegree = sieve.sieveBatch(
        config: largeConfig,
        startC: 0,
        batchSize: 1_000_000,
        primeLimit: 100000
    )
    
    print("\nSample survivors that would yield ~100-digit primes:")
    for C in largeDegree.prefix(5) {
        print("  C=\(C)")
    }
    
} catch {
    print("Error: \(error)")
}

// Compile and run with:
// swiftc -O metal_membrane_host.swift -framework Metal -framework MetalKit
// ./metal_membrane_host