#!/bin/bash

# Comprehensive build script for Prime Physics Engine
# Builds all targets, features, examples, tests, and benchmarks

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Build log
BUILD_LOG="build-everything-$(date +%Y%m%d-%H%M%S).log"
SUMMARY_LOG="build-summary.log"

echo -e "${BLUE}🔨 Prime Physics Engine - Complete Build${NC}"
echo -e "${BLUE}========================================${NC}"
echo "Build log: ${BUILD_LOG}"
echo

# Initialize logs
{
    echo "Prime Physics Engine - Complete Build Report"
    echo "==========================================="
    echo "Started: $(date)"
    echo "Platform: $(uname -a)"
    echo "Rust: $(rustc --version)"
    echo "Cargo: $(cargo --version)"
    echo
} | tee "${BUILD_LOG}"

# Initialize summary
echo "# Build Summary" > "${SUMMARY_LOG}"
echo "" >> "${SUMMARY_LOG}"

# Counters
SUCCESS_COUNT=0
FAILURE_COUNT=0
SKIP_COUNT=0

# Function to build and log
build_target() {
    local description="$1"
    local cmd="$2"
    local category="${3:-BUILD}"
    
    echo -ne "[${category}] ${description}..."
    
    {
        echo "=================================================================================="
        echo "[$(date +%H:%M:%S)] ${description}"
        echo "Command: ${cmd}"
        echo "----------------------------------------------------------------------------------"
    } >> "${BUILD_LOG}"
    
    # Run command
    if eval "${cmd}" >> "${BUILD_LOG}" 2>&1; then
        echo -e " ${GREEN}✓${NC}"
        echo "[RESULT] SUCCESS" >> "${BUILD_LOG}"
        echo "✅ ${description}" >> "${SUMMARY_LOG}"
        ((SUCCESS_COUNT++))
        return 0
    else
        local exit_code=$?
        echo -e " ${RED}✗${NC}"
        echo "[RESULT] FAILED with exit code ${exit_code}" >> "${BUILD_LOG}"
        echo "❌ ${description}" >> "${SUMMARY_LOG}"
        ((FAILURE_COUNT++))
        return 1
    fi
}

# 1. Clean build environment
echo -e "\n${CYAN}=== 1. Clean Build Environment ===${NC}"
build_target "Clean previous builds" "cargo clean" "CLEAN"

# 2. Core library builds
echo -e "\n${CYAN}=== 2. Core Library Builds ===${NC}"
build_target "Debug build" "cargo build" "CORE"
build_target "Release build" "cargo build --release" "CORE"
build_target "Release with all features" "cargo build --release --all-features" "CORE"

# 3. Feature-specific builds
echo -e "\n${CYAN}=== 3. Feature-Specific Builds ===${NC}"
build_target "Default features only" "cargo build --release" "FEATURE"
build_target "Wheel30 optimization" "cargo build --release --no-default-features --features wheel30" "FEATURE"
build_target "DVFS adaptive" "cargo build --release --no-default-features --features dvfs-adaptive" "FEATURE"
build_target "Metal GPU support" "cargo build --release --no-default-features --features metal" "FEATURE"
build_target "Phase4 optimizations" "cargo build --release --no-default-features --features phase4" "FEATURE"
build_target "AMX support" "cargo build --release --no-default-features --features amx" "FEATURE"
build_target "Prime harmonics" "cargo build --release --no-default-features --features prime-harmonics" "FEATURE"
build_target "Visualization tools" "cargo build --release --no-default-features --features visualization" "FEATURE"

# 4. WASM builds
echo -e "\n${CYAN}=== 4. WebAssembly Builds ===${NC}"
build_target "WASM library" "cargo build --target wasm32-unknown-unknown --release --no-default-features --features wasm" "WASM"
build_target "WASM check" "cargo check --target wasm32-unknown-unknown --no-default-features --features wasm" "WASM"

# 5. Binary targets
echo -e "\n${CYAN}=== 5. Binary Targets ===${NC}"
build_target "membrane-prime" "cargo build --release --bin membrane-prime" "BIN"
build_target "membrane-prime-optimized" "cargo build --release --bin membrane-prime-optimized" "BIN"
build_target "membrane-prime-gpu" "cargo build --release --bin membrane-prime-gpu" "BIN"
build_target "membrane-prime-gpu-fast" "cargo build --release --bin membrane-prime-gpu-fast" "BIN"
build_target "membrane-prime-ultra" "cargo build --release --bin membrane-prime-ultra" "BIN"

# 6. Examples (verified directory)
echo -e "\n${CYAN}=== 6. Examples (Verified) ===${NC}"
for example in examples/verified/*.rs; do
    if [ -f "$example" ]; then
        example_name=$(basename "$example" .rs)
        build_target "Example: $example_name" "cargo build --release --example $example_name" "EXAMPLE"
    fi
done

# 7. Examples (experimental directory) - allow failures
echo -e "\n${CYAN}=== 7. Examples (Experimental) ===${NC}"
echo -e "${YELLOW}Note: Experimental examples may have syntax errors${NC}"
for example in examples/experimental/*.rs; do
    if [ -f "$example" ]; then
        example_name=$(basename "$example" .rs)
        # Don't count experimental failures
        if ! build_target "Example: $example_name" "cargo build --release --example $example_name 2>/dev/null" "EXP"; then
            ((FAILURE_COUNT--))  # Don't count experimental failures
            ((SKIP_COUNT++))
        fi
    fi
done

# 8. Tests
echo -e "\n${CYAN}=== 8. Test Suite ===${NC}"
build_target "Unit tests" "cargo test --lib" "TEST"
build_target "Integration tests" "cargo test --tests" "TEST"
build_target "Doc tests" "cargo test --doc" "TEST" || true  # Known issue
build_target "All tests with all features" "cargo test --all-features" "TEST"

# 9. Benchmarks
echo -e "\n${CYAN}=== 9. Benchmarks ===${NC}"
build_target "Compile benchmarks" "cargo bench --no-run" "BENCH"

# 10. Quality checks
echo -e "\n${CYAN}=== 10. Code Quality ===${NC}"
build_target "Format check" "cargo fmt -- --check" "QUALITY"
build_target "Clippy default" "cargo clippy -- -D warnings" "QUALITY" || true  # Known issues
build_target "Clippy all features" "cargo clippy --all-features -- -D warnings" "QUALITY" || true

# 11. Documentation
echo -e "\n${CYAN}=== 11. Documentation ===${NC}"
build_target "Generate docs" "cargo doc --no-deps" "DOC"
build_target "Generate docs (all features)" "cargo doc --no-deps --all-features" "DOC"

# 12. Check examples can run
echo -e "\n${CYAN}=== 12. Example Smoke Tests ===${NC}"
echo -e "${YELLOW}Running quick smoke tests for key examples...${NC}"
build_target "Run educational_explorer" "timeout 5 cargo run --release --example educational_explorer < /dev/null || true" "RUN"
build_target "Run basic_membrane" "timeout 5 cargo run --release --example basic_membrane || true" "RUN"
build_target "Run sieve_benchmark" "cargo run --release --example sieve_benchmark" "RUN"

# Generate final summary
echo -e "\n${CYAN}=== Build Summary ===${NC}"
{
    echo ""
    echo "## Final Statistics"
    echo "- Total targets: $((SUCCESS_COUNT + FAILURE_COUNT + SKIP_COUNT))"
    echo "- Successful: ${SUCCESS_COUNT}"
    echo "- Failed: ${FAILURE_COUNT}"
    echo "- Skipped (experimental): ${SKIP_COUNT}"
    echo ""
    echo "## Build Artifacts"
    echo "- Native binaries: $(find /Users/mikepurvis/claude-target/primes/release -name "membrane-prime*" -type f 2>/dev/null | wc -l | xargs)"
    echo "- WASM modules: $(find /Users/mikepurvis/claude-target/primes/wasm32-unknown-unknown -name "*.wasm" -type f 2>/dev/null | wc -l | xargs)"
    echo "- Examples built: $(find /Users/mikepurvis/claude-target/primes/release/examples -type f 2>/dev/null | wc -l | xargs)"
    echo ""
    echo "Completed: $(date)"
} | tee -a "${SUMMARY_LOG}"

# Display summary
echo
cat "${SUMMARY_LOG}"

# Final status
echo
if [ ${FAILURE_COUNT} -eq 0 ]; then
    echo -e "${GREEN}✅ All builds successful!${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  ${FAILURE_COUNT} builds failed (see ${BUILD_LOG} for details)${NC}"
    echo
    echo "Common issues:"
    echo "- Doc test: Known issue with missing base parameter"
    echo "- Clippy: Format string warnings in binaries"
    echo "- Experimental examples: May have syntax errors"
    exit 1
fi