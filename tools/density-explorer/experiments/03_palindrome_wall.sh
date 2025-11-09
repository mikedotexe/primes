#!/bin/bash
# Experiment 3: Palindrome Wall (11-Divisibility Crater)
# Demonstrates density crater at even total lengths due to divisibility by 11
#
# With --mirror: left half mirrors to right → palindromic numbers
# Even-length palindromes are ALWAYS divisible by 11
# Variable: inner zero padding shifts total length parity

set -e

echo "=================================="
echo "Experiment 3: Palindrome Wall"
echo "=================================="
echo ""
echo "This experiment demonstrates a DRAMATIC divisibility pattern:"
echo "Palindromic numbers with EVEN total length are divisible by 11!"
echo ""
echo "With --mirror enabled, the density will show clear 'bands':"
echo "  - Low density at even total_len (divisible by 11)"
echo "  - Higher density at odd total_len"
echo ""
echo "As you increase inner zero padding, the total length changes,"
echo "and the density 'sine wave' moves across the parameter space."
echo ""
echo "Configuration: 1-digit midpoint, mirrored, outer layer (0:1)"
echo "Variable: Inner zero padding 0..10"
echo "Samples: 50,000 per configuration"
echo ""

cd "$(dirname "$0")/.."

OUTPUT="experiments/results/03_palindrome_wall.csv"
mkdir -p experiments/results

echo "Running grid sweep with --mirror (this will take ~3-5 minutes)..."
cargo run --release -- grid \
  --mid-kind free \
  --mid-len-range 1..5 \
  --inner-zero-range 0..10 \
  --inner-slot 1 \
  --outer-layers 0:1 \
  --samples 50000 \
  --allowed-last-digits 1,3,7,9 \
  --mirror \
  --track-primes 3,5,7,11 \
  --out-csv "$OUTPUT"

echo ""
echo "✓ Results saved to: $OUTPUT"
echo ""
echo "Key columns to examine:"
echo "  - total_len: Total number of digits"
echo "  - prime_density: Observed density"
echo "  - divisible_counts[3] (11): Should be ~100% for even total_len"
echo ""
echo "Expected observation:"
echo "  Create a scatter plot of (total_len, prime_density)."
echo "  You should see clear alternating bands:"
echo ""
echo "    total_len | prime_density | divisibility by 11"
echo "    --------- | ------------- | ------------------"
echo "       5      |    ~15-20%    |      ~9%"
echo "       6      |     ~0-2%     |     ~95%   ← CRATER"
echo "       7      |    ~13-18%    |      ~9%"
echo "       8      |     ~0-2%     |     ~95%   ← CRATER"
echo ""
echo "This demonstrates how structural constraints (palindromes)"
echo "create 'repulsion walls' in prime distribution space."
echo ""
echo "Visualization: Open ../../viz/index.html and drop $OUTPUT"
echo "Look for vertical bands of low density at even total_len!"
