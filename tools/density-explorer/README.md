# density-explorer

Prime-density sampling for *symmetric zero-padding templates* with optional palindromic mirroring.

## Build

```bash
cd tools/density-explorer
cargo build --release
```

## Commands

### 1) Sample a single pattern

Pattern is defined by:
- **midpoint**: either `free:<len>` (random digits) or `zeros:<len>` (fixed zeros)
- **layers**: inner-to-outer pairs `ZERO:SLOT`. Each adds ZERO zeros and then an open slot of SLOT digits on both sides.
- `--allowed-last-digits`: digits allowed in the last position. Default `1,3,7,9`.
- `--mirror`: if set, left digits mirror to the right (full palindrome).

Example: center has one free digit; inner zero-padding z with one-digit slot; plus two more outer one-digit slots.

```bash
# High-level probe
cargo run --release -- sample \
  --midpoint free:1 \
  --layers 0:1 1:1 0:1 \
  --samples 200000 \
  --allowed-last-digits 1,3,7,9
```

Palindromic variant (note: even total length → divisible by 11 → density ≈ 0):

```bash
cargo run --release -- sample \
  --midpoint free:1 \
  --layers 0:1 0:1 \
  --samples 200000 \
  --allowed-last-digits 1,3,7,9 \
  --mirror
```

### 2) Grid sweep (2D heatmap)

Sweep midpoint length and inner zero padding, keeping `inner_slot` fixed (and optional outer layers).

```bash
# Explore "rollover" (midpoint growth) vs inner zeros
cargo run --release -- grid \
  --mid_kind free \
  --mid_len_range 1..10 \
  --inner_zero_range 0..8 \
  --inner_slot 1 \
  --outer_layers 0:1 \
  --samples 50000 \
  --allowed-last-digits 1,3,7,9 \
  --out_csv grid.csv
```

Open `../viz/index.html` in your browser and drop `grid.csv` to see the interactive heatmap.

## Interactive Visualization 🎯

The enhanced heatmap (`tools/viz/index.html`) makes **enrichment factor the star of the show**:

### Features

**📊 Metric Toggle**
- **Enrichment Factor (default)**: See how many times better than random (2×, 3×, 4×!)
- **Prime Density**: Raw percentage of primes
- **Total Length**: Number of digits

**🔥 Smart Color Scales**
- Enrichment uses a heat map (red gradient) where darker = better performance
- 1× baseline marker shows where "random chance" sits
- Exceptional configs (3×+) stand out visually

**📈 Auto-Summary Dashboard**
- **Peak Enrichment**: Best factor achieved across all configs
- **Best Configuration**: Exactly which parameters gave peak results
- **Quick Stats**: Density ranges, sample counts

**💬 Rich Tooltips**
Hover any cell to see:
```
Configuration: mid_len=2, inner_zero=1
Total Length: 8 digits

📊 ENRICHMENT: ✨ 2.31× (STRONG)
   Beating random by 131%!

Prime Density: 13.36%
Expected (PNT): 5.79%
95% CI: [11.34%, 15.41%]

Samples: 10,000
Primes Found: 1,336
```

### Visual Impact

**Before**: "Here's a grid of numbers, good luck."
**After**: "This config beats random by 2.5×—exceptional performance!" 🎯

The enrichment-first view immediately answers: **"How much better is this than random chance?"**

## Notes

- **Arbitrary precision** via `num-bigint`. Deterministic Miller-Rabin for 64-bit; otherwise tests the fixed base set {2..37}.
- **Leading digit** is never 0. If `--mirror` is set, the leading digit is additionally constrained to satisfy the last-digit set (so its mirror at the end is allowed).
- **Total digits** = midpoint_len + 2 * Σ (zero_i + slot_i).

## Examples

### Exploring the "rollover → density drop → padding recovery" phenomenon

1. **Midpoint rollover → density dip**

Keep layers `0:1 0:1` (two thin open rings). Sweep `mid_len 1..10`. You'll see density slip when `mid_len` increases (numbers get longer; 1/ln n) and, under `--mirror`, even/odd parity bands (even total digits ⇒ 11-wall).

2. **Add elbow-room → rebound**

Now sweep `inner_zero 0..8` (still `inner_slot=1`). You should see the ridge (higher density) shift as `inner_zero` increases; zeros dilute divisibility by 3 via digit-sum effects and disrupt unlucky residue alignments. The heatmap ridge is the "sine" moving away as `mid_len` grows and returning as `inner_zero` expands.

3. **Small-prime diagnostics**

With `--track_primes 3,5,7,11`, compare `divisible_counts / samples` across the grid; spikes in 11 or 3 correlate with density troughs.

## Visualization

The `tools/viz/index.html` file provides an interactive heatmap visualization:
- Drag and drop your CSV file
- Axes: midpoint length vs inner zero padding
- Color: prime density (with confidence intervals in tooltips)
