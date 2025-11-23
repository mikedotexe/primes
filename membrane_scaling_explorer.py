#!/usr/bin/env python3
"""
Membrane Scaling Explorer - Multi-Dimensional Signal Hunter

Analyzes membrane scaling data across multiple bases with:
- Power law fitting (testing β vs 0.5 hypothesis)
- Density landscape mapping
- Diameter-density correlation
- Gap pattern analysis
- Cross-base universals
- Parameter interaction effects

Philosophy: Sweep parameter space, find signal, lead bravely!
"""

import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
from pathlib import Path
from scipy import stats

# Dark theme for plots
plt.style.use('dark_background')
sns.set_palette("husl")

def load_all_bases():
    """Load all available base CSV files"""
    data_files = list(Path('.').glob('membrane_scaling_base*.csv'))

    bases = {}
    for f in data_files:
        df = pd.read_csv(f)
        # Extract base identifier from filename
        base_name = f.stem.replace('membrane_scaling_', '')
        bases[base_name] = df
        print(f"✓ Loaded {len(df)} configs from {f.name}")

    return bases

def analyze_power_law(df, base_name):
    """Test k* ∝ M^β scaling hypothesis"""
    print(f"\n{'='*60}")
    print(f"POWER LAW ANALYSIS: {base_name}")
    print(f"{'='*60}\n")

    # Find optimal k for each M
    optimal = df.loc[df.groupby('middle_length')['density'].idxmax()]
    optimal = optimal.sort_values('middle_length')

    print("Optimal Configurations:")
    print("M  k_total  density   primes")
    print("-" * 35)
    for _, row in optimal.iterrows():
        k_total = row['k_outer'] + row['k_inner']
        print(f"{int(row['middle_length'])}  {k_total:7d}  {row['density']:.4f}  {int(row['primes_found']):7d}")

    M = optimal['middle_length'].values
    k_total = (optimal['k_outer'] + optimal['k_inner']).values

    # Skip if all k=0 (can't fit log(0))
    if np.all(k_total == 0):
        print("\n⚠️  All k*=0 - Cannot fit power law (log(0) undefined)")
        print("✅ MINIMAL PADDING PRINCIPLE CONFIRMED")
        return {'beta': 0.0, 'r2': np.nan, 'interpretation': 'k*≡0'}

    # Test square-root scaling: k = a * sqrt(M)
    sqrt_M = np.sqrt(M)
    if np.sum(k_total) > 0:  # Only fit if there's variation
        a_sqrt = np.sum(k_total * sqrt_M) / np.sum(sqrt_M**2)
        k_pred_sqrt = a_sqrt * sqrt_M
        ss_tot = np.sum((k_total - np.mean(k_total))**2)
        if ss_tot > 0:
            r2_sqrt = 1 - np.sum((k_total - k_pred_sqrt)**2) / ss_tot
        else:
            r2_sqrt = np.nan
    else:
        a_sqrt, r2_sqrt = 0, np.nan

    # Test general power law: k = a * M^β (only if k>0 somewhere)
    if np.any(k_total > 0):
        # Use only non-zero k values for fitting
        mask = k_total > 0
        if np.sum(mask) >= 2:
            log_M_fit = np.log(M[mask])
            log_k_fit = np.log(k_total[mask])
            coeffs = np.polyfit(log_M_fit, log_k_fit, 1)
            beta, log_a = coeffs
            a = np.exp(log_a)

            k_pred_power = a * M**beta
            ss_tot = np.sum((k_total - np.mean(k_total))**2)
            if ss_tot > 0:
                r2_power = 1 - np.sum((k_total - k_pred_power)**2) / ss_tot
            else:
                r2_power = np.nan
        else:
            beta, a, r2_power = 0, 1, np.nan
    else:
        beta, a, r2_power = 0, 1, np.nan

    print(f"\n🧮 SCALING LAW FITS:")
    print(f"√M model: k = {a_sqrt:.4f} * √M  (R² = {r2_sqrt:.6f})")
    print(f"Power law: k = {a:.4f} * M^{beta:.6f}  (R² = {r2_power:.6f})")

    # Hypothesis test
    distance = abs(beta - 0.5)
    print(f"\n🎯 HYPOTHESIS TEST:")
    print(f"Measured exponent β = {beta:.6f}")
    print(f"Distance from 0.5: {distance:.6f}")

    if distance < 0.1:
        print("\n🤯 SIGNIFICANT: β ≈ 0.5 - Square root scaling!")
        interp = f"β={beta:.3f} ≈ 0.5"
    elif distance < 0.2:
        print(f"\n🤔 INTERESTING: β = {beta:.3f} - Close to 0.5")
        interp = f"β={beta:.3f} ~ 0.5"
    elif beta < 0.1:
        print(f"\n✅ MINIMAL PADDING: β ≈ 0 - No scaling with M")
        interp = "k*≈0 (minimal padding)"
    else:
        print(f"\n📊 ALTERNATIVE SCALING: β = {beta:.3f}")
        interp = f"β={beta:.3f}"

    return {
        'beta': beta,
        'r2': r2_power,
        'r2_sqrt': r2_sqrt,
        'interpretation': interp,
        'optimal': optimal
    }

def density_landscape_heatmap(bases, output_path='density_landscape.png'):
    """3D heatmap: (M, k_total) → density for each base"""
    print(f"\n{'='*60}")
    print("DENSITY LANDSCAPE MAPPING")
    print(f"{'='*60}\n")

    n_bases = len(bases)
    fig, axes = plt.subplots(1, n_bases, figsize=(6*n_bases, 5))
    if n_bases == 1:
        axes = [axes]

    for ax, (base_name, df) in zip(axes, bases.items()):
        # Calculate total k
        df['k_total'] = df['k_outer'] + df['k_inner']

        # Pivot for heatmap
        pivot = df.pivot_table(
            values='density',
            index='k_total',
            columns='middle_length',
            aggfunc='max'  # If multiple configs with same (M,k), take best
        )

        # Plot
        sns.heatmap(
            pivot,
            ax=ax,
            cmap='YlOrRd',
            annot=True,
            fmt='.3f',
            cbar_kws={'label': 'Density ρ'},
            linewidths=0.5
        )

        ax.set_title(f'{base_name}\nDensity Landscape', fontweight='bold')
        ax.set_xlabel('Middle Length (M)', fontweight='bold')
        ax.set_ylabel('Total Padding (k)', fontweight='bold')

        # Highlight optimal configs
        optimal_mask = df.groupby('middle_length')['density'].transform('max') == df['density']
        optimal_points = df[optimal_mask][['middle_length', 'k_total']].values
        for M, k in optimal_points:
            # Find position in heatmap
            try:
                y_idx = list(pivot.index).index(k)
                x_idx = list(pivot.columns).index(M)
                ax.add_patch(plt.Rectangle((x_idx, y_idx), 1, 1,
                                          fill=False, edgecolor='cyan', lw=3))
            except ValueError:
                pass

    plt.tight_layout()
    plt.savefig(output_path, dpi=150, facecolor='#0a0a0e')
    print(f"✓ Saved density landscape to {output_path}")
    plt.close()

def diameter_density_analysis(bases, output_path='diameter_vs_density.png'):
    """Test hypothesis: Does compactness (1/total_digits) correlate with density?"""
    print(f"\n{'='*60}")
    print("DIAMETER-DENSITY HYPOTHESIS")
    print(f"{'='*60}\n")

    fig, ax = plt.subplots(figsize=(10, 6))

    for base_name, df in bases.items():
        # Calculate compactness
        df['compactness'] = 1.0 / df['total_digits']

        # Plot
        ax.scatter(df['compactness'], df['density'],
                  alpha=0.6, s=80, label=base_name, edgecolors='white', linewidths=0.5)

        # Correlation
        corr, p_value = stats.spearmanr(df['compactness'], df['density'])
        print(f"{base_name}: Spearman ρ = {corr:.4f} (p={p_value:.4e})")

    ax.set_xlabel('Compactness (1/total_digits)', fontweight='bold', fontsize=12)
    ax.set_ylabel('Prime Density ρ', fontweight='bold', fontsize=12)
    ax.set_title('Diameter-Density Correlation\n(From k-tuple minimal constellation theory)',
                fontweight='bold', fontsize=14)
    ax.legend()
    ax.grid(True, alpha=0.3)

    plt.tight_layout()
    plt.savefig(output_path, dpi=150, facecolor='#0a0a0e')
    print(f"\n✓ Saved diameter analysis to {output_path}")
    plt.close()

def cross_base_universals(bases, output_path='cross_base_correlation.png'):
    """Find (M, k) configs that work across multiple bases"""
    print(f"\n{'='*60}")
    print("CROSS-BASE UNIVERSALS")
    print(f"{'='*60}\n")

    # Combine all data
    all_data = []
    for base_name, df in bases.items():
        df = df.copy()
        df['base_name'] = base_name
        df['k_total'] = df['k_outer'] + df['k_inner']
        all_data.append(df)

    combined = pd.concat(all_data, ignore_index=True)

    # Find configs that appear in all bases
    config_cols = ['middle_length', 'k_total']
    grouped = combined.groupby(config_cols).agg({
        'density': ['mean', 'std', 'count'],
        'base_name': lambda x: list(x)
    })

    # Configs that appear in all bases
    n_bases = len(bases)
    universal = grouped[grouped[('density', 'count')] == n_bases].copy()
    universal = universal.sort_values(('density', 'mean'), ascending=False)

    print(f"Configurations tested in all {n_bases} bases:")
    print("\nM  k_total  avg_density  std_density")
    print("-" * 45)
    for idx, row in universal.head(10).iterrows():
        M, k = idx
        avg_dens = row[('density', 'mean')]
        std_dens = row[('density', 'std')]
        print(f"{M}  {k:7d}  {avg_dens:11.4f}  {std_dens:11.4f}")

    # Visualize correlation matrix
    pivot = combined.pivot_table(
        values='density',
        index=['middle_length', 'k_total'],
        columns='base_name',
        aggfunc='first'
    )

    fig, ax = plt.subplots(figsize=(8, 6))
    corr_matrix = pivot.corr(method='spearman')
    sns.heatmap(corr_matrix, annot=True, fmt='.3f', cmap='coolwarm',
               center=0, ax=ax, linewidths=1)
    ax.set_title('Cross-Base Density Correlation\n(Spearman ρ)',
                fontweight='bold', fontsize=14)

    plt.tight_layout()
    plt.savefig(output_path, dpi=150, facecolor='#0a0a0e')
    print(f"\n✓ Saved cross-base correlation to {output_path}")
    plt.close()

def main():
    print("=" * 60)
    print("MEMBRANE SCALING EXPLORER")
    print("Signal Hunter - Multi-Dimensional Analysis")
    print("=" * 60)

    # Load data
    bases = load_all_bases()

    if not bases:
        print("\n❌ No data files found! Run membrane_scaling_cli first.")
        return

    # Analysis 1: Power law fitting
    power_law_results = {}
    for base_name, df in bases.items():
        result = analyze_power_law(df, base_name)
        power_law_results[base_name] = result

    # Analysis 2: Density landscape
    density_landscape_heatmap(bases)

    # Analysis 3: Diameter-density correlation
    diameter_density_analysis(bases)

    # Analysis 4: Cross-base universals
    cross_base_universals(bases)

    # Summary
    print(f"\n{'='*60}")
    print("EXPLORATION SUMMARY")
    print(f"{'='*60}\n")

    print("Power Law Exponents:")
    for base_name, result in power_law_results.items():
        print(f"  {base_name:15s}: {result['interpretation']}")

    print("\n✅ Exploration complete!")
    print("Generated visualizations:")
    print("  - density_landscape.png")
    print("  - diameter_vs_density.png")
    print("  - cross_base_correlation.png")

if __name__ == '__main__':
    main()
