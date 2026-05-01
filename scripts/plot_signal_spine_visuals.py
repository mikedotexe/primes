#!/usr/bin/env python3
"""Create first-pass steelman visuals from a signal-spine run."""

from __future__ import annotations

import argparse
import csv
import math
import warnings
from collections import defaultdict
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.colors import LinearSegmentedColormap


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "run_dir",
        nargs="?",
        type=Path,
        help="Signal spine run directory. Defaults to the newest reports/signal-spine run.",
    )
    parser.add_argument(
        "--out-dir",
        type=Path,
        help="Output directory. Defaults to <run-dir>/steelman-visuals.",
    )
    return parser.parse_args()


def newest_run_dir() -> Path:
    root = Path("reports/signal-spine")
    runs = [path for path in root.iterdir() if path.is_dir()]
    if not runs:
        raise SystemExit("no reports/signal-spine runs found")
    return max(runs, key=lambda path: path.stat().st_mtime)


def read_rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle))


def as_float(row: dict[str, str], key: str) -> float:
    value = row.get(key, "")
    return float(value) if value else 0.0


def as_int(row: dict[str, str], key: str) -> int:
    value = row.get(key, "")
    return int(float(value)) if value else 0


def pct(value: float) -> str:
    return f"{value * 100:.1f}%"


def setup_style() -> None:
    plt.rcParams.update(
        {
            "font.family": "DejaVu Sans",
            "axes.spines.top": False,
            "axes.spines.right": False,
            "axes.titleweight": "bold",
            "axes.labelcolor": "#253238",
            "xtick.color": "#253238",
            "ytick.color": "#253238",
            "figure.facecolor": "#fbfaf7",
            "axes.facecolor": "#fbfaf7",
            "savefig.facecolor": "#fbfaf7",
        }
    )


def save(fig: plt.Figure, path: Path) -> None:
    with warnings.catch_warnings():
        warnings.filterwarnings(
            "ignore",
            message="This figure includes Axes that are not compatible with tight_layout.*",
        )
        fig.tight_layout()
    fig.savefig(path, dpi=220, bbox_inches="tight")
    plt.close(fig)


def plot_period_lock(run_dir: Path, out_dir: Path) -> Path:
    rows = read_rows(run_dir / "affine/period_lock/period_lock_modulus_rows.csv")
    total = len(rows)
    mismatches = sum(row["expected_matches_observation"] != "true" for row in rows)

    residue_counts: dict[tuple[int, int], list[int]] = defaultdict(lambda: [0, 0])
    heat_counts: dict[tuple[int, int], list[int]] = defaultdict(lambda: [0, 0])
    bases = sorted({as_int(row, "base") for row in rows})
    moduli = sorted({as_int(row, "modulus") for row in rows})

    for row in rows:
        order = as_int(row, "multiplicative_order")
        delta = as_int(row, "gradient_position_delta")
        locked = row["period_lock_expected"] == "true"
        if order:
            bucket = residue_counts[(order, delta % order)]
            bucket[0] += int(locked)
            bucket[1] += 1

        heat = heat_counts[(as_int(row, "base"), as_int(row, "modulus"))]
        heat[0] += int(locked)
        heat[1] += 1

    cmap = LinearSegmentedColormap.from_list(
        "lock_campfire", ["#f3efe7", "#78a8a3", "#244b5a"]
    )
    fig, axes = plt.subplots(1, 2, figsize=(13.4, 5.8), gridspec_kw={"width_ratios": [1.05, 1]})
    fig.suptitle("Period Lock: Gradient Equality Becomes A Zero-Residue Geometry", fontsize=16)

    xs, ys, colors, sizes = [], [], [], []
    for (order, residue), (locked_count, count) in residue_counts.items():
        xs.append(order)
        ys.append(residue)
        colors.append(locked_count / count)
        sizes.append(24 + min(count, 260) * 0.25)

    ax = axes[0]
    ax.scatter(xs, ys, c=colors, s=sizes, cmap=cmap, vmin=0, vmax=1, edgecolor="#253238", linewidth=0.25)
    ax.axhline(0, color="#b33f2f", linewidth=2.0, alpha=0.75)
    ax.set_xlabel("multiplicative order of base mod p")
    ax.set_ylabel("gradient-position delta mod order")
    ax.set_title("Lock condition")
    ax.text(
        0.03,
        0.94,
        f"expected vs observed mismatches: {mismatches}/{total}",
        transform=ax.transAxes,
        ha="left",
        va="top",
        fontsize=10,
        bbox={"boxstyle": "round,pad=0.35", "facecolor": "#fffaf0", "edgecolor": "#d8c8a6"},
    )

    matrix = []
    for base in bases:
        row_values = []
        for modulus in moduli:
            locked_count, count = heat_counts.get((base, modulus), (0, 0))
            row_values.append(locked_count / count if count else float("nan"))
        matrix.append(row_values)

    ax = axes[1]
    image = ax.imshow(matrix, cmap=cmap, vmin=0, vmax=1, aspect="auto")
    ax.set_title("Locked share by base and modulus")
    ax.set_xlabel("modulus")
    ax.set_ylabel("base")
    ax.set_xticks(range(len(moduli)), [str(m) for m in moduli], rotation=45, ha="right")
    ax.set_yticks(range(len(bases)), [str(base) for base in bases])
    fig.colorbar(image, ax=ax, fraction=0.046, pad=0.04, label="locked share")

    path = out_dir / "01_period_lock_geometry.png"
    save(fig, path)
    return path


def plot_residue_torus(run_dir: Path, out_dir: Path) -> Path:
    rows = read_rows(run_dir / "affine/period_lock/period_lock_modulus_rows.csv")
    phase_counts: dict[tuple[int, int], list[int]] = defaultdict(lambda: [0, 0])

    for row in rows:
        order = as_int(row, "multiplicative_order")
        if order == 0:
            continue
        residue = as_int(row, "delta_mod_order") % order
        bucket = phase_counts[(order, residue)]
        bucket[0] += int(row["period_lock_expected"] == "true")
        bucket[1] += 1

    cmap = LinearSegmentedColormap.from_list(
        "residue_torus", ["#f2ede4", "#d18a4a", "#244b5a"]
    )
    orders = sorted({order for order, _ in phase_counts})
    max_order = max(orders)

    fig = plt.figure(figsize=(14.6, 6.4))
    fig.suptitle("Residue Torus: Period Lock Lives On The Zero Meridian", fontsize=16)
    polar_ax = fig.add_subplot(1, 2, 1, projection="polar")
    flat_ax = fig.add_subplot(1, 2, 2)

    theta_values = []
    radius_values = []
    colors = []
    sizes = []
    flat_x = []
    flat_y = []

    for (order, residue), (locked_count, count) in phase_counts.items():
        phase = residue / order
        theta_values.append(2.0 * math.pi * phase)
        radius_values.append(order)
        colors.append(locked_count / count)
        sizes.append(20 + min(count, 420) * 0.18)
        flat_x.append(phase)
        flat_y.append(order)

    polar_ax.scatter(
        theta_values,
        radius_values,
        c=colors,
        s=sizes,
        cmap=cmap,
        vmin=0,
        vmax=1,
        edgecolor="#253238",
        linewidth=0.2,
    )
    polar_ax.plot([0, 0], [0, max_order + 1], color="#b33f2f", linewidth=2.4, alpha=0.8)
    polar_ax.set_theta_zero_location("E")
    polar_ax.set_theta_direction(1)
    polar_ax.set_title("Cyclic residue rings", pad=20)
    polar_ax.set_rlabel_position(135)
    polar_ax.set_rticks([1, 5, 10, 15, 22, 30])
    polar_ax.grid(color="#d8d0c3", alpha=0.7)
    polar_ax.text(
        math.radians(8),
        max_order * 0.70,
        "lock meridian\nresidue 0",
        color="#8c3429",
        fontsize=10,
        ha="left",
        va="center",
        bbox={"boxstyle": "round,pad=0.35", "facecolor": "#fffaf0", "edgecolor": "#d8c8a6"},
    )

    image = flat_ax.scatter(
        flat_x,
        flat_y,
        c=colors,
        s=sizes,
        cmap=cmap,
        vmin=0,
        vmax=1,
        edgecolor="#253238",
        linewidth=0.2,
    )
    flat_ax.axvline(0, color="#b33f2f", linewidth=2.0, alpha=0.8)
    flat_ax.axvline(1, color="#b33f2f", linewidth=1.2, alpha=0.35)
    flat_ax.set_xlim(-0.03, 1.03)
    flat_ax.set_ylim(0, max_order + 2)
    flat_ax.set_xlabel("unwrapped phase: delta mod order / order")
    flat_ax.set_ylabel("multiplicative order")
    flat_ax.set_title("Unwrapped torus")
    flat_ax.grid(axis="y", color="#ded6ca", alpha=0.55)
    fig.colorbar(image, ax=flat_ax, fraction=0.046, pad=0.04, label="locked share")

    path = out_dir / "04_residue_torus_period_lock.png"
    save(fig, path)
    return path


def plot_gradient_surface(run_dir: Path, out_dir: Path) -> Path:
    relation_rows = read_rows(
        run_dir / "affine/gradient_transition/gradient_relation_summary_rows.csv"
    )
    summary_rows = read_rows(run_dir / "affine/gradient_transition/gradient_summary_rows.csv")

    relation_names = [
        ("identity_share", "identity", "#2f3f46"),
        ("shift_only_share", "shift only", "#d39b3d"),
        ("gradient_only_share", "gradient only", "#c84d3a"),
        ("shift_and_gradient_share", "shift + gradient", "#4f9f8f"),
    ]
    selected = [
        row
        for row in relation_rows
        if row["scope"] == "main" and row["surface_kind"] in {"direct_all", "best_surface"}
    ]
    selected.sort(key=lambda row: (as_int(row, "middle_length"), row["surface_kind"]))

    fig, axes = plt.subplots(1, 2, figsize=(14, 5.8), gridspec_kw={"width_ratios": [1.25, 1]})
    fig.suptitle("Affine Gradient Transition: Broad Lanes Stay Noisy, Winners Tighten", fontsize=16)

    labels = [
        f"M{row['middle_length']}\n{'direct' if row['surface_kind'] == 'direct_all' else 'best'}"
        for row in selected
    ]
    x_positions = list(range(len(selected)))
    bottoms = [0.0] * len(selected)
    ax = axes[0]
    for key, label, color in relation_names:
        values = [as_float(row, key) for row in selected]
        ax.bar(x_positions, values, bottom=bottoms, color=color, label=label, width=0.72)
        bottoms = [a + b for a, b in zip(bottoms, values)]
    ax.set_xticks(x_positions, labels)
    ax.set_ylim(0, 1.0)
    ax.set_ylabel("relation share")
    ax.set_title("Relation mass on main surface")
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.15), ncol=2, frameon=False)

    main_all = [
        row
        for row in summary_rows
        if row["scope"] == "main" and row["subset"] == "all_pairs"
    ]
    main_all.sort(key=lambda row: as_int(row, "middle_length"))
    middle_lengths = [as_int(row, "middle_length") for row in main_all]
    series = [
        ("any direct gradient-only pair", "any_direct_gradient_only_pair_share", "#c84d3a", "o"),
        ("winner gradient-only", "winner_gradient_only_share", "#7b2f28", "s"),
        ("full lane collapse", "full_lane_collapse_share", "#4f9f8f", "^"),
    ]

    ax = axes[1]
    for label, key, color, marker in series:
        values = [as_float(row, key) for row in main_all]
        ax.plot(middle_lengths, values, marker=marker, linewidth=2.5, markersize=7, color=color, label=label)
    m2 = next(row for row in main_all if as_int(row, "middle_length") == 2)
    ax.annotate(
        f"M2 pocket:\ndirect-any {pct(as_float(m2, 'any_direct_gradient_only_pair_share'))}\nwinner {pct(as_float(m2, 'winner_gradient_only_share'))}",
        xy=(2, as_float(m2, "any_direct_gradient_only_pair_share")),
        xytext=(2.18, 0.47),
        arrowprops={"arrowstyle": "->", "color": "#7b2f28", "lw": 1.3},
        fontsize=10,
        bbox={"boxstyle": "round,pad=0.35", "facecolor": "#fffaf0", "edgecolor": "#d8c8a6"},
    )
    ax.set_xticks(middle_lengths)
    ax.set_ylim(0, 1.02)
    ax.set_xlabel("middle length M")
    ax.set_ylabel("pair share")
    ax.set_title("The M2 gradient pocket avoids the winner")
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.15), frameon=False)

    path = out_dir / "02_affine_gradient_surface.png"
    save(fig, path)
    return path


def plot_transfer_collapse(run_dir: Path, out_dir: Path) -> Path:
    vocab_rows = read_rows(
        run_dir / "transfer/m2_m3_transfer_collapse/vocabulary_summary_rows.csv"
    )
    base_rows = read_rows(
        run_dir / "transfer/m2_m3_transfer_collapse/base_transfer_summary_rows.csv"
    )

    fig, axes = plt.subplots(1, 2, figsize=(14, 5.8), gridspec_kw={"width_ratios": [1.05, 1.2]})
    fig.suptitle("M2 To M3 Transfer Collapse: Active Vocabulary Falls To Identity", fontsize=16)

    ax = axes[0]
    colors = {2: "#c84d3a", 3: "#244b5a"}
    markers = {"all": "o", "active": "s"}
    points = {}
    for row in vocab_rows:
        if row["scope"] == "active" and as_int(row, "row_count") == 0:
            continue
        middle = as_int(row, "middle_length")
        scope = row["scope"]
        x = as_float(row, "mean_nonidentity_transfer_share")
        y = as_float(row, "identity_collapse_share")
        size = 80 + min(as_int(row, "row_count"), 700) * 0.45
        ax.scatter(
            [x],
            [y],
            s=size,
            marker=markers.get(scope, "o"),
            color=colors.get(middle, "#777777"),
            edgecolor="#253238",
            linewidth=0.6,
            alpha=0.9,
        )
        label = f"M{middle} {scope}"
        label_offsets = {
            (2, "all"): (0.028, -0.065),
            (2, "active"): (0.012, 0.035),
            (3, "all"): (0.014, 0.030),
        }
        dx, dy = label_offsets.get((middle, scope), (0.012, 0.035 if y < 0.5 else -0.065))
        ax.text(x + dx, y + dy, label, fontsize=10)
        points[(middle, scope)] = (x, y)

    if (2, "all") in points and (3, "all") in points:
        x0, y0 = points[(2, "all")]
        x1, y1 = points[(3, "all")]
        ax.annotate("", xy=(x1, y1), xytext=(x0, y0), arrowprops={"arrowstyle": "->", "lw": 2, "color": "#244b5a"})
    if (2, "active") in points and (3, "all") in points:
        x0, y0 = points[(2, "active")]
        x1, y1 = points[(3, "all")]
        ax.annotate(
            "",
            xy=(x1, y1),
            xytext=(x0, y0),
            arrowprops={"arrowstyle": "->", "lw": 1.7, "color": "#b9792e", "linestyle": "--"},
        )

    ax.set_xlim(-0.025, 0.55)
    ax.set_ylim(-0.045, 1.08)
    ax.set_xlabel("mean nonidentity transfer share")
    ax.set_ylabel("identity collapse share")
    ax.set_title("Phase portrait: expression to collapse")

    ax = axes[1]
    m2_rows = sorted(
        [row for row in base_rows if as_int(row, "middle_length") == 2],
        key=lambda row: as_int(row, "base"),
    )
    bases = [as_int(row, "base") for row in m2_rows]
    shares = [as_float(row, "active_pair_share") for row in m2_rows]
    x_positions = list(range(len(bases)))
    ax.vlines(x_positions, 0, shares, color="#7a8b8c", linewidth=2)
    ax.scatter(x_positions, shares, s=110, color="#c84d3a", edgecolor="#253238", zorder=3, label="M2 active share")
    ax.scatter(x_positions, [0] * len(bases), s=80, color="#244b5a", edgecolor="#253238", zorder=3, label="M3 active share")
    for x, share in zip(x_positions, shares):
        if share > 0:
            ax.annotate("", xy=(x, 0.015), xytext=(x, share - 0.015), arrowprops={"arrowstyle": "->", "lw": 1.1, "color": "#244b5a"})
    ax.set_xticks(x_positions, [str(base) for base in bases])
    ax.set_ylim(0, max(shares) * 1.2 if shares else 1)
    ax.set_xlabel("base")
    ax.set_ylabel("active pair share")
    ax.set_title("Per-base active vocabulary collapses at M3")
    ax.legend(frameon=False)

    path = out_dir / "03_transfer_collapse_phase.png"
    save(fig, path)
    return path


def transition_rows(run_dir: Path) -> list[dict[str, str]]:
    rows = read_rows(run_dir / "affine/gradient_transition/gradient_summary_rows.csv")
    selected = [
        row
        for row in rows
        if row["scope"] == "main" and row["subset"] == "all_pairs"
    ]
    selected.sort(key=lambda row: as_int(row, "middle_length"))
    return selected


def plot_m_transition_storyboard(run_dir: Path, out_dir: Path) -> Path:
    rows = transition_rows(run_dir)

    fig = plt.figure(figsize=(14.5, 7.2))
    fig.suptitle("M1 To M2 To M3: 2D Storyboard For The Future Animation", fontsize=16)
    grid = fig.add_gridspec(2, 3, height_ratios=[1.05, 1], hspace=0.42, wspace=0.3)
    path_ax = fig.add_subplot(grid[0, :])

    middle_lengths = [as_int(row, "middle_length") for row in rows]
    path_series = [
        ("active winners", "best_active_pair_share", "#244b5a", "o"),
        ("direct gradient-only", "any_direct_gradient_only_pair_share", "#c84d3a", "o"),
        ("winner gradient-only", "winner_gradient_only_share", "#842e28", "s"),
        ("full collapse", "full_lane_collapse_share", "#4f9f8f", "^"),
    ]
    path_ax.axvspan(1.82, 2.18, color="#f0dfc3", alpha=0.45, zorder=0)
    path_ax.text(2, 0.965, "M2 hinge", ha="center", va="top", fontsize=10, color="#7b4e1c")
    for label, key, color, marker in path_series:
        values = [as_float(row, key) for row in rows]
        path_ax.plot(
            middle_lengths,
            values,
            marker=marker,
            color=color,
            linewidth=2.5,
            markersize=8,
            label=label,
        )
    for left, right in zip(rows, rows[1:]):
        path_ax.annotate(
            "",
            xy=(as_int(right, "middle_length"), as_float(right, "full_lane_collapse_share")),
            xytext=(as_int(left, "middle_length"), as_float(left, "full_lane_collapse_share")),
            arrowprops={"arrowstyle": "->", "lw": 1.8, "color": "#4f9f8f"},
        )
    path_ax.set_xlabel("middle length M")
    path_ax.set_ylabel("share")
    path_ax.set_xticks(middle_lengths, [f"M{m}" for m in middle_lengths])
    path_ax.set_ylim(0, 1.02)
    path_ax.set_title("The motion: direct possibility dips at M2 while collapse peaks")
    path_ax.grid(color="#ded6ca", alpha=0.6)
    path_ax.legend(loc="center left", bbox_to_anchor=(1.01, 0.5), frameon=False)

    bar_keys = [
        ("best_active_pair_share", "active winners", "#244b5a"),
        ("any_direct_gradient_only_pair_share", "direct gradient-only", "#c84d3a"),
        ("winner_gradient_only_share", "winner gradient-only", "#842e28"),
        ("full_lane_collapse_share", "full collapse", "#4f9f8f"),
    ]

    for idx, row in enumerate(rows):
        ax = fig.add_subplot(grid[1, idx])
        middle = as_int(row, "middle_length")
        values = [as_float(row, key) for key, _, _ in bar_keys]
        colors_for_bars = [color for _, _, color in bar_keys]
        labels = [label for _, label, _ in bar_keys]
        ax.bar(range(len(values)), values, color=colors_for_bars, width=0.72)
        ax.set_title(f"M{middle}")
        ax.set_ylim(0, 1.02)
        ax.set_xticks(range(len(values)), labels, rotation=30, ha="right")
        if idx == 0:
            ax.set_ylabel("share")
        for i, value in enumerate(values):
            ax.text(i, value + 0.025, pct(value), ha="center", fontsize=9)

    path = out_dir / "05_m_transition_storyboard.png"
    save(fig, path)
    return path


def write_m_transition_frames(run_dir: Path, out_dir: Path) -> list[Path]:
    frame_dir = out_dir / "m-transition-frames"
    frame_dir.mkdir(parents=True, exist_ok=True)
    rows = transition_rows(run_dir)
    paths = []
    bar_keys = [
        ("best_active_pair_share", "active winners", "#244b5a"),
        ("any_direct_gradient_only_pair_share", "direct gradient-only", "#c84d3a"),
        ("winner_gradient_only_share", "winner gradient-only", "#842e28"),
        ("full_lane_collapse_share", "full collapse", "#4f9f8f"),
    ]

    for row in rows:
        middle = as_int(row, "middle_length")
        fig, ax = plt.subplots(figsize=(8, 5))
        values = [as_float(row, key) for key, _, _ in bar_keys]
        labels = [label for _, label, _ in bar_keys]
        colors = [color for _, _, color in bar_keys]
        ax.bar(range(len(values)), values, color=colors, width=0.68)
        ax.set_title(f"M{middle} transition frame")
        ax.set_ylim(0, 1.02)
        ax.set_ylabel("share")
        ax.set_xticks(range(len(values)), labels, rotation=25, ha="right")
        for i, value in enumerate(values):
            ax.text(i, value + 0.025, pct(value), ha="center", fontsize=10)
        ax.text(
            0.02,
            0.94,
            "static frame for future animation",
            transform=ax.transAxes,
            ha="left",
            va="top",
            fontsize=10,
            bbox={"boxstyle": "round,pad=0.35", "facecolor": "#fffaf0", "edgecolor": "#d8c8a6"},
        )
        path = frame_dir / f"m{middle}_transition_frame.png"
        save(fig, path)
        paths.append(path)
    return paths


def write_readme(run_dir: Path, out_dir: Path, paths: list[Path]) -> None:
    lines = [
        "# Steelman Visuals",
        "",
        f"Generated from `{run_dir}`.",
        "",
        "These are first-pass geometric views of the three signals we are actively steelmanning:",
        "",
        "1. `01_period_lock_geometry.png` - period lock as a zero-residue condition for affine gradient position.",
        "2. `02_affine_gradient_surface.png` - relation mass and the M2 winner surface avoiding gradient-only lanes.",
        "3. `03_transfer_collapse_phase.png` - M2 active transfer vocabulary collapsing to identity at M3.",
        "4. `04_residue_torus_period_lock.png` - cyclic residue rings plus an unwrapped residue torus.",
        "5. `05_m_transition_storyboard.png` - a static 2D storyboard for a future M1/M2/M3 animation.",
        "6. `m-transition-frames/` - separate frame-style stills for M1, M2, and M3.",
        "",
        "Generated files:",
        "",
    ]
    lines.extend(f"- `{path.name}`" for path in paths)
    (out_dir / "README.md").write_text("\n".join(lines) + "\n")


def main() -> None:
    args = parse_args()
    run_dir = args.run_dir or newest_run_dir()
    out_dir = args.out_dir or run_dir / "steelman-visuals"
    out_dir.mkdir(parents=True, exist_ok=True)
    setup_style()

    paths = [
        plot_period_lock(run_dir, out_dir),
        plot_gradient_surface(run_dir, out_dir),
        plot_transfer_collapse(run_dir, out_dir),
        plot_residue_torus(run_dir, out_dir),
        plot_m_transition_storyboard(run_dir, out_dir),
    ]
    paths.extend(write_m_transition_frames(run_dir, out_dir))
    write_readme(run_dir, out_dir, paths)

    for path in paths:
        print(path)


if __name__ == "__main__":
    main()
