# 078. Math/Bitwise Operator Untangling

Date: 2026-05-18

## Status
Accepted

## Context
The "Blob" anti-pattern in `experiments/chimera-lang/src/vm/mod.rs` had resulted in multiple fundamental core math and bitwise operations (`Mod`, `BitAnd`, `BitOr`, `BitXor`, `BitNot`, `Shl`, `Shr`) being improperly placed inside `exec_havoc_op` rather than their correct domain context inside `exec_math_op`.

## Decision
Extracted these mathematical and bitwise execution blocks from `exec_havoc_op` and relocated them to `exec_math_op` within the dedicated `experiments/chimera-lang/src/vm/ops/math.rs` module.

## Consequences

### Positive
*   **Domain Alignment:** Enforces strict domain boundaries by ensuring mathematical and bitwise operations live in the `math` domain.
*   **Maintainability:** Reduces the size of `mod.rs` and simplifies the `exec_havoc_op` handler.
*   **Cohesion:** Ensures that `exec_havoc_op` is solely responsible for its designated mutation domain (`HavocRate` and `HavocScope`).

### Negative
*   **None**
