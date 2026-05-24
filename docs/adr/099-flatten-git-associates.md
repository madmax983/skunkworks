# 099. Flatten Git Associates Layer Lasagna

Date: 2026-05-24

## Status
Proposed

## Context
The `crates/git-associates` crate suffered from the "Layer Lasagna" anti-pattern, where overly nested module directories obscured its relatively simple API. This unnecessary hierarchy increased cognitive load for developers trying to use or modify the crate's core components (`Commit`, `FileChange`, `GitModel`, etc.), without providing a meaningful architectural boundary.

## Decision
Flattened the module hierarchy within `crates/git-associates` by merging the types and implementations into `src/lib.rs` and removing unnecessary nested directories and files. The structs (`GitModel`, `Commit`, `CommitStats`, `FileChange`, `DiffStats`, `Hunk`, `LineChange`) are now exported directly from the crate root.

## Consequences

### Positive
*   **Reduced Bloat:** Eliminated redundant `mod.rs` files and directory levels.
*   **Improved Navigation:** A flatter structure makes it easier to locate and understand the core types and their relationships.
*   **Simpler Usage:** Consumers can import everything directly from the crate root (`use git_associates::*;`) without worrying about internal namespace proxies.

### Negative
*   **Wider API Surface in Root:** Moving all types directly into `src/lib.rs` increases the size and surface area of the root module. However, given the crate's focused scope, this trade-off is justified to avoid excessive nesting.
