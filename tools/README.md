# Research Tools

This directory contains standalone research instruments for generating clean, reproducible datasets used in prime number analysis.

## Overview

These tools are **standalone** (zero external dependencies) and produce tidy CSV outputs with locked schemas. They serve as the canonical data sources for notebooks, visualizations, and statistical analyses.

### Relationship to Main Library

- **Library** (`src/hzlib/`): Reusable components (Hardy-Littlewood framework, statistics, sieves)
- **Tools** (`tools/`): Standalone research CLIs that generate datasets

Tools are independent of the main library to ensure:
- Reproducibility (no version conflicts)
- Portability (single-file distribution)
- Stability (locked CSV schemas for pipelines)

## prime_unified_cli

**Purpose**: Unified analysis of complementary-CRT patterns (Goldbach) and midpoint-density anomalies with quantile + wheel corrections.

### Build

```bash
cd tools
rustc prime_unified_cli.rs -O -o prime_unified
```

No external dependencies required - compiles with just `rustc`.

### Usage

#### Run All Analyses (Default)

```bash
./prime_unified --run=all --out-dir=./outputs
```

Creates:
- `outputs/ccrt_results.csv` - Complementary CRT / Goldbach coverage
- `outputs/mdr_results.csv` - Midpoint density analysis
- `outputs/SUMMARY.txt` - One-screen digest with key metrics

**Note**: `--run=all` includes CCRT and MDR but NOT N× transform (to avoid slowdown). Use `--run=ntransform` separately for N× analysis.

#### CCRT Analysis Only

Analyzes Goldbach pair coverage near bases with specific CRT zero patterns.

```bash
./prime_unified --run=ccrt \
  --ccrt-min-base=10 \
  --ccrt-max-base=500 \
  --ccrt-window=400 \
  --require-p-prime=1 \
  --out-dir=./ccrt_out
```

**Parameters**:
- `--ccrt-min-base`: Minimum base value (default: 10)
- `--ccrt-max-base`: Maximum base value (default: 500)
- `--ccrt-window`: Window size around 2×base (default: 400)
- `--require-p-prime`: Require base/2 to be prime (0/1, default: 0)

**Output Schema** (`ccrt_results.csv`):
```
base,pattern,honorary_zero,zeros,zero_count,n_evens,covered,total_pairs,coverage_rate,avg_pairs
```

- `base`: The base being analyzed
- `pattern`: Zero configuration (e.g., "3_and_11", "only_5")
- `honorary_zero`: base/2 value
- `zeros`: List of small primes dividing honorary_zero
- `zero_count`: Number of zeros in pattern
- `n_evens`: Number of even integers tested
- `covered`: Count with at least one Goldbach pair
- `total_pairs`: Total Goldbach pairs found
- `coverage_rate`: covered / n_evens
- `avg_pairs`: total_pairs / n_evens

**Patterns**:
- Single factors: `only_3`, `only_5`, `only_7`, `only_11`
- Complementary pairs: `3_and_11`, `5_and_7`
- Other pairs: `3_and_5`, `3_and_7`, `5_and_11`, `7_and_11`
- Triples: `3_5_7`, `3_5_11`, `3_7_11`, `5_7_11`

#### Midpoint Density Analysis Only

Analyzes prime density in symmetric windows around digit-block midpoints with PNT, quantile, and wheel corrections.

```bash
./prime_unified --run=mdr \
  --mdr-bases=6,10,30 \
  --mdr-targets=8,16,32 \
  --mdr-limit=200000000 \
  --q=30,210 \
  --tau=0.80 \
  --out-dir=./mdr_out
```

**Parameters**:
- `--mdr-bases`: Comma-separated bases to analyze (default: 6,10,30)
- `--mdr-targets`: Target prime counts (default: 8,16,32)
- `--mdr-limit`: Upper limit for analysis (default: 200000000)
- `--q`: Wheel moduli to test (default: 30,210)
- `--tau`: Quantile threshold (default: 0.80, range: 0.50-0.99)

**Output Schema** (`mdr_results.csv`):
```
base,k,low,high,mid,ln_mid,target,w_pred_plain,w_pred_tau,tau,ztau,
q,f_q_at_wpred,w_pred_wheel,w_min,prime_count_min,count_at_wpred,
expect_pnt_at_wpred,expect_wheel_at_wpred,ratio_w_over_pred,
ratio_w_over_wheel,chi2_int_res,chi2_prime_res
```

- `base`: Number base
- `k`: Digit length (power)
- `low`, `high`: Digit block boundaries [base^(k-1), base^k)
- `mid`: Midpoint (honorary zero)
- `ln_mid`: Natural log of midpoint
- `target`: Target prime count
- `w_pred_plain`: Plain PNT prediction for window width
- `w_pred_tau`: Quantile-corrected prediction
- `tau`, `ztau`: Quantile threshold and inverse-normal value
- `q`: Chosen wheel modulus
- `f_q_at_wpred`: Coprime fraction at predicted width
- `w_pred_wheel`: Wheel-corrected prediction
- `w_min`: Minimal window width achieving target
- `prime_count_min`: Actual prime count at w_min
- `count_at_wpred`: Prime count at predicted width
- `expect_pnt_at_wpred`: PNT expectation at predicted width
- `expect_wheel_at_wpred`: Wheel-corrected expectation
- `ratio_w_over_pred`: w_min / w_pred_plain
- `ratio_w_over_wheel`: w_min / w_pred_wheel
- `chi2_int_res`: Chi-squared for integer residues
- `chi2_prime_res`: Chi-squared for prime residues

### Summary Output

`SUMMARY.txt` provides a one-screen digest with key metrics:

**CCRT Summary**:
- Complementary vs single-factor comparison (Welch's t-test)
- Complementary vs non-complementary pairs
- Mean coverage rates by group

**MDR Summary** (per base/target combination):
- Slope of w_min vs ln(mid) regression
- Correlation coefficient
- Mean ratio of observed/predicted widths
- Correlations with chi-squared residue statistics

### Examples

```bash
# Quick test run with small limits
./prime_unified --run=all \
  --ccrt-max-base=100 \
  --mdr-limit=10000000 \
  --out-dir=./test_out

# Focus on complementary patterns with strict p-prime requirement
./prime_unified --run=ccrt \
  --ccrt-min-base=50 \
  --ccrt-max-base=200 \
  --require-p-prime=1 \
  --out-dir=./comp_analysis

# Deep midpoint analysis for base 6
./prime_unified --run=mdr \
  --mdr-bases=6 \
  --mdr-targets=4,8,16,32,64 \
  --mdr-limit=500000000 \
  --tau=0.75 \
  --out-dir=./base6_deep
```

#### N× Transform Analysis Only

Analyzes N× transform residue patterns and integer vertex distributions to test the MZR hypothesis.

```bash
./prime_unified --run=ntransform \
  --ntransform-bases=106,998,210 \
  --ntransform-N=3 \
  --ntransform-detail=1 \
  --out-dir=./ntransform_out
```

**Parameters**:
- `--ntransform-bases`: Comma-separated bases to analyze (default: 106,998)
- `--ntransform-N`: N× transform value (default: 3)
- `--ntransform-detail`: Generate per-remainder detail CSV (0/1, default: 0)

**Output Schema** (`ntransform_summary.csv`):
```
B,N,modulo,gcd_BN,integer_k_entropy_bits,integer_k_support,integer_k_uniformity,N3_trio_universal
```

- `B`: Base being analyzed
- `N`: N× transform value
- `modulo`: Range of remainders tested (p = B/2 for even B)
- `gcd_BN`: gcd(B, N)
- `integer_k_entropy_bits`: Shannon entropy of k_int distribution
- `integer_k_support`: Number of distinct k values that can be integer vertex
- `integer_k_uniformity`: 1 if all k values equally likely, 0 otherwise
- `N3_trio_universal`: 1 if N=3 and 3∤B (universal {0,⅓,⅔} property), 0 otherwise

**Detail Output Schema** (`ntransform_detail.csv` - when `--ntransform-detail=1`):
```
B,N,r,gcd_BN,k0_residue,k1_residue,k2_residue,integer_vertex_k,distinct_residue_count
```

- `r`: Remainder value
- `k0_residue`, `k1_residue`, `k2_residue`: Residues at vertex positions 0, 1, 2
- `integer_vertex_k`: Which k ∈ {0,1,2} makes (r+kB)/N an integer (empty if none)
- `distinct_residue_count`: Number of unique residues among the N vertices

**Mathematical Background**:

For base B and N× transform with remainder r, we compute N fractional vertices:
```
(r + k·B) / N    for k = 0, 1, ..., N-1
```

**Key Properties**:
- If gcd(B,N) = 1, all N residues {0, 1/N, 2/N, ..., (N-1)/N} appear for every r
- For N=3 and 3∤B: residues are exactly {0, ⅓, ⅔} (universal trio property)
- Exactly one k makes the vertex an integer: k_int ≡ -r·B⁻¹ (mod N)

**MZR Hypothesis**:
The MZR selection rule (r ≈ 0.4×HZ) may bias which vertex k becomes the integer one. The entropy and uniformity metrics test whether MZR concentrates on specific k values, which could explain downstream patterns in CCRT coverage or MDR density inflation.

## density-explorer

**Purpose**: Grid-based prime density exploration with sophisticated moduli auto-selection and statistical analysis.

### Build

```bash
cd tools/density-explorer
cargo build --release
```

### Key Features

- **Grid exploration**: Sample (mid_len, inner_zero) parameter spaces
- **Auto-track moduli**: Automatic selection of optimal moduli for tracking (Global/PerCell modes)
- **Model generation**: Compute expected densities without sampling
- **Explain mode**: Generate detailed obstruction analysis (union_p_any, per-prime P0)
- **Interactive viewer**: Web-based overlay visualization (overlay_v2.html)

### Usage

```bash
# Sample a grid
./target/release/density-explorer --base 14 grid \
  --mid-kind free --mid-len-range 1:5 --inner-zero-range 0:3 \
  --samples 10000 --auto-track --auto-mode global \
  --out-csv grid_sample.csv

# Generate model predictions
./target/release/density-explorer --base 14 model-only \
  --mid-kind free --mid-len-range 1:5 --inner-zero-range 0:3 \
  --auto-track --auto-mode global \
  --out-csv grid_model.csv

# Explain obstructions
./target/release/density-explorer --base 14 explain-grid \
  --mid-kind free --mid-len-range 1:5 --inner-zero-range 0:3 \
  --auto-track --auto-mode global \
  --out-json grid_explain.json
```

### Visualization

Open `viewer/overlay_v2.html` in a browser and load:
- `grid_sample.csv` (required)
- `grid_model.csv` (required)
- `grid_explain.json` (required)

**Interactive features**:
- 7 map modes: Obs, Pred, Δ abs, Δ enrichment, Union(any), per-prime P0, A→B compare
- Quantile/absolute clamping for outlier-robust color scaling
- Pin cells (P key) and see row/col lineouts comparing obs vs pred
- Export PNG (S key) and CSV (E key) for reproducibility
- CI fade: confidence intervals visualized as opacity

**Formal Specification**: The residue counting model is formalized in Agda:
- `agda-proofs/Specs/SpacingResidueModel.agda` - executable DP spec
- `agda-proofs/Specs/Tests.agda` - regression tests (DP vs enumeration)
- See [agda-proofs/README.md](../../agda-proofs/README.md#executable-specification-layer)

## hz

**Purpose**: Post-processing CLI for prime-density grid analysis with Fourier, polynomial fitting, and verification.

### Build

```bash
cd tools/hz
cargo build --release
```

### Subcommands

#### verify
Join sample+model grids and write verification table with Δ, enrichment, CI, and top moduli.

```bash
./target/release/hz verify \
  --sample grid_sample.csv \
  --model grid_model.csv \
  --explain grid_explain.json \
  --out verification_results.csv
```

#### overtones
Discrete Fourier spectrum of a lineout (obs|pred|enrichment).

```bash
./target/release/hz overtones \
  --sample grid_sample.csv --model grid_model.csv \
  --axis mid --fixed 0 --quantity enrichment \
  --topk 8 --out hz_out/overtones.csv
```

#### lagrange
Lagrange/Newton polynomial fit of a lineout.

```bash
./target/release/hz lagrange \
  --sample grid_sample.csv --model grid_model.csv \
  --axis mid --fixed 0 --quantity enrichment \
  --degree 5 --out hz_out/lagrange_lineout.csv
```

#### ridge
Trough/ridge detection along chosen axis.

```bash
./target/release/hz ridge \
  --sample grid_sample.csv --model grid_model.csv \
  --axis mid --quantity pred \
  --out hz_out/ridge.csv
```

#### lineout
Emit lineout table: x, obs, pred, enrichment.

```bash
./target/release/hz lineout \
  --sample grid_sample.csv --model grid_model.csv \
  --axis mid --fixed 0 \
  --out hz_out/lineout.csv
```

#### compare
Compare two sample CSVs (A vs B) with per-cell deltas and top changes.

```bash
./target/release/hz compare \
  --sample-a grid_sample_before.csv \
  --sample-b grid_sample_after.csv \
  --out hz_out/compare.csv --top 20
```

**Output**: CSV with deltas + console report of top 20 changes by |delta|.

## Data Pipeline

The CSV outputs are designed to be consumed by:

1. **Notebooks**: Pandas/R data frames for statistical analysis
2. **Visualization**: Plot generators (matplotlib, ggplot, etc.)
3. **Further tools**: Input to membrane correlation analysis (future)

### Schema Stability

CSV column names and order are **locked** for each tool version. Changes will be:
- Documented in changelog
- Versioned (tool outputs include version info in SUMMARY.txt)
- Backward-compatible when possible (new columns appended)

## Development Notes

### Why Standalone?

- **No dependency conflicts**: Works regardless of main library version
- **Reproducibility**: Same source file = same results years later
- **Distribution**: Single file can be shared, archived, or embedded
- **Performance**: No overhead from unused library features

### Relationship to Experimental Examples

These tools **supersede** the experimental examples:
- `examples/experimental/goldbach_hl_analysis.rs` → covered by `--run=ccrt`
- `examples/experimental/hz_phase2_density.rs` → covered by `--run=mdr`

The HL framework in `src/hzlib/` remains as a library for other uses.

## Future Enhancements

Planned additions (not yet implemented):

1. **Cramér baseline mode**: Generate synthetic primes under random model
2. **Membrane integration**: Third subcommand correlating membrane success with density anomalies
3. **Versioned outputs**: Embed tool version and parameters in output files
4. **CI integration**: Automated regression testing of CSV schemas

## Contributing

When adding new tools:

1. Keep them standalone (zero external dependencies)
2. Lock CSV schemas and document in this README
3. Include usage examples
4. Update main README.md to reference new tool
5. Add to CI test suite (once established)
