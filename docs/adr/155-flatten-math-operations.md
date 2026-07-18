# 155. Flatten Math Operations

Date: 2026-07-14

## Status

Accepted

## Context

The `MathOps` module in `chimera-lang/src/vm/ops/math.rs` suffered from deeply nested 'Pyramid of Doom' `match` blocks. The monolithic execution flow within `exec_math_op` created high cognitive load and made testing individual mathematical operations difficult, as the entire state had to be mocked up for the top-level executor.

## Decision

Extract the logic for each individual mathematical operation into dedicated helper functions (`apply_eq`, `apply_cmp`, `apply_add_str_concat`, `apply_div`, `apply_mod`, `apply_bit_not`) and flatten control flow using Guard Clauses and early returns.

## Consequences

- **Positive:** Reduces cognitive complexity and improves code readability.
- **Positive:** Enables unit testing of individual operations in isolation.
- **Negative:** Slightly increases the number of internal methods on `ChimeraVM`.
