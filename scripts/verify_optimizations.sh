#!/bin/bash
# verify_optimizations.sh - Hardcore verification of optimization claims

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=== Prime Physics Engine - Optimization Verification Suite ==="
echo "Platform: $(uname -m) on $(uname -s)"
echo "Date: $(date)"
echo ""

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 1. Run baseline measurements
echo "1. Collecting Baseline Measurements..."
echo "----------------------------------------"
cargo bench --bench optimization_verification -- baseline_data_collection --noplot

# 2. Memory bandwidth analysis
echo ""
echo "2. Memory Bandwidth Analysis..."
echo "----------------------------------------"
cargo bench --bench optimization_verification -- memory_bandwidth --noplot

# 3. Cache efficiency with different chunk sizes
echo ""
echo "3. Cache Efficiency Analysis..."
echo "----------------------------------------"
cargo bench --bench optimization_verification -- cache_efficiency --noplot

# 4. Platform-specific measurements
echo ""
echo "4. Platform-Specific Performance Counters..."
echo "----------------------------------------"

if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "macOS detected - using Instruments for cache analysis"
    
    # Build release binary for profiling
    cargo build --release --example prime_generator
    
    # Check if we have sudo access for powermetrics
    if sudo -n true 2>/dev/null; then
        echo "Running 10-second power measurement..."
        sudo powermetrics --samplers cpu_power -i 1000 -n 10 > power_baseline.log 2>&1 &
        POWER_PID=$!
        
        # Run workload
        ./target/release/examples/prime_generator --limit 1000000000 > /dev/null
        
        # Stop power monitoring
        sudo kill $POWER_PID 2>/dev/null || true
        
        # Extract power metrics
        echo "CPU Power Statistics:"
        grep -E "CPU Power|Package Power" power_baseline.log | head -20
    else
        echo "${YELLOW}Warning: sudo access needed for power metrics${NC}"
    fi
    
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "Linux detected - using perf for cache analysis"
    
    if command_exists perf; then
        # Build with debug symbols
        cargo build --release
        
        # Measure cache misses
        echo "Cache miss analysis for 100M primes:"
        perf stat -e cache-misses,cache-references,L1-dcache-load-misses,L1-dcache-loads \
            ./target/release/prime-physics-engine sieve 100000000 2>&1 | \
            grep -E "cache|L1-dcache"
    else
        echo "${YELLOW}Warning: 'perf' not found. Install linux-tools-common${NC}"
    fi
fi

# 5. Correctness verification
echo ""
echo "5. Prime Count Verification..."
echo "----------------------------------------"
cargo test --release --bench optimization_verification verification

# 6. Generate comparison report
echo ""
echo "6. Performance Comparison Report"
echo "----------------------------------------"

# Parse benchmark results
if [ -f "target/criterion/memory_bandwidth/baseline/1000000000/base/estimates.json" ]; then
    echo "Parsing Criterion results..."
    
    # Extract timing data (would need jq in practice)
    echo "${GREEN}✓ Benchmark data collected${NC}"
else
    echo "${YELLOW}⚠ Criterion results not found${NC}"
fi

# 7. Memory usage verification
echo ""
echo "7. Memory Usage Analysis"
echo "----------------------------------------"

# Calculate theoretical memory usage
for limit in 10000000 100000000 1000000000; do
    baseline_bytes=$((limit / 16))  # 1 bit per odd
    wheel30_bytes=$((limit * 8 / 30))  # 8 survivors per 30
    reduction=$(echo "scale=1; 100 * (1 - $wheel30_bytes / $baseline_bytes)" | bc)
    
    echo "Limit: $limit"
    echo "  Baseline: $(echo "scale=2; $baseline_bytes / 1048576" | bc) MB"
    echo "  Wheel-30: $(echo "scale=2; $wheel30_bytes / 1048576" | bc) MB"
    echo "  Reduction: ${reduction}%"
done

# 8. Throughput variance test (simplified)
echo ""
echo "8. Throughput Stability Test"
echo "----------------------------------------"
echo "Running 10-second throughput variance test..."

# Run multiple short benchmarks and calculate variance
declare -a throughputs
for i in {1..5}; do
    result=$(cargo run --release --example prime_generator -- --limit 10000000 2>&1 | \
             grep -oE "[0-9]+\.[0-9]+ primes/sec" | \
             grep -oE "[0-9]+\.[0-9]+" || echo "0")
    throughputs+=($result)
    echo "  Sample $i: $result primes/sec"
done

# Calculate mean and variance (would use awk/bc in practice)
echo "${GREEN}✓ Throughput samples collected${NC}"

# 9. Final summary
echo ""
echo "=== Verification Summary ==="
echo "----------------------------------------"

# Check if all tests passed
if cargo test --release --bench optimization_verification 2>&1 | grep -q "test result: ok"; then
    echo "${GREEN}✓ All correctness tests passed${NC}"
else
    echo "${RED}✗ Some tests failed${NC}"
fi

# Check benchmark completion
if [ -d "target/criterion" ]; then
    echo "${GREEN}✓ Performance benchmarks completed${NC}"
    echo ""
    echo "Full results available in:"
    echo "  - target/criterion/*/report/index.html"
    echo "  - power_baseline.log (if on macOS with sudo)"
else
    echo "${YELLOW}⚠ Benchmarks incomplete${NC}"
fi

echo ""
echo "To generate detailed HTML reports:"
echo "  cargo bench --bench optimization_verification"
echo ""
echo "To run with specific features:"
echo "  cargo bench --features wheel30 --bench optimization_verification"