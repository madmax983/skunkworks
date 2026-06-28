# 012. Decouple Storage from Core

## Status
Accepted

## Context
Circular dependencies were causing build failures, specifically when the `Core` module attempted to reference types defined in `Storage` that simultaneously depended back on `Core` definitions. This tight coupling made it impossible to compile them independently and complicated testing.

## Decision
Move persistence logic to a dedicated crate. The `Storage` module will now be a standalone library that `Core` depends on via a clean Trait Bound interface.

## Consequences
*   **Build times improve:** As crates can be compiled in parallel.
*   **Decoupling:** Circular dependencies are eliminated.
*   **Complexity:** FFI complexity increases as we now have to marshal data across crate boundaries.
