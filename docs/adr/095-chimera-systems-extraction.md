# 095. Extract ChimeraVM Systems

Date: 2026-05-05

## Status
Accepted

## Context
The `experiments/chimera-lang/src/vm/mod.rs` file was heavily bloated (~3500 lines) with extensive operational logic for subsystem processing (`process_environment`, `process_symbiotes`, `process_nova_environment`, `process_subsystems`, `process_chaos_and_events`). These systems intertwined pure VM core loop state with external domain logic like nova flux and diffusion.

## Decision
Created `experiments/chimera-lang/src/vm/systems/` directory. Extracted the large system processing loops into cleanly isolated, domain-specific submodules (`environment.rs`, `symbiotes.rs`, `nova_environment.rs`, `subsystems.rs`, `chaos.rs`), extending `impl crate::vm::ChimeraVM`.

## Consequences

### Positive
*   **Decoupling:** Decouples core VM looping logic from specific feature subsystem operations.
*   **Maintainability:** Significantly reduces the file size of the central `mod.rs`.
*   **Encapsulation:** Subsystem concerns are now isolated in their respective files.

### Negative
*   **Indirection:** Adds another layer of directory structure, which requires developers to navigate to `systems/` to understand how the core loops interact with the rest of the simulation.