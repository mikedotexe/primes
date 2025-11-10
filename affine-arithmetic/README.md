# Affine Arithmetic (AA)

A Rust implementation of **affine arithmetic** providing sound first-order enclosures with explicit noise symbol tracking.

## What is Affine Arithmetic?

Affine arithmetic is an extension of interval arithmetic that tracks **correlations** between variables through shared noise symbols. This dramatically reduces overestimation compared to standard interval arithmetic (IA).

### Key Idea

Instead of representing uncertain quantities as intervals `[lo, hi]`, we use affine forms:

```
x̂ = x₀ + Σᵢ xᵢ·εᵢ    where εᵢ ∈ [-1, 1]
```

- **x₀**: central value (midpoint)
- **xᵢ**: coefficients (partial deviations)
- **εᵢ**: noise symbols (shared across correlated quantities)

### Why This Matters

**Standard Interval Arithmetic (IA):**
```rust
let x = [1.95, 2.05];  // x ∈ [1.95, 2.05]
let y = x;             // Same variable!
let z = x - y;         // IA gives: [1.95-2.05, 2.05-1.95] = [-0.1, 0.1] ❌
                       // Should be 0!
```

**Affine Arithmetic (AA):**
```rust
let mut ctx = Ctx::new();
let x = Affine::from_interval(1.95, 2.05, &mut ctx);  // x̂ = 2.0 + 0.05·ε₀
let y = x.clone();                                     // Shares ε₀
let z = x - y;                                         // 0 + 0·ε₀ = 0 ✓
```

AA **remembers** that `x` and `y` are the same, preventing spurious uncertainty growth.

## Features

- **Sound enclosures**: All operations rigorously contain the true value
- **Correlation tracking**: Shared noise symbols preserve dependencies
- **Contexted API**: Explicit symbol allocation (no globals, fully deterministic)
- **Canonical form**: Sorted, merged symbols for efficient operations
- **Optional rigorous mode**: IEEE-1788 compliant interval backend via `inari`
- **no_std compatible**: Works in embedded/WASM environments

## Operations

| Operation | Complexity | Fresh Symbol? |
|-----------|------------|---------------|
| `+`, `-`  | O(m + n)   | No            |
| `scalar × a` | O(m)    | No            |
| `a × b`   | O(m + n)   | **Yes** (remainder) |
| Nonlinear | O(m)       | **Yes** (Chebyshev/Taylor remainder) |

### Multiplication Rule

For `â = a₀ + Σaᵢ·εᵢ` and `b̂ = b₀ + Σbⱼ·εⱼ`:

```
ẑ = a₀·b₀ + Σₖ(a₀·bₖ + b₀·aₖ)·εₖ + ρ·εₙₑw
```

where the remainder `ρ = (Σ|aᵢ|)·(Σ|bⱼ|)` bounds second-order terms.

## Usage

```rust
use affine_arithmetic::{Affine, Ctx};

fn main() {
    let mut ctx = Ctx::new();

    // Create affine forms from intervals
    let x = Affine::from_interval(1.95, 2.05, &mut ctx);  // [1.95, 2.05]
    let y = Affine::from_interval(2.9, 3.1, &mut ctx);    // [2.9, 3.1]

    // Affine operations (+, -, scalar mul don't need context)
    let sum = x.clone() + y.clone();
    let diff = x.clone() - y.clone();
    let scaled = x.clone() * 2.0;

    // Multiplication needs a fresh noise symbol
    let product = x.mul_ctx(&y, &mut ctx);

    // Convert back to interval for comparison
    let (lo, hi) = product.to_interval();
    println!("Product: [{lo}, {hi}]");

    // Expected: [1.95×2.9, 2.05×3.1] = [5.655, 6.355]
    // AA gives tighter bounds than naive IA when correlations exist!
}
```

## Feature Flags

### `std` (default)
Standard library support. Disable for `no_std` environments:
```toml
affine-arithmetic = { version = "0.1", default-features = false }
```

### `rigorous`
Enables IEEE-1788 compliant interval backend via `inari` for outward-rounded remainder bounds:
```toml
affine-arithmetic = { version = "0.1", features = ["rigorous"] }
```

**When to use:**
- Formal verification contexts
- Proof-level numerical guarantees
- Research requiring citation-grade rigor

**Platform requirements:**
- Requires Haswell CPU or later (x86_64 with AVX2)
- Requires GMP/MPFR native dependencies

**Note:** For most use cases, the default f64 backend provides sufficient rigor with conservative safety slack.

## Design Principles

1. **Containment**: Every operation encloses the true value
2. **Monotonicity**: Inclusion-monotone w.r.t interval arithmetic
3. **Correlation retention**: Shared sources use the same noise symbols
4. **Roundoff accounting**: Conservative handling of floating-point errors
5. **Determinism**: Reproducible results (context-based, not global state)

## Performance

- **O(m + n)** for addition/subtraction (merge sorted symbol lists)
- **O(m + n)** for multiplication (merge + one fresh symbol)
- Canonical sorted form enables fast binary search for symbol lookup

## Testing

```bash
# Basic tests (f64 backend)
cargo test

# Rigorous tests (inari backend)
cargo test --features rigorous

# Check compilation (no_std)
cargo check --no-default-features
```

## References

1. **Stolfi & Figueiredo (1997)**: "Self-Validated Numerical Methods and Applications"
   *Original AA formulation with rigorous first-order enclosures*

2. **IEEE Std 1788-2015**: Standard for Interval Arithmetic
   *Foundation for rigorous rounding and enclosure semantics*

3. **Comba & Stolfi (1993)**: "Affine Arithmetic and its Applications to Computer Graphics"
   *Practical applications and algorithm details*

4. **De Figueiredo & Stolfi (2004)**: "Affine Arithmetic: Concepts and Applications"
   *Comprehensive treatment with proofs*

## License

Dual-licensed under MIT OR Apache-2.0.

## Contributing

This crate is part of the [primes](https://github.com/mikedotexe/primes) research repository. Contributions focused on:
- Additional nonlinear functions (exp, log, sin, cos, etc.)
- Symbol condensation strategies
- Performance optimizations
- Formal verification integration

are especially welcome!

---

**Status**: Production-ready core (Phase 1). Nonlinear functions (Phase 2) planned.
