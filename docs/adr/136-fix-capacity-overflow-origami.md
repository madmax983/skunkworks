# 136. Fix Capacity Overflow Panic in Origami

Date: 2026-06-27

## Status
Accepted

## Context
During an architectural review by the Atlas persona, it was discovered that `crates/origami/src/lib.rs` calculated capacity for `Vec::with_capacity` using user-provided input parameters. It failed to bound the capacity allocation against the process limits (`isize::MAX / size_of<T>`), which caused fatal `capacity overflow` panics on absurd input sizes.

## Decision
We replaced unbounded vector capacities with bounds checks (`if c <= isize::MAX / ...`) that fallback to returning empty instances rather than panicking on absurd input sizes.

## Consequences
*   **Positive:** Prevents fatal panics and out-of-memory DoS vectors, improving the robustness and stability of the system when handling extreme user input.
