#!/usr/bin/env python3
"""
Discriminant Hypothesis Validator

Analyzes per-seed discriminant data to test the Quadratic Membrane Hypothesis.

Usage:
    python3 analyze_discriminant.py base10_m2_discriminant_full.csv
"""

import sys
import pandas as pd
import numpy as np
from scipy import stats

def load_data(filename):
    """Load discriminant CSV data."""
    df = pd.read_csv(filename)
    print(f"📊 Loaded {len(df)} seed records")
    print(f"   Bases: {df['base'].unique()}")
    print(f"   M values: {df['M'].unique()}")
    print(f"   k values: {df['k'].unique()}")
    print(f"   Seeds: {df['seed'].min()} to {df['seed'].max()}")
    print()
    return df

def test_perfect_square_hypothesis(df):
    """Test if perfect square discriminants yield fewer primes."""
    print("═" * 60)
    print("TEST 1: Perfect Square Discriminant Hypothesis")
    print("═" * 60)
    print("Hypothesis: Δ = perfect square → algebraic factorization → composite")
    print()

    perfect_sq = df[df['is_perfect_square'] == True]
    non_square = df[df['is_perfect_square'] == False]

    ps_primes = perfect_sq['is_prime'].sum()
    ps_total = len(perfect_sq)
    ps_density = ps_primes / ps_total if ps_total > 0 else 0

    ns_primes = non_square['is_prime'].sum()
    ns_total = len(non_square)
    ns_density = ns_primes / ns_total if ns_total > 0 else 0

    print(f"Perfect Squares: {ps_primes}/{ps_total} primes ({ps_density:.4f} density)")
    print(f"Non-Squares:     {ns_primes}/{ns_total} primes ({ns_density:.4f} density)")
    print()

    if ps_total > 0 and ns_total > 0:
        # Fisher's exact test
        contingency = [[ps_primes, ps_total - ps_primes],
                       [ns_primes, ns_total - ns_primes]]
        odds_ratio, p_value = stats.fisher_exact(contingency)

        print(f"Fisher's Exact Test: p = {p_value:.6f}")
        print(f"Odds Ratio: {odds_ratio:.4f}")

        if p_value < 0.05 and ps_density < ns_density:
            print("✅ HYPOTHESIS SUPPORTED: Perfect squares show lower density (p<0.05)")
        elif p_value < 0.10:
            print("⚠️  WEAK SUPPORT: Marginal significance (p<0.10)")
        else:
            print("❌ HYPOTHESIS REFUTED: No significant difference")
    else:
        print("⚠️  Insufficient data for statistical test")

    print()

def test_discriminant_primality_correlation(df):
    """Test if discriminant value correlates with primality."""
    print("═" * 60)
    print("TEST 2: Discriminant-Primality Correlation")
    print("═" * 60)
    print("Hypothesis: Discriminant magnitude/properties predict primality")
    print()

    # Point-biserial correlation (discriminant vs binary primality)
    is_prime_binary = df['is_prime'].astype(int)
    r, p_value = stats.pointbiserialr(is_prime_binary, df['discriminant'])

    print(f"Point-Biserial Correlation: r = {r:.4f}, p = {p_value:.6f}")

    # Spearman correlation with discriminant magnitude
    disc_abs = df['discriminant'].abs()
    rho, p_rho = stats.spearmanr(is_prime_binary, disc_abs)
    print(f"Spearman ρ (|Δ| vs primality): ρ = {rho:.4f}, p = {p_rho:.6f}")

    # Mean discriminant for primes vs composites
    prime_disc = df[df['is_prime'] == True]['discriminant']
    comp_disc = df[df['is_prime'] == False]['discriminant']

    print()
    print(f"Mean Δ (primes):     {prime_disc.mean():.2f} ± {prime_disc.std():.2f}")
    print(f"Mean Δ (composites): {comp_disc.mean():.2f} ± {comp_disc.std():.2f}")

    # Welch's t-test
    t_stat, t_pval = stats.ttest_ind(prime_disc, comp_disc, equal_var=False)
    print(f"Welch's t-test: t = {t_stat:.4f}, p = {t_pval:.6f}")

    if abs(r) > 0.3 and p_value < 0.05:
        print("✅ STRONG CORRELATION: Discriminant predicts primality")
    elif abs(r) > 0.1 and p_value < 0.10:
        print("⚠️  WEAK CORRELATION: Some predictive power")
    else:
        print("❌ NO CORRELATION: Discriminant does not predict primality")

    print()

def test_k0_vs_k1_discriminant_shift(df):
    """Test if k=1 systematically selects different discriminants than k=0."""
    print("═" * 60)
    print("TEST 3: k=0 vs k=1 Discriminant Shift (Base 10 M=2 Anomaly)")
    print("═" * 60)
    print("Hypothesis: k=1 advantage arises from selecting better discriminants")
    print()

    k0 = df[df['k'] == 0]
    k1 = df[df['k'] == 1]

    # Density comparison
    k0_density = k0['is_prime'].mean()
    k1_density = k1['is_prime'].mean()

    print(f"k=0 Density: {k0_density:.4f} ({k0['is_prime'].sum()}/{len(k0)})")
    print(f"k=1 Density: {k1_density:.4f} ({k1['is_prime'].sum()}/{len(k1)})")
    print(f"Δ Density:   {k1_density - k0_density:+.4f} ({(k1_density-k0_density)/k0_density*100:+.1f}%)")
    print()

    # Discriminant distribution comparison
    k0_disc = k0['discriminant']
    k1_disc = k1['discriminant']

    print(f"Mean Δ (k=0): {k0_disc.mean():.2f} ± {k0_disc.std():.2f}")
    print(f"Mean Δ (k=1): {k1_disc.mean():.2f} ± {k1_disc.std():.2f}")
    print()

    # Kolmogorov-Smirnov test (distribution difference)
    ks_stat, ks_pval = stats.ks_2samp(k0_disc, k1_disc)
    print(f"KS Test (distributions differ): D = {ks_stat:.4f}, p = {ks_pval:.6f}")

    # Perfect square rate comparison
    k0_ps_rate = k0['is_perfect_square'].mean()
    k1_ps_rate = k1['is_perfect_square'].mean()

    print(f"Perfect Square Rate (k=0): {k0_ps_rate:.4f}")
    print(f"Perfect Square Rate (k=1): {k1_ps_rate:.4f}")
    print()

    # QR positive count comparison (favorable residues)
    k0_qr = k0['qr_positive_count'].mean()
    k1_qr = k1['qr_positive_count'].mean()

    print(f"Mean QR+ Count (k=0): {k0_qr:.4f}")
    print(f"Mean QR+ Count (k=1): {k1_qr:.4f}")

    t_qr, p_qr = stats.ttest_ind(k0['qr_positive_count'], k1['qr_positive_count'], equal_var=False)
    print(f"t-test (QR+): t = {t_qr:.4f}, p = {p_qr:.6f}")
    print()

    if k1_density > k0_density and (ks_pval < 0.05 or p_qr < 0.05):
        print("✅ HYPOTHESIS SUPPORTED: k=1 selects better discriminants")
    elif k1_density > k0_density:
        print("⚠️  DENSITY ADVANTAGE EXISTS but discriminant shift not significant")
    else:
        print("❌ HYPOTHESIS REFUTED: No k=1 advantage or discriminant shift")

    print()

def test_goldbach_richness(df):
    """Test if Goldbach-rich seeds produce more primes."""
    print("═" * 60)
    print("TEST 4: Goldbach Decomposition Richness")
    print("═" * 60)
    print("Hypothesis: Seeds with more Goldbach pairs → better membranes")
    print()

    # Split into Goldbach-rich vs Goldbach-poor
    median_pairs = df['goldbach_pairs'].median()
    rich = df[df['goldbach_pairs'] > median_pairs]
    poor = df[df['goldbach_pairs'] <= median_pairs]

    rich_density = rich['is_prime'].mean()
    poor_density = poor['is_prime'].mean()

    print(f"Goldbach-Rich (> {median_pairs:.0f} pairs): {rich_density:.4f} density")
    print(f"Goldbach-Poor (≤ {median_pairs:.0f} pairs): {poor_density:.4f} density")
    print()

    # Spearman correlation
    rho, p_value = stats.spearmanr(df['goldbach_pairs'], df['is_prime'].astype(int))
    print(f"Spearman ρ: {rho:.4f}, p = {p_value:.6f}")

    if rho > 0.2 and p_value < 0.05:
        print("✅ HYPOTHESIS SUPPORTED: Goldbach richness enhances primality")
    elif abs(rho) > 0.1:
        print("⚠️  WEAK CORRELATION")
    else:
        print("❌ HYPOTHESIS REFUTED: No Goldbach effect")

    print()

def generate_summary(df):
    """Generate executive summary."""
    print("\n")
    print("═" * 60)
    print("EXECUTIVE SUMMARY")
    print("═" * 60)
    print()

    total = len(df)
    primes = df['is_prime'].sum()
    density = primes / total

    print(f"Dataset: {total} membranes tested")
    print(f"Primes Found: {primes} ({density:.4f} density)")
    print()

    # Key statistics
    perfect_sq_count = df['is_perfect_square'].sum()
    perfect_sq_primes = df[df['is_perfect_square'] == True]['is_prime'].sum()

    print(f"Perfect Square Discriminants: {perfect_sq_count}/{total}")
    print(f"  Primes from perfect squares: {perfect_sq_primes}/{perfect_sq_count}")
    print()

    # Most predictive features (correlation with primality)
    features = ['discriminant', 'qr_positive_count', 'goldbach_pairs', 'goldbach_lambda']
    is_prime_binary = df['is_prime'].astype(int)

    print("Feature Correlations with Primality:")
    for feat in features:
        if feat in df.columns:
            rho, p = stats.spearmanr(df[feat], is_prime_binary)
            sig = "***" if p < 0.001 else "**" if p < 0.01 else "*" if p < 0.05 else ""
            print(f"  {feat:20s}: ρ = {rho:+.4f} (p={p:.4f}) {sig}")

    print()

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 analyze_discriminant.py <csv_file>")
        sys.exit(1)

    filename = sys.argv[1]

    print()
    print("╔══════════════════════════════════════════════════════════╗")
    print("║     DISCRIMINANT HYPOTHESIS VALIDATION SUITE             ║")
    print("╚══════════════════════════════════════════════════════════╝")
    print()

    df = load_data(filename)

    test_perfect_square_hypothesis(df)
    test_discriminant_primality_correlation(df)
    test_k0_vs_k1_discriminant_shift(df)
    test_goldbach_richness(df)
    generate_summary(df)

    print("✅ Analysis complete!")
    print()

if __name__ == "__main__":
    main()
