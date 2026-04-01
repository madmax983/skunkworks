# 076. Merge tui-semantic into tui-shared

Date: 2024-05-24

## Status
Proposed

## Context
The `tui-semantic` crate was originally extracted to provide a standardized "Semantic Bridge" for exposing TUI application states to external agents (ADR 004). Later, the internal structure of this crate was decoupled into smaller modules (ADR 070).

However, maintaining a completely separate crate for these purely structural semantic types added unnecessary complexity to the workspace dependency tree. Because the types and traits defined in `tui-semantic` are intrinsically tied to the lifecycle of TUI applications (which are primarily managed by the `tui-shared` crate), keeping them separated resulted in redundant imports and an artificially bloated crate graph for downstream consumers.

## Decision
Merge the `tui-semantic` crate directly into the `tui-shared` crate under a new `semantic` module (`crates/tui-shared/src/semantic/`). The original data structures (`Snapshot`, `Entity`, `Region`, `Action`) remain intact, but are now accessed via the `tui_shared::semantic::*` namespace.

## Consequences

### Positive
*   **Simplified Dependency Tree:** Downstream experiments now only need to depend on `tui-shared` to gain access to both the TUI lifecycle management and the semantic state bridging types.
*   **Reduced Workspace Bloat:** Eliminates an entire crate from the workspace, speeding up compilation slightly and reducing the number of `Cargo.toml` files to maintain.

### Negative
*   **Loss of Strict Separation:** The `tui-shared` crate now carries the semantic data types, meaning applications that strictly only wanted the UI lifecycle management must also compile the semantic bridging types (though this cost is negligible as they are pure data structures).
