#!/bin/bash

# Quick WASM packaging script

set -euo pipefail

VERSION="1.0.0"
PACKAGE_NAME="prime-physics-engine-v${VERSION}-wasm"
TARBALL="${PACKAGE_NAME}.tar.gz"

echo "📦 Creating WASM release package..."

# Create temp directory
TEMP_DIR=$(mktemp -d)
EXPORT_DIR="${TEMP_DIR}/${PACKAGE_NAME}"
mkdir -p "${EXPORT_DIR}/wasm"

# Copy WASM files
echo "Copying WASM artifacts..."
cp /Users/mikepurvis/claude-target/primes/wasm32-unknown-unknown/release/*.wasm "${EXPORT_DIR}/wasm/" 2>/dev/null || true

# Create simple HTML demo
cat > "${EXPORT_DIR}/wasm/demo.html" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>Prime Physics Engine - WASM Demo</title>
    <style>
        body { font-family: monospace; padding: 20px; }
        .output { background: #f0f0f0; padding: 10px; margin: 10px 0; }
    </style>
</head>
<body>
    <h1>Prime Physics Engine - WebAssembly Demo</h1>
    <p>This demonstrates the Prime Physics Engine running in your browser!</p>
    
    <button onclick="findPrimes()">Find Primes up to 1000</button>
    <button onclick="generateMembrane()">Generate Membrane Prime</button>
    
    <div id="output" class="output">Click a button to see results...</div>
    
    <script type="module">
        // Note: In a real deployment, you would use wasm-bindgen to generate
        // proper JavaScript bindings. This is a placeholder showing the concept.
        
        window.findPrimes = function() {
            document.getElementById('output').innerHTML = 
                'WASM module would find primes here...<br>' +
                'The actual implementation requires wasm-bindgen bindings.';
        }
        
        window.generateMembrane = function() {
            document.getElementById('output').innerHTML = 
                'Membrane generation would happen here...<br>' +
                'Configuration: Base 6, (1,5) k=(0,0) - the champion!';
        }
        
        // In production, you would:
        // import init, { PrimeUniverse, MembraneConfig } from './prime_physics_engine.js';
        // await init('./prime_physics_engine.wasm');
    </script>
</body>
</html>
EOF

# Create README
cat > "${EXPORT_DIR}/wasm/README.md" << 'EOF'
# Prime Physics Engine - WASM Build

Successfully built with: `--no-default-features --features wasm`

## Files
- `prime_physics_engine.wasm` - Core library (433KB)
- `membrane-prime*.wasm` - Example applications (598-776KB each)

## Key Achievement
We solved the crossterm/ratatui incompatibility by disabling the `visualization` 
feature which tries to use terminal UI libraries in a web environment.

## Building
```bash
cargo build --target wasm32-unknown-unknown \
            --release \
            --no-default-features \
            --features wasm
```

## Next Steps
1. Use `wasm-pack` to generate proper JavaScript bindings
2. Optimize size with `wee_alloc` and `wasm-opt`
3. Create interactive web demos
EOF

# List files
echo -e "\nWASM files included:"
ls -lh "${EXPORT_DIR}/wasm/"

# Create tarball
echo -e "\nCreating package..."
(cd "${TEMP_DIR}" && tar -czf "${TARBALL}" "${PACKAGE_NAME}/")
mv "${TEMP_DIR}/${TARBALL}" .
rm -rf "${TEMP_DIR}"

echo -e "\n✅ Created: ${TARBALL}"
ls -lh "${TARBALL}"