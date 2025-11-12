# Quickstart

## 1) Build
```bash
cargo build --release -p density-explorer -p hz
```

## 2) Generate a grid + model + explain

```bash
tools/density-explorer/target/release/density-explorer run \
  --config tools/density-explorer/experiments/overlay.toml
```

Artifacts:
- `tools/density-explorer/out/grid_sample.csv`
- `tools/density-explorer/out/grid_model.csv`
- `tools/density-explorer/out/grid_explain.json`

## 3) Verify + explore

```bash
tools/target/release/hz verify \
  --sample tools/density-explorer/out/grid_sample.csv \
  --model  tools/density-explorer/out/grid_model.csv \
  --explain tools/density-explorer/out/grid_explain.json \
  --out hz_out/verification_results.csv

tools/target/release/hz lineout \
  --sample tools/density-explorer/out/grid_sample.csv \
  --model  tools/density-explorer/out/grid_model.csv \
  --axis mid --fixed 0 --out hz_out/lineout.csv
```

## 4) Visualize

Open `tools/density-explorer/viewer/overlay.html` and load the three files.

## 5) Compare two runs

```bash
tools/target/release/hz compare \
  --sample-a tools/density-explorer/out/grid_sample_A.csv \
  --sample-b tools/density-explorer/out/grid_sample_B.csv \
  --out hz_out/compare.csv
```
