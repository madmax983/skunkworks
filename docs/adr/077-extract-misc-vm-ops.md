# 077. Extract Misc VM Ops to submodule

Date: 2026-04-01

## Status
Accepted

## Context
The `experiments/chimera-lang/src/vm/mod.rs` file was a massive "Blob" anti-pattern, containing around 5,000 lines. This included numerous miscellaneous opcode implementations directly in the `ChimeraVM` `impl` block, resulting in high coupling and poor maintainability.

## Decision
Extracted miscellaneous opcode execution logic (`exec_prion_op`, `exec_transposon`, `exec_scavenge_op`, `exec_digest_op`, `exec_char_op`, `exec_mutagen_op`, `exec_findall_op`, `exec_havoc_op`) into a dedicated module at `experiments/chimera-lang/src/vm/ops/misc.rs`.

## Consequences

### Positive
*   **Maintainability:** Reduced the size of `mod.rs` by approximately 500 lines.
*   **Coupling:** Miscellaneous execution logic is properly isolated from the main VM implementation.
*   **Organization:** Improves overall project organization while retaining existing functionality.

### Negative
*   **Indirection:** Requires looking into an additional file (`misc.rs`) to understand the miscellaneous opcode execution logic.
