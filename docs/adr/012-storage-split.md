# 012. Decouple Storage from Core

## Status
Proposed

## Context
Circular dependencies were causing build failures, specifically when the `Core` module attempted to reference types defined in `Storage` that simultaneously depended back on `Core` definitions. This tight coupling made it impossible to compile them independently and complicated testing.

## Decision
Move persistence logic to a dedicated crate. The `Storage` module will now be a standalone library that `Core` depends on via a clean Trait Bound interface.

## Consequences
*   **Positive:** Build times improve as crates can be compiled in parallel. Circular dependencies are eliminated.
*   **Negative:** FFI complexity increases as we now have to marshal data across crate boundaries.
