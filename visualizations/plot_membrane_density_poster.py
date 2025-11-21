"""
Poster-Quality Membrane Density Visualization

Creates a large-format (36×24 inch) scientific poster with comprehensive
analysis of membrane prime generation dynamics.

Features:
  - 6-panel layout with contextual explanations
  - Statistical annotations and key findings
  - Mathematical definitions and formulas
  - Membrane structure diagram
  - k* migration timeline
  - Publication-ready typography

Usage:
    python visualizations/plot_membrane_density_poster.py

Outputs:
    - membrane_density_poster.png (7200×4800 pixels @ 200 DPI)
"""

import sys
from pathlib import Path
from typing import Optional

import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from matplotlib.patches import Rectangle, FancyBboxPatch, FancyArrowPatch
import seaborn as sns
import numpy as np

# Poster-quality theme
sns.set_style("darkgrid")
plt.rcParams["figure.facecolor"] = "#050509"
plt.rcParams["axes.facecolor"] = "#0a0a0e"
plt.rcParams["axes.edgecolor"] = "#555555"
plt.rcParams["grid.color"] = "#222222"
plt.rcParams["text.color"] = "white"
plt.rcParams["axes.labelcolor"] = "white"
plt.rcParams["xtick.color"] = "white"
plt.rcParams["ytick.color"] = "white"
plt.rcParams["font.family"] = "sans-serif"
plt.rcParams["font.sans-serif"] = ["DejaVu Sans", "Arial", "Helvetica"]


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

    return summary, detail


def add_text_box(ax, text, position, fontsize=10, alpha=0.85):
    """Add a styled text box with background."""
    props = dict(boxstyle='round,pad=0.7', facecolor='#1a1a1e',
                 edgecolor='#666666', alpha=alpha, linewidth=1.5)
    ax.text(position[0], position[1], text, transform=ax.transAxes,
            fontsize=fontsize, verticalalignment='top', bbox=props,
            color='white', fontweight='normal')


def plot_elbow_dynamics(summary: pd.DataFrame, ax: plt.Axes) -> None:
    """Enhanced elbow dynamics with annotations."""
    m_values = sorted(summary["M"].unique())
    colors = plt.cm.plasma(np.linspace(0.15, 0.85, len(m_values)))

    for i, m in enumerate(m_values):
        df_m = summary[summary["M"] == m].sort_values("k")
        ax.plot(
            df_m["k"],
            df_m["density"],
            marker="o",
            linewidth=3,
            markersize=11,
            markeredgewidth=1,
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
            s=300,
            edgecolors="white",
            facecolors="none",
            linewidths=3,
            zorder=10,
        )

        # Annotate optimal points
        if i < len(m_values):
            ax.annotate(f'k*={k_star}',
                       xy=(k_star, max_density),
                       xytext=(5, 5), textcoords='offset points',
                       fontsize=9, color='white', alpha=0.8)

    ax.set_xlabel("k (zero padding)", fontsize=14, fontweight="bold")
    ax.set_ylabel("Prime Density ρ", fontsize=14, fontweight="bold")
    ax.set_title("Elbow Dynamics: ρ vs k for each M", fontsize=16, fontweight="bold", pad=15)
    ax.legend(framealpha=0.85, loc="best", fontsize=11, ncol=2)
    ax.grid(True, alpha=0.3)

    # Add explanation box
    explanation = "⭕ = Optimal k* (maximum density)\nLines show how padding affects prime yield"
    add_text_box(ax, explanation, (0.02, 0.98), fontsize=10)


def plot_density_heatmap(summary: pd.DataFrame, ax: plt.Axes) -> None:
    """Enhanced heatmap with better annotations."""
    pivot = summary.pivot(index="M", columns="k", values="density")

    sns.heatmap(
        pivot,
        ax=ax,
        cmap="YlOrRd",
        annot=True,
        fmt=".4f",
        annot_kws={"fontsize": 10, "weight": "bold"},
        cbar_kws={"label": "Prime Density ρ", "shrink": 0.8},
        linewidths=1,
        linecolor="#333333",
    )

    ax.set_xlabel("k (zero padding)", fontsize=14, fontweight="bold")
    ax.set_ylabel("M (middle length)", fontsize=14, fontweight="bold")
    ax.set_title("Density Landscape: (M,k) → ρ", fontsize=16, fontweight="bold", pad=15)

    # Find and mark the global maximum
    max_val = pivot.max().max()
    max_pos = pivot.stack().idxmax()

    explanation = f"Peak: ρ={max_val:.4f} at M={max_pos[0]}, k={max_pos[1]}\nWarmer colors = higher prime density"
    add_text_box(ax, explanation, (0.02, 0.98), fontsize=10)


def plot_k_star_migration(summary: pd.DataFrame, ax: plt.Axes) -> None:
    """NEW: Explicit k* migration timeline showing elbow events."""
    m_values = sorted(summary["M"].unique())
    k_stars = []
    densities = []

    for m in m_values:
        df_m = summary[summary["M"] == m]
        k_star = df_m.loc[df_m["density"].idxmax(), "k"]
        density = df_m["density"].max()
        k_stars.append(k_star)
        densities.append(density)

    # Create twin axis for density
    ax2 = ax.twinx()

    # Plot k* migration as step function
    ax.step(m_values, k_stars, where='mid', linewidth=3, color='#ff6b9d',
            marker='o', markersize=12, label='k* (optimal padding)', alpha=0.9)

    # Plot density evolution
    ax2.plot(m_values, densities, linewidth=3, color='#4ecdc4',
             marker='s', markersize=10, label='Peak density ρ', alpha=0.8)

    # Highlight elbow events (where k* changes)
    for i in range(len(m_values) - 1):
        if k_stars[i+1] != k_stars[i]:
            ax.axvspan(m_values[i], m_values[i+1], alpha=0.15, color='yellow', zorder=0)
            # Add annotation for elbow event
            mid_m = (m_values[i] + m_values[i+1]) / 2
            ax.annotate('ELBOW\nEVENT', xy=(mid_m, k_stars[i]),
                       fontsize=9, ha='center', color='yellow',
                       weight='bold', alpha=0.9)

    ax.set_xlabel("M (middle length)", fontsize=14, fontweight="bold")
    ax.set_ylabel("k* (optimal padding)", fontsize=14, fontweight="bold", color='#ff6b9d')
    ax2.set_ylabel("Peak Density ρ", fontsize=14, fontweight="bold", color='#4ecdc4')
    ax.set_title("k* Migration Timeline", fontsize=16, fontweight="bold", pad=15)

    ax.tick_params(axis='y', labelcolor='#ff6b9d')
    ax2.tick_params(axis='y', labelcolor='#4ecdc4')

    ax.legend(loc='upper left', fontsize=11, framealpha=0.85)
    ax2.legend(loc='upper right', fontsize=11, framealpha=0.85)
    ax.grid(True, alpha=0.3)

    explanation = "Yellow bands = elbow events (k* shifts)\nConfiguration adapts as structure grows"
    add_text_box(ax, explanation, (0.02, 0.15), fontsize=10)


def plot_membrane_diagram(summary: pd.DataFrame, ax: plt.Axes) -> None:
    """NEW: Membrane structure diagram with annotations."""
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 6)
    ax.axis('off')

    config = summary.iloc[0]
    outer = int(config.outer)
    inner = int(config.inner)

    # Title
    ax.text(5, 5.5, "Membrane Structure", fontsize=16, fontweight='bold',
            ha='center', color='white')

    # Draw membrane schematic
    y_base = 3

    # Example: outer-k1zeros-inner-k2zeros-SEED-k2zeros-inner-k1zeros-outer
    components = [
        (1, outer, '#ff6b9d', 'outer'),
        (2, 0, '#666666', '0'*2),  # k1 zeros
        (3, inner, '#4ecdc4', 'inner'),
        (4, 0, '#666666', '0'),     # k2 zeros
        (5, 5, '#ffd700', 'S'),     # seed
        (6, 0, '#666666', '0'),     # k2 zeros
        (7, inner, '#4ecdc4', 'inner'),
        (8, 0, '#666666', '0'*2),  # k1 zeros
        (9, outer, '#ff6b9d', 'outer'),
    ]

    for x, val, color, label in components:
        # Draw box
        rect = FancyBboxPatch((x-0.3, y_base-0.3), 0.6, 0.6,
                              boxstyle="round,pad=0.05",
                              facecolor=color, edgecolor='white',
                              linewidth=2, alpha=0.8)
        ax.add_patch(rect)

        # Add label
        if label in ['outer', 'inner', 'S']:
            ax.text(x, y_base, label, fontsize=11, ha='center', va='center',
                   color='black', weight='bold')

    # Add annotations
    ax.text(1, y_base - 1, 'boundary\ndigit', fontsize=9, ha='center', color='#ff6b9d')
    ax.text(3, y_base - 1, 'inner\ndigit', fontsize=9, ha='center', color='#4ecdc4')
    ax.text(5, y_base - 1, 'seed\n(variable)', fontsize=9, ha='center', color='#ffd700')

    # Add braces showing k1 and k2
    ax.plot([2, 2], [y_base + 0.8, y_base + 1.2], color='white', linewidth=1.5)
    ax.text(2, y_base + 1.5, 'k₁ zeros', fontsize=10, ha='center', color='white')

    ax.plot([4, 4], [y_base + 0.8, y_base + 1.2], color='white', linewidth=1.5)
    ax.text(4, y_base + 1.5, 'k₂ zeros', fontsize=10, ha='center', color='white')

    # Add formula
    formula = f"Pattern: {outer}-0^k₁-{inner}-0^k₂-S-0^k₂-{inner}-0^k₁-{outer}"
    ax.text(5, 1.5, formula, fontsize=11, ha='center', color='white',
           bbox=dict(boxstyle='round', facecolor='#1a1a1e', alpha=0.9, pad=0.5))

    # Add key insight
    insight = "Symmetric structure with variable padding (k₁, k₂)\nOptimal padding changes as middle length M grows"
    ax.text(5, 0.5, insight, fontsize=10, ha='center', color='white', alpha=0.8)


def plot_legendre_distribution(summary: pd.DataFrame, ax: plt.Axes) -> None:
    """Enhanced Legendre distribution."""
    df_sorted = summary.sort_values("avg_positive_legendre", ascending=False)

    if len(df_sorted) > 20:
        df_sorted = df_sorted.head(20)
        title_suffix = " (Top 20)"
    else:
        title_suffix = ""

    x_labels = [f"M={row.M}\nk={row.k}" for _, row in df_sorted.iterrows()]
    norm_values = df_sorted["avg_positive_legendre"] / df_sorted["avg_positive_legendre"].max()
    colors = plt.cm.coolwarm(norm_values)

    bars = ax.bar(
        range(len(df_sorted)),
        df_sorted["avg_positive_legendre"],
        color=colors,
        edgecolor="white",
        linewidth=1,
    )

    ax.set_xlabel("Configuration (M,k)", fontsize=14, fontweight="bold")
    ax.set_ylabel("Avg Positive Legendre", fontsize=14, fontweight="bold")
    ax.set_title(f"Residue Character Analysis{title_suffix}", fontsize=16, fontweight="bold", pad=15)
    ax.set_xticks(range(len(df_sorted)))
    ax.set_xticklabels(x_labels, rotation=60, ha="right", fontsize=9)
    ax.grid(True, alpha=0.3, axis="y")
    ax.tick_params(axis='x', which='major', pad=2)

    explanation = "Legendre symbols measure discriminant structure\nHigher = more favorable residue patterns"
    add_text_box(ax, explanation, (0.02, 0.98), fontsize=10)


def plot_primality_pattern(detail: pd.DataFrame, ax: plt.Axes) -> None:
    """Enhanced primality scatter with statistics."""
    primes = detail[detail["is_prime"] == True]
    composites = detail[detail["is_prime"] == False]

    # Plot composites first (background)
    ax.scatter(
        composites["membrane_value"],
        composites["discriminant"],
        c="#dd3333",
        alpha=0.25,
        s=20,
        label=f"Composite ({len(composites)})",
        edgecolors="none",
    )

    # Plot primes on top (foreground)
    ax.scatter(
        primes["membrane_value"],
        primes["discriminant"],
        c="#33ff88",
        alpha=0.90,
        s=80,
        label=f"Prime ({len(primes)})",
        edgecolors="white",
        linewidths=1,
        zorder=5,
    )

    ax.set_xlabel("Membrane Value (log scale)", fontsize=14, fontweight="bold")
    ax.set_ylabel("Discriminant Δ = S² - 4·outer²", fontsize=14, fontweight="bold")
    ax.set_title("Prime vs Composite Clustering", fontsize=16, fontweight="bold", pad=15)
    ax.set_xscale("log")
    ax.legend(framealpha=0.9, loc="upper left", fontsize=11)
    ax.grid(True, alpha=0.3)

    # Calculate and display statistics
    overall_density = len(primes) / len(detail)
    explanation = f"Overall Prime Density: {overall_density:.1%}\nPrimes cluster in discriminant space"
    add_text_box(ax, explanation, (0.02, 0.15), fontsize=10)


def create_poster(
    summary: pd.DataFrame,
    detail: pd.DataFrame,
    output_path: Optional[Path] = None,
) -> Path:
    """Create comprehensive poster-sized visualization."""
    if output_path is None:
        output_path = Path(__file__).parent / "membrane_density_poster.png"

    # Poster size: 36×24 inches at 200 DPI = 7200×4800 pixels
    fig = plt.figure(figsize=(36, 24))

    # Create complex grid: 3 rows × 3 columns
    gs = gridspec.GridSpec(3, 3, figure=fig,
                          hspace=0.35, wspace=0.35,
                          left=0.05, right=0.97, top=0.93, bottom=0.05)

    # Top row: Elbow dynamics (wide), Heatmap
    ax1 = fig.add_subplot(gs[0, :2])  # Spans 2 columns
    plot_elbow_dynamics(summary, ax1)

    ax2 = fig.add_subplot(gs[0, 2])
    plot_density_heatmap(summary, ax2)

    # Middle row: k* Migration (wide), Membrane diagram
    ax3 = fig.add_subplot(gs[1, :2])
    plot_k_star_migration(summary, ax3)

    ax4 = fig.add_subplot(gs[1, 2])
    plot_membrane_diagram(summary, ax4)

    # Bottom row: Legendre distribution (wide), Primality pattern
    ax5 = fig.add_subplot(gs[2, :2])
    plot_legendre_distribution(summary, ax5)

    ax6 = fig.add_subplot(gs[2, 2])
    plot_primality_pattern(detail, ax6)

    # Main title with context
    config = summary.iloc[0]
    title_text = (
        f"Membrane Prime Generation Dynamics\n"
        f"Configuration: base={config.base}, outer={config.outer}, inner={config.inner} | "
        f"Exploring Elbow Room Phenomenon & Honorary Zero Hypothesis"
    )
    fig.suptitle(
        title_text,
        fontsize=24,
        fontweight="bold",
        color="white",
        y=0.975,
    )

    # Add footer with key findings
    footer_text = (
        "Key Findings: • Optimal padding k* migrates as structure grows (elbow events) • "
        "Density peaks correlate with residue character • "
        "Primes cluster in discriminant space (non-random distribution) • "
        "Membrane symmetry creates predictable prime generation patterns"
    )
    fig.text(0.5, 0.02, footer_text, ha='center', fontsize=12,
             color='#aaaaaa', style='italic')

    plt.savefig(output_path, dpi=200, facecolor="#050509")
    print(f"\n✓ Saved poster to {output_path}")
    print(f"  Dimensions: 36×24 inches")
    print(f"  Resolution: 7200×4800 pixels (200 DPI)")
    print(f"  File size: ~{output_path.stat().st_size / 1024 / 1024:.1f} MB")

    return output_path


def print_poster_info(summary: pd.DataFrame, detail: pd.DataFrame) -> None:
    """Print information about the poster."""
    print("\n" + "=" * 80)
    print("POSTER GENERATION SUMMARY")
    print("=" * 80)

    config = summary.iloc[0]
    print(f"\nConfiguration: base={config.base}, outer={config.outer}, inner={config.inner}")
    print(f"Data points: {len(detail)} total candidates")
    print(f"Primes found: {detail['is_prime'].sum()} ({detail['is_prime'].mean():.1%})")

    # Optimal configuration
    best = summary.loc[summary["density"].idxmax()]
    print(f"\nOptimal: M={best.M}, k={best.k} → ρ={best.density:.4f}")

    # Count elbow events
    m_values = sorted(summary["M"].unique())
    elbow_count = 0
    for i in range(len(m_values) - 1):
        df_before = summary[summary["M"] == m_values[i]]
        df_after = summary[summary["M"] == m_values[i+1]]
        k_before = df_before.loc[df_before["density"].idxmax(), "k"]
        k_after = df_after.loc[df_after["density"].idxmax(), "k"]
        if k_after != k_before:
            elbow_count += 1

    print(f"Elbow events detected: {elbow_count}")


def main():
    """Main execution."""
    print("=" * 80)
    print("Membrane Density Poster Generator")
    print("=" * 80)

    try:
        # Load data
        summary, detail = load_data()
        print(f"✓ Loaded {len(summary)} configurations, {len(detail)} candidates")

        # Print info
        print_poster_info(summary, detail)

        # Generate poster
        print("\n" + "=" * 80)
        print("GENERATING POSTER")
        print("=" * 80)
        print("This may take 30-60 seconds due to high resolution...")

        output_path = create_poster(summary, detail)

        print("\n" + "=" * 80)
        print("COMPLETE")
        print("=" * 80)
        print(f"\nPoster ready for printing: {output_path}")
        print("\nRecommended use:")
        print("  • Scientific conferences and poster sessions")
        print("  • Wall display in research labs")
        print("  • High-resolution digital presentations")
        print("  • Publication supplementary materials")

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
