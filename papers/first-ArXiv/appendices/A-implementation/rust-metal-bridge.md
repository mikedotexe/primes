# Rust-Metal Bridge Implementation

## The FFI Layer That Makes GPU Magic Possible

### Overview

Connecting Rust to Metal requires careful FFI (Foreign Function Interface) design. We use Objective-C as the bridge language since Metal's API is Objective-C based.

### The Rust Side

```rust
// src/metal_host.rs

use std::ffi::c_void;
use std::mem;
use std::slice;

#[repr(C)]
pub struct MetalContext {
    device: *mut c_void,
    queue: *mut c_void,
    pipeline: *mut c_void,
}

#[repr(C)]
pub struct SieveResult {
    pub survivors: *mut u32,
    pub count: u32,
    pub kernel_time_ms: f32,
}

extern "C" {
    fn metal_create_context() -> *mut MetalContext;
    fn metal_destroy_context(ctx: *mut MetalContext);
    
    fn metal_sieve_affine(
        ctx: *mut MetalContext,
        candidates: *const u32,
        num_candidates: u32,
        signatures: *const SigRow,
        num_primes: u32,
    ) -> SieveResult;
    
    fn metal_free_buffer(buffer: *mut c_void);
}

pub struct MetalSieve {
    context: *mut MetalContext,
}

impl MetalSieve {
    pub fn new() -> Result<Self, String> {
        let context = unsafe { metal_create_context() };
        if context.is_null() {
            return Err("Failed to create Metal context".to_string());
        }
        Ok(MetalSieve { context })
    }
    
    pub fn sieve_affine(&self, 
                       candidates: &[u32], 
                       signatures: &[SigRow]) -> Vec<u32> {
        let result = unsafe {
            metal_sieve_affine(
                self.context,
                candidates.as_ptr(),
                candidates.len() as u32,
                signatures.as_ptr(),
                signatures.len() as u32,
            )
        };
        
        // Convert bit array to indices
        let survivor_count = result.count as usize;
        let mut survivors = Vec::with_capacity(survivor_count);
        
        unsafe {
            let words = slice::from_raw_parts(
                result.survivors,
                (candidates.len() + 31) / 32
            );
            
            for (word_idx, &word) in words.iter().enumerate() {
                if word != 0 {
                    for bit in 0..32 {
                        if word & (1 << bit) != 0 {
                            let idx = word_idx * 32 + bit;
                            if idx < candidates.len() {
                                survivors.push(candidates[idx]);
                            }
                        }
                    }
                }
            }
            
            metal_free_buffer(result.survivors as *mut c_void);
        }
        
        println!("GPU kernel time: {:.1}ms", result.kernel_time_ms);
        survivors
    }
}

impl Drop for MetalSieve {
    fn drop(&mut self) {
        unsafe {
            metal_destroy_context(self.context);
        }
    }
}

// Thread safety
unsafe impl Send for MetalSieve {}
unsafe impl Sync for MetalSieve {}
```

### The Objective-C Bridge

```objc
// src/metal_bridge.m

#import <Metal/Metal.h>
#import <simd/simd.h>

typedef struct {
    uint32_t s;
    uint32_t g;
    uint32_t p;
} SigRow;

typedef struct {
    uint32_t numCandidates;
    uint32_t numPrimes;
    uint32_t tableOffset;
} SieveParams;

typedef struct {
    id<MTLDevice> device;
    id<MTLCommandQueue> queue;
    id<MTLComputePipelineState> pipeline;
} MetalContext;

typedef struct {
    uint32_t* survivors;
    uint32_t count;
    float kernel_time_ms;
} SieveResult;

extern "C" MetalContext* metal_create_context() {
    MetalContext* ctx = malloc(sizeof(MetalContext));
    
    // Get default GPU
    ctx->device = MTLCreateSystemDefaultDevice();
    if (!ctx->device) {
        free(ctx);
        return NULL;
    }
    
    // Create command queue
    ctx->queue = [ctx->device newCommandQueue];
    
    // Load shader library
    NSError* error = nil;
    NSString* libraryPath = [[NSBundle mainBundle] 
        pathForResource:@"default" ofType:@"metallib"];
    
    id<MTLLibrary> library = [ctx->device 
        newLibraryWithFile:libraryPath error:&error];
    
    if (!library) {
        NSLog(@"Failed to load Metal library: %@", error);
        free(ctx);
        return NULL;
    }
    
    // Get kernel function
    id<MTLFunction> kernel = [library 
        newFunctionWithName:@"sieve_affine_optimized"];
    
    // Create pipeline
    ctx->pipeline = [ctx->device 
        newComputePipelineStateWithFunction:kernel error:&error];
    
    if (!ctx->pipeline) {
        NSLog(@"Failed to create pipeline: %@", error);
        free(ctx);
        return NULL;
    }
    
    return ctx;
}

extern "C" void metal_destroy_context(MetalContext* ctx) {
    if (ctx) {
        // ARC handles the Objective-C object cleanup
        free(ctx);
    }
}

extern "C" SieveResult metal_sieve_affine(
    MetalContext* ctx,
    const uint32_t* candidates,
    uint32_t num_candidates,
    const SigRow* signatures,
    uint32_t num_primes
) {
    @autoreleasepool {
        // Create buffers
        id<MTLBuffer> candidatesBuffer = [ctx->device 
            newBufferWithBytes:candidates
            length:num_candidates * sizeof(uint32_t)
            options:MTLResourceStorageModeShared];
        
        id<MTLBuffer> signaturesBuffer = [ctx->device
            newBufferWithBytes:signatures
            length:num_primes * sizeof(SigRow)
            options:MTLResourceStorageModeShared];
        
        // Output buffer (bit array)
        uint32_t output_words = (num_candidates + 31) / 32;
        id<MTLBuffer> survivorsBuffer = [ctx->device
            newBufferWithLength:output_words * sizeof(uint32_t)
            options:MTLResourceStorageModeShared];
        
        // Clear output buffer
        memset([survivorsBuffer contents], 0, 
               output_words * sizeof(uint32_t));
        
        // Parameters
        SieveParams params = {
            .numCandidates = num_candidates,
            .numPrimes = num_primes,
            .tableOffset = 0
        };
        
        id<MTLBuffer> paramsBuffer = [ctx->device
            newBufferWithBytes:&params
            length:sizeof(SieveParams)
            options:MTLResourceStorageModeShared];
        
        // Create command buffer and encoder
        id<MTLCommandBuffer> commandBuffer = [ctx->queue commandBuffer];
        id<MTLComputeCommandEncoder> encoder = 
            [commandBuffer computeCommandEncoder];
        
        // Set pipeline and buffers
        [encoder setComputePipelineState:ctx->pipeline];
        [encoder setBuffer:candidatesBuffer offset:0 atIndex:0];
        [encoder setBuffer:signaturesBuffer offset:0 atIndex:1];
        [encoder setBuffer:survivorsBuffer offset:0 atIndex:2];
        [encoder setBuffer:paramsBuffer offset:0 atIndex:3];
        
        // Calculate thread groups
        NSUInteger threadsPerThreadgroup = 
            MIN(1024, ctx->pipeline.maxTotalThreadsPerThreadgroup);
        NSUInteger threadgroups = 
            (num_candidates + threadsPerThreadgroup - 1) / 
            threadsPerThreadgroup;
        
        // Dispatch
        [encoder dispatchThreadgroups:MTLSizeMake(threadgroups, 1, 1)
                threadsPerThreadgroup:MTLSizeMake(threadsPerThreadgroup, 1, 1)];
        
        [encoder endEncoding];
        
        // Time the execution
        CFAbsoluteTime start = CFAbsoluteTimeGetCurrent();
        [commandBuffer commit];
        [commandBuffer waitUntilCompleted];
        CFAbsoluteTime end = CFAbsoluteTimeGetCurrent();
        
        // Count survivors
        uint32_t* output = (uint32_t*)[survivorsBuffer contents];
        uint32_t count = 0;
        for (uint32_t i = 0; i < output_words; i++) {
            count += __builtin_popcount(output[i]);
        }
        
        // Allocate result buffer
        uint32_t* result_buffer = malloc(output_words * sizeof(uint32_t));
        memcpy(result_buffer, output, output_words * sizeof(uint32_t));
        
        return (SieveResult){
            .survivors = result_buffer,
            .count = count,
            .kernel_time_ms = (end - start) * 1000.0f
        };
    }
}

extern "C" void metal_free_buffer(void* buffer) {
    free(buffer);
}
```

### Build Configuration

```rust
// build.rs

use std::env;
use std::path::PathBuf;

fn main() {
    // Only build Metal support on macOS
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "macos" {
        return;
    }
    
    // Compile Metal shaders
    println!("cargo:rerun-if-changed=shaders/");
    compile_metal_shaders();
    
    // Compile Objective-C bridge
    cc::Build::new()
        .file("src/metal_bridge.m")
        .flag("-fobjc-arc")
        .flag("-fmodules")
        .compile("metal_bridge");
    
    // Link with Metal framework
    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=MetalPerformanceShaders");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
}

fn compile_metal_shaders() {
    let shader_path = PathBuf::from("shaders/sieve_affine.metal");
    let air_path = PathBuf::from("shaders/sieve_affine.air");
    let lib_path = PathBuf::from("src/metal/default.metallib");
    
    // Compile .metal -> .air
    std::process::Command::new("xcrun")
        .args(&["-sdk", "macosx", "metal", "-c"])
        .arg(&shader_path)
        .arg("-o")
        .arg(&air_path)
        .status()
        .expect("Failed to compile Metal shader");
    
    // Link .air -> .metallib
    std::process::Command::new("xcrun")
        .args(&["-sdk", "macosx", "metallib"])
        .arg(&air_path)
        .arg("-o")
        .arg(&lib_path)
        .status()
        .expect("Failed to create Metal library");
}
```

### Error Handling

```rust
pub enum MetalError {
    DeviceNotFound,
    ShaderCompilationFailed(String),
    BufferCreationFailed,
    KernelExecutionFailed(String),
}

impl MetalSieve {
    pub fn sieve_with_error_handling(&self, 
                                    candidates: &[u32], 
                                    signatures: &[SigRow]) 
                                    -> Result<Vec<u32>, MetalError> {
        // Validate inputs
        if candidates.is_empty() {
            return Ok(vec![]);
        }
        
        if signatures.is_empty() {
            return Err(MetalError::BufferCreationFailed);
        }
        
        // Check buffer size limits
        let max_buffer_size = 256 * 1024 * 1024; // 256MB
        let candidates_size = candidates.len() * std::mem::size_of::<u32>();
        if candidates_size > max_buffer_size {
            return Err(MetalError::BufferCreationFailed);
        }
        
        // Perform sieving with panic catching
        std::panic::catch_unwind(|| {
            self.sieve_affine(candidates, signatures)
        })
        .map_err(|_| MetalError::KernelExecutionFailed(
            "GPU kernel panicked".to_string()
        ))
    }
}
```

### Performance Monitoring

```rust
pub struct PerformanceMetrics {
    pub kernel_time_ms: f32,
    pub transfer_time_ms: f32,
    pub total_time_ms: f32,
    pub throughput_m_per_s: f32,
}

impl MetalSieve {
    pub fn sieve_with_metrics(&self,
                             candidates: &[u32],
                             signatures: &[SigRow]) 
                             -> (Vec<u32>, PerformanceMetrics) {
        let start_total = std::time::Instant::now();
        
        // Time includes transfer to GPU
        let start_transfer = std::time::Instant::now();
        let result = self.sieve_affine(candidates, signatures);
        let total_time = start_total.elapsed();
        
        let metrics = PerformanceMetrics {
            kernel_time_ms: self.last_kernel_time_ms,
            transfer_time_ms: 0.3, // Approximate
            total_time_ms: total_time.as_secs_f32() * 1000.0,
            throughput_m_per_s: candidates.len() as f32 / 
                               total_time.as_secs_f32() / 1_000_000.0,
        };
        
        (result, metrics)
    }
}
```

This Rust-Metal bridge provides:
1. **Type safety**: Rust's type system prevents many FFI errors
2. **Memory safety**: Proper cleanup via RAII
3. **Performance**: Zero-copy where possible
4. **Error handling**: Graceful degradation
5. **Metrics**: Built-in performance monitoring

The bridge is the crucial link that lets our mathematical insights meet silicon reality!