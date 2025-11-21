from __future__ import annotations

import json
from pathlib import Path
from typing import Any, Dict, List, Optional

from manim import *


# ---------------------------------------------------------------------------
# Palette & layout helpers
# ---------------------------------------------------------------------------

class ElbowColors:
    BACKGROUND = "#050509"

    TEXT_PRIMARY = WHITE
    TEXT_SECONDARY = GREY_B

    HONORARY_ZERO = YELLOW_E

    K_BEFORE = BLUE_C
    K_AFTER = GREEN_C

    DENSITY_LOW = GREY_D
    DENSITY_MID = GREY_A
    DENSITY_PEAK = "#ffe66d"

    PANEL_BG = BLACK


# ---------------------------------------------------------------------------
# Data loader for elbow_events.json
# ---------------------------------------------------------------------------

class ElbowData:
    """
    Loads elbow_events.json produced by extract_elbow_events.rs.

    Expected structure (approx):

    {
      "events": [
        {
          "base": 15,
          "outer": 13,
          "inner": 1,
          "m_before": 1,
          "m_after": 2,
          "k_star_before": 0,
          "k_star_after": 1,
          "density_jump": 0.0429,
          "rows_before": [
            { "k": 0, "density": 0.071429, ... },
            ...
          ],
          "rows_after": [
            { "k": 0, "density": 0.085714, ... },
            ...
          ]
        },
        ...
      ],
      "summary": {
        "total_events": 2,
        "total_boundaries": 5616,
        "fraction": 0.000356
      }
    }
    """

    def __init__(self, path: str = "elbow_events.json") -> None:
        self.path = Path(path)
        self.events: List[Dict[str, Any]] = []
        self.summary: Dict[str, Any] = {}
        self._load()

    def _load(self) -> None:
        if not self.path.exists():
            self.events = []
            self.summary = {}
            return

        data = json.loads(self.path.read_text())
        self.events = data.get("events", []) or []
        self.summary = data.get("summary", {}) or {}


# ---------------------------------------------------------------------------
# Shared render helpers (membrane, single-event view, etc.)
# ---------------------------------------------------------------------------

class ElbowEventRenderer:
    """
    Stateless helper: invoked by multiple Scene subclasses to render
    membranes + charts with clean layout (no overlaps).
    """

    # ---------------- Membrane + HZ axis ----------------

    def make_membrane_row(
        self,
        outer: int,
        inner: int,
        M: int,
        k: int,
        label: str,
        seed_color=ElbowColors.K_BEFORE,
    ) -> VGroup:
        """
        Symbolic membrane row with Honorary Zero axis at the SEED center:

           [outer] [0×k] [inner] [0×k] [SEED (width∝M)] [0×k] [inner] [0×k] [outer]
        """
        boxes = VGroup()

        def box(text: str, color=ElbowColors.TEXT_SECONDARY) -> VGroup:
            rect = RoundedRectangle(
                corner_radius=0.08,
                height=0.4,
                width=0.4,
            )
            rect.set_stroke(color=color, width=1.5)
            txt = Text(text, font_size=18, color=color)
            txt.move_to(rect.get_center())
            return VGroup(rect, txt)

        # outer
        boxes.add(box(str(outer), ElbowColors.TEXT_PRIMARY))

        # k zeros
        for _ in range(k):
            boxes.add(box("0", GREY_B))

        # inner
        boxes.add(box(str(inner), ElbowColors.TEXT_PRIMARY))

        # k zeros
        for _ in range(k):
            boxes.add(box("0", GREY_B))

        # seed block
        seed_width = max(0.6, 0.35 * M)
        seed_rect = RoundedRectangle(
            corner_radius=0.08,
            height=0.4,
            width=seed_width,
        )
        seed_rect.set_stroke(color=seed_color, width=2.0)
        seed_txt = Text("SEED", font_size=18, color=seed_color)
        seed_txt.move_to(seed_rect.get_center())
        seed_group = VGroup(seed_rect, seed_txt)
        boxes.add(seed_group)

        # k zeros
        for _ in range(k):
            boxes.add(box("0", GREY_B))

        # inner
        boxes.add(box(str(inner), ElbowColors.TEXT_PRIMARY))

        # k zeros
        for _ in range(k):
            boxes.add(box("0", GREY_B))

        # outer
        boxes.add(box(str(outer), ElbowColors.TEXT_PRIMARY))

        boxes.arrange(RIGHT, buff=0.05)

        # Honorary zero axis through SEED center
        cx = boxes.get_center()[0]
        top_y = boxes.get_top()[1] + 0.15
        bot_y = boxes.get_bottom()[1] - 0.15

        hz_line = Line(
            start=[cx, bot_y, 0],
            end=[cx, top_y, 0],
            color=ElbowColors.HONORARY_ZERO,
            stroke_width=3,
        )

        lbl = Text(label, font_size=18, color=seed_color)
        lbl.next_to(boxes, DOWN, buff=0.15)

        return VGroup(boxes, hz_line, lbl)

    # ---------------- Canonical prime side panel ----------------

    def maybe_prime_side_panel(
        self,
        base: int,
        outer: int,
        inner: int,
        M: int,
        k: int,
    ) -> Optional[VGroup]:
        """
        For the canonical base-15, outer=13, inner=1, M=2, k=1 elbow event,
        show explicit prime examples on that ridge.
        """
        if not (base == 15 and outer == 13 and inner == 1 and M == 2 and k == 1):
            return None

        title = Text(
            "Sample primes on this M=2, k=1 ridge",
            font_size=24,
            color=ElbowColors.TEXT_PRIMARY,
        )

        line1 = Text(
            "13 0 1 0 8 1 0 1 0 13₁₅  =  499 935 695 863",
            font_size=18,
            font="monospace",
            color=ElbowColors.TEXT_SECONDARY,
        )
        line2 = Text(
            "13 0 1 0 14 1 0 1 0 13₁₅ =  499 935 999 613",
            font_size=18,
            font="monospace",
            color=ElbowColors.TEXT_SECONDARY,
        )

        panel = VGroup(title, line1, line2).arrange(
            DOWN, aligned_edge=LEFT, buff=0.2
        )
        panel.add_background_rectangle(
            color=ElbowColors.PANEL_BG,
            opacity=0.8,
            buff=0.25,
        )
        return panel

    # ---------------- Single-event renderer ----------------

    def show_single_event(
        self,
        scene: Scene,
        event: Dict[str, Any],
        idx: int,
        total: int,
    ) -> None:
        """
        Render a single elbow event with:

          - top: header
          - middle: membrane row (M,k) + bar chart transition with density labels
          - bottom: short annotation
          - optional side panel for canonical base-15 event

        Timing optimized for ~5s per event.
        """
        scene.camera.background_color = ElbowColors.BACKGROUND
        frame_w = config.frame_width
        frame_h = config.frame_height

        base = event["base"]
        outer = event["outer"]
        inner = event["inner"]
        m_before = event["m_before"]
        m_after = event["m_after"]
        k_before = event["k_star_before"]
        k_after = event["k_star_after"]
        density_jump = event.get("density_jump", 0.0)

        rows_before = event.get("rows_before", [])
        rows_after = event.get("rows_after", [])
        if not rows_before or not rows_after:
            # Fallback: just say this event has no density arrays
            msg = Text(
                f"Event {idx}/{total}: missing density data.",
                font_size=32,
                color=RED,
            ).move_to(ORIGIN)
            scene.play(FadeIn(msg), run_time=0.5)
            scene.wait(0.5)
            scene.play(FadeOut(msg), run_time=0.3)
            return

        k_values = [r["k"] for r in rows_before]
        densities_before = [r["density"] for r in rows_before]
        densities_after = [r["density"] for r in rows_after]

        # ---------------- Header (top zone) ----------------

        header = VGroup(
            Text(
                f"Event {idx}/{total}: base {base}  (outer={outer}, inner={inner})",
                font_size=32,
                color=ElbowColors.TEXT_PRIMARY,
            ),
            Text(
                f"M: {m_before} → {m_after}   |   k*: {k_before} → {k_after}   |   Δρ = {density_jump:+.4f}",
                font_size=22,
                font="monospace",
                color=ElbowColors.TEXT_SECONDARY,
            ),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.15)

        header.to_edge(UP, buff=0.5)
        header.scale_to_fit_width(frame_w * 0.9)

        scene.play(FadeIn(header), run_time=0.5)

        # ---------------- Membrane (upper middle) ----------------

        mem_before = self.make_membrane_row(
            outer=outer,
            inner=inner,
            M=m_before,
            k=k_before,
            label=f"M = {m_before}, k = {k_before}",
            seed_color=ElbowColors.K_BEFORE,
        )
        mem_after = self.make_membrane_row(
            outer=outer,
            inner=inner,
            M=m_after,
            k=k_after,
            label=f"M = {m_after}, k = {k_after}",
            seed_color=ElbowColors.K_AFTER,
        )

        mem_before.scale_to_fit_width(frame_w * 0.8)
        mem_before.next_to(header, DOWN, buff=0.5)

        mem_after.scale_to_fit_width(frame_w * 0.8)
        mem_after.move_to(mem_before.get_center())

        scene.play(FadeIn(mem_before), run_time=0.5)

        # ---------------- Bar chart (lower middle) ----------------

        max_density = max(max(densities_before), max(densities_after))
        if max_density <= 0:
            max_density = 0.1

        chart = BarChart(
            values=densities_before,
            y_range=[0, max_density * 1.2, max_density / 5],
            x_length=frame_w * 0.6,
            y_length=frame_h * 0.35,
            bar_width=0.5,
            bar_colors=[
                ElbowColors.K_BEFORE if k == k_before else ElbowColors.DENSITY_MID
                for k in k_values
            ],
        )

        chart.next_to(mem_before, DOWN, buff=0.5)
        chart.align_to(mem_before, LEFT)

        m_label = Text(
            f"M = {m_before}",
            font_size=24,
            color=ElbowColors.K_BEFORE,
        ).next_to(chart, DOWN, buff=0.3)

        # k labels under bars
        k_labels = VGroup()
        for i, k in enumerate(k_values):
            x = chart.x_axis.n2p(i)[0]
            lbl = Text(f"k = {k}", font_size=18, color=ElbowColors.TEXT_SECONDARY)
            lbl.move_to([x, chart.x_axis.get_y() - 0.35, 0])
            k_labels.add(lbl)

        # Density value labels above bars (BEFORE state)
        density_labels_before = VGroup()
        for i, density in enumerate(densities_before):
            lbl = Text(
                f"{density:.3f}",
                font_size=16,
                color=ElbowColors.TEXT_SECONDARY,
            )
            lbl.next_to(chart.bars[i], UP, buff=0.1)
            density_labels_before.add(lbl)

        scene.play(
            Create(chart),
            FadeIn(k_labels),
            Write(m_label),
            FadeIn(density_labels_before),
            run_time=0.7,
        )

        # Short annotation (bottom)
        plateau_line = Text(
            f"At M = {m_before}, k* = {k_before} is optimal.",
            font_size=20,
            color=ElbowColors.TEXT_SECONDARY,
        )
        plateau_line.scale_to_fit_width(frame_w * 0.9)
        plateau_line.to_edge(DOWN, buff=0.4)

        scene.play(FadeIn(plateau_line), run_time=0.5)
        scene.wait(0.3)

        # ---------------- Transition: M_before → M_after ----------------

        # Next colors and label
        new_bar_colors = [
            ElbowColors.K_AFTER if k == k_after else ElbowColors.DENSITY_MID
            for k in k_values
        ]
        m_label_after = Text(
            f"M = {m_after}",
            font_size=24,
            color=ElbowColors.K_AFTER,
        ).next_to(chart, DOWN, buff=0.3)

        # Density labels for AFTER state
        density_labels_after = VGroup()
        for i, density in enumerate(densities_after):
            k = k_values[i]
            color = ElbowColors.K_AFTER if k == k_after else ElbowColors.TEXT_SECONDARY
            lbl = Text(
                f"{density:.3f}",
                font_size=16,
                color=color,
            )
            lbl.next_to(chart.bars[i], UP, buff=0.1)
            density_labels_after.add(lbl)

        # Replace annotation text with elbow narrative
        elbow_line = Text(
            f"As M grows to {m_after}, k* shifts to {k_after} (Δρ = {density_jump:+.3f}).",
            font_size=20,
            color=ElbowColors.TEXT_SECONDARY,
        )
        elbow_line.scale_to_fit_width(frame_w * 0.9)
        elbow_line.to_edge(DOWN, buff=0.4)

        scene.play(FadeOut(plateau_line), run_time=0.3)

        scene.play(
            ReplacementTransform(mem_before, mem_after),
            chart.animate.change_bar_values(densities_after),
            Transform(m_label, m_label_after),
            ReplacementTransform(density_labels_before, density_labels_after),
            *[
                chart.bars[i].animate.set_fill(new_bar_colors[i])
                for i in range(len(k_values))
            ],
            run_time=2.0,
        )

        # Highlight k_after bar
        try:
            k_after_index = k_values.index(k_after)
            highlight = SurroundingRectangle(
                chart.bars[k_after_index],
                buff=0.05,
            ).set_stroke(ElbowColors.K_AFTER, width=4)
            scene.play(Create(highlight), FadeIn(elbow_line), run_time=0.7)
        except ValueError:
            highlight = None
            scene.play(FadeIn(elbow_line), run_time=0.5)

        scene.wait(0.4)

        # ---------------- Optional prime side panel ----------------

        prime_panel = self.maybe_prime_side_panel(
            base=base,
            outer=outer,
            inner=inner,
            M=m_after,
            k=k_after,
        )

        if prime_panel is not None:
            # Slide membrane+chart left a bit to make room
            shift_left = frame_w * 0.25
            scene.play(
                mem_after.animate.shift(LEFT * shift_left),
                chart.animate.shift(LEFT * shift_left),
                k_labels.animate.shift(LEFT * shift_left),
                m_label.animate.shift(LEFT * shift_left),
                density_labels_after.animate.shift(LEFT * shift_left),
                (highlight.animate.shift(LEFT * shift_left) if highlight else FadeOut(VGroup())),
                elbow_line.animate.shift(LEFT * shift_left),
                run_time=0.7,
            )

            prime_panel.scale_to_fit_height(frame_h * 0.7)
            prime_panel.to_edge(RIGHT, buff=0.5)
            prime_panel.align_to(mem_after, UP)

            scene.play(FadeIn(prime_panel), run_time=0.7)
            scene.wait(0.8)
        else:
            scene.wait(0.4)

        # ---------------- Clear for next event ----------------

        scene.play(
            FadeOut(header),
            FadeOut(mem_after),
            FadeOut(chart),
            FadeOut(k_labels),
            FadeOut(m_label),
            FadeOut(density_labels_after),
            FadeOut(elbow_line),
            FadeOut(highlight) if highlight else FadeOut(VGroup()),
            FadeOut(prime_panel) if prime_panel is not None else FadeOut(VGroup()),
            run_time=0.5,
        )


# ---------------------------------------------------------------------------
# Shared intro & stats helpers (so we can reuse in multiple scenes)
# ---------------------------------------------------------------------------

def play_intro(scene: Scene) -> None:
    """
    Honorary zero introduction with membrane template & axis.
    Layout: top title, middle template, bottom one-line explanation.
    Timing optimized to ~5s.
    """
    scene.camera.background_color = ElbowColors.BACKGROUND
    frame_w = config.frame_width

    # Title + subtitle (top)
    title = Text(
        "Honorary Zero: The Symmetry Axis",
        font_size=40,
        color=ElbowColors.TEXT_PRIMARY,
    ).to_edge(UP, buff=0.5)

    subtitle = Text(
        "Membrane primes around a midpoint in base representation",
        font_size=26,
        color=ElbowColors.TEXT_SECONDARY,
    ).next_to(title, DOWN, buff=0.25)

    title.scale_to_fit_width(frame_w * 0.9)
    subtitle.scale_to_fit_width(frame_w * 0.9)

    scene.play(Write(title), run_time=0.7)
    scene.play(FadeIn(subtitle), run_time=0.5)
    scene.wait(0.3)

    # Membrane structure text (upper middle)
    structure = VGroup(
        Text(
            "Membrane structure:",
            font_size=26,
            color=ElbowColors.TEXT_SECONDARY,
        ),
        Text(
            "outer  +  0…0  +  inner  +  0…0  +  SEED  +  0…0  +  inner  +  0…0  +  outer",
            font_size=22,
            font="monospace",
            color=ElbowColors.TEXT_PRIMARY,
        ),
    ).arrange(DOWN, aligned_edge=LEFT, buff=0.15)

    structure.next_to(subtitle, DOWN, buff=0.5)
    structure.scale_to_fit_width(frame_w * 0.92)

    scene.play(FadeIn(structure), run_time=0.7)
    scene.wait(0.3)

    # Membrane row with HZ (middle zone)
    renderer = ElbowEventRenderer()
    mem = renderer.make_membrane_row(
        outer=13,
        inner=1,
        M=1,
        k=1,
        label="Example: base 15, outer=13, inner=1, with one unit of elbow room (k=1)",
        seed_color=ElbowColors.K_AFTER,
    )
    mem.scale_to_fit_width(frame_w * 0.9)
    mem.next_to(structure, DOWN, buff=0.7)

    scene.play(FadeIn(mem), run_time=0.7)
    scene.wait(0.4)

    # Explanation line (bottom zone)
    expl = Text(
        "Zero-padding around the midpoint creates 'elbow room' near the honorary zero, "
        "shaping divisibility and prime density.",
        font_size=20,
        color=ElbowColors.TEXT_SECONDARY,
    )
    expl.scale_to_fit_width(frame_w * 0.9)
    expl.to_edge(DOWN, buff=0.4)

    scene.play(FadeIn(expl), run_time=0.5)
    scene.wait(0.7)

    # Fade out to leave clean slate for event montage
    scene.play(
        FadeOut(title),
        FadeOut(subtitle),
        FadeOut(structure),
        FadeOut(mem),
        FadeOut(expl),
        run_time=0.6,
    )


def create_pipeline_diagram() -> VGroup:
    """
    Create 3-node pipeline diagram showing data provenance:
    CSV → Rust/extract_elbow_events → JSON → Manim

    This makes the video self-documenting by showing its own production pipeline.
    """
    # Node 1: CSV Data
    csv_box = Rectangle(
        width=1.8,
        height=0.6,
        fill_color="#1e3a5f",
        fill_opacity=0.7,
        stroke_color="#4a90e2",
        stroke_width=2,
    )
    csv_label = Text("CSV Data", font_size=16, color=WHITE)
    csv_detail = Text("5,616 configs", font_size=12, color=GREY_B)
    csv_stack = VGroup(csv_label, csv_detail).arrange(DOWN, buff=0.05)
    csv_group = VGroup(csv_box, csv_stack)
    csv_stack.move_to(csv_box.get_center())

    # Arrow 1
    arrow1 = Arrow(
        start=ORIGIN,
        end=RIGHT * 1.2,
        color="#4a90e2",
        stroke_width=3,
        buff=0,
    )

    # Node 2: Rust Extractor
    rust_box = Rectangle(
        width=2.2,
        height=0.6,
        fill_color="#5f331e",
        fill_opacity=0.7,
        stroke_color="#e2904a",
        stroke_width=2,
    )
    rust_label = Text("Rust Extractor", font_size=16, color=WHITE)
    rust_detail = Text("extract_elbow_events", font_size=11, color=GREY_B)
    rust_stack = VGroup(rust_label, rust_detail).arrange(DOWN, buff=0.05)
    rust_group = VGroup(rust_box, rust_stack)
    rust_stack.move_to(rust_box.get_center())

    # Arrow 2
    arrow2 = Arrow(
        start=ORIGIN,
        end=RIGHT * 1.2,
        color="#e2904a",
        stroke_width=3,
        buff=0,
    )

    # Node 3: JSON Output
    json_box = Rectangle(
        width=1.8,
        height=0.6,
        fill_color="#3a1e5f",
        fill_opacity=0.7,
        stroke_color="#904ae2",
        stroke_width=2,
    )
    json_label = Text("JSON", font_size=16, color=WHITE)
    json_detail = Text("elbow_events.json", font_size=11, color=GREY_B)
    json_stack = VGroup(json_label, json_detail).arrange(DOWN, buff=0.05)
    json_group = VGroup(json_box, json_stack)
    json_stack.move_to(json_box.get_center())

    # Arrow 3
    arrow3 = Arrow(
        start=ORIGIN,
        end=RIGHT * 1.2,
        color="#904ae2",
        stroke_width=3,
        buff=0,
    )

    # Node 4: Manim Renderer
    manim_box = Rectangle(
        width=1.8,
        height=0.6,
        fill_color="#1e5f3a",
        fill_opacity=0.7,
        stroke_color="#4ae290",
        stroke_width=2,
    )
    manim_label = Text("Manim", font_size=16, color=WHITE)
    manim_detail = Text("Honorary Zero", font_size=11, color=GREY_B)
    manim_stack = VGroup(manim_label, manim_detail).arrange(DOWN, buff=0.05)
    manim_group = VGroup(manim_box, manim_stack)
    manim_stack.move_to(manim_box.get_center())

    # Arrange pipeline horizontally
    pipeline = VGroup(
        csv_group, arrow1, rust_group, arrow2,
        json_group, arrow3, manim_group
    ).arrange(RIGHT, buff=0.1)

    return pipeline


def play_statistical_context(scene: Scene, data: ElbowData) -> None:
    """
    Statistical context scene: k*=0 universality vs rare elbow events.
    Layout: title at top, two bars in middle, short paragraph at bottom.
    Timing optimized to ~5s.
    """
    scene.camera.background_color = ElbowColors.BACKGROUND
    frame_w = config.frame_width

    summary = data.summary or {}
    total_events = summary.get("total_events", len(data.events))
    total_boundaries = summary.get("total_boundaries", 5616)
    fraction = summary.get("fraction", total_events / max(total_boundaries, 1))

    elbow_pct = fraction * 100.0
    k0_pct = max(0.0, 100.0 - elbow_pct)

    # Title
    title = Text(
        "Statistical Context",
        font_size=38,
        color=ElbowColors.TEXT_PRIMARY,
    ).to_edge(UP, buff=0.6)
    scene.play(Write(title), run_time=0.6)

    # Bars (middle)
    bar_width_max = frame_w * 0.75
    bar_height = 0.4

    k0_bar_width = bar_width_max * (k0_pct / 100.0)
    elbow_bar_width = max(bar_width_max * (elbow_pct / 100.0), bar_width_max * 0.03)

    k0_rect = Rectangle(
        width=k0_bar_width,
        height=bar_height,
        fill_color=TEAL_C,
        fill_opacity=0.8,
        stroke_width=0,
    )
    elbow_rect = Rectangle(
        width=elbow_bar_width,
        height=bar_height,
        fill_color=RED_C,
        fill_opacity=0.8,
        stroke_width=0,
    )

    k0_label = Text(
        f"k*=0 universality: {k0_pct:5.2f}%",
        font_size=24,
        color=TEAL_C,
    )
    elbow_label = Text(
        f"Elbow events: {elbow_pct:5.3f}%",
        font_size=24,
        color=RED_C,
    )

    k0_group = VGroup(k0_label, k0_rect).arrange(DOWN, buff=0.2)
    elbow_group = VGroup(elbow_label, elbow_rect).arrange(DOWN, buff=0.2)

    bars = VGroup(k0_group, elbow_group).arrange(DOWN, buff=0.5)
    bars.move_to(ORIGIN).shift(UP * 0.3)

    # Animate bars growing from left
    for rect in [k0_rect, elbow_rect]:
        rect.stretch_to_fit_width(0.001)

    scene.play(
        FadeIn(k0_label),
        FadeIn(elbow_label),
        run_time=0.5,
    )
    scene.play(
        k0_rect.animate.stretch_to_fit_width(k0_bar_width),
        run_time=0.7,
    )
    scene.play(
        elbow_rect.animate.stretch_to_fit_width(elbow_bar_width),
        run_time=0.7,
    )
    scene.wait(0.3)

    # Context text (bottom-middle)
    context_text = Text(
        "Minimal padding (k*=0) wins on essentially all boundaries. "
        "Elbow events are rare structured pockets where expansion gains.",
        font_size=20,
        color=ElbowColors.TEXT_SECONDARY,
    )
    context_text.scale_to_fit_width(frame_w * 0.92)
    context_text.to_edge(DOWN, buff=1.6)  # Shifted up to make room for pipeline

    scene.play(FadeIn(context_text), run_time=0.6)
    scene.wait(0.5)

    # Pipeline diagram (bottom)
    pipeline = create_pipeline_diagram()
    pipeline.scale_to_fit_width(frame_w * 0.85)
    pipeline.to_edge(DOWN, buff=0.3)

    scene.play(FadeIn(pipeline), run_time=0.8)
    scene.wait(1.2)

    scene.play(
        FadeOut(title),
        FadeOut(bars),
        FadeOut(context_text),
        FadeOut(pipeline),
        run_time=0.6,
    )


# ---------------------------------------------------------------------------
# Scene classes (API used by your scripts)
# ---------------------------------------------------------------------------

class HonoraryZeroIntro(Scene):
    def construct(self) -> None:
        play_intro(self)


class ElbowEventMontage(Scene):
    """
    Data-driven, multi-event montage. Uses ElbowEventRenderer with
    layout rules that avoid overlap.
    """

    def construct(self) -> None:
        data = ElbowData()
        renderer = ElbowEventRenderer()

        if not data.events:
            self.camera.background_color = ElbowColors.BACKGROUND
            msg = Text(
                "No elbow events detected.\n"
                "k*=0 universality appears absolute on this dataset.",
                font_size=30,
                color=ElbowColors.TEXT_PRIMARY,
            )
            msg.scale_to_fit_width(config.frame_width * 0.9)
            msg.move_to(ORIGIN)
            self.play(FadeIn(msg), run_time=0.7)
            self.wait(1.5)
            self.play(FadeOut(msg), run_time=0.5)
            return

        total = len(data.events)
        for i, event in enumerate(data.events, start=1):
            renderer.show_single_event(self, event, i, total)


class StatisticalContext(Scene):
    def construct(self) -> None:
        data = ElbowData()
        play_statistical_context(self, data)


class ElbowRoomComplete(Scene):
    """
    Full narrative arc in a single scene:
      1. Honorary Zero intro (~5s)
      2. Event montage (~5s per event, 2 events = ~10s)
      3. Statistical context (~5s)
    Total: ~20s for typical 2-event case
    """

    def construct(self) -> None:
        data = ElbowData()
        renderer = ElbowEventRenderer()

        # 1. Intro
        play_intro(self)

        # 2. Montage (possibly no events)
        if not data.events:
            # Short "no events" message
            self.camera.background_color = ElbowColors.BACKGROUND
            msg = Text(
                "No elbow events detected.\n"
                "Minimal padding (k*=0) holds everywhere in this dataset.",
                font_size=30,
                color=ElbowColors.TEXT_PRIMARY,
            )
            msg.scale_to_fit_width(config.frame_width * 0.9)
            msg.move_to(ORIGIN)
            self.play(FadeIn(msg), run_time=0.7)
            self.wait(1.2)
            self.play(FadeOut(msg), run_time=0.5)
        else:
            total = len(data.events)
            for i, event in enumerate(data.events, start=1):
                renderer.show_single_event(self, event, i, total)

        # 3. Statistical context
        play_statistical_context(self, data)


class ViewSingleEvent(Scene):
    """
    Debug / inspection scene for a single event.

    Set event_index to choose which event (0-based).
    """

    event_index: int = 0

    def construct(self) -> None:
        data = ElbowData()
        renderer = ElbowEventRenderer()

        if not data.events:
            self.camera.background_color = ElbowColors.BACKGROUND
            msg = Text(
                "No elbow events available (elbow_events.json is missing or empty).",
                font_size=30,
                color=RED,
            )
            msg.scale_to_fit_width(config.frame_width * 0.9)
            msg.move_to(ORIGIN)
            self.play(FadeIn(msg), run_time=0.7)
            self.wait(1.5)
            self.play(FadeOut(msg), run_time=0.5)
            return

        # If we can find the canonical base-15 elbow event, prefer that
        target_index = self.event_index
        for i, e in enumerate(data.events):
            if (
                e.get("base") == 15
                and e.get("outer") == 13
                and e.get("inner") == 1
                and e.get("m_before") == 1
                and e.get("m_after") == 2
                and e.get("k_star_after") == 1
            ):
                target_index = i
                break

        if target_index < 0 or target_index >= len(data.events):
            target_index = 0

        event = data.events[target_index]
        renderer.show_single_event(self, event, target_index + 1, len(data.events))
        self.wait(0.5)


class ElbowEventGrid(Scene):
    """
    Simple static grid view of all events:
      - Each cell shows base + (outer, inner) + a small bar chart snapshot.
    Only meant as a quick overview, not heavily animated.
    """

    def construct(self) -> None:
        data = ElbowData()
        self.camera.background_color = ElbowColors.BACKGROUND
        frame_w = config.frame_width

        if not data.events:
            msg = Text(
                "No elbow events to display in grid.",
                font_size=30,
                color=ElbowColors.TEXT_PRIMARY,
            )
            msg.scale_to_fit_width(frame_w * 0.9)
            msg.move_to(ORIGIN)
            self.play(FadeIn(msg), run_time=0.7)
            self.wait(1.5)
            self.play(FadeOut(msg), run_time=0.5)
            return

        title = Text(
            "Elbow Events Overview",
            font_size=38,
            color=ElbowColors.TEXT_PRIMARY,
        ).to_edge(UP, buff=0.5)
        title.scale_to_fit_width(frame_w * 0.9)
        self.play(Write(title), run_time=0.6)

        cells: List[VGroup] = []
        max_cells = min(len(data.events), 6)

        for idx in range(max_cells):
            event = data.events[idx]
            base = event["base"]
            outer = event["outer"]
            inner = event["inner"]

            rows_after = event.get("rows_after", [])
            if not rows_after:
                continue

            k_values = [r["k"] for r in rows_after]
            densities = [r["density"] for r in rows_after]
            max_density = max(densities) if densities else 0.1
            if max_density <= 0:
                max_density = 0.1

            chart = BarChart(
                values=densities,
                y_range=[0, max_density * 1.2, max_density / 5],
                x_length=3.0,
                y_length=1.6,
                bar_width=0.25,
                bar_colors=[ElbowColors.DENSITY_MID for _ in k_values],
            )
            header = Text(
                f"Base {base}, ({outer},{inner})",
                font_size=20,
                color=ElbowColors.TEXT_PRIMARY,
            )
            header.scale_to_fit_width(3.4)
            cell = VGroup(header, chart).arrange(DOWN, buff=0.3)
            cells.append(cell)

        if not cells:
            msg = Text(
                "No elbow events to display.",
                font_size=30,
                color=ElbowColors.TEXT_PRIMARY,
            )
            msg.move_to(ORIGIN)
            self.play(FadeIn(msg), run_time=0.7)
            self.wait(1.5)
            self.play(FadeOut(msg), run_time=0.5)
            return

        grid = VGroup(*cells)
        grid.arrange_in_grid(
            rows=2,
            cols=max(1, (len(cells) + 1) // 2),
            buff=0.6,
            aligned_edge=ORIGIN,
        )
        grid.next_to(title, DOWN, buff=0.7)
        grid.scale_to_fit_width(frame_w * 0.9)

        scene.play(FadeIn(grid), run_time=0.8)
        scene.wait(1.5)
        scene.play(FadeOut(title), FadeOut(grid), run_time=0.6)


class MathBridgeScene(Scene):
    """
    Bridge from visual membrane to mathematical CRT structure.
    Shows: A·b^{2k} + S·b^k + A with Honorary Zero axis aligned
    with the middle term, and animated density response plot.

    Example: Base 15, (13,1), M=2
    """

    def construct(self) -> None:
        self.camera.background_color = ElbowColors.BACKGROUND
        frame_w = config.frame_width
        frame_h = config.frame_height

        # Use Base 15 (13,1) M=2 as canonical example
        base = 15
        outer = 13
        inner = 1
        M = 2

        # Phase 1: Title (0-2s)
        title = Text(
            "From Visual to Mathematical Structure",
            font_size=36,
            color=ElbowColors.TEXT_PRIMARY,
        )
        title.to_edge(UP)

        subtitle = Text(
            "Connecting membrane geometry to CRT orbits",
            font_size=24,
            color=ElbowColors.TEXT_SECONDARY,
        )
        subtitle.next_to(title, DOWN)

        self.play(Write(title), run_time=1.0)
        self.play(FadeIn(subtitle), run_time=0.6)
        self.wait(0.4)

        # Phase 2: Show mathematical formula (2-5s)
        # Create LaTeX formula: 13·15^{2k} + S·15^k + 13
        formula = MathTex(
            "13", r"\cdot", "15", "^{2k}", "+", "S", r"\cdot", "15", "^{k}", "+", "13",
            font_size=48,
        )
        formula.set_color_by_tex("13", ElbowColors.TEXT_PRIMARY)
        formula.set_color_by_tex("S", ElbowColors.K_BEFORE)
        formula.set_color_by_tex("15", GREY_A)
        formula.set_color_by_tex("k", ElbowColors.HONORARY_ZERO)

        formula.move_to([0, frame_h * 0.15, 0])

        self.play(FadeIn(formula), run_time=1.2)

        # Phase 3: Honorary Zero axis through middle term (5-7s)
        # Find position of S·15^k term
        middle_term_center = formula[5:9].get_center()

        hz_line = Line(
            start=[middle_term_center[0], middle_term_center[1] - 1.2, 0],
            end=[middle_term_center[0], middle_term_center[1] + 0.8, 0],
            color=ElbowColors.HONORARY_ZERO,
            stroke_width=4,
        )

        hz_label = Text(
            "Honorary Zero axis",
            font_size=20,
            color=ElbowColors.HONORARY_ZERO,
        )
        hz_label.next_to(hz_line, DOWN, buff=0.15)

        self.play(Create(hz_line), run_time=0.8)
        self.play(FadeIn(hz_label), run_time=0.5)
        self.wait(0.7)

        # Phase 4: Introduce density plot (7-9s)
        # Hardcoded densities for Base 15 (13,1) M=2
        k_values = [0, 1, 2, 3]
        densities = [0.085714, 0.114286, 0.028571, 0.038095]

        # Create axes for density plot
        axes = Axes(
            x_range=[-0.5, 3.5, 1],
            y_range=[0, 0.15, 0.05],
            x_length=frame_w * 0.5,
            y_length=frame_h * 0.3,
            axis_config={"color": GREY_B, "stroke_width": 2},
            tips=False,
        )
        axes.move_to([0, -frame_h * 0.25, 0])

        # Axes labels
        x_label = Text("k (padding)", font_size=18, color=GREY_A)
        x_label.next_to(axes.x_axis, DOWN, buff=0.2)

        y_label = Text("Prime density ρ", font_size=18, color=GREY_A)
        y_label.next_to(axes.y_axis, LEFT, buff=0.3)
        y_label.rotate(PI / 2)

        # k tick labels
        k_labels = VGroup()
        for k in k_values:
            lbl = Text(str(k), font_size=16, color=GREY_B)
            lbl.next_to(axes.c2p(k, 0), DOWN, buff=0.15)
            k_labels.add(lbl)

        self.play(Create(axes), FadeIn(x_label), FadeIn(y_label), FadeIn(k_labels), run_time=1.0)

        # Fade subtitle to make room
        self.play(subtitle.animate.set_opacity(0.2), run_time=0.5)
        self.wait(0.5)

        # Phase 5: Animate through k values (9-17s, 2s per k)
        dots = []
        lines = []

        for i, (k, density) in enumerate(zip(k_values, densities)):
            # Create point on plot
            point = axes.c2p(k, density)
            dot = Dot(point, radius=0.08, color=ElbowColors.K_AFTER if k == 1 else GREY_A)

            # Highlight k=1 (the elbow point) in green
            if k == 1:
                highlight_circle = Circle(radius=0.15, color=ElbowColors.K_AFTER, stroke_width=3)
                highlight_circle.move_to(point)
                dot = VGroup(dot, highlight_circle)

            # Update formula to show current k value
            k_indicator = MathTex(
                f"k = {k}",
                font_size=32,
                color=ElbowColors.K_AFTER if k == 1 else GREY_A,
            )
            k_indicator.next_to(formula, DOWN, buff=0.5)

            # Density value annotation
            density_text = Text(
                f"ρ = {density:.3f}",
                font_size=20,
                color=ElbowColors.K_AFTER if k == 1 else GREY_A,
            )
            density_text.next_to(dot, UP, buff=0.2)

            if i == 0:
                # First point: introduce
                self.play(
                    FadeIn(k_indicator),
                    FadeIn(dot),
                    FadeIn(density_text),
                    run_time=0.8,
                )
            else:
                # Subsequent points: transition
                prev_k_indicator = k_indicator_prev
                prev_density_text = density_text_prev

                # Draw line segment from previous point
                if i > 0:
                    line = Line(
                        axes.c2p(k_values[i-1], densities[i-1]),
                        axes.c2p(k, density),
                        color=GREY_A,
                        stroke_width=2,
                    )
                    self.play(Create(line), run_time=0.3)
                    lines.append(line)

                self.play(
                    Transform(prev_k_indicator, k_indicator),
                    FadeIn(dot),
                    Transform(prev_density_text, density_text),
                    run_time=0.8,
                )

            dots.append(dot)
            k_indicator_prev = k_indicator
            density_text_prev = density_text

            # Hold on k=1 (the elbow point) longer
            if k == 1:
                elbow_note = Text(
                    "↑ Elbow: k*=1 optimal at M=2",
                    font_size=18,
                    color=ElbowColors.K_AFTER,
                )
                elbow_note.next_to(axes, DOWN, buff=0.4)
                self.play(FadeIn(elbow_note), run_time=0.5)
                self.wait(1.0)
                self.play(FadeOut(elbow_note), run_time=0.4)
            else:
                self.wait(0.6)

        # Phase 6: Conclusion (17-19s)
        conclusion = Text(
            "CRT orbit → Density response",
            font_size=26,
            color=ElbowColors.TEXT_SECONDARY,
        )
        conclusion.next_to(axes, DOWN, buff=0.6)

        self.play(FadeIn(conclusion), run_time=0.7)
        self.wait(1.3)

        # Fade everything out
        self.play(
            FadeOut(title),
            FadeOut(subtitle),
            FadeOut(formula),
            FadeOut(hz_line),
            FadeOut(hz_label),
            FadeOut(axes),
            FadeOut(x_label),
            FadeOut(y_label),
            FadeOut(k_labels),
            FadeOut(k_indicator_prev),
            FadeOut(density_text_prev),
            *[FadeOut(dot) for dot in dots],
            *[FadeOut(line) for line in lines],
            FadeOut(conclusion),
            run_time=1.0,
        )
        self.wait(0.5)
