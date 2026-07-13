# 155. Flatten Math Operations

Date: 2026-07-13

## Status
Accepted

## Context
In `experiments/chimera-lang/src/vm/ops/math.rs`, the `exec_math_op` function contained deeply nested `match` statements handling core arithmetic and comparison operations. This "Pyramid of Doom" made the code difficult to read, maintain, and extend, violating clean code principles.

## Decision
We flattened the `exec_math_op` function by extracting specific sub-operations (arithmetic and comparison logic) into isolated helper methods. We utilized `matches!` macros, `if let` guard clauses, and early returns to simplify the execution paths.

## Consequences
*   **Positive:** Significantly improves readability by eliminating the 'Pyramid of Doom'.
*   **Positive:** Increases maintainability by isolating distinct math operations into discrete helper methods.
*   **Negative:** Adds minor indirection by breaking a monolithic function into smaller helpers.
