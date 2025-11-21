# Dashboard Testing Checklist

**Version**: 1.0.0
**Date**: 2025-11-21
**Purpose**: Verify dashboard.html functionality with real data files

## Available Test Data

### Membrane Data (Membranes Tab)
```bash
✓ visualizations/membrane_density_summary.csv  (M,k sweep results)
✓ visualizations/membrane_density_detail.csv   (per-seed details)
```

### Ridge Data (Cross-Base Tab)
```bash
✓ visualizations/ridge_summary.csv
✓ visualizations/ridge_elbows.csv
✓ visualizations/ridges.csv
✓ tools/density-explorer/out/ridge_base6.csv
✓ tools/density-explorer/out/ridge_base10.csv
✓ tools/density-explorer/out/ridge_base12.csv
✓ tools/density-explorer/out/ridge_base30.csv
```

### Density Grid Data (Density Tab)
```bash
✓ tools/density-explorer/out/grid_sample.csv   (observed densities)
✓ tools/density-explorer/out/grid_model.csv    (predicted densities)
✓ tools/density-explorer/out/grid_explain.json (per-prime P0 contributions)
```

## Pre-Testing Setup

**Step 1: Open Dashboard**
```bash
cd /path/to/prime-physics-engine
open dashboard.html
# or
firefox dashboard.html
# or
chromium dashboard.html
```

**Step 2: Open Browser Console** (for debugging)
```
Chrome/Edge: F12 or Cmd+Option+I (Mac)
Firefox: F12 or Cmd+Option+K (Mac)
Safari: Cmd+Option+C
```

**Step 3: Verify No Console Errors**
```
Expected: No red error messages
If errors: Take screenshot and note message
```

## Test Suite

### Test 1: Overview Tab

**Objective**: Verify landing page displays correctly

**Steps**:
1. Dashboard should open to Overview tab by default
2. Should see "Welcome to Prime Physics Engine Dashboard" heading
3. Should see placeholder statistics (all zeros until data loaded)

**Expected Results**:
- [ ] Overview tab is active (highlighted)
- [ ] Welcome message displays
- [ ] No JavaScript errors in console
- [ ] Page renders within 1 second

**Status**: ___________

---

### Test 2: File Upload (Global)

**Objective**: Test main file upload functionality

**Steps**:
1. Click "Upload Data Files" button in header
2. Select `visualizations/membrane_density_summary.csv`
3. Observe file is accepted
4. Check provenance info appears

**Expected Results**:
- [ ] File input opens system dialog
- [ ] File name appears after selection
- [ ] Provenance section shows:
  - Filename: membrane_density_summary.csv
  - Row count: ~number of configurations
  - Load timestamp
- [ ] No console errors

**Status**: ___________

---

### Test 3: Membranes Tab - Elbow Dynamics

**Objective**: Verify elbow dynamics chart renders

**Prerequisites**: Load `visualizations/membrane_density_summary.csv`

**Steps**:
1. Click "Membranes" tab
2. Wait for chart to render
3. Observe elbow dynamics plot

**Expected Results**:
- [ ] Chart displays within 2 seconds
- [ ] Multiple colored lines visible (one per M value)
- [ ] X-axis labeled "k (zero padding)"
- [ ] Y-axis labeled "Prime Density ρ"
- [ ] White circles mark optimal k* for each M
- [ ] Legend shows M values
- [ ] Hover shows tooltips with exact values

**Validation**:
- [ ] At least 3 distinct lines visible
- [ ] Lines show elbow behavior (peak at specific k)
- [ ] Chart is interactive (zoom/pan with Plotly controls)

**Status**: ___________

---

### Test 4: Membranes Tab - Density Heatmap

**Objective**: Verify density heatmap visualization

**Prerequisites**: Load `visualizations/membrane_density_summary.csv`

**Steps**:
1. Scroll down in Membranes tab
2. Locate "Density Landscape (M,k) → ρ" heatmap
3. Observe color-coded cells

**Expected Results**:
- [ ] Heatmap displays with colored cells
- [ ] Rows = M values (vertical axis)
- [ ] Columns = k values (horizontal axis)
- [ ] Each cell annotated with density value
- [ ] Color bar shows density scale (yellow to red)
- [ ] Darker red = higher density

**Validation**:
- [ ] Can identify highest density cell (darkest red)
- [ ] Values match those in elbow dynamics chart
- [ ] Hover shows exact (M,k,ρ) coordinates

**Status**: ___________

---

### Test 5: Filtering - Base Selection

**Objective**: Test base filter functionality

**Prerequisites**: Load `visualizations/membrane_density_summary.csv`

**Steps**:
1. In sidebar, locate "Base:" filter
2. Select only "15" (deselect others)
3. Click "Apply Filters"
4. Observe charts update

**Expected Results**:
- [ ] Charts re-render with filtered data
- [ ] Only base=15 data visible
- [ ] Legend/labels update to reflect filter
- [ ] Data table shows only base=15 rows

**Validation**:
1. Go to Data Explorer tab
2. Verify all visible rows have base=15
3. Count should match base=15 subset

**Status**: ___________

---

### Test 6: Filtering - M Range

**Objective**: Test M range slider filter

**Prerequisites**: Reset filters first

**Steps**:
1. Set M range: min=2, max=4
2. Click "Apply Filters"
3. Check charts update

**Expected Results**:
- [ ] Elbow dynamics shows only M=2,3,4 lines
- [ ] Heatmap shows only M=2,3,4 rows
- [ ] Range sliders reflect selection

**Validation**:
- [ ] M=1 data not visible
- [ ] M≥5 data not visible
- [ ] Exactly 3 lines in elbow chart

**Status**: ___________

---

### Test 7: Filtering - Coprime Only

**Objective**: Test coprime filter with GCD algorithm

**Prerequisites**: Reset filters, load membrane data

**Steps**:
1. Check "Coprime only (gcd(outer, base) = 1)"
2. Click "Apply Filters"
3. Inspect filtered results

**Expected Results**:
- [ ] Only configurations where gcd(outer,base)=1
- [ ] Charts update to show coprime subset
- [ ] Data Explorer confirms all visible rows are coprime

**Validation** (manual check in Data Explorer):
1. Find row with base=15, outer=13
   - gcd(13,15) = 1 ✓ Should be visible
2. Find row with base=10, outer=5 (if exists)
   - gcd(5,10) = 5 ✗ Should be hidden

**Status**: ___________

---

### Test 8: Density Tab - File Loading

**Objective**: Load density grid files

**Steps**:
1. Click "Density" tab
2. Upload files:
   - Sample A: `tools/density-explorer/out/grid_sample.csv`
   - Model: `tools/density-explorer/out/grid_model.csv`
   - Explain JSON: `tools/density-explorer/out/grid_explain.json`
3. Wait for processing

**Expected Results**:
- [ ] All 3 files load without errors
- [ ] Grid Info section updates:
  - base: 10
  - mid_len: 1..X
  - inner_zero: 0..Y
  - cells: total count
- [ ] Heatmap renders automatically
- [ ] Status shows mode and clamp range

**Console Check**:
- [ ] No CSV parsing errors
- [ ] No JSON parsing errors
- [ ] No "undefined" warnings

**Status**: ___________

---

### Test 9: Density Tab - Visualization Modes

**Objective**: Test all 7 visualization modes

**Prerequisites**: Load grid_sample, grid_model, grid_explain

**Mode 1: Δ enrichment (default)**
- [ ] Select mode: "Δ enrichment (A/pred − 1)"
- [ ] Heatmap shows diverging colors (teal ↔ rose)
- [ ] Status shows clamp range
- [ ] Hover cell shows enrichment % (e.g., "+2.94%")

**Mode 2: Δ absolute**
- [ ] Select mode: "Δ absolute (A − pred)"
- [ ] Colors update (different scale than enrichment)
- [ ] Hover shows absolute difference

**Mode 3: Observed A**
- [ ] Select mode: "Observed A"
- [ ] Grayscale color scheme (not diverging)
- [ ] Hover shows obs(A) value

**Mode 4: Predicted**
- [ ] Select mode: "Predicted"
- [ ] Grayscale scheme
- [ ] Hover shows pred value

**Mode 5: Union(any)**
- [ ] Select mode: "Union(any) obstruction"
- [ ] Requires explain JSON loaded
- [ ] Shows union probability values
- [ ] Hover shows union(any) percentage

**Mode 6: Per-prime P0**
- [ ] Select mode: "Per‑prime P0(p)"
- [ ] Prime dropdown populates (from explain JSON)
- [ ] Select prime: 3
- [ ] Heatmap shows P0(p=3) contribution
- [ ] Hover shows p=3 obstruction value

**Mode 7: A→B delta**
- [ ] Load Sample B CSV (use grid_sample again for testing)
- [ ] Select mode: "A→B delta (obsB − obsA)"
- [ ] Should show difference (0 if same file)
- [ ] Colors update to show comparison

**Status**: ___________

---

### Test 10: Density Tab - Interactive Controls

**Objective**: Test pan, zoom, hover, pin

**Prerequisites**: Density grid loaded

**Pan Test**:
1. Click and drag on heatmap
2. Grid should move smoothly
3. Release mouse, grid stays in new position
- [ ] Pan works in all directions
- [ ] Labels move with grid
- [ ] No glitches or jumps

**Zoom Test**:
1. Hold Alt key
2. Scroll wheel up (zoom in)
3. Scroll wheel down (zoom out)
- [ ] Grid zooms smoothly (0.5× to 2.0×)
- [ ] Cell sizes scale proportionally
- [ ] Labels remain readable

**Hover Test**:
1. Move mouse over different cells
2. Observe crosshair follows mouse
3. Check sidebar "Inspect" section updates
- [ ] Crosshair highlights cell (blue outline)
- [ ] Crosshair extends across row/column
- [ ] Inspect shows: mid_len, inner_zero, obs, pred, enrichment, CI
- [ ] Values update in real-time

**Pin Test**:
1. Hover over interesting cell
2. Press 'P' key
3. Move mouse away
- [ ] Cell remains highlighted
- [ ] Inspect section stays locked
- [ ] Can move mouse freely
- [ ] Press 'P' again to unpin

**Status**: ___________

---

### Test 11: Density Tab - Lineout Charts

**Objective**: Verify lineout chart functionality

**Prerequisites**: Density grid loaded

**Steps**:
1. Hover over cell at (mid=3, iz=2)
2. Observe bottom panel (lineout charts)
3. Left chart: mid_len @ iz=2
4. Right chart: inner_zero @ mid=3

**Expected Results**:
- [ ] Both charts render side-by-side
- [ ] Left chart: X-axis = mid_len values, Y-axis = density
- [ ] Right chart: X-axis = inner_zero values, Y-axis = density
- [ ] Blue line = predicted density
- [ ] Green line = observed density
- [ ] Charts update on hover (real-time)

**Validation**:
- [ ] Pin cell with 'P', verify charts stay locked
- [ ] Hover different cell, charts update
- [ ] Press 'P' to pin new cell

**Status**: ___________

---

### Test 12: Density Tab - Quantile Clamping

**Objective**: Test clamp mode switching

**Prerequisites**: Density grid loaded, mode = "Δ enrichment"

**Quantile Mode**:
1. Select clamp: "quantile"
2. Set value: 0.98 (default)
3. Observe status shows clamp=[min, max]
- [ ] Central 2–98% of values visible
- [ ] Extreme outliers clipped
- [ ] Help text: "(q: keep 2–98%)"

**Adjust Quantile**:
1. Change value to 0.90
2. Observe color scale tightens
- [ ] More cells at extreme colors (more clipping)
- [ ] Status clamp range narrows

**Absolute Mode**:
1. Select clamp: "absolute"
2. Set value: 1.0
3. Help text changes to: "(abs scale: ×mult of max |Δ|)"
- [ ] Symmetric range around zero
- [ ] Max absolute value determines scale

**Status**: ___________

---

### Test 13: Density Tab - Render Options

**Objective**: Test cell size, grid lines, fade CI, crosshair toggles

**Prerequisites**: Density grid loaded

**Cell Size Slider**:
1. Drag slider from 16px → 28px
- [ ] Cells enlarge smoothly
- [ ] Labels remain positioned correctly
- [ ] Grid redraws without flickering

2. Drag slider from 28px → 8px
- [ ] Cells shrink
- [ ] More cells visible in viewport

**Grid Lines Toggle**:
1. Uncheck "grid lines"
- [ ] Grid lines disappear
- [ ] Cells remain colored
2. Check "grid lines"
- [ ] Grid lines reappear (subtle dark lines)

**Fade by CI Toggle**:
1. Mode: "Δ enrichment"
2. Uncheck "fade by CI"
- [ ] All cells full opacity
3. Check "fade by CI"
- [ ] Cells with wide CI appear dimmer (black overlay)
- [ ] Cells with narrow CI appear brighter

**Crosshair Toggle**:
1. Hover cell, observe crosshair
2. Uncheck "crosshair"
- [ ] Crosshair disappears
- [ ] Hover still updates inspect panel
3. Check "crosshair"
- [ ] Crosshair returns

**Status**: ___________

---

### Test 14: Density Tab - Inspect Panel

**Objective**: Verify detailed cell statistics

**Prerequisites**: Density grid loaded with explain JSON

**Steps**:
1. Hover cell at (mid=2, iz=1)
2. Read inspect panel content

**Expected Results**:
- [ ] **mid_len**: 2
- [ ] **inner_zero**: 1
- [ ] **obs(A)**: percentage with 3 decimals
- [ ] **pred**: percentage with 3 decimals
- [ ] **enrichment**: +X.XX% or -X.XX%
- [ ] **CI**: lo% .. hi%
- [ ] **Dominant small-mod contributors**:
  - p=3: percentage
  - p=5: percentage
  - p=7: percentage
  - (up to 6 primes)
- [ ] **union(any)**: percentage (if explain loaded)

**Validation**:
- [ ] All percentages formatted consistently (e.g., "0.245%")
- [ ] Enrichment shows sign (+ or -)
- [ ] CI range makes sense (lo < obs < hi typically)

**Status**: ___________

---

### Test 15: Density Tab - Export Functions

**Objective**: Test PNG and CSV export

**Prerequisites**: Density grid loaded

**Export PNG** (Press 'S'):
1. Configure interesting view (zoom, mode, etc.)
2. Press 'S' key
3. Check Downloads folder

**Expected Results**:
- [ ] File downloads: `density_overlay.png`
- [ ] PNG contains current heatmap view
- [ ] Image quality good (not pixelated)
- [ ] Colors match on-screen display

**Export CSV** (Press 'E'):
1. Select mode: "Δ enrichment"
2. Press 'E' key
3. Check Downloads folder

**Expected Results**:
- [ ] File downloads: `density_overlay_map.csv`
- [ ] CSV has columns: mid_len, inner_zero, value, obs, pred, ci_width
- [ ] Values match heatmap data
- [ ] All cells included (not just visible viewport)

**Validation**:
```bash
# Check CSV format
head density_overlay_map.csv

# Expected:
# mid_len,inner_zero,value,obs,pred,ci_width
# 1,0,-0.0123,0.002456,0.002580,0.000110
# 1,1,0.0045,0.002389,0.002344,0.000108
```

**Status**: ___________

---

### Test 16: Cross-Base Tab

**Objective**: Load and visualize ridge data

**Steps**:
1. Click "Cross-Base" tab
2. Upload files (use file inputs in header OR tab-specific):
   - `tools/density-explorer/out/ridge_base6.csv`
   - `tools/density-explorer/out/ridge_base10.csv`
   - `tools/density-explorer/out/ridge_base12.csv`
   - `tools/density-explorer/out/ridge_base30.csv`
3. Wait for charts to render

**Expected Results**:
- [ ] **iz* Evolution** chart displays
  - 4 lines (one per base)
  - X-axis: Middle Length M
  - Y-axis: Optimal Inner-Zero iz*
  - Legend shows "Base 6", "Base 10", etc.
- [ ] **Base Correlation Heatmap** displays
  - 4×4 grid showing correlation between bases
  - Diagonal = 1.0 (perfect self-correlation)
  - Off-diagonal = correlation coefficients

**Validation**:
- [ ] Can identify universal pattern (all bases converge?)
- [ ] Can spot outlier base (low correlation with others)

**Status**: ___________

---

### Test 17: Data Explorer Tab

**Objective**: Test sortable table and search

**Prerequisites**: Load membrane_density_summary.csv

**Steps**:
1. Click "Data Explorer" tab
2. Observe table with data

**Expected Results**:
- [ ] Table displays with columns: base, outer, inner, M, k, density, etc.
- [ ] Shows 50 rows per page (or fewer if dataset smaller)
- [ ] Pagination controls at bottom

**Sorting Test**:
1. Click "density" column header
2. Table should sort by density (ascending)
3. Click again
4. Table should sort descending (highest density first)
- [ ] Sort indicator appears (▲ or ▼)
- [ ] Values correctly ordered

**Search Test**:
1. Type "base" in search box (if implemented)
2. Table filters to matching rows
- [ ] Only matching rows visible
- [ ] Pagination updates

**Status**: ___________

---

### Test 18: Data Explorer - WolframAlpha Verification

**Objective**: Test prime verification links

**Prerequisites**: Load membrane_density_detail.csv

**Steps**:
1. In Data Explorer, find row with `membrane_value` column
2. Click "Verify Prime" button
3. New tab opens to WolframAlpha

**Expected Results**:
- [ ] Button present in "Actions" column
- [ ] Clicking opens new browser tab
- [ ] URL: `https://www.wolframalpha.com/input?i=is+{value}+prime`
- [ ] WolframAlpha shows result: "{value} is prime" (or "is not prime")

**Example**:
```
Row: membrane_value = 15451
Click "Verify Prime"
WolframAlpha: "15451 is prime" ✓
```

**Status**: ___________

---

### Test 19: Data Explorer - Export Filtered CSV

**Objective**: Test CSV export from table

**Steps**:
1. Apply some filters (e.g., base=15, M=2-3)
2. Go to Data Explorer tab
3. Click "Export Filtered CSV" button

**Expected Results**:
- [ ] File downloads: `filtered_membrane_density_summary.csv`
- [ ] CSV contains only filtered rows
- [ ] All original columns preserved
- [ ] Header row included

**Validation**:
```bash
# Check row count matches filtered view
wc -l filtered_membrane_density_summary.csv
# Should match number shown in table
```

**Status**: ___________

---

### Test 20: Filter Reset

**Objective**: Test reset functionality

**Prerequisites**: Applied filters (base, M range, coprime)

**Steps**:
1. Note current filtered state
2. Click "Reset" button in sidebar
3. Observe filters clear

**Expected Results**:
- [ ] All filter controls return to defaults:
  - Base: all selected
  - M range: min=1, max=10
  - k range: min=0, max=5
  - Checkboxes: unchecked
- [ ] Charts update to show full dataset
- [ ] Data table shows all rows again

**Validation**:
- [ ] Row count in Data Explorer = total (not subset)
- [ ] Charts show all M/k values

**Status**: ___________

---

### Test 21: Multi-Tab Navigation

**Objective**: Verify state persistence across tabs

**Steps**:
1. Load membrane data
2. Apply filters (base=15)
3. Go to Membranes tab, observe filtered charts
4. Switch to Data Explorer tab, observe filtered table
5. Switch to Overview tab
6. Switch back to Membranes tab

**Expected Results**:
- [ ] Filters persist across tab switches
- [ ] Charts don't re-render unnecessarily
- [ ] No data loss when switching tabs
- [ ] Browser back/forward buttons don't break state

**Status**: ___________

---

### Test 22: Error Handling

**Objective**: Test robustness with invalid inputs

**Test 22a: Invalid CSV**
1. Create file `bad.csv` with malformed data:
   ```csv
   base,M,k
   not,a,number
   ```
2. Try to upload
- [ ] Graceful error message (not browser crash)
- [ ] Console shows helpful error
- [ ] Can recover by uploading valid file

**Test 22b: Missing Columns**
1. Create `partial.csv` missing expected columns:
   ```csv
   base,outer
   10,3
   ```
2. Upload
- [ ] Dashboard handles missing columns gracefully
- [ ] Charts show "No data" or skip missing fields

**Test 22c: Empty File**
1. Create empty CSV (header only):
   ```csv
   base,outer,inner,M,k,density
   ```
2. Upload
- [ ] No crash
- [ ] Message: "No data to display"

**Status**: ___________

---

### Test 23: Performance - Large Dataset

**Objective**: Test dashboard with realistic large files

**Prerequisites**: Generate or use large CSV (1000+ rows)

**Steps**:
1. Load large membrane_density CSV
2. Measure load time
3. Test chart responsiveness

**Expected Results**:
- [ ] Load completes within 10 seconds
- [ ] Charts render within 5 seconds
- [ ] Filtering updates within 2 seconds
- [ ] No browser freezing/hanging

**If Slow**:
- Check browser console for warnings
- Try smaller dataset
- Close other browser tabs

**Status**: ___________

---

### Test 24: Browser Compatibility

**Objective**: Verify dashboard works across browsers

**Test in Each Browser**:

| Browser | Version | Load OK | Charts Render | Filters Work | Export OK | Overall |
|---------|---------|---------|---------------|--------------|-----------|---------|
| Chrome  | ___     | [ ]     | [ ]           | [ ]          | [ ]       | PASS/FAIL |
| Firefox | ___     | [ ]     | [ ]           | [ ]          | [ ]       | PASS/FAIL |
| Safari  | ___     | [ ]     | [ ]           | [ ]          | [ ]       | PASS/FAIL |
| Edge    | ___     | [ ]     | [ ]           | [ ]          | [ ]       | PASS/FAIL |

**Known Issues**:
- Safari: Alt+Wheel zoom may need trackpad pinch instead
- Firefox: Large canvas may clip at 16384px (zoom out)

**Status**: ___________

---

### Test 25: Keyboard Shortcuts

**Objective**: Verify all documented shortcuts work

**Global Shortcuts**:
- [ ] Tab key navigates between tabs (when nav focused)
- [ ] Ctrl+R resets filters

**Density Tab Shortcuts**:
- [ ] **P**: Pin/unpin current cell
- [ ] **S**: Save heatmap PNG
- [ ] **E**: Export map CSV
- [ ] **Alt+Wheel**: Zoom in/out
- [ ] **Escape**: Unpin cell (if implemented)

**Status**: ___________

---

## Test Summary

**Total Tests**: 25
**Tests Passed**: _____
**Tests Failed**: _____
**Tests Skipped**: _____

**Pass Rate**: _____% (Passed / Total)

**Critical Issues** (blocking):
1.
2.
3.

**Minor Issues** (non-blocking):
1.
2.
3.

**Notes**:


## Regression Testing

**When to Re-Test**:
- After code changes to dashboard.html
- After updating D3/Plotly/Papa Parse versions
- After modifying CSV generation scripts
- Before each release

**Quick Smoke Test** (5 minutes):
1. Load one CSV file (membrane_density_summary.csv)
2. Switch between all 5 tabs
3. Apply one filter
4. Export one CSV
5. Check console for errors

If smoke test passes, dashboard is likely functional.

## Sign-Off

**Tested By**: _____________________
**Date**: _____________________
**Environment**:
- OS: _____________________
- Browser: _____________________ (version: _____)
- Screen Resolution: _____________________

**Overall Assessment**: PASS / FAIL / CONDITIONAL PASS

**Approval for Release**: YES / NO

**Comments**:



---

**End of Checklist**
