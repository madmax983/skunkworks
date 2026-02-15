# 34. Extract Soroban Logic

Date: 2024-05-20

## Status

Accepted

## Context

The repository contains multiple experiments that visualize the operation of a Japanese Abacus (Soroban). Originally, the logic for bead manipulation, column state, and arithmetic operations (carrying, borrowing) was implemented directly within `experiments/soroban-market`.

As we expanded to include `experiments/soroban-specter`—which visualizes the same logic using fluid dynamics and `macroquad` instead of a TUI—we faced a dilemma:
1.  Duplicate the logic (violating DRY).
2.  Couple the new experiment to the old one (introducing unnecessary dependencies).

The domain logic itself (physical bead positions representing values) is distinct from how those beads are rendered (ASCII characters vs. fluid particles).

## Decision

We have extracted the core Soroban logic into a dedicated shared crate: `crates/soroban`.

This crate contains:
*   The `Column` struct: Modeling the 1 Heaven / 4 Earth bead configuration.
*   The `Soroban` struct: A collection of 13 columns representing $10^{13}$.
*   Arithmetic implementation: `add` and `sub` methods that simulate physical bead movement, including carry and borrow operations.

Experiments now depend on this crate via the workspace:

```toml
[dependencies]
soroban = { workspace = true }
```

## Consequences

### Positive
*   **Decoupled Logic:** The physical model of the abacus is now independent of the visualization layer (TUI, WGPU, Macroquad).
*   **Single Source of Truth:** Any bug fixes or improvements to the arithmetic logic (e.g., implementing multiplication) will propagate to all consuming experiments.
*   **Hybrid Potential:** We can easily create hybrid experiments that visualize the same `Soroban` instance state in multiple ways simultaneously.

### Negative
*   **Workspace Complexity:** Experiments are no longer self-contained single files; they require the workspace structure to build.
*   **Generic Constraints:** The `Soroban` struct imposes a specific 13-column limit which may not fit all future visualization needs (though it matches the standard physical instrument).
