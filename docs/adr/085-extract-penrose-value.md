# 085. Extract Value Enum in Penrose Genes

Date: 2026-04-19

## Status
Proposed

## Context
A circular dependency existed in the `penrose-genes` experiment between `vm.rs` and `penrose.rs` via the `Value` enum. Specifically, `vm.rs` imported `PenroseTiling` from `penrose.rs`, and `penrose.rs` imported the `Value` enum from `vm.rs`. This tight coupling caused build issues, complicated testing, and entangled the Virtual Machine execution domain with the Penrose tiling generation domain.

## Decision
Extracted the `Value` enum into a new, independent `value.rs` module. Both `vm.rs` and `penrose.rs` now depend on `value.rs` as a shared primitive, cleanly breaking the cyclic dependency.

## Consequences

### Positive
*   **Decoupling:** Eliminates the circular dependency, allowing the compiler to cleanly resolve module dependencies.
*   **Separation of Concerns:** The fundamental `Value` type is now independently available, ensuring that modules do not have to import heavy VM execution logic just to use basic value types.

### Negative
*   **Slight Overhead:** Introduces a small additional file and module boundary to manage for a single enum.
