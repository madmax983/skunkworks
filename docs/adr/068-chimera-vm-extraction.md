# 068. Chimera VM Extraction

## Status

Proposed

## Context

The `ChimeraVM` engine in `experiments/chimera-lang/src/vm/mod.rs` had grown into a "God Module" anti-pattern, containing over 1,900 lines of core execution logic intertwined with state management and initialization. This tight coupling made it difficult to maintain, test, and understand the core execution loop independently from the VM's structural state.

## Decision

We decided to extract the core execution logic out of `vm/mod.rs` into a dedicated `ops.rs` module. The `ChimeraVM` struct now delegates the actual instruction execution to functions within the `ops` module, while retaining ownership of its state.

## Consequences

### Positive

- **Maintainability:** Breaks up a massive monolithic file into more manageable, focused modules.
- **Separation of Concerns:** Clearly separates state management (`mod.rs`) from execution logic (`ops.rs`).
- **Readability:** Easier for developers to navigate the codebase and understand the distinct phases of the VM lifecycle.

### Negative

- **Internal Coupling:** The `ops` module functions still tightly couple to the `ChimeraVM` state, often taking `&mut self` (the VM itself) as an argument to perform their operations.
- **API Boundary Surface Area:** Slightly increases the number of internal files a new developer needs to be aware of when learning the VM architecture.
