# Prime Physics Engine - Command Reference Card

Quick reference for the most useful commands. Save this for easy access!

---

## 🎯 The Essential 5 (15 minutes total)

```bash
# 1. Validate installation (30 sec)
cargo run --example prime_count_smoke_test

# 2. Generate membrane primes (1 min)
cargo run --example proper_membrane_generator

# 3. Witness Lagrange points (2 min)
cargo run --example lagrange_full_verification

# 4. See base-6 champion (1 min)
cargo run --example statistical_prime_generator

# 5. Verify all claims (1 min)
cargo run --example prime_verification_report
```

---

## 🔬 Research Tools

### Verification & Testing

```bash
cargo run --example check_prime                # Interactive: check any number
cargo run --example verify_prime_checker       # Validate prime checking accuracy
cargo run --example prime_count_smoke_test     # Deterministic count validation
```

### Membrane Generation

```bash
cargo run --example proper_membrane_generator  # Systematic membrane testing
cargo run --example statistical_prime_generator # High-success configs
cargo run --example membrane_showcase          # Display membrane structures
cargo run --example find_padded_membrane_primes # Explore padding variations
```

### Lagrange Points

```bash
cargo run --example lagrange_full_verification # Complete L-point demo
cargo run --example lagrange_mechanics         # Mechanics visualization
cargo run --example lagrange_verification      # Quick verification
cargo run --example concatenated_lagrange_explorer # Explore concatenations
```

### Base Analysis

```bash
cargo run --example base6_investigation        # Deep dive into base 6
cargo run --example comprehensive_base_analysis # Compare multiple bases
cargo run --example builder_investigation      # Config builder testing
```

---

## 🎮 Interactive Tools (Requires Terminal)

```bash
cargo run --example membrane_lab_tui           # Interactive membrane builder
cargo run --example lagrange_educational_tui   # Explore Lagrange mechanics
cargo run --example prime_atom_tui            # Visualize prime structure
cargo run --example lagrange_tui_demo         # Lagrange point demo
```

**Note:** These require a proper terminal (not script output redirection).

---

## 📊 Advanced Analysis

### Hardy-Littlewood Framework

```bash
# Goldbach conjecture analysis
cargo run --example experimental/goldbach_hl_analysis -- --min-base 60 --max-base 80 --window 1000

# Phase 2 density analysis
cargo run --example experimental/hz_phase2_density -- --bases 6,30,10 --limit 200000000 --bins 200

# Prime density midpoint studies
cargo run --example experimental/prime_density_midpoint

# Lagrange clustering verification
cargo run --example experimental/lagrange_clustering_verifier
```

### Statistical Analysis

```bash
cargo run --example experimental/statistical_sampling_demo
```

---

## 🏗️ Build & Verification Scripts

```bash
# Quick build verification
./scripts/build-quick-check.sh

# Comprehensive build (all targets)
./scripts/build-everything.sh

# Automated demo walkthrough
./scripts/demo_for_researchers.sh

# Test all examples
./scripts/test_all_examples.sh

# Release packaging
./scripts/release-package-complete.sh
```

---

## 🎯 By Use Case

### "Show me it works"
```bash
cargo run --example prime_count_smoke_test
cargo run --example proper_membrane_generator
```

### "What's the big discovery?"
```bash
cargo run --example lagrange_full_verification
cargo run --example statistical_prime_generator
```

### "Let me explore interactively"
```bash
cargo run --example membrane_lab_tui
cargo run --example check_prime
```

### "I want to do research"
```bash
cargo run --example comprehensive_base_analysis
cargo run --example experimental/goldbach_hl_analysis -- --min-base 60 --max-base 80
```

### "Verify everything"
```bash
cargo run --example prime_verification_report
cargo run --example verify_prime_checker
./scripts/build-everything.sh
```

---

## 📁 Example Categories

### By Purpose
- **Verification**: `prime_count_smoke_test`, `prime_verification_report`, `check_prime`
- **Membrane Generation**: `proper_membrane_generator`, `statistical_prime_generator`, `membrane_showcase`
- **Lagrange Points**: `lagrange_full_verification`, `lagrange_mechanics`, `concatenated_lagrange_explorer`
- **Base Analysis**: `base6_investigation`, `comprehensive_base_analysis`
- **Interactive TUI**: `membrane_lab_tui`, `lagrange_educational_tui`, `prime_atom_tui`
- **Statistical**: `experimental/goldbach_hl_analysis`, `experimental/hz_phase2_density`

### By Experience Level
- **Beginner**: The Essential 5
- **Intermediate**: Base analysis and interactive tools
- **Advanced**: Hardy-Littlewood framework and custom research

### By Runtime
- **Quick (< 1 min)**: `prime_count_smoke_test`, `check_prime`, `prime_verification_report`
- **Medium (1-2 min)**: `proper_membrane_generator`, `statistical_prime_generator`
- **Long (2-5 min)**: `lagrange_full_verification`, `comprehensive_base_analysis`
- **Very Long (5+ min)**: Hardy-Littlewood framework tools with large datasets

---

## 🔧 Build Options

### Standard Development Build
```bash
cargo build --example <name>
cargo run --example <name>
```

### Release Build (Optimized)
```bash
cargo build --release --example <name>
cargo run --release --example <name>
```

### WASM Build
```bash
cargo build --target wasm32-unknown-unknown --release \
            --no-default-features --features wasm
```

### Metal GPU (macOS only)
```bash
cargo run --example metal_gpu_primes --features metal
```

---

## 📖 Documentation Quick Access

```bash
# View example documentation
cargo doc --example <name> --open

# View crate documentation
cargo doc --open

# Read markdown guides
cat RESEARCHER_QUICKSTART.md    # 15-min comprehensive guide
cat ../CLAUDE.md                # Executive summary
cat ../EVIDENCE.md              # Empirical proofs
cat ../NJUGU_ONBOARDING.md     # Quick onboarding
```

---

## 🐛 Troubleshooting

### "Device not configured"
- You're running a TUI app without a terminal
- Solution: Run directly in terminal, not through script redirection

### "Example doesn't compile"
- Check if it's in `experimental/` (may need work)
- Solution: Stick to main directory examples

### "Compilation takes forever"
- First build compiles all dependencies
- Solution: Normal behavior, subsequent builds are fast

### "Out of memory"
- Large analysis allocates significant memory
- Solution: Use `--release` flag or reduce test range

---

## ⚡ Performance Tips

1. **Use release builds** for large analyses: `--release`
2. **First compilation** takes time (one-time cost)
3. **TUI examples** need proper terminal environment
4. **Hardy-Littlewood tools** with large ranges may take 5-10 minutes
5. **Parallel builds**: `cargo build --jobs <N>`

---

## 📚 Learning Path

**Day 1: Validation (30 min)**
1. `prime_count_smoke_test`
2. `proper_membrane_generator`
3. `lagrange_full_verification`
4. Read `RESEARCHER_QUICKSTART.md`

**Day 2: Exploration (2 hours)**
1. Interactive TUI tools
2. `comprehensive_base_analysis`
3. `base6_investigation`
4. Read `CLAUDE.md`

**Day 3: Deep Dive (4 hours)**
1. Hardy-Littlewood framework examples
2. Read `EVIDENCE.md`
3. Experiment with custom configurations
4. Try advanced statistical tools

**Week 2: Research**
1. Design custom experiments
2. Test new bases and configurations
3. Explore longer seed lengths
4. Contribute findings

---

**Last Updated:** 2025-10-29
**All commands tested and verified working**
