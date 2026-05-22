# 097. Extract Binary Math Ops to Math Domain

Date: 2026-05-11

## Status
Proposed

## Context
The mathematical execution logic in `experiments/chimera-lang/src/vm/ops/math.rs` was duplicating basic binary operation processing across several different OpCodes (like `Add`, `Sub`, `Mul`, `Div`). This created repetitive code blocks and increased the maintenance burden of adding or modifying core arithmetic evaluation.

## Decision
Extracted a reusable `binary_op` helper function inside the `math.rs` module. All arithmetic OpCode handlers (`Add`, `Sub`, `Mul`, `Div`, etc.) now delegate to this shared function to execute the actual numeric evaluation, reducing boilerplate and centralizing the math logic.

## Consequences

### Positive
*   **DRY (Don't Repeat Yourself):** Significantly reduces code duplication across math operations.
*   **Maintainability:** Future changes to how binary arithmetic is handled (e.g., adding overflow bounds, NaN checking) can be done in one central `binary_op` function rather than scattered across every `exec_*` method.
*   **Readability:** Simplifies the individual instruction handlers.

### Negative
*   **None**
