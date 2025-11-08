import Foundation
import Metal
import MetalPerformanceShaders

/// Metal-accelerated membrane prime sieve
public class MetalMembraneSieve {
    private let device: MTLDevice
    private let commandQueue: MTLCommandQueue
    private let library: MTLLibrary
    private let sievePipeline: MTLComputePipelineState
    private let optimizedPipeline: MTLComputePipelineState
    private let instrumentedPipeline: MTLComputePipelineState
    
    /// Membrane configuration
    public struct Config {
        let base: UInt32
        let width: UInt32
        let lDigit: UInt32
        let rDigit: UInt32
        let r1: UInt32
        let r2: UInt32
        
        public init(base: UInt32 = 12, width: UInt32 = 3, 
                    lDigit: UInt32 = 1, rDigit: UInt32 = 1,
                    r1: UInt32 = 0, r2: UInt32 = 0) {
            self.base = base
            self.width = width
            self.lDigit = lDigit
            self.rDigit = rDigit
            self.r1 = r1
            self.r2 = r2
        }
    }
    
    /// Performance metrics
    public struct Metrics {
        let candidatesTested: Int
        let survivorsFound: Int
        let elapsedTime: TimeInterval
        let throughput: Double  // candidates/sec
        let survivalRate: Double
        let cacheMisses: Int?
        let coalescedLoads: Int?
    }
    
    public init?() {
        guard let device = MTLCreateSystemDefaultDevice() else {
            print("Metal is not supported on this device")
            return nil
        }
        
        self.device = device
        
        guard let queue = device.makeCommandQueue() else {
            print("Failed to create command queue")
            return nil
        }
        self.commandQueue = queue
        
        // Load shader library
        guard let library = device.makeDefaultLibrary() else {
            print("Failed to load Metal library")
            return nil
        }
        self.library = library
        
        // Create compute pipelines
        do {
            let sieveFunction = library.makeFunction(name: "membrane_sieve")!
            sievePipeline = try device.makeComputePipelineState(function: sieveFunction)
            
            let optimizedFunction = library.makeFunction(name: "membrane_sieve_base6_optimized")!
            optimizedPipeline = try device.makeComputePipelineState(function: optimizedFunction)
            
            let instrumentedFunction = library.makeFunction(name: "membrane_sieve_instrumented")!
            instrumentedPipeline = try device.makeComputePipelineState(function: instrumentedFunction)
        } catch {
            print("Failed to create pipeline state: \(error)")
            return nil
        }
    }
    
    /// Run the membrane sieve on GPU
    public func sieve(candidates: [UInt32], config: Config, instrumented: Bool = false) -> (survivors: [UInt32], metrics: Metrics) {
        let startTime = Date()
        
        // Create buffers
        let candidateBuffer = device.makeBuffer(bytes: candidates, 
                                              length: candidates.count * MemoryLayout<UInt32>.stride,
                                              options: .storageModeShared)!
        
        let maxSurvivors = candidates.count  // Worst case: all survive
        let survivorBuffer = device.makeBuffer(length: maxSurvivors * MemoryLayout<UInt32>.stride,
                                             options: .storageModeShared)!
        
        let counterBuffer = device.makeBuffer(bytes: [UInt32(0)], 
                                            length: MemoryLayout<UInt32>.stride,
                                            options: .storageModeShared)!
        
        // Optional instrumentation buffers
        let cacheMissBuffer = instrumented ? device.makeBuffer(bytes: [UInt32(0)], 
                                                             length: MemoryLayout<UInt32>.stride,
                                                             options: .storageModeShared) : nil
        let coalescedBuffer = instrumented ? device.makeBuffer(bytes: [UInt32(0)], 
                                                             length: MemoryLayout<UInt32>.stride,
                                                             options: .storageModeShared) : nil
        
        // Encode compute command
        let commandBuffer = commandQueue.makeCommandBuffer()!
        let computeEncoder = commandBuffer.makeComputeCommandEncoder()!
        
        // Choose pipeline
        let pipeline = instrumented ? instrumentedPipeline : 
                      (config.base == 6 ? optimizedPipeline : sievePipeline)
        computeEncoder.setComputePipelineState(pipeline)
        
        // Set buffers
        computeEncoder.setBuffer(candidateBuffer, offset: 0, index: 0)
        computeEncoder.setBuffer(survivorBuffer, offset: 0, index: 1)
        computeEncoder.setBuffer(counterBuffer, offset: 0, index: 2)
        
        // Set config
        var metalConfig = config
        computeEncoder.setBytes(&metalConfig, length: MemoryLayout<Config>.stride, index: 3)
        
        if instrumented {
            computeEncoder.setBuffer(cacheMissBuffer, offset: 0, index: 4)
            computeEncoder.setBuffer(coalescedBuffer, offset: 0, index: 5)
        }
        
        // Dispatch threads
        let threadsPerThreadgroup = MTLSize(width: pipeline.maxTotalThreadsPerThreadgroup, height: 1, depth: 1)
        let threadgroupsPerGrid = MTLSize(width: (candidates.count + threadsPerThreadgroup.width - 1) / threadsPerThreadgroup.width,
                                         height: 1, depth: 1)
        
        computeEncoder.dispatchThreadgroups(threadgroupsPerGrid, threadsPerThreadgroup: threadsPerThreadgroup)
        computeEncoder.endEncoding()
        
        // Execute and wait
        commandBuffer.commit()
        commandBuffer.waitUntilCompleted()
        
        // Read results
        let counterPtr = counterBuffer.contents().bindMemory(to: UInt32.self, capacity: 1)
        let survivorCount = Int(counterPtr[0])
        
        let survivorPtr = survivorBuffer.contents().bindMemory(to: UInt32.self, capacity: survivorCount)
        let survivorIndices = Array(UnsafeBufferPointer(start: survivorPtr, count: survivorCount))
        
        // Map indices back to candidate values
        let survivors = survivorIndices.map { candidates[Int($0)] }
        
        // Calculate metrics
        let elapsedTime = Date().timeIntervalSince(startTime)
        let throughput = Double(candidates.count) / elapsedTime
        let survivalRate = Double(survivorCount) / Double(candidates.count)
        
        var cacheMisses: Int? = nil
        var coalescedLoads: Int? = nil
        
        if instrumented {
            let missPtr = cacheMissBuffer!.contents().bindMemory(to: UInt32.self, capacity: 1)
            cacheMisses = Int(missPtr[0])
            
            let coalescedPtr = coalescedBuffer!.contents().bindMemory(to: UInt32.self, capacity: 1)
            coalescedLoads = Int(coalescedPtr[0])
        }
        
        let metrics = Metrics(
            candidatesTested: candidates.count,
            survivorsFound: survivorCount,
            elapsedTime: elapsedTime,
            throughput: throughput,
            survivalRate: survivalRate,
            cacheMisses: cacheMisses,
            coalescedLoads: coalescedLoads
        )
        
        return (survivors, metrics)
    }
    
    /// Pack base-6 or base-12 digits into 4-bit nibbles for optimized access
    public func packDigits(_ values: [UInt32], base: UInt32) -> [UInt32] {
        let digitsPerUInt: Int = 8  // 32 bits / 4 bits per digit
        let packedCount = (values.count + digitsPerUInt - 1) / digitsPerUInt
        var packed = [UInt32](repeating: 0, count: packedCount)
        
        for (i, value) in values.enumerated() {
            let wordIdx = i / digitsPerUInt
            let digitIdx = i % digitsPerUInt
            let shift = digitIdx * 4
            
            // Ensure value fits in 4 bits
            let clampedValue = min(value, 15)
            packed[wordIdx] |= (clampedValue << shift)
        }
        
        return packed
    }
}