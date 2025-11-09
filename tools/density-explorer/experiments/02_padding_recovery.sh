#!/bin/bash
# Experiment 2: Padding Recovery
# Demonstrates density rebound as inner zero padding increases
#
# Fixed: midpoint length = 3, outer layer 0:1
# Variable: inner zero padding from 0 to 10
# Expected: Density increases as zeros provide "elbow room"

set -e

echo "=================================="
echo "Experiment 2: Padding Recovery"
echo "=================================="
echo ""
echo "This experiment demonstrates that adding symmetric zero-padding"
echo "INCREASES prime density by disrupting divisibility patterns."
echo "Zeros act as 'repulsion barriers' preventing bad congruences."
echo ""
echo "Configuration: Fixed 3-digit midpoint + outer layer (0:1)"
echo "Variable: Inner zero padding 0..10"
echo "Samples: 100,000 per configuration"
echo ""

cd "$(dirname "$0")/.."

OUTPUT="experiments/results/02_padding_recovery.csv"
mkdir -p experiments/results

echo "Running grid sweep (this will take ~5-10 minutes)..."
cargo run --release -- grid \
  --mid-kind free \
  --mid-len-range 3..3 \
  --inner-zero-range 0..10 \
  --inner-slot 1 \
  --outer-layers 0:1 \
  --samples 100000 \
  --allowed-last-digits 1,3,7,9 \
  --out-csv "$OUTPUT"

echo ""
echo "✓ Results saved to: $OUTPUT"
echo ""
echo "Key columns to examine:"
echo "  - inner_zero: Amount of zero padding"
echo "  - prime_density: Observed density"
echo "  - divisible_counts: Track [3,5,7,11] divisibility"
echo "  - enrichment_factor: Ratio vs PNT baseline"
echo ""
echo "Expected observation:"
echo "  Density should INCREASE as inner_zero grows from 0→2→4,"
echo "  because zeros dilute digit-sum (divisibility by 3) and"
echo "  disrupt other modular patterns. Peak around inner_zero=1-3."
echo ""
echo "Note the divisibility by 3 (first tracked prime):"
echo "  Should decrease as zeros are added, allowing more primes."
echo ""
echo "Visualization: Open ../../viz/index.html and drop $OUTPUT"
