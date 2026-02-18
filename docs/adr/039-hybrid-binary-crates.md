# 39. Hybrid Binary Crate Strategy

Date: 2024-05-22

## Status

Accepted

## Context

The repository contains numerous experimental prototypes, many of which are implemented as binary crates (e.g., `circuit-sigil`). These experiments are designed to be self-contained and often have specific dependencies or configurations that make them difficult to reuse as libraries without significant refactoring.

However, we frequently desire to create "Hybrid Experiments" that combine the logic of two or more existing experiments (e.g., `chimera-circuit` combining `circuit-sigil`'s PCB generation with `chimera-lang`'s genetic VM).

Refactoring the original binary crates into libraries (with separate `lib.rs` and `main.rs`) is the "correct" software engineering approach but introduces friction:
1.  **Velocity**: It requires halting new development to refactor existing code.
2.  **Complexity**: It creates a web of inter-dependencies between what should be isolated experiments.
3.  **Risk**: Changes to the shared library might break the original experiment or require extensive regression testing.

## Decision

We have decided to allow **source code duplication (adaptation)** from binary crates into new hybrid experiments.

Specifically:
1.  When a new experiment requires logic from an existing binary crate, we will copy the relevant source modules (e.g., `circuit.rs`) into the new experiment's crate.
2.  We will adapt the copied code as necessary to fit the new context (e.g., removing CLI parsing, exposing internal structs).
3.  We acknowledge that these files are "forks" and do not strictly enforce synchronization with the original.

## Consequences

### Positive
*   **Decoupling**: Hybrid experiments remain self-contained and do not depend on the stability of the original experiment's internal API.
*   **Velocity**: Rapid prototyping is enabled without waiting for refactoring.
*   **Preservation**: The original experiment remains untouched and stable.

### Negative
*   **Duplication**: Bug fixes or improvements in the original experiment are not automatically propagated to the hybrid.
*   **Drift**: The two versions of the code will likely diverge over time, making future reunification difficult.

## Compliance

*   Hybrids using this strategy should explicitly mention their lineage in their `README.md` (e.g., "Adapted from `experiments/circuit-sigil`").
*   Common, stable infrastructure (e.g., `tui-shared`, `locus`, `quipu`) should still be extracted to shared libraries in `crates/`. This strategy applies primarily to *experimental domain logic*.
