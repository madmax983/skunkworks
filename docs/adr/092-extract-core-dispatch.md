# 092. Extract Core Dispatch

Date: 2026-05-01

## Status
Proposed

## Context
The `vm/mod.rs` file retained the "Blob" anti-pattern due to a large `match` block inside `exec_core_op` that functioned as a routing table for core VM operations. This central routing logic created an unnecessary bottleneck in the core VM module.

## Decision
Extracted the `exec_core_op` method from `vm/mod.rs` into its own module `vm/ops/core_dispatch.rs`.

## Consequences

### Positive
*   **Decoupling:** Correctly encapsulates the dispatching logic, improving cohesion.
*   **Maintainability:** Reduces the size and complexity of the primary VM module (`mod.rs`).
*   **Accessibility:** Keeps the dispatch logic accessible intact via `pub(crate)` without modifying runtime behavior or public APIs.

### Negative
*   **Indirection:** Adds a layer of indirection for finding core operations execution.
