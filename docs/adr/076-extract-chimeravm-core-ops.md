# 076. Extract Core VM Ops to submodule

Date: 2026-03-31

## Status
Accepted

## Context
The `experiments/chimera-lang/src/vm/mod.rs` file was a massive "Blob" anti-pattern, containing around 5,000 lines. This included nearly all core opcode implementations directly in the `ChimeraVM` `impl` block, resulting in high coupling and poor maintainability.

## Decision
Extracted `exec_core_op` and related core opcode implementations (`exec_scavenge_op`, `exec_digest_op`, `exec_char_op`, `exec_mutagen_op`, `exec_findall_op`, `exec_havoc_op`, `exec_prion_op`, `exec_transposon`, and `exec_ribosome_command`) into a new dedicated module at `experiments/chimera-lang/src/vm/ops/core.rs`.

## Consequences

### Positive
*   **Maintainability:** Reduced the size of `mod.rs` by approximately 1000 lines.
*   **Coupling:** Core execution logic is properly isolated from the main VM implementation.
*   **Organization:** Improves overall project organization while retaining existing functionality.

### Negative
*   **Indirection:** Requires looking into an additional file (`core.rs`) to understand core opcode execution logic.
