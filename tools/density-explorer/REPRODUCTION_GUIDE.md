# Reproduction Guide for Density Explorer

This guide provides step-by-step instructions to reproduce figures and analyses from the Lagrange Points research.

## Prerequisites

- Rust 1.88+ installed
- ~15 minutes for full analysis
- ~2GB disk space for output data

## Quick Start (5 Minutes)

### 1. Build the Tool

```bash
cd tools/density-explorer
cargo build --release
```

### 2. Run Basic Sampling

Generate a quick sample with seed for reproducibility:

```bash
# Set fixed RNG seed for reproducibility
export RUST_RANDOM_SEED=42

cargo run --release -- sample \
  --midpoint free:1 \
  --layers 0:1 1:1 0:1 \
  --samples 10000 \
  --allowed-last-digits 1,3,7,9
```

**Expected Output**: Prime density ~10-15% (vs ~5% random baseline)

### 3. Generate Heatmap Data

```bash
cargo run --release -- grid \
  --mid_kind free \
  --mid_len_range 1..10 \
  --inner_zero_range 0..8 \
  --inner_slot 1 \
  --outer_layers 0:1 \
  --samples 10000 \
  --allowed-last-digits 1,3,7,9 \
  --out_csv /tmp/density_grid.csv
```

### 4. Visualize Results

```bash
# Open the visualization in browser
xdg-open ../viz/index.html  # Linux
open ../viz/index.html       # macOS

# Then drag-and-drop /tmp/density_grid.csv into the browser
```

## Full Analysis Pipeline

### Configuration 1: Rollover Study

Study how prime density changes as numbers grow:

```bash
cargo run --release -- grid \
  --mid_kind free \
  --mid_len_range 1..15 \
  --inner_zero_range 0..12 \
  --inner_slot 1 \
  --outer_layers 0:1 0:1 \
  --samples 50000 \
  --allowed-last-digits 1,3,7,9 \
  --out_csv rollover_study.csv
```

**Runtime**: ~8 minutes
**Output**: `rollover_study.csv` (heatmap data)

### Configuration 2: Palindromic Patterns

Explore mirror-symmetric patterns:

```bash
cargo run --release -- grid \
  --mid_kind free \
  --mid_len_range 1..10 \
  --inner_zero_range 0..8 \
  --inner_slot 1 \
  --outer_layers 0:1 \
  --samples 50000 \
  --allowed-last-digits 1,3,7,9 \
  --mirror \
  --out_csv palindrome_study.csv
```

**Runtime**: ~8 minutes
**Output**: `palindrome_study.csv`

### Configuration 3: Diagnostic Mode

Track specific prime divisors:

```bash
cargo run --release -- grid \
  --mid_kind free \
  --mid_len_range 1..8 \
  --inner_zero_range 0..6 \
  --inner_slot 1 \
  --outer_layers 0:1 \
  --samples 20000 \
  --allowed-last-digits 1,3,7,9 \
  --track_primes 3,5,7,11 \
  --out_csv diagnostic.csv
```

**Runtime**: ~5 minutes
**Output**: `diagnostic.csv` with divisibility counts

## Reproducibility Notes

### Random Number Generation

For **deterministic** results across runs:

```bash
export RUST_RANDOM_SEED=42
```

Without this, results will vary slightly due to random sampling.

### Sample Size vs Runtime Tradeoff

| Samples | Runtime (per config) | Confidence Interval Width |
|---------|---------------------|---------------------------|
| 1,000   | ~1s                 | ±3%                       |
| 10,000  | ~3s                 | ±1%                       |
| 50,000  | ~15s                | ±0.5%                     |
| 100,000 | ~30s                | ±0.3%                     |

Use smaller samples for exploration, larger for publication figures.

### Verification

Compare your output with reference data:

```bash
# Check enrichment factors match expected ranges
head -20 rollover_study.csv

# Expected peak enrichment: 2.5-3.5×
# Expected baseline (large numbers): ~1.0×
```

## Output Formats

### CSV Schema

```csv
mid_len,inner_zero,prime_density,total_length,samples,primes,enrichment,ci_lower,ci_upper
1,0,0.1234,5,10000,1234,2.5,0.1150,0.1318
...
```

### Columns

- `mid_len`: Midpoint length (variable center)
- `inner_zero`: Inner zero-padding count
- `prime_density`: Fraction of primes found
- `enrichment`: Density / PNT_expected
- `ci_lower`/`ci_upper`: 95% confidence interval

## Troubleshooting

### "Out of memory" errors

Reduce samples or grid ranges:

```bash
--samples 5000
--mid_len_range 1..6
```

### Very slow execution

Check if running in debug mode:

```bash
cargo build --release  # Must use --release!
```

### No output file

Check write permissions:

```bash
touch /tmp/test.csv  # Should succeed
```

## Advanced: Batch Processing

Generate multiple configurations in parallel:

```bash
#!/bin/bash
# Run multiple configs concurrently
for inner_slot in 1 2 3; do
  cargo run --release -- grid \
    --mid_kind free \
    --mid_len_range 1..10 \
    --inner_zero_range 0..8 \
    --inner_slot $inner_slot \
    --samples 20000 \
    --out_csv "grid_slot_${inner_slot}.csv" &
done
wait
echo "All grids complete!"
```

## Reference Results

### Expected Benchmarks

Configuration: `mid_len=1, inner_zero=1, slot=1`

- Total length: 5 digits
- Expected density (PNT): ~6.2%
- Observed density: ~13-15%
- **Enrichment: 2.1-2.4×**

Configuration: `mid_len=5, inner_zero=0, slot=1`

- Total length: 9 digits
- Expected density (PNT): ~4.5%
- Observed density: ~4.5-5.5%
- **Enrichment: 1.0-1.2×** (padding helps!)

## Citation

If you use these tools in research, please cite:

```
Prime Construction through Symmetric Membrane Patterns
Collaborative Research, 2025
https://github.com/mikedotexe/primes
```

## Support

For questions or issues:
- File an issue at: https://github.com/mikedotexe/primes/issues
- Check existing documentation in `README.md`
- Review example scripts in `experiments/`
