"""
Cross-Base Comparison: Ridge Analysis

Compares membrane behavior across multiple bases to identify universal
patterns vs base-specific phenomena.

Visualizes:
  - Optimal inner-zero (iz_best) evolution across bases
  - Goldbach probability trends
  - Expected prime density curves
  - Cross-base correlation patterns

Usage:
    python visualizations/plot_cross_base_comparison.py

Reads:
    - tools/density-explorer/out/ridge_base*.csv

Outputs:
    - cross_base_comparison.png (comprehensive multi-panel)
"""

import sys
from pathlib import Path
from typing import Dict, List

import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import seaborn as sns
import numpy as np

# Scientific poster theme
sns.set_style("darkgrid")
plt.rcParams["figure.facecolor"] = "#050509"
plt.rcParams["axes.facecolor"] = "#0a0a0e"
plt.rcParams["axes.edgecolor"] = "#555555"
plt.rcParams["grid.color"] = "#222222"
plt.rcParams["text.color"] = "white"
plt.rcParams["axes.labelcolor"] = "white"
plt.rcParams["xtick.color"] = "white"
plt.rcParams["ytick.color"] = "white"


def load_ridge_data(bases: List[int]) -> Dict[int, pd.DataFrame]:
    """Load ridge data for multiple bases."""
    data_dir = Path(__file__).parent.parent / "tools" / "density-explorer" / "out"

    ridge_data = {}
    for base in bases:
        csv_path = data_dir / f"ridge_base{base}.csv"
        if csv_path.exists():
            df = pd.read_csv(csv_path)
            ridge_data[base] = df
            print(f"✓ Loaded base {base}: {len(df)} rows")
        else:
            print(f"⚠ Missing: {csv_path}")

    if not ridge_data:
        raise FileNotFoundError("No ridge data found!")

    return ridge_data


def plot_iz_best_evolution(ridge_data: Dict[int, pd.DataFrame], ax: plt.Axes) -> None:
    """
    Plot optimal inner-zero (iz_best) vs middle length for each base.

    Shows whether different bases converge to similar patterns or diverge.
    """
    base_colors = {
        6: '#ff6b9d',   # Pink
        10: '#4ecdc4',  # Teal
        12: '#ffd700',  # Gold
        14: '#9b59b6',  # Purple
        15: '#e74c3c',  # Red
        18: '#3498db',  # Blue
        22: '#2ecc71',  # Green
        30: '#ff9f43',  # Orange
    }

    for base, df in sorted(ridge_data.items()):
        color = base_colors.get(base, '#ffffff')
        ax.plot(
            df['mid_len'],
            df['iz_best'],
            marker='o',
            linewidth=2.5,
            markersize=8,
            markeredgewidth=0.8,
            markeredgecolor='white',
            color=color,
            label=f'Base {base}',
            alpha=0.9,
        )

    ax.set_xlabel("Middle Length M", fontsize=12, fontweight="bold")
    ax.set_ylabel("Optimal Inner-Zero iz*", fontsize=12, fontweight="bold")
    ax.set_title("iz* Evolution Across Bases", fontsize=14, fontweight="bold", pad=12)
    ax.legend(framealpha=0.85, loc="best", fontsize=10, ncol=2)
    ax.grid(True, alpha=0.3)

    # Add insight box
    props = dict(boxstyle='round,pad=0.5', facecolor='#1a1a1e',
                 edgecolor='#666666', alpha=0.9, linewidth=1)
    insight = "Universal Pattern?\nDo all bases converge to similar iz*?"
    ax.text(0.02, 0.98, insight, transform=ax.transAxes,
            fontsize=9, verticalalignment='top', bbox=props, color='white')


def plot_goldbach_probability(ridge_data: Dict[int, pd.DataFrame], ax: plt.Axes) -> None:
    """
    Plot Goldbach probability (p_any_exact) vs middle length.

    Higher probability = more likely to find prime pairs near 2*base.
    """
    base_colors = {
        6: '#ff6b9d', 10: '#4ecdc4', 12: '#ffd700', 14: '#9b59b6',
        15: '#e74c3c', 18: '#3498db', 22: '#2ecc71', 30: '#ff9f43',
    }

    for base, df in sorted(ridge_data.items()):
        color = base_colors.get(base, '#ffffff')
        ax.plot(
            df['mid_len'],
            df['p_any_exact'],
            marker='s',
            linewidth=2.5,
            markersize=7,
            markeredgewidth=0.8,
            markeredgecolor='white',
            color=color,
            label=f'Base {base}',
            alpha=0.9,
        )

    ax.set_xlabel("Middle Length M", fontsize=12, fontweight="bold")
    ax.set_ylabel("Goldbach Probability P(prime pair)", fontsize=12, fontweight="bold")
    ax.set_title("Hardy-Littlewood Coverage: Base Comparison", fontsize=14, fontweight="bold", pad=12)
    ax.legend(framealpha=0.85, loc="best", fontsize=10, ncol=2)
    ax.grid(True, alpha=0.3)

    # Add insight box
    props = dict(boxstyle='round,pad=0.5', facecolor='#1a1a1e',
                 edgecolor='#666666', alpha=0.9, linewidth=1)

    # Find best base
    best_probs = {base: df['p_any_exact'].mean() for base, df in ridge_data.items()}
    best_base = max(best_probs, key=best_probs.get)
    best_prob = best_probs[best_base]

    insight = f"Champion: Base {best_base}\nAvg probability: {best_prob:.1%}"
    ax.text(0.02, 0.15, insight, transform=ax.transAxes,
            fontsize=9, verticalalignment='top', bbox=props, color='white')


def plot_expected_density(ridge_data: Dict[int, pd.DataFrame], ax: plt.Axes) -> None:
    """
    Plot expected local density vs middle length.

    Shows theoretical prediction of prime density at optimal configuration.
    """
    base_colors = {
        6: '#ff6b9d', 10: '#4ecdc4', 12: '#ffd700', 14: '#9b59b6',
        15: '#e74c3c', 18: '#3498db', 22: '#2ecc71', 30: '#ff9f43',
    }

    for base, df in sorted(ridge_data.items()):
        color = base_colors.get(base, '#ffffff')
        ax.plot(
            df['mid_len'],
            df['expected_local'],
            marker='d',
            linewidth=2.5,
            markersize=7,
            markeredgewidth=0.8,
            markeredgecolor='white',
            color=color,
            label=f'Base {base}',
            alpha=0.9,
        )

    ax.set_xlabel("Middle Length M", fontsize=12, fontweight="bold")
    ax.set_ylabel("Expected Prime Density", fontsize=12, fontweight="bold")
    ax.set_title("Predicted Density (Hardy-Littlewood)", fontsize=14, fontweight="bold", pad=12)
    ax.legend(framealpha=0.85, loc="best", fontsize=10, ncol=2)
    ax.grid(True, alpha=0.3)
    ax.set_yscale('log')  # Log scale for better visibility

    # Add insight box
    props = dict(boxstyle='round,pad=0.5', facecolor='#1a1a1e',
                 edgecolor='#666666', alpha=0.9, linewidth=1)
    insight = "Log Scale\nDensity decreases as M grows\n(numbers get larger)"
    ax.text(0.02, 0.98, insight, transform=ax.transAxes,
            fontsize=9, verticalalignment='top', bbox=props, color='white')


def plot_base_correlation_heatmap(ridge_data: Dict[int, pd.DataFrame], ax: plt.Axes) -> None:
    """
    Correlation heatmap: Do bases with similar properties behave similarly?

    Compares iz_best patterns between bases.
    """
    # Create matrix of iz_best values (bases × mid_len)
    bases = sorted(ridge_data.keys())

    # Find common mid_len range
    all_mid_lens = set()
    for df in ridge_data.values():
        all_mid_lens.update(df['mid_len'].values)
    common_mid_lens = sorted(all_mid_lens)

    # Build matrix
    matrix = []
    for base in bases:
        df = ridge_data[base]
        row = []
        for mid_len in common_mid_lens:
            match = df[df['mid_len'] == mid_len]
            if len(match) > 0:
                row.append(match['iz_best'].iloc[0])
            else:
                row.append(np.nan)
        matrix.append(row)

    matrix = np.array(matrix)

    # Compute correlation between bases
    correlation = np.corrcoef(matrix)

    # Plot heatmap
    sns.heatmap(
        correlation,
        ax=ax,
        cmap='RdYlGn',
        annot=True,
        fmt='.2f',
        square=True,
        cbar_kws={"label": "Correlation", "shrink": 0.8},
        xticklabels=[f'B{b}' for b in bases],
        yticklabels=[f'B{b}' for b in bases],
        linewidths=1,
        linecolor='#333333',
    )

    ax.set_title("Base Similarity: iz* Pattern Correlation", fontsize=14, fontweight="bold", pad=12)

    # Add insight box
    props = dict(boxstyle='round,pad=0.5', facecolor='#1a1a1e',
                 edgecolor='#666666', alpha=0.9, linewidth=1)
    insight = "Green = similar patterns\nRed = divergent behavior"
    ax.text(0.02, 0.98, insight, transform=ax.transAxes,
            fontsize=9, verticalalignment='top', bbox=props, color='white')


def plot_iz_best_distribution(ridge_data: Dict[int, pd.DataFrame], ax: plt.Axes) -> None:
    """
    Distribution of iz_best values across all bases.

    Shows if certain inner-zero values are universally preferred.
    """
    # Collect all iz_best values with base labels
    all_iz_values = []
    base_labels = []

    for base, df in ridge_data.items():
        all_iz_values.extend(df['iz_best'].values)
        base_labels.extend([f'Base {base}'] * len(df))

    # Create combined dataframe
    plot_df = pd.DataFrame({
        'iz_best': all_iz_values,
        'base': base_labels
    })

    # Violin plot
    base_colors = {
        'Base 6': '#ff6b9d', 'Base 10': '#4ecdc4', 'Base 12': '#ffd700',
        'Base 14': '#9b59b6', 'Base 15': '#e74c3c', 'Base 18': '#3498db',
        'Base 22': '#2ecc71', 'Base 30': '#ff9f43',
    }

    parts = ax.violinplot(
        [ridge_data[base]['iz_best'].values for base in sorted(ridge_data.keys())],
        positions=range(len(ridge_data)),
        showmeans=True,
        showmedians=True,
    )

    # Color the violins
    for i, pc in enumerate(parts['bodies']):
        base = sorted(ridge_data.keys())[i]
        color = base_colors.get(f'Base {base}', '#ffffff')
        pc.set_facecolor(color)
        pc.set_alpha(0.7)

    ax.set_xlabel("Base", fontsize=12, fontweight="bold")
    ax.set_ylabel("iz* Value Distribution", fontsize=12, fontweight="bold")
    ax.set_title("Inner-Zero Preference by Base", fontsize=14, fontweight="bold", pad=12)
    ax.set_xticks(range(len(ridge_data)))
    ax.set_xticklabels([f'B{b}' for b in sorted(ridge_data.keys())], fontsize=10)
    ax.grid(True, alpha=0.3, axis='y')

    # Add insight box
    props = dict(boxstyle='round,pad=0.5', facecolor='#1a1a1e',
                 edgecolor='#666666', alpha=0.9, linewidth=1)
    insight = "Width = range of iz* values\nLine = median preference"
    ax.text(0.02, 0.98, insight, transform=ax.transAxes,
            fontsize=9, verticalalignment='top', bbox=props, color='white')


def plot_base_factorization_vs_performance(ridge_data: Dict[int, pd.DataFrame], ax: plt.Axes) -> None:
    """
    Scatter: Base factorization properties vs average Goldbach probability.

    Tests hypothesis: highly composite bases perform better?
    """
    from math import log

    # Compute metrics for each base
    base_metrics = []
    for base, df in ridge_data.items():
        avg_prob = df['p_any_exact'].mean()

        # Count prime factors (with multiplicity)
        n = base
        prime_factors = 0
        distinct_factors = set()
        d = 2
        while d * d <= n:
            while n % d == 0:
                prime_factors += 1
                distinct_factors.add(d)
                n //= d
            d += 1
        if n > 1:
            prime_factors += 1
            distinct_factors.add(n)

        base_metrics.append({
            'base': base,
            'avg_prob': avg_prob,
            'prime_factors': prime_factors,
            'distinct_factors': len(distinct_factors),
            'log_base': log(base),
        })

    df_metrics = pd.DataFrame(base_metrics)

    # Create scatter with color by distinct factors
    scatter = ax.scatter(
        df_metrics['log_base'],
        df_metrics['avg_prob'],
        s=df_metrics['prime_factors'] * 100 + 100,
        c=df_metrics['distinct_factors'],
        cmap='plasma',
        edgecolors='white',
        linewidths=2,
        alpha=0.8,
        zorder=5,
    )

    # Add colorbar
    cbar = plt.colorbar(scatter, ax=ax)
    cbar.set_label('Distinct Prime Factors', fontsize=10, color='white')
    cbar.ax.yaxis.set_tick_params(color='white')
    plt.setp(plt.getp(cbar.ax.axes, 'yticklabels'), color='white')

    # Annotate each point
    for _, row in df_metrics.iterrows():
        ax.annotate(f"B{row['base']}",
                   xy=(row['log_base'], row['avg_prob']),
                   xytext=(5, 5), textcoords='offset points',
                   fontsize=9, color='white', alpha=0.9)

    ax.set_xlabel("log(Base)", fontsize=12, fontweight="bold")
    ax.set_ylabel("Avg Goldbach Probability", fontsize=12, fontweight="bold")
    ax.set_title("Base Structure vs Performance", fontsize=14, fontweight="bold", pad=12)
    ax.grid(True, alpha=0.3)

    # Add insight box
    props = dict(boxstyle='round,pad=0.5', facecolor='#1a1a1e',
                 edgecolor='#666666', alpha=0.9, linewidth=1)
    insight = "Size = total prime factors\nColor = distinct factors\nHighly composite = better?"
    ax.text(0.02, 0.98, insight, transform=ax.transAxes,
            fontsize=9, verticalalignment='top', bbox=props, color='white')


def create_comparison_poster(
    ridge_data: Dict[int, pd.DataFrame],
    output_path: Path,
) -> Path:
    """Create comprehensive cross-base comparison poster."""

    # Large format: 30×20 inches
    fig = plt.figure(figsize=(30, 20))

    # Create 3×2 grid
    gs = gridspec.GridSpec(3, 2, figure=fig,
                          hspace=0.30, wspace=0.28,
                          left=0.06, right=0.96, top=0.93, bottom=0.06)

    # Panel 1: iz* evolution
    ax1 = fig.add_subplot(gs[0, 0])
    plot_iz_best_evolution(ridge_data, ax1)

    # Panel 2: Goldbach probability
    ax2 = fig.add_subplot(gs[0, 1])
    plot_goldbach_probability(ridge_data, ax2)

    # Panel 3: Expected density
    ax3 = fig.add_subplot(gs[1, 0])
    plot_expected_density(ridge_data, ax3)

    # Panel 4: Base correlation
    ax4 = fig.add_subplot(gs[1, 1])
    plot_base_correlation_heatmap(ridge_data, ax4)

    # Panel 5: iz* distribution
    ax5 = fig.add_subplot(gs[2, 0])
    plot_iz_best_distribution(ridge_data, ax5)

    # Panel 6: Factorization vs performance
    ax6 = fig.add_subplot(gs[2, 1])
    plot_base_factorization_vs_performance(ridge_data, ax6)

    # Main title
    bases_str = ', '.join(str(b) for b in sorted(ridge_data.keys()))
    fig.suptitle(
        f"Cross-Base Membrane Analysis: Bases {bases_str}\n"
        f"Comparing Ridge Dynamics & Hardy-Littlewood Predictions",
        fontsize=20,
        fontweight="bold",
        color="white",
        y=0.97,
    )

    # Footer
    footer = (
        "Key Questions: • Do all bases converge to similar iz* patterns? • "
        "Does base factorization predict performance? • "
        "Are highly composite bases more efficient? • "
        "What universal principles emerge?"
    )
    fig.text(0.5, 0.02, footer, ha='center', fontsize=11,
             color='#aaaaaa', style='italic')

    plt.savefig(output_path, dpi=200, facecolor="#050509")
    print(f"\n✓ Saved poster: {output_path}")
    print(f"  Dimensions: 30×20 inches (6000×4000 pixels)")

    return output_path


def main():
    """Main execution."""
    print("=" * 80)
    print("Cross-Base Comparison Generator")
    print("=" * 80)

    try:
        # Load data for all available bases
        bases_to_check = [6, 10, 12, 14, 15, 18, 22, 30]
        ridge_data = load_ridge_data(bases_to_check)

        print(f"\n✓ Loaded {len(ridge_data)} bases")

        # Print summary
        print("\nBase Summary:")
        for base, df in sorted(ridge_data.items()):
            avg_prob = df['p_any_exact'].mean()
            mid_range = f"{df['mid_len'].min()}-{df['mid_len'].max()}"
            print(f"  Base {base:2d}: M∈[{mid_range}], avg P={avg_prob:.1%}")

        # Generate poster
        print("\n" + "=" * 80)
        print("GENERATING CROSS-BASE COMPARISON POSTER")
        print("=" * 80)

        output_path = Path(__file__).parent / "cross_base_comparison.png"
        create_comparison_poster(ridge_data, output_path)

        print("\n" + "=" * 80)
        print("COMPLETE")
        print("=" * 80)
        print(f"\nVisualization ready: {output_path}")

    except FileNotFoundError as e:
        print(f"\n❌ Error: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ Unexpected error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
