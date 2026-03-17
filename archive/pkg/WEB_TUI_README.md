# Lagrange TUI - Web Version

This directory contains the web-based terminal UI for the Prime Physics Engine.

## Files

- `index.html` - Pure JavaScript version (no WASM)
- `index-wasm.html` - WASM-powered version using the Rust engine
- `prime_physics_engine.js` - Generated WASM JavaScript bindings
- `prime_physics_engine_bg.wasm` - The compiled Rust code

## Running

1. Start a web server in this directory:
   ```bash
   python3 -m http.server 8000
   # or
   npx serve .
   ```

2. Open in your browser:
   - http://localhost:8000/index-wasm.html - WASM version (recommended)
   - http://localhost:8000/index.html - Pure JS version

## Controls

- **g** - Generate new prime pair
- **t** - Test Lagrange points for primality
- **c** - Cycle through configurations
- **h** - Show help
- **←/→** - Select prime atom
- **q** - Quit

## Features

The WASM version uses the actual Rust prime physics engine compiled to WebAssembly, providing:
- Real membrane prime generation
- Accurate primality testing
- Same algorithms as the native version
- Full type safety from Rust

The interface shows:
- Two membrane primes with visible zero padding (◯)
- The distance between them
- Lagrange point calculations
- Prime testing results
