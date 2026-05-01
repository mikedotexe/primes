#!/usr/bin/env python3
"""
Polished matplotlib report for the chaos-to-threshold translation artifact.

This companion turns the maintained arithmetic CSV export into a more legible
visual report with:

- a storyboard figure with legends and stronger typography
- a dedicated threshold regime heatmap
- a dedicated M=2 decomposition plane
- bubble-size comparison views for survival-style interpretations
- residue-geometry views built from unit-circle embeddings
- a markdown summary that travels with the images

Default input:
    /tmp/primes_chaos_threshold_translation

Run:
    python3 scripts/plot_chaos_threshold_translation.py
    python3 scripts/plot_chaos_threshold_translation.py \
        --input-dir /tmp/primes_chaos_threshold_translation \
        --out-dir /tmp/primes_chaos_threshold_translation_matplotlib
"""

from __future__ import annotations

import argparse
import csv
import json
from dataclasses import dataclass
from math import cos, gcd, pi, sin
from pathlib import Path
from typing import Iterable

import matplotlib as mpl
import matplotlib.pyplot as plt
from matplotlib.colors import ListedColormap
from matplotlib.lines import Line2D
from matplotlib.patches import Arc, Circle, FancyArrowPatch, Patch, Rectangle


DEFAULT_INPUT_DIR = Path("/tmp/primes_chaos_threshold_translation")
DEFAULT_OUT_DIR = Path("/tmp/primes_chaos_threshold_translation_matplotlib")
ARTIFACT_ID = "chaos_threshold_translation_matplotlib"

BASES = [6, 10, 12, 14, 22, 26, 30, 34]
LENGTHS = [1, 2, 3]

REGIME_COLORS = {
    "stable_regime": "#6FAE6A",
    "boundary_layer": "#E4B85D",
    "anomaly_rich": "#D96B5B",
}

SOURCE_COLORS = {
    "stable_zero_led": "#2C8F62",
    "boundary_led": "#DA8230",
    "mixed_or_flat": "#7F7F7F",
    "none": "#C8C8C8",
}

ACCENT_BLUE = "#2B6E94"
GRID_COLOR = "#D9D4CB"
TEXT_DARK = "#2E2A26"
BACKGROUND = "#F7F4EE"


@dataclass
class BaseMRow:
    base: int
    middle_length: int
    ordered_pair_count: int
    active_pair_count: int
    active_pair_share: float
    k00_noninferior_share: float
    anomaly_mass_pp: float
    mean_anomaly_mass_pp_given_active: float | None
    mean_admissible_delta_pp_given_active: float | None
    mean_stable_zero_prime_delta_pp_given_active: float | None
    mean_boundary_prime_delta_pp_given_active: float | None
    mean_shared_prime_rate_delta_pp_given_active: float | None
    dominant_signal_source_label: str
    regime_label: str


@dataclass
class PairThresholdRow:
    base: int
    middle_length: int
    outer: int
    inner: int
    pair_label: str
    best_k: str
    active: bool
    anomaly_mass_pp: float
    stable_zero_prime_delta_pp: float
    boundary_prime_delta_pp: float
    shared_prime_rate_delta_pp: float
    signal_source_label: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-dir", type=Path, default=DEFAULT_INPUT_DIR)
    parser.add_argument("--out-dir", type=Path, default=DEFAULT_OUT_DIR)
    return parser.parse_args()


def configure_matplotlib() -> None:
    mpl.rcParams.update(
        {
            "figure.facecolor": BACKGROUND,
            "axes.facecolor": BACKGROUND,
            "savefig.facecolor": BACKGROUND,
            "axes.edgecolor": "#7F786D",
            "axes.labelcolor": TEXT_DARK,
            "text.color": TEXT_DARK,
            "xtick.color": TEXT_DARK,
            "ytick.color": TEXT_DARK,
            "font.family": "DejaVu Sans",
            "font.size": 11,
            "axes.titlesize": 15,
            "axes.titleweight": "bold",
            "axes.labelsize": 12,
            "axes.grid": True,
            "grid.color": GRID_COLOR,
            "grid.linewidth": 0.8,
            "grid.alpha": 0.65,
            "legend.frameon": False,
        }
    )


def parse_optional_float(value: str) -> float | None:
    return None if value == "" else float(value)


def load_base_rows(path: Path) -> list[BaseMRow]:
    with path.open() as handle:
        reader = csv.DictReader(handle)
        rows = []
        for row in reader:
            rows.append(
                BaseMRow(
                    base=int(row["base"]),
                    middle_length=int(row["middle_length"]),
                    ordered_pair_count=int(row["ordered_pair_count"]),
                    active_pair_count=int(row["active_pair_count"]),
                    active_pair_share=float(row["active_pair_share"]),
                    k00_noninferior_share=float(row["k00_noninferior_share"]),
                    anomaly_mass_pp=float(row["anomaly_mass_pp"]),
                    mean_anomaly_mass_pp_given_active=parse_optional_float(
                        row["mean_anomaly_mass_pp_given_active"]
                    ),
                    mean_admissible_delta_pp_given_active=parse_optional_float(
                        row["mean_admissible_delta_pp_given_active"]
                    ),
                    mean_stable_zero_prime_delta_pp_given_active=parse_optional_float(
                        row["mean_stable_zero_prime_delta_pp_given_active"]
                    ),
                    mean_boundary_prime_delta_pp_given_active=parse_optional_float(
                        row["mean_boundary_prime_delta_pp_given_active"]
                    ),
                    mean_shared_prime_rate_delta_pp_given_active=parse_optional_float(
                        row["mean_shared_prime_rate_delta_pp_given_active"]
                    ),
                    dominant_signal_source_label=row["dominant_signal_source_label"],
                    regime_label=row["regime_label"],
                )
            )
        return rows


def load_pair_rows(path: Path) -> list[PairThresholdRow]:
    with path.open() as handle:
        reader = csv.DictReader(handle)
        rows = []
        for row in reader:
            rows.append(
                PairThresholdRow(
                    base=int(row["base"]),
                    middle_length=int(row["middle_length"]),
                    outer=int(row["outer"]),
                    inner=int(row["inner"]),
                    pair_label=row["pair_label"],
                    best_k=row["best_k"],
                    active=row["active"].lower() == "true",
                    anomaly_mass_pp=float(row["anomaly_mass_pp"]),
                    stable_zero_prime_delta_pp=float(row["stable_zero_prime_delta_pp"]),
                    boundary_prime_delta_pp=float(row["boundary_prime_delta_pp"]),
                    shared_prime_rate_delta_pp=float(row["shared_prime_rate_delta_pp"]),
                    signal_source_label=row["signal_source_label"],
                )
            )
        return rows


def ensure_dir(path: Path) -> None:
    path.mkdir(parents=True, exist_ok=True)


def group_by_middle(rows: Iterable[BaseMRow]) -> dict[int, list[BaseMRow]]:
    grouped: dict[int, list[BaseMRow]] = {}
    for row in rows:
        grouped.setdefault(row.middle_length, []).append(row)
    return grouped


def grouped_lookup(rows: list[BaseMRow]) -> dict[tuple[int, int], BaseMRow]:
    return {(row.base, row.middle_length): row for row in rows}


def unit_residues(base: int) -> list[int]:
    return [digit for digit in range(1, base) if gcd(digit, base) == 1]


def euler_phi(n: int) -> int:
    count = 0
    for k in range(1, n + 1):
        if gcd(k, n) == 1:
            count += 1
    return count


def format_pp(value: float | None) -> str:
    return "n/a" if value is None else f"{value:.2f}pp"


def global_totals(rows: list[BaseMRow]) -> tuple[list[float], list[float]]:
    by_middle = group_by_middle(rows)
    totals = [sum(row.anomaly_mass_pp for row in by_middle[m]) for m in LENGTHS]
    active_shares = [
        sum(row.active_pair_count for row in by_middle[m])
        / max(1, sum(row.ordered_pair_count for row in by_middle[m]))
        for m in LENGTHS
    ]
    return totals, active_shares


def add_panel_header(ax: plt.Axes, title: str, subtitle: str = "") -> None:
    if subtitle:
        ax.set_title(f"{title}\n{subtitle}", loc="left", pad=12)
    else:
        ax.set_title(title, loc="left", pad=12)


def source_handles() -> list[Line2D]:
    labels = [
        ("stable_zero_led", "overlap-led"),
        ("boundary_led", "boundary-led"),
        ("mixed_or_flat", "mixed/flat"),
    ]
    return [
        Line2D(
            [0],
            [0],
            marker="o",
            color="none",
            markerfacecolor=SOURCE_COLORS[key],
            markeredgecolor="#333333",
            markersize=9,
            label=label,
        )
        for key, label in labels
    ]


def regime_handles() -> list[Patch]:
    labels = [
        ("stable_regime", "stable"),
        ("boundary_layer", "boundary"),
        ("anomaly_rich", "anomaly-rich"),
    ]
    return [Patch(facecolor=REGIME_COLORS[key], edgecolor="none", label=label) for key, label in labels]


def source_handles_with_lines() -> list[Line2D]:
    labels = [
        ("stable_zero_led", "overlap-led"),
        ("boundary_led", "boundary-led"),
        ("mixed_or_flat", "mixed/flat"),
    ]
    return [
        Line2D([0], [0], color=SOURCE_COLORS[key], lw=3, label=label)
        for key, label in labels
    ]


def survival_style_handles() -> list[Line2D]:
    return [
        Line2D([0], [0], color="#5B564F", lw=3, linestyle="-", label="retained from M=1"),
        Line2D([0], [0], color="#5B564F", lw=3, linestyle=(0, (4, 2)), label="emergent at M=2"),
    ]


def save_storyboard(rows: list[BaseMRow], out_path: Path) -> None:
    grouped = grouped_lookup(rows)
    totals, active_shares = global_totals(rows)

    fig = plt.figure(figsize=(18.5, 10.5), constrained_layout=False)
    gs = fig.add_gridspec(2, 2, height_ratios=[1.0, 1.05], width_ratios=[1.05, 1.0])
    ax_curve = fig.add_subplot(gs[0, 0])
    ax_heat = fig.add_subplot(gs[0, 1])
    ax_m2_plane = fig.add_subplot(gs[1, 0])
    ax_m2_signal = fig.add_subplot(gs[1, 1])
    fig.subplots_adjust(top=0.82, left=0.06, right=0.97, bottom=0.08, wspace=0.08, hspace=0.30)
    fig.suptitle("Chaos-To-Threshold Translation", fontsize=21, fontweight="bold", y=0.965)
    fig.text(
        0.5,
        0.935,
        "Arithmetic threshold report from bounded-k artifacts",
        ha="center",
        va="center",
        fontsize=13,
        color="#5C564E",
        fontweight="semibold",
    )

    # Panel 1: global collapse curve
    add_panel_header(ax_curve, "1. Global anomaly collapse", "Raw anomaly mass falls off a cliff by M=3")
    ax_curve.axvspan(0.8, 1.2, color=REGIME_COLORS["anomaly_rich"], alpha=0.12)
    ax_curve.axvspan(1.8, 2.2, color=REGIME_COLORS["boundary_layer"], alpha=0.12)
    ax_curve.axvspan(2.8, 3.2, color=REGIME_COLORS["stable_regime"], alpha=0.12)
    ax_curve.plot(LENGTHS, totals, marker="o", markersize=8, lw=3, color=ACCENT_BLUE)
    ax_curve.set_xlabel("Middle length M")
    ax_curve.set_ylabel("Total anomaly mass (pp)")
    ax_curve.set_xticks(LENGTHS, [f"M{m}" for m in LENGTHS])
    for x, y, share in zip(LENGTHS, totals, active_shares):
        ax_curve.annotate(
            f"{share*100:.1f}% active",
            (x, y),
            xytext=(0, 10),
            textcoords="offset points",
            ha="center",
            fontsize=10,
            fontweight="semibold",
        )
    ax_curve.text(1.0, max(totals) * 0.96, "anomaly-rich", ha="center", va="top", fontsize=10)
    ax_curve.text(2.0, max(totals) * 0.96, "boundary", ha="center", va="top", fontsize=10)
    ax_curve.text(3.0, max(totals) * 0.96, "stable", ha="center", va="top", fontsize=10)

    # Panel 2: anomaly heatmap
    add_panel_header(ax_heat, "2. Base-by-base threshold grid", "Cell labels show active / total ordered pairs")
    heat = [[grouped[(base, m)].anomaly_mass_pp for m in LENGTHS] for base in BASES]
    image = ax_heat.imshow(heat, cmap="YlOrRd", aspect="auto")
    ax_heat.set_xlabel("Middle length M")
    ax_heat.set_ylabel("Base")
    ax_heat.set_xticks(range(len(LENGTHS)), [f"M{m}" for m in LENGTHS])
    ax_heat.set_yticks(range(len(BASES)), [str(base) for base in BASES])
    ax_heat.grid(False)
    for i, base in enumerate(BASES):
        for j, m in enumerate(LENGTHS):
            row = grouped[(base, m)]
            ax_heat.text(
                j,
                i,
                f"{row.active_pair_count}/{row.ordered_pair_count}",
                ha="center",
                va="center",
                fontsize=9,
                fontweight="semibold",
            )
    cbar = fig.colorbar(image, ax=ax_heat, fraction=0.046, pad=0.04)
    cbar.set_label("anomaly mass (pp)")

    # Panel 3: M=2 decomposition plane
    add_panel_header(ax_m2_plane, "3. M=2 decomposition plane", "x = overlap-led delta, y = boundary-led delta")
    m2_rows = [row for row in rows if row.middle_length == 2]
    for row in m2_rows:
        x = row.mean_stable_zero_prime_delta_pp_given_active or 0.0
        y = row.mean_boundary_prime_delta_pp_given_active or 0.0
        size = 110 + 340 * max(row.active_pair_share, 0.01)
        ax_m2_plane.scatter(
            x,
            y,
            s=size,
            color=SOURCE_COLORS.get(row.dominant_signal_source_label, SOURCE_COLORS["mixed_or_flat"]),
            alpha=0.92,
            edgecolor="#2f2a26",
            linewidth=0.7,
        )
        ax_m2_plane.annotate(
            str(row.base),
            (x, y),
            xytext=(6, 5),
            textcoords="offset points",
            fontsize=10,
            fontweight="semibold",
        )
    ax_m2_plane.axhline(0, color="#8B8379", lw=1.1)
    ax_m2_plane.axvline(0, color="#8B8379", lw=1.1)
    ax_m2_plane.set_xlabel("mean stable-zero delta (pp)")
    ax_m2_plane.set_ylabel("mean boundary delta (pp)")
    ax_m2_plane.legend(
        handles=source_handles(),
        loc="upper right",
        title="signal source",
        fontsize=10,
        title_fontsize=10,
    )

    # Panel 4: M=2 shared-rate plane
    add_panel_header(ax_m2_signal, "4. M=2 overlap vs shared-rate", "This is where base 14 and base 34 split apart")
    for row in m2_rows:
        x = row.mean_stable_zero_prime_delta_pp_given_active or 0.0
        y = row.mean_shared_prime_rate_delta_pp_given_active or 0.0
        size = 110 + 340 * max(row.active_pair_share, 0.01)
        ax_m2_signal.scatter(
            x,
            y,
            s=size,
            color=SOURCE_COLORS.get(row.dominant_signal_source_label, SOURCE_COLORS["mixed_or_flat"]),
            alpha=0.92,
            edgecolor="#2f2a26",
            linewidth=0.7,
        )
        ax_m2_signal.annotate(
            str(row.base),
            (x, y),
            xytext=(6, 5),
            textcoords="offset points",
            fontsize=10,
            fontweight="semibold",
        )
    ax_m2_signal.axhline(0, color="#8B8379", lw=1.1)
    ax_m2_signal.axvline(0, color="#8B8379", lw=1.1)
    ax_m2_signal.set_xlabel("mean stable-zero delta (pp)")
    ax_m2_signal.set_ylabel("mean shared prime-rate delta (pp)")
    ax_m2_signal.text(
        0.98,
        0.03,
        "Base 14: overlap-positive and shared-rate positive\nBase 34: weakly overlap-negative and shared-rate negative",
        transform=ax_m2_signal.transAxes,
        ha="right",
        va="bottom",
        fontsize=10,
        bbox={"boxstyle": "round,pad=0.4", "facecolor": "#FFF9EC", "edgecolor": "#D8C89F"},
    )

    fig.savefig(out_path, dpi=200, bbox_inches="tight")
    plt.close(fig)


def save_regime_heatmap(rows: list[BaseMRow], out_path: Path) -> None:
    grouped = grouped_lookup(rows)
    regime_order = ["stable_regime", "boundary_layer", "anomaly_rich"]
    regime_to_index = {label: index for index, label in enumerate(regime_order)}
    matrix = [[regime_to_index[grouped[(base, m)].regime_label] for m in LENGTHS] for base in BASES]

    cmap = ListedColormap([REGIME_COLORS[key] for key in regime_order])
    fig, ax = plt.subplots(figsize=(9.0, 6.6), constrained_layout=True)
    add_panel_header(ax, "Threshold regime heatmap", "Colors encode the arithmetic regime, labels show active/total")
    image = ax.imshow(matrix, cmap=cmap, aspect="auto", vmin=0, vmax=len(regime_order) - 1)
    ax.set_xlabel("Middle length M")
    ax.set_ylabel("Base")
    ax.set_xticks(range(len(LENGTHS)), [f"M{m}" for m in LENGTHS])
    ax.set_yticks(range(len(BASES)), [str(base) for base in BASES])
    ax.grid(False)
    for i, base in enumerate(BASES):
        for j, m in enumerate(LENGTHS):
            row = grouped[(base, m)]
            ax.text(
                j,
                i,
                f"{row.active_pair_count}/{row.ordered_pair_count}",
                ha="center",
                va="center",
                fontsize=9,
                fontweight="semibold",
            )
    ax.legend(
        handles=regime_handles(),
        loc="upper center",
        bbox_to_anchor=(0.5, -0.08),
        ncol=3,
        title="regime",
        fontsize=10,
        title_fontsize=10,
    )
    fig.colorbar(image, ax=ax, ticks=[0, 1, 2], fraction=0.046, pad=0.04)
    fig.savefig(out_path, dpi=200, bbox_inches="tight")
    plt.close(fig)


def save_compactness_glossary(rows: list[BaseMRow], out_path: Path) -> None:
    fig = plt.figure(figsize=(15.8, 8.6), constrained_layout=True)
    gs = fig.add_gridspec(2, 2, height_ratios=[0.9, 1.0], width_ratios=[1.15, 1.0])
    ax_template = fig.add_subplot(gs[0, :])
    ax_ladder = fig.add_subplot(gs[1, 0])
    ax_threshold = fig.add_subplot(gs[1, 1])

    fig.suptitle("Compactness Glossary", fontsize=20, fontweight="bold", y=1.02)
    fig.text(
        0.5,
        0.975,
        "In the bounded-k lane, compactness means less zero padding and lower template diameter.",
        ha="center",
        va="center",
        fontsize=13,
        color="#5C564E",
        fontweight="semibold",
    )

    # Panel 1: template anatomy
    ax_template.set_xlim(0, 10)
    ax_template.set_ylim(0, 2.6)
    ax_template.axis("off")
    add_panel_header(
        ax_template,
        "1. Template anatomy",
        "Compactness is controlled by the zero runs `k_outer` and `k_inner`",
    )
    block_specs = [
        ("outer", "#DDEBF3", 1.0),
        ("0^k_outer", "#F6E7B7", 1.25),
        ("inner", "#DDEBF3", 1.0),
        ("0^k_inner", "#F6E7B7", 1.25),
        ("SEED", "#D9EFD9", 1.2),
        ("0^k_inner", "#F6E7B7", 1.25),
        ("inner", "#DDEBF3", 1.0),
        ("0^k_outer", "#F6E7B7", 1.25),
        ("outer", "#DDEBF3", 1.0),
    ]
    x = 0.35
    y = 1.15
    for label, color, width in block_specs:
        rect = Rectangle((x, y), width, 0.72, facecolor=color, edgecolor="#7F786D", linewidth=1.0)
        ax_template.add_patch(rect)
        ax_template.text(x + width / 2, y + 0.36, label, ha="center", va="center", fontsize=11, fontweight="semibold")
        x += width + 0.08
    ax_template.text(
        5.0,
        0.48,
        "More padding means larger separation between the mirrored boundary digits and the seed core.\nA simple visual diameter proxy in this lane is total extra zeros = 2·(k_outer + k_inner).",
        ha="center",
        va="center",
        fontsize=11,
        bbox={"boxstyle": "round,pad=0.4", "facecolor": "#FFF9EC", "edgecolor": "#D8C89F"},
    )

    # Panel 2: compactness ladder
    add_panel_header(
        ax_ladder,
        "2. Bounded-k compactness ladder",
        "Lower total extra padding = more compact",
    )
    k_rows = [
        ("k=(0,0)", 0, "#2C8F62"),
        ("k=(0,1)", 2, "#6BAE78"),
        ("k=(1,0)", 2, "#6BAE78"),
        ("k=(1,1)", 4, "#DAA251"),
        ("k=(2,2)", 8, "#D96B5B"),
    ]
    labels = [label for label, _, _ in k_rows]
    values = [value for _, value, _ in k_rows]
    colors = [color for _, _, color in k_rows]
    ax_ladder.bar(labels, values, color=colors, edgecolor="#3C362F", linewidth=0.8)
    ax_ladder.set_ylabel("extra zeros in full template")
    ax_ladder.set_xlabel("bounded-k configuration")
    ax_ladder.set_ylim(0, max(values) + 1.5)
    for index, (_, value, _) in enumerate(k_rows):
        adjective = "most compact" if index == 0 else ("same tier" if value == 2 else "less compact")
        ax_ladder.text(index, value + 0.22, f"{value}\n{adjective}", ha="center", va="bottom", fontsize=10)
    ax_ladder.text(
        0.01,
        0.96,
        "Compactness is a padding property, not a primality claim by itself.",
        transform=ax_ladder.transAxes,
        ha="left",
        va="top",
        fontsize=10,
        bbox={"boxstyle": "round,pad=0.3", "facecolor": "#FFF9EC", "edgecolor": "#D8C89F"},
    )

    # Panel 3: threshold relation
    add_panel_header(
        ax_threshold,
        "3. Compactness across the threshold",
        "What the maintained arithmetic lane is currently saying",
    )
    ax_threshold.axis("off")
    box_specs = [
        (
            0.08,
            0.66,
            0.84,
            0.20,
            "#F2D5CF",
            "M=1  anomaly-rich",
            "Several higher-padding lanes can still beat k=(0,0).",
        ),
        (
            0.08,
            0.38,
            0.84,
            0.20,
            "#F7E7C1",
            "M=2  boundary layer",
            "Only a sparse remnant survives; some bases remain overlap-led, others boundary-led.",
        ),
        (
            0.08,
            0.10,
            0.84,
            0.20,
            "#DCEFD8",
            "M=3  stable regime",
            "k=(0,0) becomes noninferior on the maintained catalog.",
        ),
    ]
    for x0, y0, w, h, color, title, body in box_specs:
        rect = Rectangle((x0, y0), w, h, transform=ax_threshold.transAxes, facecolor=color, edgecolor="#7F786D", linewidth=1.0)
        ax_threshold.add_patch(rect)
        ax_threshold.text(x0 + 0.03, y0 + h - 0.055, title, transform=ax_threshold.transAxes, ha="left", va="top", fontsize=12, fontweight="bold")
        ax_threshold.text(x0 + 0.03, y0 + 0.05, body, transform=ax_threshold.transAxes, ha="left", va="bottom", fontsize=10.5, wrap=True)
    ax_threshold.text(
        0.5,
        0.005,
        "Working definition: compactness dominates when extra padding no longer sustains positive anomaly mass.",
        transform=ax_threshold.transAxes,
        ha="center",
        va="bottom",
        fontsize=10.5,
        bbox={"boxstyle": "round,pad=0.32", "facecolor": "#FFF9EC", "edgecolor": "#D8C89F"},
    )

    fig.savefig(out_path, dpi=220, bbox_inches="tight")
    plt.close(fig)


def save_m2_decomposition_plane(rows: list[BaseMRow], out_path: Path) -> None:
    m2_rows = [row for row in rows if row.middle_length == 2]
    fig, ax = plt.subplots(figsize=(9.4, 7.2), constrained_layout=True)
    add_panel_header(ax, "M=2 decomposition plane", "Bubble size tracks active-pair share; colors track signal source")
    for row in m2_rows:
        x = row.mean_stable_zero_prime_delta_pp_given_active or 0.0
        y = row.mean_boundary_prime_delta_pp_given_active or 0.0
        size = 120 + 380 * max(row.active_pair_share, 0.01)
        ax.scatter(
            x,
            y,
            s=size,
            color=SOURCE_COLORS.get(row.dominant_signal_source_label, SOURCE_COLORS["mixed_or_flat"]),
            alpha=0.92,
            edgecolor="#2f2a26",
            linewidth=0.8,
        )
        ax.annotate(
            f"{row.base}",
            (x, y),
            xytext=(7, 5),
            textcoords="offset points",
            fontsize=10,
            fontweight="semibold",
        )
    ax.axhline(0, color="#8B8379", lw=1.1)
    ax.axvline(0, color="#8B8379", lw=1.1)
    ax.set_xlabel("mean stable-zero delta (pp)")
    ax.set_ylabel("mean boundary delta (pp)")
    ax.text(
        0.02,
        0.97,
        "upper-left: overlap weak / boundary strong\nupper-right: both positive\nlower-left: overlap negative / boundary weak",
        transform=ax.transAxes,
        ha="left",
        va="top",
        fontsize=10,
        bbox={"boxstyle": "round,pad=0.35", "facecolor": "#FFF9EC", "edgecolor": "#D8C89F"},
    )
    ax.legend(handles=source_handles(), loc="upper right", title="signal source", fontsize=10, title_fontsize=10)
    fig.savefig(out_path, dpi=200, bbox_inches="tight")
    plt.close(fig)


def strongest_takeaways(rows: list[BaseMRow]) -> list[str]:
    grouped = grouped_lookup(rows)
    totals, active_shares = global_totals(rows)
    base14 = grouped[(14, 2)]
    base34 = grouped[(34, 2)]
    return [
        f"M=1 remains anomaly-rich: total anomaly mass {totals[0]:.2f}pp with {active_shares[0]*100:.1f}% active ordered pairs.",
        f"M=2 is the sparse boundary layer: total anomaly mass {totals[1]:.2f}pp with only {active_shares[1]*100:.1f}% active ordered pairs.",
        f"M=3 is the stable cutoff: total anomaly mass {totals[2]:.2f}pp and {active_shares[2]*100:.1f}% active ordered pairs.",
        f"Base 14 stays overlap-led at M=2: stable-zero {format_pp(base14.mean_stable_zero_prime_delta_pp_given_active)}, boundary {format_pp(base14.mean_boundary_prime_delta_pp_given_active)}, shared-rate {format_pp(base14.mean_shared_prime_rate_delta_pp_given_active)}.",
        f"Base 34 stays boundary-led at M=2: stable-zero {format_pp(base34.mean_stable_zero_prime_delta_pp_given_active)}, boundary {format_pp(base34.mean_boundary_prime_delta_pp_given_active)}, shared-rate {format_pp(base34.mean_shared_prime_rate_delta_pp_given_active)}.",
    ]


def bubble_metric_rows(rows: list[BaseMRow]) -> list[dict[str, float | str | int]]:
    grouped = grouped_lookup(rows)
    metric_rows: list[dict[str, float | str | int]] = []
    for base in BASES:
        m1 = grouped[(base, 1)]
        m2 = grouped[(base, 2)]
        phi_sq = float(euler_phi(base) ** 2)
        retention = (
            m2.active_pair_count / m1.active_pair_count if m1.active_pair_count > 0 else 0.0
        )
        mass_retention = m2.anomaly_mass_pp / m1.anomaly_mass_pp if m1.anomaly_mass_pp > 0 else 0.0
        normalized_share = m2.active_pair_count / phi_sq if phi_sq > 0 else 0.0
        metric_rows.append(
            {
                "base": base,
                "source": m2.dominant_signal_source_label,
                "x": m2.mean_stable_zero_prime_delta_pp_given_active or 0.0,
                "y": m2.mean_shared_prime_rate_delta_pp_given_active or 0.0,
                "retention": retention,
                "mass_retention": mass_retention,
                "normalized_share": normalized_share,
                "active_pair_share": m2.active_pair_share,
            }
        )
    return metric_rows


def bubble_size(value: float, *, floor: float = 90.0, scale: float = 1200.0) -> float:
    return floor + scale * max(value, 0.0)


def save_m2_bubble_metric_comparison(rows: list[BaseMRow], out_path: Path) -> None:
    metric_rows = bubble_metric_rows(rows)
    fig, axes = plt.subplots(1, 3, figsize=(18.5, 6.2), constrained_layout=True)
    panel_specs = [
        ("retention", "retention from M=1→M=2", "active pairs at M2 / active pairs at M1"),
        ("mass_retention", "anomaly-mass retention", "anomaly mass at M2 / anomaly mass at M1"),
        ("normalized_share", "M=2 active share / phi(base)^2", "active pairs at M2 normalized by unit-pair space"),
    ]

    for ax, (metric_key, title, subtitle) in zip(axes, panel_specs):
        add_panel_header(ax, title, subtitle)
        for row in metric_rows:
            ax.scatter(
                row["x"],
                row["y"],
                s=bubble_size(float(row[metric_key])),
                color=SOURCE_COLORS.get(str(row["source"]), SOURCE_COLORS["mixed_or_flat"]),
                alpha=0.92,
                edgecolor="#2f2a26",
                linewidth=0.75,
            )
            ax.annotate(
                f"{row['base']}",
                (float(row["x"]), float(row["y"])),
                xytext=(6, 5),
                textcoords="offset points",
                fontsize=10,
                fontweight="semibold",
            )
        ax.axhline(0, color="#8B8379", lw=1.1)
        ax.axvline(0, color="#8B8379", lw=1.1)
        ax.set_xlabel("mean stable-zero delta (pp)")
        ax.set_ylabel("mean shared prime-rate delta (pp)")

    axes[0].legend(handles=source_handles(), loc="upper left", title="signal source", fontsize=10, title_fontsize=10)
    fig.suptitle(
        "M=2 bubble-size comparison\nSame geometry, different size encodings",
        fontsize=19,
        fontweight="bold",
        y=1.03,
    )
    fig.savefig(out_path, dpi=200, bbox_inches="tight")
    plt.close(fig)


def save_m2_retention_labeled_plane(rows: list[BaseMRow], out_path: Path) -> None:
    metric_rows = bubble_metric_rows(rows)
    fig, ax = plt.subplots(figsize=(9.8, 7.4), constrained_layout=True)
    add_panel_header(
        ax,
        "M=2 retention-labeled plane",
        "Bubble size and text both encode active-pair retention from M=1→M=2",
    )
    for row in metric_rows:
        retention = float(row["retention"])
        x = float(row["x"])
        y = float(row["y"])
        ax.scatter(
            x,
            y,
            s=bubble_size(retention, floor=110.0, scale=1500.0),
            color=SOURCE_COLORS.get(str(row["source"]), SOURCE_COLORS["mixed_or_flat"]),
            alpha=0.92,
            edgecolor="#2f2a26",
            linewidth=0.8,
        )
        ax.annotate(
            f"{row['base']}  r={retention:.3f}",
            (x, y),
            xytext=(7, 6),
            textcoords="offset points",
            fontsize=10,
            fontweight="semibold",
        )
    ax.axhline(0, color="#8B8379", lw=1.1)
    ax.axvline(0, color="#8B8379", lw=1.1)
    ax.set_xlabel("mean stable-zero delta (pp)")
    ax.set_ylabel("mean shared prime-rate delta (pp)")
    ax.legend(handles=source_handles(), loc="upper left", title="signal source", fontsize=10, title_fontsize=10)
    ax.text(
        0.98,
        0.03,
        "Large bubbles = more anomaly species survive the M=1→M=2 transition",
        transform=ax.transAxes,
        ha="right",
        va="bottom",
        fontsize=10,
        bbox={"boxstyle": "round,pad=0.35", "facecolor": "#FFF9EC", "edgecolor": "#D8C89F"},
    )
    fig.savefig(out_path, dpi=200, bbox_inches="tight")
    plt.close(fig)


def digit_symbol(value: int) -> str:
    if value < 10:
        return str(value)
    return chr(ord("A") + value - 10)


def residue_positions(base: int) -> dict[int, tuple[float, float]]:
    units = unit_residues(base)
    positions: dict[int, tuple[float, float]] = {}
    total = len(units)
    for index, unit in enumerate(units):
        angle = pi / 2 - (2 * pi * index / total)
        positions[unit] = (cos(angle), sin(angle))
    return positions


def draw_self_loop(ax: plt.Axes, x: float, y: float, color: str, linewidth: float) -> None:
    radius = 0.18
    arc = Arc((x, y + 0.12), radius, radius * 0.78, angle=0, theta1=35, theta2=325, color=color, lw=linewidth, alpha=0.9)
    ax.add_patch(arc)


def draw_pair_chord(
    ax: plt.Axes,
    start: tuple[float, float],
    end: tuple[float, float],
    *,
    color: str,
    linewidth: float,
    curvature: float,
    linestyle: str | tuple[int, tuple[int, ...]] = "-",
) -> None:
    patch = FancyArrowPatch(
        start,
        end,
        arrowstyle="-|>",
        mutation_scale=10 + linewidth * 1.5,
        linewidth=linewidth,
        color=color,
        connectionstyle=f"arc3,rad={curvature}",
        alpha=0.88,
        shrinkA=10,
        shrinkB=10,
        linestyle=linestyle,
    )
    ax.add_patch(patch)


def pair_retention_lookup(pair_rows: list[PairThresholdRow]) -> dict[tuple[int, int, int], bool]:
    return {
        (row.base, row.outer, row.inner): row.active
        for row in pair_rows
        if row.middle_length == 1
    }


def active_pair_rows_for_bases(pair_rows: list[PairThresholdRow], bases: tuple[int, ...]) -> list[PairThresholdRow]:
    return [
        row for row in pair_rows if row.middle_length == 2 and row.base in bases and row.active
    ]


def save_unit_circle_chord_report(pair_rows: list[PairThresholdRow], out_path: Path) -> None:
    active_rows = active_pair_rows_for_bases(pair_rows, (14, 34))
    active_rows.sort(key=lambda row: (row.base, row.anomaly_mass_pp))
    m1_lookup = pair_retention_lookup(pair_rows)
    fig, axes = plt.subplots(1, 2, figsize=(15.8, 7.4), constrained_layout=True)
    fig.suptitle(
        "M=2 unit-circle chord view\nUnit residues on the circle; active pairs drawn as directed chords",
        fontsize=20,
        fontweight="bold",
        y=1.03,
    )

    for ax, base in zip(axes, (14, 34)):
        base_rows = [row for row in active_rows if row.base == base]
        positions = residue_positions(base)
        units = unit_residues(base)
        max_anomaly = max((row.anomaly_mass_pp for row in base_rows), default=1.0)

        ax.set_aspect("equal")
        ax.set_xlim(-1.35, 1.55)
        ax.set_ylim(-1.35, 1.35)
        ax.axis("off")
        add_panel_header(
            ax,
            f"Base {base} active M=2 chord map",
            "Chord width ~ anomaly mass; color = signal source; arrow = ordered pair",
        )
        circle = Circle((0, 0), 1.0, fill=False, edgecolor="#8B8379", linewidth=1.2)
        ax.add_patch(circle)

        for unit in units:
            x, y = positions[unit]
            ax.scatter(x, y, s=70, color="#F4EFE4", edgecolor="#3C362F", linewidth=0.9, zorder=3)
            ax.text(
                x * 1.13,
                y * 1.13,
                digit_symbol(unit),
                ha="center",
                va="center",
                fontsize=11,
                fontweight="semibold",
            )

        for row in base_rows:
            start = positions[row.outer]
            end = positions[row.inner]
            color = SOURCE_COLORS.get(row.signal_source_label, SOURCE_COLORS["mixed_or_flat"])
            linewidth = 1.8 + 5.8 * (row.anomaly_mass_pp / max_anomaly if max_anomaly > 0 else 0.0)
            retained = m1_lookup.get((row.base, row.outer, row.inner), False)
            linestyle = "-" if retained else (0, (4, 2))
            if row.outer == row.inner:
                draw_self_loop(ax, start[0], start[1], color, linewidth)
            else:
                outer_index = units.index(row.outer)
                inner_index = units.index(row.inner)
                curvature = 0.18 if outer_index < inner_index else -0.18
                draw_pair_chord(
                    ax,
                    start,
                    end,
                    color=color,
                    linewidth=linewidth,
                    curvature=curvature,
                    linestyle=linestyle,
                )

        summary_lines = []
        for row in sorted(base_rows, key=lambda row: row.anomaly_mass_pp, reverse=True):
            retained = m1_lookup.get((row.base, row.outer, row.inner), False)
            summary_lines.append(
                f"{row.pair_label}  {row.anomaly_mass_pp:.2f}pp  "
                f"{'retained' if retained else 'emergent'}  {row.signal_source_label.replace('_', '-')}"
            )
        ax.text(
            1.08,
            0.96,
            "\n".join(summary_lines),
            transform=ax.transAxes,
            ha="left",
            va="top",
            fontsize=10,
            family="DejaVu Sans Mono",
            bbox={"boxstyle": "round,pad=0.4", "facecolor": "#FFF9EC", "edgecolor": "#D8C89F"},
        )

    axes[0].legend(
        handles=source_handles_with_lines() + survival_style_handles(),
        loc="lower left",
        bbox_to_anchor=(-0.02, -0.05),
        title="source / survival",
        fontsize=10,
        title_fontsize=10,
    )
    fig.savefig(out_path, dpi=220, bbox_inches="tight")
    plt.close(fig)


def save_unit_circle_chord_atlas(pair_rows: list[PairThresholdRow], out_path: Path) -> None:
    atlas_bases = (10, 12, 14, 22, 26, 34)
    active_rows = active_pair_rows_for_bases(pair_rows, atlas_bases)
    m1_lookup = pair_retention_lookup(pair_rows)

    fig, axes = plt.subplots(2, 3, figsize=(16.2, 10.8), constrained_layout=True)
    fig.suptitle(
        "M=2 unit-circle chord atlas\nSix-base residue geometry with retained (solid) vs emergent (dashed) actives",
        fontsize=20,
        fontweight="bold",
        y=1.02,
    )

    for ax, base in zip(axes.ravel(), atlas_bases):
        base_rows = [row for row in active_rows if row.base == base]
        positions = residue_positions(base)
        units = unit_residues(base)
        max_anomaly = max((row.anomaly_mass_pp for row in base_rows), default=1.0)

        ax.set_aspect("equal")
        ax.set_xlim(-1.28, 1.28)
        ax.set_ylim(-1.28, 1.28)
        ax.axis("off")
        add_panel_header(ax, f"Base {base}", f"{len(base_rows)} active M=2 pairs")
        ax.add_patch(Circle((0, 0), 1.0, fill=False, edgecolor="#8B8379", linewidth=1.1))

        for unit in units:
            x, y = positions[unit]
            ax.scatter(x, y, s=52, color="#F4EFE4", edgecolor="#3C362F", linewidth=0.8, zorder=3)
            ax.text(x * 1.11, y * 1.11, digit_symbol(unit), ha="center", va="center", fontsize=9, fontweight="semibold")

        for row in sorted(base_rows, key=lambda row: row.anomaly_mass_pp):
            start = positions[row.outer]
            end = positions[row.inner]
            retained = m1_lookup.get((row.base, row.outer, row.inner), False)
            color = SOURCE_COLORS.get(row.signal_source_label, SOURCE_COLORS["mixed_or_flat"])
            linewidth = 1.6 + 4.6 * (row.anomaly_mass_pp / max_anomaly if max_anomaly > 0 else 0.0)
            linestyle = "-" if retained else (0, (4, 2))
            if row.outer == row.inner:
                draw_self_loop(ax, start[0], start[1], color, linewidth)
            else:
                outer_index = units.index(row.outer)
                inner_index = units.index(row.inner)
                curvature = 0.16 if outer_index < inner_index else -0.16
                draw_pair_chord(ax, start, end, color=color, linewidth=linewidth, curvature=curvature, linestyle=linestyle)

    axes[0, 0].legend(
        handles=source_handles_with_lines() + survival_style_handles(),
        loc="upper left",
        bbox_to_anchor=(-0.02, 1.06),
        ncol=2,
        title="source / survival",
        fontsize=9,
        title_fontsize=9,
    )
    fig.savefig(out_path, dpi=220, bbox_inches="tight")
    plt.close(fig)


def ordered_complex_embedding(base: int, outer: int, inner: int, weight: float = 0.62) -> tuple[float, float]:
    units = unit_residues(base)
    outer_index = units.index(outer)
    inner_index = units.index(inner)
    outer_angle = pi / 2 - (2 * pi * outer_index / len(units))
    inner_angle = pi / 2 - (2 * pi * inner_index / len(units))
    x = cos(outer_angle) + weight * cos(inner_angle)
    y = sin(outer_angle) + weight * sin(inner_angle)
    return x, y


def normalized_cyclic_gap(base: int, outer: int, inner: int) -> float:
    units = unit_residues(base)
    outer_index = units.index(outer)
    inner_index = units.index(inner)
    raw = abs(outer_index - inner_index)
    cyclic = min(raw, len(units) - raw)
    return 0.0 if len(units) <= 1 else cyclic / (len(units) / 2)


def m2_geometry_rows(pair_rows: list[PairThresholdRow]) -> list[dict[str, float | str | int]]:
    rows = []
    m1_lookup = pair_retention_lookup(pair_rows)
    for base in (10, 12, 14, 22, 26, 34):
        active_rows = [
            row for row in pair_rows if row.base == base and row.middle_length == 2 and row.active
        ]
        if not active_rows:
            continue
        anomaly_total = sum(row.anomaly_mass_pp for row in active_rows)
        weighted_gap = sum(
            normalized_cyclic_gap(base, row.outer, row.inner) * row.anomaly_mass_pp
            for row in active_rows
        ) / anomaly_total
        overlap_share = sum(
            1 for row in active_rows if row.signal_source_label == "stable_zero_led"
        ) / len(active_rows)
        retained_share = sum(
            1 for row in active_rows if m1_lookup.get((row.base, row.outer, row.inner), False)
        ) / len(active_rows)
        self_loop_share = sum(1 for row in active_rows if row.outer == row.inner) / len(active_rows)
        wide_gap_share = sum(
            1
            for row in active_rows
            if normalized_cyclic_gap(base, row.outer, row.inner) >= (2.0 / 3.0)
        ) / len(active_rows)
        dominant_source = max(
            ("stable_zero_led", "boundary_led", "mixed_or_flat"),
            key=lambda label: sum(1 for row in active_rows if row.signal_source_label == label),
        )
        rows.append(
            {
                "base": base,
                "active_pair_count": len(active_rows),
                "weighted_gap": weighted_gap,
                "overlap_share": overlap_share,
                "retained_share_within_actives": retained_share,
                "self_loop_share": self_loop_share,
                "wide_gap_share": wide_gap_share,
                "dominant_source": dominant_source,
            }
        )
    return rows


def save_m2_residue_geometry_plane(pair_rows: list[PairThresholdRow], out_path: Path) -> None:
    rows = m2_geometry_rows(pair_rows)
    fig, ax = plt.subplots(figsize=(9.8, 7.2), constrained_layout=True)
    add_panel_header(
        ax,
        "M=2 residue-geometry plane",
        "x = anomaly-weighted cyclic gap, y = overlap-led share, bubble = retained share",
    )
    for row in rows:
        x = float(row["weighted_gap"])
        y = float(row["overlap_share"])
        size = bubble_size(float(row["retained_share_within_actives"]), floor=110.0, scale=1600.0)
        ax.scatter(
            x,
            y,
            s=size,
            color=SOURCE_COLORS.get(str(row["dominant_source"]), SOURCE_COLORS["mixed_or_flat"]),
            alpha=0.92,
            edgecolor="#2f2a26",
            linewidth=0.8,
        )
        ax.annotate(
            f"{row['base']}",
            (x, y),
            xytext=(6, 6),
            textcoords="offset points",
            fontsize=10,
            fontweight="semibold",
        )
    ax.set_xlabel("anomaly-weighted normalized cyclic gap")
    ax.set_ylabel("overlap-led share among active M=2 pairs")
    ax.set_xlim(-0.02, 1.02)
    ax.set_ylim(-0.02, 1.02)
    ax.legend(handles=source_handles(), loc="upper right", title="dominant source", fontsize=10, title_fontsize=10)
    ax.text(
        0.98,
        0.03,
        "Tight geometry alone is not enough.\nBase 22 is tight but not overlap-led.\nBase 14 combines tighter geometry with high overlap share.",
        transform=ax.transAxes,
        ha="right",
        va="bottom",
        fontsize=10,
        bbox={"boxstyle": "round,pad=0.35", "facecolor": "#FFF9EC", "edgecolor": "#D8C89F"},
    )
    fig.savefig(out_path, dpi=220, bbox_inches="tight")
    plt.close(fig)


def save_ordered_complex_embedding(pair_rows: list[PairThresholdRow], out_path: Path) -> None:
    embedding_bases = (10, 12, 14, 22, 26, 34)
    active_rows = active_pair_rows_for_bases(pair_rows, embedding_bases)
    m1_lookup = pair_retention_lookup(pair_rows)

    fig, ax = plt.subplots(figsize=(10.6, 8.2), constrained_layout=True)
    add_panel_header(
        ax,
        "Ordered residue complex embedding",
        "Point = exp(iθ_outer) + 0.62 exp(iθ_inner); marker shows retained vs emergent",
    )
    for row in active_rows:
        x, y = ordered_complex_embedding(row.base, row.outer, row.inner)
        retained = m1_lookup.get((row.base, row.outer, row.inner), False)
        marker = "o" if retained else "s"
        ax.scatter(
            x,
            y,
            s=90 + 220 * row.anomaly_mass_pp / max(0.01, max(r.anomaly_mass_pp for r in active_rows)),
            color=SOURCE_COLORS.get(row.signal_source_label, SOURCE_COLORS["mixed_or_flat"]),
            alpha=0.9,
            edgecolor="#2f2a26",
            linewidth=0.8,
            marker=marker,
        )
        ax.annotate(
            f"{row.base}:{row.pair_label}",
            (x, y),
            xytext=(6, 4),
            textcoords="offset points",
            fontsize=9,
        )
    ax.axhline(0, color="#8B8379", lw=1.0)
    ax.axvline(0, color="#8B8379", lw=1.0)
    ax.set_xlabel("Re(z)")
    ax.set_ylabel("Im(z)")
    marker_handles = [
        Line2D([0], [0], marker="o", color="none", markerfacecolor="#999999", markeredgecolor="#333333", markersize=8, label="retained"),
        Line2D([0], [0], marker="s", color="none", markerfacecolor="#999999", markeredgecolor="#333333", markersize=8, label="emergent"),
    ]
    ax.legend(
        handles=source_handles() + marker_handles,
        loc="upper right",
        title="source / survival",
        fontsize=10,
        title_fontsize=10,
    )
    ax.text(
        0.98,
        0.03,
        "Chosen embedding, not theorem language.\nUseful for visual clustering of ordered residue pairs.",
        transform=ax.transAxes,
        ha="right",
        va="bottom",
        fontsize=10,
        bbox={"boxstyle": "round,pad=0.35", "facecolor": "#FFF9EC", "edgecolor": "#D8C89F"},
    )
    fig.savefig(out_path, dpi=220, bbox_inches="tight")
    plt.close(fig)


def write_markdown_report(
    rows: list[BaseMRow],
    pair_rows: list[PairThresholdRow],
    out_path: Path,
    image_dir: Path,
    input_dir: Path,
) -> None:
    grouped = grouped_lookup(rows)
    metric_rows = bubble_metric_rows(rows)
    geometry_rows = m2_geometry_rows(pair_rows)
    lines = [
        "# Chaos-Threshold Visual Report",
        "",
        f"_Generated from `scripts/plot_chaos_threshold_translation.py` against the maintained arithmetic artifact in `{input_dir.as_posix()}`._",
        "",
        "This visual report is downstream of the arithmetic transition lane. It is a presentation surface, not a new source of truth.",
        "",
        "## Key Takeaways",
        "",
    ]
    for takeaway in strongest_takeaways(rows):
        lines.append(f"- {takeaway}")
    lines.extend(
        [
            "",
            "## Figures",
            "",
            f"![Storyboard]({(image_dir / 'chaos_threshold_storyboard.png').as_posix()})",
            "",
            f"![Regime heatmap]({(image_dir / 'chaos_threshold_regime_heatmap.png').as_posix()})",
            "",
            f"![M2 decomposition plane]({(image_dir / 'chaos_threshold_m2_decomposition_plane.png').as_posix()})",
            "",
            f"![M2 bubble metric comparison]({(image_dir / 'chaos_threshold_m2_bubble_metric_comparison.png').as_posix()})",
            "",
            f"![M2 retention labeled plane]({(image_dir / 'chaos_threshold_m2_retention_labeled_plane.png').as_posix()})",
            "",
            f"![Compactness glossary]({(image_dir / 'chaos_threshold_compactness_glossary.png').as_posix()})",
            "",
            f"![Unit-circle chord view]({(image_dir / 'chaos_threshold_m2_unit_circle_chords_14_34.png').as_posix()})",
            "",
            f"![Unit-circle chord atlas]({(image_dir / 'chaos_threshold_m2_unit_circle_chord_atlas.png').as_posix()})",
            "",
            f"![Ordered complex embedding]({(image_dir / 'chaos_threshold_ordered_complex_embedding.png').as_posix()})",
            "",
            f"![M2 residue geometry plane]({(image_dir / 'chaos_threshold_m2_residue_geometry_plane.png').as_posix()})",
            "",
            "## M=2 Snapshot",
            "",
            "| Base | Regime | Source | Mean stable-zero delta | Mean boundary delta | Mean shared-rate delta |",
            "|---:|---|---|---:|---:|---:|",
        ]
    )
    for base in BASES:
        row = grouped[(base, 2)]
        lines.append(
            f"| {base} | `{row.regime_label}` | `{row.dominant_signal_source_label}` | {format_pp(row.mean_stable_zero_prime_delta_pp_given_active)} | {format_pp(row.mean_boundary_prime_delta_pp_given_active)} | {format_pp(row.mean_shared_prime_rate_delta_pp_given_active)} |"
        )
    lines.extend(
        [
            "",
            "## Bubble Metrics",
            "",
            "This comparison keeps the `M=2` geometry fixed and changes only the bubble-size rule.",
            "",
            "| Base | Retention M1→M2 | Mass retention | M2 active / phi(base)^2 |",
            "|---:|---:|---:|---:|",
        ]
    )
    for row in metric_rows:
        lines.append(
            f"| {row['base']} | {float(row['retention']):.3f} | {float(row['mass_retention']):.3f} | {float(row['normalized_share']):.3f} |"
        )
    if geometry_rows:
        lines.extend(
            [
                "",
                "## Residue Geometry",
                "",
                "| Base | Weighted cyclic gap | Overlap-led share | Retained share within actives | Self-loop share | Wide-gap share |",
                "|---:|---:|---:|---:|---:|---:|",
            ]
        )
        for row in geometry_rows:
            lines.append(
                f"| {row['base']} | {float(row['weighted_gap']):.3f} | {float(row['overlap_share']):.3f} | {float(row['retained_share_within_actives']):.3f} | {float(row['self_loop_share']):.3f} | {float(row['wide_gap_share']):.3f} |"
            )
    out_path.write_text("\n".join(lines) + "\n")


def write_artifact_manifest(input_dir: Path, out_dir: Path) -> None:
    manifest = {
        "artifact_id": ARTIFACT_ID,
        "generator_cmd": "python3",
        "args": [
            "scripts/plot_chaos_threshold_translation.py",
            "--input-dir",
            input_dir.as_posix(),
            "--out-dir",
            out_dir.as_posix(),
        ],
        "upstream_inputs": [
            (input_dir / "base_m_rows.csv").as_posix(),
            (input_dir / "pair_threshold_rows.csv").as_posix(),
            (input_dir / "summary.json").as_posix(),
        ],
        "expected_outputs": [
            "chaos_threshold_storyboard.png",
            "chaos_threshold_regime_heatmap.png",
            "chaos_threshold_compactness_glossary.png",
            "chaos_threshold_m2_decomposition_plane.png",
            "chaos_threshold_m2_bubble_metric_comparison.png",
            "chaos_threshold_m2_retention_labeled_plane.png",
            "chaos_threshold_m2_unit_circle_chords_14_34.png",
            "chaos_threshold_m2_unit_circle_chord_atlas.png",
            "chaos_threshold_ordered_complex_embedding.png",
            "chaos_threshold_m2_residue_geometry_plane.png",
            "report.md",
        ],
    }
    (out_dir / "artifact_manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")


def main() -> None:
    args = parse_args()
    configure_matplotlib()
    ensure_dir(args.out_dir)
    rows = load_base_rows(args.input_dir / "base_m_rows.csv")
    pair_rows = load_pair_rows(args.input_dir / "pair_threshold_rows.csv")

    storyboard = args.out_dir / "chaos_threshold_storyboard.png"
    heatmap = args.out_dir / "chaos_threshold_regime_heatmap.png"
    plane = args.out_dir / "chaos_threshold_m2_decomposition_plane.png"
    bubble_comparison = args.out_dir / "chaos_threshold_m2_bubble_metric_comparison.png"
    retention_plane = args.out_dir / "chaos_threshold_m2_retention_labeled_plane.png"
    compactness_glossary = args.out_dir / "chaos_threshold_compactness_glossary.png"
    chord_view = args.out_dir / "chaos_threshold_m2_unit_circle_chords_14_34.png"
    chord_atlas = args.out_dir / "chaos_threshold_m2_unit_circle_chord_atlas.png"
    complex_embedding = args.out_dir / "chaos_threshold_ordered_complex_embedding.png"
    geometry_plane = args.out_dir / "chaos_threshold_m2_residue_geometry_plane.png"
    save_storyboard(rows, storyboard)
    save_regime_heatmap(rows, heatmap)
    save_compactness_glossary(rows, compactness_glossary)
    save_m2_decomposition_plane(rows, plane)
    save_m2_bubble_metric_comparison(rows, bubble_comparison)
    save_m2_retention_labeled_plane(rows, retention_plane)
    save_unit_circle_chord_report(pair_rows, chord_view)
    save_unit_circle_chord_atlas(pair_rows, chord_atlas)
    save_ordered_complex_embedding(pair_rows, complex_embedding)
    save_m2_residue_geometry_plane(pair_rows, geometry_plane)
    write_markdown_report(rows, pair_rows, args.out_dir / "report.md", args.out_dir, args.input_dir)
    write_artifact_manifest(args.input_dir, args.out_dir)

    print("Polished chaos-threshold visual report")
    print(f"  Input:  {args.input_dir}")
    print(f"  Output: {args.out_dir}")
    print("  Files:")
    print("    - chaos_threshold_storyboard.png")
    print("    - chaos_threshold_regime_heatmap.png")
    print("    - chaos_threshold_compactness_glossary.png")
    print("    - chaos_threshold_m2_decomposition_plane.png")
    print("    - chaos_threshold_m2_bubble_metric_comparison.png")
    print("    - chaos_threshold_m2_retention_labeled_plane.png")
    print("    - chaos_threshold_m2_unit_circle_chords_14_34.png")
    print("    - chaos_threshold_m2_unit_circle_chord_atlas.png")
    print("    - chaos_threshold_ordered_complex_embedding.png")
    print("    - chaos_threshold_m2_residue_geometry_plane.png")
    print("    - report.md")
    print("    - artifact_manifest.json")


if __name__ == "__main__":
    main()
