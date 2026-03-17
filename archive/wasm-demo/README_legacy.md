# Prime Physics Engine - WebAssembly Demo

Interactive browser demo showcasing membrane prime generation and cache-aware algorithms.

## Features

### 1. Membrane Prime Generator
- Interactive parameter tuning (base, boundary digits, padding)
- Real-time coprimality checking
- Batch generation with primality testing
- Success rate statistics

### 2. Cache-Aware Prime Sieve
- Bit-packed implementation optimized for cache efficiency
- Performance benchmarking up to 50 million
- Interactive performance visualization

### 3. Neural Network Demo
- 8→16→1 architecture (preparation for AMX/SME)
- Interactive input controls
- Real-time inference

## Building

### Prerequisites
- Rust toolchain with `wasm32-unknown-unknown` target
- `wasm-pack` installed (`cargo install wasm-pack`)
- Node.js (for serving locally)

### Build Steps

```bash
# From the wasm-demo directory
cd wasm-demo

# Build the WASM module
wasm-pack build --target web --out-dir www/pkg

# Optional: Build with SIMD support (for compatible browsers)
RUSTFLAGS='-C target-feature=+simd128' \
  wasm-pack build --target web --out-dir www/pkg

# Serve locally
cd www
npx http-server -p 8080
# Or use Python: python3 -m http.server 8080
```

Open http://localhost:8080 in your browser.

## Deployment

The `www` directory contains only static files and can be deployed to:
- GitHub Pages
- Netlify
- Vercel
- Any static hosting service

### GitHub Pages Deployment

```bash
# From project root
git add wasm-demo/www
git commit -m "Add WASM demo"
git push

# Enable GitHub Pages for the www directory
```

## Browser Compatibility

- Modern browsers with WebAssembly support (Chrome 57+, Firefox 52+, Safari 11+)
- SIMD features require Chrome 91+ or Firefox 89+
- Best performance on desktop browsers

## Technical Notes

### Memory Efficiency
- Membrane generation uses BigInt only when necessary
- Prime sieve uses bit-packing (8x memory reduction)
- Neural network uses stack allocation (no heap)

### Performance
- Membrane generation: ~1ms per batch of 20
- Prime sieve: ~500M candidates/sec (WASM)
- Neural network: ~10μs per inference (WASM)

Compare to native performance:
- Native sieve: ~2B candidates/sec
- Native neural: ~8ns with cache residency

### Architecture
- Thin WASM bindings over core Rust library
- No dynamic allocation in hot paths
- Console error handling for better debugging

## Development

### Adding Features

1. Add Rust functions to `src/lib.rs` with `#[wasm_bindgen]`
2. Rebuild with `wasm-pack build`
3. Import new functions in `main.js`

### Debugging

Enable console error messages:
```rust
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}
```

Use browser DevTools:
- Performance tab for profiling
- Console for error messages
- Memory tab for heap analysis

## Future Enhancements

1. **Web Workers** - Offload heavy computation
2. **WebGPU** - GPU acceleration for membrane generation
3. **Streaming** - Generate primes progressively
4. **Visualization** - 3D membrane structure rendering
5. **Comparison** - Side-by-side algorithm races

## License

Part of the Prime Physics Engine project. See main LICENSE file.
