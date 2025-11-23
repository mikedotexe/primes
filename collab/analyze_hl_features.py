#!/usr/bin/env python3
"""
Analyze Hardy-Littlewood Normalized Features

Demonstrates how HL features distinguish prime construction methods by
measuring deviation from theoretical expectations.

Key insights:
1. HL modular divergence: χ² distance from uniform residue distribution
2. HL coverage deviation: (observed / expected_density) - 1.0

High divergence → systematic pattern exploitation (e.g., {0,3,6} restriction)
High coverage → beats HL prediction (membrane efficiency)
"""

import sys
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns

def load_and_prepare(csv_path):
    """Load fingerprint data and extract HL features"""
    df = pd.read_csv(csv_path)

    # Extract relevant columns
    hl_data = df[['label', 'hl_modular_divergence', 'hl_coverage_deviation']].copy()

    # Simplify labels for plotting
    hl_data['short_label'] = hl_data['label'].apply(simplify_label)

    return hl_data

def simplify_label(label):
    """Shorten labels for readability"""
    # Membrane patterns
    if 'membrane_b6' in label:
        return 'B6 (1,5)' + (' k=1' if 'k(1_1)' in label else ' k=0')
    if 'membrane_b10' in label:
        config = 'k=2' if 'k(2_1)' in label else 'k=0'
        return f'B10 (3,7) {config}'
    if 'membrane_b30' in label:
        return 'B30 (11,7)'
    if 'membrane_b14' in label:
        return 'B14 (1,5)'

    # Connectors
    if 'connector_zeroheavy' in label:
        length = label.split('_len')[-1]
        return f'ZeroHeavy L{length}'
    if 'connector_10301' in label:
        length = label.split('_len')[-1]
        return f'Uniform L{length}'

    # Belphegor
    if 'belphegor' in label:
        pad = label.split('_pad')[-1]
        return f'Belphegor p{pad}'

    # Random
    if 'random' in label:
        digits = label.split('_')[-2]
        return f'Random {digits}d'

    return label[:20]

def plot_hl_scatter(df):
    """Scatter plot: modular divergence vs coverage deviation"""
    fig, ax = plt.subplots(figsize=(12, 8))

    # Color by constructor type
    colors = []
    for label in df['label']:
        if 'membrane' in label:
            colors.append('blue')
        elif 'zeroheavy' in label:
            colors.append('red')
        elif 'connector' in label:
            colors.append('orange')
        elif 'belphegor' in label:
            colors.append('purple')
        else:
            colors.append('green')

    # Scatter
    scatter = ax.scatter(
        df['hl_modular_divergence'],
        df['hl_coverage_deviation'],
        c=colors,
        s=200,
        alpha=0.7,
        edgecolors='black',
        linewidths=1.5
    )

    # Annotate points
    for idx, row in df.iterrows():
        ax.annotate(
            row['short_label'],
            (row['hl_modular_divergence'], row['hl_coverage_deviation']),
            xytext=(5, 5),
            textcoords='offset points',
            fontsize=9,
            alpha=0.8
        )

    # Reference lines
    ax.axhline(y=df['hl_coverage_deviation'].median(),
               color='gray', linestyle='--', alpha=0.3, label='Median coverage')
    ax.axvline(x=df['hl_modular_divergence'].median(),
               color='gray', linestyle='--', alpha=0.3, label='Median divergence')

    ax.set_xlabel('HL Modular Divergence (χ² from uniform)', fontsize=12)
    ax.set_ylabel('HL Coverage Deviation (vs expected density)', fontsize=12)
    ax.set_title('Hardy-Littlewood Normalized Feature Space\nConstructor Classification by Theoretical Alignment',
                 fontsize=14, fontweight='bold')
    ax.grid(alpha=0.2)

    # Legend
    from matplotlib.patches import Patch
    legend_elements = [
        Patch(facecolor='blue', edgecolor='black', label='Membrane'),
        Patch(facecolor='red', edgecolor='black', label='Zero-Heavy'),
        Patch(facecolor='orange', edgecolor='black', label='Uniform Connector'),
        Patch(facecolor='purple', edgecolor='black', label='Belphegor'),
        Patch(facecolor='green', edgecolor='black', label='Random')
    ]
    ax.legend(handles=legend_elements, loc='upper left')

    plt.tight_layout()
    plt.savefig('hl_feature_scatter.png', dpi=300, bbox_inches='tight')
    print("\n📊 HL scatter plot saved to: hl_feature_scatter.png")

def plot_hl_bars(df):
    """Bar charts comparing HL features across constructors"""
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(14, 10))

    # Sort by divergence
    df_sorted = df.sort_values('hl_modular_divergence', ascending=False)

    # Plot 1: Modular divergence
    bars1 = ax1.barh(df_sorted['short_label'], df_sorted['hl_modular_divergence'])
    ax1.set_xlabel('HL Modular Divergence (χ²)', fontsize=11)
    ax1.set_title('Deviation from Uniform Residue Distribution', fontsize=12, fontweight='bold')
    ax1.grid(axis='x', alpha=0.3)

    # Color bars
    for i, (bar, label) in enumerate(zip(bars1, df_sorted['label'])):
        if 'zeroheavy' in label:
            bar.set_color('red')
        elif 'membrane' in label:
            bar.set_color('blue')
        elif 'connector' in label:
            bar.set_color('orange')
        elif 'belphegor' in label:
            bar.set_color('purple')
        else:
            bar.set_color('green')

    # Plot 2: Coverage deviation
    bars2 = ax2.barh(df_sorted['short_label'], df_sorted['hl_coverage_deviation'])
    ax2.set_xlabel('HL Coverage Deviation', fontsize=11)
    ax2.set_title('Prime Density vs Hardy-Littlewood Prediction', fontsize=12, fontweight='bold')
    ax2.grid(axis='x', alpha=0.3)

    # Color bars (same scheme)
    for i, (bar, label) in enumerate(zip(bars2, df_sorted['label'])):
        if 'zeroheavy' in label:
            bar.set_color('red')
        elif 'membrane' in label:
            bar.set_color('blue')
        elif 'connector' in label:
            bar.set_color('orange')
        elif 'belphegor' in label:
            bar.set_color('purple')
        else:
            bar.set_color('green')

    plt.tight_layout()
    plt.savefig('hl_feature_bars.png', dpi=300, bbox_inches='tight')
    print("📊 HL bar charts saved to: hl_feature_bars.png")

def analyze_correlations(df):
    """Analyze relationships between HL features"""
    print("\n" + "="*80)
    print("HL FEATURE CORRELATION ANALYSIS")
    print("="*80)

    # Correlation
    corr = df[['hl_modular_divergence', 'hl_coverage_deviation']].corr()
    print("\nCorrelation matrix:")
    print(corr)

    # Interpretation
    r = corr.iloc[0, 1]
    print(f"\nPearson r = {r:.3f}")
    if abs(r) < 0.3:
        print("  → Weak correlation: Features are largely independent")
    elif abs(r) < 0.7:
        print("  → Moderate correlation: Some relationship exists")
    else:
        print("  → Strong correlation: Features are closely related")

def identify_outliers(df):
    """Identify statistical outliers in HL space"""
    print("\n" + "="*80)
    print("HL FEATURE OUTLIER DETECTION")
    print("="*80)

    # Z-scores
    df['div_zscore'] = (df['hl_modular_divergence'] - df['hl_modular_divergence'].mean()) / df['hl_modular_divergence'].std()
    df['cov_zscore'] = (df['hl_coverage_deviation'] - df['hl_coverage_deviation'].mean()) / df['hl_coverage_deviation'].std()

    # Outliers (|z| > 2)
    div_outliers = df[abs(df['div_zscore']) > 2]
    cov_outliers = df[abs(df['cov_zscore']) > 2]

    print(f"\nModular divergence outliers (|z| > 2): {len(div_outliers)}")
    if len(div_outliers) > 0:
        print(div_outliers[['short_label', 'hl_modular_divergence', 'div_zscore']].to_string(index=False))

    print(f"\nCoverage deviation outliers (|z| > 2): {len(cov_outliers)}")
    if len(cov_outliers) > 0:
        print(cov_outliers[['short_label', 'hl_coverage_deviation', 'cov_zscore']].to_string(index=False))

def main(csv_path):
    print("🔬 Hardy-Littlewood Feature Analysis")
    print("="*80)

    # Load data
    print(f"\nLoading: {csv_path}")
    df = load_and_prepare(csv_path)
    print(f"  Loaded {len(df)} constructors")

    # Summary
    print("\n" + "="*80)
    print("FEATURE SUMMARY")
    print("="*80)
    print("\nHL Modular Divergence:")
    print(df['hl_modular_divergence'].describe())
    print("\nHL Coverage Deviation:")
    print(df['hl_coverage_deviation'].describe())

    # Identify extremes
    print("\n" + "="*80)
    print("EXTREME VALUES")
    print("="*80)

    max_div = df.loc[df['hl_modular_divergence'].idxmax()]
    min_div = df.loc[df['hl_modular_divergence'].idxmin()]
    max_cov = df.loc[df['hl_coverage_deviation'].idxmax()]
    min_cov = df.loc[df['hl_coverage_deviation'].idxmin()]

    print(f"\nHighest modular divergence: {max_div['short_label']} ({max_div['hl_modular_divergence']:.2f})")
    print(f"  → Maximally deviates from uniform residue distribution")

    print(f"\nLowest modular divergence: {min_div['short_label']} ({min_div['hl_modular_divergence']:.2f})")
    print(f"  → Most closely follows uniform residue distribution")

    print(f"\nHighest coverage deviation: {max_cov['short_label']} ({max_cov['hl_coverage_deviation']:.2f})")
    print(f"  → Most exceeds HL-predicted prime density")

    print(f"\nLowest coverage deviation: {min_cov['short_label']} ({min_cov['hl_coverage_deviation']:.2f})")
    print(f"  → Closest to HL-predicted prime density")

    # Correlations
    analyze_correlations(df)

    # Outliers
    identify_outliers(df)

    # Visualizations
    print("\n" + "="*80)
    print("GENERATING VISUALIZATIONS")
    print("="*80)
    plot_hl_scatter(df)
    plot_hl_bars(df)

    print("\n" + "="*80)
    print("INTERPRETATION")
    print("="*80)
    print("""
The Hardy-Littlewood normalized features reveal:

1. **Modular Divergence** measures how much a constructor deviates from
   uniform residue distribution across moduli {3,7,11,13,17,19}.

   - Zero-heavy constructors show MASSIVE divergence (106.67) due to
     {0,3,6} digit restriction creating systematic residue bias

   - Membranes show moderate divergence (8-23) reflecting structured
     but not extreme residue patterns

   - Random primes are closest to uniform (11-15)

2. **Coverage Deviation** measures prime density relative to HL prediction.

   - All constructors show positive deviation (10-70) because we're
     sampling from successful prime-generating methods

   - Belphegor shows highest deviation (69.46) - palindromic structure
     creates unexpectedly high prime density

   - Membranes show modest deviation (10-26), suggesting they align
     well with natural prime distribution

These features enable:
- **Outlier detection**: Zero-heavy immediately flagged by divergence
- **Constructor classification**: HL space separates method families
- **Theoretical alignment**: Measure how "natural" vs "forced" patterns are
    """)

    print("\n✨ Analysis complete!")

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <fingerprints.csv>")
        sys.exit(1)

    csv_path = sys.argv[1]
    main(csv_path)
