# 16. Extract Resonance Audio Engine

Date: 2024-10-25

## Status

Accepted

## Context

Experiments involving audio synthesis and simulation, such as `resonance-chamber`, `hertzian-shimmer`, and `chimera-resonance`, rely on a shared underlying physical model: the 2D wave equation on a grid.

Initially, this physics simulation was coupled with the TUI rendering logic or the audio callback loop within each specific experiment. This coupling made it difficult to:
1.  **Reuse the Physics:** Creating a new experiment with the same wave mechanics required copy-pasting code.
2.  **Isolate Audio Logic:** The audio processing loop (often running on a separate thread) was entangled with the application state.
3.  **Maintain Consistency:** Improvements to the wave equation solver (e.g., damping, boundary conditions) in one experiment did not propagate to others.

## Decision

We have extracted the wave physics and audio state management into a dedicated shared crate: `crates/resonance-audio`.

This crate encapsulates:
1.  **PhysicsGrid:** A struct implementing the 2D wave equation solver with wall boundaries and damping.
2.  **AudioModel:** A high-level container managing the grid, oscillators, and the listener position.
3.  **AudioCommand:** An enum for thread-safe communication between the main application thread and the audio processing thread.

## Consequences

**Positive:**
-   **Decoupled Physics:** The simulation logic is purely numerical and independent of `ratatui` or `cpal`.
-   **Consistent Behavior:** All resonance-based experiments share the same fluid-like wave propagation dynamics.
-   **Thread Safety:** The `AudioCommand` pattern enforces a clean separation of concerns, reducing race conditions between UI and Audio threads.

**Negative:**
-   **Complexity:** Simple experiments now require setting up the `AudioModel` and command channels rather than just writing a simple audio callback.
-   **Performance:** The `PhysicsGrid` simulation runs on the CPU. Sharing this heavy logic across multiple active experiments (if run simultaneously) could strain the system, though typically only one runs at a time.
