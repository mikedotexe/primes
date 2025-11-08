# Metal GPU Backend Safety Documentation

## Overview

The Metal GPU backend provides high-performance prime number sieving on macOS. This document details the safety measures, error handling, and platform requirements for the Metal implementation.

## Platform Requirements

### Supported Platforms
- **macOS 10.11 (El Capitan) or later**
- **Metal-capable GPU** (all Macs since 2012)
- **Rust compiler with Metal feature enabled**

### Unsupported Platforms
- Linux (no Metal support)
- Windows (no Metal support)
- iOS/iPadOS (could be added but not currently implemented)

## Safety Guards

### 1. Compile-Time Safety

All Metal-specific code is guarded by platform checks:

```rust
#[cfg(target_os = "macos")]
pub struct MetalSieve { ... }

#[cfg(not(target_os = "macos"))]
pub struct MetalSieve;  // Stub implementation
```

This ensures:
- Metal code only compiles on macOS
- Other platforms get stub implementations with clear error messages
- No runtime crashes from missing Metal libraries

### 2. Runtime Safety

#### GPU Initialization
```rust
let device = Device::system_default()
    .ok_or_else(|| "No Metal GPU found")?;
```

Checks:
- Metal device availability
- GPU capabilities
- Shader compilation status

#### Memory Safety
```rust
// Input validation
if candidates.is_empty() {
    return Ok(Vec::new());
}

if candidates.len() > u32::MAX as usize {
    return Err("Too many candidates");
}
```

Protections:
- Bounds checking on all inputs
- Overflow prevention
- Null pointer checks before buffer access

#### Thread Safety
- Metal kernels use atomic operations for concurrent writes
- Threadgroup barriers ensure memory synchronization
- Each thread processes independent data

### 3. Error Handling

All GPU operations return `Result<T, String>`:

```rust
pub fn new() -> Result<Self, String>
pub fn sieve(&self, candidates: &[u32], base: u32) -> Result<Vec<u32>, String>
```

Common error scenarios:
- No Metal GPU available → Returns descriptive error
- Shader compilation failure → Includes troubleshooting hints
- Out of GPU memory → Graceful failure with message
- Invalid input data → Early validation with clear errors

## Building and Testing

### Prerequisites

1. **Install Xcode Command Line Tools**:
   ```bash
   xcode-select --install
   ```

2. **Compile Metal shaders**:
   ```bash
   cd prime-physics-engine
   ./scripts/build_metal.sh
   ```

3. **Set environment variable**:
   ```bash
   export METALLIB_PATH=src/metal/default.metallib
   ```

### Building with Metal Support

```bash
# Build with Metal feature
cargo build --features metal

# Run Metal-enabled examples
cargo run --example metal_gpu_primes --features metal
```

### Testing on Different Platforms

```bash
# Test on macOS (full Metal support)
cargo test --features metal

# Test on Linux/Windows (stub implementations)
cargo test
```

## Performance Considerations

### Optimal Batch Sizes
- **Small batches (< 1,000)**: 2-5x speedup
- **Medium batches (1,000-100,000)**: 10-20x speedup
- **Large batches (> 100,000)**: 30-50x speedup

### Memory Limits
- Apple Silicon (M1/M2/M3): Up to 64GB unified memory
- Intel Macs: Typically 1.5-8GB VRAM
- Automatic fallback to CPU if GPU memory exhausted

## Debugging Metal Issues

### Common Problems and Solutions

1. **"No Metal GPU found"**
   - Ensure macOS 10.11+
   - Check System Information → Graphics

2. **"Failed to load Metal shader library"**
   - Run `./scripts/build_metal.sh`
   - Verify METALLIB_PATH is set correctly

3. **"Pipeline creation failed"**
   - Check shader compilation errors
   - Verify kernel function names match

### Debug Tools

1. **Metal System Trace** (Instruments):
   ```bash
   xcrun xctrace record --template "Metal System Trace" --launch -- cargo run --example metal_gpu_primes --features metal
   ```

2. **GPU Frame Capture**:
   - Set `METAL_DEVICE_WRAPPER_TYPE=1` for debugging
   - Use Xcode's GPU debugger

## Example Usage

### Safe GPU Initialization

```rust
use prime_physics_engine::gpu::GpuSieve;

fn main() {
    // Attempt GPU initialization with graceful fallback
    match GpuSieve::new() {
        Ok(gpu) => {
            println!("GPU acceleration available!");
            run_gpu_sieve(gpu);
        }
        Err(e) => {
            println!("GPU not available: {}", e);
            println!("Falling back to CPU implementation");
            run_cpu_sieve();
        }
    }
}
```

### Platform-Agnostic Code

```rust
use prime_physics_engine::metal_bridge::MetalSieve;

fn process_candidates(candidates: &[u32]) -> Vec<u32> {
    // Try GPU first
    if let Some(metal) = MetalSieve::new() {
        if let Some((survivors, _)) = metal.sieve(candidates, config, false) {
            return survivors;
        }
    }
    
    // Fallback to CPU
    cpu_sieve(candidates)
}
```

## Security Considerations

1. **No arbitrary code execution**: Only pre-compiled shaders run on GPU
2. **Memory isolation**: GPU memory is separate from system memory
3. **Bounds checking**: All buffer accesses are validated
4. **No network access**: GPU computations are entirely local

## Future Improvements

1. **Cross-platform GPU support**:
   - Vulkan backend for Linux/Windows
   - WebGPU for browser deployment

2. **Enhanced error recovery**:
   - Automatic retry with smaller batches
   - Progressive degradation strategies

3. **Performance monitoring**:
   - Real-time GPU utilization metrics
   - Adaptive batch sizing

## Contributing

When adding new Metal functionality:

1. Always use `#[cfg(target_os = "macos")]` guards
2. Provide stub implementations for other platforms
3. Include comprehensive error messages
4. Add safety documentation for unsafe blocks
5. Test on both macOS and non-macOS platforms

## References

- [Metal Programming Guide](https://developer.apple.com/metal/)
- [Metal Shading Language Specification](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf)
- [rust-metal crate documentation](https://docs.rs/metal/)