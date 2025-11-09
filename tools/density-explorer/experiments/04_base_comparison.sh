#!/bin/bash
# Experiment 4: Cross-Base Pattern Validation
# Tests if the (1,5)-like pattern (coprime boundaries, minimal padding)
# works consistently across different bases
#
# Based on CLAUDE.md finding: "Universal patterns exist"
# The (1,5) k=(0,0) configuration works in 5+ different bases

set -e

echo "=================================="
echo "Experiment 4: Cross-Base Patterns"
echo "=================================="
echo ""
echo "This experiment tests whether optimal membrane configurations"
echo "discovered in base 10 also work in other bases."
echo ""
echo "CLAUDE.md reports that (1,5) k=(0,0) achieves:"
echo "  - Base  6: 33% success (champion!)"
echo "  - Base 14: 27% success"
echo "  - Base 18: 24% success"
echo "  - Base 10: 18.5% success"
echo ""
echo "We'll test a simplified version: single-digit midpoint with"
echo "minimal padding (0:1 inner layer) across bases 6, 10, 14, 18."
echo ""
echo "Samples: 100,000 per (base, config) pair"
echo ""

cd "$(dirname "$0")/.."

mkdir -p experiments/results

echo "Testing Base 6..."
cargo run --release -- grid \
  --base 6 \
  --mid-kind free \
  --mid-len-range 1..5 \
  --inner-zero-range 0..3 \
  --inner-slot 1 \
  --outer-layers 0:1 \
  --samples 100000 \
  --allowed-last-digits 1,5 \
  --out-csv experiments/results/04_base06.csv

echo "Testing Base 10..."
cargo run --release -- grid \
  --base 10 \
  --mid-kind free \
  --mid-len-range 1..5 \
  --inner-zero-range 0..3 \
  --inner-slot 1 \
  --outer-layers 0:1 \
  --samples 100000 \
  --allowed-last-digits 1,3,7,9 \
  --out-csv experiments/results/04_base10.csv

echo "Testing Base 14..."
cargo run --release -- grid \
  --base 14 \
  --mid-kind free \
  --mid-len-range 1..5 \
  --inner-zero-range 0..3 \
  --inner-slot 1 \
  --outer-layers 0:1 \
  --samples 100000 \
  --allowed-last-digits 1,3,5,9,11,13 \
  --out-csv experiments/results/04_base14.csv

echo "Testing Base 18..."
cargo run --release -- grid \
  --base 18 \
  --mid-kind free \
  --mid-len-range 1..5 \
  --inner-zero-range 0..3 \
  --inner-slot 1 \
  --outer-layers 0:1 \
  --samples 100000 \
  --allowed-last-digits 1,5,7,11,13,17 \
  --out-csv experiments/results/04_base18.csv

echo ""
echo "✓ Results saved to experiments/results/04_base*.csv"
echo ""
echo "Analysis steps:"
echo ""
echo "1. For each base, find the configuration with highest enrichment_factor"
echo "2. Compare peak densities across bases"
echo "3. Check if coprime digits (relative to base) perform better"
echo ""
echo "Expected observations:"
echo ""
echo "  Base | Expected Peak | Coprime Constraint"
echo "  ---- | ------------- | ------------------"
echo "    6  |    ~25-35%    | gcd(digit, 6) = 1 → {1,5}"
echo "   10  |    ~15-22%    | gcd(digit, 10) = 1 → {1,3,7,9}"
echo "   14  |    ~18-27%    | gcd(digit, 14) = 1 → {1,3,5,9,11,13}"
echo "   18  |    ~18-24%    | gcd(digit, 18) = 1 → {1,5,7,11,13,17}"
echo ""
echo "Key insight from CLAUDE.md:"
echo "  'Coprimality is essential - 100% of top configs use coprime"
echo "   boundary digits. The optimal configuration depends on base"
echo "   factorization, not even/odd distinction.'"
echo ""
echo "Compare enrichment_factor across bases to validate this finding!"
