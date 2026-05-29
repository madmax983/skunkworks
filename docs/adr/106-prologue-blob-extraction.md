# 106. Prologue Blob Extraction

Date: 2026-04-27

## Status
Accepted

## Context
The `experiments/chimera-lang/src/vm/mod.rs` file retained the "Blob" anti-pattern due to a large block of logic functioning as a routing and execution table for Prologue operations. This central execution logic created an unnecessary bottleneck in the core VM module.

## Decision
Extracted the `exec_prologue` and related execution loops from `vm/mod.rs` into the `experiments/chimera-lang/src/vm/prologue/` module structure.

## Consequences

### Positive
*   **Decoupling:** Correctly encapsulates the Prologue execution logic, improving cohesion.
*   **Maintainability:** Reduces the size and complexity of the primary VM module (`mod.rs`).

### Negative
*   **Indirection:** Adds a layer of indirection for finding Prologue operations execution.
