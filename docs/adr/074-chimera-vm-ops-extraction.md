# 074. Extract ChimeraVM ops to submodules

## Status
Accepted

## Context
The structural problem was that `experiments/chimera-lang/src/vm/mod.rs` was a 5600+ line Blob file that handled all opcode execution methods inside a single `impl ChimeraVM`.

## Decision
Extracted `exec_math_op`, `exec_stack_op`, `exec_flow_op`, `exec_grid_op`, `exec_io_op`, and `exec_bio_op` into a new `vm/ops/` module structure, with each submodule handling specific categories of execution via `impl crate::vm::ChimeraVM`.

## Consequences

### Positive
*   **Maintainability:** Reduced coupling and shorter module size.
*   **High Cohesion:** Logically separated execution handlers.

### Negative
*   **Indirection:** Execution logic is now spread across multiple submodules, requiring developers to navigate a directory structure to find specific opcode implementations.