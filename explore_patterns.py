#!/usr/bin/env python3
"""
Exploratory Pattern Discovery in Membrane Solution Space

Don't test hypotheses - FIND patterns in the data.
"""

import pandas as pd
import numpy as np
from scipy import stats
import sys

def load_and_describe(filename):
    """Load data and get basic statistics."""
    df = pd.read_csv(filename)

    print("╔═══════════════════════════════════════════════════════════╗")
    print("║         SOLUTION SPACE EXPLORATION - RAW DATA             ║")
    print("╚═══════════════════════════════════════════════════════════╝")
    print()
    print(f"Dataset: {len(df)} membranes")
    print(f"Primes: {df['is_prime'].sum()}/{len(df)} ({df['is_prime'].mean():.4f})")
    print()

    return df

def explore_seed_distribution(df):
    """What do the actual prime seeds look like?"""
    print("═" * 60)
    print("PATTERN 1: Prime Seed Distribution")
    print("═" * 60)

    primes = df[df['is_prime'] == True]['seed'].values
    composites = df[df['is_prime'] == False]['seed'].values

    print(f"\nPrime Seeds ({len(primes)}):")
    print(f"  {sorted(primes)}")

    # Look for patterns in the prime seeds themselves
    prime_seed_props = df[df['is_prime'] == True]['seed']

    # Are prime seeds themselves more likely to be prime?
    from sympy import isprime
    prime_seeds_are_prime = [isprime(int(s)) for s in prime_seed_props]
    prime_seed_prime_rate = sum(prime_seeds_are_prime) / len(prime_seeds_are_prime) if len(prime_seeds_are_prime) > 0 else 0

    comp_seeds = df[df['is_prime'] == False]['seed']
    comp_seeds_are_prime = [isprime(int(s)) for s in comp_seeds]
    comp_seed_prime_rate = sum(comp_seeds_are_prime) / len(comp_seeds_are_prime) if len(comp_seeds_are_prime) > 0 else 0

    print(f"\n🔍 Meta-Pattern: Are successful seeds themselves prime?")
    print(f"  Prime membranes: {prime_seed_prime_rate:.1%} of seeds are prime")
    print(f"  Composite membranes: {comp_seed_prime_rate:.1%} of seeds are prime")

    # Look at seed modular properties
    print(f"\n🔍 Seed Residues (mod 10):")
    for digit in range(10):
        prime_count = df[(df['is_prime'] == True) & (df['seed'] % 10 == digit)].shape[0]
        total_count = df[df['seed'] % 10 == digit].shape[0]
        if total_count > 0:
            rate = prime_count / total_count
            print(f"  Ending in {digit}: {prime_count}/{total_count} = {rate:.3f}")

    print()

def explore_discriminant_clusters(df):
    """Not correlation, but clustering - do primes cluster at certain discriminants?"""
    print("═" * 60)
    print("PATTERN 2: Discriminant Landscape")
    print("═" * 60)

    # Bin discriminants and look at density per bin
    df_sorted = df.sort_values('discriminant')

    # Create quintiles
    df['disc_quintile'] = pd.qcut(df['discriminant'], q=5, labels=False, duplicates='drop')

    print("\n🔍 Prime Density by Discriminant Range:")
    for q in sorted(df['disc_quintile'].unique()):
        subset = df[df['disc_quintile'] == q]
        disc_range = (subset['discriminant'].min(), subset['discriminant'].max())
        density = subset['is_prime'].mean()
        count = subset['is_prime'].sum()
        total = len(subset)
        print(f"  Q{q} [{disc_range[0]:.0f}, {disc_range[1]:.0f}]: {density:.4f} ({count}/{total})")

    # Look at the perfect squares specifically
    ps = df[df['is_perfect_square'] == True]
    print(f"\n🔍 Perfect Square Cases:")
    for idx, row in ps.iterrows():
        print(f"  Seed {row['seed']}, k={row['k']}: Δ={row['discriminant']}, √Δ={row['sqrt_disc']}, Prime={row['is_prime']}")

    print()

def explore_qr_patterns(df):
    """Are there QR patterns we missed?"""
    print("═" * 60)
    print("PATTERN 3: Quadratic Residue Signatures")
    print("═" * 60)

    # Create QR signature (combo of +1, -1, 0 for each prime)
    df['qr_signature'] = df.apply(
        lambda r: f"({r['qr_3']:+d},{r['qr_5']:+d},{r['qr_7']:+d},{r['qr_11']:+d})",
        axis=1
    )

    print("\n🔍 Most Common QR Signatures:")
    sig_stats = df.groupby('qr_signature').agg({
        'is_prime': ['sum', 'count', 'mean']
    }).round(4)
    sig_stats.columns = ['primes', 'total', 'density']
    sig_stats = sig_stats.sort_values('total', ascending=False).head(10)
    print(sig_stats)

    # Look at specific patterns
    print("\n🔍 Do all-positive QR help?")
    all_pos = df[df['qr_positive_count'] == 4]
    other = df[df['qr_positive_count'] < 4]
    print(f"  All +1: {all_pos['is_prime'].mean():.4f} ({all_pos['is_prime'].sum()}/{len(all_pos)})")
    print(f"  Other:  {other['is_prime'].mean():.4f} ({other['is_prime'].sum()}/{len(other)})")

    print()

def explore_goldbach_structure(df):
    """Not count, but structure - what about specific Goldbach decompositions?"""
    print("═" * 60)
    print("PATTERN 4: Goldbach Decomposition Structure")
    print("═" * 60)

    # Seeds with exactly 1 Goldbach pair vs many
    df['goldbach_category'] = pd.cut(df['goldbach_pairs'],
                                      bins=[-1, 0, 1, 2, 100],
                                      labels=['none', 'unique', 'few', 'many'])

    print("\n🔍 Prime Density by Goldbach Category:")
    for cat in ['none', 'unique', 'few', 'many']:
        subset = df[df['goldbach_category'] == cat]
        if len(subset) > 0:
            density = subset['is_prime'].mean()
            count = subset['is_prime'].sum()
            total = len(subset)
            print(f"  {cat:8s}: {density:.4f} ({count}/{total})")

    # Look at HL lambda distribution
    print("\n🔍 Hardy-Littlewood λ Distribution:")
    prime_lambda = df[df['is_prime'] == True]['goldbach_lambda']
    comp_lambda = df[df['is_prime'] == False]['goldbach_lambda']
    print(f"  Primes:     mean={prime_lambda.mean():.3f}, median={prime_lambda.median():.3f}")
    print(f"  Composites: mean={comp_lambda.mean():.3f}, median={comp_lambda.median():.3f}")

    print()

def explore_membrane_geometry(df):
    """How does membrane length affect things?"""
    print("═" * 60)
    print("PATTERN 5: Membrane Geometry")
    print("═" * 60)

    print("\n🔍 Membrane Digit Length Distribution:")
    for k in sorted(df['k'].unique()):
        subset = df[df['k'] == k]
        min_len = subset['membrane_digits'].min()
        max_len = subset['membrane_digits'].max()
        avg_len = subset['membrane_digits'].mean()
        density = subset['is_prime'].mean()
        print(f"  k={k}: {min_len}-{max_len} digits (avg={avg_len:.1f}), density={density:.4f}")

    # Correlation between actual digit count and primality
    rho, p = stats.spearmanr(df['membrane_digits'], df['is_prime'].astype(int))
    print(f"\n🔍 Membrane Length vs Primality: ρ={rho:.4f}, p={p:.4f}")

    print()

def explore_seed_parity_and_structure(df):
    """Seed internal structure patterns"""
    print("═" * 60)
    print("PATTERN 6: Seed Internal Structure")
    print("═" * 60)

    # Odd vs even seeds
    df['seed_parity'] = df['seed'].apply(lambda s: 'even' if s % 2 == 0 else 'odd')

    print("\n🔍 Seed Parity:")
    for parity in ['even', 'odd']:
        subset = df[df['seed_parity'] == parity]
        density = subset['is_prime'].mean()
        count = subset['is_prime'].sum()
        total = len(subset)
        print(f"  {parity:5s}: {density:.4f} ({count}/{total})")

    # Digit sum patterns (in base 10)
    df['seed_digitsum'] = df['seed'].apply(lambda s: sum(int(d) for d in str(s)))
    df['digitsum_mod3'] = df['seed_digitsum'] % 3

    print("\n🔍 Seed Digit Sum (mod 3):")
    for mod in [0, 1, 2]:
        subset = df[df['digitsum_mod3'] == mod]
        if len(subset) > 0:
            density = subset['is_prime'].mean()
            count = subset['is_prime'].sum()
            total = len(subset)
            print(f"  ≡{mod} (mod 3): {density:.4f} ({count}/{total})")

    print()

def find_unexpected_correlations(df):
    """Correlation matrix - what ACTUALLY correlates?"""
    print("═" * 60)
    print("PATTERN 7: Unexpected Correlations")
    print("═" * 60)

    # Select numeric features
    features = ['discriminant', 'disc_mod_base', 'disc_mod_3', 'disc_mod_5', 'disc_mod_7',
                'qr_3', 'qr_5', 'qr_7', 'qr_11', 'qr_positive_count',
                'goldbach_pairs', 'goldbach_lambda', 'membrane_digits', 'seed_digits']

    is_prime_int = df['is_prime'].astype(int)

    print("\n🔍 Feature Correlations with Primality (|ρ| > 0.05):")
    correlations = []
    for feat in features:
        if feat in df.columns:
            rho, p = stats.spearmanr(df[feat], is_prime_int)
            if abs(rho) > 0.05:
                correlations.append((feat, rho, p))

    correlations.sort(key=lambda x: abs(x[1]), reverse=True)

    for feat, rho, p in correlations:
        sig = "***" if p < 0.001 else "**" if p < 0.01 else "*" if p < 0.05 else ""
        print(f"  {feat:25s}: ρ={rho:+.4f} (p={p:.4f}) {sig}")

    if not correlations:
        print("  [No correlations > 0.05 found]")

    # Feature inter-correlations
    print("\n🔍 Strong Feature Inter-Correlations (|ρ| > 0.7):")
    for i, feat1 in enumerate(features):
        for feat2 in features[i+1:]:
            if feat1 in df.columns and feat2 in df.columns:
                rho, p = stats.spearmanr(df[feat1], df[feat2])
                if abs(rho) > 0.7:
                    print(f"  {feat1} ↔ {feat2}: ρ={rho:.4f}")

    print()

def explore_k_dependent_patterns(df):
    """What actually differs between k=0 and k=1?"""
    print("═" * 60)
    print("PATTERN 8: k=0 vs k=1 Deep Dive")
    print("═" * 60)

    k0 = df[df['k'] == 0]
    k1 = df[df['k'] == 1]

    print("\n🔍 Same Seed Comparison (first 10 shared seeds):")
    print(f"{'Seed':<6} {'k=0':>8} {'k=1':>8} {'Δ0':>10} {'Δ1':>10} {'Same?'}")
    print("-" * 60)

    # Get seeds that appear in both
    shared_seeds = set(k0['seed']) & set(k1['seed'])
    for seed in sorted(shared_seeds)[:10]:
        row0 = k0[k0['seed'] == seed].iloc[0]
        row1 = k1[k1['seed'] == seed].iloc[0]

        p0 = "PRIME" if row0['is_prime'] else "comp"
        p1 = "PRIME" if row1['is_prime'] else "comp"
        d0 = row0['discriminant']
        d1 = row1['discriminant']
        same_disc = "✓" if d0 == d1 else "✗"

        print(f"{seed:<6} {p0:>8} {p1:>8} {d0:>10.0f} {d1:>10.0f} {same_disc}")

    # What features DO differ between k=0 and k=1?
    print("\n🔍 Features that Differ Between k=0 and k=1:")
    for feat in ['membrane_digits', 'disc_mod_base', 'disc_mod_3', 'disc_mod_5', 'disc_mod_7']:
        if feat in df.columns:
            mean0 = k0[feat].mean()
            mean1 = k1[feat].mean()
            if abs(mean0 - mean1) > 0.01:
                print(f"  {feat:20s}: k=0={mean0:.3f}, k=1={mean1:.3f}")

    print()

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 explore_patterns.py <csv_file>")
        sys.exit(1)

    df = load_and_describe(sys.argv[1])

    explore_seed_distribution(df)
    explore_discriminant_clusters(df)
    explore_qr_patterns(df)
    explore_goldbach_structure(df)
    explore_membrane_geometry(df)
    explore_seed_parity_and_structure(df)
    find_unexpected_correlations(df)
    explore_k_dependent_patterns(df)

    print("═" * 60)
    print("EXPLORATION COMPLETE")
    print("═" * 60)
    print("\n💡 Look for patterns that emerged, not hypotheses that failed!")
    print()

if __name__ == "__main__":
    main()
