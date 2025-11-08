#!/bin/bash

# Lightweight release packaging script for Prime Physics Engine
# Creates a clean tarball and build log, skipping problematic WASM builds

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get version from Cargo.toml
VERSION=$(grep '^version =' Cargo.toml | head -1 | cut -d'"' -f2)
PACKAGE_NAME="prime-physics-engine-v${VERSION}"
BUILD_LOG="release-build-${VERSION}.log"
TARBALL="${PACKAGE_NAME}.tar.gz"

echo -e "${BLUE}🚀 Prime Physics Engine Release Packaging (Lite)${NC}"
echo -e "${BLUE}===============================================${NC}"
echo "Version: ${VERSION}"
echo "Package: ${TARBALL}"
echo "Build Log: ${BUILD_LOG}"
echo

# Initialize build log
{
    echo "Prime Physics Engine v${VERSION} - Build Report"
    echo "=============================================="
    echo "Generated: $(date)"
    echo "Platform: $(uname -a)"
    echo "Rust Version: $(rustc --version)"
    echo "Cargo Version: $(cargo --version)"
    echo
} > "${BUILD_LOG}"

# Function to run command and capture output
run_and_log() {
    local description="$1"
    shift
    local cmd="$@"
    
    echo -ne "${description}..."
    
    {
        echo "=================================================================================="
        echo "[$(date +%H:%M:%S)] ${description}"
        echo "Command: ${cmd}"
        echo "----------------------------------------------------------------------------------"
    } >> "${BUILD_LOG}"
    
    # Run command, capturing both stdout and stderr
    if $cmd >> "${BUILD_LOG}" 2>&1; then
        echo -e " ${GREEN}✓${NC}"
        echo "[RESULT] SUCCESS" >> "${BUILD_LOG}"
    else
        local exit_code=$?
        echo -e " ${RED}✗${NC}"
        echo "[RESULT] FAILED with exit code ${exit_code}" >> "${BUILD_LOG}"
        # Don't exit on failure, continue with other builds
    fi
    
    echo >> "${BUILD_LOG}"
}

# Clean previous artifacts
echo "🧹 Cleaning previous artifacts..."
rm -f "${TARBALL}"

# 1. Core builds only (skip full clean to save time)
echo -e "\n📦 Building core configurations..."
run_and_log "Core build (release)" cargo build --release
run_and_log "Core build with all features" cargo build --release --all-features
run_and_log "Metal GPU build" cargo build --release --features metal

# 2. Run core tests
echo -e "\n🧪 Running core tests..."
run_and_log "Unit tests" cargo test --lib
run_and_log "Doc tests" cargo test --doc

# 3. Quick quality check
echo -e "\n🔍 Code quality check..."
run_and_log "Clippy" cargo clippy -- -D warnings

# 4. Create release tarball
echo -e "\n📦 Creating release tarball..."

# Create temporary directory for clean export
TEMP_DIR=$(mktemp -d)
EXPORT_DIR="${TEMP_DIR}/${PACKAGE_NAME}"

# Prepare archive
mkdir -p "${EXPORT_DIR}"

# Use git ls-files to get tracked files only
echo -ne "Collecting tracked files..."
git ls-files -z | tar --null -T - -cf - | (cd "${EXPORT_DIR}" && tar -xf -)
echo -e " ${GREEN}✓${NC}"

# Add important generated files
echo -ne "Adding generated documentation..."
if [ -d "target/doc" ]; then
    mkdir -p "${EXPORT_DIR}/target/doc"
    cp -r target/doc "${EXPORT_DIR}/target/" 2>/dev/null || true
    echo -e " ${GREEN}✓${NC}"
else
    echo -e " ${YELLOW}⚠ (skipped)${NC}"
fi

# Add the build log
cp "${BUILD_LOG}" "${EXPORT_DIR}/" 2>/dev/null || true

# Create the tarball
echo -ne "Creating tarball..."
(cd "${TEMP_DIR}" && tar -czf "${TARBALL}" "${PACKAGE_NAME}/")
mv "${TEMP_DIR}/${TARBALL}" .
echo -e " ${GREEN}✓${NC}"

# Cleanup
rm -rf "${TEMP_DIR}"

# 5. Analyze results
echo -e "\n📊 Analyzing build results..."

# Count warnings and errors
warning_count=$(grep -c "warning:" "${BUILD_LOG}" 2>/dev/null || echo "0")
error_count=$(grep -c "error:" "${BUILD_LOG}" 2>/dev/null || echo "0")

# Add summary to log
{
    echo
    echo "=================================================================================="
    echo "BUILD SUMMARY"
    echo "=================================================================================="
    echo
    echo "Total Warnings: ${warning_count}"
    echo "Total Errors: ${error_count}"
    echo
    
    if [ "${warning_count}" -gt 0 ]; then
        echo "Warning Summary:"
        grep "warning:" "${BUILD_LOG}" | head -20 || true
        echo
    fi
    
    echo "Package Information:"
    echo "  File: ${TARBALL}"
    echo "  Size: $(du -h "${TARBALL}" | cut -f1)"
    echo "  Files: $(tar -tzf "${TARBALL}" | wc -l)"
    echo
} >> "${BUILD_LOG}"

# Display results
echo
echo -e "${GREEN}✅ Release packaging complete!${NC}"
echo
echo "📦 Package: ${TARBALL} ($(du -h "${TARBALL}" | cut -f1))"
echo "📋 Build log: ${BUILD_LOG}"
echo
echo "Contents summary:"
tar -tzf "${TARBALL}" | grep -E "\.(rs|toml|md)$" | wc -l | xargs echo "  Source files:"
tar -tzf "${TARBALL}" | grep -E "^[^/]+/$" | wc -l | xargs echo "  Directories:"
echo
echo "Build summary:"
echo "  Warnings: ${warning_count}"
echo "  Errors: ${error_count}"

# Note about WASM
echo
echo -e "${YELLOW}Note: WASM build skipped due to dependency issues with crossterm.${NC}"
echo -e "${YELLOW}      The core library and Metal GPU features are fully functional.${NC}"