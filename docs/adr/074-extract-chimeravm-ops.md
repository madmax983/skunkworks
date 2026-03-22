# 074. Extract ChimeraVM ops to submodules

## Status
Accepted

## Context
The Blob - `experiments/chimera-lang/src/vm/mod.rs` was a 5600+ line monolithic file that handled all opcode execution methods inside a single `impl ChimeraVM`. This structure made it incredibly difficult to navigate, maintain, and understand the distinct execution contexts of the virtual machine.

## Decision
Extracted the monolithic execution logic (`exec_math_op`, `exec_stack_op`, `exec_flow_op`, `exec_grid_op`, `exec_io_op`, and `exec_bio_op`) into a new `vm/ops/` module structure. Each category of execution is now handled by a dedicated submodule (`bio.rs`, `flow.rs`, `grid.rs`, `io.rs`, `math.rs`, `stack.rs`) that extends `impl crate::vm::ChimeraVM`.

## Consequences

### Positive
*   **Reduced Coupling:** Execution logic is logically separated based on the category of operations.
*   **Maintainability:** Shorter module sizes make the code easier to read and modify.
*   **Build Times:** Faster compilation due to smaller, decoupled files.

### Negative
*   **Indirection:** Requires developers to trace execution logic into multiple separate files, rather than viewing the entire `ChimeraVM` implementation in one place.
