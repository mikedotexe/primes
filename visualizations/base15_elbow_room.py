# visualizations/base15_elbow_room.py

from manim import *


class Base15ElbowRoom(Scene):
    """
    Focused, non-overlapping 1D narrative for the canonical elbow event:
        base 15, outer = 13, inner = 1, M: 1 → 2, k*: 0 → 1.
    """

    def construct(self):
        self.camera.background_color = "#050509"
        frame_w = config.frame_width
        frame_h = config.frame_height

        # ------------------------------------------------------------------
        # Phase 1: Title + Honorary Zero on number line (0–5s)
        # ------------------------------------------------------------------
        title = Text(
            "Honorary Zero and Elbow Room",
            font_size=42,
            color=WHITE,
        ).to_edge(UP)

        subtitle = Text(
            "Base 15 membrane: rare shift in optimal zero-padding",
            font_size=26,
            color=GREY_B,
        ).next_to(title, DOWN)

        self.play(Write(title), run_time=1.2)
        self.play(FadeIn(subtitle), run_time=0.8)
        self.wait(0.8)

        # Number line 0..14 with honorary zero at 7.5
        base = 15
        digits = list(range(base))

        number_line = NumberLine(
            x_range=[-0.5, base - 0.5, 1],
            length=frame_w * 0.8,
            include_numbers=False,
        )
        number_line.to_edge(DOWN, buff=0.8)

        tick_labels = VGroup()
        for d in digits:
            x = number_line.n2p(d)[0]
            lbl = Text(str(d), font_size=24, color=GREY_A)
            lbl.move_to([x, number_line.get_y() + 0.35, 0])
            tick_labels.add(lbl)

        # Honorary zero (midpoint 7.5)
        mid_x = number_line.n2p(7.5)[0]
        hz_line = Line(
            start=[mid_x, number_line.get_y() - 0.5, 0],
            end=[mid_x, number_line.get_y() + 0.9, 0],
            color=YELLOW_E,
            stroke_width=4,
        )
        hz_label = Text(
            "Honorary Zero (midpoint 7.5)",
            font_size=22,
            color=YELLOW_E,
        )
        hz_label.next_to(hz_line, UP, buff=0.1)

        # Highlight outer=13, inner=1
        outer = 13
        inner = 1
        outer_label = tick_labels[outer]
        inner_label = tick_labels[inner]
        outer_box = SurroundingRectangle(outer_label, buff=0.08).set_stroke(YELLOW, width=3)
        inner_box = SurroundingRectangle(inner_label, buff=0.08).set_stroke(BLUE_C, width=3)

        self.play(
            FadeIn(number_line),
            FadeIn(tick_labels),
            run_time=1.0,
        )
        self.play(Create(hz_line), FadeIn(hz_label), run_time=0.7)
        self.play(Create(outer_box), Create(inner_box), run_time=0.7)
        self.wait(0.8)

        # ------------------------------------------------------------------
        # Phase 2: Membrane template for (outer, inner), M=1 (5–9s)
        # ------------------------------------------------------------------
        # Gently fade subtitle to make room later
        self.play(subtitle.animate.set_opacity(0.3), run_time=0.5)

        membrane = self.make_membrane_row(
            outer=outer,
            inner=inner,
            M=1,
            k=0,
            label="M = 1, k = 0   (no elbow room yet)",
            seed_color=BLUE_C,
        )
        membrane.scale_to_fit_width(frame_w * 0.8)
        membrane.next_to(number_line, UP, buff=0.6)

        self.play(FadeIn(membrane), run_time=1.0)
        self.wait(1.0)

        # Prepare to transition: we will fade out number line while we
        # keep the "mental model" of honorary zero.
        self.play(
            FadeOut(outer_box),
            FadeOut(inner_box),
            FadeOut(tick_labels),
            FadeOut(number_line),
            FadeOut(hz_label),
            FadeOut(hz_line),
            run_time=0.8,
        )
        self.wait(0.2)

        # ------------------------------------------------------------------
        # Phase 3: Bar chart for M=1 (9–13s)
        # ------------------------------------------------------------------
        # M=1 densities for base 15, (13,1)
        # k = 0,1,2 => 0.071429; k=3 => 0.0
        ks = [0, 1, 2, 3]
        densities_m1 = [0.071429, 0.071429, 0.071429, 0.0]
        densities_m2 = [0.085714, 0.114286, 0.028571, 0.038095]

        max_density = max(max(densities_m1), max(densities_m2))
        if max_density <= 0:
            max_density = 0.1

        chart = BarChart(
            values=densities_m1,
            y_range=[0, max_density * 1.2, max_density / 5],
            x_length=frame_w * 0.6,
            y_length=frame_h * 0.35,
            bar_width=0.5,
            bar_colors=[GREY_A] * len(ks),
        )
        chart.next_to(membrane, DOWN, buff=0.7)
        chart.align_to(membrane, LEFT)

        m_label = Text(
            "M = 1",
            font_size=26,
            color=BLUE_C,
        ).next_to(chart, DOWN, buff=0.3)

        # k labels under bars
        k_labels = VGroup()
        for i, k in enumerate(ks):
            x = chart.x_axis.n2p(i)[0]
            lbl = Text(f"k = {k}", font_size=20, color=GREY_B)
            lbl.move_to([x, chart.x_axis.get_y() - 0.35, 0])
            k_labels.add(lbl)

        self.play(Create(chart), FadeIn(k_labels), Write(m_label), run_time=1.2)
        # Mark plateau (all k=0,1,2 equal)
        plateau_annotation = Text(
            "At M = 1, all k ∈ {0,1,2} have the same density (~0.071).",
            font_size=22,
            color=GREY_B,
        )
        plateau_annotation.next_to(chart, DOWN, buff=0.8)
        plateau_annotation.scale_to_fit_width(frame_w * 0.9)

        self.play(FadeIn(plateau_annotation), run_time=0.7)
        self.wait(1.0)

        # ------------------------------------------------------------------
        # Phase 4: M=2 transition, k* shifts 0 → 1 (13–19s)
        # ------------------------------------------------------------------
        # Membrane grows from (M=1,k=0) to (M=2,k=1)
        membrane_m2 = self.make_membrane_row(
            outer=outer,
            inner=inner,
            M=2,
            k=1,
            label="M = 2, k = 1   (one unit of elbow room)",
            seed_color=GREEN_C,
        )
        membrane_m2.scale_to_fit_width(frame_w * 0.8)
        membrane_m2.move_to(membrane.get_center())

        # Update chart colors for k*=1
        new_bar_colors = [
            GREEN_C if k == 1 else GREY_A
            for k in ks
        ]
        m_label_m2 = Text(
            "M = 2",
            font_size=26,
            color=GREEN_C,
        ).next_to(chart, DOWN, buff=0.3)

        # Remove plateau text to avoid crowding
        self.play(FadeOut(plateau_annotation), run_time=0.5)

        # Animate: membrane stretches, bars morph, label changes
        self.play(
            ReplacementTransform(membrane, membrane_m2),
            chart.animate.change_bar_values(densities_m2),
            Transform(m_label, m_label_m2),
            *[chart.bars[i].animate.set_fill(new_bar_colors[i]) for i in range(len(ks))],
            run_time=2.0,
        )

        # Highlight the new optimal k=1 bar
        bar_k1 = chart.bars[1]
        highlight_k1 = SurroundingRectangle(bar_k1, buff=0.05).set_stroke(GREEN_C, width=4)

        annotation_m2 = Text(
            "When the middle grows to M = 2, the best density moves to k = 1.",
            font_size=22,
            color=GREY_B,
        )
        annotation_m2.next_to(chart, DOWN, buff=0.8)
        annotation_m2.scale_to_fit_width(frame_w * 0.9)

        self.play(Create(highlight_k1), FadeIn(annotation_m2), run_time=1.0)
        self.wait(1.2)

        # ------------------------------------------------------------------
        # Phase 5: Slide chart left, show real primes on the ridge (19–24s)
        # ------------------------------------------------------------------
        # Slide membrane + chart left to make room for side panel
        self.play(
            membrane_m2.animate.shift(LEFT * frame_w * 0.25),
            chart.animate.shift(LEFT * frame_w * 0.25),
            k_labels.animate.shift(LEFT * frame_w * 0.25),
            m_label.animate.shift(LEFT * frame_w * 0.25),
            highlight_k1.animate.shift(LEFT * frame_w * 0.25),
            annotation_m2.animate.shift(LEFT * frame_w * 0.25),
            run_time=1.0,
        )

        prime_panel = self.make_prime_panel()
        prime_panel.scale_to_fit_height(frame_h * 0.7)
        prime_panel.to_edge(RIGHT, buff=0.6)
        prime_panel.align_to(membrane_m2, UP)

        self.play(FadeIn(prime_panel), run_time=1.0)
        self.wait(2.0)

        # Fade everything out cleanly
        self.play(
            FadeOut(title),
            FadeOut(subtitle),
            FadeOut(membrane_m2),
            FadeOut(chart),
            FadeOut(k_labels),
            FadeOut(m_label),
            FadeOut(highlight_k1),
            FadeOut(annotation_m2),
            FadeOut(prime_panel),
            run_time=1.2,
        )
        self.wait(0.5)

    # ----------------------------------------------------------------------
    # Helpers
    # ----------------------------------------------------------------------

    def make_membrane_row(
        self,
        outer: int,
        inner: int,
        M: int,
        k: int,
        label: str,
        seed_color=BLUE_C,
    ) -> VGroup:
        """
        Symbolic membrane row with honorary-zero axis at the seed center:

           [outer] [0×k] [inner] [0×k] [SEED (width∝M)] [0×k] [inner] [0×k] [outer]
        """
        boxes = VGroup()

        def box(text: str, color=GREY_A) -> VGroup:
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
        boxes.add(box(str(outer), WHITE))

        # k zeros
        for _ in range(k):
            boxes.add(box("0", GREY_B))

        # inner
        boxes.add(box(str(inner), WHITE))

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
        boxes.add(box(str(inner), WHITE))

        # k zeros
        for _ in range(k):
            boxes.add(box("0", GREY_B))

        # outer
        boxes.add(box(str(outer), WHITE))

        boxes.arrange(RIGHT, buff=0.05)

        # HZ axis through SEED center
        cx = boxes.get_center()[0]
        top_y = boxes.get_top()[1] + 0.15
        bot_y = boxes.get_bottom()[1] - 0.15

        hz_line = Line(
            start=[cx, bot_y, 0],
            end=[cx, top_y, 0],
            color=YELLOW_E,
            stroke_width=3,
        )

        lbl = Text(label, font_size=18, color=seed_color)
        lbl.next_to(boxes, DOWN, buff=0.15)

        return VGroup(boxes, hz_line, lbl)

    def make_prime_panel(self) -> VGroup:
        """
        Panel with concrete primes on the M=2, k=1 ridge for base 15, outer=13, inner=1.
        """
        title = Text(
            "Sample primes on the M = 2, k = 1 ridge",
            font_size=24,
            color=WHITE,
        )

        line1 = Text(
            "13 0 1 0 8 1 0 1 0 13₁₅  =  499 935 695 863",
            font_size=18,
            font="monospace",
            color=GREY_B,
        )
        line2 = Text(
            "13 0 1 0 14 1 0 1 0 13₁₅ =  499 935 999 613",
            font_size=18,
            font="monospace",
            color=GREY_B,
        )

        panel = VGroup(title, line1, line2).arrange(DOWN, aligned_edge=LEFT, buff=0.2)
        panel.set_background_stroke(width=0)
        panel.add_background_rectangle(color=BLACK, opacity=0.7, buff=0.2)
        return panel
