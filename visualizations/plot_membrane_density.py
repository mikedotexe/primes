"""
Plot Membrane Density CSV Data

Visualizes output from membrane_density_sandbox.py to explore:
  - Elbow dynamics (density vs k for each M)
  - Heatmaps showing (M,k) → density landscape
  - Legendre symbol distributions
  - Per-seed primality patterns

Usage:
    python visualizations/plot_membrane_density.py

Reads:
    - membrane_density_summary.csv
    - membrane_density_detail.csv

Outputs:
    - membrane_density_plots.png (multi-panel figure)
    - Individual plots as needed
"""

import sys
from pathlib import Path
from typing import Optional

import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import seaborn as sns
import numpy as np

# Set style
sns.set_style("darkgrid")
plt.rcParams["figure.facecolor"] = "#050509"
plt.rcParams["axes.facecolor"] = "#0a0a0e"
plt.rcParams["axes.edgecolor"] = "#444444"
plt.rcParams["grid.color"] = "#222222"
plt.rcParams["text.color"] = "white"
plt.rcParams["axes.labelcolor"] = "white"
plt.rcParams["xtick.color"] = "white"
plt.rcParams["ytick.color"] = "white"


def load_data(data_dir: Optional[Path] = None) -> tuple[pd.DataFrame, pd.DataFrame]:
    """Load summary and detail CSVs."""
    if data_dir is None:
        data_dir = Path(__file__).parent

    summary_path = data_dir / "membrane_density_summary.csv"
    detail_path = data_dir / "membrane_density_detail.csv"

    if not summary_path.exists():
        raise FileNotFoundError(
            f"Summary CSV not found: {summary_path}\n"
            f"Run membrane_density_sandbox.py first to generate data."
        )

    if not detail_path.exists():
        raise FileNotFoundError(
            f"Detail CSV not found: {detail_path}\n"
            f"Run membrane_density_sandbox.py first to generate data."
        )

    summary = pd.read_csv(summary_path)
    detail = pd.read_csv(detail_path)

    print(f"✓ Loaded {len(summary)} summary rows from {summary_path}")
    print(f"✓ Loaded {len(detail)} detail rows from {detail_path}")

    return summary, detail


def plot_elbow_dynamics(summary: pd.DataFrame, ax: plt.Axes) -> None:
    """
    Plot density vs k for each M, showing elbow behavior.

    This is the key visualization: as M increases, optimal k* may shift,
    creating the "elbow room" phenomenon aligned with the honorary zero axis.
    """
    m_values = sorted(summary["M"].unique())
    # Use a vibrant color scheme that stands out against dark background
    colors = plt.cm.plasma(np.linspace(0.15, 0.85, len(m_values)))

    for i, m in enumerate(m_values):
        df_m = summary[summary["M"] == m].sort_values("k")
        ax.plot(
            df_m["k"],
            df_m["density"],
            marker="o",
            linewidth=2.5,
            markersize=9,
            markeredgewidth=0.5,
            markeredgecolor="white",
            color=colors[i],
            label=f"M={m}",
            alpha=0.9,
        )

        # Mark optimal k* with white circle
        k_star = df_m.loc[df_m["density"].idxmax(), "k"]
        max_density = df_m["density"].max()
        ax.scatter(
            [k_star],
            [max_density],
            s=200,
            edgecolors="white",
            facecolors="none",
            linewidths=2,
            zorder=10,
        )

    ax.set_xlabel("k (zero padding)", fontsize=11, fontweight="bold")
    ax.set_ylabel("Prime Density ρ", fontsize=11, fontweight="bold")
    ax.set_title("Elbow Dynamics: ρ vs k for each M (⭕ = optimal k*)", fontsize=13, fontweight="bold")
    ax.legend(framealpha=0.8, loc="best", fontsize=9, ncol=2)
    ax.grid(True, alpha=0.3)


def plot_density_heatmap(summary: pd.DataFrame, ax: plt.Axes) -> None:
    """
    Heatmap of (M,k) → density showing the solution space landscape.

    Honorary zero interpretation: This shows how the "membrane tension"
    (density) responds to both middle length (M) and elbow room (k).
    """
    pivot = summary.pivot(index="M", columns="k", values="density")

    sns.heatmap(
        pivot,
        ax=ax,
        cmap="YlOrRd",
        annot=True,
        fmt=".3f",
        annot_kws={"fontsize": 8},
        cbar_kws={"label": "Density ρ"},
        linewidths=0.5,
        linecolor="#222222",
    )

    ax.set_xlabel("k (zero padding)", fontsize=11, fontweight="bold")
    ax.set_ylabel("M (middle length)", fontsize=11, fontweight="bold")
    ax.set_title("Density Landscape (M,k) → ρ", fontsize=13, fontweight="bold")


def plot_legendre_distribution(summary: pd.DataFrame, ax: plt.Axes) -> None:
    """
    Bar plot showing average positive Legendre symbols across configurations.

    Legendre symbols (±1) measure quadratic residue character of discriminants.
    Higher values suggest more "favorable" residue patterns.
    """
    df_sorted = summary.sort_values("avg_positive_legendre", ascending=False)

    # Limit to top 20 configurations if there are too many
    if len(df_sorted) > 20:
        df_sorted = df_sorted.head(20)
        title_suffix = " (Top 20)"
    else:
        title_suffix = ""

    x_labels = [f"M={row.M}\nk={row.k}" for _, row in df_sorted.iterrows()]
    colors = plt.cm.coolwarm(df_sorted["avg_positive_legendre"] / df_sorted["avg_positive_legendre"].max())

    bars = ax.bar(
        range(len(df_sorted)),
        df_sorted["avg_positive_legendre"],
        color=colors,
        edgecolor="white",
        linewidth=0.5,
    )

    ax.set_xlabel("Configuration (M,k)", fontsize=11, fontweight="bold")
    ax.set_ylabel("Avg Positive Legendre", fontsize=11, fontweight="bold")
    ax.set_title(f"Residue Character Analysis{title_suffix}", fontsize=13, fontweight="bold")
    ax.set_xticks(range(len(df_sorted)))
    ax.set_xticklabels(x_labels, rotation=60, ha="right", fontsize=7)
    ax.grid(True, alpha=0.3, axis="y")
    # Ensure labels don't get cut off
    ax.tick_params(axis='x', which='major', pad=2)


def plot_primality_pattern(detail: pd.DataFrame, ax: plt.Axes) -> None:
    """
    Scatter plot showing which seeds produce primes vs composites.

    X-axis: membrane value (log scale)
    Y-axis: discriminant Δ = S² - 4*outer²
    Color: prime (green) vs composite (red)

    Shows clustering patterns in the (value, discriminant) space.
    """
    primes = detail[detail["is_prime"] == True]
    composites = detail[detail["is_prime"] == False]

    # Plot composites first (background)
    ax.scatter(
        composites["membrane_value"],
        composites["discriminant"],
        c="#dd3333",
        alpha=0.25,
        s=15,
        label=f"Composite ({len(composites)})",
        edgecolors="none",
    )

    # Plot primes on top (foreground)
    ax.scatter(
        primes["membrane_value"],
        primes["discriminant"],
        c="#33ff88",
        alpha=0.85,
        s=60,
        label=f"Prime ({len(primes)})",
        edgecolors="white",
        linewidths=0.8,
        zorder=5,
    )

    ax.set_xlabel("Membrane Value (log scale)", fontsize=11, fontweight="bold")
    ax.set_ylabel("Discriminant Δ", fontsize=11, fontweight="bold")
    ax.set_title("Prime vs Composite Distribution", fontsize=13, fontweight="bold")
    ax.set_xscale("log")
    ax.legend(framealpha=0.9, loc="upper left", fontsize=9)
    ax.grid(True, alpha=0.3)


def create_comprehensive_plot(
    summary: pd.DataFrame,
    detail: pd.DataFrame,
    output_path: Optional[Path] = None,
) -> Path:
    """
    Create comprehensive multi-panel figure showing all key visualizations.
    """
    if output_path is None:
        output_path = Path(__file__).parent / "membrane_density_plots.png"

    fig = plt.figure(figsize=(18, 12))
    gs = gridspec.GridSpec(2, 2, figure=fig, hspace=0.35, wspace=0.30,
                          left=0.08, right=0.96, top=0.92, bottom=0.08)

    # Panel 1: Elbow dynamics (top-left)
    ax1 = fig.add_subplot(gs[0, 0])
    plot_elbow_dynamics(summary, ax1)

    # Panel 2: Density heatmap (top-right)
    ax2 = fig.add_subplot(gs[0, 1])
    plot_density_heatmap(summary, ax2)

    # Panel 3: Legendre distribution (bottom-left)
    ax3 = fig.add_subplot(gs[1, 0])
    plot_legendre_distribution(summary, ax3)

    # Panel 4: Primality pattern (bottom-right)
    ax4 = fig.add_subplot(gs[1, 1])
    plot_primality_pattern(detail, ax4)

    # Overall title
    config = summary.iloc[0]
    fig.suptitle(
        f"Membrane Density Analysis: base={config.base}, outer={config.outer}, inner={config.inner}",
        fontsize=17,
        fontweight="bold",
        color="white",
        y=0.97,
    )

    plt.savefig(output_path, dpi=200, facecolor="#050509")
    print(f"\n✓ Saved comprehensive plot to {output_path}")
    print(f"  Resolution: {18*200}×{12*200} pixels (200 DPI)")

    return output_path


def print_summary_stats(summary: pd.DataFrame, detail: pd.DataFrame) -> None:
    """Print key statistics from the data."""
    print("\n" + "=" * 70)
    print("SUMMARY STATISTICS")
    print("=" * 70)

    config = summary.iloc[0]
    print(f"\nConfiguration: base={config.base}, outer={config.outer}, inner={config.inner}")

    print(f"\nSummary Data:")
    print(f"  • Configurations tested: {len(summary)}")
    print(f"  • M range: {summary['M'].min()} → {summary['M'].max()}")
    print(f"  • k range: {summary['k'].min()} → {summary['k'].max()}")
    print(f"  • Density range: {summary['density'].min():.6f} → {summary['density'].max():.6f}")

    print(f"\nDetail Data:")
    print(f"  • Total seeds tested: {len(detail)}")
    print(f"  • Primes found: {detail['is_prime'].sum()}")
    print(f"  • Overall density: {detail['is_prime'].mean():.6f}")

    # Find optimal configuration
    best = summary.loc[summary["density"].idxmax()]
    print(f"\nOptimal Configuration:")
    print(f"  • M={best.M}, k={best.k}")
    print(f"  • Density: {best.density:.6f}")
    print(f"  • Primes: {best.prime_count}/{best.total_candidates}")
    print(f"  • Avg Legendre: {best.avg_positive_legendre:.2f}")

    # Elbow detection
    print(f"\nElbow Events:")
    m_values = sorted(summary["M"].unique())
    for i, m in enumerate(m_values[:-1]):
        df_before = summary[summary["M"] == m]
        df_after = summary[summary["M"] == m + 1]

        k_before = df_before.loc[df_before["density"].idxmax(), "k"]
        k_after = df_after.loc[df_after["density"].idxmax(), "k"]

        if k_after > k_before:
            density_before = df_before["density"].max()
            density_after = df_after["density"].max()
            jump = density_after - density_before
            print(f"  • M={m}→{m+1}: k*={k_before}→{k_after}, Δρ={jump:+.6f}")


def main():
    """Main execution."""
    print("=" * 70)
    print("Membrane Density Plotter")
    print("=" * 70)

    try:
        # Load data
        summary, detail = load_data()

        # Print statistics
        print_summary_stats(summary, detail)

        # Create plots
        print("\n" + "=" * 70)
        print("GENERATING PLOTS")
        print("=" * 70)
        output_path = create_comprehensive_plot(summary, detail)

        print("\n" + "=" * 70)
        print("COMPLETE")
        print("=" * 70)
        print(f"\nOutput: {output_path}")
        print("\nInterpretation:")
        print("  • Elbow dynamics → k* shifts reveal honorary zero response")
        print("  • Heatmap → shows (M,k) solution space landscape")
        print("  • Legendre distribution → residue character patterns")
        print("  • Primality scatter → clustering in (value, Δ) space")

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
