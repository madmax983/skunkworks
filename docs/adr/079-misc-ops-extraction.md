# 079. Misc Ops Extraction

Date: 2026-06-15

## Status
Accepted

## Context
The `experiments/chimera-lang/src/vm/mod.rs` file still contained loosely coupled execution logic and operations for miscellaneous and esoteric behavior (`exec_prion_op`, `exec_transposon`, `exec_scavenge_op`, `exec_digest_op`, `exec_havoc_op`, `exec_char_op`, `exec_mutagen_op`, `exec_findall_op`, `handle_unknown_opcode`), representing roughly 500 lines of unrelated domain code.

## Decision
Extracted these remaining miscellaneous operations into a newly created `experiments/chimera-lang/src/vm/ops/misc.rs` file. Added `misc` to `experiments/chimera-lang/src/vm/ops/mod.rs`.

## Consequences

### Positive
*   **Maintainability:** Cleanly delegates the remaining execution methods into a separate file, resolving the final remnants of the 'Blob' within the main `mod.rs`.
*   **Cohesion:** Further separates unrelated operations from the core VM logic.

### Negative
*   **Indirection:** Requires checking another file (`misc.rs`) for miscellaneous opcode execution implementations.
