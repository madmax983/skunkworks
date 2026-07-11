# 140. Flatten neuro-calligraphy Builder

Date: 2026-06-30

## Status

Accepted

## Context

The `neuro-calligraphy` experiment had a `Builder` struct used as a simple accumulator for glyph contours in `experiments/neuro-calligraphy/src/font.rs`. The naming "Builder" implied a speculative generality and added unnecessary cognitive overhead, violating the KISS principle.

## Decision

The `Builder` struct was renamed to `OutlineSink` to more accurately reflect its concrete role as a state sink.

## Consequences

*   **Positive:** Reduces cognitive load by eliminating speculative abstraction naming. Aligns with essentialist engineering practices (KISS).
*   **Negative:** None.
