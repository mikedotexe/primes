#!/bin/bash
# Build script for Prime Physics WASM demo

set -e

echo "🚀 Building Prime Physics WebAssembly Demo..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Please install it with:"
    echo "   cargo install wasm-pack"
    exit 1
fi

# Build options
BUILD_MODE="${1:-release}"
FEATURES=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --simd)
            echo "✨ Enabling SIMD features..."
            export RUSTFLAGS='-C target-feature=+simd128'
            FEATURES="--features simd"
            shift
            ;;
        --dev)
            BUILD_MODE="dev"
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf www/pkg

# Build WASM module
echo "🔨 Building WASM module ($BUILD_MODE mode)..."
if [ "$BUILD_MODE" = "dev" ]; then
    wasm-pack build --dev --target web --out-dir www/pkg $FEATURES
else
    wasm-pack build --release --target web --out-dir www/pkg $FEATURES
fi

# Check build success
if [ -f "www/pkg/prime_physics_wasm.js" ]; then
    echo "✅ Build successful!"
    echo ""
    echo "📦 Output files:"
    ls -la www/pkg/
    echo ""
    echo "🌐 To run locally:"
    echo "   cd www && npx http-server -p 8080"
    echo "   Open http://localhost:8080"
else
    echo "❌ Build failed!"
    exit 1
fi