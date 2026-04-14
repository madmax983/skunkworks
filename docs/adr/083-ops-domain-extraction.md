# 083. Ops Domain Extraction

Date: 2026-04-01

## Status
Accepted

## Context
The Blob anti-pattern was present in `experiments/chimera-lang/src/vm/mod.rs` (~5,000 lines long). Numerous specific operations (`exec_havoc_op`, `exec_transposon`, `exec_scavenge_op`, `exec_char_op`, etc.) were grouped alongside the main core dispatcher. This made domain boundaries fuzzy, hindered maintainability, and made testing difficult.

## Decision
Extracted the operation execution blocks into `experiments/chimera-lang/src/vm/ops/` submodules, specifically `mutation.rs`, `string.rs`, `resource.rs`, `ribosome.rs`, and `core.rs`. Created a facade via `ops/mod.rs` to maintain simple structural boundaries without breaking feature logic or the existing API.

## Consequences

### Positive
*   **Cohesion:** Ensures execution routines are grouped strictly by domain responsibility.
*   **Maintainability:** Significantly reduces the file length of the root module.
*   **Testing:** Easier to test specific domain operations independently.

### Negative
*   **Indirection:** Requires navigating into specific submodules under `ops/` to trace operation execution logic instead of having everything in `mod.rs`.
