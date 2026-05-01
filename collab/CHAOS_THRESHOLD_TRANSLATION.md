# Chaos-To-Threshold Translation

This note records the maintained arithmetic replacement for older
chaos/stability language in the repo.

## Replacement Vocabulary

- `stable regime`
  Arithmetic meaning: `k=(0,0)` is noninferior and positive anomaly mass has
  collapsed to zero on the bounded-`k` transition artifact.
- `boundary layer` / `edge of chaos`
  Arithmetic meaning: positive anomaly mass survives only at short middle
  lengths and only in sparse pair classes, with the live explanation carried by
  admissible-set change, shared-prime-yield change, or both.
- `signal source`
  Arithmetic meaning: whether the active short-length lane is led by the
  shared-admissible overlap (`stable_zero_led`) or by boundary transfer
  (`boundary_led`).

## Retired Historical Metaphors

- `deep chaos regime`
  Retired. No maintained bounded-`k` artifact currently supports a deeper
  simulation-style phase language.
- midpoint-threshold chaos claims
  Retired from the maintained lane. Midpoint-style chaos language is not part
  of the bounded-`k` arithmetic evidence surface.
- `chaos storm`
  Retired unless a future arithmetic classifier replaces it directly.
- attractor / Lyapunov threshold language for membrane claims
  Retired for public claim wording. The maintained threshold statement does not
  depend on simulation outputs.

## Why Density Is Only A Guardrail Here

The transition statement is about bounded-`k` arithmetic behavior across middle
length `M`, not about proving a prime-density theorem.

Density-like quantities still matter as guardrails:

- raw prime-rate changes keep us from confusing anomaly mass with admissible-set
  change
- shared-prime-rate deltas help separate overlap-led signal from boundary-led
  signal

But density is not the main theorem object in this lane. The maintained claim
is about when positive anomaly mass persists, when it collapses, and what exact
transfer/decomposition structure carries the surviving short-length signal.

## Boundary Of The Maintained Claim

Threshold language is only maintained when it is tied to the arithmetic
artifacts in the validation lane, especially:

- [examples/m2_m3_transition_report.rs](/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/examples/m2_m3_transition_report.rs)
- [examples/m_transition_curve_report.rs](/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/examples/m_transition_curve_report.rs)
- [examples/m_transition_phase_map_report.rs](/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/examples/m_transition_phase_map_report.rs)
- [examples/chaos_threshold_translation_report.rs](/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/examples/chaos_threshold_translation_report.rs)

It is **not** maintained when tied only to:

- [src/chaos/mod.rs](/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/src/chaos/mod.rs)
- Lyapunov outputs
- simulation-only variance or trajectory metaphors

## Practical Wording Rule

If we say `stable regime`, `boundary layer`, or `edge of chaos` in maintained
docs, we should be able to point to a bounded-`k` transition artifact and state
the exact arithmetic meaning in the same paragraph.
