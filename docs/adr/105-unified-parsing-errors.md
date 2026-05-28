# 105. Unified Error Types Across Parsing

Date: 2026-06-01

## Status
Proposed

## Context
During the evolution of `chimera-lang`, parsing operations in the VM (specifically within modules like `babel.rs`) relied on loosely structured error handling, sometimes causing implicit panics or returning ambiguous string-based errors. This fragmentation made it difficult to reliably catch, debug, and surface specific parsing failures when running complex esolang scripts or dynamically generating AST nodes.

## Decision
Established a unified `ParseError` type within the execution parsing layer (e.g., `experiments/chimera-lang/src/vm/babel.rs`). The `run_parser` function and related AST traversal methods now explicitly return `Result<T, ParseError>`, enforcing structured error boundaries.

## Consequences

### Positive
*   **Resilience:** Prevents silent failures or DoS panics caused by malformed scripts.
*   **Predictability:** Ensures all parsing errors are consistently typed, allowing the caller (the VM or REPL) to gracefully handle and report syntactical violations.
*   **Maintainability:** Centralizes error formatting via the implemented `std::error::Error` trait.

### Negative
*   **Verbosity:** Requires explicit `Result` unwrapping or propagation throughout the parsing call stack.
