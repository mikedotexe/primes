# Quick Start Guide

Welcome to the Prime Physics Engine! This guide will get you up and running in minutes.

## Installation

### From Source

```bash
git clone https://github.com/mikepurvis/prime-physics-engine
cd prime-physics-engine
cargo build --release
```

### From crates.io

```bash
cargo install prime-physics-engine --version ^1.0.0-rc
```

## Basic Usage

### 1. Generate Your First Membrane Prime

```rust
use prime_physics_engine::{
    membrane::MembraneConfig,
    is_prime_miller_rabin,
};

// Use the champion configuration
let config = MembraneConfig::new(6, 1, 5, 0, 0);

// Generate and test numbers
for seed in 0..100 {
    let number = generate_membrane_number(&config, seed);
    if is_prime_miller_rabin(&number) {
        println!("Found prime: {}", number);
    }
}
```

### 2. Run Interactive Examples

```bash
# Educational introduction
cargo run --example educational_explorer

# Real-time parameter tuning
cargo run --example membrane_lab_tui --features visualization

# Performance benchmarking
cargo run --example comprehensive_benchmark
```

## 🛠 WASM Demo

Build and run the WebAssembly demo to use Prime Physics Engine in your browser!

### Prerequisites

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Node.js dependencies
cd wasm-demo
npm install
```

### Build WASM Module

```bash
# Build the WASM package
wasm-pack build --target web --out-dir wasm-demo/pkg

# Optimize for size (optional)
wasm-opt -O3 wasm-demo/pkg/prime_physics_engine_bg.wasm -o wasm-demo/pkg/prime_physics_engine_bg.wasm
```

### Run Development Server

```bash
cd wasm-demo
npm run serve
```

Open http://localhost:8080 in your browser to see the demo!

### WASM API Example

```javascript
import init, { MembraneConfig, generate_primes_wasm } from './pkg/prime_physics_engine.js';

async function run() {
    await init();
    
    // Create configuration
    const config = new MembraneConfig(6, 1, 5, 0, 0);
    
    // Generate primes
    const primes = generate_primes_wasm(config, 1000);
    console.log(`Found ${primes.length} primes!`);
}

run();
```

## GPU Acceleration (macOS)

Enable GPU acceleration for massive performance gains:

```bash
# Build with Metal support
cargo build --release --features metal

# Run GPU example
cargo run --example metal_gpu_primes --features metal
```

## Configuration Guide

### Understanding Membrane Parameters

- **Base**: Number system (6, 10, 12, 30 work well)
- **Outer/Inner**: Boundary digits (must be coprime to base)
- **K values**: Zero padding (usually 0,0 for best results)

### Top Configurations

| Base | Config | Success Rate | Use Case |
|------|--------|--------------|----------|
| 6 | (1,5) k=(0,0) | 33% | Best overall |
| 30 | (11,7) k=(0,0) | 30% | Large primes |
| 12 | (1,5) k=(0,0) | 28% | Balanced |

## Performance Tips

1. **Use segmented sieve** for checking many candidates
2. **Enable multi-threading** with `--features parallel`
3. **GPU acceleration** provides 10-50x speedup
4. **WASM is optimized** for client-side computation

## Troubleshooting

### Common Issues

**Build fails with Metal errors**
```bash
# Build without Metal support
cargo build --no-default-features
```

**WASM module too large**
```bash
# Use release mode and optimization
wasm-pack build --release
wasm-opt -O3 -o optimized.wasm original.wasm
```

**Performance lower than expected**
- Ensure you're using release mode: `--release`
- Check CPU governor: use performance mode
- Try different configurations for your use case

## Next Steps

- Explore the [examples directory](../examples/README.md)
- Read the [architecture guide](ARCH.md)
- Check out [performance tuning](../docs/technical/PERFORMANCE.md)
- Join our [discussions](https://github.com/mikepurvis/prime-physics-engine/discussions)

Happy prime hunting! 🎯