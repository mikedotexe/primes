# Appendix C: Reproduction Instructions

## System Requirements

- macOS 10.14+ (for Metal GPU support)
- Rust 1.70+
- Xcode Command Line Tools
- 4GB+ RAM

## Quick Reproduction

```bash
# Clone repository
git clone https://github.com/anonymous/membrane-primes
cd membrane-primes

# Run base-6 benchmark (best density)
cargo run --release --features metal --bin membrane-prime-gpu-fast \
    -- --gpu --base 6 --count 4000000

# Expected output:
# Found 753,730 primes out of 4,000,000 candidates (18.8% density)
# GPU Performance: 186.6M candidates/second
```

## Verification

To verify that outputs are actually prime:

```bash
# Export first 100 primes
cargo run --release --bin membrane-prime -- --base 6 --count 1000 \
    --export primes.txt

# Verify with Python
python3 verify_primes.py primes.txt
```

## Reproducing Specific Results

### Table 1 Densities
```bash
for base in 6 10 12; do
    echo "Base $base:"
    cargo run --release --example base_comparison -- --base $base
done
```

### Exclusive Configuration
```bash
cargo run --release --example seed_exclusivity_quick
```

[Stub: Add Docker instructions, cross-platform notes, parameter tuning guide]