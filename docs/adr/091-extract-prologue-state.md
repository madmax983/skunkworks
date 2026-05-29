# 091. Extract Prologue State

Date: 2026-04-30

## Status
Accepted

## Context
The `experiments/chimera-lang/src/vm/prologue/mod.rs` file exhibited signs of becoming an architectural "Blob", housing both the massive `exec_prologue_tick` logic and the underlying shared data structures like `PrologueState` and `PrologueAgent`. Attempting to extract modular execution logic (e.g., decoupling submodules into independent units) was blocked by tight coupling: submodules relied heavily on implicit state access provided by `mod.rs`. Furthermore, extracting the execution loop without first extracting the data structures led to broken type inferences and lost implicit scope, as dependent submodules could no longer dereference the shared state.

## Decision
Extracted the foundational data structures—specifically `PrologueState`, `PrologueAgent`, and shared packing/unpacking utilities—into a dedicated `state.rs` file within the `prologue` module. `mod.rs` and all other submodules now import these core structures from `state.rs`.

## Consequences

### Positive
*   **Decoupling:** Safely separates data models from execution logic, breaking the immediate circular dependencies that arise during execution loop refactoring.
*   **Architectural Clarity:** The data structures that represent the state of the Prologue system are now independently defined and easier to test without pulling in the entire VM execution context.
*   **Unblocks Refactoring:** This separation serves as the necessary first step to further decouple the `exec_prologue_tick` function without breaking downstream logic.

### Negative
*   **Indirection:** Requires an additional explicit context import (`use super::state::PrologueState`) in submodules where implicit scoping previously sufficed.