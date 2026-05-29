# 093. Flatten TUI Shared Module Hierarchy

Date: 2026-05-02

## Status
Accepted

## Context
The `crates/tui-shared/src/semantic/` directory exhibited the "Layer Lasagna" anti-pattern. It contained a nested folder hierarchy for only four very simple modules (`action.rs`, `entity.rs`, `region.rs`, and `snapshot.rs`), requiring an unnecessary `semantic/mod.rs` to wire them up. This created redundant directory clutter and increased the cognitive load required to navigate the crate's internal structure without providing any meaningful architectural boundary.

## Decision
Flattened the module hierarchy by moving `action.rs`, `entity.rs`, `region.rs`, and `snapshot.rs` up to the `src/` root directory of `crates/tui-shared`. They are now exported directly in `lib.rs` (previously using a namespace proxy to avoid breaking the public API). The `semantic` directory and its `mod.rs` file have been removed.

## Consequences

### Positive
*   **Reduced Bloat:** Eliminated one unnecessary `mod.rs` file and one nested directory level.
*   **Improved Navigation:** Flatter structure reduces cognitive load and makes finding shared semantic types easier.

### Negative
*   **Wider Scope:** Moving types directly to the crate root increases the surface area of the root module, though in this case the simplicity of the crate justifies the flat structure.
