# Prime Density Experiments

Pre-configured experiments demonstrating key phenomena in membrane prime construction.

## Quick Start

```bash
cd tools/density-explorer

# Run any experiment
bash experiments/01_rollover_phenomenon.sh
bash experiments/02_padding_recovery.sh
bash experiments/03_palindrome_wall.sh
bash experiments/04_base_comparison.sh
```

Results are saved to `experiments/results/*.csv`.

## Experiments

### 1. Rollover Phenomenon (`01_rollover_phenomenon.sh`)

**Demonstrates**: Prime density decline as numbers get longer (PNT 1/ln n effect)

**Setup**: Fixed outer layers, sweep midpoint length 1→12

**Expected**: Gradual density drop following ~1/ln(n), but staying 2-4× above baseline due to membrane structure

**Runtime**: ~5-10 minutes

**Key Insight**: Membrane configurations can't defeat the Prime Number Theorem, but they achieve consistent enrichment across scales.

---

### 2. Padding Recovery (`02_padding_recovery.sh`)

**Demonstrates**: Density rebound as inner zero-padding increases

**Setup**: Fixed 3-digit midpoint, sweep inner zeros 0→10

**Expected**: Density increases as zeros provide "elbow room" and disrupt divisibility by 3

**Runtime**: ~5-10 minutes

**Key Insight**: Zeros aren't just inert spacers—they actively improve prime density by breaking modular patterns (especially digit-sum ≡ 0 mod 3).

---

### 3. Palindrome Wall (`03_palindrome_wall.sh`)

**Demonstrates**: Dramatic density crater at even total lengths (divisibility by 11)

**Setup**: Mirrored patterns, sweep midpoint length and inner zeros

**Expected**: Clear alternating bands—near-zero density at even `total_len`, normal density at odd `total_len`

**Runtime**: ~3-5 minutes

**Key Insight**: Palindromic numbers with even digit count are **always divisible by 11**. This creates "repulsion walls" in prime distribution space. As you add zero padding, the total length changes parity and the density wave shifts.

**Math**: For palindromic n with even length L, n = a₁...aₗ/₂ aₗ/₂...a₁. In base 10: n ≡ a₁(10^(L-1) + 1) + ... ≡ 0 (mod 11) because 10 ≡ -1 (mod 11).

---

### 4. Cross-Base Patterns (`04_base_comparison.sh`)

**Demonstrates**: Universal patterns across bases 6, 10, 14, 18

**Setup**: Test simplified membrane config in multiple bases

**Expected**: Higher densities in bases where boundary digits are coprime to the base. Base 6 should achieve highest enrichment (~3-4×).

**Runtime**: ~15-20 minutes (runs 4 separate sweeps)

**Key Insight**: Optimal configurations are **base-specific**, determined by coprimality to base factorization. The (1,5) pattern works universally in bases where gcd(1,base)=gcd(5,base)=1, achieving 33% in base 6 but only 18.5% in base 10.

---

## Understanding the Results

### Key CSV Columns

- `mid_len`: Length of variable midpoint
- `inner_zero`: Symmetric zero-padding around midpoint
- `total_len`: Total digits in generated numbers
- `prime_density`: Observed fraction of primes
- `expected_density_pnt`: PNT baseline (1/ln n̄)
- `enrichment_factor`: How many times better than random (density / expected)
- `divisible_counts`: Counts for tracked primes [3, 5, 7, 11]

### Interpreting Enrichment Factor

- **1.0×**: Random chance (membrane provides no benefit)
- **2.0×**: Twice as good as random
- **3.0×**: Three times as good as random
- **4.0×**: Exceptional performance (rare, typically base 6 with optimal config)

### Visualization

Open `tools/viz/index.html` and drag-drop any CSV file to see an interactive heatmap:
- X-axis: midpoint length
- Y-axis: inner zero padding
- Color: prime density (Viridis scale)
- Tooltips: full statistics including enrichment factor

## Research Applications

These experiments map directly to CLAUDE.md research priorities:

1. **Rollover** → Understanding length-dependent behavior
2. **Padding Recovery** → Testing "membrane repulsion" hypothesis
3. **Palindrome Wall** → Explicit divisibility constraint visualization
4. **Base Comparison** → Validating coprimality hypothesis across bases

## Extending Experiments

Create your own experiments by modifying the parameters:

```bash
cargo run --release -- grid \
  --mid-kind free \
  --mid-len-range 1..8 \
  --inner-zero-range 0..5 \
  --inner-slot 2 \          # Try larger slots!
  --outer-layers 1:1 0:1 \  # Add more layers!
  --samples 200000 \        # More samples for tighter CIs
  --allowed-last-digits 1,3,7,9 \
  --track-primes 3,5,7,11,13 \  # Track more primes
  --out-csv my_experiment.csv
```

## Citation

If you use these experiments in research, please reference:
- **CLAUDE.md**: Executive summary of membrane prime findings
- **EVIDENCE.md**: Detailed verification and reproducibility data
- This tool: `tools/density-explorer/` (systematic exploration infrastructure)
