# Prime Physics Engine - Onboarding Checklist 🚀

## Quick Start (5 minutes)

### 1. Environment Setup
```bash
# Clone and setup
git clone https://github.com/mikepurvis/prime-physics-engine.git
cd prime-physics-engine
rustup override set nightly-2025-07-12
```

### 2. Verify Installation
```bash
# All tests should pass
cargo test --all-features

# Get local baseline performance
cargo bench --bench sieve_bench

# Quick working example
cargo run --example educational_explorer
```

### 3. Platform-Specific Checks
```bash
# Apple Silicon only - DVFS sanity check
scripts/run_thermal_loop.sh  

# WASM compatibility check (if needed)
wasm-pack build --target web --features wasm --no-default-features
```

## Understanding the Architecture (10 minutes)

### 4. Core Concepts Tour
```bash
# Read architectural overview
code ./docs/ARCHITECTURE_OVERVIEW.md

# Explore main verified findings
code ./CLAUDE.md

# Check current implementation status
code ./docs/HARDENING_COMPLETE.md
```

### 5. Interactive Exploration
```bash
# Best introduction for newcomers
cargo run --example educational_explorer

# Advanced interactive lab
cargo run --example membrane_lab_tui

# Full research dashboard
cargo run --example prime_discovery_dashboard
```

## Contributing (Good First Issues)

### 6. Development Tasks by Difficulty

**🟢 Beginner (1-2 hours)**
- [ ] **#51** SIMD popcount for wheel30 (AVX2 path in `src/prime_sieve/wheel30.rs`)
- [ ] **#41** BigInt serialization in wasm-demo (`wasm-demo/src/lib.rs`)
- [ ] **Documentation** Add examples to failing scripts in `examples/legacy/`

**🟡 Intermediate (3-5 hours)** 
- [ ] **#48** DVFS frequency monitor (kpc syscall integration in `src/dvfs/mod.rs`)
- [ ] **Performance** Replace scalar `mark_composite` with 256-bit lane setter
- [ ] **Testing** Cross-check wheel30 with OEIS prime counts up to 1M

**🔴 Advanced (1+ days)**
- [ ] **GPU** Metal shader for membrane generation prototype
- [ ] **Neural** Complete AMX/SME backend for Phase 4
- [ ] **Research** Extend Lagrange point clustering to 100+ prime pairs

### 7. Code Navigation Hints

Every module has embedded **👀 AI-HINT** comments that show:
- **Safe rewrite areas** vs. **hand-tuned hot paths** 
- **Extension points** for new features
- **Performance critical sections** to avoid changing

```rust
// 👀 AI-HINT: This function is safe to modify - add SIMD here
fn mark_composite_scalar(&mut self, n: usize) { ... }

// 👀 AI-HINT: HOT PATH - cycle-critical, benchmark any changes
#[inline(always)]
fn read_cycles() -> u64 { ... }
```

## Connecting AI Assistants

### 8. For LLM Integration
- **Repository context**: Pipe entire repo to model with this checklist
- **Task guidance**: Point to `docs/DEVELOPER_TASKS.md` for specific work items  
- **Code understanding**: Start with `src/lib.rs` then follow module dependencies

### 9. Key Files for AI Context
```
src/
├── lib.rs              # Main API and re-exports
├── prime_sieve.rs      # Core sieve with critical bug fix
├── membrane.rs         # Prime generation via symmetric patterns
├── performance.rs      # Cycle-accurate timing (DVFS-aware)
├── phase4/            # Neural network backend (future)
├── optimization/      # ML-driven strategy selection
└── dvfs/              # Dynamic frequency monitoring

docs/
├── CLAUDE.md          # Executive summary with verified claims
├── EVIDENCE.md        # Detailed proofs and verification
├── PRIME_MAIN_ISSUES.md  # Current technical problems
└── guides/            # Tutorials and deep-dives
```

## Development Workflow

### 10. Standard Process
```bash
# Before starting work
git checkout -b feature/your-improvement
cargo test --all-features  # Ensure clean baseline

# During development  
cargo check --no-default-features  # Fast syntax checking
cargo clippy -- -D warnings        # Catch common issues

# Before committing
cargo test --all-features           # Full test suite
cargo bench                         # Performance regression check
scripts/verify_optimizations.sh     # Verify key optimizations
```

### 11. Performance Testing
```bash
# Critical performance benchmarks
cargo bench --bench sieve_bench     # Core sieve performance
cargo bench --bench membrane_bench  # Prime generation rates
cargo bench --bench phase4_bench    # Neural network inference

# Memory and cache analysis  
cargo run --example cache_analysis  # L1/L2/L3 residency patterns
cargo run --example memory_profile  # Allocation patterns
```

## Current Status & Priorities

### 12. Project Health
- **✅ Core functionality**: 20.4% examples working → systematic fixes planned
- **✅ Critical bugs**: BitSieve accuracy fixed (95.4% → 100%)  
- **✅ Performance**: Optimized for Apple Silicon M1/M2/M3
- **⚠️ WASM**: Build blocked by Criterion dependency (#41)
- **🔄 GPU**: Ready for Metal/CUDA acceleration work

### 13. Next Major Milestones
1. **Fix remaining examples** (74/93 failing → systematic cleanup)
2. **Complete stretch goals** (Wheel-30 + DVFS adaptive scheduling)  
3. **GPU integration** (Metal shaders + cache handoff)
4. **Production deployment** (WebAssembly showcase + benchmarks)

---

## 🎯 Ready to Contribute?

**New contributors**: Start with good first issues above
**AI assistants**: Use the file guide and AI-HINT comments  
**Performance work**: Focus on hot paths marked with benchmarks
**Research**: Explore `docs/EVIDENCE.md` for verified findings

**All paths lead to robust, high-performance prime number research! 🚀**