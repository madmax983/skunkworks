# 156. Flatten Dispatch Operations

Date: 2026-07-14

## Status

Accepted

## Context

The `CoreOps` and `NovaDispatchOps` modules in `chimera-lang/src/vm/ops/core_dispatch.rs` and `nova_dispatch.rs` were plagued by deeply nested 'Pyramid of Doom' `match` branches to resolve execution paths returning `Option<(usize, usize)>`. This repetition made the dispatcher modules hard to parse visually and prone to formatting errors.

## Decision

Introduce a `Dispatch` enum (`Handled`, `Jump(usize, usize)`, `Unhandled`) in `mod.rs` and implement `From<Option<(usize, usize)>> for Dispatch`. Refactor the large `match` blocks in the dispatch modules into idiomatic `.into()` calls, mapping subsystem execution paths sequentially.

## Consequences

- **Positive:** Flattens deeply nested `match` branches into sequential `.into()` chains.
- **Positive:** Normalizes the expected return type and control flow semantics across all operation handlers.
- **Negative:** Introduces a new internal type `Dispatch` that must be understood by contributors working on the VM execution engine.
