# 089. Extract Organelle Processing

Date: 2024-05-21

## Status
Accepted

## Context
The `experiments/chimera-lang/src/vm/mod.rs` file contained a monolithic block of over 500 lines dedicated entirely to `process_organelles` and related organelle execution functions. This created a tight coupling between the VM core execution loop and the domain-specific logic of the Metazoa system's organelles. The bloated file size made the core VM logic difficult to navigate and maintain.

## Decision
Extracted the `process_organelles` and related organelle execution functions into a new module: `experiments/chimera-lang/src/vm/organelles.rs`. The logic is encapsulated in an `impl crate::vm::ChimeraVM` block within this new file.

## Consequences

### Positive
*   **Separation of Concerns:** The core VM module (`mod.rs`) is now cleaner and focused on general execution state, while organelle-specific processing logic is isolated in its own domain file.
*   **Maintainability:** The overall length of `mod.rs` is significantly reduced, decreasing the cognitive load when modifying core VM behavior.
*   **Modularity:** The Metazoa system's processing logic is more distinct within the codebase, reflecting its conceptual boundary as an independent subsystem running within the VM.

### Negative
*   **Indirection:** VM execution tracing requires navigating to `organelles.rs` to follow the organelle step behavior rather than reading it linearly in `mod.rs`.
