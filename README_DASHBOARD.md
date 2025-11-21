# Prime Physics Engine - Interactive Dashboard

**Version**: 1.0.0
**Type**: Single-file HTML5 application
**Dependencies**: D3.js, Plotly.js, Papa Parse (loaded via CDN)

## Overview

The Prime Physics Engine Dashboard is a comprehensive visualization and analysis tool for exploring membrane prime generation data. It provides interactive charts, filtering capabilities, and a sophisticated density grid viewer—all in a single HTML file that runs entirely in your browser.

## Quick Start

1. **Open the Dashboard**:
   ```bash
   # From the prime-physics-engine directory
   open dashboard.html
   # or
   firefox dashboard.html
   # or
   chromium dashboard.html
   ```

2. **Load Data Files**:
   - Click file upload buttons in the header
   - Or use tab-specific file inputs for specialized visualizations

3. **Explore**:
   - Navigate between tabs using the navigation bar
   - Apply filters in the sidebar
   - Hover over charts for detailed tooltips
   - Export data/images as needed

## Dashboard Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         HEADER                              │
│  Title | File Upload | Provenance Info                     │
├─────────────┬───────────────────────────────────────────────┤
│             │                                               │
│   SIDEBAR   │              MAIN CONTENT                     │
│   Filters   │     (Tab-specific visualizations)            │
│             │                                               │
│   • Base    │  Overview | Membranes | Density              │
│   • M range │  Cross-Base | Data Explorer                  │
│   • k range │                                               │
│   • Coprime │                                               │
│   • Options │                                               │
│             │                                               │
└─────────────┴───────────────────────────────────────────────┘
```

## Tabs and Features

### 1. Overview Tab

**Purpose**: High-level statistics and welcome screen

**Displays**:
- Total configurations tested
- Prime density range
- Data provenance (file names, timestamps)

**Usage**:
- Default landing page
- Check data is loaded correctly
- Verify file sources

### 2. Membranes Tab

**Purpose**: Analyze membrane structure performance

**Visualizations**:
1. **Elbow Dynamics**: Prime density ρ vs k (zero padding) for each M
   - Each line = different middle length M
   - Optimal k* marked with white circle
   - Shows k* migration as M increases

2. **Density Heatmap**: (M, k) → ρ landscape
   - Rows = M values
   - Columns = k values
   - Color intensity = prime density
   - Annotated with exact density values

**Key Files**:
- `membrane_density_summary.csv`: Per-(M,k) configuration stats
- `membrane_density_detail.csv`: Individual seed results

**Example Data Structure** (summary.csv):
```csv
base,outer,inner,M,k,total_candidates,prime_count,density,avg_positive_legendre
6,1,5,2,0,100,33,0.330000,4.5
6,1,5,2,1,100,28,0.280000,4.2
```

**Interpretation**:
- **Elbow events**: When k* increases with M (indicates phase transition)
- **Density hotspots**: Configurations with highest success rates
- **Legendre correlation**: Higher avg_positive_legendre often predicts better density

### 3. Density Tab (Grid Viewer)

**Purpose**: Sophisticated density analysis with 7 visualization modes

**File Inputs** (all loaded separately in this tab):
- **Sample A CSV** (required): Observed density data from `density-explorer`
- **Model CSV** (required): Predicted density from Hardy-Littlewood theory
- **Explain JSON** (optional): Per-prime P0 contributions
- **Sample B CSV** (optional): Second dataset for A→B comparison

**7 Visualization Modes**:

| Mode | Formula | Use Case |
|------|---------|----------|
| **Δ enrichment** | `(A/pred − 1)` | Show relative over/under-representation |
| **Δ absolute** | `(A − pred)` | Direct difference between obs and pred |
| **Observed A** | `A` | Raw prime density from data |
| **Predicted** | `pred` | Model prediction (Hardy-Littlewood) |
| **Union(any)** | `P(∃p: p divides)` | Combined small-mod obstruction |
| **Per-prime P0** | `P0(p)` | Individual prime's contribution |
| **A→B delta** | `(obsB − obsA)` | Compare two datasets |

**Controls**:

```
Grid Info:
  base         14
  mid_len      2..10
  inner_zero   0..6
  cells        63

Map Controls:
  mode         [Δ enrichment ▼]
  prime        [—▼] (for P0 mode)
  clamp        [quantile ▼] [0.98]

Render Options:
  cell size    [========] 16px
  grid lines   [✓]
  fade by CI   [✓]
  crosshair    [✓]
```

**Clamp Modes**:
- **Quantile** (default 0.98): Keep central 2–98% of values, clip outliers
  - Good for noisy data with extreme outliers
  - Ensures most cells visible with good contrast
- **Absolute** (symmetric): `±max(|Δ|) × multiplier`
  - Good for symmetric deviations
  - Preserves exact zero midpoint

**Color Schemes**:
- **Δ modes** (enrichment, absolute, A→B): Diverging teal ↔ rose
  - Teal = model > sample (under-represented)
  - Rose = sample > model (over-represented)
  - Gray = perfect match
- **Monotone modes** (obs, pred, union, P0): Grayscale ramp
  - Dark = low density
  - Light = high density

**Interactive Features**:

| Action | Result |
|--------|--------|
| **Hover over cell** | Shows detailed stats in sidebar, updates lineout charts |
| **Drag** | Pan the grid |
| **Alt + Wheel** | Zoom (0.5× to 2.0×) |
| **Press P** | Pin current cell (locks selection) |
| **Press S** | Save heatmap as PNG |
| **Press E** | Export current map as CSV |

**Lineout Charts** (bottom panel):
- **Left**: mid_len variation @ fixed inner_zero
- **Right**: inner_zero variation @ fixed mid_len
- Blue line = predicted density
- Green line = observed density
- Updates in real-time on hover

**Inspect Panel** (sidebar):
```
mid_len        5
inner_zero     2
obs(A)         0.245%
pred           0.238%
enrichment     +2.94%
CI             0.232% .. 0.258%

dominant small-mod contributors:
  p=3          0.667%
  p=5          0.800%
  p=7          0.857%
  union(any)   0.238%
```

**Example Workflow**:
1. Load `tools/density-explorer/output/grid_sample_base14.csv` as **Sample A**
2. Load `tools/density-explorer/output/grid_model_base14.csv` as **Model**
3. Load `tools/density-explorer/output/explain_base14.json` as **Explain**
4. Select mode: **Δ enrichment**
5. Hover cells to find hotspots (rose = higher than predicted)
6. Switch to **Per-prime P0** mode
7. Select prime: **3** to see p=3 contribution map
8. Press **P** to pin interesting cell
9. Press **S** to save visualization

### 4. Cross-Base Tab

**Purpose**: Compare optimal configurations across different number bases

**Visualizations**:
1. **iz* Evolution**: Optimal inner-zero vs middle length M for each base
   - Shows universal patterns vs base-specific behavior
   - Identifies migrating configurations

2. **Base Correlation Heatmap**: Similarity matrix between bases
   - High correlation = similar optimal strategies
   - Low correlation = base-specific phenomena

**Key Files**:
- `ridge_base6.csv`, `ridge_base10.csv`, `ridge_base14.csv`, etc.

**Example Data Structure**:
```csv
base,mid_len,iz_best,goldbach_prob,expected_density_global
6,2,0,0.9876,0.0045
6,3,0,0.9912,0.0038
```

**Interpretation**:
- **Universal patterns**: Configurations optimal across multiple bases
- **Base-specific peaks**: Unique to certain factorization structures
- **Goldbach correlation**: Higher prob often → better membrane performance

### 5. Data Explorer Tab

**Purpose**: Raw data table with search and filtering

**Features**:
- **Sortable columns**: Click headers to sort
- **Pagination**: 50 rows per page
- **Search**: Filter by any column value
- **Actions**:
  - **Verify Prime**: Opens WolframAlpha to verify primality
  - **Export Filtered**: Download current view as CSV

**Example Use Cases**:
- Find all base=6 configurations with density > 0.30
- Export M=3 configurations for external analysis
- Verify specific membrane values are prime

## Filtering System

**Available Filters** (sidebar):

```
Base:           [6, 10, 12, 14, 15, 18, 22, 30]
                (multi-select)

M range:        [1] ─────────────── [10]
k range:        [0] ─────────────── [5]

Options:
  [✓] Coprime only (gcd(outer, base) = 1)
  [ ] Prime seeds only
  [ ] Elbow events only

[Apply Filters]  [Reset]
```

**How Filters Work**:
1. User selects criteria in sidebar
2. Click **Apply Filters**
3. All loaded datasets filtered simultaneously
4. Visualizations update to show only matching data
5. Original data preserved (can reset anytime)

**Filter Logic**:
- **Coprime filter**: Uses GCD algorithm to check `gcd(outer, base) = 1`
- **Elbow events**: Configurations where k* changes with M
- **Ranges**: Inclusive endpoints [min, max]

**Example Filtering Workflow**:
1. Load `membrane_density_summary.csv`
2. Select bases: **6, 10, 14**
3. Set M range: **2 to 5**
4. Check **Coprime only**
5. Click **Apply Filters**
6. View results in Membranes tab
7. Export filtered data from Data Explorer

## File Formats

### 1. Membrane Density Summary CSV

**Columns**:
```
base,outer,inner,M,k,total_candidates,prime_count,density,avg_positive_legendre
```

**Example**:
```csv
base,outer,inner,M,k,total_candidates,prime_count,density,avg_positive_legendre
6,1,5,2,0,100,33,0.330000,4.5
10,3,7,3,1,100,18,0.180000,3.8
```

**Generated by**:
```bash
cargo run --example proper_membrane_generator
# or
python visualizations/membrane_density_sandbox.py
```

### 2. Membrane Density Detail CSV

**Columns**:
```
base,outer,inner,M,k,seed,membrane_value,is_prime,discriminant,legendre_3,legendre_5,...
```

**Example**:
```csv
base,outer,inner,M,k,seed,membrane_value,is_prime,discriminant
6,1,5,2,0,3,15351,true,-75
6,1,5,2,0,4,15451,true,-100
```

**Use**: Per-seed analysis, discriminant studies

### 3. Grid Sample CSV (density-explorer)

**Columns**:
```
base,mid_len,inner_zero,prime_density,ci_lo,ci_hi,expected_density_local,expected_density_local_exact
```

**Example**:
```csv
base,mid_len,inner_zero,prime_density,ci_lo,ci_hi,expected_density_local_exact
14,2,0,0.002456,0.002401,0.002511,0.002438
14,2,1,0.002389,0.002335,0.002443,0.002438
```

**Generated by**:
```bash
cargo run --release --bin density-explorer -- --base 14 grid --mid 2:10 --iz 0:6
```

### 4. Grid Model CSV (density-explorer)

Same format as Grid Sample CSV, but contains only model predictions (no observed data).

### 5. Explain JSON (density-explorer)

**Structure**:
```json
[
  {
    "mid_len": 2,
    "inner_zero": 0,
    "union_p_any": 0.238,
    "model_p0": [
      [3, 0.667],
      [5, 0.800],
      [7, 0.857]
    ]
  }
]
```

**Generated by**:
```bash
cargo run --release --bin density-explorer -- --base 14 grid --mid 2:10 --iz 0:6 --explain
```

### 6. Ridge CSV (cross-base analysis)

**Columns**:
```
base,mid_len,iz_best,goldbach_prob,expected_density_global,ridge_density
```

**Example**:
```csv
base,mid_len,iz_best,goldbach_prob,expected_density_global,ridge_density
6,2,0,0.9876,0.0045,0.0052
6,3,0,0.9912,0.0038,0.0041
```

**Generated by**:
```bash
cargo run --example phase1_cross_base_validation
# or custom ridge analysis scripts
```

## Keyboard Shortcuts

### Global Shortcuts

| Key | Action |
|-----|--------|
| **Tab** | Navigate between tabs (when focused on nav) |
| **Ctrl+F** | Focus search box (Data Explorer) |
| **Ctrl+R** | Reset filters |

### Density Tab Shortcuts

| Key | Action |
|-----|--------|
| **P** | Pin current hovered cell |
| **S** | Save heatmap as PNG |
| **E** | Export current map as CSV |
| **Arrow keys** | Navigate grid (when cell pinned) |
| **Escape** | Unpin cell |
| **Alt + Wheel** | Zoom in/out |

## Exporting Data

### 1. Export Filtered Data (Data Explorer)

```
1. Apply desired filters
2. Go to Data Explorer tab
3. Click "Export Filtered CSV"
4. File saved as: filtered_{original_filename}.csv
```

### 2. Export Density Map (Density Tab)

```
1. Configure desired visualization mode
2. Press 'E' key
3. File saved as: density_overlay_map.csv
```

**Exported CSV format**:
```csv
mid_len,inner_zero,value,obs,pred,ci_width
2,0,-0.0123,0.002456,0.002580,0.000110
2,1,0.0045,0.002389,0.002344,0.000108
```

### 3. Export Heatmap Image (Density Tab)

```
1. Configure desired visualization
2. Zoom/pan to interesting region
3. Press 'S' key
4. PNG saved as: density_overlay.png
```

## Prime Verification

**WolframAlpha Integration**:

1. Navigate to **Data Explorer** tab
2. Find row with `membrane_value` or `prime_value` column
3. Click **Verify Prime** button in Actions column
4. Opens: `https://www.wolframalpha.com/input?i=is+{value}+prime`
5. WolframAlpha confirms primality

**Example**:
```
membrane_value: 300705070003
Click "Verify Prime" →
WolframAlpha: "300705070003 is prime" ✓
```

## Performance Considerations

### Loading Large Datasets

**File Size Guidelines**:
- Small: < 1 MB → Instant loading
- Medium: 1–10 MB → 1–5 seconds
- Large: 10–50 MB → 5–30 seconds
- Very large: > 50 MB → Consider filtering at source

**Optimization Tips**:
1. **Filter before export**: Generate smaller CSVs with targeted parameters
   ```bash
   # Instead of full grid 2:100
   density-explorer --mid 2:10  # Smaller range
   ```

2. **Use appropriate clamp values**: Quantile 0.95 vs 0.99 reduces outliers

3. **Disable CI fade**: Faster rendering for large grids

4. **Reduce cell size**: 8px cells render faster than 28px

### Browser Compatibility

**Recommended Browsers**:
- Chrome/Chromium 90+
- Firefox 88+
- Safari 14+
- Edge 90+

**Required Features**:
- ES6+ JavaScript (arrow functions, template literals, Map/Set)
- Canvas 2D API
- FileReader API
- CSS Grid

**Known Issues**:
- Safari: Alt+Wheel zoom may conflict with native gestures (use trackpad pinch)
- Firefox: Large canvas (>16384px) may clip (zoom out if grid disappears)

## Troubleshooting

### "Load sample+model+explain." stuck

**Problem**: Density tab shows loading message but no visualization

**Solutions**:
1. Check both **Sample A** and **Model** CSVs are loaded (both required)
2. Verify CSV format matches expected columns:
   ```
   base,mid_len,inner_zero,prime_density,...
   ```
3. Open browser console (F12) for error messages
4. Try reloading files (click file input again)

### Filters not working

**Problem**: Applied filters don't change visualizations

**Solutions**:
1. Click **Apply Filters** button (not automatic)
2. Check filter ranges include existing data:
   ```
   If data has M=2..5, don't filter M=6..10
   ```
3. Reset filters and try again
4. Reload page and re-upload files

### Heatmap rendering issues

**Problem**: Grid appears black, distorted, or clipped

**Solutions**:
1. **All black**: No data loaded, or all values out of clamp range
   - Adjust clamp value (try 0.90 instead of 0.98)
   - Switch clamp mode (quantile ↔ absolute)

2. **Clipped edges**: Canvas too large for browser
   - Zoom out (Alt+Wheel down)
   - Reduce cell size slider
   - Use Firefox/Chrome (Safari has 4096px limit)

3. **Slow rendering**: Too many cells
   - Filter to smaller mid_len/inner_zero range at source
   - Reduce cell size
   - Disable grid lines and CI fade

### Export not working

**Problem**: Clicking export does nothing or downloads empty file

**Solutions**:
1. **Empty CSV**: No data filtered/loaded
   - Apply filters to include data
   - Check Data Explorer shows rows

2. **PNG issues**: Browser blocking download
   - Check popup blocker settings
   - Try right-click canvas → Save Image As

3. **Filename issues**: Browser auto-renaming
   - Files saved to default Downloads folder
   - May append (1), (2) if name collision

## Advanced Usage

### Custom CSV Generation

Create custom datasets for dashboard:

```python
import pandas as pd

# Generate synthetic membrane results
data = []
for base in [6, 10, 14]:
    for M in range(2, 6):
        for k in range(0, 3):
            data.append({
                'base': base,
                'outer': 1,
                'inner': 5,
                'M': M,
                'k': k,
                'total_candidates': 100,
                'prime_count': 33 - k*5,  # Mock: k=0 best
                'density': (33 - k*5) / 100,
                'avg_positive_legendre': 4.5 - k*0.3
            })

df = pd.DataFrame(data)
df.to_csv('custom_membrane_summary.csv', index=False)
```

Then load in dashboard:
```
1. Open dashboard.html
2. Click "Upload Data Files"
3. Select custom_membrane_summary.csv
4. Explore in Membranes tab
```

### Combining Multiple Experiments

Merge CSVs from different runs:

```bash
# Combine multiple membrane experiments
head -1 experiment1_summary.csv > combined.csv
tail -n +2 experiment1_summary.csv >> combined.csv
tail -n +2 experiment2_summary.csv >> combined.csv
tail -n +2 experiment3_summary.csv >> combined.csv

# Load combined.csv in dashboard
```

### Scripted Analysis

Use dashboard as final visualization step:

```bash
#!/bin/bash
# Run analysis pipeline, output to dashboard-ready CSVs

# Step 1: Generate membrane data
cargo run --example proper_membrane_generator > mem_summary.csv

# Step 2: Generate density grids
density-explorer --base 14 grid --mid 2:10 --iz 0:6 --sample > grid_sample.csv
density-explorer --base 14 grid --mid 2:10 --iz 0:6 --model > grid_model.csv
density-explorer --base 14 grid --mid 2:10 --iz 0:6 --explain > explain.json

# Step 3: Generate cross-base data
for base in 6 10 14 30; do
    cargo run --example ridge_analysis -- --base $base > ridge_base${base}.csv
done

# Step 4: Open dashboard
echo "Load these files in dashboard.html:"
echo "  - mem_summary.csv (Membranes tab)"
echo "  - grid_sample.csv, grid_model.csv, explain.json (Density tab)"
echo "  - ridge_base*.csv (Cross-Base tab)"
open dashboard.html
```

## Examples

### Example 1: Finding Optimal Base 6 Configurations

```
Goal: Find best (outer, inner, k) for base 6

1. Generate data:
   cargo run --example base6_membrane_sweep > base6_results.csv

2. Load in dashboard, go to Membranes tab

3. Filter:
   - Base: 6
   - M range: 2–4
   - Coprime only: ✓
   - Apply Filters

4. Observe heatmap:
   - Hottest cell = (M=2, k=0): density 0.330
   - All M values: k*=0 (minimal padding wins)

5. Export filtered data for further analysis
```

**Result**: Base 6, (1,5), k=(0,0) achieves 33% density

### Example 2: Comparing Hardy-Littlewood Predictions

```
Goal: Validate model accuracy for base 14

1. Generate observed data:
   density-explorer --base 14 grid --mid 2:10 --iz 0:6 --sample > obs.csv

2. Generate predicted data:
   density-explorer --base 14 grid --mid 2:10 --iz 0:6 --model > pred.csv

3. Load both in Density tab

4. Select mode: "Δ enrichment (A/pred − 1)"

5. Apply quantile clamp: 0.98

6. Observe:
   - Mostly gray cells = good prediction
   - Rose hotspots = over-represented (discovery!)
   - Teal coldspots = under-represented (obstacles?)

7. Pin interesting rose cell, press 'S' to save

8. Switch to "Per-prime P0" mode, select prime=3
   - See if p=3 obstruction explains pattern
```

**Result**: Identified systematic enrichment at (mid=5, iz=2)

### Example 3: Cross-Base Universal Patterns

```
Goal: Find configurations optimal across multiple bases

1. Generate ridge data for bases 6, 10, 14, 18, 30

2. Load all ridge_base*.csv files

3. Go to Cross-Base tab

4. Observe iz* evolution chart:
   - All bases converge to iz*=0 at M≥3
   - Universal minimal padding principle

5. Check correlation heatmap:
   - Bases 6, 14, 18 highly correlated (ρ > 0.85)
   - Base 30 unique (ρ < 0.60) - highly composite anomaly

6. Export correlation data for statistical analysis
```

**Result**: iz*=0 is universal for M≥3 across tested bases

## Data Provenance Tracking

**Automatic Metadata Capture**:

When files are loaded, dashboard records:
- Original filename
- Load timestamp
- File size
- Row count

**Provenance Display** (header):
```
📊 Data Sources:
  membrane_density_summary.csv (2,450 rows, loaded 2025-11-21 10:23:45)
  grid_sample_base14.csv (63 rows, loaded 2025-11-21 10:24:12)
  grid_model_base14.csv (63 rows, loaded 2025-11-21 10:24:15)
```

**Best Practices**:
1. Use descriptive filenames:
   ```
   Good: membrane_base6_M2-5_k0-3_2025-11-21.csv
   Bad:  results.csv
   ```

2. Include parameters in filename:
   ```
   grid_sample_base14_mid2-10_iz0-6.csv
   ```

3. Version outputs:
   ```
   experiment_v1.csv
   experiment_v2_fixed.csv
   experiment_v3_final.csv
   ```

## Contributing Data

To share results via dashboard:

1. **Generate reproducible CSVs**:
   ```bash
   # Document exact command
   density-explorer --base 14 --mid 2:10 --iz 0:6 > grid_base14.csv
   ```

2. **Include metadata file**:
   ```yaml
   # metadata.yaml
   date: 2025-11-21
   author: Your Name
   base: 14
   mid_len_range: [2, 10]
   inner_zero_range: [0, 6]
   sample_size: 10000
   rust_version: 1.88.0
   commit: a1b2c3d
   ```

3. **Package for sharing**:
   ```bash
   zip experiment_base14.zip \
       grid_sample.csv \
       grid_model.csv \
       explain.json \
       metadata.yaml \
       README.txt
   ```

4. **Document how to load**:
   ```
   # README.txt
   Load files in Prime Physics Engine dashboard.html:
   1. Density tab → Upload grid_sample.csv, grid_model.csv, explain.json
   2. Select mode: "Δ enrichment"
   3. Observe hotspots at (mid=5, iz=2) and (mid=7, iz=1)
   ```

## Appendix: File Location Reference

```
prime-physics-engine/
├── dashboard.html                          # Main dashboard (open this)
├── README_DASHBOARD.md                     # This file
├── visualizations/
│   ├── membrane_density_summary.csv        # Membranes tab
│   ├── membrane_density_detail.csv         # Membranes tab
│   ├── ridge_base*.csv                     # Cross-Base tab
│   └── *.png                               # Generated plots
├── tools/density-explorer/
│   ├── output/
│   │   ├── grid_sample_base*.csv          # Density tab (sample)
│   │   ├── grid_model_base*.csv           # Density tab (model)
│   │   └── explain_base*.json             # Density tab (explain)
│   └── viewer/
│       └── overlay_v2.html                 # Original standalone viewer
└── examples/
    ├── proper_membrane_generator.rs        # Generates summary CSV
    ├── phase1_cross_base_validation.rs     # Generates ridge CSVs
    └── lagrange_full_verification.rs       # Specialized examples

Generated files (gitignored):
  media/                                     # Exported PNGs
  animations/                                # Exported videos (future)
  filtered_*.csv                             # Exported filtered data
  density_overlay.png                        # Saved heatmaps
  density_overlay_map.csv                    # Exported density maps
```

## Support and Documentation

**Main Documentation**:
- [CLAUDE.md](./CLAUDE.md) - Executive summary of membrane prime research
- [EVIDENCE.md](./EVIDENCE.md) - Detailed proofs and verification
- [tools/README.md](./tools/README.md) - Density explorer CLI documentation

**Examples**:
```bash
# List all working examples
ls examples/*.rs | grep -v experimental

# Run verification report
cargo run --example prime_verification_report

# Test membrane generation
cargo run --example proper_membrane_generator
```

**Issues**:
- For dashboard bugs: Report at https://github.com/anthropics/claude-code/issues
- For membrane theory questions: See EVIDENCE.md Section 7 (Verification Infrastructure)

---

**Version History**:
- v1.0.0 (2025-11-21): Initial release with 5 tabs, filtering, density grid viewer
