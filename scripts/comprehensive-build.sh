#!/bin/bash

# Comprehensive build script with detailed logging
# Creates a complete build report for the Prime Physics Engine

set -euo pipefail

# Setup
BUILD_LOG="comprehensive-build-$(date +%Y%m%d-%H%M%S).log"
echo "🔨 Prime Physics Engine - Comprehensive Build Report" | tee "$BUILD_LOG"
echo "===================================================" | tee -a "$BUILD_LOG"
echo "Date: $(date)" | tee -a "$BUILD_LOG"
echo "Platform: $(uname -a)" | tee -a "$BUILD_LOG"
echo "Rust: $(rustc --version)" | tee -a "$BUILD_LOG"
echo "Cargo: $(cargo --version)" | tee -a "$BUILD_LOG"
echo "" | tee -a "$BUILD_LOG"

# Function to run and log commands
run_build() {
    local description="$1"
    local command="$2"
    
    echo "" | tee -a "$BUILD_LOG"
    echo "🔹 $description" | tee -a "$BUILD_LOG"
    echo "Command: $command" | tee -a "$BUILD_LOG"
    echo "---" | tee -a "$BUILD_LOG"
    
    if eval "$command" >> "$BUILD_LOG" 2>&1; then
        echo "✅ Success" | tee -a "$BUILD_LOG"
        return 0
    else
        echo "❌ Failed (see log for details)" | tee -a "$BUILD_LOG"
        return 1
    fi
}

# Clean build
echo "📦 Starting fresh build..." | tee -a "$BUILD_LOG"
run_build "Clean previous builds" "cargo clean"

# Core builds
echo "" | tee -a "$BUILD_LOG"
echo "🏗️  Core Library Builds" | tee -a "$BUILD_LOG"
echo "=====================" | tee -a "$BUILD_LOG"
run_build "Debug build" "cargo build"
run_build "Release build" "cargo build --release"
run_build "All features" "cargo build --release --all-features"

# Feature builds
echo "" | tee -a "$BUILD_LOG"
echo "🎯 Feature-Specific Builds" | tee -a "$BUILD_LOG"
echo "=========================" | tee -a "$BUILD_LOG"
run_build "Wheel30 optimization" "cargo build --release --no-default-features --features wheel30"
run_build "Metal GPU support" "cargo build --release --no-default-features --features metal" || true
run_build "WASM support" "cargo build --target wasm32-unknown-unknown --release --no-default-features --features wasm"

# Tests
echo "" | tee -a "$BUILD_LOG"
echo "🧪 Test Suite" | tee -a "$BUILD_LOG"
echo "============" | tee -a "$BUILD_LOG"
run_build "Unit tests" "cargo test --lib"
run_build "Integration tests" "cargo test --tests"
run_build "Doc tests" "cargo test --doc" || true

# Examples
echo "" | tee -a "$BUILD_LOG"
echo "📚 Example Verification" | tee -a "$BUILD_LOG"
echo "=====================" | tee -a "$BUILD_LOG"
run_build "Prime count smoke test" "cargo run --release --example prime_count_smoke_test"
run_build "Example count" "ls examples/verified/*.rs | wc -l"

# Documentation
echo "" | tee -a "$BUILD_LOG"
echo "📖 Documentation" | tee -a "$BUILD_LOG"
echo "===============" | tee -a "$BUILD_LOG"
run_build "Generate docs" "cargo doc --no-deps --all-features"

# Summary
echo "" | tee -a "$BUILD_LOG"
echo "📊 Build Summary" | tee -a "$BUILD_LOG"
echo "===============" | tee -a "$BUILD_LOG"
echo "• Native library size: $(ls -lh ../target/release/libprime_physics_engine.rlib 2>/dev/null | awk '{print $5}' || echo 'N/A')" | tee -a "$BUILD_LOG"
echo "• WASM size: $(ls -lh ../target/wasm32-unknown-unknown/release/prime_physics_engine.wasm 2>/dev/null | awk '{print $5}' || echo 'N/A')" | tee -a "$BUILD_LOG"
echo "• Total examples: $(ls examples/verified/*.rs 2>/dev/null | wc -l)" | tee -a "$BUILD_LOG"
echo "• Build log: $BUILD_LOG" | tee -a "$BUILD_LOG"
echo "" | tee -a "$BUILD_LOG"
echo "✅ Build process complete!" | tee -a "$BUILD_LOG"

# Move log to parent directory
mv "$BUILD_LOG" ../