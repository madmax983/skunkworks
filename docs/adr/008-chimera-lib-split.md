# 8. Chimera Lang Library/Binary Split

Date: 2024-10-24

## Status

Accepted

## Context

The `chimera-lang` experiment began as a standalone TUI application for simulating a bio-inspired esoteric programming language. However, as the Skunkworks repository evolved, other experiments (such as `penrose-genes` and `story_demo`) required access to the underlying Virtual Machine (VM) and Abstract Syntax Tree (AST) definitions to implement hybrid systems or procedural narratives.

Embedding the logic by duplicating code would violate DRY principles and make maintenance of the "Nova" feature set difficult across multiple projects.

## Decision

We have restructured `experiments/chimera-lang` to function as both a binary and a library.

1.  **Exposed Modules**: The `ast`, `vm`, and `tui` modules are now public in `src/lib.rs`.
2.  **Crate Type**: The `Cargo.toml` is configured (implicitly or explicitly) to support library linkage.
3.  **Feature Flags**: The `nova` feature flag is propagated through the library, allowing consumers to opt-in to advanced biological features (endocrine system, epigenetics).

## Consequences

**Positive:**
*   **Reusability**: Other experiments can import `chimera_lang::vm::ChimeraVM` and drive the simulation programmatically.
*   **Testing**: Integration tests can be written more easily by importing the library rather than spawning child processes.
*   **Extensibility**: Hybrid experiments can implement custom `Enzyme` logic or inspect the `Helix` state directly.

**Negative:**
*   **API Stability**: Changes to the internal VM structure (e.g., `PetriDish` layout) now affect downstream consumers, requiring stricter versioning or coordination.
*   **Complexity**: The `tui` module being public might encourage tight coupling to specific rendering implementations if not carefully managed.
