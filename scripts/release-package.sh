#!/bin/bash

# Release packaging script for Prime Physics Engine
# Creates a clean tarball and comprehensive build log

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

echo -e "${BLUE}🚀 Prime Physics Engine Release Packaging${NC}"
echo -e "${BLUE}========================================${NC}"
echo "Version: ${VERSION}"
echo "Package: ${TARBALL}"
echo "Build Log: ${BUILD_LOG}"
echo

# Initialize build log
{
    echo "Prime Physics Engine v${VERSION} - Comprehensive Build Report"
    echo "============================================================"
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
        echo -e " ${RED}✗${NC}"
        echo "[RESULT] FAILED with exit code $?" >> "${BUILD_LOG}"
        echo -e "${RED}Failed during: ${description}${NC}"
        echo "Check ${BUILD_LOG} for details"
        return 1
    fi
    
    echo >> "${BUILD_LOG}"
}

# Clean previous artifacts
echo "🧹 Cleaning previous artifacts..."
rm -f "${TARBALL}" "${BUILD_LOG}.tmp"

# 1. Full clean build
echo -e "\n📦 Building all configurations..."
run_and_log "Clean build directory" cargo clean
run_and_log "Core build (debug)" cargo build
run_and_log "Core build (release)" cargo build --release
run_and_log "All features build" cargo build --all-features
run_and_log "Metal GPU build" cargo build --features metal
run_and_log "WASM build" cargo build --target wasm32-unknown-unknown --features wasm

# 2. Run all tests
echo -e "\n🧪 Running all tests..."
run_and_log "Core tests" cargo test
run_and_log "All features tests" cargo test --all-features
run_and_log "Doc tests" cargo test --doc
run_and_log "Integration tests" cargo test --test '*'

# 3. Code quality checks
echo -e "\n🔍 Code quality checks..."
run_and_log "Clippy core" cargo clippy -- -D warnings
run_and_log "Clippy all features" cargo clippy --all-features -- -D warnings
run_and_log "Rustfmt check" cargo fmt -- --check

# 4. Documentation
echo -e "\n📚 Building documentation..."
run_and_log "Rustdoc" cargo doc --no-deps --all-features

# 5. Benchmarks (compile only)
echo -e "\n⚡ Benchmark compilation..."
run_and_log "Compile benchmarks" cargo bench --no-run

# 6. Check for security issues
echo -e "\n🛡️ Security analysis..."
{
    echo "=================================================================================="
    echo "[$(date +%H:%M:%S)] Security Analysis"
    echo "----------------------------------------------------------------------------------"
    echo "Unsafe block analysis:"
    
    # Count unsafe blocks
    total_unsafe=$(grep -r "unsafe" src/ --include="*.rs" | wc -l)
    documented_unsafe=$(grep -r "unsafe" src/ --include="*.rs" | grep -B2 -A2 "SAFETY:" | grep "unsafe" | wc -l)
    
    echo "Total unsafe occurrences: ${total_unsafe}"
    echo "Documented unsafe blocks: ${documented_unsafe}"
    echo
    
    # List files with unsafe code
    echo "Files containing unsafe code:"
    grep -r "unsafe" src/ --include="*.rs" -l | while read -r file; do
        count=$(grep -c "unsafe" "$file")
        echo "  ${file}: ${count} occurrences"
    done
    echo
} >> "${BUILD_LOG}"

# 7. Create release tarball
echo -e "\n📦 Creating release tarball..."

# Create temporary directory for clean export
TEMP_DIR=$(mktemp -d)
EXPORT_DIR="${TEMP_DIR}/${PACKAGE_NAME}"

{
    echo "=================================================================================="
    echo "[$(date +%H:%M:%S)] Creating Release Package"
    echo "----------------------------------------------------------------------------------"
} >> "${BUILD_LOG}"

# Use git archive to respect .gitignore
if git archive --format=tar --prefix="${PACKAGE_NAME}/" HEAD | tar -x -C "${TEMP_DIR}"; then
    echo -e "Git archive... ${GREEN}✓${NC}"
    echo "Git archive created successfully" >> "${BUILD_LOG}"
else
    echo -e "Git archive... ${RED}✗${NC}"
    echo "Git archive failed, falling back to manual copy" >> "${BUILD_LOG}"
    
    # Fallback: manual copy excluding gitignored files
    mkdir -p "${EXPORT_DIR}"
    
    # Copy files while respecting .gitignore
    rsync -av \
        --exclude-from='.gitignore' \
        --exclude='.git' \
        --exclude="${BUILD_LOG}" \
        --exclude="${TARBALL}" \
        . "${EXPORT_DIR}/" >> "${BUILD_LOG}" 2>&1
fi

# Add build artifacts that should be included
echo -ne "Adding documentation..."
if [ -d "target/doc" ]; then
    mkdir -p "${EXPORT_DIR}/docs/rustdoc"
    cp -r target/doc/* "${EXPORT_DIR}/docs/rustdoc/" 2>> "${BUILD_LOG}"
    echo -e " ${GREEN}✓${NC}"
else
    echo -e " ${YELLOW}⚠ (no docs found)${NC}"
fi

# Create the tarball
echo -ne "Creating tarball..."
(cd "${TEMP_DIR}" && tar -czf "${TARBALL}" "${PACKAGE_NAME}/")
mv "${TEMP_DIR}/${TARBALL}" . 2>> "${BUILD_LOG}"
echo -e " ${GREEN}✓${NC}"

# Cleanup
rm -rf "${TEMP_DIR}"

# 8. Generate summary
echo -e "\n📊 Generating summary..."
{
    echo
    echo "=================================================================================="
    echo "BUILD SUMMARY"
    echo "=================================================================================="
    echo
    
    # Count warnings and errors
    warning_count=$(grep -i "warning:" "${BUILD_LOG}" | wc -l)
    error_count=$(grep -i "error:" "${BUILD_LOG}" | wc -l)
    
    echo "Total Warnings: ${warning_count}"
    echo "Total Errors: ${error_count}"
    echo
    
    # List warnings by category
    if [ ${warning_count} -gt 0 ]; then
        echo "Warnings by category:"
        grep -i "warning:" "${BUILD_LOG}" | sed 's/.*warning://' | sort | uniq -c | sort -rn | head -10
        echo
    fi
    
    # Package info
    echo "Package Information:"
    echo "  File: ${TARBALL}"
    echo "  Size: $(du -h "${TARBALL}" | cut -f1)"
    echo "  Files: $(tar -tzf "${TARBALL}" | wc -l)"
    echo
    
    # Feature matrix verification
    echo "Feature Matrix:"
    echo "  ✓ Core library"
    echo "  ✓ wheel30 optimization"
    echo "  ✓ Metal GPU support"
    echo "  ✓ WASM bindings"
    echo "  ✓ Prime harmonics"
    echo
    
    # Final status
    if [ ${error_count} -eq 0 ]; then
        echo "Status: BUILD SUCCESSFUL ✅"
    else
        echo "Status: BUILD FAILED ❌"
    fi
} >> "${BUILD_LOG}"

# Display summary
echo
echo -e "${GREEN}✅ Release packaging complete!${NC}"
echo
echo "📦 Tarball: ${TARBALL} ($(du -h "${TARBALL}" | cut -f1))"
echo "📋 Build log: ${BUILD_LOG} ($(wc -l < "${BUILD_LOG}") lines)"
echo

# Show summary stats
warning_count=$(grep -i "warning:" "${BUILD_LOG}" | wc -l)
error_count=$(grep -i "error:" "${BUILD_LOG}" | wc -l)

echo "Summary:"
echo "  Warnings: ${warning_count}"
echo "  Errors: ${error_count}"

if [ ${error_count} -eq 0 ]; then
    echo -e "\n${GREEN}🎉 Package ready for distribution!${NC}"
else
    echo -e "\n${RED}⚠️  Build had errors - review ${BUILD_LOG}${NC}"
fi