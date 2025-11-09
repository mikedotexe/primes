#!/bin/bash
# Experiment 1: Rollover Phenomenon
# Demonstrates density drop as midpoint length increases (1/ln n effect)
#
# Fixed configuration: Two thin outer rings (0:1 each)
# Variable: midpoint length from 1 to 12
# Expected: Gradual density decline as numbers get longer

set -e

echo "=================================="
echo "Experiment 1: Rollover Phenomenon"
echo "=================================="
echo ""
echo "This experiment demonstrates that prime density decreases as"
echo "the midpoint length increases, following the Prime Number Theorem"
echo "prediction of 1/ln(n). Numbers get longer → density drops."
echo ""
echo "Configuration: Two fixed outer layers (0:1 each)"
echo "Variable: Midpoint length 1..12"
echo "Samples: 100,000 per configuration"
echo ""

cd "$(dirname "$0")/.."

OUTPUT="experiments/results/01_rollover.csv"
mkdir -p experiments/results

echo "Running grid sweep (this will take ~5-10 minutes)..."
cargo run --release -- grid \
  --mid-kind free \
  --mid-len-range 1..12 \
  --inner-zero-range 0..0 \
  --inner-slot 1 \
  --outer-layers 0:1 \
  --samples 100000 \
  --allowed-last-digits 1,3,7,9 \
  --out-csv "$OUTPUT"

echo ""
echo "✓ Results saved to: $OUTPUT"
echo ""
echo "Key columns to examine:"
echo "  - mid_len: Length of free midpoint"
echo "  - total_len: Total number length"
echo "  - prime_density: Observed density"
echo "  - expected_density_pnt: PNT baseline (1/ln n)"
echo "  - enrichment_factor: How much better than random"
echo ""
echo "Expected observation:"
echo "  As mid_len increases (5→9→11 digits), density should decline"
echo "  following approximately 1/ln(n) curve, but staying 2-4x above"
echo "  the PNT baseline due to membrane structure."
echo ""
echo "Visualization: Open ../../viz/index.html and drop $OUTPUT"
