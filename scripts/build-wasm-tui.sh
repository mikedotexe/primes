#!/bin/bash
# Build WASM TUI package

set -e

echo "Building WASM TUI package..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM package
echo "Building with wasm-pack..."
wasm-pack build --target web --out-dir pkg --no-default-features --features wasm

# Copy the web files to pkg directory for easy serving
echo "Copying web files..."
cp -r web-tui/* pkg/

# Fix the import path in the WASM JavaScript file
echo "Fixing import paths..."
sed -i.bak "s|../pkg/prime_physics_engine.js|./prime_physics_engine.js|g" pkg/lagrange-tui-wasm.js
rm pkg/lagrange-tui-wasm.js.bak

# Create a simple server script
cat > pkg/serve.sh << 'EOF'
#!/bin/bash
echo "Starting web server at http://localhost:8000"
echo "Open http://localhost:8000/index-wasm.html for WASM version"
echo "Open http://localhost:8000/index.html for JS-only version"
python3 -m http.server 8000
EOF

chmod +x pkg/serve.sh

echo "Build complete!"
echo ""
echo "To run the TUI:"
echo "  cd pkg"
echo "  ./serve.sh"
echo ""
echo "Then open http://localhost:8000/index-wasm.html in your browser"