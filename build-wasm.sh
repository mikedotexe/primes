#!/bin/bash
# Build script for WebAssembly version of the membrane prime engine

echo "🚀 Building WASM membrane prime engine..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM module
echo "Building WASM module..."
wasm-pack build \
    --target web \
    --features wasm \
    --no-default-features

echo "✅ WASM build complete!"
echo "📦 Output in pkg/"
echo ""
echo "🌐 To run:"
echo "  python3 -m http.server 8000"
echo "  Open http://localhost:8000"