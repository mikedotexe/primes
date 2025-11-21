"""
Membrane Density Sandbox with CSV Export

Standalone engine for exploring membrane prime density and elbow dynamics.
Generates comprehensive CSV outputs for integration with Rust extractor
and Manim animation pipeline.

Pipeline Integration:
  1. This script → membrane_density_{summary,detail}.csv
  2. Rust extractor → elbow_events.json
  3. Manim animations → videos

Key Features:
  - Membrane geometry matches canonical template (outer/zeros/inner/zeros/seed/mirror)
  - Seeds enumerated as digit vectors (GPU-compatible)
  - Small-prime sieve in residue space (respects base factorization)
  - Discriminant logging (Δ = S² - 4*outer²) with Legendre symbols
  - Elbow detection (k* transitions as M increases)
  - CSV export for downstream analysis

Usage:
    python visualizations/membrane_density_sandbox.py

Output:
    - membrane_density_summary.csv: one row per (base,outer,inner,M,k)
    - membrane_density_detail.csv: one row per seed with discriminant data
"""

import csv
import itertools
import math
from pathlib import Path
from typing import Dict, List, Tuple, Iterable, Any, Optional

import sympy
import numpy as np


# ---------- Core membrane geometry ----------

def membrane_digits(
    base: int,
    outer: int,
    inner: int,
    k: int,
    seed_digits: Iterable[int],
    shape_type: str = "standard",
) -> List[int]:
    """
    Construct membrane digits.

    shape_type:
      - "standard": [outer] 0^k [inner] 0^k [SEED] 0^k [inner] 0^k [outer]
      - "asymmetric": left/right padding differ by 1 on the inner side
      - "no_inner": [outer] 0^k [SEED] 0^k [outer]
    """
    seed_digits = list(seed_digits)

    if shape_type == "standard":
        left = [outer] + [0] * k + [inner] + [0] * k + seed_digits
        right = [0] * k + [inner] + [0] * k + [outer]
    elif shape_type == "asymmetric":
        left = [outer] + [0] * (k + 1) + [inner] + [0] * k + seed_digits
        right = [0] * k + [inner] + [0] * (k + 1) + [outer]
    elif shape_type == "no_inner":
        left = [outer] + [0] * k + seed_digits
        right = [0] * k + [outer]
    else:
        raise ValueError(f"Unknown shape_type: {shape_type}")

    return left + right


def value_from_digits(base: int, digits: Iterable[int]) -> int:
    """Interpret a digit list in the given base as an integer."""
    n = 0
    for d in digits:
        n = n * base + d
    return n


# ---------- Small-prime sieve in residue space ----------

def modular_filter_for_membrane(
    base: int,
    outer: int,
    inner: int,
    k: int,
    seed_digits: Iterable[int],
    small_primes: Iterable[int] = (3, 5, 7, 11, 13, 17, 19, 23, 29),
    shape_type: str = "standard",
) -> bool:
    """
    Return False if the membrane is divisible by any small prime q
    with gcd(base, q) == 1, by evaluating the membrane modulo q.

    NOTE: We correctly skip primes that divide the base, since their
    residue behavior is special (positions "lock" to specific classes).
    """
    digits = membrane_digits(base, outer, inner, k, seed_digits, shape_type)

    for q in small_primes:
        if math.gcd(base, q) != 1:
            # Skip primes dividing the base; their residue behavior
            # is special and not handled by this simple evaluation.
            continue

        B = base % q
        acc = 0
        powB = 1

        # Evaluate sum d_i * base^i mod q using Horner-style backward scan
        for d in reversed(digits):
            acc = (acc + d * powB) % q
            powB = (powB * B) % q

        if acc == 0:
            return False

    return True


# ---------- Discriminant + residue diagnostics ----------

def compute_seed_value(seed_digits: Iterable[int], base: int) -> int:
    """
    Interpret the seed block as an integer in the same base.
    This is the 'effective S' used in Δ = S^2 - 4*outer^2.
    """
    return value_from_digits(base, list(seed_digits))


def compute_discriminant(seed_digits: Iterable[int], outer: int, base: int) -> int:
    """
    Quadratic-membrane discriminant:
        Δ = S^2 - 4 * outer^2
    where S is the effective seed value.
    """
    seed_val = compute_seed_value(seed_digits, base)
    return seed_val * seed_val - 4 * outer * outer


def legendre_symbol(a: int, p: int) -> int:
    """
    Legendre symbol (a/p) as 0, 1, or -1.
    We return -1 as p-1 internally, but callers treat 1 vs p-1.
    """
    a_mod = a % p
    if a_mod == 0:
        return 0
    result = pow(a_mod, (p - 1) // 2, p)  # 1 or p-1
    # Convert p-1 to -1 for cleaner output
    return result if result == 1 else -1


# ---------- Density engine ----------

def generate_membrane_density(
    base: int,
    outer: int,
    inner: int,
    k: int,
    m: int,
    use_filter: bool = True,
    max_candidates: int = 10_000,
    small_primes: Iterable[int] = (3, 5, 7, 11, 13, 17, 19, 23, 29),
    shape_type: str = "standard",
    log_discriminants: bool = False,
    legendre_primes: Iterable[int] = (3, 5, 7, 11),
) -> Tuple[float, int, int, List[Dict[str, Any]], float]:
    """
    Compute prime density over all nonzero seed digit vectors of length m.

    Returns:
      (density, prime_count, total_count, discriminant_logs, avg_positive_legendre)
    """
    primes = 0
    total = 0
    discriminant_logs: List[Dict[str, Any]] = []

    candidates = itertools.product(range(base), repeat=m)

    for seed_digits in candidates:
        if total >= max_candidates:
            break

        if all(d == 0 for d in seed_digits):
            continue

        if use_filter and not modular_filter_for_membrane(
            base, outer, inner, k, seed_digits, small_primes, shape_type
        ):
            continue

        digits = membrane_digits(base, outer, inner, k, seed_digits, shape_type)
        num = value_from_digits(base, digits)

        is_prime = False
        delta = None
        factors = {}
        legendre_symbols: Dict[int, int] = {}

        if num > 1 and sympy.isprime(num):
            is_prime = True
            primes += 1

        if log_discriminants:
            delta = compute_discriminant(seed_digits, outer, base)
            if delta > 1:
                factors = sympy.factorint(delta)
            for p in legendre_primes:
                if p != 2 and (delta % p) != 0:
                    legendre_symbols[p] = legendre_symbol(delta, p)

            discriminant_logs.append(
                {
                    "seed": tuple(seed_digits),
                    "membrane_value": num,
                    "is_prime": is_prime,
                    "delta": delta,
                    "factors": factors,
                    "legendre_symbols": legendre_symbols,
                }
            )

        total += 1

    density = primes / total if total > 0 else 0.0

    if log_discriminants and discriminant_logs:
        avg_legendre_positive = np.mean(
            [
                sum(1 for v in log["legendre_symbols"].values() if v == 1)
                for log in discriminant_logs
            ]
        )
    else:
        avg_legendre_positive = 0.0

    return density, primes, total, discriminant_logs, avg_legendre_positive


# ---------- Elbow detection ----------

def detect_elbow(
    base: int,
    outer: int,
    inner: int,
    max_k: int = 3,
    max_m: int = 2,
    use_filter: bool = True,
    max_candidates: int = 10_000,
    shape_type: str = "standard",
    log_discriminants: bool = False,
) -> Tuple[List[Dict[str, Any]], Dict[Tuple[int, int], float], List[Dict[str, Any]], List[Dict[str, Any]]]:
    """
    Scan (m, k) grid and detect elbow events:
      k*(m+1) > k*(m) with density jump.

    Returns:
      (elbows, densities, summary_data, detail_data), where:
        elbows: list of {m_transition, k_shift, density_jump, avg_legendre_shift}
        densities: map (m, k) -> density
        summary_data: list of dicts for summary CSV (one per config)
        detail_data: list of dicts for detail CSV (one per seed)
    """
    densities: Dict[Tuple[int, int], float] = {}
    legendre_avgs: Dict[Tuple[int, int], float] = {}
    summary_data: List[Dict[str, Any]] = []
    detail_data: List[Dict[str, Any]] = []

    for m in range(1, max_m + 1):
        for k in range(0, max_k + 1):
            density, primes, total, logs, avg_leg = generate_membrane_density(
                base=base,
                outer=outer,
                inner=inner,
                k=k,
                m=m,
                use_filter=use_filter,
                max_candidates=max_candidates,
                shape_type=shape_type,
                log_discriminants=log_discriminants,
            )

            densities[(m, k)] = density
            legendre_avgs[(m, k)] = avg_leg

            # Collect summary data
            summary_data.append({
                "base": base,
                "outer": outer,
                "inner": inner,
                "M": m,
                "k": k,
                "shape_type": shape_type,
                "use_filter": use_filter,
                "total_candidates": total,
                "prime_count": primes,
                "density": density,
                "avg_positive_legendre": avg_leg,
            })

            # Collect detail data (per-seed)
            for log in logs:
                detail_data.append({
                    "base": base,
                    "outer": outer,
                    "inner": inner,
                    "M": m,
                    "k": k,
                    "seed": log["seed"],
                    "membrane_value": log["membrane_value"],
                    "is_prime": log["is_prime"],
                    "discriminant": log["delta"],
                    "delta_factors": format_factors(log["factors"]),
                    **{f"legendre_{p}": log["legendre_symbols"].get(p, 0) for p in (3, 5, 7, 11)},
                })

            print(
                f"M={m} k={k} shape={shape_type} filter={use_filter}: "
                f"density={density:.6f}, primes={primes}, total={total}, "
                f"avg_positive_legendre={avg_leg:.2f}"
            )

    elbows: List[Dict[str, Any]] = []

    for m in range(1, max_m):
        k_candidates = list(range(0, max_k + 1))
        opt_k_before = max(k_candidates, key=lambda kk: densities[(m, kk)])
        opt_k_after = max(k_candidates, key=lambda kk: densities[(m + 1, kk)])

        if opt_k_after > opt_k_before:
            jump = densities[(m + 1, opt_k_after)] - densities[(m, opt_k_before)]
            leg_shift = (
                legendre_avgs[(m + 1, opt_k_after)]
                - legendre_avgs[(m, opt_k_before)]
            )
            elbows.append(
                {
                    "m_transition": f"{m}→{m+1}",
                    "k_shift": f"{opt_k_before}→{opt_k_after}",
                    "density_jump": jump,
                    "avg_legendre_shift": leg_shift,
                }
            )

    return elbows, densities, summary_data, detail_data


def format_factors(factors: Dict[int, int]) -> str:
    """Format prime factorization as 'p1:e1,p2:e2,...' for CSV."""
    if not factors:
        return ""
    return ",".join(f"{p}:{e}" for p, e in sorted(factors.items()))


# ---------- CSV Export ----------

def export_summary_csv(
    summary_data: List[Dict[str, Any]],
    filename: str = "membrane_density_summary.csv",
    output_dir: Optional[Path] = None,
) -> Path:
    """
    Export summary data: one row per (base,outer,inner,M,k) configuration.

    Columns:
      base, outer, inner, M, k, shape_type, use_filter,
      total_candidates, prime_count, density, avg_positive_legendre
    """
    if output_dir is None:
        output_dir = Path(__file__).parent
    output_path = output_dir / filename

    if not summary_data:
        print(f"No summary data to export.")
        return output_path

    fieldnames = [
        "base", "outer", "inner", "M", "k", "shape_type", "use_filter",
        "total_candidates", "prime_count", "density", "avg_positive_legendre"
    ]

    with open(output_path, "w", newline="") as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(summary_data)

    print(f"✓ Exported {len(summary_data)} summary rows to {output_path}")
    return output_path


def export_detail_csv(
    detail_data: List[Dict[str, Any]],
    filename: str = "membrane_density_detail.csv",
    output_dir: Optional[Path] = None,
) -> Path:
    """
    Export detail data: one row per seed with full discriminant/Legendre info.

    Columns:
      base, outer, inner, M, k, seed, membrane_value, is_prime,
      discriminant, delta_factors, legendre_3, legendre_5, legendre_7, legendre_11
    """
    if output_dir is None:
        output_dir = Path(__file__).parent
    output_path = output_dir / filename

    if not detail_data:
        print(f"No detail data to export.")
        return output_path

    fieldnames = [
        "base", "outer", "inner", "M", "k", "seed", "membrane_value", "is_prime",
        "discriminant", "delta_factors", "legendre_3", "legendre_5", "legendre_7", "legendre_11"
    ]

    with open(output_path, "w", newline="") as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(detail_data)

    print(f"✓ Exported {len(detail_data)} detail rows to {output_path}")
    return output_path


# ---------- Example usage ----------

if __name__ == "__main__":
    base = 15
    outer = 13
    inner = 1

    print("=" * 70)
    print("Membrane Density Sandbox - CSV Export Demo")
    print("=" * 70)
    print(f"\nConfiguration: base={base}, outer={outer}, inner={inner}")
    print(f"Scanning M∈[1,2], k∈[0,3]\n")

    print("=" * 70)
    print("Standard shape, filtered, with discriminant logging")
    print("=" * 70)
    elbows_std, dens_std, summary_std, detail_std = detect_elbow(
        base=base,
        outer=outer,
        inner=inner,
        max_k=3,
        max_m=2,
        use_filter=True,
        max_candidates=10_000,
        shape_type="standard",
        log_discriminants=True,
    )
    print(f"\nDetected elbows: {elbows_std}")

    # Export CSVs
    print("\n" + "=" * 70)
    print("CSV Export")
    print("=" * 70)
    summary_path = export_summary_csv(summary_std)
    detail_path = export_detail_csv(detail_std)

    print("\n" + "=" * 70)
    print("Pipeline Integration")
    print("=" * 70)
    print(f"Summary CSV: {summary_path}")
    print(f"Detail CSV: {detail_path}")
    print("\nThese CSVs can now:")
    print("  1. Feed into Rust extractor → elbow_events.json")
    print("  2. Be plotted directly with pandas/matplotlib")
    print("  3. Feed into Manim animations for visualization")
    print("\nHonorary Zero Connection:")
    print("  • Membrane geometry aligned with honorary-zero axis (base/2)")
    print("  • k parameter controls 'elbow room' around symmetry point")
    print("  • Residue orbits show field lines sweeping from honorary zero")
    print("  • Discriminant Δ measures quadratic residue character")
