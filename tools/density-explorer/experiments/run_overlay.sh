#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# Build if needed
if [ ! -f target/release/density-explorer ]; then
  cargo build --release -p density-explorer
fi

mkdir -p out
./target/release/density-explorer run --config experiments/overlay.toml

echo
echo "Outputs:"
echo "  - tools/density-explorer/out/grid_sample.csv"
echo "  - tools/density-explorer/out/grid_model.csv"
echo "  - tools/density-explorer/out/grid_explain.json"
echo
echo "Open the viewer: tools/density-explorer/viewer/overlay.html (no server needed)."
